//! pr-06-02 prst+框→Path。矩阵 FUNCTION.md §3.4 T01–T06。

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Once};

use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::render::rendercommand::{geometry_commands, RenderCommand};
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::render::SlideRender::LayerItem;
use ppt_core::cn::wps::show::util::layout_shape::{GeomObj, KmoShowShape, ShapePr};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util::cn::wps::graphics::RectF::RectF;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

static INIT: Once = Once::new();
fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

static NEXT_ID: AtomicI32 = AtomicI32::new(2000);

fn item_with_prst(prst: Option<i32>) -> LayerItem {
    setup_metrics();
    let mut sp = KmoShowShape::default();
    sp.id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    sp.sp_pr = ShapePr {
        geom_type: Some(0),
        geom_obj: GeomObj {
            prst,
            ..Default::default()
        },
        ..Default::default()
    };
    let typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&sp).into_shape()),
        None,
        Some(RectF::new_with(0.0, 0.0, 1270000.0, 1270000.0)),
    );
    LayerItem::Shape(typo)
}

#[test]
fn t01_line_is_path_two_points() {
    let mut item = item_with_prst(Some(20));
    let cmds = geometry_commands(&mut item);
    match &cmds[0] {
        RenderCommand::DrawPath { path, stroke, .. } => {
            assert_eq!(path.len(), 2);
            assert_eq!(*stroke, Some(0xFF000000));
        }
        other => panic!("expected DrawPath, got {other:?}"),
    }
}

#[test]
fn t02_ellipse_path_many_points() {
    let mut item = item_with_prst(Some(3));
    let cmds = geometry_commands(&mut item);
    match &cmds[0] {
        RenderCommand::DrawPath { path, .. } => assert!(path.len() >= 8),
        other => panic!("expected DrawPath, got {other:?}"),
    }
}

#[test]
fn t03_pentagon_at_least_five() {
    let mut item = item_with_prst(Some(56));
    let cmds = geometry_commands(&mut item);
    match &cmds[0] {
        RenderCommand::DrawPath { path, .. } => assert!(path.len() >= 5),
        other => panic!("expected DrawPath, got {other:?}"),
    }
}

#[test]
fn t04_rect_stays_drawrect() {
    let mut item = item_with_prst(Some(1));
    let cmds = geometry_commands(&mut item);
    assert!(matches!(cmds[0], RenderCommand::DrawRect { .. }));
}

#[test]
fn t05_missing_prst_aabb() {
    let mut item = item_with_prst(None);
    let cmds = geometry_commands(&mut item);
    assert!(matches!(cmds[0], RenderCommand::DrawRect { .. }));
}

#[test]
fn t06_zero_size_no_panic() {
    setup_metrics();
    let mut sp = KmoShowShape::default();
    sp.id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    sp.sp_pr.geom_obj.prst = Some(20);
    let typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&sp).into_shape()),
        None,
        Some(RectF::new_with(0.0, 0.0, 0.0, 0.0)),
    );
    let mut item = LayerItem::Shape(typo);
    let _ = geometry_commands(&mut item);
}
