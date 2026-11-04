// function_id=04_function_run_stream
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::layout::base::text::CpText::CpText;
use word_core::cn::wps::moffice::writer::layout::base::text::RunStream::RunStream;

fn with_main<R>(f: impl FnOnce(&mut dyn Document) -> R) -> R {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main");
    f(main)
}

fn chpx(v: i32) -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(10, v);
    m.freeze()
}

fn papx(v: i32) -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setInt(191, v);
    m.freeze()
}

fn append_para(
    doc: &mut dyn Document,
    body: &str,
    chpx: &util::moffice::property::PropertySet::PropertySet,
    papx: &util::moffice::property::PropertySet::PropertySet,
) {
    let start = doc.getLength();
    let mut s = String::from(body);
    s.push('\r');
    CoreHelper::appendText__string_ps(Some(doc), Some(&s), Some(chpx));
    CoreHelper::insertToPapxTable(Some(doc), start, Some(papx));
}

#[test]
fn RS1_empty() {
    with_main(|doc| {
        assert!(RunStream::collectMainStoryRuns(doc).is_empty());
    });
}

#[test]
fn RS2_one_run() {
    with_main(|doc| {
        append_para(doc, "Hi", &*empty, &*empty);
        let runs = RunStream::collectMainStoryRuns(doc);
        assert_eq!(runs.len(), 1);
        let concat: String = runs.iter().map(|r| r.text.as_str()).collect();
        assert_eq!(concat, CpText::getText(doc, 0, doc.getLength()));
    });
}

#[test]
fn RS3_chpx_split_same_para() {
    with_main(|doc| {
        let a = chpx(24);
        let b = chpx(48);
        CoreHelper::appendText__string_ps(Some(doc), Some("ab"), Some(&a));
        CoreHelper::appendText__string_ps(Some(doc), Some("cd"), Some(&b));
        let start = 0;
        CoreHelper::appendText__string_ps(Some(doc), Some("\r"), Some(&b));
        CoreHelper::insertToPapxTable(Some(doc), start, Some(&*empty));
        let runs = RunStream::collectMainStoryRuns(doc);
        assert!(runs.len() >= 2);
        assert_ne!(runs[0].chpx.getInt__int_int(10, 0), runs[1].chpx.getInt__int_int(10, 0));
    });
}

#[test]
fn RS4_papx_split() {
    with_main(|doc| {
        let p1 = papx(100);
        let p2 = papx(200);
        append_para(doc, "", &*empty, &p1);
        append_para(doc, "", &*empty, &p2);
        let runs = RunStream::collectMainStoryRuns(doc);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].papx.getInt__int_int(191, 0), 100);
        assert_eq!(runs[1].papx.getInt__int_int(191, 0), 200);
    });
}

#[test]
fn RS5_both_split() {
    with_main(|doc| {
        let a = chpx(24);
        let b = chpx(48);
        let p1 = papx(100);
        let p2 = papx(200);
        append_para(doc, "x", &a, &p1);
        append_para(doc, "y", &b, &p2);
        let runs = RunStream::collectMainStoryRuns(doc);
        assert_eq!(runs.len(), 2);
        assert_ne!(runs[0].chpx.getInt__int_int(10, 0), runs[1].chpx.getInt__int_int(10, 0));
        assert_ne!(runs[0].papx.getInt__int_int(191, 0), runs[1].papx.getInt__int_int(191, 0));
    });
}

#[test]
fn RS6_concat_equals_rope() {
    with_main(|doc| {
        append_para(doc, "Hello", &*empty, &*empty);
        let runs = RunStream::collectMainStoryRuns(doc);
        let concat: String = runs.iter().map(|r| r.text.as_str()).collect();
        assert_eq!(concat, CpText::getText(doc, 0, doc.getLength()));
        for w in runs.windows(2) {
            assert_eq!(w[0].end_cp, w[1].start_cp);
        }
    });
}

#[test]
fn RS7_cr_in_text() {
    with_main(|doc| {
        append_para(doc, "", &*empty, &*empty);
        let runs = RunStream::collectMainStoryRuns(doc);
        let concat: String = runs.iter().map(|r| r.text.as_str()).collect();
        assert!(concat.contains('\r'));
    });
}

#[test]
fn RS8_ff_preserved() {
    with_main(|doc| {
        append_para(doc, "A\u{000c}B", &*empty, &*empty);
        let runs = RunStream::collectMainStoryRuns(doc);
        let concat: String = runs.iter().map(|r| r.text.as_str()).collect();
        assert!(concat.contains('\u{000c}'));
    });
}

#[test]
fn RS9_no_zip_entry() {
    with_main(|doc| {
        let _ = RunStream::collectMainStoryRuns(doc);
    });
}
