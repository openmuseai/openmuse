// function_id=01_function_content_width
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::ContentWidth::ContentWidth;

/// CW1
#[test]
fn CW1_first_line_positive_indent() {
    assert_eq!(ContentWidth::lineMaxWidth(5000, 480, true), 4520);
}

/// CW2
#[test]
fn CW2_rest_line_ignores_indent() {
    assert_eq!(ContentWidth::lineMaxWidth(5000, 480, false), 5000);
}

/// CW3
#[test]
fn CW3_hanging_negative_indent_widens_first_line() {
    assert_eq!(ContentWidth::lineMaxWidth(5000, -480, true), 5480);
}

/// CW4
#[test]
fn CW4_zero_indent_matches_rest() {
    assert_eq!(ContentWidth::lineMaxWidth(5000, 0, true), 5000);
    assert_eq!(ContentWidth::lineMaxWidth(5000, 0, false), 5000);
}

/// CW5
#[test]
fn CW5_over_indent_clamps_to_zero() {
    assert_eq!(ContentWidth::lineMaxWidth(200, 480, true), 0);
}

/// CW6
#[test]
fn CW6_does_not_subtract_left_right_indent_again() {
    // content_width is already wr-02 net of PAPX 210/212 (e.g. 5000).
    // Only 208 applies on the first sub-line: 5000 - 480 = 4520, not 4520 - leftIndent.
    let wr02_width = 5000;
    let first_line_indent = 480;
    assert_eq!(
        ContentWidth::lineMaxWidth(wr02_width, first_line_indent, true),
        4520
    );
}

/// CW1 — page content minus PAPX 210 only
#[test]
fn CW1_para_only_left_210() {
    assert_eq!(ContentWidth::paraMaxWidth(5000, 360, 0), 4640);
}

/// CW2 — page content minus PAPX 212 only
#[test]
fn CW2_para_only_right_212() {
    assert_eq!(ContentWidth::paraMaxWidth(5000, 0, 240), 4760);
}

/// CW3 — missing 210/212 treated as 0 (Java getInt default)
#[test]
fn CW3_para_no_indent_equals_page() {
    assert_eq!(ContentWidth::paraMaxWidth(5000, 0, 0), 5000);
}

/// CW4 — firstLine 208 applies on paraMaxWidth, not subtracted twice from 210/212
#[test]
fn CW4_para_then_first_line() {
    let para = ContentWidth::paraMaxWidth(5000, 360, 240);
    assert_eq!(para, 4400);
    assert_eq!(ContentWidth::lineMaxWidth(para, 480, true), 3920);
}
