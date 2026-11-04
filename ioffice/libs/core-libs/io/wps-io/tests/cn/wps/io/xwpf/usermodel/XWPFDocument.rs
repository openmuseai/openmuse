// function_id=13_function_xwpf_parse
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 13_function_xwpf_parse

use std::cell::RefCell;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::rc::Rc;

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::PackageRelationshipTypes::PackageRelationshipTypes;
use core_libs_io::io::xwpf::usermodel::XWPFDocument::XWPFDocument;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Call {
    StylesEnd,
    ParaStart,
    Run(String),
}

struct RecordingImporter {
    calls: RefCell<Vec<Call>>,
}

impl RecordingImporter {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            calls: RefCell::new(Vec::new()),
        })
    }
}

impl IImporterBase for RecordingImporter {
    fn mode(&self) -> i32 {
        0
    }
}

impl IDocumentImporter for RecordingImporter {
    fn onImportStylesEnd(&self) {
        self.calls.borrow_mut().push(Call::StylesEnd);
    }
    fn onImportParagraphStart(&self, _sub: i32, _attrs: &dyn std::any::Any) {
        self.calls.borrow_mut().push(Call::ParaStart);
    }
    fn onImportRun_chars(
        &self,
        _sub: i32,
        _prop: &dyn std::any::Any,
        buf: &[char],
        start: i32,
        len: i32,
    ) {
        let text: String = buf
            .iter()
            .skip(start as usize)
            .take(len as usize)
            .collect();
        self.calls.borrow_mut().push(Call::Run(text));
    }
}

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

fn min_docx_with_body_no_styles() -> Vec<u8> {
    let main_ct =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
    let rels_ct = "application/vnd.openxmlformats-package.relationships+xml";
    let xml_ct = "application/xml";
    let types = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="{rels_ct}"/>
  <Default Extension="xml" ContentType="{xml_ct}"/>
  <Override PartName="/word/document.xml" ContentType="{main_ct}"/>
</Types>"#
    );
    let office = PackageRelationshipTypes::CORE_DOCUMENT;
    let rels = format!(
        r#"<?xml version="1.0"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="{office}" Target="word/document.xml"/>
</Relationships>"#
    );
    let document = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Hi</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
    zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", rels.as_bytes()),
        ("word/document.xml", document.as_bytes()),
    ])
}

#[test]
fn xp1_styles_before_body() {
    let pack = open_fixture();
    let mut doc = XWPFDocument::XWPFDocument__pkg(pack);
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    doc.setDocumentImporter(importer);
    doc.parse();
    let calls = rec.calls.borrow();
    let first_style = calls.iter().position(|c| matches!(c, Call::StylesEnd));
    let first_para = calls.iter().position(|c| matches!(c, Call::ParaStart));
    assert!(
        first_style.is_some() && first_para.is_some(),
        "need styles and paragraph callbacks: {calls:?}"
    );
    assert!(
        first_style.unwrap() < first_para.unwrap(),
        "styles must precede body: {calls:?}"
    );
}

#[test]
fn xp2_missing_styles_matches_java() {
    let bytes = min_docx_with_body_no_styles();
    let mut input = Cursor::new(bytes);
    let pack = XWPFPackage::open_stream(&mut input);
    let mut doc = XWPFDocument::XWPFDocument__pkg(pack);
    assert!(doc.getStyles().is_none());
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    doc.setDocumentImporter(importer);
    doc.parse();
    let calls = rec.calls.borrow();
    assert!(
        calls.iter().any(|c| matches!(c, Call::ParaStart)),
        "body still parses without styles: {calls:?}"
    );
    assert!(
        !calls.iter().any(|c| matches!(c, Call::StylesEnd)),
        "no styles part → no StylesEnd: {calls:?}"
    );
}

#[test]
fn xp3_parse_without_importer() {
    let pack = open_fixture();
    let mut doc = XWPFDocument::XWPFDocument__pkg(pack);
    doc.parse();
}
