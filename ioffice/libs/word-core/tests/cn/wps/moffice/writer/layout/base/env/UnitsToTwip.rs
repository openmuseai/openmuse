// function_id=01_function_units_to_twip
#![allow(non_snake_case)]

use ttf_parser::Face;
use word_core::cn::wps::moffice::writer::layout::base::env::UnitsToTwip::UnitsToTwip;
use word_core::cn::wps::moffice::writer::layout::text::FontStack::FontErrorKind;

fn testfont_bytes() -> Vec<u8> {
    std::fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_font_stack/TestFont.ttf"),
    )
    .expect("TestFont.ttf")
}

/// UT1
#[test]
fn UT1_1000_units_12pt_upem1000() {
    assert_eq!(UnitsToTwip::unitsToTwip(1000.0, 12.0, 1000.0).unwrap(), 240);
}

/// UT2
#[test]
fn UT2_size_twip_equals_size_pt() {
    let from_pt = UnitsToTwip::unitsToTwip(1000.0, 12.0, 1000.0).unwrap();
    let from_twip = UnitsToTwip::unitsToTwipFromSizeTwip(1000.0, 240, 1000.0).unwrap();
    assert_eq!(from_pt, from_twip);
}

/// UT3
#[test]
fn UT3_zero_units() {
    assert_eq!(UnitsToTwip::unitsToTwip(0.0, 12.0, 1000.0).unwrap(), 0);
}

/// UT4
#[test]
fn UT4_upem_zero_is_parse_err() {
    let err = UnitsToTwip::unitsToTwip(100.0, 12.0, 0.0).unwrap_err();
    assert_eq!(err.kind, FontErrorKind::Parse);
}

/// UT5
#[test]
fn UT5_24pt_is_double_12pt() {
    let w12 = UnitsToTwip::unitsToTwip(500.0, 12.0, 1000.0).unwrap();
    let w24 = UnitsToTwip::unitsToTwip(500.0, 24.0, 1000.0).unwrap();
    assert_eq!(w24, w12 * 2);
}

/// UT6
#[test]
fn UT6_rounds_half_up_style() {
    // 1031 * 12 * 20 / 2048 = 120.820... → 121
    assert_eq!(UnitsToTwip::unitsToTwip(1031.0, 12.0, 2048.0).unwrap(), 121);
}

/// UT7
#[test]
fn UT7_matches_testfont_hmtx_A() {
    let bytes = testfont_bytes();
    let face = Face::parse(&bytes, 0).unwrap();
    let gid = face.glyph_index('A').unwrap();
    let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    let upem = face.units_per_em() as f64;
    let golden = ((adv * 12.0 * 20.0) / upem).round() as i32;
    assert_eq!(UnitsToTwip::unitsToTwip(adv, 12.0, upem).unwrap(), golden);
    assert!(golden > 0);
}

/// UT8
#[test]
fn UT8_does_not_multiply_size_twip_by_20_again() {
    let correct = UnitsToTwip::unitsToTwipFromSizeTwip(1000.0, 240, 1000.0).unwrap();
    assert_eq!(correct, 240);
    let naive_feature_misread = ((1000.0_f64 * 240.0 * 20.0) / 1000.0).round() as i32;
    assert_ne!(correct, naive_feature_misread);
}
