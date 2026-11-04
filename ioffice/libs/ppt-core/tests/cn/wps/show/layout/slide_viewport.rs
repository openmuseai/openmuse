// Test 02_function_slide_viewport — EMU slideSize → render 档内容矩形
use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::layout::slide_viewport::{
    slide_viewport, ViewportError, DEFAULT_SLDSIZE_CX, DEFAULT_SLDSIZE_CY,
};
use std::sync::{Arc, Once};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

fn install_96dpi_metrics() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let metrics = KDisplayMetrics::new_with_params(
            1920, 1080, 1.0, 160, 1.0, 96.0, 96.0,
        );
        Platform::setDisplayMetrics(Arc::new(metrics));
    });
}

#[test]
fn t01_default_sldsz_is_960x540() {
    install_96dpi_metrics();
    let r = slide_viewport(None).expect("default sldSz");
    assert_eq!(r.left, 0.0);
    assert_eq!(r.top, 0.0);
    assert_eq!(r.width, 960.0);
    assert_eq!(r.height, 540.0);
    assert_eq!(DEFAULT_SLDSIZE_CX, 12_192_000);
    assert_eq!(DEFAULT_SLDSIZE_CY, 6_858_000);
}

#[test]
fn t02_explicit_sldsz_from_emu() {
    install_96dpi_metrics();
    let r = slide_viewport(Some((12_192_000, 6_858_000))).expect("explicit");
    assert_eq!(r.width, 960.0);
    assert_eq!(r.height, 540.0);
    let wide = slide_viewport(Some((13_716_000, 7_714_000))).expect("16:9-ish");
    let render = UnitsConverter::render();
    assert_eq!(wide.width, render.fromEmuX(13_716_000.0));
    assert_eq!(wide.height, render.fromEmuY(7_714_000.0));
}

#[test]
fn t03_zoom_does_not_change_viewport() {
    install_96dpi_metrics();
    let before = slide_viewport(None).unwrap();
    {
        let mut render = UnitsConverter::render();
        render.setZoom(2.0);
        assert!((render.getZoom() - 2.0).abs() < 1e-5);
    }
    let after = slide_viewport(None).unwrap();
    assert_eq!(before, after);
}

#[test]
fn t04_none_uses_default() {
    install_96dpi_metrics();
    let a = slide_viewport(None).unwrap();
    let b = slide_viewport(Some((DEFAULT_SLDSIZE_CX, DEFAULT_SLDSIZE_CY))).unwrap();
    assert_eq!(a, b);
}

#[test]
fn t05_non_positive_size_rejected() {
    install_96dpi_metrics();
    assert_eq!(
        slide_viewport(Some((0, 6_858_000))),
        Err(ViewportError::NonPositiveSize)
    );
    assert_eq!(
        slide_viewport(Some((12_192_000, -1))),
        Err(ViewportError::NonPositiveSize)
    );
}
