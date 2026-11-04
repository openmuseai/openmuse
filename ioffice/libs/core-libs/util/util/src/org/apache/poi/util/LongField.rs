#![allow(non_snake_case, dead_code, unused_variables)]

use std::io::{self, Read};
use super::LittleEndian::LittleEndian;
use super::FixedField::FixedField;

// Java: public class LongField implements FixedField
pub struct LongField {
    // Java field: long _value
    pub _value: i64,
    // Java field: int _offset
    pub _offset: i32,
}

// Java: public class LongField implements FixedField
impl LongField {
    // Java: LongField(int offset)
    pub fn new(offset: i32) -> Self {
        // Java: super();
        // Java: if (offset < 0)
        if offset < 0 {
            // Java: throw new ArrayIndexOutOfBoundsException("Illegal offset: " + offset);
            panic!("Illegal offset: {}", offset);
        // Java: else
        } else {
            // Java: this._offset = offset;
            Self { _value: 0, _offset: offset }
        }
    }

    // Java: LongField(int offset, long value)
    pub fn new_with_value(offset: i32, value: i64) -> Self {
        // Java: this(offset);
        let mut field = Self::new(offset);
        // Java: this.set(value);
        field.set(value);
        field
    }

    // Java: LongField(int offset, _data: &[u8])
    pub fn new_from_bytes(offset: i32, data: &[i8]) -> Self {
        // Java: this(offset);
        let mut field = Self::new(offset);
        // Java: this.readFromBytes(data);
        field.readFromBytes_i8(data);
        field
    }

    // Java: LongField(int offset, long value, _data: &[u8])
    pub fn new_with_value_and_data(offset: i32, value: i64, data: &mut [i8]) -> Self {
        // Java: this(offset);
        let mut field = Self::new(offset);
        // Java: this.set(value, data);
        field.set_with_data(value, data);
        field
    }

    // Java: long get()
    pub fn get(&self) -> i64 {
        // Java: return this._value;
        self._value
    }

    // Java: void set(long value)
    pub fn set(&mut self, value: i64) {
        // Java: this._value = value;
        self._value = value;
    }

    // Java: void set(long value, _data: &[u8])
    pub fn set_with_data(&mut self, value: i64, data: &mut [i8]) {
        // Java: this._value = value;
        self._value = value;
        // Java: this.writeToBytes(data);
        self.writeToBytes_i8(data);
    }

    // Java: void readFromBytes(_data: &[u8])
    pub fn readFromBytes_i8(&mut self, data: &[i8]) {
        // Java: this._value = LittleEndian.getLong(data, this._offset);
        self._value = LittleEndian::getLong(data, self._offset);
    }

    // Java: void readFromStream(InputStream stream)
    pub fn readFromStream_impl(&mut self, stream: &mut dyn Read) -> io::Result<()> {
        // Java: this._value = LittleEndian.readLong(stream);
        self._value = LittleEndian::readLong(stream)?;
        Ok(())
    }

    // Java: void writeToBytes(_data: &[u8])
    pub fn writeToBytes_i8(&self, data: &mut [i8]) {
        // Java: LittleEndian.putLong(data, this._offset, this._value);
        LittleEndian::putLong(data, self._offset, self._value);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return String.valueOf(this._value);
        self._value.to_string()
    }
}

impl FixedField for LongField {
    fn readFromBytes(&mut self, var1: &[u8]) {
        let data: Vec<i8> = var1.iter().map(|&b| b as i8).collect();
        self.readFromBytes_i8(&data);
    }
    fn readFromStream(&mut self, var1: &mut dyn Read) {
        let _ = self.readFromStream_impl(var1);
    }
    fn writeToBytes(&self, var1: &mut [u8]) {
        let mut data: Vec<i8> = var1.iter().map(|&b| b as i8).collect();
        self.writeToBytes_i8(&mut data);
        for (i, &b) in data.iter().enumerate() {
            var1[i] = b as u8;
        }
    }
    fn to_string(&self) -> String {
        self.toString()
    }
}
