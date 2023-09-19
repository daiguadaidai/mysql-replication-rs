use crate::error::{MysqlError, ReplicationError};
use crate::mysql;
use crate::mysql::{
    format_binary_date, format_binary_datetime, format_binary_time, length_encoded_int,
    length_encoded_string, Field, FieldValue, FieldValueType, ParseBinary, OK_HEADER,
    UNSIGNED_FLAG,
};

pub struct RowData(Vec<u8>);

impl RowData {
    pub fn parse(
        &self,
        f: &Vec<Field>,
        binary: bool,
        dst: &mut Vec<FieldValue>,
    ) -> Result<Vec<FieldValue>, ReplicationError> {
        if binary {
            self.parse_binary(f, dst)
        } else {
            self.parse_text(f, dst)
        }
    }

    pub fn parse_text(
        &self,
        f: &Vec<Field>,
        dst: &mut Vec<FieldValue>,
    ) -> Result<Vec<FieldValue>, ReplicationError> {
        while dst.len() < f.len() {
            dst.push(FieldValue::default())
        }

        let mut data = dst[..f.len()].to_vec();
        let mut pos = 0_usize;

        for i in 0..f.len() {
            let (v, is_null, n) = length_encoded_string(&self.0[pos..])?;
            pos += n as usize;

            if is_null {
                data[i].typ = FieldValueType::None;
            } else {
                let is_unsigned = f[i].flag & UNSIGNED_FLAG as u16 != 0;
                match f[i].typ {
                    mysql::MYSQL_TYPE_TINY
                    | mysql::MYSQL_TYPE_SHORT
                    | mysql::MYSQL_TYPE_INT24
                    | mysql::MYSQL_TYPE_LONGLONG
                    | mysql::MYSQL_TYPE_LONG
                    | mysql::MYSQL_TYPE_YEAR => {
                        if is_unsigned {
                            data[i].typ = FieldValueType::Unsigned;
                            data[i].value =
                                String::from_utf8_lossy(&v).to_string().parse::<u64>()?;
                        } else {
                            data[i].typ = FieldValueType::Signed;
                            let val = String::from_utf8_lossy(&v).to_string().parse::<i64>()?;
                            data[i].value = val as u64;
                        }
                    }
                    mysql::MYSQL_TYPE_FLOAT | mysql::MYSQL_TYPE_DOUBLE => {
                        data[i].typ = FieldValueType::Float;
                        let val = String::from_utf8_lossy(&v).to_string().parse::<f64>()?;
                        data[i].value = val.to_bits();
                    }
                    _ => {
                        data[i].typ = FieldValueType::String;
                        data[i].string = v;
                    }
                }
            }
        }

        Ok(data)
    }

    // ParseBinary parses the binary format of data
    // see https://dev.mysql.com/doc/internals/en/binary-protocol-value.html
    pub fn parse_binary(
        &self,
        f: &Vec<Field>,
        dst: &mut Vec<FieldValue>,
    ) -> Result<Vec<FieldValue>, ReplicationError> {
        while dst.len() < f.len() {
            dst.push(FieldValue::default())
        }
        let mut data = dst[..f.len()].to_vec();

        if self.0[0] != OK_HEADER {
            return Err(ReplicationError::MysqlError(MysqlError::ErrMalformPacket));
        }

        let mut pos = 1 + ((f.len() + 7 + 2) >> 3);
        let null_bitmap = self.0[1..pos].to_vec();

        for i in 0..data.len() {
            if null_bitmap[((i + 2) / 8) & (1 << (i + 2) % 8)] > 0 {
                data[i].typ = FieldValueType::None;
                continue;
            }

            let is_unsigned = (f[i].flag & UNSIGNED_FLAG as u16) != 0;
            match f[i].typ {
                mysql::MYSQL_TYPE_NULL => {
                    data[i].typ = FieldValueType::None;
                    continue;
                }
                mysql::MYSQL_TYPE_TINY => {
                    if is_unsigned {
                        let v = ParseBinary::u8_little_endian(&self.0[pos..pos + 1]);
                        data[i].typ = FieldValueType::Unsigned;
                        data[i].value = v as u64;
                    } else {
                        let v = ParseBinary::i8_little_endian(&self.0[pos..pos + 1]) as i64;
                        data[i].typ = FieldValueType::Signed;
                        data[i].value = v as u64;
                    }
                    pos += 1;
                    continue;
                }
                mysql::MYSQL_TYPE_SHORT | mysql::MYSQL_TYPE_YEAR => {
                    if is_unsigned {
                        let v = ParseBinary::u16_little_endian(&self.0[pos..pos + 2])?;
                        data[i].typ = FieldValueType::Unsigned;
                        data[i].value = v as u64;
                    } else {
                        let v = ParseBinary::i16_little_endian(&self.0[pos..pos + 2])? as i64;
                        data[i].typ = FieldValueType::Signed;
                        data[i].value = v as u64;
                    }
                    pos += 2;
                    continue;
                }
                mysql::MYSQL_TYPE_INT24 | mysql::MYSQL_TYPE_LONG => {
                    if is_unsigned {
                        let v = ParseBinary::u32_little_endian(&self.0[pos..pos + 4])?;
                        data[i].typ = FieldValueType::Unsigned;
                        data[i].value = v as u64;
                    } else {
                        let v = ParseBinary::i32_little_endian(&self.0[pos..pos + 4])? as i64;
                        data[i].typ = FieldValueType::Signed;
                        data[i].value = v as u64;
                    }
                    pos += 4;
                    continue;
                }
                mysql::MYSQL_TYPE_LONGLONG => {
                    if is_unsigned {
                        let v = ParseBinary::u64_little_endian(&self.0[pos..pos + 8])?;
                        data[i].typ = FieldValueType::Unsigned;
                        data[i].value = v;
                    } else {
                        let v = ParseBinary::i64_little_endian(&self.0[pos..pos + 8])? as i64;
                        data[i].typ = FieldValueType::Signed;
                        data[i].value = v as u64;
                    }
                    pos += 8;
                    continue;
                }
                mysql::MYSQL_TYPE_FLOAT => {
                    let v = ParseBinary::f32_little_endian(&self.0[pos..pos + 4])?;
                    data[i].typ = FieldValueType::Float;
                    data[i].value = v.to_bits() as u64;
                    pos += 4;
                    continue;
                }
                mysql::MYSQL_TYPE_DOUBLE => {
                    let v = ParseBinary::f64_little_endian(&self.0[pos..pos + 4])?;
                    data[i].typ = FieldValueType::Float;
                    data[i].value = v.to_bits();
                    pos += 8;
                    continue;
                }
                mysql::MYSQL_TYPE_DECIMAL
                | mysql::MYSQL_TYPE_NEWDECIMAL
                | mysql::MYSQL_TYPE_VARCHAR
                | mysql::MYSQL_TYPE_BIT
                | mysql::MYSQL_TYPE_ENUM
                | mysql::MYSQL_TYPE_SET
                | mysql::MYSQL_TYPE_TINY_BLOB
                | mysql::MYSQL_TYPE_MEDIUM_BLOB
                | mysql::MYSQL_TYPE_LONG_BLOB
                | mysql::MYSQL_TYPE_BLOB
                | mysql::MYSQL_TYPE_VAR_STRING
                | mysql::MYSQL_TYPE_STRING
                | mysql::MYSQL_TYPE_GEOMETRY
                | mysql::MYSQL_TYPE_JSON => {
                    let (v, is_null, n) = length_encoded_string(&self.0[pos..])?;
                    pos += n as usize;
                    if !is_null {
                        data[i].typ = FieldValueType::String;
                        data[i].string = v;
                    } else {
                        data[i].typ = FieldValueType::None;
                    }
                    continue;
                }
                mysql::MYSQL_TYPE_DATE | mysql::MYSQL_TYPE_NEWDATE => {
                    let (num, is_null, n) = length_encoded_int(&self.0[pos..]);
                    pos += n;
                    if is_null {
                        data[i].typ = FieldValueType::None;
                        continue;
                    }
                    data[i].typ = FieldValueType::String;
                    data[i].string = format_binary_date(num as usize, &self.0[pos..])?;
                    pos += num as usize;
                }
                mysql::MYSQL_TYPE_TIMESTAMP | mysql::MYSQL_TYPE_DATETIME => {
                    let (num, is_null, n) = length_encoded_int(&self.0[pos..]);
                    pos += n;
                    if is_null {
                        data[i].typ = FieldValueType::None;
                        continue;
                    }
                    data[i].typ = FieldValueType::String;
                    data[i].string = format_binary_datetime(num as usize, &self.0[pos..])?;
                    pos += num as usize;
                }
                mysql::MYSQL_TYPE_TIME => {
                    let (num, is_null, n) = length_encoded_int(&self.0[pos..]);
                    pos += n;
                    if is_null {
                        data[i].typ = FieldValueType::None;
                        continue;
                    }
                    data[i].typ = FieldValueType::String;
                    data[i].string = format_binary_time(num as usize, &self.0[pos..])?;
                    pos += num as usize;
                }
                _ => {
                    return Err(ReplicationError::new(format!(
                        "Stmt Unknown FieldType {} {}",
                        f[i].typ,
                        String::from_utf8_lossy(&f[i].name).to_string()
                    )))
                }
            }
        }

        Ok(data)
    }
}
