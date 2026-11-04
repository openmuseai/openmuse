// function_id=04_function_kstyle_final
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_kstyle_final
// src:  libs/word-core/src/cn/wps/moffice/writer/core/KStyle.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/core/KStyle.rs

use std::sync::Arc;
use stubs::java::lang::Object::Object;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument, Toggle};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;

fn freeze_float(prop: i32, val: f32) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setFloat(prop, val);
    mps.freeze()
}

fn freeze_string(prop: i32, val: &str) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setString(prop, val.to_string());
    mps.freeze()
}

fn freeze_int(prop: i32, val: i32) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(prop, val);
    mps.freeze()
}

fn freeze_toggle(prop: i32, t: Toggle) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setValue(prop, Some(Arc::new(t)));
    mps.freeze()
}

fn style(id: i32, name: &str, base: i32, property: &PropertySet) -> KStyle {
    KStyle::new__int_string_int_int_int_propertyset(id, name, base, 4095, 1, Some(property))
}

fn sheet() -> KStyles {
    let td = TextDocument::default();
    KStyles::KStyles(&td)
}

fn toggle_of(ps: &PropertySet, id: i32) -> Option<Toggle> {
    ps.getValue(id)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<Toggle>().copied())
}

#[test]
fn KS1_no_stylesheet_returns_property() {
    let ps = freeze_float(10, 10.5);
    let mut s = style(1, "Heading1", 4095, &ps);
    let final_ps = s.getFinalProperty().expect("property present");
    assert!((final_ps.getFloat(10) - 10.5).abs() < 1e-4);
    assert!(final_ps.contains(10));
}

#[test]
fn KS2_set_property_invalidates_cache() {
    let mut styles = sheet();
    let ps = freeze_float(10, 10.5);
    let mut s = style(1, "H", 4095, &ps);
    styles.add(&mut s);
    let first = s.getFinalProperty().expect("first final");
    assert!((first.getFloat(10) - 10.5).abs() < 1e-4);
    let again = s.getFinalProperty().expect("cached final");
    assert!((again.getFloat(10) - 10.5).abs() < 1e-4);
    let newer = freeze_float(10, 12.0);
    s.setProperty(&newer);
    let after = s.getFinalProperty().expect("recomputed final");
    assert!((after.getFloat(10) - 12.0).abs() < 1e-4);
}

#[test]
fn KS3_base_sti_nil_does_not_merge() {
    let mut styles = sheet();
    let normal_ps = freeze_string(3, "Times");
    let mut normal = style(0, "Normal", 4095, &normal_ps);
    let child_ps = freeze_float(10, 10.5);
    let mut child = style(2, "Child", 4095, &child_ps);
    styles.add(&mut normal);
    styles.add(&mut child);
    let final_ps = child.getFinalProperty().expect("child final");
    assert!(final_ps.contains(10));
    assert!(!final_ps.contains(3), "base=4095 must not pull Normal keys");
}

#[test]
fn KS4_based_on_chain_merges() {
    let mut styles = sheet();
    let parent_ps = freeze_string(3, "Arial");
    let mut parent = style(1, "Parent", 4095, &parent_ps);
    let child_ps = freeze_string(4, "宋体");
    let mut child = style(2, "Child", 1, &child_ps);
    styles.add(&mut parent);
    styles.add(&mut child);
    let final_ps = child.getFinalProperty().expect("merged final");
    assert_eq!(final_ps.getString(3), "Arial");
    assert_eq!(final_ps.getString(4), "宋体");
}

#[test]
fn KS5_cycle_does_not_hang() {
    let mut styles = sheet();
    let a_ps = freeze_int(10, 1);
    let b_ps = freeze_int(11, 2);
    let mut a = style(10, "A", 11, &a_ps);
    let mut b = style(11, "B", 10, &b_ps);
    styles.add(&mut a);
    styles.add(&mut b);
    let final_ps = a.getFinalProperty().expect("cycle must return");
    assert!(final_ps.contains(10) || final_ps.contains(11));
}

#[test]
fn KS6_space_after_214_drops_215_216() {
    let mut styles = sheet();
    let mut base_mps = MutablePropertySet::new();
    base_mps.setInt(215, 240);
    base_mps.setInt(216, 1);
    let base_ps = base_mps.freeze();
    let mut parent = style(1, "Parent", 4095, &base_ps);
    let child_ps = freeze_int(214, 120);
    let mut child = style(2, "Child", 1, &child_ps);
    styles.add(&mut parent);
    styles.add(&mut child);
    let final_ps = child.getFinalProperty().expect("justified final");
    assert!(final_ps.contains(214));
    assert!(!final_ps.contains(215));
    assert!(!final_ps.contains(216));
}

#[test]
fn KS7_space_before_217_drops_218_219() {
    let mut styles = sheet();
    let mut base_mps = MutablePropertySet::new();
    base_mps.setInt(218, 240);
    base_mps.setInt(219, 1);
    let base_ps = base_mps.freeze();
    let mut parent = style(1, "Parent", 4095, &base_ps);
    let child_ps = freeze_int(217, 120);
    let mut child = style(2, "Child", 1, &child_ps);
    styles.add(&mut parent);
    styles.add(&mut child);
    let final_ps = child.getFinalProperty().expect("justified final");
    assert!(final_ps.contains(217));
    assert!(!final_ps.contains(218));
    assert!(!final_ps.contains(219));
}

#[test]
fn KS8_toggle_57_current_keeps_base() {
    let mut styles = sheet();
    let parent_ps = freeze_toggle(57, Toggle::ON);
    let mut parent = style(1, "Parent", 4095, &parent_ps);
    let child_ps = freeze_toggle(57, Toggle::CURRENT);
    let mut child = style(2, "Child", 1, &child_ps);
    styles.add(&mut parent);
    styles.add(&mut child);
    let final_ps = child.getFinalProperty().expect("toggle final");
    assert_eq!(toggle_of(&final_ps, 57), Some(Toggle::ON));
}

#[test]
fn KS9_is_default_ids() {
    let empty = freeze_int(193, 3);
    assert!(style(0, "Normal", 4095, &empty).isDefault());
    assert!(style(65, "DefaultParagraphFont", 4095, &empty).isDefault());
    assert!(style(105, "NormalTable", 4095, &empty).isDefault());
    assert!(style(107, "TableSimple", 4095, &empty).isDefault());
    assert!(!style(1, "heading 1", 4095, &empty).isDefault());
}

#[test]
fn KS10_is_user_custom_threshold() {
    let empty = freeze_int(193, 3);
    assert!(!style(4095, "nil", 4095, &empty).isUserCustom());
    assert!(style(4096, "custom", 4095, &empty).isUserCustom());
    assert!(style(4097, "user", 4095, &empty).isUserCustom());
}
