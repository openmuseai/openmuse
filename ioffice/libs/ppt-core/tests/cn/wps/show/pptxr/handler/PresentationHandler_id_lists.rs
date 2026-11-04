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
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::show::pptxr::handler::PresentationHandler::{
    NotesAndHandoutMasterIdListHandler, PresentationHandler, SlideAndMasterIdListHandler,
};

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

fn parent(cb: Rc<RecordingCallback>) -> PresentationHandler {
    PresentationHandler::PresentationHandler(pres(), HandlerCallbackRef::new(cb))
}

#[test]
fn t01_slide_id_list_end_callback_order() {
    let cb = Rc::new(RecordingCallback::default());
    let mut p = parent(cb.clone());
    let mut h = p.enterSubElement(3211512).expect("sldIdLst");
    h.startElement(3211473, &attrs(&[]));
    h.endElement(3211473);
    assert!(cb.on_slide_ids.borrow().is_empty());
    h.startElement(3211473, &attrs(&[(4259841, "rId1")]));
    h.startElement(3211473, &attrs(&[(4259841, "rId2")]));
    h.endElement(3211512);
    assert_eq!(&*cb.on_slide_ids.borrow(), &vec!["rId1".to_string(), "rId2".to_string()]);
}

#[test]
fn t02_slide_master_id_list() {
    let cb = Rc::new(RecordingCallback::default());
    let mut h = SlideAndMasterIdListHandler::SlideAndMasterIdListHandler();
    h.handlerCallback = HandlerCallbackRef::new(cb.clone());
    h.startElement(3211474, &attrs(&[(4259841, "m1")]));
    h.startElement(3211474, &attrs(&[(4259841, "rId9")]));
    h.endElement(3211509);
    assert_eq!(&*cb.on_slide_master_ids.borrow(), &["m1".to_string(), "rId9".to_string()]);
}

#[test]
fn t03_enter_sub_returns_same_kind() {
    // java public ElementHandler enterSubElement(int subElement) { return this; }
    //
    // rust: ★W6c-6 —— 断言面修正。原断言期望 `None`（注释称「用 `None` 表达同一语义」），
    //   但 `SlideAndMasterIdListHandler::enterSubElement` 的真实实现是
    //   `Some(Box::new(self.clone()))`，且该结构体的 `rids` 是
    //   `Rc<RefCell<Vec<String>>>` ⇒ 克隆体与本体**共享同一份列表**，
    //   子元素事件写回同一列表 —— 这正是 Java `return this` 的可观察语义。
    //   原断言既与实现不符，也把「返回 handler」误当成「返回 None」。
    //   owner: tests.pptxr.handler.PresentationHandler    wave: M-PPT-KMOSLIDE-W6c-6
    let mut h = SlideAndMasterIdListHandler::SlideAndMasterIdListHandler();
    let sub = h.enterSubElement(0);
    assert!(sub.is_some(), "java `return this` ⇒ 必须返回一个 handler");
    assert_eq!(sub.unwrap().kind(), "SlideAndMasterIdListHandler");
    assert_eq!(h.kind(), "SlideAndMasterIdListHandler");
}

#[test]
fn t04_notes_and_handout_lists() {
    let cb = Rc::new(RecordingCallback::default());
    let mut notes = NotesAndHandoutMasterIdListHandler::NotesAndHandoutMasterIdListHandler();
    notes.handlerCallback = HandlerCallbackRef::new(cb.clone());
    notes.startElement(3211475, &attrs(&[(4259841, "n1")]));
    notes.endElement(3211510);
    let mut hand = NotesAndHandoutMasterIdListHandler::NotesAndHandoutMasterIdListHandler();
    hand.handlerCallback = HandlerCallbackRef::new(cb.clone());
    hand.startElement(3211476, &attrs(&[(4259841, "h1")]));
    hand.endElement(3211511);
    assert_eq!(&*cb.on_notes_master_ids.borrow(), &["n1".to_string()]);
    assert_eq!(&*cb.on_handout_master_ids.borrow(), &["h1".to_string()]);
}

#[test]
fn t05_slide_id_skips_missing_rid() {
    let cb = Rc::new(RecordingCallback::default());
    let mut h = SlideAndMasterIdListHandler::SlideAndMasterIdListHandler();
    h.handlerCallback = HandlerCallbackRef::new(cb.clone());
    h.startElement(3211473, &attrs(&[]));
    h.startElement(3211473, &attrs(&[(4259841, "keep")]));
    h.endElement(3211512);
    assert_eq!(&*cb.on_slide_ids.borrow(), &["keep".to_string()]);
}
