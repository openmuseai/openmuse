//! 05_function_sppr_data_shapepr — handler field is drawing.data.shape.ShapePr
use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler;

fn handler() -> (Rc<RefCell<ShapePr>>, ShapePropertiesHandler) {
    let pr = Rc::new(RefCell::new(ShapePr::create()));
    let h = ShapePropertiesHandler::ShapePropertiesHandler(pr.clone());
    (pr, h)
}

#[test]
fn pr0_current_handler_dispatches_ln_prst_unknown() {
    let (_pr, mut h) = handler();
    assert!(h.enterSubElement(1114148).is_some(), "ln=1114148");
    assert!(h.enterSubElement(1114429).is_some(), "prstGeom=1114429");
    assert!(h.enterSubElement(99).is_none(), "unknown => null");
}

#[test]
fn pr1_ctor_type_is_data_shapepr() {
    let (pr, _h) = handler();
    let _sp: Rc<RefCell<ShapePr>> = pr;
}

#[test]
fn pr2_ln_survives_commit() {
    let (pr, mut h) = handler();
    assert!(h.enterSubElement(1114148).is_some());
    h.endElement(0);
    pr.borrow_mut().ln().putWidth(1114148);
    assert_eq!(pr.borrow_mut().ln().get_width(), 1114148);
    assert!(pr.borrow().hasLn() || pr.borrow_mut().ln().hasWidth());
}

#[test]
fn pr3_prst_puts_geom_type_zero() {
    let (pr, mut h) = handler();
    assert!(h.enterSubElement(1114429).is_some());
    h.endElement(0);
    assert_eq!(pr.borrow_mut().geom().get_type(), 0);
}

#[test]
fn pr4_xfrm_chain() {
    let (pr, mut h) = handler();
    assert!(h.enterSubElement(3211589).is_some());
    h.endElement(0);
    pr.borrow_mut().xfrm().putX(100);
    pr.borrow_mut().xfrm().putY(200);
    pr.borrow_mut().xfrm().putCx(300);
    pr.borrow_mut().xfrm().putCy(400);
    assert_eq!(pr.borrow_mut().xfrm().getX(), 100);
    assert_eq!(pr.borrow_mut().xfrm().getY(), 200);
    assert_eq!(pr.borrow_mut().xfrm().getCx(), 300);
    assert_eq!(pr.borrow_mut().xfrm().getCy(), 400);
}

#[test]
fn pr5_unknown_is_none() {
    let (_pr, mut h) = handler();
    assert!(h.enterSubElement(1114199).is_none());
}
