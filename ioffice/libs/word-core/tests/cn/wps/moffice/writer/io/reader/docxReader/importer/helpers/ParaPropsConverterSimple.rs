// function_id=06_function_para_convert
// cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_para_convert
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs

#![allow(non_snake_case)]

#[path = "wc03_word_attrs.rs"]
mod wc03_word_attrs;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use wc03_word_attrs::{
    EID_ILVL, EID_IND, EID_JC, EID_NUMID, EID_PSTYLE, EID_SPACING, HASH_CENTER, HASH_LEFT, WordAttrs,
};
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ParaPropsConverterSimple::ParaPropsConverterSimple;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }

    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
    }
}

fn converter() -> ParaPropsConverterSimple {
    ParaPropsConverterSimple::ParaPropsConverterSimple(0, None, None)
}

fn empty_style_map() -> StyleId2Sti {
    StyleId2Sti::StyleId2Sti(UUSti::UUSti())
}

fn empty_num_map() -> NumIdRedirection {
    NumIdRedirection::NumIdRedirection(UUNumberingId::new())
}

#[test]
fn PP1_pstyle_hit_writes_sti() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let mut styles = empty_style_map();
    styles.put__string_integer("heading1", Some(1));
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("heading1");
    conv.convertProp(&mut dst, EID_PSTYLE, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(191), 1);
}

#[test]
fn PP2_pstyle_miss_writes_zero() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("missing");
    conv.convertProp(&mut dst, EID_PSTYLE, &attrs, &styles, &mut nums);
    assert!(dst.contains(191));
    assert_eq!(dst.getInt(191), 0);
}

#[test]
fn PP3_jc_left_is_zero() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("left");
    conv.convertProp(&mut dst, EID_JC, &attrs, &styles, &mut nums);
    assert_eq!(attrs.get_value(wc03_word_attrs::MAIN_NS, "val").unwrap().chars().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32)), HASH_LEFT);
    assert_eq!(dst.getInt(193), 0);
}

#[test]
fn PP4_jc_center_is_one() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("center");
    conv.convertProp(&mut dst, EID_JC, &attrs, &styles, &mut nums);
    let _ = HASH_CENTER;
    assert_eq!(dst.getInt(193), 1);
}

#[test]
fn PP5_jc_missing_val_does_not_write() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::empty();
    conv.convertProp(&mut dst, EID_JC, &attrs, &styles, &mut nums);
    assert!(!dst.contains(193));
}

#[test]
fn PP6_ind_left_1440_writes_210() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("left", "1440");
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(210), 1440);
    assert!(!dst.contains(208));
}

#[test]
fn PP7_ilvl_two() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("2");
    conv.convertProp(&mut dst, EID_ILVL, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(234), 2);
}

#[test]
fn PP8_numid_zero_writes_zero_without_put() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("0");
    conv.convertProp(&mut dst, EID_NUMID, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(235), 0);
    assert!(nums.getId(0).is_none());
}

#[test]
fn PP9_numid_ten_first_put_is_one() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::val("10");
    conv.convertProp(&mut dst, EID_NUMID, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(235), 1);
    assert_eq!(nums.getId(10), Some(1));
}

#[test]
fn PP10_unknown_element_does_not_panic_or_write() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::empty();
    conv.convertProp(&mut dst, 1, &attrs, &styles, &mut nums);
    assert!(!dst.contains(191));
    assert!(!dst.contains(193));
}

#[test]
fn IN1_firstLine_480_writes_208() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("firstLine", "480");
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert!(dst.contains(208));
    assert_eq!(dst.getInt(208), 480);
}

#[test]
fn IN2_left_360_writes_210() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("left", "360");
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(210), 360);
    assert!(!dst.contains(208));
}

#[test]
fn IN3_right_240_writes_212() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("right", "240");
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(212), 240);
}

#[test]
fn IN4_no_ind_attrs_does_not_write() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::empty();
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert!(!dst.contains(208));
    assert!(!dst.contains(210));
    assert!(!dst.contains(212));
}

#[test]
fn IN5_hanging_480_writes_neg_208() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("hanging", "480");
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(208), -480);
}

#[test]
fn IN6_hangingChars_100_writes_neg_209() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("hangingChars", "100");
    conv.convertProp(&mut dst, EID_IND, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(209), -100);
}

#[test]
fn SP1_before_writes_217() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("before", "240");
    conv.convertProp(&mut dst, EID_SPACING, &attrs, &styles, &mut nums);
    assert!(dst.contains(217));
    assert_eq!(dst.getInt(217), 240);
}

#[test]
fn SP2_after_writes_214() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("after", "120");
    conv.convertProp(&mut dst, EID_SPACING, &attrs, &styles, &mut nums);
    assert_eq!(dst.getInt(214), 120);
}

#[test]
fn SP3_line_auto_writes_220_object() {
    use word_core::cn::wps::moffice::writer::data::value::LineSpacing::LineSpacing;
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::named("line", "240").with(wc03_word_attrs::MAIN_NS, "lineRule", "auto");
    conv.convertProp(&mut dst, EID_SPACING, &attrs, &styles, &mut nums);
    let pv = dst.getValue(220).expect("sprm 220");
    let ls = pv
        .as_ref()
        .as_any()
        .downcast_ref::<LineSpacing>()
        .expect("LineSpacing");
    assert_eq!(ls.getRule(), 0);
    assert!((ls.getSpacing() - 1.0).abs() < 1e-5);
}

#[test]
fn SP4_no_spacing_attrs_does_not_write() {
    let conv = converter();
    let mut dst = MutablePropertySet::new();
    let styles = empty_style_map();
    let mut nums = empty_num_map();
    let attrs = WordAttrs::empty();
    conv.convertProp(&mut dst, EID_SPACING, &attrs, &styles, &mut nums);
    assert!(!dst.contains(217));
    assert!(!dst.contains(214));
    assert!(!dst.contains(220));
}
