// `s0_04_function_zip_name_guard` —— 测试镜像（⇄ `src/cn/wps/kfc/opcr/OpcrPackage.rs` 的名字门禁面）。
// 覆盖矩阵与 PASS/FAIL 判据见 `features/s0-01-zip-source/functions/s0_04_function_zip_name_guard/FUNCTION.md` §3。
// 运行：cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test s0_04_function_zip_name_guard
//
// Java 锚点：`kfc/opcr/OpcrPackage.java:26`（`INAVALID_NAME = "../"`）、`:243-247`（`checkEntryNameLegal`：
// `if (entryName.contains("../")) throw new FileDamagedException();`）。
//
// ★签名口径（既定裁定）：Java 用 `throw`，Rust 现状用 `panic!`；**panic→Result 的签名整改一次性归 `s0_14`**
//   ⇒ 本轮断言「拒绝 = panic」的**现状**，并把 §3.2 里以 `Err(kind=…)` 表述的行登记为「待 s0_14」。

use core_libs_kfc::kfc::opcr::OpcrPackage::OpcrPackage;

/// 调用名字门禁并返回 `true` = 被拒（Java `throw FileDamagedException`）。
fn rejected(name: &str) -> bool {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(|| OpcrPackage::checkEntryNameLegal(name));
    std::panic::set_hook(prev);
    match r {
        Ok(()) => false,
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_default();
            assert!(
                msg.contains("FileDamagedException"),
                "拒绝必须带 Java 异常语义（FileDamagedException），实测：{msg}"
            );
            true
        }
    }
}

// ── T01 合法名通过（COV-1） ──────────────────────────────────────────────
#[test]
fn t01_legal_name_passes() {
    assert!(!rejected("ppt/presentation.xml"));
    assert!(!rejected("ppt/slides/slide1.xml"));
}

// ── T02 `../` 被拒（COV-2） ─────────────────────────────────────────────
#[test]
fn t02_parent_traversal_rejected() {
    assert!(rejected("../evil.xml"));
    assert!(rejected("ppt/../../etc/passwd"));
}

// ── T03 `word/..`（`..` 后无 `/`）通过（COV-3） ─────────────────────────
#[test]
fn t03_dotdot_without_trailing_slash_passes() {
    // Java `:245` 只查子串 `"../"` ⇒ 末段 `..` 不算穿越
    assert!(!rejected("word/.."));
    assert!(!rejected(".."));
}

// ── T04 反斜杠形态通过（只查 `/`）（COV-4） ──────────────────────────────
#[test]
fn t04_backslash_variant_passes() {
    assert!(!rejected("..\\evil.xml"), "Java 只查 `../`，反斜杠形态不拦");
    assert!(!rejected("ppt\\..\\evil.xml"));
}

// ── T05 子串命中即拒（`.../x`）（COV-5） ─────────────────────────────────
#[test]
fn t05_substring_hit_rejected() {
    // `".../x"` 的第 1..4 字符恰为 `"../"` ⇒ Java 子串语义同样拒绝
    assert!(rejected(".../x"));
}

// ── T06 空名通过（COV-6） ───────────────────────────────────────────────
#[test]
fn t06_empty_name_passes() {
    assert!(!rejected(""));
}

// ── T07 前导斜杠穿越被拒（COV-7） ────────────────────────────────────────
#[test]
fn t07_leading_slash_traversal_rejected() {
    assert!(rejected("/../x"));
}

// ── T08 方括号名不改写通过（COV-8） ─────────────────────────────────────
#[test]
fn t08_bracket_name_passes() {
    assert!(!rejected("[Content_Types].xml"));
    assert!(!rejected("_rels/.rels"));
}

// ── T09 目录名（尾 `/`）通过（COV-9） ───────────────────────────────────
#[test]
fn t09_directory_name_passes() {
    assert!(!rejected("ppt/slides/"));
}

// ── T10 纯函数性：同输入重复调用结果相同（COV-10） ──────────────────────
#[test]
fn t10_pure_function_repeatable() {
    for _ in 0..3 {
        assert!(rejected("../evil.xml"));
        assert!(!rejected("ppt/presentation.xml"));
    }
}

// ── T11 边界表：一次覆盖判据集合（COV-1..COV-9 的边界汇总） ─────────────
#[test]
fn t11_boundary_table() {
    let expects: &[(&str, bool)] = &[
        ("ppt/presentation.xml", false),
        ("ppt/slides/slide1.xml", false),
        ("../evil.xml", true),
        ("word/..", false),
        ("..", false),
        ("..\\evil.xml", false),
        ("ppt\\..\\evil.xml", false),
        (".../x", true),
        ("", false),
        ("/../x", true),
        ("[Content_Types].xml", false),
        ("ppt/slides/", false),
        ("ppt/media/image1.png", false),
        ("a/../b", true),
    ];
    for (name, want) in expects {
        assert_eq!(rejected(name), *want, "名字 `{name}` 的判据不符（期望被拒={want}）");
    }
}
