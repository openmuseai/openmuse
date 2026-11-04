// function_id=05_function_run_metrics
#![allow(non_snake_case)]

use std::path::PathBuf;
use ttf_parser::Face;
use word_core::cn::wps::moffice::writer::layout::base::env::TypoFontStyle::TypoFontStyle;
use word_core::cn::wps::moffice::writer::layout::base::line::RunMetrics::measure_run_metrics;
use word_core::cn::wps::moffice::writer::layout::base::tools::TypefaceTool::TypefaceTool;
use word_core::cn::wps::moffice::writer::layout::text::FontHost::{
    FontHost, InjectedFace, SchemaItem,
};
use word_core::cn::wps::moffice::writer::layout::text::FontStack::{FontErrorKind, FontStack};

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_font_stack/TestFont.ttf"),
    )
    .unwrap()
}

fn injected_stack() -> FontStack {
    let mut host = FontHost::from_faces(vec![InjectedFace {
        id: "TestFont".into(),
        bytes: fixture_bytes(),
        collection_index: 0,
    }])
    .unwrap();
    host.add_font_map_scheme([
        SchemaItem::map("Times New Roman", "TestFont"),
        SchemaItem::map("宋体", "TestFont"),
    ]);
    host.into_font_stack().unwrap()
}

fn latin_style(size_twip: i32) -> TypoFontStyle {
    let mut s = TypoFontStyle::TypoFontStyle();
    s.ascii = "Times New Roman".into();
    s.eastAsia = "宋体".into();
    s.fontName = "Times New Roman".into();
    s.fontSize = size_twip;
    s
}

fn hmtx(ch: char, size_pt: f64) -> i32 {
    let bytes = fixture_bytes();
    let face = Face::parse(&bytes, 0).unwrap();
    let gid = face.glyph_index(ch).unwrap();
    let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    let upem = face.units_per_em() as f64;
    ((adv * size_pt * 20.0) / upem).round() as i32
}

/// RM1
#[test]
fn RM1_empty_text() {
    let stack = injected_stack();
    let m = measure_run_metrics(&stack, &latin_style(240), "").unwrap();
    assert!(m.advances.is_empty());
    assert_eq!(m.width, 0);
    assert!(m.ascent != 0 || m.descent != 0 || m.line_gap != 0);
}

/// RM2
#[test]
fn RM2_helo_per_cp() {
    let stack = injected_stack();
    let m = measure_run_metrics(&stack, &latin_style(240), "Helo").unwrap();
    let gold: Vec<i32> = "Helo".chars().map(|c| hmtx(c, 12.0)).collect();
    assert_eq!(m.advances, gold);
    assert_eq!(m.width, gold.iter().sum::<i32>());
}

/// RM3
#[test]
fn RM3_controls_zero_advance() {
    let stack = injected_stack();
    let a = measure_run_metrics(&stack, &latin_style(240), "A").unwrap();
    let with = measure_run_metrics(&stack, &latin_style(240), "A\n\t").unwrap();
    assert_eq!(with.advances, vec![a.advances[0], 0, 0]);
    assert_eq!(with.width, a.width);
}

/// RM4
#[test]
fn RM4_missing_cjk_fails() {
    let stack = injected_stack();
    let err = measure_run_metrics(&stack, &latin_style(240), "我").unwrap_err();
    assert_eq!(err.kind, FontErrorKind::MissingGlyph);
}

/// RM5
#[test]
fn RM5_mixed_fails_on_missing_han() {
    let stack = injected_stack();
    let err = measure_run_metrics(&stack, &latin_style(240), "He我llo").unwrap_err();
    assert_eq!(err.kind, FontErrorKind::MissingGlyph);
}

/// RM6
#[test]
fn RM6_uses_ascii_name_for_latin() {
    let stack = injected_stack();
    let m = measure_run_metrics(&stack, &latin_style(240), "A").unwrap();
    assert_eq!(m.advances[0], hmtx('A', 12.0));
}

/// RM7
#[test]
fn RM7_bold_italic_same_hmtx() {
    let stack = injected_stack();
    let mut bi = latin_style(240);
    bi.bold = true;
    bi.italic = true;
    let plain = measure_run_metrics(&stack, &latin_style(240), "Helo").unwrap();
    let styled = measure_run_metrics(&stack, &bi, "Helo").unwrap();
    assert_eq!(plain.advances, styled.advances);
}

/// RM8
#[test]
fn RM8_size_scales() {
    let stack = injected_stack();
    let w12 = measure_run_metrics(&stack, &latin_style(240), "He").unwrap();
    let w24 = measure_run_metrics(&stack, &latin_style(480), "He").unwrap();
    assert_eq!(w24.width, w12.width * 2);
}

/// RM9
#[test]
fn RM9_width_is_sum() {
    let stack = injected_stack();
    let m = measure_run_metrics(&stack, &latin_style(240), "Helo").unwrap();
    assert_eq!(m.width, m.advances.iter().sum::<i32>());
}

/// RM10
#[test]
fn RM10_line_gap_present() {
    let stack = injected_stack();
    let m = measure_run_metrics(&stack, &latin_style(240), "A").unwrap();
    let bytes = fixture_bytes();
    let face = Face::parse(&bytes, 0).unwrap();
    let upem = face.units_per_em() as f64;
    let gap = (face.line_gap() as f64 * 12.0 * 20.0 / upem).round() as i32;
    assert_eq!(m.line_gap, gap);
}

/// RM11
#[test]
fn RM11_fonthost_scheme_names() {
    let stack = injected_stack();
    assert!(stack.name_hits_file("Times New Roman") || stack.name_hits_file("TestFont"));
    let m = measure_run_metrics(&stack, &latin_style(240), "A").unwrap();
    assert_eq!(m.width, hmtx('A', 12.0));
}

/// RM12
#[test]
fn RM12_no_disk_and_adj_factor() {
    let stack = injected_stack();
    assert!(stack.default_path().is_none());
    let tool = TypefaceTool::default();
    assert_eq!(tool.getAdjCalcFactor(false), 1.0);
    assert_eq!(tool.getAdjCalcFactor(true), 4.0);
}
