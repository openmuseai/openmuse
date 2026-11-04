//! pr-07-01 clip 到 slide。矩阵 FUNCTION.md T01–T03。

use std::sync::{Arc, Once};

use ppt_core::cn::wps::show::render::mvp_pipeline::render_slide;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShapeTree, KmoShowShape, KmoSlide};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

static INIT: Once = Once::new();
fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn slide_with(sp: KmoShowShape, w: i32, h: i32) -> KmoSlide {
    KmoSlide {
        slide_width: w,
        slide_height: h,
        sp_tree: KmoShapeTree {
            children: vec![sp],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn t01_negative_x_does_not_paint_host_chrome() {
    setup_metrics();
    let sp = KmoShowShape {
        has_xfrm: true,
        transform2DX: -5_000_000,
        transform2DY: 100_000,
        transform2DW: 1_000_000,
        transform2DH: 1_000_000,
        ..KmoShowShape::default()
    };
    let cmds = render_slide(&slide_with(sp, 12_192_000, 6_858_000), 0);
    for c in &cmds {
        match c {
            RenderCommand::DrawText { x, .. } => assert!(*x >= 0.0),
            RenderCommand::DrawRect { x, .. } => assert!(*x >= 0.0),
            RenderCommand::DrawPath { path, .. } => {
                for (px, _) in path {
                    assert!(*px >= 0.0);
                }
            }
            RenderCommand::DrawImage { x, .. } => assert!(*x >= 0.0),
        }
    }
}

#[test]
fn t02_inside_slide_unchanged() {
    setup_metrics();
    let sp = KmoShowShape {
        has_xfrm: true,
        transform2DX: 1_270_000,
        transform2DY: 1_270_000,
        transform2DW: 1_270_000,
        transform2DH: 1_270_000,
        ..KmoShowShape::default()
    };
    let cmds = render_slide(&slide_with(sp, 12_192_000, 6_858_000), 0);
    let rects: Vec<_> = cmds
        .iter()
        .filter_map(|c| match c {
            RenderCommand::DrawRect { x, y, w, h, fill: None, .. } if *x > 1.0 => {
                Some((*x, *y, *w, *h))
            }
            _ => None,
        })
        .collect();
    assert!(!rects.is_empty());
}

#[test]
fn t03_zero_slide_size_no_panic() {
    setup_metrics();
    let sp = KmoShowShape {
        has_xfrm: true,
        transform2DX: 10,
        transform2DY: 10,
        transform2DW: 10,
        transform2DH: 10,
        ..KmoShowShape::default()
    };
    let _ = render_slide(&slide_with(sp, 0, 0), 0);
}
