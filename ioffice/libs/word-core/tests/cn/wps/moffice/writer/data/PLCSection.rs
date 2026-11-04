// function_id=05_function_plcsection
// cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_plcsection
// src:  libs/word-core/src/cn/wps/moffice/writer/data/{PLCSection,PLC}.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/data/PLCSection.rs

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::PLCSection::PLCSection;
use word_core::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;

const INVALID_CP: i32 = i32::MIN;
const FORM_FEED: char = '\u{000C}';

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

#[test]
fn sc1_set_last_section_end_cp_is_section_end() {
    let _ = FORM_FEED; // 0x0C: importer passes getLength() after writing this char
    let mut doc = StorageDoc { length: 10 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    table.setLastSectionEndCP(4);
    let last = table.getLastSection().expect("last section");
    assert_eq!(table.getSectionEnd(&last), 4);
}

#[test]
fn sc2_unset_end_cp_uses_document_length() {
    assert_eq!(INVALID_CP, i32::MIN);
    let mut doc = StorageDoc { length: 8 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    let last = table.getLastSection().expect("last section");
    assert_eq!(table.getSectionEnd(&last), doc.getLength());
}

#[test]
fn sc3_on_end_load_resets_invalid_cp() {
    let mut doc = StorageDoc { length: 8 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    table.setLastSectionEndCP(99);
    table.onEndLoad();
    let last = table.getLastSection().expect("last section");
    assert_eq!(table.getSectionEnd(&last), doc.getLength());
}

#[test]
fn sc4_add_section_on_empty_tree() {
    let mut doc = StorageDoc { length: 0 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    assert!(table.getFirstSection().is_some());
}

#[test]
fn sc5_add_section_cp0_nonempty_insert_at_upper_bound() {
    let mut doc = StorageDoc { length: 0 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    table.addSection(0);
    assert_eq!(table.size(), 2);
}

#[test]
fn sc6_get_section_hits() {
    let mut doc = StorageDoc { length: 0 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    assert!(table.getSection(0).is_some());
}

#[test]
fn sc7_get_section_empty_is_none() {
    let table = PLCSection::new(None);
    assert!(table.getSection(0).is_none());
}

#[test]
fn sc8_first_section_start_is_position() {
    let mut doc = StorageDoc { length: 0 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    let first = table.addSection(0);
    assert_eq!(first.position(), 0);
    assert_eq!(table.getSectionStart(&first), 0);
}

#[test]
fn sc9_non_first_section_start_skips_form_feed() {
    let mut doc = StorageDoc { length: 10 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    let second = table.addSection(3);
    assert_eq!(second.position(), 3);
    assert_eq!(table.getSectionStart(&second), 4);
}

#[test]
fn sc10_range_factory_packs_start_in_high_dword() {
    let mut doc = StorageDoc { length: 10 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    let first = table.addSection(0);
    table.setLastSectionEndCP(10);
    let rg = table.getSectionRange(&first);
    assert_eq!(RANGEFactory::getStart(rg), 0);
    assert_eq!(RANGEFactory::getEnd(rg), 10);
    assert_eq!(rg, RANGEFactory::getRANGE(0, 10));
}

#[test]
fn sc11_do_on_insert_text_shifts_following_section() {
    let mut doc = StorageDoc { length: 10 };
    let mut table = PLCSection::PLCSection(Some(&doc));
    table.addSection(0);
    let second = table.addSection(5);
    assert_eq!(second.position(), 5);
    table.doOnInsertText(0, 2);
    assert_eq!(second.position(), 7);
}
