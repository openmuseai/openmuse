#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Java `cn.wps.moffice.kfs.zip.Streams` 的 Rust 侧。
//!
//! 两层结构（见 `ZipErrors.rs` 的说明）：
//!   * `try_xxx` 是核心，返回 `std::io::Result`，Rust 原生调用方可以恢复；
//!   * `xxx` 保持 Java 签名，遇到 IOException 谱系的错误就 panic，用来 1:1 对齐 Java 调用点。

use std::io::{self, Read};

use crate::kfs::zip::ZipErrors;

// Java: public class Streams
pub struct Streams {}

// Java: public class Streams
impl Streams {
    // Java: public static void readFully(InputStream in, byte[] dst, int offset, int byteCount) throws IOException
    pub fn readFully(r#in: &mut dyn Read, dst: &mut [u8], offset: usize, byteCount: usize) {
        Self::try_readFully(r#in, dst, offset, byteCount)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: readFully 的可恢复版本
    pub fn try_readFully(
        r#in: &mut dyn Read,
        dst: &mut [u8],
        offset: usize,
        byteCount: usize,
    ) -> io::Result<()> {
        // Java: if (byteCount != 0)
        if byteCount == 0 {
            return Ok(());
        }
        // Java: if (in == null) throw new NullPointerException("in == null");
        // Rust: &mut dyn Read 非空，null 由类型系统排除。
        // Java: ArrayUtils.checkOffsetAndCount(dst.length, offset, byteCount);
        // Java: ArrayIndexOutOfBoundsException 是 RuntimeException，不是 IOException —— 直接 panic。
        if offset > dst.len() || dst.len() - offset < byteCount {
            panic!("ArrayIndexOutOfBoundsException");
        }

        let mut remaining = byteCount;
        let mut off = offset;
        // Java: while(byteCount > 0)
        while remaining > 0 {
            // Java: int bytesRead = in.read(dst, offset, byteCount);
            let slice = &mut dst[off..off + remaining];
            let bytes_read = r#in.read(slice);
            // Java: if (bytesRead < 0) throw new EOFException();
            // Rust: std::io::Read 用 Ok(0) 表示 EOF
            let bytes_read = match bytes_read {
                Ok(0) => return Err(ZipErrors::eofException()),
                Ok(n) => n,
                // Java: read 抛出的 IOException 原样向上抛（旧实现把它误当成 EOFException）
                Err(e) => return Err(e),
            };
            // Java: offset += bytesRead;
            off += bytes_read;
            // Java: byteCount -= bytesRead;
            remaining -= bytes_read;
        }
        Ok(())
    }

    // Java: public static void skipAll(InputStream in) throws IOException
    pub fn skipAll(r#in: &mut dyn Read) {
        Self::try_skipAll(r#in).unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: skipAll 的可恢复版本
    pub fn try_skipAll(r#in: &mut dyn Read) -> io::Result<()> {
        // Java: do { in.skip(Long.MAX_VALUE); } while(in.read() != -1)
        let mut buffer = [0u8; 4096];
        loop {
            match r#in.read(&mut buffer) {
                // Rust: Ok(0) == Java 的 read() == -1，读到流尾就结束
                Ok(0) => break,
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    // Java: public static int readSingleByte(InputStream in) throws IOException
    pub fn readSingleByte(r#in: &mut dyn Read) -> i32 {
        Self::try_readSingleByte(r#in).unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: readSingleByte 的可恢复版本
    pub fn try_readSingleByte(r#in: &mut dyn Read) -> io::Result<i32> {
        // Java: byte[] buffer = new byte[1];
        let mut buffer = [0u8; 1];
        // Java: int result = in.read(buffer, 0, 1);
        match r#in.read(&mut buffer) {
            Ok(0) => Ok(-1),
            Ok(_) => Ok((buffer[0] as i32) & 255),
            Err(e) => Err(e),
        }
    }

    // Java: public static long skipByReading(InputStream in, long byteCount) throws IOException
    pub fn skipByReading(r#in: &mut dyn Read, byteCount: i64) -> i64 {
        Self::try_skipByReading(r#in, byteCount).unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: skipByReading 的可恢复版本
    pub fn try_skipByReading(r#in: &mut dyn Read, byteCount: i64) -> io::Result<i64> {
        // Java: byte[] buffer = (byte[])skipBuffer.getAndSet(null); if (buffer == null) buffer = new byte[4096];
        // Rust: 每次分配即可（无跨线程复用缓冲的需求）
        let mut buffer = [0u8; 4096];
        // Java: long skipped = 0L;
        let mut skipped: i64 = 0;
        // Java: while(skipped < byteCount)
        while skipped < byteCount {
            // Java: int toRead = (int)Math.min(byteCount - skipped, (long)buffer.length);
            let to_read = std::cmp::min((byteCount - skipped) as usize, buffer.len());
            // Java: int read = in.read(buffer, 0, toRead);
            match r#in.read(&mut buffer[..to_read]) {
                // Java: if (read == -1) break;  → Rust: Ok(0) == EOF
                Ok(0) => break,
                Err(e) => return Err(e),
                Ok(n) => {
                    // Java: skipped += (long)read;
                    skipped += n as i64;
                    if n < to_read {
                        break;
                    }
                }
            }
        }
        // Java: return skipped;
        Ok(skipped)
    }
}
