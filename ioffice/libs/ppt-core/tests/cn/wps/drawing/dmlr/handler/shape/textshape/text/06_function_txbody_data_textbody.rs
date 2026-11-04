//! 06_function_txbody_data_textbody — TextBodyHandler writes data::text::TextBody.paraLst
use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::data::text::TextBody::TextBody;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextBodyHandler::TextBodyHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_body_surf::{
    TextBodyHandler as SurfHandler, TextBodySurf,
};

fn empty_attrs() -> XmlRoAttrList {
    XmlRoAttrList::new()
}

#[test]
fn tx0_current_handler_collects_one_paragraph() {
    let body = Rc::new(RefCell::new(TextBodySurf::default()));
    let mut h = SurfHandler::TextBodyHandler(body.clone());
    let mut p = h.enterSubElement(1114562).expect("p");
    let mut run = p.enterSubElement(1114193).expect("r");
    let mut t = run.enterSubElement(1114214).expect("t");
    t.characters("x");
    t.endElement(1114214);
    run.endElement(1114193);
    p.endElement(1114562);
    h.endElement(1114561);
    assert_eq!(body.borrow().paras.len(), 1);
    assert_eq!(body.borrow().paras[0].runs.len(), 1);
}

#[test]
fn tx1_no_paragraph_skips_force_write() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    h.endElement(1114561);
    assert!(!body.borrow().hasParaLst());
}

#[test]
fn tx2_one_para_force_write() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    let mut p = h.enterSubElement(1114562).expect("p");
    p.startElement(1114562, &empty_attrs());
    p.endElement(1114562);
    h.endElement(1114561);
    assert_eq!(body.borrow_mut().paraLst().size(), 1);
}

#[test]
fn tx3_two_paras() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    let mut p1 = h.enterSubElement(1114562).expect("p1");
    p1.endElement(1114562);
    let mut p2 = h.enterSubElement(1114562).expect("p2");
    p2.endElement(1114562);
    h.endElement(1114561);
    assert_eq!(body.borrow_mut().paraLst().size(), 2);
}
