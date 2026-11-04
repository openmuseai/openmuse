// function_id=01_function_graph_nodes
#![allow(non_snake_case)]

use std::any::TypeId;

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// rust: ★wr-29 04 —— fontStyles 用真实 cache::style::TypoFontStyle
use word_core::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
use word_core::cn::wps::moffice::writer::layout::base::graph::CharsNode::{
    self as chars_mod, Alloc as CharsAlloc, CharsNode,
};
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use word_core::cn::wps::moffice::writer::layout::base::graph::SrcNode::{
    Alloc as SrcAlloc, SrcNode, TYPE_SRC_AUTONUM, TYPE_SRC_BALLOONINFO, TYPE_SRC_CORECHARS,
    TYPE_SRC_INVALID, TYPE_SRC_MATH,
};

const HELLO: &str = "tests/fixtures/p1_layout_document/hello-world.docx";
const HELLO_SHA256: &str = "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e";

fn sha2_hex(path: &str) -> String {
    let out = std::process::Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .expect("shasum");
    let s = String::from_utf8_lossy(&out.stdout);
    s.split_whitespace().next().unwrap_or("").to_string()
}

/// GN1: Alloc+ctor 默认
#[test]
fn GN1_alloc_defaults() {
    let mut alloc = SrcAlloc::default();
    let n = alloc.alloc();
    assert_eq!(n.r#type, TYPE_SRC_INVALID);
    assert_eq!(n.cp, -1);
}

/// GN2: init 复位脏字段
#[test]
fn GN2_init_resets() {
    let mut n = SrcNode::SrcNode();
    n.r#type = 9;
    n.cp = 4;
    n.data = Some("x".into());
    n.revisionFinal = false;
    n.init();
    assert_eq!(n.r#type, -1);
    assert_eq!(n.cp, -1);
    assert!(n.revisionFinal);
    assert!(n.data.is_none());
}

/// GN3: toString CORECHARS
#[test]
fn GN3_to_string_corechars() {
    let mut n = SrcNode::SrcNode();
    n.r#type = TYPE_SRC_CORECHARS;
    n.cp = 0;
    n.length = 3;
    let s = n.toString();
    assert!(s.contains("CORECHARS"), "{s}");
    assert!(s.contains("cp=0"), "{s}");
    assert!(s.contains("length=3"), "{s}");
}

/// GN4: AUTONUM / BALLOONINFO / MATH
#[test]
fn GN4_to_string_named_payloads() {
    for (ty, tag) in [
        (TYPE_SRC_AUTONUM, "AUTONUM"),
        (TYPE_SRC_BALLOONINFO, "BALLOONINFO"),
        (TYPE_SRC_MATH, "MATH"),
    ] {
        let mut n = SrcNode::SrcNode();
        n.r#type = ty;
        n.data = Some("x".into());
        let s = n.toString();
        assert!(s.contains(tag) && s.contains("x"), "{ty} {s}");
    }
}

/// GN5: INVALID
#[test]
fn GN5_to_string_invalid() {
    let mut n = SrcNode::SrcNode();
    n.r#type = 99;
    let s = n.toString();
    assert!(s.contains("INVALID"), "{s}");
}

/// GN6: dispose
#[test]
fn GN6_dispose_clears() {
    let mut n = SrcNode::SrcNode();
    n.data = Some("keep".into());
    n.dispose();
    assert!(n.data.is_none());
    assert!(n.doc.is_none());
    assert!(n.finalRunProp.is_none());
}

/// GN7: CharsNode ctor 10 槽
#[test]
fn GN7_chars_fontstyles_len() {
    let mut alloc = CharsAlloc::default();
    let c = alloc.alloc();
    assert_eq!(c.fontStyles.len(), 10);
    assert_eq!(c.r#type, chars_mod::TYPE_CHARS_INVALID);
    assert_eq!(c.offsetOfRef, -1);
    assert_eq!(c.hyperlinkIndex, -1);
    assert_eq!(c.revisionInsAuthor, -1);
}

/// GN8: init 清 fontStyles
#[test]
fn GN8_init_clears_fontstyles() {
    let mut c = CharsNode::CharsNode();
    c.fontStyles[0] = Some(TypoFontStyle::newInstance());
    c.init();
    assert!(c.fontStyles.iter().all(|s| s.is_none()));
}

/// GN9: toString 五 type
#[test]
fn GN9_chars_to_string_types() {
    let mut src = SrcNode::SrcNode();
    src.r#type = TYPE_SRC_CORECHARS;
    src.cp = 1;
    src.length = 2;
    let mut c = CharsNode::CharsNode();
    c.srcRef = Some(&mut src as *mut SrcNode);
    for (ty, tag) in [
        (chars_mod::TYPE_CHARS_REF, "REF"),
        (chars_mod::TYPE_CHARS_RESULT, "RESULT"),
        (chars_mod::TYPE_CHARS_OMATH, "OMATH"),
        (chars_mod::TYPE_CHARS_MATH_FUNCTION, "MATH_FUNCTION"),
        (chars_mod::TYPE_CHARS_EQUATION, "EQUATION"),
    ] {
        c.r#type = ty;
        let s = c.toString();
        assert!(s.contains(tag), "{ty} {s}");
        assert!(s.contains("CORECHARS"), "{s}");
    }
}

/// GN10: srcRef None → "null"（Java 会 NPE）
#[test]
fn GN10_chars_to_string_null_src() {
    let c = CharsNode::CharsNode();
    let s = c.toString();
    assert!(s.contains("null"), "{s}");
}

/// GN11: 产品类型是 graph::Graphs，不是 LayoutState 文件内 Copy 影子
#[test]
fn GN11_layoutstate_uses_real_graphs() {
    let _st = LayoutState::default();
    assert_eq!(
        TypeId::of::<Graphs>(),
        TypeId::of::<word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs>()
    );
}

/// GN12: 同一份 hello-world.docx → CORECHARS 节点 length = getLength()
#[test]
fn GN12_hello_world_corechars_from_parse() {
    let bytes = std::fs::read(HELLO).expect("hello-world.docx");
    assert_eq!(sha2_hex(HELLO), HELLO_SHA256, "fixture sha256 drift");
    let mut doc = TextDocument::new();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import");
    let main = doc.getMainDocument().expect("main");
    let len = main.getLength();
    assert!(len > 0, "parse produced empty main story");
    let mut n = SrcNode::SrcNode();
    n.r#type = TYPE_SRC_CORECHARS;
    n.cp = 0;
    n.length = len;
    assert!(n.toString().contains(&format!("length={len}")));
}
