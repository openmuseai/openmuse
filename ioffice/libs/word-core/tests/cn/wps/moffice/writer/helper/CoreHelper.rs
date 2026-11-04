// function_id=07_function_corehelper_write
// cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_corehelper_write
// src:  libs/word-core/src/cn/wps/moffice/writer/helper/CoreHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/helper/CoreHelper.rs

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::collections::textrope::KNotifyTextRope::KNotifyTextRope;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use word_core::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use word_core::cn::wps::moffice::writer::data::internal_v2::ChpxTable::ChpxTable;
use word_core::cn::wps::moffice::writer::data::internal_v2::PapxTable::PapxTable;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;

const PROP_STI: i32 = 191;
const PROP_ROW_END: i32 = 225;
const CR: char = '\u{000D}';
const FF: char = '\u{000C}';
const CELL_MARK: char = '\u{0007}';

struct StorageDoc {
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
        if self.rope.is_null() {
            0
        } else {
            unsafe { (*self.rope).length() }
        }
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

fn chpx_ps() -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(193, 1);
    mps.freeze()
}

fn papx_sti() -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_STI, 0);
    mps.freeze()
}

fn papx_row_end() -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setBoolean(PROP_ROW_END, true);
    mps.freeze()
}

struct Harness {
    rope: KNotifyTextRope,
    chpx: ChpxTable,
    papx: PapxTable,
    doc: StorageDoc,
}

impl Harness {
    fn uninit() -> Self {
        Self {
            rope: KNotifyTextRope::default(),
            chpx: ChpxTable::default(),
            papx: PapxTable::default(),
            doc: StorageDoc {
                rope: std::ptr::null_mut::<KNotifyTextRope>() as *mut dyn ITextRope,
                chpx: std::ptr::null_mut::<ChpxTable>() as *mut dyn IChpxTable,
                papx: std::ptr::null_mut::<PapxTable>() as *mut dyn IPapxTable,
            },
        }
    }

    fn setup(&mut self) {
        self.chpx = ChpxTable::ChpxTable(&mut self.doc as *mut dyn Document);
        self.papx = PapxTable::PapxTable(&mut self.doc as *mut dyn Document);
        self.bind();
    }

    fn bind(&mut self) {
        self.doc.rope = &mut self.rope as *mut dyn ITextRope;
        self.doc.chpx = &mut self.chpx as *mut dyn IChpxTable;
        self.doc.papx = &mut self.papx as *mut dyn IPapxTable;
    }

    fn doc_mut(&mut self) -> &mut dyn Document {
        self.bind();
        &mut self.doc
    }
}

#[test]
fn ch1_append_text_returns_pre_append_length() {
    let mut h = Harness::uninit();
    h.setup();
    let ps = chpx_ps();
    let ret = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("Hi"), Some(&ps));
    assert_eq!(ret, 0);
    assert_eq!(ITextRope::length(&h.rope), 2);
    assert_eq!(ITextRope::charAt(&mut h.rope, 0), 'H');
    assert_eq!(ITextRope::charAt(&mut h.rope, 1), 'i');
    let e = IChpxTable::seek(&h.chpx, 0).expect("chpx");
    assert_eq!(e.position(), 0);
}

#[test]
fn ch2_second_append_returns_old_length() {
    let mut h = Harness::uninit();
    h.setup();
    let ps = chpx_ps();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("Hi"), Some(&ps));
    let ret = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("!"), Some(&ps));
    assert_eq!(ret, 2);
    let e = IChpxTable::seek(&h.chpx, 2).expect("chpx");
    assert_eq!(e.position(), 2);
}

#[test]
fn ch3_append_char_matches_string() {
    let mut h = Harness::uninit();
    h.setup();
    let ps = chpx_ps();
    let ret = CoreHelper::appendText__char_ps(Some(h.doc_mut()), 'A', Some(&ps));
    assert_eq!(ret, 0);
    assert_eq!(ITextRope::length(&h.rope), 1);
    assert_eq!(ITextRope::charAt(&mut h.rope, 0), 'A');
}

#[test]
fn ch4_append_chararr_uses_start_end() {
    let mut h = Harness::uninit();
    h.setup();
    let ps = chpx_ps();
    let buf = ['a', 'b', 'c'];
    let ret = CoreHelper::appendText__chararr_int_int_ps(
        Some(h.doc_mut()),
        Some(&buf),
        0,
        2,
        Some(&ps),
    );
    assert_eq!(ret, 0);
    assert_eq!(ITextRope::length(&h.rope), 2);
    assert_eq!(ITextRope::charAt(&mut h.rope, 0), 'a');
    assert_eq!(ITextRope::charAt(&mut h.rope, 1), 'b');
}

#[test]
fn ch5_append_paragraph_with_cp0_hangs_papx_at_start() {
    let mut h = Harness::uninit();
    h.setup();
    let chpx = chpx_ps();
    let papx = papx_sti();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("Hi"), Some(&chpx));
    let e = CoreHelper::appendParagraph__char_int_ps_ps(
        Some(h.doc_mut()),
        CR,
        0,
        Some(&chpx),
        Some(&papx),
    )
    .expect("papx");
    assert_eq!(e.position(), 0);
    assert_eq!(ITextRope::charAt(&mut h.rope, 2), CR);
}

#[test]
fn ch6_append_paragraph_without_cp_uses_endmark_cp() {
    let mut h = Harness::uninit();
    h.setup();
    let chpx = chpx_ps();
    let papx = papx_sti();
    let e = CoreHelper::appendParagraph__char_ps_ps(Some(h.doc_mut()), CR, Some(&chpx), Some(&papx))
        .expect("papx");
    assert_eq!(e.position(), 0);
    assert_eq!(ITextRope::charAt(&mut h.rope, 0), CR);
}

#[test]
fn ch7_form_feed_is_only_a_character() {
    let mut h = Harness::uninit();
    h.setup();
    let ps = chpx_ps();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("\u{000C}"), Some(&ps));
    assert_eq!(ITextRope::charAt(&mut h.rope, 0), FF);
    assert!(h.papx.isEmpty());
}

#[test]
#[should_panic(expected = "text.length() > 0 should be true!")]
fn ch8_empty_string_asserts() {
    let mut h = Harness::uninit();
    h.setup();
    let ps = chpx_ps();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some(""), Some(&ps));
}

#[test]
fn ch9_none_document_returns_zero() {
    let ps = chpx_ps();
    assert_eq!(
        CoreHelper::appendText__string_ps(None, Some("a"), Some(&ps)),
        0
    );
}

#[test]
fn ch10_supply_end_para_on_empty_head() {
    let mut h = Harness::uninit();
    h.setup();
    CoreHelper::supplyEndPara(Some(h.doc_mut()));
    assert_eq!(ITextRope::length(&h.rope), 1);
    assert_eq!(ITextRope::charAt(&mut h.rope, 0), CR);
    let e = IPapxTable::seek(&h.papx, 0).expect("papx");
    assert_eq!(e.position(), 0);
}

#[test]
fn ch11_supply_end_para_on_row_end() {
    let mut h = Harness::uninit();
    h.setup();
    let chpx = chpx_ps();
    let row = papx_row_end();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("Hi"), Some(&chpx));
    let _ = CoreHelper::insertToPapxTable(Some(h.doc_mut()), 0, Some(&row));
    let before = ITextRope::length(&h.rope);
    CoreHelper::supplyEndPara(Some(h.doc_mut()));
    assert_eq!(ITextRope::length(&h.rope), before + 1);
    assert_eq!(ITextRope::charAt(&mut h.rope, before), CR);
}

#[test]
fn ch12_supply_end_para_skips_normal_last() {
    let mut h = Harness::uninit();
    h.setup();
    let chpx = chpx_ps();
    let papx = papx_sti();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("Hi"), Some(&chpx));
    let _ = CoreHelper::appendParagraph__char_int_ps_ps(
        Some(h.doc_mut()),
        CR,
        0,
        Some(&chpx),
        Some(&papx),
    );
    let before = ITextRope::length(&h.rope);
    CoreHelper::supplyEndPara(Some(h.doc_mut()));
    assert_eq!(ITextRope::length(&h.rope), before);
}

#[test]
fn ch13_set_cell_mark_writes_0x07() {
    let mut h = Harness::uninit();
    h.setup();
    let chpx = chpx_ps();
    let papx = papx_sti();
    let _ = CoreHelper::appendText__string_ps(Some(h.doc_mut()), Some("abc"), Some(&chpx));
    let e = CoreHelper::appendParagraph__char_int_ps_ps(
        Some(h.doc_mut()),
        CR,
        0,
        Some(&chpx),
        Some(&papx),
    )
    .expect("papx");
    assert_eq!(e.endCp(), 4);
    CoreHelper::setCellMark(CELL_MARK, Some(h.doc_mut()), Some(e.as_ref()));
    assert_eq!(ITextRope::charAt(&mut h.rope, 3), CELL_MARK);
}
