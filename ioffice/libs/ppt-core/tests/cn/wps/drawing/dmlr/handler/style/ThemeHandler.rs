use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::style::ThemeHandler::ThemeHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::show::util::layout_shape::ThemeObj;

fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut a = XmlRoAttrList::new();
    for (id, v) in pairs {
        a.add_attr(*id, v);
    }
    a
}

fn theme_handler() -> (ThemeHandler, Rc<RefCell<ThemeObj>>) {
    let recv = Rc::new(RefCell::new(ThemeObj::default()));
    let h = ThemeHandler::ThemeHandler(recv.clone(), None);
    (h, recv)
}

#[test]
fn t01_known_theme_children_kinds() {
    let (mut h, _) = theme_handler();
    assert_eq!(
        h.enterSubElement(1114459).unwrap().kind(),
        "ColorMapHandler"
    );
    assert_eq!(
        h.enterSubElement(1114461).unwrap().kind(),
        "ThemeElementsHandler"
    );
    assert_eq!(
        h.enterSubElement(1114462).unwrap().kind(),
        "ObjectDefaultsHandler"
    );
    assert_eq!(
        h.enterSubElement(1114463).unwrap().kind(),
        "ColorSchemeListHandler"
    );
    assert_eq!(
        h.enterSubElement(1114464).unwrap().kind(),
        "CustomColorListHandler"
    );
}

#[test]
fn t02_unknown_subelement_none() {
    let (mut h, _) = theme_handler();
    assert!(h.enterSubElement(1114468).is_none());
    assert!(h.enterSubElement(1114460).is_none());
}

#[test]
fn t03_name_attr_put_name() {
    let (mut h, _) = theme_handler();
    h.startElement(0, &attrs(&[(1114136, "Office Theme")]));
    assert_eq!(
        h.themeData().borrow().name.as_deref(),
        Some("Office Theme")
    );
}

#[test]
fn t04_object_defaults_dispatch_chain() {
    let (mut h, _) = theme_handler();
    let mut od = h.enterSubElement(1114462).unwrap();
    assert_eq!(od.kind(), "ObjectDefaultsHandler");
    for def in [1114450, 1114451, 1114452] {
        let mut dsd = od.enterSubElement(def).unwrap();
        assert_eq!(dsd.kind(), "DefaultShapeDefinitionHandler");
        assert_eq!(
            dsd.enterSubElement(1114448).unwrap().kind(),
            "BodyPropertiesHandler"
        );
        assert_eq!(
            dsd.enterSubElement(1114449).unwrap().kind(),
            "TextListStyleHandler"
        );
        assert_eq!(
            dsd.enterSubElement(3211578).unwrap().kind(),
            "ShapePropertiesHandler"
        );
        assert_eq!(
            dsd.enterSubElement(3211579).unwrap().kind(),
            "ShapeStyleHandler"
        );
        assert!(dsd.enterSubElement(0).is_none());
    }
}

#[test]
fn t05_cust_clr_lst_save_write() {
    let (mut h, _) = theme_handler();
    let mut lst = h.enterSubElement(1114464).unwrap();
    let mut c1 = lst.enterSubElement(1114139).unwrap();
    assert_eq!(c1.kind(), "CustomColorHandler");
    c1.startElement(0, &attrs(&[(1114136, "accentExtra")]));
    let c2 = lst.enterSubElement(1114139).unwrap();
    assert_eq!(c2.kind(), "CustomColorHandler");
    lst.endElement(0);
    let items = h.themeData().borrow().custom_color_lst.items.clone();
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|c| c.borrow().saved));
    assert_eq!(items[0].borrow().name.as_deref(), Some("accentExtra"));
}

#[test]
fn t06_extra_clr_scheme_lst_and_dispatch() {
    let (mut h, _) = theme_handler();
    let mut lst = h.enterSubElement(1114463).unwrap();
    let mut a = lst.enterSubElement(1114460).unwrap();
    assert_eq!(a.kind(), "ColorSchemeAndMappingHandler");
    assert_eq!(a.enterSubElement(1114154).unwrap().kind(), "ColorSchemeHandler");
    assert_eq!(a.enterSubElement(1114459).unwrap().kind(), "ColorMapHandler");
    let b = lst.enterSubElement(1114460).unwrap();
    assert_eq!(b.kind(), "ColorSchemeAndMappingHandler");
    lst.endElement(0);
    let items = h.themeData().borrow().extra_color_scheme_list.items.clone();
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|c| c.borrow().saved));
}

#[test]
fn t07_callback_null_dispatch() {
    let (mut h, _) = theme_handler();
    assert!(h.enterSubElement(1114459).is_some());
    assert!(h.enterSubElement(1114464).is_some());
}

#[test]
fn t08_end_element_update_protocol() {
    let (mut h, recv) = theme_handler();
    h.startElement(0, &attrs(&[(1114136, "Theme1")]));
    assert!(!recv.borrow().saved);
    h.endElement(0);
    assert!(recv.borrow().saved);
    assert_eq!(recv.borrow().name.as_deref(), Some("Theme1"));
}
