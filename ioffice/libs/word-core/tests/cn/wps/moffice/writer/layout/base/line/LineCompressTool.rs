// function_id=02_function_compress
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use word_core::cn::wps::moffice::writer::layout::base::graph::SrcNode::{SrcNode, TYPE_SRC_AUTONUM};
use word_core::cn::wps::moffice::writer::layout::base::line::LineCompressTool::LineCompressTool;
use word_core::cn::wps::moffice::writer::layout::base::line::LineRuleGraphsPos::LineRuleGraphsPos;

const TYPE_NORMAL: i32 = 1;
const TYPE_SPACE: i32 = 2;
const TYPE_PUNCT: i32 = 18;

fn tool_with_doc(doc: DocumentEnv) -> LineCompressTool {
    let mut t = LineCompressTool::LineCompressTool(
        MainState::default(),
        doc,
        ViewEnv::default(),
    );
    t.mSubLineEnv
        .as_mut()
        .unwrap()
        .lineCompressEnv
        .as_mut()
        .unwrap()
        .reset();
    t
}

fn tool() -> LineCompressTool {
    tool_with_doc(DocumentEnv::default())
}

fn graphs(runType: i32, graphIndex: i32, graphLength: i32, auto_num: bool) -> Graphs {
    let mut g = Graphs::Graphs();
    g.runType = runType;
    g.graphIndex = graphIndex;
    g.graphLength = graphLength;
    g.mXPos = 0;
    let mut chars = CharsNode::default();
    if auto_num {
        // rust: wr-29 —— SrcNode 真实类型（含 super_ 等）；测试用 leak 保活
        let mut src = SrcNode::SrcNode();
        src.r#type = TYPE_SRC_AUTONUM;
        chars.srcRef = Some(Box::leak(Box::new(src)) as *mut SrcNode);
    }
    g.charsRef = Some(chars);
    g
}

fn graphs_punct_lid(ch_lid: i32, graphIndex: i32, graphLength: i32) -> Graphs {
    let mut g = graphs(TYPE_PUNCT, graphIndex, graphLength, false);
    let mut m = MutablePropertySet::new();
    m.setInt(42, ch_lid);
    if let Some(chars) = g.charsRef.as_mut() {
        chars.finalRunProp = Some(m.freeze());
    }
    g
}

fn layout_state(chars: Vec<char>, widths: Vec<i32>) -> LayoutState {
    let n = chars.len() as i32;
    let mut state = LayoutState::default();
    state.typoPara = TypoPara {
        mGraphSize: n,
        chars,
        widths,
        embeds: Vec::new(),
        snapshot: std::ptr::null_mut(),
        cp_map: Vec::new(),
    };
    state.paraEnv = ParaEnv::ParaEnv();
    state
}

fn pos_of(g: &Graphs, idx: i32) -> LineRuleGraphsPos {
    let mut p = LineRuleGraphsPos::default();
    p.init(g.clone(), idx);
    p
}

fn env_of(t: &LineCompressTool) -> &word_core::cn::wps::moffice::writer::layout::base::line::LineCompressEnv::LineCompressEnv {
    t.mSubLineEnv
        .as_ref()
        .unwrap()
        .lineCompressEnv
        .as_ref()
        .unwrap()
}

/// CP1: overflow + shouldBeFollowed (autoNum) → true, no hang/rollback.
#[test]
fn CP1_should_be_followed_returns_true() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 1, true);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['A'], vec![100]);
    assert!(t.compress(&mut pos, 50, &mut ls));
    assert_eq!(pos.getGraphIndex(), 0);
    assert_eq!(env_of(&t).firstExcessSpaceGraphIndex, i32::MIN);
    assert_eq!(env_of(&t).firstExcessPunctGraphIndex, i32::MIN);
}

/// CP2: space overflow + wordWrap + !wrapTrailSpaces → hang, firstExcessSpace set.
#[test]
fn CP2_hang_space() {
    let mut t = tool();
    let g = graphs(TYPE_SPACE, 0, 1, false);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec![' '], vec![100]);
    ls.paraEnv.wordWrap = true;
    assert!(t.compress(&mut pos, 50, &mut ls));
    assert_eq!(pos.getGraphIndex(), g.getEndGraphIndex());
    assert_eq!(env_of(&t).firstExcessSpaceGraphIndex, 0);
}

/// CP3: wrapTrailSpaces → cannot hang space; compressLine off → rollback/invalid.
#[test]
fn CP3_no_hang_space_wrap_trail() {
    let mut doc = DocumentEnv::default();
    doc.wrapTrailSpaces = true;
    let mut t = tool_with_doc(doc);
    let g = graphs(TYPE_SPACE, 0, 1, false);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec![' '], vec![100]);
    ls.paraEnv.wordWrap = true;
    ls.paraEnv.compressLine = false;
    let ok = t.compress(&mut pos, 50, &mut ls);
    assert!(!ok);
    assert_eq!(env_of(&t).firstExcessSpaceGraphIndex, i32::MIN);
}

/// CP4: runType 18 + hangable ideographic period (lid 2052) → firstExcessPunct.
#[test]
fn CP4_hang_punct() {
    let mut t = tool();
    let g = graphs_punct_lid(2052, 0, 1);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['。'], vec![100]);
    ls.paraEnv.overflowPunct = true;
    ls.paraEnv.compressLine = false;
    assert!(t.compress(&mut pos, 50, &mut ls));
    assert_eq!(pos.getGraphIndex(), g.getEndGraphIndex());
    assert_eq!(env_of(&t).firstExcessPunctGraphIndex, 0);
}

/// CP5: overflowPunct=false → do not hang punct.
#[test]
fn CP5_no_overflow_punct() {
    let mut t = tool();
    let g = graphs_punct_lid(2052, 0, 1);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['。'], vec![100]);
    ls.paraEnv.overflowPunct = false;
    ls.paraEnv.compressLine = false;
    let ok = t.compress(&mut pos, 50, &mut ls);
    assert!(!ok);
    assert_eq!(env_of(&t).firstExcessPunctGraphIndex, i32::MIN);
}

/// CP6: compressLine=false, non-hangable letter overflow → not fit.
#[test]
fn CP6_normal_compress_off() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 2, false);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['A', 'B'], vec![40, 80]);
    ls.paraEnv.compressLine = false;
    let ok = t.compress(&mut pos, 50, &mut ls);
    assert!(!ok);
}

/// CP7: compressLine on + enough compressable space → consume and fit.
#[test]
fn CP7_normal_compress_on_fit() {
    let mut t = tool();
    t.mSubLineEnv
        .as_mut()
        .unwrap()
        .lineCompressEnv
        .as_mut()
        .unwrap()
        .addOtherCompressableSpace(1, 80);
    let g = graphs(TYPE_NORMAL, 0, 2, false);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['A', 'B'], vec![40, 80]);
    ls.paraEnv.compressLine = true;
    ls.paraEnv.wordWrap = true;
    assert!(t.compress(&mut pos, 50, &mut ls));
    assert_eq!(pos.getGraphIndex(), g.getEndGraphIndex());
    assert!(env_of(&t).getUsedCompressableSpaceSum() > 0);
}

/// CP8: cannot hang/compress; prev within-run rolls graphIdx back from excess.
#[test]
fn CP8_rollback_prev() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 2, false);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['A', 'B'], vec![40, 80]);
    ls.paraEnv.compressLine = false;
    let ok = t.compress(&mut pos, 50, &mut ls);
    assert!(!ok);
    assert!(!pos.isInvalid());
    assert_eq!(pos.getGraphIndex(), 0);
}

/// CP9: prev fails and !atLeastOneChar → isInvalid, compress false.
#[test]
fn CP9_invalid_after_failed_prev() {
    let mut t = tool();
    t.mSubLineEnv.as_mut().unwrap().atLeastOneChar = false;
    let g = graphs(TYPE_NORMAL, 0, 1, false);
    let mut pos = pos_of(&g, 0);
    let mut ls = layout_state(vec!['A'], vec![100]);
    ls.paraEnv.compressLine = false;
    let ok = t.compress(&mut pos, 50, &mut ls);
    assert!(!ok);
    assert!(pos.isInvalid());
}
