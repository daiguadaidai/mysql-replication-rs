use crate::error::ReplicationError;
use crate::mysql::{
    decompress_mariadb_data, fixed_length_int, length_encoded_int, GtidSetEnum, MariadbGTID,
};
use crate::replication::{
    micro_sec_timestamp_to_time, EventType, IntVarEventType, BINLOG_CHECKSUM_ALG_UNDEF,
    BINLOG_MARIADB_FL_DDL, BINLOG_MARIADB_FL_GROUP_COMMIT_ID, BINLOG_MARIADB_FL_STANDALONE,
};
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::NaiveDateTime;
use std::io::{Cursor, Seek, SeekFrom, Write};
use uuid::Uuid;

pub const EVENT_HEADER_SIZE: usize = 19;
pub const SID_LENGTH: usize = 16;
pub const LOGICAL_TIMESTAMP_TYPE_CODE: usize = 2;
pub const PART_LOGICAL_TIMESTAMP_LENGTH: usize = 8;
pub const BINLOG_CHECKSUM_LENGTH: usize = 4;
pub const UNDE_FINED_SERVER_VER: usize = 999999; // UNDEFINED_SERVER_VERSION

pub trait Event {
    //Dump Event, format like python-mysql-replication
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError>;

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError>;
}

#[derive(Debug, Default, Clone)]
pub struct EventHeader {
    pub timestamp: u32,
    pub event_type: EventType,
    pub server_id: u32,
    pub event_size: u32,
    pub log_pos: u32,
    pub flags: u16,
}

impl Event for EventHeader {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(
            writer,
            "=== {event_type} ===\n",
            event_type = &self.event_type.to_string()
        )?;

        let date = NaiveDateTime::from_timestamp_opt(self.timestamp as i64, 0).ok_or(
            ReplicationError::new(format!(
                "timestamp parse datetime error. timestamp: {timestamp}",
                timestamp = self.timestamp
            )),
        )?;
        write!(
            writer,
            "Date: {date}\n",
            date = date.format("%Y-%m-%d %H:%M:%S").to_string()
        )?;

        write!(writer, "Log position: {log_pos}\n", log_pos = self.log_pos)?;

        write!(
            writer,
            "Event size: {event_size}\n",
            event_size = self.event_size
        )?;

        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        if data.len() < EVENT_HEADER_SIZE {
            return Err(ReplicationError::NormalError(format!(
                "header size too short {data_len}, must 19",
                data_len = data.len()
            )));
        }

        let mut rdr = Cursor::new(data);

        self.timestamp = rdr.read_u32::<LittleEndian>()?;
        self.event_type = EventType::from(rdr.read_u8()?);
        self.server_id = rdr.read_u32::<LittleEndian>()?;
        self.event_size = rdr.read_u32::<LittleEndian>()?;
        self.log_pos = rdr.read_u32::<LittleEndian>()?;
        self.flags = rdr.read_u16::<LittleEndian>()?;

        if self.event_size < EVENT_HEADER_SIZE as u32 {
            return Err(ReplicationError::NormalError(format!(
                "invalid event size {event_size}, must >= 19",
                event_size = self.event_size
            )));
        }

        Ok(())
    }
}

const CHECKSUM_VERSION_SPLIT_MYSQL: [isize; 3] = [5, 6, 1];
const CHECKSUM_VERSION_PRODUCT_MYSQL: isize =
    (CHECKSUM_VERSION_SPLIT_MYSQL[0] * 256 + CHECKSUM_VERSION_SPLIT_MYSQL[1]) * 256
        + CHECKSUM_VERSION_SPLIT_MYSQL[2];

const CHECKSUM_VERSION_SPLIT_MARIA_DB: [isize; 3] = [5, 3, 0];
const CHECKSUM_VERSION_PRODUCT_MARIA_DB: isize =
    (CHECKSUM_VERSION_SPLIT_MARIA_DB[0] * 256 + CHECKSUM_VERSION_SPLIT_MARIA_DB[1]) * 256
        + CHECKSUM_VERSION_SPLIT_MARIA_DB[2];

// server version format X.Y.Zabc, a is not . or number
fn split_server_version(server: &str) -> Vec<isize> {
    let seps = server.split(".").collect::<Vec<&str>>();
    if seps.len() < 3 {
        return vec![0, 0, 0];
    }

    let x = seps[0].parse::<isize>().map_or(0, |v| v);
    let y = seps[1].parse::<isize>().map_or(0, |v| v);

    let mut index = 0;
    for (i, c) in seps[2].char_indices() {
        if !c.is_numeric() {
            index = i;
            break;
        }
    }

    let z = seps[2][0..index].parse::<isize>().map_or(0, |v| v);

    vec![x, y, z]
}

fn calc_version_product(server: &str) -> isize {
    let version_split = split_server_version(server);

    return (version_split[0] * 256 + version_split[1]) * 256 + version_split[2];
}

#[derive(Debug, Default, Clone)]
pub struct FormatDescriptionEvent {
    pub version: u16,
    //len = 50
    pub server_version: Vec<u8>,
    pub create_timestamp: u32,
    pub event_header_length: u8,
    pub event_type_header_lengths: Vec<u8>,

    // 0 is off, 1 is for CRC32, 255 is undefined
    pub check_sum_algorithm: u8,
}

impl Event for FormatDescriptionEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Version: {version}\n", version = self.version)?;
        write!(
            writer,
            "Server ServerVersion: {server_version}\n",
            server_version = String::from_utf8_lossy(&self.server_version)
        )?;
        write!(
            writer,
            "Checksum algorithm: {check_sum_algorithm}\n",
            check_sum_algorithm = self.check_sum_algorithm
        )?;

        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);

        self.version = rdr.read_u16::<LittleEndian>()?;

        let start_pos = rdr.position() as usize;
        let end_pos = start_pos + 50;
        self.server_version = data[start_pos..end_pos].to_vec();
        rdr.seek(SeekFrom::Current(50))?;

        self.create_timestamp = rdr.read_u32::<LittleEndian>()?;
        self.event_header_length = rdr.read_u8()?;

        if self.event_header_length != EVENT_HEADER_SIZE as u8 {
            return Err(ReplicationError::NormalError(format!(
                "invalid event header length {event_header_length}, must 19",
                event_header_length = self.event_header_length
            )));
        }

        let server = String::from_utf8_lossy(&self.server_version).to_string();
        let mut checksum_product = CHECKSUM_VERSION_PRODUCT_MYSQL;
        if server.contains("mariadb") {
            checksum_product = CHECKSUM_VERSION_PRODUCT_MARIA_DB;
        }

        if calc_version_product(&server) >= checksum_product {
            // here, the last 5 bytes is 1 byte check sum alg type and 4 byte checksum if exists
            self.check_sum_algorithm = data[data.len() - 5];
            self.event_type_header_lengths = data[rdr.position() as usize..data.len() - 5].to_vec()
        } else {
            self.check_sum_algorithm = BINLOG_CHECKSUM_ALG_UNDEF;
            self.event_type_header_lengths = data[rdr.position() as usize..].to_vec()
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct RotateEvent {
    pub position: u64,
    pub next_log_name: Vec<u8>,
}

impl Event for RotateEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Position: {position}\n", position = self.position)?;
        write!(
            writer,
            "Next log name: {next_log_name}\n",
            next_log_name = String::from_utf8_lossy(&self.next_log_name)
        )?;

        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.position = rdr.read_u64::<LittleEndian>()?;
        self.next_log_name = data[rdr.position() as usize..].to_vec();

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreviousGTIDsEvent {
    pub gtid_sets: String,
}

impl Event for PreviousGTIDsEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(
            writer,
            "Previous GTID Event: {gtid_sets}\n",
            gtid_sets = &self.gtid_sets
        )?;

        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        let uuid_count = rdr.read_u16::<LittleEndian>()?;
        rdr.seek(SeekFrom::Current(6))?;

        let mut previous_gtid_sets = Vec::<String>::new();
        for _ in 0..uuid_count {
            let uuid =
                self.decode_uuid(&data[rdr.position() as usize..rdr.position() as usize + 16]);
            rdr.seek(SeekFrom::Current(16))?;

            let slice_count = rdr.read_u16::<LittleEndian>()?;
            rdr.seek(SeekFrom::Current(6))?;

            let mut intervals = Vec::<String>::new();
            for _ in 0..slice_count {
                let start = self
                    .decode_interval(&data[rdr.position() as usize..rdr.position() as usize + 8])?;
                rdr.seek(SeekFrom::Current(8))?;

                let stop = self
                    .decode_interval(&data[rdr.position() as usize..rdr.position() as usize + 8])?;
                rdr.seek(SeekFrom::Current(8))?;

                let interval = if stop == start + 1 {
                    format!("{}", start)
                } else {
                    format!("{}-{}", start, stop - 1)
                };

                intervals.push(interval)
            }

            previous_gtid_sets.push(format!("{}:{}", uuid, intervals.join(":")));
        }

        Ok(())
    }
}

impl PreviousGTIDsEvent {
    fn decode_uuid(&self, data: &[u8]) -> String {
        format!(
            "{}-{}-{}-{}-{}",
            hex::encode(&data[0..4]),
            hex::encode(&data[4..6]),
            hex::encode(&data[6..8]),
            hex::encode(&data[8..10]),
            hex::encode(&data[10..]),
        )
    }

    fn decode_interval(&self, data: &[u8]) -> Result<u64, ReplicationError> {
        let mut rdr = Cursor::new(data);
        return Ok(rdr.read_u64::<LittleEndian>()?);
    }
}

#[derive(Debug, Default, Clone)]
pub struct XIDEvent {
    pub xid: u64,

    // in fact XIDEvent dosen't have the GTIDSet information, just for beneficial to use
    pub gset: Option<GtidSetEnum>,
}

impl Event for XIDEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "XID: {xid}\n", xid = self.xid)?;

        if let Some(g) = self.gset.as_ref() {
            write!(writer, "GTIDSet: {g}\n", g = g.to_string())?;
        }

        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.xid = rdr.read_u64::<LittleEndian>()?;

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueryEvent {
    pub slave_proxy_id: u32,
    pub execution_time: u32,
    pub error_code: u16,
    pub status_vars: Vec<u8>,
    pub schema: Vec<u8>,
    pub query: Vec<u8>,

    // for mariadb QUERY_COMPRESSED_EVENT
    pub compressed: bool,

    // in fact QueryEvent dosen't have the GTIDSet information, just for beneficial to use
    pub gset: Option<GtidSetEnum>,
}

impl Event for QueryEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Slave proxy ID: {}\n", self.slave_proxy_id)?;
        write!(writer, "Execution time: {}\n", self.execution_time)?;
        write!(writer, "Error code: {}\n", self.error_code)?;
        write!(writer, "Status vars: \n{}", hex::encode(&self.status_vars))?;
        write!(
            writer,
            "Schema: {}\n",
            String::from_utf8_lossy(&self.schema)
        )?;
        write!(writer, "Query: {}\n", String::from_utf8_lossy(&self.query))?;

        if let Some(g) = self.gset.as_ref() {
            write!(writer, "GTIDSet: {g}\n", g = g.to_string())?;
        } else {
            writeln!(writer)?;
        }

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.slave_proxy_id = rdr.read_u32::<LittleEndian>()?;
        self.execution_time = rdr.read_u32::<LittleEndian>()?;
        let schema_length = rdr.read_u8()?;
        self.error_code = rdr.read_u16::<LittleEndian>()?;

        let status_vars_length = rdr.read_u16::<LittleEndian>()?;

        let status_vars_start = rdr.position() as usize;
        let status_vars_stop = status_vars_start + status_vars_length as usize;
        self.status_vars = data[status_vars_start..status_vars_stop].to_vec();
        rdr.seek(SeekFrom::Current(status_vars_length as i64))?;

        let schema_start = rdr.position() as usize;
        let schema_stop = schema_start + schema_length as usize;
        self.schema = data[schema_start..schema_stop].to_vec();
        rdr.seek(SeekFrom::Current(schema_length as i64))?;

        //skip 0x00
        rdr.seek(SeekFrom::Current(1))?;

        if self.compressed {
            self.query = decompress_mariadb_data(&data[rdr.position() as usize..])?;
        } else {
            self.query = data[rdr.position() as usize..].to_vec();
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct GTIDEvent {
    pub commit_flag: u8,
    pub sid: Vec<u8>,
    pub gno: i64,
    pub last_committed: i64,
    pub sequence_number: i64,

    // ImmediateCommitTimestamp/OriginalCommitTimestamp are introduced in MySQL-8.0.1, see:
    // https://mysqlhighavailability.com/replication-features-in-mysql-8-0-1/
    pub immediate_commit_timestamp: u64,
    pub original_commit_timestamp: u64,

    // Total transaction length (including this GTIDEvent), introduced in MySQL-8.0.2, see:
    // https://mysqlhighavailability.com/taking-advantage-of-new-transaction-length-metadata/
    pub transaction_length: u64,

    // ImmediateServerVersion/OriginalServerVersion are introduced in MySQL-8.0.14, see
    // https://dev.mysql.com/doc/refman/8.0/en/replication-compatibility.html
    pub immediate_server_version: u32,
    pub original_server_version: u32,
}

impl Event for GTIDEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Commit flag: {}\n", self.commit_flag)?;
        let u = Uuid::from_slice(&self.sid)?;
        write!(writer, "GTID_NEXT: {}:{}\n", u.to_string(), self.gno)?;
        write!(writer, "LAST_COMMITTED: {}\n", self.last_committed)?;
        write!(writer, "SEQUENCE_NUMBER: {}\n", self.sequence_number)?;
        write!(
            writer,
            "Immediate cmmmit timestamp: {} ({})\n",
            self.immediate_commit_timestamp,
            GTIDEvent::fmt_time(&self.immediate_commit_time())
        )?;
        write!(
            writer,
            "Orignal commit timestamp: {} ({})\n",
            self.original_commit_timestamp,
            GTIDEvent::fmt_time(&self.original_commit_time())
        )?;
        write!(writer, "Transaction length: {}\n", self.transaction_length)?;
        write!(
            writer,
            "Immediate server version: {}\n",
            self.immediate_server_version
        )?;
        write!(
            writer,
            "Orignal server version: {}\n",
            self.original_server_version
        )?;

        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.commit_flag = rdr.read_u8()?;

        let sid_start = rdr.position() as usize;
        let sid_stop = rdr.position() as usize + SID_LENGTH;
        self.sid = data[sid_start..sid_stop].to_vec();
        rdr.seek(SeekFrom::Current(SID_LENGTH as i64))?;

        self.gno = rdr.read_i64::<LittleEndian>()?;

        if data.len() >= 42 {
            if data[rdr.position() as usize] == LOGICAL_TIMESTAMP_TYPE_CODE as u8 {
                rdr.seek(SeekFrom::Current(1))?;

                self.last_committed = rdr.read_i64::<LittleEndian>()?;
                self.sequence_number = rdr.read_i64::<LittleEndian>()?;

                // IMMEDIATE_COMMIT_TIMESTAMP_LENGTH = 7
                if (data.len() - rdr.position() as usize) < 7 {
                    return Ok(());
                }

                let start = rdr.position() as usize;
                let stop = start + 7;
                self.immediate_commit_timestamp = fixed_length_int(&data[start..stop]);
                rdr.seek(SeekFrom::Current(7))?;
                if (self.immediate_commit_timestamp & (1_u64 << 55)) != 0 {
                    // If the most significant bit set, another 7 byte follows representing OriginalCommitTimestamp
                    self.immediate_commit_timestamp &= !(1_u64 << 55);
                    let start = rdr.position() as usize;
                    let stop = start + 7;
                    self.original_commit_timestamp = fixed_length_int(&data[start..stop]);
                    rdr.seek(SeekFrom::Current(7))?;
                } else {
                    // Otherwise OriginalCommitTimestamp == ImmediateCommitTimestamp
                    self.original_commit_timestamp = self.immediate_commit_timestamp;
                }

                // TRANSACTION_LENGTH_MIN_LENGTH = 1
                if (data.len() - rdr.position() as usize) < 1 {
                    return Ok(());
                }
                let (transaction_length, _, n) =
                    length_encoded_int(&data[rdr.position() as usize..]);
                self.transaction_length = transaction_length;
                rdr.seek(SeekFrom::Current(n as i64))?;

                // IMMEDIATE_SERVER_VERSION_LENGTH = 4
                self.immediate_server_version = UNDE_FINED_SERVER_VER as u32;
                self.original_server_version = UNDE_FINED_SERVER_VER as u32;
                if (data.len() - rdr.position() as usize) < 4 {
                    return Ok(());
                }
                self.immediate_server_version = rdr.read_u32::<LittleEndian>()?;
                if (self.immediate_server_version & (1_u32 << 31)) != 0 {
                    // If the most significant bit set, another 4 byte follows representing OriginalServerVersion
                    self.immediate_server_version &= !(1_u32 << 31);
                    self.original_server_version = rdr.read_u32::<LittleEndian>()?;
                } else {
                    // Otherwise OriginalServerVersion == ImmediateServerVersion
                    self.original_server_version = self.immediate_server_version;
                }
            }
        }

        Ok(())
    }
}

impl GTIDEvent {
    fn fmt_time(datetime: &NaiveDateTime) -> String {
        if datetime.timestamp() == 0 && datetime.timestamp_subsec_nanos() == 0 {
            return String::from("<n/a>");
        }

        // RFC3339Nano
        return datetime.format("%Y-%m-%dT%H:%M:%S%.9fZ%:z").to_string();
    }

    // ImmediateCommitTime returns the commit time of this trx on the immediate server
    // or zero time if not available.
    pub fn immediate_commit_time(&self) -> NaiveDateTime {
        return micro_sec_timestamp_to_time(self.immediate_commit_timestamp);
    }

    // OriginalCommitTime returns the commit time of this trx on the original server
    // or zero time if not available.
    pub fn original_commit_time(&self) -> NaiveDateTime {
        return micro_sec_timestamp_to_time(self.original_commit_timestamp);
    }
}

#[derive(Debug, Default, Clone)]
pub struct BeginLoadQueryEvent {
    pub filed_id: u32,
    pub block_data: Vec<u8>,
}

impl Event for BeginLoadQueryEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "File ID: {}\n", self.filed_id)?;
        write!(
            writer,
            "Block data: {}\n",
            String::from_utf8_lossy(&self.block_data)
        )?;
        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.filed_id = rdr.read_u32::<LittleEndian>()?;
        self.block_data = data[rdr.position() as usize..].to_vec();

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExecuteLoadQueryEvent {
    pub slave_proxy_id: u32,
    pub execution_time: u32,
    pub schema_length: u8,
    pub error_code: u16,
    pub status_vars: u16,
    pub file_id: u32,
    pub start_pos: u32,
    pub end_pos: u32,
    pub dup_handling_flags: u8,
}

impl Event for ExecuteLoadQueryEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Slave proxy ID: {}\n", self.slave_proxy_id)?;
        write!(writer, "Execution time: {}\n", self.execution_time)?;
        write!(writer, "Schame length: {}\n", self.schema_length)?;
        write!(writer, "Error code: {}\n", self.error_code)?;
        write!(writer, "Status vars length: {}\n", self.status_vars)?;
        write!(writer, "File ID: {}\n", self.file_id)?;
        write!(writer, "Start pos: {}\n", self.start_pos)?;
        write!(writer, "End pos: {}\n", self.end_pos)?;
        write!(writer, "Dup handling flags: {}\n", self.dup_handling_flags)?;

        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.slave_proxy_id = rdr.read_u32::<LittleEndian>()?;
        self.execution_time = rdr.read_u32::<LittleEndian>()?;
        self.schema_length = rdr.read_u8()?;
        self.error_code = rdr.read_u16::<LittleEndian>()?;
        self.status_vars = rdr.read_u16::<LittleEndian>()?;
        self.file_id = rdr.read_u32::<LittleEndian>()?;
        self.start_pos = rdr.read_u32::<LittleEndian>()?;
        self.end_pos = rdr.read_u32::<LittleEndian>()?;
        self.dup_handling_flags = rdr.read_u8()?;

        Ok(())
    }
}

// case MARIADB_ANNOTATE_ROWS_EVENT:
// 	return "MariadbAnnotateRowsEvent"
#[derive(Debug, Default, Clone)]
pub struct MariadbAnnotateRowsEvent {
    pub query: Vec<u8>,
}

impl Event for MariadbAnnotateRowsEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Query: {}\n", String::from_utf8_lossy(&self.query))?;
        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        self.query = data.to_vec();

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct MariadbBinlogCheckPointEvent {
    pub info: Vec<u8>,
}

impl Event for MariadbBinlogCheckPointEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Info: {}\n", String::from_utf8_lossy(&self.info))?;
        writeln!(writer)?;
        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        self.info = data.to_vec();

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct MariadbGTIDEvent {
    pub gtid: MariadbGTID,
    pub flags: u8,
    pub commit_id: u64,
}

impl MariadbGTIDEvent {
    pub fn is_ddl(&self) -> bool {
        (self.flags as i64 & BINLOG_MARIADB_FL_DDL) != 0
    }

    pub fn is_standalone(&self) -> bool {
        (self.flags as i64 & BINLOG_MARIADB_FL_STANDALONE) != 0
    }

    pub fn is_group_commit(&self) -> bool {
        (self.flags as i64 & BINLOG_MARIADB_FL_GROUP_COMMIT_ID) != 0
    }
}

impl Event for MariadbGTIDEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "GTID: {}\n", self.gtid.to_string())?;
        write!(writer, "Flags: {}\n", self.flags)?;
        write!(writer, "CommitID: {}\n", self.commit_id)?;

        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.gtid.sequence_number = rdr.read_u64::<LittleEndian>()?;
        self.gtid.domain_id = rdr.read_u32::<LittleEndian>()?;
        self.flags = rdr.read_u8()?;

        if (self.flags as i64 & BINLOG_MARIADB_FL_GROUP_COMMIT_ID) > 0 {
            self.commit_id = rdr.read_u64::<LittleEndian>()?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct MariadbGTIDListEvent {
    pub gtids: Vec<MariadbGTID>,
}

impl Event for MariadbGTIDListEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(
            writer,
            "{}",
            self.gtids
                .iter()
                .map(|gtid| gtid.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )?;
        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        let v = rdr.read_u32::<LittleEndian>()?;
        let count = v & ((1_u32 << 28) - 1);

        for _ in 0..count {
            let mut gtid = MariadbGTID {
                domain_id: 0,
                server_id: 0,
                sequence_number: 0,
            };
            gtid.domain_id = rdr.read_u32::<LittleEndian>()?;
            gtid.server_id = rdr.read_u32::<LittleEndian>()?;
            gtid.sequence_number = rdr.read_u64::<LittleEndian>()?;
            self.gtids.push(gtid);
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntVarEvent {
    pub type_i: IntVarEventType,
    pub value: u64,
}

impl Event for IntVarEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Type: {}\n", self.type_i.get_int())?;
        write!(writer, "Value: {}\n", self.value)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.type_i = IntVarEventType::from(rdr.read_u8()?);
        self.value = rdr.read_u64::<LittleEndian>()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::{Cursor, Seek, SeekFrom, Write};

    #[test]
    fn f() {
        for i in 0..7 {
            println!("{}", i)
        }
    }

    #[test]
    fn e() {
        let v = &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("{:?}", v[0..4].to_vec());
        println!("{:?}", &v[4..6]);

        ee(&v[2..8]);
        ee(v);
    }

    fn ee(v: &[i32]) {
        println!("{:?}", &v[0..4]);
        println!("{:?}", &v[4..6]);
    }

    #[test]
    fn decode_interval_test() {
        let v = &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        decode_interval(&v[2..]);
        println!("{:?}", v)
    }

    fn decode_interval(data: &[u8]) {
        println!("{:?}", data);
        let mut rdr = Cursor::new(data);
        println!("{}", rdr.read_u64::<BigEndian>().unwrap())
    }

    #[test]
    fn d() {
        let v = &vec![2, 5, 3, 0, 9, 8, 7];
        let mut rdr = Cursor::new(v);
        // Note that we use type parameters to indicate which kind of byte order
        // we want!
        rdr.seek(SeekFrom::Current(2)).unwrap();
        println!("seek 2 pos: {}", rdr.position());

        let b = v[rdr.position() as usize..rdr.position() as usize + 3].to_vec();
        println!("b: {:?}, pos: {}", &b, rdr.position());

        rdr.seek(SeekFrom::Current(3)).unwrap();
        println!("seek 2 pos: {}", rdr.position());

        let a = rdr.read_u8().unwrap();
        println!("a: {:?}, pos: {}", a, rdr.position());
    }

    #[test]
    fn c() {
        let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);

        assert_eq!(buff.position(), 0);

        buff.seek(SeekFrom::Current(2)).unwrap();
        assert_eq!(buff.position(), 2);

        buff.seek(SeekFrom::Current(-1)).unwrap();
        assert_eq!(buff.position(), 1);
    }

    #[test]
    fn b() {
        let mut buffer = Vec::new();

        // 将值写入到实现了 io::Write trait 的对象中
        write!(&mut buffer, "Hello, {}", "World").unwrap();

        // 将 buffer 转换为字符串并打印
        let result = String::from_utf8_lossy(&buffer);
        println!("{}", result);

        let result = String::from_utf8_lossy(&buffer);
        println!("{}", result);
    }

    #[test]
    fn a() {
        let s = "abc123xyz";

        for c in s.chars() {
            if c.is_numeric() {
                println!("{} is a digit.", c);
            } else {
                println!("{} is not a digit.", c);
            }
        }
    }
}
