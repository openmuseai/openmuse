//! 03_function_open_ln_survives — open 后 dgShape.spPr().ln() 仍可查

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use std::cell::RefCell;
use std::rc::Rc;
use visual_zip::visual_pptx;

fn ln_w_xml() -> &'static str {
    r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm><a:ln w="25400"/></p:spPr></p:sp>"#
}

fn ln_nofill_xml() -> &'static str {
    r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm><a:ln w="25400"><a:noFill/></a:ln></p:spPr></p:sp>"#
}

fn ln_miss_xml() -> &'static str {
    r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp>"#
}

/// Java: `Shape.open(kmo.drawingShape()).spPr()` after open.
fn open_dg_sp_pr(xml: &str) -> (ppt_core::cn::wps::show::pptxr::PptxReader::PptxDocument, Rc<RefCell<ShapePr>>) {
    let doc = open_pptx_document_bytes(&visual_pptx(xml, false, false, false)).unwrap();
    let pr = {
        let mut film = doc.films[0].borrow_mut();
        let tree = film.spTree();
        assert_eq!(tree.shapeCount(), 1);
        let child = tree.shapeAt(0);
        let store = unsafe { (*child.parent().unwrap()).getPresentation().drawingStore() };
        let store = unsafe { &*store };
        store
            .shapePr(child.id())
            .expect("dgShape.spPr() after CommonSlideDataHandler / SwShapeHandler")
    };
    (doc, pr)
}

#[test]
fn ln_w() {
    let (_doc, pr) = open_dg_sp_pr(ln_w_xml());
    let mut pr = pr.borrow_mut();
    assert!(pr.hasLn(), "a:ln w=25400 → hasLn");
    assert_eq!(pr.ln().get_width(), 25400, "ln 宽度 == 25400 EMU");
}

#[test]
fn ln_nf() {
    let (_doc, pr) = open_dg_sp_pr(ln_nofill_xml());
    let mut pr = pr.borrow_mut();
    assert!(pr.hasLn(), "a:ln/a:noFill still has ln object");
    // Java Outline.fill().type()==5 (noFill) → ConvertTool 不产 stroke / getfLine false
    assert_eq!(
        pr.ln().fill().r#type(),
        5,
        "noFill → fill.type==5 ≡ getfLine false"
    );
}

#[test]
fn ln_miss() {
    let (_doc, pr) = open_dg_sp_pr(ln_miss_xml());
    assert!(
        !pr.borrow().hasLn(),
        "spPr 仅 xfrm → hasLn==false，不捏造默认黑线"
    );
}
