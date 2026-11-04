#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.OutputStream
use std::io::{self, Write};

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PAD: char = '=';

// Java: public class Base64OutputStream extends OutputStream
pub struct Base64OutputStream {
    // Java field: OutputStream outputStream
    outputStream: Box<dyn Write>,
    // Java field: int buffer
    buffer: i32,
    // Java field: int bytecounter
    bytecounter: i32,
    // Java field: int linecounter
    linecounter: i32,
    // Java field: int linelength
    linelength: i32,
}

// Java: public class Base64OutputStream extends OutputStream
impl Base64OutputStream {
    // Java: Base64OutputStream(OutputStream outputStream)
    pub fn new(outputStream: Box<dyn Write>) -> Self {
        // Java: this(outputStream, 76);
        Self::new_with_wrap(outputStream, 76)
    }

    // Java: Base64OutputStream(OutputStream outputStream, int wrapAt)
    pub fn new_with_wrap(outputStream: Box<dyn Write>, wrapAt: i32) -> Self {
        // Java: super();
        // Java: this.outputStream = null;
        // Java: this.buffer = 0;
        // Java: this.bytecounter = 0;
        // Java: this.linecounter = 0;
        // Java: this.linelength = 0;
        // Java: this.outputStream = outputStream;
        // Java: this.linelength = wrapAt;
        Self {
            outputStream,
            buffer: 0,
            bytecounter: 0,
            linecounter: 0,
            linelength: wrapAt,
        }
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, b: i32) -> io::Result<()> {
        // Java: int value = (b & 255) << 16 - this.bytecounter * 8;
        let value = (b & 255) << (16 - self.bytecounter * 8);
        // Java: this.buffer |= value;
        self.buffer |= value;
        // Java: ++this.bytecounter;
        self.bytecounter += 1;
        // Java: if (this.bytecounter == 3)
        if self.bytecounter == 3
        {
            // Java: this.commit();
            self.commit()?;
        }
        Ok(())
    }

    // Java: void close()
    pub fn close(&mut self) -> io::Result<()> {
        // Java: this.commit();
        self.commit()?;
        // Java: this.outputStream.close();
        self.outputStream.flush()?;
        Ok(())
    }

    // Java: void commit()
    pub fn commit(&mut self) -> io::Result<()> {
        // Java: if (this.bytecounter > 0)
        if self.bytecounter > 0
        {
            // Java: if (this.linelength > 0 && this.linecounter == this.linelength)
            if self.linelength > 0 && self.linecounter == self.linelength
            {
                // Java: this.outputStream.write("\r\n".getBytes());
                self.outputStream.write_all(b"\r\n")?;
                // Java: this.linecounter = 0;
                self.linecounter = 0;
            }
            // Java: char b1 = Shared.chars.charAt(this.buffer << 8 >>> 26);
            let b1 = CHARS.as_bytes()[((self.buffer << 8) as u32 >> 26) as usize] as char;
            // Java: char b2 = Shared.chars.charAt(this.buffer << 14 >>> 26);
            let b2 = CHARS.as_bytes()[((self.buffer << 14) as u32 >> 26) as usize] as char;
            // Java: char b3 = this.bytecounter < 2 ? Shared.pad : Shared.chars.charAt(this.buffer << 20 >>> 26);
            let b3 = if self.bytecounter < 2 { PAD } else { CHARS.as_bytes()[((self.buffer << 20) as u32 >> 26) as usize] as char };
            // Java: char b4 = this.bytecounter < 3 ? Shared.pad : Shared.chars.charAt(this.buffer << 26 >>> 26);
            let b4 = if self.bytecounter < 3 { PAD } else { CHARS.as_bytes()[((self.buffer << 26) as u32 >> 26) as usize] as char };
            // Java: this.outputStream.write(b1);
            self.outputStream.write_all(&[b1 as u8])?;
            // Java: this.outputStream.write(b2);
            self.outputStream.write_all(&[b2 as u8])?;
            // Java: this.outputStream.write(b3);
            self.outputStream.write_all(&[b3 as u8])?;
            // Java: this.outputStream.write(b4);
            self.outputStream.write_all(&[b4 as u8])?;
            // Java: this.linecounter += 4;
            self.linecounter += 4;
            // Java: this.bytecounter = 0;
            self.bytecounter = 0;
            // Java: this.buffer = 0;
            self.buffer = 0;
        }
        Ok(())
    }
}
