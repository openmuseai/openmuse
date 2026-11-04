// function_id=02_function_flow_to_page
#![allow(non_snake_case)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
use word_core::cn::wps::moffice::writer::layout::pageview::PageViewLayouter::PageViewLayouter;

fn complete_empty() -> PageViewLayouter {
    let mut pvl = PageViewLayouter::default();
    pvl.mFinishLoading = true;
    pvl.mLayoutState = Some(Box::new(LayoutState::default()));
    pvl.mPageLayouter = Some(PageLayouter::PageLayouter(
        DocumentEnv::default(),
        ViewEnv::default(),
    ));
    pvl
}

#[test]
fn TP1_hello_empty_complete_is_zero_pages() {
    // Java flowToPage: pageIndex < 1 → -1; size 0 → min(pageIndex, -1) → -1.
    // Hello 1-page production needs LineLayouter.flow (FUNCTION.md §1.7 deferred).
    assert_eq!(PageViewLayouter::default().flowToPage(0), -1);
    assert_eq!(PageViewLayouter::default().flowToPage(-3), -1);
    let mut pvl = complete_empty();
    let empty = LayoutState::default();
    assert!(pvl.isComplete(&empty));
    assert_eq!(pvl.flowToPage(1), -1);
}

#[test]
fn TP2_form_feed_hits_deferred_line_flow() {
    // Java \f → 2 pages requires flowPage → TextLayouter.flow → flowTextLine.
    use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
    use util::moffice::transaction::Transaction::Transaction;
    use word_core::cn::wps::moffice::writer::core::Document::Document;

    struct HelloDoc;
    impl IDocument for HelloDoc {
        fn getType(&self) -> i32 {
            0
        }
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

        fn getLength(&self) -> i32 {
            5
        }
        fn getTransaction(&self) -> Option<&Transaction> {
            None
        }
    }

    let mut pvl = complete_empty();
    if let Some(st) = pvl.mLayoutState.as_mut() {
        st.srcList.obtainAndAddLast();
        st.pageEndLimitCP = i32::MAX;
        st.cp = 0;
        st.document = Some(Box::leak(Box::new(HelloDoc)) as *mut dyn Document);
    }
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        pvl.flowToPage(2);
    }))
    .is_err();
    assert!(
        panicked,
        "nonempty src must enter flowPage→flow→DEFERRED flowTextLine (no fake \\f pages)"
    );
}

#[test]
fn TP3_widow_does_not_invent_pages() {
    // processPagination / widow 197 is §1.7 deferred; empty complete must not invent a page.
    let mut pvl = complete_empty();
    assert_eq!(pvl.flowToPage(197), -1);
}
