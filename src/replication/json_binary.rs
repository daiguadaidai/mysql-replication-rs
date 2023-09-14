use crate::error::ReplicationError;
use crate::mysql;
use crate::mysql::ParseBinary;
use crate::replication::{decode_helper, DecodeDecimal};
use crate::replication::{DecodeDatetime, DecodeJson};
use std::collections::HashMap;

pub const JSONB_SMALL_OBJECT: u8 = 0; // small JSON object
pub const JSONB_LARGE_OBJECT: u8 = 1; // large JSON object
pub const JSONB_SMALL_ARRAY: u8 = 2; // small JSON array
pub const JSONB_LARGE_ARRAY: u8 = 3; // large JSON array
pub const JSONB_LITERAL: u8 = 4; // literal (true/false/null)
pub const JSONB_INT16: u8 = 5; // int16
pub const JSONB_UINT16: u8 = 6; // uint16
pub const JSONB_INT32: u8 = 7; // int32
pub const JSONB_UINT32: u8 = 8; // uint32
pub const JSONB_INT64: u8 = 9; // int64
pub const JSONB_UINT64: u8 = 10; // uint64
pub const JSONB_DOUBLE: u8 = 11; // double
pub const JSONB_STRING: u8 = 12; // string
pub const JSONB_OPAQUE: u8 = 0x0f; // custom data (any MySQL data type)

pub const JSONB_NULL_LITERAL: u8 = 0x00;
pub const JSONB_TRUE_LITERAL: u8 = 0x01;
pub const JSONB_FALSE_LITERAL: u8 = 0x02;

const _JSONB_SMALL_OFFSET_SIZE: isize = 2;
const _JSONB_LARGE_OFFSET_SIZE: isize = 4;

const _JSONB_KEY_ENTRY_SIZE_SMALL: isize = 2 + _JSONB_SMALL_OFFSET_SIZE;
const _JSONB_KEY_ENTRY_SIZE_LARGE: isize = 2 + _JSONB_LARGE_OFFSET_SIZE;

const _JSONB_VALUE_ENTRY_SIZE_SMALL: isize = 1 + _JSONB_SMALL_OFFSET_SIZE;
const _JSONB_VALUE_ENTRY_SIZE_LARGE: isize = 1 + _JSONB_LARGE_OFFSET_SIZE;

pub const ERR_CORRUPTED_JSON_DIFF: &str = "corrupted JSON diff"; // ER_CORRUPTED_JSON_DIFF

fn _jsonb_get_offset_size(is_small: bool) -> isize {
    if is_small {
        return _JSONB_SMALL_OFFSET_SIZE;
    }

    return _JSONB_LARGE_OFFSET_SIZE;
}

fn _jsonb_get_key_entry_size(is_small: bool) -> isize {
    if is_small {
        return _JSONB_KEY_ENTRY_SIZE_SMALL;
    }

    return _JSONB_KEY_ENTRY_SIZE_LARGE;
}

fn _jsonb_get_value_entry_size(is_small: bool) -> isize {
    if is_small {
        return _JSONB_VALUE_ENTRY_SIZE_SMALL;
    }

    return _JSONB_VALUE_ENTRY_SIZE_LARGE;
}

pub struct JsonBinaryDecoder {
    pub use_decimal: bool,
    pub ignore_decode_err: bool,
    pub err: Result<(), ReplicationError>,
}

fn _is_inline_value(tp: u8, is_small: bool) -> bool {
    match tp {
        JSONB_INT16 | JSONB_UINT16 | JSONB_LITERAL => true,
        JSONB_INT32 | JSONB_UINT32 => !is_small,
        _ => false,
    }
}

impl JsonBinaryDecoder {
    pub fn decode_value(&mut self, tp: u8, data: &[u8]) -> DecodeJson {
        if self.err.is_err() {
            return DecodeJson::None;
        }

        match tp {
            JSONB_SMALL_OBJECT => self._decode_object_or_array(data, true, true),
            JSONB_LARGE_OBJECT => self._decode_object_or_array(data, false, true),
            JSONB_SMALL_ARRAY => self._decode_object_or_array(data, true, false),
            JSONB_LARGE_ARRAY => self._decode_object_or_array(data, false, false),
            JSONB_LITERAL => self._decode_literal(data),
            JSONB_INT16 => DecodeJson::Isize(self._decode_int16(data) as isize),
            JSONB_INT32 => DecodeJson::Isize(self._decode_int32(data) as isize),
            JSONB_INT64 => DecodeJson::Isize(self._decode_int64(data) as isize),
            JSONB_UINT16 => DecodeJson::Usize(self._decode_uint16(data) as usize),
            JSONB_UINT32 => DecodeJson::Usize(self._decode_uint32(data) as usize),
            JSONB_UINT64 => DecodeJson::Usize(self._decode_uint64(data) as usize),
            JSONB_DOUBLE => DecodeJson::F64(self._decode_double(data)),
            JSONB_STRING => DecodeJson::String(self._decode_string(data)),
            JSONB_OPAQUE => self._decode_opaque(data),
            _ => {
                self.err = Err(ReplicationError::new(format!("invalid json type {}", tp)));
                DecodeJson::None
            }
        }
    }

    fn _decode_object_or_array(
        &mut self,
        data: &[u8],
        is_small: bool,
        is_object: bool,
    ) -> DecodeJson {
        let offset_size = _jsonb_get_offset_size(is_small);
        if self.is_data_short(data, 2 * offset_size) {
            return DecodeJson::None;
        }

        let count = self._decode_count(data, is_small);
        let size = self._decode_count(&data[offset_size as usize..], is_small);

        if self.is_data_short(data, size) {
            // Before MySQL 5.7.22, json type generated column may have invalid value,
            // bug ref: https://bugs.mysql.com/bug.php?id=88791
            // As generated column value is not used in replication, we can just ignore
            // this error and return a dummy value for this column.
            if self.ignore_decode_err {
                self.err = Ok(());
            }

            return DecodeJson::None;
        }

        let key_entry_size = _jsonb_get_key_entry_size(is_small);
        let value_entry_size = _jsonb_get_value_entry_size(is_small);
        let mut header_size = 2 * offset_size + count * value_entry_size;

        if is_object {
            header_size += count * key_entry_size
        }
        if header_size > size {
            self.err = Err(ReplicationError::new(format!(
                "header size {} > size {}",
                header_size, size
            )));
            return DecodeJson::None;
        }

        let mut keys = Vec::<String>::new();
        if is_object {
            keys = vec![String::from(""); count as usize];
            for i in 0..count {
                // decode key
                let entry_offset = 2 * offset_size + key_entry_size * i;
                let key_offset = self._decode_count(&data[entry_offset as usize..], is_small);
                let key_length =
                    self._decode_uint16(&data[(entry_offset + offset_size) as usize..]) as isize;

                // Key must start after value entry
                if key_offset < header_size {
                    self.err = Err(ReplicationError::new(format!(
                        "invalid key offset {}, must > {}",
                        key_offset, header_size
                    )));
                    return DecodeJson::None;
                }

                if self.is_data_short(data, key_offset + key_length) {
                    return DecodeJson::None;
                }

                keys[i as usize] = String::from_utf8_lossy(
                    &data[key_offset as usize..(key_offset + key_length) as usize],
                )
                .to_string()
            }
        }

        if self.err.is_err() {
            return DecodeJson::None;
        }

        let mut values = vec![DecodeJson::None; count as usize];
        for i in 0..count {
            // decode value
            let mut entry_offset = 2 * offset_size + value_entry_size * i;
            if is_object {
                entry_offset += key_entry_size * count;
            }

            let tp = data[entry_offset as usize];
            if _is_inline_value(tp, is_small) {
                values[i as usize] = self.decode_value(
                    tp,
                    &data[(entry_offset + 1) as usize..(entry_offset + value_entry_size) as usize],
                );
                continue;
            }

            let value_offset = self._decode_count(&data[(entry_offset + 1) as usize..], is_small);
            if self.is_data_short(data, value_offset) {
                return DecodeJson::None;
            }

            values[i as usize] = self.decode_value(tp, &data[value_offset as usize..]);
        }

        if self.err.is_err() {
            return DecodeJson::None;
        }
        if !is_object {
            return DecodeJson::Vec(values);
        }

        let mut m = HashMap::<String, DecodeJson>::with_capacity(count as usize);
        for i in 0..count as usize {
            m.insert(keys[i].clone(), values[i].clone());
        }

        DecodeJson::Map(m)
    }

    fn _decode_literal(&mut self, data: &[u8]) -> DecodeJson {
        if self.is_data_short(data, 1) {
            return DecodeJson::None;
        }
        let tp = data[0];

        match tp {
            JSONB_NULL_LITERAL => return DecodeJson::None,
            JSONB_TRUE_LITERAL => return DecodeJson::Bool(true),
            JSONB_FALSE_LITERAL => return DecodeJson::Bool(false),
            _ => {}
        }

        self.err = Err(ReplicationError::new(format!("invalid literal {}", tp)));

        DecodeJson::None
    }

    pub fn is_data_short(&mut self, data: &[u8], expected: isize) -> bool {
        if self.err.is_err() {
            return true;
        }

        if data.len() < expected as usize {
            self.err = Err(ReplicationError::new(format!(
                "data len {} < expected {}",
                data.len(),
                expected
            )))
        }

        return self.err.is_err();
    }

    fn _decode_int16(&mut self, data: &[u8]) -> i16 {
        if self.is_data_short(data, 2) {
            return 0;
        }

        match ParseBinary::i16_little_endian(&data[0..2]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0
            }
        }
    }

    fn _decode_uint16(&mut self, data: &[u8]) -> u16 {
        if self.is_data_short(data, 2) {
            return 0;
        }

        match ParseBinary::u16_little_endian(&data[0..2]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0
            }
        }
    }

    fn _decode_int32(&mut self, data: &[u8]) -> i32 {
        if self.is_data_short(data, 4) {
            return 0;
        }

        match ParseBinary::i32_little_endian(&data[0..4]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0
            }
        }
    }

    fn _decode_uint32(&mut self, data: &[u8]) -> u32 {
        if self.is_data_short(data, 4) {
            return 0;
        }

        match ParseBinary::u32_little_endian(&data[0..4]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0
            }
        }
    }

    fn _decode_int64(&mut self, data: &[u8]) -> i64 {
        if self.is_data_short(data, 8) {
            return 0;
        }

        match ParseBinary::i64_little_endian(&data[0..8]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0
            }
        }
    }

    fn _decode_uint64(&mut self, data: &[u8]) -> u64 {
        if self.is_data_short(data, 8) {
            return 0;
        }

        match ParseBinary::u64_little_endian(&data[0..8]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0
            }
        }
    }

    fn _decode_double(&mut self, data: &[u8]) -> f64 {
        if self.is_data_short(data, 8) {
            return 0.0;
        }

        match ParseBinary::f64_little_endian(&data[0..8]) {
            Ok(v) => v,
            Err(e) => {
                self.err = Err(ReplicationError::new(format!("{}", e.to_string())));
                0.0
            }
        }
    }

    fn _decode_string(&mut self, data: &[u8]) -> String {
        if self.err.is_err() {
            return String::from("");
        }

        let (l, n) = self._decode_variable_length(data);
        if self.is_data_short(data, l + n) {
            return String::from("");
        }

        let data = &data[n as usize..];
        String::from_utf8_lossy(&data[0..l as usize]).to_string()
    }

    fn _decode_opaque(&mut self, data: &[u8]) -> DecodeJson {
        if self.is_data_short(data, 1) {
            return DecodeJson::None;
        }

        let tp = data[0];
        let data = &data[1..];

        let (l, n) = self._decode_variable_length(data);

        if self.is_data_short(data, l + n) {
            return DecodeJson::None;
        }

        let data = &data[n as usize..(l + n) as usize];

        match tp {
            mysql::MYSQL_TYPE_NEWDECIMAL => self._decode_decimal(data),
            mysql::MYSQL_TYPE_TIME => self._decode_time(data),
            mysql::MYSQL_TYPE_DATE | mysql::MYSQL_TYPE_DATETIME | mysql::MYSQL_TYPE_TIMESTAMP => {
                self._decode_datetime(data)
            }
            _ => DecodeJson::None,
        }
    }

    fn _decode_decimal(&mut self, data: &[u8]) -> DecodeJson {
        let precision = data[0] as isize;
        let scale = data[1] as isize;
        match decode_helper::decode_decimal(&data[2..], precision, scale, self.use_decimal) {
            Ok((v, _)) => DecodeJson::Decimal(v),
            Err(e) => {
                self.err = Err(e);
                DecodeJson::Decimal(DecodeDecimal::Unknown)
            }
        }
    }

    fn _decode_time(&mut self, data: &[u8]) -> DecodeJson {
        let mut v = self._decode_int64(data);
        if v == 0 {
            return DecodeJson::Datetime(DecodeDatetime::String(String::from("00:00:00")));
        }

        let mut sign = "";
        if v < 0 {
            sign = "-";
            v = -v
        }

        let int_part = v >> 24;
        let hour = (int_part >> 12) % (1 << 10);
        let min = (int_part >> 6) % (1 << 6);
        let sec = int_part % (1 << 6);
        let frac = v % (1 << 24);

        return DecodeJson::Datetime(DecodeDatetime::String(format!(
            "{}%{:02}:{:02}:{:02}.{:06}",
            sign, hour, min, sec, frac
        )));
    }

    fn _decode_datetime(&mut self, data: &[u8]) -> DecodeJson {
        let mut v = self._decode_int64(data);
        if v == 0 {
            return DecodeJson::Datetime(DecodeDatetime::String(String::from(
                "0000-00-00 00:00:00",
            )));
        }

        // handle negative?
        if v < 0 {
            v = -v
        }

        let int_part = v >> 24;
        let ymd = int_part >> 17;
        let ym = ymd >> 5;
        let hms = int_part % (1 << 17);

        let year = ym / 13;
        let month = ym % 13;
        let day = ymd % (1 << 5);
        let hour = hms >> 12;
        let minute = (hms >> 6) % (1 << 6);
        let second = hms % (1 << 6);
        let frac = v % (1 << 24);

        DecodeJson::Datetime(DecodeDatetime::String(format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
            year, month, day, hour, minute, second, frac
        )))
    }

    fn _decode_count(&mut self, data: &[u8], is_small: bool) -> isize {
        if is_small {
            return self._decode_uint16(data) as isize;
        }

        return self._decode_uint32(data) as isize;
    }

    fn _decode_variable_length(&mut self, data: &[u8]) -> (isize, isize) {
        // The max size for variable length is math.MaxUint32, so
        // here we can use 5 bytes to save it.
        let mut max_count = 5;
        if data.len() < max_count {
            max_count = data.len()
        }

        let mut pos = 0_usize;
        let mut length = 0_usize;
        while pos < max_count {
            let v = data[pos];
            length |= ((v & 0x7F) as usize) << (7 * pos);

            if v & 0x80 == 0 {
                if length > u32::MAX as usize {
                    self.err = Err(ReplicationError::new(format!(
                        "variable length {} must <= {}",
                        length,
                        u32::MAX
                    )));
                    return (0, 0);
                }
                pos += 1;
                // TODO: should consider length overflow int here.
                return (length as isize, pos as isize);
            }
            pos += 1;
        }
        self.err = Err(ReplicationError::new(format!(
            "decode variable length failed"
        )));

        (0, 0)
    }
}
