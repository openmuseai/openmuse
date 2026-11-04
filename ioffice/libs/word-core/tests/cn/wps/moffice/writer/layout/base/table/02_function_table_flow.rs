// function_id=02_function_table_flow
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::table::TableLayouter::{
    Document, ErrorCollector, IPos, IStateEnvoy, LayoutState, ParentLayouterTask, TableInfo,
    TableInfos, TableLayouter, TypoDrawing, TypoPage,
};

struct Envoy {
    doc: Document,
    page: TypoPage,
}

impl IStateEnvoy for Envoy {
    fn getDocument(&self) -> &Document {
        &self.doc
    }
    fn getCurPage(&self) -> &TypoPage {
        &self.page
    }
}

fn tool() -> TableLayouter {
    TableLayouter::TableLayouter(
        ParentLayouterTask::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
        MainState::default(),
    )
}

fn layout_state(doc: Document) -> LayoutState {
    LayoutState {
        envoy: Some(Box::new(Envoy {
            doc,
            page: TypoPage::default(),
        })),
    }
}

fn table_doc(pos: Option<IPos>, type_: i32, has_error: bool) -> Document {
    Document {
        type_,
        errorCollector: ErrorCollector { has_error },
        tableInfos: TableInfos {
            table: Some(TableInfo {
                pos,
                size: 1,
                row: Some(Default::default()),
                ..Default::default()
            }),
        },
    }
}

/// TF1: 1×1 in-flow table (no wrap pos) → flowNormalTableRow.
#[test]
fn tf1_normal_table_row() {
    let mut t = tool();
    let mut st = layout_state(table_doc(None, 1, false));
    t.flow(false, &mut st);
    assert!(t.mTableState.flowedNormal >= 1);
    assert_eq!(t.mTableState.flowedWrap, 0);
}

/// TF2: mHasBreakRow is stored on table state (continuation).
#[test]
fn tf2_break_row() {
    let mut t = tool();
    let mut st = layout_state(table_doc(None, 1, false));
    t.flow(true, &mut st);
    assert!(t.mTableState.hasBreakCellOfLayoutPage);
}

/// TF3: wrap pos present → flowWrapTable.
#[test]
fn tf3_wrap_table() {
    let mut t = tool();
    let mut st = layout_state(table_doc(Some(IPos::default()), 1, false));
    t.flow(false, &mut st);
    assert!(t.mTableState.flowedWrap >= 1);
    assert_eq!(t.mTableState.flowedNormal, 0);
}

/// TF4: missing table + error collector → return without panic.
#[test]
fn tf4_empty_table_with_error() {
    let mut t = tool();
    let mut st = layout_state(Document {
        type_: 1,
        errorCollector: ErrorCollector { has_error: true },
        tableInfos: TableInfos { table: None },
    });
    t.flow(false, &mut st);
    assert_eq!(t.mTableState.flowedNormal, 0);
    assert_eq!(t.mTableState.flowedWrap, 0);
}

/// flowRows records a wrap-row pass (TF1 companion I/O).
#[test]
fn tf_flow_rows() {
    let mut t = tool();
    t.mTable = Some(TableInfo {
        size: 1,
        row: Some(Default::default()),
        ..Default::default()
    });
    let mut env = Envoy {
        doc: Document::default(),
        page: TypoPage::default(),
    };
    t.flowRows(&TypoDrawing::default(), &mut env);
    assert!(t.mTableState.flowedRows >= 1);
}
