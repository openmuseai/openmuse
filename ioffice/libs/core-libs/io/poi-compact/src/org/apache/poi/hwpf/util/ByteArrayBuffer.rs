use std::sync::{Arc, Mutex};

// Java: public class ByteArrayBuffer
pub struct ByteArrayBuffer {
    // Java field: byte[] buffer
    pub(crate) buffer: Vec<u8>,
    // Java field: int len
    pub(crate) len: i32,
}

// Java: public class ByteArrayBuffer
impl ByteArrayBuffer {
    // Java: ByteArrayBuffer(int capacity)
    pub fn new(capacity: i32) -> Self {
      // Java: super();
        if capacity < 0
      {
         // Java: throw new IllegalArgumentException("Buffer capacity may not be negative");
         panic!("Buffer capacity may not be negative");
      }
      {
         // Java: this.buffer = new byte[capacity];
         ByteArrayBuffer { buffer: vec![0u8; capacity as usize], len: 0 }
      }
    }

    // Java: void append(byte[] b, int off, int len)
    pub fn append_bytes(&mut self, b: &[u8], off: i32, len: i32) {
        if b.len() > 0
      {
            if off >= 0 && off <= b.len() as i32 && len >= 0 && off + len <= b.len() as i32
         {
               // Java: int newlen = this.len + len;
               let newlen = self.len + len;
                    if newlen > self.buffer.len() as i32
               {
                  // Java: this.expand(newlen);
                  self.expand(newlen);
               }
               // Java: System.arraycopy(b, off, this.buffer, this.len, len);
               self.buffer[self.len as usize..(self.len + len) as usize].copy_from_slice(&b[off as usize..(off + len) as usize]);
               // Java: this.len = newlen;
               self.len = newlen;
         }
            else
         {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
         }
      }
    }

    // Java: void append(char[] b, int off, int len)
    pub fn append_chars(&mut self, b: &[char], off: i32, len: i32) {
        if b.len() > 0
      {
            if off >= 0 && off <= b.len() as i32 && len >= 0 && off + len <= b.len() as i32
         {
               // Java: int oldlen = this.len;
               let _oldlen = self.len;
               // Java: int newlen = oldlen + len;
               let newlen = self.len + len;
                    if newlen > self.buffer.len() as i32
               {
                  // Java: this.expand(newlen);
                  self.expand(newlen);
               }
               // Java: int i1 = off;
               let mut i1 = off;
                    for i2 in self.len..newlen
               {
                  // Java: this.buffer[i2] = (byte)b[i1];
                  self.buffer[i2 as usize] = b[i1 as usize] as u8;
                  // Java: ++i1;
                  i1 += 1;
               }
               // Java: this.len = newlen;
               self.len = newlen;
         }
            else
         {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
         }
      }
    }

    // Java: void append(int b)
    pub fn append_int(&mut self, b: i32) {
      // Java: int newlen = this.len + 1;
      let newlen = self.len + 1;
        if newlen > self.buffer.len() as i32
      {
         // Java: this.expand(newlen);
         self.expand(newlen);
      }
      // Java: this.buffer[this.len] = (byte)b;
      self.buffer[self.len as usize] = b as u8;
      // Java: this.len = newlen;
      self.len = newlen;
    }

    // Java: byte[] buffer()
    pub fn buffer(&self) -> &[u8] {
      // Java: return this.buffer;
      &self.buffer
    }

    // Java: int byteAt(int i)
    pub fn byteAt(&self, i: i32) -> i32 {
      // Java: return this.buffer[i];
      self.buffer[i as usize] as i32
    }

    // Java: int capacity()
    pub fn capacity(&self) -> i32 {
      // Java: return this.buffer.length;
      self.buffer.len() as i32
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.len = 0;
      self.len = 0;
    }

    // Java: void expand(int newlen)
    pub fn expand(&mut self, newlen: i32) {
      // Java: byte[] newbuffer = new byte[Math.max(this.buffer.length << 1, newlen)];
      let new_cap = std::cmp::max((self.buffer.len() << 1) as i32, newlen) as usize;
      let mut newbuffer = vec![0u8; new_cap];
      // Java: System.arraycopy(this.buffer, 0, newbuffer, 0, this.len);
      newbuffer[..self.len as usize].copy_from_slice(&self.buffer[..self.len as usize]);
      // Java: this.buffer = newbuffer;
      self.buffer = newbuffer;
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.len == 0;
      self.len == 0
    }

    // Java: boolean isFull()
    pub fn isFull(&self) -> bool {
      // Java: return this.len == this.buffer.length;
      self.len == self.buffer.len() as i32
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: return this.len;
      self.len
    }

    // Java: void setLength(int len)
    pub fn setLength(&mut self, len: i32) {
        if len >= 0 && len <= self.buffer.len() as i32
      {
         // Java: this.len = len;
         self.len = len;
      }
        else
      {
         // Java: throw new IndexOutOfBoundsException();
         panic!("IndexOutOfBoundsException");
      }
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: byte[] b = new byte[this.len];
      let mut b = vec![0u8; self.len as usize];
        if self.len > 0
      {
         // Java: System.arraycopy(this.buffer, 0, b, 0, this.len);
         b.copy_from_slice(&self.buffer[..self.len as usize]);
      }
      // Java: return b;
      b
    }

}
