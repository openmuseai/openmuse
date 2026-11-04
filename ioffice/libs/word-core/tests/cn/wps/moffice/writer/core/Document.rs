// function_id=01_function_document_types
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_document_types
// src:  libs/word-core/src/cn/wps/moffice/writer/core/Document.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/core/Document.rs

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use word_core::cn::wps::moffice::writer::core::Document::{
    COMMENT_DOCUMENT, Document, ENDNOTE_DOCUMENT, FOOTNOTE_DOCUMENT, HEADERTEXTBOX_DOCUMENT,
    HEADER_DOCUMENT, MAIN_DOCUMENT, SUBDOCUMENT_COUNT, TEXTBOX_DOCUMENT,
};

struct TypeDoc {
    ty: i32,
}

impl IDocument for TypeDoc {
    fn getType(&self) -> i32 {
        self.ty
    }
}

impl Document for TypeDoc {
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
        0
    }
}

#[test]
fn DT1_main_document_is_zero() {
    assert_eq!(MAIN_DOCUMENT, 0);
}

#[test]
fn DT2_footnote_document_is_one() {
    assert_eq!(FOOTNOTE_DOCUMENT, 1);
}

#[test]
fn DT3_header_document_is_two() {
    assert_eq!(HEADER_DOCUMENT, 2);
}

#[test]
fn DT4_comment_document_is_three() {
    assert_eq!(COMMENT_DOCUMENT, 3);
}

#[test]
fn DT5_endnote_document_is_four() {
    assert_eq!(ENDNOTE_DOCUMENT, 4);
}

#[test]
fn DT6_textbox_document_is_five() {
    assert_eq!(TEXTBOX_DOCUMENT, 5);
}

#[test]
fn DT7_headertextbox_document_is_six() {
    assert_eq!(HEADERTEXTBOX_DOCUMENT, 6);
}

#[test]
fn DT8_subdocument_count_is_seven() {
    assert_eq!(SUBDOCUMENT_COUNT, 7);
}

#[test]
fn DT9_type_ids_are_distinct_zero_through_six() {
    let ids = [
        MAIN_DOCUMENT,
        FOOTNOTE_DOCUMENT,
        HEADER_DOCUMENT,
        COMMENT_DOCUMENT,
        ENDNOTE_DOCUMENT,
        TEXTBOX_DOCUMENT,
        HEADERTEXTBOX_DOCUMENT,
    ];
    for (i, id) in ids.iter().enumerate() {
        assert_eq!(*id, i as i32);
    }
    assert_eq!(ids.len() as i32, SUBDOCUMENT_COUNT);
}

#[test]
fn DT10_get_type_returns_header_document() {
    let doc = TypeDoc {
        ty: HEADER_DOCUMENT,
    };
    let as_idoc: &dyn IDocument = &doc;
    assert_eq!(as_idoc.getType(), HEADER_DOCUMENT);
    assert_eq!(as_idoc.getType(), 2);
}
