use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
use ppt_core::cn::wps::show::pptxr::handler::timing::TimeNodeListHandler::TimeNodeListHandler;
use ppt_core::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNode;

fn cb() -> Rc<RefCell<HandlerCallback>> {
    Rc::new(RefCell::new(HandlerCallback::pptx()))
}

#[test]
fn t01_seq_par_excl() {
    let list = Rc::new(RefCell::new(Vec::new()));
    let mut h = TimeNodeListHandler::TimeNodeListHandler(list.clone(), cb());
    let seq = h.enterSubElement(3211324).expect("seq");
    assert_eq!(seq.kind(), "TimeNodeSequenceHandler");
    let par = h.enterSubElement(3211323).expect("par");
    assert_eq!(par.kind(), "TimeNodeParallelHandler");
    let excl = h.enterSubElement(3211325).expect("excl");
    assert_eq!(excl.kind(), "TimeNodeExclusiveHandler");
    let types: Vec<i32> = list.borrow().iter().map(|n| n.borrow().type_.unwrap()).collect();
    assert_eq!(types, vec![1, 0, 8]);
}

#[test]
fn t02_unknown_id_null() {
    let list = Rc::new(RefCell::new(Vec::new()));
    let mut h = TimeNodeListHandler::TimeNodeListHandler(list.clone(), cb());
    assert!(h.enterSubElement(3211320).is_none());
    assert!(list.borrow().is_empty());
}

#[test]
fn t03_zero_one_n() {
    let list = Rc::new(RefCell::new(Vec::new()));
    let mut h = TimeNodeListHandler::TimeNodeListHandler(list.clone(), cb());
    assert_eq!(list.borrow().len(), 0);
    let _ = h.enterSubElement(3211324);
    assert_eq!(list.borrow().len(), 1);
    let _ = h.enterSubElement(3211323);
    let _ = h.enterSubElement(3211325);
    assert_eq!(list.borrow().len(), 3);
}

#[test]
fn t04_anim_fixture() {
    let list = Rc::new(RefCell::new(Vec::new()));
    let mut h = TimeNodeListHandler::TimeNodeListHandler(list.clone(), cb());
    let anim = h.enterSubElement(3211326).expect("anim");
    assert_eq!(anim.kind(), "AnimateBehaviorHandler");
    assert_eq!(list.borrow()[0].borrow().type_, Some(9));
}
