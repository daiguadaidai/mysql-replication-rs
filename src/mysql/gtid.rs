use crate::error::ReplicationError;
use crate::mysql;
use crate::mysql::{GtidSetEnum, MariadbGTIDSet, MysqlGTIDSet};
use std::fmt::{Debug, Display};

pub trait GTIDSet: Display + Debug + Clone {
    // Encode GTID set into binary format used in binlog dump commands
    fn encode(&self) -> Result<Vec<u8>, ReplicationError>;

    fn equal(&self, o: &Self) -> bool;

    fn contain(&self, o: &Self) -> bool;

    fn update(&mut self, gtid_str: &str) -> Result<(), ReplicationError>;

    fn len(&self) -> usize;
}

pub fn parse_gtid_set(flavor: &str, s: &str) -> Result<GtidSetEnum, ReplicationError> {
    match flavor {
        mysql::MYSQL_FLAVOR => Ok(GtidSetEnum::MysqlGTIDSet(MysqlGTIDSet::parse_gtid_set(s)?)),
        mysql::MARIA_DB_FLAVOR => Ok(GtidSetEnum::MariadbGTIDSet(MariadbGTIDSet::parse_gtid_set(
            s,
        )?)),
        _ => Err(ReplicationError::new(format!("invalid flavor {}", flavor))),
    }
}
