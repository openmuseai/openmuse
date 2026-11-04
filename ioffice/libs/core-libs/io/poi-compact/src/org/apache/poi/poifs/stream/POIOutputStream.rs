// Java: import cn.wps.ole.IOLEStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream

use std::io;

pub trait IOLEStreamWriter {
    fn seek(&mut self, pos: i32);
    fn tell(&self) -> i32;
    fn write_bytes(&mut self, b: &[u8], off: i32, len: i32);
    fn write_all_bytes(&mut self, b: &[u8]);
    fn write_int(&mut self, b: i32);
    fn close(&mut self);
}

// Java: public class POIOutputStream extends OutputStream
pub struct POIOutputStream {
    // Java field: IOLEStream mStream
    mStream: Option<Box<dyn IOLEStreamWriter>>,
}

// Java: public class POIOutputStream extends OutputStream
impl POIOutputStream {
    // Java: POIOutputStream(IOLEStream pStream)
    pub fn new(pStream: Box<dyn IOLEStreamWriter>) -> Self {
        // Java: super();
        // Java: this.mStream = pStream;
        POIOutputStream {
            mStream: Some(pStream),
        }
    }

    // Java: void write(byte[] b, int off, int len)
    pub fn write_buf(&mut self, b: &[u8], off: i32, len: i32) {
        // Java: this.mStream.write(b, off, len);
        self.mStream.as_mut().unwrap().write_bytes(b, off, len);
    }

    // Java: void write(byte[] b)
    pub fn write_all(&mut self, b: &[u8]) {
        // Java: this.mStream.write(b);
        self.mStream.as_mut().unwrap().write_all_bytes(b);
    }

    // Java: void write(int b)
    pub fn write_int(&mut self, b: i32) {
        // Java: this.mStream.write(b);
        self.mStream.as_mut().unwrap().write_int(b);
    }

    // Java: void flush()
    pub fn flush(&mut self) {
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.mStream.close();
        self.mStream.as_mut().unwrap().close();
        // Java: this.mStream = null;
        self.mStream = None;
    }

    // Java: int getOffset()
    pub fn getOffset(&self) -> i32 {
        // Java: try
        // Java: return this.mStream.tell();
        match &self.mStream {
            Some(s) => s.tell(),
            // Java: catch (IOException var2)
            // Java: return -1;
            None => -1,
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.mStream.seek(0);
        self.mStream.as_mut().unwrap().seek(0);
    }

    // Java: void seek(int pos)
    pub fn seek(&mut self, pos: i32) {
        // Java: this.mStream.seek(pos);
        self.mStream.as_mut().unwrap().seek(pos);
    }

    // Java: IOLEStream getOLEStream()
    pub fn getOLEStream(&mut self) -> &mut dyn IOLEStreamWriter {
        // Java: return this.mStream;
        self.mStream.as_mut().unwrap().as_mut()
    }
}

impl io::Write for POIOutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_all(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
