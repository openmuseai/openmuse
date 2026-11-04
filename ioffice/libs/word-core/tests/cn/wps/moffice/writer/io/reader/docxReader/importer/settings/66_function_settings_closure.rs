// function_id=66_function_settings_closure
// cargo test --manifest-path libs/word-core/Cargo.toml --test 66_function_settings_closure
//
// ★M-STAGE1 P2 —— `SettingsImporter` 依赖闭包的**首批新建类型**，按 §24.4 判据逐个做实。
//
// 背景：`SettingsImporter`（Java 1774 行）在产品路径上被真实调用
//   （`DocumentImporter.rs:1369/1379/1387` 调 `importSettingsStart` /
//    `importSettingsSimpleChild` / `importSettingsEnd`），但其实体是
//   102 处 `todo!()` 的 auto-generated 桩，且**依赖闭包内 4 个类全仓缺失**。
//
// 本用例锁定本轮新建的两个闭包成员：
//   - `DocVar`（Java 23 行，零依赖）
//   - `NoLineBreaksImporter`（Java 81 行，继承 `ImporterBase`）
//
// ★这些类型此前**完全不存在**（0 hit）⇒ 空壳时代其行为不可观测。
//   本用例断言的是**真实语义**（字段存取、语言映射、属性表写入），
//   而不是"能编译"。
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::data::value::DocVar::DocVar;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::NoLineBreaksImporter::NoLineBreaksImporter;

// ───────────────── SC1：`DocVar` 构造与存取 ─────────────────

/// SC1：`DocVar(name, val)` 须原样保存两个字段。
/// Java 的 `assertNotNull` 在 Rust 侧由按值 `String` 的类型系统保证。
#[test]
fn SC1_docvar_stores_name_and_val() {
    let v = DocVar::DocVar("MyVar".to_string(), "MyValue".to_string());
    assert_eq!(v.getName(), Some("MyVar".to_string()), "getName");
    assert_eq!(v.getVal(), Some("MyValue".to_string()), "getVal");
}

// ───────────────── SC2：`NoLineBreaksImporter` 语言映射（Java 四分支） ─────────────────

/// SC2：`convertLanguage` 的 4 个 Java 分支必须逐项对应。
///
/// 该方法是 `private`，故通过 `correctNoLineBreaks` 的**可观测效果**间接验证：
/// 仅当 before/after 的 `lang` 相同且非空时才会写入 381/379（语言值）。
/// 因此用 4 种 lang 各跑一次，断言写入的属性值等于 Java 的映射结果。
#[test]
fn SC2_language_mapping_matches_java_branches() {
    // Java: "zh-CN"→2, "zh-TW"→4, "ja-JP"→1, "ko-KR"→3
    let cases = [("zh-CN", 2), ("zh-TW", 4), ("ja-JP", 1), ("ko-KR", 3)];

    for (lang, expected) in cases {
        let mut imp = NoLineBreaksImporter::NoLineBreaksImporter(0, None, None);
        let mut dst = MutablePropertySet::new();

        // 构造带 lang + val 的属性面，使 before/after 的 lang 相同
        let attrs = Attrs {
            len: 2,
            val: Some("x".to_string()),
            lang: Some(lang.to_string()),
        };
        imp.setNoLineBreaksBefore(&attrs);
        imp.setNoLineBreaksAfter(&attrs);
        imp.correctNoLineBreaks(&mut dst);

        // Java: dst.setInt(381, lang) / dst.setInt(379, lang)
        assert_eq!(
            dst.getInt(381),
            expected,
            "lang={lang} 时 381 应为 {expected}（Java 映射）"
        );
        assert_eq!(
            dst.getInt(379),
            expected,
            "lang={lang} 时 379 应为 {expected}（Java 映射）"
        );
        // Java: dst.setInt(384, dstVal)，两处命中 ⇒ dstVal=2
        assert_eq!(dst.getInt(384), 2, "lang={lang} 时 384 应为 2");
    }
}

// ───────────────── SC3：lang 不同 ⇒ 不写语言属性（Java 的 equals 判定） ─────────────────

/// SC3：Java 要求 `mBeforeLang.equals(mAfterLang)` 才换算 lang。
/// 两者不同时必须**不**写入 381/379，且 `dstVal` 保持 0。
#[test]
fn SC3_differing_langs_skip_language_write() {
    let mut imp = NoLineBreaksImporter::NoLineBreaksImporter(0, None, None);
    let mut dst = MutablePropertySet::new();

    let before = Attrs { len: 2, val: Some("b".to_string()), lang: Some("zh-CN".to_string()) };
    let after = Attrs { len: 2, val: Some("a".to_string()), lang: Some("ja-JP".to_string()) };
    imp.setNoLineBreaksBefore(&before);
    imp.setNoLineBreaksAfter(&after);
    imp.correctNoLineBreaks(&mut dst);

    assert!(!dst.contains(381), "lang 不同 ⇒ 不得写 381");
    assert!(!dst.contains(379), "lang 不同 ⇒ 不得写 379");
    assert_eq!(dst.getInt(384), 0, "无命中 ⇒ dstVal 保持 0");
}

// ───────────────── SC4：`isStrict`(468) 为 true ⇒ 短路为 1 ─────────────────

/// SC4：Java 中若 `dst.getValue(468)` 为 `Boolean.TRUE`，
/// 则 `dstVal = 1` 且**跳过**全部 before/after 处理（含属性写入）。
#[test]
fn SC4_isStrict_true_short_circuits_to_one() {
    let mut imp = NoLineBreaksImporter::NoLineBreaksImporter(0, None, None);
    let mut dst = MutablePropertySet::new();
    // java Boolean isStrict = (Boolean)dst.getValue(468);
    dst.setBoolean(468, true);

    let attrs = Attrs { len: 2, val: Some("x".to_string()), lang: Some("zh-CN".to_string()) };
    imp.setNoLineBreaksBefore(&attrs);
    imp.setNoLineBreaksAfter(&attrs);
    imp.correctNoLineBreaks(&mut dst);

    assert_eq!(dst.getInt(384), 1, "isStrict ⇒ dstVal = 1");
    assert!(!dst.contains(381), "★短路 ⇒ 即使 lang 合法也不写 381");
}

// ───────────────── 测试用 SAX `Attributes` 适配器 ─────────────────

/// 实现 `ConvertHelper` 定义的真实 `Attributes` trait。
struct Attrs {
    len: i32,
    val: Option<String>,
    lang: Option<String>,
}

impl word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes
    for Attrs
{
    fn getLength(&self) -> i32 {
        self.len
    }
    fn getValue(&self, _uri: &str, localName: &str) -> Option<String> {
        match localName {
            "val" => self.val.clone(),
            "lang" => self.lang.clone(),
            _ => None,
        }
    }
}
