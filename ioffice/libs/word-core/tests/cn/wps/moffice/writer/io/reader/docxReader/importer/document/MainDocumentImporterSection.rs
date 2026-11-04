// function_id=07_function_section_add
// cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_section_add
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/MainDocumentImporterSection.rs

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::{empty, PropertySet};
use util::moffice::transaction::Transaction::Transaction;
use util_adapter::cn::wps::moffice::io::IDLProvider::IDLProvider;
use word_core::cn::wps::collections::textrope::ITextPool::ITextPool;
use word_core::cn::wps::collections::textrope::KTextPool::KTextPool;
use word_core::cn::wps::moffice::writer::core::Document::MAIN_DOCUMENT;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::data::PLC::Node;
use word_core::cn::wps::moffice::writer::data::PLCSection::SectionNode;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::MainDocumentImporter::MainDocumentImporter;

struct TestDl;
impl IDLProvider for TestDl {
    fn setOpenForceQuit(&mut self, _var1: bool) {}
    fn isOpenForceQuit(&self) -> bool {
        false
    }
    fn isOpenFirstPageForceQuit(&self) -> bool {
        false
    }
    fn getReadingLimit(&self) -> Option<i32> {
        None
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

fn importer(sd: &mut SubDocument) -> MainDocumentImporter {
    MainDocumentImporter::MainDocumentImporter(
        sd,
        style_map(),
        num_map(),
        None,
        Some(Box::new(TestDl)),
        false,
        0,
        None,
        None,
    )
}

fn props() -> MutablePropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(10, 42);
    mps
}

fn dummy_node() -> Node {
    SectionNode::SectionNode(std::ptr::null_mut()).as_node()
}

fn none6() -> (Option<Node>, Option<Node>, Option<Node>, Option<Node>, Option<Node>, Option<Node>) {
    (None, None, None, None, None, None)
}

#[test]
fn SC1_first_empty_sets_cp0_and_adds_section() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    assert!(sd.getPLCSection().unwrap().isEmpty());
    let mut imp = importer(sd);
    assert!(imp.mCpLastSectionEnd.is_none());
    let mut p = props();
    let n = none6();
    imp.doImportSectionPropertiesSimple(Some(&mut p), n.0, n.1, n.2, n.3, n.4, n.5);
    assert_eq!(imp.mCpLastSectionEnd, Some(0));
    let plc = sd.getPLCSection().unwrap();
    assert!(!plc.isEmpty());
    assert_eq!(plc.size(), 1);
}

#[test]
fn SC2_first_nonempty_does_not_add_section() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut p = props();
    let n = none6();
    imp.doImportSectionPropertiesSimple(Some(&mut p), n.0, n.1, n.2, n.3, n.4, n.5);
    assert_eq!(sd.getPLCSection().unwrap().size(), 1);
    imp.mCpLastSectionEnd = None;
    let mut p2 = props();
    imp.doImportSectionPropertiesSimple(Some(&mut p2), n.0, n.1, n.2, n.3, n.4, n.5);
    assert_eq!(imp.mCpLastSectionEnd, Some(0));
    assert_eq!(sd.getPLCSection().unwrap().size(), 1);
}

#[test]
fn SC3_later_adds_section_at_remembered_cp() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.mCpLastSectionEnd = Some(4);
    let mut p = props();
    let n = none6();
    imp.doImportSectionPropertiesSimple(Some(&mut p), n.0, n.1, n.2, n.3, n.4, n.5);
    let plc = sd.getPLCSection().unwrap();
    assert_eq!(plc.size(), 1);
    assert_eq!(plc.getLastSection().unwrap().position(), 4);
}

#[test]
fn SC4_sepx_is_frozen_section_props() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.mCpLastSectionEnd = Some(4);
    let mut p = props();
    let n = none6();
    imp.doImportSectionPropertiesSimple(Some(&mut p), n.0, n.1, n.2, n.3, n.4, n.5);
    let sepx = sd.getPLCSection().unwrap().getLastSection().unwrap().getPropertySet();
    assert!(sepx.contains(10));
    assert_eq!(sepx.getInt(10), 42);
}

#[test]
fn SC5_null_section_props_uses_empty() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.mCpLastSectionEnd = Some(4);
    let n = none6();
    imp.doImportSectionPropertiesSimple(None, n.0, n.1, n.2, n.3, n.4, n.5);
    let sepx = sd.getPLCSection().unwrap().getLastSection().unwrap().getPropertySet();
    let empty_ps: PropertySet = (*empty).clone();
    assert_eq!(sepx.size(), empty_ps.size());
}

#[test]
fn SC6_all_header_nodes_attach() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.mCpLastSectionEnd = Some(4);
    let mut p = props();
    let even_h = dummy_node();
    let odd_h = dummy_node();
    let even_f = dummy_node();
    let odd_f = dummy_node();
    let first_h = dummy_node();
    let first_f = dummy_node();
    imp.doImportSectionPropertiesSimple(
        Some(&mut p),
        Some(even_h),
        Some(odd_h),
        Some(even_f),
        Some(odd_f),
        Some(first_h),
        Some(first_f),
    );
    let node = sd.getPLCSection().unwrap().getLastSection().unwrap();
    assert!(node.getEvenHeader() == even_h);
    assert!(node.getOddHeader() == odd_h);
    assert!(node.getEvenFooter() == even_f);
    assert!(node.getOddFooter() == odd_f);
    assert!(node.getFirstHeader() == first_h);
    assert!(node.getFirstFooter() == first_f);
}

#[test]
fn SC7_partial_headers_skip_attach() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.mCpLastSectionEnd = Some(4);
    let mut p = props();
    let even_h = dummy_node();
    imp.doImportSectionPropertiesSimple(
        Some(&mut p),
        Some(even_h),
        Some(dummy_node()),
        Some(dummy_node()),
        Some(dummy_node()),
        Some(dummy_node()),
        None,
    );
    let node = sd.getPLCSection().unwrap().getLastSection().unwrap();
    assert!(node.getEvenHeader() != even_h);
}

#[test]
#[ignore = "spec: SectionPropsImporter"]
fn SC8_convert_document_grid() {
    let _ = 1827547294i32;
}
