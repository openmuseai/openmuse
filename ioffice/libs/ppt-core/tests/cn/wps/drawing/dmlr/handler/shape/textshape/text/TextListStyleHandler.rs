use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextListStyleHandler::{
    TextListStyle, TextListStyleHandler,
};
use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;

const LST_IDS: [i32; 10] = [
    1114533, 1114534, 1114535, 1114536, 1114537, 1114538, 1114539, 1114540, 1114541, 1114542,
];

fn handler() -> TextListStyleHandler {
    TextListStyleHandler::TextListStyleHandler(
        Rc::new(RefCell::new(TextListStyle::create())),
        HandlerCallback(),
    )
}

#[test]
fn t01_ten_known_lststyle_ids() {
    let mut h = handler();
    for id in LST_IDS {
        let child = h.enterSubElement(id);
        assert!(child.is_some(), "id {id}");
        assert_eq!(child.unwrap().kind(), "TextParagraphPropertiesHandler");
    }
}

#[test]
fn t02_unknown_id_none() {
    let mut h = handler();
    assert!(h.enterSubElement(1114448).is_none());
    assert!(h.enterSubElement(0).is_none());
    assert_eq!(h.reused_handler_init_count(), 0);
}

#[test]
fn t03_title_body_other_slots() {
    let cb = HandlerCallback();
    let title = Rc::new(RefCell::new(TextListStyle::create()));
    let body = Rc::new(RefCell::new(TextListStyle::create()));
    let other = Rc::new(RefCell::new(TextListStyle::create()));
    let mut ht = TextListStyleHandler::TextListStyleHandler(title, cb);
    let mut hb = TextListStyleHandler::TextListStyleHandler(body, cb);
    let mut ho = TextListStyleHandler::TextListStyleHandler(other, cb);
    assert!(ht.enterSubElement(1114533).is_some());
    assert!(hb.enterSubElement(1114533).is_some());
    assert!(ho.enterSubElement(1114533).is_some());
}

#[test]
fn t04_reuse_same_handler_via_init() {
    let mut h = handler();
    assert!(h.enterSubElement(1114533).is_some());
    assert_eq!(h.reused_handler_init_count(), 1);
    assert!(h.enterSubElement(1114534).is_some());
    assert_eq!(h.reused_handler_init_count(), 2);
}
