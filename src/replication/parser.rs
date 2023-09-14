use crate::error::{EventError, ReplicationError};
use crate::replication::{
    BeginLoadQueryEvent, BinlogEvent, Event, EventEnum, EventHeader, EventType,
    ExecuteLoadQueryEvent, FormatDescriptionEvent, GTIDEvent, GenericEvent, IntVarEvent,
    MariadbAnnotateRowsEvent, MariadbBinlogCheckPointEvent, MariadbGTIDEvent, MariadbGTIDListEvent,
    PreviousGTIDsEvent, QueryEvent, RotateEvent, RowsEvent, RowsQueryEvent, TableMapEvent,
    TransactionPayloadEvent, XIDEvent, BINLOG_CHECKSUM_ALG_CRC32, BINLOG_CHECKSUM_LENGTH,
    BINLOG_FILE_HEADER, ERR_MISSING_TABLE_MAP_EVENT, EVENT_HEADER_SIZE, ROWS_EVENT_STMT_END_FLAG,
};
use byteorder::{LittleEndian, WriteBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic;
use std::sync::atomic::Ordering;

// ErrChecksumMismatch indicates binlog checksum mismatch.
pub const ERR_CHECKSUM_MISMATCH: &str = "binlog checksum mismatch, data may be corrupted";

#[derive(Default)]
pub struct BinlogParse {
    // "mysql" or "mariadb", if not set, use "mysql" by default
    pub flavor: String,
    pub format: Option<FormatDescriptionEvent>,
    pub tables: HashMap<u64, TableMapEvent>,
    // for rawMode, we only parse FormatDescriptionEvent and RotateEvent
    pub raw_mode: bool,
    pub parse_time: bool,
    pub timestamp_string_location: Option<chrono_tz::Tz>,
    // used to start/stop processing
    pub stop_processing: atomic::AtomicU32,
    pub use_decimal: bool,
    pub ignore_json_decode_err: bool,
    pub verify_checksum: bool,
    pub rows_event_decode_func:
        Option<Box<dyn Fn(&mut RowsEvent, &[u8]) -> Result<(), ReplicationError>>>,
}

impl BinlogParse {
    pub fn new() -> BinlogParse {
        BinlogParse::default()
    }

    pub fn stop(&mut self) {
        self.stop_processing.store(1, Ordering::SeqCst);
    }

    pub fn resume(&mut self) {
        self.stop_processing.store(0, Ordering::SeqCst);
    }

    pub fn reset(&mut self) {
        self.format = None;
    }

    pub fn parse_file<F>(
        &mut self,
        name: &str,
        offset: i64,
        on_event: &F,
    ) -> Result<(), ReplicationError>
    where
        F: Fn(&BinlogEvent) -> Result<(), ReplicationError>,
    {
        let mut f = File::open(name)?;
        let mut b = vec![0_u8; 4];
        let _ = f.read(&mut b)?;
        if b != BINLOG_FILE_HEADER {
            return Err(ReplicationError::new(format!(
                "{} is not a valid binlog file, head 4 bytes must fe'bin' ",
                name
            )));
        }

        let mut offset = offset;
        if offset < 4 {
            offset = 4;
        } else if offset > 4 {
            //  FORMAT_DESCRIPTION event should be read by default always (despite that fact passed offset may be higher than 4)
            if let Err(e) = f.seek(SeekFrom::Start(4)) {
                return Err(ReplicationError::new(format!(
                    "seek {} to {} error {}",
                    name,
                    offset,
                    e.to_string()
                )));
            }

            if let Err(e) = self._e_format_description_event(&mut f, on_event) {
                return Err(ReplicationError::new(format!(
                    "{} parse FormatDescriptionEvent",
                    e.to_string()
                )));
            }
        }

        if let Err(e) = f.seek(SeekFrom::Start(offset as u64)) {
            return Err(ReplicationError::new(format!(
                "seek {} to {} error {}",
                name,
                offset,
                e.to_string()
            )));
        }

        self.parse_reader(&mut f, on_event)
    }

    fn _e_format_description_event<R, F>(
        &mut self,
        r: &mut R,
        on_event: &F,
    ) -> Result<(), ReplicationError>
    where
        R: Read,
        F: Fn(&BinlogEvent) -> Result<(), ReplicationError>,
    {
        let _ = self._parse_single_event(r, on_event)?;

        Ok(())
    }

    // ParseSingleEvent parses single binlog event and passes the event to onEvent function.
    pub fn parse_single_event<R, F>(
        &mut self,
        r: &mut R,
        on_event: &F,
    ) -> Result<bool, ReplicationError>
    where
        R: Read,
        F: Fn(&BinlogEvent) -> Result<(), ReplicationError>,
    {
        self._parse_single_event(r, on_event)
    }

    fn _parse_single_event<R, F>(
        &mut self,
        r: &mut R,
        on_event: &F,
    ) -> Result<bool, ReplicationError>
    where
        R: Read,
        F: Fn(&BinlogEvent) -> Result<(), ReplicationError>,
    {
        let mut raw_data: Vec<u8> = vec![];
        let mut buf = vec![0_u8; EVENT_HEADER_SIZE];

        // @todo: to avoid allocate/destroy buffers frequently
        match r.read_exact(&mut buf) {
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(true),
            Err(e) => {
                return Err(ReplicationError::new(format!(
                    "get event header err {}, need {}",
                    e.to_string(),
                    EVENT_HEADER_SIZE,
                )))
            }
            _ => {}
        };
        raw_data.extend(&buf);

        let h = self._parse_header(&buf)?;
        if h.event_size < EVENT_HEADER_SIZE as u32 {
            return Err(ReplicationError::new(format!(
                "invalid event header, event size is {}, too small",
                h.event_size
            )));
        }

        let raw_length = buf.len() + (h.event_size as usize - EVENT_HEADER_SIZE);
        if raw_length > buf.len() {
            buf.resize(h.event_size as usize - EVENT_HEADER_SIZE, 0);
            if let Err(e) = r.read_exact(&mut buf) {
                return Err(ReplicationError::new(format!(
                    "get event err {}, need {}",
                    e.to_string(),
                    h.event_size
                )));
            }
            raw_data.extend(&buf);
        }
        if raw_data.len() != h.event_size as usize {
            return Err(ReplicationError::new(format!(
                "invalid raw data size in event {}, need {}, but got {}",
                h.event_type.to_string(),
                h.event_size,
                raw_data.len()
            )));
        }

        let body_len = h.event_size as usize - EVENT_HEADER_SIZE;
        let body = &raw_data[EVENT_HEADER_SIZE..];
        if body.len() != body_len {
            return Err(ReplicationError::new(format!(
                "invalid body data size in event {}, need {}, but got {}",
                h.event_type.to_string(),
                body_len,
                body.len()
            )));
        }

        let e = match self._parse_event(&h, body, &raw_data) {
            Ok(v) => v,
            Err(e) => {
                if e.to_string() == ERR_MISSING_TABLE_MAP_EVENT {
                    return Ok(false);
                }
                return Err(e);
            }
        };

        on_event(&BinlogEvent {
            raw_data,
            header: Some(h),
            event: Some(e),
        })?;

        Ok(false)
    }

    pub fn parse_reader<R, F>(&mut self, r: &mut R, on_event: &F) -> Result<(), ReplicationError>
    where
        R: Read,
        F: Fn(&BinlogEvent) -> Result<(), ReplicationError>,
    {
        loop {
            if self.stop_processing.load(Ordering::SeqCst) == 1 {
                break;
            }

            let done = match self._parse_single_event(r, on_event) {
                Ok(v) => v,
                Err(e) => {
                    if e.to_string() == ERR_MISSING_TABLE_MAP_EVENT {
                        continue;
                    }
                    return Err(e);
                }
            };

            if done {
                break;
            }
        }
        Ok(())
    }

    pub fn set_raw_mode(&mut self, mode: bool) {
        self.raw_mode = mode;
    }

    pub fn set_parse_time(&mut self, parse_time: bool) {
        self.parse_time = parse_time;
    }

    pub fn set_timestamp_string_location(
        &mut self,
        timestamp_string_location: Option<chrono_tz::Tz>,
    ) {
        self.timestamp_string_location = timestamp_string_location;
    }

    pub fn set_use_decimal(&mut self, use_decimal: bool) {
        self.use_decimal = use_decimal;
    }

    pub fn set_ignore_json_decode_error(&mut self, ignore_json_decode_err: bool) {
        self.ignore_json_decode_err = ignore_json_decode_err;
    }

    pub fn set_verify_checksum(&mut self, verify: bool) {
        self.verify_checksum = verify;
    }

    pub fn set_flavor(&mut self, flavor: String) {
        self.flavor = flavor;
    }

    pub fn set_rows_event_decode_func(
        &mut self,
        rows_event_decode_func: Option<
            Box<dyn Fn(&mut RowsEvent, &[u8]) -> Result<(), ReplicationError>>,
        >,
    ) {
        self.rows_event_decode_func = rows_event_decode_func;
    }

    fn _parse_header(&self, data: &[u8]) -> Result<EventHeader, ReplicationError> {
        let mut h = EventHeader::default();
        h.decode(data)?;

        Ok(h)
    }

    fn _parse_event(
        &mut self,
        h: &EventHeader,
        data: &[u8],
        row_data: &[u8],
    ) -> Result<EventEnum, ReplicationError> {
        let mut data = data;

        let mut e = if h.event_type == EventType::FormatDescriptionEvent {
            EventEnum::FormatDescriptionEvent(FormatDescriptionEvent::default())
        } else {
            if let Some(format) = &self.format {
                if format.check_sum_algorithm == BINLOG_CHECKSUM_ALG_CRC32 {
                    self._verify_crc32_checksum(row_data)?;
                    data = &data[..(data.len() - BINLOG_CHECKSUM_LENGTH)];
                }
            }

            if h.event_type == EventType::RotateEvent {
                EventEnum::RotateEvent(RotateEvent::default())
            } else if !self.raw_mode {
                match h.event_type {
                    EventType::QueryEvent => EventEnum::QueryEvent(QueryEvent::default()),
                    EventType::MariadbQueryCompressedEvent => {
                        let mut ev = QueryEvent::default();
                        ev.compressed = true;
                        EventEnum::QueryEvent(ev)
                    }
                    EventType::XidEvent => EventEnum::XIDEvent(XIDEvent::default()),
                    EventType::TableMapEvent => {
                        let mut ev = TableMapEvent::default();
                        ev.flavor = self.flavor.clone();
                        if self.format.as_ref().unwrap().event_type_header_lengths
                            [EventType::TableMapEvent as usize - 1]
                            == 6
                        {
                            ev.table_id_size = 4;
                        } else {
                            ev.table_id_size = 6;
                        }

                        EventEnum::TableMapEvent(ev)
                    }
                    EventType::WriteRowsEventv0
                    | EventType::UpdateRowsEventv0
                    | EventType::DeleteRowsEventv0
                    | EventType::WriteRowsEventv1
                    | EventType::UpdateRowsEventv1
                    | EventType::DeleteRowsEventv1
                    | EventType::WriteRowsEventv2
                    | EventType::UpdateRowsEventv2
                    | EventType::DeleteRowsEventv2
                    | EventType::MariadbWriteRowsCompressedEventV1
                    | EventType::MariadbUpdateRowsCompressedEventV1
                    | EventType::MariadbDeleteRowsCompressedEventV1
                    | EventType::PartialUpdateRowsEvent => {
                        // Extension of UPDATE_ROWS_EVENT, allowing partial values according to binlog_row_value_options
                        EventEnum::RowsEvent(self._new_rows_event(h))
                    }
                    EventType::RowsQueryEvent => {
                        EventEnum::RowsQueryEvent(RowsQueryEvent::default())
                    }
                    EventType::GtidEvent => EventEnum::GTIDEvent(GTIDEvent::default()),
                    EventType::AnonymousGtidEvent => EventEnum::GTIDEvent(GTIDEvent::default()),
                    EventType::BeginLoadQueryEvent => {
                        EventEnum::BeginLoadQueryEvent(BeginLoadQueryEvent::default())
                    }
                    EventType::ExecuteLoadQueryEvent => {
                        EventEnum::ExecuteLoadQueryEvent(ExecuteLoadQueryEvent::default())
                    }
                    EventType::MariadbAnnotateRowsEvent => {
                        EventEnum::MariadbAnnotateRowsEvent(MariadbAnnotateRowsEvent::default())
                    }
                    EventType::MariadbBinlogCheckpointEvent => {
                        EventEnum::MariadbBinlogCheckPointEvent(
                            MariadbBinlogCheckPointEvent::default(),
                        )
                    }
                    EventType::MariadbGtidListEvent => {
                        EventEnum::MariadbGTIDListEvent(MariadbGTIDListEvent::default())
                    }
                    EventType::MariadbGtidEvent => {
                        let mut ev = MariadbGTIDEvent::default();
                        ev.gtid.server_id = h.server_id;
                        EventEnum::MariadbGTIDEvent(ev)
                    }
                    EventType::PreviousGtidsEvent => {
                        EventEnum::PreviousGTIDsEvent(PreviousGTIDsEvent::default())
                    }
                    EventType::IntvarEvent => EventEnum::IntVarEvent(IntVarEvent::default()),
                    EventType::TransactionPayloadEvent => {
                        EventEnum::TransactionPayloadEvent(self._new_transaction_payload_event())
                    }
                    _ => EventEnum::GenericEvent(GenericEvent::default()),
                }
            } else {
                EventEnum::GenericEvent(GenericEvent::default())
            }
        };

        let rs = if let EventEnum::RowsEvent(ref mut re) = e {
            if self.rows_event_decode_func.is_some() {
                self.rows_event_decode_func.as_ref().unwrap()(re, &data)
            } else {
                e.decode(&data)
            }
        } else {
            e.decode(&data)
        };
        if let EventEnum::FormatDescriptionEvent(fde) = &e {
            self.format = Some(fde.clone());
        }

        if let Err(err) = rs {
            return Err(ReplicationError::EventError(EventError {
                header: h.clone(),
                err: err.to_string(),
                data: data.to_vec(),
            }));
        }

        if let EventEnum::TableMapEvent(ref te) = e {
            self.tables.insert(te.table_id, te.clone());
        }

        if let EventEnum::RowsEvent(ref re) = e {
            if (re.flags & ROWS_EVENT_STMT_END_FLAG as u16) > 0 {
                // Refer https://github.com/alibaba/canal/blob/38cc81b7dab29b51371096fb6763ca3a8432ffee/dbsync/src/main/java/com/taobao/tddl/dbsync/binlog/event/RowsLogEvent.java#L176
                self.tables = HashMap::<u64, TableMapEvent>::new();
            }
        }

        Ok(e)
    }

    // Parse: Given the bytes for a a binary log event: return the decoded event.
    // With the exception of the FORMAT_DESCRIPTION_EVENT event type
    // there must have previously been passed a FORMAT_DESCRIPTION_EVENT
    // into the parser for this to work properly on any given event.
    // Passing a new FORMAT_DESCRIPTION_EVENT into the parser will replace
    // an existing one.
    pub fn parse(&mut self, data: &[u8]) -> Result<BinlogEvent, ReplicationError> {
        let raw_data = data;
        let h = self._parse_header(data)?;
        let data = &data[EVENT_HEADER_SIZE..];
        let event_len = h.event_size as isize - EVENT_HEADER_SIZE as isize;

        if data.len() as isize != event_len {
            return Err(ReplicationError::new(format!(
                "invalid data size {} in event {}, less event length {}",
                data.len(),
                h.event_type.to_string(),
                event_len,
            )));
        }

        let e = self._parse_event(&h, data, &raw_data)?;

        return Ok(BinlogEvent {
            raw_data: raw_data.to_vec(),
            header: Some(h),
            event: Some(e),
        });
    }

    fn _verify_crc32_checksum(&self, raw_data: &[u8]) -> Result<(), ReplicationError> {
        if !self.verify_checksum {
            return Ok(());
        }

        let stop = raw_data.len() - BINLOG_CHECKSUM_LENGTH;
        let calculated_part = &raw_data[0..stop];
        let expected_checksum = &raw_data[stop..];

        // mysql use zlib's CRC32 implementation, which uses polynomial 0xedb88320UL.
        // reference: https://github.com/madler/zlib/blob/master/crc32.c
        // https://github.com/madler/zlib/blob/master/doc/rfc1952.txt#L419
        let checksum = crc32fast::hash(calculated_part);
        let mut computed = Vec::<u8>::with_capacity(BINLOG_CHECKSUM_LENGTH);
        computed.write_u32::<LittleEndian>(checksum)?;
        if expected_checksum != computed {
            return Err(ReplicationError::new(ERR_CHECKSUM_MISMATCH.to_string()));
        }

        Ok(())
    }

    fn _new_rows_event(&self, h: &EventHeader) -> RowsEvent {
        let mut e = RowsEvent::default();
        let post_header_len = self.format.as_ref().unwrap().event_type_header_lengths
            [(h.event_type.clone() as usize) - 1];
        if post_header_len == 6 {
            e.table_id_size = 4
        } else {
            e.table_id_size = 6
        }

        e.need_bitmap2 = false;
        e.tables = self.tables.clone();
        e.event_type = h.event_type.clone();
        e.parse_time = self.parse_time;
        e.timestamp_string_location = self.timestamp_string_location.clone();
        e.use_decimal = self.use_decimal;
        e.ignore_json_decode_err = self.ignore_json_decode_err;

        match h.event_type {
            EventType::WriteRowsEventv0 => e.version = 0,
            EventType::UpdateRowsEventv0 => e.version = 0,
            EventType::DeleteRowsEventv0 => e.version = 0,
            EventType::WriteRowsEventv1 => e.version = 1,
            EventType::UpdateRowsEventv1 => {
                e.version = 1;
                e.need_bitmap2 = true;
            }
            EventType::DeleteRowsEventv1 => e.version = 1,
            EventType::WriteRowsEventv2 => e.version = 2,
            EventType::UpdateRowsEventv2 => {
                e.version = 2;
                e.need_bitmap2 = true;
            }
            EventType::DeleteRowsEventv2 => e.version = 2,
            EventType::MariadbWriteRowsCompressedEventV1 => {
                e.version = 1;
                e.compressed = true;
            }
            EventType::MariadbUpdateRowsCompressedEventV1 => {
                e.version = 1;
                e.compressed = true;
                e.need_bitmap2 = true;
            }
            EventType::MariadbDeleteRowsCompressedEventV1 => {
                e.version = 1;
                e.compressed = true;
            }
            EventType::PartialUpdateRowsEvent => {
                e.version = 2;
                e.need_bitmap2 = true;
            }
            _ => {}
        }

        e
    }

    pub fn _new_transaction_payload_event(&self) -> TransactionPayloadEvent {
        let mut e = TransactionPayloadEvent::default();
        e.format = self.format.clone();

        e
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use crate::replication::BINLOG_FILE_HEADER;
    use byteorder::{LittleEndian, WriteBytesExt};
    #[test]
    fn b() -> Result<(), ReplicationError> {
        let mut buf = vec![0_u8; 10];
        println!("{:?}", &buf);
        buf.write_u32::<LittleEndian>(1)?;
        println!("{:?}", &buf);

        buf.write_u32::<LittleEndian>(2)?;
        println!("{:?}", &buf);

        println!("----------------------------------");
        let mut buf = Vec::with_capacity(10);
        println!("{:?}", &buf);
        buf.write_u32::<LittleEndian>(1)?;
        println!("{:?}", &buf);

        buf.write_u32::<LittleEndian>(2)?;
        println!("{:?}", &buf);

        Ok(())
    }

    #[test]
    fn a() {
        let v = vec![0xfe, 0x62, 0x69, 0x6e];
        assert_eq!(v, BINLOG_FILE_HEADER);
    }
}
