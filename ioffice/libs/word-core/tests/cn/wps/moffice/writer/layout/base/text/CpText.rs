// function_id=01_function_cp_text
#![allow(non_snake_case)]

use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::layout::base::text::CpText::CpText;

fn with_main<R>(f: impl FnOnce(&mut dyn Document) -> R) -> R {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main");
    f(main)
}

fn append(doc: &mut dyn Document, s: &str) {
    CoreHelper::appendText__string_ps(Some(doc), Some(s), Some(&*empty));
}

#[test]
fn CT1_empty_document() {
    with_main(|doc| {
        assert_eq!(CpText::getText(doc, 0, 0), "");
        assert_eq!(CpText::getText(doc, 0, doc.getLength()), "");
    });
}

#[test]
fn CT2_full_range() {
    with_main(|doc| {
        append(doc, "Hi");
        assert_eq!(CpText::getText(doc, 0, doc.getLength()), "Hi");
    });
}

#[test]
fn CT3_slice() {
    with_main(|doc| {
        append(doc, "Hello");
        assert_eq!(CpText::getText(doc, 1, 4), "ell");
    });
}

#[test]
fn CT4_cr_preserved() {
    with_main(|doc| {
        append(doc, "A\r");
        assert_eq!(CpText::getText(doc, 0, 2), "A\r");
    });
}

#[test]
fn CT5_ff_preserved() {
    with_main(|doc| {
        append(doc, "A\u{000c}B");
        assert_eq!(CpText::getText(doc, 0, 3), "A\u{000c}B");
    });
}

#[test]
fn CT6_clamp_neg_start() {
    with_main(|doc| {
        append(doc, "X");
        assert_eq!(CpText::getText(doc, -3, 1), "X");
    });
}

#[test]
fn CT7_clamp_end_past_len() {
    with_main(|doc| {
        append(doc, "X");
        assert_eq!(CpText::getText(doc, 0, 99), "X");
    });
}

#[test]
fn CT8_start_gt_end() {
    with_main(|doc| {
        append(doc, "X");
        assert_eq!(CpText::getText(doc, 1, 0), "");
    });
}

#[test]
fn CT9_zero_width() {
    with_main(|doc| {
        append(doc, "X");
        assert_eq!(CpText::getText(doc, 0, 0), "");
    });
}
