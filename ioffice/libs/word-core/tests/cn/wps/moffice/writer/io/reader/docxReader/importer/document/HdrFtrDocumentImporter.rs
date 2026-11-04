// function_id=01_function_hdrftr_rid
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// rust: `SubDocument::SubDocument` 的形参类型是 `core::KStyle::TextDocument`（再导出）
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::HdrFtrDocumentImporter::HdrFtrDocumentImporter;

/// ★M-STAGE1 P2-6 —— 本组用例原先用 `HdrFtrDocumentImporter::default()` 构造。
/// 那个 `Default` 里 `mPLCHeader = Some(PLCHeader::default())` 是**假对象**：
/// 它不与任何 `SubDocument` 关联，`addNode*` 写进去的节点无处可去。
///
/// 现在改为走**真实构造路径**（Java:
/// `new HdrFtrDocumentImporter(document.createSubDocument(2), ...)` +
/// `this.mPLCHeader = this.mDocument.getPLCHeader();`）⇒
/// `mPLCHeader` 是 type-2 子文档的**真实** `PLCHeader`（由 `SubDocument.createPLC()`
/// 的 `case 2` 建立），`addNode*` 真的写进该子文档的 PLC。
///
/// `HdrFtrDocumentImporter` 内部按 Java 引用语义持有 `*mut SubDocument` / `*mut PLCHeader`
/// ⇒ 子文档必须在 importer 存活期间保持存活，故用闭包形式把两者绑定在同一栈帧内。
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

/// HF0：真实构造必须拿到**非空** `mPLCHeader`（Java `IOAssert.assertNotNull` 的前提）。
#[test]
fn hf0_real_ctor_obtains_plc_header_from_subdocument() {
    with_importer(|imp| {
        assert!(
            imp.mPLCHeader.is_some(),
            "HF0 真构造器必须从 type-2 子文档取得 mPLCHeader（Java: this.mPLCHeader = this.mDocument.getPLCHeader()）"
        );
    });
}

/// HF1: addNodeHeaderEven allocates even+odd PLC nodes.
#[test]
fn hf1_add_node_header_even() {
    with_importer(|imp| {
        imp.addNodeHeaderEven();
        assert!(imp.getHeaderEven().is_some(), "HF1 mHeaderEven");
        assert!(imp.getHeaderOdd().is_some(), "HF1 mHeaderOdd");
    });
}

/// HF2: second addNodeHeaderEven must fail (invoked only once).
#[test]
fn hf2_second_add_panics() {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        with_importer(|imp| {
            imp.addNodeHeaderEven();
            imp.addNodeHeaderEven();
        });
    }));
    assert!(result.is_err(), "HF2 第二次 addNodeHeaderEven 必须 panic");
}

/// HF3: disposeHeaderFooterData clears nodes; add again succeeds.
#[test]
fn hf3_dispose_then_add() {
    with_importer(|imp| {
        imp.addNodeHeaderEven();
        imp.disposeHeaderFooterData();
        assert!(imp.getHeaderEven().is_none());
        assert!(imp.getHeaderOdd().is_none());
        imp.addNodeHeaderEven();
        assert!(imp.getHeaderEven().is_some());
    });
}

/// HF4: even → headerOdd → footerEven chain (add order, CPs from previous node).
#[test]
fn hf4_footer_odd_chain() {
    with_importer(|imp| {
        imp.addNodeHeaderEven();
        let even_cp = imp.getHeaderEven().unwrap().getCP();
        let odd_cp = imp.getHeaderOdd().unwrap().getCP();
        assert_eq!(odd_cp, even_cp, "odd starts at even CP before revise");
        imp.addNodeHeaderOdd();
        let footer_even = imp.getFooterEven().expect("HF4 mFooterEven");
        assert_eq!(footer_even.getCP(), odd_cp);
        imp.addNodeFooterEven();
        let footer_odd = imp.getFooterOdd().expect("HF4 mFooterOdd");
        assert_eq!(footer_odd.getCP(), footer_even.getCP());
    });
}
