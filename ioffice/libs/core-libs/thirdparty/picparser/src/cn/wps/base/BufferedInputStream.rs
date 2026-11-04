#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use std::io;

// Java: import cn.wps.moffice.kfs.FileInputStream
use core_libs_kfs::kfs::mfs::FileInputStream::FileInputStream;
// Java: import java.io.FileNotFoundException
// Java: import java.io.FilterInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream

use super::BufferBytes::BufferBytes;

fn stream_closed_err() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "BufferedInputStream is closed")
}

// Java: public class BufferedInputStream extends FilterInputStream
pub struct BufferedInputStream {
    // Java field: BufferBytes buffer
    buffer: Option<BufferBytes>,
    // Java field: byte[] buf
    pub(crate) buf: Option<Vec<u8>>,
    // Java field: int count
    pub(crate) count: i32,
    // Java field: int marklimit
    pub(crate) marklimit: i32,
    // Java field: int markpos
    pub(crate) markpos: i32,
    // Java field: int pos
    pub(crate) pos: i32,
    inner: Option<FileInputStream>,
}

// Java: public class BufferedInputStream extends FilterInputStream
impl BufferedInputStream {
    // Java: BufferedInputStream(String fileName)
    pub fn from_filename(fileName: &str) -> io::Result<Self> {
        // Java: super(new FileInputStream(fileName));
        let inner = FileInputStream::from_name(fileName)?;
        // Java: this.buffer = BufferBytes.obtain();
        let mut buffer = BufferBytes::obtain();
        // Java: this.buf = this.buffer.bytes;
        let buf = std::mem::take(&mut buffer.bytes);
        Ok(BufferedInputStream {
            inner: Some(inner),
            buffer: Some(buffer),
            buf: Some(buf),
            count: 0,
            marklimit: 0,
            markpos: -1,
            pos: 0,
        })
    }

    // Java: BufferedInputStream(InputStream in)
    pub fn from_stream(input: FileInputStream) -> Self {
        // Java: super(in);
        // Java: this.buffer = BufferBytes.obtain();
        let mut buffer = BufferBytes::obtain();
        // Java: this.buf = this.buffer.bytes;
        let buf = std::mem::take(&mut buffer.bytes);
        BufferedInputStream {
            inner: Some(input),
            buffer: Some(buffer),
            buf: Some(buf),
            count: 0,
            marklimit: 0,
            markpos: -1,
            pos: 0,
        }
    }

    // Java: BufferedInputStream(InputStream in, byte[] bytes)
    pub fn from_stream_with_buf(input: FileInputStream, bytes: Vec<u8>) -> Self {
        // Java: super(in);
        // Java: this.buf = bytes;
        BufferedInputStream {
            inner: Some(input),
            buffer: None,
            buf: Some(bytes),
            count: 0,
            marklimit: 0,
            markpos: -1,
            pos: 0,
        }
    }

    // Java: int available()
    pub fn available(&self) -> io::Result<i32> {
        // Java: InputStream localIn = this.in;
        // Java: if (this.buf != null && localIn != null)
        if self.buf.is_some() && self.inner.is_some() {
            // Java: return this.count - this.pos + localIn.available();
            let avail = self.inner.as_ref().unwrap().available()?;
            Ok(self.count - self.pos + avail)
        }
        // Java: else
        else {
            // Java: throw this.streamClosed();
            Err(stream_closed_err())
        }
    }

    // Java: IOException streamClosed()
    fn streamClosed(&self) -> io::Error {
        // Java: throw new IOException("BufferedInputStream is closed");
        stream_closed_err()
    }

    // Java: void close()
    pub fn close(&mut self) -> io::Result<()> {
        // Java: this.buf = null;
        self.buf = None;
        // Java: InputStream localIn = this.in;
        // Java: this.in = null;
        let local_in = self.inner.take();
        // Java: if (localIn != null)
        if let Some(mut local_in) = local_in {
            // Java: localIn.close();
            local_in.close()?;
        }
        // Java: if (this.buffer != null)
        if let Some(buffer) = self.buffer.take() {
            // Java: this.buffer.recycle();
            buffer.recycle();
            // Java: this.buffer = null;
        }
        Ok(())
    }

    // Java: void lightClose()
    pub fn lightClose(&mut self) {
        // Java: this.buf = null;
        self.buf = None;
        // Java: if (this.buffer != null)
        if let Some(buffer) = self.buffer.take() {
            // Java: this.buffer.recycle();
            buffer.recycle();
            // Java: this.buffer = null;
        }
    }

    // Java: int fillbuf(InputStream localIn, byte[] localBuf)
    fn fillbuf(&mut self) -> io::Result<i32> {
        // Java: if (this.markpos != -1 && this.pos - this.markpos < this.marklimit)
        if self.markpos != -1 && self.pos - self.markpos < self.marklimit {
            {
                let buf = self.buf.as_mut().ok_or_else(stream_closed_err)?;
                let buf_len = buf.len();
                // Java: if (this.markpos == 0 && this.marklimit > localBuf.length)
                if self.markpos == 0 && self.marklimit > buf_len as i32 {
                    // Java: int newLength = localBuf.length * 2;
                    let mut newLength = buf_len * 2;
                    // Java: if (newLength > this.marklimit)
                    if newLength > self.marklimit as usize {
                        // Java: newLength = this.marklimit;
                        newLength = self.marklimit as usize;
                    }
                    // Java: byte[] newbuf = new byte[newLength];
                    let mut newbuf = vec![0u8; newLength];
                    // Java: System.arraycopy(localBuf, 0, newbuf, 0, localBuf.length);
                    newbuf[..buf_len].copy_from_slice(buf);
                    // Java: localBuf = this.buf = newbuf;
                    *buf = newbuf;
                }
                // Java: else if (this.markpos > 0)
                else if self.markpos > 0 {
                    // Java: System.arraycopy(localBuf, this.markpos, localBuf, 0, localBuf.length - this.markpos);
                    let mp = self.markpos as usize;
                    buf.copy_within(mp.., 0);
                }
            }
            // Java: this.pos -= this.markpos;
            self.pos -= self.markpos;
            // Java: this.count = this.markpos = 0;
            self.markpos = 0;
            self.count = 0;
            // Java: int bytesread = localIn.read(localBuf, this.pos, localBuf.length - this.pos);
            let pos = self.pos as usize;
            let (buf_opt, inner_opt) = (&mut self.buf, &mut self.inner);
            let buf = buf_opt.as_mut().ok_or_else(stream_closed_err)?;
            let buf_len = buf.len();
            let inner = inner_opt.as_mut().ok_or_else(stream_closed_err)?;
            let bytesread = inner.read_buf(buf, pos, buf_len - pos)?;
            // Java: this.count = bytesread <= 0 ? this.pos : this.pos + bytesread;
            self.count = if bytesread <= 0 { self.pos } else { self.pos + bytesread };
            // Java: return bytesread;
            Ok(bytesread)
        }
        // Java: else
        else {
            // Java: int result = localIn.read(localBuf);
            let (buf_opt, inner_opt) = (&mut self.buf, &mut self.inner);
            let buf = buf_opt.as_mut().ok_or_else(stream_closed_err)?;
            let buf_len = buf.len();
            let inner = inner_opt.as_mut().ok_or_else(stream_closed_err)?;
            let result = inner.read_buf(buf, 0, buf_len)?;
            // Java: if (result > 0)
            if result > 0 {
                // Java: this.markpos = -1;
                self.markpos = -1;
                // Java: this.pos = 0;
                self.pos = 0;
                // Java: this.count = result == -1 ? 0 : result;
                self.count = if result == -1 { 0 } else { result };
            }
            // Java: return result;
            Ok(result)
        }
    }

    // Java: void mark(int readlimit)
    pub fn mark(&mut self, readlimit: i32) {
        // Java: this.marklimit = readlimit;
        self.marklimit = readlimit;
        // Java: this.markpos = this.pos;
        self.markpos = self.pos;
    }

    // Java: boolean markSupported()
    pub fn markSupported(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: int read()
    pub fn read_one(&mut self) -> io::Result<i32> {
        // Java: byte[] localBuf = this.buf;
        // Java: InputStream localIn = this.in;
        // Java: if (localBuf != null && localIn != null)
        if self.buf.is_some() && self.inner.is_some() {
            // Java: if (this.pos >= this.count && this.fillbuf(localIn, localBuf) == -1)
            if self.pos >= self.count && self.fillbuf()? == -1 {
                // Java: return -1;
                return Ok(-1);
            }
            // Java: else
            {
                // Java: if (localBuf != this.buf)
                // Java: localBuf = this.buf;
                // Java: if (localBuf == null)
                if self.buf.is_none() {
                    // Java: throw this.streamClosed();
                    return Err(stream_closed_err());
                }
                // Java: return this.count - this.pos > 0 ? localBuf[this.pos++] & 255 : -1;
                let buf = self.buf.as_ref().unwrap();
                if self.count - self.pos > 0 {
                    let byte_val = buf[self.pos as usize];
                    self.pos += 1;
                    Ok((byte_val as i32) & 255)
                } else {
                    Ok(-1)
                }
            }
        }
        // Java: else
        else {
            // Java: throw this.streamClosed();
            Err(stream_closed_err())
        }
    }

    // Java: int read(byte[] buffer, int offset, int length)
    pub fn read_to_buf(&mut self, output: &mut [u8], mut offset: usize, length: usize) -> io::Result<i32> {
        // Java: byte[] localBuf = this.buf;
        // Java: if (localBuf == null)
        if self.buf.is_none() {
            // Java: throw this.streamClosed();
            return Err(stream_closed_err());
        }
        // Java: else if (buffer == null)
        // (not applicable in Rust - output is always valid)
        // Java: throw new NullPointerException("buffer == null");
        // Java: else if ((offset | length) >= 0 && offset <= buffer.length - length)
        if (offset | length) <= output.len() && offset + length <= output.len() {
            // Java: if (length == 0)
            if length == 0 {
                // Java: return 0;
                return Ok(0);
            }
            // Java: else
            {
                // Java: InputStream localIn = this.in;
                // Java: if (localIn == null)
                if self.inner.is_none() {
                    // Java: throw this.streamClosed();
                    return Err(stream_closed_err());
                }
                // Java: else
                {
                    // Java: int required;
                    let mut required: usize;
                    // Java: if (this.pos < this.count)
                    if self.pos < self.count {
                        // Java: int copylength = this.count - this.pos >= length ? length : this.count - this.pos;
                        let avail = (self.count - self.pos) as usize;
                        let copylength = if avail >= length { length } else { avail };
                        // Java: System.arraycopy(localBuf, this.pos, buffer, offset, copylength);
                        let buf = self.buf.as_ref().unwrap();
                        let pos = self.pos as usize;
                        output[offset..offset + copylength].copy_from_slice(&buf[pos..pos + copylength]);
                        // Java: this.pos += copylength;
                        self.pos += copylength as i32;
                        // Java: if (copylength == length || localIn.available() == 0)
                        let in_avail = self.inner.as_ref().unwrap().available()?;
                        if copylength == length || in_avail == 0 {
                            // Java: return copylength;
                            return Ok(copylength as i32);
                        }
                        // Java: offset += copylength;
                        offset += copylength;
                        // Java: required = length - copylength;
                        required = length - copylength;
                    }
                    // Java: else
                    else {
                        // Java: required = length;
                        required = length;
                    }
                    // Java: while(true)
                    loop {
                        // Java: int read;
                        let read: i32;
                        let buf_len = self.buf.as_ref().unwrap().len();
                        // Java: if (this.markpos == -1 && required >= localBuf.length)
                        if self.markpos == -1 && required >= buf_len {
                            // Java: read = localIn.read(buffer, offset, required);
                            let inner = self.inner.as_mut().ok_or_else(stream_closed_err)?;
                            read = inner.read_buf(output, offset, required)?;
                            // Java: if (read == -1)
                            if read == -1 {
                                // Java: return required == length ? -1 : length - required;
                                return Ok(if required == length { -1 } else { (length - required) as i32 });
                            }
                        }
                        // Java: else
                        else {
                            // Java: if (this.fillbuf(localIn, localBuf) == -1)
                            if self.fillbuf()? == -1 {
                                // Java: return required == length ? -1 : length - required;
                                return Ok(if required == length { -1 } else { (length - required) as i32 });
                            }
                            // Java: if (localBuf != this.buf)
                            // Java: localBuf = this.buf;
                            // Java: if (localBuf == null)
                            if self.buf.is_none() {
                                // Java: throw this.streamClosed();
                                return Err(stream_closed_err());
                            }
                            // Java: read = this.count - this.pos >= required ? required : this.count - this.pos;
                            let avail = (self.count - self.pos) as usize;
                            let r = if avail >= required { required } else { avail };
                            // Java: System.arraycopy(localBuf, this.pos, buffer, offset, read);
                            let buf = self.buf.as_ref().unwrap();
                            let pos = self.pos as usize;
                            output[offset..offset + r].copy_from_slice(&buf[pos..pos + r]);
                            // Java: this.pos += read;
                            self.pos += r as i32;
                            read = r as i32;
                        }
                        // Java: required -= read;
                        required -= read as usize;
                        // Java: if (required == 0)
                        if required == 0 {
                            // Java: return length;
                            return Ok(length as i32);
                        }
                        // Java: if (localIn.available() == 0)
                        let in_avail = self.inner.as_ref().unwrap().available()?;
                        if in_avail == 0 {
                            // Java: return length - required;
                            return Ok((length - required) as i32);
                        }
                        // Java: offset += read;
                        offset += read as usize;
                    }
                }
            }
        }
        // Java: else
        else {
            // Java: throw new IndexOutOfBoundsException();
            Err(io::Error::new(io::ErrorKind::InvalidInput, "IndexOutOfBoundsException"))
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) -> io::Result<()> {
        // Java: if (this.buf == null)
        if self.buf.is_none() {
            // Java: throw new IOException("Stream is closed");
            return Err(io::Error::new(io::ErrorKind::Other, "Stream is closed"));
        }
        // Java: else if (-1 == this.markpos)
        else if -1 == self.markpos {
            // Java: throw new IOException("Mark has been invalidated.");
            return Err(io::Error::new(io::ErrorKind::Other, "Mark has been invalidated."));
        }
        // Java: else
        else {
            // Java: this.pos = this.markpos;
            self.pos = self.markpos;
        }
        Ok(())
    }

    // Java: long skip(long amount)
    pub fn skip(&mut self, amount: i64) -> io::Result<i64> {
        // Java: byte[] localBuf = this.buf;
        // Java: InputStream localIn = this.in;
        // Java: if (localBuf == null)
        if self.buf.is_none() {
            // Java: throw this.streamClosed();
            return Err(stream_closed_err());
        }
        // Java: else if (amount < 1L)
        else if amount < 1 {
            // Java: return 0L;
            return Ok(0);
        }
        // Java: else if (localIn == null)
        else if self.inner.is_none() {
            // Java: throw this.streamClosed();
            return Err(stream_closed_err());
        }
        // Java: else if ((long)(this.count - this.pos) >= amount)
        else if (self.count - self.pos) as i64 >= amount {
            // Java: this.pos = (int)((long)this.pos + amount);
            self.pos = (self.pos as i64 + amount) as i32;
            // Java: return amount;
            return Ok(amount);
        }
        // Java: else
        else {
            // Java: long read = (long)(this.count - this.pos);
            let mut read = (self.count - self.pos) as i64;
            // Java: this.pos = this.count;
            self.pos = self.count;
            // Java: if (this.markpos != -1 && amount <= (long)this.marklimit)
            if self.markpos != -1 && amount <= self.marklimit as i64 {
                // Java: if (this.fillbuf(localIn, localBuf) == -1)
                if self.fillbuf()? == -1 {
                    // Java: return read;
                    return Ok(read);
                }
                // Java: else if ((long)(this.count - this.pos) >= amount - read)
                else if (self.count - self.pos) as i64 >= amount - read {
                    // Java: this.pos = (int)((long)this.pos + (amount - read));
                    self.pos = (self.pos as i64 + (amount - read)) as i32;
                    // Java: return amount;
                    return Ok(amount);
                }
                // Java: else
                else {
                    // Java: read += (long)(this.count - this.pos);
                    read += (self.count - self.pos) as i64;
                    // Java: this.pos = this.count;
                    self.pos = self.count;
                    // Java: return read;
                    return Ok(read);
                }
            }
            // Java: else
            else {
                // Java: return read + localIn.skip(amount - read);
                let skipped = self.inner.as_mut().unwrap().skip(amount - read)?;
                return Ok(read + skipped);
            }
        }
    }
}
