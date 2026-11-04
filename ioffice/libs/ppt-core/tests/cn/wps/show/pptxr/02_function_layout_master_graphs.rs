#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::{visual_pptx, zip_pptx_str, presentation_xml, presentation_rels};

#[test]
fn t01_layout_and_master_trees_parsed() {
    let doc = open_pptx_document_bytes(&visual_pptx(
        r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr></p:sp>"#,
        true,
        false,
        false,
    ))
    .unwrap();
    let layout = doc.slides[0].layout.as_ref().expect("layout bound");
    let tree = layout.sp_tree.as_ref().expect("layout tree");
    assert!(!tree.children.is_empty());
    let master = layout.master.as_ref().expect("master bound");
    assert!(!master.sp_tree.children.is_empty());
}

#[test]
fn t02_missing_layout_xml_does_not_drop_slide() {
    let bytes = zip_pptx_str(&[
        ("[Content_Types].xml", "<Types/>"),
        ("ppt/presentation.xml", &presentation_xml()),
        ("ppt/_rels/presentation.xml.rels", &presentation_rels("")),
        (
            "ppt/slides/slide1.xml",
            r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp></p:spTree></p:cSld></p:sld>"#,
        ),
        (
            "ppt/slides/_rels/slide1.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdL" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout9.xml"/></Relationships>"#,
        ),
    ]);
    let doc = open_pptx_document_bytes(&bytes).unwrap();
    assert_eq!(doc.slides.len(), 1);
    assert!(doc.slides[0].layout.is_none());
}

#[test]
fn t03_empty_layout_tree_does_not_panic() {
    let bytes = zip_pptx_str(&[
        ("[Content_Types].xml", "<Types/>"),
        ("ppt/presentation.xml", &presentation_xml()),
        ("ppt/_rels/presentation.xml.rels", &presentation_rels("")),
        (
            "ppt/slides/slide1.xml",
            r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp></p:spTree></p:cSld></p:sld>"#,
        ),
        (
            "ppt/slides/_rels/slide1.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdL" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/></Relationships>"#,
        ),
        (
            "ppt/slideLayouts/slideLayout1.xml",
            r#"<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:cSld><p:spTree></p:spTree></p:cSld></p:sldLayout>"#,
        ),
    ]);
    let doc = open_pptx_document_bytes(&bytes).unwrap();
    let layout = doc.slides[0].layout.as_ref().unwrap();
    assert!(layout.sp_tree.as_ref().unwrap().children.is_empty());
}
