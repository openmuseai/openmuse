use word_core::cn::wps::moffice::writer::api::Viewport::Viewport;
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    CommandDocument, DrawText, RenderCommand,
};
use word_core::cn::wps::moffice::writer::rendercommand::Serialize::STAGE3_VERSION;

fn hello() -> CommandDocument {
    CommandDocument {
        version: STAGE3_VERSION,
        commands: vec![RenderCommand::Text(DrawText {
            page: 0,
            x: 0,
            y: 0,
            text: "Hi".into(),
            font_id: "f".into(),
            size: 240,
            color: 0,
            style: 0,
            xpos: Vec::new(),
        })],
    }
}

#[test]
fn ia1_zoom_keeps_command_count() {
    let doc = hello();
    let mut vp = Viewport::default();
    let n1 = vp.paint_ops(&doc).len();
    vp.on_scale(2.0);
    let n2 = vp.paint_ops(&doc).len();
    assert_eq!(n1, n2);
    assert_eq!(n1, doc.commands.len());
    assert!((vp.zoom - 2.0).abs() < 1e-9);
}
