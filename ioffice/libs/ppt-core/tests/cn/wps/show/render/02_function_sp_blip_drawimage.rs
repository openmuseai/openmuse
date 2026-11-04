//! pp-18-02 type=1 + bytes → DrawImage。矩阵 FUNCTION.md T01–T03。

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

fn slide_of(sp: KmoShowShape) -> KmoSlide {
    KmoSlide {
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        sp_tree: KmoShapeTree {
            children: vec![sp],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn t01_type1_with_bytes_emits_drawimage() {
    setup_metrics();
    let sp = KmoShowShape {
        shape_type: 1,
        has_xfrm: true,
        transform2DX: 100,
        transform2DY: 200,
        transform2DW: 300,
        transform2DH: 400,
        media_id: 7,
        media_r_id: Some("rIdImg".into()),
        media_bytes: Some(vec![1, 2, 3]),
        ..KmoShowShape::default()
    };
    let cmds = render_slide(&slide_of(sp), 0);
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawImage {
            media_id: 7,
            bytes_b64: Some(_),
            ..
        }
    )));
    let img = cmds
        .iter()
        .find_map(|c| match c {
            RenderCommand::DrawImage { x, y, w, h, .. } => Some((*x, *y, *w, *h)),
            _ => None,
        })
        .unwrap();
    assert!(
        !cmds.iter().any(|c| matches!(
            c,
            RenderCommand::DrawRect { x, y, w, h, fill: Some(_), .. }
                if (*x, *y, *w, *h) == img
        )),
        "filled DrawRect must not cover the DrawImage box"
    );
}

#[test]
fn t02_no_media_no_drawimage() {
    setup_metrics();
    let sp = KmoShowShape {
        shape_type: 1,
        has_xfrm: true,
        transform2DX: 0,
        transform2DY: 0,
        transform2DW: 10,
        transform2DH: 10,
        ..KmoShowShape::default()
    };
    let cmds = render_slide(&slide_of(sp), 0);
    assert!(cmds
        .iter()
        .all(|c| !matches!(c, RenderCommand::DrawImage { .. })));
}

#[test]
fn t03_drawimage_rect_matches_shape() {
    setup_metrics();
    let sp = KmoShowShape {
        shape_type: 1,
        has_xfrm: true,
        transform2DX: 1_270_000,
        transform2DY: 2_540_000,
        transform2DW: 3_810_000,
        transform2DH: 1_270_000,
        media_bytes: Some(vec![9]),
        media_id: 1,
        ..KmoShowShape::default()
    };
    let cmds = render_slide(&slide_of(sp), 0);
    let img = cmds
        .iter()
        .find_map(|c| match c {
            RenderCommand::DrawImage { x, y, w, h, .. } => Some((*x, *y, *w, *h)),
            _ => None,
        })
        .expect("DrawImage");
    assert!(img.2 > 0.0 && img.3 > 0.0);
}
