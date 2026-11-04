#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Java `cn.wps.moffice.kfs.zip.ZipConstants`（纯常量接口）的 Rust 侧。
//!
//! Java 里这些常量靠 `implements ZipConstants` 进入 ZipEntry/ZipFile/ZipInputStream/
//! ZipOutputStream 的命名空间；Rust 没有“接口常量继承”，因此落成模块级 `pub const`，
//! 引用方式是 `ZipConstants::LOCSIG`。
//!
//! 类型严格照抄 Java（`long` → `i64`，`int` → `i32`），数值不做任何“顺手修正”，
//! 这样和 Java 源逐行对照时不会有歧义。
//!
//! 注意：kfs 的 zip 实现内部其实是直接用字面量写的（Java 侧也一样，只有
//! `ZipInputStream` 的报错文本里出现过 EXTSIG 的名字），所以这里提供常量属于
//! “把 Java 的公共契约补齐”，并没有要求调用点改写。

// Java: public interface ZipConstants
pub trait ZipConstants {}

// Java: long LOCSIG = 67324752L;
pub const LOCSIG: i64 = 67324752;
// Java: long EXTSIG = 134695760L;
pub const EXTSIG: i64 = 134695760;
// Java: long CENSIG = 33639248L;
pub const CENSIG: i64 = 33639248;
// Java: long ENDSIG = 101010256L;
pub const ENDSIG: i64 = 101010256;
// Java: int LOCHDR = 30;
pub const LOCHDR: i32 = 30;
// Java: int EXTHDR = 16;
pub const EXTHDR: i32 = 16;
// Java: int CENHDR = 46;
pub const CENHDR: i32 = 46;
// Java: int ENDHDR = 22;
pub const ENDHDR: i32 = 22;
// Java: int LOCVER = 4;
pub const LOCVER: i32 = 4;
// Java: int LOCFLG = 6;
pub const LOCFLG: i32 = 6;
// Java: int LOCHOW = 8;
pub const LOCHOW: i32 = 8;
// Java: int LOCTIM = 10;
pub const LOCTIM: i32 = 10;
// Java: int LOCCRC = 14;
pub const LOCCRC: i32 = 14;
// Java: int LOCSIZ = 18;
pub const LOCSIZ: i32 = 18;
// Java: int LOCLEN = 22;
pub const LOCLEN: i32 = 22;
// Java: int LOCNAM = 26;
pub const LOCNAM: i32 = 26;
// Java: int LOCEXT = 28;
pub const LOCEXT: i32 = 28;
// Java: int EXTCRC = 4;
pub const EXTCRC: i32 = 4;
// Java: int EXTSIZ = 8;
pub const EXTSIZ: i32 = 8;
// Java: int EXTLEN = 12;
pub const EXTLEN: i32 = 12;
// Java: int CENVEM = 4;
pub const CENVEM: i32 = 4;
// Java: int CENVER = 6;
pub const CENVER: i32 = 6;
// Java: int CENFLG = 8;
pub const CENFLG: i32 = 8;
// Java: int CENHOW = 10;
pub const CENHOW: i32 = 10;
// Java: int CENTIM = 12;
pub const CENTIM: i32 = 12;
// Java: int CENCRC = 16;
pub const CENCRC: i32 = 16;
// Java: int CENSIZ = 20;
pub const CENSIZ: i32 = 20;
// Java: int CENLEN = 24;
pub const CENLEN: i32 = 24;
// Java: int CENNAM = 28;
pub const CENNAM: i32 = 28;
// Java: int CENEXT = 30;
pub const CENEXT: i32 = 30;
// Java: int CENCOM = 32;
pub const CENCOM: i32 = 32;
// Java: int CENDSK = 34;
pub const CENDSK: i32 = 34;
// Java: int CENATT = 36;
pub const CENATT: i32 = 36;
// Java: int CENATX = 38;
pub const CENATX: i32 = 38;
// Java: int CENOFF = 42;
pub const CENOFF: i32 = 42;
// Java: int ENDSUB = 8;
pub const ENDSUB: i32 = 8;
// Java: int ENDTOT = 10;
pub const ENDTOT: i32 = 10;
// Java: int ENDSIZ = 12;
pub const ENDSIZ: i32 = 12;
// Java: int ENDOFF = 16;
pub const ENDOFF: i32 = 16;
// Java: int ENDCOM = 20;
pub const ENDCOM: i32 = 20;
