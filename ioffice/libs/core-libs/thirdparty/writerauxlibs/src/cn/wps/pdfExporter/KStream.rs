// Java: import java.io.IOException
// Java: import java.util.Locale

// Java: public class KStream
pub struct KStream {
    // Java field: int length
    pub(crate) length: i32,
}

// Java: public class KStream
impl KStream {
    // Java: KStream()
    pub fn new() -> Self {
      // Java: super();
      Self { length: 0 }
    }

    // Java: abstract void write(byte[] var1, int var2, int var3)
    pub fn write_bytes_range(&mut self, _var1: &[u8], _var2: i32, _var3: i32) {
    }

    // Java: void write(byte[] data)
    pub fn write_bytes(&mut self, data: &[u8]) {
      // Java: this.write(data, 0, data.length);
      self.write_bytes_range(data, 0, data.len() as i32);
    }

    // Java: void write(String text)
    pub fn write_str(&mut self, text: &str) {
      // Java: if (text != null && text.length() != 0)
      if !text.is_empty()
      {
         // Java: int len = KPdfUtils.bytesLength(text);
         let len = text.len();
         // Java: byte[] buf = new byte[len];
         let buf = text.as_bytes();
         // Java: int count = KPdfUtils.getBytes((String)text, buf, 0);
         let count = buf.len();
         // Java: this.write(buf, 0, count);
         self.write_bytes_range(buf, 0, count as i32);
      }
    }

    // Java: void writeBytes(byte[] data)
    pub fn writeBytes(&mut self, data: &[u8]) {
      // Java: this.write(data, 0, data.length);
      self.write_bytes_range(data, 0, data.len() as i32);
    }

    // Java: void write(int val)
    pub fn write_int(&mut self, val: i32) {
      // Java: String text = String.valueOf(val);
      let text = val.to_string();
      // Java: this.write(text);
      self.write_str(&text);
    }

    // Java: void write(long val)
    pub fn write_long(&mut self, val: i64) {
      // Java: String text = String.valueOf(val);
      let text = val.to_string();
      // Java: this.write(text);
      self.write_str(&text);
    }

    // Java: void write10(long val)
    pub fn write10(&mut self, val: i64) {
      // Java: String text = String.format(Locale.ENGLISH, "%010d", val);
      let text = format!("{:010}", val);
      // Java: this.write(text);
      self.write_str(&text);
    }

    // Java: void writeHex4(int val)
    pub fn writeHex4(&mut self, val: i32) {
      // Java: String text = String.format(Locale.ENGLISH, "%04x", val);
      let text = format!("{:04x}", val);
      // Java: this.write(text);
      self.write_str(&text);
    }

    // Java: void write(float val)
    pub fn write_float(&mut self, val: f32) {
      // Java: int intVal = (int)val;
      let int_val = val as i32;
      // Java: boolean bInt = (double)Math.abs(val - (float)intVal) < 1.0E-5;
      let b_int = ((val - int_val as f32) as f64).abs() < 1.0e-5;
      // Java: String text = bInt ? String.valueOf(intVal) : String.valueOf(val);
      let text = if b_int { int_val.to_string() } else { val.to_string() };
      // Java: this.write(text);
      self.write_str(&text);
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
      // Java: return this.length;
      self.length
    }

    // Java: byte[] copyData()
    pub fn copyData(&self) -> Option<Vec<u8>> {
      // Java: return null;
      None
    }

    // Java: int read(byte[] buffer)
    pub fn read_to_buf(&self, _buffer: &mut [u8]) -> i32 {
      // Java: return 0;
      0
    }

    // Java: int read(byte[] buf, int offset, int start, int count)
    pub fn read_to_buf_range(&self, _buf: &mut [u8], _offset: i32, _start: i32, _count: i32) -> i32 {
      // Java: return 0;
      0
    }

    // Java: byte read(int pos)
    pub fn read_at(&self, _pos: i32) -> i8 {
      // Java: return 0;
      0
    }

    // Java: void close()
    pub fn close(&mut self) {
    }
}
