// function_id=14_function_importer_boundary
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 14_function_importer_boundary

use std::cell::RefCell;

use core_libs_io::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::io::xwpf::ISubDocumentImporter::ISubDocumentImporter;
use core_libs_util::base::io::SubDocType::SubDocType;

struct ModeImporter(i32);

impl IImporterBase for ModeImporter {
    fn mode(&self) -> i32 { self.0 }
}

impl IDocumentImporter for ModeImporter {}

#[derive(Debug, PartialEq, Eq)]
enum Call {
    MainStart(bool),
    ParagraphStart(i32),
    Run { subdoc: i32, text: String, start: i32, len: i32 },
    RunStr { subdoc: i32, text: String },
    ParagraphEnd { subdoc: i32, layer: i32 },
    StylesEnd,
    MainEnd,
}

struct RecordingImporter {
    calls: RefCell<Vec<Call>>,
    convert_ids: RefCell<Vec<i32>>,
    run_prop_ids: RefCell<Vec<i32>>,
}

impl RecordingImporter {
    fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            convert_ids: RefCell::new(Vec::new()),
            run_prop_ids: RefCell::new(Vec::new()),
        }
    }
}

impl IImporterBase for RecordingImporter {
    fn mode(&self) -> i32 { 0 }
}

impl IDocumentImporter for RecordingImporter {
    fn onMainDocumentStart(&self, has_header_footer: bool) {
        self.calls.borrow_mut().push(Call::MainStart(has_header_footer));
    }

    fn onImportParagraphStart(&self, subdoc: i32, _attributes: &dyn std::any::Any) {
        self.calls.borrow_mut().push(Call::ParagraphStart(subdoc));
    }

    fn onImportRun_chars(
        &self,
        subdoc: i32,
        _props: &dyn std::any::Any,
        buffer: &[char],
        start: i32,
        len: i32,
    ) {
        let text = buffer
            .iter()
            .skip(start as usize)
            .take(len as usize)
            .collect();
        self.calls.borrow_mut().push(Call::Run { subdoc, text, start, len });
    }

    fn onImportRun_str(&self, subdoc: i32, _props: &dyn std::any::Any, text: &str) {
        self.calls.borrow_mut().push(Call::RunStr { subdoc, text: text.to_string() });
    }

    fn convertProp(
        &self,
        _prop_type: i32,
        _prop: &mut dyn std::any::Any,
        _ns: &str,
        prop_id: i32,
        _attrs: &dyn std::any::Any,
    ) {
        self.convert_ids.borrow_mut().push(prop_id);
    }

    fn convertRunProp(
        &self,
        _prop: &mut dyn std::any::Any,
        _ns: &str,
        element_id: i32,
        _attrs: &dyn std::any::Any,
    ) {
        self.run_prop_ids.borrow_mut().push(element_id);
    }

    fn correctProps(
        &self,
        _element_id: i32,
        _prop: &dyn std::any::Any,
        _original: Option<&dyn std::any::Any>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        Box::new(42i32)
    }

    fn onImportParagraphEnd(&self, _paragraph: &dyn std::any::Any, subdoc: i32, layer: i32) {
        self.calls.borrow_mut().push(Call::ParagraphEnd { subdoc, layer });
    }

    fn onImportStylesEnd(&self) {
        self.calls.borrow_mut().push(Call::StylesEnd);
    }

    fn onMainDocumentEnd(&self) {
        self.calls.borrow_mut().push(Call::MainEnd);
    }
}

#[test]
fn i1_subdoctype_ordinals() {
    assert_eq!(
        [
            SubDocType::MAIN_DOCUMENT as i32,
            SubDocType::FOOTNOTE_DOCUMENT as i32,
            SubDocType::HEADER_DOCUMENT as i32,
            SubDocType::COMMENT_DOCUMENT as i32,
            SubDocType::ENDNOTE_DOCUMENT as i32,
            SubDocType::TEXTBOX_DOCUMENT as i32,
            SubDocType::HEADERTEXTBOX_DOCUMENT as i32,
        ],
        [0, 1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn i2_mode_drives_exactly_one_format_branch() {
    for (mode, expected) in [
        (0, [true, false, false, false]),
        (1, [false, true, false, false]),
        (2, [false, false, true, false]),
        (3, [false, false, false, true]),
    ] {
        let handler = XmlSimpleNodeElementHandler::new_from_doc(Box::new(ModeImporter(mode)));
        assert_eq!(
            [handler.isDocx(), handler.isXml07(), handler.isGVml(), handler.isXml03()],
            expected
        );
    }
}

#[test]
fn i3_missing_importer_matches_no_format() {
    let handler = XmlSimpleNodeElementHandler::new();
    assert_eq!(
        [handler.isDocx(), handler.isXml07(), handler.isGVml(), handler.isXml03()],
        [false; 4]
    );
}

#[test]
fn i4_recording_importer_preserves_hot_path_shapes() {
    let importer = RecordingImporter::new();
    let chars: Vec<char> = "xhello!".chars().collect();
    let main = SubDocType::MAIN_DOCUMENT as i32;

    importer.onMainDocumentStart(true);
    importer.onImportParagraphStart(main, &());
    importer.onImportRun_chars(main, &(), &chars, 1, 5);
    importer.onImportRun_str(main, &(), "wave0");
    importer.onImportParagraphEnd(&(), main, 2);
    importer.onImportStylesEnd();
    importer.onMainDocumentEnd();

    assert_eq!(
        *importer.calls.borrow(),
        vec![
            Call::MainStart(true),
            Call::ParagraphStart(main),
            Call::Run { subdoc: main, text: "hello".to_string(), start: 1, len: 5 },
            Call::RunStr { subdoc: main, text: "wave0".to_string() },
            Call::ParagraphEnd { subdoc: main, layer: 2 },
            Call::StylesEnd,
            Call::MainEnd,
        ]
    );
}

#[test]
fn i5_is_lock_defaults_false() {
    assert!(!ModeImporter(0).isLock());
}

struct SubImporter;

impl IImporterBase for SubImporter {
    fn mode(&self) -> i32 { 3 }
}

impl ISubDocumentImporter for SubImporter {}

fn subdocument_mode(importer: &dyn ISubDocumentImporter) -> i32 {
    importer.mode()
}

#[test]
fn i6_subdocument_importer_inherits_mode_contract() {
    assert_eq!(subdocument_mode(&SubImporter), 3);
}

#[test]
fn i7_prop_and_default_surfaces() {
    let rec = RecordingImporter::new();
    rec.convertProp(0, &mut (), "http://schemas.openxmlformats.org/wordprocessingml/2006/main", 3385, &());
    rec.convertRunProp(&mut (), "http://schemas.openxmlformats.org/wordprocessingml/2006/main", 98, &());
    let corrected = rec.correctProps(110226, &(), None);
    assert_eq!(*rec.convert_ids.borrow(), vec![3385]);
    assert_eq!(*rec.run_prop_ids.borrow(), vec![98]);
    assert_eq!(corrected.downcast_ref::<i32>().copied(), Some(42));

    let defaults = ModeImporter(0);
    assert!(!defaults.hasContent());
    let _ = defaults.getVmlImporter(SubDocType::MAIN_DOCUMENT as i32);
    let _ = defaults.getDmlImporter(SubDocType::MAIN_DOCUMENT as i32);
    let _ = defaults.getDmlTextImporter(&());
    defaults.dispose();
}
