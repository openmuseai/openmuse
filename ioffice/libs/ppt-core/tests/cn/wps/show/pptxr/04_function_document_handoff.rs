use std::io::{Cursor, Write};

use ppt_core::cn::wps::show::pptxr::PptxReader::{open_pptx_bytes, open_pptx_document_bytes};
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

fn two_page_pptx() -> Vec<u8> {
    zip_pptx(&[
        ("[Content_Types].xml", "<Types/>"),
        ("_rels/.rels", PACKAGE_RELS),
        ("ppt/presentation.xml", r#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldSz cx="12192000" cy="6858000"/><p:sldIdLst><p:sldId r:id="rIdSecond"/><p:sldId r:id="rIdFirst"/></p:sldIdLst></p:presentation>"#),
        ("ppt/_rels/presentation.xml.rels", r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdFirst" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/><Relationship Id="rIdSecond" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/></Relationships>"#),
        ("ppt/slides/slide1.xml", r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:t>First</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sld>"#),
        ("ppt/slides/slide2.xml", r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:t>Second</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sld>"#),
    ])
}

#[test]
fn t01_sample_ppt_exact_23() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx"
    );
    let bytes = std::fs::read(path).expect("Sample_ppt.pptx");
    let document = open_pptx_document_bytes(&bytes).unwrap();
    assert_eq!(document.slides.len(), 23);
    assert_eq!(document.slide_names.len(), 23);
    assert_eq!(document.slides.len(), document.slide_names.len());
}

#[test]
fn t02_rid_reverse_order() {
    let document = open_pptx_document_bytes(&two_page_pptx()).unwrap();
    assert_eq!(
        document.slide_names,
        vec!["ppt/slides/slide2.xml", "ppt/slides/slide1.xml"]
    );
    assert_eq!(document.slide_width, 12_192_000);
    assert_eq!(document.slide_height, 6_858_000);
}

#[test]
fn t03_empty_slide_list_errors() {
    let bytes = zip_pptx(&[
        ("[Content_Types].xml", "<Types/>"),
        ("_rels/.rels", PACKAGE_RELS),
        ("ppt/presentation.xml", r#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:sldSz cx="1" cy="1"/></p:presentation>"#),
    ]);
    // `PptxDocument` is not `Debug`, so `unwrap_err()` cannot be used: match the
    // error out directly, keeping the same requirement that opening must fail.
    let err = match open_pptx_document_bytes(&bytes) {
        Ok(_) => panic!("expected an error for an empty slide list"),
        Err(err) => err,
    };
    assert!(err.contains("SuffixError"), "{err}");
}

#[test]
fn t04_compat_adapter_returns_first_page() {
    let document = open_pptx_document_bytes(&two_page_pptx()).unwrap();
    let first = open_pptx_bytes(&two_page_pptx()).unwrap();
    assert_eq!(
        first.sp_tree.children[0].text_op.as_ref().unwrap().paras[0].runs[0].text,
        document.slides[0].sp_tree.children[0]
            .text_op
            .as_ref()
            .unwrap()
            .paras[0]
            .runs[0]
            .text
    );
}
