use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    CommandDocument, DrawLine, DrawRect, DrawText, RenderCommand,
};
use word_core::cn::wps::moffice::writer::rendercommand::Serialize::{
    from_json, to_json, STAGE3_VERSION,
};

fn sample() -> CommandDocument {
    CommandDocument {
        version: STAGE3_VERSION,
        commands: vec![
            RenderCommand::Text(DrawText {
                page: 0,
                x: 1,
                y: 2,
                text: "A".into(),
                font_id: "f".into(),
                size: 240,
                color: 0,
                style: 0,
                xpos: Vec::new(),
            }),
            RenderCommand::Rect(DrawRect {
                page: 0,
                x: 0,
                y: 0,
                w: 10,
                h: 10,
                color: 1,
                kind: "bg".into(),
            }),
            RenderCommand::Line(DrawLine {
                page: 0,
                x1: 0,
                y1: 0,
                x2: 10,
                y2: 0,
                kind: "underline".into(),
                color: 0,
                width: 15,
            }),
        ],
    }
}

#[test]
fn s1_roundtrip() {
    let d = sample();
    let j = to_json(&d).unwrap();
    let back = from_json(&j).unwrap();
    assert_eq!(d, back);
}

#[test]
fn s2_stable_json() {
    let j = to_json(&sample()).unwrap();
    let j2 = to_json(&sample()).unwrap();
    assert_eq!(j, j2);
    assert!(j.contains("\"t\":\"text\""));
}

#[test]
fn s3_version() {
    assert_eq!(from_json(&to_json(&sample()).unwrap()).unwrap().version, 1);
}

#[test]
fn s4_order() {
    let back = from_json(&to_json(&sample()).unwrap()).unwrap();
    assert!(matches!(back.commands[0], RenderCommand::Text(_)));
    assert!(matches!(back.commands[1], RenderCommand::Rect(_)));
    assert!(matches!(back.commands[2], RenderCommand::Line(_)));
}
