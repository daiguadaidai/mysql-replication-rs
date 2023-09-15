use crate::error::ReplicationError;
use crate::replication::RowsEvent;
use std::rc::Rc;

pub type RowsEventDecodeFunc = Rc<dyn Fn(&mut RowsEvent, &[u8]) -> Result<(), ReplicationError>>;
