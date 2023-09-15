use crate::error::ReplicationError;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

// For binlog filename + position based replication
pub struct Position {
    pub name: String,
    pub pos: u32,
}

impl Position {
    pub fn compare(&self, o: &Position) -> Result<Ordering, ReplicationError> {
        // First compare binlog name
        let name_cmp = compare_binlog_filename(&self.name, &o.name)?;
        match name_cmp {
            Ordering::Equal => {}
            _ => return Ok(name_cmp),
        }

        // Same binlog file, compare position
        if self.pos > o.pos {
            return Ok(Ordering::Greater);
        } else if self.pos < o.pos {
            return Ok(Ordering::Less);
        } else {
            return Ok(Ordering::Equal);
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", &self.name, self.pos)
    }
}

fn _split_binlog_name(n: &str) -> Result<(String, usize), ReplicationError> {
    // mysqld appends a numeric extension to the binary log base name to generate binary log file names
    // ...
    // If you supply an extension in the log name (for example, --log-bin=base_name.extension),
    // the extension is silently removed and ignored.
    // ref: https://dev.mysql.com/doc/refman/8.0/en/binary-log.html

    if let Some(i) = n.as_bytes().iter().rposition(|&byte| byte == b'.') {
        let seq = n[i + 1..].parse::<usize>().map_err(|e| {
            ReplicationError::new(format!(
                "binlog file {} doesn't contain numeric extension",
                e.to_string()
            ))
        })?;

        Ok((n[..i].to_string(), seq))
    } else {
        // try keeping backward compatibility
        Ok((n.to_string(), 0))
    }
}

pub fn compare_binlog_filename(a: &str, b: &str) -> Result<Ordering, ReplicationError> {
    // sometimes it's convenient to construct a `Position` literal with no `Name`
    if a == "" && b == "" {
        return Ok(Ordering::Equal);
    } else if a == "" {
        return Ok(Ordering::Less);
    } else if b == "" {
        return Ok(Ordering::Greater);
    }

    let (a_base, a_seq) = _split_binlog_name(a)?;
    let (b_base, b_seq) = _split_binlog_name(b)?;

    if a_base > b_base {
        return Ok(Ordering::Greater);
    } else if a_base < b_base {
        return Ok(Ordering::Less);
    }

    if a_seq > b_seq {
        return Ok(Ordering::Greater);
    } else if a_seq < b_seq {
        return Ok(Ordering::Less);
    } else {
        return Ok(Ordering::Equal);
    }
}
