use backtrace::Backtrace;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use ring::digest;
use rsa::rand_core::OsRng;
use rsa::Oaep;
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn pstack() -> String {
    let bt = Backtrace::new();
    format!("{:?}", bt)
}

pub fn calc_password(scramble: &[u8], password: &[u8]) -> Vec<u8> {
    if password.len() == 0 {
        return vec![];
    }

    // stage1Hash = SHA1(password)
    let mut ctx = digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY);
    ctx.update(password);
    let stage1 = ctx.finish();
    let stage1 = stage1.as_ref();

    // scrambleHash = SHA1(scramble + SHA1(stage1Hash))
    // inner Hash
    let mut ctx2 = digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY);
    ctx2.update(stage1);
    let hash = ctx2.finish();
    let hash = hash.as_ref();

    // outer Hash
    let mut ctx3 = digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY);
    ctx3.update(scramble);
    ctx3.update(hash);
    let mut scramble = ctx3.finish().as_ref().to_vec();

    for i in 0..scramble.len() {
        scramble[i] ^= stage1[i]
    }

    scramble
}

// CalcCachingSha2Password: Hash password using MySQL 8+ method (SHA256)
pub fn calc_caching_sha2_password(scramble: &[u8], password: &[u8]) -> Vec<u8> {
    if password.len() == 0 {
        return vec![];
    }

    // XOR(SHA256(password), SHA256(SHA256(SHA256(password)), scramble))

    let mut ctx = digest::Context::new(&digest::SHA256);
    ctx.update(password);
    let message1 = ctx.finish();
    let mut message1 = message1.as_ref().to_vec();

    let mut ctx2 = digest::Context::new(&digest::SHA256);
    ctx2.update(&message1);
    let message1_hash = ctx2.finish();
    let message1_hash = message1_hash.as_ref();

    let mut ctx3 = digest::Context::new(&digest::SHA256);
    ctx3.update(message1_hash);
    ctx3.update(scramble);
    let message2 = ctx3.finish();
    let message2 = message2.as_ref();

    for i in 0..message1.len() {
        message1[i] ^= message2[i]
    }

    message1
}

pub fn encrypt_password(
    password: &str,
    seed: &[u8],
    pub_key: &rsa::RsaPublicKey,
) -> rsa::Result<Vec<u8>> {
    let mut plain = password.as_bytes().to_vec();
    plain.push(0);

    for i in 0..plain.len() {
        let j = i % seed.len();
        plain[i] ^= seed[j]
    }

    let mut rng = OsRng;
    let padding = Oaep::new::<sha1::Sha1>();
    pub_key.encrypt(&mut rng, padding, &plain)
}

pub fn decompress_mariadb_data(data: &[u8]) -> io::Result<Vec<u8>> {
    // algorithm always 0=zlib
    // algorithm := (data[pos] & 0x07) >> 4
    let header_size = data[0] as usize & 0x07;
    let compressed_data = &data[(header_size + 1)..];
    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut uncompressed_data = Vec::<u8>::new();
    let _ = decoder.read_to_end(&mut uncompressed_data)?;

    Ok(uncompressed_data)
}

// AppendLengthEncodedInteger: encodes a uint64 value and appends it to the given bytes slice
#[allow(arithmetic_overflow)]
pub fn append_length_encoded_integer(b: &[u8], n: u64) -> Vec<u8> {
    let mut rs = b.to_vec();
    match n {
        tmp_n if tmp_n <= 250 => rs.push(tmp_n as u8),
        tmp_n if tmp_n <= 0xffff => rs.extend(vec![0xfc, tmp_n as u8, (tmp_n >> 8) as u8]),
        tmp_n if tmp_n <= 0xffffff => rs.extend(vec![
            0xfd,
            tmp_n as u8,
            (tmp_n >> 8) as u8,
            (tmp_n >> 16) as u8,
        ]),
        _ => rs.extend(vec![
            0xfe,
            n as u8,
            (n >> 8) as u8,
            (n >> 16) as u8,
            (n >> 24) as u8,
            (n >> 32) as u8,
            (n >> 40) as u8,
            (n >> 48) as u8,
            (n >> 56) as u8,
        ]),
    }

    rs
}

pub fn random_buf(size: usize) -> Vec<u8> {
    let mut buf = vec![0_u8; size];
    let mut rng = StdRng::seed_from_u64(chrono::Local::now().timestamp() as u64);
    let (min, max) = (30_u8, 137);
    for i in 0..size {
        buf[i] = min + rng.gen_range(0..(max - min))
    }

    buf
}

// FixedLengthInt: little endian
pub fn fixed_length_int(buf: &[u8]) -> u64 {
    let mut num = 0_u64;
    for (i, b) in buf.iter().enumerate() {
        let b = *b as u64;
        let move_size = i as u64 * 8;

        num |= b << move_size
    }

    num
}

// BFixedLengthInt: big endian
pub fn bfixed_length_int(buf: &[u8]) -> u64 {
    let mut num = 0_u64;
    for (i, b) in buf.iter().enumerate() {
        let b = *b as u64;
        let move_size = ((buf.len() - i - 1) * 8) as u64;
        num |= b << move_size
    }

    num
}

pub fn length_encoded_int(b: &[u8]) -> (u64, bool, usize) {
    if b.len() == 0 {
        return (0, true, 0);
    }

    match b[0] {
        // 251: NULL
        0xfb => return (0, true, 1),
        0xfc => {
            // 252: value of following 2
            let data1 = b[1] as u64;
            let data2 = b[2] as u64;
            return (data1 | (data2 << 8), false, 3);
        }
        0xfd => {
            // 253: value of following 3
            let data1 = b[1] as u64;
            let data2 = b[2] as u64;
            let data3 = b[3] as u64;
            return (data1 | (data2 << 8) | (data3 << 16), false, 4);
        }
        0xfe => {
            let data1 = b[1] as u64;
            let data2 = b[2] as u64;
            let data3 = b[3] as u64;
            let data4 = b[4] as u64;
            let data5 = b[5] as u64;
            let data6 = b[6] as u64;
            let data7 = b[7] as u64;
            let data8 = b[8] as u64;
            return (
                data1
                    | data2 << 8
                    | data3 << 16
                    | data4 << 24
                    | data5 << 32
                    | data6 << 40
                    | data7 << 48
                    | data8 << 56,
                false,
                9,
            );
        }
        _ => {}
    }

    // 0-250: value of first byte
    (b[0] as u64, false, 1)
}

#[allow(arithmetic_overflow)]
pub fn put_length_encoded_int(n: u64) -> Vec<u8> {
    match n {
        tmp_n if n <= 250 => vec![tmp_n as u8],
        tmp_n if n <= 0xffff => vec![0xfc, tmp_n as u8, (tmp_n >> 8) as u8],
        tmp_n if n <= 0xffffff => vec![0xfd, tmp_n as u8, (tmp_n >> 8) as u8, (tmp_n >> 16) as u8],
        tmp_n if n <= u64::MAX => {
            vec![
                0xfe,
                tmp_n as u8,
                (tmp_n >> 8) as u8,
                (tmp_n >> 16) as u8,
                (tmp_n >> 24) as u8,
                (tmp_n >> 32) as u8,
                (tmp_n >> 40) as u8,
                (tmp_n >> 48) as u8,
                (tmp_n >> 56) as u8,
            ]
        }
        _ => Vec::<u8>::new(),
    }
}

// LengthEncodedString returns the string read as a bytes slice, whether the value is NULL,
// the number of bytes read and an error, in case the string is longer than
// the input slice
pub fn length_encoded_string(b: &[u8]) -> io::Result<(Vec<u8>, bool, usize)> {
    // Get length
    let (num, is_null, mut n) = length_encoded_int(b);
    if num < 1 {
        return Ok((vec![], is_null, n));
    }

    n += num as usize;

    // Check data length
    if b.len() >= n {
        let rs = b[(n - num as usize)..n].to_vec();

        return Ok((rs, false, n));
    }

    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
}

pub fn skip_length_encoded_string(b: &[u8]) -> io::Result<usize> {
    // Get length
    let (num, _, mut n) = length_encoded_int(b);
    if num < 1 {
        return Ok(n);
    }

    n += num as usize;

    // Check data length
    if b.len() >= n {
        return Ok(n);
    }

    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
}

pub fn put_length_encoded_string(b: &[u8]) -> Vec<u8> {
    let mut data = Vec::<u8>::with_capacity(b.len() + 9);
    data.extend(put_length_encoded_int(b.len() as u64));
    data.extend(b);

    data
}

#[allow(arithmetic_overflow)]
pub fn uint16_to_bytes(n: u16) -> Vec<u8> {
    vec![n as u8, (n >> 8) as u8]
}

#[allow(arithmetic_overflow)]
pub fn uint32_to_bytes(n: u32) -> Vec<u8> {
    vec![n as u8, (n >> 8) as u8, (n >> 16) as u8, (n >> 24) as u8]
}

#[allow(arithmetic_overflow)]
pub fn uint64_to_bytes(n: u64) -> Vec<u8> {
    vec![
        n as u8,
        (n >> 8) as u8,
        (n >> 16) as u8,
        (n >> 24) as u8,
        (n >> 32) as u8,
        (n >> 40) as u8,
        (n >> 48) as u8,
        (n >> 56) as u8,
    ]
}

pub fn format_binary_date(n: usize, data: &[u8]) -> io::Result<Vec<u8>> {
    match n {
        0 => Ok("0000-00-00".as_bytes().to_vec()),
        4 => {
            let mut rdr = Cursor::new(data);
            let read_data = rdr.read_u16::<LittleEndian>()?;
            Ok(format!("{:04}-{:02}-{:02}", read_data, data[2], data[3])
                .as_bytes()
                .to_vec())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("invalid date packet length {}", n),
        )),
    }
}

pub fn format_binary_datetime(n: usize, data: &[u8]) -> io::Result<Vec<u8>> {
    match n {
        0 => Ok("0000-00-00 00:00:00".as_bytes().to_vec()),
        4 => {
            let mut rdr = Cursor::new(data);
            let read_data = rdr.read_u16::<LittleEndian>()?;
            Ok(
                format!("{:04}-{:02}-{:02} 00:00:00", read_data, data[2], data[3])
                    .as_bytes()
                    .to_vec(),
            )
        }
        7 => {
            let mut rdr = Cursor::new(data);
            let read_data = rdr.read_u16::<LittleEndian>()?;

            Ok(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                read_data, data[2], data[3], data[4], data[5], data[6]
            )
            .as_bytes()
            .to_vec())
        }
        11 => {
            let mut rdr = Cursor::new(data);
            let read_data1 = rdr.read_u16::<LittleEndian>()?;
            rdr.seek(SeekFrom::Current(5))?;
            let read_data6 = rdr.read_u32::<LittleEndian>()?;

            Ok(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                read_data1, data[2], data[3], data[4], data[5], data[6], read_data6
            )
            .as_bytes()
            .to_vec())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("invalid date packet length {}", n),
        )),
    }
}

pub fn format_binary_time(n: usize, data: &[u8]) -> io::Result<Vec<u8>> {
    if n == 0 {
        return Ok("0000-00-00".as_bytes().to_vec());
    }

    let sign = if data[0] == 1 { '-' } else { char::from(0) };

    match n {
        8 => {
            let data2 = data[1] as u16 * 24 + data[5] as u16;
            Ok(
                format!("{}{:02}:{:02}:{:02}", sign, data2, data[6], data[7])
                    .as_bytes()
                    .to_vec(),
            )
        }
        12 => {
            let data2 = data[1] as u16 * 24 + data[5] as u16;

            let mut rdr = Cursor::new(data);
            rdr.seek(SeekFrom::Current(8))?;
            let read_data5 = rdr.read_u32::<LittleEndian>()?;

            Ok(format!(
                "{}{:02}:{:02}:{:02}.{:06}",
                sign, data2, data[6], data[7], read_data5
            )
            .as_bytes()
            .to_vec())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("invalid date packet length {}", n),
        )),
    }
}

pub const DONTESCAPE: u8 = 255;
lazy_static! {
    pub static ref ENCODE_MAP: Vec<u8> = {
        let mut encode_ref = HashMap::<u8, u8>::new();
        encode_ref.insert('\x00' as u8, '0' as u8);
        encode_ref.insert('\'' as u8, '\'' as u8);
        encode_ref.insert('"' as u8, '"' as u8);
        encode_ref.insert(8, 'b' as u8); // \b 退格
        encode_ref.insert('\n' as u8, 'n' as u8);
        encode_ref.insert('\r' as u8, 'r' as u8);
        encode_ref.insert('\t' as u8, 't' as u8);
        encode_ref.insert(26, 'Z' as u8); // ctl-Z 代替
        encode_ref.insert('\\' as u8, '\\' as u8);
        let mut datas = vec![DONTESCAPE; 256];

        for i in 0..datas.len() {
            let index = i as u8;
            if let Some(v) = encode_ref.get(&index) {
                datas[index as usize] = *v
            }
        }

        datas
    };
}

// Escape: only support utf-8
pub fn escape(sql: &str) -> String {
    let mut dest = String::with_capacity(sql.len() * 2);
    for w in sql.as_bytes() {
        let c = ENCODE_MAP[(*w) as usize];
        if c == DONTESCAPE {
            dest.push(char::from(*w));
        } else {
            dest.push('\\');
            dest.push(char::from(c));
        }
    }

    dest
}

pub fn get_net_proto(addr: &str) -> String {
    if addr.contains("/") {
        String::from("unix")
    } else {
        String::from("tcp")
    }
}

// ErrorEqual returns a boolean indicating whether err1 is equal to err2.
pub fn error_equal(err1: Box<dyn std::error::Error>, err2: Box<dyn std::error::Error>) -> bool {
    if err1.type_id().eq(&err2.type_id()) {
        err1.to_string() == err2.to_string()
    } else {
        false
    }
}

pub fn compare_server_versions(a: &str, b: &str) -> io::Result<Ordering> {
    let a_version = lenient_semver::parse(a).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("cannot parse {} as semver: {}", a, e.to_string()),
        )
    })?;

    let b_version = lenient_semver::parse(b).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("cannot parse {:?} as semver: {}", b, e.to_string()),
        )
    })?;

    Ok(a_version.cmp(&b_version))
}

#[cfg(test)]
mod tests {
    use crate::error::ReplicationError;
    use std::any::Any;
    use std::io;

    #[test]
    fn d() {
        println!("{}", (u64::MAX - 100) as i64);
    }

    #[test]
    fn c() {
        println!("{}", 0xffffffffffffffff);
        println!("{}", u64::MAX)
    }

    #[test]
    fn b() {
        let error1 = io::Error::new(io::ErrorKind::Other, "Custom error");
        let error2 = io::Error::new(io::ErrorKind::Other, "Custom error");
        assert_eq!(error1.type_id().eq(&error2.type_id()), true);

        let error3 = ReplicationError::NormalError("ooo".to_string());
        assert_eq!(error1.type_id().eq(&error3.type_id()), false);
    }

    #[test]
    fn a() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let v2 = &v1[1..1];
        println!("{:?}", v1);
        println!("{:?}", v2);
    }
}
