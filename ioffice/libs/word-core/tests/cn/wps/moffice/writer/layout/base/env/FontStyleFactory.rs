// function_id=05_function_font_style_factory
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::{empty, PropertySet};
use word_core::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument as StyleTextDocument};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;

fn freeze_float(id: i32, v: f32) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setFloat(id, v);
    m.freeze()
}

fn heading_styles() -> KStyles {
    let td = StyleTextDocument::default();
    let mut styles = KStyles::KStyles(&td);
    let mut normal = KStyle::KStyle();
    normal.setId(0);
    normal.setBase(4095);
    normal.setProperty(&freeze_float(10, 10.0));
    styles.add(&mut normal);
    let mut heading = KStyle::KStyle();
    heading.setId(1);
    heading.setBase(0);
    let mut m = MutablePropertySet::new();
    m.setFloat(10, 16.0);
    m.setBoolean(5, true);
    heading.setProperty(&m.freeze());
    styles.add(&mut heading);
    styles
}

#[test]
fn FF1_defaults() {
    let s = FontStyleFactory::createMin(&*empty, 0);
    assert_eq!(s.fontSize, 200);
    assert!(!s.bold);
    assert!(!s.italic);
    assert_eq!(s.ascii, "Times New Roman");
    assert_eq!(s.eastAsia, "宋体");
    assert_eq!(s.fontName, "Times New Roman");
}

#[test]
fn FF2_direct_run_props() {
    let mut m = MutablePropertySet::new();
    m.setBoolean(5, true);
    m.setBoolean(6, true);
    m.setFloat(10, 12.0);
    m.setString(3, "Arial".to_string());
    let s = FontStyleFactory::createMin(&m.freeze(), 0);
    assert_eq!(s.fontSize, 240);
    assert!(s.bold);
    assert!(s.italic);
    assert_eq!(s.ascii, "Arial");
    assert_eq!(s.fontName, "Arial");
}

#[test]
fn FF3_cjk_resolved_name() {
    let mut m = MutablePropertySet::new();
    m.setString(4, "黑体".to_string());
    let s = FontStyleFactory::createMin(&m.freeze(), 2);
    assert_eq!(s.fontName, "黑体");
    assert_eq!(s.eastAsia, "黑体");
}

#[test]
fn FF4_hint_switches_to_east_asia() {
    let mut m = MutablePropertySet::new();
    m.setString(3, "Arial".to_string());
    m.setString(4, "黑体".to_string());
    m.setInt(40, 1);
    let s = FontStyleFactory::createMin(&m.freeze(), 0);
    assert_eq!(s.fontName, "黑体");
}

#[test]
fn FF5_heading_from_run() {
    let styles = heading_styles();
    let mut papx = MutablePropertySet::new();
    papx.setInt(191, 1);
    let s = FontStyleFactory::createFromRun(&*empty, &papx.freeze(), Some(&styles), 0);
    assert_eq!(s.fontSize, 320);
    assert!(s.bold);
}

#[test]
fn FF6_direct_italic_on_heading() {
    let styles = heading_styles();
    let mut papx = MutablePropertySet::new();
    papx.setInt(191, 1);
    let mut chpx = MutablePropertySet::new();
    chpx.setBoolean(6, true);
    let s = FontStyleFactory::createFromRun(&chpx.freeze(), &papx.freeze(), Some(&styles), 0);
    assert!(s.italic);
    assert_eq!(s.fontSize, 320);
    assert!(s.bold);
}

#[test]
fn FF7_optional_color_underline() {
    let mut m = MutablePropertySet::new();
    m.setInt(11, 255);
    m.setInt(8, 1);
    let s = FontStyleFactory::createMin(&m.freeze(), 0);
    assert_eq!(s.fontColor, 255);
    assert_eq!(s.underline, 1);
}
