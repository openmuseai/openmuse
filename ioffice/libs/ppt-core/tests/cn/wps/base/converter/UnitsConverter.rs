// Test 01_function_emu_convert — EMU↔layout/render 单位换算（pl-01-units-view）
// 冻结矩阵见 FUNCTION.md §3.2（T01–T11）。期望值由 Java UnitsConverter/UnitConverter
// 公式推导（koffice base/converter），非手工编造：
//   POINT 档 = EMU/12700；PIXEL 档 = EMU*xdpi/914400；EMU 档 identity；TWIP 档 = EMU/635。
// 设备密度经 Platform::setDisplayMetrics 固定（density=1.0、xdpi=ydpi=96、scaledDensity=1.0），
// 使 dip/zoom 语义确定。每个测试先 setup_metrics() 再触达转换器单例（Once 保证先行）。
use std::sync::{Arc, Once};

use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
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

fn assert_close(actual: f32, expected: f32, eps: f32) {
    assert!(
        (actual - expected).abs() <= eps,
        "expected {expected} ± {eps}, got {actual}"
    );
}

#[test]
fn t01_emu_to_point() {
    setup_metrics();
    // Java: render().convertX((byte)2, 12192000F, (byte)0) == 12192000/12700 == 960.0
    let v = UnitsConverter::render().convertX(2, 12192000.0, 0);
    assert_eq!(v, 960.0);
}

#[test]
fn t02_point_to_emu() {
    setup_metrics();
    // Java: convertX((byte)0, 72F, (byte)2) == 72*12700 == 914400.0
    let v = UnitsConverter::render().convertX(0, 72.0, 2);
    assert_eq!(v, 914400.0);
}

#[test]
fn t03_twip_to_emu() {
    setup_metrics();
    // Java: convertX((byte)0, 20F, (byte)3) == 20*635 == 12700.0
    let v = UnitsConverter::render().convertX(0, 20.0, 3);
    assert_eq!(v, 12700.0);
}

#[test]
fn t04_pixel_to_emu() {
    setup_metrics();
    // Java: xdpi=96 → 1px == 914400/96 == 9525.0 EMU
    let v = UnitsConverter::render().convertX(0, 1.0, 1);
    assert_eq!(v, 9525.0);
}

#[test]
#[should_panic]
fn t05_illegal_src_unit_4() {
    setup_metrics();
    // Java: switch default → IllegalArgumentException
    let _ = UnitsConverter::render().convertX(2, 1.0, 4);
}

#[test]
#[should_panic]
fn t05_illegal_src_unit_minus1() {
    setup_metrics();
    let _ = UnitsConverter::render().convertX(2, 1.0, -1);
}

#[test]
fn t06_from_emu_four_singletons() {
    setup_metrics();
    // render/layout=POINT（÷12700），doc=EMU（identity），pixel=xdpi/914400 比例
    assert_close(UnitsConverter::render().fromEmuX(12700.0), 1.0, 1e-6);
    assert_close(UnitsConverter::layout().fromEmuX(12700.0), 1.0, 1e-6);
    assert_close(UnitsConverter::doc().fromEmuX(12700.0), 12700.0, 1e-3);
    assert_close(UnitsConverter::pixel().fromEmuX(12700.0), 12700.0 * 96.0 / 914400.0, 1e-3);
    // Y 轴同公式
    assert_close(UnitsConverter::render().fromEmuY(12700.0), 1.0, 1e-6);
}

#[test]
fn t07_dip_conversions() {
    setup_metrics();
    // density=1.0 → fromDipX = fromPixelX(dip) = dip/96*72（POINT 档）
    assert_close(UnitsConverter::render().fromDipX(1.0), 1.0 / 96.0 * 72.0, 1e-4);
    // fromDipIntX = Math.round(...)：1.6dip → 1.6/96*72 == 1.2 → 1
    assert_eq!(UnitsConverter::render().fromDipIntX(1.6), 1);
}

#[test]
fn t08_reverse_to_emu() {
    setup_metrics();
    // render 档（POINT）toEmuX(960) == 960*12700 == 12192000
    let v = UnitsConverter::render().toEmuX(960.0);
    assert_eq!(v, 12192000.0);
}

#[test]
fn t09_static_scale_memo() {
    setup_metrics();
    // render 与 layout 同为 POINT → 比例 1.0；两次调用同值（惰性 memo）
    let a = UnitsConverter::layoutRenderScaleX();
    let b = UnitsConverter::layoutRenderScaleX();
    assert_eq!(a, b);
    assert_eq!(a, 1.0);
    // doc(EMU) → render(POINT)：1EMU == 1/12700 POINT
    let d = UnitsConverter::docRenderScaleX();
    assert_close(d, 1.0 / 12700.0, 1e-8);
    assert_eq!(UnitsConverter::docRenderScaleX(), d);
}

#[test]
fn t10_point_emu_static() {
    setup_metrics();
    assert_eq!(UnitsConverter::pointToEmu(1.0), 12700.0);
    assert_eq!(UnitsConverter::emuToPoint(12700.0), 1.0);
    assert_eq!(UnitsConverter::pointToEmu(0.0), 0.0);
    assert_eq!(UnitsConverter::emuToPoint(0.0), 0.0);
}

#[test]
fn t11_zoom_state_only() {
    setup_metrics();
    let before = UnitsConverter::render().convertX(2, 12192000.0, 0);
    {
        let mut u = UnitsConverter::render();
        u.setZoom(2.0);
        // scaledDensity=1.0 → mScaleFactor = 1.0*2.0
        assert_eq!(u.getZoom(), 2.0);
    }
    let after = UnitsConverter::render().convertX(2, 12192000.0, 0);
    assert_eq!(before, after, "zoom 不得进入换算");
    {
        let mut u = UnitsConverter::render();
        u.setZoom(1.0);
    }
}