// Java: cn.wps.moffice.writer.layout.pageview.PageViewParaLayouter
#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::{IStateEnvoy, ParaLayouter};
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// Java: public class PageViewParaLayouter extends ParaLayouter
// Rust's TextLayouter owns the base value. Its page-view dispatch flag retains
// Java's virtual overrides when this wrapper is moved into that owner.
pub struct PageViewParaLayouter { pub super_: ParaLayouter }

impl PageViewParaLayouter {
    // Java: public PageViewParaLayouter(DocumentEnv docEnv, ViewEnv viewEnv, MainState mainState)
    pub fn PageViewParaLayouter(docEnv: DocumentEnv, viewEnv: ViewEnv, mainState: MainState) -> Self {
        let mut base = ParaLayouter::ParaLayouter(docEnv, viewEnv, mainState);
        base.mPageView = true;
        Self { super_: base }
    }

    // Java: protected boolean shouldCalcLastParaAfter(ParaLayouter.IStateEnvoy stateEnvoy)
    pub fn shouldCalcLastParaAfter(&self, stateEnvoy: &dyn IStateEnvoy) -> bool {
        stateEnvoy.getPageEnv().isFirstPageOfSection && !stateEnvoy.getCurPage().hasLines()
    }

    // Java: protected boolean shouldAdjustRightMarginInCharGridModel()
    pub fn shouldAdjustRightMarginInCharGridModel(&self) -> bool { true }
}
