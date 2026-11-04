use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::show::pptxr::ppt::PresentationHandlerCallback::HandlerCallbackRef;
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

#[path = "_recording_callback.rs"]
mod recording_callback;
use recording_callback::RecordingCallback;

fn pres() -> Rc<RefCell<KmoPresentation>> {
    Rc::new(RefCell::new(KmoPresentation::default()))
}
use ppt_core::cn::wps::show::pptxr::handler::PresentationHandler::PresentationHandler;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

fn parent() -> PresentationHandler {
    PresentationHandler::PresentationHandler(
        pres(),
        HandlerCallbackRef::new(Rc::new(RecordingCallback::default())),
    )
}

#[test]
fn t01_sldsz_type_cx_cy() {
    let mut p = parent();
    let mut h = p.enterSubElement(3211513).expect("sldSz");
    h.startElement(
        3211513,
        &attrs(&[(3211302, "screen4x3"), (3211488, "9144000"), (3211489, "6858000")]),
    );
    let sz = p.presentation_data().unwrap().slideSize.borrow().clone();
    assert_eq!(sz.type_, Some(0));
    assert_eq!(sz.cx, Some(9144000));
    assert_eq!(sz.cy, Some(6858000));
}

#[test]
fn t02_screen16x9_overrides_type() {
    let mut p = parent();
    let mut h = p.enterSubElement(3211513).unwrap();
    h.startElement(
        3211513,
        &attrs(&[(3211302, "letter"), (3211488, "12192000"), (3211489, "6858000")]),
    );
    let sz = p.presentation_data().unwrap().slideSize.borrow().clone();
    assert_eq!(sz.type_, Some(15));
    assert_eq!(sz.cx, Some(12192000));
    assert_eq!(sz.cy, Some(6858000));
}

#[test]
fn t03_missing_attrs_skip() {
    let mut p = parent();
    let mut h = p.enterSubElement(3211513).unwrap();
    h.startElement(3211513, &attrs(&[(3211488, "100")]));
    let sz = p.presentation_data().unwrap().slideSize.borrow().clone();
    assert_eq!(sz.type_, None);
    assert_eq!(sz.cx, Some(100));
    assert_eq!(sz.cy, None);
}

#[test]
fn t04_notessz_a_and_p_namespaces() {
    let mut p = parent();
    let mut h = p.enterSubElement(3211514).expect("notesSz");
    assert_eq!(h.kind(), "PositiveSize2DHandler");
    h.startElement(3211514, &attrs(&[(1114163, "6858000"), (1114164, "9144000")]));
    {
        let n = p.presentation_data().unwrap().notesSize.borrow().clone();
        assert_eq!(n.cx, Some(6858000));
        assert_eq!(n.cy, Some(9144000));
    }
    h.startElement(3211514, &attrs(&[(3211488, "1"), (3211489, "2")]));
    let n = p.presentation_data().unwrap().notesSize.borrow().clone();
    assert_eq!(n.cx, Some(1));
    assert_eq!(n.cy, Some(2));
}
