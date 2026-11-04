// function_id=09_function_section_grid
// cargo test --test 09_function_section_grid -- --test-threads=1
// ★wr-29 09 —— sample2 sectPr docGrid → SectionTool.calcGrid → 行高贴网格；WPS p1 不含「三、」
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document, open_text_document, FontBlob, ViewParams, DEFAULT_MVP_FONT,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::DrawText;

const SAMPLE2: &str =
    "../../samples/docx-mvp-render/flutter/assets/samples/sample2.docx";
const HELLO: &str = "tests/fixtures/p1_layout_document/hello-world.docx";

fn cjk_fonts() -> Vec<FontBlob> {
    let mut fonts = Vec::new();
    for cand in [
        "/System/Library/Fonts/Geneva.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ] {
        if let Ok(b) = std::fs::read(cand) {
            fonts.push(FontBlob {
                id: "DEFAULT_MVP_FONT".into(),
                bytes: b,
            });
            break;
        }
    }
    if fonts.is_empty() {
        fonts.push(FontBlob {
            id: "DEFAULT_MVP_FONT".into(),
            bytes: DEFAULT_MVP_FONT.to_vec(),
        });
    }
    let unicode = "/System/Library/Fonts/Supplemental/Arial Unicode.ttf";
    if let Ok(b) = std::fs::read(unicode) {
        fonts.push(FontBlob {
            id: "宋体".into(),
            bytes: b,
        });
    }
    fonts
}

fn layout_sample2() -> (
    word_core::cn::wps::moffice::writer::api::LayoutDocument::LayoutFrame,
    Vec<DrawText>,
) {
    let bytes = std::fs::read(SAMPLE2).unwrap();
    let frame = layout_document(&bytes, &cjk_fonts(), &ViewParams::default_a4()).expect("sample2");
    let texts: Vec<DrawText> = frame.text_commands().into_iter().cloned().collect();
    (frame, texts)
}

/// G1 — parse 后 PLCSection.sepx 含 docGrid type=lines(677=2)、linePitch=312
#[test]
fn G1_sample2_section_docgrid_imported() {
    let bytes = std::fs::read(SAMPLE2).unwrap();
    let doc = open_text_document(&bytes).expect("open");
    let plc = doc.getPLCSection().expect("PLCSection");
    let sec = plc
        .getLastSection()
        .or_else(|| plc.getFirstSection())
        .expect("section");
    let sepx = sec.getPropertySet();
    assert_eq!(
        sepx.getInt__int_int(677, -1),
        2,
        "w:docGrid type=lines → Java convertDocGridType=2"
    );
    assert_eq!(
        sepx.getInt__int_int(678, -1),
        312,
        "w:docGrid linePitch=312"
    );
}

/// G2 — 正文相邻行 dy 贴 1.5×312=468 网格（允许 ±40 twip 度量差）
#[test]
fn G2_body_line_height_snaps_to_grid() {
    let (_frame, texts) = layout_sample2();
    let start = texts
        .iter()
        .position(|t| t.text.contains("近年来"))
        .expect("近年来");
    let end = texts
        .iter()
        .position(|t| t.text.contains("然而"))
        .expect("然而");
    let body = &texts[start..end];
    assert!(body.len() >= 2, "折行后至少两行");
    let dy = body[1].y - body[0].y;
    assert!(
        (dy - 468).abs() <= 40,
        "Java auto 1.5 × gridHeight 312 = 468; got dy={dy}"
    );
}

/// G3 — WPS sample2_01 在「三、」之前分页；Rust p0 不得出现该标题
#[test]
fn G3_wps_page1_excludes_section_three() {
    let (_frame, texts) = layout_sample2();
    let p0: String = texts
        .iter()
        .filter(|t| t.page == 0)
        .map(|t| t.text.as_str())
        .collect();
    assert!(p0.contains("大学生社会实践"), "p0 标题");
    assert!(p0.contains("概述"), "p0 概述");
    assert!(p0.contains("近年来"), "p0 第一段");
    assert!(p0.contains("调查方式"), "p0 调查方式");
    assert!(
        !p0.contains("三、农村中小学教育现状"),
        "WPS p1 在「三、」前结束；got p0 ending {:?}",
        texts
            .iter()
            .filter(|t| t.page == 0)
            .last()
            .map(|t| t.text.chars().take(24).collect::<String>())
    );
}

/// G4 — hello 回归仍单行
#[test]
fn G4_hello_still_one_line() {
    let bytes = std::fs::read(HELLO).unwrap();
    let fonts = cjk_fonts();
    let frame = layout_document(&bytes, &fonts[..1], &ViewParams::default_a4()).expect("hello");
    let texts = frame.text_commands();
    assert_eq!(texts.len(), 1);
    assert!(texts[0].text.contains("Hello"));
}

/// G5 — 标题仍居中
#[test]
fn G5_title_centered() {
    let (_frame, texts) = layout_sample2();
    assert!(texts[0].text.contains("大学生"));
    assert!(texts[0].x > 1800, "居中 x>1800 got {}", texts[0].x);
}

/// G6 — 产品仍 Java 栈
#[test]
fn G6_product_java_path() {
    let src = include_str!(
        "../../../../../../../src/cn/wps/moffice/writer/api/LayoutDocument.rs"
    );
    assert!(src.contains("SectionTool::calcGrid"));
    let (frame, _) = layout_sample2();
    assert_eq!(frame.pipeline, "word-core-layout-v1");
}
