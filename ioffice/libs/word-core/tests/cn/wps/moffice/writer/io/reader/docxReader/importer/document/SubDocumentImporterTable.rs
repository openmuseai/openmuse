// function_id=04_function_table_cell
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_table_cell
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporterTable.rs

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::transaction::Transaction::Transaction;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::moffice::writer::core::Document::MAIN_DOCUMENT;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::data::IPapxTable::Entry as IPapxEntry;
use word_core::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::SubDocumentImporter;

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

fn end_para(imp: &mut SubDocumentImporter, layer: i32) {
    let mut p = paragraph();
    imp.importParagraphEnd(Some(&p), layer);
}

fn last_prop(sd: &SubDocument) -> PropertySet {
    let entry = IPapxTable::last(sd.getPapxTable().unwrap()).unwrap();
    IPapxEntry::getProperty(entry.as_ref()).unwrap().clone()
}

#[test]
fn TB1_same_layer_appends_empty_cr() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    end_para(&mut imp, 1);
    let before = sd.getLength();
    imp.importTableStart(1);
    assert_eq!(sd.getLength(), before + 1);
}

#[test]
fn TB2_different_layer_does_not_append() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    end_para(&mut imp, 2);
    let before = sd.getLength();
    imp.importTableStart(1);
    assert_eq!(sd.getLength(), before);
}

#[test]
fn TB3_nested_layer_writes_223_and_224_minus_one() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    end_para(&mut imp, 2);
    imp.importTableStart(2);
    let prop = last_prop(&sd);
    assert!(prop.contains(223));
    assert_eq!(prop.getInt(224), 1);
}

#[test]
fn TB4_cell_start_clears_entry_keeps_cp0() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importRunText("Hello", &{
        let mut mps = MutablePropertySet::new();
        mps.setInt(193, 1);
        mps.freeze()
    });
    imp.importParagraphStart(None);
    end_para(&mut imp, 0);
    {
        let ctx = imp.mCurrentParagraph.as_ref().unwrap();
        assert_eq!(ctx.mPositionInserted, Some(5));
        assert!(ctx.mEntry.is_some());
    }
    imp.importTableCellStartSimple();
    let ctx = imp.mCurrentParagraph.as_ref().unwrap();
    assert!(ctx.mEntry.is_none());
    assert_eq!(ctx.mPositionInserted, Some(5));
}

#[test]
fn TB5_empty_cell_end_appends_cell_mark_para() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    assert_eq!(sd.getLength(), 0);
    imp.importTableCellEndSimple(1, 'x');
    assert_eq!(sd.getLength(), 1);
}

#[test]
fn TB6_cell_end_writes_223_224_226() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    imp.importTableCellEndSimple(1, 'x');
    let prop = last_prop(&sd);
    assert!(prop.contains(223));
    assert_eq!(prop.getInt(224), 1);
    assert!(prop.contains(226));
}

#[test]
fn TB7_set_cell_mark_rewrites_last_char() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importParagraphStart(None);
    imp.importTableCellEndSimple(1, 'x');
    let ch = unsafe {
        let r = sd.getTextRope().expect("rope");
        ITextRope::charAt(&mut *r, sd.getLength() - 1)
    };
    assert_eq!(ch, 'x');
}

#[test]
fn TB8_row_end_appends_row_mark_and_table_props() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut table_props = MutablePropertySet::new();
    table_props.setBoolean(223, true);
    table_props.setInt(224, 1);
    let run = MutablePropertySet::new().freeze();
    imp.doImportTableRowEndSimple(&mut table_props, &run, '\r');
    assert_eq!(sd.getLength(), 1);
    let prop = last_prop(&sd);
    assert!(prop.contains(223));
    assert_eq!(prop.getInt(224), 1);
}
