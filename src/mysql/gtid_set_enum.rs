use crate::error::ReplicationError;
use crate::mysql::{GTIDSet, MariadbGTIDSet, MysqlGTIDSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum GtidSetEnum {
    MariadbGTIDSet(MariadbGTIDSet),
    MysqlGTIDSet(MysqlGTIDSet),
}

impl Display for GtidSetEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GtidSetEnum::MariadbGTIDSet(ref v) => v.fmt(f),
            GtidSetEnum::MysqlGTIDSet(ref v) => v.fmt(f),
        }
    }
}

impl GtidSetEnum {
    pub fn encode(&self) -> Result<Vec<u8>, ReplicationError> {
        match self {
            GtidSetEnum::MariadbGTIDSet(ref v) => v.encode(),
            GtidSetEnum::MysqlGTIDSet(ref v) => v.encode(),
        }
    }

    pub fn equal(&self, o: &GtidSetEnum) -> bool {
        match self {
            GtidSetEnum::MariadbGTIDSet(ref v) => {
                if let GtidSetEnum::MariadbGTIDSet(ref other) = o {
                    v.equal(other)
                } else {
                    false
                }
            }
            GtidSetEnum::MysqlGTIDSet(ref v) => {
                if let GtidSetEnum::MysqlGTIDSet(ref other) = o {
                    v.equal(other)
                } else {
                    false
                }
            }
        }
    }

    pub fn contain(&self, o: &Self) -> bool {
        match self {
            GtidSetEnum::MariadbGTIDSet(ref v) => {
                if let GtidSetEnum::MariadbGTIDSet(ref other) = o {
                    v.contain(other)
                } else {
                    false
                }
            }
            GtidSetEnum::MysqlGTIDSet(ref v) => {
                if let GtidSetEnum::MysqlGTIDSet(ref other) = o {
                    v.contain(other)
                } else {
                    false
                }
            }
        }
    }

    pub fn update(&mut self, gtid_str: &str) -> Result<(), ReplicationError> {
        match self {
            GtidSetEnum::MariadbGTIDSet(ref mut v) => v.update(gtid_str),
            GtidSetEnum::MysqlGTIDSet(ref mut v) => v.update(gtid_str),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            GtidSetEnum::MariadbGTIDSet(ref v) => v.len(),
            GtidSetEnum::MysqlGTIDSet(ref v) => v.len(),
        }
    }
}
