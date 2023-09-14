use crate::replication::EventHeader;

#[derive(Debug, Clone)]
pub struct EventError {
    pub header: EventHeader,

    //Error message
    pub err: String,

    //Event data
    pub data: Vec<u8>,
}

impl std::error::Error for EventError {}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Header {header:?}, Data {data:?}, Err: {err}",
            header = &self.header,
            data = String::from_utf8_lossy(&self.data),
            err = &self.err
        )
    }
}
