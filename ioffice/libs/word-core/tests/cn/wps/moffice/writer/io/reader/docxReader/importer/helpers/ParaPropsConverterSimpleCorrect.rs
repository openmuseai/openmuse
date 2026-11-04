// function_id=07_function_correct_prop
// cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_correct_prop
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimpleCorrect.rs

#![allow(non_snake_case)]

use std::sync::Arc;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::data::value::TBD::TBD;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ParaPropsConverterSimple::ParaPropsConverterSimple;

#[test]
fn CP1_empty_set_gets_style_zero() {
    let mut dst = MutablePropertySet::new();
    assert!(!dst.contains(191));
    ParaPropsConverterSimple::correctProp(Some(&mut dst));
    assert!(dst.contains(191));
    assert_eq!(dst.getInt(191), 0);
}

#[test]
fn CP2_existing_style_is_kept() {
    let mut dst = MutablePropertySet::new();
    dst.setInt(191, 5);
    ParaPropsConverterSimple::correctProp(Some(&mut dst));
    assert_eq!(dst.getInt(191), 5);
}

#[test]
fn CP3_null_dst_does_not_panic() {
    ParaPropsConverterSimple::correctProp(None);
}

#[test]
fn CP4_no_tabs_does_not_write_236() {
    let mut dst = MutablePropertySet::new();
    ParaPropsConverterSimple::correctProp(Some(&mut dst));
    assert!(dst.contains(191));
    assert!(!dst.contains(236));
}

#[test]
fn CP5_explicit_zero_stays_zero() {
    let mut dst = MutablePropertySet::new();
    dst.setInt(191, 0);
    ParaPropsConverterSimple::correctProp(Some(&mut dst));
    assert!(dst.contains(191));
    assert_eq!(dst.getInt(191), 0);
    ParaPropsConverterSimple::correctProp(Some(&mut dst));
    assert_eq!(dst.getInt(191), 0);
}

#[test]
fn CP6_tabs_synthesize_236_and_clear_raw() {
    let mut dst = MutablePropertySet::new();
    dst.setValue(281, Some(Arc::new(vec![TBD::default()])));
    dst.setValue(280, Some(Arc::new(vec![0i32])));
    ParaPropsConverterSimple::correctProp(Some(&mut dst));
    assert!(dst.contains(236));
    assert!(dst.getValue(280).is_none());
    assert!(dst.getValue(281).is_none());
}
