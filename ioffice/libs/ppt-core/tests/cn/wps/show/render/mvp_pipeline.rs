// MVP shortcut: fixture slide → RenderCommand[]
use ppt_core::cn::wps::show::render::mvp_pipeline::{
    render_hello_envelope_json, render_hello_slide,
};
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;

#[test]
fn t01_hello_has_white_fill_and_text() {
    let cmds = render_hello_slide();
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawRect {
            fill: Some(0xFFFFFFFF),
            ..
        }
    )));
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawRect {
            fill: Some(0xFF4472C4),
            ..
        }
    )));
    assert!(cmds.iter().any(|c| match c {
        RenderCommand::DrawText { text, .. } => text.contains("Hello"),
        _ => false,
    }));
}

#[test]
fn t02_envelope_json_roundtrip_shape() {
    let s = render_hello_envelope_json().expect("json");
    assert!(s.contains("ppt-core-render_slide-v0-mvp"));
    assert!(s.contains("DrawRect"));
    assert!(s.contains("Hello"));
}
