use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::io::Cursor;

pub struct ParseBinary {}

impl ParseBinary {
    pub fn i8_little_endian(data: &[u8]) -> i8 {
        return data[0] as i8;
    }
    pub fn u8_little_endian(data: &[u8]) -> u8 {
        return data[0];
    }

    pub fn i16_little_endian(data: &[u8]) -> Result<i16, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_i16::<LittleEndian>()
    }
    pub fn u16_little_endian(data: &[u8]) -> Result<u16, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_u16::<LittleEndian>()
    }

    pub fn i24_little_endian(data: &[u8]) -> i32 {
        let mut uint32 = ParseBinary::u24_little_endian(data);
        if uint32 & 0x00800000 != 0 {
            uint32 |= 0xFF000000
        }

        uint32 as i32
    }
    pub fn u24_little_endian(data: &[u8]) -> u32 {
        let data_0 = data[0] as u32;
        let data_1 = data[1] as u32;
        let data_2 = data[2] as u32;

        data_0 | (data_1 << 8) | (data_2 << 16)
    }

    pub fn i32_little_endian(data: &[u8]) -> Result<i32, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_i32::<LittleEndian>()
    }
    pub fn u32_little_endian(data: &[u8]) -> Result<u32, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_u32::<LittleEndian>()
    }

    pub fn i64_little_endian(data: &[u8]) -> Result<i64, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_i64::<LittleEndian>()
    }
    pub fn u64_little_endian(data: &[u8]) -> Result<u64, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_u64::<LittleEndian>()
    }

    pub fn f32_little_endian(data: &[u8]) -> Result<f32, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_f32::<LittleEndian>()
    }
    pub fn f64_little_endian(data: &[u8]) -> Result<f64, io::Error> {
        let mut rdr = Cursor::new(data);
        rdr.read_f64::<LittleEndian>()
    }
}
