// Java: import cn.wps.base.assertion.KSAssert
// use util::cn::wps::base::assertion::KSAssert; // not available in this crate

// Java: public class KMemoryStream extends KStream
pub struct KMemoryStream {
    // Java field: byte[] mData
    pub(crate) mData: Vec<u8>,
    pub(crate) length: i32,
}

// Java: public class KMemoryStream extends KStream
#[allow(non_snake_case)]
impl KMemoryStream {
    // Java: KMemoryStream(byte[] data)
    pub fn new(data: Vec<u8>) -> Self {
      // Java: super();
      // Java: this.mData = data;
      // Java: this.length = this.mData.length;
      let length = data.len() as i32;
      Self { mData: data, length }
    }

    // Java: void write(byte[] data, int start, int count)
    pub fn write(&mut self, _data: &[u8], _start: i32, _count: i32) {
    }

    // Java: int read(byte[] buf, int offset, int start, int count)
    pub fn read_to_buf(&mut self, buf: &mut [u8], offset: i32, start: i32, count: i32) -> i32 {
      // Java: int n = count;
      let mut n = count;
      // Java: if (buf.length < offset + count)
        if (buf.len() as i32) < offset + count
      {
         // Java: n = count - (offset + count - buf.length);
         n = count - (offset + count - buf.len() as i32);
      }
      // Java: if (start + n > this.mData.length)
        if start + n > self.mData.len() as i32
      {
         // Java: n -= start + n - this.mData.length;
         n -= start + n - self.mData.len() as i32;
      }
      // Java: if (n < 0)
        if n < 0
      {
         // Java: return 0;
         return 0;
      }
      // Java: else
      {
         // Java: System.arraycopy(this.mData, start, buf, offset, n);
         let src_start = start as usize;
         let dst_start = offset as usize;
         let len = n as usize;
         buf[dst_start..dst_start + len].copy_from_slice(&self.mData[src_start..src_start + len]);
         // Java: return n;
         return n;
      }
    }

    // Java: int read(byte[] buffer)
    pub fn read_all(&mut self, _buffer: &mut [u8]) -> i32 {
      // Java: KSAssert.assertTrue(false);
      panic!("KMemoryStream::read_all should not be called");
      // Java: return 0;
    }

    // Java: byte[] copyData()
    pub fn copyData(&self) -> Vec<u8> {
      // Java: return this.mData;
      self.mData.clone()
    }

}
