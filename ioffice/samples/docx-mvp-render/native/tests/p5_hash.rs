//! P5: same canonical Stage3 hash from word-core and word_core_ffi (five-end L2).

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document as core_layout, ViewParams as CoreView,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawImage, DrawLine, DrawRect, DrawText, RenderCommand,
};
use word_core::cn::wps::moffice::writer::rendercommand::Stage3Normalize::{
    canonical_hash, normalize_commands, normalize_frame,
};
use word_core_ffi::api::{layout_document, FontBlob, RenderCommandWire, ViewParams};

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

fn wire_to_core(cmd: &RenderCommandWire) -> RenderCommand {
    match cmd {
        RenderCommandWire::Text {
            page,
            x,
            y,
            text,
            font_id,
            size,
            color,
            style,
            xpos,
        } => RenderCommand::Text(DrawText {
            page: *page,
            x: *x,
            y: *y,
            text: text.clone(),
            font_id: font_id.clone(),
            size: *size,
            color: *color,
            style: *style,
            xpos: xpos.clone(),
        }),
        RenderCommandWire::Rect {
            page,
            x,
            y,
            w,
            h,
            color,
            kind,
        } => RenderCommand::Rect(DrawRect {
            page: *page,
            x: *x,
            y: *y,
            w: *w,
            h: *h,
            color: *color,
            kind: kind.clone(),
        }),
        RenderCommandWire::Line {
            page,
            x1,
            y1,
            x2,
            y2,
            kind,
            color,
            width,
        } => RenderCommand::Line(DrawLine {
            page: *page,
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            kind: kind.clone(),
            color: *color,
            width: *width,
        }),
        RenderCommandWire::Image {
            page, x, y, w, h, shape_id, media_id, media_path,
            rotation, crop_left, crop_top, crop_right, crop_bottom, ..
        } => RenderCommand::Image(DrawImage {
            page: *page,
            x: *x,
            y: *y,
            w: *w,
            h: *h,
            shape_id: *shape_id,
            media_id: *media_id,
            media_path: media_path.clone(),
            rotation: *rotation,
            crop_left: *crop_left,
            crop_top: *crop_top,
            crop_right: *crop_right,
            crop_bottom: *crop_bottom,
        }),
    }
}

#[test]
fn p5_ffi_canonical_hash_matches_core() {
    let bytes = hello_docx();
    let font = mvp_font();
    let core = core_layout(
        &bytes,
        &[word_core::cn::wps::moffice::writer::api::LayoutDocument::FontBlob {
            id: font.id.clone(),
            bytes: font.bytes.clone(),
        }],
        &CoreView::default_a4(),
    )
    .unwrap();
    let ffi = layout_document(bytes, vec![font], ViewParams::default_a4()).unwrap();
    let core_cmds: Vec<RenderCommand> = ffi.commands.iter().map(wire_to_core).collect();
    let n_core = normalize_frame(&core);
    let n_ffi = normalize_commands(ffi.version, &core_cmds);
    assert_eq!(
        canonical_hash(&n_core).unwrap(),
        canonical_hash(&n_ffi).unwrap()
    );
    let frozen = std::fs::read_to_string(fixtures().join("p4_stage3/hello.hash"))
        .unwrap()
        .trim()
        .to_string();
    assert_eq!(canonical_hash(&n_core).unwrap(), frozen);
}

#[test]
fn p5_require_injected_fonts_is_web_gate() {
    let mut v = ViewParams::default_a4();
    v.require_injected_fonts = true;
    let err = layout_document(hello_docx(), vec![], v).unwrap_err();
    assert!(err.contains("require_injected_fonts"), "{err}");
}
