// `s0_06_function_mod_wiring` —— `mod.rs` 声明与目录实际文件集的一致性（孤儿接线）测试镜像。
// 覆盖矩阵与判据见 `features/s0-02-compile-surface/functions/s0_06_function_mod_wiring/FUNCTION.md` §3。
// 运行：cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test s0_06_function_mod_wiring
//
// ★口径（照抄 FUNCTION.md §3.3）：
//   1. 「接线」的判据必须是**编译级**（T06 用类型绑定），不能只做文本 `contains`（`// pub mod X;` 也含子串）。
//   2. 文本断言**必须先剥注释行**（否则注释态声明会被误判为「已声明」）。
//   3. `todo!` 计数口径 = 字面出现次数（`matches("todo!").count()`），与 `grep -o 'todo!' | wc -l` 一致。
//   4. 越界项只断言「未被误判为必须接线」，不断言它们「不应接线」。
//
// ★与 FUNCTION.md §3.1/§3.4 的**登记偏离**（实测驱动，逐条可查）：
//   · T02/T03（`ooxfr/mod.rs` 含 `pub mod OxfPptRelations;`/`OxfStrictRelations;`）：按 §1.1 的分批裁定属 **W2.5**
//     （归 `s0_19_function_relation_constants`）。实测：接线 `OxfPptRelations` 立即产生 **113** 个 `OpcRelation` 未解析错误
//     （见 T11 清单）⇒ 本轮**登记不接线**，不给假绿断言。
//   · T05（`pub mod OxfrCorePart;`）：该文件除 `ElementStrHandler` 外还引用 `Date`
//     （定义在 `core-libs/stubs/src/java/util/Date.rs:3`），而 `kfc_alpha` 当前**不依赖** `stubs`
//     ⇒ 接线需改依赖图，归 `s0_36_function_app_part_parse`；本轮**登记不接线**。
//   · 已接线并达编译级判据的：`opcr/OpcrContentTypes.rs`（解锁 `s0-03`）+ `ooxfr/OxfrAppPart.rs`。
//   · 跨 crate 读取：kfc 侧 5 个孤儿用编译期 `include_str!`（文件缺失即编译失败，最强判据）；
//     产品侧 2 个（`ppt-core`）+ T08/T12/T13 用运行期读盘，路径由 `CARGO_MANIFEST_DIR` 向上找**哨兵** `libs/ppt-core`
//     推导（不写「往上数 N 层」的相对路径，避免 CWD/层数耦合）。

use std::path::{Path, PathBuf};

// ── kfc 侧源文本（编译期嵌入） ───────────────────────────────────────────
const OPCR_MOD_RS: &str = include_str!("../../../../../src/cn/wps/kfc/opcr/mod.rs");
const OOXFR_MOD_RS: &str = include_str!("../../../../../src/cn/wps/kfc/ooxfr/mod.rs");
const LIBCRATE_RS: &str = include_str!("../../../../../src/lib.rs");

// S0 面 5 个 kfc 侧孤儿（含未接线的两个 W2.5 + 一个待依赖的 OxfrCorePart）
const OPCR_CONTENT_TYPES_RS: &str = include_str!("../../../../../src/cn/wps/kfc/opcr/OpcrContentTypes.rs");
const OXF_PPT_RELATIONS_RS: &str = include_str!("../../../../../src/cn/wps/kfc/ooxfr/OxfPptRelations.rs");
const OXF_STRICT_RELATIONS_RS: &str = include_str!("../../../../../src/cn/wps/kfc/ooxfr/OxfStrictRelations.rs");
const OXFR_APP_PART_RS: &str = include_str!("../../../../../src/cn/wps/kfc/ooxfr/OxfrAppPart.rs");
const OXFR_CORE_PART_RS: &str = include_str!("../../../../../src/cn/wps/kfc/ooxfr/OxfrCorePart.rs");

// ── 编译级接线证明（COV-6）：未接线则本文件**编译失败** ─────────────────
use core_libs_kfc::cn::wps::kfc::opcr::OpcrContentTypes::OpcrContentTypes;
use core_libs_kfc::cn::wps::kfc::ooxfr::OxfrAppPart::OxfrAppPart;

/// S0 面 7 个孤儿（`FUNCTION.md` §2.1 冻结表，判据①的范围）。
const S0_ORPHANS_7: [&str; 7] = [
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/opcr/OpcrContentTypes.rs",
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfPptRelations.rs",
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfStrictRelations.rs",
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrAppPart.rs",
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrCorePart.rs",
    "libs/ppt-core/src/cn/wps/show/pptxr/util/PptxrFileDecrypt.rs",
    "libs/ppt-core/src/cn/wps/show/pptxr/docprops/PptxrCustom.rs",
];

/// 接线前冻结的 `todo!` 字面计数（§2.1；判据②要求接线后逐个相等）。
const TODO_BEFORE: [usize; 7] = [10, 110, 2, 19, 23, 5, 2];
const TODO_TOTAL_BEFORE: usize = 171;

/// ★越界清单（判据①的反证面）：**不得**被断言为「S0 面必须接线」。
const NON_S0_SCOPE: [&str; 3] = [
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xpxml/reader/DefaultElementHandler.rs",
    "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/memor4j/util/KmoUndoableList.rs",
    "libs/core-libs/kfc/kfc_alpha/.codeTrees/*.typeoracle.rs",
];

/// ★本轮实测的「接线暴露的编译错误」清单（判据③，COV-11）。
/// 字段：`未解析名`、`站点（文件:行:列）`、`归属 function`。
const UNRESOLVED_INVENTORY: [(&str, &str, &str); 6] = [
    ("ElementStrHandler", "opcr/OpcrContentTypes.rs:13:64", "s0_09_function_content_types_parse"),
    ("ElementStrHandler", "ooxfr/OxfrAppPart.rs:20:64", "s0_36_function_app_part_parse"),
    ("Date", "ooxfr/OxfrCorePart.rs:13:33", "s0_36_function_app_part_parse"),
    ("Date", "ooxfr/OxfrCorePart.rs:15:34", "s0_36_function_app_part_parse"),
    ("ElementStrHandler", "ooxfr/OxfrCorePart.rs:22:64", "s0_36_function_app_part_parse"),
    ("OpcRelation", "ooxfr/OxfPptRelations.rs（实测 113 处）", "s0_19_function_relation_constants"),
];

// ── 工具 ────────────────────────────────────────────────────────────────
/// 剥掉**整行**注释（`//` 或 `///` 开头），用于文本声明断言（§3.3 规则 2）。
fn strip_line_comments(s: &str) -> String {
    s.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// 剥掉 `/* … */` 块（含 `/* DEFERRED_NON_FUNCTION … */`），用于「真引用」类断言。
fn strip_block_comments(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("/*") {
        out.push_str(&rest[..i]);
        match rest[i..].find("*/") {
            Some(j) => rest = &rest[i + j + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

/// 仓库根：从 `CARGO_MANIFEST_DIR` 向上找含哨兵 `libs/ppt-core` 的目录。
fn repo_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if p.join("libs/ppt-core").is_dir() {
            return p;
        }
        p = p.parent().expect("repo root 未找到（哨兵 libs/ppt-core）").to_path_buf();
    }
}

fn read_repo_file(rel: &str) -> String {
    let p = repo_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("读不到 {}: {e}", p.display()))
}

/// 递归收集目录下的文件（跳过 `.git`/`target`/`.codeTrees` 由调用方决定）。
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

fn count_todo(s: &str) -> usize {
    s.matches("todo!").count()
}

// ── T01 / T04：接线的文本面（剥注释后） ────────────────────────────────
#[test]
fn t01_opcr_mod_declares_content_types() {
    let body = strip_line_comments(OPCR_MOD_RS);
    assert!(
        body.contains("pub mod OpcrContentTypes;"),
        "opcr/mod.rs 必须声明 OpcrContentTypes（剥注释后）：\n{OPCR_MOD_RS}"
    );
}

#[test]
fn t04_ooxfr_mod_declares_app_part() {
    let body = strip_line_comments(OOXFR_MOD_RS);
    assert!(
        body.contains("pub mod OxfrAppPart;"),
        "ooxfr/mod.rs 必须声明 OxfrAppPart（剥注释后）：\n{OOXFR_MOD_RS}"
    );
}

// ── T06：编译级接线（COV-6） ───────────────────────────────────────────
#[test]
fn t06_type_references_compile() {
    // 类型路径可解析 = 已进编译面（未接线时本文件编译失败）
    let _: Option<OpcrContentTypes> = None;
    let _: Option<OxfrAppPart> = None;
    // ★`OpcrContentTypes` 是**无字段结构体**（ZST）⇒ 不能用 `size_of::<T>() > 0` 判「存在」
    //   （那会恒假）；改用「具名类型的绑定 + 函数项绑定」。
    let _ctor: fn() -> OpcrContentTypes = OpcrContentTypes::new;
    let _ctor2: fn() -> OxfrAppPart = OxfrAppPart::new;
    assert!(std::mem::size_of::<Option<OpcrContentTypes>>() > 0);
}

// ── T07：7 个孤儿文件的存在性 ──────────────────────────────────────────
#[test]
fn t07_orphan_files_exist_non_empty() {
    // kfc 侧 5 个：编译期 `include_str!`（文件缺失即本测试编译失败）
    for (name, s) in [
        ("OpcrContentTypes.rs", OPCR_CONTENT_TYPES_RS),
        ("OxfPptRelations.rs", OXF_PPT_RELATIONS_RS),
        ("OxfStrictRelations.rs", OXF_STRICT_RELATIONS_RS),
        ("OxfrAppPart.rs", OXFR_APP_PART_RS),
        ("OxfrCorePart.rs", OXFR_CORE_PART_RS),
    ] {
        assert!(!s.is_empty(), "kfc 侧孤儿 {name} 为空");
    }
    // 产品侧 2 个：运行期读盘（跨 crate，编译期相对路径不稳健）
    for rel in &S0_ORPHANS_7[5..] {
        let s = read_repo_file(rel);
        assert!(!s.is_empty(), "产品侧孤儿 {rel} 为空");
    }
}

// ── T08：悬空声明已处理（剥注释后不含未注释声明） ──────────────────────
#[test]
fn t08_dangling_decl_handled() {
    let rel = "libs/ppt-core/src/cn/wps/show/pptxr/util/mod.rs";
    let s = read_repo_file(rel);
    let body = strip_line_comments(&s);
    assert!(
        !body.contains("pub mod ModularGroupParseUtils;"),
        "{rel} 剥注释后仍含未注释的 ModularGroupParseUtils 声明"
    );
    // 反证：该文件确实存在注释态声明（否则本测试没测到东西）
    assert!(
        s.contains("ModularGroupParseUtils"),
        "{rel} 里既无声明也无注释态声明 ⇒ 本用例失去意义"
    );
}

// ── T09：越界反证（判据①的范围限定） ──────────────────────────────────
#[test]
fn t09_non_s0_orphans_not_required() {
    for x in NON_S0_SCOPE {
        assert!(
            !S0_ORPHANS_7.iter().any(|o| *o == x),
            "越界项 `{x}` 不得进入 S0 面 7 元组"
        );
    }
    assert_eq!(S0_ORPHANS_7.len(), 7, "S0 面判据范围恒为 7");
    // 实测事实：这些越界项确实存在/或为整目录未接线 ⇒ 只登记，不判 FAIL
    let default_handler =
        "libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xpxml/reader/DefaultElementHandler.rs";
    let body = read_repo_file("libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xpxml/reader/mod.rs");
    assert!(
        !strip_line_comments(&body).contains("pub mod DefaultElementHandler;"),
        "{default_handler} 按 §1.6 实测应仍是漏声明的越界孤儿"
    );
}

// ── T10 / T14：`todo!` 计数守恒（判据②） ───────────────────────────────
#[test]
fn t10_todo_counts_conserved() {
    let kfc_counts = [
        count_todo(OPCR_CONTENT_TYPES_RS),
        count_todo(OXF_PPT_RELATIONS_RS),
        count_todo(OXF_STRICT_RELATIONS_RS),
        count_todo(OXFR_APP_PART_RS),
        count_todo(OXFR_CORE_PART_RS),
    ];
    assert_eq!(
        kfc_counts,
        TODO_BEFORE[..5],
        "kfc 侧 5 个孤儿的 `todo!` 字面计数必须与接线前冻结表相等（只接线、不改体）"
    );
    let product_counts: Vec<usize> = S0_ORPHANS_7[5..]
        .iter()
        .map(|rel| count_todo(&read_repo_file(rel)))
        .collect();
    assert_eq!(
        product_counts,
        TODO_BEFORE[5..],
        "产品侧 2 个孤儿的 `todo!` 计数必须与冻结表相等"
    );
    let total: usize = kfc_counts.iter().sum::<usize>() + product_counts.iter().sum::<usize>();
    assert_eq!(total, TODO_TOTAL_BEFORE, "S0 面孤儿 `todo!` 合计守恒（171）");
}

#[test]
fn t14_counts_stable_across_rounds() {
    let round = || {
        let mut v = vec![
            count_todo(OPCR_CONTENT_TYPES_RS),
            count_todo(OXF_PPT_RELATIONS_RS),
            count_todo(OXF_STRICT_RELATIONS_RS),
            count_todo(OXFR_APP_PART_RS),
            count_todo(OXFR_CORE_PART_RS),
        ];
        for rel in &S0_ORPHANS_7[5..] {
            v.push(count_todo(&read_repo_file(rel)));
        }
        v
    };
    assert_eq!(round(), round(), "两轮计数必须相同（确定性）");
}

// ── T11：接线暴露的编译错误清单（判据③） ──────────────────────────────
#[test]
fn t11_unresolved_inventory_registered() {
    assert!(!UNRESOLVED_INVENTORY.is_empty(), "清单不得为空");
    for (name, site, owner) in UNRESOLVED_INVENTORY {
        assert!(!name.trim().is_empty(), "unresolved_name 不得为空");
        assert!(site.contains(':') || site.contains('（'), "site 必须可定位：{site}");
        assert!(owner.starts_with("s0_"), "owner_function 必须是 function_id：{owner}");
    }
    // 4 类未解析名的覆盖面（判据③要求「逐条登记」）
    let names: Vec<&str> = UNRESOLVED_INVENTORY.iter().map(|r| r.0).collect();
    for expect in ["ElementStrHandler", "Date", "OpcRelation"] {
        assert!(names.contains(&expect), "清单缺少 `{expect}` 类");
    }
    // ★这些名字**不是「不存在」**：都在本 crate 已声明的模块里 ⇒ 差异面是「裸名可达性」，
    //   由 crate 根重导出解决（偏离登记 S0M68）。
    assert!(
        strip_line_comments(LIBCRATE_RS).contains("pub use cn::wps::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;"),
        "crate 根必须重导出 ElementStrHandler（S0M68 裁定）"
    );
    assert!(
        strip_line_comments(LIBCRATE_RS).contains("pub use cn::wps::kfc::opcr::OpcRelation::OpcRelation;"),
        "crate 根必须重导出 OpcRelation（S0M68 裁定）"
    );
}

// ── T12：隐式通道反证（`#[path]` / `include!` 各 0 命中） ──────────────
#[test]
fn t12_no_implicit_channel() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    walk(&src, &mut files);
    let mut hits = Vec::new();
    for f in &files {
        if f.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let in_code_trees = f.components().any(|c| c.as_os_str() == ".codeTrees");
        let Ok(s) = std::fs::read_to_string(f) else { continue };
        let body = strip_block_comments(&strip_line_comments(&s));
        if body.contains("#[path") || body.contains("include!") {
            hits.push((f.clone(), in_code_trees));
        }
    }
    let unexpected: Vec<_> = hits.iter().filter(|(_, ct)| !ct).collect();
    assert!(
        unexpected.is_empty(),
        "`src/` 内（非 .codeTrees）不得出现 `#[path]`/`include!`（唯一通道必须是 `mod` 声明）：{unexpected:?}"
    );
    // `.codeTrees/*.typeoracle.rs` 是 type-oracle 产物（设计上不编译）⇒ 只登记，不判 FAIL
    // （但它们不应进入「孤儿」统计，见 COV-9）
}

// ── T13：`ModularGroupParseUtils.rs` 不存在性 ──────────────────────────
#[test]
fn t13_modular_group_parse_utils_absent() {
    let libs = repo_root().join("libs");
    let mut files = Vec::new();
    walk(&libs, &mut files);
    let found: Vec<_> = files
        .iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) == Some("ModularGroupParseUtils.rs"))
        .collect();
    assert!(found.is_empty(), "`libs/` 内不得存在 ModularGroupParseUtils.rs：{found:?}");
}
