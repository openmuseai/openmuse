// function_id=06_function_add_normal_style
// cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_add_normal_style
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/helper/IOHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/helper/IOHelper.rs

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;

fn sheet() -> KStyles {
    KStyles::KStyles(&TextDocument::default())
}

fn keep_me() -> KStyle {
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 9);
    let ps = mps.freeze();
    KStyle::new__int_string_int_int_int_propertyset(0, "KeepMe", 4095, 4095, 1, Some(&ps))
}

#[test]
fn NS1_inserts_when_id_zero_missing() {
    let mut styles = sheet();
    IOHelper::addNormalStyle(&mut styles);
    assert!(styles.getStyleById__int(0).is_some());
}

#[test]
fn NS2_id_name_type() {
    let mut styles = sheet();
    IOHelper::addNormalStyle(&mut styles);
    let n = styles.getStyleById__int(0).unwrap();
    assert_eq!(n.getId(), 0);
    assert_eq!(n.getName(), "Normal");
    assert_eq!(n.getType(), 1);
}

#[test]
fn NS3_base_and_next_are_sti_nil() {
    let mut styles = sheet();
    IOHelper::addNormalStyle(&mut styles);
    let n = styles.getStyleById__int(0).unwrap();
    assert_eq!(n.getBase(), 4095);
    assert_eq!(n.getNext(), 4095);
}

#[test]
fn NS4_font_size_prop_10() {
    let mut styles = sheet();
    IOHelper::addNormalStyle(&mut styles);
    let n = styles.getStyleById__int(0).unwrap();
    let ps = n.getProperty().expect("Normal property");
    assert!((ps.getFloat(10) - 10.5).abs() < 1e-4);
}

#[test]
fn NS5_prop_193_is_three() {
    let mut styles = sheet();
    IOHelper::addNormalStyle(&mut styles);
    let n = styles.getStyleById__int(0).unwrap();
    let ps = n.getProperty().expect("Normal property");
    assert_eq!(ps.getInt(193), 3);
}

#[test]
fn NS6_second_call_does_not_overlay_existing_zero() {
    let mut styles = sheet();
    let mut keep = keep_me();
    styles.add(&mut keep);
    IOHelper::addNormalStyle(&mut styles);
    assert_eq!(styles.size(), 1);
    assert_eq!(styles.getStyleById__int(0).unwrap().getName(), "KeepMe");
}
