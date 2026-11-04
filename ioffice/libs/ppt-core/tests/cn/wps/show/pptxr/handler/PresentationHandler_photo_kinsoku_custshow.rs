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

fn parent() -> (PresentationHandler, Rc<RecordingCallback>) {
    let cb = Rc::new(RecordingCallback::default());
    (
        PresentationHandler::PresentationHandler(pres(), HandlerCallbackRef::new(cb.clone())),
        cb,
    )
}

#[test]
fn t01_photo_album_four_attrs() {
    let (mut p, cb) = parent();
    let mut h = p.enterSubElement(3211518).expect("photoAlbum");
    h.startElement(
        3211518,
        &attrs(&[
            (3211484, "true"),
            (3211485, "false"),
            (3211486, "1pic"),
            (3211487, "frameStyle2"),
        ]),
    );
    let album = p.presentation_data().unwrap().photoAlbum.borrow().clone();
    assert_eq!(album.blackAndWhite, Some(true));
    assert_eq!(album.showCaptions, Some(false));
    assert_eq!(album.layout, Some(1));
    assert_eq!(album.frame, Some(1));
}

#[test]
fn t02_kinsoku_three_strings() {
    let (mut p, cb) = parent();
    let mut h = p.enterSubElement(3211520).expect("kinsoku");
    h.startElement(
        3211520,
        &attrs(&[
            (3211490, "zh-CN"),
            (3211491, "abc"),
            (3211492, "xyz"),
        ]),
    );
    let k = p.presentation_data().unwrap().kinsoku.borrow().clone();
    assert_eq!(k.lang.as_deref(), Some("zh-CN"));
    assert_eq!(k.invalStChars.as_deref(), Some("abc"));
    assert_eq!(k.invalEndChars.as_deref(), Some("xyz"));
}

#[test]
fn t03_custshow_rids_callback_and_write() {
    let (mut p, cb) = parent();
    let mut lst = p.enterSubElement(3211517).expect("custShowLst");
    let mut show = lst.enterSubElement(3211443).expect("custShow");
    show.startElement(3211443, &attrs(&[(3211342, "7"), (3211449, "Mine")]));
    let mut sld_lst = show.enterSubElement(3211483).expect("sldLst");
    let mut s1 = sld_lst.enterSubElement(3211440).expect("sld");
    s1.startElement(3211440, &attrs(&[(4259841, "rId1")]));
    let mut s2 = sld_lst.enterSubElement(3211440).expect("sld");
    s2.startElement(3211440, &attrs(&[(4259841, "rId2")]));
    show.endElement(3211443);
    lst.endElement(3211517);

    let calls = cb.on_custom_show.borrow().clone();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, vec!["rId1".to_string(), "rId2".to_string()]);
    assert_eq!(calls[0].1.id, 7);
    assert_eq!(calls[0].1.name, "Mine");
    let written = p.presentation_data().unwrap().customShowList.borrow().items.clone();
    assert_eq!(written.len(), 1);
    assert_eq!(written[0].id, 7);
}

#[test]
fn t04_empty_custshow_list_no_write() {
    let (mut p, cb) = parent();
    let mut lst = p.enterSubElement(3211517).unwrap();
    lst.endElement(3211517);
    assert!(cb.on_custom_show.borrow().is_empty());
    assert!(p.presentation_data().unwrap().customShowList.borrow().items.is_empty());
}

#[test]
fn t05_sld_id_and_rid_paths() {
    let (mut p, cb) = parent();
    let mut lst = p.enterSubElement(3211517).unwrap();
    let mut show = lst.enterSubElement(3211443).unwrap();
    show.startElement(3211443, &attrs(&[(3211342, "1")]));
    let mut sld_lst = show.enterSubElement(3211483).unwrap();
    let mut by_id = sld_lst.enterSubElement(3211440).unwrap();
    by_id.startElement(3211440, &attrs(&[(3211342, "slideA")]));
    let mut by_rid = sld_lst.enterSubElement(3211440).unwrap();
    by_rid.startElement(3211440, &attrs(&[(4259841, "rIdZ")]));
    show.endElement(3211443);
    lst.endElement(3211517);
    let calls = cb.on_custom_show.borrow().clone();
    assert_eq!(calls[0].0, vec!["slideA".to_string(), "rIdZ".to_string()]);
}

#[test]
fn t06_custshow_without_sldlst_no_callback_still_write() {
    let (mut p, cb) = parent();
    let mut lst = p.enterSubElement(3211517).unwrap();
    let mut show = lst.enterSubElement(3211443).unwrap();
    show.startElement(3211443, &attrs(&[(3211342, "3"), (3211449, "NoSlides")]));
    show.endElement(3211443);
    lst.endElement(3211517);
    assert!(cb.on_custom_show.borrow().is_empty());
    let written = p.presentation_data().unwrap().customShowList.borrow().items.clone();
    assert_eq!(written.len(), 1);
    assert_eq!(written[0].name, "NoSlides");
}
