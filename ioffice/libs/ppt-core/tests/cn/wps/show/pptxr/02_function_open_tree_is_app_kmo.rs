//! 02_function_open_tree_is_app_kmo — open 产物是 app::KmoShapeTree

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData;
use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::visual_pptx;

fn one_sp_xml() -> &'static str {
    r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp>"#
}

fn grp_one_child_xml() -> &'static str {
    r#"<p:grpSp><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp></p:grpSp>"#
}

#[test]
fn ot_cnt() {
    let doc = open_pptx_document_bytes(&visual_pptx(one_sp_xml(), false, false, false)).unwrap();
    let mut film = doc.films[0].borrow_mut();
    assert_eq!(
        film.spTree().shapeCount(),
        1,
        "Java film.spTree().shapeCount() after CommonSlideDataHandler.endElement"
    );
}

#[test]
fn ot_typ() {
    let doc = open_pptx_document_bytes(&visual_pptx(one_sp_xml(), false, false, false)).unwrap();
    let mut film = doc.films[0].borrow_mut();
    let tree = film.spTree();
    assert_eq!(tree.shapeCount(), 1);
    let child = tree.shapeAt(0);
    assert_eq!(
        child.r#type(),
        DgStoreData::queryType(child.id()),
        "type()==DgStore.queryType(id)"
    );
}

#[test]
fn ot_grp() {
    let doc = open_pptx_document_bytes(&visual_pptx(grp_one_child_xml(), false, false, false)).unwrap();
    let mut film = doc.films[0].borrow_mut();
    let tree = film.spTree();
    assert_eq!(tree.shapeCount(), 1, "spTree has the grp child");
    let grp = tree.shapeAt(0);
    assert_eq!(grp.r#type(), 0);
    assert_eq!(grp.childCount(), 1, "grp+1 子 → childCount==1");
}

#[test]
fn ot_grep() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cn/wps/show/pptxr/pptx_mvp.rs"
    ));
    let csld = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cn/wps/show/pptxr/handler/slides/CommonSlideDataHandler.rs"
    ));
    assert!(
        src.contains("CommonSlideDataHandler") && csld.contains("initRoot"),
        "open path must call Java CommonSlideDataHandler; endElement initRoot"
    );
    let start = src
        .find("pub fn open_pptx_document_bytes")
        .expect("open_pptx_document_bytes");
    let rest = &src[start..];
    let end = rest.find("\npub fn open_pptx_bytes").unwrap_or(rest.len());
    let body = &rest[..end];
    assert!(
        !body.contains("layout_shape::KmoSlide"),
        "open return path must not name layout_shape::KmoSlide"
    );
}
