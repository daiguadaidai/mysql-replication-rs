use crate::error::ReplicationError;
use crate::mysql::ParseBinary;
use crate::replication::{
    decode_helper, DecodeDatetime, DecodeFieldData, DecodeJson, Event, EventType, FracTime,
    JsonBinaryDecoder, JsonDiff, JsonDiffOperation,
};
use crate::{mysql, replication, utils};
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::rc::Rc;

pub const ERR_MISSING_TABLE_MAP_EVENT: &str = "invalid table id, no corresponding table map event";

#[derive(Debug, Default, Clone)]
pub struct TableMapEvent {
    pub flavor: String,
    pub table_id_size: isize,

    pub table_id: u64,

    pub flags: u16,

    pub schema: Vec<u8>,
    pub table: Vec<u8>,

    pub column_count: u64,
    pub column_type: Vec<u8>,
    pub column_meta: Vec<u16>,

    // len = (ColumnCount + 7) / 8
    pub null_bitmap: Vec<u8>,

    /*
        The following are available only after MySQL-8.0.1 or MariaDB-10.5.0
        By default MySQL and MariaDB do not log the full row metadata.
        see:
            - https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_row_metadata
            - https://mariadb.com/kb/en/replication-and-binary-log-system-variables/#binlog_row_metadata
    */
    // SignednessBitmap stores signedness info for numeric columns.
    pub signedness_bitmap: Vec<u8>,

    // DefaultCharset/ColumnCharset stores collation info for character columns.

    // DefaultCharset[0] is the default collation of character columns.
    // For character columns that have different charset,
    // (character column index, column collation) pairs follows
    pub default_charset: Vec<u64>,
    // ColumnCharset contains collation sequence for all character columns
    pub column_charset: Vec<u64>,

    // SetStrValue stores values for set columns.
    pub set_str_value: Vec<Vec<Vec<u8>>>,
    _set_str_value_string: Vec<Vec<String>>,

    // EnumStrValue stores values for enum columns.
    pub enum_str_value: Vec<Vec<Vec<u8>>>,
    _enum_str_value_string: Vec<Vec<String>>,

    // ColumnName list all column names.
    pub column_name: Vec<Vec<u8>>,
    _column_name_string: Vec<String>, // the same as ColumnName in string type, just for reuse

    // GeometryType stores real type for geometry columns.
    pub geometry_type: Vec<u64>,

    // PrimaryKey is a sequence of column indexes of primary key.
    pub primary_key: Vec<u64>,

    // PrimaryKeyPrefix is the prefix length used for each column of primary key.
    // 0 means that the whole column length is used.
    pub primary_key_prefix: Vec<u64>,

    // EnumSetDefaultCharset/EnumSetColumnCharset is similar to DefaultCharset/ColumnCharset but for enum/set columns.
    pub enum_set_default_charset: Vec<u64>,
    pub enum_set_column_charset: Vec<u64>,
}

impl Event for TableMapEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "TableID: {}\n", self.table_id)?;
        write!(writer, "TableID size: {}\n", self.table_id_size)?;
        write!(writer, "Flags: {}\n", self.flags)?;
        write!(
            writer,
            "Schema: {}\n",
            String::from_utf8_lossy(&self.schema)
        )?;
        write!(writer, "Table: {}\n", String::from_utf8_lossy(&self.table))?;
        write!(writer, "Column count: {}\n", self.column_count)?;
        write!(writer, "Column type: \n{}", hex::encode(&self.column_type))?;
        write!(writer, "NULL bitmap: \n{}", hex::encode(&self.null_bitmap))?;

        write!(
            writer,
            "Signedness bitmap: \n{}",
            hex::encode(&self.signedness_bitmap)
        )?;
        write!(writer, "Default charset: {:?}\n", self.default_charset)?;
        write!(writer, "Column charset: {:?}\n", self.column_charset)?;
        write!(writer, "Set str value: {:?}\n", self.set_str_value_string())?;
        write!(
            writer,
            "Enum str value: {:?}\n",
            self.enum_str_value_string()
        )?;
        write!(writer, "Column name: {:?}\n", self.column_name_string())?;
        write!(writer, "Geometry type: {:?}\n", self.geometry_type)?;
        write!(writer, "Primary key: {:?}\n", self.primary_key)?;
        write!(
            writer,
            "Primary key prefix: {:?}\n",
            self.primary_key_prefix
        )?;
        write!(
            writer,
            "Enum/set default charset: {:?}\n",
            self.enum_set_default_charset
        )?;
        write!(
            writer,
            "Enum/set column charset: {:?}\n",
            self.enum_set_column_charset
        )?;

        let unsigned_map = self.unsigned_map();
        write!(writer, "UnsignedMap: {:?}\n", unsigned_map)?;

        let collation_map = self.collation_map();
        write!(writer, "CollationMap: {:?}\n", collation_map)?;

        let enum_set_collation_map = self.enum_set_collation_map();
        write!(
            writer,
            "EnumSetCollationMap: {:?}\n",
            enum_set_collation_map
        )?;

        let enum_str_value_map = self.enum_str_value_map();
        write!(writer, "EnumStrValueMap: {:?}\n", enum_str_value_map)?;

        let set_str_value_map = self.set_str_value_map();
        write!(writer, "SetStrValueMap: {:?}\n", set_str_value_map)?;

        let geometry_type_map = self.geometry_type_map();
        write!(writer, "GeometryTypeMap: {:?}\n", geometry_type_map)?;

        let mut name_max_len = 0;
        for name in &self.column_name {
            if name.len() > name_max_len {
                name_max_len = name.len()
            }
        }

        let mut name_fmt = String::from("  {}");
        if name_max_len > 0 {
            name_fmt = format!("  {{:<{len}}}", len = name_max_len);
        }

        let mut primary_key = HashSet::<isize>::new();
        for &pk in &self.primary_key {
            primary_key.insert(pk as isize);
        }

        write!(writer, "Columns: \n")?;
        for i in 0..self.column_count as isize {
            if self.column_name.len() == 0 {
                let fmt_data = utils::format::fmt_str_vec(
                    &name_fmt,
                    &vec![utils::format::FVariant::String(String::from("<n/a>"))],
                );
                write!(writer, "{}", fmt_data)?;
            } else {
                let fmt_data = utils::format::fmt_str_vec(
                    &name_fmt,
                    &vec![utils::format::FVariant::String(
                        String::from_utf8_lossy(&self.column_name[i as usize]).to_string(),
                    )],
                );
                write!(writer, "{}", fmt_data)?;
            }

            write!(writer, "  type={:<3}", self.real_type(i as usize))?;

            if self.is_numeric_column(i as usize) {
                if unsigned_map.len() == 0 {
                    write!(writer, "  unsigned=<n/a>")?;
                } else if unsigned_map[&i] {
                    write!(writer, "  unsigned=yes")?;
                } else {
                    write!(writer, "  unsigned=no ")?;
                }
            }
            if self.is_character_column(i as usize) {
                if collation_map.len() == 0 {
                    write!(writer, "  unsigned=<n/a>")?;
                } else {
                    write!(writer, "  collation={} ", collation_map.get(&i).unwrap())?;
                }
            }
            if self.is_enum_column(i as usize) {
                if enum_set_collation_map.len() == 0 {
                    write!(writer, "  enum_collation=<n/a>")?;
                } else {
                    write!(
                        writer,
                        "  enum_collation={}",
                        enum_set_collation_map.get(&i).unwrap()
                    )?;
                }
                if enum_str_value_map.len() == 0 {
                    write!(writer, "  enum=<n/a>")?;
                } else {
                    write!(writer, "  enum={:?}", enum_str_value_map.get(&i).unwrap())?;
                }
            }
            if self.is_set_column(i as usize) {
                if enum_set_collation_map.len() == 0 {
                    write!(writer, "  set_collation=<n/a>")?;
                } else {
                    write!(
                        writer,
                        "  set_collation={}",
                        enum_set_collation_map.get(&i).unwrap()
                    )?;
                }

                if set_str_value_map.len() == 0 {
                    write!(writer, "  set=<n/a>")?;
                } else {
                    write!(writer, "  set={:?}", set_str_value_map.get(&i).unwrap())?;
                }
            }
            if self.is_geometry_column(i as usize) {
                if geometry_type_map.len() == 0 {
                    write!(writer, "  geometry_type=<n/a>")?;
                } else {
                    write!(
                        writer,
                        "  geometry_type={}",
                        geometry_type_map.get(&i).unwrap()
                    )?;
                }
            }

            let (available, nullable) = self.nullable(i as usize);
            if !available {
                write!(writer, "  null=<n/a>")?;
            } else if nullable {
                write!(writer, "  null=yes")?;
            } else {
                write!(writer, "  null=no ")?;
            }

            if primary_key.contains(&i) {
                write!(writer, "  pri")?;
            }
            write!(writer, "\n")?;
        }

        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.table_id = mysql::fixed_length_int(&data[..self.table_id_size as usize]);
        rdr.seek(SeekFrom::Current(self.table_id_size as i64))?;

        self.flags = rdr.read_u16::<LittleEndian>()?;
        let schema_length = rdr.read_u8()?;

        let start = rdr.position() as usize;
        let stop = start + schema_length as usize;
        self.schema = data[start..stop].to_vec();
        rdr.seek(SeekFrom::Current(schema_length as i64))?;

        // skip 0x00
        rdr.seek(SeekFrom::Current(1))?;

        let table_length = rdr.read_u8()? as usize;
        let start = rdr.position() as usize;
        let stop = start + table_length;
        self.table = data[start..stop].to_vec();
        rdr.seek(SeekFrom::Current(table_length as i64))?;

        // skip 0x00
        rdr.seek(SeekFrom::Current(1))?;

        let mut _n = 0;
        (self.column_count, _, _n) = mysql::length_encoded_int(&data[rdr.position() as usize..]);
        rdr.seek(SeekFrom::Current(_n as i64))?;

        let start = rdr.position() as usize;
        let stop = start + self.column_count as usize;
        self.column_type = data[start..stop].to_vec();
        rdr.seek(SeekFrom::Current(self.column_count as i64))?;

        let (meta_data, _, n) = mysql::length_encoded_string(&data[rdr.position() as usize..])?;
        self.decode_meta(&meta_data)?;
        rdr.seek(SeekFrom::Current(n as i64))?;

        let null_bitmap_size = bitmap_byte_size(self.column_count as isize) as usize;
        if data[rdr.position() as usize..].len() < null_bitmap_size {
            return Err(ReplicationError::IoError(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "EOF",
            )));
        }
        let start = rdr.position() as usize;
        let stop = start + null_bitmap_size;
        self.null_bitmap = data[start..stop].to_vec();
        rdr.seek(SeekFrom::Current(null_bitmap_size as i64))?;

        self.decode_optional_meta(&data[rdr.position() as usize..])?;
        Ok(())
    }
}

// see mysql sql/log_event.h
/*
    0 byte
    MYSQL_TYPE_DECIMAL
    MYSQL_TYPE_TINY
    MYSQL_TYPE_SHORT
    MYSQL_TYPE_LONG
    MYSQL_TYPE_NULL
    MYSQL_TYPE_TIMESTAMP
    MYSQL_TYPE_LONGLONG
    MYSQL_TYPE_INT24
    MYSQL_TYPE_DATE
    MYSQL_TYPE_TIME
    MYSQL_TYPE_DATETIME
    MYSQL_TYPE_YEAR

    1 byte
    MYSQL_TYPE_FLOAT
    MYSQL_TYPE_DOUBLE
    MYSQL_TYPE_BLOB
    MYSQL_TYPE_GEOMETRY

    //maybe
    MYSQL_TYPE_TIME2
    MYSQL_TYPE_DATETIME2
    MYSQL_TYPE_TIMESTAMP2

    2 byte
    MYSQL_TYPE_VARCHAR
    MYSQL_TYPE_BIT
    MYSQL_TYPE_NEWDECIMAL
    MYSQL_TYPE_VAR_STRING
    MYSQL_TYPE_STRING

    This enumeration value is only used internally and cannot exist in a binlog.
    MYSQL_TYPE_NEWDATE
    MYSQL_TYPE_ENUM
    MYSQL_TYPE_SET
    MYSQL_TYPE_TINY_BLOB
    MYSQL_TYPE_MEDIUM_BLOB
    MYSQL_TYPE_LONG_BLOB
*/
impl TableMapEvent {
    fn decode_meta(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.column_meta = vec![0; self.column_count as usize];
        for (i, t) in self.column_type.iter().enumerate() {
            match *t {
                mysql::MYSQL_TYPE_STRING => {
                    let mut x = (rdr.read_u8()? as u16) << 8;
                    x += rdr.read_u8()? as u16;
                    self.column_meta[i] = x;
                }
                mysql::MYSQL_TYPE_NEWDECIMAL => {
                    let mut x = (rdr.read_u8()? as u16) << 8; // precision
                    x += rdr.read_u8()? as u16; // decimals
                    self.column_meta[i] = x;
                }
                mysql::MYSQL_TYPE_VAR_STRING
                | mysql::MYSQL_TYPE_VARCHAR
                | mysql::MYSQL_TYPE_BIT => {
                    self.column_meta[i] = rdr.read_u16::<LittleEndian>()?;
                }
                mysql::MYSQL_TYPE_BLOB
                | mysql::MYSQL_TYPE_DOUBLE
                | mysql::MYSQL_TYPE_FLOAT
                | mysql::MYSQL_TYPE_GEOMETRY
                | mysql::MYSQL_TYPE_JSON => {
                    self.column_meta[i] = rdr.read_u8()? as u16;
                }
                mysql::MYSQL_TYPE_TIME2
                | mysql::MYSQL_TYPE_DATETIME2
                | mysql::MYSQL_TYPE_TIMESTAMP2 => {
                    self.column_meta[i] = rdr.read_u8()? as u16;
                }
                mysql::MYSQL_TYPE_NEWDATE
                | mysql::MYSQL_TYPE_ENUM
                | mysql::MYSQL_TYPE_SET
                | mysql::MYSQL_TYPE_TINY_BLOB
                | mysql::MYSQL_TYPE_MEDIUM_BLOB
                | mysql::MYSQL_TYPE_LONG_BLOB => {
                    return Err(ReplicationError::new(format!(
                        "unsupport type in binlog {}",
                        t
                    )))
                }
                _ => self.column_meta[i] = 0,
            }
        }

        Ok(())
    }

    fn decode_optional_meta(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let mut rdr = Cursor::new(data);

        while rdr.position() < data.len() as u64 {
            // optional metadata fields are stored in Type, Length, Value(TLV) format
            // Type takes 1 byte. Length is a packed integer value. Values takes Length bytes
            let t = rdr.read_u8()?;
            let (l, _, n) = mysql::length_encoded_int(&data[rdr.position() as usize..]);
            rdr.seek(SeekFrom::Current(n as i64))?;

            let start = rdr.position() as usize;
            let stop = start + l as usize;
            let v = data[start..stop].to_vec();
            rdr.seek(SeekFrom::Current(l as i64))?;

            match t {
                replication::TABLE_MAP_OPT_META_SIGNEDNESS => {
                    self.signedness_bitmap = v;
                }
                replication::TABLE_MAP_OPT_META_DEFAULT_CHARSET => {
                    self.default_charset = self.decode_default_charset(&v)?;
                }
                replication::TABLE_MAP_OPT_META_COLUMN_CHARSET => {
                    self.column_charset = self.decode_int_seq(&v)?;
                }
                replication::TABLE_MAP_OPT_META_COLUMN_NAME => {
                    self.decode_column_names(&v)?;
                }
                replication::TABLE_MAP_OPT_META_SET_STR_VALUE => {
                    self.set_str_value = self.decode_str_value(&v)?;
                }
                replication::TABLE_MAP_OPT_META_ENUM_STR_VALUE => {
                    self.enum_str_value = self.decode_str_value(&v)?;
                }
                replication::TABLE_MAP_OPT_META_GEOMETRY_TYPE => {
                    self.geometry_type = self.decode_int_seq(&v)?;
                }
                replication::TABLE_MAP_OPT_META_SIMPLE_PRIMARY_KEY => {
                    self.decode_simple_primary_key(&v)?;
                }
                replication::TABLE_MAP_OPT_META_PRIMARY_KEY_WITH_PREFIX => {
                    self.decode_primary_key_with_prefix(&v)?;
                }
                replication::TABLE_MAP_OPT_META_ENUM_AND_SET_DEFAULT_CHARSET => {
                    self.enum_set_default_charset = self.decode_default_charset(&v)?;
                }
                replication::TABLE_MAP_OPT_META_ENUM_AND_SET_COLUMN_CHARSET => {
                    self.enum_set_column_charset = self.decode_int_seq(&v)?;
                }
                _ => {
                    // Ignore for future extension
                }
            }
        }

        Ok(())
    }

    fn decode_int_seq(&self, v: &[u8]) -> Result<Vec<u64>, ReplicationError> {
        let mut ret = Vec::<u64>::new();
        let mut p = 0_usize;
        while p < v.len() {
            let (i, _, n) = mysql::length_encoded_int(&v[p..]);
            p += n as usize;
            ret.push(i);
        }

        Ok(ret)
    }

    fn decode_default_charset(&mut self, v: &[u8]) -> Result<Vec<u64>, ReplicationError> {
        let ret = self.decode_int_seq(&v)?;
        if ret.len() % 2 != 1 {
            return Err(ReplicationError::new(format!(
                "Expect odd item in DefaultCharset but got {}",
                ret.len()
            )));
        }

        Ok(ret)
    }

    fn decode_column_names(&mut self, v: &[u8]) -> Result<(), ReplicationError> {
        let mut p = 0_usize;
        while p < v.len() {
            let n = v[p] as usize;
            p += 1;
            self.column_name.push(v[p..p + n].to_vec());
            p += n;
        }

        if self.column_name.len() != self.column_count as usize {
            return Err(ReplicationError::new(format!(
                "Expect {} column names but got {}",
                self.column_count,
                self.column_name.len()
            )));
        }

        Ok(())
    }

    fn decode_str_value(&self, v: &[u8]) -> Result<Vec<Vec<Vec<u8>>>, ReplicationError> {
        let mut ret: Vec<Vec<Vec<u8>>> = vec![];
        let mut p = 0_usize;
        while p < v.len() {
            let (n_val, _, n) = mysql::length_encoded_int(&v[p..]);
            p += n as usize;
            let mut vals = Vec::<Vec<u8>>::with_capacity(n_val as usize);
            for _ in 0..n_val {
                let (val, _, n) = mysql::length_encoded_string(&v[p..])?;
                p += n as usize;
                vals.push(val);
            }

            ret.push(vals);
        }

        Ok(ret)
    }

    fn decode_simple_primary_key(&mut self, v: &[u8]) -> Result<(), ReplicationError> {
        let mut p = 0_usize;
        while p < v.len() {
            let (i, _, n) = mysql::length_encoded_int(&v[p..]);
            self.primary_key.push(i);
            self.primary_key_prefix.push(0);
            p += n as usize;
        }

        Ok(())
    }

    fn decode_primary_key_with_prefix(&mut self, v: &[u8]) -> Result<(), ReplicationError> {
        let mut p = 0_usize;
        while p < v.len() {
            let (i, _, n) = mysql::length_encoded_int(&v[p..]);
            self.primary_key.push(i);
            p += n as usize;

            let (i, _, n) = mysql::length_encoded_int(&v[p..]);
            self.primary_key_prefix.push(i);
            p += n as usize;
        }

        Ok(())
    }

    // Nullable returns the nullablity of the i-th column.
    // If null bits are not available, available is false.
    // i must be in range [0, ColumnCount).
    pub fn nullable(&self, i: usize) -> (bool, bool) {
        if self.null_bitmap.len() == 0 {
            return (false, false);
        }

        return (
            true,
            (self.null_bitmap[i / 8] & (1_usize << (i % 8)) as u8) != 0,
        );
    }

    // SetStrValueString returns values for set columns as string slices.
    // nil is returned if not available or no set columns at all.
    pub fn set_str_value_string(&mut self) -> Vec<Vec<String>> {
        if self._set_str_value_string.len() == 0 {
            if self.set_str_value.len() == 0 {
                return vec![];
            }
            for vals in &self.set_str_value {
                self._set_str_value_string
                    .push(self.bytes_slice_2_str_slice(vals))
            }
        }

        self._set_str_value_string.clone()
    }

    // EnumStrValueString returns values for enum columns as string slices.
    // nil is returned if not available or no enum columns at all.
    pub fn enum_str_value_string(&mut self) -> Vec<Vec<String>> {
        if self._enum_str_value_string.len() == 0 {
            if self.enum_str_value.len() == 0 {
                return vec![];
            }
            for vals in &self.enum_str_value {
                self._enum_str_value_string
                    .push(self.bytes_slice_2_str_slice(vals))
            }
        }

        return self._enum_str_value_string.clone();
    }

    // ColumnNameString returns column names as string slice.
    // nil is returned if not available.
    pub fn column_name_string(&mut self) -> Vec<String> {
        if self._column_name_string.len() == 0 {
            self._column_name_string = self.bytes_slice_2_str_slice(&self.column_name)
        }

        self._column_name_string.clone()
    }

    fn bytes_slice_2_str_slice(&self, src: &[Vec<u8>]) -> Vec<String> {
        if src.len() == 0 {
            return vec![];
        }

        let mut ret = Vec::<String>::new();
        for item in src {
            ret.push(String::from_utf8_lossy(item).to_string());
        }

        ret
    }
    // UnsignedMap returns a map: column index -> unsigned.
    // Note that only numeric columns will be returned.
    // nil is returned if not available or no numeric columns at all.
    pub fn unsigned_map(&self) -> HashMap<isize, bool> {
        if self.signedness_bitmap.len() == 0 {
            return HashMap::default();
        }

        let mut p = 0_usize;
        let mut ret = HashMap::default();
        for i in 0..self.column_count as isize {
            if !self.is_numeric_column(i as usize) {
                continue;
            }
            ret.insert(
                i,
                (self.signedness_bitmap[p / 8] & (1_usize << (7 - p % 8)) as u8) != 0,
            );
            p += 1
        }

        ret
    }

    // CollationMap returns a map: column index -> collation id.
    // Note that only character columns will be returned.
    // nil is returned if not available or no character columns at all.
    pub fn collation_map(&self) -> HashMap<isize, u64> {
        self._collation_map(
            |i| self.is_character_column(i),
            &self.default_charset,
            &self.column_charset,
        )
    }

    // EnumSetCollationMap returns a map: column index -> collation id.
    // Note that only enum or set columns will be returned.
    // nil is returned if not available or no enum/set columns at all.
    pub fn enum_set_collation_map(&self) -> HashMap<isize, u64> {
        self._collation_map(
            |i| self.is_enum_or_set_column(i),
            &self.enum_set_default_charset,
            &self.enum_set_column_charset,
        )
    }

    fn _collation_map<F>(
        &self,
        include_type: F,
        default_charset: &[u64],
        column_charset: &[u64],
    ) -> HashMap<isize, u64>
    where
        F: Fn(usize) -> bool,
    {
        if default_charset.len() != 0 {
            let default_collation = default_charset[0];

            // character column index -> collation
            let mut collations = HashMap::<isize, u64>::new();
            for i in (1..default_charset.len()).step_by(2) {
                collations.insert(default_charset[i] as isize, default_charset[i + 1]);
            }

            let mut p = 0;
            let mut ret = HashMap::<isize, u64>::new();
            for i in 0..self.column_count as isize {
                if !include_type(i as usize) {
                    continue;
                }

                if let Some(&collation) = collations.get(&p) {
                    ret.insert(i, collation);
                } else {
                    ret.insert(i, default_collation);
                }
                p += 1;
            }

            return ret;
        }

        if column_charset.len() != 0 {
            let mut p = 0;

            let mut ret = HashMap::<isize, u64>::new();
            for i in 0..self.column_count as isize {
                if !include_type(i as usize) {
                    continue;
                }
                ret.insert(i, column_charset[p]);
                p += 1
            }

            return ret;
        }

        HashMap::default()
    }

    // EnumStrValueMap returns a map: column index -> enum string value.
    // Note that only enum columns will be returned.
    // nil is returned if not available or no enum columns at all.
    pub fn enum_str_value_map(&mut self) -> HashMap<isize, Vec<String>> {
        let str_value = self.enum_str_value_string();
        self.str_value_map(|i| self.is_enum_column(i), &str_value)
    }

    // SetStrValueMap returns a map: column index -> set string value.
    // Note that only set columns will be returned.
    // nil is returned if not available or no set columns at all.
    pub fn set_str_value_map(&mut self) -> HashMap<isize, Vec<String>> {
        let str_value = self.set_str_value_string();
        self.str_value_map(|i| self.is_set_column(i), &str_value)
    }

    fn str_value_map<F>(
        &self,
        include_type: F,
        str_value: &[Vec<String>],
    ) -> HashMap<isize, Vec<String>>
    where
        F: Fn(usize) -> bool,
    {
        if str_value.len() == 0 {
            return HashMap::default();
        }

        let mut p = 0;
        let mut ret = HashMap::<isize, Vec<String>>::default();
        for i in 0..self.column_count as isize {
            if !include_type(i as usize) {
                continue;
            }
            ret.insert(i, str_value[p].clone());
            p += 1
        }

        ret
    }

    // GeometryTypeMap returns a map: column index -> geometry type.
    // Note that only geometry columns will be returned.
    // nil is returned if not available or no geometry columns at all.
    pub fn geometry_type_map(&self) -> HashMap<isize, u64> {
        if self.geometry_type.len() == 0 {
            return HashMap::default();
        }

        let mut p = 0;
        let mut ret = HashMap::<isize, u64>::new();
        for i in 0..self.column_count as isize {
            if !self.is_geometry_column(i as usize) {
                continue;
            }
            ret.insert(i, self.geometry_type[p]);
            p += 1
        }

        ret
    }

    // Below realType and IsXXXColumn are base from:
    //   table_def::type in sql/rpl_utility.h
    //   Table_map_log_event::print_columns in mysql-8.0/sql/log_event.cc and mariadb-10.5/sql/log_event_client.cc
    fn real_type(&self, i: usize) -> u8 {
        let typ = self.column_type[i];

        match typ {
            mysql::MYSQL_TYPE_STRING => {
                let rtyp = (self.column_meta[i] >> 8) as u8;
                if rtyp == mysql::MYSQL_TYPE_ENUM || rtyp == mysql::MYSQL_TYPE_SET {
                    return rtyp;
                }
            }
            mysql::MYSQL_TYPE_DATE => return mysql::MYSQL_TYPE_NEWDATE,
            _ => {}
        }

        typ
    }

    pub fn is_numeric_column(&self, i: usize) -> bool {
        match self.real_type(i) {
            mysql::MYSQL_TYPE_TINY
            | mysql::MYSQL_TYPE_SHORT
            | mysql::MYSQL_TYPE_INT24
            | mysql::MYSQL_TYPE_LONG
            | mysql::MYSQL_TYPE_LONGLONG
            | mysql::MYSQL_TYPE_NEWDECIMAL
            | mysql::MYSQL_TYPE_FLOAT
            | mysql::MYSQL_TYPE_DOUBLE => true,
            _ => false,
        }
    }

    // IsCharacterColumn returns true if the column type is considered as character type.
    // Note that JSON/GEOMETRY types are treated as character type in mariadb.
    // (JSON is an alias for LONGTEXT in mariadb: https://mariadb.com/kb/en/json-data-type/)
    pub fn is_character_column(&self, i: usize) -> bool {
        match self.real_type(i) {
            mysql::MYSQL_TYPE_STRING
            | mysql::MYSQL_TYPE_VAR_STRING
            | mysql::MYSQL_TYPE_VARCHAR
            | mysql::MYSQL_TYPE_BLOB => true,
            mysql::MYSQL_TYPE_GEOMETRY => {
                if self.flavor == "mariadb" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_enum_column(&self, i: usize) -> bool {
        self.real_type(i) == mysql::MYSQL_TYPE_ENUM
    }

    pub fn is_set_column(&self, i: usize) -> bool {
        self.real_type(i) == mysql::MYSQL_TYPE_SET
    }

    pub fn is_geometry_column(&self, i: usize) -> bool {
        self.real_type(i) == mysql::MYSQL_TYPE_GEOMETRY
    }

    pub fn is_enum_or_set_column(&self, i: usize) -> bool {
        let rtyp = self.real_type(i);
        rtyp == mysql::MYSQL_TYPE_ENUM || rtyp == mysql::MYSQL_TYPE_SET
    }

    // JsonColumnCount returns the number of JSON columns in this table
    pub fn json_column_count(&self) -> u64 {
        let mut count = 0_u64;
        for &t in &self.column_type {
            if t == mysql::MYSQL_TYPE_JSON {
                count += 1
            }
        }

        count
    }
}

fn bitmap_byte_size(column_count: isize) -> isize {
    (column_count + 7) / 8
}

// RowsEventStmtEndFlag is set in the end of the statement.
pub const ROWS_EVENT_STMT_END_FLAG: u8 = 0x01;

#[derive(Debug, Clone, Default)]
pub struct RowsEvent {
    // 0, 1, 2
    pub version: isize,

    pub table_id_size: isize,
    pub tables: HashMap<u64, TableMapEvent>,
    pub need_bitmap2: bool,

    // for mariadb *_COMPRESSED_EVENT_V1
    pub compressed: bool,

    pub event_type: EventType,

    pub table: TableMapEvent,

    pub table_id: u64,

    pub flags: u16,

    // if version == 2
    pub extra_data: Vec<u8>,

    // lenenc_int
    pub column_count: u64,

    /*
        By default MySQL and MariaDB log the full row image.
        see
            - https://dev.mysql.com/doc/refman/8.0/en/replication-options-binary-log.html#sysvar_binlog_row_image
            - https://mariadb.com/kb/en/replication-and-binary-log-system-variables/#binlog_row_image

        ColumnBitmap1, ColumnBitmap2 and SkippedColumns are not set on the full row image.
    */
    // len = (ColumnCount + 7) / 8
    pub column_bitmap1: Rc<Vec<u8>>,

    // if UPDATE_ROWS_EVENTv1 or v2, or PARTIAL_UPDATE_ROWS_EVENT
    // len = (ColumnCount + 7) / 8
    pub column_bitmap2: Rc<Vec<u8>>,

    // rows: all return types from RowsEvent.decodeValue()
    pub rows: Vec<Vec<DecodeFieldData>>,
    pub skipped_columns: Vec<Vec<isize>>,

    pub parse_time: bool,
    pub timestamp_string_location: Option<chrono_tz::Tz>,
    pub use_decimal: bool,
    pub ignore_json_decode_err: bool,
}

impl Event for RowsEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "TableID: {}\n", self.table_id)?;
        write!(writer, "Flags: {}\n", self.flags)?;
        write!(writer, "Column count: {}\n", self.column_count)?;

        write!(writer, "Values:\n")?;
        for row in &self.rows {
            write!(writer, "--\n")?;
            for (j, d) in row.iter().enumerate() {
                write!(writer, "{}:{}\n", j, d.to_string())?;
            }
        }
        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        let pos = self.decode_header(data)?;
        if self.compressed {
            let uncompressed_data = mysql::decompress_mariadb_data(&data[pos as usize..])?;
            return self.decode_data(0, &uncompressed_data);
        }
        self.decode_data(pos, data)
    }
}

// EnumRowImageType is allowed types for every row in mysql binlog.
// See https://github.com/mysql/mysql-server/blob/1bfe02bdad6604d54913c62614bde57a055c8332/sql/rpl_record.h#L39
// enum class enum_row_image_type { WRITE_AI, UPDATE_BI, UPDATE_AI, DELETE_BI };
#[derive(Debug, Clone, PartialEq)]
pub enum EnumRowImageType {
    WriteAI = 0,
    UpdateBI = 1,
    UpdateAI = 2,
    DeleteBI = 3,
    Unknown = 99,
}

impl Display for EnumRowImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumRowImageType::WriteAI => write!(f, "WriteAI"),
            EnumRowImageType::UpdateBI => write!(f, "UpdateBI"),
            EnumRowImageType::UpdateAI => write!(f, "UpdateAI"),
            EnumRowImageType::DeleteBI => write!(f, "DeleteBI"),
            _ => {
                write!(f, "Unknown")
            }
        }
    }
}

impl From<u8> for EnumRowImageType {
    fn from(value: u8) -> Self {
        match value {
            0 => EnumRowImageType::WriteAI,
            1 => EnumRowImageType::UpdateBI,
            2 => EnumRowImageType::UpdateAI,
            3 => EnumRowImageType::DeleteBI,
            _ => EnumRowImageType::Unknown,
        }
    }
}

// Bits for binlog_row_value_options sysvar
pub enum EnumBinlogRowValueOptions {
    // Store JSON updates in partial form
    Unknown = 0,
    PartialJsonUpdates = 1,
}

impl From<u8> for EnumBinlogRowValueOptions {
    fn from(value: u8) -> Self {
        match value {
            1 => EnumBinlogRowValueOptions::PartialJsonUpdates,
            _ => EnumBinlogRowValueOptions::Unknown,
        }
    }
}

impl RowsEvent {
    pub fn decode_header(&mut self, data: &[u8]) -> Result<isize, ReplicationError> {
        let mut rdr = Cursor::new(data);
        self.table_id = mysql::fixed_length_int(&data[0..self.table_id_size as usize]);
        rdr.seek(SeekFrom::Current(self.table_id_size as i64))?;

        self.flags = rdr.read_u16::<LittleEndian>()?;

        if self.version == 2 {
            let data_len = rdr.read_u16::<LittleEndian>()?;
            let start = rdr.position() as usize;
            let stop = start + (data_len as usize - 2);
            self.extra_data = data[start..stop].to_vec();
            rdr.seek(SeekFrom::Current((data_len - 2) as i64))?;
        }

        let (column_count, _, n) = mysql::length_encoded_int(&data[rdr.position() as usize..]);
        self.column_count = column_count;
        rdr.seek(SeekFrom::Current(n as i64))?;

        let bit_count = bitmap_byte_size(self.column_count as isize);
        let start = rdr.position() as usize;
        let stop = start + bit_count as usize;
        self.column_bitmap1 = Rc::new(data[start..stop].to_vec());
        rdr.seek(SeekFrom::Current(bit_count as i64))?;

        if self.need_bitmap2 {
            let start = rdr.position() as usize;
            let stop = start + bit_count as usize;
            self.column_bitmap2 = Rc::new(data[start..stop].to_vec());
            rdr.seek(SeekFrom::Current(bit_count as i64))?;
        }

        if let Some(table) = self.tables.get(&self.table_id) {
            self.table = table.clone()
        } else {
            if self.tables.len() > 0 {
                return Err(ReplicationError::new(format!(
                    "invalid table id {}, no corresponding table map event",
                    self.table_id
                )));
            } else {
                return Err(ReplicationError::new(format!(
                    "{}, table id {}",
                    ERR_MISSING_TABLE_MAP_EVENT, self.table_id
                )));
            }
        }

        Ok(rdr.position() as isize)
    }

    pub fn decode_data(&mut self, pos: isize, data: &[u8]) -> Result<(), ReplicationError> {
        // Rows_log_event::print_verbose()
        // Pre-allocate memory for rows: before image + (optional) after image
        let mut rows_len = 1;
        if self.need_bitmap2 {
            rows_len += 1;
        }

        self.skipped_columns = Vec::<Vec<isize>>::with_capacity(rows_len);
        self.rows = Vec::<Vec<DecodeFieldData>>::with_capacity(rows_len);

        let row_image_type = match self.event_type {
            EventType::WriteRowsEventv0
            | EventType::WriteRowsEventv1
            | EventType::WriteRowsEventv2
            | EventType::MariadbWriteRowsCompressedEventV1 => EnumRowImageType::WriteAI,
            EventType::DeleteRowsEventv0
            | EventType::DeleteRowsEventv1
            | EventType::DeleteRowsEventv2
            | EventType::MariadbDeleteRowsCompressedEventV1 => EnumRowImageType::DeleteBI,
            _ => EnumRowImageType::UpdateBI,
        };

        let mut pos = pos as usize;
        while pos < data.len() {
            // Parse the first image
            let n = self.decode_image(
                &data[pos..],
                &self.column_bitmap1.clone(),
                row_image_type.clone(),
            )?;
            pos += n as usize;

            // Parse the second image (for UPDATE only)
            if self.need_bitmap2 {
                let n = self.decode_image(
                    &data[pos..],
                    &self.column_bitmap2.clone(),
                    EnumRowImageType::UpdateAI,
                )?;
                pos += n as usize;
            }
        }

        Ok(())
    }

    fn _is_bit_set(&self, bitmap: &[u8], i: isize) -> bool {
        bitmap[i as usize >> 3] & (1 << ((i as usize) & 7)) > 0
    }

    fn _is_bit_set_incr(&self, bitmap: &[u8], i: &mut isize) -> bool {
        let v = self._is_bit_set(bitmap, *i);
        *i += 1;

        v
    }

    pub fn decode_image(
        &mut self,
        data: &[u8],
        bitmap: &[u8],
        row_image_type: EnumRowImageType,
    ) -> Result<isize, ReplicationError> {
        // Rows_log_event::print_verbose_one_row()
        let mut pos = 0;
        let is_partial_json_update = false;
        let mut partial_bitmap = Vec::<u8>::new();

        if self.event_type == EventType::PartialUpdateRowsEvent
            && row_image_type == EnumRowImageType::UpdateAI
        {
            // binlog_row_value_options
            let (binlog_row_value_options, _, n) = mysql::length_encoded_int(&data[pos..]);
            pos += n as usize;

            let is_partial_json_update =
                (EnumBinlogRowValueOptions::from(binlog_row_value_options as u8) as u8)
                    & (EnumBinlogRowValueOptions::PartialJsonUpdates as u8)
                    != 0;
            if is_partial_json_update {
                let byte_count = bitmap_byte_size(self.table.json_column_count() as isize) as usize;
                partial_bitmap = data[pos..pos + byte_count].to_vec();
                pos += byte_count;
            }
        }

        let mut row = vec![DecodeFieldData::None; self.column_count as usize];
        let mut skips = Vec::<isize>::new();

        // refer: https://github.com/alibaba/canal/blob/c3e38e50e269adafdd38a48c63a1740cde304c67/dbsync/src/main/java/com/taobao/tddl/dbsync/binlog/event/RowsLogBuffer.java#L63
        let mut count = 0;
        for i in 0..self.column_count as isize {
            if decode_helper::is_bit_set(bitmap, i) {
                count += 1;
            }
        }
        count = bitmap_byte_size(count);

        let null_bit_map = &data[pos..pos + count as usize];
        pos += count as usize;

        let mut partial_bitmap_index = 0;
        let mut null_bitmap_index = 0;

        for i in 0..self.column_count as isize {
            /*
               Note: need to read partial bit before reading cols_bitmap, since
               the partial_bits bitmap has a bit for every JSON column
               regardless of whether it is included in the bitmap or not.
            */
            let is_partial = is_partial_json_update
                && (row_image_type == EnumRowImageType::UpdateAI)
                && decode_helper::is_bit_set_incr(&partial_bitmap, &mut partial_bitmap_index);

            if !decode_helper::is_bit_set(bitmap, i) {
                skips.push(i);
                continue;
            }

            if decode_helper::is_bit_set_incr(&null_bit_map, &mut null_bitmap_index) {
                row[i as usize] = DecodeFieldData::None;
                continue;
            }

            let (field_data, n) = self.decode_value(
                &data[pos..],
                self.table.column_type[i as usize],
                self.table.column_meta[i as usize],
                is_partial,
            )?;
            row[i as usize] = field_data;
            pos += n as usize;
        }

        self.rows.push(row);
        self.skipped_columns.push(skips);

        Ok(pos as isize)
    }

    fn _parse_frac_time(&self, t: &DecodeDatetime) -> DecodeDatetime {
        let v = match t {
            DecodeDatetime::String(v) => return DecodeDatetime::String(v.clone()),
            DecodeDatetime::FracTime(v) => v,
        };

        if !self.parse_time {
            return DecodeDatetime::String(v.to_string());
        }

        DecodeDatetime::FracTime(v.clone())
    }

    // see mysql sql/log_event.cc log_event_print_value
    pub fn decode_value(
        &mut self,
        data: &[u8],
        tp: u8,
        meta: u16,
        is_partial: bool,
    ) -> Result<(DecodeFieldData, isize), ReplicationError> {
        let mut length = 0;
        let mut tp = tp;

        if tp == mysql::MYSQL_TYPE_STRING {
            if meta >= 256 {
                let b0 = (meta >> 8) as u8;
                let b1 = (meta & 0xFF) as u8;

                if b0 & 0x30 != 0x30 {
                    length = ((b1 as u16) | (((b0 & 0x30) ^ 0x30) as u16) << 4) as isize;
                    tp = b0 | 0x30;
                } else {
                    length = (meta & 0xFF) as isize;
                    tp = b0;
                }
            } else {
                length = meta as isize;
            }
        }

        let rs = match tp {
            mysql::MYSQL_TYPE_NULL => return Ok((DecodeFieldData::None, 0)),
            mysql::MYSQL_TYPE_LONG => Ok((
                DecodeFieldData::Isize(ParseBinary::i32_little_endian(data)? as isize),
                4,
            )),
            mysql::MYSQL_TYPE_TINY => Ok((
                DecodeFieldData::Isize(ParseBinary::i8_little_endian(data) as isize),
                1,
            )),
            mysql::MYSQL_TYPE_SHORT => Ok((
                DecodeFieldData::Isize(ParseBinary::i16_little_endian(data)? as isize),
                2,
            )),
            mysql::MYSQL_TYPE_INT24 => Ok((
                DecodeFieldData::Isize(ParseBinary::i24_little_endian(data) as isize),
                3,
            )),
            mysql::MYSQL_TYPE_LONGLONG => Ok((
                DecodeFieldData::Isize(ParseBinary::i64_little_endian(data)? as isize),
                8,
            )),
            mysql::MYSQL_TYPE_NEWDECIMAL => {
                let prec = (meta >> 8) as u8;
                let scale = (meta & 0xFF) as u8;
                let (v, n) = decode_helper::decode_decimal(
                    data,
                    prec as isize,
                    scale as isize,
                    self.use_decimal,
                )?;
                Ok((DecodeFieldData::Decimal(v), n))
            }
            mysql::MYSQL_TYPE_FLOAT => Ok((
                DecodeFieldData::F64(ParseBinary::f32_little_endian(data)? as f64),
                4,
            )),
            mysql::MYSQL_TYPE_DOUBLE => Ok((
                DecodeFieldData::F64(ParseBinary::f64_little_endian(data)?),
                8,
            )),
            mysql::MYSQL_TYPE_BIT => {
                let nbits = ((meta >> 8) * 8) + (meta & 0xFF);
                let n = (nbits + 7) as isize / 8;

                // use int64 for bit
                let v = decode_helper::decode_bit(data, nbits as isize, n)?;
                Ok((DecodeFieldData::Isize(v as isize), n))
            }
            mysql::MYSQL_TYPE_TIMESTAMP => {
                let n = 4;
                let mut rdr = Cursor::new(data);
                let t = rdr.read_u32::<LittleEndian>()?;
                let v = if t == 0 {
                    DecodeDatetime::String(replication::format_zero_time(0, 0))
                } else {
                    self._parse_frac_time(&DecodeDatetime::FracTime(FracTime {
                        f_time: NaiveDateTime::from_timestamp_opt(t as i64, 0).unwrap(),
                        dec: 0,
                        timestamp_string_location: self.timestamp_string_location.clone(),
                    }))
                };
                Ok((DecodeFieldData::Datetime(v), n))
            }
            mysql::MYSQL_TYPE_TIMESTAMP2 => {
                let (v, n) = decode_helper::decode_timestamp2(
                    data,
                    meta,
                    self.timestamp_string_location.clone(),
                )?;
                let v = self._parse_frac_time(&v);
                Ok((DecodeFieldData::Datetime(v), n))
            }
            mysql::MYSQL_TYPE_DATETIME => {
                let n = 8;
                let mut rdr = Cursor::new(data);
                let uint64 = rdr.read_u64::<LittleEndian>()?;
                let v = if uint64 == 0 {
                    DecodeDatetime::String(replication::format_zero_time(0, 0))
                } else {
                    let d = uint64 / 1000001;
                    let t = uint64 % 1000001;

                    let year = (d / 10000) as i32;
                    let month = ((d % 10000) / 100) as u32;
                    let day = (d % 100) as u32;
                    let hour = (t / 10000) as u32;
                    let min = ((t % 10000) / 100) as u32;
                    let sec = (t % 100) as u32;
                    self._parse_frac_time(&DecodeDatetime::FracTime(FracTime {
                        f_time: NaiveDate::from_ymd_opt(year, month, day)
                            .unwrap()
                            .and_hms_opt(hour, min, sec)
                            .unwrap(),
                        dec: 0,
                        timestamp_string_location: None,
                    }))
                };
                Ok((DecodeFieldData::Datetime(v), n))
            }
            mysql::MYSQL_TYPE_DATETIME2 => {
                let (v, n) = decode_helper::decode_datetime2(data, meta)?;
                Ok((DecodeFieldData::Datetime(self._parse_frac_time(&v)), n))
            }
            mysql::MYSQL_TYPE_TIME => {
                let n = 3;
                let int32 = mysql::fixed_length_int(&data[0..3]) as u32;
                let v = if int32 == 0 {
                    DecodeFieldData::String(String::from("00:00:00"))
                } else {
                    DecodeFieldData::String(format!(
                        "{:02}:{:02}:{:02}",
                        int32 / 10000,
                        (int32 % 10000) / 100,
                        int32 % 100
                    ))
                };
                Ok((v, n))
            }
            mysql::MYSQL_TYPE_TIME2 => {
                let (v, n) = decode_helper::decode_time2(data, meta)?;
                Ok((DecodeFieldData::String(v), n))
            }
            mysql::MYSQL_TYPE_DATE => {
                let n = 3;
                let int32 = mysql::fixed_length_int(&data[0..3]) as u32;
                let v = if int32 == 0 {
                    DecodeFieldData::String(String::from("0000-00-00"))
                } else {
                    DecodeFieldData::String(format!(
                        "{:04}-{:02}-{:02}",
                        int32 / (16 * 32),
                        int32 / 32 % 16,
                        int32 % 32
                    ))
                };
                Ok((v, n))
            }
            mysql::MYSQL_TYPE_YEAR => {
                let n = 1;
                let year = data[0] as isize;
                let v = if year == 0 { year } else { year + 1900 };
                Ok((DecodeFieldData::Isize(v), n))
            }
            mysql::MYSQL_TYPE_ENUM => {
                let l = meta & 0xFF;
                let (v, n) = match l {
                    1 => Ok((data[0] as isize, 1)),
                    2 => {
                        let mut rdr = Cursor::new(data);
                        let v = rdr.read_u16::<LittleEndian>()?;
                        Ok((v as isize, 2))
                    }
                    _ => Err(ReplicationError::new(format!("Unknown ENUM packlen={}", l))),
                }?;
                Ok((DecodeFieldData::Isize(v), n))
            }
            mysql::MYSQL_TYPE_SET => {
                let n = (meta & 0xFF) as isize;
                let nbits = n * 8;
                let v = decode_helper::little_decode_bit(data, nbits, n)? as isize;

                Ok((DecodeFieldData::Isize(v), n))
            }
            mysql::MYSQL_TYPE_BLOB => {
                let (v, n) = decode_helper::decode_blob(data, meta)?;
                Ok((DecodeFieldData::Bytes(v), n))
            }
            mysql::MYSQL_TYPE_VARCHAR | mysql::MYSQL_TYPE_VAR_STRING => {
                let length = meta as isize;
                let (v, n) = decode_helper::decode_string(data, length)?;
                Ok((DecodeFieldData::String(v), n))
            }
            mysql::MYSQL_TYPE_STRING => {
                let (v, n) = decode_helper::decode_string(data, length)?;
                Ok((DecodeFieldData::String(v), n))
            }
            mysql::MYSQL_TYPE_JSON => {
                // Refer: https://github.com/shyiko/mysql-binlog-connector-java/blob/master/src/main/java/com/github/shyiko/mysql/binlog/event/deserialization/AbstractRowsEventDataDeserializer.java#L404
                let length = mysql::fixed_length_int(&data[0..meta as usize]) as isize;
                let n = length + meta as isize;
                /*
                   See https://github.com/mysql/mysql-server/blob/7b6fb0753b428537410f5b1b8dc60e5ccabc9f70/sql-common/json_binary.cc#L1077

                   Each document should start with a one-byte type specifier, so an
                   empty document is invalid according to the format specification.
                   Empty documents may appear due to inserts using the IGNORE keyword
                   or with non-strict SQL mode, which will insert an empty string if
                   the value NULL is inserted into a NOT NULL column. We choose to
                   interpret empty values as the JSON null literal.

                   In our implementation (go-mysql) for backward compatibility we prefer return empty slice.
                */
                let v = if length == 0 {
                    DecodeJson::Bytes(vec![])
                } else {
                    if is_partial {
                        let diff = self
                            ._decode_json_partial_binary(&data[meta as usize..n as usize])
                            .map_err(|e| {
                                ReplicationError::new(format!(
                                    "decodeJsonPartialBinary({:?}) fail: {}\n",
                                    &data[meta as usize..n as usize],
                                    e.to_string()
                                ))
                            })?;
                        DecodeJson::JsonDiff(diff)
                    } else {
                        let d = self._decode_json_binary(&data[meta as usize..n as usize])?;
                        DecodeJson::String(String::from_utf8_lossy(&d).to_string())
                    }
                };

                Ok((DecodeFieldData::Json(v), n))
            }
            mysql::MYSQL_TYPE_GEOMETRY => {
                // MySQL saves Geometry as Blob in binlog
                // Seem that the binary format is SRID (4 bytes) + WKB, outer can use
                // MySQL GeoFromWKB or others to create the geometry data.
                // Refer https://dev.mysql.com/doc/refman/5.7/en/gis-wkb-functions.html
                // I also find some go libs to handle WKB if possible
                // see https://github.com/twpayne/go-geom or https://github.com/paulmach/go.geo
                let (v, n) = decode_helper::decode_blob(data, meta)?;
                Ok((DecodeFieldData::Bytes(v), n))
            }
            _ => Err(ReplicationError::new(format!(
                "unsupport type {} in binlog and don't know how to handle",
                tp
            ))),
        };

        rs
    }

    // decodeJsonBinary decodes the JSON binary encoding data and returns
    // the common JSON encoding data.
    fn _decode_json_binary(&mut self, data: &[u8]) -> Result<Vec<u8>, ReplicationError> {
        let mut d = JsonBinaryDecoder {
            use_decimal: self.use_decimal,
            ignore_decode_err: self.ignore_json_decode_err,
            err: Ok(()),
        };

        if d.is_data_short(data, 1) {
            return match d.err {
                Ok(_) => Ok(vec![]),
                Err(e) => Err(e),
            };
        }

        let v = d.decode_value(data[0], &data[1..]);
        if let Err(e) = d.err {
            return Err(e);
        }

        Ok(serde_json::to_vec(&v)?)
    }

    fn _decode_json_partial_binary(&mut self, data: &[u8]) -> Result<JsonDiff, ReplicationError> {
        // see Json_diff_vector::read_binary() in mysql-server/sql/json_diff.cc
        let operation_number = JsonDiffOperation::from(data[0]);
        match operation_number {
            JsonDiffOperation::Replace => {}
            JsonDiffOperation::Insert => {}
            JsonDiffOperation::Remove => {}
            JsonDiffOperation::Unknown => {
                return Err(ReplicationError::new(format!(
                    "{}",
                    replication::ERR_CORRUPTED_JSON_DIFF
                )))
            }
        }

        let data = &data[1..];
        let (path_length, _, n) = mysql::length_encoded_int(data);
        let data = &data[n as usize..];

        let path = &data[..path_length as usize];
        let data = &data[path_length as usize..];

        let mut diff = JsonDiff {
            op: operation_number.clone(),
            path: String::from_utf8_lossy(path).to_string(),
            // Value will be filled below
            value: String::from(""),
        };

        if operation_number == JsonDiffOperation::Remove {
            return Ok(diff);
        }

        let (value_length, _, n) = mysql::length_encoded_int(data);
        let data = &data[n as usize..];

        let d = self
            ._decode_json_binary(&data[..value_length as usize])
            .map_err(|e| {
                ReplicationError::new(format!(
                    "cannot read json diff for field {}: {}",
                    String::from_utf8_lossy(path),
                    e.to_string()
                ))
            })?;

        diff.value = String::from_utf8_lossy(&d).to_string();

        Ok(diff)
    }
}

#[derive(Debug, Default, Clone)]
pub struct RowsQueryEvent {
    pub query: Vec<u8>,
}

impl Event for RowsQueryEvent {
    fn dump<W: Write>(&mut self, writer: &mut W) -> Result<(), ReplicationError> {
        write!(writer, "Query: {}\n", String::from_utf8_lossy(&self.query))?;
        writeln!(writer)?;

        Ok(())
    }

    fn decode(&mut self, data: &[u8]) -> Result<(), ReplicationError> {
        // ignore length byte 1
        self.query = data[1..].to_vec();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a() {
        let a = 33;
        println!("aa {{:<{len}}}", len = a);
    }
}
