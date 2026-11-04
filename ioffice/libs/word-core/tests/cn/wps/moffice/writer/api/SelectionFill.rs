// function_id=03_function_sel_fill
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    LayoutFrame, PageMetrics, LAYOUT_FRAME_VERSION, PIPELINE_LAYOUT_V1,
};
use word_core::cn::wps::moffice::writer::api::SelectionFill::{
    overlay, KIND_SELECTION, SELECTION_COLOR,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawText, RenderCommand,
};

fn hello_frame() -> LayoutFrame {
    LayoutFrame {
        version: LAYOUT_FRAME_VERSION,
        pipeline: PIPELINE_LAYOUT_V1.into(),
        page: PageMetrics {
            page_count: 1,
            page_width_twip: 11906,
            page_height_twip: 16838,
            margin_left_twip: 1800,
            margin_right_twip: 1800,
            margin_top_twip: 1440,
            margin_bottom_twip: 1440,
            paper_size: "A4".into(),
        },
        commands: vec![RenderCommand::Text(DrawText {
            page: 0,
            x: 1800,
            y: 1632,
            text: "Hello".into(),
            font_id: "default".into(),
            size: 240,
            color: 0,
            style: 0,
            xpos: vec![1800, 1900, 2000, 2100, 2200],
        })],
    }
}

#[test]
fn sf1_kind_is_selection() {
    let cmds = overlay(&hello_frame(), 1, 3);
    let rect = cmds.iter().find_map(|c| match c {
        RenderCommand::Rect(r) => Some(r),
        _ => None,
    });
    let rect = rect.expect("rect");
    assert_eq!(rect.kind, KIND_SELECTION);
    assert_eq!(rect.x, 1900);
    assert_eq!(rect.w, 200);
}

#[test]
fn sf2_rect_before_text() {
    let cmds = overlay(&hello_frame(), 1, 3);
    assert!(matches!(cmds.first(), Some(RenderCommand::Rect(_))));
    assert!(cmds.iter().any(|c| matches!(c, RenderCommand::Text(_))));
}

#[test]
fn sf3_empty_sel_unchanged_len() {
    let frame = hello_frame();
    let cmds = overlay(&frame, 2, 2);
    assert_eq!(cmds.len(), frame.commands.len());
    assert!(cmds.iter().all(|c| !matches!(c, RenderCommand::Rect(_))));
}

#[test]
fn sf4_color_constant() {
    let cmds = overlay(&hello_frame(), 0, 1);
    match &cmds[0] {
        RenderCommand::Rect(r) => assert_eq!(r.color, SELECTION_COLOR),
        _ => panic!("expected rect"),
    }
}
