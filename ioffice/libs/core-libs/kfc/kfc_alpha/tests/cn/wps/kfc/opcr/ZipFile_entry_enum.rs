// `s0_02_function_zip_entry_enum` —— 测试镜像（⇄ `src/cn/wps/kfc/opcr/ZipFile.rs` 的枚举面）。
// 覆盖矩阵与 PASS/FAIL 判据见 `features/s0-01-zip-source/functions/s0_02_function_zip_entry_enum/FUNCTION.md` §3。
// 运行：cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test s0_02_function_zip_entry_enum
//
// Java 锚点：`kfs/zip/ZipFile.java:105-119`（`entries()` 的 Enumeration 面）、`:198-255`（`readCentralDir` 逐 entry 循环）、
// `kfs/zip/ZipEntry.java:59-108`（字段访问器族）。

use core_libs_kfc::kfc::opcr::ZipFile::{ZipEntry, ZipFile, DEFLATED, STORED};
use std::io::{Cursor, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const C1: &str = "samples/ppt-render/flutter/assets/Sample_ppt.pptx";
const C2: &str = "samples/ppt-render/flutter/assets/hello.pptx";

fn repoRoot() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn corpus(rel: &str) -> String {
    repoRoot().join(rel).to_string_lossy().to_string()
}

fn tempDir() -> PathBuf {
    let d = std::env::temp_dir().join(format!("s0_02_entry_enum_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&d);
    d
}

fn writeTemp(name: &str, bytes: &[u8]) -> String {
    let p = tempDir().join(name);
    std::fs::write(&p, bytes).expect("write temp fixture");
    p.to_string_lossy().to_string()
}

/// 合成 zip：`(name, bytes, stored)`；`name` 以 `/` 结尾 ⇒ 目录项（`add_directory`，Java 侧 `isDirectory()` 判据同源）。
fn zipWith(items: &[(&str, &[u8], bool)]) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut w = ZipWriter::new(Cursor::new(&mut buf));
        for (name, data, stored) in items {
            let opt = SimpleFileOptions::default().compression_method(if *stored {
                CompressionMethod::Stored
            } else {
                CompressionMethod::Deflated
            });
            if name.ends_with('/') {
                w.add_directory(*name, opt).expect("add_directory");
            } else {
                w.start_file(*name, opt).expect("start_file");
                w.write_all(data).expect("write entry");
            }
        }
        w.finish().expect("finish zip");
    }
    buf
}

/// 只 EOCD 的「空 zip」（22 字节）；`s0_01` 起与 Java `ZipFile.java:204-205` 同口径拒绝。
fn eocdOnlyZip() -> Vec<u8> {
    let mut v = vec![0x50, 0x4b, 0x05, 0x06];
    v.extend_from_slice(&[0u8; 18]);
    v
}

/// dump 口径的行：`(index, name, is_dir, method, uncompressed_size, compressed_size, local_header_offset)`。
fn rows(zf: &ZipFile) -> Vec<(usize, String, bool, i32, i64, i64, i64)> {
    zf.entries()
        .iter()
        .enumerate()
        .map(|(i, e)| {
            (
                i,
                e.getName(),
                e.isDirectory(),
                e.getMethod(),
                e.getSize(),
                e.getCompressedSize(),
                e.getLocalHeaderRelOffset(),
            )
        })
        .collect()
}

fn names(zf: &ZipFile) -> Vec<String> {
    zf.entries().iter().map(|e| e.getName()).collect()
}

// ── T01 C2 表的形状（COV-1） ────────────────────────────────────────────────
#[test]
fn t01_c2_entry_table_shape() {
    let zf = ZipFile::from_path(&corpus(C2)).expect("C2 应打开成功");
    let table = rows(&zf);
    assert!(!table.is_empty(), "C2 的表不得为空");
    // index 必须严格 0..n-1 无洞
    for (i, row) in table.iter().enumerate() {
        assert_eq!(row.0, i, "index 必须严格递增无洞");
        assert!(!row.1.is_empty(), "name 不得为空");
    }
    let mut sorted = names(&zf);
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), table.len(), "name 不得重复");
}

// ── T02 顺序 = 写入序（非字典序、非哈希序）（COV-2） ────────────────────────
#[test]
fn t02_insertion_order_preserved() {
    let bytes = zipWith(&[
        ("zz.txt", b"z", false),
        ("aa.txt", b"a", false),
        ("mm.txt", b"m", false),
    ]);
    let p = writeTemp("order.zip", &bytes);
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    // 硬断言：写入序与字典序**相反** ⇒ 顺序错会立刻红
    assert_eq!(names(&zf), vec!["zz.txt", "aa.txt", "mm.txt"]);
}

// ── T03 目录项（COV-3） ─────────────────────────────────────────────────────
#[test]
fn t03_directory_entry_flagged() {
    let bytes = zipWith(&[
        ("ppt/slides/", b"", false),
        ("ppt/slides/s1.xml", b"<x/>", false),
    ]);
    let p = writeTemp("dir.zip", &bytes);
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    assert_eq!(names(&zf), vec!["ppt/slides/", "ppt/slides/s1.xml"]);
    let table = rows(&zf);
    assert!(table[0].2, "尾 `/` 的项必须 isDirectory == true");
    assert!(!table[1].2, "普通文件必须 isDirectory == false");
}

// ── T04 压缩方法码（COV-4）：Java STORED=0 / DEFLATED=8 ─────────────────────
#[test]
fn t04_method_stored_and_deflated() {
    let bytes = zipWith(&[("s.txt", b"abc", true), ("d.txt", b"aaaa", false)]);
    let p = writeTemp("method.zip", &bytes);
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let table = rows(&zf);
    assert_eq!(table[0].1, "s.txt");
    assert_eq!(table[0].3, STORED, "Stored 必须映射为 Java STORED(0)");
    assert_eq!(table[1].1, "d.txt");
    assert_eq!(table[1].3, DEFLATED, "Deflated 必须映射为 Java DEFLATED(8)");
}

// ── T05 未压缩大小（COV-5）：4 KiB 可压缩文本 ──────────────────────────────
#[test]
fn t05_uncompressed_size_field() {
    let payload = vec![b'a'; 4096];
    let bytes = zipWith(&[("d.txt", &payload, false)]);
    let p = writeTemp("size.zip", &bytes);
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let table = rows(&zf);
    assert_eq!(table[0].4, 4096, "uncompressed_size 必须等于原始字节数");
    assert_ne!(table[0].5, 4096, "Deflated 夹具上 compressed_size 必不相等");
}

// ── T06 name 逐字原样（COV-6）：方括号 / 前导 `/` / 反斜杠 / 大小写 ────────
#[test]
fn t06_names_verbatim() {
    let bytes = zipWith(&[
        ("[Content_Types].xml", b"a", false),
        ("/abs.xml", b"b", false),
        ("ppt\\win.xml", b"c", false),
        ("MixedCase.XML", b"d", false),
    ]);
    let p = writeTemp("names.zip", &bytes);
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    assert_eq!(
        names(&zf),
        vec!["[Content_Types].xml", "/abs.xml", "ppt\\win.xml", "MixedCase.XML"],
        "name 必须逐字原样（不得归一/去方括号/去前导斜杠/改大小写）"
    );
}

// ── T07 两后端同表（COV-7） ────────────────────────────────────────────────
#[test]
fn t07_backends_identical_tables() {
    let bytes = std::fs::read(corpus(C2)).expect("读 C2");
    let viaPath = ZipFile::from_path(&corpus(C2)).expect("path 后端");
    let mut cur = Cursor::new(bytes);
    let viaReader = ZipFile::from_reader(&mut cur).expect("reader 后端");
    assert_eq!(rows(&viaPath), rows(&viaReader), "两后端必须逐字段相同");
}

// ── T08 0 entry 的包在打开期即被拒（COV-8，★按现状更正） ───────────────────
#[test]
fn t08_zero_entry_package_unreachable() {
    // 合成「0 entry 的合法 zip」= 只有 EOCD ⇒ `s0_01` 起与 Java `:204-205` 同口径**打开即拒**，
    // 因此「空表」在 S0 轨不可达（Java 同样到不了 `entries()`）。登记：现状空表只可能来自关闭/坏句柄。
    let p = writeTemp("zero.zip", &eocdOnlyZip());
    assert!(
        ZipFile::from_path(&p).is_err(),
        "仅 EOCD 的包必须在打开期被拒 ⇒ 0 行表不可达"
    );
}

// ── T09 C2 表长（COV-9，★实测 3 行，非 1 行） ──────────────────────────────
#[test]
fn t09_c2_entry_count_measured() {
    let zf = ZipFile::from_path(&corpus(C2)).expect("C2 应打开成功");
    // ★本轮实测：C2 `hello.pptx` 中央目录 3 项
    assert_eq!(
        names(&zf),
        vec!["[Content_Types].xml", "ppt/presentation.xml", "ppt/slides/slide1.xml"]
    );
    assert_eq!(zf.size(), 3, "size() 必须等于表长");
}

// ── T10 关闭态返回空表且不 panic（COV-10，登记分歧） ───────────────────────
#[test]
fn t10_closed_handle_returns_empty_table() {
    let mut zf = ZipFile::from_path(&corpus(C2)).expect("C2 应打开成功");
    zf.close();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let t = zf.entries();
        (t.len(), zf.size())
    }));
    let (len, size) = r.expect("关闭态枚举不得 panic");
    assert_eq!(len, 0, "Rust 现状：关闭态返回空表（Java `:106` 抛 IllegalStateException）");
    assert_eq!(size, 0);
    // 关闭态的第二个外部可观测判据：`getEntry` 不再命中（Java 侧同样抛 `IllegalStateException`）
    assert!(zf.getEntry("ppt/presentation.xml").is_none());
}

// ── T11 重复枚举稳定（COV-11） ────────────────────────────────────────────
#[test]
fn t11_repeat_enumeration_stable() {
    let zf = ZipFile::from_path(&corpus(C1)).expect("C1 应打开成功");
    let a = rows(&zf);
    let b = rows(&zf);
    assert_eq!(a, b, "两次枚举必须逐字段相同（哈希随机种子不得进入顺序）");
    assert!(a.len() > 1, "C1 表应多行");
}

// ── T12 字段 provenance 与 `zip` crate 一致（COV-12，★按改造后口径更正） ───
#[test]
fn t12_metadata_provenance_matches_zip_crate() {
    let bytes = std::fs::read(corpus(C1)).expect("读 C1");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("crate 直读");
    let zf = ZipFile::from_path(&corpus(C1)).expect("C1 应打开成功");
    let table = rows(&zf);
    assert!(table.len() > 1);
    for (i, row) in table.iter().enumerate() {
        let f = archive.by_index_raw(i).expect("by_index_raw");
        assert_eq!(row.1, f.name(), "name");
        assert_eq!(row.2, f.is_dir(), "is_dir");
        assert_eq!(
            row.3,
            match f.compression() {
                CompressionMethod::Stored => STORED,
                _ => DEFLATED,
            },
            "method"
        );
        assert_eq!(row.4, f.size() as i64, "uncompressed_size");
        assert_eq!(row.5, f.compressed_size() as i64, "compressed_size");
        assert_eq!(row.6, f.header_start() as i64, "local_header_offset");
    }
}

// ── T13 枚举无落盘副作用（COV-16） ────────────────────────────────────────
#[test]
fn t13_enumeration_has_no_disk_side_effect() {
    let src = corpus(C1);
    let before = std::fs::read(&src).expect("读 C1 前");
    let zf = ZipFile::from_path(&src).expect("C1 应打开成功");
    let first = zf.entries();
    assert!(zf.getEntry("ppt/presentation.xml").is_some(), "getEntry 应命中");
    let after = std::fs::read(&src).expect("读 C1 后");
    assert_eq!(before, after, "枚举前后源文件字节必须完全相同");
    assert_eq!(first.len(), zf.entries().len());
}

// ── T14 `size()` == 表长（Java `:193` `entries.size()`） ──────────────────
#[test]
fn t14_size_method_matches_table_len() {
    for rel in [C1, C2] {
        let zf = ZipFile::from_path(&corpus(rel)).expect("语料应打开成功");
        assert_eq!(zf.size() as usize, zf.entries().len(), "size() 必须等于表长：{rel}");
    }
}

// ── T15 `getEntry` 与表的字段一致（同口径装填） ───────────────────────────
#[test]
fn t15_getEntry_fields_consistent_with_table() {
    let bytes = zipWith(&[
        ("s.txt", b"abc", true),
        ("ppt/slides/s1.xml", b"<x/>", false),
    ]);
    let p = writeTemp("consistency.zip", &bytes);
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    for row in rows(&zf) {
        let e = zf.getEntry(&row.1).expect("表里的 name 必须可再取到");
        assert_eq!(e.getName(), row.1, "name");
        assert_eq!(e.isDirectory(), row.2, "is_dir");
        assert_eq!(e.getMethod(), row.3, "method");
        assert_eq!(e.getSize(), row.4, "uncompressed_size");
        assert_eq!(e.getCompressedSize(), row.5, "compressed_size");
        assert_eq!(e.getLocalHeaderRelOffset(), row.6, "local_header_offset");
    }
}

// ── T16 `ZipEntry::default()` 取 Java 字段默认值（`-1L` / `-1`） ──────────
#[test]
fn t16_default_entry_fields_java_defaults() {
    let e = ZipEntry::default();
    // Java `ZipEntry.java:17-26`：`crc = -1L`、`compressedSize = -1L`、`size = -1L`、
    // `compressionMethod = -1`、`localHeaderRelOffset = -1L`、`dataOffset = -1L`
    assert_eq!(e.getName(), "");
    assert_eq!(e.getSize(), -1);
    assert_eq!(e.getCompressedSize(), -1);
    assert_eq!(e.getMethod(), -1);
    assert_eq!(e.getLocalHeaderRelOffset(), -1);
    assert!(!e.isDirectory(), "空名必须 isDirectory == false（Java 对空名会越界抛，Rust 侧显式安全）");
}
