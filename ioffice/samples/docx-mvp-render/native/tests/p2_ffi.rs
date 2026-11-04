use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document as core_layout, ViewParams as CoreView,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand as CoreCmd;
use word_core_ffi::api::{
    dump_json, layout_document, FontBlob, RenderCommandWire, ViewParams, WordSession,
};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../libs/word-core/tests/fixtures")
}

fn hello_docx() -> Vec<u8> {
    std::fs::read(fixtures().join("01_function_uniffi_export/plain-one-para.docx")).unwrap()
}

fn mvp_font() -> FontBlob {
    FontBlob {
        id: "default".into(),
        bytes: std::fs::read(fixtures().join("p1_layout_document/MvpLatin.ttf")).unwrap(),
    }
}

fn testfont() -> FontBlob {
    FontBlob {
        id: "test".into(),
        bytes: std::fs::read(fixtures().join("01_function_font_stack/TestFont.ttf")).unwrap(),
    }
}

fn a4() -> ViewParams {
    ViewParams::default_a4()
}

/// F1 — FFI frame matches core layout_document (twip, no px).
#[test]
fn f1_ffi_matches_core_layout() {
    let bytes = hello_docx();
    let core = core_layout(&bytes, &[], &CoreView::default_a4()).expect("core");
    let ffi = layout_document(bytes, vec![], a4()).expect("ffi");
    assert_eq!(ffi.pipeline, core.pipeline);
    assert_eq!(ffi.version, core.version);
    assert_eq!(ffi.page.page_count, core.page.page_count);
    assert_eq!(ffi.commands.len(), core.commands.len());
    match &ffi.commands[0] {
        RenderCommandWire::Text { x, y, text, size, xpos, .. } => {
            assert_eq!(*x, 1800);
            assert_eq!(text, "Hello");
            assert_eq!(*size, 240, "size stays twip");
            assert_eq!(xpos.len(), 5);
            assert_eq!(xpos[0], *x);
            let CoreCmd::Text(ct) = &core.commands[0] else {
                panic!("core not text");
            };
            assert_eq!(*y, ct.y);
            assert_eq!(xpos, &ct.xpos);
        }
        other => panic!("{other:?}"),
    }
}

/// F3 — require_injected_fonts + empty fonts → error (Web). Missing glyph still errors.
#[test]
fn f3_missing_or_required_fonts_err() {
    let mut view = a4();
    view.require_injected_fonts = true;
    let err = layout_document(hello_docx(), vec![], view).unwrap_err();
    assert!(err.contains("require_injected_fonts"), "{err}");

    let err2 = layout_document(
        std::fs::read(fixtures().join("p1_layout_document/hello-world.docx")).unwrap(),
        vec![testfont()],
        a4(),
    )
    .unwrap_err();
    assert!(
        err2.contains("glyph") || err2.contains("U+") || err2.contains("Measure"),
        "{err2}"
    );
}

/// F5 — dump_json roundtrips into Stage3-shaped envelope (golden tool).
#[test]
fn f5_dump_json_stage3_shape() {
    let frame = layout_document(hello_docx(), vec![mvp_font()], a4()).expect("ffi");
    let json = dump_json(frame).expect("json");
    assert!(json.contains("\"t\": \"text\"") || json.contains("\"t\":\"text\""));
    assert!(json.contains("word-core-layout-v1"));
    assert!(json.contains("1800"));
}

/// Default A4 content box is still twip (not device px).
#[test]
fn f_view_stays_twip() {
    let v = a4();
    assert_eq!(v.page_width_twip, 11906);
    assert!(v.content_width_twip.is_none());
}

/// Heap session: insert at 0 then layout paints Z + Hello (wr-20 over FRB types).
#[test]
fn f_session_insert_relayout() {
    let s = WordSession::open(hello_docx(), vec![mvp_font()], a4()).expect("open");
    assert!(s.main_length().expect("len") > 0);
    let before = s.layout().expect("layout");
    let hello = before.commands.iter().any(|c| match c {
        RenderCommandWire::Text { text, .. } => text.contains("Hello"),
        _ => false,
    });
    assert!(hello);
    s.insert_text(0, "Z".into()).expect("insert");
    let after = s.layout().expect("relayout");
    let joined: String = after
        .commands
        .iter()
        .filter_map(|c| match c {
            RenderCommandWire::Text { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert!(joined.contains('Z'), "{joined}");
    assert!(joined.contains("Hello"), "{joined}");
    let hit = s.caret_at(0, 1800, 1632).expect("caret").expect("hit");
    assert_eq!(hit.cp, 0);
    let sel = s.set_selection(0, 2).expect("sel");
    assert!(sel.commands.iter().any(|c| match c {
        RenderCommandWire::Rect { kind, .. } => kind == "selection",
        _ => false,
    }));
}
