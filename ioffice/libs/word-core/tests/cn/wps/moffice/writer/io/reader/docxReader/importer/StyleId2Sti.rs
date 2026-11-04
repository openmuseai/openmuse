// function_id=03_function_styleid2sti
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_styleid2sti
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/StyleId2Sti.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/StyleId2Sti.rs

use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BuildinStyleSheet::BuildinStyleSheet;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;

fn map_with_fresh_maker() -> StyleId2Sti {
    StyleId2Sti::StyleId2Sti(UUSti::UUSti())
}

#[test]
fn SI1_first_uusti_is_4097() {
    let mut maker = UUSti::UUSti();
    assert_eq!(maker.getUUSti(), 4097);
    assert_eq!(maker.getUUSti(), 4098);
}

#[test]
fn SI2_heading1_non_custom_uses_builtin_sti() {
    let mut m = map_with_fresh_maker();
    assert_eq!(
        m.put__string_string_int_boolean("h1", "heading 1", 1, false),
        1
    );
    assert_eq!(m.getSti("h1"), Some(1));
}

#[test]
fn SI3_normal_non_custom_uses_sti_zero() {
    let mut m = map_with_fresh_maker();
    assert_eq!(m.put__string_string_int_boolean("n", "Normal", 1, false), 0);
    assert_eq!(m.getSti("n"), Some(0));
}

#[test]
fn SI4_is_custom_forces_uusti() {
    let mut m = map_with_fresh_maker();
    let core = m.put__string_string_int_boolean("h1", "heading 1", 1, true);
    assert!(core >= 4097, "isCustom must not use builtin sti=1, got {core}");
    assert_ne!(core, 1);
    assert_eq!(m.getSti("h1"), Some(core));
}

#[test]
fn SI5_type_mismatch_forces_uusti() {
    let mut m = map_with_fresh_maker();
    let core = m.put__string_string_int_boolean("h1", "heading 1", 2, false);
    assert!(
        core >= 4097,
        "CHARACTER type vs TypeSti.type=1 must UUSti, got {core}"
    );
}

#[test]
fn SI6_unknown_name_forces_uusti() {
    let mut m = map_with_fresh_maker();
    let core = m.put__string_string_int_boolean("x", "zzz", 1, false);
    assert!(core >= 4097, "unknown name must UUSti, got {core}");
}

#[test]
fn SI7_get_sti_miss_is_null_not_4095() {
    let m = map_with_fresh_maker();
    assert_eq!(m.getSti("missing"), None);
}

#[test]
fn SI8_put_style_id_only_allocates() {
    let mut m = map_with_fresh_maker();
    let core = m.put__string("only-id");
    assert_eq!(core, 4097);
    assert_eq!(m.getSti("only-id"), Some(4097));
}

#[test]
fn SI9_put_forced_sti() {
    let mut m = map_with_fresh_maker();
    m.put__string_integer("a", Some(99));
    assert_eq!(m.getSti("a"), Some(99));
}

#[test]
fn SI10_next_user_sti_does_not_write_map() {
    let mut m = map_with_fresh_maker();
    assert_eq!(m.nextUserSti(), 4097);
    assert_eq!(m.getSti("a"), None);
}

#[test]
fn SI11_dispose_clears_map() {
    let mut m = map_with_fresh_maker();
    m.put__string_string_int_boolean("h1", "heading 1", 1, false);
    m.dispose();
    assert_eq!(m.getSti("h1"), None);
    let ts = BuildinStyleSheet::getTypeSti("heading 1").expect("table can lazy re-init");
    assert_eq!(ts.sti, 1);
}
