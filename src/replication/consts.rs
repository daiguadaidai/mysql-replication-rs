use std::fmt::{Display, Formatter};

// we only support MySQL 5.0.0+ binlog format, maybe???
pub const MIN_BINLOG_VERSION: u8 = 4;

// binlog header [ fe `bin` ]
pub const BINLOG_FILE_HEADER: [u8; 4] = [0xfe, 0x62, 0x69, 0x6e];

pub const SEMI_SYNC_INDICATOR: u8 = 0xef;

pub const LOG_EVENT_BINLOG_IN_USE_F: u16 = 0x0001;
pub const LOG_EVENT_FORCED_ROTATE_F: u16 = 0x0002;
pub const LOG_EVENT_THREAD_SPECIFIC_F: u16 = 0x0004;
pub const LOG_EVENT_SUPPRESS_USE_F: u16 = 0x0008;
pub const LOG_EVENT_UPDATE_TABLE_MAP_VERSION_F: u16 = 0x0010;
pub const LOG_EVENT_ARTIFICIAL_F: u16 = 0x0020;
pub const LOG_EVENT_RELAY_LOG_F: u16 = 0x0040;
pub const LOG_EVENT_IGNORABLE_F: u16 = 0x0080;
pub const LOG_EVENT_NO_FILTER_F: u16 = 0x0100;
pub const LOG_EVENT_MTS_ISOLATE_F: u16 = 0x0200;

pub const BINLOG_DUMP_NEVER_STOP: u16 = 0x00;
pub const BINLOG_DUMP_NON_BLOCK: u16 = 0x01;
pub const BINLOG_SEND_ANNOTATE_ROWS_EVENT: u16 = 0x02;
pub const BINLOG_THROUGH_POSITION: u16 = 0x02;
pub const BINLOG_THROUGH_GTID: u16 = 0x04;

pub const BINLOG_ROW_IMAGE_FULL: &str = "FULL";
pub const BINLOG_ROW_IMAGE_MINIMAL: &str = "MINIMAL";
pub const BINLOG_ROW_IMAGE_NOBLOB: &str = "NOBLOB";

pub const BINLOG_MARIADB_FL_STANDALONE: i64 = 1; /*1  - FL_STANDALONE is set when there is no terminating COMMIT event*/
pub const BINLOG_MARIADB_FL_GROUP_COMMIT_ID: i64 = BINLOG_MARIADB_FL_STANDALONE << 2; /*2  - FL_GROUP_COMMIT_ID is set when event group is part of a group commit on the master. Groups with same commit_id are part of the same group commit.*/
pub const BINLOG_MARIADB_FL_TRANSACTIONAL: i64 = BINLOG_MARIADB_FL_STANDALONE << 3; /*4  - FL_TRANSACTIONAL is set for an event group that can be safely rolled back (no MyISAM, eg.).*/
pub const BINLOG_MARIADB_FL_ALLOW_PARALLEL: i64 = BINLOG_MARIADB_FL_STANDALONE << 4; /*8  - FL_ALLOW_PARALLEL reflects the (negation of the) value of @@SESSION.skip_parallel_replication at the time of commit*/
pub const BINLOG_MARIADB_FL_WAITED: i64 = BINLOG_MARIADB_FL_STANDALONE << 5; /*16 = FL_WAITED is set if a row lock wait (or other wait) is detected during the execution of the transaction*/
pub const BINLOG_MARIADB_FL_DDL: i64 = BINLOG_MARIADB_FL_STANDALONE << 6; /*32 - FL_DDL is set for event group containing DDL*/

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    UnknownEvent = 0,
    StartEventV3 = 1,
    QueryEvent = 2,
    StopEvent = 3,
    RotateEvent = 4,
    IntvarEvent = 5,
    LoadEvent = 6,
    SlaveEvent = 7,
    CreateFileEvent = 8,
    AppendBlockEvent = 9,
    ExecLoadEvent = 10,
    DeleteFileEvent = 11,
    NewLoadEvent = 12,
    RandEvent = 13,
    UserVarEvent = 14,
    FormatDescriptionEvent = 15,
    XidEvent = 16,
    BeginLoadQueryEvent = 17,
    ExecuteLoadQueryEvent = 18,
    TableMapEvent = 19,
    WriteRowsEventv0 = 20,
    UpdateRowsEventv0 = 21,
    DeleteRowsEventv0 = 22,
    WriteRowsEventv1 = 23,
    UpdateRowsEventv1 = 24,
    DeleteRowsEventv1 = 25,
    IncidentEvent = 26,
    HeartbeatEvent = 27,
    IgnorableEvent = 28,
    RowsQueryEvent = 29,
    WriteRowsEventv2 = 30,
    UpdateRowsEventv2 = 31,
    DeleteRowsEventv2 = 32,
    GtidEvent = 33,
    AnonymousGtidEvent = 34,
    PreviousGtidsEvent = 35,
    TransactionContextEvent = 36,
    ViewChangeEvent = 37,
    XaPrepareLogEvent = 38,
    PartialUpdateRowsEvent = 39,
    TransactionPayloadEvent = 40,
    HeartbeatLogEventV2 = 41,

    // MariaDB event starts from 160
    MariadbAnnotateRowsEvent = 160,
    MariadbBinlogCheckpointEvent = 161,
    MariadbGtidEvent = 162,
    MariadbGtidListEvent = 163,
    MariadbStartEncryptionEvent = 164,
    MariadbQueryCompressedEvent = 165,
    MariadbWriteRowsCompressedEventV1 = 166,
    MariadbUpdateRowsCompressedEventV1 = 167,
    MariadbDeleteRowsCompressedEventV1 = 168,
}

impl Default for EventType {
    fn default() -> Self {
        EventType::UnknownEvent
    }
}

//将ParseIntError转为IdError::ParseError
impl From<u8> for EventType {
    fn from(data: u8) -> EventType {
        match data {
            0 => EventType::UnknownEvent,
            1 => EventType::StartEventV3,
            2 => EventType::QueryEvent,
            3 => EventType::StopEvent,
            4 => EventType::RotateEvent,
            5 => EventType::IntvarEvent,
            6 => EventType::LoadEvent,
            7 => EventType::SlaveEvent,
            8 => EventType::CreateFileEvent,
            9 => EventType::AppendBlockEvent,
            10 => EventType::ExecLoadEvent,
            11 => EventType::DeleteFileEvent,
            12 => EventType::NewLoadEvent,
            13 => EventType::RandEvent,
            14 => EventType::UserVarEvent,
            15 => EventType::FormatDescriptionEvent,
            16 => EventType::XidEvent,
            17 => EventType::BeginLoadQueryEvent,
            18 => EventType::ExecuteLoadQueryEvent,
            19 => EventType::TableMapEvent,
            20 => EventType::WriteRowsEventv0,
            21 => EventType::UpdateRowsEventv0,
            22 => EventType::DeleteRowsEventv0,
            23 => EventType::WriteRowsEventv1,
            24 => EventType::UpdateRowsEventv1,
            25 => EventType::DeleteRowsEventv1,
            26 => EventType::IncidentEvent,
            27 => EventType::HeartbeatEvent,
            28 => EventType::IgnorableEvent,
            29 => EventType::RowsQueryEvent,
            30 => EventType::WriteRowsEventv2,
            31 => EventType::UpdateRowsEventv2,
            32 => EventType::DeleteRowsEventv2,
            33 => EventType::GtidEvent,
            34 => EventType::AnonymousGtidEvent,
            35 => EventType::PreviousGtidsEvent,
            36 => EventType::TransactionContextEvent,
            37 => EventType::ViewChangeEvent,
            38 => EventType::XaPrepareLogEvent,
            39 => EventType::PartialUpdateRowsEvent,
            40 => EventType::TransactionPayloadEvent,
            41 => EventType::HeartbeatLogEventV2,
            160 => EventType::MariadbAnnotateRowsEvent,
            161 => EventType::MariadbBinlogCheckpointEvent,
            162 => EventType::MariadbGtidEvent,
            163 => EventType::MariadbGtidListEvent,
            164 => EventType::MariadbStartEncryptionEvent,
            165 => EventType::MariadbQueryCompressedEvent,
            166 => EventType::MariadbWriteRowsCompressedEventV1,
            167 => EventType::MariadbUpdateRowsCompressedEventV1,
            168 => EventType::MariadbDeleteRowsCompressedEventV1,
            _ => EventType::UnknownEvent,
        }
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EventType::UnknownEvent => write!(f, "{}", "UnknownEvent"),
            EventType::StartEventV3 => write!(f, "{}", "StartEventV3"),
            EventType::QueryEvent => write!(f, "{}", "QueryEvent"),
            EventType::StopEvent => write!(f, "{}", "StopEvent"),
            EventType::RotateEvent => write!(f, "{}", "RotateEvent"),
            EventType::IntvarEvent => write!(f, "{}", "IntvarEvent"),
            EventType::LoadEvent => write!(f, "{}", "LoadEvent"),
            EventType::SlaveEvent => write!(f, "{}", "SlaveEvent"),
            EventType::CreateFileEvent => write!(f, "{}", "CreateFileEvent"),
            EventType::AppendBlockEvent => write!(f, "{}", "AppendBlockEvent"),
            EventType::ExecLoadEvent => write!(f, "{}", "ExecLoadEvent"),
            EventType::DeleteFileEvent => write!(f, "{}", "DeleteFileEvent"),
            EventType::NewLoadEvent => write!(f, "{}", "NewLoadEvent"),
            EventType::RandEvent => write!(f, "{}", "RandEvent"),
            EventType::UserVarEvent => write!(f, "{}", "UserVarEvent"),
            EventType::FormatDescriptionEvent => write!(f, "{}", "FormatDescriptionEvent"),
            EventType::XidEvent => write!(f, "{}", "XidEvent"),
            EventType::BeginLoadQueryEvent => write!(f, "{}", "BeginLoadQueryEvent"),
            EventType::ExecuteLoadQueryEvent => write!(f, "{}", "ExecuteLoadQueryEvent"),
            EventType::TableMapEvent => write!(f, "{}", "TableMapEvent"),
            EventType::WriteRowsEventv0 => write!(f, "{}", "WriteRowsEventv0"),
            EventType::UpdateRowsEventv0 => write!(f, "{}", "UpdateRowsEventv0"),
            EventType::DeleteRowsEventv0 => write!(f, "{}", "DeleteRowsEventv0"),
            EventType::WriteRowsEventv1 => write!(f, "{}", "WriteRowsEventv1"),
            EventType::UpdateRowsEventv1 => write!(f, "{}", "UpdateRowsEventv1"),
            EventType::DeleteRowsEventv1 => write!(f, "{}", "DeleteRowsEventv1"),
            EventType::IncidentEvent => write!(f, "{}", "IncidentEvent"),
            EventType::HeartbeatEvent => write!(f, "{}", "HeartbeatEvent"),
            EventType::IgnorableEvent => write!(f, "{}", "IgnorableEvent"),
            EventType::RowsQueryEvent => write!(f, "{}", "RowsQueryEvent"),
            EventType::WriteRowsEventv2 => write!(f, "{}", "WriteRowsEventv2"),
            EventType::UpdateRowsEventv2 => write!(f, "{}", "UpdateRowsEventv2"),
            EventType::DeleteRowsEventv2 => write!(f, "{}", "DeleteRowsEventv2"),
            EventType::GtidEvent => write!(f, "{}", "GtidEvent"),
            EventType::AnonymousGtidEvent => write!(f, "{}", "AnonymousGtidEvent"),
            EventType::PreviousGtidsEvent => write!(f, "{}", "PreviousGtidsEvent"),
            EventType::TransactionContextEvent => write!(f, "{}", "TransactionContextEvent"),
            EventType::ViewChangeEvent => write!(f, "{}", "ViewChangeEvent"),
            EventType::XaPrepareLogEvent => write!(f, "{}", "XaPrepareLogEvent"),
            EventType::PartialUpdateRowsEvent => write!(f, "{}", "PartialUpdateRowsEvent"),
            EventType::TransactionPayloadEvent => write!(f, "{}", "TransactionPayloadEvent"),
            EventType::HeartbeatLogEventV2 => write!(f, "{}", "HeartbeatLogEventV2"),
            EventType::MariadbAnnotateRowsEvent => write!(f, "{}", "MariadbAnnotateRowsEvent"),
            EventType::MariadbBinlogCheckpointEvent => {
                write!(f, "{}", "MariadbBinlogCheckpointEvent")
            }
            EventType::MariadbGtidEvent => write!(f, "{}", "MariadbGtidEvent"),
            EventType::MariadbGtidListEvent => write!(f, "{}", "MariadbGtidListEvent"),
            EventType::MariadbStartEncryptionEvent => {
                write!(f, "{}", "MariadbStartEncryptionEvent")
            }
            EventType::MariadbQueryCompressedEvent => {
                write!(f, "{}", "MariadbQueryCompressedEvent")
            }
            EventType::MariadbWriteRowsCompressedEventV1 => {
                write!(f, "{}", "MariadbWriteRowsCompressedEventV1")
            }
            EventType::MariadbUpdateRowsCompressedEventV1 => {
                write!(f, "{}", "MariadbUpdateRowsCompressedEventV1")
            }
            EventType::MariadbDeleteRowsCompressedEventV1 => {
                write!(f, "{}", "MariadbDeleteRowsCompressedEventV1")
            }
        }
    }
}

pub const BINLOG_CHECKSUM_ALG_OFF: u8 = 0; // Events are without checksum though its generator
                                           // is checksum-capable New Master (NM).
pub const BINLOG_CHECKSUM_ALG_CRC32: u8 = 1; // CRC32 of zlib algorithm.
                                             //  BINLOG_CHECKSUM_ALG_ENUM_END,  // the cut line: valid alg range is [1, 0x7f].
pub const BINLOG_CHECKSUM_ALG_UNDEF: u8 = 255; // special value to tag undetermined yet checksum
                                               // or events from checksum-unaware servers

// These are TABLE_MAP_EVENT's optional metadata field type, from: libbinlogevents/include/rows_event.h
pub const TABLE_MAP_OPT_META_SIGNEDNESS: u8 = 1;
pub const TABLE_MAP_OPT_META_DEFAULT_CHARSET: u8 = 2;
pub const TABLE_MAP_OPT_META_COLUMN_CHARSET: u8 = 3;
pub const TABLE_MAP_OPT_META_COLUMN_NAME: u8 = 4;
pub const TABLE_MAP_OPT_META_SET_STR_VALUE: u8 = 5;
pub const TABLE_MAP_OPT_META_ENUM_STR_VALUE: u8 = 6;
pub const TABLE_MAP_OPT_META_GEOMETRY_TYPE: u8 = 7;
pub const TABLE_MAP_OPT_META_SIMPLE_PRIMARY_KEY: u8 = 8;
pub const TABLE_MAP_OPT_META_PRIMARY_KEY_WITH_PREFIX: u8 = 9;
pub const TABLE_MAP_OPT_META_ENUM_AND_SET_DEFAULT_CHARSET: u8 = 10;
pub const TABLE_MAP_OPT_META_ENUM_AND_SET_COLUMN_CHARSET: u8 = 11;

#[derive(Debug, Clone, PartialEq)]
pub enum IntVarEventType {
    Invalid = 0,
    LastInsertId = 1,
    InsertId = 2,
}

impl Default for IntVarEventType {
    fn default() -> Self {
        IntVarEventType::Invalid
    }
}

impl From<u8> for IntVarEventType {
    fn from(value: u8) -> Self {
        match value {
            0 => IntVarEventType::Invalid,
            1 => IntVarEventType::LastInsertId,
            2 => IntVarEventType::InsertId,
            _ => IntVarEventType::Invalid,
        }
    }
}

impl IntVarEventType {
    pub fn new(t: u8) -> IntVarEventType {
        match t {
            0 => IntVarEventType::Invalid,
            1 => IntVarEventType::LastInsertId,
            2 => IntVarEventType::InsertId,
            _ => IntVarEventType::Invalid,
        }
    }

    pub fn get_int(&self) -> isize {
        match self {
            IntVarEventType::Invalid => 0_isize,
            IntVarEventType::LastInsertId => 1_isize,
            IntVarEventType::InsertId => 2_isize,
        }
    }
}

impl Display for IntVarEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IntVarEventType::Invalid => write!(f, "Invalid"),
            IntVarEventType::LastInsertId => write!(f, "LastInsertId"),
            IntVarEventType::InsertId => write!(f, "InsertId"),
        }
    }
}
