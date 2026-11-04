// Test 02_function_run_style_command
use ppt_core::cn::wps::show::render::rendercommand::{apply_run_style, RenderCommand};

#[test]
fn t01_attach_style() {
    let c = RenderCommand::DrawText {
        text: "x".into(),
        x: 1.0,
        y: 2.0,
        font: None,
        size: None,
        color: None,
        bold: None,
        italic: None,
    };
    let o = apply_run_style(c, Some("Arial".into()), Some(18.0), Some(0xFF0000), Some(true), Some(false));
    match o {
        RenderCommand::DrawText {
            font,
            size,
            color,
            bold,
            italic,
            ..
        } => {
            assert_eq!(font.as_deref(), Some("Arial"));
            assert_eq!(size, Some(18.0));
            assert_eq!(color, Some(0xFF0000));
            assert_eq!(bold, Some(true));
            assert_eq!(italic, Some(false));
        }
        _ => panic!(),
    }
}

#[test]
fn t02_rect_passthrough() {
    let c = RenderCommand::DrawRect {
        x: 0.0,
        y: 0.0,
        w: 1.0,
        h: 1.0,
        fill: None,
        stroke: None,
    };
    assert_eq!(apply_run_style(c.clone(), Some("a".into()), None, None, None, None), c);
}
