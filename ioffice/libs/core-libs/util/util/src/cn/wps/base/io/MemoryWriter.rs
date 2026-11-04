#![allow(non_snake_case, dead_code)]

// Java: import java.io.IOException
// Java: import java.io.OutputStream

// Java: public class MemoryWriter extends OutputStream
pub struct MemoryWriter {
    // Java field: _mBuffer: &[u8]
    pub(crate) mBuffer: Vec<u8>,
    // Java field: int mPosition
    pub(crate) mPosition: usize,
}

// Java: public class MemoryWriter extends OutputStream
impl MemoryWriter {
    // Java: MemoryWriter(_buffer: &[u8])
    pub fn new(buffer: Vec<u8>) -> Self {
        // Java: this(buffer, 0);
        Self::new_with_position(buffer, 0)
    }

    // Java: MemoryWriter(_buffer: &[u8], int position)
    pub fn new_with_position(buffer: Vec<u8>, position: usize) -> Self {
        // Java: super();
        // Java: this.mBuffer = buffer;
        // Java: this.mPosition = position;
        MemoryWriter { mBuffer: buffer, mPosition: position }
    }

    // Java: void seek(int position)
    pub fn seek(&mut self, position: usize) {
        // Java: this.mPosition = position;
        self.mPosition = position;
    }

    // Java: int tell()
    pub fn tell(&self) -> i32 {
        // Java: return this.mPosition;
        self.mPosition as i32
    }

    // Java: void write(_b: &[u8], int off, int len)
    pub fn write(&mut self, b: &[u8], off: usize, len: usize) {
        // Java: System.arraycopy(b, off, this.mBuffer, this.mPosition, len);
        self.mBuffer[self.mPosition..self.mPosition + len].copy_from_slice(&b[off..off + len]);
        // Java: this.mPosition += len;
        self.mPosition += len;
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, b: i32) {
        // Java: this.mBuffer[this.mPosition++] = (byte)b;
        self.mBuffer[self.mPosition] = b as u8;
        self.mPosition += 1;
    }
}
