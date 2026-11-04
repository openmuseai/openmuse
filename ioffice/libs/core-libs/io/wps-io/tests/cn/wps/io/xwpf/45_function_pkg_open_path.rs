// 45_function_pkg_open_path —— 路径入口认包（P1–P16）
//
// 规格：`features/s01-07-package-open/functions/45_function_pkg_open_path/FUNCTION.md` §3.4。
//
// ★实测基线（本轮现场回放后才写断言）：
//   P1  `open_path(C1, 0)`   → part 表 **9**、表已建
//   P3  `open_path(C1, 1)`   → 表 **None**（WRITE 不建表）
//   P15 `-1` / `99`          → 同 WRITE（`access == 0` 才是 READ）
//   P4/P5/P6                 → panic `"path"`（空/空白/目录）
//   P7                       → panic `"Can't open the specified file: '<abs>'"`

use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::path::{Path, PathBuf};

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

/// 写一个临时文件（测试自清理由 `tempfile` 语义保证：路径唯一即可）。
fn write_temp(name: &str, bytes: &[u8]) -> PathBuf {
    let p = std::env::temp_dir().join(format!("s01_45_{}_{}", std::process::id(), name));
    std::fs::write(&p, bytes).expect("写临时文件");
    p
}

// ───────────────── P1 / P2 / P11 / P12：C1 成功路径 ─────────────────
#[test]
fn p1_p2_p11_p12_c1_success() {
    let mut pk = XWPFPackage::open_path(c1_path().to_str().unwrap(), 0);
    assert!(pk.getPartList().is_some(), "getPartList 为 None ⇒ FAIL（P2）");
    let n = pk.getParts().len();
    assert!(n > 0, "part_count=0 且 error=null ⇒ FAIL（P1）");
    assert_eq!(n, 9, "C1 实测 part 表应为 9，实得 {}", n);
    // P11：path 入口**必须**置 originalPackagePath
    assert!(
        pk.getOriginalPackagePath().is_some(),
        "path 入口必须置位 originalPackagePath ⇒ FAIL（P11）"
    );
    assert!(
        pk.getOriginalPackagePath().unwrap().ends_with("standard-hello.docx"),
        "应指向 C1 的绝对路径"
    );
    // P12：open_entry == "path"
    assert_eq!(
        core_libs_io::io::opc::ApiTrace::OpenEntry::Path.as_str(),
        "path"
    );
}

// ───────────────── P3 / P15：access != 0 ⇒ WRITE，不建表 ─────────────────
#[test]
fn p3_write_access_does_not_build_table() {
    // ★口径澄清（实测 + 两侧源码对照）：
    // `XWPFPackage.java:76` 的守卫是 `if (pack.partList == null && access != WRITE)`，
    // 它只挡**隐式**调用；显式再调 `getParts()` **仍会**建表（两侧同为 `if partList == null`）。
    // ⇒ P3 的正确断言是「**构造后立刻**表为 None」，而不是「调用 getParts 后仍为 None」。
    //
    // （前次断言写成「getParts 后仍 0」⇒ 实测 9，是**测试写错**，不是实现缺陷。）
    let p = c1_path();
    let pk = XWPFPackage::open_path(p.to_str().unwrap(), 1);
    assert!(
        pk.getPartList().is_none(),
        "access=1 是 WRITE ⇒ **构造后**不得建表（认了包 ⇒ FAIL，P3）"
    );
    // 显式调用才会建表——这与 Java `:76` 的守卫语义**一致**（守卫只挡隐式调用）
    let mut pk2 = XWPFPackage::open_path(p.to_str().unwrap(), 1);
    assert_eq!(
        pk2.getParts().len(),
        9,
        "显式 getParts() 在两侧都会建表（Java :76 守卫只挡隐式调用）"
    );
}

#[test]
fn p15_negative_and_large_access_are_write() {
    // ★实测：只有 `access == 0` 是 READ，其余（含 -1 / 99）全是 WRITE
    let p = c1_path();
    for a in [-1i32, 99] {
        let pk = XWPFPackage::open_path(p.to_str().unwrap(), a);
        assert!(
            pk.getPartList().is_none(),
            "access={} 必须走 WRITE 分支（**构造后**不得建表 ⇒ FAIL，P15）",
            a
        );
    }
}

// ───────────────── P4 / P5 / P6：空 / 空白 / 目录 ⇒ 拒绝 ─────────────────
#[test]
fn p4_p5_p6_reject_invalid_paths() {
    let e = catch(|| {
        let _ = XWPFPackage::open_path("", 0);
    })
    .expect_err("空串必须被拒绝（返回包 ⇒ FAIL，P4）");
    assert_eq!(e, "path", "实测 payload 为 \"path\"，实得 {:?}", e);

    let e = catch(|| {
        let _ = XWPFPackage::open_path("   ", 0);
    })
    .expect_err("纯空白必须被拒绝（P5）");
    assert_eq!(e, "path");

    let dir = std::env::temp_dir();
    let e = catch(|| {
        let _ = XWPFPackage::open_path(dir.to_str().unwrap(), 0);
    })
    .expect_err("目录必须被拒绝（P6）");
    assert_eq!(e, "path");
}

// ───────────────── P7：不存在的文件 ⇒ 底座拒绝 ─────────────────
#[test]
fn p7_missing_file_rejected() {
    let missing = std::env::temp_dir().join("s01_45_definitely_no_such_file.docx");
    assert!(!missing.exists(), "前提：文件不得存在");
    let e = catch(move || {
        let _ = XWPFPackage::open_path(missing.to_str().unwrap(), 0);
    })
    .expect_err("不存在的文件必须被拒绝（P7）");
    assert!(
        e.starts_with("Can't open the specified file"),
        "实测 payload 前缀应为 \"Can't open the specified file\"，实得 {:?}",
        e
    );
}

// ───────────────── P8 / P9：非 zip / 半截 ─────────────────
#[test]
fn p8_not_zip_kind() {
    let f = write_temp("notzip.docx", b"not a zip at all");
    let e = catch(|| {
        let _ = XWPFPackage::open_path(f.to_str().unwrap(), 0);
    })
    .expect_err("非 zip 文件必须被拒绝");
    // 与 44 的 B4 同因：原始文本被 W4 丢弃 ⇒ 必须靠 at 语境
    let k = kind_at(
        &e,
        "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
    );
    assert_eq!(k, "not_zip", "实得 {}", k);
    let _ = std::fs::remove_file(&f);
}

#[test]
fn p9_truncated_path_backend_reports_eocd_missing() {
    // ★★关键实测（**更正规格**）：两个后端对「截断」的语义**不同**！
    //
    //   · **流后端**（`open_stream` / `44` 的 B5）：`ZipInputStream` 边读边解、
    //     **不要求 EOCD** ⇒ 半截 C1 **不报错**，解出 4 条 entry。
    //   · **路径后端**（`open_path` / 本用例）：`ZipFile` 走中央目录，
    //     **要求 EOCD** ⇒ 半截文件 panic
    //     `"ZipException: End of Central Directory signature not found"`
    //     （`kfs_app/src/cn/wps/moffice/kfs/zip/ZipFile.rs:518`）。
    //
    // 规格原写「P9 ⇒ zip_corrupt」；**实测**路径后端确实**失败**，方向一致，
    // 但**失败形态是 panic（非 `Result`）**，且 `kind` 需靠 at 语境判定。
    // 两侧一致性：Java `ZipFile` 同样要求 EOCD ⇒ **两侧同语义**。
    let b = std::fs::read(c1_path()).unwrap();
    let f = write_temp("trunc.docx", &b[..b.len() / 2]);
    let e = catch(|| {
        let _ = XWPFPackage::open_path(f.to_str().unwrap(), 0);
    })
    .expect_err("★实测：路径后端**要求 EOCD** ⇒ 半截文件必须失败");
    // ★更精确的实测：内层 `ZipFile.rs:518` 的 EOCD panic **被 `ZipHelper::openZipFile`
    //   捕获并转为 `None`** ⇒ 外层 `OPCPackage.rs:375` 抛出的是**统一的**
    //   `"Can't open the specified file: '<path>'"`，**原始 EOCD 文本被吞掉**。
    //   ⇒ 这是 P9 场景下的**第二个静默点**（与 W4 同型：原始原因在边界被丢弃）。
    //   ⇒ `kind` **无法**从 payload 判定，只能靠 `at` 语境（与 B4/C11 同构）。
    assert!(
        e.starts_with("Can't open the specified file"),
        "内层 EOCD panic 被吞 ⇒ 外层应为统一文本，实得 {:?}",
        e
    );
    assert!(
        !e.contains("Central Directory"),
        "原始 EOCD 文本**不得**出现在外层 payload（若出现说明边界行为变了）：{:?}",
        e
    );
    // 无 at 语境 ⇒ 只能兜底
    assert_eq!(
        kind_at(&e, "unknown#x"),
        "io_error",
        "丢失原因的 payload 无 at 语境时只能兜底"
    );
    let _ = std::fs::remove_file(&f);
}

/// 与 P9 对照：**流后端**对同一份半截文件**不报错**（两个后端语义不同，须登记）。
#[test]
fn p9_backend_difference_stream_tolerates_truncation() {
    use std::io::Cursor;
    let b = std::fs::read(c1_path()).unwrap();
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(b[..b.len() / 2].to_vec()));
    assert_eq!(
        pk.getParts().len(),
        4,
        "★流后端**不要求 EOCD** ⇒ 半截仍解出 4 条（与路径后端相反，已登记差异）"
    );
}

// ───────────────── P10：C14 加密 —— 未采 ─────────────────
#[test]
#[ignore = "C14（加密 docx）语料尚未造 ⇒ 未采；见 CORPUS.md §2"]
fn p10_encrypted_unsupported() {
    let p = crate_root().join("tests/fixtures/s01_negative/C14_encrypted.docx");
    assert!(p.exists(), "C14 缺失 ⇒ 保持 ignored（未采）");
}

// ───────────────── P11 反向：拒绝前不得置位 ─────────────────
#[test]
fn p11_rejected_path_leaves_no_side_effect() {
    // P4 拒绝时不得留下「已置位」的假状态（函数整体 panic ⇒ 无返回值，
    // 此处用「同一进程内后续调用不受影响」验证无全局污染）。
    let _ = catch(|| {
        let _ = XWPFPackage::open_path("", 0);
    });
    let mut pk = XWPFPackage::open_path(c1_path().to_str().unwrap(), 0);
    assert!(
        pk.getParts().len() > 0,
        "拒绝路径不得污染后续认包（P11 反向）"
    );
}

// ───────────────── P13 / P16：确定性与语料完整性 ─────────────────
#[test]
fn p13_p16_deterministic_and_corpus_intact() {
    let b = std::fs::read(c1_path()).unwrap();
    assert_eq!(b.len(), 3502, "C1 大小应为 3,502 B（语料被改 ⇒ FAIL，P16）");
    let mut a = XWPFPackage::open_path(c1_path().to_str().unwrap(), 0);
    let mut c = XWPFPackage::open_path(c1_path().to_str().unwrap(), 0);
    assert_eq!(
        a.getParts().len(),
        c.getParts().len(),
        "两次认包结果必须相同（P13）"
    );
}

// ───────────────── P14：三入口同一 C1 的 parts[] 序列相等 ─────────────────
#[test]
fn p14_three_entries_same_corpus_same_parts() {
    use std::io::Cursor;
    let p = c1_path();
    let b = std::fs::read(&p).unwrap();

    let mut via_bytes = XWPFPackage::open_stream(&mut Cursor::new(b.clone()));
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
    assert_eq!(a.len(), 9, "bytes 入口 parts[] 应为 9");
    assert_eq!(a, c, "bytes 与 path 的 parts[] 序列必须相等（P14）");
    assert_eq!(a, d, "bytes 与 raf 的 parts[] 序列必须相等（P14）");
}

// ───────────────── 路径类型健壮性：Path 与 &str 一致 ─────────────────
#[test]
fn path_and_str_agree() {
    let p = c1_path();
    let mut a = XWPFPackage::open_path(p.to_str().unwrap(), 0);
    let mut b = XWPFPackage::open_path(Path::new(&p).to_str().unwrap(), 0);
    assert_eq!(a.getParts().len(), b.getParts().len());
    assert_eq!(a.getParts().len(), 9);
}
