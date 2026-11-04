use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::{
    GroupShapePropertiesHandler, ShapePropertiesHandler,
};
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::transform::Transform2DHandler::{
    Point2DHandler, PositiveSize2DHandler, Transform2DHandler,
};
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr as DataShapePr;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::show::util::layout_shape::GroupShape;
// rust: ★2B-③-2 测试面收口 —— `layout_*_obj()`/`Transform2DHandler` 现用**真身**类型。
use ppt_core::cn::wps::drawing::data::basic::Transform2D::Transform2D;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

#[test]
fn t01_bw_mode_dual_id_and_miss() {
    let sp = Rc::new(RefCell::new(DataShapePr::create()));
    let mut h = ShapePropertiesHandler::ShapePropertiesHandler(sp.clone());
    h.startElement(0, &attrs(&[(1114443, "clr")]));
    assert_eq!(sp.borrow().bwMode(), 0);
    let sp2 = Rc::new(RefCell::new(DataShapePr::create()));
    let mut h = ShapePropertiesHandler::ShapePropertiesHandler(sp2.clone());
    h.startElement(0, &attrs(&[(3211605, "gray")]));
    assert_eq!(sp2.borrow().bwMode(), 2);
    let sp3 = Rc::new(RefCell::new(DataShapePr::create()));
    let mut h = ShapePropertiesHandler::ShapePropertiesHandler(sp3.clone());
    h.startElement(0, &attrs(&[(1114443, "not-a-mode")]));
    assert!(!sp3.borrow().hasBwMode());
    let g = Rc::new(RefCell::new(GroupShape::default()));
    let mut gh = GroupShapePropertiesHandler::GroupShapePropertiesHandler(g.clone());
    gh.startElement(0, &attrs(&[(1114443, "white")]));
    assert_eq!(g.borrow().sp_pr.bw_mode, Some(9));
}

#[test]
fn t02_enter_subelement_dispatch() {
    let sp = Rc::new(RefCell::new(DataShapePr::create()));
    let mut h = ShapePropertiesHandler::ShapePropertiesHandler(sp.clone());
    assert_eq!(h.enterSubElement(1114148).unwrap().kind(), "OutlineHandler");
    assert_eq!(h.enterSubElement(1114371).unwrap().kind(), "SolidFillHandler");
    assert_eq!(
        h.enterSubElement(1114372).unwrap().kind(),
        "GradientFillHandler"
    );
    assert!(h.enterSubElement(1114370).is_none());
    h.enterSubElement(1114370);
    h.endElement(0);
    // rust: ★2B-③-2 —— 真 `Fill` 的类型经访问器读取（`fill_type` 是方法，不是字段）。
    // rust: ★2B-③-2 —— `layout_fill_obj()` 现返回真身共享句柄，需借出后读取。
    assert_eq!(h.layout_fill_obj().borrow_mut().fill_type(), 5);
    assert_eq!(
        h.enterSubElement(1114429).unwrap().kind(),
        "PresetGeometryHandler"
    );
    assert_eq!(sp.borrow_mut().geom().get_type(), 0);
    assert_eq!(
        h.enterSubElement(1114428).unwrap().kind(),
        "CustomGeometryHandler"
    );
    assert_eq!(sp.borrow_mut().geom().get_type(), 1);
    assert_eq!(
        h.enterSubElement(3211589).unwrap().kind(),
        "Transform2DHandler"
    );
}

#[test]
fn t03_unknown_subelement_none() {
    let sp = Rc::new(RefCell::new(DataShapePr::create()));
    let mut h = ShapePropertiesHandler::ShapePropertiesHandler(sp);
    assert!(h.enterSubElement(1114199).is_none());
}

#[test]
fn t04_xfrm_flip_rot_dual_id() {
    let x = Rc::new(RefCell::new(Transform2D::create()));
    let mut h = Transform2DHandler::Transform2DHandler(x.clone());
    h.startElement(0, &attrs(&[(1114202, "true"), (1114203, "1"), (1114201, "5400000")]));
    assert!(x.borrow().isFlipHorizontal());
    assert!(x.borrow().isFlipVertical());
    assert_eq!(x.borrow().rotation(), 5_400_000);
    let x2 = Rc::new(RefCell::new(Transform2D::create()));
    let mut h = Transform2DHandler::Transform2DHandler(x2.clone());
    h.startElement(0, &attrs(&[(3276881, "1200000")]));
    assert_eq!(x2.borrow().rotation(), 1_200_000);
}

#[test]
fn t05_ext_off_dual_namespace() {
    let x = Rc::new(RefCell::new(Transform2D::create()));
    let mut ext = PositiveSize2DHandler::PositiveSize2DHandler(x.clone());
    ext.startElement(0, &attrs(&[(1114163, "1270000"), (1114164, "2540000")]));
    assert_eq!(
        (x.borrow().extents().cx(), x.borrow().extents().cy()),
        (1_270_000, 2_540_000)
    );
    let mut off = Point2DHandler::Point2DHandler(x.clone());
    off.startElement(0, &attrs(&[(3211405, "100"), (3211406, "200")]));
    assert_eq!((x.borrow().point().x(), x.borrow().point().y()), (100, 200));
}
