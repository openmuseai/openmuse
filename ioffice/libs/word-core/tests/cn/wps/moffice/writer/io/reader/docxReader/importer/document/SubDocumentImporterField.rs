// function_id=05_function_field_control
// cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_field_control
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporterField.rs

#![allow(non_snake_case)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::transaction::Transaction::Transaction;
use word_core::cn::wps::collections::textrope::ITextPool::ITextPool;
use word_core::cn::wps::collections::textrope::KTextPool::KTextPool;
use word_core::cn::wps::moffice::writer::core::Document::MAIN_DOCUMENT;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::data::IChpxTable::Entry as IChpxEntry;
use word_core::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::{
    IOListener, Shape, SubDocumentImporter,
};

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

fn importer(sd: &mut SubDocument) -> SubDocumentImporter {
    SubDocumentImporter::SubDocumentImporter(sd, style_map(), num_map(), None, false, 0, None, None)
}

fn importer_listen(sd: &mut SubDocument, rec: Rec) -> SubDocumentImporter {
    SubDocumentImporter::SubDocumentImporter(
        sd,
        style_map(),
        num_map(),
        Some(Box::new(rec)),
        false,
        0,
        None,
        None,
    )
}

fn chpx_at(sd: &SubDocument, cp: i32) -> util::moffice::property::PropertySet::PropertySet {
    let e = IChpxTable::seek(sd.getChpxTable().unwrap(), cp).unwrap();
    IChpxEntry::getProperty(e.as_ref()).unwrap().clone()
}

const BEGIN: char = '\u{0013}';
const END: char = '\u{0015}';

struct FieldCharAttrs(&'static str);
impl word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes for FieldCharAttrs {
    fn getLength(&self) -> i32 { 1 }
    fn getValue(&self, _uri: &str, local_name: &str) -> Option<String> {
        (local_name == "fldCharType").then(|| self.0.to_string())
    }
}

#[test]
fn FLD1_control_char_writes_rope_and_prop_28() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let cp = imp.appendControlCharField__char(BEGIN);
    assert_eq!(sd.getLength(), 1);
    assert_eq!(cp, 0);
    assert!(chpx_at(&sd, 0).contains(28));
}

#[test]
fn FLD2_ignored_sets_758() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.appendControlCharField__char_boolean(BEGIN, true);
    assert!(chpx_at(&sd, 0).contains(758));
}

#[test]
fn FLD3_shape_sets_56_30_29() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let shape = Shape::default();
    imp.appendControlCharField__char_boolean_shape(BEGIN, false, Some(&shape));
    let p = chpx_at(&sd, 0);
    assert!(p.contains(56));
    assert!(p.contains(30));
    assert!(p.contains(29));
}

#[test]
fn FLD4_begin_increments_not_ended() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let cp = imp.appendControlCharField__char(BEGIN);
    imp.appendToPLCField(BEGIN, cp);
    assert_eq!(imp.mFieldsNotEnded, 1);
}

#[test]
fn FLD5_end_decrements_not_ended() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let cp0 = imp.appendControlCharField__char(BEGIN);
    imp.appendToPLCField(BEGIN, cp0);
    let cp1 = imp.appendControlCharField__char(END);
    imp.appendToPLCField(END, cp1);
    assert_eq!(imp.mFieldsNotEnded, 0);
}

#[test]
fn FLD11_field_nodes_keep_java_identity_in_plc() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    for ch in ['\u{0013}', '\u{0014}', '\u{0015}'] {
        let cp = imp.appendControlCharField__char(ch);
        imp.appendToPLCField(ch, cp);
    }
    let triples = sd.getPLCField().expect("PLCField").getFieldTriples();
    assert_eq!(triples.len(), 1);
    assert_eq!((triples[0].getStartCP(), triples[0].getSepCP(), triples[0].getEndCP()), (0, 1, 2));
}

#[test]
fn FLD12_sax_field_chars_reach_plc() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    for name in ["begin", "separate", "end"] {
        imp.importFldCharSimple(&FieldCharAttrs(name), None);
    }
    let triples = sd.getPLCField().expect("PLCField").getFieldTriples();
    assert_eq!(triples.len(), 1);
    assert_eq!((triples[0].getStartCP(), triples[0].getSepCP(), triples[0].getEndCP()), (0, 1, 2));
}

#[test]
fn FLD6_lock_creates_lock_no_callback_when_len_nonzero() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let rec = Rec::new();
    let watch = rec.clone();
    let mut imp = importer_listen(sd, rec);
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 1);
    imp.importRunText("Hi", &mps.freeze());
    imp.lock();
    assert!(imp.mWriteLock.is_some());
    assert_eq!(imp.mWriteLock.as_ref().unwrap().getEnd(), 2);
    assert_eq!(watch.locks.get(), 0);
}

#[test]
fn FLD7_repeat_lock_is_noop() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 1);
    imp.importRunText("Hi", &mps.freeze());
    imp.lock();
    let end = imp.mWriteLock.as_ref().unwrap().getEnd();
    imp.lock();
    assert_eq!(imp.mWriteLock.as_ref().unwrap().getEnd(), end);
}

#[test]
fn FLD8_unlock_clears_lock() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let rec = Rec::new();
    let watch = rec.clone();
    let mut imp = importer_listen(sd, rec);
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 1);
    imp.importRunText("Hi", &mps.freeze());
    imp.lock();
    imp.unlock();
    assert!(imp.mWriteLock.is_none());
    assert_eq!(watch.unlocks.get(), 1);
}

#[test]
fn FLD9_unlock_without_lock_is_noop() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let rec = Rec::new();
    let watch = rec.clone();
    let mut imp = importer_listen(sd, rec);
    imp.unlock();
    assert!(imp.mWriteLock.is_none());
    assert_eq!(watch.unlocks.get(), 0);
    assert_eq!(watch.locks.get(), 0);
}

#[test]
fn FLD10_empty_doc_lock_calls_on_first_lock() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let rec = Rec::new();
    let watch = rec.clone();
    let mut imp = importer_listen(sd, rec);
    imp.lock();
    assert_eq!(watch.locks.get(), 1);
}
