// function_id=04_function_line_metrics
#![allow(non_snake_case)]

use std::path::PathBuf;
use ttf_parser::Face;
use word_core::cn::wps::moffice::writer::layout::base::env::LineMetrics::LineMetrics;
use word_core::cn::wps::moffice::writer::layout::text::FontHost::{FontHost, InjectedFace};
use word_core::cn::wps::moffice::writer::layout::text::FontStack::FontStack;

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_font_stack/TestFont.ttf"),
    )
    .unwrap()
}

fn injected_stack() -> FontStack {
    FontHost::from_faces(vec![InjectedFace {
        id: "TestFont".into(),
        bytes: fixture_bytes(),
        collection_index: 0,
    }])
    .unwrap()
    .into_font_stack()
    .unwrap()
}

fn hhea_box(size_pt: f64) -> (i32, i32, i32) {
    let bytes = fixture_bytes();
    let face = Face::parse(&bytes, 0).unwrap();
    let upem = face.units_per_em() as f64;
    let scale = size_pt * 20.0 / upem;
    (
        (face.ascender() as f64 * scale).round() as i32,
        (face.descender() as f64 * scale).round() as i32,
        (face.line_gap() as f64 * scale).round() as i32,
    )
}

/// LM1
#[test]
fn LM1_ascent_descent_match_hhea() {
    let stack = injected_stack();
    let box_ = LineMetrics::lineBoxTwip(&stack, Some("TestFont"), 12.0).unwrap();
    let (a, d, _) = hhea_box(12.0);
    assert_eq!(box_.ascent_twip, a);
    assert_eq!(box_.descent_twip, d);
}

/// LM2
#[test]
fn LM2_line_gap_match_hhea() {
    let stack = injected_stack();
    let box_ = LineMetrics::lineBoxTwip(&stack, Some("TestFont"), 12.0).unwrap();
    let (_, _, g) = hhea_box(12.0);
    assert_eq!(box_.line_gap_twip, g);
}

/// LM3
#[test]
fn LM3_scales_linearly() {
    let stack = injected_stack();
    let b12 = LineMetrics::lineBoxTwip(&stack, Some("TestFont"), 12.0).unwrap();
    let b24 = LineMetrics::lineBoxTwip(&stack, Some("TestFont"), 24.0).unwrap();
    assert_eq!(b24.ascent_twip, b12.ascent_twip * 2);
    assert_eq!(b24.descent_twip, b12.descent_twip * 2);
    assert_eq!(b24.line_gap_twip, b12.line_gap_twip * 2);
}

/// LM4
#[test]
fn LM4_named_equals_default_same_blob() {
    let stack = injected_stack();
    let named = LineMetrics::lineBoxTwip(&stack, Some("TestFont"), 12.0).unwrap();
    let def = LineMetrics::lineBoxTwip(&stack, None, 12.0).unwrap();
    assert_eq!(named, def);
}

/// LM5
#[test]
fn LM5_unknown_name_uses_default() {
    let stack = injected_stack();
    let def = LineMetrics::lineBoxTwip(&stack, None, 12.0).unwrap();
    let unknown = LineMetrics::lineBoxTwip(&stack, Some("NoSuch"), 12.0).unwrap();
    assert_eq!(def, unknown);
}

/// LM6
#[test]
fn LM6_legacy_line_metrics_pair() {
    let stack = injected_stack();
    let (a, d) = stack.line_metrics_twip(12.0).unwrap();
    let box_ = LineMetrics::lineBoxTwip(&stack, None, 12.0).unwrap();
    assert_eq!(a, box_.ascent_twip);
    assert_eq!(d, box_.descent_twip);
}

/// LM7
#[test]
fn LM7_no_disk_probe() {
    let stack = injected_stack();
    assert!(stack.default_path().is_none());
}
