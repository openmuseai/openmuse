// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.nio.BufferUnderflowException
use std::io::Read;
use super::FixedField::FixedField;

// Java field: byte _default_value
pub(crate) const DEFAULT_VALUE: u8 = 0;

// Java: public class ByteField implements FixedField
pub struct ByteField {
    // Java field: byte _value
    pub(crate) value: u8,
    // Java field: int _offset
    offset: i32,
}

// Java: public class ByteField implements FixedField
impl ByteField {
    // Java: ByteField(int offset)
    pub fn new(offset: i32) -> Self {
        // Java: this(offset, (byte)0);
        Self::new_with_value(offset, 0)
    }

    // Java: ByteField(int offset, byte value)
    pub fn new_with_value(offset: i32, value: u8) -> Self {
        // Java: super();
        // Java: if (offset < 0)
        if offset < 0 {
            // Java: throw new ArrayIndexOutOfBoundsException("offset cannot be negative");
            panic!("offset cannot be negative");
        }
        // Java: else
        else {
            // Java: this._offset = offset;
            let mut bf = ByteField { value: 0, offset };
            // Java: this.set(value);
            bf.set(value);
            bf
        }
    }

    // Java: ByteField(int offset, byte[] data)
    pub fn new_from_data(offset: i32, data: &[u8]) -> Self {
        // Java: this(offset);
        let mut bf = Self::new(offset);
        // Java: this.readFromBytes(data);
        bf.readFromBytes(data);
        bf
    }

    // Java: ByteField(int offset, byte value, byte[] data)
    pub fn new_with_value_and_data(offset: i32, value: u8, data: &mut [u8]) -> Self {
        // Java: this(offset, value);
        let mut bf = Self::new_with_value(offset, value);
        // Java: this.writeToBytes(data);
        bf.writeToBytes(data);
        bf
    }

    // Java: byte get()
    pub fn get(&self) -> u8 {
        // Java: return this._value;
        self.value
    }

    // Java: void set(byte value)
    pub fn set(&mut self, value: u8) {
        // Java: this._value = value;
        self.value = value;
    }

    // Java: void set(byte value, byte[] data)
    pub fn set_with_data(&mut self, value: u8, data: &mut [u8]) {
        // Java: this.set(value);
        self.set(value);
        // Java: this.writeToBytes(data);
        self.writeToBytes(data);
    }

    // Java: void readFromBytes(byte[] data)
    pub fn readFromBytes(&mut self, data: &[u8]) {
        // Java: this._value = data[this._offset];
        self.value = data[self.offset as usize];
    }

    // Java: void readFromStream(InputStream stream)
    pub fn readFromStream_impl(&mut self, stream: &mut dyn Read) {
        // Java: int ib = stream.read();
        let mut buf = [0u8; 1];
        let ib = match stream.read(&mut buf) {
            Ok(0) => -1,
            Ok(_) => buf[0] as i32,
            Err(_) => -1,
        };
        // Java: if (ib < 0)
        if ib < 0 {
            // Java: throw new BufferUnderflowException();
            panic!("BufferUnderflowException");
        }
        // Java: else
        else {
            // Java: this._value = (byte)ib;
            self.value = ib as u8;
        }
    }

    // Java: void writeToBytes(byte[] data)
    pub fn writeToBytes(&self, data: &mut [u8]) {
        // Java: data[this._offset] = this._value;
        data[self.offset as usize] = self.value;
    }

    // Java: String toString()
    pub fn to_string_value(&self) -> String {
        // Java: return String.valueOf(this._value);
        self.value.to_string()
    }
}

impl FixedField for ByteField {
    fn readFromBytes(&mut self, var1: &[u8]) {
        self.readFromBytes(var1);
    }
    fn readFromStream(&mut self, var1: &mut dyn Read) {
        self.readFromStream_impl(var1);
    }
    fn writeToBytes(&self, var1: &mut [u8]) {
        self.writeToBytes(var1);
    }
    fn to_string(&self) -> String {
        self.to_string_value()
    }
}
