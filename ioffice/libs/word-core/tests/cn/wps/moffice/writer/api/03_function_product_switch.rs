// function_id=03_function_product_switch
#![allow(non_snake_case)]

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document, layout_source_runs, FontBlob, ViewParams,
};
use word_core::cn::wps::moffice::writer::layout::FlowLayout::{FlowLayout, MainLayouter};

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
    std::fs::read(
        crate_root().join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

fn sample2_docx() -> Vec<u8> {
    std::fs::read(
        crate_root().join("../../samples/docx-mvp-render/flutter/assets/sample1/sample2.docx"),
    )
    .expect("sample2.docx")
}

#[test]
fn SW1_hello_drawable_on_pageview_mode() {
    assert_eq!(MainLayouter::PAGEVIEW_LAYOUTER, 0);
    let mut fl = FlowLayout::default();
    fl.createDocLayouter(MainLayouter::PAGEVIEW_LAYOUTER);
    assert!(fl.mMainLayouter.is_some());
    let frame = layout_document(&hello_docx(), &[mvp_font()], &ViewParams::default_a4())
        .expect("layout hello");
    let texts = frame.text_commands();
    assert!(!texts.is_empty(), "expected DrawText Hello");
    assert_eq!(texts[0].text, "Hello");
}

#[test]
fn SW2_sample2_title_x_beyond_left_margin() {
    let unicode = PathBuf::from("/System/Library/Fonts/Supplemental/Arial Unicode.ttf");
    assert!(unicode.is_file(), "need CJK-capable Arial Unicode.ttf for sample2");
    let fonts = [
        mvp_font(),
        FontBlob {
            id: "宋体".into(),
            bytes: std::fs::read(&unicode).unwrap(),
        },
    ];
    let frame = layout_document(&sample2_docx(), &fonts, &ViewParams::default_a4())
        .expect("layout sample2");
    let texts = frame.text_commands();
    assert!(!texts.is_empty(), "sample2 must emit text");
    assert!(
        texts[0].x > 1800,
        "title should be centered (x>margin), got x={}",
        texts[0].x
    );
}

#[test]
fn SW3_wr04_source_runs_still_green() {
    let frame = layout_source_runs(
        &["Hello".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("source runs");
    assert!(!frame.text_commands().is_empty());
}
