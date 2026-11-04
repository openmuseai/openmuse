// function_id=07_function_e2e_render
// cargo test --test 07_function_e2e_render
// ★wr-29 07 —— 真实 docx 端到端：hello / sample2 / multipage 走产品 Java 栈 Frame
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    java_pipeline_pages, layout_document, FontBlob, ViewParams, DEFAULT_MVP_FONT,
    PIPELINE_LAYOUT_V1,
};
use word_core::cn::wps::moffice::writer::cache::Stage2Rows::rows_from_pages;
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

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

/// E1 — hello-world.docx 产品 Frame：≥1 DrawText 含 Hello，page_count≥1
#[test]
fn E1_hello_world_java_stack_paints() {
    let bytes = std::fs::read("tests/fixtures/p1_layout_document/hello-world.docx").unwrap();
    let frame = layout_document(&bytes, &geneva_or_mvp(), &a4()).expect("hello-world");
    assert_eq!(frame.pipeline, PIPELINE_LAYOUT_V1);
    assert!(frame.page.page_count >= 1);
    let texts = frame.text_commands();
    assert!(!texts.is_empty(), "hello 应有 DrawText");
    assert!(
        texts.iter().any(|t| t.text.contains("Hello")),
        "正文可见: {:?}",
        texts.iter().map(|t| t.text.as_str()).collect::<Vec<_>>()
    );
    assert_eq!(texts[0].page, 0);
}

/// E2 — App hello.docx（Flutter 资产）：DrawText 非空（与 screenshot 同语料）
#[test]
fn E2_app_hello_docx_drawtext() {
    let path = "../../samples/docx-mvp-render/flutter/assets/hello.docx";
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let frame = layout_document(&bytes, &geneva_or_mvp(), &a4()).expect("app hello");
    let texts = frame.text_commands();
    assert!(!texts.is_empty());
    assert!(texts[0].text.contains("Hello"), "{}", texts[0].text);
    assert!(texts[0].xpos.len() >= 1);
}

/// E3 — two-para.docx：≥2 条 DrawText（两段）
#[test]
fn E3_two_para_two_runs() {
    let bytes = std::fs::read("tests/fixtures/p1_layout_document/two-para.docx").unwrap();
    let frame = layout_document(&bytes, &geneva_or_mvp(), &a4()).expect("two-para");
    let texts = frame.text_commands();
    assert!(texts.len() >= 2, "两段应有 ≥2 DrawText，实际 {}", texts.len());
    assert!(texts[1].y > texts[0].y, "第二段基线应低于第一段");
}

/// E4 — multipage.docx：Java 栈 page_count≥2，第二页有正文
#[test]
fn E4_multipage_two_pages() {
    let bytes =
        std::fs::read("tests/fixtures/03_function_line_corpus/docx/multipage.docx").unwrap();
    let frame = layout_document(&bytes, &geneva_or_mvp(), &a4()).expect("multipage");
    assert!(
        frame.page.page_count >= 2,
        "page_count={}",
        frame.page.page_count
    );
    let pages: Vec<i32> = frame.text_commands().iter().map(|t| t.page).collect();
    assert!(
        pages.iter().any(|p| *p >= 1),
        "第二页应有 DrawText，pages={pages:?}"
    );
}

/// E5 — sample2.docx 标题：CJK 注入后 DrawText 非空；居中标题 x>左边距 1800
#[test]
fn E5_sample2_title_visible() {
    let path = "../../samples/docx-mvp-render/flutter/assets/sample1/sample2.docx";
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let fonts = cjk_fonts();
    let frame = layout_document(&bytes, &fonts, &a4()).expect("sample2");
    let texts = frame.text_commands();
    assert!(!texts.is_empty(), "sample2 应有正文");
    assert!(texts[0].text.contains("大学生"), "{}", texts[0].text);
    assert_eq!(texts[0].size, 360, "18pt 标题");
    assert!(texts[0].x > 1800, "居中标题 x>左边距误测，got {}", texts[0].x);
    assert_eq!(texts[0].font_id, "宋体");
    assert_eq!(texts[0].style & 1, 1);
}

/// E6 — L2 行表来自 Java TypoTextLine（rows_from_pages），与产品 Frame 页数一致
#[test]
fn E6_l2_rows_from_java_pages() {
    use word_core::cn::wps::moffice::writer::api::DocumentOpen;
    use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
    let bytes = std::fs::read("tests/fixtures/p1_layout_document/hello-world.docx").unwrap();
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import");
    let fonts = geneva_or_mvp();
    let view = a4();
    let pages = java_pipeline_pages(&doc, &fonts, &view).expect("pages");
    let rows = rows_from_pages(&pages);
    assert!(!rows.is_empty());
    assert_eq!(rows[0].start_cp, 0);
    assert!(rows[0].end_cp > 0);
    let frame = layout_document(&bytes, &fonts, &view).expect("frame");
    assert_eq!(pages.len() as i32, frame.page.page_count);
}

/// E7 — G-cmd：产品指令非空；无手写 layout_lines 在默认入口
#[test]
fn E7_product_commands_nonempty_java_path() {
    let src = include_str!(
        "../../../../../../src/cn/wps/moffice/writer/api/LayoutDocument.rs"
    );
    let default_fn = src
        .split("pub fn layout_document(")
        .nth(1)
        .and_then(|rest| rest.split("pub fn extract_paragraphs_from_docx").next())
        .expect("layout_document");
    assert!(!default_fn.contains("layout_lines"));
    let bytes = std::fs::read("tests/fixtures/p1_layout_document/hello-world.docx").unwrap();
    let frame = layout_document(&bytes, &geneva_or_mvp(), &a4()).expect("hello");
    assert!(!frame.commands.is_empty());
    assert!(matches!(
        frame.commands[0],
        RenderCommand::Text(_) | RenderCommand::BeginPage(_)
    ));
}

/// E8 — a real DOCX rFonts theme attribute reaches the imported CHPX table.
#[test]
fn E8_theme_font_real_sax_to_chpx() {
    use word_core::cn::wps::moffice::writer::api::DocumentOpen;
    use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
    use word_core::cn::wps::moffice::writer::layout::base::text::RunStream::RunStream;
    let bytes = std::fs::read("tests/fixtures/m3_09h/theme-font-sample2.docx").unwrap();
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import theme DOCX");
    let runs = RunStream::collectMainStoryRuns(doc.getMainDocument().expect("main"));
    let theme = doc.getTheme_mut().expect("theme");
    assert!(unsafe { &*theme }.getPackagePart().is_some(), "theme package part did not reach TextDocument");
    let scheme = unsafe { &mut *(*theme).getThemeFontScheme_mut() };
    assert!(scheme.getMajorFont().is_some(), "theme major font did not reach TextDocument");
    assert!(scheme.getMinorFont().is_some(), "theme minor font did not reach TextDocument");
    let title = runs.iter().find(|run| run.text.contains("大学生")).expect("themed title run");
    assert_eq!(title.chpx.getString__int_string(3, ""), "等线 Light");
}
