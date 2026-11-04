// Test 02_function_fill_line_command
use ppt_core::cn::wps::show::render::rendercommand::{apply_fill_line, RenderCommand};

#[test]
fn t01_fill_on_rect() {
    let c = RenderCommand::DrawRect {
        x: 0.0,
        y: 0.0,
        w: 1.0,
        h: 1.0,
        fill: None,
        stroke: None,
    };
    let o = apply_fill_line(c, Some(0xFF0000FF), Some(0xFF00FF00));
    match o {
        RenderCommand::DrawRect { fill, stroke, .. } => {
            assert_eq!(fill, Some(0xFF0000FF));
            assert_eq!(stroke, Some(0xFF00FF00));
        }
        _ => panic!(),
    }
}

#[test]
fn t02_fill_on_path() {
    let c = RenderCommand::DrawPath {
        path: vec![(0.0, 0.0)],
        fill: None,
        stroke: None,
    };
    let o = apply_fill_line(c, Some(1), None);
    match o {
        RenderCommand::DrawPath { fill, stroke, .. } => {
            assert_eq!(fill, Some(1));
            assert_eq!(stroke, None);
        }
        _ => panic!(),
    }
}

#[test]
fn t03_none_keeps_none() {
    let c = RenderCommand::DrawRect {
        x: 0.0,
        y: 0.0,
        w: 1.0,
        h: 1.0,
        fill: Some(9),
        stroke: Some(8),
    };
    let o = apply_fill_line(c, None, None);
    match o {
        RenderCommand::DrawRect { fill, stroke, .. } => {
            assert!(fill.is_none() && stroke.is_none());
        }
        _ => panic!(),
    }
}

#[test]
fn t04_text_passthrough() {
    let c = RenderCommand::DrawText {
        text: "a".into(),
        x: 0.0,
        y: 0.0,
        font: None,
        size: None,
        color: None,
        bold: None,
        italic: None,
    };
    let o = apply_fill_line(c.clone(), Some(1), Some(2));
    assert_eq!(o, c);
}
