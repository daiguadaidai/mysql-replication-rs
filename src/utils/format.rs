use rt_format::{Format, FormatArgument, ParsedFormat, Specifier};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Result as FmtResult};

#[derive(Debug, PartialEq)]
pub enum FVariant {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Float32(f32),
    Float64(f64),
    String(String),
    Str(&'static str),
    Char(char),
}

impl Display for FVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> FmtResult {
        match self {
            Self::Int8(val) => fmt::Display::fmt(&val, f),
            Self::Int16(val) => fmt::Display::fmt(&val, f),
            Self::Int32(val) => fmt::Display::fmt(&val, f),
            Self::Int64(val) => fmt::Display::fmt(&val, f),
            Self::Int128(val) => fmt::Display::fmt(&val, f),
            Self::Uint8(val) => fmt::Display::fmt(&val, f),
            Self::Uint16(val) => fmt::Display::fmt(&val, f),
            Self::Uint32(val) => fmt::Display::fmt(&val, f),
            Self::Uint64(val) => fmt::Display::fmt(&val, f),
            Self::Uint128(val) => fmt::Display::fmt(&val, f),
            Self::Float32(val) => fmt::Display::fmt(&val, f),
            Self::Float64(val) => fmt::Display::fmt(&val, f),
            Self::String(val) => fmt::Display::fmt(&val, f),
            Self::Str(val) => fmt::Display::fmt(&val, f),
            Self::Char(val) => fmt::Display::fmt(&val, f),
        }
    }
}

impl FormatArgument for FVariant {
    fn supports_format(&self, spec: &Specifier) -> bool {
        match self {
            Self::Int8(_)
            | Self::Int16(_)
            | Self::Int32(_)
            | Self::Int64(_)
            | Self::Int128(_)
            | Self::Uint8(_)
            | Self::Uint16(_)
            | Self::Uint32(_)
            | Self::Uint64(_)
            | Self::Uint128(_) => true,
            Self::Float32(_) | Self::Float64(_) | Self::Char(_) => match spec.format {
                Format::Display | Format::Debug | Format::LowerExp | Format::UpperExp => true,
                _ => false,
            },
            Self::String(_) | Self::Str(_) => match spec.format {
                Format::Display | Format::Debug => true,
                _ => false,
            },
        }
    }

    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::Display::fmt(&val, f),
            Self::Int16(val) => fmt::Display::fmt(&val, f),
            Self::Int32(val) => fmt::Display::fmt(&val, f),
            Self::Int64(val) => fmt::Display::fmt(&val, f),
            Self::Int128(val) => fmt::Display::fmt(&val, f),
            Self::Uint8(val) => fmt::Display::fmt(&val, f),
            Self::Uint16(val) => fmt::Display::fmt(&val, f),
            Self::Uint32(val) => fmt::Display::fmt(&val, f),
            Self::Uint64(val) => fmt::Display::fmt(&val, f),
            Self::Uint128(val) => fmt::Display::fmt(&val, f),
            Self::Float32(val) => fmt::Display::fmt(&val, f),
            Self::Float64(val) => fmt::Display::fmt(&val, f),
            Self::String(val) => fmt::Display::fmt(&val, f),
            Self::Str(val) => fmt::Display::fmt(&val, f),
            Self::Char(val) => fmt::Display::fmt(&val, f),
        }
    }

    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }

    fn fmt_octal(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::Octal::fmt(&val, f),
            Self::Int16(val) => fmt::Octal::fmt(&val, f),
            Self::Int32(val) => fmt::Octal::fmt(&val, f),
            Self::Int64(val) => fmt::Octal::fmt(&val, f),
            Self::Int128(val) => fmt::Octal::fmt(&val, f),
            Self::Uint8(val) => fmt::Octal::fmt(&val, f),
            Self::Uint16(val) => fmt::Octal::fmt(&val, f),
            Self::Uint32(val) => fmt::Octal::fmt(&val, f),
            Self::Uint64(val) => fmt::Octal::fmt(&val, f),
            Self::Uint128(val) => fmt::Octal::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::LowerHex::fmt(&val, f),
            Self::Int16(val) => fmt::LowerHex::fmt(&val, f),
            Self::Int32(val) => fmt::LowerHex::fmt(&val, f),
            Self::Int64(val) => fmt::LowerHex::fmt(&val, f),
            Self::Int128(val) => fmt::LowerHex::fmt(&val, f),
            Self::Uint8(val) => fmt::LowerHex::fmt(&val, f),
            Self::Uint16(val) => fmt::LowerHex::fmt(&val, f),
            Self::Uint32(val) => fmt::LowerHex::fmt(&val, f),
            Self::Uint64(val) => fmt::LowerHex::fmt(&val, f),
            Self::Uint128(val) => fmt::LowerHex::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::UpperHex::fmt(&val, f),
            Self::Int16(val) => fmt::UpperHex::fmt(&val, f),
            Self::Int32(val) => fmt::UpperHex::fmt(&val, f),
            Self::Int64(val) => fmt::UpperHex::fmt(&val, f),
            Self::Int128(val) => fmt::UpperHex::fmt(&val, f),
            Self::Uint8(val) => fmt::UpperHex::fmt(&val, f),
            Self::Uint16(val) => fmt::UpperHex::fmt(&val, f),
            Self::Uint32(val) => fmt::UpperHex::fmt(&val, f),
            Self::Uint64(val) => fmt::UpperHex::fmt(&val, f),
            Self::Uint128(val) => fmt::UpperHex::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::Binary::fmt(&val, f),
            Self::Int16(val) => fmt::Binary::fmt(&val, f),
            Self::Int32(val) => fmt::Binary::fmt(&val, f),
            Self::Int64(val) => fmt::Binary::fmt(&val, f),
            Self::Int128(val) => fmt::Binary::fmt(&val, f),
            Self::Uint8(val) => fmt::Binary::fmt(&val, f),
            Self::Uint16(val) => fmt::Binary::fmt(&val, f),
            Self::Uint32(val) => fmt::Binary::fmt(&val, f),
            Self::Uint64(val) => fmt::Binary::fmt(&val, f),
            Self::Uint128(val) => fmt::Binary::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::LowerExp::fmt(&val, f),
            Self::Int16(val) => fmt::LowerExp::fmt(&val, f),
            Self::Int32(val) => fmt::LowerExp::fmt(&val, f),
            Self::Int64(val) => fmt::LowerExp::fmt(&val, f),
            Self::Int128(val) => fmt::LowerExp::fmt(&val, f),
            Self::Uint8(val) => fmt::LowerExp::fmt(&val, f),
            Self::Uint16(val) => fmt::LowerExp::fmt(&val, f),
            Self::Uint32(val) => fmt::LowerExp::fmt(&val, f),
            Self::Uint64(val) => fmt::LowerExp::fmt(&val, f),
            Self::Uint128(val) => fmt::LowerExp::fmt(&val, f),
            Self::Float32(val) => fmt::LowerExp::fmt(&val, f),
            Self::Float64(val) => fmt::LowerExp::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int8(val) => fmt::LowerExp::fmt(&val, f),
            Self::Int16(val) => fmt::UpperExp::fmt(&val, f),
            Self::Int32(val) => fmt::UpperExp::fmt(&val, f),
            Self::Int64(val) => fmt::UpperExp::fmt(&val, f),
            Self::Int128(val) => fmt::UpperExp::fmt(&val, f),
            Self::Uint8(val) => fmt::UpperExp::fmt(&val, f),
            Self::Uint16(val) => fmt::UpperExp::fmt(&val, f),
            Self::Uint32(val) => fmt::UpperExp::fmt(&val, f),
            Self::Uint64(val) => fmt::UpperExp::fmt(&val, f),
            Self::Uint128(val) => fmt::UpperExp::fmt(&val, f),
            Self::Float32(val) => fmt::UpperExp::fmt(&val, f),
            Self::Float64(val) => fmt::UpperExp::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }
}

pub fn fmt_str_vec(format: &str, positional: &Vec<FVariant>) -> String {
    let named = HashMap::<String, FVariant>::new();
    ParsedFormat::parse(format, positional, &named)
        .unwrap()
        .to_string()
}

pub fn fmt_str_map(format: &str, named: &HashMap<String, FVariant>) -> String {
    let positional = Vec::<FVariant>::new();
    ParsedFormat::parse(format, &positional, named)
        .unwrap()
        .to_string()
}

pub fn fmt_str_vec_map(
    format: &str,
    positional: &Vec<FVariant>,
    named: &HashMap<String, FVariant>,
) -> String {
    ParsedFormat::parse(format, positional, named)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::format::FVariant;
    use rt_format::ParsedFormat;
    use std::collections::HashMap;

    #[test]
    fn a() {
        let v = vec![
            FVariant::Int32(1),
            FVariant::Int32(2),
            FVariant::Float64(0.00000003),
            FVariant::Str("HH"),
        ];
        let named = HashMap::<String, FVariant>::new();
        let format = "{}, {:>.10} {:<.2} {:>10}";
        let a = ParsedFormat::parse(format, &v, &named).unwrap();
        println!("{}", a.to_string())
    }
}
