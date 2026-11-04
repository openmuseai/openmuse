// function_id=12_function_handler_body
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 12_function_handler_body

use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use core_libs_io::io::opt::XmlReader::XmlReader;
use core_libs_io::io::simple::ElementHandlerSimple::ElementHandlerRef;
use core_libs_io::io::xwpf::element_handler::DocumentHandler::DocumentHandler;
use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;

const NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Call {
    MainStart,
    ParaStart,
    Run(String),
    ParaEnd,
    MainEnd,
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
    fn onMainDocumentStart(&self, _has_hdr_ftr: bool) {
        self.calls.borrow_mut().push(Call::MainStart);
    }
    fn onMainDocumentEnd(&self) {
        self.calls.borrow_mut().push(Call::MainEnd);
    }
    fn onImportParagraphStart(&self, _sub: i32, _attrs: &dyn std::any::Any) {
        self.calls.borrow_mut().push(Call::ParaStart);
    }
    fn onImportParagraphEnd(&self, _para: &dyn std::any::Any, _sub: i32, _layer: i32) {
        self.calls.borrow_mut().push(Call::ParaEnd);
    }
    fn onImportRun_chars(&self, _sub: i32, _prop: &dyn std::any::Any, buf: &[char], start: i32, len: i32) {
        let text: String = buf
            .iter()
            .skip(start as usize)
            .take(len as usize)
            .collect();
        self.calls.borrow_mut().push(Call::Run(text));
    }
}

fn parse_document(xml: &str, importer: Rc<dyn IDocumentImporter>) {
    let mut root: ElementHandlerRef = DocumentHandler::as_handler_ref(importer);
    let mut input = Cursor::new(xml.as_bytes().to_vec());
    XmlReader::new()
        .parse(&mut input, &mut root, "utf-8", -1, 0)
        .expect("XmlReader.parse");
}

fn wrap_body(inner: &str) -> String {
    format!(
        r#"<?xml version="1.0"?>
<w:document xmlns:w="{NS}">
  <w:body>
{inner}
  </w:body>
</w:document>"#
    )
}

#[test]
fn b1_single_run_hi() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let xml = wrap_body(
        r#"    <w:p>
      <w:r>
        <w:t>Hi</w:t>
      </w:r>
    </w:p>"#,
    );
    parse_document(&xml, importer);
    assert_eq!(
        *rec.calls.borrow(),
        vec![
            Call::MainStart,
            Call::ParaStart,
            Call::Run("Hi".into()),
            Call::ParaEnd,
            Call::MainEnd,
        ]
    );
}

#[test]
fn b2_two_para_end_count() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let xml = wrap_body(
        r#"    <w:p><w:r><w:t>A</w:t></w:r></w:p>
    <w:p><w:r><w:t>B</w:t></w:r></w:p>"#,
    );
    parse_document(&xml, importer);
    let calls = rec.calls.borrow().clone();
    let ends = calls.iter().filter(|c| matches!(c, Call::ParaEnd)).count();
    let runs: Vec<_> = calls
        .iter()
        .filter_map(|c| match c {
            Call::Run(s) => Some(s.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(ends, 2);
    assert_eq!(runs, vec!["A".to_string(), "B".to_string()]);
}

#[test]
fn b3_empty_body_no_run() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    parse_document(&wrap_body(""), importer);
    let calls = rec.calls.borrow();
    assert!(
        !calls.iter().any(|c| matches!(c, Call::Run(_))),
        "empty body must not emit Run: {calls:?}"
    );
    assert!(
        !calls.iter().any(|c| matches!(c, Call::ParaStart)),
        "empty body must not emit ParagraphStart: {calls:?}"
    );
    assert!(calls.contains(&Call::MainStart));
    assert!(calls.contains(&Call::MainEnd));
}
