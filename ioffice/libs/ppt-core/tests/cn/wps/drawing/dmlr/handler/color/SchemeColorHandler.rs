use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
use ppt_core::cn::wps::drawing::dmlr::handler::color::SchemeColorHandler::SchemeColorHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotify;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::util::DGMap::DGMap;
use ppt_core::cn::wps::show::util::layout_shape::ColorSlot;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

#[test]
fn t01_scheme_tokens() {
    for token in ["accent1", "bg1", "tx1"] {
        let color = Rc::new(RefCell::new(ColorSlot::default()));
        let mut h = SchemeColorHandler::SchemeColorHandler(color.clone(), None, None, 0);
        h.startElement(0, &attrs(&[(1114158, token)]));
        assert_eq!(color.borrow().mode, Some(4));
        assert_eq!(
            color.borrow().scheme,
            Some(*DGMap::schemeColorMap().get(token).unwrap())
        );
    }
}

#[test]
fn t02_missing_and_unknown_token() {
    let color = Rc::new(RefCell::new(ColorSlot::default()));
    let mut h = SchemeColorHandler::SchemeColorHandler(color.clone(), None, None, 0);
    h.startElement(0, &attrs(&[]));
    assert_eq!(color.borrow().scheme, None);
    let color = Rc::new(RefCell::new(ColorSlot::default()));
    let mut h = SchemeColorHandler::SchemeColorHandler(color.clone(), None, None, 0);
    h.startElement(0, &attrs(&[(1114158, "not-a-scheme")]));
    assert_eq!(color.borrow().scheme, None);
}

#[test]
fn t03_callback_null_and_belonger() {
    let color = Rc::new(RefCell::new(ColorSlot::default()));
    let mut h = SchemeColorHandler::SchemeColorHandler(color.clone(), None, None, 1114371);
    h.startElement(0, &attrs(&[(1114158, "accent1")]));
    let notify = Rc::new(RefCell::new(FillNotify::default()));
    let color = Rc::new(RefCell::new(ColorSlot::default()));
    let mut h = SchemeColorHandler::SchemeColorHandler(
        color,
        None,
        Some(notify.clone()),
        1114371,
    );
    h.startElement(0, &attrs(&[(1114158, "accent1")]));
    assert_eq!(notify.borrow().theme_belonger, vec![1114371]);
}

#[test]
fn t04_factory_scheme_branch() {
    let color = Rc::new(RefCell::new(ColorSlot::default()));
    let h = ColorHandlerFactory::createColorHandler(1114219, color, None, None, 1114371).unwrap();
    assert_eq!(h.kind(), "SchemeColorHandler");
}
