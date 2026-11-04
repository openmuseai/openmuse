// function_id=03_function_alignment
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::LineAlign::LineAlign;

/// AL1
#[test]
fn AL1_left() {
    assert_eq!(LineAlign::alignX(0, 100, 200), 0);
}

/// AL2
#[test]
fn AL2_center() {
    assert_eq!(LineAlign::alignX(1, 100, 200), 50);
}

/// AL3
#[test]
fn AL3_right() {
    assert_eq!(LineAlign::alignX(2, 100, 200), 100);
}

/// AL4
#[test]
fn AL4_full_width() {
    assert_eq!(LineAlign::alignX(1, 200, 200), 0);
}

/// AL5
#[test]
fn AL5_overwide_clamps_to_zero() {
    assert_eq!(LineAlign::alignX(2, 250, 200), 0);
}

/// AL6
#[test]
fn AL6_justify_stubs_left() {
    assert_eq!(LineAlign::alignX(3, 100, 200), 0);
}

/// AL7
#[test]
fn AL7_center_odd_truncates() {
    assert_eq!(LineAlign::alignX(1, 101, 200), 49);
}

/// AL8
#[test]
fn AL8_unknown_jc_is_left() {
    assert_eq!(LineAlign::alignX(99, 100, 200), 0);
}
