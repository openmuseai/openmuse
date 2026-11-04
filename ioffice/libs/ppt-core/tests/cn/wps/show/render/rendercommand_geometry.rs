// Test 01_function_shape_geometry_command
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::render::rendercommand::{geometry_commands, RenderCommand};
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::render::SlideRender::LayerItem;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;
use util::cn::wps::graphics::RectF::RectF;

#[test]
fn t01_white_draw_rect() {
    let mut item = LayerItem::WhiteBackground { w: 10.0, h: 20.0 };
    let cmds = geometry_commands(&mut item);
    assert_eq!(
        cmds[0],
        RenderCommand::DrawRect {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 20.0,
            fill: Some(0xFFFFFFFF),
            stroke: None,
        }
    );
}

#[test]
fn t02_shape_rect() {
    let mut typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&KmoShowShape::default()).into_shape()),
        None,
        Some(RectF::new_with(0.0, 0.0, 12700.0, 25400.0)),
    );
    let mut item = LayerItem::Shape(typo);
    let cmds = geometry_commands(&mut item);
    match &cmds[0] {
        RenderCommand::DrawRect { x, y, w, h, .. } => {
            assert_eq!((*x, *y), (0.0, 0.0));
            assert!(*w > 0.0 && *h > 0.0);
        }
        _ => panic!("rect"),
    }
}

#[test]
fn t03_preset_path() {
    let mut typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&KmoShowShape {
            id: 301,
            graphics_path: Some(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]),
            ..KmoShowShape::default()
        }).into_shape()),
        None,
        None,
    );
    let mut item = LayerItem::Shape(typo);
    let cmds = geometry_commands(&mut item);
    match &cmds[0] {
        RenderCommand::DrawPath { path, .. } => assert_eq!(path.len(), 3),
        _ => panic!("path"),
    }
}

#[test]
fn t04_background_full() {
    let mut item = LayerItem::Background { w: 96.0, h: 54.0, fill: None };
    let cmds = geometry_commands(&mut item);
    match &cmds[0] {
        RenderCommand::DrawRect { w, h, .. } => assert_eq!((*w, *h), (96.0, 54.0)),
        _ => panic!("bg"),
    }
}

#[test]
fn t05_none_shape_empty() {
    let mut typo = ShowTypoShape::typo_shape_snapshot(None, None, None);
    let mut item = LayerItem::Shape(typo);
    assert!(geometry_commands(&mut item).is_empty());
    let mut typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&KmoShowShape {
            id: 302,
            graphics_path: Some(vec![]),
            ..KmoShowShape::default()
        }).into_shape()),
        None,
        None,
    );
    let mut item = LayerItem::Shape(typo);
    assert!(geometry_commands(&mut item).is_empty());
}
