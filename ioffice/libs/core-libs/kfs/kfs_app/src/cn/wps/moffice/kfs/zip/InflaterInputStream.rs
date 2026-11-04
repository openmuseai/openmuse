#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Java `cn.wps.moffice.kfs.zip.InflaterInputStream` 的 Rust 侧。
//!
//! 关键对齐点：这是**真正做解压**的地方（`ZipInputStream.read` 的 method=8 分支、
//! `ZipFile.getInputStream` 都经过它），`inf` 是 `java.util.zip.Inflater`，
//! Rust 侧对应 `Inflater`（`Inflater(true)` = raw deflate）。
//!
//! 两层结构：`try_xxx` 核心返回 `std::io::Result`，Java 签名的 `xxx` 用 panic 还原抛出语义。

use std::io::{self, Read};

use crate::kfs::zip::Inflater::Inflater;
use crate::kfs::zip::Streams::Streams;
use crate::kfs::zip::ZipErrors;

// Java field:    static final int BUF_SIZE = 512;
pub const BUF_SIZE: i32 = 512;

// Java: public class InflaterInputStream extends FilterInputStream
#[derive(Default)]
pub struct InflaterInputStream {
    // Java field:    protected Inflater inf;
    pub inf: Inflater,
    // Java field:    protected byte[] buf;
    pub buf: Vec<u8>,
    // Java field:    protected int len;
    pub len: i32,
    // Java field:    boolean closed;
    pub closed: bool,
    // Java field:    boolean eof;
    pub eof: bool,
    // Java field:    int nativeEndBufSize;
    pub nativeEndBufSize: i32,
    // Java field: FilterInputStream.in
    pub input: Option<Box<dyn Read>>,
}

// Java: public class InflaterInputStream extends FilterInputStream
impl InflaterInputStream {
    // Java: public InflaterInputStream(InputStream is)
    pub fn new(is: Box<dyn Read>) -> InflaterInputStream {
        Self::new__input_deflater_int(is, Inflater::new(false), BUF_SIZE)
    }

    // Java: public InflaterInputStream(InputStream is, Inflater inflater)
    pub fn new__input_deflater(is: Box<dyn Read>, inf: Inflater) -> InflaterInputStream {
        Self::new__input_deflater_int(is, inf, BUF_SIZE)
    }

    // Java: public InflaterInputStream(InputStream is, Inflater inflater, int bufferSize)
    pub fn new__input_deflater_int(
        is: Box<dyn Read>,
        inf: Inflater,
        bufferSize: i32,
    ) -> InflaterInputStream {
        // Java: super(is); this.nativeEndBufSize = 0;
        // Java: if (is == null) throw new NullPointerException("is == null");
        // Java: else if (inflater == null) throw new NullPointerException("inflater == null");
        // Rust: `is` / `inf` 都是值类型，null 由类型系统排除。
        // Java: else if (bufferSize <= 0) throw new IllegalArgumentException("bufferSize <= 0: " + bufferSize);
        if bufferSize <= 0 {
            panic!("IllegalArgumentException: bufferSize <= 0: {}", bufferSize);
        }
        Self {
            inf,
            // Java: if (is instanceof ZipFile.RAFStream) { this.nativeEndBufSize = bufferSize; }
            // Rust: RAFStream 不在 Rust 侧走 native 缓冲，统一分配读缓冲（记录为平台差异）
            buf: vec![0u8; bufferSize as usize],
            nativeEndBufSize: 0,
            input: Some(is),
            len: 0,
            closed: false,
            eof: false,
        }
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        // Java: return Streams.readSingleByte(this);
        let mut buffer = [0u8; 1];
        match self.read__bytearr_int_int(&mut buffer, 0, 1) {
            // Java: result != -1 ? buffer[0] & 255 : -1
            -1 => -1,
            _ => (buffer[0] as i32) & 255,
        }
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read__bytearr_int_int(
        &mut self,
        buffer: &mut [u8],
        byteOffset: usize,
        byteCount: usize,
    ) -> i32 {
        self.try_read__bytearr_int_int(buffer, byteOffset, byteCount)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: read(byte[],int,int) 的可恢复版本
    pub fn try_read__bytearr_int_int(
        &mut self,
        buffer: &mut [u8],
        byteOffset: usize,
        byteCount: usize,
    ) -> io::Result<i32> {
        // Java: this.checkClosed();
        self.try_checkClosed()?;
        // Java: ArrayUtils.checkOffsetAndCount(buffer.length, byteOffset, byteCount);
        // Java: ArrayIndexOutOfBoundsException（RuntimeException）→ panic
        if byteOffset > buffer.len() || buffer.len() - byteOffset < byteCount {
            panic!("ArrayIndexOutOfBoundsException");
        }
        // Java: if (byteCount == 0) return 0;
        if byteCount == 0 {
            return Ok(0);
        }
        // Java: else if (this.eof) return -1;
        if self.eof {
            return Ok(-1);
        }

        // Java: while(true)
        loop {
            // Java: if (this.inf.needsInput()) this.fill();
            if self.inf.needsInput() {
                self.try_fill()?;
            }

            // Java: int result = this.inf.inflate(buffer, byteOffset, byteCount);
            let result = match self.inf.inflate(buffer, byteOffset, byteCount) {
                Ok(result) => result,
                Err(e) => {
                    // Java: catch (DataFormatException e) { this.eof = true; if (this.len == -1) throw new EOFException();
                    //        throw (IOException)(new IOException()).initCause(e); }
                    self.eof = true;
                    if self.len == -1 {
                        return Err(ZipErrors::eofException());
                    }
                    return Err(e);
                }
            };
            // Java: this.eof = this.inf.finished();
            self.eof = self.inf.finished();
            // Java: if (result > 0) return result;
            if result > 0 {
                return Ok(result);
            }
            // Java: if (this.eof) return -1;
            if self.eof {
                return Ok(-1);
            }
            // Java: if (this.inf.needsDictionary()) { this.eof = true; return -1; }
            if self.inf.needsDictionary() {
                self.eof = true;
                return Ok(-1);
            }
            // Java: if (this.len == -1) { this.eof = true; throw new EOFException(); }
            if self.len == -1 {
                self.eof = true;
                return Err(ZipErrors::eofException());
            }
            // Rust: 到这里说明本次 inflate 既没产出、也没置 finished、又不需要输入（Java 会在
            // while(true) 里自旋）。返回 0 以避免活锁；正常 zlib 流不会走到这里。
            if !self.inf.needsInput() {
                return Ok(0);
            }
        }
    }

    // Java: protected void fill() throws IOException
    pub fn fill(&mut self) {
        self.try_fill().unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: fill 的可恢复版本
    pub fn try_fill(&mut self) -> io::Result<()> {
        // Java: this.checkClosed();
        self.try_checkClosed()?;
        // Java: if ((this.len = this.in.read(this.buf)) > 0) this.inf.setInput(this.buf, 0, this.len);
        let len = match self.input.as_mut() {
            Some(input) => {
                // Rust: 借用拆分——input 与 buf 是不同字段
                match input.read(&mut self.buf) {
                    Ok(0) => -1,
                    Ok(n) => n as i32,
                    Err(e) => return Err(e),
                }
            }
            None => -1,
        };
        self.len = len;
        if self.len > 0 {
            // Java: this.inf.setInput(this.buf, 0, this.len);
            self.inf.setInput(&self.buf, 0, self.len);
        }
        Ok(())
    }

    // Java: public long skip(long byteCount) throws IOException
    pub fn skip(&mut self, byteCount: i64) -> i64 {
        // Java: if (byteCount < 0L) throw new IllegalArgumentException("byteCount < 0");
        if byteCount < 0 {
            panic!("IllegalArgumentException: byteCount < 0");
        }
        // Java: return Streams.skipByReading(this, byteCount);
        Streams::skipByReading(self, byteCount)
    }

    // Java: public int available() throws IOException
    pub fn available(&self) -> i32 {
        // Java: this.checkClosed();
        self.checkClosed();
        // Java: return this.eof ? 0 : 1;
        if self.eof {
            0
        } else {
            1
        }
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        // Java: if (!this.closed)
        if !self.closed {
            // Java: this.inf.end();
            self.inf.end();
            // Java: this.closed = true;
            self.closed = true;
            // Java: this.eof = true;
            self.eof = true;
            // Java: super.close();  → FilterInputStream.close() → in.close()
            // Rust: std::io::Read 没有 close()，释放持有权即可（底层流随 Drop 关闭）
            self.input = None;
        }
    }

    // Java: public void mark(int readlimit)
    pub fn mark(&mut self, _readlimit: i32) {}

    // Java: public void reset() throws IOException
    pub fn reset(&mut self) {
        // Java: throw new IOException();
        panic!("IOException: mark/reset not supported");
    }

    // Java: public boolean markSupported()
    pub fn markSupported(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: private void checkClosed() throws IOException
    pub fn checkClosed(&self) {
        self.try_checkClosed()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: checkClosed 的可恢复版本
    pub fn try_checkClosed(&self) -> io::Result<()> {
        // Java: if (this.closed) throw new IOException("Stream is closed");
        if self.closed {
            return Err(ZipErrors::ioException("Stream is closed"));
        }
        Ok(())
    }
}

// Rust-native: 让 InflaterInputStream 能直接当 `Box<dyn Read>` / `&mut dyn Read` 用
impl Read for InflaterInputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let ret = self.try_read__bytearr_int_int(buf, 0, buf.len())?;
        if ret < 0 {
            Ok(0)
        } else {
            Ok(ret as usize)
        }
    }
}
