// function_id=08_function_wrap_para_env
// cargo test --test 08_function_wrap_para_env
// ★wr-29 08 —— sample2 折行 + 首行缩进 + 段间距；结构对齐 WPS sample2_01（行内像素差允许）
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document, FontBlob, ViewParams, DEFAULT_MVP_FONT, PIPELINE_LAYOUT_V1,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::DrawText;

const SAMPLE2: &str =
    "../../samples/docx-mvp-render/flutter/assets/samples/sample2.docx";
const HELLO: &str = "tests/fixtures/p1_layout_document/hello-world.docx";

fn mvp_font() -> FontBlob {
    FontBlob {
        id: "DEFAULT_MVP_FONT".into(),
        bytes: DEFAULT_MVP_FONT.to_vec(),
    }
}

fn geneva_or_mvp() -> Vec<FontBlob> {
    for cand in [
        "/System/Library/Fonts/Geneva.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ] {
        if let Ok(b) = std::fs::read(cand) {
            return vec![FontBlob {
                id: "DEFAULT_MVP_FONT".into(),
                bytes: b,
            }];
        }
    }
    vec![mvp_font()]
}

fn cjk_fonts() -> Vec<FontBlob> {
    let mut fonts = geneva_or_mvp();
    let unicode = "/System/Library/Fonts/Supplemental/Arial Unicode.ttf";
    if let Ok(b) = std::fs::read(unicode) {
        fonts.push(FontBlob {
            id: "宋体".into(),
            bytes: b,
        });
    }
    fonts
}

fn a4() -> ViewParams {
    ViewParams::default_a4()
}

fn dump_texts(tag: &str, texts: &[&DrawText]) {
    eprintln!("=== {tag} n={} ===", texts.len());
    for (i, t) in texts.iter().take(80).enumerate() {
        let end_x = t.xpos.last().copied().unwrap_or(t.x);
        let preview: String = t.text.chars().take(28).collect();
        eprintln!(
            "  [{i:02}] p={} y={:<5} x={:<5} endx={:<5} sz={} nch={} | {preview}",
            t.page,
            t.y,
            t.x,
            end_x,
            t.size,
            t.text.chars().count()
        );
    }
}

fn layout_sample2() -> (word_core::cn::wps::moffice::writer::api::LayoutDocument::LayoutFrame, Vec<DrawText>) {
    let bytes = std::fs::read(SAMPLE2).unwrap_or_else(|e| panic!("{SAMPLE2}: {e}"));
    let frame = layout_document(&bytes, &cjk_fonts(), &a4()).expect("sample2");
    let texts: Vec<DrawText> = frame
        .text_commands()
        .into_iter()
        .cloned()
        .collect();
    (frame, texts)
}

/// W0 — dump：对照 WPS sample2_01 结构（标题 / 概述 / 折行正文 / 调查方式）
#[test]
fn W0_dump_sample2_drawtexts() {
    let (frame, texts) = layout_sample2();
    eprintln!("pipeline={} page_count={}", frame.pipeline, frame.page.page_count);
    dump_texts("sample2", &texts.iter().collect::<Vec<_>>());
    assert!(!texts.is_empty());
}

/// W1 — 第一段正文必须折行：≥3 条 DrawText 含「近年来」，每条可视宽 < 内容区
#[test]
fn W1_sample2_first_body_wraps() {
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
    assert!(
        body.len() >= 3,
        "WPS p1 第一段约 6 行；未折行则 1 条溢出。got {} {:?}",
        body.len(),
        body.iter().map(|t| t.text.chars().take(20).collect::<String>()).collect::<Vec<_>>()
    );
    let content_w = a4().content_width();
    for t in body {
        let span = t.xpos.last().copied().unwrap_or(t.x).saturating_sub(t.x);
        assert!(
            span < content_w + 200,
            "行溢出 content_w={content_w} span={span} text={}",
            t.text.chars().take(40).collect::<String>()
        );
    }
}

/// W2 — 首行缩进：第一段正文首行 x ≳ 左边距+480twip（PAPX 208）
#[test]
fn W2_first_line_indent() {
    let (_frame, texts) = layout_sample2();
    let first = texts
        .iter()
        .find(|t| t.text.contains("近年来"))
        .expect("first body");
    let left = a4().margin_left_twip;
    assert!(
        first.x >= left + 400,
        "首行应缩进 PAPX208=480；got x={} left={left}",
        first.x
    );
}

/// W3 — 空段/1.5 倍行距造成的纵向间隙：概述与第一段正文、两段正文之间 y 差 > 单倍行高
#[test]
fn W3_empty_para_and_line_spacing_gaps() {
    let (_frame, texts) = layout_sample2();
    let overview = texts.iter().find(|t| t.text.contains("概述")).expect("概述");
    let first = texts.iter().find(|t| t.text.contains("近年来")).expect("近年来");
    let second = texts.iter().find(|t| t.text.contains("然而")).expect("然而");
    assert!(
        first.y - overview.y > 240,
        "概述与第一段之间应有空段间隙，dy={}",
        first.y - overview.y
    );
    assert!(
        second.y - first.y > 400,
        "两段正文之间应有空段+1.5 行距，dy={}",
        second.y - first.y
    );
}

/// W4 — 折行后页数向 WPS 8 页靠拢（至少 >1；结构门，非整页 SSIM）
#[test]
fn W4_page_count_after_wrap() {
    let (frame, texts) = layout_sample2();
    eprintln!("page_count={} n_draw={}", frame.page.page_count, texts.len());
    assert!(
        frame.page.page_count >= 2,
        "WPS=8 页；未折行时常 1 页 overflow。got {}",
        frame.page.page_count
    );
    assert!(texts.iter().any(|t| t.page >= 1), "第二页应有正文");
}

/// W5 — hello 仍单行（回归 05/07）
#[test]
fn W5_hello_still_one_line() {
    let bytes = std::fs::read(HELLO).unwrap();
    let frame = layout_document(&bytes, &geneva_or_mvp(), &a4()).expect("hello");
    let texts = frame.text_commands();
    assert_eq!(texts.len(), 1, "hello 不应被误折，got {:?}", texts.iter().map(|t| t.text.as_str()).collect::<Vec<_>>());
    assert!(texts[0].text.contains("Hello"));
}

/// W6 — 标题仍居中 18pt（回归 07 E5）
#[test]
fn W6_title_still_centered() {
    let (_frame, texts) = layout_sample2();
    assert!(texts[0].text.contains("大学生"), "{}", texts[0].text);
    assert_eq!(texts[0].size, 360);
    assert!(texts[0].x > 1800, "居中标题 x>1800 got {}", texts[0].x);
}

/// W7 — WPS p1 可见块顺序：标题 → 概述 → 近年来… → 然而… → 农村中小学… → 调查方式
#[test]
fn W7_wps_page1_block_order() {
    let (_frame, texts) = layout_sample2();
    let joined: String = texts.iter().map(|t| t.text.as_str()).collect();
    let i_title = joined.find("大学生社会实践").expect("title");
    let i_ov = joined.find("概述").expect("概述");
    let i_p1 = joined.find("近年来").expect("近年来");
    let i_p2 = joined.find("然而").expect("然而");
    let i_p3 = joined.find("农村中小学教育所存在").expect("p3");
    let i_way = joined.find("调查方式").expect("调查方式");
    assert!(i_title < i_ov && i_ov < i_p1 && i_p1 < i_p2 && i_p2 < i_p3 && i_p3 < i_way);
}

/// W8 — 产品入口仍是 Java 栈（无 layout_lines）
#[test]
fn W8_product_java_path() {
    let src = include_str!(
        "../../../../../../../../src/cn/wps/moffice/writer/api/LayoutDocument.rs"
    );
    let default_fn = src
        .split("pub fn layout_document(")
        .nth(1)
        .and_then(|rest| rest.split("pub fn extract_paragraphs_from_docx").next())
        .expect("layout_document");
    assert!(!default_fn.contains("layout_lines"));
    let (frame, _) = layout_sample2();
    assert_eq!(frame.pipeline, PIPELINE_LAYOUT_V1);
}
