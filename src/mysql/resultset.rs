use crate::error::ReplicationError;
use crate::mysql::{Field, FieldValue, FieldValueEnum, RowData};
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum StreamingType {
    // StreamingNone means there is no streaming
    None = 0,
    // StreamingSelect is used with select queries for which each result is
    // directly returned to the client
    Select = 1,
    // StreamingMultiple is used when multiple queries are given at once
    // usually in combination with SERVER_MORE_RESULTS_EXISTS flag set
    Multiple = 2,
}

impl std::fmt::Display for StreamingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamingType::None => write!(f, "{}", "None"),
            StreamingType::Select => write!(f, "{}", "Select"),
            StreamingType::Multiple => write!(f, "{}", "Multiple"),
        }
    }
}

impl Default for StreamingType {
    fn default() -> Self {
        StreamingType::None
    }
}

impl StreamingType {
    pub fn new(typ: isize) -> StreamingType {
        match typ {
            1 => StreamingType::Select,
            2 => StreamingType::Multiple,
            _ => StreamingType::None,
        }
    }

    pub fn from_isize(typ: isize) -> StreamingType {
        match typ {
            1 => StreamingType::Select,
            2 => StreamingType::Multiple,
            _ => StreamingType::None,
        }
    }

    pub fn to_isize(&self) -> isize {
        match self {
            StreamingType::None => 0,
            StreamingType::Select => 1,
            StreamingType::Multiple => 2,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResultSet {
    pub fields: Vec<Field>,
    pub field_names: HashMap<String, usize>,
    pub values: Vec<Vec<FieldValue>>,
    pub raw_pkg: Vec<u8>,
    pub row_datas: Vec<RowData>,
    pub streaming: StreamingType,
    pub streaming_done: bool,
}

impl ResultSet {
    pub fn new() -> ResultSet {
        let rs = ResultSet::default();
        rs
    }

    pub fn row_number(&self) -> usize {
        return self.values.len();
    }

    pub fn column_number(&self) -> usize {
        return self.fields.len();
    }

    pub fn get_value(&self, row: usize, column: usize) -> Result<FieldValueEnum, ReplicationError> {
        if row >= self.values.len() {
            return Err(ReplicationError::new(format!("invalid row index {}", row)));
        }

        if column >= self.fields.len() {
            return Err(ReplicationError::new(format!(
                "invalid column index {}",
                column
            )));
        }

        return Ok(self.values[row][column].value());
    }

    pub fn name_index(&self, name: &str) -> Result<usize, ReplicationError> {
        if let Some(column) = self.field_names.get(name) {
            Ok(*column)
        } else {
            Err(ReplicationError::new(format!(
                "invalid field name {}",
                name
            )))
        }
    }

    pub fn get_value_by_name(
        &self,
        row: usize,
        name: &str,
    ) -> Result<FieldValueEnum, ReplicationError> {
        let column = self.name_index(name)?;
        self.get_value(row, column)
    }

    pub fn is_null(&self, row: usize, column: usize) -> Result<bool, ReplicationError> {
        let d = self.get_value(row, column)?;
        Ok(d == FieldValueEnum::None)
    }

    pub fn is_null_by_name(&self, row: usize, name: &str) -> Result<bool, ReplicationError> {
        let column = self.name_index(name)?;
        self.is_null(row, column)
    }

    pub fn get_uint(&self, row: usize, column: usize) -> Result<u64, ReplicationError> {
        let d = self.get_value(row, column)?;
        let v = match d {
            FieldValueEnum::U64(v) => v,
            FieldValueEnum::I64(v) => v as u64,
            FieldValueEnum::F64(v) => v.to_bits(),
            FieldValueEnum::String(v) => String::from_utf8_lossy(&v).to_string().parse::<u64>()?,
            FieldValueEnum::None => 0,
        };

        Ok(v)
    }

    pub fn get_uint_by_name(&self, row: usize, name: &str) -> Result<u64, ReplicationError> {
        let column = self.name_index(name)?;
        self.get_uint(row, column)
    }

    pub fn get_int(&self, row: usize, column: usize) -> Result<i64, ReplicationError> {
        let v = self.get_uint(row, column)?;
        Ok(v as i64)
    }

    pub fn get_int_by_name(&self, row: usize, name: &str) -> Result<i64, ReplicationError> {
        let v = self.get_uint_by_name(row, name)?;
        Ok(v as i64)
    }

    pub fn get_float(&self, row: usize, column: usize) -> Result<f64, ReplicationError> {
        let d = self.get_value(row, column)?;
        let v = match d {
            FieldValueEnum::None => 0.0,
            FieldValueEnum::U64(v) => f64::from_bits(v),
            FieldValueEnum::I64(v) => f64::from_bits(v as u64),
            FieldValueEnum::F64(v) => v,
            FieldValueEnum::String(v) => String::from_utf8_lossy(&v).to_string().parse::<f64>()?,
        };

        Ok(v)
    }

    pub fn get_float_by_name(&self, row: usize, name: &str) -> Result<f64, ReplicationError> {
        let column = self.name_index(name)?;
        self.get_float(row, column)
    }

    pub fn get_string(&self, row: usize, column: usize) -> Result<String, ReplicationError> {
        let d = self.get_value(row, column)?;
        let v = match d {
            FieldValueEnum::None => String::new(),
            FieldValueEnum::U64(v) => v.to_string(),
            FieldValueEnum::I64(v) => v.to_string(),
            FieldValueEnum::F64(v) => v.to_string(),
            FieldValueEnum::String(v) => String::from_utf8_lossy(&v).to_string(),
        };

        Ok(v)
    }

    pub fn get_string_by_name(&self, row: usize, name: &str) -> Result<String, ReplicationError> {
        let column = self.name_index(name)?;
        self.get_string(row, column)
    }
}
