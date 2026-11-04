// function_id=02_function_sel_rects
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    LayoutFrame, PageMetrics, LAYOUT_FRAME_VERSION, PIPELINE_LAYOUT_V1,
};
use word_core::cn::wps::moffice::writer::api::SelectionRects::rects_for_range;
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
fn sx1_full_run_width_includes_last_glyph() {
    let r = rects_for_range(&hello_frame(), 0, 5);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].x, 1800);
    assert_eq!(r[0].w, 500);
}

#[test]
fn sx2_mid_slice() {
    let r = rects_for_range(&hello_frame(), 1, 3);
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].x, 1900);
    assert_eq!(r[0].w, 200);
}

#[test]
fn sx3_empty_interval_no_rects() {
    assert!(rects_for_range(&hello_frame(), 2, 2).is_empty());
}

#[test]
fn sx4_miss_is_empty() {
    assert!(rects_for_range(&hello_frame(), 10, 12).is_empty());
}

#[test]
fn sx5_line_top_is_baseline_minus_size() {
    let r = rects_for_range(&hello_frame(), 1, 3);
    assert_eq!(r[0].y, 1392);
    assert_eq!(r[0].h, 240);
}
