use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
use ppt_core::cn::wps::show::pptxr::handler::timing::TimingHandler::TimingHandler;
use ppt_core::cn::wps::show::pptxr::handler::timing::timing_model_surface::Timing;
use ppt_core::cn::wps::show::pptxr::handler::transition::TransitionAlternateContentHandler::TransitionAlternateContentHandler;
use ppt_core::cn::wps::show::pptxr::handler::transition::TransitionHandler::TransitionHandler;
use ppt_core::cn::wps::show::pptxr::handler::transition::transition_model_surface::Transition;

fn cb() -> Rc<RefCell<HandlerCallback>> {
    Rc::new(RefCell::new(HandlerCallback::pptx()))
}

fn trans() -> Rc<RefCell<Transition>> {
    Rc::new(RefCell::new(Transition::create()))
}

#[test]
fn t01_start_element_four_attrs() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let mut a = XmlRoAttrList::new();
    a.add_attr(3211297, "true");
    a.add_attr(3211298, "42");
    a.add_attr(3211296, "med");
    a.add_attr(3276828, "500");
    h.startElement(3211402, &a);
    let v = t.borrow();
    assert_eq!(v.adv_click, Some(true));
    assert_eq!(v.adv_tm, Some(42));
    assert_eq!(v.spd, Some(1));
    assert_eq!(v.dur, Some(500));
}

#[test]
fn t02_unknown_spd_does_not_dirty() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let mut a = XmlRoAttrList::new();
    a.add_attr(3211296, "not-a-speed");
    h.startElement(3211402, &a);
    assert_eq!(t.borrow().spd, None);
}

#[test]
fn t03_pure_mode_children() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let cases = [
        (3211275, 27),
        (3211276, 5),
        (3211280, 17),
        (3211282, 22),
        (3211283, 18),
        (3211286, 1),
        (3211290, 19),
        (3276806, 108),
        (3276821, 103),
    ];
    for (id, mode) in cases {
        assert!(h.enterSubElement(id).is_none());
        assert_eq!(t.borrow().mode, Some(mode));
    }
}

#[test]
fn t04_orientation_blinds_horz() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let mut child = h.enterSubElement(3211273).expect("orientation");
    let mut a = XmlRoAttrList::new();
    a.add_attr(3211265, "horz");
    child.startElement(3211273, &a);
    let v = t.borrow();
    assert_eq!(v.mode, Some(2));
    assert_eq!(v.blinds_direction, Some(0));
}

#[test]
fn t05_leftright_flip() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let mut child = h.enterSubElement(3276804).expect("leftright");
    let mut a = XmlRoAttrList::new();
    a.add_attr(3276801, "l");
    child.startElement(3276804, &a);
    let v = t.borrow();
    assert_eq!(v.mode, Some(104));
    assert_eq!(v.flip_direction, Some(0));
}

#[test]
fn t06_wheel_spokes() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let mut w1 = h.enterSubElement(3211291).expect("wheel");
    let mut a = XmlRoAttrList::new();
    a.add_attr(3211268, "4");
    w1.startElement(3211291, &a);
    assert_eq!(t.borrow().mode, Some(26));
    assert_eq!(t.borrow().wheel_spokes, Some(4));
    let mut w2 = h.enterSubElement(3276827).expect("wheel reverse");
    let mut b = XmlRoAttrList::new();
    b.add_attr(3276882, "3");
    w2.startElement(3276827, &b);
    assert_eq!(t.borrow().mode, Some(117));
    assert_eq!(t.borrow().wheel_revere_spokes, Some(3));
}

#[test]
fn t07_shred_defaults() {
    let t = trans();
    let mut h = TransitionHandler::TransitionHandler(t.clone(), cb());
    let mut child = h.enterSubElement(3276822).expect("shred");
    child.startElement(3276822, &XmlRoAttrList::new());
    let v = t.borrow();
    assert_eq!(v.mode, Some(113));
    assert_eq!(v.shred_pattern, Some(1));
    assert_eq!(v.shred_direction, Some(1));
}

#[test]
fn t08_alternate_content_choice_and_fallback() {
    let t = trans();
    let timing = Rc::new(RefCell::new(Timing::create()));
    let mut h = TransitionAlternateContentHandler::TransitionAlternateContentHandler(timing, t, cb());
    let mut choice = h.enterSubElement(2162693).expect("choice");
    assert_eq!(choice.kind(), "TransitionAlternateContentChoice");
    let inner_t = choice.enterSubElement(3211402).expect("transition");
    assert_eq!(inner_t.kind(), "TransitionHandler");
    let inner_tm = choice.enterSubElement(3211609).expect("timing");
    assert_eq!(inner_tm.kind(), "TimingHandler");
    assert!(h.enterSubElement(2162695).is_none());
}
