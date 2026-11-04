// function_id=01_function_io_style
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_io_style
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/IOStyle.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/IOStyle.rs

#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::IOStyle::IOStyle;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;

fn sti_map() -> StyleId2Sti {
    StyleId2Sti::StyleId2Sti(UUSti::UUSti())
}

fn convert(style: &mut IOStyle) -> i32 {
    let mut map = sti_map();
    style.convertTypeAndId(&mut map);
    style.mCoreId
}

#[test]
fn IOS1_paragraph_core_type_1() {
    let mut s = IOStyle::default();
    s.mType = Some("paragraph".into());
    s.mStyleId = Some("p1".into());
    convert(&mut s);
    assert_eq!(s.mCoreType, 1);
    assert_ne!(s.mCoreId, 4095);
}

#[test]
fn IOS2_character_core_type_2() {
    let mut s = IOStyle::default();
    s.mType = Some("character".into());
    s.mStyleId = Some("c1".into());
    convert(&mut s);
    assert_eq!(s.mCoreType, 2);
}

#[test]
fn IOS3_table_core_type_3() {
    let mut s = IOStyle::default();
    s.mType = Some("table".into());
    s.mStyleId = Some("t1".into());
    convert(&mut s);
    assert_eq!(s.mCoreType, 3);
}

#[test]
fn IOS4_numbering_core_type_4() {
    let mut s = IOStyle::default();
    s.mType = Some("numbering".into());
    s.mStyleId = Some("n1".into());
    convert(&mut s);
    assert_eq!(s.mCoreType, 4);
}

#[test]
#[should_panic(expected = "it should not reach here")]
fn IOS5_illegal_type_fails() {
    let mut s = IOStyle::default();
    s.mType = Some("foo".into());
    s.mStyleId = Some("x".into());
    convert(&mut s);
}

#[test]
fn IOS6_named_put_four_args() {
    let mut map = sti_map();
    let mut s = IOStyle::default();
    s.mType = Some("paragraph".into());
    s.mStyleId = Some("H1".into());
    s.mName = Some("Heading1".into());
    s.mIsCustom = None;
    s.convertTypeAndId(&mut map);
    assert_eq!(map.getSti("H1"), Some(s.mCoreId));
    assert_ne!(s.mCoreId, 4095);
}

#[test]
fn IOS7_bare_put_one_arg() {
    let mut map = sti_map();
    let mut s = IOStyle::default();
    s.mType = Some("paragraph".into());
    s.mStyleId = Some("Bare".into());
    s.mName = None;
    s.convertTypeAndId(&mut map);
    assert_eq!(map.getSti("Bare"), Some(s.mCoreId));
    assert_ne!(s.mCoreId, 4095);
}

#[test]
fn IOS8_is_custom_changes_builtin_sti() {
    let mut builtin = IOStyle::default();
    builtin.mType = Some("paragraph".into());
    builtin.mStyleId = Some("norm".into());
    builtin.mName = Some("Normal".into());
    builtin.mIsCustom = Some(false);
    let id_false = convert(&mut builtin);

    let mut custom = IOStyle::default();
    custom.mType = Some("paragraph".into());
    custom.mStyleId = Some("normC".into());
    custom.mName = Some("Normal".into());
    custom.mIsCustom = Some(true);
    let id_true = convert(&mut custom);

    let mut as_null = IOStyle::default();
    as_null.mType = Some("paragraph".into());
    as_null.mStyleId = Some("normN".into());
    as_null.mName = Some("Normal".into());
    as_null.mIsCustom = None;
    let id_null = convert(&mut as_null);

    assert_eq!(id_false, 0);
    assert_eq!(id_null, 0);
    assert!(id_true >= 4097);
    assert_ne!(id_true, id_false);
}

#[test]
fn IOS9_core_id_never_4095() {
    let mut s = IOStyle::default();
    s.mType = Some("paragraph".into());
    s.mStyleId = Some("ok".into());
    convert(&mut s);
    assert_ne!(s.mCoreId, 4095);
}

#[test]
fn IOS10_rectify_style_id_priority() {
    let mut a = IOStyle::default();
    a.mStyleId = Some("a".into());
    a.mName = Some("b".into());
    assert_eq!(a.rectifyStyleId(), "a");

    let mut b = IOStyle::default();
    b.mStyleId = Some("".into());
    b.mName = Some("b".into());
    assert_eq!(b.rectifyStyleId(), "b");

    let c = IOStyle::default();
    assert_eq!(c.rectifyStyleId(), "");
}

#[test]
fn IOS11_core_pr_set_default_usable() {
    let mut s = IOStyle::default();
    s.mCorePrSet.setInt(10, 1);
    assert!(s.mCorePrSet.get().contains(10));
}
