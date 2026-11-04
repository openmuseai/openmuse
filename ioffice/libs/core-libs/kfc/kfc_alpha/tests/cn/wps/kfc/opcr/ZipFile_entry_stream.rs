// `s0_03_function_zip_entry_stream` —— 测试镜像（⇄ `src/cn/wps/kfc/opcr/ZipFile.rs` 的取流面）。
// 覆盖矩阵与 PASS/FAIL 判据见 `features/s0-01-zip-source/functions/s0_03_function_zip_entry_stream/FUNCTION.md` §3。
// 运行：cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test s0_03_function_zip_entry_stream
//
// Java 锚点：`kfs/zip/ZipFile.java:140-151`（4 个入口）、`:153-187`（static 取流：本地文件头解析 + `:173-175` STORED 分支
// + `:178` 两态分支）、`:362-403`（`ZipInflaterInputStream`，含 `:381-383` 尺寸校验）。

use core_libs_kfc::kfc::opcr::ZipFile::{ZipFile, STORED};
use std::io::{Cursor, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const C1: &str = "samples/ppt-render/flutter/assets/Sample_ppt.pptx";
const C2: &str = "samples/ppt-render/flutter/assets/hello.pptx";
const XML_ENTRY: &str = "ppt/presentation.xml";

fn repoRoot() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn corpus(rel: &str) -> String {
    repoRoot().join(rel).to_string_lossy().to_string()
}

fn tempDir() -> PathBuf {
    let d = std::env::temp_dir().join(format!("s0_03_entry_stream_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&d);
    d
}

fn writeTemp(name: &str, bytes: &[u8]) -> String {
    let p = tempDir().join(name);
    std::fs::write(&p, bytes).expect("write temp fixture");
    p.to_string_lossy().to_string()
}

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
            w.start_file(*name, opt).expect("start_file");
            w.write_all(data).expect("write entry");
        }
        w.finish().expect("finish zip");
    }
    buf
}

/// 可压缩文本（Deflated 后长度必小于原始长度）。
fn compressible(seed: u8, len: usize) -> Vec<u8> {
    (0..len).map(|i| b'a' + ((i as u8 + seed) % 26)).collect()
}

/// 把 C2 第 `idx` 个中央目录项的 **uncompressed size**（CDE 偏移 +24）改写成 `declared`。
fn patchCentralUncompressedSize(src: &[u8], idx: usize, declared: u32) -> Vec<u8> {
    let mut out = src.to_vec();
    let mut seen = 0usize;
    let mut at = 0usize;
    while at + 46 <= out.len() {
        if out[at..at + 4] == [0x50, 0x4b, 0x01, 0x02] {
            if seen == idx {
                out[at + 24..at + 28].copy_from_slice(&declared.to_le_bytes());
                return out;
            }
            seen += 1;
        }
        at += 1;
    }
    panic!("中央目录项 {idx} 未找到");
}

// ── T01 缺省态：Deflated entry ⇒ 解压字节（COV-1） ────────────────────────
#[test]
fn t01_deflated_entry_inflated_by_default() {
    let zf = ZipFile::from_path(&corpus(C2)).expect("C2 应打开成功");
    let entry = zf.getEntry(XML_ENTRY).expect("entry 应存在");
    let bytes = zf.getInputStream(&entry, false).expect("缺省态必须成功");
    assert_eq!(bytes.len() as i64, entry.getSize(), "解压字节数 == uncompressed_size");
    assert_ne!(entry.getMethod(), STORED, "C2 该 entry 应为 Deflated");
    assert!(
        String::from_utf8_lossy(&bytes).contains("<p:presentation"),
        "解压内容必须是 XML 文本（实测前 40 字节：{:?}）",
        String::from_utf8_lossy(&bytes[..40.min(bytes.len())])
    );
}

// ── T02 原样态：`true` ⇒ **压缩态原样字节**（COV-2，★本轮由「预期红」转绿） ─
#[test]
fn t02_origin_input_returns_compressed_bytes() {
    let src = std::fs::read(corpus(C2)).expect("读 C2");
    let mut archive = zip::ZipArchive::new(Cursor::new(&src)).expect("crate 直读");
    let zf = ZipFile::from_path(&corpus(C2)).expect("C2 应打开成功");
    let entry = zf.getEntry(XML_ENTRY).expect("entry 应存在");
    let raw = zf.getInputStream(&entry, true).expect("原样态必须成功");
    let idx = archive.index_for_name(XML_ENTRY).expect("按名定位");
    let f = archive.by_index_raw(idx).expect("by_index_raw");
    // Java `:182`（`entry.getMode() == MODE_RAW || isOriginInput`）⇒ RAFStream：`endOffset = offset + compressedSize`
    assert_eq!(raw.len() as i64, entry.getCompressedSize(), "原样态长度 == compressed_size");
    assert_eq!(raw.len() as i64, f.compressed_size() as i64, "与 crate 口径一致");
    // provenance：必须是源文件里该 entry 的**原始切片**（deflate 流，非明文）
    let start = f.data_start() as usize;
    let end = start + f.compressed_size() as usize;
    assert_eq!(raw, src[start..end].to_vec(), "原样态必须等于文件内 data_start..+compressed_size 的切片");
    let inflated = zf.getInputStream(&entry, false).expect("缺省态");
    assert_ne!(raw, inflated, "两态必须不同（Deflated 夹具上）");
    assert!(raw.len() < inflated.len(), "deflate 后应更短（实测 {} < {}）", raw.len(), inflated.len());
}

// ── T03 STORED：两态相同（Java `:173-175`）（COV-3） ─────────────────────
#[test]
fn t03_stored_entry_same_for_both_flags() {
    let payload = b"stored-entry-bytes".to_vec();
    let p = writeTemp("stored.zip", &zipWith(&[("s.txt", &payload, true)]));
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let entry = zf.getEntry("s.txt").expect("entry 应存在");
    assert_eq!(entry.getMethod(), STORED);
    let a = zf.getInputStream(&entry, false).expect("false");
    let b = zf.getInputStream(&entry, true).expect("true");
    assert_eq!(a, payload, "STORED 的缺省态 = 原始字节");
    assert_eq!(a, b, "STORED 两态必须完全相同");
}

// ── T04 0 字节 entry ⇒ `Some(vec![])`（COV-5） ───────────────────────────
#[test]
fn t04_zero_length_entry() {
    let p = writeTemp("empty.zip", &zipWith(&[("empty.txt", b"", true)]));
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let entry = zf.getEntry("empty.txt").expect("entry 应存在");
    assert_eq!(zf.getInputStream(&entry, false), Some(Vec::new()));
    assert_eq!(zf.getInputStream(&entry, true), Some(Vec::new()));
}

// ── T05 名字不在本包 ⇒ `None`；名字在本包 ⇒ **重解析为本包 entry**（COV-7） ────
#[test]
fn t05_name_resolution_semantics() {
    // (a) 名字不在本包（合成包独有的名字）⇒ 两态都 `None`
    //     Java `:145`：`entry = this.getEntry(entry.getName()); return entry == null ? null : ...`
    let synth = writeTemp(
        "only-synthetic.zip",
        &zipWith(&[("only-in-synthetic.txt", b"payload", false)]),
    );
    let zfSynth = ZipFile::from_path(&synth).expect("合成包应打开成功");
    let foreign = zfSynth.getEntry("only-in-synthetic.txt").expect("合成 entry");
    let zf1 = ZipFile::from_path(&corpus(C1)).expect("C1");
    assert_eq!(zf1.getInputStream(&foreign, false), None, "名字不在本包 ⇒ 缺省态 None");
    assert_eq!(zf1.getInputStream(&foreign, true), None, "名字不在本包 ⇒ 原样态 None");

    // (b) 名字**在**本包（C1 也有 `ppt/presentation.xml`）⇒ 必须重解析为**本包**的 entry：
    //     返回的是 C1 自己的字节流，而不是 C2 对象里那 160 字节的尺寸口径。
    let zf2 = ZipFile::from_path(&corpus(C2)).expect("C2");
    let c2entry = zf2.getEntry(XML_ENTRY).expect("C2 的 entry");
    let local = zf1.getEntry(XML_ENTRY).expect("C1 同名 entry");
    let bytes = zf1.getInputStream(&c2entry, false).expect("同名 ⇒ 必须成功");
    assert_eq!(
        bytes.len() as i64,
        local.getSize(),
        "长度必须按**本包** entry 的 uncompressed_size 校验（重解析语义）"
    );
    assert_eq!(bytes, zf1.getInputStream(&local, false).expect("用本包 entry 直取"), "两者必须同字节");
}

// ── T06 关闭句柄 ⇒ `None`（COV-11） ─────────────────────────────────────
#[test]
fn t06_closed_handle_returns_none() {
    let mut zf = ZipFile::from_path(&corpus(C2)).expect("C2");
    let entry = zf.getEntry(XML_ENTRY).expect("entry 应存在");
    zf.close();
    assert_eq!(zf.getInputStream(&entry, false), None);
    assert_eq!(zf.getInputStream(&entry, true), None);
}

// ── T07 真语料全量往返：解压字节数 == 声明尺寸；原样字节数 == 压缩尺寸（COV-6） ─
#[test]
fn t07_all_c1_entries_round_trip_sizes() {
    let zf = ZipFile::from_path(&corpus(C1)).expect("C1 应打开成功");
    let table = zf.entries();
    assert!(table.len() > 1, "C1 表应多行");
    for entry in &table {
        let inflated = zf
            .getInputStream(entry, false)
            .unwrap_or_else(|| panic!("C1 entry `{}` 解压失败", entry.getName()));
        assert_eq!(
            inflated.len() as i64,
            entry.getSize(),
            "entry `{}` 解压字节数必须等于 uncompressed_size",
            entry.getName()
        );
        let raw = zf
            .getInputStream(entry, true)
            .unwrap_or_else(|| panic!("C1 entry `{}` 原样取流失败", entry.getName()));
        if entry.getMethod() == STORED {
            assert_eq!(raw.len() as i64, entry.getSize(), "STORED 原样 == 数据");
        } else {
            assert_eq!(
                raw.len() as i64,
                entry.getCompressedSize(),
                "entry `{}` 原样字节数必须等于 compressed_size",
                entry.getName()
            );
            assert!(raw.len() <= inflated.len(), "压缩态不得大于解压态：{}", entry.getName());
        }
    }
}

// ── T08 尺寸不符 ⇒ `None`（COV-10，★按本轮实现口径更正） ──────────────────
#[test]
fn t08_size_mismatch_returns_none() {
    let src = std::fs::read(corpus(C2)).expect("读 C2");
    // 中央目录里第 1 项声明 uncompressed size = 真值 + 1
    let mut archive = zip::ZipArchive::new(Cursor::new(&src)).expect("crate 直读");
    let real = archive.by_index_raw(1).expect("by_index_raw").size() as u32;
    let patched = patchCentralUncompressedSize(&src, 1, real + 1);
    let p = writeTemp("size-mismatch.zip", &patched);
    let zf = ZipFile::from_path(&p).expect("坏尺寸包仍应能打开（打开期只读中央目录）");
    let name = archive.by_index_raw(1).expect("by_index_raw").name().to_string();
    let entry = zf.getEntry(&name).expect("entry 应存在");
    assert_eq!(entry.getSize(), real as i64 + 1, "改写后的声明值必须生效");
    // Java：`ZipInflaterInputStream.read` `:381-383` 抛 `Size mismatch on inflated file`；
    // S0 轨：解压态做同口径校验 ⇒ `None`（观测等价于上层 catch ⇒ null）。
    assert_eq!(zf.getInputStream(&entry, false), None, "尺寸不符必须失败（不得静默给错字节）");
    // 原样态不校验尺寸（Java `:182` RAFStream 无尺寸校验）⇒ 仍应成功
    assert!(zf.getInputStream(&entry, true).is_some(), "原样态无尺寸校验，必须仍可取");
}

// ── T09 取流可重复（COV-12/16） ─────────────────────────────────────────
#[test]
fn t09_stream_read_is_repeatable() {
    let zf = ZipFile::from_path(&corpus(C2)).expect("C2");
    let entry = zf.getEntry(XML_ENTRY).expect("entry 应存在");
    let a1 = zf.getInputStream(&entry, false).expect("第一次");
    let a2 = zf.getInputStream(&entry, false).expect("第二次");
    let b1 = zf.getInputStream(&entry, true).expect("原样第一次");
    let b2 = zf.getInputStream(&entry, true).expect("原样第二次");
    assert_eq!(a1, a2, "缺省态两次必须相同");
    assert_eq!(b1, b2, "原样态两次必须相同");
    // 取流后句柄仍可用（Java `:187` 的 raf 不被关闭）
    assert!(zf.getEntry(XML_ENTRY).is_some());
}
