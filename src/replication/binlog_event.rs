use crate::error::ReplicationError;
use crate::replication::{Event, EventEnum, EventHeader};
use std::io::Write;

#[derive(Debug, Clone, Default)]
pub struct BinlogEvent {
    // raw binlog data which contains all data, including binlog header and event body, and including crc32 checksum if exists
    pub raw_data: Vec<u8>,

    pub header: Option<EventHeader>,
    pub event: Option<EventEnum>,
}

impl BinlogEvent {
    pub fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        self.header.as_mut().unwrap().dump(writer)?;
        self.event.as_mut().unwrap().dump(writer)?;

        Ok(())
    }
}
