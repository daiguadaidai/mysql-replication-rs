use crate::error::ReplicationError;
use crate::mysql::GTIDSet;
use crate::utils;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Seek, SeekFrom, Write};
use uuid::Uuid;

// Like MySQL GTID Interval struct, [start, stop), left closed and right open
// See MySQL rpl_gtid.h
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Interval {
    pub start: i64,
    pub stop: i64,
}

impl Interval {
    pub fn cmp(&self, other: &Interval) -> Ordering {
        if self.start < other.start {
            return Ordering::Less;
        } else if self.start > other.start {
            return Ordering::Greater;
        } else {
            return self.stop.cmp(&other.stop);
        }
    }
}

// Interval is [start, stop), but the GTID string's format is [n] or [n1-n2], closed interval
impl Interval {
    pub fn parse_interval(data: &str) -> Result<Interval, ReplicationError> {
        let mut interval = Interval { start: 0, stop: 0 };

        let p = data.split("-").collect::<Vec<&str>>();
        match p.len() {
            1 => match p[0].parse::<i64>() {
                Ok(start) => {
                    interval.start = start;
                    interval.stop = interval.start + 1;
                    Ok(())
                }
                Err(e) => Err(ReplicationError::ParseError(e)),
            },
            2 => match p[0].parse::<i64>() {
                Ok(start) => {
                    interval.start = start;
                    let rs = match p[1].parse::<i64>() {
                        Ok(stop) => {
                            interval.stop = stop;
                            Ok(())
                        }
                        Err(e) => Err(ReplicationError::ParseError(e)),
                    };
                    interval.stop += 1;

                    rs
                }
                Err(e) => Err(ReplicationError::ParseError(e)),
            },

            _ => Err(ReplicationError::NormalError(String::from(
                "invalid interval format, must n[-n]",
            ))),
        }?;

        if interval.stop <= interval.start {
            return Err(ReplicationError::NormalError(String::from(
                "invalid interval format, must n[-n] and the end must >= start",
            )));
        }

        Ok(interval)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.stop == self.start + 1 {
            write!(f, "{start}", start = self.start)
        } else {
            write!(
                f,
                "{start}-{stop}",
                start = self.start,
                stop = self.stop - 1
            )
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IntervalSlice {
    pub s: Vec<Interval>,
}

impl IntervalSlice {
    pub fn sort(&mut self) {
        self.s.sort_by(|a, b| a.cmp(b));
    }

    pub fn replace(&mut self, i: usize, data: Interval) {
        self.s[i] = data
    }

    pub fn normalize(&mut self) -> IntervalSlice {
        let mut n = IntervalSlice { s: vec![] };
        if self.s.len() == 0 {
            return n;
        }

        self.sort();

        n.s.push(self.s[0].clone());

        for i in 1..self.s.len() {
            let last = &n.s[n.s.len() - 1];
            if self.s[i].start > last.stop {
                n.s.push(self.s[i].clone());
                continue;
            } else {
                let mut stop = self.s[i].stop;
                if last.stop > stop {
                    stop = last.stop;
                }

                let new_interval = Interval {
                    start: last.start,
                    stop,
                };
                n.replace(n.s.len() - 1, new_interval);
            }
        }

        n
    }

    pub fn insert_interval(&mut self, interval: Interval) {
        let mut count = 0;

        self.s.push(interval);
        let total = self.s.len();
        let mut i = total - 1;
        while i > 0 {
            if self.s[i].stop < self.s[i - 1].start {
                self.s.swap(i, i - 1)
            } else if self.s[i].start > self.s[i - 1].stop {
                break;
            } else {
                self.s[i - 1].start = cmp::min(self.s[i - 1].start, self.s[i].start);
                self.s[i - 1].stop = cmp::max(self.s[i - 1].stop, self.s[i].stop);
                count += 1;
            }

            i -= 1;
        }
        if count > 0 {
            i += 1;

            if i + count < total {
                let (left, right) = self.s[i..].split_at_mut(count);
                utils::vec::slice_copy(left, right);
            }

            let start_remove_index = total - count;
            for _ in start_remove_index..total {
                self.s.remove(start_remove_index);
            }
        }
    }

    // Contain returns true if sub in s
    pub fn contain(&self, sub: &IntervalSlice) -> bool {
        let mut j = 0;
        for i in 0..sub.s.len() {
            for _ in j..self.s.len() {
                if sub.s[i].start > self.s[j].stop {
                    j += 1;
                    continue;
                } else {
                    break;
                }
            }
            if j == self.s.len() {
                return false;
            }

            if sub.s[i].start < self.s[j].start || sub.s[i].stop > self.s[j].stop {
                return false;
            }
        }

        return true;
    }

    pub fn equal(&self, o: &IntervalSlice) -> bool {
        if self.s.len() != o.s.len() {
            return false;
        }

        for i in 0..self.s.len() {
            if self.s[i].start != o.s[i].start || self.s[i].stop != o.s[i].stop {
                return false;
            }
        }

        true
    }

    pub fn compare(&self, o: &IntervalSlice) -> isize {
        if self.equal(o) {
            return 0;
        } else if self.contain(o) {
            return 1;
        } else {
            return -1;
        }
    }
}

// Refer http://dev.mysql.com/doc/refman/5.6/en/replication-gtids-concepts.html
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UUIDSet {
    pub sid: Uuid,

    pub sntervals: IntervalSlice,
}

impl Display for UUIDSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bytes = self.bytes().unwrap();
        write!(f, "{data}", data = String::from_utf8_lossy(&bytes))
    }
}

impl UUIDSet {
    pub fn parse_uuid_set(data: &str) -> Result<UUIDSet, ReplicationError> {
        let data = data.trim();
        let sep = data.split(":").collect::<Vec<&str>>();
        if sep.len() < 2 {
            return Err(ReplicationError::NormalError(String::from(
                "invalid GTID format, must UUID:interval[:interval]",
            )));
        }

        let mut s = UUIDSet {
            sid: Default::default(),
            sntervals: IntervalSlice { s: vec![] },
        };
        s.sid = Uuid::parse_str(sep[0])?;

        // Handle interval
        for i in 1..sep.len() {
            let interval = Interval::parse_interval(sep[i])?;
            s.sntervals.s.push(interval)
        }

        s.sntervals = s.sntervals.normalize();

        Ok(s)
    }

    pub fn contain(&self, sub: &UUIDSet) -> bool {
        if self.sid != sub.sid {
            return false;
        }

        return self.sntervals.contain(&sub.sntervals);
    }

    pub fn bytes(&self) -> Result<Vec<u8>, ReplicationError> {
        let mut buf = Vec::<u8>::new();
        buf.write(self.sid.to_string().as_bytes())?;

        for interval in &self.sntervals.s {
            buf.push(u8::try_from(':')?);
            buf.write(interval.to_string().as_bytes())?;
        }

        Ok(buf)
    }

    pub fn add_interval(&mut self, other: &IntervalSlice) {
        self.sntervals.s.append(&mut other.s.clone());
        self.sntervals = self.sntervals.normalize();
    }

    pub fn minus_interval(&mut self, interval_slice: &mut IntervalSlice) {
        let mut n = IntervalSlice { s: vec![] };
        let interval_slice = &interval_slice.normalize();

        let (mut i, mut j) = (0, 0);
        let mut minuend = Interval { start: 0, stop: 0 };
        let mut subtrahend;
        while i < self.sntervals.s.len() {
            if minuend.stop != self.sntervals.s[i].stop {
                // `i` changed?
                minuend = self.sntervals.s[i].clone()
            }

            if j < interval_slice.s.len() {
                subtrahend = interval_slice.s[j].clone()
            } else {
                subtrahend = Interval {
                    start: i64::MAX,
                    stop: i64::MAX,
                }
            }

            if minuend.stop <= subtrahend.start {
                // no overlapping
                n.s.push(minuend.clone());
                i += 1;
            } else if minuend.start >= subtrahend.stop {
                // no overlapping
                j += 1;
            } else {
                if minuend.start < subtrahend.start && minuend.stop <= subtrahend.stop {
                    n.s.push(Interval {
                        start: minuend.start,
                        stop: subtrahend.start,
                    });
                    i += 1;
                } else if minuend.start >= subtrahend.start && minuend.stop <= subtrahend.stop {
                    // minuend is completely removed
                    i += 1;
                } else if minuend.start < subtrahend.start && minuend.stop > subtrahend.stop {
                    n.s.push(Interval {
                        start: minuend.start,
                        stop: subtrahend.start,
                    });
                    minuend = Interval {
                        start: subtrahend.stop,
                        stop: subtrahend.stop,
                    };
                    j += 1;
                } else {
                    panic!("should never be here")
                }
            }
        }

        self.sntervals = n.normalize();
    }

    fn encode_private<W: Write>(&self, w: &mut W) {
        let _ = w.write(self.sid.as_ref());

        let n = self.sntervals.s.len() as i64;

        let _ = w.write_i64::<LittleEndian>(n);
        for interval in &self.sntervals.s {
            let _ = w.write_i64::<LittleEndian>(interval.start);
            let _ = w.write_i64::<LittleEndian>(interval.stop);
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        self.encode_private(&mut buf);
        buf
    }

    fn decode_private(&mut self, data: &[u8]) -> Result<usize, ReplicationError> {
        if data.len() < 24 {
            return Err(ReplicationError::NormalError(String::from(
                "invalid uuid set buffer, less 24",
            )));
        }

        let mut rdr = Cursor::new(data);
        self.sid = Uuid::from_slice(&data[..16])?;
        rdr.seek(SeekFrom::Current(16))?;

        let n = rdr.read_i64::<LittleEndian>()?;
        let right = (16 * n as usize) + rdr.position() as usize;
        if data.len() < right {
            return Err(ReplicationError::NormalError(format!(
                "invalid uuid set buffer, must {}, but {}",
                right,
                data.len()
            )));
        }

        self.sntervals = IntervalSlice { s: vec![] };

        for _ in 0..n {
            let mut interval = Interval { start: 0, stop: 0 };
            interval.start = rdr.read_i64::<LittleEndian>()?;
            interval.stop = rdr.read_i64::<LittleEndian>()?;
            self.sntervals.s.push(interval);
        }

        Ok(rdr.position() as usize)
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let n = self.decode_private(data)?;
        if n != data.len() {
            return Err(ReplicationError::NormalError(format!(
                "invalid uuid set buffer, must {}, but {}",
                n,
                data.len()
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MysqlGTIDSet {
    pub sets: HashMap<String, UUIDSet>,
}

impl MysqlGTIDSet {
    pub fn parse_gtid_set(data: &str) -> Result<MysqlGTIDSet, ReplicationError> {
        let mut s = MysqlGTIDSet {
            sets: Default::default(),
        };
        if data.is_empty() {
            return Ok(s);
        }
        let sp = data.split(",").collect::<Vec<&str>>();
        for i in 0..sp.len() {
            let set = UUIDSet::parse_uuid_set(sp[i])?;
            s.add_set(&set)
        }

        return Ok(s);
    }

    pub fn decode(data: &[u8]) -> Result<MysqlGTIDSet, ReplicationError> {
        if data.len() < 8 {
            return Err(ReplicationError::NormalError(String::from(
                "invalid gtid set buffer, less 4",
            )));
        }

        let mut rdr = Cursor::new(data);
        let n = rdr.read_u64::<LittleEndian>()? as usize;

        let mut s = MysqlGTIDSet {
            sets: HashMap::with_capacity(n),
        };
        for _ in 0..n {
            let mut set = UUIDSet {
                sid: Default::default(),
                sntervals: IntervalSlice { s: vec![] },
            };
            let size = set.decode_private(&data[rdr.position() as usize..])? as i64;
            rdr.seek(SeekFrom::Current(size))?;

            s.add_set(&set)
        }

        Ok(s)
    }

    pub fn add_set(&mut self, set: &UUIDSet) {
        let sid = set.sid.to_string();
        if let Some(old_set) = self.sets.get_mut(&sid) {
            old_set.add_interval(&set.sntervals)
        } else {
            self.sets.insert(sid, set.clone());
        }
    }

    pub fn minus_set(&mut self, set: &mut UUIDSet) {
        let sid = set.sid.to_string();
        if let Some(uuid_set) = self.sets.get_mut(&sid) {
            uuid_set.minus_interval(&mut set.sntervals);
            if uuid_set.sntervals.s.len() == 0 {
                self.sets.remove(&sid);
            }
        }
    }

    pub fn add_gtid(&mut self, uuid: &Uuid, gno: i64) {
        let sid = uuid.to_string();
        if let Some(uuid_set) = self.sets.get_mut(&sid) {
            uuid_set.sntervals.insert_interval(Interval {
                start: gno,
                stop: gno + 1,
            });
        } else {
            self.sets.insert(
                sid,
                UUIDSet {
                    sid: uuid.clone(),
                    sntervals: IntervalSlice {
                        s: vec![Interval {
                            start: gno,
                            stop: gno + 1,
                        }],
                    },
                },
            );
        }
    }

    pub fn add(&mut self, addend: &MysqlGTIDSet) -> Result<(), ReplicationError> {
        for (_, uuid_set) in &addend.sets {
            self.add_set(uuid_set)
        }

        Ok(())
    }

    pub fn minus(&mut self, subtrahend: &mut MysqlGTIDSet) -> Result<(), ReplicationError> {
        for (_, uuid_set) in subtrahend.sets.iter_mut() {
            self.minus_set(uuid_set)
        }

        Ok(())
    }
}

impl Display for MysqlGTIDSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // there is only one element in gtid set
        if self.sets.len() == 1 {
            for (_, set) in &self.sets {
                return write!(f, "{}", set.to_string());
            }
        }

        // sort multi set
        let mut sets = Vec::<String>::with_capacity(self.sets.len());
        for (_, set) in &self.sets {
            sets.push(set.to_string());
        }
        sets.sort();

        let mut sep = "";
        for set in &sets {
            write!(f, "{}", sep)?;
            write!(f, "{}", set)?;
            sep = ","
        }

        Ok(())
    }
}

impl GTIDSet for MysqlGTIDSet {
    fn encode(&self) -> Result<Vec<u8>, ReplicationError> {
        let mut buf = Vec::<u8>::new();
        buf.write_u64::<LittleEndian>(self.sets.len() as u64)?;
        for (_, set) in &self.sets {
            set.encode_private(&mut buf)
        }

        Ok(buf)
    }

    fn equal(&self, o: &Self) -> bool {
        if o.sets.len() != self.sets.len() {
            return false;
        }

        for (key, o_set) in &o.sets {
            if let Some(s_set) = self.sets.get(key) {
                if !o_set.sntervals.equal(&s_set.sntervals) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn contain(&self, o: &Self) -> bool {
        for (key, o_set) in &o.sets {
            if let Some(s_set) = self.sets.get(key) {
                if !s_set.contain(o_set) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn update(&mut self, gtid_str: &str) -> Result<(), ReplicationError> {
        let gtid_set = MysqlGTIDSet::parse_gtid_set(gtid_str)?;
        for (_, uuid_set) in &gtid_set.sets {
            self.add_set(uuid_set)
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.sets.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use crate::mysql::Interval;
    use crate::utils;
    use uuid::Uuid;

    #[test]
    fn f() {
        let total = 10;
        for i in (0..total - 1).rev() {
            print!("{:?},", i);
        }
    }

    #[test]
    fn e() {
        let uuid = Uuid::now_v1(&[1, 2, 3, 4, 5, 6]);
        println!("{}", uuid.to_string())
    }

    #[test]
    fn d() {
        let mut data = vec![1, 2, 3];
        let mut data1 = vec![5, 6, 7, 8];

        data.append(&mut data1);
        println!("{:?}", data);
        println!("{:?}", data1);
    }

    #[test]
    fn c() {
        let mut data = vec![1, 2, 3, 9, 8, 7, 6, 5];
        let (left, right) = data.split_at_mut(4);

        println!("{:?}", left);
        println!("{:?}", right);

        utils::vec::slice_copy(left, &right[2..]);

        println!("{:?}", left);
        println!("{:?}", &right[2..]);
        println!("{:?}", data)
    }

    #[test]
    fn b() {
        let i = 0;
        for i in (3..10).rev() {
            println!("{}", i)
        }
        println!("{}", i);
    }

    #[test]
    fn a() {
        let s = "sss-ssh";
        let p = s.split("-").collect::<Vec<&str>>();
        let mut interval = Interval { start: 0, stop: 0 };

        let a = match p.len() {
            1 => match p[0].parse::<i64>() {
                Ok(start) => {
                    interval.start = start;
                    interval.stop = interval.start + 1;
                    Ok(())
                }
                Err(e) => Err(ReplicationError::ParseError(e)),
            },
            2 => match p[0].parse::<i64>() {
                Ok(start) => {
                    interval.start = start;
                    let rs = match p[1].parse::<i64>() {
                        Ok(stop) => {
                            interval.stop = stop;
                            Ok(())
                        }
                        Err(e) => Err(ReplicationError::ParseError(e)),
                    };
                    interval.stop += 1;

                    rs
                }
                Err(e) => Err(ReplicationError::ParseError(e)),
            },

            _ => Err(ReplicationError::NormalError(String::from(
                "invalid interval format, must n[-n]",
            ))),
        };

        if let Err(e) = a {
            println!("{}", e);
            return;
        }
    }
}
