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

fn three_arg() -> PresentationHandler {
    PresentationHandler::PresentationHandler(
        pres(),
        HandlerCallbackRef::new(Rc::new(RecordingCallback::default())),
    )
}

#[test]
fn t01_twelve_presentation_attrs() {
    let mut h = three_arg();
    h.startElement(
        3211535,
        &attrs(&[
            (3211523, "50"),
            (3211524, "2"),
            (3211525, "true"),
            (3211526, "true"),
            (3211527, "1"),
            (3211528, "true"),
            (3211529, "false"),
            (3211530, "true"),
            (3211531, "on"),
            (3211532, "true"),
            (3211533, "7"),
            (3211534, "strict"),
        ]),
    );
    let d = h.presentation_data().unwrap();
    assert_eq!(d.serverZoom, Some(50.0));
    assert_eq!(d.firstSlideNum, Some(2));
    assert_eq!(d.showSpecialPlsOnTitleSld, Some(true));
    assert_eq!(d.rightToLeft, Some(true));
    assert_eq!(d.removePersonalInfoOnSave, Some(true));
    assert_eq!(d.compatMode, Some(true));
    assert_eq!(d.strictFirstAndLastChars, Some(false));
    assert_eq!(d.embedTrueTypeFonts, Some(true));
    assert_eq!(d.saveSubsetFonts, Some(true));
    assert_eq!(d.autoCompressPictures, Some(true));
    assert_eq!(d.bookmardIdSeed, Some(7));
    assert_eq!(d.conformace, Some(0));
}

#[test]
fn t02_conformance_miss_defaults_one() {
    let mut h = three_arg();
    h.startElement(3211535, &attrs(&[(3211534, "picky")]));
    assert_eq!(h.presentation_data().unwrap().conformace, Some(1));
}

#[test]
fn t03_empty_attrs_keep_create_defaults() {
    let mut h = three_arg();
    h.startElement(3211535, &attrs(&[]));
    assert_eq!(
        h.presentation_data().unwrap(),
        &ppt_core::cn::wps::show::pptxr::handler::PresentationHandler::Presentation::default()
    );
    h.endElement(3211535);
    // rust: `last_update_protocol()` 是端口可观测面 —— 反映 `KmoPresentation` 装配面快照
    assert!(h.last_update_protocol().unwrap().starts_with("slideSize:"));
}

#[test]
fn t04_callback_ctor_end_skips_update() {
    let mut h = PresentationHandler::PresentationHandler__callback(HandlerCallbackRef::new(Rc::new(
        RecordingCallback::default(),
    )));
    h.endElement(3211535);
    assert!(h.last_update_protocol().is_none());
}

#[test]
fn t05_three_arg_end_updates_protocol() {
    let mut h = three_arg();
    h.endElement(0);
    let payload = h.last_update_protocol().expect("updateProtocol");
    assert!(!payload.is_empty());
}
