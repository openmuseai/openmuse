// function_id=64_function_textdocument_document_trait
// cargo test --manifest-path libs/word-core/Cargo.toml --test 64_function_textdocument_document_trait
//
// ★M-STAGE2 S2-2 —— 「TextDocument 真实装配为 Document」的验证。
//   Java: `public class TextDocument extends Atom implements IDocument, ITextOpenDocument, DocumentProtection`
//   Rust 此前只把 `SubDocument` 实现为 `Document`，`TextDocument` **两个 trait 都没实现**
//   ⇒ 布局/渲染管线无法直读顶层文档对象，只能靠 `getMainDocument()` 逃逸。
//
// 本用例锁定接线后的不变量：
//   T2_1  TextDocument 可作为 &dyn Document（装配闭环前提）
//   T2_2  Document::getLength() 委托到主文档（管线 41 处调用）
//   T2_3  Document::getMainDocument() 返回主文档
//   T2_4  getTextRope/getChpxTable/getPapxTable 委托（RunStream::collectRuns 读的正是这三个）
//   T2_5  IDocument::getType() == 0（主文档槽位，Java 用作 mSubdocuments[] 下标）
//   T2_6  经 &dyn Document 读到真实 rope，长度与 getLength() 一致（真实读路径）
#![allow(non_snake_case)]

use std::path::PathBuf;

use word_core::cn::wps::moffice::writer::api::DocumentOpen::open_boxed_text_document_from_docx;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// C1：真实 OOXML 标准包（3,502 B）→ `"Hello, wave0!\r"`
fn c1_bytes() -> Vec<u8> {
    let p = crate_root()
        .join("../core-libs/io/wps-io/tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    std::fs::read(p).expect("read standard-hello.docx")
}

fn fixture() -> Box<TextDocument> {
    let bytes = c1_bytes();
    open_boxed_text_document_from_docx(&bytes, &String::new())
        .expect("open_boxed_text_document_from_docx")
}

#[test]
fn T2_1_textdocument_is_dyn_document() {
    let doc = fixture();
    // ★ 装配闭环前提：TextDocument 必须是 Document（Java: implements IDocument）
    let dyn_doc: &dyn Document = doc.as_ref();
    assert!(dyn_doc.getLength() > 0, "T2-1 dyn Document getLength > 0");
}

#[test]
fn T2_2_get_length_delegates_to_main() {
    let doc = fixture();
    let via_trait = Document::getLength(doc.as_ref());
    let via_inherent = TextDocument::getLength(doc.as_ref());
    let main_len = doc.getMainDocument().expect("main").getLength();
    assert_eq!(via_trait, main_len, "T2-2 trait getLength == main.getLength()");
    assert_eq!(via_inherent, main_len, "T2-2 inherent getLength == main.getLength()");
    assert!(main_len > 0, "T2-2 main length > 0");
}

#[test]
fn T2_3_get_main_document_returns_main() {
    let doc = fixture();
    let via_trait = Document::getMainDocument(doc.as_ref());
    assert!(via_trait.is_some(), "T2-3 dyn getMainDocument is Some");
    let t = via_trait.expect("main");
    assert_eq!(t.getType(), 0, "T2-3 main document type == 0");
}

#[test]
fn T2_4_layout_read_interface_delegates() {
    let doc = fixture();
    let d: &dyn Document = doc.as_ref();
    // ★ 布局管线 RunStream::collectRuns 读的正是这三个；委托断裂 ⇒ collectRuns 返回空 ⇒ 布局空帧
    assert!(d.getTextRope().is_some(), "T2-4 getTextRope delegated (not None)");
    assert!(d.getChpxTable().is_some(), "T2-4 getChpxTable delegated (not None)");
    assert!(d.getPapxTable().is_some(), "T2-4 getPapxTable delegated (not None)");
}

#[test]
fn T2_5_get_type_is_main_slot() {
    let doc = fixture();
    let t = IDocument::getType(doc.as_ref());
    assert_eq!(t, 0, "T2-5 TextDocument getType == 0 (mSubdocuments[0])");
}

#[test]
fn T2_6_rope_length_matches_document_length() {
    let doc = fixture();
    let d: &dyn Document = doc.as_ref();
    let rope_ptr = d.getTextRope().expect("rope");
    let rope_len = unsafe { ITextRope::length(&*rope_ptr) };
    assert!(rope_len > 0, "T2-6 rope length > 0");
    assert_eq!(
        rope_len,
        d.getLength(),
        "T2-6 rope length == Document::getLength() (真实读路径一致)"
    );
}
