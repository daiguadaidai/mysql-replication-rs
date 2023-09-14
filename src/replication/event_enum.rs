use crate::error::ReplicationError;
use crate::replication::{
    BeginLoadQueryEvent, Event, EventHeader, ExecuteLoadQueryEvent, FormatDescriptionEvent,
    GTIDEvent, GenericEvent, IntVarEvent, MariadbAnnotateRowsEvent, MariadbBinlogCheckPointEvent,
    MariadbGTIDEvent, MariadbGTIDListEvent, PreviousGTIDsEvent, QueryEvent, RotateEvent, RowsEvent,
    RowsQueryEvent, TableMapEvent, TransactionPayloadEvent, XIDEvent,
};
use std::io::Write;

#[derive(Debug, Clone)]
pub enum EventEnum {
    TableMapEvent(TableMapEvent),
    RowsEvent(RowsEvent),
    RowsQueryEvent(RowsQueryEvent),
    EventHeader(EventHeader),
    FormatDescriptionEvent(FormatDescriptionEvent),
    RotateEvent(RotateEvent),
    PreviousGTIDsEvent(PreviousGTIDsEvent),
    XIDEvent(XIDEvent),
    QueryEvent(QueryEvent),
    GTIDEvent(GTIDEvent),
    BeginLoadQueryEvent(BeginLoadQueryEvent),
    ExecuteLoadQueryEvent(ExecuteLoadQueryEvent),
    MariadbAnnotateRowsEvent(MariadbAnnotateRowsEvent),
    MariadbBinlogCheckPointEvent(MariadbBinlogCheckPointEvent),
    MariadbGTIDEvent(MariadbGTIDEvent),
    MariadbGTIDListEvent(MariadbGTIDListEvent),
    IntVarEvent(IntVarEvent),
    TransactionPayloadEvent(TransactionPayloadEvent),
    GenericEvent(GenericEvent),
}

impl EventEnum {
    pub fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        match self {
            EventEnum::TableMapEvent(ref mut r) => r.decode(data),
            EventEnum::RowsEvent(ref mut r) => r.decode(data),
            EventEnum::RowsQueryEvent(ref mut r) => r.decode(data),
            EventEnum::EventHeader(ref mut r) => r.decode(data),
            EventEnum::FormatDescriptionEvent(ref mut r) => r.decode(data),
            EventEnum::RotateEvent(ref mut r) => r.decode(data),
            EventEnum::PreviousGTIDsEvent(ref mut r) => r.decode(data),
            EventEnum::XIDEvent(ref mut r) => r.decode(data),
            EventEnum::QueryEvent(ref mut r) => r.decode(data),
            EventEnum::GTIDEvent(ref mut r) => r.decode(data),
            EventEnum::BeginLoadQueryEvent(ref mut r) => r.decode(data),
            EventEnum::ExecuteLoadQueryEvent(ref mut r) => r.decode(data),
            EventEnum::MariadbAnnotateRowsEvent(ref mut r) => r.decode(data),
            EventEnum::MariadbBinlogCheckPointEvent(ref mut r) => r.decode(data),
            EventEnum::MariadbGTIDEvent(ref mut r) => r.decode(data),
            EventEnum::MariadbGTIDListEvent(ref mut r) => r.decode(data),
            EventEnum::IntVarEvent(ref mut r) => r.decode(data),
            EventEnum::TransactionPayloadEvent(ref mut r) => r.decode(data),
            EventEnum::GenericEvent(ref mut r) => r.decode(data),
        }
    }

    pub fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        match self {
            EventEnum::TableMapEvent(ref mut r) => r.dump(writer),
            EventEnum::RowsEvent(ref mut r) => r.dump(writer),
            EventEnum::RowsQueryEvent(ref mut r) => r.dump(writer),
            EventEnum::EventHeader(ref mut r) => r.dump(writer),
            EventEnum::FormatDescriptionEvent(ref mut r) => r.dump(writer),
            EventEnum::RotateEvent(ref mut r) => r.dump(writer),
            EventEnum::PreviousGTIDsEvent(ref mut r) => r.dump(writer),
            EventEnum::XIDEvent(ref mut r) => r.dump(writer),
            EventEnum::QueryEvent(ref mut r) => r.dump(writer),
            EventEnum::GTIDEvent(ref mut r) => r.dump(writer),
            EventEnum::BeginLoadQueryEvent(ref mut r) => r.dump(writer),
            EventEnum::ExecuteLoadQueryEvent(ref mut r) => r.dump(writer),
            EventEnum::MariadbAnnotateRowsEvent(ref mut r) => r.dump(writer),
            EventEnum::MariadbBinlogCheckPointEvent(ref mut r) => r.dump(writer),
            EventEnum::MariadbGTIDEvent(ref mut r) => r.dump(writer),
            EventEnum::MariadbGTIDListEvent(ref mut r) => r.dump(writer),
            EventEnum::IntVarEvent(ref mut r) => r.dump(writer),
            EventEnum::TransactionPayloadEvent(ref mut r) => r.dump(writer),
            EventEnum::GenericEvent(ref mut r) => r.dump(writer),
        }
    }
}
