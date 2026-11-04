#![allow(non_snake_case, dead_code, unused_variables)]

use std::io::{self, Read};
use super::LittleEndian::LittleEndian;
use super::FixedField::FixedField;

// Java: public class ShortField implements FixedField
pub struct ShortField {
    // Java field: short _value
    pub _value: i16,
    // Java field: int _offset
    pub _offset: i32,
}

// Java: public class ShortField implements FixedField
impl ShortField {
    // Java: ShortField(int offset)
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

    // Java: ShortField(int offset, short value)
    pub fn new_with_value(offset: i32, value: i16) -> Self {
        // Java: this(offset);
        let mut field = Self::new(offset);
        // Java: this.set(value);
        field.set(value);
        field
    }

    // Java: ShortField(int offset, _data: &[u8])
    pub fn new_from_bytes(offset: i32, data: &[i8]) -> Self {
        // Java: this(offset);
        let mut field = Self::new(offset);
        // Java: this.readFromBytes(data);
        field.readFromBytes_i8(data);
        field
    }

    // Java: ShortField(int offset, short value, _data: &[u8])
    pub fn new_with_value_and_data(offset: i32, value: i16, data: &mut [i8]) -> Self {
        // Java: this(offset);
        let mut field = Self::new(offset);
        // Java: this.set(value, data);
        field.set_with_data(value, data);
        field
    }

    // Java: short get()
    pub fn get(&self) -> i16 {
        // Java: return this._value;
        self._value
    }

    // Java: void set(short value)
    pub fn set(&mut self, value: i16) {
        // Java: this._value = value;
        self._value = value;
    }

    // Java: void set(short value, _data: &[u8])
    pub fn set_with_data(&mut self, value: i16, data: &mut [i8]) {
        // Java: this._value = value;
        self._value = value;
        // Java: this.writeToBytes(data);
        self.writeToBytes_i8(data);
    }

    // Java: void readFromBytes(_data: &[u8])
    pub fn readFromBytes_i8(&mut self, data: &[i8]) {
        // Java: this._value = LittleEndian.getShort(data, this._offset);
        self._value = LittleEndian::getShort(data, self._offset);
    }

    // Java: void readFromStream(InputStream stream)
    pub fn readFromStream_impl(&mut self, stream: &mut dyn Read) -> io::Result<()> {
        // Java: this._value = LittleEndian.readShort(stream);
        self._value = LittleEndian::readShort(stream)?;
        Ok(())
    }

    // Java: void writeToBytes(_data: &[u8])
    pub fn writeToBytes_i8(&self, data: &mut [i8]) {
        // Java: LittleEndian.putShort(data, this._offset, this._value);
        LittleEndian::putShort(data, self._offset, self._value);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return String.valueOf(this._value);
        self._value.to_string()
    }
}

impl FixedField for ShortField {
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
