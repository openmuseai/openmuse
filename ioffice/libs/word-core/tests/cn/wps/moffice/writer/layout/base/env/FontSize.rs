// function_id=01_function_font_size
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::layout::base::env::FontSize::FontSize;

fn ps_float(id: i32, v: f32) -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setFloat(id, v);
    m.freeze()
}

#[test]
fn FS1_default_10pt() {
    assert_eq!(FontSize::getFontSize(&*empty, 0), 200);
}

#[test]
fn FS2_twelve_pt() {
    assert_eq!(FontSize::getFontSize(&ps_float(10, 12.0), 0), 240);
}

#[test]
fn FS3_heading_16pt() {
    assert_eq!(FontSize::getFontSize(&ps_float(10, 16.0), 0), 320);
}

#[test]
fn FS4_complex_script_szcs() {
    assert_eq!(FontSize::getFontSize(&ps_float(37, 14.0), 5), 280);
}

#[test]
fn FS5_complex_fallback_to_sz() {
    assert_eq!(FontSize::getFontSize(&ps_float(10, 12.0), 5), 240);
}

#[test]
fn FS6_zero_falls_back_to_10pt() {
    assert_eq!(FontSize::getFontSize(&ps_float(10, 0.0), 0), 200);
}

#[test]
fn FS7_max_font_size_clamp() {
    let got = FontSize::getFontSizeLimited(&ps_float(10, 12.0), 0, 100);
    assert_eq!(got, 100);
}

#[test]
fn FS8_is_complex_script() {
    assert!(!FontSize::isComplexScript(0));
    assert!(!FontSize::isComplexScript(2));
    assert!(FontSize::isComplexScript(5));
}
