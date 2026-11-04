use std::io::{Cursor, Write};
use std::path::PathBuf;

use ppt_core::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const OFFICE_DOCUMENT_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const APP_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
const CORE_REL: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
const CUSTOM_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties";

fn package_bytes(include_office_document: bool) -> Vec<u8> {
    let mut relationships = Vec::new();
    if include_office_document {
        relationships.push(("rId1", OFFICE_DOCUMENT_REL, "ppt/presentation.xml"));
    }
    relationships.extend([
        ("rId2", APP_REL, "docProps/app.xml"),
        ("rId3", CORE_REL, "docProps/core.xml"),
        ("rId4", CUSTOM_REL, "docProps/custom.xml"),
    ]);
    let relationship_xml = relationships
        .into_iter()
        .map(|(id, relationship_type, target)| {
            format!(
                r#"<Relationship Id="{id}" Type="{relationship_type}" Target="{target}"/>"#
            )
        })
        .collect::<String>();
    let relationship_xml = format!(
        r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{relationship_xml}</Relationships>"#
    );
    let content_types = br#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/></Types>"#;

    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in [
        ("[Content_Types].xml", content_types.as_slice()),
        ("_rels/.rels", relationship_xml.as_bytes()),
        ("ppt/presentation.xml", b"<p:presentation/>".as_slice()),
        ("docProps/app.xml", b"<Properties/>".as_slice()),
        ("docProps/core.xml", b"<cp:coreProperties/>".as_slice()),
        ("docProps/custom.xml", b"<Properties/>".as_slice()),
    ] {
        writer.start_file(name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn temporary_pptx(bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ioffice-pp12-01-{}-{}.pptx",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn part_name(part: Option<String>) -> Option<String> {
    part
}

#[test]
fn t01_open_path_and_locate_office_document() {
    let path = temporary_pptx(&package_bytes(true));
    let mut package = OxfrPackage::OxfrPackage(path.to_str().unwrap()).expect("open path");
    assert_eq!(package.r#type(), 1);
    assert_eq!(
        part_name(package.getOfficeDocument().map(|p| p.getPartName())).as_deref(),
        Some("ppt/presentation.xml")
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn t02_invalid_path_is_an_error_not_an_escaping_panic() {
    let result = OxfrPackage::OxfrPackage("/definitely/missing/ioffice-pp12.pptx");
    assert!(result.is_err());
}

#[test]
fn t03_raf_equivalent_opens_from_a_reader() {
    let mut bytes = Cursor::new(package_bytes(true));
    let mut package = OxfrPackage::OxfrPackage__raf(&mut bytes).expect("open reader");
    assert_eq!(
        part_name(package.getOfficeDocument().map(|p| p.getPartName())).as_deref(),
        Some("ppt/presentation.xml")
    );

    let mut damaged = Cursor::new(b"not-a-zip".to_vec());
    assert!(OxfrPackage::OxfrPackage__raf(&mut damaged).is_err());
}

#[test]
fn t04_missing_office_document_relationship_returns_none() {
    let mut bytes = Cursor::new(package_bytes(false));
    let mut package = OxfrPackage::OxfrPackage__raf(&mut bytes).expect("open reader");
    assert!(package.getOfficeDocument().is_none());
}

#[test]
fn t05_metadata_relationships_locate_the_expected_parts() {
    let mut bytes = Cursor::new(package_bytes(true));
    let mut package = OxfrPackage::OxfrPackage__raf(&mut bytes).expect("open reader");
    assert_eq!(part_name(package.getAppPart().map(|p| p.getPartName())).as_deref(), Some("docProps/app.xml"));
    assert_eq!(part_name(package.getCorePart().map(|p| p.getPartName())).as_deref(), Some("docProps/core.xml"));
    assert_eq!(
        part_name(package.getCustomPart().map(|p| p.getPartName())).as_deref(),
        Some("docProps/custom.xml")
    );
    assert_eq!(package.getRelationships().size(), 4);
}

#[test]
fn t06_close_is_idempotent_and_disables_part_lookup() {
    let mut bytes = Cursor::new(package_bytes(true));
    let mut package = OxfrPackage::OxfrPackage__raf(&mut bytes).expect("open reader");
    package.close().expect("first close");
    package.close().expect("second close");
    assert!(package.getOfficeDocument().is_none());
}
