// function_id=07_function_opc_package
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 07_function_opc_package
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/OPCPackage.rs

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::OPCPackage::{OPCPackage, DOCUMENT_CREATOR};
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::PackageRelationshipTypes::PackageRelationshipTypes;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::io::{Cursor, Write};
use std::path::PathBuf;

const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const XML_CT: &str = "application/xml";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str = PackageRelationshipTypes::CORE_DOCUMENT;

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="{RELS_CT}"/>
  <Default Extension="xml" ContentType="{XML_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
</Types>"#
    )
}

fn package_rels_xml() -> String {
    format!(
        r#"<?xml version="1.0"?>
<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
</Relationships>"#
    )
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

fn min_docx_bytes() -> Vec<u8> {
    let types = types_xml();
    let rels = package_rels_xml();
    zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", rels.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ])
}

fn pkg_from_bytes(bytes: Vec<u8>) -> OPCPackage {
    let mut input = Cursor::new(bytes);
    OPCPackage::from_stream(&mut input)
}

fn doc_name() -> core_libs_io::io::opc::PackagePartName::PackagePartName {
    PackagingURIHelper::createPartName_str("/word/document.xml").unwrap()
}

fn write_temp_docx() -> PathBuf {
    // ★修复既有 flake（本轮实测命中 1 次 `o4_from_path_roundtrip` 失败，重跑 10/10 通过）：
    //   原实现只用**纳秒时间戳**做文件名 ⇒ 并行测试线程在同纳秒内可能拿到**同一路径**，
    //   彼此 `remove_file` 互相删掉对方的临时文件。
    //   这里加入**线程 id** *并且* 用 `create_new` 保证路径唯一（碰撞则换名重试）。
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    loop {
        let path = std::env::temp_dir().join(format!(
            "ioffice-opc-07-{}-{}-{}.docx",
            std::process::id(),
            SEQ.fetch_add(1, Ordering::Relaxed),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                use std::io::Write;
                f.write_all(&min_docx_bytes()).unwrap();
                return path;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => panic!("写临时 docx 失败: {}", e),
        }
    }
}

#[test]
fn o1_main_document_part_exists() {
    let mut pkg = pkg_from_bytes(min_docx_bytes());
    pkg.getParts();
    let part = pkg.getPart(&doc_name()).expect("main document");
    assert!(
        part.getContentType().toString().contains("wordprocessingml.document.main"),
        "{}",
        part.getContentType().toString()
    );
}

#[test]
fn o2_office_document_rel_resolves() {
    let mut pkg = pkg_from_bytes(min_docx_bytes());
    let rels = pkg.getRelationshipsByType(OFFICE_DOC);
    assert_eq!(rels.size(), 1);
    let rel = rels.getRelationship_by_index(0).unwrap();
    assert!(
        rel.getTargetURI().toString().contains("word/document.xml"),
        "{}",
        rel.getTargetURI().toString()
    );
}

#[test]
fn o3_missing_content_types_fails_open() {
    let bytes = zip_bytes(&[("word/document.xml", b"<w:document/>")]);
    let mut pkg = pkg_from_bytes(bytes);
    pkg.getParts();
    assert!(pkg.getPart(&doc_name()).is_none());
}

#[test]
fn o4_from_path_roundtrip() {
    let path = write_temp_docx();
    let mut pkg = OPCPackage::from_path(path.to_str().unwrap(), PackageAccess::READ);
    let part = pkg.getPart(&doc_name());
    let _ = std::fs::remove_file(&path);
    assert!(part.is_some());
}

#[test]
fn o5_from_raf_pathbuf() {
    let path = write_temp_docx();
    let mut pkg = OPCPackage::from_raf(&path, PackageAccess::READ);
    let part = pkg.getPart(&doc_name());
    let _ = std::fs::remove_file(&path);
    assert!(part.is_some());
}

#[test]
fn o6_get_part_triggers_scan() {
    let mut pkg = pkg_from_bytes(min_docx_bytes());
    assert!(pkg.getPartList().is_none());
    assert!(pkg.getPart(&doc_name()).is_some());
}

#[test]
fn o7_read_close_drops_archive() {
    let mut pkg = pkg_from_bytes(min_docx_bytes());
    pkg.getParts();
    pkg.close().unwrap();
    assert!(pkg.getZipArchive().is_none());
}

#[test]
fn o8_document_creator() {
    assert_eq!(DOCUMENT_CREATOR, "WPS Office");
}

#[test]
#[should_panic(expected = "Can't open")]
fn o9_missing_path_fails() {
    let _ = OPCPackage::from_path("/no/such/ioffice-opc-07.docx", PackageAccess::READ);
}

#[test]
fn o10_no_package_rels_is_empty() {
    let types = types_xml();
    let bytes = zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ]);
    let mut pkg = pkg_from_bytes(bytes);
    assert_eq!(pkg.getRelationships().size(), 0);
}

#[test]
fn o11_get_part_from_office_document_rel() {
    let mut pkg = pkg_from_bytes(min_docx_bytes());
    let _ = pkg.getRelationshipsByType(OFFICE_DOC);
    let rel = pkg.getRelationship("rId1").expect("rId1");
    let part = pkg.getPart_rel(&rel).expect("resolved part");
    assert_eq!(part.getPartName().getName(), "/word/document.xml");
}

#[test]
fn o12_content_types_only_empty_part_list() {
    let types = types_xml();
    let mut pkg = pkg_from_bytes(zip_bytes(&[("[Content_Types].xml", types.as_bytes())]));
    pkg.getParts();
    assert_eq!(pkg.getPartList().unwrap().size(), 0);
}
