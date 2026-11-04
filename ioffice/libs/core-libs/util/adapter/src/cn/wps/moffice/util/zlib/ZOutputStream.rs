use super::ZStream::ZStream;

// Java: public class ZOutputStream extends OutputStream
pub struct ZOutputStream {
    // Java field: ZStream z
    pub(crate) z: Option<ZStream>,
    // Java field: int bufsize
    pub(crate) bufsize: i32,
    // Java field: int flush
    pub(crate) flush: i32,
    // Java field: byte[] buf
    pub(crate) buf: Vec<u8>,
    // Java field: byte[] buf1
    pub(crate) buf1: Vec<u8>,
    // Java field: boolean compress
    pub(crate) compress: bool,
    // Java field: OutputStream out
    pub(crate) out: Option<Box<dyn std::any::Any>>,
}

// Java: public class ZOutputStream extends OutputStream
impl ZOutputStream {
    // Java: ZOutputStream(OutputStream out)
    pub fn new(out: Box<dyn std::any::Any>) -> Self {
        // Java: super();
        // Java: this.z = new ZStream();
        // Java: this.bufsize = 512;
        // Java: this.flush = 0;
        // Java: this.buf = new byte[this.bufsize];
        // Java: this.buf1 = new byte[1];
        // Java: this.out = out;
        // Java: this.z.inflateInit();
        // Java: this.compress = false;
        let mut z = ZStream::new();
        z.inflateInit();
        ZOutputStream {
            z: Some(z),
            bufsize: 512,
            flush: 0,
            buf: vec![0u8; 512],
            buf1: vec![0u8; 1],
            compress: false,
            out: Some(out),
        }
    }

    // Java: ZOutputStream(OutputStream out, int level)
    pub fn new_with_level(out: Box<dyn std::any::Any>, level: i32) -> Self {
        // Java: this(out, level, false);
        Self::new_with_level_nowrap(out, level, false)
    }

    // Java: ZOutputStream(OutputStream out, int level, boolean nowrap)
    pub fn new_with_level_nowrap(out: Box<dyn std::any::Any>, level: i32, nowrap: bool) -> Self {
        // Java: super();
        // Java: this.z = new ZStream();
        // Java: this.bufsize = 512;
        // Java: this.flush = 0;
        // Java: this.buf = new byte[this.bufsize];
        // Java: this.buf1 = new byte[1];
        // Java: this.out = out;
        // Java: this.z.deflateInit(level, nowrap);
        // Java: this.compress = true;
        let mut z = ZStream::new();
        z.deflateInit_nowrap(level, nowrap);
        ZOutputStream {
            z: Some(z),
            bufsize: 512,
            flush: 0,
            buf: vec![0u8; 512],
            buf1: vec![0u8; 1],
            compress: true,
            out: Some(out),
        }
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, _b: i32) {
        // Java: this.buf1[0] = (byte)b;
        // Java: this.write(this.buf1, 0, 1);
        todo!()
    }

    // Java: void write(byte[] b, int off, int len)
    pub fn write(&mut self, _b: &[u8], _off: i32, _len: i32) {
        // Java: if (len != 0)
        {
            // Java: this.z.next_in = b;
            // Java: this.z.next_in_index = off;
            // Java: this.z.avail_in = len;
            // Java: do
            {
                // Java: this.z.next_out = this.buf;
                // Java: this.z.next_out_index = 0;
                // Java: this.z.avail_out = this.bufsize;
                // Java: int err;
                // Java: if (this.compress)
                {
                    // Java: err = this.z.deflate(this.flush);
                }
                // Java: else
                {
                    // Java: err = this.z.inflate(this.flush);
                }
                // Java: if (err != 0)
                {
                    // Java: throw new ZStreamException((this.compress ? "de" : "in") + "flating: " + this.z.msg);
                }
                // Java: this.out.write(this.buf, 0, this.bufsize - this.z.avail_out);
            }
            // Java: while(this.z.avail_in > 0 || this.z.avail_out == 0);
        }
        todo!()
    }

    // Java: int getFlushMode()
    pub fn getFlushMode(&self) -> i32 {
        // Java: return this.flush;
        self.flush
    }

    // Java: void setFlushMode(int flush)
    pub fn setFlushMode(&mut self, flush: i32) {
        // Java: this.flush = flush;
        self.flush = flush;
    }

    // Java: void finish()
    pub fn finish(&mut self) {
        // Java: do
        {
            // Java: this.z.next_out = this.buf;
            // Java: this.z.next_out_index = 0;
            // Java: this.z.avail_out = this.bufsize;
            // Java: int err;
            // Java: if (this.compress)
            {
                // Java: err = this.z.deflate(4);
            }
            // Java: else
            {
                // Java: err = this.z.inflate(4);
            }
            // Java: if (err != 1 && err != 0)
            {
                // Java: throw new ZStreamException((this.compress ? "de" : "in") + "flating: " + this.z.msg);
            }
            // Java: if (this.bufsize - this.z.avail_out > 0)
            {
                // Java: this.out.write(this.buf, 0, this.bufsize - this.z.avail_out);
            }
        }
        // Java: while(this.z.avail_in > 0 || this.z.avail_out == 0);
        // Java: this.flush();
        todo!()
    }

    // Java: void end()
    pub fn end(&mut self) {
        // Java: if (this.z != null)
        if let Some(ref mut z) = self.z {
            // Java: if (this.compress)
            if self.compress {
                // Java: this.z.deflateEnd();
                z.deflateEnd();
            // Java: else
            } else {
                // Java: this.z.inflateEnd();
                z.inflateEnd();
            }
            // Java: this.z.free();
            z.free();
        }
        // Java: this.z = null;
        self.z = None;
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: try
        // Java: this.finish();
        // Java: catch (IOException var5)
        // Java: finally
        // Java: this.end();
        // Java: this.out.close();
        // Java: this.out = null;
        self.end();
        self.out = None;
    }

    // Java: long getTotalIn()
    pub fn getTotalIn(&self) -> i64 {
        // Java: return this.z.total_in;
        self.z.as_ref().map_or(0, |z| z.total_in)
    }

    // Java: long getTotalOut()
    pub fn getTotalOut(&self) -> i64 {
        // Java: return this.z.total_out;
        self.z.as_ref().map_or(0, |z| z.total_out)
    }

    // Java: void flush()
    pub fn flush(&mut self) {
        // Java: this.out.flush();
        todo!()
    }

}
