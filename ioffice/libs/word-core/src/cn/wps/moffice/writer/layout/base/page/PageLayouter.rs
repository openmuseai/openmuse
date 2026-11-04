// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.layout.base.LayouterBase
use crate::cn::wps::moffice::writer::layout::base::LayouterBase::{IStateEnvoy as LayouterIStateEnvoy, LayouterBase};
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// java public class PageLayouter extends LayouterBase<E>
#[derive(Default)]
pub struct PageLayouter {
    // java LayouterBase<E> super_
    pub(crate) super_: LayouterBase,
    // java protected final DocumentEnv mDocEnv;
    pub(crate) mDocEnv: DocumentEnv,
    // java protected final ViewEnv mViewEnv;
    pub(crate) mViewEnv: ViewEnv,
    // java PageViewPageLayouter overrides onStart/onEnd. The heap object stays
    // stable when this PageLayouter moves with its owner.
    pub(crate) page_view_self: *mut (),
    pub(crate) page_view_on_start: Option<fn(*mut (), *mut LayoutState)>,
    pub(crate) page_view_on_end: Option<fn(*mut (), *mut LayoutState)>,
    // java PageViewLayouter.preFlow runs around TextLayouter.preFlow.
    pub(crate) page_view_owner: *mut (),
    pub(crate) page_view_pre_prefix: Option<fn(*mut (), *mut LayoutState)>,
    pub(crate) page_view_pre_suffix: Option<fn(*mut (), *mut LayoutState)>,
    pub(crate) page_view_finish: Option<fn(*mut (), *mut LayoutState) -> bool>,
    // java PageViewLayouter.initDrawingLayouter overrides TextLayouter.initDrawingLayouter.
    pub(crate) page_view_init_drawing: Option<fn(*mut ())>,
}

// java public interface IStateEnvoy extends LayouterBase.IStateEnvoy
pub trait IStateEnvoy: LayouterIStateEnvoy {
    // java  TypoPage getCurPage()
    fn getCurPage(&self) -> &TypoPage;
    // java  PageEnv getPageEnv()
    fn getPageEnv(&self) -> &PageEnv;
    // java  ParaEnv getParaEnv()
    fn getParaEnv(&mut self) -> &mut ParaEnv;
    // java  int getCP()
    fn getCP(&self) -> i32;
    // java  void setCurLayoutHeight(int var1)
    fn setCurLayoutHeight(&mut self, var1: i32);
    // java  int getRangeEnd()
    fn getRangeEnd(&self) -> i32;
    // java  Document getDocument()
    fn getDocument(&self) -> Option<&dyn Document>;
}

// java public class PageLayouter extends LayouterBase<E>
impl PageLayouter {
    // java public  PageLayouter(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn PageLayouter(docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java this.mDocEnv = docEnv;
      Self { super_: LayouterBase::default(), mDocEnv: docEnv, mViewEnv: viewEnv, page_view_self: std::ptr::null_mut(), page_view_on_start: None, page_view_on_end: None, page_view_owner: std::ptr::null_mut(), page_view_pre_prefix: None, page_view_pre_suffix: None, page_view_finish: None, page_view_init_drawing: None }
      // java this.mViewEnv = viewEnv;
    }

    pub fn start(&mut self, stateEnvoy: &mut dyn LayouterIStateEnvoy) {
      if !self.super_.bStarted {
        self.super_.verifyEnvoy(Some(stateEnvoy));
        self.onStart(stateEnvoy);
        self.super_.bStarted = true;
      }
    }

    pub fn end(&mut self, stateEnvoy: &mut dyn LayouterIStateEnvoy) {
      if self.super_.bStarted {
        self.onEnd(stateEnvoy);
      }
      self.super_.bStarted = false;
    }

    // java PageLayouter.start dispatches to PageViewPageLayouter.onStart.
    // The hook is returned so the caller can run it after this borrow ends.
    pub fn begin_page_view(&mut self) -> (*mut (), Option<fn(*mut (), *mut LayoutState)>) {
        if self.super_.bStarted {
            return (std::ptr::null_mut(), None);
        }
        self.super_.bStarted = true;
        (self.page_view_self, self.page_view_on_start)
    }

    // java PageLayouter.end dispatches to PageViewPageLayouter.onEnd.
    pub fn finish_page_view(&mut self) -> (*mut (), Option<fn(*mut (), *mut LayoutState)>) {
        if !self.super_.bStarted {
            return (std::ptr::null_mut(), None);
        }
        self.super_.bStarted = false;
        (self.page_view_self, self.page_view_on_end)
    }

    pub fn bind_page_view(&mut self, this: *mut (), on_start: fn(*mut (), *mut LayoutState), on_end: fn(*mut (), *mut LayoutState)) {
        self.page_view_self = this;
        self.page_view_on_start = Some(on_start);
        self.page_view_on_end = Some(on_end);
    }

    pub fn bind_page_view_flow(&mut self, owner: *mut (), prefix: fn(*mut (), *mut LayoutState), suffix: fn(*mut (), *mut LayoutState), finish: fn(*mut (), *mut LayoutState) -> bool) {
        self.page_view_owner = owner;
        self.page_view_pre_prefix = Some(prefix);
        self.page_view_pre_suffix = Some(suffix);
        self.page_view_finish = Some(finish);
    }

    pub fn bind_page_view_init_drawing(&mut self, init_drawing: fn(*mut ())) {
        self.page_view_init_drawing = Some(init_drawing);
    }

    pub fn page_view_init_drawing(&self) -> (*mut (), Option<fn(*mut ())>) {
        (self.page_view_owner, self.page_view_init_drawing)
    }

    pub fn page_view_finish(&self) -> (*mut (), Option<fn(*mut (), *mut LayoutState) -> bool>) {
        (self.page_view_owner, self.page_view_finish)
    }

    pub fn page_view_prefix(&self) -> (*mut (), Option<fn(*mut (), *mut LayoutState)>) {
        (self.page_view_owner, self.page_view_pre_prefix)
    }

    pub fn page_view_suffix(&self) -> (*mut (), Option<fn(*mut (), *mut LayoutState)>) {
        (self.page_view_owner, self.page_view_pre_suffix)
    }

    // java protected void onStart(IStateEnvoy stateEnvoy)
    pub fn onStart(&mut self, stateEnvoy: &mut dyn LayouterIStateEnvoy) {
      let _ = stateEnvoy;
    }

    // java protected void onEnd(IStateEnvoy stateEnvoy)
    pub fn onEnd(&mut self, stateEnvoy: &mut dyn LayouterIStateEnvoy) {
      let _ = stateEnvoy;
    }

    // java protected void clearParaBorderSettingBefore(ParaEnv paraEnv)
    pub fn clearParaBorderSettingBefore(&mut self, paraEnv: &mut ParaEnv) {
      // java paraEnv.init(this.mDocEnv.pools);
      paraEnv.init(&mut self.mDocEnv.pools);
    }

}

impl std::ops::Deref for PageLayouter {
    type Target = LayouterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PageLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
