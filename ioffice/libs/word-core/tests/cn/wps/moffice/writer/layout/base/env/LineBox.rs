// function_id=04_function_line_box
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::LineBox::{LineBox, LineSpacing};

fn mid(ascent: i32, descent: i32, spacing: LineSpacing) -> LineBox {
    LineBox::measure(ascent, descent, spacing, 0, 0, false, false, 0)
}

/// LB1
#[test]
fn LB1_auto_single() {
    let box_ = mid(180, 60, LineSpacing { rule: 0, spacing: 1.0 });
    assert_eq!(box_.height, 240);
}

/// LB2
#[test]
fn LB2_auto_one_and_half() {
    let box_ = mid(180, 60, LineSpacing { rule: 0, spacing: 1.5 });
    assert_eq!(box_.height, 360);
}

/// LB3
#[test]
fn LB3_exact_ignores_metrics() {
    let box_ = mid(999, 999, LineSpacing { rule: 1, spacing: 400.0 });
    assert_eq!(box_.height, 400);
}

/// LB4
#[test]
fn LB4_at_least_below_channel() {
    let box_ = mid(180, 60, LineSpacing { rule: 2, spacing: 100.0 });
    assert_eq!(box_.height, 240);
}

/// LB5
#[test]
fn LB5_at_least_above_channel() {
    let box_ = mid(180, 60, LineSpacing { rule: 2, spacing: 400.0 });
    assert_eq!(box_.height, 400);
}

/// LB6
#[test]
fn LB6_first_line_adds_para_before() {
    let box_ = LineBox::measure(
        180,
        60,
        LineSpacing::auto(1.0),
        200,
        0,
        true,
        false,
        0,
    );
    assert_eq!(box_.height, 440);
}

/// LB7
#[test]
fn LB7_para_before_collapses_with_prev_after() {
    let box_ = LineBox::measure(
        180,
        60,
        LineSpacing::auto(1.0),
        200,
        0,
        true,
        false,
        80,
    );
    assert_eq!(box_.height, 360);
    let clamped = LineBox::measure(
        180,
        60,
        LineSpacing::auto(1.0),
        50,
        0,
        true,
        false,
        80,
    );
    assert_eq!(clamped.height, 240);
}

/// LB8
#[test]
fn LB8_last_line_adds_para_after() {
    let box_ = LineBox::measure(
        180,
        60,
        LineSpacing::auto(1.0),
        0,
        120,
        false,
        true,
        0,
    );
    assert_eq!(box_.height, 360);
}

/// LB9
#[test]
fn LB9_middle_line_ignores_para_spacing() {
    let box_ = LineBox::measure(
        180,
        60,
        LineSpacing::auto(1.0),
        200,
        200,
        false,
        false,
        0,
    );
    assert_eq!(box_.height, 240);
}

/// LB10
#[test]
fn LB10_exact_negative_uses_abs() {
    let box_ = mid(180, 60, LineSpacing { rule: 1, spacing: -400.0 });
    assert_eq!(box_.height, 400);
}

/// LB11
#[test]
fn LB11_baseline_is_ascent() {
    let box_ = mid(180, 60, LineSpacing::auto(1.0));
    assert_eq!(box_.baseline, 180);
}

/// LB12
#[test]
fn LB12_src_does_not_call_wr04_line_height() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cn/wps/moffice/writer/layout/base/env/LineBox.rs"
    ));
    assert!(
        !src.contains("text::LineHeight") && !src.contains("line_height("),
        "LineBox must not call wr-04 LineHeight.rs"
    );
}
