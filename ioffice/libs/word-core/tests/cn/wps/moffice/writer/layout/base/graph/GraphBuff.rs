// function_id=02_function_graph_buff
#![allow(non_snake_case)]

use std::fs;

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphBuff::{EmbedArg, EqArg, GraphBuff};
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

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

fn mk_buff_mode(mode: i32) -> GraphBuff {
    let main = MainState::default();
    let doc = DocumentEnv::default();
    let mut view = ViewEnv::default();
    view.setViewMode(mode);
    GraphBuff::GraphBuff(main, doc, view)
}

fn mk_buff() -> GraphBuff {
    mk_buff_mode(0)
}

/// GB1: ctor 非 phone → MAX_FONT_SIZE
#[test]
fn GB1_ctor_page_mode() {
    let _b = mk_buff_mode(0);
    assert_eq!(FontStyleFactory::maxFontSize(), i32::MAX);
}

/// GB2: ctor phone → 520
#[test]
fn GB2_ctor_phone_mode() {
    let _b = mk_buff_mode(2);
    assert_eq!(FontStyleFactory::maxFontSize(), 520);
}

/// GB3: dispose 不 panic
#[test]
fn GB3_dispose() {
    let mut b = mk_buff();
    b.dispose();
    b.dispose();
}

/// GB4: recommend CP
#[test]
fn GB4_recommend_cp() {
    let mut b = mk_buff();
    b.setRecommendCPRef(9);
    assert_eq!(b.getRecommendCPRef(), 9);
}

/// GB5: addMap 首项
#[test]
fn GB5_add_map_first() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    assert_eq!(b.getCP(0), 10);
    assert_eq!(b.getCP(1), 11);
}

/// GB6: 相邻合并
#[test]
fn GB6_add_map_merge() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    b.addMap(12, 2, 2, 2);
    assert_eq!(b.getGraphIndex(10), 0);
    assert_eq!(b.getGraphIndex(13), 3);
    assert_eq!(b.getCP(3), 13);
}

/// GB7: 非衔接新项
#[test]
fn GB7_add_map_split() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    b.addMap(20, 1, 5, 1);
    assert_eq!(b.getCP(0), 10);
    assert_eq!(b.getCP(5), 20);
    assert_eq!(b.getGraphIndex(20), 5);
}

/// GB8: 扩容（5 个不合并项）
#[test]
fn GB8_add_map_grow() {
    let mut b = mk_buff();
    let mut i = 0i32;
    while i < 5 {
        b.addMap(i * 10, 1, i * 10, 2);
        i += 1;
    }
    assert_eq!(b.getCP(40), 40);
    assert_eq!(b.getGraphIndex(40), 40);
}

/// GB9: getGraphIndex 命中
#[test]
fn GB9_get_graph_index_hit() {
    let mut b = mk_buff();
    b.addMap(100, 4, 0, 4);
    assert_eq!(b.getGraphIndex(102), 2);
}

/// GB10: getGraphIndex miss
#[test]
fn GB10_get_graph_index_miss() {
    let mut b = mk_buff();
    b.addMap(100, 4, 0, 4);
    assert_eq!(b.getGraphIndex(99), -1);
    assert_eq!(b.getGraphIndex(104), -1);
}

/// GB11: getCP 空 map
#[test]
fn GB11_get_cp_empty() {
    let mut b = mk_buff();
    assert_eq!(b.getCP(0), -1);
}

/// GB12: getCP 命中
#[test]
fn GB12_get_cp_hit() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    assert_eq!(b.getCP(1), 11);
}

/// GB13: getEmbedSize null → 0
#[test]
fn GB13_embed_size_null() {
    let b = mk_buff();
    assert_eq!(b.getEmbedSize(), 0);
    assert_eq!(b.getArgsForEmbedSize(), 0);
}

/// GB14: prepareGraphs 无内容，map 保留
#[test]
fn GB14_prepare_empty() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    let mut st = LayoutState::default();
    assert!(!st.hasContents());
    assert!(!b.prepareGraphs(0, &mut st));
    assert_eq!(b.getCP(0), 10);
}

/// GB15: 有内容 + lists 空 → doClear 清 map
#[test]
fn GB15_prepare_contents() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    let mut st = LayoutState::default();
    st.srcList.obtainAndAddLast();
    assert!(st.hasContents());
    assert!(st.graphList.isEmpty());
    assert!(!b.prepareGraphs(0, &mut st));
    assert_eq!(b.getCP(0), -1);
}

/// GB16: lists nonempty + hasReadEnd → 早退，map 保留
#[test]
fn GB16_prepare_has_read_end() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    let mut st = LayoutState::default();
    st.srcList.obtainAndAddLast();
    st.graphList.obtainAndAddLast();
    st.charsList.obtainAndAddLast();
    st.hasReadEnd = true;
    assert!(!b.prepareGraphs(0, &mut st));
    assert_eq!(b.getCP(0), 10);
}

/// GB17: clear
#[test]
fn GB17_clear() {
    let mut b = mk_buff();
    b.addMap(10, 2, 0, 2);
    let mut st = LayoutState::default();
    st.srcList.obtainAndAddLast();
    b.clear(&mut st);
    assert!(st.srcList.isEmpty());
    assert_eq!(b.getCP(0), -1);
}

/// GB18: EqArg + addEqation
#[test]
fn GB18_eq_arg_embed() {
    let arg = EqArg::EqArg(1, 4, true, false);
    assert_eq!(arg.start, 1);
    assert_eq!(arg.end, 4);
    assert!(arg.enableRevision);
    assert!(!arg.revisionFinal);
    let mut b = mk_buff();
    let mut graph = Graphs::default();
    b.addEqation(&mut graph, 1, 4, true, false);
    assert_eq!(b.getEmbedSize(), 1);
    assert_eq!(b.getGraphsForEmbed(0), Some(&mut graph as *mut Graphs));
    match b.getArgsForEmbed(0) {
        Some(EmbedArg::Eq(e)) => {
            assert_eq!(e.start, 1);
            assert_eq!(e.end, 4);
        }
        other => panic!("expected EqArg, got {other:?}"),
    }
}

/// GB19: hello-world.docx → prepareGraphs 空 srcList
#[test]
fn GB19_hello_prepare_empty() {
    assert_eq!(sha2_hex(HELLO), HELLO_SHA256);
    let bytes = fs::read(HELLO).expect("hello-world.docx");
    let mut doc = TextDocument::new();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import");
    let main = doc.getMainDocument().expect("main");
    assert!(main.getLength() > 0);
    let mut b = mk_buff();
    let mut st = LayoutState::default();
    assert!(!b.prepareGraphs(0, &mut st));
}

/// GB20: TextLayouter 无本地 GraphBuff 影子
#[test]
fn GB20_textlayouter_uses_real_graphbuff() {
    let src = include_str!(
        "../../../../../../../../src/cn/wps/moffice/writer/layout/base/TextLayouter.rs"
    );
    assert!(
        !src.contains("pub struct GraphBuff;"),
        "TextLayouter still has GraphBuff shadow"
    );
    assert!(src.contains("graph::GraphBuff::GraphBuff"));
}
