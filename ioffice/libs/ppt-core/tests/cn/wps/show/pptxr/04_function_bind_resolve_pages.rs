#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::{visual_pptx, zip_pptx_str, presentation_xml, presentation_rels};

#[test]
fn t01_bind_layout_then_inherit_xfrm() {
    let slide = r#"<p:sp><p:nvSpPr><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, true, false, false)).unwrap();
    assert!(doc.slides[0].layout.is_some());
    let sp = &doc.slides[0].sp_tree.children[0];
    assert!(sp.is_placeholder);
    assert_eq!(sp.transform2DX, 100);
    assert_eq!(sp.transform2DY, 200);
}

#[test]
fn t02_missing_layout_still_opens() {
    let doc = open_pptx_document_bytes(&visual_pptx(
        r#"<p:sp><p:spPr><a:xfrm><a:off x="3" y="4"/><a:ext cx="5" cy="6"/></a:xfrm></p:spPr></p:sp>"#,
        false,
        false,
        false,
    ))
    .unwrap();
    assert!(doc.slides[0].layout.is_none());
    assert_eq!(doc.slides.len(), 1);
}

#[test]
fn t03_bind_does_not_shorten_page_count() {
    let bytes = zip_pptx_str(&[
        ("[Content_Types].xml", "<Types/>"),
        (
            "ppt/presentation.xml",
            r#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldSz cx="1" cy="1"/><p:sldIdLst><p:sldId r:id="rId1"/><p:sldId r:id="rId2"/></p:sldIdLst></p:presentation>"#,
        ),
        (
            "ppt/_rels/presentation.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/></Relationships>"#,
        ),
        (
            "ppt/slides/slide1.xml",
            r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp></p:spTree></p:cSld></p:sld>"#,
        ),
        (
            "ppt/slides/slide2.xml",
            r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp></p:spTree></p:cSld></p:sld>"#,
        ),
    ]);
    let _ = presentation_xml();
    let _ = presentation_rels("");
    let doc = open_pptx_document_bytes(&bytes).unwrap();
    assert_eq!(doc.slides.len(), 2);
    assert_eq!(doc.slide_names.len(), 2);
}
