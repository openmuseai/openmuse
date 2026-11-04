//! pr-06-03 文本框不发射空 AABB。矩阵 FUNCTION.md §3.4 T01–T04。

use std::sync::{Arc, Once};

use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::render::rendercommand::geometry_commands;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::render::SlideRender::LayerItem;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShowShape, ShapePr};
use ppt_core::cn::wps::show::util::layout_textbox::KmoTextOperator;
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

fn text_shape(has_fill: bool, media: bool) -> LayerItem {
    setup_metrics();
    let mut sp = KmoShowShape {
        id: if media { 401 } else if has_fill { 402 } else { 403 },
        is_text_box: true,
        has_text_body: true,
        text_op: Some(KmoTextOperator::default()),
        ..KmoShowShape::default()
    };
    sp.sp_pr = ShapePr {
        has_fill,
        fill: if has_fill { 0xFF112233u32 as i32 } else { 0 },
        ..Default::default()
    };
    if media {
        sp.media_bytes = Some(vec![0, 1, 2]);
        sp.media_r_id = Some("rId3".into());
    }
    let typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&sp).into_shape()),
        None,
        Some(RectF::new_with(0.0, 0.0, 100.0, 100.0)),
    );
    LayerItem::Shape(typo)
}

#[test]
fn t01_text_no_fill_no_geom() {
    let mut item = text_shape(false, false);
    assert!(geometry_commands(&mut item).is_empty());
}

#[test]
fn t02_text_with_fill_keeps_rect() {
    let mut item = text_shape(true, false);
    assert!(!geometry_commands(&mut item).is_empty());
}

#[test]
fn t03_line_without_text_not_skipped() {
    setup_metrics();
    let mut sp = KmoShowShape {
        id: 404,
        is_text_box: false,
        has_text_body: false,
        ..KmoShowShape::default()
    };
    sp.sp_pr.geom_obj.prst = Some(20);
    let typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&sp).into_shape()),
        None,
        Some(RectF::new_with(0.0, 0.0, 100.0, 10.0)),
    );
    let mut item = LayerItem::Shape(typo);
    assert!(!geometry_commands(&mut item).is_empty());
}

#[test]
fn t04_media_not_skipped_as_textbox() {
    let mut item = text_shape(false, true);
    assert!(
        !geometry_commands(&mut item).is_empty(),
        "media shapes must not take the text-wireframe skip"
    );
}
