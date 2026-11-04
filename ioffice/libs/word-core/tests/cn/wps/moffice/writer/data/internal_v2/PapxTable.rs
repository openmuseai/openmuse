// function_id=04_function_papx_insert
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_papx_insert
// src:  libs/word-core/src/cn/wps/moffice/writer/data/internal_v2/{PapxTable,PapxTableFactory}.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/data/internal_v2/PapxTable.rs

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::internal_v2::PapxTable::PapxTable;
use word_core::cn::wps::moffice::writer::data::internal_v2::PapxTableFactory::PapxTableFactory;

const PROP_STI: i32 = 191;
const PROP_ROW_END: i32 = 225;

struct StorageDoc {
    length: i32,
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
}

fn papx_sti(v: i32) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_STI, v);
    mps.freeze()
}

fn papx_row_end() -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setBoolean(PROP_ROW_END, true);
    mps.freeze()
}

#[test]
fn px1_factory_null_document_returns_none() {
    let null_doc: *mut dyn Document = std::ptr::null_mut::<StorageDoc>() as *mut dyn Document;
    assert!(PapxTableFactory::createPapxTable(null_doc).is_none());
}

#[test]
fn px2_factory_non_null_returns_table() {
    let mut doc = StorageDoc { length: 0 };
    assert!(PapxTableFactory::createPapxTable(&mut doc as *mut dyn Document).is_some());
}

#[test]
fn px3_insert_at_start_roundtrip_sti() {
    let mut doc = StorageDoc { length: 0 };
    let mut table = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let ps = papx_sti(1);
    let e = table.insert_entry(0, Some(&ps));
    assert_eq!(e.position(), 0);
    assert_eq!(e.getProperty().getInt(PROP_STI), 1);
}

#[test]
fn px4_second_paragraph_inserts_at_cp0_not_endmark() {
    let mut doc = StorageDoc { length: 6 };
    let mut table = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let a = papx_sti(1);
    let b = papx_sti(2);
    table.insert_entry(0, Some(&a));
    let second = table.insert_entry(3, Some(&b));
    assert_eq!(second.position(), 3);
    assert_ne!(second.position(), 4);
    assert_eq!(second.getProperty().getInt(PROP_STI), 2);
}

#[test]
fn px5_seek_inside_paragraph_returns_start_entry() {
    let mut doc = StorageDoc { length: 3 };
    let mut table = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let ps = papx_sti(1);
    table.insert_entry(0, Some(&ps));
    let inner = table.seek_entry(1);
    assert_eq!(inner.position(), 0);
    assert_eq!(inner.getProperty().getInt(PROP_STI), 1);
}

#[test]
fn px6_boolean_225_roundtrip() {
    let mut doc = StorageDoc { length: 1 };
    let mut table = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let ps = papx_row_end();
    let e = table.insert_entry(0, Some(&ps));
    assert!(e.getProperty().getBoolean_default(PROP_ROW_END, false));
}

#[test]
fn px7_before_remove_text_same_entry_erases_space() {
    let mut doc = StorageDoc { length: 4 };
    let mut table = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let ps = papx_sti(1);
    table.insert_entry(0, Some(&ps));
    assert_eq!(table.size(), 1);
    table.beforeRemoveText(1, 3);
    assert_eq!(table.size(), 1, "same-entry remove must not drop the paragraph node");
    let remaining = table.seek_entry(0);
    assert!(!remaining.isHead());
    assert_eq!(remaining.position(), 0);
    assert_eq!(remaining.getProperty().getInt(PROP_STI), 1);
}

#[test]
fn px8_before_remove_text_drops_middle_paragraph() {
    let mut doc = StorageDoc { length: 6 };
    let mut table = PapxTable::PapxTable(&mut doc as *mut dyn Document);
    let a = papx_sti(1);
    let b = papx_sti(2);
    table.insert_entry(0, Some(&a));
    table.insert_entry(3, Some(&b));
    assert_eq!(table.size(), 2);
    table.beforeRemoveText(0, 3);
    assert_eq!(table.size(), 1);
    let remaining = table.seek_entry(0);
    assert!(!remaining.isHead());
    assert_eq!(remaining.getProperty().getInt(PROP_STI), 2);
}
