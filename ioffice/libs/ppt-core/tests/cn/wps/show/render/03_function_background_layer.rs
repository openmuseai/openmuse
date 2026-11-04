//! pp-17-03 Background 层。矩阵 FUNCTION.md T01–T03。

use std::sync::{Arc, Once};

use ppt_core::cn::wps::show::render::mvp_pipeline::render_slide;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use ppt_core::cn::wps::show::render::SlideRender::{LayerItem, SlideRender};
use ppt_core::cn::wps::show::util::layout_shape::KmoSlide;
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

#[test]
fn t01_has_background_emits_layer() {
    setup_metrics();
    let slide = KmoSlide {
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        has_background: true,
        bg_fill: Some(0xFF112233),
        ..Default::default()
    };
    let layers = SlideRender::layer_order(Some(&slide), 0, 0);
    assert!(layers.iter().any(|i| matches!(i, LayerItem::Background { .. })));
}

#[test]
fn t02_false_skips_background_layer() {
    setup_metrics();
    let slide = KmoSlide {
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        has_background: false,
        ..Default::default()
    };
    let layers = SlideRender::layer_order(Some(&slide), 0, 0);
    assert!(!layers.iter().any(|i| matches!(i, LayerItem::Background { .. })));
}

#[test]
fn t03_background_fill_is_page_argb() {
    setup_metrics();
    let slide = KmoSlide {
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        has_background: true,
        bg_fill: Some(0xFFAABBCC),
        ..Default::default()
    };
    let cmds = render_slide(&slide, 0);
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawRect {
            fill: Some(0xFFAABBCC),
            ..
        }
    )));
}
