// wave0-ooxml-standards feature smoke: standard OOXML Transitional .docx
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test wave0_e2e_standard_docx
//
// This is NOT the full importer pipeline (10–14 still spec). It exercises the
// wired path: unzip → Content Types → parts → package open → document.xml events.

use core_libs_io::io::opt::XmlReader::XmlReader;
use core_libs_io::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use core_libs_io::io::simple::ElementIds;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use core_libs_io::io::opc::PackageRelationshipTypes::PackageRelationshipTypes;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::any::Any;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::rc::Rc;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx")
}

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(fixture_path()).expect("standard-hello.docx fixture")
}

fn java_string_hash(s: &str) -> i32 {
    s.bytes()
        .fold(0i32, |h, b| h.wrapping_mul(31).wrapping_add(b as i32))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Ev {
    Start(i32),
    Chars(String),
    End(i32),
}

struct Recording {
    events: Rc<RefCell<Vec<Ev>>>,
}

impl ElementHandlerSimple for Recording {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        -1
    }
    fn onStart(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
        _d: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        id: i32,
        _ns: &str,
        _local: &str,
        _q: &str,
        _attrs: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.events.borrow_mut().push(Ev::Start(id));
        Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.events.borrow_mut().push(Ev::End(id));
        Ok(())
    }
    fn onCharacters(
        &mut self,
        value: &[char],
        start: i32,
        len: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let s = start as usize;
        let l = len as usize;
        let text: String = value.get(s..s + l).unwrap_or(&[]).iter().collect();
        self.events.borrow_mut().push(Ev::Chars(text));
        Ok(())
    }
    fn onCharacters_str(&mut self, persist: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.events
            .borrow_mut()
            .push(Ev::Chars(format!("path:{persist}")));
        Ok(())
    }
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        Some(Rc::new(RefCell::new(Recording {
            events: self.events.clone(),
        })))
    }
}

fn parse_part_xml(bytes: &[u8]) -> Vec<Ev> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut handler: ElementHandlerRef = Rc::new(RefCell::new(Recording {
        events: events.clone(),
    }));
    let mut cur = Cursor::new(bytes.to_vec());
    XmlReader::new()
        .parse(&mut cur, &mut handler, "utf-8", -1, 0)
        .expect("XmlReader::parse");
    let out = events.borrow().clone();
    out
}

fn t_text(events: &[Ev]) -> String {
    let mut inside = false;
    let mut acc = String::new();
    for e in events {
        match e {
            Ev::Start(id) if *id == ElementIds::T => inside = true,
            Ev::End(id) if *id == ElementIds::T => inside = false,
            Ev::Chars(s) if inside => acc.push_str(s),
            _ => {}
        }
    }
    acc
}

#[test]
fn e2e_unzip_lists_opc_parts() {
    let mut input = Cursor::new(fixture_bytes());
    let src = ZipInputStreamZipEntrySource::try_new(&mut input).expect("unzip");
    let names: Vec<String> = src
        .getEntries()
        .as_ref()
        .expect("entries")
        .iter()
        .map(|e| e.getName().to_string())
        .collect();
    for required in [
        "[Content_Types].xml",
        "_rels/.rels",
        "word/document.xml",
        "word/styles.xml",
        "word/_rels/document.xml.rels",
        "docProps/core.xml",
        "docProps/app.xml",
    ] {
        assert!(
            names.iter().any(|n| n == required),
            "missing {required} in {names:?}"
        );
    }
}

#[test]
fn e2e_open_stream_resolves_office_document() {
    let mut input = Cursor::new(fixture_bytes());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    let main = PackagingURIHelper::createPartName_str("/word/document.xml").unwrap();
    let part = pkg.getPart(&main).expect("/word/document.xml");
    assert!(
        part.getContentType()
            .toString()
            .contains("wordprocessingml.document.main"),
        "{}",
        part.getContentType().toString()
    );
    let office = pkg.getRelationshipsByType(PackageRelationshipTypes::CORE_DOCUMENT);
    assert_eq!(office.size(), 1);
    assert!(office
        .getRelationship_by_index(0)
        .unwrap()
        .getTargetURI()
        .toString()
        .contains("word/document.xml"));
}

#[test]
fn e2e_open_path_and_parse_document_text() {
    let path = fixture_path();
    assert!(path.is_file(), "{}", path.display());
    let mut pkg = XWPFPackage::open_path(path.to_str().unwrap(), 0);
    let main = PackagingURIHelper::createPartName_str("/word/document.xml").unwrap();
    let part = pkg.getPart(&main).expect("main part");
    let mut xml = Vec::new();
    part.getInputStream_default()
        .expect("document.xml stream")
        .read_to_end(&mut xml)
        .unwrap();
    let events = parse_part_xml(&xml);
    assert!(
        events.iter().any(|e| matches!(e, Ev::Start(id) if *id == ElementIds::P)),
        "missing w:p: {events:?}"
    );
    assert!(
        events.iter().any(|e| matches!(e, Ev::Start(id) if *id == ElementIds::RPR_B)),
        "missing w:b: {events:?}"
    );
    assert_eq!(t_text(&events), "Hello, wave0!");
    let r = java_string_hash("r");
    assert!(
        events.iter().any(|e| matches!(e, Ev::Start(id) if *id == r)),
        "missing w:r"
    );
}

#[test]
fn e2e_parse_styles_part() {
    let mut pkg = XWPFPackage::open_stream(&mut Cursor::new(fixture_bytes()));
    let name = PackagingURIHelper::createPartName_str("/word/styles.xml").unwrap();
    let part = pkg.getPart(&name).expect("styles");
    let mut xml = Vec::new();
    part.getInputStream_default()
        .unwrap()
        .read_to_end(&mut xml)
        .unwrap();
    let events = parse_part_xml(&xml);
    let style_id = java_string_hash("style");
    assert!(
        events.iter().any(|e| matches!(e, Ev::Start(id) if *id == style_id)),
        "missing w:style: {events:?}"
    );
}

#[test]
fn e2e_core_and_extended_props_parts_registered() {
    let mut pkg = XWPFPackage::open_stream(&mut Cursor::new(fixture_bytes()));
    let core = PackagingURIHelper::createPartName_str("/docProps/core.xml").unwrap();
    let app = PackagingURIHelper::createPartName_str("/docProps/app.xml").unwrap();
    assert!(pkg.getPart(&core).is_some());
    assert!(pkg.getPart(&app).is_some());
    assert_eq!(
        pkg.getRelationshipsByType(PackageRelationshipTypes::CORE_PROPERTIES)
            .size(),
        1
    );
    assert_eq!(
        pkg.getRelationshipsByType(PackageRelationshipTypes::EXTENDED_PROPERTIES)
            .size(),
        1
    );
}

#[test]
fn e2e_open_raf_pathbuf() {
    let path = fixture_path();
    let mut pkg = XWPFPackage::open_raf(&path, 0);
    let main = PackagingURIHelper::createPartName_str("/word/document.xml").unwrap();
    assert!(pkg.getPart(&main).is_some());
}

#[test]
fn e2e_file_backend_unzip() {
    let mut file = File::open(fixture_path()).unwrap();
    let src = ZipInputStreamZipEntrySource::try_new(&mut file).expect("file unzip");
    assert!(src.size() >= 8);
}
