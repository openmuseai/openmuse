// function_id=01_function_textrope_append
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_textrope_append
// src:  libs/word-core/src/cn/wps/collections/textrope/{KNotifyTextRope,KTextRope,KTextPool,KTextPieceTable}.rs
// test: libs/word-core/tests/cn/wps/collections/textrope/KNotifyTextRope.rs

use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::collections::textrope::KNotifyTextRope::KNotifyTextRope;

fn rope_text(rope: &mut KNotifyTextRope, start: i32, end: i32) -> String {
    let n = (end - start).max(0) as usize;
    let mut buf = vec!['\0'; n];
    ITextRope::getText(rope, start, end, &mut buf, 0);
    buf.into_iter().collect()
}

#[test]
fn tr1_append_string_length_and_charat() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "Hi", 0, 2);
    assert_eq!(ITextRope::length(&rope), 2);
    assert_eq!(ITextRope::charAt(&mut rope, 0), 'H');
    assert_eq!(ITextRope::charAt(&mut rope, 1), 'i');
}

#[test]
fn tr2_append_char() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, 'A');
    assert_eq!(ITextRope::length(&rope), 1);
    assert_eq!(ITextRope::charAt(&mut rope, 0), 'A');
}

#[test]
fn tr3_append_chararr_slice() {
    let mut rope = KNotifyTextRope::default();
    let buf = vec!['x', 'y', 'z'];
    ITextRope::append__chararr_int_int(&mut rope, &buf, 1, 3);
    assert_eq!(ITextRope::length(&rope), 2);
    assert_eq!(rope_text(&mut rope, 0, 2), "yz");
}

#[test]
fn tr4_append_empty_slice_does_not_grow() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "ab", 1, 1);
    assert_eq!(ITextRope::length(&rope), 0);
}

#[test]
#[should_panic(expected = "IndexOutOfBoundsException")]
fn tr5_charat_negative_panics() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, 'A');
    let _ = ITextRope::charAt(&mut rope, -1);
}

#[test]
#[should_panic(expected = "IndexOutOfBoundsException")]
fn tr5b_charat_past_end_panics() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, 'A');
    let _ = ITextRope::charAt(&mut rope, 1);
}

#[test]
fn tr6_gettext_roundtrip() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "Hi", 0, 2);
    assert_eq!(rope_text(&mut rope, 0, 2), "Hi");
}

#[test]
fn tr7_erase_shortens() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "Hi", 0, 2);
    ITextRope::erase(&mut rope, 0, 1);
    assert_eq!(ITextRope::length(&rope), 1);
    assert_eq!(ITextRope::charAt(&mut rope, 0), 'i');
}

#[test]
fn tr8_erase_empty_range_is_noop() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "Hi", 0, 2);
    ITextRope::erase(&mut rope, 1, 1);
    assert_eq!(ITextRope::length(&rope), 2);
    assert_eq!(rope_text(&mut rope, 0, 2), "Hi");
}

#[test]
fn tr9_insert_middle_shifts_following_cps() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "AC", 0, 2);
    ITextRope::insert__int_char(&mut rope, 1, 'B');
    assert_eq!(ITextRope::length(&rope), 3);
    assert_eq!(rope_text(&mut rope, 0, 3), "ABC");
}

#[test]
fn tr10_cr_and_ff_are_literal_codepoints() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, '\u{000D}');
    ITextRope::append__char(&mut rope, '\u{000C}');
    assert_eq!(ITextRope::length(&rope), 2);
    assert_eq!(ITextRope::charAt(&mut rope, 0), '\u{000D}');
    assert_eq!(ITextRope::charAt(&mut rope, 1), '\u{000C}');
}

#[test]
fn tr11_two_appends_concatenate() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "a", 0, 1);
    ITextRope::append__string_int_int(&mut rope, "b", 0, 1);
    assert_eq!(ITextRope::length(&rope), 2);
    assert_eq!(rope_text(&mut rope, 0, 2), "ab");
}
