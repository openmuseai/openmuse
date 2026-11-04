// function_id=05_function_kstyles_table
// cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_kstyles_table
// src:  libs/word-core/src/cn/wps/moffice/writer/core/KStyles.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/core/KStyles.rs

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;

fn freeze_int(prop: i32, val: i32) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(prop, val);
    mps.freeze()
}

fn freeze_string(prop: i32, val: &str) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setString(prop, val.to_string());
    mps.freeze()
}

fn style(id: i32, name: &str) -> KStyle {
    let ps = freeze_int(193, 3);
    KStyle::new__int_string_int_int_int_propertyset(id, name, 4095, 4095, 1, Some(&ps))
}

fn sheet() -> KStyles {
    KStyles::KStyles(&TextDocument::default())
}

#[test]
fn KT1_add_new_id_sets_stylesheet() {
    let mut styles = sheet();
    let mut h1 = style(1, "heading 1");
    styles.add(&mut h1);
    assert_eq!(styles.size(), 1);
    let got = styles.getStyleById__int(1).expect("added");
    assert_eq!(got.getId(), 1);
    assert_eq!(got.getName(), "heading 1");
}

#[test]
fn KT2_add_same_id_overlays() {
    let mut styles = sheet();
    let mut a = style(1, "A");
    let mut b = style(1, "B");
    styles.add(&mut a);
    styles.add(&mut b);
    assert_eq!(styles.size(), 1);
    assert_eq!(styles.getStyleById__int(1).unwrap().getName(), "B");
}

#[test]
fn KT3_get_style_by_id_hit() {
    let mut styles = sheet();
    let mut h1 = style(1, "heading 1");
    styles.add(&mut h1);
    assert!(styles.getStyleById__int(1).is_some());
}

#[test]
fn KT4_miss_force_normal_falls_back() {
    let mut styles = sheet();
    let mut normal = style(0, "Normal");
    styles.add(&mut normal);
    let miss = styles.getStyleById__int(99).expect("force Normal");
    assert_eq!(miss.getId(), 0);
    let miss_true = styles.getStyleById__int_boolean(99, true).expect("force");
    assert_eq!(miss_true.getId(), 0);
}

#[test]
fn KT5_miss_without_force_is_null() {
    let mut styles = sheet();
    let mut normal = style(0, "Normal");
    styles.add(&mut normal);
    assert!(styles.getStyleById__int_boolean(99, false).is_none());
}

#[test]
fn KT6_miss_id_zero_does_not_recurse() {
    let styles = sheet();
    assert!(styles.getStyleById__int(0).is_none());
    assert!(styles.getStyleById__int_boolean(0, true).is_none());
}

#[test]
fn KT7_get_style_by_name_null() {
    let mut styles = sheet();
    let mut normal = style(0, "Normal");
    styles.add(&mut normal);
    assert!(styles.getStyleByName(None).is_none());
}

#[test]
fn KT8_get_style_by_name_hit() {
    let mut styles = sheet();
    let mut h1 = style(1, "heading 1");
    styles.add(&mut h1);
    let got = styles.getStyleByName(Some("heading 1")).expect("name hit");
    assert_eq!(got.getId(), 1);
}

#[test]
fn KT9_get_style_by_name_miss_falls_back_to_normal() {
    let mut styles = sheet();
    let mut normal = style(0, "Normal");
    styles.add(&mut normal);
    let got = styles.getStyleByName(Some("zzz")).expect("fallback Normal");
    assert_eq!(got.getId(), 0);
}

#[test]
fn KT10_rpr_default_writes_only_if_empty() {
    let mut styles = sheet();
    let a = freeze_string(3, "Arial");
    let b = freeze_string(3, "Times");
    styles.setRPrDefault(&a);
    styles.setRPrDefault(&b);
    assert_eq!(styles.getRPrDefault().getString(3), "Arial");
}

#[test]
fn KT11_ppr_default_writes_only_if_empty() {
    let mut styles = sheet();
    let a = freeze_int(191, 0);
    let b = freeze_int(191, 1);
    styles.setPPrDefault(&a);
    styles.setPPrDefault(&b);
    assert_eq!(styles.getPPrDefault().getInt(191), 0);
}
