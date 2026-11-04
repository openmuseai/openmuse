// function_id=05_function_run_highfreq
// cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_run_highfreq
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/RunPropsConverterSimple.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/RunPropsConverterSimple.rs

#![allow(non_snake_case)]

#[path = "wc03_word_attrs.rs"]
mod wc03_word_attrs;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use wc03_word_attrs::{COLOR_FF0000, EID_B, EID_COLOR, EID_I, EID_RFONTS, EID_RSTYLE, EID_SZ, WordAttrs};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::RunPropsConverterSimple::RunPropsConverterSimple;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ThemeFontTool::ThemeFontTool;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::data::theme::font::FontCollection::FontCollection;
use word_core::cn::wps::moffice::writer::data::theme::font::SupplementalFont::SupplementalFont;
use word_core::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }

    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
    }
}

fn converter() -> RunPropsConverterSimple {
    RunPropsConverterSimple::RunPropsConverterSimple(0, None, None)
}

fn empty_style_map() -> StyleId2Sti {
    StyleId2Sti::StyleId2Sti(UUSti::UUSti())
}

fn mock_doc() -> TextDocument {
    TextDocument::TextDocument__boolean(true)
}

#[test]
fn RP1_bold_empty_val_is_true_on_prop_five() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::empty();
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_B, &attrs, &styles));
    assert!(dst.getBoolean(5));
}

#[test]
fn RP2_bold_off_is_false() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::val("off");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_B, &attrs, &styles));
    assert!(!dst.getBoolean(5));
}

#[test]
fn RP3_italic_prop_six() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::empty();
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_I, &attrs, &styles));
    assert!(dst.getBoolean(6));
}

#[test]
fn RP4_sz_24_half_points_is_12() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::val("24");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_SZ, &attrs, &styles));
    assert_eq!(dst.getFloat(10), 12.0);
}

#[test]
fn RP5_color_hex_ff0000() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::val("FF0000");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_COLOR, &attrs, &styles));
    assert_eq!(dst.getInt(11), COLOR_FF0000);
}

#[test]
fn RP6_rstyle_hit_writes_prop_two() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let mut styles = empty_style_map();
    styles.put__string_integer("heading1", Some(1));
    let attrs = WordAttrs::val("heading1");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_RSTYLE, &attrs, &styles));
    assert_eq!(dst.getInt(2), 1);
}

#[test]
fn RP7_rstyle_miss_does_not_write_two() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::val("missing");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_RSTYLE, &attrs, &styles));
    assert!(!dst.contains(2));
}

#[test]
fn RP8_unknown_id_returns_false_empty_dst() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let attrs = WordAttrs::empty();
    assert!(!conv.convertRunPropsBase(&mut doc, &mut dst, 1, &attrs, &styles));
    assert!(!dst.contains(5));
    assert!(!dst.contains(6));
    assert!(!dst.contains(10));
}

#[test]
fn RP9_unknown_id_keeps_existing_bold() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let empty = WordAttrs::empty();
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_B, &empty, &styles));
    assert!(dst.getBoolean(5));
    assert!(!conv.convertRunPropsBase(&mut doc, &mut dst, 1, &empty, &styles));
    assert!(dst.getBoolean(5));
}

#[test]
fn RP10_rfonts_preserves_java_direct_names_and_east_asia_hint() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let attrs = WordAttrs::named("ascii", "宋体")
        .with(wc03_word_attrs::MAIN_NS, "hAnsi", "宋体")
        .with(wc03_word_attrs::MAIN_NS, "hint", "eastAsia");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_RFONTS, &attrs, &empty_style_map()));
    assert_eq!(dst.getString(3), "宋体");
    assert_eq!(dst.getString(35), "宋体");
    assert_eq!(dst.getInt(40), 1);
}

#[test]
fn RP11_rfonts_xml03_alias_and_hint_mapping() {
    let conv = converter();
    let mut doc = mock_doc();
    let mut dst = MutablePropertySet::new();
    let attrs = WordAttrs::named("h-ansi", "Times New Roman")
        .with(wc03_word_attrs::MAIN_NS, "fareast", "宋体")
        .with(wc03_word_attrs::MAIN_NS, "cs", "Arial")
        .with(wc03_word_attrs::MAIN_NS, "hint", "cs");
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_RFONTS, &attrs, &empty_style_map()));
    assert_eq!(dst.getString(35), "Times New Roman");
    assert_eq!(dst.getString(4), "宋体");
    assert_eq!(dst.getString(36), "Arial");
    assert_eq!(dst.getInt(40), 2);
    assert_eq!(conv.toHint("default"), 0);
    assert_eq!(conv.toHint("fareast"), 1);
}

fn font_info(name: &str) -> FontInfo {
    let mut info = FontInfo::default();
    info.setFontName(name.to_string());
    info
}

#[test]
fn RP12_rfonts_theme_names_override_direct_names_in_java_order() {
    let conv = converter();
    let mut doc = mock_doc();
    let theme = doc.getTheme_mut().expect("Java document theme");
    let scheme = unsafe { &mut *(*theme).getThemeFontScheme_mut() };
    let mut major = FontCollection::new();
    major.setLatinFont(Some(font_info("MajorLatin")));
    major.setEastAsian(Some(font_info("MajorEastAsia")));
    let mut minor = FontCollection::new();
    minor.setLatinFont(Some(font_info("MinorLatin")));
    minor.setComplexScript(Some(font_info("MinorBidi")));
    scheme.setMajorFont(Some(major));
    scheme.setMinorFont(Some(minor));
    let attrs = WordAttrs::named("ascii", "DirectLatin")
        .with(wc03_word_attrs::MAIN_NS, "asciiTheme", "majorAscii")
        .with(wc03_word_attrs::MAIN_NS, "hAnsiTheme", "minorHAnsi")
        .with(wc03_word_attrs::MAIN_NS, "eastAsiaTheme", "majorEastAsia")
        .with(wc03_word_attrs::MAIN_NS, "cstheme", "minorBidi");
    let mut dst = MutablePropertySet::new();
    assert!(conv.convertRunPropsBase(&mut doc, &mut dst, EID_RFONTS, &attrs, &empty_style_map()));
    assert_eq!(dst.getString(3), "MajorLatin");
    assert_eq!(dst.getString(35), "MinorLatin");
    assert_eq!(dst.getString(4), "MajorEastAsia");
    assert_eq!(dst.getString(36), "MinorBidi");
    assert!(doc.getFontTable().get__string_opt("MajorLatin").is_some());
}

#[test]
fn RP13_theme_supplemental_font_uses_document_language_and_registers_name() {
    let mut doc = mock_doc();
    let mut document_props = MutablePropertySet::new();
    document_props.setString(572, "zh-CN".to_string());
    doc.setProperty__propertyset(&document_props.freeze());
    let theme = doc.getTheme_mut().expect("Java document theme");
    let scheme = unsafe { &mut *(*theme).getThemeFontScheme_mut() };
    let mut major = FontCollection::new();
    major.setEastAsian(Some(font_info("DefaultEastAsia")));
    let mut supplemental = SupplementalFont::new();
    supplemental.setScript(2241684); // Java HANI, first script for LID 2052.
    supplemental.setTypeface(Some("SupplementalHan".to_string()));
    major.addFont(supplemental);
    scheme.setMajorFont(Some(major));
    assert_eq!(ThemeFontTool::toFontsTheme("majorEastAsia", &mut doc).as_deref(), Some("SupplementalHan"));
    assert!(doc.getFontTable().get__string_opt("SupplementalHan").is_some());
}
