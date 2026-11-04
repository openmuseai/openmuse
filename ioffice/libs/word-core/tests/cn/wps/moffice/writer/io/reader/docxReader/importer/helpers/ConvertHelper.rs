// function_id=02_function_attr_parse
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_attr_parse
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.rs

#![allow(non_snake_case)]

#[path = "wc03_word_attrs.rs"]
mod wc03_word_attrs;

use wc03_word_attrs::{WordAttrs, HASH_LEFT, MAIN_NS};
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
fn AP1_empty_get_string_is_none() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::getString(&attrs).is_none());
}

#[test]
fn AP2_get_string_reads_val() {
    let attrs = WordAttrs::val("Normal");
    assert_eq!(
        ConvertHelper::getString(&attrs).as_deref(),
        Some("Normal")
    );
}

#[test]
fn AP3_get_string_named_miss_left() {
    let attrs = WordAttrs::val("Normal");
    assert!(attrs.get_value(MAIN_NS, "left").is_none());
    assert!(ConvertHelper::getString__String(&attrs, "left").is_none());
}

#[test]
fn AP4_empty_to_boolean_is_true() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::toBoolean(&attrs));
}

#[test]
fn AP5_empty_string_to_boolean_is_true() {
    let attrs = WordAttrs::val("");
    assert!(ConvertHelper::toBoolean(&attrs));
}

#[test]
fn AP6_to_boolean_false_tokens() {
    for v in ["off", "0", "false", "f", "FALSE", "OFF", "False", "F"] {
        assert!(
            !ConvertHelper::toBoolean(&WordAttrs::val(v)),
            "expected false for val={v}"
        );
    }
}

#[test]
fn AP7_to_boolean_true_tokens() {
    for v in ["true", "on", "1"] {
        assert!(
            ConvertHelper::toBoolean(&WordAttrs::val(v)),
            "expected true for val={v}"
        );
    }
}

#[test]
fn AP8_get_boolean_miss_is_none() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::getBoolean(&attrs, "w").is_none());
}

#[test]
fn AP9_to_integer_miss_is_none() {
    let attrs = WordAttrs::empty();
    assert!(ConvertHelper::toInteger(&attrs).is_none());
}

#[test]
fn AP10_to_integer_val_two() {
    let attrs = WordAttrs::val("2");
    assert_eq!(ConvertHelper::toInteger(&attrs), Some(2));
}

#[test]
fn AP11_empty_hash_is_minus_one() {
    let attrs = WordAttrs::empty();
    assert_eq!(ConvertHelper::getStringHashCode(&attrs), -1);
}

#[test]
fn AP12_hash_left_is_java_hashcode() {
    let attrs = WordAttrs::val("left");
    assert_eq!(ConvertHelper::getStringHashCode(&attrs), HASH_LEFT);
    assert_eq!(HASH_LEFT, 3317767);
}
