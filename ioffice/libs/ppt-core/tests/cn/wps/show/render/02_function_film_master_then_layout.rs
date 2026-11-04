//! 02_function_film_master_then_layout — Java SlideRender.drawShapesOnLayout:104-110
//! 全 true → master → layout → slide；layout 关 master → 无 master 层。

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
fn fm_ord() {
    setup_metrics();
    let doc = open_pptx_document_bytes(&visual_pptx_show_master(SLIDE_SP, "", "")).unwrap();
    let slide = &doc.slides[0];
    assert!(slide.isShowMasterShape(), "01 缺省 true");
    assert!(
        slide.layout().unwrap().isShowMasterShape(),
        "layout 缺省 true"
    );
    let mut layers = SlideRender::layer_order(Some(slide), 0, 0);
    let expected = {
        let u = UnitsConverter::render();
        vec![u.fromEmuX(1.0), u.fromEmuX(100.0), u.fromEmuX(5000.0)]
    };
    // master off.x=1, layout off.x=100, slide off.x=5000
    assert_eq!(
        shape_lefts(&mut layers),
        expected,
        "Java drawShapesOnLayout: master then layout; draw() then slide tree"
    );
}

#[test]
fn fm_lay() {
    setup_metrics();
    let doc = open_pptx_document_bytes(&visual_pptx_show_master(
        SLIDE_SP,
        "",
        r#" showMasterSp="0""#,
    ))
    .unwrap();
    let slide = &doc.slides[0];
    assert!(slide.isShowMasterShape());
    assert!(!slide.layout().unwrap().isShowMasterShape());
    let mut layers = SlideRender::layer_order(Some(slide), 0, 0);
    let expected = {
        let u = UnitsConverter::render();
        vec![u.fromEmuX(100.0), u.fromEmuX(5000.0)]
    };
    assert_eq!(
        shape_lefts(&mut layers),
        expected,
        "layout.isShowMasterShape false → 不 film master"
    );
}
