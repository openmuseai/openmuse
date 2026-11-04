// function_id=02_function_kinsoku_rollback
#![allow(non_snake_case)]

use util::collections::KPList::KPList;
use word_core::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use word_core::cn::wps::moffice::writer::layout::base::line::LineRuleGraphsPos::LineRuleGraphsPos;
use word_core::cn::wps::moffice::writer::core::AsiaTypography::AsiaTypography;
use word_core::cn::wps::moffice::writer::layout::base::line::SubLineLayouterBase::SubLineLayouterBase;

const TYPE_NORMAL: i32 = 1;
const TYPE_TAB: i32 = 20;

fn tool() -> SubLineLayouterBase {
    SubLineLayouterBase::SubLineLayouterBase(
        MainState::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
    )
}

fn graphs(runType: i32, graphIndex: i32, graphLength: i32) -> Graphs {
    let mut g = Graphs::Graphs();
    g.runType = runType;
    g.graphIndex = graphIndex;
    g.graphLength = graphLength;
    g
}

fn pos_of(g: &Graphs, idx: i32) -> LineRuleGraphsPos {
    let mut p = LineRuleGraphsPos::default();
    p.init(g.clone(), idx);
    p
}

fn empty_list() -> KPList {
    KPList {
        alloc_fn: None,
        head: std::ptr::null_mut(),
        tail: std::ptr::null_mut(),
    }
}

fn typo(chars: Vec<char>) -> TypoPara {
    let n = chars.len() as i32;
    TypoPara {
        mGraphSize: n,
        chars,
        widths: vec![10; n as usize],
        embeds: Vec::new(),
        snapshot: std::ptr::null_mut(),
        cp_map: Vec::new(),
    }
}

/// KR1: kinsoku=false does not take the kinsoku head-punct branch.
#[test]
fn KR1_kinsoku_false_does_not_rollback_for_kinsoku() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 1);
    let mut last = pos_of(&g, 0);
    let mut next = pos_of(&g, 0);
    let tp = typo(vec!['A']);
    t.doAlternativeRollback(true, false, &AsiaTypography::default(), &mut last, &mut next, &tp, &empty_list());
    assert!(!t.mAdjusted);
    assert!(!t.mRollbackedAll);
    assert_eq!(last.getGraphIndex(), 0);
    assert_eq!(next.getGraphIndex(), 0);
}

/// KR2: next line starts with head punct `（`.
#[test]
fn KR2_kinsoku_head_punct() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 2);
    let mut last = pos_of(&g, 0);
    let mut next = pos_of(&g, 1);
    let tp = typo(vec!['A', '（']);
    t.rollbackIfNextLineStartsWithHeadPuncts(true, &mut last, &mut next, &tp, &empty_list());
    assert!(t.mNextLineStartsWithHeadPunct);
}

/// KR3: west letters spanning a run — partial-word rollback sets adjusted.
#[test]
fn KR3_partial_word() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 2);
    let mut last = pos_of(&g, 0);
    let mut next = pos_of(&g, 1);
    let tp = typo(vec!['A', 'B']);
    t.rollbackIfCurLineEndsWithPartOfWord(true, &mut last, &mut next, &tp, &empty_list());
    assert!(t.mAdjusted);
}

/// KR4: last and next both tabs → copy last onto next and try prev.
#[test]
fn KR4_tab_line_end() {
    let mut t = tool();
    let g = graphs(TYPE_TAB, 0, 1);
    let mut last = pos_of(&g, 0);
    let mut next = pos_of(&g, 0);
    t.rollbackIfCurLineEndsWithTab(true, true, &mut last, &mut next, &empty_list());
    assert!(t.mAdjusted);
    assert!(t.mRollbackedAll);
}

/// KR5: kinsoku=true + line-start punct after prev → alternative rollback takes kinsoku path.
#[test]
fn KR5_alternative_rollback_kinsoku() {
    let mut t = tool();
    let g = graphs(TYPE_NORMAL, 0, 3);
    let mut last = pos_of(&g, 1);
    let mut next = pos_of(&g, 2);
    let tp = typo(vec!['A', '（', 'B']);
    t.doAlternativeRollback(true, true, &AsiaTypography::default(), &mut last, &mut next, &tp, &empty_list());
    assert!(t.mNextLineStartsWithHeadPunct || t.mAdjusted || t.mRollbackedAll);
}
