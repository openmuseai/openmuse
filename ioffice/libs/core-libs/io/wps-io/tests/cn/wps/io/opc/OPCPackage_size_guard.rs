// function_id=19_function_zip_size_guard
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 19_function_zip_size_guard

use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::OPCPackage::{OPCPackage, MAX_SIZE_ENTRY};
use core_libs_io::io::xwpf::util::ZipEntrySource::FakeZipEntry;
use core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry as KfsZipEntry;
use std::io::Cursor;
use std::panic::AssertUnwindSafe;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// 文件后端 entry 的 Java 形态：它**就是** kfs `ZipEntry`（`size`/`compressionMethod` 来自中央目录）
/// ⇒ Rust 侧包进 `FakeZipEntry` 的父类部分（`FakeZipEntry::from_zip_entry`）。
fn declared_entry(name: &str, data: &[u8], size: i64, method: i32) -> FakeZipEntry {
    let mut zip_entry = KfsZipEntry::new__string(name.to_string());
    zip_entry.setSize(size);
    zip_entry.setMethod(method);
    FakeZipEntry::from_zip_entry(zip_entry, data.to_vec())
}

#[test]
fn g_small_sizes_pass() {
    for size in [0_i64, 1, 1024, 1_048_576] {
        assert!(!OPCPackage::isEntryTooLarge(size), "size {size}");
    }
}

#[test]
fn g_threshold_itself_passes() {
    assert!(!OPCPackage::isEntryTooLarge(1_073_741_824));
}

#[test]
fn g_one_over_threshold_rejected() {
    assert!(OPCPackage::isEntryTooLarge(1_073_741_825));
}

#[test]
fn g_i64_max_rejected_without_overflow() {
    assert!(OPCPackage::isEntryTooLarge(i64::MAX));
}

#[test]
fn g_negative_size_passes() {
    assert!(!OPCPackage::isEntryTooLarge(-1));
}

#[test]
fn g_const_is_2_pow_30() {
    assert_eq!(MAX_SIZE_ENTRY, 1_073_741_824);
}

/// Java: `FakeZipEntry(String,ZipInputStream)` 只调 `super(entryName)` ⇒ `getSize()` 是继承来的
/// `ZipEntry.size` 默认值 **-1**（`ZipEntry.java:19`），**不是**物化字节长度 —— 旧 Rust 写成
/// `data.len()` 属移植错误（本轮修正）。
#[test]
fn g_entry_size_is_declared_not_data_len() {
    let stream_entry = FakeZipEntry::new("n", vec![0u8; 7]);
    assert_eq!(stream_entry.getSize(), -1);
    // 文件后端的 entry 在 Java 侧就是 kfs `ZipEntry`（`size` 来自中央目录）⇒ 声明尺寸可注入。
    let declared = declared_entry("n", &[0u8; 7], 1_073_741_825, 8);
    assert_eq!(declared.getSize(), 1_073_741_825);
    assert_eq!(declared.getMethod(), 8);
    assert!(OPCPackage::isEntryTooLarge(declared.getSize()));
}

/// Java: `if (entry.getSize() > 1073741824L) throw new FileDamagedException();`
/// （`OPCPackage.java:528-530`）—— 只读**声明**尺寸 ⇒ 不需要真的分配 1 GiB 字节。
#[test]
fn g_get_parts_impl_panics_on_oversized() {
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(ZipInputStreamZipEntrySource::from_entries(vec![
        declared_entry("/word/document.xml", b"tiny", 1_073_741_825, 8),
    ])));
    let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| pkg.getPartsImpl()));
    let payload = outcome.expect_err("oversized declared size must reject the package");
    let message = payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    assert_eq!(message, "FileDamagedException");
}

/// Java 循环体内顺序：先尺寸门禁（`:528`）后名字门禁（`:533`）。⚠️ **顺序本身在两侧都不可观测**：
/// 两个门禁抛的都是 `FileDamagedException`（无 discriminative message）⇒ 本用例只能断言「两条门禁
/// 同时命中时仍被拒」，**不**断言是哪一条先抛（不伪造更强的绿）。
#[test]
fn g_size_gate_runs_before_name_gate() {
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(ZipInputStreamZipEntrySource::from_entries(vec![
        // 非法名（含 `../`）**且**声明尺寸超限
        declared_entry("../evil.xml", b"tiny", 1_073_741_825, 8),
    ])));
    let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| pkg.getPartsImpl()));
    let payload = outcome.expect_err("oversized + illegal name must reject");
    let message = payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    assert_eq!(message, "FileDamagedException");
}

/// Java: 判据是 `>`（严格大于）⇒ 声明尺寸 **恰好 = 1 GiB** 必须放行（`OPCPackage.java:528`）。
/// 声明尺寸可注入 ⇒ 不需要真的构造 2^30 字节（旧 Rust 用 `data.len()` 时本用例无法写）。
#[test]
fn g_scan_at_threshold_passes() {
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(ZipInputStreamZipEntrySource::from_entries(vec![
        declared_entry("/word/document.xml", b"tiny", MAX_SIZE_ENTRY as i64, 8),
    ])));
    let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| pkg.getPartsImpl()));
    assert!(outcome.is_ok(), "declared size == threshold must pass the gate");
    assert!(pkg.getPartList().is_some(), "scan completed without rejecting");
}

#[test]
fn g_verdict_is_deterministic() {
    for _ in 0..100 {
        assert!(!OPCPackage::isEntryTooLarge(1_073_741_824));
        assert!(OPCPackage::isEntryTooLarge(1_073_741_825));
    }
}

#[test]
fn c1_no_entry_exceeds_threshold() {
    let bytes = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"),
    )
    .unwrap();
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    for entry in source.getEntries().as_ref().unwrap() {
        assert!(
            !OPCPackage::isEntryTooLarge(entry.getSize()),
            "{} size {}",
            entry.getName(),
            entry.getSize()
        );
    }
}

fn single_entry_zip(name: &str, bytes: &[u8]) -> Vec<u8> {
    use std::io::Write as _;
    // Deflated：kfs 读流时用 `endOffset = offset + entry.compressedSize`（`ZipFile.rs:403`）
    // ⇒ 改**中央目录里的 uncompressed size** 不会影响实际读取，只影响 `getSize()`（正是本用例要的）。
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    writer.start_file(name, options).unwrap();
    writer.write_all(bytes).unwrap();
    writer.finish().unwrap().into_inner()
}

/// 把中央目录第一条记录的 `uncompressed size` 改成 `declared`（`PK\x01\x02` + 24）。
fn patch_central_dir_size(mut bytes: Vec<u8>, declared: u32) -> Vec<u8> {
    let mut i = 0usize;
    while i + 28 <= bytes.len() {
        if bytes[i..i + 4] == [0x50, 0x4b, 0x01, 0x02] {
            bytes[i + 24..i + 28].copy_from_slice(&declared.to_le_bytes());
            return bytes;
        }
        i += 1;
    }
    panic!("central directory record not found");
}

fn temp_path(tag: &str) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "ioffice-19-size-guard-{}-{}-{}.zip",
        tag,
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ))
}

/// 「撒谎 header」语料：**中央目录**声明 2 GiB，实际内容只有 4 字节（真实 zip，非合成 entry）。
/// Java 文件后端拿到的是 kfs `ZipEntry` ⇒ `getSize()` = **声明**尺寸 ⇒ 门禁拒绝；
/// 旧 Rust 读 `data.len()` ⇒ 放行（本用例正是这条差异的回归网；★实测 2026-09-18 已对齐）。
#[test]
fn lying_header_declared_size() {
    let lying = patch_central_dir_size(single_entry_zip("word/document.xml", b"tiny"), 2_147_483_648);
    let path = temp_path("lying");
    std::fs::write(&path, &lying).unwrap();

    let source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).expect("open lying zip");
    let entries = source.getEntries().expect("entries");
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(entry.getName(), "word/document.xml");
    // 声明尺寸来自中央目录（不是物化字节长度）
    assert_eq!(entry.getSize(), 2_147_483_648, "declared size must come from the central directory");
    assert!(OPCPackage::isEntryTooLarge(entry.getSize()));
    // 撒谎的只是 header：物化出来的**字节**仍是那 4 字节（尺寸≠字节长度，故不能用长度当门禁输入）
    assert_eq!(entry.data(), b"tiny");
    let _ = std::fs::remove_file(&path);
}
