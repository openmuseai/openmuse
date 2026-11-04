// function_id=02_function_hdrftr_story
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::HdrFtrDocumentImporter::HdrFtrDocumentImporter;

/// ★M-STAGE1 P2-6 —— 原先用 `HdrFtrDocumentImporter::default()`，其 `mPLCHeader` 是
/// 一个**与任何 `SubDocument` 无关的假对象** ⇒ `moveRight` 改的是假 PLC。
/// 现改为走真实构造路径（type-2 子文档的真实 `PLCHeader`），
/// 使 HS1/HS2 断言的是**真实 PLC 节点**的位置。
fn with_importer<F: FnOnce(&mut HdrFtrDocumentImporter)>(f: F) {
    let mut text_document =
        word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument::new();
    let sub_document = text_document
        .getSubdocumentMut(2)
        .expect("header SubDocument");
    let mut imp = HdrFtrDocumentImporter::HdrFtrDocumentImporter(
        sub_document,
        StyleId2Sti::StyleId2Sti(UUSti::UUSti()),
        NumIdRedirection::NumIdRedirection(UUNumberingId::new()),
        None,
        false,
        0,
        None,
        None,
    );
    f(&mut imp);
}

/// HS1: after writing story length, reviseNodeHeaderEven moves odd node to endCp.
#[test]
fn hs1_revise_even_header() {
    with_importer(|imp| {
        imp.addNodeHeaderEven();
        let before = imp.getHeaderOdd().unwrap().position();
        imp.mDocumentLen = 12;
        imp.reviseNodeHeaderEven();
        let after = imp.getHeaderOdd().unwrap().position();
        assert_eq!(
            after, 12,
            "HS1 odd node should move to document length; before={before}"
        );
    });
}

/// HS2: empty header part — length 0 revise is legal (no panic).
#[test]
fn hs2_empty_header() {
    with_importer(|imp| {
        imp.addNodeHeaderEven();
        imp.mDocumentLen = 0;
        imp.reviseNodeHeaderEven();
        assert_eq!(imp.getHeaderOdd().unwrap().position(), 0);
    });
}

/// HS3: revise without addNodeHeaderEven → assert / panic.
#[test]
fn hs3_revise_without_add() {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        with_importer(|imp| {
            imp.reviseNodeHeaderEven();
        });
    }));
    assert!(
        result.is_err(),
        "HS3 未 addNodeHeaderEven 就 revise 必须 panic（Java IOAssert.assertNotNull）"
    );
}
