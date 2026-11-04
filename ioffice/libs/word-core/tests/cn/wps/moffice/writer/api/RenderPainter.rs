use word_core::cn::wps::moffice::writer::api::RenderPainter::{paint, PaintOp};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    CommandDocument, DrawRect, DrawText, RenderCommand,
};
use word_core::cn::wps::moffice::writer::rendercommand::Serialize::STAGE3_VERSION;

#[test]
fn fp1_text() {
    let doc = CommandDocument {
        version: STAGE3_VERSION,
        commands: vec![RenderCommand::Text(DrawText {
            page: 0,
            x: 0,
            y: 0,
            text: "Hello".into(),
            font_id: "f".into(),
            size: 240,
            color: 0,
            style: 0,
            xpos: Vec::new(),
        })],
    };
    let ops = paint(&doc, 1.0);
    assert!(ops.iter().any(|o| matches!(o, PaintOp::Text { text, .. } if text == "Hello")));
}

#[test]
fn fp2_rect() {
    let doc = CommandDocument {
        version: STAGE3_VERSION,
        commands: vec![RenderCommand::Rect(DrawRect {
            page: 0,
            x: 0,
            y: 0,
            w: 100,
            h: 100,
            color: 0,
            kind: "bg".into(),
        })],
    };
    let ops = paint(&doc, 1.0);
    assert!(ops.iter().any(|o| matches!(o, PaintOp::Rect { .. })));
}
