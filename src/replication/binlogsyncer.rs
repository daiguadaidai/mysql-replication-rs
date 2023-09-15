use crate::error::ReplicationError;
use crate::loggerop;
use crate::mysql::{GtidSetEnum, Position};
use crate::replication::parser::BinlogParser;
use crate::replication::{common, GTIDEvent};
use std::fmt::Formatter;
use std::net;
use tokio::sync;

const _ERR_SYNC_RUNNING: &str = "Sync is running, must Close first";

impl std::fmt::Debug for BinlogSyncerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinlogSyncerConfig")
            .field("server_id", &self.server_id)
            .field("flavor", &self.flavor)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("user", &self.user)
            .field("password", &self.password)
            .field("localhost", &self.localhost)
            .field("charset", &self.charset)
            .field("semi_sync_enabled", &self.semi_sync_enabled)
            .field("raw_mode_enabled", &self.raw_mode_enabled)
            .field("parse_time", &self.parse_time)
            .field("timestamp_string_location", &self.timestamp_string_location)
            .field("use_decimal", &self.use_decimal)
            .field("heartbeat_period", &self.heartbeat_period)
            .field("read_timeout", &self.read_timeout)
            .field("max_reconnect_attempts", &self.max_reconnect_attempts)
            .field("disable_retry_sync", &self.disable_retry_sync)
            .field("verify_checksum", &self.verify_checksum)
            .field("dump_command_flag", &self.dump_command_flag)
            .field(
                "option",
                &"Box<dyn Fn(&net::TcpStream) -> Result<(), ReplicationError>>",
            )
            .field("dialer", &self.dialer)
            .field(
                "rows_event_decode_func",
                &"Box<dyn Fn(&mut RowsEvent, &[u8]) -> Result<(), ReplicationError>>",
            )
            .field("discard_gtid_set", &self.discard_gtid_set)
            .finish()
    }
}

// BinlogSyncerConfig is the configuration for BinlogSyncer.
pub struct BinlogSyncerConfig {
    // ServerID is the unique ID in cluster.
    pub server_id: u32,
    // Flavor is "mysql" or "mariadb", if not set, use "mysql" default.
    pub flavor: String,

    // Host is for MySQL server host.
    pub host: String,
    // Port is for MySQL server port.
    pub port: u16,
    // User is for MySQL user.
    pub user: String,
    // Password is for MySQL password.
    pub password: String,

    // Localhost is local hostname if register salve.
    // If not set, use os.Hostname() instead.
    pub localhost: String,
    // Charset is for MySQL client character set
    pub charset: String,
    // SemiSyncEnabled enables semi-sync or not.
    pub semi_sync_enabled: bool,
    // RawModeEnabled is for not parsing binlog event.
    pub raw_mode_enabled: bool,
    // // If not nil, use the provided tls.Config to connect to the database using TLS/SSL.
    // TLSConfig *tls.Config

    // Use replication.Time structure for timestamp and datetime.
    // We will use Local location for timestamp and UTC location for datatime.
    pub parse_time: bool,
    // If ParseTime is false, convert TIMESTAMP into this specified timezone. If
    // ParseTime is true, this option will have no effect and TIMESTAMP data will
    // be parsed into the local timezone and a full time.Time struct will be
    // returned.
    //
    // Note that MySQL TIMESTAMP columns are offset from the machine local
    // timezone while DATETIME columns are offset from UTC. This is consistent
    // with documented MySQL behaviour as it return TIMESTAMP in local timezone
    // and DATETIME in UTC.
    //
    // Setting this to UTC effectively equalizes the TIMESTAMP and DATETIME time
    // strings obtained from MySQL.
    pub timestamp_string_location: Option<chrono_tz::Tz>,
    // Use decimal.Decimal structure for decimals.
    pub use_decimal: bool,
    // RecvBufferSize sets the size in bytes of the operating system's receive buffer associated with the connection.
    pub recv_buffer_size: usize,
    // master heartbeat period
    pub heartbeat_period: std::time::Duration,
    // read timeout
    pub read_timeout: std::time::Duration,
    // maximum number of attempts to re-establish a broken connection, zero or negative number means infinite retry.
    // this configuration will not work if DisableRetrySync is true
    pub max_reconnect_attempts: usize,
    // whether disable re-sync for broken connection
    pub disable_retry_sync: bool,
    // Only works when MySQL/MariaDB variable binlog_checksum=CRC32.
    // For MySQL, binlog_checksum was introduced since 5.6.2, but CRC32 was set as default value since 5.6.6 .
    // https://dev.mysql.com/doc/refman/5.6/en/replication-options-binary-log.html#option_mysqld_binlog-checksum
    // For MariaDB, binlog_checksum was introduced since MariaDB 5.3, but CRC32 was set as default value since MariaDB 10.2.1 .
    // https://mariadb.com/kb/en/library/replication-and-binary-log-server-system-variables/#binlog_checksum
    pub verify_checksum: bool,
    // DumpCommandFlag is used to send binglog dump command. Default 0, aka BINLOG_DUMP_NEVER_STOP.
    // For MySQL, BINLOG_DUMP_NEVER_STOP and BINLOG_DUMP_NON_BLOCK are available.
    // https://dev.mysql.com/doc/internals/en/com-binlog-dump.html#binlog-dump-non-block
    // For MariaDB, BINLOG_DUMP_NEVER_STOP, BINLOG_DUMP_NON_BLOCK and BINLOG_SEND_ANNOTATE_ROWS_EVENT are available.
    // https://mariadb.com/kb/en/library/com_binlog_dump/
    // https://mariadb.com/kb/en/library/annotate_rows_event/
    pub dump_command_flag: u16,

    //Option function is used to set outside of BinlogSyncerConfig， between mysql connection and COM_REGISTER_SLAVE
    //For MariaDB: slave_gtid_ignore_duplicates、skip_replication、slave_until_gtid
    pub option: Box<dyn Fn(&net::TcpStream) -> Result<(), ReplicationError>>,

    // Set Dialer
    pub dialer: Option<net::TcpStream>,

    // Dialer client.Dialer
    pub rows_event_decode_func: Option<common::RowsEventDecodeFunc>,
    pub discard_gtid_set: bool,
}

// BinlogSyncer syncs binlog event from server.
pub struct BinlogSyncer {
    _m: sync::RwLock<()>,
    _cfg: BinlogSyncerConfig,
    _c: Option<net::TcpStream>,
    _parser: BinlogParser,
    _next_pos: Option<Position>,
    _prev_gset: Option<GtidSetEnum>,
    _curr_gset: Option<GtidSetEnum>,
    // instead of GTIDSet.Clone, use this to speed up calculate prevGset
    _prev_mysql_gtid_event: Option<GTIDEvent>,
    _running: bool,
    _ctx: tokio_context::context::Context,
    _cancel: tokio_context::context::Handle,
    _last_connection_id: u32,
    _retry_count: usize,
}

impl BinlogSyncer {
    // NewBinlogSyncer creates the BinlogSyncer with cfg.
    pub fn new(mut cfg: BinlogSyncerConfig) -> Result<BinlogSyncer, ReplicationError> {
        loggerop::init_log_once()?;
        if cfg.server_id == 0 {
            let err_msg = "can't use 0 as the server ID".to_string();
            log::error!("{}", &err_msg);
            return Err(ReplicationError::new(err_msg));
        }

        // if cfg.dialer.is_none() {
        //     cfg.dialer = Some()
        // }
        /*
        if cfg.Dialer == nil {
            dialer := &net.Dialer{}
            cfg.Dialer = dialer.DialContext
        }
        */

        // Clear the Password to avoid outputing it in log.
        let pass = cfg.password.clone();
        cfg.password = String::new();
        log::info!("create BinlogSyncer with config {:?}", &cfg);
        cfg.password = pass;

        let mut parser = BinlogParser::new();
        parser.set_flavor(cfg.flavor.clone());
        parser.set_raw_mode(cfg.raw_mode_enabled);
        parser.set_parse_time(cfg.parse_time);
        parser.set_timestamp_string_location(cfg.timestamp_string_location.clone());
        parser.set_use_decimal(cfg.use_decimal);
        parser.set_verify_checksum(cfg.verify_checksum);
        parser.set_rows_event_decode_func(cfg.rows_event_decode_func.clone());

        let (ctx, cancel) = tokio_context::context::Context::new();
        Ok(BinlogSyncer {
            _m: sync::RwLock::new(()),
            _cfg: cfg,
            _c: Default::default(),
            _parser: parser,
            _next_pos: None,
            _prev_gset: None,
            _curr_gset: None,
            _prev_mysql_gtid_event: None,
            _running: false,
            _ctx: ctx,
            _cancel: cancel,
            _last_connection_id: 0,
            _retry_count: 0,
        })
    }
}

/*
// Close closes the BinlogSyncer.
func (b *BinlogSyncer) Close() {
    b.m.Lock()
    defer b.m.Unlock()

    b.close()
}

func (b *BinlogSyncer) close() {
    if b.isClosed() {
        return
    }

    b.cfg.Logger.Info("syncer is closing...")

    b.running = false
    b.cancel()

    if b.c != nil {
        err := b.c.SetReadDeadline(time.Now().Add(100 * time.Millisecond))
        if err != nil {
            b.cfg.Logger.Warnf(`could not set read deadline: %s`, err)
        }
    }

    // kill last connection id
    if b.lastConnectionID > 0 {
        // Use a new connection to kill the binlog syncer
        // because calling KILL from the same connection
        // doesn't actually disconnect it.
        c, err := b.newConnection(context.Background())
        if err == nil {
            b.killConnection(c, b.lastConnectionID)
            c.Close()
        }
    }

    b.wg.Wait()

    if b.c != nil {
        b.c.Close()
    }

    b.cfg.Logger.Info("syncer is closed")
}

func (b *BinlogSyncer) isClosed() bool {
    select {
    case <-b.ctx.Done():
        return true
    default:
        return false
    }
}

func (b *BinlogSyncer) registerSlave() error {
    if b.c != nil {
        b.c.Close()
    }

    var err error
    b.c, err = b.newConnection(b.ctx)
    if err != nil {
        return errors.Trace(err)
    }

    if b.cfg.Option != nil {
        if err = b.cfg.Option(b.c); err != nil {
            return errors.Trace(err)
        }
    }

    if len(b.cfg.Charset) != 0 {
        if err = b.c.SetCharset(b.cfg.Charset); err != nil {
            return errors.Trace(err)
        }
    }

    //set read timeout
    if b.cfg.ReadTimeout > 0 {
        _ = b.c.SetReadDeadline(time.Now().Add(b.cfg.ReadTimeout))
    }

    if b.cfg.RecvBufferSize > 0 {
        if tcp, ok := b.c.Conn.Conn.(*net.TCPConn); ok {
            _ = tcp.SetReadBuffer(b.cfg.RecvBufferSize)
        }
    }

    // kill last connection id
    if b.lastConnectionID > 0 {
        b.killConnection(b.c, b.lastConnectionID)
    }

    // save last last connection id for kill
    b.lastConnectionID = b.c.GetConnectionID()

    //for mysql 5.6+, binlog has a crc32 checksum
    //before mysql 5.6, this will not work, don't matter.:-)
    if r, err := b.c.Execute("SHOW GLOBAL VARIABLES LIKE 'BINLOG_CHECKSUM'"); err != nil {
        return errors.Trace(err)
    } else {
        s, _ := r.GetString(0, 1)
        if s != "" {
            // maybe CRC32 or NONE

            // mysqlbinlog.cc use NONE, see its below comments:
            // Make a notice to the server that this client
            // is checksum-aware. It does not need the first fake Rotate
            // necessary checksummed.
            // That preference is specified below.

            if _, err = b.c.Execute(`SET @master_binlog_checksum='NONE'`); err != nil {
                return errors.Trace(err)
            }

            // if _, err = b.c.Execute(`SET @master_binlog_checksum=@@global.binlog_checksum`); err != nil {
            // 	return errors.Trace(err)
            // }
        }
    }

    if b.cfg.Flavor == MariaDBFlavor {
        // Refer https://github.com/alibaba/canal/wiki/BinlogChange(MariaDB5&10)
        // Tell the server that we understand GTIDs by setting our slave capability
        // to MARIA_SLAVE_CAPABILITY_GTID = 4 (MariaDB >= 10.0.1).
        if _, err := b.c.Execute("SET @mariadb_slave_capability=4"); err != nil {
            return errors.Errorf("failed to set @mariadb_slave_capability=4: %v", err)
        }
    }

    if b.cfg.HeartbeatPeriod > 0 {
        _, err = b.c.Execute(fmt.Sprintf("SET @master_heartbeat_period=%d;", b.cfg.HeartbeatPeriod))
        if err != nil {
            b.cfg.Logger.Errorf("failed to set @master_heartbeat_period=%d, err: %v", b.cfg.HeartbeatPeriod, err)
            return errors.Trace(err)
        }
    }

    if err = b.writeRegisterSlaveCommand(); err != nil {
        return errors.Trace(err)
    }

    if _, err = b.c.ReadOKPacket(); err != nil {
        return errors.Trace(err)
    }

    serverUUID, err := uuid.NewUUID()
    if err != nil {
        b.cfg.Logger.Errorf("failed to get new uud %v", err)
        return errors.Trace(err)
    }
    if _, err = b.c.Execute(fmt.Sprintf("SET @slave_uuid = '%s', @replica_uuid = '%s'", serverUUID, serverUUID)); err != nil {
        b.cfg.Logger.Errorf("failed to set @slave_uuid = '%s', err: %v", serverUUID, err)
        return errors.Trace(err)
    }

    return nil
}

func (b *BinlogSyncer) enableSemiSync() error {
    if !b.cfg.SemiSyncEnabled {
        return nil
    }

    if r, err := b.c.Execute("SHOW VARIABLES LIKE 'rpl_semi_sync_master_enabled';"); err != nil {
        return errors.Trace(err)
    } else {
        s, _ := r.GetString(0, 1)
        if s != "ON" {
            b.cfg.Logger.Errorf("master does not support semi synchronous replication, use no semi-sync")
            b.cfg.SemiSyncEnabled = false
            return nil
        }
    }

    _, err := b.c.Execute(`SET @rpl_semi_sync_slave = 1;`)
    if err != nil {
        return errors.Trace(err)
    }

    return nil
}

func (b *BinlogSyncer) prepare() error {
    if b.isClosed() {
        return errors.Trace(ErrSyncClosed)
    }

    if err := b.registerSlave(); err != nil {
        return errors.Trace(err)
    }

    if err := b.enableSemiSync(); err != nil {
        return errors.Trace(err)
    }

    return nil
}

func (b *BinlogSyncer) startDumpStream() *BinlogStreamer {
    b.running = true

    s := NewBinlogStreamer()

    b.wg.Add(1)
    go b.onStream(s)
    return s
}

// GetNextPosition returns the next position of the syncer
func (b *BinlogSyncer) GetNextPosition() Position {
    return b.nextPos
}

// StartSync starts syncing from the `pos` position.
func (b *BinlogSyncer) StartSync(pos Position) (*BinlogStreamer, error) {
    b.cfg.Logger.Infof("begin to sync binlog from position %s", pos)

    b.m.Lock()
    defer b.m.Unlock()

    if b.running {
        return nil, errors.Trace(errSyncRunning)
    }

    if err := b.prepareSyncPos(pos); err != nil {
        return nil, errors.Trace(err)
    }

    return b.startDumpStream(), nil
}

// StartSyncGTID starts syncing from the `gset` GTIDSet.
func (b *BinlogSyncer) StartSyncGTID(gset GTIDSet) (*BinlogStreamer, error) {
    b.cfg.Logger.Infof("begin to sync binlog from GTID set %s", gset)

    b.prevMySQLGTIDEvent = nil
    b.prevGset = gset

    b.m.Lock()
    defer b.m.Unlock()

    if b.running {
        return nil, errors.Trace(errSyncRunning)
    }

    // establishing network connection here and will start getting binlog events from "gset + 1", thus until first
    // MariadbGTIDEvent/GTIDEvent event is received - we effectively do not have a "current GTID"
    b.currGset = nil

    if err := b.prepare(); err != nil {
        return nil, errors.Trace(err)
    }

    var err error
    switch b.cfg.Flavor {
    case MariaDBFlavor:
        err = b.writeBinlogDumpMariadbGTIDCommand(gset)
    default:
        // default use MySQL
        err = b.writeBinlogDumpMysqlGTIDCommand(gset)
    }

    if err != nil {
        return nil, err
    }

    return b.startDumpStream(), nil
}

func (b *BinlogSyncer) writeBinlogDumpCommand(p Position) error {
    b.c.ResetSequence()

    data := make([]byte, 4+1+4+2+4+len(p.Name))

    pos := 4
    data[pos] = COM_BINLOG_DUMP
    pos++

    binary.LittleEndian.PutUint32(data[pos:], p.Pos)
    pos += 4

    binary.LittleEndian.PutUint16(data[pos:], b.cfg.DumpCommandFlag)
    pos += 2

    binary.LittleEndian.PutUint32(data[pos:], b.cfg.ServerID)
    pos += 4

    copy(data[pos:], p.Name)

    return b.c.WritePacket(data)
}

func (b *BinlogSyncer) writeBinlogDumpMysqlGTIDCommand(gset GTIDSet) error {
    p := Position{Name: "", Pos: 4}
    gtidData := gset.Encode()

    b.c.ResetSequence()

    data := make([]byte, 4+1+2+4+4+len(p.Name)+8+4+len(gtidData))
    pos := 4
    data[pos] = COM_BINLOG_DUMP_GTID
    pos++

    binary.LittleEndian.PutUint16(data[pos:], 0)
    pos += 2

    binary.LittleEndian.PutUint32(data[pos:], b.cfg.ServerID)
    pos += 4

    binary.LittleEndian.PutUint32(data[pos:], uint32(len(p.Name)))
    pos += 4

    n := copy(data[pos:], p.Name)
    pos += n

    binary.LittleEndian.PutUint64(data[pos:], uint64(p.Pos))
    pos += 8

    binary.LittleEndian.PutUint32(data[pos:], uint32(len(gtidData)))
    pos += 4
    n = copy(data[pos:], gtidData)
    pos += n

    data = data[0:pos]

    return b.c.WritePacket(data)
}

func (b *BinlogSyncer) writeBinlogDumpMariadbGTIDCommand(gset GTIDSet) error {
    // Copy from vitess

    startPos := gset.String()

    // Set the slave_connect_state variable before issuing COM_BINLOG_DUMP to
    // provide the start position in GTID form.
    query := fmt.Sprintf("SET @slave_connect_state='%s'", startPos)

    if _, err := b.c.Execute(query); err != nil {
        return errors.Errorf("failed to set @slave_connect_state='%s': %v", startPos, err)
    }

    // Real slaves set this upon connecting if their gtid_strict_mode option was
    // enabled. We always use gtid_strict_mode because we need it to make our
    // internal GTID comparisons safe.
    if _, err := b.c.Execute("SET @slave_gtid_strict_mode=1"); err != nil {
        return errors.Errorf("failed to set @slave_gtid_strict_mode=1: %v", err)
    }

    // Since we use @slave_connect_state, the file and position here are ignored.
    return b.writeBinlogDumpCommand(Position{Name: "", Pos: 0})
}

// localHostname returns the hostname that register slave would register as.
func (b *BinlogSyncer) localHostname() string {
    if len(b.cfg.Localhost) == 0 {
        h, _ := os.Hostname()
        return h
    }
    return b.cfg.Localhost
}

func (b *BinlogSyncer) writeRegisterSlaveCommand() error {
    b.c.ResetSequence()

    hostname := b.localHostname()

    // This should be the name of slave host not the host we are connecting to.
    data := make([]byte, 4+1+4+1+len(hostname)+1+len(b.cfg.User)+1+len(b.cfg.Password)+2+4+4)
    pos := 4

    data[pos] = COM_REGISTER_SLAVE
    pos++

    binary.LittleEndian.PutUint32(data[pos:], b.cfg.ServerID)
    pos += 4

    // This should be the name of slave hostname not the host we are connecting to.
    data[pos] = uint8(len(hostname))
    pos++
    n := copy(data[pos:], hostname)
    pos += n

    data[pos] = uint8(len(b.cfg.User))
    pos++
    n = copy(data[pos:], b.cfg.User)
    pos += n

    data[pos] = uint8(len(b.cfg.Password))
    pos++
    n = copy(data[pos:], b.cfg.Password)
    pos += n

    binary.LittleEndian.PutUint16(data[pos:], b.cfg.Port)
    pos += 2

    //replication rank, not used
    binary.LittleEndian.PutUint32(data[pos:], 0)
    pos += 4

    // master ID, 0 is OK
    binary.LittleEndian.PutUint32(data[pos:], 0)

    return b.c.WritePacket(data)
}

func (b *BinlogSyncer) replySemiSyncACK(p Position) error {
    b.c.ResetSequence()

    data := make([]byte, 4+1+8+len(p.Name))
    pos := 4
    // semi sync indicator
    data[pos] = SemiSyncIndicator
    pos++

    binary.LittleEndian.PutUint64(data[pos:], uint64(p.Pos))
    pos += 8

    copy(data[pos:], p.Name)

    err := b.c.WritePacket(data)
    if err != nil {
        return errors.Trace(err)
    }

    return nil
}

func (b *BinlogSyncer) retrySync() error {
    b.m.Lock()
    defer b.m.Unlock()

    b.parser.Reset()
    b.prevMySQLGTIDEvent = nil

    if b.prevGset != nil {
        msg := fmt.Sprintf("begin to re-sync from %s", b.prevGset.String())
        if b.currGset != nil {
            msg = fmt.Sprintf("%v (last read GTID=%v)", msg, b.currGset)
        }
        b.cfg.Logger.Infof(msg)

        if err := b.prepareSyncGTID(b.prevGset); err != nil {
            return errors.Trace(err)
        }
    } else {
        b.cfg.Logger.Infof("begin to re-sync from %s", b.nextPos)
        if err := b.prepareSyncPos(b.nextPos); err != nil {
            return errors.Trace(err)
        }
    }

    return nil
}

func (b *BinlogSyncer) prepareSyncPos(pos Position) error {
    // always start from position 4
    if pos.Pos < 4 {
        pos.Pos = 4
    }

    if err := b.prepare(); err != nil {
        return errors.Trace(err)
    }

    if err := b.writeBinlogDumpCommand(pos); err != nil {
        return errors.Trace(err)
    }

    return nil
}

func (b *BinlogSyncer) prepareSyncGTID(gset GTIDSet) error {
    var err error

    // re establishing network connection here and will start getting binlog events from "gset + 1", thus until first
    // MariadbGTIDEvent/GTIDEvent event is received - we effectively do not have a "current GTID"
    b.currGset = nil

    if err = b.prepare(); err != nil {
        return errors.Trace(err)
    }

    switch b.cfg.Flavor {
    case MariaDBFlavor:
        err = b.writeBinlogDumpMariadbGTIDCommand(gset)
    default:
        // default use MySQL
        err = b.writeBinlogDumpMysqlGTIDCommand(gset)
    }

    if err != nil {
        return err
    }
    return nil
}

func (b *BinlogSyncer) onStream(s *BinlogStreamer) {
    defer func() {
        if e := recover(); e != nil {
            s.closeWithError(fmt.Errorf("Err: %v\n Stack: %s", e, Pstack()))
        }
        b.wg.Done()
    }()

    for {
        data, err := b.c.ReadPacket()
        select {
        case <-b.ctx.Done():
            s.close()
            return
        default:
        }

        if err != nil {
            b.cfg.Logger.Error(err)
            // we meet connection error, should re-connect again with
            // last nextPos or nextGTID we got.
            if len(b.nextPos.Name) == 0 && b.prevGset == nil {
                // we can't get the correct position, close.
                s.closeWithError(err)
                return
            }

            if b.cfg.DisableRetrySync {
                b.cfg.Logger.Warn("retry sync is disabled")
                s.closeWithError(err)
                return
            }

            for {
                select {
                case <-b.ctx.Done():
                    s.close()
                    return
                case <-time.After(time.Second):
                    b.retryCount++
                    if err = b.retrySync(); err != nil {
                        if b.cfg.MaxReconnectAttempts > 0 && b.retryCount >= b.cfg.MaxReconnectAttempts {
                            b.cfg.Logger.Errorf("retry sync err: %v, exceeded max retries (%d)", err, b.cfg.MaxReconnectAttempts)
                            s.closeWithError(err)
                            return
                        }

                        b.cfg.Logger.Errorf("retry sync err: %v, wait 1s and retry again", err)
                        continue
                    }
                }

                break
            }

            // we connect the server and begin to re-sync again.
            continue
        }

        //set read timeout
        if b.cfg.ReadTimeout > 0 {
            _ = b.c.SetReadDeadline(time.Now().Add(b.cfg.ReadTimeout))
        }

        // Reset retry count on successful packet receieve
        b.retryCount = 0

        switch data[0] {
        case OK_HEADER:
            if err = b.parseEvent(s, data); err != nil {
                s.closeWithError(err)
                return
            }
        case ERR_HEADER:
            err = b.c.HandleErrorPacket(data)
            s.closeWithError(err)
            return
        case EOF_HEADER:
            // refer to https://dev.mysql.com/doc/internals/en/com-binlog-dump.html#binlog-dump-non-block
            // when COM_BINLOG_DUMP command use BINLOG_DUMP_NON_BLOCK flag,
            // if there is no more event to send an EOF_Packet instead of blocking the connection
            b.cfg.Logger.Info("receive EOF packet, no more binlog event now.")
            continue
        default:
            b.cfg.Logger.Errorf("invalid stream header %c", data[0])
            continue
        }
    }
}

func (b *BinlogSyncer) parseEvent(s *BinlogStreamer, data []byte) error {
    //skip OK byte, 0x00
    data = data[1:]

    needACK := false
    if b.cfg.SemiSyncEnabled && (data[0] == SemiSyncIndicator) {
        needACK = data[1] == 0x01
        //skip semi sync header
        data = data[2:]
    }

    e, err := b.parser.Parse(data)
    if err != nil {
        return errors.Trace(err)
    }

    if e.Header.LogPos > 0 {
        // Some events like FormatDescriptionEvent return 0, ignore.
        b.nextPos.Pos = e.Header.LogPos
    }

    getCurrentGtidSet := func() GTIDSet {
        if b.currGset == nil {
            return nil
        }
        return b.currGset.Clone()
    }

    switch event := e.Event.(type) {
    case *RotateEvent:
        b.nextPos.Name = string(event.NextLogName)
        b.nextPos.Pos = uint32(event.Position)
        b.cfg.Logger.Infof("rotate to %s", b.nextPos)
    case *GTIDEvent:
        if b.prevGset == nil {
            break
        }
        if b.currGset == nil {
            b.currGset = b.prevGset.Clone()
        }
        u, _ := uuid.FromBytes(event.SID)
        b.currGset.(*MysqlGTIDSet).AddGTID(u, event.GNO)
        if b.prevMySQLGTIDEvent != nil {
            u, _ = uuid.FromBytes(b.prevMySQLGTIDEvent.SID)
            b.prevGset.(*MysqlGTIDSet).AddGTID(u, b.prevMySQLGTIDEvent.GNO)
        }
        b.prevMySQLGTIDEvent = event
    case *MariadbGTIDEvent:
        if b.prevGset == nil {
            break
        }
        if b.currGset == nil {
            b.currGset = b.prevGset.Clone()
        }
        prev := b.currGset.Clone()
        err = b.currGset.(*MariadbGTIDSet).AddSet(&event.GTID)
        if err != nil {
            return errors.Trace(err)
        }
        // right after reconnect we will see same gtid as we saw before, thus currGset will not get changed
        if !b.currGset.Equal(prev) {
            b.prevGset = prev
        }
    case *XIDEvent:
        if !b.cfg.DiscardGTIDSet {
            event.GSet = getCurrentGtidSet()
        }
    case *QueryEvent:
        if !b.cfg.DiscardGTIDSet {
            event.GSet = getCurrentGtidSet()
        }
    }

    needStop := false
    select {
    case s.ch <- e:
    case <-b.ctx.Done():
        needStop = true
    }

    if needACK {
        err := b.replySemiSyncACK(b.nextPos)
        if err != nil {
            return errors.Trace(err)
        }
    }

    if needStop {
        return errors.New("sync is been closing...")
    }

    return nil
}

// LastConnectionID returns last connectionID.
func (b *BinlogSyncer) LastConnectionID() uint32 {
    return b.lastConnectionID
}

func (b *BinlogSyncer) newConnection(ctx context.Context) (*client.Conn, error) {
    var addr string
    if b.cfg.Port != 0 {
        addr = net.JoinHostPort(b.cfg.Host, strconv.Itoa(int(b.cfg.Port)))
    } else {
        addr = b.cfg.Host
    }

    timeoutCtx, cancel := context.WithTimeout(ctx, time.Second*10)
    defer cancel()

    return client.ConnectWithDialer(timeoutCtx, "", addr, b.cfg.User, b.cfg.Password,
        "", b.cfg.Dialer, func(c *client.Conn) {
            c.SetTLSConfig(b.cfg.TLSConfig)
            c.SetAttributes(map[string]string{"_client_role": "binary_log_listener"})
        })
}

func (b *BinlogSyncer) killConnection(conn *client.Conn, id uint32) {
    cmd := fmt.Sprintf("KILL %d", id)
    if _, err := conn.Execute(cmd); err != nil {
        b.cfg.Logger.Errorf("kill connection %d error %v", id, err)
        // Unknown thread id
        if code := ErrorCode(err.Error()); code != ER_NO_SUCH_THREAD {
            b.cfg.Logger.Error(errors.Trace(err))
        }
    }
    b.cfg.Logger.Infof("kill last connection id %d", id)
}

 */
