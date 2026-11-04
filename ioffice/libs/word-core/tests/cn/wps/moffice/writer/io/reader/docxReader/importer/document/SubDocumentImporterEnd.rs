// function_id=03_function_paragraph_end
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_paragraph_end
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporterEnd.rs

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
use io_wps_io::cn::wps::io::xwpf::element_handler::PprHandler::PprHandler;
use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::transaction::Transaction::Transaction;
use word_core::cn::wps::collections::textrope::ITextPool::ITextPool;
use word_core::cn::wps::collections::textrope::KTextPool::KTextPool;
use word_core::cn::wps::moffice::writer::core::Document::{Document, MAIN_DOCUMENT};
use word_core::cn::wps::moffice::writer::core::KStyle::{KStyle, TextDocument};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::data::IPapxTable::Entry as IPapxEntry;
use word_core::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::SubDocumentImporter;

struct DummyImporter;
impl IImporterBase for DummyImporter {
    fn mode(&self) -> i32 {
        0
    }
}
impl IDocumentImporter for DummyImporter {}

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
fn PE1_papx_uses_start_cp0() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importRunText("Hi", &chpx_ps());
    assert_eq!(sd.getLength(), 2);
    imp.importParagraphStart(None);
    let mut p = paragraph();
    let entry = imp.importParagraphEnd(Some(&p), 0).unwrap();
    assert_eq!(entry.position(), 2);
    let at_end = IPapxTable::seek(sd.getPapxTable().unwrap(), sd.getLength()).unwrap();
    assert_eq!(at_end.position(), 2);
}

#[test]
fn PE2_endmark_cr_writes_length_one() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 0);
    assert_eq!(sd.getLength(), 1);
}

#[test]
fn PE3_paraid_writes_282() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(Some("p1"));
    let mut p = paragraph();
    let entry = imp.importParagraphEnd(Some(&p), 0).unwrap();
    let prop = IPapxEntry::getProperty(entry.as_ref()).unwrap();
    assert!(prop.contains(282));
    assert_eq!(prop.getString(282), "p1");
}

#[test]
fn PE4_no_paraid_does_not_write_282() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 0);
    let entry = IPapxTable::seek(sd.getPapxTable().unwrap(), 0).unwrap();
    assert!(!IPapxEntry::getProperty(entry.as_ref()).unwrap().contains(282));
}

#[test]
fn PE5_correct_prop_default_style_zero() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 0);
    let entry = IPapxTable::seek(sd.getPapxTable().unwrap(), 0).unwrap();
    assert_eq!(IPapxEntry::getProperty(entry.as_ref()).unwrap().getInt(191), 0);
}

#[test]
fn PE6_table_amend_sets_223_224() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 1);
    let entry = IPapxTable::seek(sd.getPapxTable().unwrap(), 0).unwrap();
    let prop = IPapxEntry::getProperty(entry.as_ref()).unwrap();
    assert!(prop.contains(223));
    assert_eq!(prop.getInt(224), 1);
}

#[test]
fn PE7_formfeed_sets_last_section_end_cp() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    let mut p = paragraph_formfeed();
    imp.importParagraphEnd(Some(&p), 0);
    let plc = sd.getPLCSection().expect("PLCSection");
    assert_eq!(plc.getLastSectionEndCP(), sd.getLength());
}

#[test]
fn PE8_numbering_direct_234() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut mps = MutablePropertySet::new();
    mps.setInt(234, 2);
    let ps = mps.freeze();
    let entry = CoreHelper::insertToPapxTable(Some(&mut *sd as &mut dyn Document), 0, Some(&ps))
        .expect("papx");
    assert!(imp.isContainNumbering(entry.as_ref()));
}

#[test]
fn PE9_numbering_via_style_191() {
    let mut env = ParentEnv::new();
    let mut style_ps = MutablePropertySet::new();
    style_ps.setInt(235, 1);
    let frozen = style_ps.freeze();
    let stand_in = TextDocument::default();
    let mut ks = KStyle::KStyle__textdocument_int_string_int_int_int_propertyset(
        &stand_in, 7, "n", 0, 0, 1, &frozen,
    );
    env.td.getStylesMut().expect("styles").add(&mut ks);
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut mps = MutablePropertySet::new();
    mps.setInt(191, 7);
    let ps = mps.freeze();
    let entry = CoreHelper::insertToPapxTable(Some(&mut *sd as &mut dyn Document), 0, Some(&ps))
        .expect("papx");
    assert!(imp.isContainNumbering(entry.as_ref()));
}

#[test]
fn PE10_reset_clears_entry_keeps_cp0() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importRunText("Hello", &chpx_ps());
    imp.importParagraphStart(None);
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), 0);
    {
        let ctx = imp.mCurrentParagraph.as_ref().unwrap();
        assert_eq!(ctx.mPositionInserted, Some(5));
        assert!(ctx.mEntry.is_some());
    }
    imp.resetParagraph();
    let ctx = imp.mCurrentParagraph.as_ref().unwrap();
    assert!(ctx.mEntry.is_none());
    assert_eq!(ctx.mPositionInserted, Some(5));
}

#[test]
#[should_panic(expected = "paragraph should not be null")]
fn PE11_null_paragraph_panics() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    let _ = imp.importParagraphEnd(None, 0);
}
