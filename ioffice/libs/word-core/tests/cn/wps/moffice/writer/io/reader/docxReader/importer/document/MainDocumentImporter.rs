// function_id=06_function_maindoc_guard
// cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_maindoc_guard
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporter.rs

#![allow(non_snake_case)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use io_wps_io::cn::wps::io::simple::ForceQuitException::ForceQuitException;
use io_wps_io::cn::wps::io::simple::ReadingLimitException::ReadingLimitException;
use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
use io_wps_io::cn::wps::io::xwpf::element_handler::PprHandler::PprHandler;
use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::transaction::Transaction::Transaction;
use util_adapter::cn::wps::moffice::io::IDLProvider::IDLProvider;
use word_core::cn::wps::collections::textrope::ITextPool::ITextPool;
use word_core::cn::wps::collections::textrope::KTextPool::KTextPool;
use word_core::cn::wps::moffice::writer::core::Document::MAIN_DOCUMENT;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::MainDocumentImporter::MainDocumentImporter;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;

struct DummyImporter;
impl IImporterBase for DummyImporter {
    fn mode(&self) -> i32 {
        0
    }
}
impl IDocumentImporter for DummyImporter {}

struct TestDl {
    force: bool,
    limit: Option<i32>,
}
impl TestDl {
    fn ok() -> Self {
        Self {
            force: false,
            limit: None,
        }
    }
    fn force_quit() -> Self {
        Self {
            force: true,
            limit: None,
        }
    }
    fn limit(n: i32) -> Self {
        Self {
            force: false,
            limit: Some(n),
        }
    }
}
impl IDLProvider for TestDl {
    fn setOpenForceQuit(&mut self, var1: bool) {
        self.force = var1;
    }
    fn isOpenForceQuit(&self) -> bool {
        self.force
    }
    fn isOpenFirstPageForceQuit(&self) -> bool {
        false
    }
    fn getReadingLimit(&self) -> Option<i32> {
        self.limit
    }
    fn getNotePath(&self) -> String {
        String::new()
    }
    fn getWritePassword(&self) -> String {
        String::new()
    }
    fn isOpenOnReadOnly(&self) -> bool {
        false
    }
}

#[derive(Clone)]
struct Rec {
    locks: Rc<Cell<i32>>,
    unlocks: Rc<Cell<i32>>,
}
impl Rec {
    fn new() -> Self {
        Self {
            locks: Rc::new(Cell::new(0)),
            unlocks: Rc::new(Cell::new(0)),
        }
    }
}
impl IOListener for Rec {
    fn onFirstLock(&mut self) {
        self.locks.set(self.locks.get() + 1);
    }
    fn onFirstUnLock(&mut self) {
        self.unlocks.set(self.unlocks.get() + 1);
    }
    // rust: ★M-STAGE2 P1-D1 —— 真实 IOListener（9 方法）补齐此前只对着
    //   3 方法影子写的空实现。
    fn onLoadParas(&mut self, _cp: i32) {}
    fn onError(&mut self, _resultCode: i32, _msg: &dyn stubs::java::lang::Object::Object) {}

    fn beginLoadOnlineSecurityDoc(&mut self) {}

    fn onFinish(&mut self) {}

    fn onFinishDumpObjects(&mut self) {}

    fn onHtmlOpenError(&mut self) {}

    fn onFinishVerifyWritePassword(&mut self) {}

}

struct ParentEnv {
    td: Box<word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument>,
}

impl ParentEnv {
    fn new() -> Self {
        Self {
            td: word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument::new(),
        }
    }
    fn main_mut(&mut self) -> &mut SubDocument {
        self.td
            .getSubdocumentMut(MAIN_DOCUMENT)
            .expect("main SubDocument")
    }
}

fn style_map() -> StyleId2Sti {
    StyleId2Sti::StyleId2Sti(UUSti::UUSti())
}

fn num_map() -> NumIdRedirection {
    NumIdRedirection::NumIdRedirection(UUNumberingId::new())
}

fn importer(sd: &mut SubDocument, dl: TestDl) -> MainDocumentImporter {
    MainDocumentImporter::MainDocumentImporter(
        sd,
        style_map(),
        num_map(),
        None,
        Some(Box::new(dl)),
        false,
        0,
        None,
        None,
    )
}

fn importer_listen(sd: &mut SubDocument, dl: TestDl, rec: Rec) -> MainDocumentImporter {
    MainDocumentImporter::MainDocumentImporter(
        sd,
        style_map(),
        num_map(),
        Some(Box::new(rec)),
        Some(Box::new(dl)),
        false,
        0,
        None,
        None,
    )
}

fn paragraph() -> ParagraphSimple {
    ParagraphSimple::new_optional(Box::new(()), None)
}

fn paragraph_formfeed() -> ParagraphSimple {
    let mut ppr = PprHandler::new(Box::new(()), Rc::new(DummyImporter));
    let _ = ppr.getElementHandler(-906275931, "");
    ParagraphSimple::new_optional(Box::new(()), Some(ppr))
}

fn chpx_ps() -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 1);
    mps.freeze()
}

#[test]
fn GD1_force_quit_throws() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd, TestDl::force_quit());
    let err = imp.importParagraphStart(None).unwrap_err();
    assert!(err.downcast_ref::<ForceQuitException>().is_some());
}

#[test]
fn GD2_reading_limit_throws() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd, TestDl::limit(10));
    imp.importRunText("12345678901", &chpx_ps());
    assert_eq!(sd.getLength(), 11);
    let err = imp.importParagraphStart(None).unwrap_err();
    assert!(err.downcast_ref::<ReadingLimitException>().is_some());
}

#[test]
fn GD3_start_ok_without_throw() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd, TestDl::ok());
    imp.importParagraphStart(None).unwrap();
    assert!(imp.mCurrentParagraph.is_some());
}

#[test]
fn GD4_lock_on_start_when_fields_ended() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let rec = Rec::new();
    let locks = rec.locks.clone();
    let mut imp = importer_listen(sd, TestDl::ok(), rec);
    imp.importParagraphStart(None).unwrap();
    assert_eq!(locks.get(), 1);
    assert!(imp.mWriteLock.is_some());
}

#[test]
fn GD5_unlock_on_end_when_fields_ended() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let rec = Rec::new();
    let locks = rec.locks.clone();
    let unlocks = rec.unlocks.clone();
    let mut imp = importer_listen(sd, TestDl::ok(), rec);
    imp.importParagraphStart(None).unwrap();
    assert_eq!(locks.get(), 1);
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 0);
    assert_eq!(unlocks.get(), 1);
    assert!(imp.mWriteLock.is_none());
}

#[test]
fn GD6_formfeed_records_section_cp() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd, TestDl::ok());
    imp.importRunText("abcd", &chpx_ps());
    imp.importParagraphStart(None).unwrap();
    let mut p = paragraph_formfeed();
    imp.importParagraphEnd(Some(&p), 0);
    assert_eq!(sd.getLength(), 5);
    assert_eq!(imp.mCpLastSectionEnd, Some(4));
}

#[test]
fn GD7_cr_does_not_record_section_cp() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd, TestDl::ok());
    imp.importRunText("abcd", &chpx_ps());
    imp.importParagraphStart(None).unwrap();
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 0);
    assert_eq!(sd.getLength(), 5);
    assert_eq!(imp.mCpLastSectionEnd, None);
}

#[test]
fn GD8_fields_have_ended_without_fields() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let imp = importer(sd, TestDl::ok());
    assert!(imp.fieldsHaveEnded());
}
