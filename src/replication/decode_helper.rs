use crate::error::ReplicationError;
use crate::mysql;

use crate::common::row_fields::{DecodeDatetime, DecodeDecimal};
use crate::replication::{self, FracTime};
use bigdecimal::BigDecimal;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime};
use std::io::{Cursor, Seek, SeekFrom};
use std::str::FromStr;

pub fn is_bit_set(bitmap: &[u8], i: isize) -> bool {
    let b = bitmap[i as usize >> 3] as usize;
    let c = 1_usize << (i as usize & 7);
    (b & c) > 0
}

pub fn is_bit_set_incr(bitmap: &[u8], i: &mut isize) -> bool {
    let v = is_bit_set(bitmap, *i);
    *i += 1;
    v
}

pub fn decode_string(data: &[u8], length: isize) -> Result<(String, isize), ReplicationError> {
    let mut rdr = Cursor::new(&data);

    if length < 256 {
        let length = rdr.read_u8()?;
        let n = length + 1;
        Ok((
            String::from_utf8_lossy(&data[rdr.position() as usize..n as usize]).to_string(),
            n as isize,
        ))
    } else {
        let length = rdr.read_u16::<LittleEndian>()?;
        let n = length + 2;
        Ok((
            String::from_utf8_lossy(&data[rdr.position() as usize..n as usize]).to_string(),
            n as isize,
        ))
    }
}

// ref: https://github.com/mysql/mysql-server/blob/a9b0c712de3509d8d08d3ba385d41a4df6348775/strings/decimal.c#L137
pub const DIGITS_PER_INTEGER: isize = 9;
lazy_static! {
    pub static ref COMPRESSED_BYTES: Vec<isize> = vec![0_isize, 1, 1, 2, 2, 3, 3, 4, 4, 4];
}

pub fn decode_decimal_decompress_value(comp_indx: isize, data: &[u8], mask: u8) -> (isize, u32) {
    let size = COMPRESSED_BYTES[comp_indx as usize];
    let value = match size {
        0 => 0,
        1 => (data[0] ^ mask) as u32,
        2 => (data[1] ^ mask) as u32 | ((data[0] ^ mask) as u32) << 8,
        3 => {
            (data[2] ^ mask) as u32
                | ((data[1] ^ mask) as u32) << 8
                | ((data[0] ^ mask) as u32) << 16
        }
        4 => {
            (data[3] ^ mask) as u32
                | ((data[2] ^ mask) as u32) << 8
                | ((data[1] ^ mask) as u32) << 16
                | ((data[0] ^ mask) as u32) << 24
        }
        _ => 0,
    };

    (size, value)
}

lazy_static! {
    pub static ref ZEROS: Vec<u8> = vec![48_u8, 48, 48, 48, 48, 48, 48, 48, 48];
}

#[allow(arithmetic_overflow)]
pub fn decode_decimal(
    data: &[u8],
    precision: isize,
    decimals: isize,
    use_decimal: bool,
) -> Result<(DecodeDecimal, isize), ReplicationError> {
    // see python mysql replication and https://github.com/jeremycole/mysql_binlog
    let integral = precision - decimals;
    let uncomp_integral = integral / DIGITS_PER_INTEGER;
    let uncomp_fractional = decimals / DIGITS_PER_INTEGER;
    let comp_integral = integral - (uncomp_integral * DIGITS_PER_INTEGER);
    let comp_fractional = decimals - (uncomp_fractional * DIGITS_PER_INTEGER);
    let bin_size = uncomp_integral * 4
        + COMPRESSED_BYTES[comp_integral as usize]
        + uncomp_fractional * 4
        + COMPRESSED_BYTES[comp_fractional as usize];

    let buf = data[..bin_size as usize].to_vec();

    // must copy the data for later change
    let data = buf;
    let mut rdr = Cursor::new(&data);

    // Support negative
    // The sign is encoded in the high bit of the the byte
    // But this bit can also be used in the value
    let value = rdr.read_u8()? as u32;
    let mut res = String::with_capacity(precision as usize + 2);
    let mask = if value & 0x80 == 0 {
        res.push_str("-");
        ((1_u64 << 32) - 1) as u32
    } else {
        0_u32
    };

    let old_pos = rdr.position();
    let mut data = data;
    // clear sign
    data[0] ^= 0x80;
    let mut rdr = Cursor::new(&data);
    rdr.seek(SeekFrom::Start(old_pos))?;

    let (pos, mut value) = decode_decimal_decompress_value(comp_integral, &data, mask as u8);
    let mut zero_leading = if value != 0 {
        res.push_str(&value.to_string());
        false
    } else {
        true
    };

    rdr.seek(SeekFrom::Start(pos as u64))?;
    for _ in 0..uncomp_integral {
        value = rdr.read_u32::<BigEndian>()? ^ mask;
        if zero_leading {
            if value != 0 {
                zero_leading = false;
                res.push_str(&value.to_string());
            }
        } else {
            let to_write = value.to_string();
            let stop = DIGITS_PER_INTEGER as usize - to_write.len();
            for &v in &ZEROS[..stop] {
                res.push(char::from(v));
            }
            res.push_str(&to_write);
        }
    }

    if zero_leading {
        res.push_str("0");
    }

    if (rdr.position() as usize) < data.len() {
        res.push_str(".");

        for _ in 0..uncomp_fractional {
            value = rdr.read_u32::<BigEndian>()? ^ mask;
            let to_write = value.to_string();
            let stop = DIGITS_PER_INTEGER as usize - to_write.len();
            for &v in &ZEROS[..stop] {
                res.push(char::from(v));
            }
            res.push_str(&to_write);
        }

        let (size, value) = decode_decimal_decompress_value(
            comp_fractional,
            &data[rdr.position() as usize..],
            mask as u8,
        );
        if size > 0 {
            let to_write = value.to_string();
            let padding = comp_fractional - to_write.len() as isize;
            if padding > 0 {
                for &v in &ZEROS[..padding as usize] {
                    res.push(char::from(v));
                }
            }
            res.push_str(&to_write);
            rdr.seek(SeekFrom::Current(size as i64))?;
        }
    }

    if use_decimal {
        return Ok((
            DecodeDecimal::Decimal(BigDecimal::from_str(&res)?),
            rdr.position() as isize,
        ));
    }

    Ok(((DecodeDecimal::String(res)), rdr.position() as isize))
}

pub fn decode_bit(data: &[u8], nbits: isize, length: isize) -> Result<i64, ReplicationError> {
    let mut rdr = Cursor::new(data);
    if nbits > 1 {
        match length {
            1 => Ok(rdr.read_u8()? as i64),
            2 => Ok(rdr.read_u16::<BigEndian>()? as i64),
            3 => Ok(mysql::fixed_length_int(&data[0..3]) as i64),
            4 => Ok(rdr.read_u32::<BigEndian>()? as i64),
            5 => Ok(mysql::fixed_length_int(&data[0..5]) as i64),
            6 => Ok(mysql::fixed_length_int(&data[0..7]) as i64),
            7 => Ok(mysql::fixed_length_int(&data[0..7]) as i64),
            8 => Ok(rdr.read_i64::<BigEndian>()?),
            _ => Err(ReplicationError::new(format!(
                "invalid bit length {}",
                length
            ))),
        }
    } else {
        if length != 1 {
            Err(ReplicationError::new(format!(
                "invalid bit length {}",
                length
            )))
        } else {
            Ok(rdr.read_u8()? as i64)
        }
    }
}

pub fn little_decode_bit(
    data: &[u8],
    nbits: isize,
    length: isize,
) -> Result<i64, ReplicationError> {
    let mut rdr = Cursor::new(data);
    if nbits > 1 {
        match length {
            1 => Ok(rdr.read_u8()? as i64),
            2 => Ok(rdr.read_u16::<LittleEndian>()? as i64),
            3 => Ok(mysql::fixed_length_int(&data[0..3]) as i64),
            4 => Ok(rdr.read_u32::<LittleEndian>()? as i64),
            5 => Ok(mysql::fixed_length_int(&data[0..5]) as i64),
            6 => Ok(mysql::fixed_length_int(&data[0..7]) as i64),
            7 => Ok(mysql::fixed_length_int(&data[0..7]) as i64),
            8 => Ok(rdr.read_i64::<LittleEndian>()?),
            _ => Err(ReplicationError::new(format!(
                "invalid bit length {}",
                length
            ))),
        }
    } else {
        if length != 1 {
            Err(ReplicationError::new(format!(
                "invalid bit length {}",
                length
            )))
        } else {
            Ok(rdr.read_u8()? as i64)
        }
    }
}

pub fn decode_timestamp2(
    data: &[u8],
    dec: u16,
    timestamp_string_location: Option<chrono_tz::Tz>,
) -> Result<(DecodeDatetime, isize), ReplicationError> {
    // get timestamp binary length
    let n = (4 + (dec + 1) / 2) as isize;
    let mut rdr = Cursor::new(data);
    let sec = rdr.read_u32::<BigEndian>()?;
    let usec = match dec {
        1 | 2 => rdr.read_u8()? as i64 * 10000,
        3 | 4 => rdr.read_u16::<BigEndian>()? as i64 * 100,
        5 | 6 => mysql::bfixed_length_int(&data[rdr.position() as usize..7]) as i64,
        _ => 0,
    };

    if sec == 0 {
        return Ok((
            DecodeDatetime::String(replication::format_zero_time(usec as isize, dec as isize)),
            n,
        ));
    }

    Ok((
        DecodeDatetime::FracTime(FracTime {
            f_time: NaiveDateTime::from_timestamp_opt(sec as i64, usec as u32 * 1000).unwrap(),
            dec: dec as isize,
            timestamp_string_location: timestamp_string_location,
        }),
        n,
    ))
}

pub const DATETIMEF_INT_OFS: i64 = 0x8000000000;

pub fn decode_datetime2(
    data: &[u8],
    dec: u16,
) -> Result<(DecodeDatetime, isize), ReplicationError> {
    // get datetime binary length
    let n = (5 + (dec + 1) / 2) as isize;
    let int_part = mysql::bfixed_length_int(&data[0..5]) as i64 - DATETIMEF_INT_OFS;
    let mut frac = 0_i64;

    match dec {
        1 | 2 => {
            frac = data[5] as i64 * 10000;
        }
        3 | 4 => {
            let mut rdr = Cursor::new(data);
            rdr.seek(SeekFrom::Start(5))?;
            frac = rdr.read_u16::<BigEndian>()? as i64 * 100;
        }
        5 | 6 => {
            frac = mysql::bfixed_length_int(&data[5..8]) as i64;
        }
        _ => {}
    }

    if int_part == 0 {
        return Ok((
            DecodeDatetime::String(replication::format_zero_time(frac as isize, dec as isize)),
            n,
        ));
    }

    let mut tmp = (int_part << 24) + frac;
    // handle sign???
    if tmp < 0 {
        tmp = -tmp
    }

    // var secPart int64 = tmp % (1 << 24)
    let ymdhms = tmp >> 24;

    let ymd = ymdhms >> 17;
    let ym = ymd >> 5;
    let hms = ymdhms % (1 << 17);

    let day = (ymd % (1 << 5)) as isize;
    let month = (ym % 13) as isize;
    let year = (ym / 13) as isize;

    let second = (hms % (1 << 6)) as isize;
    let minute = ((hms >> 6) % (1 << 6)) as isize;
    let hour = (hms >> 12) as isize;

    // DATETIME encoding for nonfractional part after MySQL 5.6.4
    // https://dev.mysql.com/doc/internals/en/date-and-time-data-type-representation.html
    // integer value for 1970-01-01 00:00:00 is
    // year*13+month = 25611 = 0b110010000001011
    // day = 1 = 0b00001
    // hour = 0 = 0b00000
    // minute = 0 = 0b000000
    // second = 0 = 0b000000
    // integer value = 0b1100100000010110000100000000000000000 = 107420450816

    if int_part < 107420450816 {
        return Ok((
            DecodeDatetime::String(replication::format_before_unix_zero_time(
                year,
                month,
                day,
                hour,
                minute,
                second,
                frac as isize,
                dec as isize,
            )),
            n,
        ));
    }

    Ok((
        DecodeDatetime::FracTime(FracTime {
            f_time: NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                .unwrap()
                .and_hms_nano_opt(
                    hour as u32,
                    minute as u32,
                    second as u32,
                    frac as u32 * 1000,
                )
                .unwrap(),
            dec: dec as isize,
            timestamp_string_location: None,
        }),
        n,
    ))
}

pub const TIMEF_OFS: i64 = 0x800000000000;
pub const TIMEF_INT_OFS: i64 = 0x800000;

pub fn decode_time2(data: &[u8], dec: u16) -> Result<(String, isize), ReplicationError> {
    // time  binary length
    let n = (3 + (dec + 1) / 2) as isize;

    let mut frac = 0_i64;
    let (tmp, int_part) = match dec {
        1 | 2 => {
            let mut int_part = mysql::bfixed_length_int(&data[0..3]) as i64 - TIMEF_INT_OFS;
            frac = data[3] as i64;
            if int_part < 0 && frac != 0 {
                /*
                   Negative values are stored with reverse fractional part order,
                   for binary sort compatibility.

                     Disk value  intpart frac   Time value   Memory value
                     800000.00    0      0      00:00:00.00  0000000000.000000
                     7FFFFF.FF   -1      255   -00:00:00.01  FFFFFFFFFF.FFD8F0
                     7FFFFF.9D   -1      99    -00:00:00.99  FFFFFFFFFF.F0E4D0
                     7FFFFF.00   -1      0     -00:00:01.00  FFFFFFFFFF.000000
                     7FFFFE.FF   -1      255   -00:00:01.01  FFFFFFFFFE.FFD8F0
                     7FFFFE.F6   -2      246   -00:00:01.10  FFFFFFFFFE.FE7960

                     Formula to convert fractional part from disk format
                     (now stored in "frac" variable) to absolute value: "0x100 - frac".
                     To reconstruct in-memory value, we shift
                     to the next integer value and then substruct fractional part.
                */
                int_part += 1; /* Shift to the next integer value */
                frac -= 0x100; /* -(0x100 - frac) */
            }
            ((int_part << 24) + frac * 10000, int_part)
        }
        3 | 4 => {
            let mut int_part = mysql::bfixed_length_int(&data[0..3]) as i64 - TIMEF_INT_OFS;
            let mut rdr = Cursor::new(data);
            rdr.seek(SeekFrom::Current(3))?;
            frac = rdr.read_u16::<BigEndian>()? as i64;
            if int_part < 0 && frac != 0 {
                /*
                   Fix reverse fractional part order: "0x10000 - frac".
                   See comments for FSP=1 and FSP=2 above.
                */
                int_part += 1; /* Shift to the next integer value */
                frac -= 0x10000; /* -(0x100 - frac) */
            }
            ((int_part << 24) + frac * 100, int_part)
        }
        5 | 6 => {
            let len_int = mysql::bfixed_length_int(&data[0..6]) as i64 - TIMEF_OFS;
            return time_format(len_int, dec, n);
        }
        _ => {
            let int_part = mysql::bfixed_length_int(&data[0..3]) as i64 - TIMEF_INT_OFS;
            (int_part << 24, int_part)
        }
    };

    if int_part == 0 && frac == 0 {
        return Ok(("00:00:00".to_string(), n));
    }

    time_format(tmp, dec, n)
}

pub fn time_format(tmp: i64, dec: u16, n: isize) -> Result<(String, isize), ReplicationError> {
    let mut tmp = tmp;
    let mut sign = "";
    if tmp < 0 {
        tmp = -tmp;
        sign = "-";
    }

    let hms = tmp >> 24;

    let hour = (hms >> 12) % (1 << 10); /* 10 bits starting at 12th */
    let minute = (hms >> 6) % (1 << 6); /* 6 bits starting at 6th   */
    let second = hms % (1 << 6); /* 6 bits starting at 0th   */
    let sec_part = tmp % (1 << 24);

    if sec_part != 0 {
        let s = format!(
            "{}{:02}:{:02}:{:02}.{:06}",
            sign, hour, minute, second, sec_part
        );

        let stop = s.len() - (6 - dec as usize);
        return Ok((s[0..stop].to_string(), n));
    }

    Ok((
        format!("{}{:02}:{:02}:{:02}", sign, hour, minute, second),
        0,
    ))
}

pub fn decode_blob(data: &[u8], meta: u16) -> Result<(Vec<u8>, isize), ReplicationError> {
    let mut rdr = Cursor::new(data);
    match meta {
        1 => {
            let length = rdr.read_u8()? as usize;
            let n = 1 + length;
            let v = data[1..n].to_vec();
            Ok((v, n as isize))
        }
        2 => {
            let length = rdr.read_u16::<LittleEndian>()? as usize;
            let n = length + 2;
            let v = data[2..n].to_vec();
            Ok((v, n as isize))
        }
        3 => {
            let length = mysql::fixed_length_int(&data[0..3]) as usize;
            let n = length + 3;
            let v = data[3..n].to_vec();
            Ok((v, n as isize))
        }
        4 => {
            let length = rdr.read_u32::<LittleEndian>()? as usize;
            let n = length + 4;
            let v = data[4..n].to_vec();
            Ok((v, n as isize))
        }
        _ => Err(ReplicationError::new(format!(
            "invalid blob packlen = {}",
            meta
        ))),
    }
}
