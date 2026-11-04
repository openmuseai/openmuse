// function_id=02_function_glyph_advance
#![allow(non_snake_case)]

use std::path::PathBuf;
use ttf_parser::Face;
use word_core::cn::wps::moffice::writer::layout::base::env::GlyphAdvance::GlyphAdvance;
use word_core::cn::wps::moffice::writer::layout::text::FontHost::{FontHost, InjectedFace};
use word_core::cn::wps::moffice::writer::layout::text::FontStack::{FontErrorKind, FontStack};

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

fn hmtx_twip(ch: char, size_pt: f64) -> i32 {
    let bytes = fixture_bytes();
    let face = Face::parse(&bytes, 0).unwrap();
    let gid = face.glyph_index(ch).unwrap();
    let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    let upem = face.units_per_em() as f64;
    ((adv * size_pt * 20.0) / upem).round() as i32
}

/// GA1
#[test]
fn GA1_A_matches_hmtx() {
    let stack = injected_stack();
    let got = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), 'A', 12.0).unwrap();
    assert_eq!(got, hmtx_twip('A', 12.0));
    assert!(got > 0);
}

/// GA2
#[test]
fn GA2_H_positive() {
    let stack = injected_stack();
    let got = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), 'H', 12.0).unwrap();
    assert_eq!(got, hmtx_twip('H', 12.0));
    assert!(got > 0);
}

/// GA3
#[test]
fn GA3_missing_cjk_is_err_not_size() {
    let stack = injected_stack();
    let err = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), '我', 12.0).unwrap_err();
    assert_eq!(err.kind, FontErrorKind::MissingGlyph);
}

/// GA4
#[test]
fn GA4_missing_w_is_err() {
    let stack = injected_stack();
    let err = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), 'w', 12.0).unwrap_err();
    assert_eq!(err.kind, FontErrorKind::MissingGlyph);
}

/// GA5
#[test]
fn GA5_scales_linearly() {
    let stack = injected_stack();
    let a12 = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), 'A', 12.0).unwrap();
    let a24 = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), 'A', 24.0).unwrap();
    assert_eq!(a24, a12 * 2);
}

/// GA6
#[test]
fn GA6_named_face() {
    let stack = injected_stack();
    let named = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), 'e', 12.0).unwrap();
    let default = GlyphAdvance::glyphAdvanceTwip(&stack, None, 'e', 12.0).unwrap();
    assert_eq!(named, default);
}

/// GA7
#[test]
fn GA7_unknown_name_falls_back_default_for_latin() {
    let stack = injected_stack();
    let a = GlyphAdvance::glyphAdvanceTwip(&stack, Some("NoSuchFont"), 'A', 12.0).unwrap();
    assert_eq!(a, hmtx_twip('A', 12.0));
}

/// GA8
#[test]
fn GA8_strict_missing_does_not_invent_width() {
    let stack = injected_stack();
    match GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), '我', 12.0) {
        Err(e) => assert_eq!(e.kind, FontErrorKind::MissingGlyph),
        Ok(w) => panic!("must not fake width={w}"),
    }
}

/// GA9
#[test]
fn GA9_hello_letters() {
    let stack = injected_stack();
    for ch in "Helo".chars() {
        let got = GlyphAdvance::glyphAdvanceTwip(&stack, Some("TestFont"), ch, 12.0).unwrap();
        assert_eq!(got, hmtx_twip(ch, 12.0));
    }
}

/// GA10
#[test]
fn GA10_does_not_probe_disk() {
    let stack = injected_stack();
    assert!(stack.default_path().is_none());
}
