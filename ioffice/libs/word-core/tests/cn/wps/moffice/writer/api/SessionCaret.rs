// function_id=03_function_caret_hit
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_caret_hit

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    LayoutFrame, PageMetrics, LAYOUT_FRAME_VERSION, PIPELINE_LAYOUT_V1,
};
use word_core::cn::wps::moffice::writer::api::SessionCaret::cp_for_point;
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

/// CA1 — click at first glyph origin → cp 0 (Hello[0]).
#[test]
fn ca1_left_edge_is_first_cp() {
    let cp = cp_for_point(&hello_frame(), 0, 1800, 1632);
    assert_eq!(cp, Some(0));
}

/// CA2 — click past last glyph → cp after last visible char.
#[test]
fn ca2_past_last_glyph_clamps_end() {
    let cp = cp_for_point(&hello_frame(), 0, 10_000, 1632);
    assert_eq!(cp, Some(5));
}

/// CA3 — y far from the run baseline band → None (not a second hit-test).
#[test]
fn ca3_y_miss_is_none() {
    let cp = cp_for_point(&hello_frame(), 0, 1800, 0);
    assert_eq!(cp, None);
}

/// CA4 — wrong page index → None.
#[test]
fn ca4_missing_page_is_none() {
    let cp = cp_for_point(&hello_frame(), 1, 1800, 1632);
    assert_eq!(cp, None);
}

/// CA5 — empty commands → None.
#[test]
fn ca5_empty_frame_is_none() {
    let mut frame = hello_frame();
    frame.commands.clear();
    assert_eq!(cp_for_point(&frame, 0, 1800, 1632), None);
}
