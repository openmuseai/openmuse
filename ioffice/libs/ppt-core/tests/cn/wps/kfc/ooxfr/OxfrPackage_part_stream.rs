use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use ppt_core::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const OFFICE_DOCUMENT_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

const SLIDE_BYTES: &[u8] = b"<p:sld/>";

fn pptx_bytes() -> Vec<u8> {
    let relationship_xml = format!(
        r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="{OFFICE_DOCUMENT_REL}" Target="ppt/presentation.xml"/></Relationships>"#
    );
    let content_types = br#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/></Types>"#;
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    writer.start_file("[Content_Types].xml", options).unwrap();
    writer.write_all(content_types).unwrap();
    writer.start_file("_rels/.rels", options).unwrap();
    writer.write_all(relationship_xml.as_bytes()).unwrap();
    writer.start_file("ppt/presentation.xml", options).unwrap();
    writer.write_all(b"<p:presentation/>").unwrap();
    writer.start_file("ppt/slides/slide1.xml", options).unwrap();
    writer.write_all(SLIDE_BYTES).unwrap();
    writer.finish().unwrap().into_inner()
}

fn open_pack() -> OxfrPackage {
    let bytes = pptx_bytes();
    let path = PathBuf::from(format!(
        "{}/ioffice-pp12-03-{}-{}.pptx",
        std::env::temp_dir().display(),
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&path, bytes).unwrap();
    OxfrPackage::OxfrPackage(path.to_str().unwrap()).expect("open")
}

#[test]
fn t01_decompressed_stream_matches_entry() {
    let mut pack = open_pack();
    let part = pack.getOfficeDocument().expect("pres");
    let mut stream = OxfrPackage::getInputStream_part(part, false).expect("stream");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();
    assert_eq!(buf, b"<p:presentation/>");
}

#[test]
fn t02_default_is_origin_false() {
    let mut pack = open_pack();
    let part = pack.getOfficeDocument().expect("pres");
    let a = OxfrPackage::getInputStream_part(part, false).unwrap();
    let b = part.getInputStream_default().expect("default");
    let mut ba = Vec::new();
    let mut bb = Vec::new();
    let mut a = a;
    let mut b = b;
    a.read_to_end(&mut ba).unwrap();
    b.read_to_end(&mut bb).unwrap();
    assert_eq!(ba, bb);
}

#[test]
fn t03_missing_entry_err() {
    let mut pack = open_pack();
    assert!(pack.getCustomPart().is_none());
}

#[test]
fn t04_empty_entry_ok() {
    let relationship_xml = format!(
        r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="{OFFICE_DOCUMENT_REL}" Target="ppt/presentation.xml"/></Relationships>"#
    );
    let content_types = br#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/></Types>"#;
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    writer.start_file("[Content_Types].xml", options).unwrap();
    writer.write_all(content_types).unwrap();
    writer.start_file("_rels/.rels", options).unwrap();
    writer.write_all(relationship_xml.as_bytes()).unwrap();
    writer.start_file("ppt/presentation.xml", options).unwrap();
    writer.write_all(b"").unwrap();
    let bytes = writer.finish().unwrap().into_inner();
    let path = PathBuf::from(format!(
        "{}/ioffice-pp12-03-empty-{}-{}.pptx",
        std::env::temp_dir().display(),
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&path, bytes).unwrap();
    let mut pack = OxfrPackage::OxfrPackage(path.to_str().unwrap()).expect("open");
    let part = pack.getOfficeDocument().expect("pres");
    let mut stream = OxfrPackage::getInputStream_part(part, false).expect("stream");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();
    assert_eq!(buf, b"");
}

#[test]
fn t05_part_name() {
    let mut pack = open_pack();
    let part = pack.getOfficeDocument().expect("pres");
    assert!(part.getPartName().contains("ppt"));
}

#[test]
fn t02b_is_origin_true_reads() {
    let mut pack = open_pack();
    let part = pack.getOfficeDocument().expect("pres");
    let mut stream = OxfrPackage::getInputStream_part(part, true).expect("origin");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();
    assert!(!buf.is_empty());
}
