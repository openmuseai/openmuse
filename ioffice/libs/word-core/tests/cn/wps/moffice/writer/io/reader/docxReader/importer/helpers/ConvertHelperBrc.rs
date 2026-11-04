// function_id=04_function_brc_shd
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_brc_shd
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelperBrc.rs

#![allow(non_snake_case)]

#[path = "wc03_word_attrs.rs"]
mod wc03_word_attrs;

use wc03_word_attrs::{WordAttrs, COLOR_AUTO, COLOR_FF0000};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{
    Attributes, ConvertHelper,
};

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }

    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
    }
}

#[test]
fn BS1_line_width_default_half_point() {
    let attrs = WordAttrs::empty();
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getLineWidth(), 0.5);
}

#[test]
fn BS2_sz_eight_is_one_point() {
    let attrs = WordAttrs::named("sz", "8");
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getLineWidth(), 1.0);
}

#[test]
fn BS3_val_none_type_zero() {
    let attrs = WordAttrs::named("val", "none");
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getBorderType(), 0);
}

#[test]
fn BS4_val_single_type_one() {
    let attrs = WordAttrs::named("val", "single");
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getBorderType(), 1);
}

#[test]
fn BS5_missing_val_type_zero() {
    let attrs = WordAttrs::empty();
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getBorderType(), 0);
}

#[test]
fn BS6_missing_color_is_auto_black() {
    let attrs = WordAttrs::empty();
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getColor(), COLOR_AUTO);
    assert_eq!(COLOR_AUTO, -16777216);
}

#[test]
fn BS7_missing_space_is_zero() {
    let attrs = WordAttrs::empty();
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert_eq!(brc.getSpace(), 0.0);
}

#[test]
fn BS8_missing_shadow_frame_are_false() {
    let attrs = WordAttrs::empty();
    let brc = ConvertHelper::toBRC(&attrs, false);
    assert!(!brc.isShadow());
    assert!(!brc.isFrame());
}

#[test]
fn BS9_empty_to_shd() {
    let attrs = WordAttrs::empty();
    let shd = ConvertHelper::toSHD(&attrs);
    assert_eq!(shd.getColorFore(), COLOR_AUTO);
    assert_eq!(shd.getColorBack(), COLOR_AUTO);
    assert_eq!(shd.getPattern(), -2);
}

#[test]
fn BS10_solid_ipat_one() {
    let attrs = WordAttrs::val("solid");
    let shd = ConvertHelper::toSHD(&attrs);
    assert_eq!(shd.getPattern(), 1);
}

#[test]
fn BS11_clear_ipat_zero() {
    let attrs = WordAttrs::val("clear");
    let shd = ConvertHelper::toSHD(&attrs);
    assert_eq!(shd.getPattern(), 0);
}

#[test]
fn BS12_fill_ff0000_is_color_back() {
    let attrs = WordAttrs::named("fill", "FF0000");
    let shd = ConvertHelper::toSHD(&attrs);
    assert_eq!(shd.getColorBack(), COLOR_FF0000);
    assert_eq!(COLOR_FF0000, 16711680);
    assert_eq!(shd.getColorFore(), COLOR_AUTO);
}
