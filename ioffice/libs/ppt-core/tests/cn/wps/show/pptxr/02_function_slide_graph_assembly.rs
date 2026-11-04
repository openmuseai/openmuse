use std::io::{Cursor, Write};

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const PACKAGE_RELS: &str = r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdPkg" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/></Relationships>"#;

fn zip_pptx(files: &[(&str, &str)]) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut z = ZipWriter::new(&mut buf);
        let opt = SimpleFileOptions::default();
        for (name, body) in files {
            z.start_file(*name, opt).unwrap();
            z.write_all(body.as_bytes()).unwrap();
        }
        z.finish().unwrap();
    }
    buf.into_inner()
}

fn slide_xml(kinds: &str) -> String {
    format!(
        r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><p:cSld><p:spTree>{kinds}</p:spTree></p:cSld></p:sld>"#
    )
}

fn two_page_pptx() -> Vec<u8> {
    zip_pptx(&[
        ("[Content_Types].xml", r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#),
        ("_rels/.rels", PACKAGE_RELS),
        ("ppt/presentation.xml", r#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldSz cx="12192000" cy="6858000"/><p:sldIdLst><p:sldId id="256" r:id="rIdSecond"/><p:sldId id="257" r:id="rIdFirst"/></p:sldIdLst></p:presentation>"#),
        ("ppt/_rels/presentation.xml.rels", r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdFirst" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/><Relationship Id="rIdSecond" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/></Relationships>"#),
        ("ppt/slides/slide1.xml", &slide_xml(r#"<p:sp><p:spPr><a:xfrm><a:off x="1" y="1"/><a:ext cx="100" cy="100"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:t>First</a:t></a:r></a:p></p:txBody></p:sp>"#)),
        ("ppt/slides/slide2.xml", &slide_xml(r#"<p:sp><p:spPr><a:xfrm><a:off x="2" y="2"/><a:ext cx="100" cy="100"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:t>Second</a:t></a:r></a:p></p:txBody></p:sp>"#)),
    ])
}

fn mixed_tree_pptx() -> Vec<u8> {
    zip_pptx(&[
        ("[Content_Types].xml", "<Types/>"),
        ("_rels/.rels", PACKAGE_RELS),
        ("ppt/presentation.xml", r#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldSz cx="1" cy="1"/><p:sldIdLst><p:sldId r:id="rId1"/></p:sldIdLst></p:presentation>"#),
        ("ppt/_rels/presentation.xml.rels", r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/></Relationships>"#),
        ("ppt/slides/slide1.xml", &slide_xml(
            r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:sp>
<p:pic><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>
<p:cxnSp><p:spPr><a:xfrm><a:off x="2" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:cxnSp>
<p:grpSp><p:grpSpPr><a:xfrm><a:off x="3" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:grpSpPr><p:sp><p:spPr><a:xfrm><a:off x="4" y="0"/><a:ext cx="5" cy="5"/></a:xfrm></p:spPr></p:sp></p:grpSp>
<p:graphicFrame><p:xfrm><a:off x="5" y="0"/><a:ext cx="10" cy="10"/></p:xfrm></p:graphicFrame>"#,
        )),
    ])
}

fn page_text(document: &ppt_core::cn::wps::show::pptxr::PptxReader::PptxDocument, i: usize) -> String {
    document.slides[i].sp_tree.children[0]
        .text_op
        .as_ref()
        .unwrap()
        .paras[0]
        .runs[0]
        .text
        .clone()
}

#[test]
fn t01_two_page_order_and_text() {
    let document = open_pptx_document_bytes(&two_page_pptx()).unwrap();
    assert_eq!(document.slides.len(), document.slide_names.len());
    assert_eq!(
        document.slide_names,
        vec!["ppt/slides/slide2.xml", "ppt/slides/slide1.xml"]
    );
    assert_eq!(page_text(&document, 0), "Second");
    assert_eq!(page_text(&document, 1), "First");
}

#[test]
fn t02_sample_ppt_23_pages() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx"
    );
    let bytes = std::fs::read(path).expect("Sample_ppt.pptx");
    let document = open_pptx_document_bytes(&bytes).unwrap();
    assert_eq!(document.slides.len(), 23);
    assert_eq!(document.slide_names.len(), 23);
}

#[test]
fn t03_missing_slide_part_errors() {
    let bytes = zip_pptx(&[
        ("[Content_Types].xml", "<Types/>"),
        ("_rels/.rels", PACKAGE_RELS),
        ("ppt/presentation.xml", r#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldSz cx="1" cy="1"/><p:sldIdLst><p:sldId r:id="rId1"/></p:sldIdLst></p:presentation>"#),
        ("ppt/_rels/presentation.xml.rels", r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide9.xml"/></Relationships>"#),
    ]);
    // `PptxDocument` is not `Debug`, so `unwrap_err()` cannot be used: match the
    // error out directly, keeping the same requirement that opening must fail.
    let err = match open_pptx_document_bytes(&bytes) {
        Ok(_) => panic!("expected an error for a missing slide part"),
        Err(err) => err,
    };
    assert!(err.contains("slide9"), "{err}");
}

#[test]
fn t04_pic_cxn_grp_graphic_dispatch() {
    let document = open_pptx_document_bytes(&mixed_tree_pptx()).unwrap();
    let kids = &document.slides[0].sp_tree.children;
    assert_eq!(kids.len(), 5);
    assert_eq!(kids[0].shape_type, 1);
    assert_eq!(kids[1].shape_type, 2);
    assert_eq!(kids[2].shape_type, 3);
    assert_eq!(kids[3].shape_type, 0);
    assert_eq!(kids[3].children.len(), 1);
    assert_eq!(kids[4].shape_type, 4);
}
