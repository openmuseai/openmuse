// Test 01_function_command_json
use ppt_core::cn::wps::show::render::rendercommand::{from_json, to_json, RenderCommand};

#[test]
fn t01_roundtrip_rect() {
    let cmds = vec![RenderCommand::DrawRect {
        x: 1.0,
        y: 2.0,
        w: 3.0,
        h: 4.0,
        fill: Some(0xFFFFFFFF),
        stroke: None,
    }];
    let s = to_json(&cmds).unwrap();
    assert!(s.contains("DrawRect"));
    let back = from_json(&s).unwrap();
    assert_eq!(back, cmds);
}

#[test]
fn t02_roundtrip_text() {
    let cmds = vec![RenderCommand::DrawText {
        text: "hello".into(),
        x: 0.5,
        y: 1.5,
        font: Some("Arial".into()),
        size: Some(12.0),
        color: Some(1),
        bold: Some(false),
        italic: Some(true),
    }];
    let back = from_json(&to_json(&cmds).unwrap()).unwrap();
    assert_eq!(back, cmds);
}

#[test]
fn t03_invalid_json() {
    assert!(from_json("not-json").is_err());
}

#[test]
fn t04_unknown_variant() {
    assert!(from_json("[{\"type\":\"DrawFoo\"}]").is_err());
}
