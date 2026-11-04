//! 03_function_false_skips_master — slide.isShowMasterShape false 不 film master 也不 film layout。

use std::sync::{Arc, Once};

use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::render::SlideRender::{LayerItem, SlideRender};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

#[path = "../pptxr/visual_zip.rs"]
mod visual_zip;
use visual_zip::visual_pptx_show_master;

static INIT: Once = Once::new();
fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

const SLIDE_SP: &str = r#"<p:sp><p:spPr><a:xfrm><a:off x="5000" y="6000"/><a:ext cx="70" cy="80"/></a:xfrm></p:spPr></p:sp>"#;

fn shape_lefts(layers: &mut [LayerItem]) -> Vec<f32> {
    layers
        .iter_mut()
        .filter_map(|item| match item {
            LayerItem::Shape(typo) => Some(typo.getShapeRect().left),
            _ => None,
        })
        .collect()
}

#[test]
fn fs_m() {
    setup_metrics();
    let doc = open_pptx_document_bytes(&visual_pptx_show_master(
        SLIDE_SP,
        r#" showMasterSp="0""#,
        "",
    ))
    .unwrap();
    let slide = &doc.slides[0];
    assert!(!slide.isShowMasterShape());
    let mut layers = SlideRender::layer_order(Some(slide), 0, 0);
    let (master_x, slide_x) = {
        let u = UnitsConverter::render();
        (u.fromEmuX(1.0), u.fromEmuX(5000.0))
    };
    let lefts = shape_lefts(&mut layers);
    assert!(
        !lefts.contains(&master_x),
        "slide showMasterSp=false → 无 master 指令"
    );
    assert_eq!(lefts, vec![slide_x]);
}

#[test]
fn fs_l() {
    setup_metrics();
    let doc = open_pptx_document_bytes(&visual_pptx_show_master(
        SLIDE_SP,
        r#" showMasterSp="0""#,
        "",
    ))
    .unwrap();
    let slide = &doc.slides[0];
    let mut layers = SlideRender::layer_order(Some(slide), 0, 0);
    let (layout_x, slide_x) = {
        let u = UnitsConverter::render();
        (u.fromEmuX(100.0), u.fromEmuX(5000.0))
    };
    let lefts = shape_lefts(&mut layers);
    assert!(
        !lefts.contains(&layout_x),
        "Java: if (slide.isShowMasterShape()) 才 film layout 树"
    );
    assert_eq!(lefts, vec![slide_x]);
}
