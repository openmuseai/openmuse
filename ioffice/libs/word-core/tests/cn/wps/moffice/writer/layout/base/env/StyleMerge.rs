// function_id=04_function_style_merge
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::{empty, PropertySet};
use word_core::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument as StyleTextDocument};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::layout::base::env::StyleMerge::StyleMerge;

fn freeze_float(id: i32, v: f32) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setFloat(id, v);
    m.freeze()
}

fn freeze_bool(id: i32, v: bool) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setBoolean(id, v);
    m.freeze()
}

fn freeze_int(id: i32, v: i32) -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(id, v);
    m.freeze()
}

fn chp_heading() -> PropertySet {
    let mut m = MutablePropertySet::new();
    m.setFloat(10, 16.0);
    m.setBoolean(5, true);
    m.freeze()
}

fn make_styles() -> KStyles {
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
    heading.setProperty(&chp_heading());
    styles.add(&mut heading);
    styles
}

fn papx_style(id: i32) -> PropertySet {
    freeze_int(191, id)
}

#[test]
fn SM1_no_styles_returns_chpx() {
    let chpx = freeze_bool(5, true);
    let got = StyleMerge::merge(&chpx, &*empty, None);
    assert!(got.getBoolean__int_boolean(5, false));
}

#[test]
fn SM2_rpr_default() {
    let td = StyleTextDocument::default();
    let mut styles = KStyles::KStyles(&td);
    styles.setRPrDefault(&freeze_float(10, 11.0));
    let got = StyleMerge::merge(&*empty, &*empty, Some(&styles));
    assert_eq!(got.getFloat__int_float(10, -1.0), 11.0);
}

#[test]
fn SM3_heading_based_on() {
    let styles = make_styles();
    let got = StyleMerge::merge(&*empty, &papx_style(1), Some(&styles));
    assert_eq!(got.getFloat__int_float(10, -1.0), 16.0);
    assert!(got.getBoolean__int_boolean(5, false));
}

#[test]
fn SM4_direct_size_overrides_heading() {
    let styles = make_styles();
    let got = StyleMerge::merge(&freeze_float(10, 14.0), &papx_style(1), Some(&styles));
    assert_eq!(got.getFloat__int_float(10, -1.0), 14.0);
    assert!(got.getBoolean__int_boolean(5, false));
}

#[test]
fn SM5_empty_chpx_keeps_heading_bold() {
    let styles = make_styles();
    let got = StyleMerge::merge(&*empty, &papx_style(1), Some(&styles));
    assert!(got.getBoolean__int_boolean(5, false));
}

#[test]
fn SM6_missing_pstyle_skips_heading() {
    let styles = make_styles();
    let got = StyleMerge::merge(&*empty, &*empty, Some(&styles));
    assert_eq!(got.getFloat__int_float(10, -1.0), -1.0);
}

#[test]
fn SM7_char_style_bold() {
    let td = StyleTextDocument::default();
    let mut styles = KStyles::KStyles(&td);
    let mut cs = KStyle::KStyle();
    cs.setId(2);
    cs.setBase(4095);
    cs.setProperty(&freeze_bool(5, true));
    styles.add(&mut cs);
    let chpx = freeze_int(2, 2);
    let got = StyleMerge::merge(&chpx, &*empty, Some(&styles));
    assert!(got.getBoolean__int_boolean(5, false));
}
