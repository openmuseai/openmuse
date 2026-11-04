#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Rust-native counterpart of `java.util.zip.Inflater`.
//! (Rust-native: JDK 类，本仓库没有对应的 Java 源文件；由 `InflaterInputStream.inf` / `ZipInputStream.inf` 使用。)
//!
//! Java: `new Inflater(true)` —— `nowrap=true`，即 **raw deflate（不带 zlib 头）**，
//! ZIP 条目的压缩数据流就是这个格式；`false` 才带 zlib 头（zlib/gzip 场景）。
//!
//! 平台映射：JDK 的 Inflater 是 zlib 的 native 包装，Rust 侧由 `flate2`(miniz_oxide/zlib)
//! 提供同一 zlib 状态机，逐字节结果一致。`flate2::Decompress::new(zlib_header)` 的
//! 参数与 Java 的 `nowrap` 正好相反（`zlib_header == !nowrap`）。
//!
//! Java 的 Inflater 方法语义与 Rust 对应关系：
//!   `setInput/setDictionary/inflate/needsInput/needsDictionary/finished/getTotalIn/getTotalOut/reset/end`
//!   → `setInput` 会**拷贝**输入（Java 保存调用方数组引用；Rust 拷贝避免自引用结构，
//!     调用方 (`InflaterInputStream`) 每次只喂 512 字节，代价可忽略）。

use flate2::{Decompress, FlushDecompress, Status};
use std::io;

use crate::kfs::zip::ZipErrors;

// Rust-native: java.util.zip.Inflater
pub struct Inflater {
    // Java field:    private final long zsRef;  → flate2 的 raw 解压状态机
    decomp: Option<Decompress>,
    // Java: Inflater.setInput(byte[], int, int) 之后待消费的输入（Rust 拷贝持有）
    input: Vec<u8>,
    // Java: 下一次 inflate 从 input 的哪个位置开始（= 已被 zlib 消费的字节数）
    input_pos: usize,
    // Java: Inflater.finished() —— 收到 Z_STREAM_END 后置位
    finished: bool,
    // Java: Inflater(boolean nowrap) 的构造参数，reset() 需要复用
    nowrap: bool,
}

impl Default for Inflater {
    // Java: public Inflater()  —— 默认 nowrap == false（带 zlib 头）
    fn default() -> Self {
        Inflater::new(false)
    }
}

impl Inflater {
    // Java: public Inflater(boolean nowrap)
    pub fn new(nowrap: bool) -> Inflater {
        Self {
            // Rust: flate2 的参数是 zlib_header，与 Java 的 nowrap 相反
            decomp: Some(Decompress::new(!nowrap)),
            input: Vec::new(),
            input_pos: 0,
            finished: false,
            nowrap,
        }
    }

    // Java: public void setInput(byte[] b, int off, int len)
    pub fn setInput(&mut self, b: &[u8], off: i32, len: i32) {
        let off = off as usize;
        let len = len as usize;
        // Java: 直接替换输入缓冲（调用方保证 needsInput() 为真），未消费的旧数据被丢弃
        self.input.clear();
        self.input.extend_from_slice(&b[off..off + len]);
        self.input_pos = 0;
    }

    // Java: public boolean needsInput()
    pub fn needsInput(&self) -> bool {
        self.input_pos >= self.input.len()
    }

    // Java: public boolean needsDictionary()
    pub fn needsDictionary(&self) -> bool {
        // ZIP 条目的 deflate 流不带预设字典
        false
    }

    // Java: public int inflate(byte[] b, int off, int len) throws DataFormatException
    pub fn inflate(&mut self, b: &mut [u8], off: usize, len: usize) -> io::Result<i32> {
        // Java: inflate 在输出长度为 0 时返回 0
        if len == 0 {
            return Ok(0);
        }
        if self.decomp.is_none() {
            // Java: end() 之后再调用是未定义行为；这里按 IOException 处理
            return Err(ZipErrors::ioException("Inflater has been ended"));
        }

        let input_start = self.input_pos;
        let result = {
            let decomp = self.decomp.as_mut().expect("Inflater has been ended");
            let before_in = decomp.total_in();
            let before_out = decomp.total_out();
            // Rust: 借用拆分——decomp 借用自身字段，src 借用 input 字段，互不冲突
            let src = &self.input[input_start..];
            match decomp.decompress(src, &mut b[off..off + len], FlushDecompress::None) {
                // Java: DataFormatException → 上游 ZipInputStream 转成 ZipException(e.getMessage())
                Ok(status) => Ok((
                    status,
                    (decomp.total_in() - before_in) as usize,
                    (decomp.total_out() - before_out) as usize,
                )),
                Err(e) => Err(ZipErrors::zipException(e.to_string())),
            }
        };

        match result {
            Ok((status, consumed, produced)) => {
                self.input_pos += consumed;
                // Java: Z_STREAM_END → finished() == true
                if status == Status::StreamEnd {
                    self.finished = true;
                }
                Ok(produced as i32)
            }
            Err(e) => Err(e),
        }
    }

    // Java: public boolean finished()
    pub fn finished(&self) -> bool {
        self.finished
    }

    // Java: public int getTotalIn()
    pub fn getTotalIn(&self) -> i32 {
        // Java 的 getTotalIn 返回 int（超过 2GB 的条目本来就会溢出，保持同样的取整语义）
        self.decomp.as_ref().map(|d| d.total_in() as i32).unwrap_or(0)
    }

    // Java: public int getTotalOut()
    pub fn getTotalOut(&self) -> i32 {
        self.decomp.as_ref().map(|d| d.total_out() as i32).unwrap_or(0)
    }

    // Java: public void reset()
    pub fn reset(&mut self) {
        if let Some(decomp) = self.decomp.as_mut() {
            decomp.reset(!self.nowrap);
        }
        self.input.clear();
        self.input_pos = 0;
        self.finished = false;
    }

    // Java: public void end()
    pub fn end(&mut self) {
        // Rust: 释放 native 状态（Java 的 zsRef 引用清零）
        self.decomp = None;
        self.input.clear();
        self.input_pos = 0;
    }
}
