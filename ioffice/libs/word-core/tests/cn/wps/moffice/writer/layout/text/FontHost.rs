// function_id=04_function_font_host
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_font_host

use std::path::PathBuf;
use std::sync::Arc;
use ttf_parser::Face;
use word_core::cn::wps::moffice::writer::layout::text::FontHost::{
    get_font_table, set_font_table, FontHost, InjectedFace, SchemaItem,
};
use word_core::cn::wps::moffice::writer::layout::text::FontStack::FontErrorKind;

fn fixture_ttf() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/01_function_font_stack/TestFont.ttf")
}

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(fixture_ttf()).expect("TestFont.ttf")
}

fn hmtx_a(bytes: &[u8]) -> i32 {
    let face = Face::parse(bytes, 0).unwrap();
    let gid = face.glyph_index('A').unwrap();
    let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    let upem = face.units_per_em() as f64;
    ((adv * 12.0 * 20.0) / upem).round() as i32
}

fn one_face(id: &str) -> InjectedFace {
    InjectedFace {
        id: id.into(),
        bytes: fixture_bytes(),
        collection_index: 0,
    }
}

/// FH1
#[test]
fn fh1_empty_faces_is_err() {
    let err = FontHost::from_faces(Vec::new()).expect_err("empty must fail");
    assert_eq!(err.kind, FontErrorKind::Io);
}

/// FH2
#[test]
fn fh2_single_fixture_is_default_and_measures() {
    let host = FontHost::from_faces(vec![one_face("TestFont")]).unwrap();
    assert_eq!(host.default_font_id(), "TestFont");
    let golden = hmtx_a(&fixture_bytes());
    let stack = host.into_font_stack().unwrap();
    assert_eq!(stack.glyph_advance_twip(Some("TestFont"), 'A', 12.0).unwrap(), golden);
}

/// FH3
#[test]
fn fh3_scheme_maps_display_name_to_actual() {
    let mut host = FontHost::from_faces(vec![one_face("TestFont")]).unwrap();
    host.add_font_map_scheme([SchemaItem::map("宋体", "TestFont")]);
    assert_eq!(host.resolve_actual_name(Some("宋体")), "TestFont");
    let golden = hmtx_a(&fixture_bytes());
    let stack = host.into_font_stack().unwrap();
    assert!(stack.name_hits_file("宋体"));
    assert_eq!(stack.glyph_advance_twip(Some("宋体"), 'A', 12.0).unwrap(), golden);
}

/// FH4
#[test]
fn fh4_unknown_and_empty_fall_back_to_default() {
    let host = FontHost::from_faces(vec![one_face("TestFont")]).unwrap();
    assert_eq!(host.resolve_actual_name(None), "TestFont");
    assert_eq!(host.resolve_actual_name(Some("")), "TestFont");
    assert_eq!(host.resolve_actual_name(Some("没有这种字体")), "TestFont");
}

/// FH5
#[test]
fn fh5_second_id_is_on_name_map() {
    let host = FontHost::from_faces(vec![one_face("default"), one_face("Calibri")]).unwrap();
    assert_eq!(host.default_font_id(), "default");
    assert_eq!(host.resolve_actual_name(Some("Calibri")), "Calibri");
    let stack = host.into_font_stack().unwrap();
    assert!(stack.name_hits_file("Calibri"));
}

/// FH6
#[test]
fn fh6_garbage_bytes_are_parse_err() {
    let err = FontHost::from_faces(vec![InjectedFace {
        id: "bad".into(),
        bytes: vec![0, 1, 2, 3],
        collection_index: 0,
    }])
    .expect_err("garbage");
    assert_eq!(err.kind, FontErrorKind::Parse);
}

/// FH7 — extra injected face covers glyphs the default cmap lacks (sample2 / 宋体).
#[test]
fn fh7_fallback_face_covers_missing_cmap() {
    let unicode = std::path::Path::new("/System/Library/Fonts/Supplemental/Arial Unicode.ttf");
    if !unicode.is_file() {
        return;
    }
    let host = FontHost::from_faces(vec![
        one_face("default"),
        InjectedFace {
            id: "宋体".into(),
            bytes: std::fs::read(unicode).unwrap(),
            collection_index: 0,
        },
    ])
    .unwrap();
    let stack = host.into_font_stack().unwrap();
    assert!(!stack.has_glyph(None, '大'));
    assert_eq!(stack.face_id_for_char(None, '大').as_deref(), Some("宋体"));
    assert!(stack.glyph_advance_twip(None, '大', 12.0).unwrap() > 0);
}

/// FH8: Java FontHost.setFontTable replaces the process table. A retry after
/// MissingGlyph must see its newly injected CJK face, while an older layout
/// can keep its Arc until that layout completes.
#[test]
fn fh8_reinject_replaces_table_without_invalidating_old_reader() {
    let first = Arc::new(
        FontHost::from_faces(vec![one_face("first")])
            .unwrap()
            .into_font_stack()
            .unwrap(),
    );
    set_font_table(first.clone());
    let old_reader = get_font_table().unwrap();
    assert!(old_reader.name_hits_file("first"));

    let second = Arc::new(
        FontHost::from_faces(vec![one_face("second")])
            .unwrap()
            .into_font_stack()
            .unwrap(),
    );
    set_font_table(second.clone());
    let new_reader = get_font_table().unwrap();
    assert!(new_reader.name_hits_file("second"));
    assert!(!new_reader.name_hits_file("first"));
    assert!(old_reader.name_hits_file("first"));
}
