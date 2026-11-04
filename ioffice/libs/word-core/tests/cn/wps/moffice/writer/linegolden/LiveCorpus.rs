// function_id=03_function_live_corpus
#![allow(non_snake_case)]

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{FontBlob, ViewParams};
use word_core::cn::wps::moffice::writer::cache::RustStage2::from_layout;
use word_core::cn::wps::moffice::writer::cache::Stage2Json::to_stage2_json;
use word_core::cn::wps::moffice::writer::linegolden::LineCompare::compare_lines;
use word_core::cn::wps::moffice::writer::linegolden::LiveCorpus::{run_live_corpus, LiveCase};

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn mvp_font() -> FontBlob {
    FontBlob {
        id: "default".into(),
        bytes: std::fs::read(crate_root().join("tests/fixtures/p1_layout_document/MvpLatin.ttf"))
            .expect("MvpLatin.ttf"),
    }
}

fn hello_docx() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"))
        .expect("plain-one-para.docx")
}

fn sample2_docx() -> Vec<u8> {
    std::fs::read(crate_root().join("../../samples/docx-mvp-render/flutter/assets/sample2.docx"))
        .expect("sample2.docx")
}

fn cjk_fonts() -> Vec<FontBlob> {
    let unicode = PathBuf::from("/System/Library/Fonts/Supplemental/Arial Unicode.ttf");
    assert!(unicode.is_file(), "need Arial Unicode.ttf for sample2");
    vec![
        mvp_font(),
        FontBlob {
            id: "宋体".into(),
            bytes: std::fs::read(&unicode).unwrap(),
        },
    ]
}

const LATIN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/03_function_line_corpus/java/latin.json"
));

#[test]
fn LV1_latin_live_not_static_json() {
    let fonts = [mvp_font()];
    let view = ViewParams::default_a4();
    let docx = hello_docx();
    let live = from_layout(&docx, &fonts, &view).expect("live hello");
    assert_eq!(live[0].start_cp, 0);
    let live_json = to_stage2_json(&live).expect("json");
    assert_ne!(live_json.replace([' ', '\n'], ""), LATIN.replace([' ', '\n'], ""));
    let again = from_layout(&docx, &fonts, &view).expect("second live");
    assert!(
        compare_lines(live.clone(), again, 0).is_ok(),
        "two live layouts of the same docx must match"
    );
    run_live_corpus(
        &[LiveCase {
            name: "latin-hello",
            docx: Some(&docx),
            fonts: &fonts,
            view: &view,
            java_json: Some(&live_json),
        }],
        0,
    )
    .expect("live dump vs second live layout");
}

#[test]
fn LV2_cjk_injected_font() {
    let fonts = cjk_fonts();
    let rows = from_layout(&sample2_docx(), &fonts, &ViewParams::default_a4()).expect("sample2");
    assert!(!rows.is_empty());
    assert_eq!(rows[0].start_cp, 0);
}

#[test]
fn LV3_heading_cp() {
    let fonts = cjk_fonts();
    let rows = from_layout(&sample2_docx(), &fonts, &ViewParams::default_a4()).expect("sample2");
    assert!(rows[0].end_cp > rows[0].start_cp, "title line must have a cp range");
}

#[test]
fn LV4_sample2_title_x_beyond_origin() {
    let fonts = cjk_fonts();
    let rows = from_layout(&sample2_docx(), &fonts, &ViewParams::default_a4()).expect("sample2");
    assert!(
        rows[0].x > 0,
        "centered title getLeft should be >0, got {}",
        rows[0].x
    );
}

#[test]
fn LV5_missing_java_dump_is_error() {
    let fonts = [mvp_font()];
    let view = ViewParams::default_a4();
    let docx = hello_docx();
    let e = run_live_corpus(
        &[LiveCase {
            name: "no-java",
            docx: Some(&docx),
            fonts: &fonts,
            view: &view,
            java_json: None,
        }],
        0,
    )
    .unwrap_err();
    assert_eq!(e.diff.field, "java-json");
    let e2 = run_live_corpus(
        &[LiveCase {
            name: "bad-java",
            docx: Some(&docx),
            fonts: &fonts,
            view: &view,
            java_json: Some("{"),
        }],
        0,
    )
    .unwrap_err();
    assert_eq!(e2.diff.field, "java-json");
}
