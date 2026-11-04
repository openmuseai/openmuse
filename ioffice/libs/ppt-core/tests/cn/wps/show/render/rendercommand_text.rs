// Test 01_function_text_line_command
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::layout::typo::GreedyWrap::ShowTypoLine;
use ppt_core::cn::wps::show::render::rendercommand::text_line_commands;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::util::DrawFlags::DrawFlags;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShowShape, Placeholder};

fn app_default() -> ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape {
    ConvertKmo::from_layout(&KmoShowShape::default()).into_shape()
}

fn line(text: &str, x: f32, y: f32) -> ShowTypoLine {
    ShowTypoLine {
        text: text.into(),
        startCp: 0,
        endCp: text.len() as i32,
        x,
        y,
        w: 10.0,
        h: 12.0,
    }
}

#[test]
fn t01_origin_plus_line() {
    let typo = ShowTypoShape::typo_shape_snapshot(Some(app_default()), None, None);
    let cmds = text_line_commands(&typo, &[line("Hi", 1.0, 2.0)], 0, 10.0, 20.0);
    match &cmds[0] {
        ppt_core::cn::wps::show::render::rendercommand::RenderCommand::DrawText { text, x, y, .. } => {
            assert_eq!(text, "Hi");
            assert_eq!((*x, *y), (11.0, 22.0));
        }
        _ => panic!(),
    }
}

#[test]
fn t02_empty_lines() {
    let typo = ShowTypoShape::typo_shape_snapshot(Some(app_default()), None, None);
    assert!(text_line_commands(&typo, &[], 0, 0.0, 0.0).is_empty());
}

#[test]
fn t03_placeholder_text_flag() {
    let typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&KmoShowShape {
            id: 701,
            is_placeholder: true,
            ph: Placeholder {
                has_type: true,
                type_: 1,
                has_idx: false,
                idx: 0,
            },
            ..KmoShowShape::default()
        }).into_shape()),
        None,
        None,
    );
    let cmds = text_line_commands(
        &typo,
        &[line("ph", 0.0, 0.0)],
        DrawFlags::NOT_DRAW_PLACEHOLDER_TEXT,
        0.0,
        0.0,
    );
    assert!(cmds.is_empty());
}

#[test]
fn t04_two_lines() {
    let typo = ShowTypoShape::typo_shape_snapshot(Some(app_default()), None, None);
    let cmds = text_line_commands(&typo, &[line("a", 0.0, 0.0), line("b", 0.0, 12.0)], 0, 0.0, 0.0);
    assert_eq!(cmds.len(), 2);
}
