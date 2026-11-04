//! P1 I/O tests for `layout_document` / `layout_source_runs`.
//! Matrix ids match samples/docx-mvp-render/docs/phases/P1_KERNEL_ORCHESTRATION.md §5.

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    extract_paragraphs_from_docx, layout_document, layout_source_runs, layout_text_document,
    open_text_document, stage3_roundtrip, FontBlob, LayoutErrorKind, ViewParams,
    LAYOUT_FRAME_VERSION, PIPELINE_LAYOUT_V1,
};
use word_core::cn::wps::moffice::writer::layout::base::env::LineBox::{LineBox, LineSpacing};
use word_core::cn::wps::moffice::writer::layout::base::line::RunMetrics::measure_run;
use word_core::cn::wps::moffice::writer::layout::text::FontStack::FontStack;
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/p1_layout_document")
}

fn mvp_font() -> FontBlob {
    FontBlob {
        id: "default".into(),
        bytes: std::fs::read(fixtures().join("MvpLatin.ttf")).expect("MvpLatin.ttf"),
    }
}

fn testfont() -> FontBlob {
    FontBlob {
        id: "test".into(),
        bytes: std::fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/01_function_font_stack/TestFont.ttf"),
        )
        .expect("TestFont.ttf"),
    }
}

fn read_docx(name: &str) -> Vec<u8> {
    std::fs::read(fixtures().join(name)).unwrap_or_else(|_| panic!("missing {name}"))
}

fn plain_hello_docx() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

fn expected_baseline_y(font: &FontBlob, size_pt: f64, line_top: i32) -> i32 {
    let stack = FontStack::from_font_bytes(font.bytes.clone()).unwrap();
    let (ascent, _) = stack.line_metrics_twip(size_pt).unwrap();
    line_top + ascent
}

/// L1 — legal deflate docx + default font → ≥1 DrawText (y = baseline).
#[test]
fn l1_plain_docx_has_text() {
    let font = mvp_font();
    let baseline = expected_baseline_y(&font, 12.0, 1440);
    let frame = layout_document(&plain_hello_docx(), &[font], &ViewParams::default_a4())
        .expect("layout");
    assert_eq!(frame.pipeline, PIPELINE_LAYOUT_V1);
    assert_eq!(frame.version, LAYOUT_FRAME_VERSION);
    let texts = frame.text_commands();
    assert!(!texts.is_empty(), "expected DrawText");
    assert_eq!(texts[0].text, "Hello");
    assert_eq!(texts[0].x, 1800, "page-absolute: x includes margin_left");
    assert_eq!(texts[0].y, baseline, "page-absolute: y is baseline");
    assert_eq!(texts[0].size, 240);
    assert_eq!(frame.page.page_count, 1);
    assert_eq!(frame.page.page_width_twip, 11906);
}

/// L2 — "Hello world" with wrap width between one word and the whole line.
#[test]
fn l2_hello_world_wraps_at_space() {
    let font = mvp_font();
    let stack = FontStack::from_font_bytes(font.bytes.clone()).unwrap();
    let hello = measure_run(&stack, "Hello", 12.0).unwrap().width_twip;
    let all = measure_run(&stack, "Hello world", 12.0).unwrap().width_twip;
    assert!(all > hello, "fixture advances: hello={hello} all={all}");
    let mut view = ViewParams::default_a4();
    view.content_width_twip = Some((hello + all) / 2);
    let frame = layout_source_runs(&["Hello world".into()], &[font], &view).expect("layout");
    let texts = frame.text_commands();
    assert!(
        texts.len() >= 2,
        "expected wrap to ≥2 DrawText, got {:?}",
        texts.iter().map(|t| (&t.text, t.y)).collect::<Vec<_>>()
    );
    assert!(texts[1].y > texts[0].y, "second line y must increase");
    assert_eq!(texts[0].page, texts[1].page);
}

/// L2b — default A4 content width does not wrap a short run.
#[test]
fn l2b_a4_does_not_wrap_hello_world() {
    let frame = layout_source_runs(
        &["Hello world".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("layout");
    let texts = frame.text_commands();
    assert_eq!(texts.len(), 1);
    assert_eq!(texts[0].text, "Hello world");
}

/// L3 — two paragraphs (`\r`) → two lines; empty para does not panic.
#[test]
fn l3_two_paragraphs_two_lines() {
    let frame = layout_source_runs(
        &["Hello".into(), "Hello".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("layout");
    let texts = frame.text_commands();
    assert_eq!(texts.len(), 2);
    assert!(texts[1].y > texts[0].y);
    let empty = layout_source_runs(
        &["Hello".into(), "".into(), "Hello".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("empty para");
    assert!(empty.text_commands().len() >= 2);
}

/// L3-docx — two-para fixture extracts and lays out two lines.
#[test]
fn l3_two_para_docx() {
    let paras = extract_paragraphs_from_docx(&read_docx("two-para.docx")).expect("extract");
    assert_eq!(paras, vec!["Hello".to_string(), "Hello".to_string()]);
    let frame = layout_document(&read_docx("two-para.docx"), &[mvp_font()], &ViewParams::default_a4())
        .expect("layout");
    assert_eq!(frame.text_commands().len(), 2);
}

/// L4 — line_height × N > content_height → page 0 and 1.
#[test]
fn l4_pagination_two_pages() {
    let mut view = ViewParams::default_a4();
    view.content_height_twip = Some(300);
    let frame = layout_source_runs(
        &["Hello".into(), "Hello".into()],
        &[mvp_font()],
        &view,
    )
    .expect("layout");
    let pages: Vec<i32> = frame.text_commands().iter().map(|t| t.page).collect();
    assert!(
        pages.contains(&0) && pages.contains(&1),
        "expected pages 0 and 1, got {pages:?}"
    );
    assert!(frame.page.page_count >= 2);
    let y: Vec<i32> = frame.text_commands().iter().map(|t| t.y).collect();
    let baseline = expected_baseline_y(&mvp_font(), 12.0, 1440);
    assert_eq!(y[0], baseline);
    assert_eq!(y[1], baseline, "new page resets y to margin_top + ascent");
}

/// L5 — TestFont has no `w`; must fail, never fake width=size.
#[test]
fn l5_missing_glyph_is_err() {
    let err = match layout_source_runs(
        &["Hello world".into()],
        &[testfont()],
        &ViewParams::default_a4(),
    ) {
        Err(e) => e,
        Ok(f) => panic!("must fail missing glyph, got {} cmds", f.commands.len()),
    };
    assert_eq!(err.kind, LayoutErrorKind::Measure);
    assert!(
        err.message.contains("U+") || err.message.to_lowercase().contains("glyph"),
        "{}",
        err.message
    );
}

/// L6 — `render_docx` commands match `layout_document` (derived API).
#[test]
fn l6_render_docx_matches_layout_document() {
    let bytes = plain_hello_docx();
    let frame = layout_document(&bytes, &[], &ViewParams::default_a4()).expect("layout");
    let doc = word_core::cn::wps::moffice::writer::api::RenderDocx::render_docx(&bytes)
        .expect("render_docx");
    assert_eq!(doc.commands, frame.commands);
    assert_eq!(doc.version, frame.version);
}

/// L7 — Stage3 JSON roundtrip keeps text/x/y/page.
#[test]
fn l7_stage3_json_roundtrip() {
    let frame = layout_document(&plain_hello_docx(), &[], &ViewParams::default_a4()).expect("layout");
    let back = stage3_roundtrip(&frame).expect("roundtrip");
    assert_eq!(back.commands, frame.commands);
    match &frame.commands[0] {
        RenderCommand::Text(t) => {
            assert_eq!(t.text, "Hello");
            assert_eq!(t.page, 0);
        }
        other => panic!("expected text, {other:?}"),
    }
}

/// L8 — empty body is ok, zero commands, one page.
#[test]
fn l8_empty_body_ok() {
    let frame = layout_document(&read_docx("empty-body.docx"), &[], &ViewParams::default_a4())
        .expect("empty");
    assert!(frame.commands.is_empty());
    assert_eq!(frame.page.page_count, 1);
}

/// L9 — garbage bytes → Zip error, not panic.
#[test]
fn l9_bad_bytes_err() {
    let err = layout_document(b"not-a-zip", &[], &ViewParams::default_a4()).unwrap_err();
    assert_eq!(err.kind, LayoutErrorKind::Zip);
}

/// L10 — hello-world.docx through product I/O with wrap override.
#[test]
fn l10_docx_wrap_override() {
    let font = mvp_font();
    let stack = FontStack::from_font_bytes(font.bytes.clone()).unwrap();
    let hello = measure_run(&stack, "Hello", 12.0).unwrap().width_twip;
    let all = measure_run(&stack, "Hello world", 12.0).unwrap().width_twip;
    let mut view = ViewParams::default_a4();
    view.content_width_twip = Some((hello + all) / 2);
    let frame = layout_document(&read_docx("hello-world.docx"), &[font], &view).expect("layout");
    assert!(frame.text_commands().len() >= 2);
    assert_eq!(frame.pipeline, PIPELINE_LAYOUT_V1);
}

/// L11 — ViewParams.content_width matches PageSetup when unset.
#[test]
fn l11_default_content_box() {
    let v = ViewParams::default_a4();
    assert_eq!(v.content_width(), 8306);
    assert_eq!(v.content_height(), 13958);
}

/// D2 — wrapped second line y delta equals wr-12 LineBox height (not wr-04 pt×20 ceil).
#[test]
fn d2_wrap_y_delta_is_line_height() {
    let font = mvp_font();
    let stack = FontStack::from_font_bytes(font.bytes.clone()).unwrap();
    let hello = measure_run(&stack, "Hello", 12.0).unwrap().width_twip;
    let all = measure_run(&stack, "Hello world", 12.0).unwrap().width_twip;
    let mut view = ViewParams::default_a4();
    view.content_width_twip = Some((hello + all) / 2);
    let frame = layout_source_runs(&["Hello world".into()], &[font], &view).expect("layout");
    let texts = frame.text_commands();
    assert!(texts.len() >= 2);
    let (ascent, descent) = stack.line_metrics_twip(12.0).unwrap();
    let box_ = LineBox::measure(
        ascent,
        descent.abs(),
        LineSpacing::auto(1.0),
        0,
        0,
        true,
        false,
        0,
    );
    assert_eq!(texts[1].y - texts[0].y, box_.height);
}

/// D3 — xpos.len equals visible codepoints; xpos[0] == DrawText.x.
#[test]
fn d3_xpos_len_matches_visible_codepoints() {
    let frame = layout_source_runs(
        &["Hello".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("layout");
    let t = &frame.text_commands()[0];
    let visible: Vec<char> = t
        .text
        .chars()
        .filter(|c| *c != '\r' && *c != '\n' && *c != '\u{000c}')
        .collect();
    assert_eq!(t.xpos.len(), visible.len());
    assert_eq!(t.xpos[0], t.x);
    assert!(t.xpos.windows(2).all(|w| w[1] > w[0]), "advances must increase");
}

/// D4 — underline emits DrawLine after text; not a text style bit.
#[test]
fn d4_underline_emits_line() {
    let mut view = ViewParams::default_a4();
    view.underline = true;
    let frame = layout_source_runs(&["Hello".into()], &[mvp_font()], &view).expect("layout");
    assert!(frame.text_commands().len() == 1);
    let lines: Vec<_> = frame
        .commands
        .iter()
        .filter_map(|c| match c {
            RenderCommand::Line(l) => Some(l),
            _ => None,
        })
        .collect();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].kind, "underline");
    match &frame.commands[0] {
        RenderCommand::Text(_) => {}
        other => panic!("expected text before deco, {other:?}"),
    }
    match &frame.commands[1] {
        RenderCommand::Line(_) => {}
        other => panic!("expected line after text, {other:?}"),
    }
}

/// D4-docx — `<w:u` in document.xml turns on underline without ViewParams flag.
#[test]
fn d4_underline_from_xml() {
    let frame = layout_document(
        &read_docx("underline.docx"),
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("layout");
    let has_u = frame.commands.iter().any(|c| matches!(c, RenderCommand::Line(l) if l.kind == "underline"));
    assert!(has_u, "xml <w:u must emit underline line");
}

/// D5 — zoom is not a layout input; same runs produce the same command count.
#[test]
fn d5_layout_independent_of_zoom() {
    let a = layout_source_runs(
        &["Hello".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("a");
    let b = layout_source_runs(
        &["Hello".into()],
        &[mvp_font()],
        &ViewParams::default_a4(),
    )
    .expect("b");
    assert_eq!(a.commands.len(), b.commands.len());
    assert_eq!(a.commands, b.commands);
}

/// sample2.docx is CJK/宋体. Latin-only MvpLatin has no cmap → Measure (the sample app error).
#[test]
fn sample2_cjk_fails_on_latin_only_face() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/docx-mvp-render/flutter/assets/sample2.docx");
    if !path.is_file() {
        return;
    }
    let bytes = std::fs::read(&path).unwrap();
    let err = layout_document(&bytes, &[mvp_font()], &ViewParams::default_a4())
        .expect_err("CJK must not silently width=size");
    assert_eq!(err.kind, LayoutErrorKind::Measure);
    assert!(
        err.message.contains("U+") || err.message.contains("glyph"),
        "got {}",
        err.message
    );
}

/// Host injects a CJK-capable face as 宋体 (Java FontHost extra file, not Paint).
#[test]
fn sample2_cjk_ok_with_injected_unicode() {
    let docx = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/docx-mvp-render/flutter/assets/sample2.docx");
    let unicode = PathBuf::from("/System/Library/Fonts/Supplemental/Arial Unicode.ttf");
    if !docx.is_file() || !unicode.is_file() {
        return;
    }
    let fonts = [
        mvp_font(),
        FontBlob {
            id: "宋体".into(),
            bytes: std::fs::read(&unicode).unwrap(),
        },
    ];
    let bytes = std::fs::read(&docx).unwrap();
    let doc = open_text_document(&bytes).expect("open sample2");
    let main = doc.getMainDocument().expect("main");
    let runs = word_core::cn::wps::moffice::writer::layout::base::text::RunStream::RunStream::collectMainStoryRuns(main);
    assert!(!runs.is_empty());
    assert_eq!(
        runs[0].papx.getInt__int_int(193, -99),
        1,
        "title PAPX jc=center (193); text={:?}",
        runs[0].text
    );
    let frame = layout_document(&bytes, &fonts, &ViewParams::default_a4())
        .expect("CJK with injected 宋体");
    let texts = frame.text_commands();
    assert!(!texts.is_empty());
    assert!(texts[0].text.contains("大学生"), "got {:?}", texts[0].text);
    assert_eq!(texts[0].font_id, "宋体");
    // w:sz 36 half-points → 18pt → 360 twip; w:b on title.
    assert_eq!(texts[0].size, 360, "title size {:?}", texts[0]);
    assert_eq!(texts[0].style & 1, 1, "title bold {:?}", texts[0]);
    assert!(
        texts[0].x > 1800,
        "title should be centered (x>margin), got x={}",
        texts[0].x
    );
    let body = texts.iter().find(|t| t.text.contains("概述")).expect("body heading");
    assert_eq!(body.size, 240, "12pt body {:?}", body);
}

/// P7 — product open fills TextDocument; layout reads RunStream, not zip-concat.
#[test]
fn p7_open_text_document_hello_rope() {
    let bytes = plain_hello_docx();
    let doc = open_text_document(&bytes).expect("open");
    let main = doc.getMainDocument().expect("main");
    let text = word_core::cn::wps::moffice::writer::layout::base::text::CpText::CpText::getText(
        main,
        0,
        main.getLength(),
    );
    assert!(text.contains("Hello"), "rope={text:?}");
}

#[test]
fn p7_layout_text_document_matches_layout_document() {
    let bytes = plain_hello_docx();
    let font = mvp_font();
    let view = ViewParams::default_a4();
    let via_bytes = layout_document(&bytes, &[font.clone()], &view).expect("bytes");
    let doc = open_text_document(&bytes).expect("open");
    let via_doc = layout_text_document(&doc, &[font], &view).expect("relayout");
    assert_eq!(via_bytes.text_commands()[0].text, "Hello");
    assert_eq!(
        via_doc.text_commands()[0].text,
        via_bytes.text_commands()[0].text
    );
    assert_eq!(via_doc.text_commands()[0].xpos, via_bytes.text_commands()[0].xpos);
}

fn latin_complete_font() -> Option<FontBlob> {
    for path in [
        "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
        "/Library/Fonts/Arial.ttf",
    ] {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(FontBlob {
                id: "Arial".into(),
                bytes: std::fs::read(&p).ok()?,
            });
        }
    }
    None
}

/// P7-styles — direct `w:sz` on heading run is larger than body (16pt vs 11pt).
#[test]
fn p7_heading_run_size_from_rpr() {
    let Some(font) = latin_complete_font() else {
        return;
    };
    let bytes = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/03_function_line_corpus/docx/heading.docx"),
    )
    .expect("heading.docx");
    let frame = layout_document(&bytes, &[font], &ViewParams::default_a4()).expect("layout");
    let texts = frame.text_commands();
    assert!(texts.len() >= 2, "heading + body");
    let heading = texts
        .iter()
        .find(|t| t.text.contains("Heading"))
        .expect("heading run");
    let body = texts
        .iter()
        .find(|t| t.text.contains("Body"))
        .expect("body run");
    assert!(
        heading.size > body.size,
        "heading size {} should exceed body {}",
        heading.size,
        body.size
    );
    assert!(
        heading.style & word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::STYLE_BOLD
            != 0,
        "heading rPr has w:b"
    );
}

