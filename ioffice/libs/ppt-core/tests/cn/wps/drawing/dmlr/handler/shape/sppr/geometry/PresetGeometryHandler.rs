use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::CustomGeometryHandler::CustomGeometryHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::PresetGeometryHandler::PresetGeometryHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 测试面收口 —— 处理器现持**真身** `Geometry2D`（Java: `shapePr.geom()`）。
use ppt_core::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

#[test]
fn t01_prst_round_rect_and_avlst() {
    let g = Rc::new(RefCell::new(Geometry2D::create()));
    let mut h = PresetGeometryHandler::PresetGeometryHandler(g.clone());
    h.startElement(0, &attrs(&[(1114293, "roundRect")]));
    // rust: ★2B-③-2 测试面收口 —— 真 `PresetGeometry::prst()` 与 `avLst` 具体访问器。
    assert_eq!(g.borrow_mut().prstGeom().prst(), 2);
    let mut av = h.enterSubElement(1114422).unwrap();
    let mut gd = av.enterSubElement(1114392).unwrap();
    gd.startElement(0, &attrs(&[(1114136, "adj"), (1114391, "val 50000")]));
    h.endElement(0);
    // rust: ★2B-③-2 测试面收口 —— `AdjustValueList::read` 现为真实实现，
    //   返回真 `GeomGuide`（不再 `todo!()`）。
    let mut lst: Vec<Box<dyn ppt_core::cn::wps::kso::kproto::KoAr::KoAr>> = Vec::new();
    g.borrow_mut().prstGeom().avLst_concrete().read(&mut lst);
    assert_eq!(lst.len(), 1);
    let bin = ppt_core::cn::wps::kso::kproto::KoAr::KoAr::lastBin(
        lst[0].as_ref(),
        ppt_core::cn::wps::kso::kproto::KoZone::KoZone::enter(),
    );
    let gg = ppt_core::cn::wps::drawing::data::geometry::GeomGuide::GeomGuide::open(
        bin.as_ref(),
    );
    assert_eq!(gg.get_name(), "adj");
    assert_eq!(gg.get_fmla(), "val 50000");
}

#[test]
fn t02_cust_rect_coord_and_guide() {
    let g = Rc::new(RefCell::new(Geometry2D::create()));
    let mut h = CustomGeometryHandler::CustomGeometryHandler(g.clone());
    let carrier = h.carrier();
    let mut rect = h.enterSubElement(1114426).unwrap();
    rect.startElement(
        0,
        &attrs(&[
            (1114213, "0"),
            (1114193, "21600"),
            (1114214, "g1"),
            (1114195, "5"),
        ]),
    );
    // rust: ★2B-③-2 测试面收口 —— 采集载体持真实状态；四角另桥接进真身 KProto 槽位。
    assert_eq!(carrier.borrow().rect.l.coordinate, Some(0));
    assert_eq!(carrier.borrow().rect.r.coordinate, Some(21600));
    assert_eq!(carrier.borrow().rect.t.guide_name.as_deref(), Some("g1"));
    assert_eq!(carrier.borrow().rect.b.coordinate, Some(5));
    rect.endElement(0);
    let mut gb = g.borrow_mut();
    let cg = gb.custGeom();
    assert_eq!(cg.readRectCorner(6), Some(0));
    assert_eq!(cg.readRectCorner(8), Some(21600));
    assert_eq!(cg.readRectCorner(9), Some(5));
}

#[test]
fn t03_path_move_ln_close() {
    let g = Rc::new(RefCell::new(Geometry2D::create()));
    let mut h = CustomGeometryHandler::CustomGeometryHandler(g.clone());
    let carrier = h.carrier();
    let mut paths = h.enterSubElement(1114427).unwrap();
    let mut path = paths.enterSubElement(1114340).unwrap();
    let mut mv = path.enterSubElement(1114415).unwrap();
    let mut pt = mv.enterSubElement(1114409).unwrap();
    pt.startElement(0, &attrs(&[(1114161, "0"), (1114162, "0")]));
    let mut ln = path.enterSubElement(1114416).unwrap();
    let mut pt2 = ln.enterSubElement(1114409).unwrap();
    pt2.startElement(0, &attrs(&[(1114161, "21600"), (1114162, "0")]));
    assert!(path.enterSubElement(1114414).is_none());
    path.endElement(0);
    paths.endElement(0);
    assert_eq!(carrier.borrow().paths.len(), 1);
    let segs = &carrier.borrow().paths[0].segments;
    assert_eq!(segs.len(), 3);
    assert_eq!(segs[0].seg_type, 1);
    assert_eq!(segs[1].seg_type, 2);
    assert_eq!(segs[2].seg_type, 0);
    assert_eq!(segs[1].point.x.coordinate, Some(21600));
}

#[test]
fn t04_path_attrs_and_fill_miss() {
    let g = Rc::new(RefCell::new(Geometry2D::create()));
    let mut h = CustomGeometryHandler::CustomGeometryHandler(g.clone());
    let carrier = h.carrier();
    let mut paths = h.enterSubElement(1114427).unwrap();
    let mut path = paths.enterSubElement(1114340).unwrap();
    path.startElement(
        0,
        &attrs(&[
            (1114304, "21600"),
            (1114305, "10800"),
            (1114380, "norm"),
            (1114420, "true"),
            (1114421, "false"),
        ]),
    );
    path.endElement(0);
    paths.endElement(0);
    let p = &carrier.borrow().paths[0];
    assert_eq!(p.w, Some(21600));
    assert_eq!(p.h, Some(10800));
    assert_eq!(p.fill, Some(5));
    assert_eq!(p.stroke, Some(true));
    assert_eq!(p.extrusion_ok, Some(false));
    let g2 = Rc::new(RefCell::new(Geometry2D::create()));
    let mut h = CustomGeometryHandler::CustomGeometryHandler(g2.clone());
    let carrier2 = h.carrier();
    let mut paths = h.enterSubElement(1114427).unwrap();
    let mut path = paths.enterSubElement(1114340).unwrap();
    path.startElement(0, &attrs(&[(1114380, "not-a-fill")]));
    path.endElement(0);
    paths.endElement(0);
    assert_eq!(carrier2.borrow().paths[0].fill, None);
}

#[test]
fn t05_deferred_path_segments_none() {
    let g = Rc::new(RefCell::new(Geometry2D::create()));
    let mut h = CustomGeometryHandler::CustomGeometryHandler(g.clone());
    let carrier = h.carrier();
    let mut paths = h.enterSubElement(1114427).unwrap();
    let mut path = paths.enterSubElement(1114340).unwrap();
    assert!(path.enterSubElement(1114418).is_none());
    assert!(path.enterSubElement(1114419).is_none());
    assert!(path.enterSubElement(1114417).is_none());
    path.endElement(0);
    paths.endElement(0);
    assert!(carrier.borrow().paths[0].segments.is_empty());
}
