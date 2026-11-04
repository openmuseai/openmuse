// 46_function_pkg_open_raf —— 随机访问句柄入口认包（R1–R15）
//
// 规格：`features/s01-07-package-open/functions/46_function_pkg_open_raf/FUNCTION.md` §3.4。
//
// ★实测基线（本轮现场回放后才写断言）：
//   R1  `open_raf(&PathBuf(C1), 0)` → part 表 **9**、表已建、`originalPackagePath` 为 `None`
//   R3  `open_raf(&path, 1)`        → 表 **None**（WRITE）
//   R4  `open_raf(&String(C1), 0)`  → 与 R1 同结果（**类型无关**）
//   R5  `open_raf(&42i32, 0)`       → panic `"Can't open the specified raf"`（**单侧**，Java 不可复现）

use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c1_path() -> PathBuf {
    crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx")
}

fn catch<F: FnOnce() -> T, T>(f: F) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string payload>".to_string())
    })
}

fn kind_at(payload: &str, at: &str) -> String {
    use core_libs_io::io::opc::S01Error::S01Taxonomy;
    S01Taxonomy::classify_panic_at(payload, Some(at))
        .kind
        .as_str()
        .to_string()
}

fn write_temp(name: &str, bytes: &[u8]) -> PathBuf {
    let p = std::env::temp_dir().join(format!("s01_46_{}_{}", std::process::id(), name));
    std::fs::write(&p, bytes).expect("写临时文件");
    p
}

// ───────────────── R1 / R2 / R10 / R11：C1 成功路径 ─────────────────
#[test]
fn r1_r2_r10_r11_c1_success() {
    let p = c1_path();
    let mut pk = XWPFPackage::open_raf(&p, 0);
    assert!(pk.getPartList().is_some(), "getPartList 为 None ⇒ FAIL（R2）");
    let n = pk.getParts().len();
    assert!(n > 0, "part_count=0 且 error=null ⇒ FAIL（R1）");
    assert_eq!(n, 9, "C1 实测 part 表应为 9，实得 {}", n);
    // R11：raf 入口**不得**置 originalPackagePath（置位说明误走了 path 分支）
    assert!(
        pk.getOriginalPackagePath().is_none(),
        "raf 入口被置 originalPackagePath ⇒ FAIL（R11）"
    );
    // R10
    assert_eq!(
        core_libs_io::io::opc::ApiTrace::OpenEntry::Raf.as_str(),
        "raf"
    );
}

// ───────────────── R3 / R14：WRITE 分支 ─────────────────
#[test]
fn r3_write_access_does_not_build_table() {
    let p = c1_path();
    let pk = XWPFPackage::open_raf(&p, 1);
    assert!(
        pk.getPartList().is_none(),
        "access=1 是 WRITE ⇒ **构造后**不得建表（认了包 ⇒ FAIL，R3）"
    );
}

#[test]
fn r14_negative_and_large_access_are_write() {
    let p = c1_path();
    for a in [-1i32, 99] {
        let pk = XWPFPackage::open_raf(&p, a);
        assert!(
            pk.getPartList().is_none(),
            "access={} 必须走 WRITE 分支 ⇒ FAIL（R14）",
            a
        );
    }
}

// ───────────────── R4：`&String` 与 `&PathBuf` 同结果（类型无关） ─────────────────
#[test]
fn r4_string_source_same_as_pathbuf() {
    let p = c1_path();
    let s = p.to_string_lossy().to_string();
    let mut a = XWPFPackage::open_raf(&p, 0);
    let mut b = XWPFPackage::open_raf(&s, 0);
    assert_eq!(
        a.getParts().len(),
        b.getParts().len(),
        "`&String` 与 `&PathBuf` 必须同结果（R4）"
    );
    assert_eq!(a.getParts().len(), 9);
}

// ───────────────── R5：非路径类型 ⇒ 拒绝（**单侧**） ─────────────────
#[test]
fn r5_non_path_type_rejected() {
    let x = 42i32;
    let e = catch(|| {
        let _ = XWPFPackage::open_raf(&x, 0);
    })
    .expect_err("非路径类型必须被拒绝（返回包 ⇒ FAIL，R5）");
    assert_eq!(
        e, "Can't open the specified raf",
        "实测 payload 应为固定串，实得 {:?}",
        e
    );
    // ★R5 是**单侧**用例：Java 的类型系统在编译期就不允许传 `int`
    //   ⇒ 无可比对的 Java 行为；此处只做 Rust 侧现状断言。
}

// ───────────────── R6：不存在的文件 ⇒ 拒绝 ─────────────────
#[test]
fn r6_missing_file_rejected() {
    let missing = std::env::temp_dir().join("s01_46_definitely_no_such.docx");
    assert!(!missing.exists(), "前提：文件不得存在");
    let e = catch(move || {
        let _ = XWPFPackage::open_raf(&missing, 0);
    })
    .expect_err("不存在的文件必须被拒绝（R6）");
    assert!(
        e.starts_with("Can't open") || e.starts_with("Can't open the specified raf"),
        "实得 {:?}",
        e
    );
}

// ───────────────── R7 / R8：非 zip / 截断 ─────────────────
#[test]
fn r7_not_zip_kind() {
    let f = write_temp("notzip.docx", b"not a zip at all");
    let e = catch(|| {
        let _ = XWPFPackage::open_raf(&f, 0);
    })
    .expect_err("非 zip 必须被拒绝（R7）");
    // 与 44/45 同因：原因文本在边界被丢弃 ⇒ 必须靠 at 语境
    let k = kind_at(
        &e,
        "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
    );
    assert!(
        ["not_zip", "zip_corrupt", "io_error"].contains(&k.as_str()),
        "实得 {}",
        k
    );
    let _ = std::fs::remove_file(&f);
}

#[test]
fn r8_truncated_fails_on_path_backend() {
    // ★实测（与 45 的 P9 同后端）：`ZipFile` 要求 EOCD ⇒ 半截文件失败；
    // 且内层 EOCD panic 被 `ZipHelper::openZipFile` 吞成 `None`，
    // 外层为统一文本 `"Can't open the specified raf"`。
    let b = std::fs::read(c1_path()).unwrap();
    let f = write_temp("trunc.docx", &b[..b.len() / 2]);
    let e = catch(|| {
        let _ = XWPFPackage::open_raf(&f, 0);
    })
    .expect_err("★实测：路径类后端要求 EOCD ⇒ 半截必须失败（R8）");
    assert!(!e.is_empty(), "payload 必须非空");
    let _ = std::fs::remove_file(&f);
}

// ───────────────── R9：C14 加密 —— 未采 ─────────────────
#[test]
#[ignore = "C14（加密 docx）语料尚未造 ⇒ 未采；见 CORPUS.md §2"]
fn r9_encrypted_unsupported() {
    let p = crate_root().join("tests/fixtures/s01_negative/C14_encrypted.docx");
    assert!(p.exists(), "C14 缺失 ⇒ 保持 ignored（未采）");
}

// ───────────────── R12 / R15：确定性与语料完整性 ─────────────────
#[test]
fn r12_r15_deterministic_and_corpus_intact() {
    let b = std::fs::read(c1_path()).unwrap();
    assert_eq!(b.len(), 3502, "C1 大小应为 3,502 B（语料被改 ⇒ FAIL，R15）");
    let p = c1_path();
    let mut a = XWPFPackage::open_raf(&p, 0);
    let mut c = XWPFPackage::open_raf(&p, 0);
    assert_eq!(a.getParts().len(), c.getParts().len(), "两次结果必须相同（R12）");
}

// ───────────────── R13：三入口同一 C1 ⇒ parts[] 序列相等 ─────────────────
#[test]
fn r13_three_entries_same_corpus_same_parts() {
    use std::io::Cursor;
    let p = c1_path();
    let b = std::fs::read(&p).unwrap();

    let mut via_bytes = XWPFPackage::open_stream(&mut Cursor::new(b));
    let mut via_path = XWPFPackage::open_path(p.to_str().unwrap(), 0);
    let mut via_raf = XWPFPackage::open_raf(&p, 0);

    let mut names = |pk: &mut XWPFPackage| -> Vec<String> {
        let mut v: Vec<String> = pk
            .getParts()
            .iter()
            .map(|x| format!("{:?}", x.getPartName().getURI()))
            .collect();
        v.sort();
        v
    };
    let a = names(&mut via_bytes);
    let c = names(&mut via_path);
    let d = names(&mut via_raf);
    assert_eq!(a, c, "bytes 与 path 的 parts[] 必须相等（R13）");
    assert_eq!(a, d, "bytes 与 raf 的 parts[] 必须相等（R13）");
    assert_eq!(a.len(), 9, "C1 三入口均应认出 9 个 part");
}

// ───────────────── R11 反向：raf 与 path 的置位差异是稳定可区分的 ─────────────────
#[test]
fn r11_raf_vs_path_differ_only_in_original_package_path() {
    let p = c1_path();
    let mut raf = XWPFPackage::open_raf(&p, 0);
    let mut path = XWPFPackage::open_path(p.to_str().unwrap(), 0);
    // 唯一差异：originalPackagePath（raf = None、path = Some）
    assert!(raf.getOriginalPackagePath().is_none(), "raf 不得置位");
    assert!(path.getOriginalPackagePath().is_some(), "path 必须置位");
    // parts[] 本体相等
    assert_eq!(raf.getParts().len(), path.getParts().len());
    assert_eq!(raf.getParts().len(), 9);
}
