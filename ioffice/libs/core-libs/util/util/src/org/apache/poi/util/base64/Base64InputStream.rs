#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::{self, Read};

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PAD: char = '=';

// Java: public class Base64InputStream extends InputStream
pub struct Base64InputStream {
    // Java field: InputStream inputStream
    inputStream: Box<dyn Read>,
    // Java field: int[] buffer
    pub(crate) buffer: Option<Vec<i32>>,
    // Java field: int bufferCounter
    bufferCounter: i32,
    // Java field: boolean eof
    eof: bool,
    // Java field: boolean endIgnoreLineFeed
    endIgnoreLineFeed: bool,
}

// Java: public class Base64InputStream extends InputStream
impl Base64InputStream {
    // Java: Base64InputStream(InputStream inputStream)
    pub fn new(inputStream: Box<dyn Read>) -> Self {
        // Java: this(inputStream, false);
        Self::new_with_options(inputStream, false)
    }

    // Java: Base64InputStream(InputStream inputStream, boolean endIgnoreLineFeed)
    pub fn new_with_options(inputStream: Box<dyn Read>, endIgnoreLineFeed: bool) -> Self {
        // Java: super();
        // Java: this.bufferCounter = 0;
        // Java: this.eof = false;
        // Java: this.endIgnoreLineFeed = false;
        // Java: this.inputStream = inputStream;
        // Java: this.endIgnoreLineFeed = endIgnoreLineFeed;
        Self {
            inputStream,
            buffer: None,
            bufferCounter: 0,
            eof: false,
            endIgnoreLineFeed,
        }
    }

    fn read_byte(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 1];
        match self.inputStream.read(&mut buf) {
            Ok(0) => Ok(-1),
            Ok(_) => Ok(buf[0] as i32),
            Err(e) => Err(e),
        }
    }

    // Java: int read()
    pub fn read(&mut self) -> io::Result<i32> {
        // Java: if (this.buffer == null || this.bufferCounter == this.buffer.length)
        if self.buffer.is_none() || self.bufferCounter == self.buffer.as_ref().map_or(0, |b| b.len() as i32)
        {
            // Java: if (this.eof)
            if self.eof
            {
                // Java: return -1;
                return Ok(-1);
            }
            // Java: this.acquire();
            self.acquire()?;
            // Java: if (this.buffer.length == 0)
            if self.buffer.as_ref().map_or(true, |b| b.is_empty())
            {
                // Java: this.buffer = null;
                self.buffer = None;
                // Java: return -1;
                return Ok(-1);
            }
            // Java: this.bufferCounter = 0;
            self.bufferCounter = 0;
        }
        // Java: return this.buffer[this.bufferCounter++];
        let val = self.buffer.as_ref().unwrap()[self.bufferCounter as usize];
        self.bufferCounter += 1;
        Ok(val)
    }

    // Java: void acquire()
    fn acquire(&mut self) -> io::Result<()> {
        // Java: char[] four = new char[4];
        let mut four = ['\0'; 4];
        // Java: int i = 0;
        let mut i = 0;
        // Java: do
        loop
        {
            // Java: int b = this.inputStream.read();
            let b = self.read_byte()?;
            // Java: if (b == -1)
            if b == -1
            {
                // Java: if (i != 0)
                if i != 0
                {
                    // Java: throw new IOException("Bad base64 stream");
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad base64 stream"));
                }
                // Java: this.buffer = new int[0];
                self.buffer = Some(Vec::new());
                // Java: this.eof = true;
                self.eof = true;
                // Java: return;
                return Ok(());
            }
            // Java: char c = (char)b;
            let c = b as u8 as char;
            // Java: if (Shared.chars.indexOf(c) == -1 && c != Shared.pad)
            if CHARS.find(c).is_none() && c != PAD
            {
                // Java: if (c != '\r' && c != '\n')
                if c != '\r' && c != '\n'
                {
                    // Java: throw new IOException("Bad base64 stream");
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad base64 stream"));
                }
            }
            // Java: else
            else
            {
                // Java: four[i++] = c;
                four[i] = c;
                i += 1;
            }
            if i >= 4 { break; }
        }
        // Java: boolean padded = false;
        let mut padded = false;
        // Java: for(int var6 = 0; var6 < 4; ++var6)
        for var6 in 0..4
        {
            // Java: if (four[var6] != Shared.pad)
            if four[var6] != PAD
            {
                // Java: if (padded)
                if padded
                {
                    // Java: throw new IOException("Bad base64 stream");
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad base64 stream"));
                }
            }
            // Java: else if (!padded)
            else if !padded
            {
                // Java: padded = true;
                padded = true;
            }
        }
        // Java: int l;
        let l: i32;
        // Java: if (four[3] == Shared.pad)
        if four[3] == PAD
        {
            // Java: this.endJudgeException();
            self.endJudgeException()?;
            // Java: this.eof = true;
            self.eof = true;
            // Java: if (four[2] == Shared.pad)
            if four[2] == PAD
            {
                // Java: l = 1;
                l = 1;
            }
            // Java: else
            else
            {
                // Java: l = 2;
                l = 2;
            }
        }
        // Java: else
        else
        {
            // Java: l = 3;
            l = 3;
        }
        // Java: int aux = 0;
        let mut aux: i32 = 0;
        // Java: for(int var7 = 0; var7 < 4; ++var7)
        for var7 in 0..4
        {
            // Java: if (four[var7] != Shared.pad)
            if four[var7] != PAD
            {
                // Java: aux |= Shared.chars.indexOf(four[var7]) << 6 * (3 - var7);
                let idx = CHARS.find(four[var7]).unwrap_or(0) as i32;
                aux |= idx << (6 * (3 - var7));
            }
        }
        // Java: this.buffer = new int[l];
        let mut buf = vec![0i32; l as usize];
        // Java: for(int var8 = 0; var8 < l; ++var8)
        for var8 in 0..l as usize
        {
            // Java: this.buffer[var8] = aux >>> 8 * (2 - var8) & 255;
            buf[var8] = (aux >> (8 * (2 - var8 as i32))) & 255;
        }
        self.buffer = Some(buf);
        Ok(())
    }

    // Java: void close()
    pub fn close(&mut self) -> io::Result<()> {
        // Java: this.inputStream.close();
        Ok(())
    }

    // Java: void endJudgeException()
    fn endJudgeException(&mut self) -> io::Result<()> {
        // Java: if (!this.endIgnoreLineFeed)
        if !self.endIgnoreLineFeed
        {
            // Java: if (this.inputStream.read() != -1)
            if self.read_byte()? != -1
            {
                // Java: throw new IOException("Bad base64 stream");
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad base64 stream"));
            }
        }
        // Java: else
        else
        {
            // Java: int read;
            let mut read;
            // Java: do
            loop
            {
                // Java: read = this.inputStream.read();
                read = self.read_byte()?;
                if read != 13 && read != 10 { break; }
            }
            // Java: if (read != -1)
            if read != -1
            {
                // Java: throw new IOException("Bad base64 stream");
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad base64 stream"));
            }
        }
        Ok(())
    }
}
