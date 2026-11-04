use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::style::ColorMapHandler::ColorMapHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::util::DGMap::DGMap;
use ppt_core::cn::wps::show::util::layout_shape::ColorMappingObj;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

#[test]
fn t01_twelve_scheme_slots() {
    let m = Rc::new(RefCell::new(ColorMappingObj::default()));
    let mut h = ColorMapHandler::ColorMapHandler(m.clone());
    h.startElement(
        0,
        &attrs(&[
            (1114128, "accent1"),
            (1114129, "accent2"),
            (1114130, "accent3"),
            (1114131, "accent4"),
            (1114132, "accent5"),
            (1114133, "accent6"),
            (1114134, "hlink"),
            (1114135, "folHlink"),
            (1114453, "bg1"),
            (1114454, "tx1"),
            (1114455, "bg2"),
            (1114456, "tx2"),
        ]),
    );
    let map = DGMap::schemeColorMap();
    let b = m.borrow();
    assert_eq!(b.accent1, Some(*map.get("accent1").unwrap()));
    assert_eq!(b.accent6, Some(*map.get("accent6").unwrap()));
    assert_eq!(b.hlink, Some(*map.get("hlink").unwrap()));
    assert_eq!(b.fol_hlink, Some(*map.get("folHlink").unwrap()));
    assert_eq!(b.bg1, Some(*map.get("bg1").unwrap()));
    assert_eq!(b.tx1, Some(*map.get("tx1").unwrap()));
    assert_eq!(b.bg2, Some(*map.get("bg2").unwrap()));
    assert_eq!(b.tx2, Some(*map.get("tx2").unwrap()));
}

#[test]
fn t02_null_attr_skip() {
    let m = Rc::new(RefCell::new(ColorMappingObj::default()));
    let mut h = ColorMapHandler::ColorMapHandler(m.clone());
    h.startElement(
        0,
        &attrs(&[(1114128, "accent1"), (0, ""), (1114453, "bg1")]),
    );
    assert_eq!(m.borrow().accent1, Some(4));
    assert_eq!(m.borrow().bg1, Some(12));
}

#[test]
fn t03_empty_and_unknown() {
    let m = Rc::new(RefCell::new(ColorMappingObj::default()));
    let mut h = ColorMapHandler::ColorMapHandler(m.clone());
    h.startElement(0, &attrs(&[]));
    assert_eq!(*m.borrow(), ColorMappingObj::default());
    h.startElement(0, &attrs(&[(999, "accent1")]));
    assert_eq!(*m.borrow(), ColorMappingObj::default());
}

#[test]
fn t04_mixed_and_repeat_slot() {
    let m = Rc::new(RefCell::new(ColorMappingObj::default()));
    let mut h = ColorMapHandler::ColorMapHandler(m.clone());
    h.startElement(
        0,
        &attrs(&[(1114128, "accent1"), (1114453, "bg1"), (1114456, "tx2")]),
    );
    h.startElement(0, &attrs(&[(1114128, "dk1")]));
    assert_eq!(m.borrow().accent1, Some(0));
    assert_eq!(m.borrow().bg1, Some(12));
    assert_eq!(m.borrow().tx2, Some(16));
}
