// function_id=03_function_papx_paras
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::layout::base::text::PapxParas::PapxParas;

fn with_main<R>(f: impl FnOnce(&mut dyn Document) -> R) -> R {
    // rust: ★P0 —— 必须用**堆上**的 `TextDocument`。
    //   `TextDocument::TextDocument()` 按值返回，`owned` 位于栈上；
    //   而 `createSubDocument` 会把 `mParent` 与 Chpx/Papx 两张表的
    //   `document` 裸指针钉在**当时的 `self` 地址**上
    //   （见 `TextDocument.rs:1102` 的 `new()`：同一原因要求先 `Box` 再 `_create`）。
    //   一旦 `owned` 被移动/重借用，这些指针即悬垂，
    //   `beforeAddReference → document.getStyles()` 读到野指针
    //   ⇒ `misaligned pointer dereference`（实测地址退化为 `0x1`）。
    //   改用 `TextDocument::new()`（`Box` + `_create`），地址稳定。
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main");
    f(main)
}

fn ps(v: i32) -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(191, v);
    m.freeze()
}

fn walk(doc: &dyn Document, start: i32, end: i32) -> Vec<(i32, i32, i32)> {
    let t = doc.getPapxTable().expect("papx");
    let spans = unsafe { PapxParas::walkPapx(&*t, start, end) };
    spans
        .into_iter()
        .map(|s| (s.start_cp, s.end_cp, s.property.getInt__int_int(191, 0)))
        .collect()
}

#[test]
fn PX1_empty() {
    with_main(|doc| {
        let spans = walk(doc, 0, doc.getLength());
        assert!(spans.is_empty() || doc.getLength() == 0);
    });
}

#[test]
fn PX2_one_para() {
    with_main(|doc| {
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&*empty));
        let spans = walk(doc, 0, doc.getLength());
        assert_eq!(spans.len(), 1);
        assert!(spans[0].1 > spans[0].0);
    });
}

#[test]
fn PX3_two_paras() {
    with_main(|doc| {
        let p1 = ps(100);
        let p2 = ps(200);
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&p1));
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&p2));
        let spans = walk(doc, 0, doc.getLength());
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].2, 100);
        assert_eq!(spans[1].2, 200);
    });
}

#[test]
fn PX4_clip() {
    with_main(|doc| {
        let p1 = ps(100);
        let p2 = ps(200);
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&p1));
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&p2));
        let spans = walk(doc, 1, doc.getLength());
        assert_eq!(spans[0].0, 1);
    });
}

#[test]
fn PX5_empty_window() {
    with_main(|doc| {
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&*empty));
        assert!(walk(doc, 0, 0).is_empty());
    });
}

#[test]
fn PX6_union_covers() {
    with_main(|doc| {
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&*empty));
        CoreHelper::appendParagraph__char_ps_ps(Some(doc), '\r', Some(&*empty), Some(&*empty));
        let end = doc.getLength();
        let spans = walk(doc, 0, end);
        assert!(!spans.is_empty());
        assert_eq!(spans.first().unwrap().0, 0);
        assert_eq!(spans.last().unwrap().1, end);
        for w in spans.windows(2) {
            assert_eq!(w[0].1, w[1].0);
        }
    });
}
