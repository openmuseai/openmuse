// function_id=04_function_graph_factory
// cargo test --test 04_function_graph_factory
#![allow(non_snake_case)]

use std::sync::Arc;

use word_core::cn::wps::moffice::writer::api::LayoutDocument::DEFAULT_MVP_FONT;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphFactory::GraphFactory;
use word_core::cn::wps::moffice::writer::layout::base::graph::SrcNode::{SrcNode, TYPE_SRC_CORECHARS};
use word_core::cn::wps::moffice::writer::layout::text::FontHost;
use word_core::cn::wps::moffice::writer::layout::text::FontStack::FontStack;

/// 注入进程字体表（Java FontHost.getFontTable 对等；MvpLatin.ttf = Space + A-Za-z）
fn inject_fonts() {
    let stack = FontStack::from_font_bytes(DEFAULT_MVP_FONT.to_vec()).expect("mvp latin");
    FontHost::set_font_table(Arc::new(stack));
}

fn make_state() -> LayoutState {
    let mut st = LayoutState::default();
    st.paraEnv.curParaPropEnv = Some(ParaPropEnv::default());
    if let Some(env) = st.paraEnv.curParaPropEnv.as_mut() {
        env.end = 12;
        env.paraProperty = Some(util::moffice::property::PropertySet::PropertySet::default());
    }
    st
}

/// GF1: ctor + dispose 不 panic；opEnv 装配（runType=1、mathUnderlineColor、指针非空）
#[test]
fn GF1_ctor_dispose_wires_op_env() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    assert_eq!(gf.test_op_runType(), 1);
    assert_eq!(gf.test_op_mathUnderlineColor(), -16777216);
    assert_eq!(gf.test_op_highSurrogateType(), 30);
    // rust: documentEnv/typoFontStylePool 指针在 createNodes 入口刷新（GF2 度量路径实测非空）
    gf.dispose();
}

fn core_chars_node() -> CharsNode {
    let mut src = SrcNode::SrcNode();
    src.r#type = TYPE_SRC_CORECHARS;
    let mut cn = CharsNode::CharsNode();
    cn.finalRunProp = Some(util::moffice::property::PropertySet::PropertySet::default());
    cn.srcRef = Some(Box::leak(Box::new(src)) as *mut SrcNode);
    cn.isInMath = false;
    cn
}

/// GF2: createNodes 对 "Hello World!" 产出逐字符 Graphs（runType=1/2/18，mWidth>0）
///     宽度 = round(hmtx * fontSize / upe)，fontSize=200twip（10pt 默认）
#[test]
fn GF2_hello_chars_become_graphs_with_width() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    // 预填 chars 平面（readRun/commitReadText 在真实管线里做）
    let text: Vec<char> = "Hello World!".chars().collect();
    st.typoPara.allocGraphs(12);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 12, 0);

    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 12, &mut st);

    // 12 chars → 12 graphs（"Hello"/"World" 单词、空格、'!' 独立成图；无合并断言性别）
    let n = st.graphList.size();
    // Java WestLayout 合并语义："Hello" + " " + "World" + "!" = 4 图
    assert_eq!(n, 4, "WestLayout 字母合并后应为 4 图");

    // 枚举经 KPList.getNext（Graphs 的 KPNode 链在 compile-surface 不接线）
    let mut score: Vec<(i32, i32, i32)> = Vec::new();
    let mut ptr = st.graphList.firstElement();
    while !ptr.is_null() {
        let g = unsafe { &*ptr };
        score.push((g.runType, g.graphLength, g.mWidth));
        ptr = st.graphList.getNext(ptr);
    }
    let _ = text;
    assert_eq!(score.len(), n as usize);
    assert_eq!(score[0].0, 1);
    assert_eq!(score[0].1, 5, "Hello 合并 len=5");
    assert_eq!(score[1].0, 2, "space runType=2");
    assert_eq!(score[2].0, 1);
    assert_eq!(score[2].1, 5, "World 合并 len=5");
    assert_eq!(score[3].0, 18, "标点 '!' runType=18");
    assert!(score[0].2 > 0, "Hello 应有正宽");
    assert!(score[2].2 > 0, "World 应有正宽");
    // '!'：MvpLatin 只含 Space+A-Za-z（无 '!' 字形）→ hmtx 缺失 0 宽（与 wr-11 度量一致）
    assert_eq!(score[3].2, 0, "'!' 缺字形 → 0 宽（字体覆盖语义）");

    // 字母宽 = sum(round(hmtx*200/upe)) per char（与 glyph_advance_twip on face 同式）
    let stack = FontHost::get_font_table().expect("table");
    let upe = stack.units_per_em(None).unwrap();
    let mut expect_hello = 0i32;
    for ch in "Hello".chars() {
        let h = stack.glyph_hmtx_units(None, ch).unwrap() as f64;
        expect_hello += (h * 200.0 / upe as f64).round() as i32;
    }
    assert_eq!(score[0].2, expect_hello, "Hello 合并宽 = 逐字 hmtx×200/upe 和");
}

/// GF3: 前缀空格合并 —— "A  B"（两个空格）→ 3 图，空格图 graphLength=2
#[test]
fn GF3_runs_of_spaces_merge() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    let text: Vec<char> = "A  B".chars().collect();
    st.typoPara.allocGraphs(4);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 4, 0);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 4, &mut st);
    assert_eq!(st.graphList.size(), 3, "A + 双空格图 + B");
    let mut ptr = st.graphList.firstElement();
    let mut idx = 0;
    while !ptr.is_null() {
        let g = unsafe { &*ptr };
        if idx == 1 {
            assert_eq!(g.runType, 2);
            assert_eq!(g.graphLength, 2);
        }
        idx += 1;
        ptr = st.graphList.getNext(ptr);
    }
    assert_eq!(idx, 3);
}

/// GF4: setCalcGraphHeight/setMathUnderline/setFootEndNoteRefType/scaleChar 写 opEnv
#[test]
fn GF4_flags_and_math_underline() {
    inject_fonts();
    let mut gf = GraphFactory::GraphFactory(MainState::default(), DocumentEnv::default(), ViewEnv::default());
    gf.setCalcGraphHeight(true);
    assert!(gf.test_op_isCalcGraphHeight());
    gf.setMathUnderline(2, 0xFF0000);
    assert_eq!(gf.test_op_mathUnderline(), 2);
    assert_eq!(gf.test_op_mathUnderlineColor(), 0xFF0000);
    gf.setFootEndNoteRefType(35);
    assert_eq!(gf.test_footEndNoteRefType(), 35);
    gf.scaleChar();
    assert!(gf.test_op_isScralForSmallCaps());
    gf.setMathContent(true);
    assert!(gf.test_op_isMathContent());
    gf.setIgnoreLeading(true);
    assert!(gf.test_op_isIgnoreLeading());
}

/// GF5: createNodes 后 opEnv 无残留节点（commitAll 均提交）；graphList 与 typoPara.widths 一致
#[test]
fn GF5_commit_all_clears_pending_nodes() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    let text: Vec<char> = "Hi".chars().collect();
    st.typoPara.allocGraphs(2);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 2, 0);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 2, &mut st);
    assert!(gf.test_langGraphNode_null(), "langGraphNode 应提交清空");
    assert!(gf.test_commonGraphNode_null(), "commonGraphNode 应提交清空");
    // typoPara.widths 平面已被填充（每宽>0）
    let w0 = st.typoPara.getWidth(0);
    let w1 = st.typoPara.getWidth(1);
    assert!(w0 > 0 && w1 > 0);
}

/// Java CharMap → FarEastLayout: contiguous Han chars form one graph, and
/// the East Asian default advance is one scaled font size per char.
#[test]
fn GF6_han_uses_far_east_layout() {
    use word_core::cn::wps::moffice::writer::core::CharParse::CharParse;

    let mut parse = CharParse::CharParse();
    assert_eq!(parse.getCharType('大'), Some(2));
    assert_eq!(parse.getCharType('A'), Some(1));

    inject_fonts();
    let mut doc_env = DocumentEnv::default();
    doc_env.propToolFactory = Some(PropertyToolFactory::default());
    doc_env.fontStyleCacheFactory = Some(Default::default());
    let mut factory = GraphFactory::GraphFactory(MainState::default(), doc_env, ViewEnv::default());
    let mut state = make_state();
    let text: Vec<char> = "大学生实践文".chars().collect();
    state.typoPara.allocGraphs(text.len() as i32);
    state.typoPara.putChars__chararr_int_int_int(&text, 0, text.len() as i32, 0);
    let node = core_chars_node();
    factory.createNodes(&node, 0, text.len() as i32, &mut state);
    assert_eq!(state.graphList.size(), 1, "Java FarEastLayout 合并连续汉字");
    let graph = unsafe { &*state.graphList.firstElement() };
    assert_eq!(graph.graphLength, text.len() as i32);
    assert_eq!(graph.autoSpaceType, 2);
    assert_eq!(graph.mWidth, 200 * text.len() as i32);
}
