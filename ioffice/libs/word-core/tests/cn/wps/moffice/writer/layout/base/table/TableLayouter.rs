// function_id=01_function_table_detect
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::table::TableLayouter::{
    Document, IPos, IStateEnvoy, ParentLayouterTask, TableInfo, TableLayouter, TypoPage,
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

fn layouter(pos: Option<IPos>) -> TableLayouter {
    let mut t = TableLayouter::TableLayouter(
        ParentLayouterTask::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
        MainState::default(),
    );
    t.mTable = Some(TableInfo { pos, ..Default::default() });
    t
}

fn envoy(doc_type: i32, visualTextflow: i32) -> Envoy {
    Envoy {
        doc: Document { type_: doc_type, ..Default::default() },
        page: TypoPage { visualTextflow, ..Default::default() },
    }
}

/// TD1: mTable.getPos() != null → true (wrap-table position present).
#[test]
fn td1_has_table_pos() {
    let t = layouter(Some(IPos::default()));
    assert!(t.hasTablePos(&envoy(1, 0)));
}

/// TD2: no pos, document type neither 0 nor 2 → false.
#[test]
fn td2_no_pos_non_textflow_doc() {
    let t = layouter(None);
    assert!(!t.hasTablePos(&envoy(1, 1)));
}

/// TD3: no pos, type 0 (main doc) and vert textflow → true.
#[test]
fn td3_main_doc_vert_textflow() {
    let t = layouter(None);
    assert!(t.hasTablePos(&envoy(0, 1)));
}
