// function_id=11_function_poixml_load
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 11_function_poixml_load

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::xwpf::usermodel::XWPFDocument::XWPFDocument;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::{Cursor, Write};
use std::path::PathBuf;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx")
}

fn open_fixture() -> XWPFPackage {
    let bytes = std::fs::read(fixture_path()).expect("standard-hello.docx");
    let mut input = Cursor::new(bytes);
    XWPFPackage::open_stream(&mut input)
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

#[test]
fn l1_styles_part_attached() {
    let pack = open_fixture();
    let doc = XWPFDocument::XWPFDocument__pkg(pack);
    assert!(
        doc.getStyles().is_some(),
        "standard-hello.docx has word/styles.xml; getStyles must be Some"
    );
    assert!(doc.getImporter().is_none(), "load must not set importer");
}

#[test]
fn l2_numbering_optional() {
    let pack = open_fixture();
    let doc = XWPFDocument::XWPFDocument__pkg(pack);
    assert!(
        doc.getNumbering().is_none(),
        "fixture without numbering rel must leave getNumbering None"
    );
}

#[test]
#[should_panic(expected = "The number of main document relationship should be only 1.")]
fn l3_missing_office_document_fails() {
    let types = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
</Types>"#
    );
    let rels = r#"<?xml version="1.0"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>"#;
    let bytes = zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", rels.as_bytes()),
    ]);
    let mut input = Cursor::new(bytes);
    let pack = XWPFPackage::open_stream(&mut input);
    let _ = XWPFDocument::XWPFDocument__pkg(pack);
}
