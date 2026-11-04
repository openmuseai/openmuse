use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::data::text::TextBody::TextBody;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextBodyHandler::TextBodyHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;

fn empty_attrs() -> XmlRoAttrList {
    XmlRoAttrList::new()
}

fn drive_regular_run(p: &mut dyn ElementHandler, text: Option<&str>, rpr: Option<&XmlRoAttrList>) {
    let mut run = p.enterSubElement(1114193).expect("r");
    run.startElement(1114193, &empty_attrs());
    if let Some(chars) = text {
        let mut t = run.enterSubElement(1114214).expect("t");
        t.characters(chars);
        t.endElement(1114214);
    }
    if let Some(attrs) = rpr {
        let mut rp = run.enterSubElement(1114603).expect("rPr");
        rp.startElement(1114603, attrs);
        rp.endElement(1114603);
    }
    run.endElement(1114193);
}

#[test]
fn t01_three_paragraphs_including_empty() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    for i in 0..3 {
        let mut p = h.enterSubElement(1114562).expect("p");
        if i != 1 {
            drive_regular_run(&mut *p, Some("x"), None);
        }
        p.endElement(1114562);
    }
    h.endElement(1114561);
    assert_eq!(body.borrow_mut().paraLst().size(), 3);
    let paras = h.surface_paras();
    assert_eq!(paras.len(), 3);
    assert_eq!(paras[0].borrow().runs.len(), 1);
    assert_eq!(paras[1].borrow().runs.len(), 0);
    assert_eq!(paras[2].borrow().runs.len(), 1);
}

#[test]
fn t02_run_modes_r_br_fld_and_fld_without_id() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    let mut p = h.enterSubElement(1114562).expect("p");

    drive_regular_run(&mut *p, Some("a"), None);

    let mut br = p.enterSubElement(1114618).expect("br");
    br.startElement(1114618, &empty_attrs());
    br.endElement(1114618);

    let mut fld_attrs = XmlRoAttrList::new();
    fld_attrs.add_attr(1114245, "fld1");
    fld_attrs.add_attr(1114388, "datetime");
    let mut fld = p.enterSubElement(1114619).expect("fld");
    fld.startElement(1114619, &fld_attrs);
    let mut t = fld.enterSubElement(1114214).expect("fld t");
    t.characters("1");
    t.endElement(1114214);
    fld.endElement(1114619);

    let mut fld_no_id = p.enterSubElement(1114619).expect("fld no id");
    fld_no_id.startElement(1114619, &empty_attrs());
    fld_no_id.endElement(1114619);

    p.endElement(1114562);
    h.endElement(1114561);

    let runs = h.surface_paras()[0].borrow().runs.clone();
    assert_eq!(runs.len(), 4);
    assert_eq!(runs[0].borrow().mode, 0);
    assert_eq!(runs[1].borrow().mode, 1);
    assert_eq!(runs[1].borrow().length, 1);
    assert_eq!(runs[2].borrow().mode, 2);
    assert_eq!(runs[2].borrow().field_id.as_deref(), Some("fld1"));
    assert_eq!(runs[2].borrow().field_type.as_deref(), Some("datetime"));
    assert_ne!(runs[3].borrow().mode, 2);
}

#[test]
fn t03_characters_preserve_whitespace_and_newline() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    let mut p = h.enterSubElement(1114562).expect("p");
    let text = " 甲\n乙 ";
    drive_regular_run(&mut *p, Some(text), None);
    p.endElement(1114562);
    h.endElement(1114561);
    let run = h.surface_paras()[0].borrow().runs[0].clone();
    assert_eq!(run.borrow().regular_text, text);
    assert_eq!(run.borrow().length, text.len() as i32);
}

#[test]
fn t04_rpr_min_set_and_underline_map() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    let mut p = h.enterSubElement(1114562).expect("p");
    let mut attrs = XmlRoAttrList::new();
    attrs.add_attr(1114589, "1800");
    attrs.add_attr(1114195, "true");
    attrs.add_attr(1114504, "true");
    attrs.add_attr(1114590, "sng");
    attrs.add_attr(1114587, "zh-CN");
    attrs.add_attr(1114595, "30000");
    drive_regular_run(&mut *p, Some("a"), Some(&attrs));

    let mut miss = XmlRoAttrList::new();
    miss.add_attr(1114590, "not-a-underline");
    drive_regular_run(&mut *p, Some("b"), Some(&miss));

    p.endElement(1114562);
    h.endElement(1114561);

    let r0 = h.surface_paras()[0].borrow().runs[0].clone();
    let rpr = r0.borrow().rpr.borrow().clone();
    assert_eq!(rpr.fontSize, Some(1800));
    assert_eq!(rpr.options.bold, Some(true));
    assert_eq!(rpr.options.italics, Some(true));
    assert_eq!(rpr.underline, Some(13));
    assert_eq!(rpr.lang.as_deref(), Some("zh-CN"));
    assert_eq!(rpr.baseline, Some(30000.0 / 100000.0));

    let r1 = h.surface_paras()[0].borrow().runs[1].clone();
    assert_eq!(r1.borrow().rpr.borrow().underline, None);
}

#[test]
fn t05_gvml_merge_callback_timing() {
    let body = Rc::new(RefCell::new(TextBody::create()));
    let mut h = TextBodyHandler::TextBodyHandler(body.clone(), None);
    let mut p = h.enterSubElement(1114562).expect("p");
    drive_regular_run(&mut *p, Some("a"), None);
    p.endElement(1114562);
    h.endElement(1114561);

    let gvml = Rc::new(RefCell::new(HandlerCallback::gvml()));
    let body2 = Rc::new(RefCell::new(TextBody::create()));
    let mut h2 = TextBodyHandler::TextBodyHandler(body2.clone(), Some(gvml.clone()));
    let mut p2 = h2.enterSubElement(1114562).expect("p");
    drive_regular_run(&mut *p2, Some("a"), None);
    drive_regular_run(&mut *p2, Some("b"), None);
    p2.endElement(1114562);
    h2.endElement(1114561);
    assert_eq!(gvml.borrow().merge_count.get(), 2);

    let pptx = Rc::new(RefCell::new(HandlerCallback::pptx()));
    let body3 = Rc::new(RefCell::new(TextBody::create()));
    let mut h3 = TextBodyHandler::TextBodyHandler(body3, Some(pptx.clone()));
    let mut p3 = h3.enterSubElement(1114562).expect("p");
    drive_regular_run(&mut *p3, Some("a"), None);
    p3.endElement(1114562);
    h3.endElement(1114561);
    assert_eq!(pptx.borrow().merge_count.get(), 0);
}
