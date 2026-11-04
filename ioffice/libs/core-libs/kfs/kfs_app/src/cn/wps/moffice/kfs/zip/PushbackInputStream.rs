#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Rust-native implementation of `java.io.PushbackInputStream`.
//! (Rust-native: JDK 类。`libs/core-libs/stubs/src/java/io/PushbackInputStream.rs` 只是
//! 自动生成的签名占位（`new()` → `todo!()`），本 crate 需要一个**可用**的回推流，
//! 因此在这里给出一份实现；若之后其他 crate 也需要，应提升到共享位置。)
//!
//! 为什么 ZIP 读路径非它不可：`ZipInputStream.closeEntry()` 需要把“已经从底层流读进
//! 缓冲区、但压缩器没有消费掉”的尾巴还回去（Java 侧就是
//! `((PushbackInputStream)this.in).unread(this.buf, this.len - diff, diff)`），
//! 否则下一个 entry 的 local header 就会错位。Java 用 `PushbackInputStream(stream, 512)`，
//! 回推容量 512 == `InflaterInputStream` 的读缓冲 512，所以 `diff` 永远装得下。
//!
//! 与 Java 的差异（其余保持 1:1）：
//!   * `InputStream.available()` 在 Rust 的 `std::io::Read` 里没有对应物，
//!     `available()` 只返回回推缓冲里剩余字节数（保守值，绝不虚报）；
//!   * `close()` 只能释放持有的底层流（Java 是 `in.close()`），Rust 的 `Read` 没有 close，
//!     真正的关闭发生在 `Drop`；
//!   * `mark/reset/markSupported/skip` 与本 crate 的使用场景无关，未移植。

use std::io::{self, Read};

use crate::kfs::zip::ZipErrors;

// Java: public class PushbackInputStream extends FilterInputStream
pub struct PushbackInputStream {
    // Java field:    protected byte[] buf;
    buf: Vec<u8>,
    // Java field:    protected int pos;
    // Java 语义：buf[pos..] 是已回推但还没被读走的字节；pos == buf.length 表示缓冲为空
    pos: usize,
    // Java field: FilterInputStream.in
    input: Option<Box<dyn Read>>,
}

impl PushbackInputStream {
    // Java: public PushbackInputStream(InputStream in, int size)
    pub fn new__inputstream_int(input: Box<dyn Read>, size: i32) -> PushbackInputStream {
        // Java: if (size <= 0) throw new IllegalArgumentException("size <= 0");
        if size <= 0 {
            panic!("IllegalArgumentException: size <= 0");
        }
        Self {
            buf: vec![0u8; size as usize],
            // Java: this.pos = size;  —— 初始状态缓冲为空
            pos: size as usize,
            input: Some(input),
        }
    }

    // Java: public PushbackInputStream(InputStream in)
    pub fn new__inputstream(input: Box<dyn Read>) -> PushbackInputStream {
        Self::new__inputstream_int(input, 1)
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> io::Result<i32> {
        self.ensureOpen()?;
        // Java: if (this.pos < this.buf.length) return this.buf[this.pos++] & 255;
        if self.pos < self.buf.len() {
            let b = self.buf[self.pos];
            self.pos += 1;
            return Ok((b as i32) & 255);
        }
        // Java: return super.read();
        let mut one = [0u8; 1];
        let read = self.readFromSource(&mut one)?;
        if read == -1 {
            Ok(-1)
        } else {
            Ok((one[0] as i32) & 255)
        }
    }

    // Java: public int read(byte[] b) throws IOException
    pub fn read__bytearr(&mut self, buffer: &mut [u8]) -> io::Result<i32> {
        let len = buffer.len() as i32;
        self.read__bytearr_int_int(buffer, 0, len)
    }

    // Java: public int read(byte[] b, int off, int len) throws IOException
    pub fn read__bytearr_int_int(
        &mut self,
        buffer: &mut [u8],
        byteOffset: i32,
        byteCount: i32,
    ) -> io::Result<i32> {
        self.ensureOpen()?;
        // Java: if (len <= 0) { if (len < 0) throw new IndexOutOfBoundsException(); else if (len == 0) return 0; }
        if byteCount < 0 || byteOffset < 0 {
            // Java: IndexOutOfBoundsException —— RuntimeException，不属 IOException 谱系
            panic!("IndexOutOfBoundsException");
        }
        if byteCount == 0 {
            return Ok(0);
        }
        let byteOffset = byteOffset as usize;
        let mut byteCount = byteCount as usize;

        // Java: int avail = this.buf.length - this.pos;
        let mut avail = self.buf.len() - self.pos;
        if avail > 0 {
            if byteCount < avail {
                avail = byteCount;
            }
            // Java: System.arraycopy(this.buf, this.pos, b, off, avail);
            buffer[byteOffset..byteOffset + avail]
                .copy_from_slice(&self.buf[self.pos..self.pos + avail]);
            self.pos += avail;
            byteCount -= avail;
        }

        if byteCount > 0 {
            // Java: len = super.read(b, off, len);
            let read = self.readFromSource(&mut buffer[byteOffset + avail..byteOffset + avail + byteCount])?;
            // Java: if (len == -1) return avail == 0 ? -1 : avail;
            if read == -1 {
                return Ok(if avail == 0 { -1 } else { avail as i32 });
            }
            // Java: return avail + len;
            return Ok((avail as i32) + read);
        }
        // Java: return avail;
        Ok(avail as i32)
    }

    // Java: public void unread(int b) throws IOException
    pub fn unread(&mut self, b: i32) -> io::Result<()> {
        self.ensureOpen()?;
        // Java: if (this.pos == 0) throw new IOException("Push back buffer is full");
        if self.pos == 0 {
            return Err(ZipErrors::ioException("Push back buffer is full"));
        }
        self.pos -= 1;
        self.buf[self.pos] = b as u8;
        Ok(())
    }

    // Java: public void unread(byte[] b) throws IOException
    pub fn unread__bytearr(&mut self, b: &[u8]) -> io::Result<()> {
        let len = b.len() as i32;
        self.unread__bytearr_int_int(b, 0, len)
    }

    // Java: public void unread(byte[] b, int off, int len) throws IOException
    pub fn unread__bytearr_int_int(
        &mut self,
        b: &[u8],
        off: i32,
        len: i32,
    ) -> io::Result<()> {
        self.ensureOpen()?;
        // Java: if (len > this.pos) throw new IOException("Push back buffer is full");
        if len < 0 || len as usize > self.pos {
            return Err(ZipErrors::ioException("Push back buffer is full"));
        }
        let off = off as usize;
        let len = len as usize;
        self.pos -= len;
        // Java: System.arraycopy(b, off, this.buf, this.pos, len);
        self.buf[self.pos..self.pos + len].copy_from_slice(&b[off..off + len]);
        Ok(())
    }

    // Java: public int available() throws IOException
    pub fn available(&self) -> io::Result<i32> {
        self.ensureOpen()?;
        // Java: int n = this.buf.length - this.pos; int avail = super.available();
        // Rust: std::io::Read 没有 available()，只报告回推缓冲里的剩余字节（保守值）
        Ok((self.buf.len() - self.pos) as i32)
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) -> io::Result<()> {
        // Java: if (this.in == null) return; this.in.close(); this.buf = null; this.in = null;
        // Rust: 释放持有的底层流（其真正关闭在 Drop），并清空回推缓冲
        self.input = None;
        self.buf.clear();
        self.pos = 0;
        Ok(())
    }

    // Java: private void ensureOpen() throws IOException
    fn ensureOpen(&self) -> io::Result<()> {
        if self.input.is_none() {
            // Java: throw new IOException("Stream closed");
            return Err(ZipErrors::ioException("Stream closed"));
        }
        Ok(())
    }

    // Rust-native: 从底层流读取；Java 的 InputStream.read 用 -1 表示 EOF，std::io::Read 用 Ok(0)
    fn readFromSource(&mut self, buffer: &mut [u8]) -> io::Result<i32> {
        let input = self
            .input
            .as_mut()
            .ok_or_else(|| ZipErrors::ioException("Stream closed"))?;
        match input.read(buffer) {
            Ok(0) => Ok(-1),
            Ok(n) => Ok(n as i32),
            Err(e) => Err(e),
        }
    }
}

// Rust-native: 让 PushbackInputStream 可以直接当 `Box<dyn Read>` 用
impl Read for PushbackInputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let ret = self.read__bytearr_int_int(buf, 0, buf.len() as i32)?;
        if ret < 0 {
            Ok(0)
        } else {
            Ok(ret as usize)
        }
    }
}
