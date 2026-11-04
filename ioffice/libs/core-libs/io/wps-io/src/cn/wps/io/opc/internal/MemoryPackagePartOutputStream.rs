#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream

use crate::io::opc::internal::MemoryPackagePart::MemoryPackagePart;

// Java: public class MemoryPackagePartOutputStream extends OutputStream
pub struct MemoryPackagePartOutputStream {
    // Java field: MemoryPackagePart _part
    pub(crate) _part: *mut MemoryPackagePart,
    // Java field: ByteArrayOutputStream _buff
    pub(crate) _buff: Vec<u8>,
}

// Java: public class MemoryPackagePartOutputStream extends OutputStream
impl MemoryPackagePartOutputStream {
    // Java: MemoryPackagePartOutputStream(MemoryPackagePart part)
    pub fn new(part: *mut MemoryPackagePart) -> Self {
        // Java: super();
        // Java: this._part = part;
        // Java: this._buff = new ByteArrayOutputStream();
        MemoryPackagePartOutputStream {
            _part: part,
            _buff: Vec::new(),
        }
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, b: u8) {
        // Java: this._buff.write(b);
        self._buff.push(b);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.flush();
        self.flush();
    }

    // Java: void flush()
    pub fn flush(&mut self) {
        // Java: this._buff.flush();
        // Java: if (this._part.data != null)
        // Java: byte[] newArray = new byte[this._part.data.length + this._buff.size()];
        // Java: System.arraycopy(this._part.data, 0, newArray, 0, this._part.data.length);
        // Java: byte[] buffArr = this._buff.toByteArray();
        // Java: System.arraycopy(buffArr, 0, newArray, this._part.data.length, buffArr.length);
        // Java: this._part.data = newArray;
        // Java: else
        // Java: this._part.data = this._buff.toByteArray();
        // Java: this._buff.reset();
        unsafe {
            let part = &mut *self._part;
            if let Some(ref mut existing_data) = part.data {
                existing_data.extend_from_slice(&self._buff);
            } else {
                part.data = Some(self._buff.clone());
            }
        }
        self._buff.clear();
    }

    // Java: void write(byte[] b, int off, int len)
    pub fn write_buf(&mut self, b: &[u8], off: usize, len: usize) {
        // Java: this._buff.write(b, off, len);
        self._buff.extend_from_slice(&b[off..off + len]);
    }

    // Java: void write(byte[] b)
    pub fn write_all(&mut self, b: &[u8]) {
        // Java: this._buff.write(b);
        self._buff.extend_from_slice(b);
    }
}
