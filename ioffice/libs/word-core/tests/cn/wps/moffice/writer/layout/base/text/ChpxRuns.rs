// function_id=02_function_chpx_runs
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::layout::base::text::ChpxRuns::ChpxRuns;

fn with_main<R>(f: impl FnOnce(&mut dyn Document) -> R) -> R {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main");
    f(main)
}

fn ps(v: i32) -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(10, v);
    m.freeze()
}

fn walk(doc: &dyn Document, start: i32, end: i32) -> Vec<(i32, i32, i32)> {
    let t = doc.getChpxTable().expect("chpx");
    let spans = unsafe { ChpxRuns::walkChpx(&*t, start, end) };
    spans
        .into_iter()
        .map(|s| (s.start_cp, s.end_cp, s.property.getInt__int_int(10, 0)))
        .collect()
}

#[test]
fn CX1_empty() {
    with_main(|doc| {
        let spans = walk(doc, 0, doc.getLength());
        assert!(spans.is_empty() || doc.getLength() == 0);
    });
}

#[test]
fn CX2_one_run() {
    with_main(|doc| {
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&*empty));
        let spans = walk(doc, 0, doc.getLength());
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].0, 0);
        assert_eq!(spans[0].1, 2);
    });
}

#[test]
fn CX3_split_on_chpx_change() {
    with_main(|doc| {
        let a = ps(24);
        let b = ps(48);
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&a));
        CoreHelper::appendText__string_ps(Some(doc), Some("cd"), Some(&b));
        let spans = walk(doc, 0, doc.getLength());
        assert_eq!(spans.len(), 2);
        assert_eq!((spans[0].0, spans[0].1, spans[0].2), (0, 2, 24));
        assert_eq!((spans[1].0, spans[1].1, spans[1].2), (2, 4, 48));
    });
}

#[test]
fn CX4_clip_left() {
    with_main(|doc| {
        let a = ps(24);
        let b = ps(48);
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&a));
        CoreHelper::appendText__string_ps(Some(doc), Some("cd"), Some(&b));
        let spans = walk(doc, 1, 4);
        assert_eq!(spans[0].0, 1);
        assert_eq!(spans[0].1, 2);
    });
}

#[test]
fn CX5_clip_right() {
    with_main(|doc| {
        let a = ps(24);
        let b = ps(48);
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&a));
        CoreHelper::appendText__string_ps(Some(doc), Some("cd"), Some(&b));
        let spans = walk(doc, 0, 3);
        let last = spans.last().unwrap();
        assert_eq!(last.1, 3);
    });
}

#[test]
fn CX6_empty_window() {
    with_main(|doc| {
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&*empty));
        assert!(walk(doc, 1, 1).is_empty());
    });
}

#[test]
fn CX7_union_covers_window() {
    with_main(|doc| {
        let a = ps(24);
        let b = ps(48);
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&a));
        CoreHelper::appendText__string_ps(Some(doc), Some("cd"), Some(&b));
        let end = doc.getLength();
        let spans = walk(doc, 0, end);
        assert_eq!(spans.first().unwrap().0, 0);
        assert_eq!(spans.last().unwrap().1, end);
        for w in spans.windows(2) {
            assert_eq!(w[0].1, w[1].0);
        }
    });
}
