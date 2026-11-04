// function_id=03_function_flow_loop
#![allow(non_snake_case)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
use word_core::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;

fn layouter() -> TextLayouter {
    let mut t = TextLayouter::default();
    t.mPageLayouter = Some(PageLayouter::PageLayouter(
        DocumentEnv::default(),
        ViewEnv::default(),
    ));
    t
}

#[test]
fn FL1_empty_src_zero_lines() {
    let mut lo = layouter();
    let mut st = LayoutState::default();
    lo.flow(&mut st);
    assert!(lo.isComplete(&st));
    assert!(!st.isFinishPage());
}

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::transaction::Transaction::Transaction;
use word_core::cn::wps::moffice::writer::core::Document::Document;

struct HelloDoc;
impl IDocument for HelloDoc {
    fn getType(&self) -> i32 { 0 }
}
impl Document for HelloDoc {
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

    fn getLength(&self) -> i32 { 5 }
    fn getTransaction(&self) -> Option<&Transaction> { None }
}

#[test]
fn FL2_hello_needs_line_layouter() {
    let mut lo = layouter();
    let mut st = LayoutState::default();
    st.srcList.obtainAndAddLast();
    st.pageEndLimitCP = i32::MAX;
    st.cp = 0;
    st.document = Some(Box::leak(Box::new(HelloDoc)) as *mut dyn Document);
    let panicked = catch_unwind(AssertUnwindSafe(|| lo.flow(&mut st))).is_err();
    assert!(panicked, "flowLine→doFlowLine→flowTextLine still DEFERRED (LineLayouter.flow)");
}

#[test]
fn FL3_cp_past_document_length_finishes_page() {
    // Java flowLine: if !(cp < pageEndLimitCP && cp < document.getLength()) → setFinishPage.
    // preFlow resets pageEndLimitCP to MAX, so the honest “page full” fixture is cp ≥ getLength.
    let mut lo = layouter();
    let mut st = LayoutState::default();
    st.srcList.obtainAndAddLast();
    st.cp = 0;
    st.document = Some(Box::leak(Box::new(EmptyDoc)) as *mut dyn Document);
    lo.flow(&mut st);
    assert!(st.isFinishPage());
}

struct EmptyDoc;
impl IDocument for EmptyDoc {
    fn getType(&self) -> i32 { 0 }
}
impl Document for EmptyDoc {
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

    fn getLength(&self) -> i32 { 0 }
    fn getTransaction(&self) -> Option<&Transaction> { None }
}
