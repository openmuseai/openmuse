// function_id=02_function_wrap
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::LineWrap::{LineRange, LineWrap};

fn ids(lines: &[LineRange]) -> Vec<(i32, i32, i32)> {
    lines
        .iter()
        .map(|r| (r.start_cp, r.end_cp, r.width))
        .collect()
}

/// WP1
#[test]
fn WP1_fits_on_one_line() {
    let lines = LineWrap::wrap(&[100, 100], "ab", 500);
    assert_eq!(ids(&lines), vec![(0, 2, 200)]);
}

/// WP2
#[test]
fn WP2_wrap_at_space_keeps_trailing_space() {
    // "aa bb" letters 100, space 50, max=250 → Java rollback leaves the space on line 1.
    let lines = LineWrap::wrap(&[100, 100, 50, 100, 100], "aa bb", 250);
    assert_eq!(ids(&lines), vec![(0, 3, 250), (3, 5, 200)]);
}

/// WP3
#[test]
fn WP3_cjk_breaks_per_char() {
    let lines = LineWrap::wrap(&[400, 400], "你好", 500);
    assert_eq!(ids(&lines), vec![(0, 1, 400), (1, 2, 400)]);
}

/// WP4
#[test]
fn WP4_force_cut_overwide_latin_word() {
    let lines = LineWrap::wrap(&[300, 300], "WW", 400);
    assert!(lines.len() >= 2, "got {} lines", lines.len());
    for line in &lines {
        assert!(line.end_cp - line.start_cp >= 1);
    }
}

/// WP5
#[test]
fn WP5_slack_one_twip_stays() {
    let lines = LineWrap::wrap(&[101], "X", 100);
    assert_eq!(ids(&lines), vec![(0, 1, 101)]);
}

/// WP6
#[test]
fn WP6_overflow_two_twip_wraps() {
    let lines = LineWrap::wrap(&[100, 2], "ab", 100);
    assert_eq!(ids(&lines), vec![(0, 1, 100), (1, 2, 2)]);
}

/// WP7
#[test]
fn WP7_cr_ends_paragraph_no_next_line() {
    let lines = LineWrap::wrap(&[100, 0], "a\r", 500);
    assert_eq!(ids(&lines), vec![(0, 2, 100)]);
}

/// WP8
#[test]
fn WP8_nbsp_is_not_a_break() {
    // If NBSP were U+0020, hanging-space absorb would yield [0,2)+[2,3).
    let s = "a\u{00A0}b";
    let lines = LineWrap::wrap(&[200, 200, 200], s, 250);
    assert!(!lines.is_empty());
    assert_eq!(lines[0].end_cp, 1, "must not wrap after NBSP as a space");
}

/// WP9
#[test]
fn WP9_empty_is_zero_lines() {
    let lines = LineWrap::wrap(&[], "", 200);
    assert!(lines.is_empty());
}

/// WP10
#[test]
fn WP10_cjk_five_chars_five_lines() {
    let lines = LineWrap::wrap(&[400; 5], "一二三四五", 400);
    assert_eq!(lines.len(), 5);
    for (i, line) in lines.iter().enumerate() {
        let i = i as i32;
        assert_eq!((line.start_cp, line.end_cp, line.width), (i, i + 1, 400));
    }
}

/// WP11
#[test]
fn WP11_larger_advance_yields_more_lines() {
    let small = LineWrap::wrap(&[50, 50, 50, 50], "abcd", 150);
    let large = LineWrap::wrap(&[200, 200, 200, 200], "abcd", 150);
    assert!(large.len() >= small.len());
}

/// WP12
#[test]
fn WP12_src_does_not_call_wr04_wrap() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cn/wps/moffice/writer/layout/base/env/LineWrap.rs"
    ));
    assert!(
        !src.contains("text::Wrap") && !src.contains("use super::super::text::Wrap"),
        "LineWrap must not call wr-04 Wrap.rs"
    );
}
