use std::path::PathBuf;
use ttf_parser::Face;
use word_core::cn::wps::moffice::writer::layout::text::FontStack::{
    pt_to_twip, FontErrorKind, FontStack,
};

fn fixture_ttf() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/01_function_font_stack/TestFont.ttf")
}

fn hmtx_advance_twip(bytes: &[u8], ch: char, size_pt: f64) -> i32 {
    let face = Face::parse(bytes, 0).expect("fixture Face::parse");
    let gid = face.glyph_index(ch).expect("glyph");
    let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    let upem = face.units_per_em() as f64;
    ((adv * size_pt * 20.0) / upem).round() as i32
}

/// FS1 / FS-PT
#[test]
fn fs1_pt_to_twip_rounds_half_up() {
    assert_eq!(pt_to_twip(10.5), 210);
    assert_eq!(pt_to_twip(0.0), 0);
    assert_eq!(pt_to_twip(12.0), 240);
    assert_eq!(pt_to_twip(10.51), 210);
    assert_eq!(pt_to_twip(10.53), 211);
}

/// FS2 / FS-LOAD
#[test]
fn fs2_from_font_file_parses_fixture() {
    let stack = FontStack::from_font_file(fixture_ttf()).expect("load fixture");
    assert!(stack.glyph_count().expect("glyph_count") > 0);
    stack.font_id("unused").expect("face");
}

/// FS3 / FS-FAIL
#[test]
fn fs3_missing_file_is_err() {
    let err = match FontStack::from_font_file("/no/such.ttf") {
        Err(e) => e,
        Ok(_) => panic!("must fail"),
    };
    assert_eq!(err.kind, FontErrorKind::Io);
}

/// FS4 / FS-ADVANCE — golden = ttf-parser hmtx, not a guessed width=size.
#[test]
fn fs4_advance_matches_hmtx_golden() {
    let path = fixture_ttf();
    let bytes = std::fs::read(&path).unwrap();
    let golden = hmtx_advance_twip(&bytes, 'A', 12.0);
    assert!(golden > 0);
    let stack = FontStack::from_font_file(&path).unwrap();
    let got = stack.glyph_advance_twip(None, 'A', 12.0).unwrap();
    assert_eq!(got, golden);
    let delta = (got as f64 - golden as f64).abs();
    assert!(delta <= 0.5, "advance delta {delta} twip");
}

/// FS5 / FS-FALLBACK
#[test]
fn fs5_unknown_name_falls_back_to_default() {
    let stack = FontStack::from_font_file(fixture_ttf()).unwrap();
    let a_default = stack.glyph_advance_twip(None, 'A', 12.0).unwrap();
    let a_unknown = stack
        .glyph_advance_twip(Some("不存在的字体"), 'A', 12.0)
        .unwrap();
    assert_eq!(a_default, a_unknown);
    stack.font_id("不存在的字体").expect("fallback Face");
}

/// FS6 / FS-NAME-MAP
#[test]
fn fs6_name_map_hits_registered_file() {
    let path = fixture_ttf();
    let mut stack = FontStack::from_font_file(&path).unwrap();
    stack.register_name("TestFont", &path).unwrap();
    assert!(stack.name_hits_file("TestFont"));
    assert!(!stack.name_hits_file("Other"));
    let via_name = stack.glyph_advance_twip(Some("TestFont"), 'A', 12.0).unwrap();
    let via_default = stack.glyph_advance_twip(None, 'A', 12.0).unwrap();
    assert_eq!(via_name, via_default);
}

/// Fallback selection must be reproducible across processes. Android Paint
/// follows the host font list order; HashMap iteration made this unstable.
#[test]
fn fs7_fallback_selection_preserves_host_injection_order() {
    let bytes = std::fs::read(fixture_ttf()).unwrap();
    let mut stack = FontStack::from_font_bytes(bytes.clone()).unwrap();
    stack.register_name_bytes("first", bytes.clone()).unwrap();
    stack.register_name_bytes("second", bytes.clone()).unwrap();
    assert_eq!(stack.face_id_for_char(None, 'A').as_deref(), Some("first"));
    assert_eq!(stack.face_id_for_char(Some("second"), 'A').as_deref(), Some("second"));

    // Replacing a face must keep its position in the platform fallback list.
    stack.register_name_bytes("first", bytes).unwrap();
    assert_eq!(stack.face_id_for_char(None, 'A').as_deref(), Some("first"));
}
