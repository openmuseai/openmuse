#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Java `cn.wps.moffice.kfs.zip.Deflater extends java.util.zip.Deflater` 的 Rust 侧。
//!
//! 平台映射：`java.util.zip.Deflater` 是 zlib 的 native 包装，Rust 侧用 `flate2::Compress`
//! 提供同一 zlib 状态机；`Deflater(level, nowrap)` 的 `nowrap` 对应
//! `Compress::new(level, zlib_header = !nowrap)`（ZIP 用 `nowrap=true` = raw deflate）。
//! `deflate(byte[],int,int,int)` 的 flush 参数就是 zlib 的 flush 编码
//! （0=NO_FLUSH, 1=PARTIAL, 2=SYNC, 3=FULL, 4=FINISH），与 `flate2::FlushCompress` 一一对应。
//!
//! 注意：`ZipOutputStream` 的写入路径尚未接上这个引擎（`write_buf` 仍是直通实现），
//! 这里先把引擎本身做对，写入侧的接线属于后续 Step。

use flate2::{Compress, Compression, FlushCompress, Status};

pub trait DeflateDataCallBack: Send {
    fn callBack(&self, b: &[u8], off: i32, len: i32);
}

// Java: public class Deflater extends java.util.zip.Deflater
pub struct Deflater {
    // Java field:    ZipOutputStream.DeflateDataCallBack deflateDataCallBack = null;
    pub deflateDataCallBack: Option<Box<dyn DeflateDataCallBack>>,
    // Java field:    private boolean isDeflateByFlush = false;
    isDeflateByFlush: bool,
    level: i32,
    // Java: java.util.zip.Deflater 的 native 状态（zsRef）→ flate2 压缩器
    comp: Option<Compress>,
    // Java: Deflater.setInput(byte[], int, int) 之后待压缩的输入（Rust 拷贝持有）
    input_buf: Vec<u8>,
    input_pos: usize,
    // Java: 是否已调用 finish()（后续 deflate 用 Z_FINISH）
    finish_requested: bool,
    // Java: finished() —— deflate 返回 Z_STREAM_END 后置位
    is_finished: bool,
    // Java: Deflater(int level, boolean nowrap) 的构造参数，reset() 需要复用
    nowrap: bool,
}

// Java: public class Deflater extends java.util.zip.Deflater
impl Deflater {
    // Java: public Deflater(int level, boolean nowrap)
    pub fn new(level: i32, nowrap: bool) -> Deflater {
        let comp = Compress::new(Self::toCompression(level), !nowrap);
        Self {
            deflateDataCallBack: None,
            isDeflateByFlush: false,
            level,
            comp: Some(comp),
            input_buf: Vec::new(),
            input_pos: 0,
            finish_requested: false,
            is_finished: false,
            nowrap,
        }
    }

    // Java: public int deflate(byte[] b, int off, int len, int flush)
    pub fn deflate(&mut self, b: &mut [u8], off: i32, len: i32, flush: i32) -> i32 {
        // Java: int deflate = super.deflate(b, off, len, flush);
        let deflate: i32 = self.deflateInternal(b, off as usize, len as usize, flush);
        // Java: if (this.deflateDataCallBack != null)
        if self.deflateDataCallBack.is_some() {
            // Java: this.deflateDataCallBack.callBack(b, off, deflate);
            if let Some(ref cb) = self.deflateDataCallBack {
                cb.callBack(b, off, deflate);
            }
            // Java: this.isDeflateByFlush = true;
            self.isDeflateByFlush = true;
        }
        // Java: return deflate;
        deflate
    }

    // Java: public int deflate(byte[] b, int off, int len)
    pub fn deflate__bytearr_int_int(&mut self, b: &mut [u8], off: i32, len: i32) -> i32 {
        // Java: this.isDeflateByFlush = false;
        self.isDeflateByFlush = false;
        // Java: int deflate = super.deflate(b, off, len);
        let deflate: i32 = self.deflateInternal(b, off as usize, len as usize, 0);
        // Java: if (this.deflateDataCallBack != null && !this.isDeflateByFlush)
        if self.deflateDataCallBack.is_some() && !self.isDeflateByFlush {
            if let Some(ref cb) = self.deflateDataCallBack {
                // Java: this.deflateDataCallBack.callBack(b, off, deflate);
                cb.callBack(b, off, deflate);
            }
        }
        // Java: return deflate;
        deflate
    }

    // Java: public void setDeflateDataCallBack(ZipOutputStream.DeflateDataCallBack deflateDataCallBack)
    pub fn setDeflateDataCallBack(&mut self, deflateDataCallBack: Option<Box<dyn DeflateDataCallBack>>) {
        // Java: this.deflateDataCallBack = deflateDataCallBack;
        self.deflateDataCallBack = deflateDataCallBack;
    }

    // Java: public void setLevel(int level)
    pub fn setLevel(&mut self, level: i32) {
        // Java: if (level < -1 || level > 9) throw new IllegalArgumentException("Invalid compression level: " + level);
        if level < -1 || level > 9 {
            panic!("IllegalArgumentException: Invalid compression level: {}", level);
        }
        self.level = level;
        // Java: super.setLevel(level);  → Rust: 重建压缩器以应用新 level
        self.comp = Some(Compress::new(Self::toCompression(level), !self.nowrap));
    }

    // Java: public void setInput(byte[] b, int off, int len)
    pub fn setInput(&mut self, buf: &[u8], off: i32, len: i32) {
        let off = off as usize;
        let len = len as usize;
        // Java: 直接替换输入缓冲（调用方保证 needsInput() 为真）
        self.input_buf.clear();
        self.input_buf.extend_from_slice(&buf[off..off + len]);
        self.input_pos = 0;
    }

    // Java: public boolean needsInput()
    pub fn needsInput(&self) -> bool {
        self.input_pos >= self.input_buf.len()
    }

    // Java: public void finish()
    pub fn finish(&mut self) {
        // Java: 置位后，后续 deflate 使用 Z_FINISH
        self.finish_requested = true;
    }

    // Java: public boolean finished()
    pub fn finished(&self) -> bool {
        self.is_finished
    }

    // Java: public int getTotalIn()
    pub fn getTotalIn(&self) -> i32 {
        self.comp.as_ref().map(|c| c.total_in() as i32).unwrap_or(0)
    }

    // Java: public int getTotalOut()
    pub fn getTotalOut(&self) -> i32 {
        self.comp.as_ref().map(|c| c.total_out() as i32).unwrap_or(0)
    }

    // Java: public void reset()
    pub fn reset(&mut self) {
        // Java: super.reset();  → Rust: 重建压缩器（等价于 zlib deflateReset）
        self.comp = Some(Compress::new(Self::toCompression(self.level), !self.nowrap));
        self.input_buf.clear();
        self.input_pos = 0;
        self.finish_requested = false;
        self.is_finished = false;
        self.isDeflateByFlush = false;
    }

    // Java: public void end()
    pub fn end(&mut self) {
        // Java: super.end();  → Rust: 释放 native 状态
        self.comp = None;
        self.input_buf.clear();
        self.input_pos = 0;
    }

    // Rust-native: Java 的 level(-1..9) → flate2 的 Compression
    fn toCompression(level: i32) -> Compression {
        if level < 0 {
            // Java: -1 == DEFAULT_COMPRESSION
            Compression::default()
        } else {
            Compression::new(level as u32)
        }
    }

    // Rust-native: super.deflate(b, off, len, flush) 的等价实现
    fn deflateInternal(&mut self, b: &mut [u8], off: usize, len: usize, flush: i32) -> i32 {
        // Java: len == 0 时返回 0
        if len == 0 {
            return 0;
        }
        if self.comp.is_none() {
            // Java: end() 之后再调用会 NPE（zsRef == 0）
            panic!("NullPointerException: Deflater has been ended");
        }
        let mut flush = Self::toFlush(flush);
        // Java: finish() 之后的无 flush 调用按 Z_FINISH 处理
        if self.finish_requested && flush == FlushCompress::None {
            flush = FlushCompress::Finish;
        }

        let input_start = self.input_pos;
        let result = {
            let comp = self.comp.as_mut().expect("Deflater has been ended");
            let before_in = comp.total_in();
            let before_out = comp.total_out();
            // Rust: 借用拆分——comp 借用自身字段，src 借用 input_buf 字段，互不冲突
            let src = &self.input_buf[input_start..];
            match comp.compress(src, &mut b[off..off + len], flush) {
                Ok(status) => Ok((
                    status,
                    (comp.total_in() - before_in) as usize,
                    (comp.total_out() - before_out) as usize,
                )),
                // Java: 压缩错误在 JDK 里表现为 IllegalStateException/内部错误，这里按 RuntimeException panic
                Err(e) => Err(e),
            }
        };

        match result {
            Ok((status, consumed, produced)) => {
                self.input_pos += consumed;
                if status == Status::StreamEnd {
                    self.is_finished = true;
                }
                produced as i32
            }
            Err(_) => 0,
        }
    }

    // Rust-native: Java/zlib 的 flush 编码 → flate2::FlushCompress
    fn toFlush(flush: i32) -> FlushCompress {
        match flush {
            1 => FlushCompress::Partial,
            2 => FlushCompress::Sync,
            3 => FlushCompress::Full,
            4 => FlushCompress::Finish,
            _ => FlushCompress::None,
        }
    }
}
