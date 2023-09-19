use crate::error::{MysqlError, ReplicationError};
use crate::mysql::{
    length_encoded_int, length_encoded_string, put_length_encoded_string,
    skip_length_encoded_string, uint16_to_bytes, uint32_to_bytes, uint64_to_bytes,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Seek, SeekFrom};

#[derive(Debug, Clone, Default)]
pub struct Field {
    pub data: Vec<u8>,
    pub schema: Vec<u8>,
    pub table: Vec<u8>,
    pub org_table: Vec<u8>,
    pub name: Vec<u8>,
    pub org_name: Vec<u8>,
    pub charset: u16,
    pub column_length: u32,
    pub typ: u8,
    pub flag: u16,
    pub decimal: u8,

    pub default_value_length: u64,
    pub default_value: Vec<u8>,
}

impl Field {
    pub fn parse_from_bytes(field_data: Vec<u8>) -> Result<Field, ReplicationError> {
        let mut field = Field::default();
        let _ = field.parse(field_data);

        Ok(field)
    }

    pub fn parse(&mut self, p: Vec<u8>) -> Result<(), ReplicationError> {
        let mut pos = 0_usize;
        self.data = p;
        //skip catelog, always def
        let n = skip_length_encoded_string(&self.data)?;
        pos += n;

        //schema
        let (schema, _, n) = length_encoded_string(&self.data[pos..])?;
        self.schema = schema;
        pos += n as usize;

        //table
        let (table, _, n) = length_encoded_string(&self.data[pos..])?;
        self.table = table;
        pos += n as usize;

        //org_table
        let (org_table, _, n) = length_encoded_string(&self.data[pos..])?;
        self.org_table = org_table;
        pos += n as usize;

        //name
        let (name, _, n) = length_encoded_string(&self.data[pos..])?;
        self.name = name;
        pos += n as usize;

        //name
        let (org_name, _, n) = length_encoded_string(&self.data[pos..])?;
        self.org_name = org_name;
        pos += n as usize;

        //skip oc
        pos += 1;

        let mut rdr = Cursor::new(&self.data);
        rdr.seek(SeekFrom::Start(pos as u64))?;

        //charset
        self.charset = rdr.read_u16::<LittleEndian>()?;

        //column length
        self.column_length = rdr.read_u32::<LittleEndian>()?;

        //type
        self.typ = rdr.read_u8()?;

        //flag
        self.flag = rdr.read_u16::<LittleEndian>()?;

        //decimals 1
        self.decimal = rdr.read_u8()?;

        //filter [0x00][0x00]
        rdr.seek(SeekFrom::Current(2))?;
        pos = rdr.position() as usize;

        //if more data, command was field list
        if self.data.len() > pos {
            //length of default value lenenc-int
            let (default_value_length, _, n) = length_encoded_int(&self.data[pos..]);
            self.default_value_length = default_value_length;
            pos += n;

            let default_value_start = pos + self.default_value_length as usize;
            if default_value_start > self.data.len() {
                return Err(ReplicationError::MysqlError(MysqlError::ErrMalformPacket));
            }

            //default value string[$len]
            self.default_value = self.data[pos..default_value_start].to_vec();
        }

        Ok(())
    }

    pub fn dump(&self) -> Vec<u8> {
        if self.data.len() == 0 {
            return vec![];
        }

        let l = self.schema.len()
            + self.table.len()
            + self.org_table.len()
            + self.name.len()
            + self.org_name.len()
            + self.default_value.len()
            + 48;

        let mut data = Vec::<u8>::with_capacity(l);
        data.extend(put_length_encoded_string(b"def"));
        data.extend(put_length_encoded_string(&self.schema));
        data.extend(put_length_encoded_string(&self.table));
        data.extend(put_length_encoded_string(&self.org_table));
        data.extend(put_length_encoded_string(&self.name));
        data.extend(put_length_encoded_string(&self.org_name));
        data.push(0x0c);
        data.extend(uint16_to_bytes(self.charset));
        data.extend(uint32_to_bytes(self.column_length));
        data.push(self.typ);
        data.extend(uint16_to_bytes(self.flag));
        data.push(self.decimal);
        data.extend(vec![0, 0]);

        if self.default_value.len() > 0 {
            data.extend(uint64_to_bytes(self.default_value_length));
            data.extend(&self.default_value)
        }

        data
    }
}

#[derive(Debug, Clone)]
pub enum FieldValueType {
    None = 0,
    Unsigned = 1,
    Signed = 2,
    Float = 3,
    String = 4,
}

impl Default for FieldValueType {
    fn default() -> Self {
        FieldValueType::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldValue {
    pub typ: FieldValueType,
    pub value: u64, // Also for int64 and float64
    pub string: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum FieldValueEnum {
    None,
    U64(u64),
    I64(i64),
    F64(f64),
    String(Vec<u8>),
}

impl FieldValue {
    pub fn as_uint64(&self) -> FieldValueEnum {
        FieldValueEnum::U64(self.value)
    }

    pub fn as_int64(&self) -> FieldValueEnum {
        FieldValueEnum::I64(self.value as i64)
    }

    pub fn as_float64(&self) -> FieldValueEnum {
        FieldValueEnum::F64(f64::from_bits(self.value))
    }

    pub fn as_string(&self) -> FieldValueEnum {
        FieldValueEnum::String(self.string.clone())
    }

    pub fn value(&self) -> FieldValueEnum {
        match self.typ {
            FieldValueType::None => FieldValueEnum::None,
            FieldValueType::Unsigned => self.as_uint64(),
            FieldValueType::Signed => self.as_int64(),
            FieldValueType::Float => self.as_float64(),
            FieldValueType::String => self.as_string(),
        }
    }
}
