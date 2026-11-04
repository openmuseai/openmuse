// function_id=03_function_chpx_insert
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_chpx_insert
// src:  libs/word-core/src/cn/wps/moffice/writer/data/internal_v2/{ChpxTable,ChpxTableFactory,CPTable}.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/data/internal_v2/ChpxTable.rs

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::internal_v2::ChpxTable::ChpxTable;
use word_core::cn::wps::moffice::writer::data::internal_v2::ChpxTableFactory::ChpxTableFactory;

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

fn ps_int(id: i32, v: i32) -> util::moffice::property::PropertySet::PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(id, v);
    mps.freeze()
}

#[test]
fn cx1_factory_null_document_returns_none() {
    let null_doc: *mut dyn Document = std::ptr::null_mut::<StorageDoc>() as *mut dyn Document;
    assert!(ChpxTableFactory::createChpxTable(null_doc).is_none());
}

#[test]
fn cx2_factory_non_null_returns_table() {
    let mut doc = StorageDoc { length: 0 };
    let table = ChpxTableFactory::createChpxTable(&mut doc as *mut dyn Document);
    assert!(table.is_some());
}

#[test]
fn cx3_insert_at_cp0_roundtrip_property() {
    let mut doc = StorageDoc { length: 0 };
    let mut table = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let ps = ps_int(193, 1);
    let e = table.insert_entry(0, Some(&ps));
    assert_eq!(e.position(), 0);
    assert_eq!(e.getProperty().getInt(193), 1);
    let sought = table.seek_entry(0);
    assert_eq!(sought.position(), 0);
    assert_eq!(sought.getProperty().getInt(193), 1);
}

#[test]
fn cx4_seek_inside_interval_returns_covering_entry() {
    let mut doc = StorageDoc { length: 3 };
    let mut table = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let ps = ps_int(193, 1);
    let inserted = table.insert_entry(0, Some(&ps));
    assert_eq!(inserted.position(), 0);
    let inner = table.seek_entry(1);
    assert_eq!(inner.position(), 0);
    assert_eq!(inner.getProperty().getInt(193), 1);
}

#[test]
fn cx5_after_insert_text_shifts_following_entry() {
    let mut doc = StorageDoc { length: 10 };
    let mut table = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let ps = ps_int(193, 1);
    let e = table.insert_entry(5, Some(&ps));
    assert_eq!(e.position(), 5);
    table.afterInsertText(0, 0, 2);
    let shifted = table.seek_entry(7);
    assert_eq!(shifted.position(), 7);
    assert_eq!(shifted.getProperty().getInt(193), 1);
}

#[test]
fn cx6_after_insert_text_skips_head() {
    let mut doc = StorageDoc { length: 4 };
    let mut table = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    assert_eq!(table.size(), 0);
    table.afterInsertText(4, 0, 2);
    assert_eq!(table.size(), 0);
    assert!(table.begin_entry().isHead() || table.begin_entry() == table.end_entry());
}

#[test]
fn cx7_before_remove_text_drops_covered_entries() {
    let mut doc = StorageDoc { length: 4 };
    let mut table = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let a = ps_int(193, 1);
    let b = ps_int(193, 2);
    table.insert_entry(0, Some(&a));
    table.insert_entry(2, Some(&b));
    assert_eq!(table.size(), 2);
    table.beforeRemoveText(0, 2);
    assert_eq!(table.size(), 1);
    let remaining = table.seek_entry(0);
    assert!(!remaining.isHead());
    assert_eq!(remaining.getProperty().getInt(193), 2);
}

#[test]
fn cx8_before_remove_text_splits_interior() {
    let mut doc = StorageDoc { length: 4 };
    let mut table = ChpxTable::ChpxTable(&mut doc as *mut dyn Document);
    let ps = ps_int(193, 1);
    table.insert_entry(0, Some(&ps));
    assert_eq!(table.size(), 1);
    table.beforeRemoveText(1, 3);
    assert!(table.size() >= 2, "interior remove must split remnants, not drop the whole run; size={}", table.size());
    let left = table.seek_entry(0);
    assert_eq!(left.position(), 0);
    assert_eq!(left.getProperty().getInt(193), 1);
    assert!(!table.seek_entry(0).isHead());
}
