// function_id=06_function_finish_work
// cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_finish_work
#![allow(non_snake_case)]

use std::mem::ManuallyDrop;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use io_wps_io::cn::wps::io::xwpf::IDmlImporter::{DmlImporterRef, IDmlImporter};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use word_core::cn::wps::moffice::writer::io::reader::tools::FinishReadingWork::FinishReadingWork;

struct RecordingDmlImporter { count: Arc<AtomicI32> }

// Java's interface has a broad surface. The full test double is local to this
// integration test; only buildTxbxChain is exercised by FinishReadingWork.
include!("recording_dml_importer_impl.rs");

struct RecImporter {
    ends: Arc<AtomicI32>,
    dml: Option<DmlImporterRef>,
}

impl IImporterBase for RecImporter {
    fn mode(&self) -> i32 {
        0
    }
}

impl IDocumentImporter for RecImporter {
    fn onMainDocumentEnd(&self) {
        self.ends.fetch_add(1, Ordering::SeqCst);
    }
    fn getDmlImporter(&self, _var1: i32) -> Option<DmlImporterRef> {
        self.dml.clone()
    }
}

fn last_main_char(doc: &TextDocument) -> Option<char> {
    let sd = doc.getMainDocument()?;
    let r = sd.getTextRope()?;
    unsafe {
        let n = ITextRope::length(&mut *r);
        if n <= 0 {
            return None;
        }
        Some(ITextRope::charAt(&mut *r, n - 1))
    }
}

fn main_len(doc: &TextDocument) -> i32 {
    doc.getMainDocument().map(|m| m.getLength()).unwrap_or(0)
}

#[test]
fn FRW_all_finish_work_matrix() {
    let mut doc = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let ends = Arc::new(AtomicI32::new(0));
    let chain_count = Arc::new(AtomicI32::new(0));
    let chain: DmlImporterRef = Rc::new(std::cell::RefCell::new(RecordingDmlImporter { count: chain_count.clone() }));
    let importer = RecImporter {
        ends: ends.clone(),
        dml: Some(chain.clone()),
    };
    let mut work = FinishReadingWork::FinishReadingWork(doc.as_mut(), Box::new(importer));

    let before = main_len(doc.as_ref());
    work.go();
    // FRW1/FRW3: empty main gets supplyEndPara `\r`
    let after = main_len(doc.as_ref());
    assert!(after >= before, "FRW1");
    assert_eq!(last_main_char(doc.as_ref()), Some('\r'), "FRW3");
    let after_first = after;
    work.go();
    assert_eq!(main_len(doc.as_ref()), after_first, "FRW4");

    assert_eq!(ends.load(Ordering::SeqCst), 2, "FRW15 order: end after go");
    assert!(chain_count.load(Ordering::SeqCst) >= 1, "FRW12");

    {
        let main = doc.getMainDocumentMut().expect("main");
        let plc = main.getPLCSectionMut().expect("plc");
        let sec = plc.getFirstSection().expect("FRW5 section");
        let ps = sec.getPropertySet();
        assert_eq!(ps.getInt(678), 312, "FRW7");
        assert_eq!(ps.getInt(677), 0, "FRW7");
        assert_eq!(ps.getInt(682), 720, "FRW7");
        assert_eq!(ps.getInt(683), 720, "FRW7");
    }

    {
        let styles = doc.getStylesMut().expect("styles");
        assert!(styles.getStyleById__int(0).is_some(), "FRW13");
        let n = styles.getStyles().len();
        work.go();
        let n2 = doc.getStylesMut().expect("styles").getStyles().len();
        assert_eq!(n2, n, "FRW14");
    }

    IOHelper::endAllSubdocumentsDrawingContainer(doc.as_mut());
    if let Some(fnnote) = doc.getSubdocumentMut(1) {
        CoreHelper::supplyEndPara(Some(fnnote as &mut dyn Document));
        fnnote.setFootnoteInvalidCount(3);
        assert_eq!(fnnote.footnoteInvalidCount(), 3, "FRW9 seed");
    }
    work.go();
    if let Some(fnnote) = doc.getSubdocument(1) {
        assert_eq!(fnnote.footnoteInvalidCount(), 0, "FRW9");
    }
    let r = catch_unwind(AssertUnwindSafe(|| {
        IOHelper::endAllSubdocumentsDrawingContainer(doc.as_mut());
    }));
    assert!(r.is_ok(), "FRW16");

    let mut null_dml_doc = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let ends2 = Arc::new(AtomicI32::new(0));
    let importer2 = RecImporter {
        ends: ends2.clone(),
        dml: None,
    };
    let mut work2 = FinishReadingWork::FinishReadingWork(null_dml_doc.as_mut(), Box::new(importer2));
    work2.go();
    assert_eq!(ends2.load(Ordering::SeqCst), 1, "FRW11/FRW2");
}
