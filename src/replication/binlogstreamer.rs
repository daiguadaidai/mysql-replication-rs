use crate::error::ReplicationError;
use crate::replication::BinlogEvent;
use chrono::NaiveDateTime;
use tokio::select;
use tokio_context::context;

pub const ERR_NEED_SYNC_AGIN: &str = "Last sync error or closed, try sync and get event again";
pub const ERR_SYNC_CLOSED: &str = "Sync was closed";
pub const BINLOG_STREAMER_RECEIVER_SIZE: usize = 10240;
pub const BINLOG_STREAMER_ERROR_SIZE: usize = 4;

// BinlogStreamer gets the streaming event.
pub struct BinlogStreamer {
    binlog_event_sender: async_channel::Sender<BinlogEvent>,
    binlog_event_recv: async_channel::Receiver<BinlogEvent>,
    err_sender: async_channel::Sender<ReplicationError>,
    err_recv: async_channel::Receiver<ReplicationError>,
    err: Result<(), ReplicationError>,
}

// GetEvent gets the binlog event one by one, it will block until Syncer receives any events from MySQL
// or meets a sync error. You can pass a context (like Cancel or Timeout) to break the block.
impl BinlogStreamer {
    pub fn new() -> BinlogStreamer {
        let (binlog_event_sender, binlog_event_recv) =
            async_channel::bounded::<BinlogEvent>(BINLOG_STREAMER_RECEIVER_SIZE);
        let (err_sender, err_recv) =
            async_channel::bounded::<ReplicationError>(BINLOG_STREAMER_ERROR_SIZE);

        BinlogStreamer {
            binlog_event_sender,
            binlog_event_recv,
            err_sender,
            err_recv,
            err: Ok(()),
        }
    }

    pub fn clone_with_no_error(&self) -> BinlogStreamer {
        BinlogStreamer {
            binlog_event_sender: self.binlog_event_sender.clone(),
            binlog_event_recv: self.binlog_event_recv.clone(),
            err_sender: self.err_sender.clone(),
            err_recv: self.err_recv.clone(),
            err: Ok(()),
        }
    }

    pub async fn get_event(
        &mut self,
        mut ctx: context::Context,
    ) -> Result<Option<BinlogEvent>, ReplicationError> {
        if self.err.is_err() {
            return Err(ReplicationError::new(ERR_NEED_SYNC_AGIN.to_string()));
        }

        select! {
            _ = ctx.done() => {
                return Ok(None);
            }
            err = self.err_recv.recv() => {
                return Err(self._receive_error(err));
            }
            binlog_event = self.binlog_event_recv.recv() => {
                let be = self._receive_binlog_event(binlog_event)?;
                return Ok(Some(be));
            }
        }
    }

    // GetEventWithStartTime gets the binlog event with starttime, if current binlog event timestamp smaller than specify starttime
    // return nil event
    pub async fn get_event_with_start_time(
        &mut self,
        mut ctx: context::Context,
        start_time: NaiveDateTime,
    ) -> Result<Option<BinlogEvent>, ReplicationError> {
        if self.err.is_err() {
            return Err(ReplicationError::new(ERR_NEED_SYNC_AGIN.to_string()));
        }

        let start_unix = start_time.timestamp();
        select! {
            _ = ctx.done() => {
                return Ok(None);
            }
            err = self.err_recv.recv() => {
                return Err(self._receive_error(err));
            }
            binlog_event = self.binlog_event_recv.recv() => {
                let be = self._receive_binlog_event(binlog_event)?;
                if be.header.as_ref().unwrap().timestamp as i64 >= start_unix {
                    return Ok(Some(be));
                }
                return Ok(None);
            }
        }
    }

    fn _receive_binlog_event(
        &mut self,
        binlog_event_rs: Result<BinlogEvent, async_channel::RecvError>,
    ) -> Result<BinlogEvent, ReplicationError> {
        match binlog_event_rs {
            Ok(v) => Ok(v),
            Err(e) => {
                self.err = Err(ReplicationError::new(e.to_string()));
                Err(ReplicationError::new(e.to_string()))
            }
        }
    }

    fn _receive_error(
        &mut self,
        error_rs: Result<ReplicationError, async_channel::RecvError>,
    ) -> ReplicationError {
        let e = match error_rs {
            Ok(v) => v,
            Err(e) => ReplicationError::new(e.to_string()),
        };
        self.err = Err(ReplicationError::new(e.to_string()));
        e
    }

    pub fn get_binlog_event_tx(&self) -> async_channel::Sender<BinlogEvent> {
        self.binlog_event_sender.clone()
    }

    pub fn get_err_tx(&self) -> async_channel::Sender<ReplicationError> {
        self.err_sender.clone()
    }

    // DumpEvents dumps all left events
    pub async fn dump_events(&mut self) -> Result<Vec<BinlogEvent>, ReplicationError> {
        let count = self.binlog_event_recv.len();
        let mut events = Vec::<BinlogEvent>::with_capacity(count);
        for _ in 0..count {
            let ev = self.binlog_event_recv.recv().await?;
            events.push(ev);
        }

        Ok(events)
    }

    async fn _close(&mut self) -> Result<(), ReplicationError> {
        let _ = self._close_with_error(Ok(())).await?;
        Ok(())
    }

    async fn _close_with_error(
        &mut self,
        err: Result<(), ReplicationError>,
    ) -> Result<(), ReplicationError> {
        let new_err = match err {
            Ok(_) => ReplicationError::new(ERR_NEED_SYNC_AGIN.to_string()),
            Err(e) => {
                log::error!("{}", e.to_string());
                e
            }
        };

        if let Err(e) = self.err_sender.send(new_err).await {
            return Err(ReplicationError::new(e.to_string()));
        }

        Ok(())
    }

    // AddEventToStreamer adds a binlog event to the streamer. You can use it when you want to add an event to the streamer manually.
    // can be used in replication handlers
    pub async fn add_event_to_streamer(&mut self, ev: BinlogEvent) -> Result<(), ReplicationError> {
        select! {
            err = self.err_recv.recv() => {
                return Err(self._receive_error(err));
            }
            rs = self.binlog_event_sender.send(ev) => {
                return match rs {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ReplicationError::new(e.to_string()))
                }
            }
        }
    }

    // AddErrorToStreamer adds an error to the streamer.
    pub async fn add_error_to_streamer(
        &mut self,
        err: ReplicationError,
    ) -> Result<(), ReplicationError> {
        select! {
            rs = self.err_sender.send(err) => {
                return match rs {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ReplicationError::new(e.to_string()))
                }
            }
        }
    }
}
