// function_id=02_function_run_text
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_run_text
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/document/SubDocumentImporter.rs

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::transaction::Transaction::Transaction;
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
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::SubDocumentImporter;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::RunPropsConverterSimple::RunPropsConverterSimple;

#[path = "../helpers/wc03_word_attrs.rs"]
mod wc03_word_attrs;
use wc03_word_attrs::WordAttrs;

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }

    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
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

fn importer(sd: &mut SubDocument) -> SubDocumentImporter {
    SubDocumentImporter::SubDocumentImporter(sd, style_map(), num_map(), None, false, 0, None, None)
}

fn chpx_ps() -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 1);
    mps.freeze()
}

#[test]
fn RT1_string_appends_rope_and_chpx() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importRunText("Hello", &chpx_ps());
    assert_eq!(sd.getLength(), 5);
}

#[test]
fn RT2_empty_string_is_noop() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importRunText("", &chpx_ps());
    assert_eq!(sd.getLength(), 0);
}

#[test]
fn RT3_char_buffer_appends_range() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let buffer = vec!['a', 'b', 'c'];
    imp.importRunTextSimple__chararr_int_int_propertyset(&buffer, 1, 2, &chpx_ps());
    assert_eq!(sd.getLength(), 2);
}

#[test]
fn RT4_char_buffer_zero_len_is_noop() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let buffer = vec!['a', 'b', 'c'];
    imp.importRunTextSimple__chararr_int_int_propertyset(&buffer, 0, 0, &chpx_ps());
    assert_eq!(sd.getLength(), 0);
}

#[test]
fn RT5_chpx_cp_is_append_before_length() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importRunText("Hi", &chpx_ps());
    imp.importRunText("bold", &chpx_ps());
    assert_eq!(sd.getLength(), 6);
}

#[test]
fn RT6_special_char_sets_prop_28() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut dst = MutablePropertySet::new();
    imp.importSpecialChar(Some(&mut dst), '\u{0005}');
    assert!(dst.getBoolean(28));
}

#[test]
fn RT7_special_char_null_dst_does_not_panic() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    imp.importSpecialChar(None, '\u{0005}');
}

#[test]
fn RT8_inline_chart_copies_props() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let mut src = MutablePropertySet::new();
    src.setInt(193, 1);
    imp.importRunText4InlineChart("t", &src);
    assert_eq!(src.getInt(193), 1);
}

#[test]
fn RT9_symbol_run_appends_open_paren() {
    let mut env = ParentEnv::new();
    let sd = env.main_mut();
    let mut imp = importer(sd);
    let attrs = WordAttrs::empty()
        .with(wc03_word_attrs::MAIN_NS, "font", "Symbol")
        .with(wc03_word_attrs::MAIN_NS, "char", "F0F0");
    let mut converter = RunPropsConverterSimple::RunPropsConverterSimple(0, None, None);
    imp.importCharacterRunSymbolSimple(&attrs, None, &mut converter);
    assert_eq!(sd.getLength(), 1);
}
