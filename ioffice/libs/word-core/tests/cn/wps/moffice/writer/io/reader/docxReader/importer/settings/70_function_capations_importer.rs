// function_id=70_function_capations_importer
// cargo test --manifest-path libs/word-core/Cargo.toml --test 70_function_capations_importer
//
// ★M-STAGE1 P2 —— `SettingsImporter` 依赖闭包的**最后一个成员**做实后的行为锁定。
//   `CapationsImporter`（Java 166 行）此前全仓缺失。
//
// 本轮连带新建了它依赖的两个值类型：
//   - `CaptionType`      （Java 76 行，7 字段 + 7 组 setter/getter）
//   - `AutoCaptionType`  （Java 23 行，2 字段）
//
// ★本用例重点锁定 **Java 原文的一处疑似笔误**（CP6），以及
//   两个枚举映射表的完整分支覆盖（CP3/CP4）。
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::data::value::AutoCaptionType::AutoCaptionType;
use word_core::cn::wps::moffice::writer::data::value::CaptionType::CaptionType;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::CapationsImporter::CapationsImporter;

// ───────────────── CT1：`CaptionType` 字段语义 ─────────────────

/// CT1：Java `CaptionType(String name)` 的 7 个字段初值必须是 `null`
/// （除 `name`），setter 写入后 getter 必须可读回。
#[test]
fn CT1_caption_type_defaults_and_round_trip() {
    let mut c = CaptionType::CaptionType("Figure".to_string());

    assert_eq!(c.getName(), Some("Figure".to_string()), "name");
    // Java: 其余 6 个字段 `= null`
    assert_eq!(c.getPos(), None, "mPos 初值 null");
    assert_eq!(c.getChapNum(), None, "mChapNum 初值 null");
    assert_eq!(c.getHeading(), None, "mHeading 初值 null");
    assert_eq!(c.getNoLabel(), None, "mNoLabel 初值 null");
    assert_eq!(c.getNumFmt(), None, "mNumFmt 初值 null");
    assert_eq!(c.getSep(), None, "mSep 初值 null");

    c.setPos(1);
    c.setChapNum(true);
    c.setHeading(2);
    c.setNoLabel(false);
    c.setNumFmt(3);
    c.setSep(4);

    assert_eq!(c.getPos(), Some(1), "setPos");
    assert_eq!(c.getChapNum(), Some(true), "setChapNum");
    assert_eq!(c.getHeading(), Some(2), "setHeading");
    assert_eq!(c.getNoLabel(), Some(false), "★setNoLabel 必须写 mNoLabel（非 mChapNum）");
    assert_eq!(c.getNumFmt(), Some(3), "setNumFmt");
    assert_eq!(c.getSep(), Some(4), "setSep");
}

// ───────────────── CT2：`AutoCaptionType` ─────────────────

/// CT2：`AutoCaptionType(name, caption)` 两字段存取。
#[test]
fn CT2_auto_caption_type_round_trip() {
    let a = AutoCaptionType::AutoCaptionType("Table".to_string(), "Table Caption".to_string());
    assert_eq!(a.getName(), Some("Table".to_string()), "getName");
    assert_eq!(a.getCaption(), Some("Table Caption".to_string()), "getCaption");
}

// ───────────────── CP1：`convertCaptionPos` 的 Java 映射表 ─────────────────

/// CP1：`pos` 属性的 4 个 Java 分支必须逐项对应
/// （`above→0`, `below→1`, `left→2`, `right→3`）。
///
/// 该方法为 `private static`，经 `setCaption` 的**可观测效果**间接验证。
#[test]
fn CP1_caption_pos_mapping_matches_java() {
    // Java: "above"→0, "below"→1, "left"→2, "right"→3
    for (pos_str, expected) in [("above", 0), ("below", 1), ("left", 2), ("right", 3)] {
        let attrs = Attrs::new(&[("name", "Figure"), ("pos", pos_str)]);
        let mut dst = MutablePropertySet::new();
        let mut imp = new_imp();
        imp.setCaption(&attrs, &mut dst);

        // 读取 561 属性位中的 CaptionType 列表
        let list = read_caption_list(&dst);
        assert_eq!(list.len(), 1, "pos={pos_str} ⇒ 应写入 1 个 CaptionType");
        assert_eq!(
            list[0].getPos(),
            Some(expected),
            "★pos=\"{pos_str}\" 应映射为 {expected}（Java 映射表）"
        );
    }
}

// ───────────────── CP2：`convertChapterSep` 的 Java 映射表 ─────────────────

/// CP2：`sep` 属性的 5 个 Java 分支必须逐项对应。
/// ★注意 Java 的**非顺序**映射：`hyphen→0`, `period→1`, `colon→2`,
///   `emDash→3`, `enDash→4`。
#[test]
fn CP2_chapter_sep_mapping_matches_java() {
    for (sep_str, expected) in [
        ("colon", 2),
        ("emDash", 3),
        ("enDash", 4),
        ("hyphen", 0),
        ("period", 1),
    ] {
        let attrs = Attrs::new(&[("name", "Figure"), ("sep", sep_str)]);
        let mut dst = MutablePropertySet::new();
        let mut imp = new_imp();
        imp.setCaption(&attrs, &mut dst);

        let list = read_caption_list(&dst);
        assert_eq!(
            list[0].getSep(),
            Some(expected),
            "★sep=\"{sep_str}\" 应映射为 {expected}（Java 映射表）"
        );
    }
}

// ───────────────── CP3：`setCaption` 的列表累积语义 ─────────────────

/// CP3：Java `setCaption` 在属性位已有列表时**追加**而非替换。
/// 连续两次调用后，列表长度必须为 2。
#[test]
fn CP3_setCaption_appends_to_existing_list() {
    let mut dst = MutablePropertySet::new();
    let mut imp = new_imp();

    imp.setCaption(&Attrs::new(&[("name", "First")]), &mut dst);
    imp.setCaption(&Attrs::new(&[("name", "Second")]), &mut dst);

    let names: Vec<Option<String>> = read_caption_list(&dst).iter().map(|c| c.getName()).collect();
    assert_eq!(names.len(), 2, "★两次 setCaption ⇒ 列表应有 2 项（Java 追加语义）");
    assert!(names.contains(&Some("First".to_string())), "应含 First");
    assert!(names.contains(&Some("Second".to_string())), "应含 Second");
}

// ───────────────── CP4：`setAutoCaption` 与 `AutoCaptionType` ─────────────────

/// CP4：`setAutoCaption` 必须经 `convertAutoCaptions` 产生
/// `AutoCaptionType(name, caption)`，写入属性位 562。
#[test]
fn CP4_setAutoCaption_writes_562() {
    let attrs = Attrs::new(&[("name", "Table"), ("caption", "Table Caption")]);
    let mut dst = MutablePropertySet::new();
    let mut imp = new_imp();
    imp.setAutoCaption(&attrs, &mut dst);

    let list = read_auto_caption_list(&dst);
    assert_eq!(list.len(), 1, "应写入 1 个 AutoCaptionType");
    assert_eq!(list[0].getName(), Some("Table".to_string()), "name");
    assert_eq!(list[0].getCaption(), Some("Table Caption".to_string()), "caption");
}

// ───────────────── ★CP5：`noLabel` 的 Java 笔误必须被复现 ─────────────────

/// ★CP5：Java 原文 `convertCaption` 中，`noLabel` 属性读入后调用的是
/// **`captionType.setChapNum(label)`** 而非 `setNoLabel(label)`：
///
/// ```java
/// Boolean label = ConvertHelper.getBoolean(attributes, "noLabel");
/// if (null != label) {
///    captionType.setChapNum(label);   // ← 疑似笔误：应为 setNoLabel
/// }
/// ```
///
/// 按既定纪律「移植不是改进 Java，是复现 Java」⇒ **原样保留**并由本用例锁定。
/// 若日后 Java 修正该行，本用例应随之更新（而非先行"修好"）。
#[test]
fn CP5_java_typo_nolabel_writes_chapnum_not_nolabel() {
    let attrs = Attrs::new(&[("name", "Figure"), ("noLabel", "true")]);
    let mut dst = MutablePropertySet::new();
    let mut imp = new_imp();
    imp.setCaption(&attrs, &mut dst);

    let list = read_caption_list(&dst);
    assert_eq!(
        list[0].getChapNum(),
        Some(true),
        "★Java 笔误：noLabel=true 写入的是 mChapNum ⇒ 必须复现"
    );
    assert_eq!(
        list[0].getNoLabel(),
        None,
        "★Java 笔误：mNoLabel 必须**保持 null**（若此处变 Some，说明有人改好了 Java）"
    );
}

// ───────────────── CP6：`correctCaptions` 不破坏既有数据 ─────────────────

/// CP6：`correctCaptions` 在 Java 中把 `List` 转成数组存回同一属性位。
/// Rust 侧 `Vec` 即数组 ⇒ 该转换为恒等。**关键判据**：数据不得丢失。
#[test]
fn CP6_correctCaptions_preserves_data() {
    let mut dst = MutablePropertySet::new();
    let mut imp = new_imp();
    imp.setCaption(&Attrs::new(&[("name", "Keep")]), &mut dst);
    imp.setAutoCaption(&Attrs::new(&[("name", "T"), ("caption", "C")]), &mut dst);

    CapationsImporter::correctCaptions(&mut dst);

    let captions = read_caption_list(&dst);
    let autos = read_auto_caption_list(&dst);
    assert_eq!(captions.len(), 1, "★correctCaptions 不得清空 561");
    assert_eq!(captions[0].getName(), Some("Keep".to_string()), "★数据必须保留");
    assert_eq!(autos.len(), 1, "★correctCaptions 不得清空 562");
}

// ───────────────── 辅助 ─────────────────

fn new_imp() -> CapationsImporter {
    CapationsImporter::CapationsImporter(0, None, None)
}

/// 从属性位 561 读出 `Vec<CaptionType>`。
fn read_caption_list(dst: &MutablePropertySet) -> Vec<CaptionType> {
    match dst.getValue(561) {
        Some(v) => v
            .as_ref()
            .as_any()
            .downcast_ref::<Vec<CaptionType>>()
            .cloned()
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

/// 从属性位 562 读出 `Vec<AutoCaptionType>`。
fn read_auto_caption_list(dst: &MutablePropertySet) -> Vec<AutoCaptionType> {
    match dst.getValue(562) {
        Some(v) => v
            .as_ref()
            .as_any()
            .downcast_ref::<Vec<AutoCaptionType>>()
            .cloned()
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

/// 测试用 SAX `Attributes` 适配器（实现本仓权威的 `ConvertHelper::Attributes`）。
struct Attrs {
    map: std::collections::HashMap<String, String>,
}

impl Attrs {
    fn new(pairs: &[(&str, &str)]) -> Self {
        Attrs {
            map: pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl Attributes for Attrs {
    fn getLength(&self) -> i32 {
        self.map.len() as i32
    }
    fn getValue(&self, _uri: &str, localName: &str) -> Option<String> {
        self.map.get(localName).cloned()
    }
}
