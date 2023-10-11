use crate::error::{MysqlError, ReplicationError};
use crate::mysql;
use std::io::{BufReader, Read, Write};
use std::net;

const _CONN_READ_BUFFER_SIZE: usize = 65536;

pub struct Conn {
    _conn: Option<net::TcpStream>,
    // we removed the buffer reader because it will cause the SSLRequest to block (tls connection handshake won't be
    // able to read the "Client Hello" data since it has been buffered into the buffer reader)
    // @todo
    // bufPool *BufPool
    // br      *bufio.Reader
    // reader  io.Reader
    _copy_n_buf: Vec<u8>,
    _header: [u8; 4],
    pub sequence: u8,
    pub compression: u8,
    pub compressed_sequence: u8,
    _compressed_header: [u8; 7],
    compressed_reader_active: bool,
    // @todo
    // compressedReader io.Reader
}

impl Conn {
    pub fn new(conn: net::TcpStream) -> Conn {
        /*
        c := new(Conn)
        c.Conn = conn

        c.bufPool = NewBufPool()
        c.br = bufio.NewReaderSize(c, 65536) // 64kb
        c.reader = c.br

        c.copyNBuf = make([]byte, 16*1024)

        return c
             */

        Conn {
            _conn: Some(conn),
            _copy_n_buf: Vec::with_capacity(16 * 1024),
            _header: [0, 0, 0, 0],
            sequence: 0,
            compression: 0,
            compressed_sequence: 0,
            _compressed_header: [0, 0, 0, 0, 0, 0, 0],
            compressed_reader_active: false,
        }
    }

    pub fn new_tls_conn(conn: net::TcpStream) -> Conn {
        /*
        c := new(Conn)
        c.Conn = conn

        c.bufPool = NewBufPool()
        c.reader = c

        c.copyNBuf = make([]byte, 16*1024)

        return c
             */

        Conn {
            _conn: Some(conn),
            _copy_n_buf: Vec::with_capacity(16 * 1024),
            _header: [0, 0, 0, 0],
            sequence: 0,
            compression: 0,
            compressed_sequence: 0,
            _compressed_header: [0, 0, 0, 0, 0, 0, 0],
            compressed_reader_active: false,
        }
    }

    pub fn read_packet(&mut self) -> Result<Vec<u8>, ReplicationError> {
        return self.read_packet_reuse_mem(&mut vec![]);
    }

    pub fn read_packet_reuse_mem(&mut self, dst: &mut [u8]) -> Result<Vec<u8>, ReplicationError> {
        // Here we use `sync.Pool` to avoid allocate/destroy buffers frequently.
        let mut buf = Vec::<u8>::new();

        let mut reader = BufReader::with_capacity(
            _CONN_READ_BUFFER_SIZE,
            self._conn
                .as_mut()
                .ok_or(ReplicationError::new("conn is none".to_string()))?,
        );
        if self.compression != mysql::MYSQL_COMPRESS_NONE {
            if !self.compressed_reader_active {
                let _ = reader
                    .read_exact(&mut self._compressed_header)
                    .map_err(|e| {
                        ReplicationError::new(format!(
                            "{}. io.ReadFull(compressedHeader) failed. err {}",
                            MysqlError::ErrBadConn.to_string(),
                            e.to_string()
                        ))
                    })?;

                let compressed_sequence = self._compressed_header[3];
                let uncompressed_length = (self._compressed_header[4] as u32
                    | (self._compressed_header[5] as u32) << 8
                    | (self._compressed_header[6] as u32) << 16)
                    as usize;
                if compressed_sequence != self.compressed_sequence {
                    return Err(ReplicationError::new(format!(
                        "invalid compressed sequence {} != {}",
                        compressed_sequence, self.compressed_sequence
                    )));
                }
                if uncompressed_length > 0 {
                    match self.compression {
                        mysql::MYSQL_COMPRESS_ZLIB => {}
                        mysql::MYSQL_COMPRESS_ZSTD => {}
                        _ => {
                            return Err(ReplicationError::new(format!(
                                "invalid compressed type {} != ({}|{})",
                                self.compression,
                                mysql::MYSQL_COMPRESS_ZLIB,
                                mysql::MYSQL_COMPRESS_ZSTD
                            )))
                        }
                    }
                }
                self.compressed_reader_active = true;
            }
        }

        Ok(vec![])
    }
    /*
    func (c *Conn) ReadPacketReuseMem(dst []byte) ([]byte, error) {
        if c.Compression != MYSQL_COMPRESS_NONE {
            if !c.compressedReaderActive {
                if uncompressedLength > 0 {
                    var err error
                    switch c.Compression {
                    case MYSQL_COMPRESS_ZLIB:
                        c.compressedReader, err = zlib.NewReader(c.reader)
                    case MYSQL_COMPRESS_ZSTD:
                        c.compressedReader, err = zstd.NewReader(c.reader)
                    }
                    if err != nil {
                        return nil, err
                    }
                }
                c.compressedReaderActive = true
            }
        }

        if c.compressedReader != nil {
            if err := c.ReadPacketTo(buf, c.compressedReader); err != nil {
                return nil, errors.Trace(err)
            }
        } else {
            if err := c.ReadPacketTo(buf, c.reader); err != nil {
                return nil, errors.Trace(err)
            }
        }

        readBytes := buf.Bytes()
        readSize := len(readBytes)
        var result []byte
        if len(dst) > 0 {
            result = append(dst, readBytes...)
            // if read block is big, do not cache buf any more
            if readSize > utils.TooBigBlockSize {
                buf = nil
            }
        } else {
            if readSize > utils.TooBigBlockSize {
                // if read block is big, use read block as result and do not cache buf any more
                result = readBytes
                buf = nil
            } else {
                result = append(dst, readBytes...)
            }
        }

        return result, nil
    }
    */
    fn _copy_n<R: Read, W: Write>(
        &self,
        std: &mut W,
        src: &mut R,
        n: usize,
    ) -> Result<usize, ReplicationError> {
        let mut n = n;
        let mut written = 0_usize;
        while n > 0 {
            let mut bcap = self._copy_n_buf.capacity();
            if bcap > n {
                bcap = n
            }
            let mut buf = Vec::with_capacity(bcap);
            let _ = src.read_exact(&mut buf)?;

            let wr = std.write(&buf)?;
            written += wr;

            if n > buf.len() {
                break;
            } else {
                n -= buf.len();
            }
        }

        Ok(written)
    }

    pub fn read_packet_to<R: Read, W: Write>(
        &mut self,
        w: &mut W,
        r: &mut R,
    ) -> Result<(), ReplicationError> {
        MysqlError::ErrBadConn.to_string();
        let _ = r.read_exact(&mut self._header).map_err(|e| {
            ReplicationError::new(format!(
                "{}. io.ReadFull(header) failed. err: {}",
                MysqlError::ErrBadConn.to_string(),
                e.to_string()
            ))
        })?;

        let length = (self._header[0] as u32
            | (self._header[1] as u32) << 8
            | (self._header[2] as u32) << 16) as usize;
        let sequence = self._header[3];

        if sequence != self.sequence {
            return Err(ReplicationError::new(format!(
                "invalid sequence {} != {}",
                sequence, self.sequence
            )));
        }
        self.sequence += 1;

        let n = self._copy_n(w, r, length).map_err(|e| {
            ReplicationError::new(format!(
                "{}. io.CopyN failed. err {}, expected {}",
                MysqlError::ErrBadConn.to_string(),
                e.to_string(),
                length
            ))
        })?;

        if n != length {
            return Err(ReplicationError::new(format!(
                "io.CopyN failed(n != length). {} bytes copied, while {} expected",
                n, length
            )));
        } else {
            if length < mysql::MAX_PAYLOAD_LEN {
                return Ok(());
            }

            self.read_packet_to(w, r).map_err(|e| {
                ReplicationError::new(format!("{} ReadPacketTo failed", e.to_string()))
            })?;
        }

        Ok(())
    }

    // WritePacket: data already has 4 bytes header
    // will modify data inplace
    pub fn write_packet(&mut self, mut data: &mut [u8]) -> Result<(), ReplicationError> {
        let mut length = data.len() - 4;
        while length >= mysql::MAX_PAYLOAD_LEN {
            data[0] = 0xff;
            data[1] = 0xff;
            data[2] = 0xff;
            data[3] = self.sequence;

            let write_len = 4 + mysql::MAX_PAYLOAD_LEN;
            let n = self
                ._conn
                .as_mut()
                .ok_or(ReplicationError::new("conn is none".to_string()))?
                .write(&data[..write_len])
                .map_err(|e| {
                    ReplicationError::new(format!(
                        "{}. Write(payload portion) failed. err {}",
                        MysqlError::ErrBadConn.to_string(),
                        e.to_string()
                    ))
                })?;
            if n != write_len {
                return Err(ReplicationError::new(format!(
                    "{}. Write(payload portion) failed. only {} bytes written, while {} expected",
                    MysqlError::ErrBadConn.to_string(),
                    n,
                    write_len
                )));
            } else {
                self.sequence += 1;
                length -= mysql::MAX_PAYLOAD_LEN;
                data = &mut data[write_len..]
            }
        }

        data[0] = length as u8;
        data[1] = (length >> 8) as u8;
        data[2] = (length >> 16) as u8;
        data[3] = self.sequence;

        match self.compression {
            mysql::MYSQL_COMPRESS_NONE => {
                let n = self
                    ._conn
                    .as_mut()
                    .ok_or(ReplicationError::new("conn is none".to_string()))?
                    .write(data)
                    .map_err(|e| {
                        ReplicationError::new(format!(
                            "{}. Write failed. err {}",
                            MysqlError::ErrBadConn.to_string(),
                            e.to_string()
                        ))
                    })?;

                if n != data.len() {
                    return Err(ReplicationError::new(format!(
                        "{}. Write failed. only {} bytes written, while {} expected",
                        MysqlError::ErrBadConn.to_string(),
                        n,
                        data.len()
                    )));
                }
            }
            mysql::MYSQL_COMPRESS_ZLIB | mysql::MYSQL_COMPRESS_ZSTD => {
                let n = self._write_compressed(data).map_err(|e| {
                    ReplicationError::new(format!(
                        "{}. Write failed. err {}",
                        MysqlError::ErrBadConn.to_string(),
                        e.to_string()
                    ))
                })?;

                if n != data.len() {
                    return Err(ReplicationError::new(format!(
                        "{}. Write failed. only {} bytes written, while {} expected",
                        MysqlError::ErrBadConn.to_string(),
                        n,
                        data.len()
                    )));
                }

                // @todo
                // c.compressedReader = nil
                self.compressed_reader_active = false;
            }
            _ => {
                return Err(ReplicationError::new(format!(
                    "{}. Write failed. Unsuppored compression algorithm set",
                    MysqlError::ErrBadConn.to_string(),
                )));
            }
        }

        self.sequence += 1;

        Ok(())
    }

    fn _write_compressed(&mut self, data: &mut [u8]) -> Result<usize, ReplicationError> {
        match self.compression {
            mysql::MYSQL_COMPRESS_ZLIB => self._write_compressed_zlib(data),
            mysql::MYSQL_COMPRESS_ZSTD => self._write_compressed_zstd(data),
            _ => Err(ReplicationError::new(format!(
                "can't found compression type. compression value: {}",
                self.compression
            ))),
        }
    }

    fn _write_compressed_zlib(&mut self, mut data: &mut [u8]) -> Result<usize, ReplicationError> {
        let (mut compressed_length, mut uncompressed_length) = (0_usize, 0_usize);
        let (mut payload, mut compressed_packet) = (Vec::<u8>::new(), Vec::<u8>::new());

        let min_compress_length = 50_usize;
        let mut compressed_header = [0, 0, 0, 0, 0, 0, 0];

        let mut n = {
            let mut w = flate2::write::ZlibEncoder::new(&mut payload, flate2::Compression::new(9));
            let mut n = 0_usize;
            if data.len() > min_compress_length {
                uncompressed_length = data.len();
                n = w.write(&data)?;
                drop(w);
            }

            n
        };

        if data.len() > min_compress_length {
            compressed_length = payload.len();
        } else {
            compressed_length = data.len()
        }

        self.compressed_sequence = 0;
        compressed_header[0] = compressed_length as u8;
        compressed_header[1] = (compressed_length >> 8) as u8;
        compressed_header[2] = (compressed_length >> 16) as u8;
        compressed_header[3] = self.compressed_sequence;
        compressed_header[4] = (uncompressed_length) as u8;
        compressed_header[5] = (uncompressed_length >> 8) as u8;
        compressed_header[6] = (uncompressed_length >> 16) as u8;

        let _ = std::io::Write::write(&mut compressed_packet, &compressed_header)?;
        self.compressed_sequence += 1;

        if data.len() > min_compress_length {
            let _ = std::io::Write::write(&mut compressed_packet, &payload)?;
        } else {
            n = std::io::Write::write(&mut compressed_packet, &data)?;
        }

        let _ = self
            ._conn
            .as_mut()
            .ok_or(ReplicationError::new("conn is none".to_string()))?
            .write(&compressed_packet)?;

        Ok(n)
    }

    fn _write_compressed_zstd(&mut self, mut data: &mut [u8]) -> Result<usize, ReplicationError> {
        let (mut compressed_length, mut uncompressed_length) = (0_usize, 0_usize);
        let (mut payload, mut compressed_packet) = (Vec::<u8>::new(), Vec::<u8>::new());

        let min_compress_length = 50_usize;
        let mut compressed_header = [0, 0, 0, 0, 0, 0, 0];
        let mut w = zstd::stream::write::Encoder::new(&mut payload, 0).unwrap();

        let mut n = {
            let mut n = 0_usize;
            if data.len() > min_compress_length {
                uncompressed_length = data.len();
                n = w.write(&data)?;
                drop(w);
            }
            n
        };

        if data.len() > min_compress_length {
            compressed_length = payload.len();
        } else {
            compressed_length = data.len()
        }

        self.compressed_sequence = 0;
        compressed_header[0] = compressed_length as u8;
        compressed_header[1] = (compressed_length >> 8) as u8;
        compressed_header[2] = (compressed_length >> 16) as u8;
        compressed_header[3] = self.compressed_sequence;
        compressed_header[4] = (uncompressed_length) as u8;
        compressed_header[5] = (uncompressed_length >> 8) as u8;
        compressed_header[6] = (uncompressed_length >> 16) as u8;

        let _ = std::io::Write::write(&mut compressed_packet, &compressed_header)?;
        self.compressed_sequence += 1;

        if data.len() > min_compress_length {
            let _ = std::io::Write::write(&mut compressed_packet, &payload)?;
        } else {
            n = std::io::Write::write(&mut compressed_packet, &data)?;
        }

        let _ = self
            ._conn
            .as_mut()
            .ok_or(ReplicationError::new("conn is none".to_string()))?
            .write(&compressed_packet)?;

        Ok(n)
    }

    // WriteClearAuthPacket: Client clear text authentication packet
    // http://dev.mysql.com/doc/internals/en/connection-phase-packets.html#packet-Protocol::AuthSwitchResponse
    pub fn write_clear_auth_packet(&mut self, password: &str) -> Result<(), ReplicationError> {
        // Calculate the packet length and add a tailing 0
        let pkt_len = password.len() + 1;
        let mut data = vec![0_u8; 4 + pkt_len];

        // Add the clear password [null terminated string]
        {
            let password = password.as_bytes();
            let data = &mut data[4..4 + password.len()];
            data.copy_from_slice(password);
        }

        data[4 + pkt_len - 1] = 0x00;

        let _ = self
            .write_packet(&mut data)
            .map_err(|e| ReplicationError::new(format!("{} WritePacket failed", e.to_string())))?;

        Ok(())
    }

    // WritePublicKeyAuthPacket: Caching sha2 authentication. Public key request and send encrypted password
    // http://dev.mysql.com/doc/internals/en/connection-phase-packets.html#packet-Protocol::AuthSwitchResponse
    pub fn write_public_key_auth_packet(
        &mut self,
        password: &str,
        cipher: &[u8],
    ) -> Result<(), ReplicationError> {
        // request public key
        let mut data = vec![0_u8; 4 + 1];
        data[4] = 2; // cachingSha2PasswordRequestPublicKey

        let _ = self.write_packet(&mut data).map_err(|e| {
            ReplicationError::new(format!("{} WritePacket(single byte) failed", e.to_string()))
        })?;

        let data = self
            .read_packet()
            .map_err(|e| ReplicationError::new(format!("{} ReadPacket failed", e.to_string())))?;

        let pub_key = openssl::pkey::PKey::public_key_from_pem(&data).map_err(|e| {
            ReplicationError::new(format!("{}.  public_key_from_pem failed", e.to_string()))
        })?;

        let mut plain = vec![0_u8; password.len() + 1];
        {
            plain.copy_from_slice(password.as_bytes());
        }

        for i in 0..plain.len() {
            let j = i % cipher.len();
            plain[i] ^= cipher[j];
        }
        let sha1_digest = openssl::sha::sha1(&plain);
        let rsa = pub_key.rsa()?;
        let mut encrypted_data = Vec::<u8>::new();
        let _ = rsa.public_encrypt(
            &sha1_digest,
            &mut encrypted_data,
            openssl::rsa::Padding::PKCS1_OAEP,
        )?;

        let mut data = vec![0_u8; 4 + encrypted_data.len()];
        {
            let data = &mut data[4..4 + encrypted_data.len()];
            data.copy_from_slice(&encrypted_data);
        }

        self.write_packet(&mut data).map_err(|e| {
            ReplicationError::new(format!("{}.  WritePacket failed", e.to_string()))
        })?;

        Ok(())
    }

    pub fn write_encrypted_password(
        &mut self,
        password: &str,
        seed: &[u8],
        pub_key: &openssl::pkey::PKey<openssl::pkey::Public>,
    ) -> Result<(), ReplicationError> {
        let enc = mysql::encrypt_password(password, seed, pub_key).map_err(|e| {
            ReplicationError::new(format!("{}. EncryptPassword failed", e.to_string()))
        })?;

        self.write_auth_switch_packet(&enc, false).map_err(|e| {
            ReplicationError::new(format!("{}. WriteAuthSwitchPacket failed", e.to_string()))
        })?;

        Ok(())
    }

    // http://dev.mysql.com/doc/internals/en/connection-phase-packets.html#packet-Protocol::AuthSwitchResponse
    pub fn write_auth_switch_packet(
        &mut self,
        auth_data: &[u8],
        add_nul: bool,
    ) -> Result<(), ReplicationError> {
        let mut pkt_len = 4 + auth_data.len();
        if add_nul {
            pkt_len += 1;
        }
        let mut data = vec![0_u8; pkt_len];

        // Add the auth data [EOF]
        {
            let data = &mut data[4..4 + auth_data.len()];
            data.copy_from_slice(&auth_data);
        }
        if add_nul {
            data[pkt_len - 1] = 0x00;
        }

        let _ = self
            .write_packet(&mut data)
            .map_err(|e| ReplicationError::new(format!("{}. WritePacket failed", e.to_string())))?;

        Ok(())
    }

    pub fn reset_sequence(&mut self) {
        self.sequence = 0;
    }

    pub fn close(&mut self) -> Result<(), ReplicationError> {
        self.sequence = 0;
        if let Some(mut conn) = self._conn.take() {
            let _ = conn.shutdown(net::Shutdown::Both);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use std::io::{BufReader, Read};

    #[test]
    fn option_task_test() {
        #[derive(Debug)]
        struct A(i64);
        #[derive(Debug)]
        struct B {
            a: Option<A>,
        }

        let mut b = B { a: Some(A(1)) };
        println!("{:?}", &b);

        if let Some(a) = b.a.take() {
            println!("{:?}", &a)
        }
        println!("{:?}", &b);
    }

    #[test]
    fn copy_from_slice_test() {
        let mut data: Vec<u8> = vec![0; 20];
        let password: &[u8] = b"password";

        {
            let data = &mut data[4..4 + password.len()];
            data.copy_from_slice(password);
        }

        println!("{:?}", data);
    }

    fn write_data(mut data: &mut [u8]) {
        data[0] = 100;
        data[1] = 101;
        println!("write_data data: {:?}", data);

        data = &mut data[5..];
        println!("write_data data: {:?}", data);
        data[0] = 200;
        data[1] = 201;
        println!("write_data data: {:?}", data);
    }

    #[test]
    fn write_data_test() -> Result<(), ReplicationError> {
        let mut data = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        println!("write_data_test data: {:?}", &data);
        write_data(&mut data);
        println!("write_data_test data: {:?}", &data);

        Ok(())
    }

    #[test]
    fn read_arr() -> Result<(), ReplicationError> {
        #[derive(Debug)]
        struct A {
            pub name: [u8; 4],
        }

        let mut a = A { name: [0, 0, 0, 0] };
        println!("{:?}", &a);
        let b = "1234567890".as_bytes();
        let mut read_buf = BufReader::new(b);
        let _ = read_buf.read_exact(&mut a.name)?;
        println!("{:?}", &a);

        Ok(())
    }
}
