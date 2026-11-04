//! ★M-STAGE1 P2-8 —— theme/font 家族真实接线测试。
//!
//! 覆盖：`SupplementalFont` / `FontCollection` / `ThemeFontScheme` /
//! `FontSchemeImporter` / `ConvertHelper::{getTextFont,setPanose,convertPanose2Core,panoseItem,
//! getSupplementalFont}`。
//!
//! ★本文件的意义在于**抓 Java 引用语义**：`FontSchemeImporter` 里
//! `setMajorFont(this.mFontCollection)` 后仍在**同一对象**上 `setComplexScript`，
//! 若 Rust 用值语义（克隆），scheme 侧将**永远看不到**结果 —— 静默失效。

use word_core::cn::wps::moffice::writer::data::theme::font::FontCollection::FontCollection;
use word_core::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
use word_core::cn::wps::moffice::writer::data::theme::font::SupplementalFont::SupplementalFont;
use word_core::cn::wps::moffice::writer::data::theme::font::ThemeFontScheme::ThemeFontScheme;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::theme::FontSchemeImporter::FontSchemeImporter;

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

/// ST1 —— `SupplementalFont` 3 个字段往返。
#[test]
fn st1_supplemental_font_roundtrip() {
    let mut f = SupplementalFont::new();
    assert_eq!(f.getScript(), 0, "默认 mScript 应为 0");
    assert_eq!(f.getTypeface(), None, "默认 mTypeface 应为 null");
    assert_eq!(f.getScriptStr(), None, "默认 mScriptStr 应为 null");

    f.setScript(0x1234_5678);
    f.setTypeface(Some("Arial".to_string()));
    f.setScriptStr(Some("Latn".to_string()));

    assert_eq!(f.getScript(), 0x1234_5678);
    assert_eq!(f.getTypeface().as_deref(), Some("Arial"));
    assert_eq!(f.getScriptStr().as_deref(), Some("Latn"));
}

/// ST2 —— ★`FontCollection.addFont` 的 **Java 惰性初始化**：
/// 未 `addFont` 前 `getSupplementalFont()` 必须返回 `null`（不是空表）。
#[test]
fn st2_font_collection_lazy_init_is_null_before_add() {
    let mut c = FontCollection::new();
    assert!(
        c.getSupplementalFont().is_none(),
        "★Java: 未 addFont 前 mSupplementalFonts 为 null ⇒ getSupplementalFont() 返回 null"
    );

    let mut f1 = SupplementalFont::new();
    f1.setTypeface(Some("T1".to_string()));
    c.addFont(f1);

    let after = c.getSupplementalFont().expect("addFont 后应非 null");
    assert_eq!(after.len(), 1, "addFont 一次应有 1 个元素");
    assert_eq!(after[0].getTypeface().as_deref(), Some("T1"));

    let mut f2 = SupplementalFont::new();
    f2.setTypeface(Some("T2".to_string()));
    c.addFont(f2);

    let after2 = c.getSupplementalFont().unwrap();
    assert_eq!(after2.len(), 2, "★第二次 addFont 不得重置表（Java 只在 null 时 new）");
    assert_eq!(after2[1].getTypeface().as_deref(), Some("T2"));
}

/// ST3 —— `FontCollection` 三个字体槽位往返。
#[test]
fn st3_font_collection_font_slots() {
    let mut c = FontCollection::new();
    assert!(c.getLatinFont().is_none());
    assert!(c.getEastAsian().is_none());
    assert!(c.getComplexScript().is_none());

    let mut f: FontInfo = FontInfo::default();
    f.setFontName("Latin1".to_string());
    c.setLatinFont(Some(f.clone()));
    assert_eq!(c.getLatinFont().expect("latin").getFontName(), "Latin1");

    let mut f2: FontInfo = FontInfo::default();
    f2.setFontName("EA1".to_string());
    c.setEastAsian(Some(f2));
    assert_eq!(c.getEastAsian().expect("ea").getFontName(), "EA1");

    let mut f3: FontInfo = FontInfo::default();
    f3.setFontName("CS1".to_string());
    c.setComplexScript(Some(f3));
    assert_eq!(c.getComplexScript().expect("cs").getFontName(), "CS1");
}

/// ST4 —— `ThemeFontScheme` 3 个字段往返（`setX` 为非引用版本）。
#[test]
fn st4_theme_font_scheme_roundtrip() {
    let mut s = ThemeFontScheme::new();
    assert_eq!(s.getName(), None);
    assert!(s.getMajorFont().is_none());
    assert!(s.getMinorFont().is_none());

    s.setName(Some("Office".to_string()));
    s.setMajorFont(Some(FontCollection::new()));
    s.setMinorFont(Some(FontCollection::new()));

    assert_eq!(s.getName().as_deref(), Some("Office"));
    assert!(s.getMajorFont().is_some());
    assert!(s.getMinorFont().is_some());
}

/// ST5 —— ★★`ThemeFontScheme::setMajorFont_ref` 的**引用语义**（本文件核心用例）。
///
/// Java: `this.mFontCollection = new FontCollection(); scheme.setMajorFont(this.mFontCollection);`
/// 之后在 `mFontCollection` 上的写入必须**经 `scheme.getMajorFont()` 可见**。
/// 若实现为值语义克隆，本用例会在最后一步失败 —— 这正是要防的静默失效。
#[test]
fn st5_set_major_font_ref_shares_one_object() {
    let mut s = ThemeFontScheme::new();
    let ptr: *mut FontCollection = s.setMajorFont_ref(FontCollection::new());

    // 通过 importer 持有的指针写入（等价于 Java 侧 this.mFontCollection.setComplexScript）
    let mut cs: FontInfo = FontInfo::default();
    cs.setFontName("SharedCS".to_string());
    unsafe { (*ptr).setComplexScript(Some(cs)); }

    let seen = s.getMajorFont().expect("major font 应存在");
    assert_eq!(
        seen.getComplexScript().expect("★共享语义：应看到经指针写入的值").getFontName(),
        "SharedCS",
        "★setMajorFont_ref 必须与 scheme 内部是同一对象，否则 complexScript 写入丢失"
    );

    // 反证：minor 不受影响（证明没有串到别的槽位）
    assert!(
        s.getMinorFont().is_none(),
        "★写 major 不得影响 minor"
    );
}

/// ST6 —— `setMinorFont_ref` 同理。
#[test]
fn st6_set_minor_font_ref_shares_one_object() {
    let mut s = ThemeFontScheme::new();
    let ptr: *mut FontCollection = s.setMinorFont_ref(FontCollection::new());

    let mut ea: FontInfo = FontInfo::default();
    ea.setFontName("SharedEA".to_string());
    unsafe { (*ptr).setEastAsian(Some(ea)); }

    assert_eq!(
        s.getMinorFont().unwrap().getEastAsian().unwrap().getFontName(),
        "SharedEA"
    );
    assert!(s.getMajorFont().is_none(), "★写 minor 不得影响 major");
}

// ---------------------------------------------------------------------------
// ConvertHelper::getSupplementalFont
// ---------------------------------------------------------------------------

/// ST7 —— `getSupplementalFont` 的 `script.hashCode()` 必须是 **Java 的** String.hashCode。
///
/// Java `"Latn".hashCode()` = 2361039（`L*31^3 + a*31^2 + t*31 + n`）。
#[test]
fn st7_get_supplemental_font_uses_java_string_hash_code() {
    let a = Attrs::new(&[("script", "Latn"), ("typeface", "Arial")]);
    let f = ConvertHelper::getSupplementalFont(&a);

    let expected = "Latn"
        .bytes()
        .fold(0i32, |h, b| h.wrapping_mul(31).wrapping_add(b as i32));
    assert_eq!(expected, 2361039, "Java \"Latn\".hashCode() 应为 2361039");
    assert_eq!(f.getScript(), expected, "应使用 Java String.hashCode 算法");
    assert_eq!(f.getTypeface().as_deref(), Some("Arial"));
}

/// ST8 —— `getSupplementalFont` 缺属性时：script 缺失 ⇒ mScript 保持 0，typeface 为 null。
#[test]
fn st8_get_supplemental_font_missing_attrs() {
    let a = Attrs::new(&[("unrelated", "x")]);
    let f = ConvertHelper::getSupplementalFont(&a);
    assert_eq!(f.getScript(), 0, "★script 缺失 ⇒ 不调用 setScript，保持默认 0（非 hashCode(\"\")）");
    assert_eq!(f.getTypeface(), None);
}

// ---------------------------------------------------------------------------
// ConvertHelper::getTextFont / setPanose / convertPanose2Core / panoseItem
// ---------------------------------------------------------------------------

/// ST9 —— `getTextFont` 的 charset / pitchFamily / typeface（无 panose）。
#[test]
fn st9_get_text_font_basic_attrs() {
    let a = Attrs::new(&[("charset", "204"), ("pitchFamily", "34"), ("typeface", "Times New Roman")]);
    let fi = ConvertHelper::getTextFont(&a);
    assert_eq!(fi.getCharacterSet(), 204i16, "charset 应走 shortValue()");
    assert_eq!(fi.getFontFamily(), 34i8, "pitchFamily 应走 byteValue()");
    assert_eq!(fi.getFontName(), "Times New Roman");
}

/// ST10 —— `setPanose` 的 `length == 20` 门限：19/21 字符必须**不生效**。
#[test]
fn st10_set_panose_requires_exactly_20_chars() {
    let mut fi: FontInfo = FontInfo::default();

    // 19 字符 ⇒ 不转换
    ConvertHelper::setPanose("0202060305040502030", &mut fi);
    assert_eq!(fi.getPanoseFamilyType(), 0, "★19 字符不得触发转换");

    // 21 字符 ⇒ 也不转换（Java 用 == 而非 >=）
    ConvertHelper::setPanose("020206030504050203040", &mut fi);
    assert_eq!(fi.getPanoseFamilyType(), 0, "★21 字符不得触发转换（Java 是 == 20）");
}

/// ST11 —— ★★`convertPanose2Core` 的**忠实缺陷**：Java 原文
/// `setPanoseStrokeVariation` 内部写的是 `this.mPanoseArmStyle`。
///
/// 调用顺序（Java 原文）：… setPanoseArmStyle(armStyle) … 最后 setPanoseStrokeVariation(strokeVariation)。
/// ⇒ `mPanoseArmStyle` 被 **strokeVariation 覆盖**，而 `mPanoseStrokeVariation` 始终为 0。
/// 这不是移植 bug —— 原文即如此，属规格的一部分。
#[test]
fn st11_convert_panose2_core_preserves_java_arm_style_overwrite_quirk() {
    let mut fi: FontInfo = FontInfo::default();
    // familyType=02 serifStyle=02 weight=06 proportion=03 constrast=05
    // strokeVariation=04 armStyle=05 letterForm=02 midline=03 height=04
    ConvertHelper::setPanose("02020603050405020304", &mut fi);

    assert_eq!(fi.getPanoseFamilyType(), 2);
    assert_eq!(fi.getPanoseSerifStyle(), 2);
    assert_eq!(fi.getPanoseWeight(), 6);
    assert_eq!(fi.getPanoseProportion(), 3);
    assert_eq!(fi.getPanoseConstrast(), 5);
    assert_eq!(fi.getPanoseLetterform(), 2);
    assert_eq!(fi.getPanoseMidline(), 3);
    assert_eq!(fi.getPanoseHeight(), 4);

    // ★核心：armStyle 被 strokeVariation(4) 覆盖（Java 原文缺陷，忠实保留）
    assert_eq!(
        fi.getPanoseArmStyle(),
        4,
        "★Java 原文 setPanoseStrokeVariation 写的是 mPanoseArmStyle ⇒ 被 strokeVariation 覆盖"
    );
    assert_eq!(
        fi.getPanoseStrokeVariation(),
        0,
        "★mPanoseStrokeVariation 从未被写 ⇒ 保持默认 0"
    );
}

/// ST12 —— `getTextFont` 带 panose 属性：应经 `setPanose` 落到 FontInfo。
#[test]
fn st12_get_text_font_with_panose() {
    let a = Attrs::new(&[
        ("panose", "02020603050405020304"),
        ("typeface", "Times New Roman"),
        ("charset", "0"),
    ]);
    let fi = ConvertHelper::getTextFont(&a);
    assert_eq!(fi.getFontName(), "Times New Roman");
    assert_eq!(fi.getCharacterSet(), 0i16);
    assert_eq!(fi.getPanoseFamilyType(), 2, "panose 应已解析");
    assert_eq!(fi.getPanoseHeight(), 4);
    assert_eq!(fi.getPanoseArmStyle(), 4, "★含 quirk 覆盖");
}

/// ST13 —— `panoseItem` 的十六进制解析与越界行为。
#[test]
fn st13_panose_item_hex_parsing() {
    let v = "02020603050405020304";
    assert_eq!(ConvertHelper::panoseItem(v, 0), Some(2), "0x02");
    assert_eq!(ConvertHelper::panoseItem(v, 4), Some(6), "offset 4 ⇒ 0x06");
    assert_eq!(ConvertHelper::panoseItem(v, 18), Some(4), "offset 18 ⇒ 0x04");
    // 非十六进制 ⇒ hexString2Integer 返回 None ⇒ panoseItem 返回 None
    assert_eq!(ConvertHelper::panoseItem("ZZ020603050405020304", 0), None);
}

// ---------------------------------------------------------------------------
// FontSchemeImporter 端到端（真接线）
// ---------------------------------------------------------------------------

/// ST14 —— ★★`FontSchemeImporter` 端到端：`importMajorFontStart` 后
/// `importCSFontStart` / `importEAFontStart` / `importLatinFontStart` / `importFontStart`
/// 的写入必须经 `themeFontScheme.getMajorFont()` **可见**（Java 引用语义）。
#[test]
fn st14_font_scheme_importer_writes_are_visible_through_scheme() {
    let mut scheme = ThemeFontScheme::new();
    let scheme_ptr: *mut ThemeFontScheme = &mut scheme;
    let mut imp = FontSchemeImporter::FontSchemeImporter(scheme_ptr);

    // java this.mFontSchemeImporter.importFontSchemeStart(attributes) —— name
    let name_attrs = Attrs::new(&[("name", "Office")]);
    imp.importFontSchemeStart(&name_attrs);
    assert_eq!(
        scheme.getName().as_deref(),
        Some("Office"),
        "★importFontSchemeStart 应经指针落到 scheme"
    );

    // java importMajorFontStart(): new FontCollection + setMajorFont(引用)
    imp.importMajorFontStart();
    assert!(scheme.getMajorFont().is_some(), "★major 槽位应有对象");

    // java importCSFontStart(attributes) —— 写的是**同一个** FontCollection
    let cs_attrs = Attrs::new(&[("typeface", "CS-Face"), ("charset", "204")]);
    imp.importCSFontStart(&cs_attrs);

    let major = scheme.getMajorFont().expect("major");
    assert_eq!(
        major.getComplexScript().expect("★共享语义：CS 应经 scheme 可见").getFontName(),
        "CS-Face",
        "★若 FontSchemeImporter 用值语义，此处 complexScript 为 None —— 静默失效"
    );

    let ea_attrs = Attrs::new(&[("typeface", "EA-Face")]);
    imp.importEAFontStart(&ea_attrs);
    assert_eq!(
        scheme.getMajorFont().unwrap().getEastAsian().unwrap().getFontName(),
        "EA-Face"
    );

    let latin_attrs = Attrs::new(&[("typeface", "Latin-Face")]);
    imp.importLatinFontStart(&latin_attrs);
    assert_eq!(
        scheme.getMajorFont().unwrap().getLatinFont().unwrap().getFontName(),
        "Latin-Face"
    );

    // java importFontStart(attributes) —— addFont 到同一个 FontCollection
    let sup_attrs = Attrs::new(&[("script", "Latn"), ("typeface", "Sup-Face")]);
    imp.importFontStart(&sup_attrs);

    let sups = scheme
        .getMajorFont()
        .unwrap()
        .getSupplementalFont()
        .expect("★addFont 应经 scheme 可见");
    assert_eq!(sups.len(), 1);
    assert_eq!(sups[0].getTypeface().as_deref(), Some("Sup-Face"));
    assert_eq!(sups[0].getScript(), 2361039, "script 应为 Java hashCode");
}

/// ST15 —— `importMinorFontStart` 写的是 **minor** 槽位，不得串到 major。
#[test]
fn st15_import_minor_font_start_targets_minor_slot() {
    let mut scheme = ThemeFontScheme::new();
    let mut imp = FontSchemeImporter::FontSchemeImporter(&mut scheme);

    imp.importMajorFontStart();
    imp.importMinorFontStart();

    let minor_attrs = Attrs::new(&[("typeface", "Minor-Latin")]);
    imp.importLatinFontStart(&minor_attrs);

    assert_eq!(
        scheme.getMinorFont().unwrap().getLatinFont().unwrap().getFontName(),
        "Minor-Latin",
        "★importMinorFontStart 之后写入应落到 minor"
    );
    assert!(
        scheme.getMajorFont().unwrap().getLatinFont().is_none(),
        "★不得串写到 major"
    );
}
