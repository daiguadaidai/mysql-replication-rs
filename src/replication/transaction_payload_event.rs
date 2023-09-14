// On The Wire: Field Types
// See also binary_log::codecs::binary::Transaction_payload::fields in MySQL
// https://dev.mysql.com/doc/dev/mysql-server/latest/classbinary__log_1_1codecs_1_1binary_1_1Transaction__payload.html#a9fff7ac12ba064f40e9216565c53d07b

use crate::error::ReplicationError;
use crate::mysql;
use crate::mysql::fixed_length_int;
use crate::replication::parser::BinlogParse;
use crate::replication::{BinlogEvent, Event, FormatDescriptionEvent, BINLOG_CHECKSUM_ALG_OFF};
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, Write};

pub const OTW_PAYLOAD_HEADER_END_MARK: u64 = 0;
pub const OTW_PAYLOAD_SIZE_FIELD: u64 = 1;
pub const OTW_PAYLOAD_COMPRESSION_TYPE_FIELD: u64 = 2;
pub const OTW_PAYLOAD_UNCOMPRESSED_SIZE_FIELD: u64 = 3;

// Compression Types
pub const ZSTD: u64 = 0;
pub const NONE: u64 = 255;

#[derive(Debug, Clone, Default)]
pub struct TransactionPayloadEvent {
    pub format: Option<FormatDescriptionEvent>,
    pub size: u64,
    pub uncompressed_size: u64,
    pub compression_type: u64,
    pub payload: Vec<u8>,
    pub events: Vec<BinlogEvent>,
}

impl Event for TransactionPayloadEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Payload Size: {}\n", self.size)?;
        write!(
            writer,
            "Payload Uncompressed Size: {}\n",
            self.uncompressed_size
        )?;
        write!(
            writer,
            "Payload CompressionType: {}\n",
            self.compression_type()
        )?;
        write!(writer, "Payload Body: \n{}", hex::encode(&self.payload))?;
        writeln!(
            writer,
            "=== Start of events decoded from compressed payload ==="
        )?;
        for event in self.events.iter_mut() {
            event.dump(writer)?;
        }
        writeln!(
            writer,
            "=== End of events decoded from compressed payload ==="
        )?;
        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        self.decode_fields(data)?;
        self.decode_payload()
    }
}

impl TransactionPayloadEvent {
    pub fn compression_type(&self) -> String {
        match self.compression_type {
            ZSTD => String::from("ZSTD"),
            NONE => String::from("NONE"),
            _ => String::from("Unknown"),
        }
    }

    pub fn decode_fields(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut offset = 0_usize;
        loop {
            let field_type = mysql::fixed_length_int(&data[offset..offset + 1]);
            offset += 1;

            if field_type == OTW_PAYLOAD_HEADER_END_MARK {
                self.payload = data[offset..].to_vec();
                break;
            } else {
                let field_length = mysql::fixed_length_int(&data[offset..offset + 1]) as usize;
                offset += 1;

                match field_type {
                    OTW_PAYLOAD_SIZE_FIELD => {
                        self.size = fixed_length_int(&data[offset..offset + field_length]);
                    }
                    OTW_PAYLOAD_COMPRESSION_TYPE_FIELD => {
                        self.compression_type =
                            fixed_length_int(&data[offset..offset + field_length]);
                    }
                    OTW_PAYLOAD_UNCOMPRESSED_SIZE_FIELD => {
                        self.uncompressed_size =
                            fixed_length_int(&data[offset..offset + field_length])
                    }
                    _ => {
                        return Err(ReplicationError::new(format!(
                            "OTW_PAYLOAD field type Unknown"
                        )))
                    }
                }

                offset += field_length
            }
        }

        Ok(())
    }

    pub fn decode_payload(&mut self) -> Result<(), ReplicationError> {
        if self.compression_type != ZSTD {
            return Err(ReplicationError::new(format!(
                "TransactionPayloadEvent has compression type {} ({})",
                self.compression_type,
                self.compression_type()
            )));
        }

        let mut decoder = zstd::stream::read::Decoder::new(self.payload.as_slice())?;
        let mut payload_uncompressed = Vec::new();
        decoder.read_to_end(&mut payload_uncompressed)?;

        // The uncompressed data needs to be split up into individual events for Parse()
        // to work on them. We can't use e.parser directly as we need to disable checksums
        // but we still need the initialization from the FormatDescriptionEvent. We can't
        // modify e.parser as it is used elsewhere.
        let mut parser = BinlogParse::new();
        let mut format = self.format.clone();
        format.as_mut().unwrap().check_sum_algorithm = BINLOG_CHECKSUM_ALG_OFF;
        parser.format = format;

        let mut offset = 0_usize;
        loop {
            let payload_uncompressed_length = payload_uncompressed.len();
            if offset + 13 > payload_uncompressed_length {
                break;
            }

            let event_length =
                LittleEndian::read_u32(&payload_uncompressed[offset + 9..offset + 13]) as usize;
            if offset + event_length > payload_uncompressed_length {
                return Err(ReplicationError::new(format!("Event length of {} with offset {} in uncompressed payload exceeds payload length of {}", event_length, offset, payload_uncompressed_length)));
            }

            let data = &payload_uncompressed[offset..offset + event_length];
            let pe = parser.parse(data)?;
            self.events.push(pe);
            offset += event_length;
        }

        Ok(())
    }
}
