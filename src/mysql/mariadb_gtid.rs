use crate::error::ReplicationError;
use crate::mysql::GTIDSet;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// MariadbGTID represent mariadb gtid, [domain ID]-[server-id]-[sequence]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MariadbGTID {
    pub domain_id: u32,
    pub server_id: u32,
    pub sequence_number: u64,
}

impl MariadbGTID {
    // ParseMariadbGTID parses mariadb gtid, [domain ID]-[server-id]-[sequence]
    pub fn parse_gtid(data: &str) -> Result<MariadbGTID, ReplicationError> {
        let mut gtid = MariadbGTID {
            domain_id: 0,
            server_id: 0,
            sequence_number: 0,
        };

        if data.is_empty() {
            return Ok(gtid);
        }

        let seps = data.split("-").collect::<Vec<&str>>();
        if seps.len() != 3 {
            return Err(ReplicationError::new(format!(
                "invalid Mariadb GTID {}, must domain-server-sequence",
                data
            )));
        }

        gtid.domain_id = seps[0].parse::<u32>().map_err(|e| {
            ReplicationError::new(format!(
                "invalid MariaDB GTID Domain ID ({}): {}",
                seps[0],
                e.to_string()
            ))
        })?;

        gtid.server_id = seps[1].parse::<u32>().map_err(|e| {
            ReplicationError::new(format!(
                "invalid MariaDB GTID Server ID ({}): {}",
                seps[1],
                e.to_string()
            ))
        })?;

        gtid.sequence_number = seps[2].parse::<u64>().map_err(|e| {
            ReplicationError::new(format!(
                "invalid MariaDB GTID Sequence number ({}): {}",
                seps[2],
                e.to_string()
            ))
        })?;

        Ok(gtid)
    }

    // Contain return whether one mariadb gtid covers another mariadb gtid
    pub fn contain(&self, other: &MariadbGTID) -> bool {
        self.domain_id == other.domain_id && self.sequence_number >= other.sequence_number
    }

    pub fn forward(&mut self, newer: &MariadbGTID) -> Result<(), ReplicationError> {
        if newer.domain_id != self.domain_id {
            return Err(ReplicationError::new(format!(
                "{newer} is not same with doamin of {self_gtid}",
                newer = newer.to_string(),
                self_gtid = self.to_string()
            )));
        }
        /*
            Here's a simplified example of binlog events.
            Although I think one domain should have only one update at same time, we can't limit the user's usage.
            we just output a warn log and let it go on
            | mysqld-bin.000001 | 1453 | Gtid              |       112 |        1495 | BEGIN GTID 0-112-6  |
            | mysqld-bin.000001 | 1624 | Xid               |       112 |        1655 | COMMIT xid=74       |
            | mysqld-bin.000001 | 1655 | Gtid              |       112 |        1697 | BEGIN GTID 0-112-7  |
            | mysqld-bin.000001 | 1826 | Xid               |       112 |        1857 | COMMIT xid=75       |
            | mysqld-bin.000001 | 1857 | Gtid              |       111 |        1899 | BEGIN GTID 0-111-5  |
            | mysqld-bin.000001 | 1981 | Xid               |       111 |        2012 | COMMIT xid=77       |
            | mysqld-bin.000001 | 2012 | Gtid              |       112 |        2054 | BEGIN GTID 0-112-8  |
            | mysqld-bin.000001 | 2184 | Xid               |       112 |        2215 | COMMIT xid=116      |
            | mysqld-bin.000001 | 2215 | Gtid              |       111 |        2257 | BEGIN GTID 0-111-6  |
        */
        if newer.sequence_number <= self.sequence_number {
            log::warn!("out of order binlog appears with gtid {newer} vs current position gtid {self_gtid}", newer=newer.to_string(), self_gtid = self.to_string());
        }

        self.server_id = newer.server_id;
        self.sequence_number = newer.sequence_number;

        Ok(())
    }
}

impl Display for MariadbGTID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.domain_id == 0 && self.server_id == 0 && self.sequence_number == 0 {
            Ok(())
        } else {
            write!(
                f,
                "{domain_id}-{server_id}-{sequence_number}",
                domain_id = self.domain_id,
                server_id = self.server_id,
                sequence_number = self.sequence_number
            )
        }
    }
}

// MariadbGTIDSet is a set of mariadb gtid
#[derive(Debug, Default, Clone)]
pub struct MariadbGTIDSet {
    pub sets: HashMap<u32, MariadbGTID>,
}

impl MariadbGTIDSet {
    // ParseMariadbGTIDSet parses str into mariadb gtid sets
    pub fn parse_gtid_set(data: &str) -> Result<Self, ReplicationError> {
        let mut s = MariadbGTIDSet {
            sets: Default::default(),
        };

        if data.is_empty() {
            return Ok(s);
        }

        s.update(data)?;

        Ok(s)
    }

    // AddSet adds mariadb gtid into mariadb gtid set
    pub fn add_set(&mut self, gtid: &MariadbGTID) -> Result<(), ReplicationError> {
        if let Some(o) = self.sets.get_mut(&gtid.domain_id) {
            o.forward(&gtid)?;
        } else {
            self.sets.insert(gtid.domain_id, gtid.clone());
        }

        Ok(())
    }
}

impl Display for MariadbGTIDSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sets = self
            .sets
            .iter()
            .map(|(_, set)| set.to_string())
            .collect::<Vec<String>>();
        sets.sort();

        write!(f, "{}", sets.join(","))
    }
}

impl GTIDSet for MariadbGTIDSet {
    // Encode encodes mariadb gtid set
    fn encode(&self) -> Result<Vec<u8>, ReplicationError> {
        let mut buf = String::new();

        let mut sep = "";
        for (_, gtid) in &self.sets {
            buf.push_str(sep);
            buf.push_str(&gtid.to_string());
            sep = ","
        }

        Ok(buf.into_bytes())
    }

    fn equal(&self, o: &Self) -> bool {
        if o.sets.len() != self.sets.len() {
            return false;
        }

        for (domain_id, gtid) in &o.sets {
            if let Some(s_set) = self.sets.get(domain_id) {
                if gtid != s_set {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn contain(&self, o: &Self) -> bool {
        for (domain_id, gtid) in &o.sets {
            if let Some(s_set) = self.sets.get(domain_id) {
                if !s_set.contain(&gtid) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    // Update updates mariadb gtid set
    fn update(&mut self, gtid_str: &str) -> Result<(), ReplicationError> {
        let sp = gtid_str.split(",").collect::<Vec<&str>>();
        for i in 0..sp.len() {
            let gtid = MariadbGTID::parse_gtid(sp[i])?;
            self.add_set(&gtid)?;
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.sets.len()
    }
}
