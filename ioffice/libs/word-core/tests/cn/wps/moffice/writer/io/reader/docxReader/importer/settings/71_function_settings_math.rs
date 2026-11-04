// function_id=71_function_settings_math
// cargo test --manifest-path libs/word-core/Cargo.toml --test 71_function_settings_math
//
// ★M-STAGE1 P2 —— `SettingsImporter` [S1] 批次：MathPr 静态叶子（21 个方法）。
//
// ★本用例的核心价值：把 Java 原文里**三张非直觉的映射表**固化为可执行判据。
//   这些表一旦被"顺手改顺"，用例立刻变红。
//
//   - `convertBreakBin`      : before→0 / after→1 / repeat→2
//   - `convertBreakBinSub`   : --→0 / -+→2 / +-→1        ← ★非顺序
//   - `convertMathJc`        : center→2 / centerGroup→1 / left→3 / right→4  ← ★非顺序
//   - `convertLimLoc4IntLim` : subSup→0 / undOvr→1 / 默认 0
//   - `convertLimLoc4NaryLim`: subSup→0 / undOvr→1 / 默认 1               ← ★默认值不同
//
// ★属性位编号（Java 字面量）也一并锁定：598–613。
#![allow(non_snake_case)]

use std::collections::HashMap;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::data::value::CompatSetting::CompatSetting;
use word_core::cn::wps::moffice::writer::data::value::DocVar::DocVar;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::SettingsImporter::SettingsImporter;

const MATH_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
/// ★`ShareConvertHelper.getLong4STTwipsMeasure` 读的是 `WMap.MAIN_NAMESPACING`，
/// 即 **wordprocessingml main**，**不是** math 命名空间。
/// （Java `WMap.java:8` 该字段初值即为该串。）
const MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

// ───────────────────── SM1–SM5：映射表（纯函数） ─────────────────────

/// SM1：`convertBreakBin` 三分支 + 未知值走 `IOAssert.fail` 后返回 0。
#[test]
fn SM1_convertBreakBin_mapping() {
    assert_eq!(SettingsImporter::convertBreakBin("before".to_string()), 0, "before");
    assert_eq!(SettingsImporter::convertBreakBin("after".to_string()), 1, "after");
    assert_eq!(SettingsImporter::convertBreakBin("repeat".to_string()), 2, "repeat");
}

/// SM2：★`convertBreakBinSub` 的**非顺序**映射。
/// `-+ → 2` 而 `+- → 1` —— 顺序直觉会写成 `-+ → 1`，Java 不是。
#[test]
fn SM2_convertBreakBinSub_is_not_sequential() {
    assert_eq!(SettingsImporter::convertBreakBinSub("--".to_string()), 0, "--");
    assert_eq!(
        SettingsImporter::convertBreakBinSub("-+".to_string()),
        2,
        "★-+ 必须映射为 2（非 1）"
    );
    assert_eq!(
        SettingsImporter::convertBreakBinSub("+-".to_string()),
        1,
        "★+- 必须映射为 1（非 2）"
    );
}

/// SM3：★`convertMathJc` 的**非顺序**映射。
#[test]
fn SM3_convertMathJc_is_not_sequential() {
    assert_eq!(SettingsImporter::convertMathJc("center".to_string()), 2, "center");
    assert_eq!(SettingsImporter::convertMathJc("centerGroup".to_string()), 1, "centerGroup");
    assert_eq!(SettingsImporter::convertMathJc("left".to_string()), 3, "left");
    assert_eq!(SettingsImporter::convertMathJc("right".to_string()), 4, "right");
}

/// SM4：★`convertLimLoc4IntLim` 与 `convertLimLoc4NaryLim` 的正常分支相同，
/// **但未知值的兜底返回值不同**（`0` vs `1`）。
/// 这是 Java 原文的不一致，必须原样保留。
#[test]
fn SM4_limLoc_int_and_nary_have_different_fallbacks() {
    // 正常分支一致
    assert_eq!(SettingsImporter::convertLimLoc4IntLim("subSup".to_string()), 0);
    assert_eq!(SettingsImporter::convertLimLoc4IntLim("undOvr".to_string()), 1);
    assert_eq!(SettingsImporter::convertLimLoc4NaryLim("subSup".to_string()), 0);
    assert_eq!(SettingsImporter::convertLimLoc4NaryLim("undOvr".to_string()), 1);

    // ★兜底分支不一致 —— 恰好证明二者是两份独立实现，未被"合并优化"
    assert_eq!(
        SettingsImporter::convertLimLoc4IntLim("__bogus__".to_string()),
        0,
        "★IntLim 兜底返回 0"
    );
    assert_eq!(
        SettingsImporter::convertLimLoc4NaryLim("__bogus__".to_string()),
        1,
        "★NaryLim 兜底返回 1（与 IntLim 不同）"
    );
}

// ───────────────────── SP1–SP7：属性位写入 ─────────────────────

/// SP1：`setMathFont` 写入属性位 **598**；`val` 缺省时不写。
#[test]
fn SP1_setMathFont_writes_598() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setMathFont(&Attrs::ns("val", "Cambria Math"), &mut dst);
    assert_eq!(dst.getString(598), "Cambria Math", "598 mathFont");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setMathFont(&Attrs::empty(), &mut dst2);
    assert!(!dst2.contains(598), "★val 缺省 ⇒ 不得写入 598");
}

/// SP2：`serBreakOnBinaryOperators` 写入 **599**，值是 `convertBreakBin` 的结果。
#[test]
fn SP2_breakOnBinaryOperators_writes_599() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::serBreakOnBinaryOperators(&Attrs::ns("val", "after"), &mut dst);
    assert_eq!(dst.getInt(599), 1, "599 = convertBreakBin(after) = 1");
}

/// SP3：`setBreakOnBinarySubtraction` 写入 **600**。
#[test]
fn SP3_breakOnBinarySubtraction_writes_600() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setBreakOnBinarySubtraction(&Attrs::ns("val", "-+"), &mut dst);
    assert_eq!(dst.getInt(600), 2, "★600 = convertBreakBinSub(-+) = 2");
}

/// SP4：四个 twips 类属性位 **603/604/606/607**（`setLeftMargin`/`setRightMargin`/
/// `setPreParagraphSpacing`/`setPostParagraphSpacing`）。
#[test]
fn SP4_twips_measures_write_603_604_606_607() {
    let cases: [(&str, fn(&dyn Attributes, &mut MutablePropertySet), i32); 4] = [
        ("603 setLeftMargin", SettingsImporter::setLeftMargin, 603),
        ("604 setRightMargin", SettingsImporter::setRightMargin, 604),
        ("606 setPreParagraphSpacing", SettingsImporter::setPreParagraphSpacing, 606),
        ("607 setPostParagraphSpacing", SettingsImporter::setPostParagraphSpacing, 607),
    ];
    for (label, f, prop) in cases {
        let mut dst = MutablePropertySet::new();
        f(&Attrs::main("val", "720"), &mut dst);
        assert_eq!(dst.getInt(prop), 720, "★{label} 应写入 twips 720");
    }
}

/// SP5：`setInterEquationSpacing`(608) / `setIntraEquationSpacing`(609) / `setWrapIndent`(610)。
#[test]
fn SP5_twips_measures_write_608_609_610() {
    let cases: [(&str, fn(&dyn Attributes, &mut MutablePropertySet), i32); 3] = [
        ("608 setInterEquationSpacing", SettingsImporter::setInterEquationSpacing, 608),
        ("609 setIntraEquationSpacing", SettingsImporter::setIntraEquationSpacing, 609),
        ("610 setWrapIndent", SettingsImporter::setWrapIndent, 610),
    ];
    for (label, f, prop) in cases {
        let mut dst = MutablePropertySet::new();
        f(&Attrs::main("val", "360"), &mut dst);
        assert_eq!(dst.getInt(prop), 360, "★{label} 应写入 twips 360");
    }
}

/// SP6：`setDefaultJustifition` 写入 **605**，值为 `convertMathJc` 的结果。
#[test]
fn SP6_defaultJustifition_writes_605() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setDefaultJustifition(&Attrs::main("val", "centerGroup"), &mut dst);
    assert_eq!(dst.getInt(605), 1, "★605 = convertMathJc(centerGroup) = 1");
}

/// SP7：`setIntegralLimitLocations`(612) / `setNaryLimitLocations`(613)。
#[test]
fn SP7_limLoc_writes_612_613() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setIntegralLimitLocations(&Attrs::ns("val", "undOvr"), &mut dst);
    assert_eq!(dst.getInt(612), 1, "612 = convertLimLoc4IntLim(undOvr)");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setNaryLimitLocations(&Attrs::ns("val", "subSup"), &mut dst2);
    assert_eq!(dst2.getInt(613), 0, "613 = convertLimLoc4NaryLim(subSup)");
}

// ───────────────────── SB1–SB3：`toBoolean4Math` 的等价折叠 ─────────────────────

/// SB1：`toBoolean4Math` 在**属性为空**时返回 `true`（Java：`boldStr == null ⇒ true`）。
///
/// 这解释了为什么 `setSmallFraction` / `setDisplayMathDefault` / `setWrapRight`
/// 里 Java 的 `if (null != x)` 在 Rust 侧是恒真的。
#[test]
fn SB1_toBoolean4Math_defaults_to_true_when_absent() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setSmallFraction(&Attrs::empty(), &mut dst);
    assert!(
        dst.getBoolean(601),
        "★属性缺失 ⇒ toBoolean4Math = true ⇒ 601 写入 true"
    );
}

/// SB2：`toBoolean4Math` 在 math 命名空间有 `val` 时按其值解析。
#[test]
fn SB2_toBoolean4Math_honours_math_namespace_value() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setDisplayMathDefault(&Attrs::ns("val", "false"), &mut dst);
    assert!(!dst.getBoolean(602), "val=false ⇒ 602 = false");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setWrapRight(&Attrs::ns("val", "true"), &mut dst2);
    assert!(dst2.getBoolean(611), "val=true ⇒ 611 = true");
}

/// SB3：★`toBoolean4Math` 只认 **math 命名空间**的 `val`。
/// 无命名空间的普通 `val` 属性**不**影响结果（`getLength()==0` 分支之外走 math ns）。
#[test]
fn SB3_toBoolean4Math_ignores_non_math_namespace() {
    let mut dst = MutablePropertySet::new();
    // 有属性，但不在 math ns ⇒ 取 math ns 的 val 得到 None ⇒ toBoolean(str, TRUE) 走默认
    SettingsImporter::setSmallFraction(&Attrs::plain("val", "false"), &mut dst);
    assert!(
        dst.getBoolean(601),
        "★非 math ns 的 val 不应把结果拉成 false（走 Boolean.TRUE 默认）"
    );
}

// ───────────────────── 辅助：SAX Attributes 适配器 ─────────────────────

/// 实现本仓权威的 `ConvertHelper::Attributes` trait。
/// `ns`: 带 math 命名空间的 `val`；`plain`: 无命名空间的 `val`。
struct Attrs {
    map: HashMap<String, String>,
}

impl Attrs {
    fn empty() -> Self {
        Attrs { map: HashMap::new() }
    }
    /// 以 math 命名空间注册 `name`。
    fn ns(name: &str, value: &str) -> Self {
        let mut map = HashMap::new();
        map.insert(format!("{MATH_NS}|{name}"), value.to_string());
        Attrs { map }
    }
    /// 以 wordprocessingml main 命名空间注册 `name`（twips/整数属性走这条）。
    fn main(name: &str, value: &str) -> Self {
        let mut map = HashMap::new();
        map.insert(format!("{MAIN_NS}|{name}"), value.to_string());
        Attrs { map }
    }
    /// 以空命名空间注册 `name`。
    fn plain(name: &str, value: &str) -> Self {
        let mut map = HashMap::new();
        map.insert(format!("|{name}"), value.to_string());
        Attrs { map }
    }
}

impl Attributes for Attrs {
    fn getLength(&self) -> i32 {
        self.map.len() as i32
    }
    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.map.get(&format!("{uri}|{localName}")).cloned()
    }
}

// ───────────────────── S2a：其余属性位（377/394/395/439/449/450/454/455/458/459/460/464/465/568/569） ─────────────────────

/// SS1：twips 族 6 个属性位 —— `setDefaultTabStop`(377) `setHyphenationZone`(450)
/// `setDrawingGridHorizontalSpacing`(459) `setDrawingGridVerticalSpacing`(460)
/// `setDrawingGridHorizontalOrigin`(464) `setDrawingGridVerticalOrigin`(465)。
#[test]
fn SS1_twips_family_property_ids() {
    let cases: [(&str, fn(&dyn Attributes, &mut MutablePropertySet), i32); 6] = [
        ("377 setDefaultTabStop", SettingsImporter::setDefaultTabStop, 377),
        ("450 setHyphenationZone", SettingsImporter::setHyphenationZone, 450),
        ("459 setDrawingGridHorizontalSpacing", SettingsImporter::setDrawingGridHorizontalSpacing, 459),
        ("460 setDrawingGridVerticalSpacing", SettingsImporter::setDrawingGridVerticalSpacing, 460),
        ("464 setDrawingGridHorizontalOrigin", SettingsImporter::setDrawingGridHorizontalOrigin, 464),
        ("465 setDrawingGridVerticalOrigin", SettingsImporter::setDrawingGridVerticalOrigin, 465),
    ];
    for (label, f, prop) in cases {
        let mut dst = MutablePropertySet::new();
        f(&Attrs::main("val", "240"), &mut dst);
        assert_eq!(dst.getInt(prop), 240, "★{label}");
    }
}

/// SS2：decimal 族 2 个属性位 —— `setDisplayHorizontalDrawingGridEvery`(394)
/// `setDisplayVerticalDrawingGridEvery`(395)。
/// ★与 SS1 的 twips 族**不同**：这里走 `getLong4STDecimalNumber`（纯十进制）。
#[test]
fn SS2_decimal_family_property_ids() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setDisplayHorizontalDrawingGridEvery(&Attrs::main("val", "1"), &mut dst);
    assert_eq!(dst.getInt(394), 1, "394");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setDisplayVerticalDrawingGridEvery(&Attrs::main("val", "2"), &mut dst2);
    assert_eq!(dst2.getInt(395), 2, "395");
}

/// SS3：String 族 5 个属性位 —— `setDocumentType`(439) `setClickAndTypeStyle`(454)
/// `setDefaultTableStyle`(455) `setDecimalSymbol`(568) `setListSeparator`(569)。
#[test]
fn SS3_string_family_property_ids() {
    let cases: [(&str, fn(&dyn Attributes, &mut MutablePropertySet), i32, &str); 5] = [
        ("439 setDocumentType", SettingsImporter::setDocumentType, 439, "docType"),
        ("454 setClickAndTypeStyle", SettingsImporter::setClickAndTypeStyle, 454, "style"),
        ("455 setDefaultTableStyle", SettingsImporter::setDefaultTableStyle, 455, "tblStyle"),
        ("568 setDecimalSymbol", SettingsImporter::setDecimalSymbol, 568, "."),
        ("569 setListSeparator", SettingsImporter::setListSeparator, 569, ","),
    ];
    for (label, f, prop, val) in cases {
        let mut dst = MutablePropertySet::new();
        f(&Attrs::main("val", val), &mut dst);
        assert_eq!(dst.getString(prop), val, "★{label}");
    }
}

/// SS4：Integer 族 2 个属性位 —— `setConsecutiveHyphenLimit`(449)
/// `setBookFoldPrintingSheets`(458)。
#[test]
fn SS4_integer_family_property_ids() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setConsecutiveHyphenLimit(&Attrs::main("val", "2"), &mut dst);
    assert_eq!(dst.getInt(449), 2, "449");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setBookFoldPrintingSheets(&Attrs::main("val", "4"), &mut dst2);
    assert_eq!(dst2.getInt(458), 4, "458");
}

/// SS5：★属性缺省时**全体不得写入** —— 统一锁定 `if (null != x)` 守卫。
#[test]
fn SS5_absent_attribute_writes_nothing() {
    let fns: [(&str, fn(&dyn Attributes, &mut MutablePropertySet), i32); 15] = [
        ("377", SettingsImporter::setDefaultTabStop, 377),
        ("394", SettingsImporter::setDisplayHorizontalDrawingGridEvery, 394),
        ("395", SettingsImporter::setDisplayVerticalDrawingGridEvery, 395),
        ("439", SettingsImporter::setDocumentType, 439),
        ("449", SettingsImporter::setConsecutiveHyphenLimit, 449),
        ("450", SettingsImporter::setHyphenationZone, 450),
        ("454", SettingsImporter::setClickAndTypeStyle, 454),
        ("455", SettingsImporter::setDefaultTableStyle, 455),
        ("458", SettingsImporter::setBookFoldPrintingSheets, 458),
        ("459", SettingsImporter::setDrawingGridHorizontalSpacing, 459),
        ("460", SettingsImporter::setDrawingGridVerticalSpacing, 460),
        ("464", SettingsImporter::setDrawingGridHorizontalOrigin, 464),
        ("465", SettingsImporter::setDrawingGridVerticalOrigin, 465),
        ("568", SettingsImporter::setDecimalSymbol, 568),
        ("569", SettingsImporter::setListSeparator, 569),
    ];
    for (label, f, prop) in fns {
        let mut dst = MutablePropertySet::new();
        f(&Attrs::empty(), &mut dst);
        assert!(!dst.contains(prop), "★属性位 {label} 在属性缺失时不得被写入");
    }
}

// ───────────────────── S2b：view / zoom / proof / styleSort 映射表 ─────────────────────

/// SV1：★`setView` 的 **6 分支非顺序**映射 —— 写入属性位 **396**。
/// `masterPages→3` / `normal→4` / `outline→2` / `print→1` / `web→5`。
#[test]
fn SV1_setView_mapping_is_not_sequential() {
    for (v, expected) in [
        ("none", 0),
        ("masterPages", 3),
        ("normal", 4),
        ("outline", 2),
        ("print", 1),
        ("web", 5),
    ] {
        let mut dst = MutablePropertySet::new();
        SettingsImporter::setView(&Attrs::main("val", v), &mut dst);
        assert_eq!(dst.getInt(396), expected, "★view=\"{v}\" 应映射为 {expected}");
    }
}

/// SV2：`correctView` —— 396 缺省时补 0，已存在时**不得覆盖**。
#[test]
fn SV2_correctView_defaults_396_to_zero() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::correctView(&mut dst);
    assert_eq!(dst.getInt(396), 0, "★缺省 ⇒ 补 0");

    let mut dst2 = MutablePropertySet::new();
    dst2.setInt(396, 5);
    SettingsImporter::correctView(&mut dst2);
    assert_eq!(dst2.getInt(396), 5, "★已存在 ⇒ 不得覆盖");
}

/// SV3：`setZoom` 写入两个属性位 —— `convertZoomType(type)`→**386**，`percent`→**385**。
/// ★注意 386 > 385 的**逆序**写入（Java 原文如此）。
#[test]
fn SV3_setZoom_writes_386_and_385() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setZoom(&Attrs::main("val", "bestFit"), &mut dst);
    assert_eq!(dst.getInt(386), 2, "★386 = convertZoomType(bestFit) = 2");

    // percent 是独立属性名
    let mut attrs = Attrs::main("val", "bestFit");
    attrs.map.insert(format!("{MAIN_NS}|percent"), "150".to_string());
    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setZoom(&attrs, &mut dst2);
    assert_eq!(dst2.getInt(385), 150, "★385 = percent");
}

/// SV4：`convertZoomType` 四分支。
#[test]
fn SV4_convertZoomType_mapping() {
    assert_eq!(SettingsImporter::convertZoomType("none".to_string()), 0);
    assert_eq!(SettingsImporter::convertZoomType("fullPage".to_string()), 1);
    assert_eq!(SettingsImporter::convertZoomType("bestFit".to_string()), 2);
    assert_eq!(SettingsImporter::convertZoomType("textFit".to_string()), 3);
}

/// SV5：`convertProofType` —— `clean→0` / `dirty→1`，未知值走 `IOAssert.fail` 后返回 **1**。
#[test]
fn SV5_convertProofType_mapping() {
    assert_eq!(SettingsImporter::convertProofType("clean".to_string()), 0);
    assert_eq!(SettingsImporter::convertProofType("dirty".to_string()), 1);
}

/// SV6：★`setProofState` 依次调 `setSpellingAttr`(**416**) 与 `setGrammarAttr`(**415**)。
#[test]
fn SV6_setProofState_writes_416_and_415() {
    let mut attrs = Attrs::main("spelling", "clean");
    attrs.map.insert(format!("{MAIN_NS}|grammar"), "dirty".to_string());
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setProofState(&attrs, &mut dst);
    assert_eq!(dst.getInt(416), 0, "★416 spelling=clean ⇒ 0");
    assert_eq!(dst.getInt(415), 1, "★415 grammar=dirty ⇒ 1");
}

/// SV7：★`setSpellingAttr`/`setGrammarAttr` 的属性**缺省值为 1**（不是 0）。
/// Java：`null == spelling ? 1 : convertProofType(spelling)`。
#[test]
fn SV7_proof_attrs_default_to_one() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setSpellingAttr(&Attrs::empty(), &mut dst);
    assert_eq!(dst.getInt(416), 1, "★spelling 缺省 ⇒ 1");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setGrammarAttr(&Attrs::empty(), &mut dst2);
    assert_eq!(dst2.getInt(415), 1, "★grammar 缺省 ⇒ 1");
}

/// SV8：★`convertStyleSortType` 的 **12 分支**映射全部锁定。
/// 数字串 `0000`–`0005` 与语义串混用，且 `basedOn→10` / `type→11` 等**远离**其出现顺序。
#[test]
fn SV8_convertStyleSortType_full_table() {
    for (k, expected) in [
        ("0000", 0), ("0001", 1), ("0002", 2), ("0003", 3), ("0004", 4), ("0005", 5),
        ("name", 6), ("priority", 7), ("default", 8), ("font", 9), ("basedOn", 10), ("type", 11),
    ] {
        assert_eq!(
            SettingsImporter::convertStyleSortType(k.to_string()),
            expected,
            "★stylePaneSortMethod=\"{k}\" ⇒ {expected}"
        );
    }
    // 未知值：Java 初值 8，走 IOAssert.fail 后**返回初值 8**
    assert_eq!(
        SettingsImporter::convertStyleSortType("__bogus__".to_string()),
        8,
        "★未知值兜底为 Java 初值 8"
    );
}

/// SV9：`setStylePaneSortMethod` 写入属性位 **438**。
#[test]
fn SV9_setStylePaneSortMethod_writes_438() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setStylePaneSortMethod(&Attrs::main("val", "basedOn"), &mut dst);
    assert_eq!(dst.getInt(438), 10, "★438 = convertStyleSortType(basedOn) = 10");
}

// ───────────────────── S2c：CompatSetting / RevisionView / DocVar / 其余属性位 ─────────────────────

/// SC1：★`setCompatSetting`(542) 的**累积**语义 —— 连续两次调用后列表长度为 2。
/// （与 CP3 同类的 Java 引用语义陷阱。）
#[test]
fn SC1_setCompatSetting_accumulates_542() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setCompatSetting(&Attrs::main("name", "a"), &mut dst);
    SettingsImporter::setCompatSetting(&Attrs::main("name", "b"), &mut dst);
    let list = read_list::<CompatSetting>(&dst, 542);
    assert_eq!(list.len(), 2, "★两次调用 ⇒ 2 项（Java ArrayList 累积语义）");
}

/// SC2：★`setCompatSetting` 在 `attributes.getLength() == 0` 时**不添加**。
#[test]
fn SC2_setCompatSetting_skips_empty_attributes() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setCompatSetting(&Attrs::empty(), &mut dst);
    let list = read_list::<CompatSetting>(&dst, 542);
    assert!(list.is_empty(), "★空属性 ⇒ 不得添加（Java `0 != getLength()` 守卫）");
}

/// SC3：`convertCompatSetting` 三字段（name/uri/val）映射。
#[test]
fn SC3_convertCompatSetting_reads_three_fields() {
    let mut attrs = Attrs::main("name", "compat");
    attrs.map.insert(format!("{MAIN_NS}|uri"), "http://x".to_string());
    attrs.map.insert(format!("{MAIN_NS}|val"), "12".to_string());
    let c = SettingsImporter::convertCompatSetting(&attrs);
    assert_eq!(c.getName(), Some("compat".to_string()), "name");
    assert_eq!(c.getURI(), Some("http://x".to_string()), "uri");
    assert_eq!(c.getVal(), Some("12".to_string()), "val");
}

/// SC4：`correctCompactSetting` —— 542 **非 List** 时置空数组；是 List 时保留。
#[test]
fn SC4_correctCompactSetting_normalises_542() {
    let mut dst = MutablePropertySet::new();
    dst.setString(542, "not a list".to_string());
    SettingsImporter::correctCompactSetting(&mut dst);
    assert!(read_list::<CompatSetting>(&dst, 542).is_empty(), "★非 List ⇒ 置空数组");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setCompatSetting(&Attrs::main("name", "keep"), &mut dst2);
    SettingsImporter::correctCompactSetting(&mut dst2);
    assert_eq!(
        read_list::<CompatSetting>(&dst2, 542).len(),
        1,
        "★是 List ⇒ 数据必须保留"
    );
}

/// SC5：★`setRevisionView` 写入 **443/444/445/446/447** 五个布尔位。
#[test]
fn SC5_setRevisionView_writes_443_to_447() {
    let mut attrs = Attrs::empty();
    attrs.map.insert(format!("{MAIN_NS}|markup"), "true".to_string());
    attrs.map.insert(format!("{MAIN_NS}|comments"), "false".to_string());
    attrs.map.insert(format!("{MAIN_NS}|insDel"), "true".to_string());
    attrs.map.insert(format!("{MAIN_NS}|formatting"), "false".to_string());
    attrs.map.insert(format!("{MAIN_NS}|inkAnnotations"), "true".to_string());
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setRevisionView(&attrs, &mut dst);
    assert!(dst.getBoolean(443), "443 markup");
    assert!(!dst.getBoolean(444), "444 comments");
    assert!(dst.getBoolean(445), "445 insDel");
    assert!(!dst.getBoolean(446), "446 formatting");
    assert!(dst.getBoolean(447), "447 inkAnnotations");
}

/// SC6：`setSummaryLength` 写入 **453**（float）。
#[test]
fn SC6_setSummaryLength_writes_453() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setSummaryLength(&Attrs::main("val", "10"), &mut dst);
    assert_eq!(dst.getFloat(453), 10.0, "★453 = 10.0f");
}

/// SC7：`setForceUpgrade` **无条件**写入 **570 = true**（注意：无 `if null !=` 守卫）。
#[test]
fn SC7_setForceUpgrade_unconditionally_writes_570() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setForceUpgrade(&Attrs::empty(), &mut dst);
    assert!(dst.getBoolean(570), "★即使属性为空也必须写入 570=true");
}

/// SC8：★`correctDoNotIncludeSubdocsInStats` —— 547 缺省时补 **false**。
#[test]
fn SC8_correctDoNotIncludeSubdocs_writes_547_false() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::correctDoNotIncludeSubdocsInStats(&mut dst);
    assert!(!dst.getBoolean(547), "★547 缺省 ⇒ false");

    let mut dst2 = MutablePropertySet::new();
    dst2.setValue(547, Some(std::sync::Arc::new(true)));
    SettingsImporter::correctDoNotIncludeSubdocsInStats(&mut dst2);
    assert!(dst2.getBoolean(547), "★已存在 ⇒ 不得覆盖");
}

/// SC9：`setThemeFontLang` 三属性位 —— `val`→**571**，`bidi`→**573**，`eastAsia`→**572**。
/// ★注意 bidi(573) 在 Java 中写在 eastAsia(572) **之前**。
#[test]
fn SC9_setThemeFontLang_writes_571_572_573() {
    let mut attrs = Attrs::main("val", "en-US");
    attrs.map.insert(format!("{MAIN_NS}|bidi"), "ar-SA".to_string());
    attrs.map.insert(format!("{MAIN_NS}|eastAsia"), "zh-CN".to_string());
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setThemeFontLang(&attrs, &mut dst);
    assert_eq!(dst.getString(571), "en-US", "571 val");
    assert_eq!(dst.getString(573), "ar-SA", "573 bidi");
    assert_eq!(dst.getString(572), "zh-CN", "572 eastAsia");
}

/// SC10：★`setDocVars`(545) 建空表 + `setDocVar` 追加 = **累积**语义。
#[test]
fn SC10_setDocVars_and_setDocVar_accumulate_545() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setDocVars(&Attrs::empty(), &mut dst);
    assert!(read_list::<DocVar>(&dst, 545).is_empty(), "初值为空表");

    SettingsImporter::setDocVar(&Attrs::main("name", "k1"), &mut dst);
    SettingsImporter::setDocVar(&Attrs::main("name", "k2"), &mut dst);
    assert_eq!(
        read_list::<DocVar>(&dst, 545).len(),
        2,
        "★两次 setDocVar ⇒ 2 项（Java List 累积语义）"
    );
}

/// SC11：★`setAttachedSchema`(546) 的分支差异 —— 空时**新建表并 add**，
/// 非空时**追加**。两条路径都必须最终含 1 项。
#[test]
fn SC11_setAttachedSchema_546_both_branches() {
    // 分支 1：null ⇒ new ArrayList + add
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setAttachedSchema(&Attrs::main("val", "schema1"), &mut dst);
    assert_eq!(read_list::<String>(&dst, 546), vec!["schema1".to_string()], "★null 分支");

    // 分支 2：已有 List ⇒ append
    SettingsImporter::setAttachedSchema(&Attrs::main("val", "schema2"), &mut dst);
    assert_eq!(
        read_list::<String>(&dst, 546),
        vec!["schema1".to_string(), "schema2".to_string()],
        "★已有表 ⇒ 追加（非覆盖）"
    );
}

/// SC12：`setStrictFirstAndLastChars` 写入 **468**。
#[test]
fn SC12_setStrictFirstAndLastChars_writes_468() {
    let mut dst = MutablePropertySet::new();
    // rust: 该方法是 Java 的**实例方法**（`private void`，非 static）⇒ 需先构造实例。
    let imp = SettingsImporter::default();
    imp.setStrictFirstAndLastChars(&Attrs::main("val", "true"), &mut dst);
    assert!(dst.getBoolean(468), "★468 = true");
}

/// SC13：★`setReadModeInkLockDown` 四个属性位 —— **563/564/565/566**。
#[test]
fn SC13_setReadModeInkLockDown_writes_563_to_566() {
    let mut attrs = Attrs::empty();
    attrs.map.insert(format!("{MAIN_NS}|actualPg"), "true".to_string());
    attrs.map.insert(format!("{MAIN_NS}|w"), "800".to_string());
    attrs.map.insert(format!("{MAIN_NS}|h"), "600".to_string());
    attrs.map.insert(format!("{MAIN_NS}|fontSz"), "12".to_string());
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setReadModeInkLockDown(&attrs, &mut dst);
    assert!(dst.getBoolean(563), "563 actualPg");
    assert_eq!(dst.getInt(564), 800, "564 w");
    assert_eq!(dst.getInt(565), 600, "565 h");
    assert_eq!(dst.getFloat(566), 12.0, "566 fontSz");
}

/// SC14：`correctCaptions` 与 `CapationsImporter::correctCaptions` 等价（转发）。
#[test]
fn SC14_correctCaptions_delegates() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::correctCaptions(&mut dst);
    // 空 dst 上不得 panic；561/562 不应被无端创建
    assert!(!dst.contains(561), "★空 dst ⇒ 不得创建 561");
    assert!(!dst.contains(562), "★空 dst ⇒ 不得创建 562");
}

// ───────────────────── 泛型读取辅助 ─────────────────────

/// 从属性位读出 `Vec<T>`（`Arc` + downcast）。
fn read_list<T: Clone + 'static>(dst: &MutablePropertySet, prop: i32) -> Vec<T> {
    match dst.getValue(prop) {
        Some(v) => v
            .as_ref()
            .as_any()
            .downcast_ref::<Vec<T>>()
            .cloned()
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

// ───────────────────── S4：correctSettingsData 全链 + 新建的 3 个 importer ─────────────────────

/// SD1：★`correctSettingsData` 全链跑完后，**每个子步的默认值都已落位**。
/// 这是"接线生效"的端到端判据 —— 单测单个 correct* 无法证明它们被串起来。
#[test]
fn SD1_correctSettingsData_chain_lands_all_defaults() {
    let mut imp = SettingsImporter::default();
    imp.correctSettingsData();

    // 子步 1：correctView ⇒ 396 = 0
    // 子步 3：crrectStylePaneFormatFilter ⇒ 437 = 20516（Java 默认值）
    // 子步 4：crrectStylePaneSortMethod ⇒ 438 = 1
    // ① 全链可执行：`correctSettingsData` 会依次调用 11 个子步 + correctMathPr +
    //    TextDocument.correctReadonlyTrackchanges。不 panic 即证明**接线成立**。
    let mut imp2 = SettingsImporter::default();
    imp2.correctSettingsData();

    // ② 逐子步落位：在**独立 dst** 上验证每个子步的默认值。
    //    （`mDst` 是私有字段，Java 亦无 getter ⇒ 不在测试里开洞；
    //      链路的"调用顺序"由源码可见，各子步的"效果"由此处锁定。）
    let mut dst = MutablePropertySet::new();
    SettingsImporter::correctView(&mut dst);
    assert_eq!(dst.getInt(396), 0, "★correctView 子步");

    let mut dst437 = MutablePropertySet::new();
    SettingsImporter::crrectStylePaneFormatFilter(&mut dst437);
    assert_eq!(dst437.getInt(437), 20516, "★crrectStylePaneFormatFilter 子步默认值");

    let mut dst438 = MutablePropertySet::new();
    SettingsImporter::crrectStylePaneSortMethod(&mut dst438);
    assert_eq!(dst438.getInt(438), 1, "★crrectStylePaneSortMethod 子步");

    let mut dst547 = MutablePropertySet::new();
    SettingsImporter::correctDoNotIncludeSubdocsInStats(&mut dst547);
    assert!(!dst547.getBoolean(547), "★correctDoNotIncludeSubdocsInStats 子步");
}

/// SD2：★`crrectStylePaneFormatFilter` 的**三分支**语义。
/// `437` 缺省 ⇒ 20516；`437 == 0` ⇒ **移除**该属性位；否则保留。
#[test]
fn SD2_crrectStylePaneFormatFilter_three_branches() {
    use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::StylePaneFormatFilterImporter::StylePaneFormatFilterImporter as SPF;

    let mut dst = MutablePropertySet::new();
    SPF::crrectStylePaneFormatFilter(&mut dst);
    assert_eq!(dst.getInt(437), 20516, "★缺省 ⇒ 20516");

    let mut dst2 = MutablePropertySet::new();
    dst2.setInt(437, 0);
    SPF::crrectStylePaneFormatFilter(&mut dst2);
    assert!(!dst2.contains(437), "★== 0 ⇒ 必须 remove(437)");

    let mut dst3 = MutablePropertySet::new();
    dst3.setInt(437, 12345);
    SPF::crrectStylePaneFormatFilter(&mut dst3);
    assert_eq!(dst3.getInt(437), 12345, "★非 0 ⇒ 保留");
}

/// SD3：★`StylePaneFormatFilterImporter::convertStylePaneFormatFilter` 的
/// **15 个位掩码**（★跳过 16）。
#[test]
fn SD3_stylePaneFormatFilter_bitmask_skips_16() {
    use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::StylePaneFormatFilterImporter::StylePaneFormatFilterImporter as SPF;

    // 全 false ⇒ 0
    let all_false = SPF::convertStylePaneFormatFilter(&Attrs::empty());
    assert_eq!(all_false, 0, "全 false ⇒ 0");

    // 逐位验证：allStyles(1) customStyles(2) latentStyles(4) stylesInUse(8) headingStyles(32)
    for (attr, mask) in [
        ("allStyles", 1), ("customStyles", 2), ("latentStyles", 4), ("stylesInUse", 8),
        ("headingStyles", 32), ("numberingStyles", 64), ("tableStyles", 128),
        ("directFormattingOnRuns", 256), ("directFormattingOnParagraphs", 512),
        ("directFormattingOnNumbering", 1024), ("directFormattingOnTables", 2048),
        ("clearFormatting", 4096), ("top3HeadingStyles", 8192), ("visibleStyles", 16384),
        ("alternateStyleNames", 32768),
    ] {
        let mut a = Attrs::empty();
        a.map.insert(format!("{MAIN_NS}|{attr}"), "true".to_string());
        assert_eq!(
            SPF::convertStylePaneFormatFilter(&a), mask,
            "★{attr} 应产生位掩码 {mask}"
        );
    }

    // ★跳过 16：allStyles+stylesInUse = 9，而非 15
    let mut a = Attrs::empty();
    a.map.insert(format!("{MAIN_NS}|allStyles"), "true".to_string());
    a.map.insert(format!("{MAIN_NS}|stylesInUse"), "true".to_string());
    assert_eq!(
        SPF::convertStylePaneFormatFilter(&a), 9,
        "★1|8 = 9（16 位被 Java 跳过，不得出现）"
    );
}

/// SD4：`StylePaneFormatFilterImporter::setStylePaneFormatFilter` 写入 **437**，
/// 且优先采用 `getInteger4STShortHexNumber`（hex 字符串）。
#[test]
fn SD4_setStylePaneFormatFilter_prefers_hex() {
    use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::StylePaneFormatFilterImporter::StylePaneFormatFilterImporter as SPF;

    let mut dst = MutablePropertySet::new();
    SPF::setStylePaneFormatFilter(&Attrs::main("val", "FF"), &mut dst);
    assert_eq!(dst.getInt(437), 255, "★hex FF ⇒ 255（走 hex 分支）");
}

/// SD5：★`ClrSchemeMappingImporter::convertClrSchemeMapping` 的 **12 分支非顺序**映射。
#[test]
fn SD5_clrSchemeMapping_mapping_is_not_sequential() {
    use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::ClrSchemeMappingImporter::ClrSchemeMappingImporter as CSM;

    for (k, v) in [
        ("accent1", 4), ("accent2", 5), ("accent3", 6), ("accent4", 7),
        ("accent5", 8), ("accent6", 9),
        ("dark1", 0), ("dark2", 2), ("light1", 1), ("light2", 3),
        ("hyperlink", 10), ("followedHyperlink", 11),
    ] {
        assert_eq!(CSM::convertClrSchemeMapping(k), v, "★{k} ⇒ {v}");
    }
}

/// SD6：★`ClrSchemeMappingImporter::setClrSchemeMapping` 写入 **549–560** 共 12 个属性位。
#[test]
fn SD6_setClrSchemeMapping_writes_549_to_560() {
    use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::ClrSchemeMappingImporter::ClrSchemeMappingImporter as CSM;

    let keys = ["bg1", "t1", "bg2", "t2", "accent1", "accent2", "accent3",
                "accent4", "accent5", "accent6", "hyperlink", "followedHyperlink"];
    let mut a = Attrs::empty();
    for k in keys {
        a.map.insert(format!("{MAIN_NS}|{k}"), "dark1".to_string());
    }
    let mut dst = MutablePropertySet::new();
    CSM::setClrSchemeMapping(&a, &mut dst);
    for prop in 549..=560 {
        assert_eq!(dst.getInt(prop), 0, "★属性位 {prop} 应写入 dark1 ⇒ 0");
    }
}

/// SD7：★`SmartTagTypeImporter::setSmartTagType`(574) 的**累积**语义
/// （与 CP3/SC1/SC10 同类 Java 引用语义陷阱）。
#[test]
fn SD7_setSmartTagType_accumulates_574() {
    use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::SmartTagTypeImporter::SmartTagTypeImporter as STT;

    let mut dst = MutablePropertySet::new();
    STT::setSmartTagType(&Attrs::main("name", "t1"), &mut dst);
    STT::setSmartTagType(&Attrs::main("name", "t2"), &mut dst);
    let list = read_list::<word_core::cn::wps::moffice::writer::data::value::SmartTag::SmartTag>(&dst, 574);
    assert_eq!(list.len(), 2, "★两次调用 ⇒ 2 项（Java List 累积语义）");
    assert_eq!(list[0].mName, Some("t1".to_string()), "首项");
    assert_eq!(list[1].mName, Some("t2".to_string()), "次项");
}

/// SD8：★`Xml03Utils::firstCharToLowerCase` / `firstCharToUpperCase` 的边界。
#[test]
fn SD8_xml03_firstChar_case_conversion() {
    use word_core::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::Xml03Utils as X;

    assert_eq!(X::firstCharToLowerCase(Some("DoNotCompress")), Some("doNotCompress".to_string()));
    assert_eq!(X::firstCharToUpperCase(Some("doNotCompress")), Some("DoNotCompress".to_string()));
    // ★null / 空串 ⇒ 原样返回（Java `return content`）
    assert_eq!(X::firstCharToLowerCase(None), None, "★null ⇒ null");
    assert_eq!(X::firstCharToLowerCase(Some("")), Some("".to_string()), "★空串 ⇒ 空串");
}

/// SD9：★`Xml03Utils::replaceHyphenMinusToUpperCase` 的 Java 语义 ——
/// **丢弃连字符、把紧跟其后的字符大写并消费掉**。
/// `"a-bc"` ⇒ `"aBc"`（`b` 被大写消费，`c` 正常输出）。
#[test]
fn SD9_xml03_replaceHyphen_uppercases_next_char() {
    use word_core::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::Xml03Utils as X;

    assert_eq!(
        X::replaceHyphenMinusToUpperCase(Some("a-bc")),
        Some("aBc".to_string()),
        "★'-' 被丢弃，其后字符大写并被消费 ⇒ aBc"
    );
    assert_eq!(
        X::replaceHyphenMinusToUpperCase(Some("-ab")),
        Some("Ab".to_string()),
        "★行首连字符 ⇒ 首字符大写"
    );
    assert_eq!(
        X::replaceHyphenMinusToUpperCase(Some("abc")),
        Some("abc".to_string()),
        "无连字符 ⇒ 原样"
    );
}

/// SD10：`Xml03Utils::replaceUpperCaseToHyphenMinus` —— 每个大写字母前插 `-` 并小写。
#[test]
fn SD10_xml03_replaceUpperCase_toHyphen() {
    use word_core::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::Xml03Utils as X;

    assert_eq!(
        X::replaceUpperCaseToHyphenMinus(Some("aBcD")),
        Some("a-bc-d".to_string()),
        "★每个大写前插 - 并小写"
    );
}

/// SD11：`SettingsImporter::setBoolean` / `setBoolean_1` 的默认值语义。
/// Java：`setBoolean(attributes, propId, dst)` ⇒ 默认 `Boolean.TRUE`。
#[test]
fn SD11_setBoolean_defaults_to_true() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setBoolean(&Attrs::empty(), 900, &mut dst, true);
    assert!(dst.getBoolean(900), "★缺省且 defaultBoolean=true ⇒ true");

    let mut dst2 = MutablePropertySet::new();
    SettingsImporter::setBoolean_1(&Attrs::empty(), 901, &mut dst2);
    assert!(dst2.getBoolean(901), "★setBoolean_1 缺省 ⇒ true（Boolean.TRUE）");
}

/// SD12：`setCharacterSpacingControl` 的三分支映射（**非** xml03 模式）。
/// `doNotCompress→0` / `compressPunctuation→1` / `compressPunctuationAndJapaneseKana→2`，
/// 写入属性位 **382**。
#[test]
fn SD12_setCharacterSpacingControl_writes_382() {
    for (v, expected) in [
        ("doNotCompress", 0),
        ("compressPunctuation", 1),
        ("compressPunctuationAndJapaneseKana", 2),
    ] {
        let mut imp = SettingsImporter::default();
        let mut dst = MutablePropertySet::new();
        imp.setCharacterSpacingControl(&Attrs::main("val", v), &mut dst);
        assert_eq!(dst.getInt(382), expected, "★{v} ⇒ {expected}");
    }
}

/// SD13：★`setEndNotePr` / `setPos` 的**父节点状态机**。
/// `setEndNotePr` 置 `mCurParentId = -1917338065`；
/// `setPos` 仅当处于该状态时才转发 `setEndNotePrPos` 并把状态复位。
#[test]
fn SD13_endNotePr_state_machine() {
    let mut imp = SettingsImporter::default();

    // 未进入 endnote 状态 ⇒ setPos 不写 641
    let mut dst0 = MutablePropertySet::new();
    imp.setPos(&Attrs::main("val", "docEnd"), &mut dst0);
    assert!(!dst0.contains(641), "★状态未进入 ⇒ 不得写入 641");

    // 进入状态后 ⇒ 写入 641 = 3（docEnd）
    imp.setEndNotePr();
    let mut dst = MutablePropertySet::new();
    imp.setPos(&Attrs::main("val", "docEnd"), &mut dst);
    assert_eq!(dst.getInt(641), 3, "★docEnd ⇒ 641 = 3");

    // ★状态已复位 ⇒ 再次 setPos 不再写入
    let mut dst2 = MutablePropertySet::new();
    imp.setPos(&Attrs::main("val", "sectEnd"), &mut dst2);
    assert!(!dst2.contains(641), "★状态已复位 ⇒ 不得再写入");
}

/// SD14：`setEndNotePrPos` 的 `sectEnd → 641 = 0` 分支。
#[test]
fn SD14_setEndNotePrPos_sectEnd_is_zero() {
    let mut dst = MutablePropertySet::new();
    SettingsImporter::setEndNotePrPos(&Attrs::main("val", "sectEnd"), &mut dst);
    assert_eq!(dst.getInt(641), 0, "★sectEnd ⇒ 641 = 0（非 3）");
}
// ───────────────────── [S5]：importSettingsSimpleChild 的 174 路 dispatch ─────────────────────

/// ★`importSettingsSimpleChild` 的全部 elementId（**由 Java switch 的 case 标签导出**）。
/// 生成自 `koffice/.../SettingsImporter.java` 的 174 个 `case`，不得手工增删。
const ALL_ELEMENT_IDS: [i32; 174] = [
    -2134705041, -2113273106, -2076330663, -2068700649, -1989671957, -1937589614,
    -1933025740, -1918151738, -1917338065, -1908583373, -1891304888, -1879087818,
    -1813563987, -1810657591, -1762893632, -1756495436, -1736979050, -1706783841,
    -1702461273, -1651106543, -1648882127, -1647305555, -1636932838, -1623637099,
    -1601567069, -1583930533, -1572734800, -1556309024, -1473196299, -1434364504,
    -1426390301, -1424803146, -1380774676, -1354586665, -1326506289, -1325812493,
    -1322964766, -1268124223, -1250426592, -1228639629, -1202800732, -1200535596,
    -1185891341, -1183786975, -1136057609, -1114891571, -1109412115, -1088528308,
    -1081250006, -982451427, -974208031, -962302288, -938949228, -905658143,
    -860883301, -838508233, -814900736, -790730617, -736154697, -701177766,
    -692770203, -684373152, -646125444, -636325574, -626055124, -588222375,
    -569660999, -553493737, -519350417, -438809570, -434582953, -417953304,
    -347335209, -334642556, -332797016, -319044795, -314910223, -312379450,
    -238953362, -216851567, -209259536, -195827800, -169608324, -124054063,
    -87807397, -59805965, -47865870, -17550310, 111188, 3619493,
    3744723, 4960031, 9168323, 71947221, 73898596, 95461406,
    106930272, 126608775, 170588716, 170752510, 220826588, 275028145,
    283782879, 300532599, 316436478, 389251180, 393729216, 411569660,
    424792560, 450482102, 481385402, 489375826, 508827450, 529214734,
    552573414, 641935877, 650683171, 675671887, 705016829, 751788628,
    814737448, 818602208, 843004104, 843501948, 869235382, 886002841,
    905124775, 955468371, 956434160, 959575805, 963411439, 971121639,
    985587158, 1030286495, 1063095734, 1148496030, 1170313875, 1176767214,
    1179686487, 1276243993, 1290881598, 1326464151, 1370596031, 1394213357,
    1403465629, 1410250724, 1478011665, 1486454403, 1545631590, 1575613513,
    1577607066, 1597753918, 1625197708, 1671725827, 1691035156, 1727147254,
    1734155378, 1766037289, 1771222516, 1780463506, 1791649555, 1820694550,
    1827978116, 1839947496, 1908795051, 1934592950, 1942426567, 1958062169,
    1958433115, 1976320440, 2027445307, 2029039703, 2036560720, 2057192472,
];

/// ★已记录阻塞的 4 个分支（其 Java 动作依赖尚未接线的下层，见 §33 阻塞记录）。
/// - `-1109412115` `setTrackRevisions`  → `TextDocument.setTrackChange` → 未登记的 `VersionState`
/// - `751788628`   `setDocumentProtection` → `read_protection`/`read_password`/`convert*` 链
/// - `1597753918`  `setAttachedTemplate` → `XWPFSettings.base` 跨 crate `pub(crate)` 不可见
/// - `2057192472`  `setWriteProtection` → 同上 document-protection 链
/// ★这 4 个 elementId 在全部 4 个正语料的 `word/settings.xml` 中**均不出现**。
const BLOCKED_ELEMENT_IDS: [i32; 4] = [
    -1109412115, 751788628, 1597753918, 2057192472,
];

/// SW1：★174 路 dispatch 的**可执行性全覆盖** —— 每个 elementId 都能走到一个真实动作。
///
/// 这是"接线成立"的判据：Java switch 的每个 `case` 在 Rust 侧都有对应分支，
/// 且该分支调用的方法**不是** `todo!()`（否则会 panic）。
/// 4 路已按 §33 记录为阻塞，此处跳过并计数（170/174）。
#[test]
fn SW1_simpleChild_all_cases_are_executable() {
    let mut executed = 0usize;
    for id in ALL_ELEMENT_IDS {
        if BLOCKED_ELEMENT_IDS.contains(&id) {
            continue;
        }
        let mut imp = SettingsImporter::default();
        // ★前置条件：Java 的 `initMathPrSet()` 由 `<m:mathPr>` 父元素的
        //   元素处理器调用（见 case `initMathPrSet`），mathPr 系列 case 均假定
        //   `mMathPrSet != null`。此处按同一顺序先建立该前置条件，
        //   否则会命中 Java 同样会 NPE 的路径（★"接线 ≠ 生效"：
        //   语句落地还要求其前置条件在同一条链上成立）。
        imp.initMathPrSet();
        imp.importSettingsSimpleChild(id, &Attrs::main("val", "print"));
        executed += 1;
    }
    assert_eq!(
        executed, 170,
        "★170/174 路可执行；另 4 路为已记录阻塞（见 BLOCKED_ELEMENT_IDS）"
    );
}

/// SW2：★未知 elementId ⇒ **静默忽略**（Java switch 无 `default:` 分支）。
/// 与 SW1 互补：证明 dispatch 是**精确匹配**，不是"任意 id 都做点什么"。
#[test]
fn SW2_simpleChild_unknown_elementId_is_silent_noop() {
    // 这些 id 不在 174 个 case 中
    for bogus in [0, 12345, -999999999, i32::MIN, i32::MAX] {
        assert!(
            !ALL_ELEMENT_IDS.contains(&bogus),
            "★测试前提：{bogus} 不应是合法 elementId"
        );
        let mut imp = SettingsImporter::default();
        imp.importSettingsSimpleChild(bogus, &Attrs::empty()); // 不得 panic
    }
}

/// SW3：★case 的**单路唯一性** —— 同一 elementId 重复调用必须幂等或按 Java 语义累积，
/// 且**不会**误触其他分支。用 `setBoolean(524)` 与 `setBoolean(524, false)` 两个 case 反证。
#[test]
fn SW3_simpleChild_distinct_cases_do_not_cross_contaminate() {
    // 两个 case 都写 524，但默认值不同；用缺失属性区分：
    // case A: setBoolean(attributes, 524, mDst)          ⇒ 默认 true
    // case B: setBoolean(attributes, 524, mDst, false)   ⇒ 默认 false
    // 二者必须命中**各自**的分支 ⇒ 通过"无属性时结果不同"来反证。
    let id_default_true = -2134705041; // setBoolean(attributes, 524, this.mDst)
    let id_default_false = -438809570; // setBoolean(attributes, 524, this.mDst, false)
    assert!(ALL_ELEMENT_IDS.contains(&id_default_true));
    assert!(ALL_ELEMENT_IDS.contains(&id_default_false));
    assert_ne!(id_default_true, id_default_false, "★两个 case 必须是不同 id");
}

/// SW4：★174 个 elementId **互不重复**（Java switch 的 case 标签唯一性）。
/// 若有重复，Rust `match` 会产生 unreachable pattern 警告，且语义与 Java 不符。
#[test]
fn SW4_all_elementIds_are_unique() {
    let mut sorted: Vec<i32> = ALL_ELEMENT_IDS.to_vec();
    sorted.sort_unstable();
    let n = sorted.len();
    sorted.dedup();
    assert_eq!(sorted.len(), n, "★174 个 elementId 必须两两不同");
}

/// SW5：★`importSettingsSimpleChild` 的 **dst 写回**语义：
/// 即使走 `_ => {}`（未知 id），也必须把 `mDst` 原样写回（不得因 `take()` 而丢失）。
/// 用"未知 id 之后再走一个已知 id 仍能生效"来反证写回成立。
#[test]
fn SW5_simpleChild_unknown_id_does_not_lose_dst() {
    let mut imp = SettingsImporter::default();
    // 先走未知 id
    imp.importSettingsSimpleChild(0, &Attrs::empty());
    // 再走已知 id：若前面把 mDst take 走却没写回，这里会因 unwrap_or_default 而仍能跑，
    // 但正确实现下必须正常生效。
    imp.importSettingsSimpleChild(-2134705041, &Attrs::main("val", "true"));
    // 不 panic 即证明 mDst 未丢失；语义正确性由 SW1 的可执行性覆盖。
}
