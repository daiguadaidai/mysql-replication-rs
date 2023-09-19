use crate::replication::FracTime;
use bigdecimal::BigDecimal;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum DecodeFieldData {
    None,
    Char(char),
    Isize(isize),
    Usize(usize),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Datetime(DecodeDatetime),
    Decimal(DecodeDecimal),
    Json(DecodeJson),
}

impl Display for DecodeFieldData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeFieldData::None => write!(f, "null"),
            DecodeFieldData::Char(ref r) => r.fmt(f),
            DecodeFieldData::Isize(ref r) => r.fmt(f),
            DecodeFieldData::Usize(ref r) => r.fmt(f),
            DecodeFieldData::String(ref r) => r.fmt(f),
            DecodeFieldData::Datetime(ref r) => r.fmt(f),
            DecodeFieldData::Decimal(ref r) => r.fmt(f),
            DecodeFieldData::Json(ref r) => r.fmt(f),
            DecodeFieldData::F64(ref r) => r.fmt(f),
            DecodeFieldData::Bytes(ref r) => write!(
                f,
                "[{}]",
                r.iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeDatetime {
    String(String),
    FracTime(FracTime),
}

impl Display for DecodeDatetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeDatetime::String(ref r) => r.fmt(f),
            DecodeDatetime::FracTime(ref r) => r.fmt(f),
        }
    }
}

impl Serialize for DecodeDatetime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DecodeDatetime::String(v) => serializer.serialize_str(v),
            DecodeDatetime::FracTime(v) => serializer.serialize_str(&v.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeDecimal {
    String(String),
    Decimal(BigDecimal),
    Unknown,
}

impl Display for DecodeDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeDecimal::String(ref r) => r.fmt(f),
            DecodeDecimal::Decimal(ref r) => r.fmt(f),
            _ => {
                write!(f, "")
            }
        }
    }
}

impl Serialize for DecodeDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DecodeDecimal::String(v) => v.serialize(serializer),
            DecodeDecimal::Decimal(v) => serializer.serialize_str(&v.to_string()),
            _ => serializer.serialize_unit(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum DecodeJson {
    None,
    Bool(bool),
    String(String),
    Decimal(DecodeDecimal),
    Datetime(DecodeDatetime),
    Isize(isize),
    Usize(usize),
    F64(f64),
    Bytes(Vec<u8>),
    Vec(Vec<DecodeJson>),
    Map(HashMap<String, DecodeJson>),
    JsonDiff(JsonDiff),
    Unknown,
}

impl Display for DecodeJson {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeJson::String(ref r) => r.fmt(f),
            DecodeJson::None => write!(f, "null"),
            DecodeJson::Bool(ref r) => r.fmt(f),
            DecodeJson::Decimal(ref r) => r.fmt(f),
            DecodeJson::Datetime(ref r) => r.fmt(f),
            DecodeJson::Isize(r) => write!(f, "{}", r),
            DecodeJson::Usize(r) => write!(f, "{}", r),
            DecodeJson::F64(r) => write!(f, "{}", r),
            DecodeJson::Bytes(ref r) => write!(f, "{:?}", r),
            DecodeJson::Vec(ref r) => write!(f, "{}", serde_json::to_string(r).unwrap()),
            DecodeJson::Map(ref r) => write!(f, "{}", serde_json::to_string(r).unwrap()),
            DecodeJson::JsonDiff(ref r) => r.fmt(f),
            _ => {
                write!(f, "unknown")
            }
        }
    }
}

// JsonDiffOperation is an enum that describes what kind of operation a JsonDiff object represents.
// https://github.com/mysql/mysql-server/blob/8.0/sql/json_diff.h
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum JsonDiffOperation {
    // The JSON value in the given path is replaced with a new value.
    //
    // It has the same effect as `JSON_REPLACE(col, path, value)`.
    Replace = 0,

    // Add a new element at the given path.
    //
    //  If the path specifies an array element, it has the same effect as `JSON_ARRAY_INSERT(col, path, value)`.
    //
    //  If the path specifies an object member, it has the same effect as `JSON_INSERT(col, path, value)`.
    Insert = 1,

    // The JSON value at the given path is removed from an array or object.
    //
    // It has the same effect as `JSON_REMOVE(col, path)`.
    Remove = 2,

    Unknown = 99,
}

impl From<u8> for JsonDiffOperation {
    fn from(value: u8) -> Self {
        match value {
            0 => JsonDiffOperation::Replace,
            1 => JsonDiffOperation::Insert,
            2 => JsonDiffOperation::Remove,
            _ => JsonDiffOperation::Unknown,
        }
    }
}

impl Display for JsonDiffOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonDiffOperation::Replace => write!(f, "Replace"),
            JsonDiffOperation::Insert => write!(f, "Insert"),
            JsonDiffOperation::Remove => write!(f, "Remove"),
            _ => write!(f, "Unknown({})", self),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct JsonDiff {
    pub op: JsonDiffOperation,
    pub path: String,
    pub value: String,
}

impl Display for JsonDiff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "json_diff(op:{} path:{} value:{})",
            self.op, &self.path, &self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::common::row_fields::{DecodeDecimal, DecodeFieldData, DecodeJson};
    use bigdecimal::BigDecimal;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn d() {
        let mut m = HashMap::<String, DecodeJson>::new();
        m.insert("col_1".to_string(), DecodeJson::Isize(1));
        m.insert("col_2".to_string(), DecodeJson::Unknown);
        m.insert("col_3".to_string(), DecodeJson::None);
        m.insert("col_4".to_string(), DecodeJson::F64(1.1));
        m.insert("col_5".to_string(), DecodeJson::Bool(true));
        m.insert(
            "col_6".to_string(),
            DecodeJson::Decimal(DecodeDecimal::Decimal("2.3".parse().unwrap())),
        );

        let json_data = DecodeJson::Map(m);
        let field_data = DecodeFieldData::Json(json_data);
        println!(
            "decode_json: {}",
            serde_json::to_string(&field_data).unwrap()
        );
    }

    #[test]
    fn c() {
        let mut m = HashMap::<String, DecodeJson>::new();
        m.insert("col_1".to_string(), DecodeJson::Isize(1));
        m.insert("col_2".to_string(), DecodeJson::Unknown);
        m.insert("col_3".to_string(), DecodeJson::None);
        m.insert("col_4".to_string(), DecodeJson::F64(1.1));
        m.insert("col_5".to_string(), DecodeJson::Bool(true));

        let mut m1 = HashMap::<String, DecodeJson>::new();
        m1.insert("col_1".to_string(), DecodeJson::Isize(2));
        m1.insert("col_2".to_string(), DecodeJson::Unknown);
        m1.insert("col_3".to_string(), DecodeJson::None);
        m1.insert("col_4".to_string(), DecodeJson::F64(2.2));
        m1.insert("col_5".to_string(), DecodeJson::Bool(false));
        m1.insert("col_6".to_string(), DecodeJson::Map(m));

        let decode_json = DecodeJson::Map(m1);
        println!(
            "decode_json: {}",
            serde_json::to_string(&decode_json).unwrap()
        );
    }

    #[test]
    fn b() {
        let decode_json = DecodeJson::Decimal(DecodeDecimal::Decimal(
            BigDecimal::from_str("1.33").unwrap(),
        ));
        println!(
            "decode_json: {}",
            serde_json::to_string(&decode_json).unwrap()
        );
    }

    #[test]
    fn a() {
        let decode_json = DecodeDecimal::Decimal(BigDecimal::from_str("1.33").unwrap());
        println!(
            "decode_json: {}",
            serde_json::to_string(&decode_json).unwrap()
        );
    }
}
