// function_id=06_function_cpiecetable
// cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_cpiecetable
// src:  libs/word-core/src/cn/wps/moffice/writer/data/CPieceTable.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/data/CPieceTable.rs

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::collections::textrope::KNotifyTextRope::KNotifyTextRope;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::CPieceTable::CPieceTable;
use word_core::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use word_core::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use word_core::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
use word_core::cn::wps::moffice::writer::data::internal_v2::ChpxTable::ChpxTable;
use word_core::cn::wps::moffice::writer::data::internal_v2::PapxTable::PapxTable;

const PROP_STI: i32 = 191;

struct StorageDoc {
    length: i32,
    rope: *mut dyn ITextRope,
    chpx: *mut dyn IChpxTable,
    papx: *mut dyn IPapxTable,
}

impl IDocument for StorageDoc {
    fn getType(&self) -> i32 {
        0
    }
}

impl Document for StorageDoc {
    // rust: ★门禁修复 —— `Document::asAny`/`asAny_mut`（Java 强制类型转换入口，
    //   见 `core/Document.rs:410-415`）是**无默认体**的必需项；测试替身必须实现，
    //   否则测试目标无法编译（`E0046`），全量回归门禁形同虚设。
    // Java: 等价于 Java 的强制类型转换入口
    fn asAny(&self) -> &dyn std::any::Any {
        self
    }
    fn asAny_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn getLength(&self) -> i32 {
        self.length
    }
    fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
        if self.rope.is_null() {
            None
        } else {
            Some(self.rope)
        }
    }
    fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
        if self.chpx.is_null() {
            None
        } else {
            Some(self.chpx)
        }
    }
    fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
        if self.papx.is_null() {
            None
        } else {
            Some(self.papx)
        }
    }
}

fn null_rope() -> *mut dyn ITextRope {
    std::ptr::null_mut::<KNotifyTextRope>() as *mut dyn ITextRope
}
fn null_chpx() -> *mut dyn IChpxTable {
    std::ptr::null_mut::<ChpxTable>() as *mut dyn IChpxTable
}
fn null_papx() -> *mut dyn IPapxTable {
    std::ptr::null_mut::<PapxTable>() as *mut dyn IPapxTable
}

fn empty_doc(length: i32) -> StorageDoc {
    StorageDoc {
        length,
        rope: null_rope(),
        chpx: null_chpx(),
        papx: null_papx(),
    }
}

fn papx_sti(v: i32) -> util::moffice::property::PropertySet::PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_STI, v);
    mps.freeze()
}

#[test]
fn pt1_empty_construction_guessed_zero_and_null_triple() {
    let table = CPieceTable::CPieceTable__empty();
    assert_eq!(table.getGuessedTotalLength(), 0);
    assert!(table.getTextRope().is_none());
    assert!(table.getChpxTable().is_none());
    assert!(table.getPapxTable().is_none());
}

#[test]
fn pt2_guessed_total_length_roundtrip() {
    let mut table = CPieceTable::CPieceTable__empty();
    table.setGuessedTotalLength(1234);
    assert_eq!(table.getGuessedTotalLength(), 1234);
}

#[test]
fn pt3_bind_from_document() {
    let mut doc = empty_doc(1);
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, 'A');
    let mut chpx = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let mut papx = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    doc.rope = &mut rope as *mut dyn ITextRope;
    doc.chpx = &mut chpx as *mut dyn IChpxTable;
    doc.papx = &mut papx as *mut dyn IPapxTable;
    let table = CPieceTable::CPieceTable(&mut doc as *mut dyn Document);
    assert!(table.getTextRope().is_some());
    assert!(table.getChpxTable().is_some());
    assert!(table.getPapxTable().is_some());
    assert_eq!(table.getTextRope(), Some(doc.rope));
    assert_eq!(table.getChpxTable(), Some(doc.chpx));
    assert_eq!(table.getPapxTable(), Some(doc.papx));
}

#[test]
fn pt4_charat_delegates_to_rope() {
    let mut doc = empty_doc(1);
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, 'A');
    doc.rope = &mut rope as *mut dyn ITextRope;
    let mut table = CPieceTable::CPieceTable(&mut doc as *mut dyn Document);
    assert_eq!(table.charAt(0), 'A');
}

#[test]
fn pt5_dispose_clears_triple() {
    let mut doc = empty_doc(1);
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__char(&mut rope, 'A');
    let mut chpx = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let mut papx = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    doc.rope = &mut rope as *mut dyn ITextRope;
    doc.chpx = &mut chpx as *mut dyn IChpxTable;
    doc.papx = &mut papx as *mut dyn IPapxTable;
    let mut table = CPieceTable::CPieceTable(&mut doc as *mut dyn Document);
    table.dispose();
    assert!(table.getTextRope().is_none());
    assert!(table.getChpxTable().is_none());
    assert!(table.getPapxTable().is_none());
}

#[test]
fn pt6_paragraph_range_from_papx() {
    let mut doc = empty_doc(3);
    let mut papx = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let ps = papx_sti(0);
    let _ = papx.insert_entry(0, Some(&ps));
    doc.papx = &mut papx as *mut dyn IPapxTable;
    let table = CPieceTable::CPieceTable(&mut doc as *mut dyn Document);
    let rg = table.getParagraphRange__int(1);
    assert_eq!(RANGEFactory::getStart(rg), 0);
    assert_eq!(RANGEFactory::getEnd(rg), 3);
}

#[test]
fn pt7_paragraph_range_without_papx_is_zero() {
    let table = CPieceTable::CPieceTable__empty();
    assert_eq!(table.getParagraphRange__int(0), 0);
}

#[test]
fn pt8_null_document_construction() {
    let null_doc: *mut dyn Document = std::ptr::null_mut::<StorageDoc>() as *mut dyn Document;
    let table = CPieceTable::CPieceTable(null_doc);
    assert!(table.getTextRope().is_none());
    assert!(table.getChpxTable().is_none());
    assert!(table.getPapxTable().is_none());
    assert_eq!(table.getGuessedTotalLength(), 0);
}
