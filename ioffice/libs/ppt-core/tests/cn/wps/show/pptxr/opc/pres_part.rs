use std::io::{Cursor, Write};
use std::path::PathBuf;

use ppt_core::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ppt_core::cn::wps::show::pptxr::PptxReader::{getPresPart, PresPartError};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const OFFICE_DOCUMENT_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

fn zip_with_offdoc(target: Option<&str>) -> Vec<u8> {
    let relationship_xml = match target {
        Some(t) => format!(
            r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="{OFFICE_DOCUMENT_REL}" Target="{t}"/></Relationships>"#
        ),
        None => r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"></Relationships>"#.into(),
    };
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
    writer.start_file("word/document.xml", options).unwrap();
    writer.write_all(b"<w:document/>").unwrap();
    writer.finish().unwrap().into_inner()
}

fn open_pack(bytes: &[u8]) -> OxfrPackage {
    let path = PathBuf::from(format!(
        "{}/ioffice-pp12-02-{}-{}.pptx",
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
fn t01_missing_office_document_file_damaged() {
    let mut pack = open_pack(&zip_with_offdoc(None));
    match getPresPart(&mut pack) {
        Err(PresPartError::FileDamaged(s)) => assert_eq!(s, "Not OOXML file"),
        Err(PresPartError::SuffixError(_)) => panic!("expected FileDamaged"),
        Ok(_) => panic!("expected FileDamaged"),
    }
}

#[test]
fn t02_legal_pres_part() {
    let mut pack = open_pack(&zip_with_offdoc(Some("ppt/presentation.xml")));
    let part = getPresPart(&mut pack).expect("ok");
    assert!(part.getPartName().contains("ppt"));
}

#[test]
fn t03_word_part_suffix_error() {
    let mut pack = open_pack(&zip_with_offdoc(Some("word/document.xml")));
    match getPresPart(&mut pack) {
        Err(PresPartError::SuffixError(s)) => assert_eq!(s, "Not pptx file"),
        Err(PresPartError::FileDamaged(_)) => panic!("expected SuffixError"),
        Ok(_) => panic!("expected SuffixError"),
    }
}

#[test]
fn t04_messages_match_java() {
    assert_eq!(
        format!("{}", {
            match PresPartError::FileDamaged("Not OOXML file") {
                PresPartError::FileDamaged(s) => s,
                _ => "",
            }
        }),
        "Not OOXML file"
    );
    assert_eq!(
        match PresPartError::SuffixError("Not pptx file") {
            PresPartError::SuffixError(s) => s,
            _ => "",
        },
        "Not pptx file"
    );
}
