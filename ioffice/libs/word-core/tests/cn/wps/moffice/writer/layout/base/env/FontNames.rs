// function_id=03_function_font_names
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::layout::base::env::FontNames::FontNames;

fn latin_cjk() -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setString(3, "Arial".to_string());
    m.setString(4, "黑体".to_string());
    m.freeze()
}

#[test]
fn FN1_defaults() {
    assert_eq!(FontNames::getAscii(&*empty), "Times New Roman");
    assert_eq!(FontNames::getEastAsia(&*empty), "宋体");
}

#[test]
fn FN2_ascii() {
    let mut m = MutablePropertySet::new();
    m.setString(3, "Arial".to_string());
    assert_eq!(FontNames::getAscii(&m.freeze()), "Arial");
}

#[test]
fn FN3_east_asia() {
    let mut m = MutablePropertySet::new();
    m.setString(4, "黑体".to_string());
    assert_eq!(FontNames::getEastAsia(&m.freeze()), "黑体");
}

#[test]
fn FN4_latin_resolved() {
    let ps = latin_cjk();
    assert_eq!(FontNames::getResolvedFontName(&ps, 0), "Arial");
}

#[test]
fn FN5_cjk_resolved() {
    let ps = latin_cjk();
    assert_eq!(FontNames::getResolvedFontName(&ps, 2), "黑体");
}

#[test]
fn FN6_hint_fareast() {
    let mut m = MutablePropertySet::new();
    m.setString(3, "Arial".to_string());
    m.setString(4, "黑体".to_string());
    m.setInt(40, 1);
    let ps = m.freeze();
    assert_eq!(FontNames::applyHint(&ps, 0), 2);
    assert_eq!(FontNames::getResolvedFontName(&ps, 0), "黑体");
}

#[test]
fn FN7_complex_slot_36() {
    let mut m = MutablePropertySet::new();
    m.setString(36, "Tahoma".to_string());
    assert_eq!(FontNames::getResolvedFontName(&m.freeze(), 5), "Tahoma");
}
