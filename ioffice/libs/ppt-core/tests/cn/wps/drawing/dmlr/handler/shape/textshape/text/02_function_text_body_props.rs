use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::BodyPropertiesHandler::BodyPropertiesHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::TextBodyProperties;

fn handler(pr: &Rc<RefCell<TextBodyProperties>>) -> BodyPropertiesHandler {
    BodyPropertiesHandler::BodyPropertiesHandler(pr.clone(), None)
}

#[test]
fn t01_insets_and_rotation() {
    let pr = Rc::new(RefCell::new(TextBodyProperties::create()));
    let mut h = handler(&pr);
    let mut a = XmlRoAttrList::new();
    a.add_attr(1114551, "91440");
    a.add_attr(1114552, "91440");
    a.add_attr(1114553, "91440");
    a.add_attr(1114554, "91440");
    a.add_attr(1114201, "60000");
    h.startElement(1114448, &a);
    let p = pr.borrow();
    assert_eq!(p.leftInset, Some(91440));
    assert_eq!(p.topInset, Some(91440));
    assert_eq!(p.rightInset, Some(91440));
    assert_eq!(p.bottomInset, Some(91440));
    assert_eq!(p.rotation, Some(60000));
}

#[test]
fn t02_anchor_vert_wrap_maps_and_miss() {
    let pr = Rc::new(RefCell::new(TextBodyProperties::create()));
    let mut h = handler(&pr);
    let mut a = XmlRoAttrList::new();
    a.add_attr(1114301, "t");
    a.add_attr(1114480, "eaVert");
    a.add_attr(1114550, "none");
    h.startElement(1114448, &a);
    {
        let p = pr.borrow();
        assert_eq!(p.anchor, Some(0));
        assert_eq!(p.verticalText, Some(4));
        assert_eq!(p.wrap, Some(0));
    }
    let mut miss = XmlRoAttrList::new();
    miss.add_attr(1114301, "not-anchor");
    h.startElement(1114448, &miss);
    assert_eq!(pr.borrow().anchor, Some(0));
}

#[test]
fn t03_autofit_three_types() {
    let pr = Rc::new(RefCell::new(TextBodyProperties::create()));
    let mut h = handler(&pr);
    assert!(h.enterSubElement(1114545).is_none());
    assert_eq!(pr.borrow().autofit_ty, Some(0));

    let mut n = h.enterSubElement(1114546).expect("normAutofit");
    let mut attrs = XmlRoAttrList::new();
    attrs.add_attr(1114543, "100000");
    attrs.add_attr(1114544, "20000");
    n.startElement(1114546, &attrs);
    assert_eq!(pr.borrow().autofit_ty, Some(1));
    assert_eq!(pr.borrow().fontScale, Some(1.0));
    assert_eq!(pr.borrow().lnSpcReduction, Some(0.2));

    assert!(h.enterSubElement(1114547).is_none());
    assert_eq!(pr.borrow().autofit_ty, Some(2));
}

#[test]
fn t04_flat_tx_with_and_without_z() {
    let pr = Rc::new(RefCell::new(TextBodyProperties::create()));
    let mut h = handler(&pr);
    let mut z = XmlRoAttrList::new();
    z.add_attr(1114206, "1");
    let mut ft = h.enterSubElement(1114313).expect("flatTx");
    ft.startElement(1114313, &z);
    assert_eq!(pr.borrow().flatTx, Some(1));

    let mut ft2 = h.enterSubElement(1114313).expect("flatTx empty");
    ft2.startElement(1114313, &XmlRoAttrList::new());
    assert_eq!(pr.borrow().flatTx, Some(0));
}

#[test]
fn t05_unknown_and_deferred_subelements() {
    let pr = Rc::new(RefCell::new(TextBodyProperties::create()));
    let mut h = handler(&pr);
    let mut a = XmlRoAttrList::new();
    a.add_attr(1114551, "10");
    h.startElement(1114448, &a);
    assert!(h.enterSubElement(1114144).is_none());
    assert!(h.enterSubElement(1114430).is_none());
    let mut unk = XmlRoAttrList::new();
    unk.add_attr(999, "x");
    h.startElement(1114448, &unk);
    assert_eq!(pr.borrow().leftInset, Some(10));
}
