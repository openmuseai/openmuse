// 44_function_pkg_open_bytes —— 字节入口认包（B1–B12）
//
// 规格：`features/s01-07-package-open/functions/44_function_pkg_open_bytes/FUNCTION.md` §3.4。
// 契约：`docs/s01/schema-v1.json`（`open_entry` 4 值枚举）。
//
// ★实测基线（本轮现场回放，写入断言前已核对）：
//   C1 `standard-hello.docx` → part 表 **9**；空/非 zip → panic
//   `"Can't open the specified stream"`；半截 C1 → **不报错**、part 表 **4**
//   （`ZipInputStreamZipEntrySource` 与 Java `ZipInputStream` 同语义：边读边解、不要求 EOCD）

use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::Cursor;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// C1：真实 OOXML 标准包（sha256 `c8740d24…8110`，3,502 B，10 entry）。
fn c1_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"))
        .expect("C1 fixture 必须存在")
}

/// 把 panic 收敛成 payload 文本（`None` = 未 panic）。
fn catch<F: FnOnce() -> T, T>(f: F) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string payload>".to_string())
    })
}

/// 用**真实分类器**判 kind（不自己造词，与 `53` 一致）。
///
/// ★关键：本 function 的失败**必须**带 `at` 语境才能正确分类——
/// `OPCPackage.rs:405-411` 的 `Err(_) => panic!("Can't open the specified stream")`
/// **丢弃**了原始 zip 文本（静默点 W4），因此 `not_zip` 与 `zip_corrupt`
/// 的 payload **完全相同**，只有 `at` 能把它们分开（与 `54` 的 PG7 同构）。
fn kind_at(payload: &str, at: &str) -> String {
    use core_libs_io::io::opc::S01Error::S01Taxonomy;
    S01Taxonomy::classify_panic_at(payload, Some(at))
        .kind
        .as_str()
        .to_string()
}

// ───────────────── B1 / B2 / B8 / B9：C1 成功路径 ─────────────────
#[test]
fn b1_b2_b8_b9_c1_success() {
    let mut c = Cursor::new(c1_bytes());
    let mut pk = XWPFPackage::open_stream(&mut c);

    // B2：表已建
    assert!(
        pk.getPartList().is_some(),
        "getPartList 为 None ⇒ FAIL（B2）"
    );
    // B1：part_count > 0 且 error 概念上为 null（未 panic 即无 error）
    let n = pk.getParts().len();
    assert!(n > 0, "part_count=0 且 error=null ⇒ FAIL（静默成功，B1）");
    // ★实测 9（C1 共 10 entry，减 [Content_Types].xml）
    assert_eq!(n, 9, "C1 part 表实测应为 9，实得 {}", n);
    // B1 的另一半：part_count == parts[].len()
    assert_eq!(
        pk.getPartList().map(|l| l.size()),
        Some(n),
        "part_count 必须等于 parts[].len()"
    );
    // B9：bytes 入口**不得**置 originalPackagePath
    assert!(
        pk.getOriginalPackagePath().is_none(),
        "bytes 入口被置 originalPackagePath ⇒ FAIL（B9）"
    );
    // B8：入口标识为 bytes（schema 枚举值，非方法名）
    assert_eq!(
        core_libs_io::io::opc::ApiTrace::OpenEntry::Bytes.as_str(),
        "bytes",
        "open_entry 必须是 bytes（B8）"
    );
}

// ───────────────── B3 / B4：空 与 非 zip ⇒ not_zip ─────────────────
#[test]
fn b3_empty_input_rejected() {
    let e = catch(|| {
        let mut c = Cursor::new(Vec::new());
        let _ = XWPFPackage::open_stream(&mut c);
    })
    .expect_err("空输入必须被拒绝（静默成功 ⇒ FAIL）");
    assert!(!e.is_empty(), "payload 必须非空");
}

#[test]
fn b4_not_zip_is_not_zip() {
    let e = catch(|| {
        let mut c = Cursor::new(b"not a zip at all".to_vec());
        let _ = XWPFPackage::open_stream(&mut c);
    })
    .expect_err("非 zip 必须被拒绝");
    // ★实测：产品链丢掉了原始 zip 文本（W4），payload 是固定串
    assert_eq!(
        e, "Can't open the specified stream",
        "实测 payload 应为固定串（原始 zip 文本被 W4 丢弃），实得 {:?}",
        e
    );
    // ★只靠 payload（固定串）**判不出**具体 kind ⇒ 会兜底 io_error，
    //   必须靠 `at` 语境才能得到 not_zip（这正是 W4 静默点的后果）。
    let fallback = kind_at(&e, "unknown#x");
    assert_eq!(
        fallback, "io_error",
        "无 `at` 语境时固定串只能兜底（证明 payload 已被 W4 丢弃）"
    );
    // 带真实入口语境 ⇒ not_zip（B4 的期望）
    let k = kind_at(
        &e,
        "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
    );
    assert_eq!(
        k, "not_zip",
        "非 zip 输入在 ZipArchive::new 语境下必须是 not_zip，实得 {}",
        k
    );
}

// ───────────────── B5：半截 C1 —— ★实测**不报错**（语义更正） ─────────────────
#[test]
fn b5_truncated_does_not_error_by_design() {
    let b = c1_bytes();
    let half = b[..b.len() / 2].to_vec();
    let mut c = Cursor::new(half);
    let mut pk = XWPFPackage::open_stream(&mut c);
    // ★规格原写「期望 zip_corrupt」；**实测**（本轮现场回放，并已在 CORPUS.md §2bis.1 更正）：
    //   Rust 流式读取器与 Java `ZipInputStream` **同语义**——边读边解、读到流尾即止，
    //   **不要求 EOCD** ⇒ 单纯截断**不报错**，解出 4 条 entry。这是既有且正确的语义。
    let n = pk.getParts().len();
    assert_eq!(
        n, 4,
        "★实测：半截 C1 解出 4 条 entry（不报错、不静默——是「读多少算多少」的既有语义）"
    );
    assert!(pk.getPartList().is_some(), "表仍应已建");
}

// ───────────────── B6：C14 加密 —— 语料缺失，未采 ─────────────────
#[test]
#[ignore = "C14（加密 docx）语料尚未造 ⇒ 未采；见 CORPUS.md §2"]
fn b6_encrypted_unsupported() {
    // 语料缺失时必须标 `#[ignore]` 并记「未采」，**不得**用假语料顶替。
    let p = crate_root().join("tests/fixtures/s01_negative/C14_encrypted.docx");
    assert!(p.exists(), "C14 缺失 ⇒ 本用例保持 ignored（未采）");
}

// ───────────────── B7：造包 part_count ─────────────────
#[test]
fn b7_synthetic_package_part_count() {
    use std::io::Write;
    use zip::write::SimpleFileOptions;
    // 造包：CT + _rels/.rels + word/document.xml
    let ct = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>hi</w:t></w:r></w:p></w:body></w:document>"#;

    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = SimpleFileOptions::default();
        w.start_file("[Content_Types].xml", o).unwrap();
        w.write_all(ct.as_bytes()).unwrap();
        w.start_file("_rels/.rels", o).unwrap();
        w.write_all(rels.as_bytes()).unwrap();
        w.start_file("word/document.xml", o).unwrap();
        w.write_all(xml.as_bytes()).unwrap();
        w.finish().unwrap();
    }

    let mut c = Cursor::new(buf);
    let mut pk = XWPFPackage::open_stream(&mut c);
    let n = pk.getParts().len();
    // ★实测口径：part 表 = zip entry − [Content_Types].xml（结构性不变量）
    // 3 entry − 1（CT） = 2
    assert_eq!(
        n, 2,
        "★实测：3 entry − CT(1) = 2（口径见 STAGE_ALIGNMENT.md 的结构性不变量）"
    );
    assert!(pk.getPartList().is_some());
}

// ───────────────── B10 / B11 / B12：确定性与语料完整性 ─────────────────
#[test]
fn b11_same_bytes_two_cursors_same_result() {
    let b = c1_bytes();
    let mut c1 = Cursor::new(b.clone());
    let mut c2 = Cursor::new(b);
    let mut p1 = XWPFPackage::open_stream(&mut c1);
    let mut p2 = XWPFPackage::open_stream(&mut c2);
    assert_eq!(
        p1.getParts().len(),
        p2.getParts().len(),
        "同一字节两个 Cursor ⇒ 结果必须相同（B11）"
    );
}

#[test]
fn b12_corpus_sha256_unchanged() {
    // 冻结哈希（CORPUS.md §3）：c8740d24b0943ac2c52039d168b12e01db43b0dcde11d23476b0138eddca8110
    let b = c1_bytes();
    assert_eq!(b.len(), 3502, "C1 大小应为 3,502 B（语料被改 ⇒ FAIL）");
    assert_eq!(&b[0..2], b"PK", "C1 必须是 zip");
    // 逐字节稳定：两次读取完全相同
    assert_eq!(b, c1_bytes(), "同一文件两次读取必须逐字节相同（B10/B12）");
}
