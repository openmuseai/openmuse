// function_id=03_function_measure_color
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_measure_color
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelperMeasure.rs

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
fn MC1_get_long_missing_is_none() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::getLong(&attrs, "left").is_none());
}

#[test]
fn MC2_get_long_unitless_is_raw() {
    let attrs = WordAttrs::named("left", "1440");
    assert_eq!(ConvertHelper::getLong(&attrs, "left"), Some(1440));
}

#[test]
fn MC3_get_long_72pt_is_1440() {
    let attrs = WordAttrs::named("left", "72pt");
    assert_eq!(ConvertHelper::getLong(&attrs, "left"), Some(1440));
}

#[test]
fn MC4_get_long_12pt_is_240() {
    let attrs = WordAttrs::named("left", "12pt");
    assert_eq!(ConvertHelper::getLong(&attrs, "left"), Some(240));
}

#[test]
fn MC5_get_float_12pt() {
    let attrs = WordAttrs::named("sz", "12pt");
    assert_eq!(ConvertHelper::getFloat(&attrs, "sz"), Some(240.0));
}

#[test]
fn MC6_get_float_missing_is_none() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::getFloat(&attrs, "sz").is_none());
}

#[test]
fn MC7_get_int_72pt() {
    let attrs = WordAttrs::named("left", "72pt");
    assert_eq!(ConvertHelper::getInt(&attrs, "left"), Some(1440));
}

#[test]
fn MC8_to_color_missing_is_auto_black() {
    let attrs = WordAttrs::empty();
    assert_eq!(ConvertHelper::toColor(&attrs, "color"), COLOR_AUTO);
    assert_eq!(COLOR_AUTO, -16777216);
}

#[test]
fn MC9_to_color_auto() {
    assert_eq!(
        ConvertHelper::toColor(&WordAttrs::named("color", "auto"), "color"),
        COLOR_AUTO
    );
    assert_eq!(
        ConvertHelper::toColor(&WordAttrs::named("color", "AUTO"), "color"),
        COLOR_AUTO
    );
}

#[test]
fn MC10_to_color_hex_ff0000() {
    let attrs = WordAttrs::named("color", "FF0000");
    assert_eq!(ConvertHelper::toColor(&attrs, "color"), COLOR_FF0000);
    assert_eq!(COLOR_FF0000, 16711680);
}

#[test]
fn MC11_to_color_bad_hex_is_minus_two() {
    let attrs = WordAttrs::named("color", "xyz");
    assert_eq!(ConvertHelper::toColor(&attrs, "color"), -2);
}

#[test]
fn MC12_hex2int_missing_is_none() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::hex2Int(&attrs, "color").is_none());
}
