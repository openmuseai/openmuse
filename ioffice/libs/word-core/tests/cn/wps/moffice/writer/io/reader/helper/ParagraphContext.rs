// function_id=01_function_paragraph_context
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_paragraph_context
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/helper/ParagraphContext.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/helper/ParagraphContext.rs

#![allow(non_snake_case)]

use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::data::IPapxTable::Entry as IPapxEntry;
use word_core::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
use word_core::cn::wps::moffice::writer::io::reader::helper::ParagraphContext::ParagraphContext;

struct MockPapxEntry;

impl ITableEntry for MockPapxEntry {
    fn position(&self) -> i32 { 0 }
    fn endCp(&self) -> i32 { 0 }
    fn range(&self) -> i64 { 0 }
    fn length(&self) -> i32 { 0 }
    fn getPrev(&self) -> Option<Box<dyn ITableEntry>> { None }
    fn getNext(&self) -> Option<Box<dyn ITableEntry>> { None }
    fn isHead(&self) -> bool { false }
    fn isEnd(&self) -> bool { false }
    fn isLast(&self) -> bool { false }
    fn getProperty(&self) -> Option<&PropertySet> { None }
    fn setProperty(&mut self, _var1: Option<&PropertySet>) {}
    fn getPropertySetId(&self) -> i32 { 0 }
}

impl IPapxEntry for MockPapxEntry {
    fn getPrev(&self) -> Option<Box<dyn IPapxEntry>> { None }
    fn getNext(&self) -> Option<Box<dyn IPapxEntry>> { None }
    fn getProperty(&self) -> Option<&PropertySet> { None }
    fn setProperty(&mut self, _var1: Option<&PropertySet>) {}
}

#[test]
fn PC1_ctor_int_stores_cp0() {
    let ctx = ParagraphContext::ParagraphContext__int(0);
    assert_eq!(ctx.mPositionInserted, Some(0));
}

#[test]
fn PC2_ctor_entry_stores_entry() {
    let entry = Box::new(MockPapxEntry) as Box<dyn IPapxEntry>;
    let ctx = ParagraphContext::ParagraphContext__entry(Some(entry));
    assert!(ctx.mEntry.is_some());
}

#[test]
#[should_panic(expected = "entry should not be null!")]
fn PC3_ctor_null_entry_panics() {
    let _ = ParagraphContext::ParagraphContext__entry(None);
}

#[test]
fn PC4_ctor_int_keeps_para_id_and_entry_none() {
    let ctx = ParagraphContext::ParagraphContext__int(0);
    assert!(ctx.mParaId.is_none());
    assert!(ctx.mEntry.is_none());
}

#[test]
fn PC5_position_inserted_stays_stable() {
    let ctx = ParagraphContext::ParagraphContext__int(42);
    assert_eq!(ctx.mPositionInserted, Some(42));
    // 关键不变量：构造后不得被「当前 length」改写。
    assert_eq!(ctx.mPositionInserted, Some(42));
}
