// `s0_05_function_zip_size_guard` —— 测试镜像（⇄ `src/cn/wps/kfc/opcr/OpcrPackage.rs` 的尺寸门禁面）。
// 覆盖矩阵与 PASS/FAIL 判据见 `features/s0-01-zip-source/functions/s0_05_function_zip_size_guard/FUNCTION.md` §3。
// 运行：cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test s0_05_function_zip_size_guard
//
// Java 锚点：`kfc/opcr/OpcrPackage.java:25`（`MAX_SIZE_ENTRY = 1073741824`）、`:220-242`（`getEntry`：
// 精确名 → `/`→`\` 试探 → `toLowerCase` 试探 → `if (entry.getSize() > 1073741824L) throw new FileDamagedException();`）。
//
// ★签名口径（既定裁定）：Java `throw` ⇄ Rust 现状 `panic!`；**panic→Result 一次性归 `s0_14`**
//   ⇒ 本轮锁「`>`（不是 `>=`）的边界」与三条试探顺序，并把 `Err(zip_corrupt)` 一类的期望登记为「待 s0_14」。

use core_libs_kfc::kfc::opcr::OpcrPackage::OpcrPackage;
use core_libs_kfc::kfc::opcr::ZipFile::ZipFile;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const C1: &str = "samples/ppt-render/flutter/assets/Sample_ppt.pptx";
const C2: &str = "samples/ppt-render/flutter/assets/hello.pptx";

fn repoRoot() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn corpus(rel: &str) -> String {
    repoRoot().join(rel).to_string_lossy().to_string()
}

fn tempDir() -> PathBuf {
    let d = std::env::temp_dir().join(format!("s0_05_size_guard_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&d);
    d
}

fn writeTemp(name: &str, bytes: &[u8]) -> String {
    let p = tempDir().join(name);
    std::fs::write(&p, bytes).expect("write temp fixture");
    p.to_string_lossy().to_string()
}

fn zipWith(items: &[(&str, &[u8])]) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut w = ZipWriter::new(Cursor::new(&mut buf));
        for (name, data) in items {
            w.start_file(*name, SimpleFileOptions::default())
                .expect("start_file");
            w.write_all(data).expect("write entry");
        }
        w.finish().expect("finish zip");
    }
    buf
}

/// 把第 `idx` 个中央目录项的 **uncompressed size**（CDE 偏移 +24）改写成 `declared`。
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

/// 声明尺寸被改写成 `declared` 的单 entry 包（entry 名 `a.xml`）。
/// ★`tag` 必须每处不同：同名临时文件被并行用例共享时，`fs::write` 的截断窗口会让并发读者看到半写文件（实测竞态）。
fn packageWithDeclaredSize(tag: &str, declared: u32) -> (String, ZipFile) {
    let bytes = patchCentralUncompressedSize(&zipWith(&[("a.xml", b"payload")]), 0, declared);
    let p = writeTemp(&format!("declared-{declared}-{tag}.zip"), &bytes);
    let zf = ZipFile::from_path(&p).expect("改尺寸后的包仍应能打开（打开期只读中央目录）");
    (p, zf)
}

/// `getEntry` 是否被尺寸门禁拒绝（Java `throw FileDamagedException` ⇄ Rust `panic!`）。
fn guardRejects(zf: &ZipFile, name: &str) -> bool {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        OpcrPackage::getEntry(zf, Some(name))
    }));
    std::panic::set_hook(prev);
    r.is_err()
}

// ── T01 声明 0 ⇒ 通过（COV-1） ──────────────────────────────────────────
#[test]
fn t01_declared_zero_passes() {
    let (_p, zf) = packageWithDeclaredSize("t01", 0);
    let e = OpcrPackage::getEntry(&zf, Some("a.xml")).expect("0 尺寸必须通过");
    assert_eq!(e.getSize(), 0);
}

// ── T02 声明 2³⁰−1 ⇒ 通过（COV-2） ─────────────────────────────────────
#[test]
fn t02_declared_2pow30_minus_1_passes() {
    let (_p, zf) = packageWithDeclaredSize("t02", 1073741823);
    let e = OpcrPackage::getEntry(&zf, Some("a.xml")).expect("2³⁰−1 必须通过");
    assert_eq!(e.getSize(), 1073741823);
}

// ── T03 **恰为 2³⁰ ⇒ 通过**（`>` 不是 `>=`）（COV-3） ───────────────────
#[test]
fn t03_declared_exactly_2pow30_passes() {
    let (_p, zf) = packageWithDeclaredSize("t03", 1073741824);
    let e = OpcrPackage::getEntry(&zf, Some("a.xml")).expect("恰为 MAX_SIZE_ENTRY 必须通过（Java 用 `>`）");
    assert_eq!(e.getSize(), 1073741824);
    assert!(!guardRejects(&zf, "a.xml"), "恰为边界值不得被拒");
}

// ── T04 2³⁰+1 ⇒ 被拒（COV-4） ──────────────────────────────────────────
#[test]
fn t04_declared_2pow30_plus_1_rejected() {
    let (_p, zf) = packageWithDeclaredSize("t04", 1073741825);
    assert!(guardRejects(&zf, "a.xml"), "2³⁰+1 必须被拒（Java throw / Rust panic，归 s0_14 改 Err）");
}

// ── T05 u32 上限 ⇒ 被拒（无溢出/wrap）（COV-5） ─────────────────────────
#[test]
fn t05_declared_u32_max_rejected() {
    // 中央目录尺寸字段是 u32（`i64::MAX` 无法编码）⇒ 取可表示的最大值 4294967295
    let (_p, zf) = packageWithDeclaredSize("t05", u32::MAX);
    assert!(guardRejects(&zf, "a.xml"), "u32 上限必须被拒且不得 wrap");
    assert_eq!(zf.getEntry("a.xml").expect("元数据可取").getSize(), 4294967295);
}

// ── T06 `None` 名 ⇒ `None` 且不做尺寸判定（COV-7） ──────────────────────
#[test]
fn t06_none_name_returns_none() {
    let (_p, zf) = packageWithDeclaredSize("t06", u32::MAX);
    assert!(OpcrPackage::getEntry(&zf, None).is_none(), "None 名必须直接 None");
    assert!(!guardRejects(&zf, ""), "空名不命中 ⇒ `None`，不得因尺寸被拒");
}

// ── T07 名字未命中 ⇒ `None`（三连试探全失败）（COV-8） ──────────────────
#[test]
fn t07_name_not_found_returns_none() {
    let (_p, zf) = packageWithDeclaredSize("t07", u32::MAX);
    assert!(OpcrPackage::getEntry(&zf, Some("nope/missing.xml")).is_none());
}

// ── T08 试探 2（`/`→`\`）生效（COV-9） ─────────────────────────────────
#[test]
fn t08_backslash_trial_hits() {
    let p = writeTemp("backslash.zip", &zipWith(&[("ppt\\a.xml", b"x")]));
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let e = OpcrPackage::getEntry(&zf, Some("ppt/a.xml")).expect("试探 2 必须命中");
    assert_eq!(e.getName(), "ppt\\a.xml");
}

// ── T09 试探 3（`toLowerCase`）生效（COV-10，★判据方向已更正） ──────────
#[test]
fn t09_lowercase_trial_hits() {
    // ★Java `:232` 是 `entryName.toLowerCase()` —— 小写化的是**查询名**，不是 entry 名 ⇒
    //   试探 3 只在「entry 以**小写**存储 + 查询名含大写」时命中（旧文档把两者写反了）。
    let p = writeTemp("lower-stored.zip", &zipWith(&[("ppt/a.xml", b"x")]));
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let e = OpcrPackage::getEntry(&zf, Some("PPT/A.XML")).expect("试探 3 必须命中");
    assert_eq!(e.getName(), "ppt/a.xml", "命中后返回的是**小写存储**的那个 entry");

    // 反证：entry 存大写 + 查询小写 ⇒ 三次试探全失败（`PPT/A.XML` 不在小写化路径上）
    let p2 = writeTemp("upper-stored.zip", &zipWith(&[("PPT/A.XML", b"x")]));
    let zf2 = ZipFile::from_path(&p2).expect("合成包应打开成功");
    assert!(
        OpcrPackage::getEntry(&zf2, Some("ppt/a.xml")).is_none(),
        "小写查询不命中大写 entry（Java 侧同样：只小写化查询名，不做大小写无关匹配）"
    );
    assert!(
        OpcrPackage::getEntry(&zf2, Some("PPT/A.XML")).is_some(),
        "精确名必须命中"
    );
}

// ── T10 试探 1 命中时不误报（顺序语义的可观测部分）（COV-11） ────────────
#[test]
fn t10_exact_trial_wins() {
    let p = writeTemp(
        "exact.zip",
        &zipWith(&[("ppt/a.xml", b"lower"), ("PPT/A.XML", b"upper")]),
    );
    let zf = ZipFile::from_path(&p).expect("合成包应打开成功");
    let e = OpcrPackage::getEntry(&zf, Some("ppt/a.xml")).expect("精确名必须命中");
    assert_eq!(e.getName(), "ppt/a.xml", "试探 1 命中即返回，不得落到试探 3");
}

// ── T11 下层 `ZipFile::getEntry` 不受尺寸门禁影响（COV-17，不扩散） ──────
#[test]
fn t11_zipfile_getentry_not_guarded() {
    let (_p, zf) = packageWithDeclaredSize("t11", u32::MAX);
    assert!(
        zf.getEntry("a.xml").is_some(),
        "尺寸门禁只在上层 `OpcrPackage::getEntry`；`ZipFile::getEntry` 不得拦"
    );
}

// ── T12 真语料全部 entry 都不超限（COV-15，不误拒） ────────────────────
#[test]
fn t12_real_corpus_entries_pass_guard() {
    for rel in [C1, C2] {
        let zf = ZipFile::from_path(&corpus(rel)).expect("语料应打开成功");
        for entry in zf.entries() {
            let name = entry.getName();
            assert!(
                !guardRejects(&zf, &name),
                "{} 的 entry `{}`（size={}）不得被尺寸门禁拒绝",
                rel,
                name,
                entry.getSize()
            );
            assert!(
                OpcrPackage::getEntry(&zf, Some(&name)).is_some(),
                "{} 的 entry `{}` 必须可取",
                rel,
                name
            );
        }
    }
}

// ── T13 纯函数性：同输入重复调用结果相同（COV-14） ─────────────────────
#[test]
fn t13_pure_function_repeatable() {
    let (_p, zf) = packageWithDeclaredSize("t13a", 1073741825);
    for _ in 0..3 {
        assert!(guardRejects(&zf, "a.xml"));
    }
    let (_p2, zf2) = packageWithDeclaredSize("t13b", 1073741824);
    for _ in 0..3 {
        assert!(OpcrPackage::getEntry(&zf2, Some("a.xml")).is_some());
    }
}
