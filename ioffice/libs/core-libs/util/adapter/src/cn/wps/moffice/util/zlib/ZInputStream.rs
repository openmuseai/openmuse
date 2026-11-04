// Java: import java.io.FilterInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream

use super::ZStream::ZStream;

// Java: public class ZInputStream extends FilterInputStream
pub struct ZInputStream {
    // Java field: ZStream z
    pub(crate) z: Option<ZStream>,
    // Java field: int bufsize
    pub(crate) bufsize: i32,
    // Java field: int flush
    pub(crate) flush: i32,
    // Java field: byte[] buf
    pub(crate) buf: Option<Vec<u8>>,
    // Java field: byte[] buf1
    pub(crate) buf1: Option<Vec<u8>>,
    // Java field: boolean compress
    pub(crate) compress: bool,
    // Java field: InputStream in
    pub(crate) input: Option<Box<dyn std::any::Any>>,
    // Java field: boolean nomoreinput
    nomoreinput: bool,
}

// Java: public class ZInputStream extends FilterInputStream
impl ZInputStream {
    // Java: ZInputStream(InputStream in)
    pub fn new(_input: Box<dyn std::any::Any>) -> Self {
      // Java: this(in, false);
      Self::new_with_nowrap(_input, false)
    }

    // Java: ZInputStream(InputStream in, boolean nowrap)
    pub fn new_with_nowrap(_input: Box<dyn std::any::Any>, _nowrap: bool) -> Self {
      // Java: super(in);

      // Java: this.z = new ZStream();

      // Java: this.bufsize = 512;

      // Java: this.flush = 0;

      // Java: this.buf = new byte[this.bufsize];

      // Java: this.buf1 = new byte[1];

      // Java: this.in = null;

      // Java: this.nomoreinput = false;

      // Java: this.in = in;

      // Java: this.z.inflateInit(nowrap);

      // Java: this.compress = false;

      // Java: this.z.next_in = this.buf;

      // Java: this.z.next_in_index = 0;

      // Java: this.z.avail_in = 0;

      todo!()
    }

    // Java: ZInputStream(InputStream in, int wbit, boolean nowrap)
    pub fn new_with_wbit(_input: Box<dyn std::any::Any>, _wbit: i32, _nowrap: bool) -> Self {
      // Java: super(in);

      // Java: this.z = new ZStream();

      // Java: this.bufsize = 512;

      // Java: this.flush = 0;

      // Java: this.buf = new byte[this.bufsize];

      // Java: this.buf1 = new byte[1];

      // Java: this.in = null;

      // Java: this.nomoreinput = false;

      // Java: this.in = in;

      // Java: this.z.inflateInit(wbit, nowrap);

      // Java: this.compress = false;

      // Java: this.z.next_in = this.buf;

      // Java: this.z.next_in_index = 0;

      // Java: this.z.avail_in = 0;

      todo!()
    }

    // Java: ZInputStream(InputStream in, int level)
    pub fn new_with_level(_input: Box<dyn std::any::Any>, _level: i32) -> Self {
      // Java: super(in);

      // Java: this.z = new ZStream();

      // Java: this.bufsize = 512;

      // Java: this.flush = 0;

      // Java: this.buf = new byte[this.bufsize];

      // Java: this.buf1 = new byte[1];

      // Java: this.in = null;

      // Java: this.nomoreinput = false;

      // Java: this.in = in;

      // Java: this.z.deflateInit(level);

      // Java: this.compress = true;

      // Java: this.z.next_in = this.buf;

      // Java: this.z.next_in_index = 0;

      // Java: this.z.avail_in = 0;

      todo!()
    }

    // Java: int read()
    pub fn read(&mut self) -> i32 {
      // Java: return this.read(this.buf1, 0, 1) == -1 ? -1 : this.buf1[0] & 255;

      todo!()
    }

    // Java: int read(byte[] b, int off, int len)
    pub fn read_buf(&mut self, _b: &mut [u8], _off: i32, _len: i32) -> i32 {
      // Java: if (len == 0)

      {
         // Java: return 0;

      }
      // Java: else

      {
         // Java: this.z.next_out = b;

         // Java: this.z.next_out_index = off;

         // Java: this.z.avail_out = len;

         // Java: int err;

         // Java: do

         {
            // Java: if (this.z.avail_in == 0 && !this.nomoreinput)

            {
               // Java: this.z.next_in_index = 0;

               // Java: this.z.avail_in = this.in.read(this.buf, 0, this.bufsize);

               // Java: if (this.z.avail_in == -1)

               {
                  // Java: this.z.avail_in = 0;

                  // Java: this.nomoreinput = true;

               }
            }
            // Java: if (this.compress)

            {
               // Java: err = this.z.deflate(this.flush);

            }
            // Java: else

            {
               // Java: err = this.z.inflate(this.flush);

            }
            // Java: if (this.nomoreinput && err == -5)

            {
               // Java: return -1;

            }
            // Java: if (err != 0 && err != 1)

            {
               // Java: throw new ZStreamException((this.compress ? "de" : "in") + "flating: " + this.z.msg);

            }
            // Java: if ((this.nomoreinput || err == 1) && this.z.avail_out == len)

            {
               // Java: return -1;

            }
         }
         // Java: return len - this.z.avail_out;

      }
      todo!()
    }

    // Java: long skip(long n)
    pub fn skip(&mut self, _n: i64) -> i64 {
      // Java: int len = 512;

      // Java: if (n < (long)len)

      {
         // Java: len = (int)n;

      }
      // Java: byte[] tmp = new byte[len];

      // Java: return (long)this.read(tmp);

      todo!()
    }

    // Java: int getFlushMode()
    pub fn getFlushMode(&self) -> i32 {
      // Java: return this.flush;

      todo!()
    }

    // Java: void setFlushMode(int flush)
    pub fn setFlushMode(&mut self, _flush: i32) {
      // Java: this.flush = flush;

    }

    // Java: long getTotalIn()
    pub fn getTotalIn(&self) -> i64 {
      // Java: return this.z.total_in;

      todo!()
    }

    // Java: long getTotalOut()
    pub fn getTotalOut(&self) -> i64 {
      // Java: return this.z.total_out;

      todo!()
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.in.close();

    }

}
