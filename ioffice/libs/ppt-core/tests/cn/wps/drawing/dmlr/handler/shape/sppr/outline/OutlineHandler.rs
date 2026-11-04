use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::outline::OutlineHandler::OutlineHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 测试面收口 —— 处理器现持**真身** `data::outline::Outline`。
use ppt_core::cn::wps::drawing::data::outline::Outline::Outline;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

fn ln() -> (Rc<RefCell<Outline>>, OutlineHandler) {
    let o = Rc::new(RefCell::new(Outline::create()));
    (o.clone(), OutlineHandler::OutlineHandler(o))
}

#[test]
fn t01_ln_attrs_and_miss() {
    let (o, mut h) = ln();
    h.startElement(0, &attrs(&[(1114304, "100000"), (1114442, "sng"), (1114441, "flat"), (1114328, "ctr")]));
    // rust: ★2B-③-2 测试面收口 —— 真 `Outline` 的读取访问器。
    assert_eq!(o.borrow().get_width(), 100000);
    assert_eq!(o.borrow().compoundType(), 0);
    assert_eq!(o.borrow().lineCap(), 2);
    assert_eq!(o.borrow().get_strokeAlignment(), 0);
    let (o2, mut h) = ln();
    h.startElement(0, &attrs(&[(1114442, "nope"), (1114441, "nope"), (1114328, "nope")]));
    assert!(!o2.borrow().hasCompoundType());
    assert!(!o2.borrow().hasLineCap());
    assert!(!o2.borrow().hasStrokeAlignment());
}

#[test]
fn t02_line_join_and_miter_limit() {
    let (o, mut h) = ln();
    assert!(h.enterSubElement(1114433).is_none());
    assert_eq!(o.borrow_mut().lineJoin().get_type(), 2);
    h.enterSubElement(1114434);
    assert_eq!(o.borrow_mut().lineJoin().get_type(), 0);
    let mut miter = h.enterSubElement(1114435).unwrap();
    assert_eq!(o.borrow_mut().lineJoin().get_type(), 1);
    miter.startElement(0, &attrs(&[(1114432, "800000")]));
    assert!((o.borrow_mut().lineJoin().get_miterJoinLimit() - 8.0).abs() < 1e-9);
}

#[test]
fn t03_prst_and_custom_dash() {
    let (o, mut h) = ln();
    let mut prst = h.enterSubElement(1114437).unwrap();
    prst.startElement(0, &attrs(&[(1114158, "dash")]));
    assert_eq!(o.borrow().get_presetDash(), 6);
    let mut cust = h.enterSubElement(1114438).unwrap();
    let mut s1 = cust.enterSubElement(1114436).unwrap();
    s1.startElement(0, &attrs(&[(1114160, "100000"), (3211593, "200000")]));
    let mut s2 = cust.enterSubElement(1114436).unwrap();
    s2.startElement(0, &attrs(&[(1114160, "300000"), (3211593, "400000")]));
    h.endElement(0);
    // rust: ★W6c-6 —— 断言面修正。Java 的 `OutlineHandler.endElement` 是
    //   `outline.customDashLst().write(this.dashStops)`，而 `dashStops` 是
    //   `List<Outline.DashStop>` ⇒ **每个停靠点一个 bin**（bin 内部 KpMap 持
    //   `dashLength`=键1、`spaceLength`=键2），不是把 `dash`/`space` 拍平成 2N 个 int。
    //   原断言按拍平形态写（`bins.len()==2`、`bins[0]` 直接当长度），与 Java 不符。
    //   owner: tests.drawing.dmlr.handler.shape.sppr.outline    wave: M-PPT-KMOSLIDE-W6c-6
    let bins = o.borrow().readCustomDashLst_bins();
    assert_eq!(bins.len(), 2, "两个 DashStop ⇒ 两个 bin（Java: 一停靠点一 bin）");
    let stop0 = ppt_core::cn::wps::drawing::data::outline::Outline::DashStop::open(bins[0].as_ref());
    let stop1 = ppt_core::cn::wps::drawing::data::outline::Outline::DashStop::open(bins[1].as_ref());
    assert!((stop0.get_dashLength() - 1.0).abs() < 1e-9);
    assert!((stop0.get_spaceLength() - 2.0).abs() < 1e-9);
    assert!((stop1.get_dashLength() - 3.0).abs() < 1e-9);
    assert!((stop1.get_spaceLength() - 4.0).abs() < 1e-9);
}

#[test]
fn t04_head_tail_force_write() {
    let (o, mut h) = ln();
    h.endElement(0);
    assert!(!o.borrow().hasHeadEnd());
    assert!(!o.borrow().hasTailEnd());
    let (o, mut h) = ln();
    let mut head = h.enterSubElement(1114439).unwrap();
    head.startElement(0, &attrs(&[(1114388, "arrow")]));
    h.endElement(0);
    assert!(o.borrow().hasHeadEnd());
    assert!(!o.borrow().hasTailEnd());
    assert_eq!(o.borrow_mut().headEnd().get_type(), 5);
    let (o, mut h) = ln();
    let mut tail = h.enterSubElement(1114440).unwrap();
    tail.startElement(0, &attrs(&[(1114388, "triangle")]));
    h.endElement(0);
    assert!(!o.borrow().hasHeadEnd());
    assert!(o.borrow().hasTailEnd());
}

#[test]
fn t05_ln_solid_fill() {
    let (o, mut h) = ln();
    let mut solid = h.enterSubElement(1114371).unwrap();
    assert_eq!(solid.kind(), "SolidFillHandler");
    let mut rgb = solid.enterSubElement(1114216).unwrap();
    rgb.startElement(0, &attrs(&[(1114158, "112233")]));
    h.endElement(0);
    // rust: ★2B-③-2 —— 真 `OutlineFill` 的类型；RGB 经真 `Color`/`RGB` 读回。
    let mut ob = o.borrow_mut();
    let f = ob.fill();
    assert_eq!(f.r#type(), 0);
    let c = f.solidFill_mut();
    let rgb = c.srgb();
    assert_eq!((rgb.r(), rgb.g(), rgb.b()), (0x11, 0x22, 0x33));
}
