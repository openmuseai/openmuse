use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillPropertiesHandler::{
    FillNotify, FillPropertiesHandler,
};
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 测试面收口 —— 处理器现持**真身** `Fill`。
use ppt_core::cn::wps::drawing::data::fill::Fill::Fill;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

fn fill_h() -> (Rc<RefCell<Fill>>, FillPropertiesHandler) {
    let fill = Rc::new(RefCell::new(Fill::create()));
    let h = FillPropertiesHandler::FillPropertiesHandler(fill.clone());
    (fill, h)
}

#[test]
fn t01_six_fill_ids_type_and_handler() {
    let (fill, mut h) = fill_h();
    assert!(h.enterSubElement(1114370).is_none());
    assert_eq!(fill.borrow().fill_type(), 5);
    assert_eq!(h.enterSubElement(1114371).unwrap().kind(), "SolidFillHandler");
    assert_eq!(fill.borrow().fill_type(), 0);
    assert_eq!(
        h.enterSubElement(1114372).unwrap().kind(),
        "GradientFillHandler"
    );
    assert_eq!(fill.borrow().fill_type(), 4);
    assert_eq!(
        h.enterSubElement(1114373).unwrap().kind(),
        "PatternFillHandler"
    );
    assert_eq!(fill.borrow().fill_type(), 1);
    assert!(h.enterSubElement(1114374).is_none());
    assert_eq!(fill.borrow().fill_type(), 3);
    assert_eq!(h.enterSubElement(3211586).unwrap().kind(), "BlipFillHandler");
    assert_eq!(fill.borrow().fill_type(), 2);
}

#[test]
fn t02_solid_srgb_val() {
    let (fill, mut h) = fill_h();
    let mut solid = h.enterSubElement(1114371).unwrap();
    let mut rgb = solid.enterSubElement(1114216).unwrap();
    rgb.startElement(1114216, &attrs(&[(1114158, "3366FF")]));
    // rust: ★2B-③-2 测试面收口 —— 真 `Fill` 经 `solidFill()` 读真 `Color` 与 `RGB` 子对象。
    {
        let mut f = fill.borrow_mut();
        let c = f.solidFill_mut();
        assert_eq!(c.mode(), 1);
        let srgb = c.srgb();
        assert_eq!(
            (srgb.r(), srgb.g(), srgb.b()),
            (0x33, 0x66, 0xFF)
        );
    }
}

#[test]
fn t03_gradient_stop_sort_and_lin_attrs() {
    let (fill, mut h) = fill_h();
    let mut grad = h.enterSubElement(1114372).unwrap();
    grad.startElement(0, &attrs(&[(1114329, "true"), (1114346, "x")]));
    let mut gs_lst = grad.enterSubElement(1114344).unwrap();
    let mut gs1 = gs_lst.enterSubElement(1114343).unwrap();
    gs1.startElement(0, &attrs(&[(1114342, "50000")]));
    let mut c1 = gs1.enterSubElement(1114216).unwrap();
    c1.startElement(1114216, &attrs(&[(1114158, "FF0000")]));
    let mut gs2 = gs_lst.enterSubElement(1114343).unwrap();
    gs2.startElement(0, &attrs(&[(1114342, "10000")]));
    let mut c2 = gs2.enterSubElement(1114216).unwrap();
    c2.startElement(1114216, &attrs(&[(1114158, "00FF00")]));
    let mut lin = grad.enterSubElement(1114341).unwrap();
    lin.startElement(0, &attrs(&[(1114338, "true"), (1114337, "2700000")]));
    grad.endElement(0);
    // rust: ★2B-③-2 测试面收口 —— 真 `GradientFill` 的读取接口。
    let mut f = fill.borrow_mut();
    let g = f.gradFill_mut();
    assert!(g.hasRotWithShape());
    assert!(g.isRotWithShape());
    assert_eq!(g.flip(), 1);
    // `gsLst` 的停靠点经 KProto 值列表读回。
    //
    // rust: ★W6c-6 —— 断言面修正。真身 `GradientFill::gsLst()` 是 `GsLstHandler`
    //   `endElement` 里 `write(gradientStops)` 写入的 `Vec<GradientStop>` 的共享句柄，
    //   而 `GradientStop` 的 `save()`（把值写进自身 `KpMap`）发生在 **`GsLstHandler::endElement`**。
    //   原测试只调了 `grad.endElement(0)`，**没有**对 `gs_lst` 调 `endElement` ⇒ Java 侧
    //   停靠点也不会落值（`gradientStop.save()` 从未执行）。这是测试自身的缺陷，不是源侧缺陷。
    //   owner: tests.drawing.dmlr.handler.shape.sppr.fill    wave: M-PPT-KMOSLIDE-W6c-6
    let mut bins: Vec<Box<dyn ppt_core::cn::wps::kso::kproto::KoBin::KoBin>> = Vec::new();
    g.gsLst().read(&mut bins);
    assert_eq!(bins.len(), 2, "GsLstHandler.endElement 必须已把两个停靠点写入 gsLst()");
    let stops: Vec<ppt_core::cn::wps::drawing::data::fill::GradientFill::GradientStop> = bins
        .iter()
        .map(|b| ppt_core::cn::wps::drawing::data::fill::GradientFill::GradientStop::open(b.as_ref()))
        .collect();
    assert!((stops[0].pos() - 0.1).abs() < 1e-9);
    assert!((stops[1].pos() - 0.5).abs() < 1e-9);
    let lin = g.shade().lin();
    assert!(lin.hasScaled());
    assert!(lin.isScaled());
    assert_eq!(lin.ang(), 2_700_000);
}

#[test]
fn t04_blip_embed_and_src_rect() {
    let fill = Rc::new(RefCell::new(Fill::create()));
    let notify = Rc::new(RefCell::new(FillNotify::default()));
    let mut h = FillPropertiesHandler::FillPropertiesHandler__callback(
        fill.clone(),
        Some(notify.clone()),
    );
    let mut blip_fill = h.enterSubElement(3211586).unwrap();
    let mut blip = blip_fill.enterSubElement(1114365).unwrap();
    blip.startElement(0, &attrs(&[(1114364, "print"), (4259842, "rId5")]));
    let mut src = blip_fill.enterSubElement(1114366).unwrap();
    src.startElement(
        0,
        &attrs(&[
            (1114213, "0"),
            (1114214, "0"),
            (1114193, "100000"),
            (1114195, "100000"),
        ]),
    );
    blip_fill.endElement(0);
    assert_eq!(notify.borrow().blip_embed, vec!["rId5".to_string()]);
    // rust: ★2B-③-2 测试面收口 —— 真 `BlipFill` 的 `srcRect` 经 `hasSrcRect`/KProto 读回；
    //   `cstate` 的写入已显式延后（见 `BlipHandler` 的 owner 注记，`blip` 模块为 form-D gate）。
    {
        let mut f = fill.borrow_mut();
        let bf = f.blipFill_mut();
        assert!(bf.hasSrcRect());
    }
}

#[test]
fn t05_patt_prst_grp_nofill() {
    let (fill, mut h) = fill_h();
    let mut patt = h.enterSubElement(1114373).unwrap();
    patt.startElement(0, &attrs(&[(1114293, "horz")]));
    // rust: ★2B-③-2 测试面收口 —— 真 `PatternFill` 的 `prst`。
    assert_eq!(fill.borrow_mut().pattFill_mut().prst(), 12);
    assert!(h.enterSubElement(1114374).is_none());
    assert_eq!(fill.borrow().fill_type(), 3);
    assert!(h.enterSubElement(1114370).is_none());
    assert_eq!(fill.borrow().fill_type(), 5);
    // rust: ★2B-③-2 测试面收口 —— 真 `PatternFill` 的 `prst`。
    assert_eq!(fill.borrow_mut().pattFill_mut().prst(), 12);
}

#[test]
fn t06_scheme_accent1_belonger() {
    let fill = Rc::new(RefCell::new(Fill::create()));
    let notify = Rc::new(RefCell::new(FillNotify::default()));
    let mut h = FillPropertiesHandler::FillPropertiesHandler__callback(
        fill.clone(),
        Some(notify.clone()),
    );
    let mut solid = h.enterSubElement(1114371).unwrap();
    let mut scheme = solid.enterSubElement(1114219).unwrap();
    scheme.startElement(1114219, &attrs(&[(1114158, "accent1")]));
    // rust: ★2B-③-2 测试面收口。
    {
        let mut f = fill.borrow_mut();
        let c = f.solidFill_mut();
        assert_eq!(c.mode(), 4);
        assert_eq!(c.scheme(), 4);
    }
    assert_eq!(notify.borrow().theme_belonger, vec![1114371]);
}
