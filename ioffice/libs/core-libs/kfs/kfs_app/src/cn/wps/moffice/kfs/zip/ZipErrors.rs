#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Rust-native bridge for the Java exception family used by the ZIP port.
//! (Rust-native: 本文件没有对应的 Java 源文件，只为承载 Java 的异常谱系。)
//!
//! Java 侧：
//!   `java.util.zip.ZipException extends java.io.IOException`
//!   `java.io.EOFException      extends java.io.IOException`
//! 三个异常同属 IOException 谱系，Java 调用方统一 `catch (IOException e)` 处理
//! （例如 `cn.wps.io.xwpf.util.ZipInputStreamZipEntrySource` 就是这么做的）。
//!
//! Rust 侧约定（本模块 `try_*` 核心 + Java 签名外壳的两层结构）：
//!   * **数据/环境错误**（损坏输入、提前 EOF、IO 失败）→ `std::io::Error`，
//!     用 `ErrorKind` 承载 Java 的异常种类：
//!       ZipException → `InvalidData`，EOFException → `UnexpectedEof`，其他 IOException → `Other`；
//!   * 核心逻辑写成 `try_xxx() -> std::io::Result<T>`，Rust 原生调用方（`impl Read`、
//!     新写的 Rust 代码）可以真正恢复；
//!   * 与 Java 同签名的方法（`read(..) -> i32`、`closeEntry()` …）保留为薄外壳，
//!     用 `panicLikeJava` 还原“抛出”语义，使 1:1 对齐仍可读、可 diff；
//!   * **编程性错误**（`ArrayIndexOutOfBoundsException`、`IllegalArgumentException`、
//!     `NullPointerException` 之类 RuntimeException）继续直接 panic —— 这些不是调用方应该恢复的东西。
//!
//! 迁移路径：逐个调用点从外壳切到 `try_*`；当某个类型的 Java 签名外壳不再有调用方时删掉它，
//! 该类型就完成了从“移植产物”到“惯用 Rust”的转换。

use std::io;

// Rust-native: java.util.zip.ZipException
pub fn zipException(message: impl Into<String>) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("ZipException: {}", message.into()),
    )
}

// Rust-native: new ZipException(e.getMessage()) —— 把底层错误重新包装成 ZipException
pub fn zipExceptionFrom(error: &io::Error) -> io::Error {
    zipException(error.to_string())
}

// Rust-native: java.io.EOFException
pub fn eofException() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "EOFException")
}

// Rust-native: java.io.IOException
pub fn ioException(message: impl Into<String>) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        format!("IOException: {}", message.into()),
    )
}

// Rust-native: java.io.UTFDataFormatException（extends IOException）
// 用 InvalidInput 与 ZipException 的 InvalidData 区分开。
pub fn utfDataFormatException(message: impl Into<String>) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("UTFDataFormatException: {}", message.into()),
    )
}

// Rust-native: 还原 `catch (IOException e)` 里 e.getClass().getSimpleName() 的判别结果。
// 先看消息前缀（构造时写了异常名），再退回 ErrorKind 的映射。
pub fn javaExceptionName(error: &io::Error) -> &'static str {
    let text = error.to_string();
    for name in [
        "ZipException",
        "EOFException",
        "UTFDataFormatException",
        "IOException",
    ] {
        if text.starts_with(name) {
            return name;
        }
    }
    match error.kind() {
        io::ErrorKind::InvalidData => "ZipException",
        io::ErrorKind::UnexpectedEof => "EOFException",
        _ => "IOException",
    }
}

// Rust-native: Java 签名外壳的抛出动作。`throws IOException` → panic（消息带 Java 异常名前缀）。
pub fn panicLikeJava(error: &io::Error) -> ! {
    panic!("{}", javaExceptionMessage(error))
}

// Rust-native: panic 消息文本，形如 "ZipException: CRC mismatch"
pub fn javaExceptionMessage(error: &io::Error) -> String {
    let name = javaExceptionName(error);
    let text = error.to_string();
    if text.starts_with(name) {
        text
    } else {
        format!("{}: {}", name, text)
    }
}
