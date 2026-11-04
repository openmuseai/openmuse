#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: public class TIFFField
pub struct TIFFField {
    pub(crate) tag: i32,
    pub(crate) field_type: i32,
    pub(crate) count: i32,
    pub(crate) data: TIFFData,
}

pub enum TIFFData {
    Bytes(Vec<u8>),
    Chars(Vec<u16>),
    Shorts(Vec<i16>),
    Ints(Vec<i32>),
    Longs(Vec<i64>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    Strings(Vec<String>),
    Rationals(Vec<(i64, i64)>),
    SRationals(Vec<(i32, i32)>),
}

impl TIFFField {
    pub fn new(tag: i32, field_type: i32, count: i32, data: TIFFData) -> Self {
        Self { tag, field_type, count, data }
    }

    pub fn getTag(&self) -> i32 { self.tag }
    pub fn getType(&self) -> i32 { self.field_type }
    pub fn getCount(&self) -> i32 { self.count }

    pub fn getAsBytes(&self) -> &[u8] {
        match &self.data {
            TIFFData::Bytes(v) => v,
            _ => &[],
        }
    }

    pub fn getAsLong(&self, index: i32) -> i64 {
        match &self.data {
            TIFFData::Longs(v) => v[index as usize],
            TIFFData::Ints(v) => v[index as usize] as i64,
            TIFFData::Shorts(v) => v[index as usize] as i64,
            TIFFData::Bytes(v) => v[index as usize] as i64,
            TIFFData::Rationals(v) => v[index as usize].0,
            TIFFData::SRationals(v) => v[index as usize].0 as i64,
            _ => 0,
        }
    }

    pub fn getAsFloat(&self, index: i32) -> f32 {
        match &self.data {
            TIFFData::Floats(v) => v[index as usize],
            TIFFData::Doubles(v) => v[index as usize] as f32,
            TIFFData::Rationals(v) => {
                let (n, d) = v[index as usize];
                if d != 0 { n as f32 / d as f32 } else { 0.0 }
            }
            TIFFData::SRationals(v) => {
                let (n, d) = v[index as usize];
                if d != 0 { n as f32 / d as f32 } else { 0.0 }
            }
            _ => self.getAsLong(index) as f32,
        }
    }

    pub fn getAsDouble(&self, index: i32) -> f64 {
        match &self.data {
            TIFFData::Doubles(v) => v[index as usize],
            TIFFData::Floats(v) => v[index as usize] as f64,
            TIFFData::Rationals(v) => {
                let (n, d) = v[index as usize];
                if d != 0 { n as f64 / d as f64 } else { 0.0 }
            }
            TIFFData::SRationals(v) => {
                let (n, d) = v[index as usize];
                if d != 0 { n as f64 / d as f64 } else { 0.0 }
            }
            _ => self.getAsLong(index) as f64,
        }
    }
}
