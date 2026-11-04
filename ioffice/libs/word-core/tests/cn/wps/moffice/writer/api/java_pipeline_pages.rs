// function_id=06_function_product_switch
// cargo test --test 06_function_product_switch
// ★wr-29 06 —— 产品切线：layout_document 默认 Java 管线；GraphBuffFactory；删 attach_pageview_hot_path
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    java_layout_text_document, java_pipeline_pages, layout_document, layout_document_stage2,
    layout_text_document, FontBlob, ViewParams, PIPELINE_LAYOUT_V1, DEFAULT_MVP_FONT,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphBuffFactory::GraphBuffFactory;

const HELLO: &str = "tests/fixtures/p1_layout_document/hello-world.docx";

fn hello_fonts() -> Vec<FontBlob> {
    vec![FontBlob {
        id: "DEFAULT_MVP_FONT".into(),
        bytes: DEFAULT_MVP_FONT.to_vec(),
    }]
}

fn hello_view() -> ViewParams {
    ViewParams {
        page_width_twip: 11906,
        page_height_twip: 16838,
        margin_left_twip: 1134,
        margin_right_twip: 1134,
        margin_top_twip: 1440,
        margin_bottom_twip: 1440,
        content_width_twip: Some(9626),
        content_height_twip: Some(12454),
        ..Default::default()
    }
}

fn bytes_to_doc(bytes: &[u8]) -> Box<TextDocument> {
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    DocumentOpen::fill_text_document_from_docx(&mut doc, bytes, "").expect("import");
    doc
}

#[test]
fn J1_java_pipeline_hello_produces_pages_with_lines() {
    let bytes = std::fs::read(HELLO).expect("fixture");
    let doc = bytes_to_doc(&bytes);
    let pages = java_pipeline_pages(&doc, &hello_fonts(), &hello_view()).expect("java pages");
    assert_eq!(pages.len(), 1, "hello 单页");
    assert!(!pages[0].lines.is_empty(), "≥1 行");
    let line = &pages[0].lines[0].line;
    assert_eq!(line.getStartCP(), 0);
    assert!(line.getEndCP() > 0, "有正文");
}

#[test]
fn J9_new_paragraph_keeps_absolute_cp_after_graph_index_reset() {
    let bytes = std::fs::read("tests/fixtures/p1_layout_document/two-para.docx")
        .expect("two paragraphs");
    let doc = bytes_to_doc(&bytes);
    let pages = java_pipeline_pages(&doc, &hello_fonts(), &hello_view())
        .expect("java pages");
    let lines: Vec<_> = pages.iter().flat_map(|page| &page.lines).collect();
    assert!(lines.len() >= 2);
    assert_eq!(lines[0].line.getStartCP(), 0);
    assert!(lines[1].line.getStartCP() > 0,
        "new TypoPara graph index zero must resolve to the next paragraph CP");
    assert_eq!(lines[1].line.getStartCP(), lines[0].line.getEndCP());
}


/// J2 — Java 栈 Frame：hello → 1 页、"Hello World!" 文字命令、margin 绝对坐标契约
#[test]
fn J2_java_frame_hello_render_contract() {
    let bytes = std::fs::read(HELLO).expect("fixture");
    let doc = bytes_to_doc(&bytes);
    let frame = java_layout_text_document(&doc, &hello_fonts(), &ViewParams::default_a4())
        .expect("java frame");
    assert_eq!(
        frame.pipeline, PIPELINE_LAYOUT_V1,
        "06 默认栈：Java 引擎输出同 v1 格式契约"
    );
    assert_eq!(frame.page.page_count, 1);
    let texts = frame.text_commands();
    assert!(!texts.is_empty(), "应有 DrawText");
    assert!(texts[0].text.contains("Hello"), "{}", texts[0].text);
    assert_eq!(texts[0].x, 1800, "page-absolute x 含 margin_left");
    assert_eq!(texts[0].size, 240);
    assert_eq!(texts[0].page, 0);
}

/// J3 — 产品入口 layout_document == java_layout_text_document（G-cut：默认即 Java 栈）
#[test]
fn J3_layout_document_is_java_stack() {
    let bytes = std::fs::read(HELLO).expect("fixture");
    let fonts = hello_fonts();
    let view = ViewParams::default_a4();
    let java = java_layout_text_document(&bytes_to_doc(&bytes), &fonts, &view).expect("java");
    let product = layout_document(&bytes, &fonts, &view).expect("product");
    let via_text = layout_text_document(&bytes_to_doc(&bytes), &fonts, &view).expect("text doc");
    let jt: Vec<String> = java.text_commands().iter().map(|t| t.text.clone()).collect();
    let pt: Vec<String> = product
        .text_commands()
        .iter()
        .map(|t| t.text.clone())
        .collect();
    let tt: Vec<String> = via_text
        .text_commands()
        .iter()
        .map(|t| t.text.clone())
        .collect();
    assert_eq!(jt, pt, "layout_document 默认 Java 栈");
    assert_eq!(jt, tt, "layout_text_document 默认 Java 栈");
    assert!(pt.iter().any(|s| s.contains("Hello")), "产品 Frame 有正文");
}

/// J4 — GraphBuffFactory.createGraphBuff → GraphFactory（Java `return new GraphFactory`）
#[test]
fn J4_graph_buff_factory_returns_graph_factory() {
    let gf = GraphBuffFactory::createGraphBuff(
        MainState::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
    );
    assert_eq!(gf.test_op_runType(), 1, "GraphFactory 初值 runType=1");
}

/// J5 — G-cut：默认 layout_document 路径不调用 env::layout_lines；占位 attach_pageview 已删
#[test]
fn J5_g_cut_no_layout_lines_on_default_path() {
    let src = include_str!(
        "../../../../../../src/cn/wps/moffice/writer/api/LayoutDocument.rs"
    );
    assert!(
        !src.contains("fn attach_pageview_hot_path"),
        "attach_pageview_hot_path 必须删除"
    );
    assert!(
        !src.contains("attach_pageview_hot_path()"),
        "不得再调用 attach_pageview_hot_path"
    );
    let default_fn = src
        .split("pub fn layout_document(")
        .nth(1)
        .and_then(|rest| rest.split("pub fn extract_paragraphs_from_docx").next())
        .expect("layout_document body");
    assert!(
        !default_fn.contains("layout_lines"),
        "layout_document 默认路径不得调用 layout_lines"
    );
    let java_host = src
        .split("pub fn java_pipeline_pages(")
        .nth(1)
        .and_then(|rest| rest.split("pub fn java_layout_text_document").next())
        .expect("java_pipeline_pages body");
    assert!(
        !java_host.contains("layout_lines"),
        "java_pipeline_pages 不得调用 layout_lines"
    );
    assert!(
        java_host.contains("GraphBuffFactory::createGraphBuff"),
        "产品宿主必须走 GraphBuffFactory"
    );
    assert!(
        java_host.contains("doFlowTextLine"),
        "产品宿主必须走 TextLayouter.doFlowTextLine（→ LineLayouter.flow）"
    );
}

/// J6 — Stage2 回退仍可用（显式 layout_document_stage2，不经默认入口）
#[test]
fn J6_stage2_legacy_still_layouts_hello() {
    let bytes = std::fs::read(HELLO).expect("fixture");
    let rows = layout_document_stage2(&bytes, &hello_fonts(), &ViewParams::default_a4())
        .expect("stage2");
    assert!(!rows.is_empty(), "Stage2 回退仍出行");
    assert_eq!(rows[0].start_cp, 0);
    assert!(rows[0].end_cp > 0);
}

/// J8 — G-P0-pages：切线后 Java 栈仍消费 rope `'\f'` → page_count≥2
#[test]
fn J8_java_stack_multipage_form_feed() {
    let path = "tests/fixtures/03_function_line_corpus/docx/multipage.docx";
    let bytes = std::fs::read(path).expect("multipage");
    let mut fonts = hello_fonts();
    for cand in [
        "/System/Library/Fonts/Geneva.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ] {
        if let Ok(b) = std::fs::read(cand) {
            fonts = vec![FontBlob {
                id: "DEFAULT_MVP_FONT".into(),
                bytes: b,
            }];
            break;
        }
    }
    let frame = layout_document(&bytes, &fonts, &ViewParams::default_a4()).expect("multipage");
    assert!(
        frame.page.page_count >= 2,
        "Java 栈必须消费 '\\f'：page_count={} ",
        frame.page.page_count
    );
    let pages: std::collections::BTreeSet<i32> = frame
        .text_commands()
        .iter()
        .map(|t| t.page)
        .collect();
    assert!(
        pages.contains(&1) || frame.page.page_count >= 2,
        "第二页应有独立页索引或 page_count≥2"
    );
}

/// J7 — G-probe：hello 产品 Frame 有 DrawText，行来自 Java TypoTextLine（startCP/endCP）
#[test]
fn J7_product_hello_drawtext_from_java_lines() {
    let bytes = std::fs::read(HELLO).expect("fixture");
    let fonts = hello_fonts();
    let view = ViewParams::default_a4();
    let pages = java_pipeline_pages(&bytes_to_doc(&bytes), &fonts, &view).expect("pages");
    let frame = layout_document(&bytes, &fonts, &view).expect("frame");
    assert_eq!(pages.len() as i32, frame.page.page_count);
    assert!(
        pages[0].lines[0].line.getEndCP() > 0,
        "Java 行 endCP>0（LineLayouter.flow 写入）"
    );
    let texts = frame.text_commands();
    assert!(!texts.is_empty());
    assert!(texts[0].text.contains("Hello"));
}
