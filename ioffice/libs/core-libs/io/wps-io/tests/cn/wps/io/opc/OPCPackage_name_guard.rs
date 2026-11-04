// function_id=18_function_zip_name_guard
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 18_function_zip_name_guard

use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::xwpf::util::ZipEntrySource::FakeZipEntry;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn passes(name: &str) {
    catch_unwind(|| OPCPackage::checkEntryNameLegal(name)).unwrap_or_else(|_| {
        panic!("legal name was rejected: {name:?}")
    });
}

fn rejected(name: &str) {
    assert!(
        catch_unwind(|| OPCPackage::checkEntryNameLegal(name)).is_err(),
        "illegal name was accepted: {name:?}"
    );
}

#[test]
fn n_legal_relative_name_passes() {
    passes("word/document.xml");
}

#[test]
fn n_legal_content_types_name_passes() {
    passes("[Content_Types].xml");
}

#[test]
fn n_legal_rels_name_passes() {
    passes("word/_rels/document.xml.rels");
}

#[test]
fn n_leading_slash_passes_unchanged() {
    passes("/word/document.xml");
}

#[test]
fn n_empty_name_passes() {
    passes("");
}

#[test]
fn n_directory_entry_name_passes() {
    passes("word/");
}

#[test]
fn n_trailing_dotdot_without_slash_passes() {
    passes("word/..");
}

#[test]
fn n_dotdot_substring_without_slash_passes() {
    passes("word/..hidden.xml");
    passes("a..b/c.xml");
}

#[test]
fn n_backslash_traversal_passes() {
    passes(r"..\evil.xml");
}

#[test]
fn n_non_ascii_legal_name_passes() {
    passes("文档/正常.xml");
}

#[test]
fn n_leading_traversal_rejected() {
    rejected("../evil.xml");
}

#[test]
fn n_middle_traversal_rejected() {
    rejected("word/../evil.xml");
}

#[test]
fn n_nested_traversal_rejected() {
    rejected("a/../../b");
}

#[test]
fn n_non_ascii_traversal_rejected() {
    rejected("文档/../x.xml");
}

#[test]
fn n_exact_traversal_rejected() {
    rejected("../");
}

#[test]
fn n_build_part_name_panics_on_illegal() {
    assert!(catch_unwind(|| {
        let _ = OPCPackage::buildPartName("word/../evil.xml");
    })
    .is_err());
}

#[test]
fn n_get_parts_impl_panics_on_illegal() {
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(ZipInputStreamZipEntrySource::from_entries(vec![
        FakeZipEntry::new("word/../evil.xml", Vec::new()),
    ])));
    assert!(catch_unwind(AssertUnwindSafe(|| pkg.getPartsImpl())).is_err());
}

#[test]
fn n_wrapper_delegates_same_verdict() {
    assert!(catch_unwind(|| XWPFPackage::checkEntryNameLegal("word/document.xml")).is_ok());
    assert!(catch_unwind(|| XWPFPackage::checkEntryNameLegal("word/../evil.xml")).is_err());
}

#[test]
fn n_verdict_is_deterministic() {
    for _ in 0..100 {
        passes("word/document.xml");
        rejected("word/../evil.xml");
    }
}

#[test]
fn n_judgement_is_stateless() {
    for i in 0..50 {
        if i % 2 == 0 {
            passes("word/document.xml");
        } else {
            rejected("../evil.xml");
        }
    }
    passes("word/document.xml");
    rejected("../evil.xml");
}

/// C7 语料（**用例内构造**的真 zip，不是合成 entry）：`[Content_Types].xml` 在前（合法，先把 CT 认出来），
/// 非法名 entry `word/../evil.xml` 在后 —— 走的是**文件后端**：名字来自 kfs 中央目录，而非测试注入的内存 entry。
/// ⚠️ 与 `n_get_parts_impl_panics_on_illegal`（内存注入）互补：本用例覆盖 `from_path` → kfs 句柄那一侧。
fn c7_zip_bytes() -> Vec<u8> {
    use std::io::Write as _;
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    writer.start_file("[Content_Types].xml", options).unwrap();
    writer
        .write_all(br#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#)
        .unwrap();
    // Java: entryName.contains("../") ⇒ FileDamagedException（`OPCPackage.java:533` 调用点）
    writer.start_file("word/../evil.xml", options).unwrap();
    writer.write_all(b"pwn").unwrap();
    writer.finish().unwrap().into_inner()
}

fn temp_path(tag: &str) -> std::path::PathBuf {
    static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "ioffice-18-name-guard-{}-{}-{}.docx",
        tag,
        std::process::id(),
        SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}

#[test]
fn c7_corpus_illegal_name_rejected() {
    let path = temp_path("c7");
    std::fs::write(&path, c7_zip_bytes()).unwrap();

    // 先确认 kfs 中央目录确实把非法名原样带出来（否则用例会退化成「入口就没读到」的假绿）
    let source = core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource::from_path(
        path.to_str().unwrap(),
    )
    .expect("C7 包应能打开");
    let names: Vec<String> = source
        .getEntries()
        .unwrap()
        .iter()
        .map(|e| e.getName().to_string())
        .collect();
    assert_eq!(names, vec!["[Content_Types].xml", "word/../evil.xml"]);

    // 完整读链：`08/16` 的路径后端 → `getPartsImpl` 的扫描
    // Java: `OPCPackage(String path, PackageAccess access)` —— Rust 同签名（`OPCPackage.rs:290`），不是 `Result`
    let mut pkg = OPCPackage::from_path(path.to_str().unwrap(), PackageAccess::READ);
    let outcome = catch_unwind(AssertUnwindSafe(|| pkg.getPartsImpl()));
    let _ = std::fs::remove_file(&path);

    let payload = outcome.expect_err("含 `../` 的 entry 名必须被拒绝");
    let message = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    // 判据只可能是名门禁：本包的 entry 都只有几个字节 ⇒ 尺寸门禁（1 GiB）不可能触发
    assert_eq!(message, "FileDamagedException");
}
