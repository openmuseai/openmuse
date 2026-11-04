#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.collections.KList
// CARGO_OFF_SURFACE use util::collections::KList::KList;
// Java: import cn.wps.graphics.Rect
// CARGO_OFF_SURFACE use util::graphics::Rect::Rect;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.util.UnitUtil
// CARGO_OFF_SURFACE use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.ITypoRect
use crate::cn::wps::moffice::writer::cache::ITypoRect::ITypoRect;
// Java: import cn.wps.moffice.writer.cache.TypoCPRefUsage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoCPRefUsage::TypoCPRefUsage;
// Java: import cn.wps.moffice.writer.cache.TypoDocument
use crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoHeaderFooterPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoHeaderFooterPage::TypoHeaderFooterPage;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.cache.TypoSections
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoSections::TypoSections;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTemplate
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTemplate::TypoTemplate;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.IInterruptState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::IInterruptState::IInterruptState;
// Java: import cn.wps.moffice.writer.data.PLC
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLC::PLC;
// Java: import cn.wps.moffice.writer.data.PLCSection
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCSection::PLCSection;
// Java: import cn.wps.moffice.writer.layout.base.MainLayouter
use crate::cn::wps::moffice::writer::layout::base::MainLayouter::MainLayouter;
// Java: import cn.wps.moffice.writer.layout.base.SectionLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::SectionLayouter::SectionLayouter;
// Java: import cn.wps.moffice.writer.layout.base.SectionTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::SectionTool::SectionTool;
// Java: import cn.wps.moffice.writer.layout.base.balloons.BalloonCacheLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::balloons::BalloonCacheLayouter::BalloonCacheLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingCacheTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCacheTool::DrawingCacheTool;
// Java: import cn.wps.moffice.writer.layout.base.edit.Editor
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::Editor::Editor;
// Java: import cn.wps.moffice.writer.layout.base.edit.Event
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::Event::Event;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.FootEndNoteEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FootEndNoteEnv::FootEndNoteEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;
// Java: import cn.wps.moffice.writer.layout.base.env.TableCellLayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::footendnote::IFootEndNoteNumLayouter::IFootEndNoteNumLayouter;
// Java: import cn.wps.moffice.writer.layout.base.footendnote.FootEndNoteLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::footendnote::FootEndNoteLayouter::FootEndNoteLayouter;
// Java: import cn.wps.moffice.writer.layout.base.smallerscale.IPageGrid
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::smallerscale::IPageGrid::IPageGrid;
// Java: import cn.wps.moffice.writer.layout.base.table.TableLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::TableLayouter;
// Java: import cn.wps.moffice.writer.layout.base.tools.EnvTools
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::EnvTools::EnvTools;
// Java: import cn.wps.moffice.writer.layout.base.tools.LocksHelper
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::LocksHelper::LocksHelper;
// Java: import cn.wps.moffice.writer.layout.pageview.columns.ColumnsLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::columns::ColumnsLayouter::ColumnsLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.headerfooter.HeaderFooterTemplateContentLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::headerfooter::HeaderFooterTemplateContentLayouter::HeaderFooterTemplateContentLayouter;
// Java: import cn.wps.moffice.writer.service.LayoutServiceCache
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::LayoutServiceCache::LayoutServiceCache;
// Java: import cn.wps.moffice.writer.service.LocateResult
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::LocateResult::LocateResult;
// Java: import cn.wps.moffice.writer.service.ScrollMemoryService
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::ScrollMemoryService::ScrollMemoryService;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewSectionLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewSectionLayouter::PageViewSectionLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewSwapLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewSwapLayouter::PageViewSwapLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewEventHandler
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewEventHandler::PageViewEventHandler;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewLayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewLayoutState::PageViewLayoutState;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewFootEndNoteNumLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewFootEndNoteNumLayouter::PageViewFootEndNoteNumLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewFootEndNoteLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewFootEndNoteLayouter::PageViewFootEndNoteLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewPageLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewParaLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewParaLayouter::PageViewParaLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewLineLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewLineLayouter::PageViewLineLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewDrawingLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewDrawingLayouter::PageViewDrawingLayouter;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewLayoutStateEnvoy
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewLayoutStateEnvoy::PageViewLayoutStateEnvoy;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;


use core::marker::PhantomData as _;
// CARGO_OFF_SURFACE_TYPES
#[derive(Default)]
pub struct KList;
#[derive(Default)]
pub struct Rect;
#[derive(Default)]
pub struct TypoCPRefUsage;
#[derive(Default)]
pub struct ColumnsLayouter;
#[derive(Default)]
pub struct Editor;
#[derive(Default)]
pub struct HeaderFooterTemplateContentLayouter;
#[derive(Default)]
pub struct PageViewSwapLayouter;
impl PageViewSwapLayouter {
    pub fn swapToLocal(&mut self, _sync: bool) {}
    pub fn swapFromLocal(&mut self) -> bool { false }
}
#[derive(Default)]
pub struct LayoutServiceCache;
#[derive(Default)]
pub struct LocksHelper;
#[derive(Default)]
pub struct Event;
pub struct FootEndNoteEnv {
    pub curFootNoteRefCP: i32,
    pub curEndNoteRefCP: i32,
}
impl Default for FootEndNoteEnv {
    fn default() -> Self {
        Self { curFootNoteRefCP: -1, curEndNoteRefCP: -1 }
    }
}
#[derive(Default)]
pub struct IInterruptState;
pub trait IMathDrawer {}
#[derive(Default)]
pub struct ScrollMemoryService;
#[derive(Default)]
pub struct FootEndNoteLayouter;

// java private static final String TAG = null;
const TAG: &str = "";

// java public final class PageViewLayouter extends MainLayouter
#[derive(Default)]
pub struct PageViewLayouter {
    // java MainLayouter super_
    pub(crate) super_: MainLayouter,
    // java private final int MAX_GRAPHS_PER_PARA = 8192;
    MAX_GRAPHS_PER_PARA: i32,
    // java private PageViewSectionLayouter mSectionLayouter;
    mSectionLayouter: Option<Box<crate::cn::wps::moffice::writer::layout::pageview::PageViewSectionLayouter::PageViewSectionLayouter>>,
    // java template index computed before super.preFlow and applied after it.
    mPendingTemplateIndex: i32,
    // java private ColumnsLayouter mColumnsLayouter;
    mColumnsLayouter: Option<Box<ColumnsLayouter>>,
    // java private Editor mEditor;
    mEditor: Option<Box<Editor>>,
    // java private boolean mHasProcessColumns;
    mHasProcessColumns: bool,
    // java private HeaderFooterTemplateContentLayouter mTemplateContentLayouter;
    mTemplateContentLayouter: Option<Box<HeaderFooterTemplateContentLayouter>>,
    // java private TypoCPRefUsage mCPRefUsage = new TypoCPRefUsage();
    mCPRefUsage: TypoCPRefUsage,
    // java private boolean mHasProcessEnsureValidEndPage = false;
    mHasProcessEnsureValidEndPage: bool,
    // java PageViewPageLayouter stored on the heap so its pointer stays valid in PageLayouter.
    mPageViewPageLayouter: Option<Box<crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter>>,
    // java private PageViewSwapLayouter mPageViewBlocksSwap;
    mPageViewBlocksSwap: Option<Box<PageViewSwapLayouter>>,
    // java PageViewLayoutState.footEndNoteEnv — heap so the numbering layouter pointer stays valid.
    foot_end_note_env: Option<Box<crate::cn::wps::moffice::writer::layout::base::env::FootEndNoteEnv::FootEndNoteEnv>>,
}

fn page_view_on_start(this: *mut (), state: *mut LayoutState) {
    unsafe {
        (*this.cast::<crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter>()).onStart(&mut *state);
    }
}

fn page_view_on_end(this: *mut (), state: *mut LayoutState) {
    unsafe {
        (*this.cast::<crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter>()).onEnd(&mut *state);
    }
}

fn page_view_init_drawing(this: *mut ()) {
    unsafe {
        (*this.cast::<PageViewLayouter>()).initDrawingLayouter();
    }
}

fn page_view_pre_prefix(this: *mut (), state: *mut LayoutState) {
    unsafe {
        let layouter = &mut *this.cast::<PageViewLayouter>();
        let state = &mut *state;
        if let Some(section) = layouter.mSectionLayouter.as_mut() {
            section.start(state);
        }
        let sec_start = layouter.super_.super_.mMainState.secStart;
        state.pageEnv.isFirstPageOfSection = state.cp == sec_start && state.cellStateList.isEmpty();
        layouter.mPendingTemplateIndex = layouter.mSectionLayouter.as_ref()
            .map(|section| section.getTemplateIndex(state))
            .unwrap_or(0);
        layouter.super_.super_.mMainState.needTableFit = false;
        let main = &mut layouter.super_.super_.mMainState;
        let snap = &mut main.snapshot as *mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
        main.layoutPage.alloc(&mut *snap);
        // Java curPage and layoutPage are the same object, so alloc is visible
        // to the line list. A new page starts with no lines.
        if state.curPage.is_none() {
            state.curPage = Some(crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default());
        }
        if let Some(page) = state.curPage.as_mut() {
            main.layoutPage.fill_page(page);
            page.lines.clear();
        }
        state.committed_line_advances.clear();
        state.table_line_advances.clear();
    }
}

fn page_view_finish(this: *mut (), state: *mut LayoutState) -> bool {
    unsafe { (*this.cast::<PageViewLayouter>()).finishPage(&mut *state) }
}

fn page_view_pre_suffix(this: *mut (), state: *mut LayoutState) {
    unsafe {
        let layouter = &mut *this.cast::<PageViewLayouter>();
        let state = &mut *state;
        let main = &mut layouter.super_.super_.mMainState;
        main.layoutPage.setTemplateIndex(layouter.mPendingTemplateIndex);
        main.layoutPage.setDrawingLayers(main.drawingLayers.getPointer());
        state.pageEnv.minLayoutHeight = 0;
        state.curLayoutHeight = state.pageEnv.minLayoutHeight;
        state.pageEnv.maxLayoutHeight = state.pageEnv.getCoreHeight();
        if main.layoutPage.getTemplate() == 0 {
            main.headerFooterDrawingLayers.clear();
        }
        layouter.mHasProcessColumns = false;
        layouter.super_.super_.mEnableProcessPagination = true;
        if let Some(num) = layouter.super_.super_.mDocEnv.footEndNoteNumLayouter.as_mut() {
          // java numLayouter.reset(this.mStateSnapshot);
          num.reset();
        }
        let doc_len = state.document.map(|doc| (*doc).getLength()).unwrap_or(0);
        if main.columns <= 1 || (state.cp >= doc_len && layouter.super_.mFinishLoading) {
            state.atLeastOneLine = true;
        }
    }
}

// java public final class PageViewLayouter extends MainLayouter
impl PageViewLayouter {
    // java public PageViewLayouter(MainState, DocumentEnv, ViewEnv, LocksHelper, KList<Event>, LayoutServiceCache)
    // Java TextLayouter constructor calls init(), which calls the PageViewLayouter override of initLayouters.
    pub fn PageViewLayouter(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      let mut this = Self::default();
      this.super_.super_ = crate::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter::TextLayouter(mainState, docEnv, viewEnv);
      this.MAX_GRAPHS_PER_PARA = 8192;
      // java this.mPageViewBlocksSwap = new PageViewSwapLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
      this.mPageViewBlocksSwap = Some(Box::new(PageViewSwapLayouter::default()));
      // java TextLayouter.init: createGraphBuff already ran in TextLayouter ctor, then initLayouters.
      this.initLayouters();
      this.super_.super_.finish_java_init();
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mTemplateContentLayouter != null)
      if let Some(mTemplateContentLayouter) = self.mTemplateContentLayouter.as_mut() {
        // java this.mTemplateContentLayouter.dispose();
        mTemplateContentLayouter.dispose();
        // java this.mTemplateContentLayouter = null;
        mTemplateContentLayouter = None;
      }
      // java if (this.mSectionLayouter != null)
      if let Some(mSectionLayouter) = self.mSectionLayouter.as_mut() {
        // java this.mSectionLayouter.dispose();
        mSectionLayouter.dispose();
        // java this.mSectionLayouter = null;
        mSectionLayouter = None;
      }
      // java if (null != this.mColumnsLayouter)
      if let Some(mColumnsLayouter) = self.mColumnsLayouter.as_mut() {
        // java this.mColumnsLayouter.dispose();
        mColumnsLayouter.dispose();
        // java this.mColumnsLayouter = null;
        mColumnsLayouter = None;
      }
      // java if (null != this.mEditor)
      if let Some(mEditor) = self.mEditor.as_mut() {
        // java this.mEditor.dispose();
        mEditor.dispose();
        // java this.mEditor = null;
        mEditor = None;
      }
      // java super.dispose();
      self.super_.dispose();
      */
    }

    // java protected void preFlow(LayoutState layoutState)
    pub fn preFlow(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)layoutState;
      let mut pageViewLayoutState: PageViewLayoutState = (layoutState).downcast_ref::<PageViewLayoutState>().unwrap();
      // java this.mSectionLayouter.start((SectionLayouter.IStateEnvoy)pageViewLayoutState.getEnvoy());
      self.mSectionLayouter.start((pageViewLayoutState.getEnvoy()).downcast_ref::<SectionLayouter::IStateEnvoy>().unwrap());
      // java layoutState.pageEnv.isFirstPageOfSection = layoutState.cp == this.mMainState.secStart && pageViewLayoutState.cellStateList.isEmpty();
      layoutState.pageEnv.isFirstPageOfSection = layoutState.cp == self.mMainState.secStart && pageViewLayoutState.cellStateList.is_empty();
      // java int templateIndex = this.mSectionLayouter.getTemplateIndex(pageViewLayoutState.getEnvoy());
      let mut templateIndex: i32 = self.mSectionLayouter.getTemplateIndex(pageViewLayoutState.getEnvoy());
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java this.mMainState.needTableFit = false;
      self.mMainState.needTableFit = false;
      // java this.mMainState.layoutPage.alloc(snapshot);
      self.mMainState.layoutPage.alloc(snapshot);
      // java this.mMainState.revisions.init(snapshot);
      self.mMainState.revisions.init(snapshot);
      // java this.mMainState.footEndNoteCache.clear();
      self.mMainState.footEndNoteCache.clear();
      // java this.mCPRefUsage.init(snapshot);
      self.mCPRefUsage.init(snapshot);
      // java super.preFlow(layoutState);
      self.super_.preFlow(layoutState);
      // java TypoLayoutPage layoutPage = this.mMainState.layoutPage;
      let mut layoutPage: TypoLayoutPage = self.mMainState.layoutPage;
      // java layoutPage.setTemplateIndex(templateIndex);
      layoutPage.setTemplateIndex(templateIndex);
      // java PageEnv pageEnv = layoutState.pageEnv;
      let mut pageEnv: PageEnv = layoutState.pageEnv;
      // java pageEnv.minLayoutHeight = 0;
      pageEnv.minLayoutHeight = 0;
      // java layoutState.curLayoutHeight = pageEnv.minLayoutHeight;
      layoutState.curLayoutHeight = pageEnv.minLayoutHeight;
      // java pageEnv.maxLayoutHeight = pageEnv.getCoreHeight();
      pageEnv.maxLayoutHeight = pageEnv.getCoreHeight();
      // java layoutPage.setDrawingLayers(this.mMainState.drawingLayers.getPointer());
      layoutPage.setDrawingLayers(self.mMainState.drawingLayers.getPointer());
      // java int template = layoutPage.getTemplate();
      let mut template: i32 = layoutPage.getTemplate();
      // java if (0 != template)
      if 0 != template {
        // java int headerPage = TypoTemplate.getHeader(template, snapshot);
        let mut headerPage: i32 = TypoTemplate::getHeader__int_typosnapshot(template, snapshot);
        // java boolean checkedMirrorMargin = false;
        let mut checkedMirrorMargin: bool = false;
        // java if (this.hasEffectContent(headerPage, pageEnv))
        if self.hasEffectContent(headerPage, &pageEnv) {
          // java pageEnv.minLayoutHeight = Math.max(0, TypoHeaderFooterPage.getBottom(headerPage, snapshot) - pageEnv.marginTop);
          pageEnv.minLayoutHeight = (0).max(TypoHeaderFooterPage::getBottom(headerPage, snapshot) - pageEnv.marginTop);
          // java checkedMirrorMargin = this.checkAndSetMirrorMaringOffset(pageEnv, this.mMainState.layoutPage, headerPage);
          checkedMirrorMargin = self.checkAndSetMirrorMaringOffset(&mut pageEnv, self.mMainState.layoutPage, headerPage);
        }
        // java int footerPage = TypoTemplate.getFooter(template, snapshot);
        let mut footerPage: i32 = TypoTemplate::getFooter__int_typosnapshot(template, snapshot);
        // java if (this.hasEffectContent(footerPage, pageEnv))
        if self.hasEffectContent(footerPage, &pageEnv) {
          // java pageEnv.maxLayoutHeight = Math.min(pageEnv.maxLayoutHeight, TypoHeaderFooterPage.getTop(footerPage, snapshot) - pageEnv.marginTop);
          pageEnv.maxLayoutHeight = (pageEnv.maxLayoutHeight).min(TypoHeaderFooterPage::getTop(footerPage, snapshot) - pageEnv.marginTop);
          // java if (!checkedMirrorMargin)
          if !checkedMirrorMargin {
            // java this.checkAndSetMirrorMaringOffset(pageEnv, this.mMainState.layoutPage, headerPage);
            self.checkAndSetMirrorMaringOffset(&mut pageEnv, self.mMainState.layoutPage, headerPage);
          }
        }
        // java layoutState.curLayoutHeight = pageEnv.minLayoutHeight;
        layoutState.curLayoutHeight = pageEnv.minLayoutHeight;
        // java int pHeaderFooterDrawingLayers = TypoTemplate.getDrawingLayers(template, snapshot);
        let mut pHeaderFooterDrawingLayers: i32 = TypoTemplate::getDrawingLayers__int_typosnapshot(template, snapshot);
        // java if (0 != pHeaderFooterDrawingLayers)
        if 0 != pHeaderFooterDrawingLayers {
          // java this.mMainState.headerFooterDrawingLayers.assignment(pHeaderFooterDrawingLayers, snapshot);
          self.mMainState.headerFooterDrawingLayers.assignment(pHeaderFooterDrawingLayers, snapshot);
        }
        // java else
        else {
          // java this.mMainState.headerFooterDrawingLayers.clear();
          self.mMainState.headerFooterDrawingLayers.clear();
        }
        // java int args = TypoTemplate.getArgs(template, snapshot);
        let mut args: i32 = TypoTemplate::getArgs(template, snapshot);
        // java if (0 != args)
        if 0 != args {
          // java if (null == this.mTemplateContentLayouter)
          if self.mTemplateContentLayouter.is_none() {
            // java this.mTemplateContentLayouter = new HeaderFooterTemplateContentLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
            self.mTemplateContentLayouter = HeaderFooterTemplateContentLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
          }
          // java this.mTemplateContentLayouter.flowContent(layoutPage, args);
          self.mTemplateContentLayouter.flowContent(layoutPage, args);
        }
      }
      // java else
      else {
        // java this.mMainState.headerFooterDrawingLayers.clear();
        self.mMainState.headerFooterDrawingLayers.clear();
      }
      // java layoutPage.setStartState(this.mStateSnapshot.saveToTypoMem(this.mMainState.snapshot));
      layoutPage.setStartState(self.mStateSnapshot.saveToTypoMem(self.mMainState.snapshot));
      // java PageViewFootEndNoteNumLayouter numLayouter = (PageViewFootEndNoteNumLayouter)this.mDocEnv.footEndNoteNumLayouter;
      let mut numLayouter: PageViewFootEndNoteNumLayouter = (self.mDocEnv.footEndNoteNumLayouter).downcast_ref::<PageViewFootEndNoteNumLayouter>().unwrap();
      // java numLayouter.reset(this.mStateSnapshot);
      numLayouter.reset(self.mStateSnapshot);
      // java this.mHasProcessColumns = false;
      self.mHasProcessColumns = false;
      // java this.mEnableProcessPagination = true;
      self.mEnableProcessPagination = true;
      // java if (TypoList.size(this.mMainState.headerFooterDrawingLayers.getWraps(), snapshot) > 0)
      if TypoList::size__int_typosnapshot(self.mMainState.headerFooterDrawingLayers.getWraps(), snapshot) > 0 {
        // java this.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
        self.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
      }
      // java if (this.mMainState.columns <= 1 || layoutState.cp >= layoutState.document.getLength() && this.mFinishLoading)
      if self.mMainState.columns <= 1 || layoutState.cp >= layoutState.document.getLength() && self.mFinishLoading {
        // java FootEndNoteEnv footEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
        let mut footEndNoteEnv: FootEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
        // java pageViewLayoutState.atLeastOneLine = true;
        pageViewLayoutState.atLeastOneLine = true;
        // java boolean needRestartSection = false;
        let mut needRestartSection: bool = false;
        // java if (footEndNoteEnv.curFootNoteRefCP != -1)
        if footEndNoteEnv.curFootNoteRefCP != -1 {
          // java PageViewFootEndNoteLayouter pageViewFootEndNoteLayouter = (PageViewFootEndNoteLayouter)this.getFootEndNoteLayouter();
          let mut pageViewFootEndNoteLayouter: PageViewFootEndNoteLayouter = (self.getFootEndNoteLayouter()).downcast_ref::<PageViewFootEndNoteLayouter>().unwrap();
          // java if (pageViewFootEndNoteLayouter.flowBreakFootNotes(pageViewLayoutState))
          if pageViewFootEndNoteLayouter.flowBreakFootNotes(pageViewLayoutState) {
            // java pageViewLayoutState.setFinishPage(true);
            pageViewLayoutState.setFinishPage(true);
          }
          // java else
          else {
            // java pageViewLayoutState.atLeastOneLine = false;
            pageViewLayoutState.atLeastOneLine = false;
            // java if (pageViewLayoutState.cp >= this.mMainState.secEnd)
            if pageViewLayoutState.cp >= self.mMainState.secEnd {
              // java needRestartSection = true;
              needRestartSection = true;
            }
          }
        }
        // java if (!pageViewLayoutState.isFinishPage() && footEndNoteEnv.curEndNoteRefCP != -1)
        if !pageViewLayoutState.isFinishPage() && footEndNoteEnv.curEndNoteRefCP != -1 {
          // java PageViewFootEndNoteLayouter pageViewFootEndNoteLayouter = (PageViewFootEndNoteLayouter)this.getFootEndNoteLayouter();
          let mut pageViewFootEndNoteLayouter: PageViewFootEndNoteLayouter = (self.getFootEndNoteLayouter()).downcast_ref::<PageViewFootEndNoteLayouter>().unwrap();
          // java if (!pageViewFootEndNoteLayouter.flowEndNotes(pageViewLayoutState))
          if !pageViewFootEndNoteLayouter.flowEndNotes(pageViewLayoutState) {
            // java needRestartSection = true;
            needRestartSection = true;
          }
          // java else
          else {
            // java pageViewLayoutState.setFinishPage(true);
            pageViewLayoutState.setFinishPage(true);
          }
        }
        // java if (needRestartSection)
        if needRestartSection {
          // java this.mSectionLayouter.end(pageViewLayoutState.getEnvoy());
          self.mSectionLayouter.end(pageViewLayoutState.getEnvoy());
          // java int breakCode = this.mMainState.secBreakCode;
          let mut breakCode: i32 = self.mMainState.secBreakCode;
          // java pageViewLayoutState.setFinishPage(breakCode != 0 || !this.mDocEnv.canContinueFlowNextSec);
          pageViewLayoutState.setFinishPage(breakCode != 0 || !self.mDocEnv.canContinueFlowNextSec);
          // java if (!pageViewLayoutState.isFinishPage())
          if !pageViewLayoutState.isFinishPage() {
            // java this.mSectionLayouter.startForNoBreakPage(pageViewLayoutState.getEnvoy());
            self.mSectionLayouter.startForNoBreakPage(pageViewLayoutState.getEnvoy());
          }
        }
      }
      */
    }

    // java private boolean checkAndSetMirrorMaringOffset(PageEnv pageEnv, TypoLayoutPage newPage, int pHeaderFooterPage)
    pub fn checkAndSetMirrorMaringOffset(&mut self, pageEnv: &mut PageEnv, newPage: TypoLayoutPage, pHeaderFooterPage: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pHeaderFooterPage != 0 && this.mDocEnv.mirrorMargins)
      if pHeaderFooterPage != 0 && self.mDocEnv.mirrorMargins {
        // java if (this.mDocEnv.mirrorMargins)
        if self.mDocEnv.mirrorMargins {
          // java int mirrorOffset = pageEnv.marginLeft - TypoHeaderFooterPage.getMarginLeft(pHeaderFooterPage, this.mMainState.snapshot);
          let mut mirrorOffset: i32 = pageEnv.marginLeft - TypoHeaderFooterPage::getMarginLeft(pHeaderFooterPage, self.mMainState.snapshot);
          // java if (mirrorOffset != 0)
          if mirrorOffset != 0 {
            // java newPage.setMirrorMarginOffset(mirrorOffset);
            newPage.setMirrorMarginOffset(mirrorOffset);
            // java return true;
            return true;
          }
        }
        // java return false;
        return false;
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java  boolean hasEffectContent(int headerFooterPage, PageEnv pageEnv)
    pub fn hasEffectContent(&self, headerFooterPage: i32, pageEnv: &PageEnv) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (pageEnv.textFlow)
      match pageEnv.textFlow {
        // java case 0: case 4:
        0 | 4 => {
          // java if (0 == headerFooterPage)
          if 0 == headerFooterPage {
            // java return false;
            return false;
          }
          // java return TypoHeaderFooterPage.isEffect(headerFooterPage, this.mMainState.snapshot);
          return TypoHeaderFooterPage::isEffect__int_typosnapshot(headerFooterPage, self.mMainState.snapshot);
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
      */
    }

    // java protected void postFlow(LayoutState layoutState)
    pub fn postFlow(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)layoutState;
      let mut pageViewLayoutState: PageViewLayoutState = (layoutState).downcast_ref::<PageViewLayoutState>().unwrap();
      // java PageViewFootEndNoteLayouter pageViewFootEndNoteLayouter = (PageViewFootEndNoteLayouter)this.mFootEndNoteLayouter;
      let mut pageViewFootEndNoteLayouter: PageViewFootEndNoteLayouter = (self.mFootEndNoteLayouter).downcast_ref::<PageViewFootEndNoteLayouter>().unwrap();
      // java if (null != pageViewFootEndNoteLayouter)
      if true {
        // java pageViewFootEndNoteLayouter.flowFootNoteBeneathText(pageViewLayoutState);
        pageViewFootEndNoteLayouter.flowFootNoteBeneathText(pageViewLayoutState);
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java TypoLayoutPage layoutPage = this.mMainState.layoutPage;
      let mut layoutPage: TypoLayoutPage = self.mMainState.layoutPage;
      // java if (layoutPage.hasLines())
      if layoutPage.hasLines() {
        // java int lastLine = TypoList.last(layoutPage.getLines(), snapshot);
        let mut lastLine: i32 = TypoList::last__int_typosnapshot(layoutPage.getLines(), snapshot);
        // java boolean shouldRecCellEnv = false;
        let mut shouldRecCellEnv: bool = false;
        // java switch (TypoObjBase.getType(lastLine, snapshot))
        match TypoObjBase::getType(lastLine, snapshot) {
          // java case 3:
          3 => {
            // java shouldRecCellEnv = true;
            shouldRecCellEnv = true;
          }
          // java default:
          _ => {
            // java if (shouldRecCellEnv)
            if shouldRecCellEnv {
              // java this.mDocEnv.pools.recycleTableCellLayoutStates(layoutState.cellStateList);
              self.mDocEnv.pools.recycleTableCellLayoutStates(layoutState.cellStateList);
            }
          }
        }
      }
      // java FootEndNoteEnv footEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
      let mut footEndNoteEnv: FootEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
      // java if (-1 != footEndNoteEnv.curFootNoteRefCP && null != footEndNoteEnv.curFootNoteLayoutState)
      if -1 != footEndNoteEnv.curFootNoteRefCP && footEndNoteEnv.curFootNoteLayoutState.is_some() {
        // java EnvTools.clearPageForCellEnvList(footEndNoteEnv.curFootNoteLayoutState.cellStateList, this.mDocEnv.pools);
        EnvTools::clearPageForCellEnvList(footEndNoteEnv.curFootNoteLayoutState.cellStateList, self.mDocEnv.pools);
      }
      // java if (-1 != footEndNoteEnv.curEndNoteRefCP && null != footEndNoteEnv.curEndNoteLayoutState)
      if -1 != footEndNoteEnv.curEndNoteRefCP && footEndNoteEnv.curEndNoteLayoutState.is_some() {
        // java EnvTools.clearPageForCellEnvList(footEndNoteEnv.curEndNoteLayoutState.cellStateList, this.mDocEnv.pools);
        EnvTools::clearPageForCellEnvList(footEndNoteEnv.curEndNoteLayoutState.cellStateList, self.mDocEnv.pools);
      }
      // java super.postFlow(layoutState);
      self.super_.postFlow(layoutState);
      // java this.mMainState.revisions.cleanInvalid(this.mMainState.layoutPage.getPointer());
      self.mMainState.revisions.cleanInvalid(self.mMainState.layoutPage.getPointer());
      // java if (this.mViewEnv.getShowBalloon())
      if self.mViewEnv.getShowBalloon() {
        // java if (null == this.balloonCacheLayouter)
        if self.balloonCacheLayouter.is_none() {
          // java this.balloonCacheLayouter = new BalloonCacheLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
          self.balloonCacheLayouter = BalloonCacheLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
        }
        // java this.balloonCacheLayouter.makeBalloonCache(layoutPage.getPointer());
        self.balloonCacheLayouter.makeBalloonCache(layoutPage.getPointer());
      }
      // java this.mMainState.curPageTop = layoutPage.getBottom();
      self.mMainState.curPageTop = layoutPage.getBottom();
      // java ++this.mMainState.curPageIndex;
      { let _tmp = self.mMainState.curPageIndex; self.mMainState.curPageIndex += 1; _tmp };
      // java ++this.mMainState.curSecPageIndex;
      { let _tmp = self.mMainState.curSecPageIndex; self.mMainState.curSecPageIndex += 1; _tmp };
      // java if (!this.mSectionLayouter.hasStart())
      if !self.mSectionLayouter.hasStart() {
        // java this.mSectionLayouter.start((SectionLayouter.IStateEnvoy)pageViewLayoutState.getEnvoy());
        self.mSectionLayouter.start((pageViewLayoutState.getEnvoy()).downcast_ref::<SectionLayouter::IStateEnvoy>().unwrap());
        // java this.mSectionLayouter.reset();
        self.mSectionLayouter.reset();
      }
      // java this.makeSnapshot(layoutState);
      self.super_.makeSnapshot(layoutState);
      // java layoutPage.setEndState(this.mStateSnapshot.saveToTypoMem(snapshot));
      layoutPage.setEndState(self.mStateSnapshot.saveToTypoMem(snapshot));
      // java int revison = this.mMainState.revisions.commit();
      let mut revison: i32 = self.mMainState.revisions.commit();
      // java if (0 != revison)
      if 0 != revison {
        // java layoutPage.setRevisions(revison);
        layoutPage.setRevisions(revison);
      }
      // java layoutPage.setCPRefUsage(this.mCPRefUsage.commit());
      layoutPage.setCPRefUsage(self.mCPRefUsage.commit());
      // java layoutPage.setMemEnd(snapshot.getAllocSize());
      layoutPage.setMemEnd(snapshot.getAllocSize());
      // java int left = (this.mMainState.snapshot.getDocumentWidth() - layoutPage.width()) / 2;
      let mut left: i32 = self.mMainState.snapshot.getDocumentWidth() - layoutPage.width() / 2;
      // java layoutPage.offsetTo(left, layoutPage.getTop());
      layoutPage.offsetTo(left, layoutPage.getTop());
      // java if (this.mMainState.needTableFit)
      if self.mMainState.needTableFit {
        // java layoutPage.setHasTableNeedFit(true);
        layoutPage.setHasTableNeedFit(true);
      }
      // java layoutPage.setPosIndex(pageViewLayoutState.pagePos);
      layoutPage.setPosIndex(pageViewLayoutState.pagePos);
      // java this.mMainState.snapshot.setModify(pageViewLayoutState.pagePos);
      self.mMainState.snapshot.setModify(pageViewLayoutState.pagePos);
      // java this.mMainState.snapshot.getPosVisitor().addPage(pageViewLayoutState.pagePos, layoutPage.width(), layoutPage.height());
      self.mMainState.snapshot.getPosVisitor().addPage(pageViewLayoutState.pagePos, layoutPage.width(), layoutPage.height());
      // java this.mMainState.snapshot.insertPage(layoutPage.getPointer(), pageViewLayoutState.pagePos);
      self.mMainState.snapshot.insertPage(layoutPage.getPointer(), pageViewLayoutState.pagePos);
      // java this.mViewEnv.onPageInserted(this.mMainState.layoutPage.getPointer(), pageViewLayoutState.pagePos);
      self.mViewEnv.onPageInserted(self.mMainState.layoutPage.getPointer(), pageViewLayoutState.pagePos);
      // java ++pageViewLayoutState.pagePos;
      { let _tmp = pageViewLayoutState.pagePos; pageViewLayoutState.pagePos += 1; _tmp };
      // java this.mDocEnv.pools.recycleDrawingCacheItems(this.mMainState.floatsCache);
      self.mDocEnv.pools.recycleDrawingCacheItems(self.mMainState.floatsCache);
      */
    }

    // java public TypoLayoutPage flowPage()
    pub fn flowPage(&mut self) -> Option<TypoLayoutPage> {
      // The owner address is stable for this call. Java fields are references.
      self.rebind_java_aliases();
      // java TypoLayoutPage result = null;
      let mut result: Option<TypoLayoutPage> = None;
      if let Some(mut st) = self.super_.mLayoutState.take() {
        // java if (!this.isComplete(this.mLayoutState))
        if !self.isComplete(&st) {
          // java this.flow(this.mLayoutState);
          self.super_.super_.flow(&mut st);
          // java result = this.mMainState.layoutPage;
          result = Some(self.super_.super_.mMainState.layoutPage.clone());
          // java TypoRectCoreImpl rect = TypoRectCoreImpl.obtain();
          let mut rect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
          // java rect.set((ITypoRect)result);
          // rust: TypoLayoutPage is not ITypoRect on this compile surface
          let _ = &rect;
          // java this.onModified(rect);
          self.super_.onModified(None);
          // java rect.recycle();
          let _ = rect;
        }
        self.super_.mLayoutState = Some(st);
      }
      // java return result;
      return result;
    }

    // java public void changePageSize()
    pub fn changePageSize(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FontStyleFactory.setSmartFontSize(false);
      FontStyleFactory::setSmartFontSize(false);
      */
    }

    // java public boolean ensureValidEndPage(Rect rect)
    pub fn ensureValidEndPage(&mut self, rect: Option<&Rect>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mHasProcessEnsureValidEndPage)
      if self.mHasProcessEnsureValidEndPage {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)this.mLayoutState;
        let mut pageViewLayoutState: PageViewLayoutState = (self.mLayoutState).downcast_ref::<PageViewLayoutState>().unwrap();
        // java this.mSectionLayouter.end(pageViewLayoutState.getEnvoy());
        self.mSectionLayouter.end(pageViewLayoutState.getEnvoy());
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int pList = snapshot.getLayoutPageList();
        let mut pList: i32 = snapshot.getLayoutPageList();
        // java int size = TypoList.size(pList, snapshot);
        let mut size: i32 = TypoList::size__int_typosnapshot(pList, snapshot);
        // java boolean hasRecycle = size > pageViewLayoutState.pagePos;
        let mut hasRecycle: bool = size > pageViewLayoutState.pagePos;
        let mut i: i32 = pageViewLayoutState.pagePos;
        // java for(int i = pageViewLayoutState.pagePos; i < size; ++i)
        while i < size {
          // java int page = TypoList.quickGet(i, pList, snapshot);
          let mut page: i32 = TypoList::quickGet__int_int_typosnapshot(i, pList, snapshot);
          // java this.mViewEnv.onPageDeleted(page);
          self.mViewEnv.onPageDeleted(page);
          // java if (null != this.mLayoutServiceCache)
          if let Some(mLayoutServiceCache) = self.mLayoutServiceCache.as_mut() {
            // java this.mLayoutServiceCache.onRemove(page);
            mLayoutServiceCache.onRemove(page);
          }
        }
        // java TypoList.setSize(pageViewLayoutState.pagePos, pList, snapshot);
        TypoList::setSize(pageViewLayoutState.pagePos, pList, snapshot);
        // java if (pageViewLayoutState.pagePos > 0)
        if pageViewLayoutState.pagePos > 0 {
          // java snapshot.setDocumentBottom(TypoLayoutPage.getBottom(TypoList.last(pList, snapshot), snapshot));
          snapshot.setDocumentBottom(TypoLayoutPage::getBottom(TypoList::last__int_typosnapshot(pList, snapshot), snapshot));
        }
        // java if (null != this.mTemplateContentLayouter)
        if let Some(mTemplateContentLayouter) = self.mTemplateContentLayouter.as_mut() {
          // java this.mTemplateContentLayouter.updateTemplateContent(rect);
          mTemplateContentLayouter.updateTemplateContent(rect);
        }
        // java if (null != snapshot.getPosVisitor())
        if snapshot.getPosVisitor().is_some() {
          // java snapshot.getPosVisitor().updateGridMinLeft();
          snapshot.getPosVisitor().updateGridMinLeft();
          // java snapshot.getPosVisitor().setMinLeft(0);
          snapshot.getPosVisitor().setMinLeft(0);
        }
        // java this.mHasProcessEnsureValidEndPage = true;
        self.mHasProcessEnsureValidEndPage = true;
        // java return hasRecycle;
        return hasRecycle;
      }
      */
    }

    // java protected void initLayouters()
    pub fn initLayouters(&mut self) {
      // java this.mLayoutState = this.createLayoutState();
      let mut state = self.createLayoutState();
      // java this.mLayoutState.curPage = this.mMainState.layoutPage;
      let mut cur_page = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default();
      self.super_.super_.mMainState.layoutPage.fill_page(&mut cur_page);
      state.curPage = Some(cur_page);
      self.super_.mLayoutState = Some(Box::new(state));
      // java this.mPageLayouter = new PageViewPageLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
      let main = &mut self.super_.super_.mMainState as *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
      let doc = &mut self.super_.super_.mDocEnv as *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
      let view = &mut self.super_.super_.mViewEnv as *mut crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
      self.mPageViewPageLayouter = Some(Box::new(
        crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter::PageViewPageLayouter(main, doc, view),
      ));
      let page_view = self.mPageViewPageLayouter.as_mut().map(|p| {
        p.as_mut() as *mut crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter as *mut ()
      }).unwrap_or(std::ptr::null_mut());
      let main = &mut self.super_.super_.mMainState as *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
      let doc = &mut self.super_.super_.mDocEnv as *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
      let view = &mut self.super_.super_.mViewEnv as *mut crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
      self.mSectionLayouter = Some(Box::new(
        crate::cn::wps::moffice::writer::layout::pageview::PageViewSectionLayouter::PageViewSectionLayouter::new(main, doc, view),
      ));
      let owner = self as *mut PageViewLayouter as *mut ();
      let mut page_layouter = crate::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter::default();
      page_layouter.bind_page_view(page_view, page_view_on_start, page_view_on_end);
      page_layouter.bind_page_view_flow(owner, page_view_pre_prefix, page_view_pre_suffix, page_view_finish);
      page_layouter.bind_page_view_init_drawing(page_view_init_drawing);
      self.super_.super_.mPageLayouter = Some(page_layouter);
      // java this.mParaLayouter = new PageViewParaLayouter(this.mDocEnv, this.mViewEnv, this.mMainState);
      self.super_.super_.mParaLayouter = Some(
        crate::cn::wps::moffice::writer::layout::pageview::PageViewParaLayouter::PageViewParaLayouter::PageViewParaLayouter(
          crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv::default(),
          crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv::default(),
          crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState::default(),
        ).super_,
      );
      self.rebind_java_aliases();
      // java this.mLineLayouter = new PageViewLineLayouter(...); calcParaBefore is the override.
      if let Some(line) = self.super_.super_.mLineLayouter.as_mut() {
        line.mPageView = true;
      }
      // java if (null != this.mDocEnv.footEndNoteNumLayouter) this.mDocEnv.footEndNoteNumLayouter.dispose();
      if let Some(old) = self.super_.super_.mDocEnv.footEndNoteNumLayouter.as_mut() {
        old.dispose();
      }
      self.super_.super_.mDocEnv.footEndNoteNumLayouter = None;
      // java this.mDocEnv.footEndNoteNumLayouter = new PageViewFootEndNoteNumLayouter(..., layoutState.footEndNoteEnv);
      self.foot_end_note_env = Some(Box::new(
        crate::cn::wps::moffice::writer::layout::base::env::FootEndNoteEnv::FootEndNoteEnv::new(),
      ));
      let env_ptr = self.foot_end_note_env.as_mut().unwrap().as_mut()
        as *mut crate::cn::wps::moffice::writer::layout::base::env::FootEndNoteEnv::FootEndNoteEnv;
      self.super_.super_.mDocEnv.footEndNoteNumLayouter = Some(Box::new(
        crate::cn::wps::moffice::writer::layout::pageview::PageViewFootEndNoteNumLayouter::PageViewFootEndNoteNumLayouter::new(
          std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), env_ptr,
        ),
      ));
      // java FlowLayout.doInit: this.mMainState.footEndNoteCache = new FootNoteCache(this.mPools);
      if self.super_.super_.mMainState.footEndNoteCache.is_none() {
        self.super_.super_.mMainState.footEndNoteCache = Some(
          crate::cn::wps::moffice::writer::layout::base::env::MainState::FootNoteCache::new(),
        );
      }
    }

    // java protected LayoutState createLayoutState()
    pub fn createLayoutState(&self) -> LayoutState {
      // java return new PageViewLayoutState((PageViewLayoutStateEnvoy)null);
      LayoutState::LayoutState(None)
    }

    // java protected void initTableLayouter()
    pub fn initTableLayouter(&mut self) {
      // java this.mTableLayouter = new TableLayouter(this, this.mDocEnv, this.mViewEnv, this.mMainState);
      self.super_.super_.initTableLayouter();
    }

    // java protected void initDrawingLayouter()
    pub fn initDrawingLayouter(&mut self) {
      // java this.mDrawingLayouter = new PageViewDrawingLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this.mLineLayouter);
      // PageViewDrawingLayouter's constructor only calls MainDrawingLayouter's constructor.
      // getQuasiFloatTopOfPara is the override; MainDrawingLayouter is not on this call yet.
      self.super_.super_.install_drawing_layouter();
    }

    // java protected void initFootEndNoteLayouter()
    pub fn initFootEndNoteLayouter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteLayouter = new PageViewFootEndNoteLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
      self.mFootEndNoteLayouter = PageViewFootEndNoteLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
      */
    }

    // java protected boolean finishPage(LayoutState layoutState)
    pub fn finishPage(&mut self, layoutState: &mut LayoutState) -> bool {
      // java this.mHasProcessColumns = false;
      self.mHasProcessColumns = false;
      if let Some(para) = self.super_.super_.mParaLayouter.as_mut() {
        para.end(layoutState.getEnvoy());
      }
      // java flowFootNoteBeneathText runs only when a foot/end layouter exists.
      // That layouter is still outside this compile surface, so both break flags stay false.
      if layoutState.cp < self.super_.super_.mMainState.secEnd {
        return true;
      }
      let break_code = self.super_.super_.mMainState.secBreakCode;
      let close_page = break_code != 0 || !self.super_.super_.mDocEnv.canContinueFlowNextSec;
      if let Some(section) = self.mSectionLayouter.as_mut() {
        section.end();
      }
      if self.super_.super_.mMainState.layoutPage.getFootnotes() != 0 {
        return true;
      }
      if !close_page {
        if let Some(section) = self.mSectionLayouter.as_mut() {
          section.startForNoBreakPage(layoutState);
        }
      }
      return close_page;
      /* DEFERRED_NON_FUNCTION
      // java this.mHasProcessColumns = false;
      self.mHasProcessColumns = false;
      // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)layoutState;
      let mut pageViewLayoutState: PageViewLayoutState = (layoutState).downcast_ref::<PageViewLayoutState>().unwrap();
      // java this.mParaLayouter.end(pageViewLayoutState.getEnvoy());
      self.mParaLayouter.end(pageViewLayoutState.getEnvoy());
      // java PageViewFootEndNoteLayouter pageViewFootEndNoteLayouter = (PageViewFootEndNoteLayouter)(null != this.mFootEndNoteLayouter ? this.mFootEndNoteLayouter : (layoutState.document.getEndnoteRef().isEmpty() ? null : this.getFootEndNoteLayouter()));
      let mut pageViewFootEndNoteLayouter: PageViewFootEndNoteLayouter = ((if self.mFootEndNoteLayouter.is_some() { self.mFootEndNoteLayouter } else { (if layoutState.document.getEndnoteRef().is_empty() { false } else { self.getFootEndNoteLayouter() }) })).downcast_ref::<PageViewFootEndNoteLayouter>().unwrap();
      // java boolean hasBreakFootNotes = null != pageViewFootEndNoteLayouter && pageViewFootEndNoteLayouter.flowFootNoteBeneathText(layoutState);
      let mut hasBreakFootNotes: bool = true && pageViewFootEndNoteLayouter.flowFootNoteBeneathText(layoutState);
      // java if (layoutState.cp < this.mMainState.secEnd)
      if layoutState.cp < self.mMainState.secEnd {
        // java return true;
        return true;
      }
      // java else
      else {
        // java boolean hasBreakEndNotes = null != pageViewFootEndNoteLayouter && pageViewFootEndNoteLayouter.flowEndNotes(layoutState);
        let mut hasBreakEndNotes: bool = true && pageViewFootEndNoteLayouter.flowEndNotes(layoutState);
        // java if (!hasBreakFootNotes && !hasBreakEndNotes)
        if !hasBreakFootNotes && !hasBreakEndNotes {
          // java int breakCode = this.mMainState.secBreakCode;
          let mut breakCode: i32 = self.mMainState.secBreakCode;
          // java boolean closePage = 0 != breakCode || !this.mDocEnv.canContinueFlowNextSec;
          let mut closePage: bool = 0 != breakCode || !self.mDocEnv.canContinueFlowNextSec;
          // java this.mSectionLayouter.end(pageViewLayoutState.getEnvoy());
          self.mSectionLayouter.end(pageViewLayoutState.getEnvoy());
          // java if (0 != this.mMainState.layoutPage.getFootnotes())
          if 0 != self.mMainState.layoutPage.getFootnotes() {
            // java return true;
            return true;
          }
          // java else
          else {
            // java if (!closePage)
            if !closePage {
              // java this.mSectionLayouter.startForNoBreakPage(pageViewLayoutState.getEnvoy());
              self.mSectionLayouter.startForNoBreakPage(pageViewLayoutState.getEnvoy());
            }
            // java return closePage;
            return closePage;
          }
        }
        // java else
        else {
          // java return true;
          return true;
        }
      }
      */
    }

    // java private boolean needFlowRestFootnote(int docType)
    pub fn needFlowRestFootnote(&self, docType: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (docType == 1)
      if docType == 1 {
        // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)this.mLayoutState;
        let mut pageViewLayoutState: PageViewLayoutState = (self.mLayoutState).downcast_ref::<PageViewLayoutState>().unwrap();
        // java FootEndNoteEnv footEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
        let mut footEndNoteEnv: FootEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
        // java if (footEndNoteEnv.curFootNoteRefCP != -1)
        if footEndNoteEnv.curFootNoteRefCP != -1 {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public void flowCP(int cp, int endCp, int docType)
    pub fn flowCP(&mut self, cp: i32, endCp: i32, docType: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (cp >= 0)
      if cp >= 0 {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int oldViewTop = this.mViewEnv.getViewTop();
        let mut oldViewTop: i32 = self.mViewEnv.getViewTop();
        // java LayoutState layoutState = this.mLayoutState;
        let mut layoutState: LayoutState = { let __x: LayoutState = todo!("no align strategy matched"); __x };
        // java int lastPageMax = snapshot.getLayoutPageSize();
        let mut lastPageMax: i32 = snapshot.getLayoutPageSize();
        // java while (!this.hasFlowCP(layoutState, cp) && !this.isComplete(layoutState) || this.needFlowRestFootnote(docType))
        while !self.hasFlowCP(&layoutState, cp) && !self.isComplete(&layoutState) || self.needFlowRestFootnote(docType) {
          // java this.flowPage();
          self.flowPage();
          // java int curPageMax = snapshot.getLayoutPageSize();
          let mut curPageMax: i32 = snapshot.getLayoutPageSize();
          // java if (curPageMax > lastPageMax)
          if curPageMax > lastPageMax {
            // java int viewTop = snapshot.getPosVisitor().getContentRect().getBottom() - TypoLayoutPage.height(TypoList.quickGet(curPageMax - 1, snapshot.getLayoutPageList(), snapshot), snapshot);
            let mut viewTop: i32 = snapshot.getPosVisitor().getContentRect().getBottom() - TypoLayoutPage::height(TypoList::quickGet__int_int_typosnapshot(curPageMax - 1, snapshot.getLayoutPageList(), snapshot), snapshot);
            // java this.mViewEnv.onScroll(viewTop);
            self.mViewEnv.onScroll(viewTop);
            // java this.swapToLocal(true);
            self.swapToLocal(true);
            // java lastPageMax = curPageMax;
            lastPageMax = curPageMax;
          }
        }
        // java snapshot.setDocumentBottom(Math.max(this.getCurPageTop(), snapshot.getDocumentBottom()));
        snapshot.setDocumentBottom((self.super_.getCurPageTop()).max(snapshot.getDocumentBottom()));
        // java this.mViewEnv.onScroll(oldViewTop);
        self.mViewEnv.onScroll(oldViewTop);
        // java this.swapFromLocal();
        self.swapFromLocal();
      }
      */
    }

    // java private boolean hasFlowCP(LayoutState layoutState, int cp)
    pub fn hasFlowCP(&self, layoutState: &LayoutState, cp: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (cp >= layoutState.cp)
      if cp >= layoutState.cp {
        // java return false;
        return false;
      }
      // java else
      else {
        // java KList<TableCellLayoutState> cellStateList = layoutState.cellStateList;
        let mut cellStateList: KList = layoutState.cellStateList;
        // java if (!cellStateList.isEmpty())
        if !cellStateList.is_empty() {
          // java KList.Iterator<TableCellLayoutState> it = cellStateList.iterator();
          let mut it: Vec<TableCellLayoutState> = cellStateList.iterator();
          // java TableCellLayoutState cellLayoutState;
          let mut cellLayoutState: TableCellLayoutState = TableCellLayoutState::default();
          // java while (null != (cellLayoutState = (TableCellLayoutState)it.next()))
          while { cellLayoutState = (it.next()).downcast_ref::<TableCellLayoutState>().unwrap(); cellLayoutState }.is_some() {
            // java TableCellLayoutState firstMergerCellState = cellLayoutState.firstMergerCellState;
            let mut firstMergerCellState: TableCellLayoutState = cellLayoutState.firstMergerCellState;
            // java if (null != firstMergerCellState && cp >= firstMergerCellState.getRangeStart() && cp < firstMergerCellState.getRangeEnd())
            if true && cp >= firstMergerCellState.getRangeStart() && cp < firstMergerCellState.getRangeEnd() {
              // java it.recycle();
              it.recycle();
              // java return this.hasFlowCP(firstMergerCellState, cp);
              return self.hasFlowCP({ let __x: &LayoutState = todo!("no align strategy matched"); __x }, cp);
            }
            // java if (cp >= cellLayoutState.getRangeStart() && cp < cellLayoutState.getRangeEnd())
            if cp >= cellLayoutState.getRangeStart() && cp < cellLayoutState.getRangeEnd() {
              // java it.recycle();
              it.recycle();
              // java return this.hasFlowCP(cellLayoutState, cp);
              return self.hasFlowCP({ let __x: &LayoutState = todo!("no align strategy matched"); __x }, cp);
            }
          }
          // java it.recycle();
          it.recycle();
        }
        // java return true;
        return true;
      }
      */
    }

    // java public boolean prepareToJumpToLastView()
    pub fn prepareToJumpToLastView(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ScrollMemoryService service = ScrollMemoryService.getScrollMemoryService();
      let mut service: ScrollMemoryService = ScrollMemoryService::getScrollMemoryService();
      // java TypoDocument typoDocument = this.mDocEnv.typoDocument;
      let mut typoDocument: TypoDocument = self.mDocEnv.typoDocument;
      // java service.setTypoDocument(typoDocument);
      service.setTypoDocument(typoDocument);
      // java if (service.getVersion() == 0)
      if service.getVersion() == 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (!this.jumpForGrid(service))
        if !self.jumpForGrid(Some(&service)) {
          // java this.onScroll(UnitUtil.PT2TWIPS(service.getScrollTop()));
          self.super_.onScroll__int(UnitUtil::PT2TWIPS(service.getScrollTop()));
        }
        // java return true;
        return true;
      }
      */
    }

    // java private boolean jumpForGrid(ScrollMemoryService service)
    pub fn jumpForGrid(&mut self, service: Option<&ScrollMemoryService>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java if (snapshot == null)
      if false {
        // java return false;
        return false;
      }
      // java else
      else {
        // java IPageGrid pageGrid = snapshot.getPosVisitor().getPageGrid();
        let mut pageGrid: Box<dyn IPageGrid> = snapshot.getPosVisitor().getPageGrid();
        // java if (pageGrid != null && pageGrid.isTrendToMultiColumn())
        if true && pageGrid.isTrendToMultiColumn() {
          // java int pageIndex = service.getPageIndex();
          let mut pageIndex: i32 = service.getPageIndex();
          // java if (pageIndex <= 0)
          if pageIndex <= 0 {
            // java return false;
            return false;
          }
          // java else
          else {
            // java this.flowToPage(pageIndex);
            self.flowToPage(pageIndex);
            // java int pageSize = snapshot.getLayoutPageSize();
            let mut pageSize: i32 = snapshot.getLayoutPageSize();
            // java if (pageIndex < pageSize)
            if pageIndex < pageSize {
              // java TypoRectCoreImpl pageRect = TypoRectCoreImpl.obtain();
              let mut pageRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
              // java snapshot.getPosVisitor().rectPageForGlobal(pageRect, TypoList.quickGet(pageIndex, snapshot.getLayoutPageList(), snapshot));
              snapshot.getPosVisitor().rectPageForGlobal(pageRect, TypoList::quickGet__int_int_typosnapshot(pageIndex, snapshot.getLayoutPageList(), snapshot));
              // java this.onScroll(pageRect.top);
              self.super_.onScroll__int(pageRect.top);
              // java pageRect.recycle();
              pageRect.recycle();
              // java return true;
              return true;
            }
            // java else
            else {
              // java return false;
              return false;
            }
          }
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
      */
    }

    // java protected void flowTextFrame(LayoutState layoutState)
    pub fn flowTextFrame(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.flowTextFrame(layoutState);
      self.super_.flowTextFrame(layoutState);
      // java if (layoutState.cp >= this.mMainState.secEnd)
      if layoutState.cp >= self.mMainState.secEnd {
        // java this.mSectionLayouter.end(((PageViewLayoutState)layoutState).getEnvoy());
        self.mSectionLayouter.end((layoutState).downcast_ref::<PageViewLayoutState>().unwrap().getEnvoy());
      }
      */
    }

    // java public int flowToPage(int pageIndex)
    pub fn flowToPage(&mut self, mut pageIndex: i32) -> i32 {
      // java if (pageIndex < 1)
      if pageIndex < 1 {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot.clone();
        // rust: owned clone so &mut self (flowPage/onScroll) does not alias the Java snapshot ref
        // java int oldViewTop = this.mViewEnv.getViewTop();
        let mut oldViewTop: i32 = self.mViewEnv.getViewTop();
        // java int lastPageMax = snapshot.getLayoutPageSize();
        let mut lastPageMax: i32 = snapshot.getLayoutPageSize();
        // java int curIndex = snapshot.getLayoutPageSize() - 2;
        let mut curIndex: i32 = snapshot.getLayoutPageSize() - 2;
        // java if (curIndex < pageIndex)
        if curIndex < pageIndex {
          // java while (!this.isComplete())
          while !self.super_.mLayoutState.as_ref().map(|st| {
            let finish = self.super_.mFinishLoading;
            let err = self.super_.mFindIOError;
            if !finish && !err { return false; }
            let fn_env = FootEndNoteEnv::default();
            !st.hasContents() && st.cp >= st.getRangeEnd() && fn_env.curEndNoteRefCP == -1 && fn_env.curFootNoteRefCP == -1
          }).unwrap_or(true) {
            // java this.flowPage();
            self.flowPage();
            snapshot = self.mMainState.snapshot.clone();
            // java int curPageMax = snapshot.getLayoutPageSize();
            let mut curPageMax: i32 = snapshot.getLayoutPageSize();
            // java if (curPageMax > lastPageMax)
            if curPageMax > lastPageMax {
              // java int viewTop = snapshot.getPosVisitor().getContentRect().getBottom() - TypoLayoutPage.height(TypoList.quickGet(curPageMax - 1, snapshot.getLayoutPageList(), snapshot), snapshot);
              let mut viewTop: i32 = snapshot.getPosVisitor().getContentRect().getBottom() - TypoLayoutPage::height(TypoList::quickGet__int_int_typosnapshot(curPageMax - 1, snapshot.getLayoutPageList(), &mut snapshot), &mut snapshot);
              // java this.mViewEnv.onScroll(viewTop);
              self.mViewEnv.onScroll(viewTop);
              // java this.swapToLocal(true);
              self.swapToLocal(true);
              // java lastPageMax = curIndex;
              lastPageMax = curIndex;
            }
            // java curIndex = snapshot.getLayoutPageSize() - 2;
            curIndex = snapshot.getLayoutPageSize() - 2;
            // java if (curIndex == pageIndex)
            if curIndex == pageIndex {
              // java break;
              break;
            }
          }
        }
        // java snapshot.setDocumentBottom(Math.max(this.getCurPageTop(), snapshot.getDocumentBottom()));
        snapshot.setDocumentBottom((self.super_.getCurPageTop()).max(snapshot.getDocumentBottom()));
        self.mMainState.snapshot.setDocumentBottom((self.super_.getCurPageTop()).max(self.mMainState.snapshot.getDocumentBottom()));
        // java this.mViewEnv.onScroll(oldViewTop);
        self.mViewEnv.onScroll(oldViewTop);
        // java this.swapFromLocal();
        self.swapFromLocal();
        // java pageIndex = Math.min(pageIndex, snapshot.getLayoutPageSize() - 1);
        pageIndex = (pageIndex).min(snapshot.getLayoutPageSize() - 1);
        // java if (pageIndex < 1)
        if pageIndex < 1 {
          // java return -1;
          return -1;
        }
        // java else
        else {
          // java int page = TypoList.get(pageIndex, snapshot.getLayoutPageList(), snapshot);
          let mut page: i32 = TypoList::get__int_int_typosnapshot(pageIndex, snapshot.getLayoutPageList(), &mut snapshot);
          // java return 0 != page ? TypoLayoutPage.getStartCP(page, snapshot) : -1;
          return (if 0 != page { TypoLayoutPage::getStartCP(page, &snapshot) } else { -1 });
        }
      }
    }

    // java protected void rollbackLine(int textLine, LayoutState layoutState)
    pub fn rollbackLine(&mut self, textLine: i32, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.rollbackLine(textLine, layoutState);
      self.super_.rollbackLine(textLine, layoutState);
      // java this.restartSectionLayouter(layoutState);
      self.restartSectionLayouter(layoutState);
      */
    }

    // java private void restartSectionLayouter(LayoutState layoutState)
    pub fn restartSectionLayouter(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (layoutState.cp < this.mMainState.secStart || !layoutState.curPage.hasLines())
      if layoutState.cp < self.mMainState.secStart || !layoutState.curPage.hasLines() {
        // java PageViewLayoutStateEnvoy envoy = ((PageViewLayoutState)layoutState).getEnvoy();
        let mut envoy: PageViewLayoutStateEnvoy = (layoutState).downcast_ref::<PageViewLayoutState>().unwrap().getEnvoy();
        // java this.mSectionLayouter.end(envoy);
        self.mSectionLayouter.end(envoy);
        // java this.mSectionLayouter.start((SectionLayouter.IStateEnvoy)envoy);
        self.mSectionLayouter.start((envoy).downcast_ref::<SectionLayouter::IStateEnvoy>().unwrap());
      }
      */
    }

    // java protected void rollbackLines(LayoutState layoutState)
    pub fn rollbackLines(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.rollbackLines(layoutState);
      self.super_.rollbackLines(layoutState);
      // java this.restartSectionLayouter(layoutState);
      self.restartSectionLayouter(layoutState);
      // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)layoutState;
      let mut pageViewLayoutState: PageViewLayoutState = (layoutState).downcast_ref::<PageViewLayoutState>().unwrap();
      // java FootEndNoteEnv footEndNote = pageViewLayoutState.footEndNoteEnv;
      let mut footEndNote: FootEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
      // java if (!layoutState.curPage.hasLines() && (footEndNote.curEndNoteRefCP != -1 || footEndNote.curFootNoteRefCP != -1))
      if !layoutState.curPage.hasLines() && footEndNote.curEndNoteRefCP != -1 || footEndNote.curFootNoteRefCP != -1 {
        // java if (this.mMainState.columns > 1 && (layoutState.cp < layoutState.document.getLength() || !this.mFinishLoading))
        if self.mMainState.columns > 1 && layoutState.cp < layoutState.document.getLength() || !self.mFinishLoading {
          // java return;
          return;
        }
        // java TypoLayoutPage layoutPage = this.mMainState.layoutPage;
        let mut layoutPage: TypoLayoutPage = self.mMainState.layoutPage;
        // java if (0 != layoutPage.getFootnotes())
        if 0 != layoutPage.getFootnotes() {
          // java layoutPage.setFootnotes(0);
          layoutPage.setFootnotes(0);
        }
        // java PageViewFootEndNoteLayouter pageViewFootEndNoteLayouter = (PageViewFootEndNoteLayouter)this.mFootEndNoteLayouter;
        let mut pageViewFootEndNoteLayouter: PageViewFootEndNoteLayouter = (self.mFootEndNoteLayouter).downcast_ref::<PageViewFootEndNoteLayouter>().unwrap();
        // java if (footEndNote.curFootNoteRefCP != -1)
        if footEndNote.curFootNoteRefCP != -1 {
          // java pageViewFootEndNoteLayouter.flowBreakFootNotes(pageViewLayoutState);
          pageViewFootEndNoteLayouter.flowBreakFootNotes(pageViewLayoutState);
        }
        // java if (footEndNote.curEndNoteRefCP != -1)
        if footEndNote.curEndNoteRefCP != -1 {
          // java pageViewFootEndNoteLayouter.flowEndNotes(pageViewLayoutState);
          pageViewFootEndNoteLayouter.flowEndNotes(pageViewLayoutState);
          // java PageViewLayoutStateEnvoy envoy = pageViewLayoutState.getEnvoy();
          let mut envoy: PageViewLayoutStateEnvoy = pageViewLayoutState.getEnvoy();
          // java this.mSectionLayouter.end(envoy);
          self.mSectionLayouter.end(envoy);
          // java this.mSectionLayouter.startForNoBreakPage(envoy);
          self.mSectionLayouter.startForNoBreakPage(envoy);
        }
      }
      */
    }

    // java protected void onNewRangeEnd(int lastRangeEnd, int rangeEnd)
    pub fn onNewRangeEnd(&mut self, lastRangeEnd: i32, rangeEnd: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.hasFinishIoLoading())
      if !self.super_.hasFinishIoLoading() {
        // java this.mMainState.secEnd = rangeEnd;
        self.mMainState.secEnd = rangeEnd;
      }
      */
    }

    // java public boolean isComplete(LayoutState layoutState)
    pub fn isComplete(&mut self, layoutState: &LayoutState) -> bool {
      // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)layoutState;
      let pageViewLayoutState = layoutState;
      // java if (!this.mFinishLoading)
      if !self.super_.mFinishLoading {
        // java if (!this.mFindIOError)
        if !self.super_.mFindIOError {
          // java return false;
          return false;
        }
        // java else
        else {
          // java FootEndNoteEnv footEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
          let footEndNoteEnv: FootEndNoteEnv = FootEndNoteEnv::default();
          // java boolean result = !pageViewLayoutState.hasContents() && pageViewLayoutState.cp >= pageViewLayoutState.getRangeEnd() && footEndNoteEnv.curEndNoteRefCP == -1 && footEndNoteEnv.curFootNoteRefCP == -1;
          let result: bool = !pageViewLayoutState.hasContents() && pageViewLayoutState.cp >= pageViewLayoutState.getRangeEnd() && footEndNoteEnv.curEndNoteRefCP == -1 && footEndNoteEnv.curFootNoteRefCP == -1;
          // java return result;
          return result;
        }
      }
      // java else
      else {
        // java FootEndNoteEnv footEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
        let footEndNoteEnv: FootEndNoteEnv = FootEndNoteEnv::default();
        // java boolean result = !pageViewLayoutState.hasContents() && pageViewLayoutState.cp >= pageViewLayoutState.getRangeEnd() && footEndNoteEnv.curEndNoteRefCP == -1 && footEndNoteEnv.curFootNoteRefCP == -1;
        let result: bool = !pageViewLayoutState.hasContents() && pageViewLayoutState.cp >= pageViewLayoutState.getRangeEnd() && footEndNoteEnv.curEndNoteRefCP == -1 && footEndNoteEnv.curFootNoteRefCP == -1;
        // java this.mViewEnv.setHasFlowToEnd(result);
        self.super_.super_.mViewEnv.setHasFlowToEnd(result);
        // java return result;
        return result;
      }
    }

    // java protected void doAfterFlowLine(LayoutState layoutState)
    pub fn doAfterFlowLine(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mFinishLoading && layoutState.cp >= layoutState.getRangeEnd())
      if !self.mFinishLoading && layoutState.cp >= layoutState.getRangeEnd() {
        // java this.initGraphBuffForLoading();
        self.super_.initGraphBuffForLoading();
      }
      // java super.doAfterFlowLine(layoutState);
      self.super_.doAfterFlowLine(layoutState);
      // java if (layoutState.curPage.hasLines())
      if layoutState.curPage.hasLines() {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int lastLine = TypoList.last(layoutState.curPage.getLines(), snapshot);
        let mut lastLine: i32 = TypoList::last__int_typosnapshot(layoutState.curPage.getLines(), snapshot);
        // java if (13 == TypoObjBase.getType(lastLine, snapshot))
        if 13 == TypoObjBase::getType(lastLine, snapshot) {
          // java return;
          return;
        }
      }
      // java if (this.hasFinishIoLoading() && layoutState.cp >= layoutState.getRangeEnd() && layoutState.document.getTextRope().charAt(layoutState.cp - 1) != '\f')
      if self.super_.hasFinishIoLoading() && layoutState.cp >= layoutState.getRangeEnd() && layoutState.document.getTextRope().charAt(layoutState.cp - 1) != '\u{000c}' {
        // java PageViewFootEndNoteLayouter pageViewFootEndNoteLayouter = (PageViewFootEndNoteLayouter)(null != this.mFootEndNoteLayouter ? this.mFootEndNoteLayouter : (layoutState.document.getEndnoteRef().isEmpty() ? null : this.getFootEndNoteLayouter()));
        let mut pageViewFootEndNoteLayouter: PageViewFootEndNoteLayouter = ((if self.mFootEndNoteLayouter.is_some() { self.mFootEndNoteLayouter } else { (if layoutState.document.getEndnoteRef().is_empty() { None } else { self.getFootEndNoteLayouter() }) })).downcast_ref::<PageViewFootEndNoteLayouter>().unwrap();
        // java if (null != pageViewFootEndNoteLayouter)
        if true {
          // java pageViewFootEndNoteLayouter.flowFootNoteBeneathText(layoutState);
          pageViewFootEndNoteLayouter.flowFootNoteBeneathText(layoutState);
          // java pageViewFootEndNoteLayouter.flowEndNotes(layoutState);
          pageViewFootEndNoteLayouter.flowEndNotes(layoutState);
        }
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
      */
    }

    // java private FootEndNoteLayouter getFootEndNoteLayouter()
    pub fn getFootEndNoteLayouter(&self) -> Option<&FootEndNoteLayouter> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.initFootEndNoteLayouter();
      self.initFootEndNoteLayouter();
      // java return this.mFootEndNoteLayouter;
      return self.mFootEndNoteLayouter.as_ref();
      */
    }

    // java public boolean prepareFlowFromDocumentBegin()
    pub fn prepareFlowFromDocumentBegin(&mut self) -> bool {
      // java this.mViewEnv.setHasFlowToBegin(true);
      self.super_.super_.mViewEnv.setHasFlowToBegin(true);
      // java this.onModifyFirst(0, true);
      self.onModifyFirst(0, true);
      // java return true;
      true
    }

    // Rust: re-seat Java field aliases after PageViewLayouter moves.
    pub fn rebind_java_aliases(&mut self) {
      let main = &mut self.super_.super_.mMainState as *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
      let doc = &mut self.super_.super_.mDocEnv as *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
      let view = &mut self.super_.super_.mViewEnv as *mut crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
      if let Some(section) = self.mSectionLayouter.as_mut() {
        section.rebind(main, doc, view);
      }
      if let Some(page) = self.mPageViewPageLayouter.as_mut() {
        page.rebind(main, doc, view);
      }
      let page_view = self.mPageViewPageLayouter.as_mut().map(|p| {
        p.as_mut() as *mut crate::cn::wps::moffice::writer::layout::pageview::PageViewPageLayouter::PageViewPageLayouter as *mut ()
      }).unwrap_or(std::ptr::null_mut());
      let owner = self as *mut PageViewLayouter as *mut ();
      if let Some(page_layouter) = self.super_.super_.mPageLayouter.as_mut() {
        page_layouter.bind_page_view(page_view, page_view_on_start, page_view_on_end);
        page_layouter.bind_page_view_flow(owner, page_view_pre_prefix, page_view_pre_suffix, page_view_finish);
        page_layouter.bind_page_view_init_drawing(page_view_init_drawing);
      }
      self.super_.super_.bind_graph_factory();
    }

    // java FlowLayout.onSetTextDocument plus DocumentEnv.update for an already-open document.
    pub fn bind_open_document(&mut self, doc: &crate::cn::wps::moffice::writer::core::TextDocument::TextDocument) {
      self.rebind_java_aliases();
      self.super_.super_.mDocEnv.updateFromDocument(doc);
      self.super_.super_.mDocEnv.propToolFactory = Some(
        crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory::default(),
      );
      if let Some(gf) = self.super_.super_.mGraphBuff.as_mut() {
        if let Some(env) = gf.super_.super_.mDocEnv.as_mut() {
          env.updateFromDocument(doc);
          env.propToolFactory = Some(
            crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory::default(),
          );
        }
      }
      if let Some(para) = self.super_.super_.mParaLayouter.as_mut() {
        para.mDocEnv.updateFromDocument(doc);
        para.mDocEnv.propToolFactory = Some(
          crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory::default(),
        );
      }
      self.super_.super_.rebase_line_env();
      if let Some(line) = self.super_.super_.mLineLayouter.as_mut() {
        if let Some(env) = line.super_mut().super_mut().super_mut().mDocEnv.as_mut() {
          env.updateFromDocument(doc);
          env.propToolFactory = Some(
            crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory::default(),
          );
        }
      }
      let doc_len = doc.getMainDocument().map(|main| main.getLength()).unwrap_or(0);
      let main_ptr = doc.getMainDocument().map(|main| {
        main as *const dyn crate::cn::wps::moffice::writer::core::Document::Document
          as *mut dyn crate::cn::wps::moffice::writer::core::Document::Document
      });
      if let Some(st) = self.super_.mLayoutState.as_mut() {
        st.document = main_ptr;
        st.setRange__int_int(0, doc_len);
        st.pageEndLimitCP = doc_len;
        st.cp = 0;
        st.pagePos = 0;
        st.paraEnv.isFirstLineOfPara = true;
        let list = &mut st.graphList as *mut _;
        st.mGraphsIt.attach(list);
      }
      // java MainLayouter.calcMaxValidRangeEnd sets mFinishLoading when IO is done.
      if !doc.isInIOState() {
        self.super_.mFinishLoading = true;
      }
      self.rebind_java_aliases();
    }

    pub fn layout_cp(&self) -> i32 {
      self.super_.mLayoutState.as_ref().map(|st| st.cp).unwrap_or(0)
    }

    // java PageViewLayouter.isComplete for the product page loop.
    pub fn document_flow_complete(&mut self) -> bool {
      let (cp, end, has) = match self.super_.mLayoutState.as_ref() {
        None => return true,
        Some(st) => (st.cp, st.getRangeEnd(), st.hasContents()),
      };
      if !self.super_.mFinishLoading {
        if !self.super_.mFindIOError {
          return false;
        }
      }
      let result = !has && cp >= end;
      if self.super_.mFinishLoading {
        self.super_.super_.mViewEnv.setHasFlowToEnd(result);
      }
      result
    }

    pub fn flowed_lines(&self) -> (
      Vec<crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine>,
      Vec<Vec<(i32, i32)>>,
      Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32)>,
      Vec<Option<crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame>>,
    ) {
      let Some(st) = self.super_.mLayoutState.as_ref() else {
        return (Vec::new(), Vec::new(), Vec::new(), Vec::new());
      };
      let mut lines: Vec<crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine> = st.curPage.as_ref().map(|page| {
        (0..page.lines_len()).filter_map(|idx| page.line_at(idx).cloned()).collect()
      }).unwrap_or_default();
      let mut cells = vec![None; lines.len()];
      let page_line_count = lines.len();
      self.append_snapshot_tables(st, &mut lines, &mut cells);
      let mut advances = st.committed_line_advances.clone();
      advances.resize_with(lines.len(), Vec::new);
      for (index, line) in lines.iter().enumerate().skip(page_line_count) {
        if let Some((_, _, widths)) = st.table_line_advances.iter().find(|(start, end, _)|
            *start == line.getStartCP() && *end == line.getEndCP()) {
          advances[index] = widths.clone();
        }
      }
      (lines, advances, st.embed_places.clone(), cells)
    }

    // Java PageRender.drawLines -> TableRender.drawTableRow reads cells from
    // TypoSnapshot. Preserve that plane for the platform-neutral renderer.
    pub fn snapshot_table_cells(&self) -> Vec<crate::cn::wps::moffice::writer::layout::base::env::PageLayout::TableCellPaint> {
      let mut out = Vec::new();
      let Some(st) = self.super_.mLayoutState.as_ref() else { return out; };
      let Some(page) = st.curPage.as_ref() else { return out; };
      let page_ptr = page.getPointer();
      if page_ptr == 0 { return out; }
      let snapshot = &self.super_.super_.mMainState.snapshot;
      let lines = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLines__int_typosnapshot(page_ptr, snapshot);
      self.snapshot_table_rows(snapshot, lines, 0, 0, &mut out);
      let wraps = self.super_.super_.mMainState.drawingLayers.getWraps();
      self.snapshot_wrap_tables(snapshot, wraps, page_ptr, 0, 0, &mut out);
      out
    }

    fn snapshot_wrap_tables(
      &self,
      snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
      wraps: i32,
      parent: i32,
      origin_x: i32,
      origin_y: i32,
      out: &mut Vec<crate::cn::wps::moffice::writer::layout::base::env::PageLayout::TableCellPaint>,
    ) {
      use crate::cn::wps::moffice::writer::cache::{TypoDrawing::TypoDrawing, TypoList::TypoList, TypoObjBase::TypoObjBase, TypoPage::TypoPage};
      if wraps == 0 || parent == 0 { return; }
      for i in 0..TypoList::size__int_typosnapshot(wraps, snapshot) {
        let drawing = snapshot.get__int(wraps + 3 + i);
        if TypoDrawing::getDrawingType__int_typosnapshot(drawing, snapshot) != 7
          || TypoDrawing::getParent__int_typosnapshot(drawing, snapshot) != parent { continue; }
        let textbox = TypoDrawing::getTextbox__int_typosnapshot(drawing, snapshot);
        if textbox == 0 { continue; }
        let x = origin_x + TypoObjBase::getLeft__int_typosnapshot(drawing, snapshot);
        let y = origin_y + TypoObjBase::getTop__int_typosnapshot(drawing, snapshot);
        let lines = TypoPage::getLines__int_typosnapshot(textbox, snapshot);
        self.snapshot_table_rows(snapshot, lines, x, y, out);
      }
    }

    fn snapshot_table_rows(
      &self,
      snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
      list: i32,
      origin_x: i32,
      origin_y: i32,
      out: &mut Vec<crate::cn::wps::moffice::writer::layout::base::env::PageLayout::TableCellPaint>,
    ) {
      use crate::cn::wps::moffice::writer::cache::{TypoList::TypoList, TypoObjBase::TypoObjBase, TypoPage::TypoPage, TypoRectCoreImpl::TypoRectCoreImpl, TypoTableCell::TypoTableCell, TypoTableRow::TypoTableRow};
      use crate::cn::wps::moffice::writer::layout::base::env::PageLayout::TableCellPaint;
      use crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase;
      if list == 0 { return; }
      let wraps = self.super_.super_.mMainState.drawingLayers.getWraps();
      for i in 0..TypoList::size__int_typosnapshot(list, snapshot) {
        let row_ptr = snapshot.get__int(list + 3 + i);
        if snapshot.get__int(row_ptr) != 5 || TypoTableRow::isRevisionDelete__int_typosnapshot(row_ptr, snapshot) { continue; }
        let mut row = TypoTableRow::new();
        row.assignment(row_ptr, snapshot as *const _ as *mut _);
        let rtl = TypoTableRow::isRTL__int_typosnapshot(row_ptr, snapshot);
        let row_start_cp = TypoRangeBase::getStartCP__int_typosnapshot(row_ptr, snapshot);
        let row_bottom = origin_y + TypoObjBase::getBottom__int_typosnapshot(row_ptr, snapshot);
        for index in 0..row.getCellsCount() {
          let cell_ptr = row.getCell(index);
          if cell_ptr == 0 { continue; }
          let mut cell = TypoTableCell::new();
          cell.assignment(cell_ptr, snapshot as *const _ as *mut _);
          let mut rect = TypoRectCoreImpl::default();
          cell.rectForPageCore(&mut rect);
          let r = &rect.super_;
          let cell_x = origin_x + r.left;
          let cell_y = origin_y + r.top;
          let clip = if cell.isVertMergeCell() && !cell.isLastVertMergeCell() { row_bottom } else { i32::MAX };
          out.push(TableCellPaint {
            row_start_cp,
            rect: (cell_x, cell_y, origin_x + r.right, origin_y + r.bottom),
            brc_left: snapshot.get_brc(if rtl { cell.getBrcRight() } else { cell.getBrcLeft() }),
            brc_top: snapshot.get_brc(cell.getBrcTop()),
            brc_right: snapshot.get_brc(if rtl { cell.getBrcLeft() } else { cell.getBrcRight() }),
            brc_bottom: snapshot.get_brc(cell.getBrcBottom()),
            is_rtl_row: rtl,
            max_vert_brc_bottom: clip,
          });
          let cell_lines = TypoPage::getLines__int_typosnapshot(cell_ptr, snapshot);
          self.snapshot_table_rows(snapshot, cell_lines, cell_x, cell_y, out);
          self.snapshot_wrap_tables(snapshot, wraps, cell_ptr, cell_x, cell_y, out);
        }
      }
    }

    // Java PageRender.drawLines: type 3 text, type 5 TableRender.drawTableRow → cell → drawPage.
    // Wrap tables live in a type-7 drawing textbox, not in the page mirror Vec.
    fn append_snapshot_tables(
      &self,
      st: &crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState,
      lines: &mut Vec<crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine>,
      cells: &mut Vec<Option<crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame>>,
    ) {
      let Some(page) = st.curPage.as_ref() else { return; };
      let page_ptr = page.getPointer();
      if page_ptr == 0 { return; }
      let snapshot = &self.super_.super_.mMainState.snapshot;
      let doc = st.document;
      let page_lines = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLines__int_typosnapshot(page_ptr, snapshot);
      self.walk_table_rows(snapshot, doc, page_lines, 0, 0, lines, cells);
      let wraps = self.super_.super_.mMainState.drawingLayers.getWraps();
      self.walk_wrap_drawings(snapshot, doc, wraps, page_ptr, 0, 0, lines, cells);
    }

    fn walk_wrap_drawings(
      &self,
      snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
      doc: Option<*mut dyn crate::cn::wps::moffice::writer::core::Document::Document>,
      wraps: i32,
      parent: i32,
      origin_x: i32,
      origin_y: i32,
      lines: &mut Vec<crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine>,
      cells: &mut Vec<Option<crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame>>,
    ) {
      if wraps == 0 || parent == 0 { return; }
      let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(wraps, snapshot);
      for i in 0..size {
        let drawing = snapshot.get__int(wraps + 3 + i);
        if crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getDrawingType__int_typosnapshot(drawing, snapshot) != 7 {
          continue;
        }
        if crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getParent__int_typosnapshot(drawing, snapshot) != parent {
          continue;
        }
        let textbox = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getTextbox__int_typosnapshot(drawing, snapshot);
        if textbox == 0 { continue; }
        let dx = origin_x + crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(drawing, snapshot);
        let dy = origin_y + crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(drawing, snapshot);
        let text_lines = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLines__int_typosnapshot(textbox, snapshot);
        self.walk_table_rows(snapshot, doc, text_lines, dx, dy, lines, cells);
      }
    }

    fn walk_table_rows(
      &self,
      snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
      doc: Option<*mut dyn crate::cn::wps::moffice::writer::core::Document::Document>,
      list: i32,
      origin_x: i32,
      origin_y: i32,
      lines: &mut Vec<crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine>,
      cells: &mut Vec<Option<crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame>>,
    ) {
      if list == 0 { return; }
      let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(list, snapshot);
      for i in 0..size {
        let row = snapshot.get__int(list + 3 + i);
        if snapshot.get__int(row) != 5 { continue; }
        let row_top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(row, snapshot);
        let count = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCellsCount__int_typosnapshot(row, snapshot);
        for c in 0..count {
          let cell = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCell__int_int_typosnapshot(c, row, snapshot);
          if cell == 0 { continue; }
          let cell_top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(cell, snapshot);
          let cell_lines = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLines__int_typosnapshot(cell, snapshot);
          let y = origin_y.wrapping_add(row_top).wrapping_add(cell_top);
          self.emit_cell_text(snapshot, doc, cell_lines, origin_x, y, lines, cells);
          self.walk_table_rows(snapshot, doc, cell_lines, origin_x, y, lines, cells);
          let wraps = self.super_.super_.mMainState.drawingLayers.getWraps();
          self.walk_wrap_drawings(snapshot, doc, wraps, cell, origin_x, y, lines, cells);
        }
      }
    }

    fn emit_cell_text(
      &self,
      snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
      doc: Option<*mut dyn crate::cn::wps::moffice::writer::core::Document::Document>,
      list: i32,
      origin_x: i32,
      origin_y: i32,
      lines: &mut Vec<crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine>,
      cells: &mut Vec<Option<crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame>>,
    ) {
      if list == 0 { return; }
      let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(list, snapshot);
      for i in 0..size {
        let pointer = snapshot.get__int(list + 3 + i);
        if snapshot.get__int(pointer) != 3 { continue; }
        let start_cp = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getStartCP__int_typosnapshot(pointer, snapshot);
        let end_cp = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getEndCP__int_typosnapshot(pointer, snapshot);
        if start_cp < 0 || end_cp <= start_cp { continue; }
        let left = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(pointer, snapshot);
        let top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(pointer, snapshot);
        let right = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getRight__int_typosnapshot(pointer, snapshot);
        let bottom = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(pointer, snapshot);
        let mut line = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::new();
        line.start_cp = start_cp;
        line.end_cp = end_cp;
        line.left = left;
        line.width = right.wrapping_sub(left);
        line.top = origin_y.wrapping_add(top);
        line.height = bottom.wrapping_sub(top).max(0);
        line.ascentPos = snapshot.get__int(pointer + 10);
        lines.push(line);
        cells.push(self.cell_frame(doc, start_cp, origin_x));
      }
    }

    fn cell_frame(
      &self,
      doc: Option<*mut dyn crate::cn::wps::moffice::writer::core::Document::Document>,
      cp: i32,
      origin_x: i32,
    ) -> Option<crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame> {
      use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
      use crate::cn::wps::moffice::writer::layout::base::env::WidowControl::CellFrame;
      let doc = doc?;
      let infos = unsafe { (*doc).getTableInfos() }?;
      let mut found = None;
      for level in 1..12 {
        if let Some(mut node) = unsafe { (*infos).getTableInfoByLevel(cp, level) } {
          if let Some(box_) = node.cell_layout_box(cp) {
            found = Some(CellFrame {
              text_dx: box_.text_dx.wrapping_add(origin_x),
              border_left: box_.border_left.wrapping_add(origin_x),
              border_right: box_.border_right.wrapping_add(origin_x),
              content_width: box_.content_width,
              row_start: box_.row_start,
              row_height: box_.row_height,
              first_row: box_.first_row,
              margin_top: box_.margin_top,
              margin_bottom: box_.margin_bottom,
              top_w: box_.top_w,
              left_w: box_.left_w,
              bottom_w: box_.bottom_w,
              right_w: box_.right_w,
              color: box_.color,
            });
          }
        }
      }
      found
    }

    // java public boolean prepareFlowInScrollDirection()
    pub fn prepareFlowInScrollDirection(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasFinishIoLoading())
      if self.super_.hasFinishIoLoading() {
        // java this.initGraphBuffForLoading();
        self.super_.initGraphBuffForLoading();
      }
      // java else
      else {
        // java this.mLayoutState.setRangeEnd(this.mLayoutState.document.getLength());
        self.mLayoutState.setRangeEnd(self.mLayoutState.document.getLength());
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int curPagePos = snapshot.getPosVisitor().getPageIndexFromPoint(0, this.mViewEnv.getViewTop(), true);
      let mut curPagePos: i32 = snapshot.getPosVisitor().getPageIndexFromPoint(0, self.mViewEnv.getViewTop(), true);
      // java int afterPageCount = snapshot.getLayoutPageSize() - curPagePos - 1;
      let mut afterPageCount: i32 = snapshot.getLayoutPageSize() - curPagePos - 1;
      // java if (afterPageCount > 64)
      if afterPageCount > 64 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return !this.isComplete(this.mLayoutState);
        return !self.isComplete({ let __x: &LayoutState = todo!("no align strategy matched"); __x });
      }
      */
    }

    // java public boolean prepareFlowToDocumentEnd()
    pub fn prepareFlowToDocumentEnd(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasFinishIoLoading())
      if self.super_.hasFinishIoLoading() {
        // java this.initGraphBuffForLoading();
        self.super_.initGraphBuffForLoading();
      }
      // java else
      else {
        // java this.mLayoutState.setRangeEnd(this.mLayoutState.document.getLength());
        self.mLayoutState.setRangeEnd(self.mLayoutState.document.getLength());
      }
      // java return !this.isComplete(this.mLayoutState);
      return !self.isComplete({ let __x: &LayoutState = todo!("no align strategy matched"); __x });
      */
    }

    // java public boolean acceptFirstCache(TypoSnapshot snapshot)
    pub fn acceptFirstCache(&mut self, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pFirsPage = TypoList.first(snapshot.getLayoutPageList(), snapshot);
      let mut pFirsPage: i32 = TypoList::first__int_typosnapshot(snapshot.getLayoutPageList(), snapshot);
      // java if (pFirsPage == 0)
      if pFirsPage == 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)this.mLayoutState;
        let mut pageViewLayoutState: PageViewLayoutState = (self.mLayoutState).downcast_ref::<PageViewLayoutState>().unwrap();
        // java this.mMainState.curPageIndex = 1;
        self.mMainState.curPageIndex = 1;
        // java this.mMainState.curSecPageIndex = 1;
        self.mMainState.curSecPageIndex = 1;
        // java Pools pools = this.mDocEnv.pools;
        let mut pools: Pools = self.mDocEnv.pools;
        // java StateSnapshot stateSnapshot = pools.obtainStateSnapshot();
        let mut stateSnapshot: StateSnapshot = pools.obtainStateSnapshot();
        // java stateSnapshot.loadFromTypoMem(TypoLayoutPage.getEndState(pFirsPage, snapshot), snapshot, pools);
        stateSnapshot.loadFromTypoMem(TypoLayoutPage::getEndState__int_typosnapshot(pFirsPage, snapshot), snapshot, pools);
        // java pageViewLayoutState.getEnvoy().restoreFromSnapShot(stateSnapshot, this.mDocEnv);
        pageViewLayoutState.getEnvoy().restoreFromSnapShot(stateSnapshot, self.mDocEnv);
        // java pageViewLayoutState.setRangeEnd(pageViewLayoutState.document.getLength());
        pageViewLayoutState.setRangeEnd(pageViewLayoutState.document.getLength());
        // java pools.recycle(stateSnapshot);
        pools.recycle(stateSnapshot);
        // java this.mMainState.curPageTop = TypoLayoutPage.getBottom(pFirsPage, snapshot);
        self.mMainState.curPageTop = TypoLayoutPage::getBottom(pFirsPage, snapshot);
        // java pageViewLayoutState.pagePos = 1;
        pageViewLayoutState.pagePos = 1;
        // java this.mViewEnv.setHasFlowToBegin(true);
        self.mViewEnv.setHasFlowToBegin(true);
        // java if (!this.mFinishLoading && !this.mDocEnv.textDocument.isInIOState())
        if !self.mFinishLoading && !self.mDocEnv.textDocument.isInIOState() {
          // java this.mFinishLoading = true;
          self.mFinishLoading = true;
        }
        // java return true;
        return true;
      }
      */
    }

    // java public void onModifyFirst(int pageIndex, boolean recycleAfter)
    pub fn onModifyFirst(&mut self, pageIndex: i32, recycleAfter: bool) {
      let _ = recycleAfter;
      // java if (pageIndex >= 0)
      if pageIndex >= 0 {
        // java this.mDocEnv.update();
        self.super_.super_.mDocEnv.update();
        // java this.mSectionLayouter.reset();
        if let Some(section) = self.mSectionLayouter.as_mut() {
          section.reset();
        }
        // java this.mMainState.curPageIndex = pageIndex;
        self.super_.super_.mMainState.curPageIndex = pageIndex;
        // java pageIndex <= 0 resets the page-view state to the document start.
        if pageIndex <= 0 {
          if let Some(st) = self.super_.mLayoutState.as_mut() {
            st.pagePos = 0;
            st.cp = 0;
            st.curFootNoteRefCP = -1;
            st.curEndNoteRefCP = -1;
          }
          self.super_.super_.mMainState.curPageTop = 0;
          self.super_.super_.mMainState.curSecPageIndex = 0;
        }
        // java this.mHasProcessEnsureValidEndPage = false;
        self.mHasProcessEnsureValidEndPage = false;
      }
      /* DEFERRED_NON_FUNCTION
      // java if (pageIndex >= 0)
      if pageIndex >= 0 {
        // java if (null == this.mMainState.snapshot)
        if self.mMainState.snapshot.is_none() {
          // java this.mMainState.snapshot = this.mDocEnv.typoDocument.newSnapshot();
          self.mMainState.snapshot = self.mDocEnv.typoDocument.newSnapshot();
        }
        // java this.mDocEnv.update();
        self.mDocEnv.update();
        // java this.mSectionLayouter.reset();
        self.mSectionLayouter.reset();
        // java this.mMainState.curPageIndex = pageIndex;
        self.mMainState.curPageIndex = pageIndex;
        // java this.mMainState.curSecPageIndex = this.mMainState.snapshot.getSectionPageIndex(pageIndex);
        self.mMainState.curSecPageIndex = self.mMainState.snapshot.getSectionPageIndex(pageIndex);
        // java PageViewLayoutState pageViewLayoutState = (PageViewLayoutState)this.mLayoutState;
        let mut pageViewLayoutState: PageViewLayoutState = (self.mLayoutState).downcast_ref::<PageViewLayoutState>().unwrap();
        // java if (pageIndex <= 0)
        if pageIndex <= 0 {
          // java pageViewLayoutState.pagePos = 0;
          pageViewLayoutState.pagePos = 0;
          // java pageViewLayoutState.cp = 0;
          pageViewLayoutState.cp = 0;
          // java this.mDocEnv.pools.recycleTableCellLayoutStates(pageViewLayoutState.cellStateList);
          self.mDocEnv.pools.recycleTableCellLayoutStates(pageViewLayoutState.cellStateList);
          // java FootEndNoteEnv footEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
          let mut footEndNoteEnv: FootEndNoteEnv = pageViewLayoutState.footEndNoteEnv;
          // java footEndNoteEnv.hasInitSecProp = false;
          footEndNoteEnv.hasInitSecProp = false;
          // java footEndNoteEnv.curEndNoteRefCP = -1;
          footEndNoteEnv.curEndNoteRefCP = -1;
          // java footEndNoteEnv.curFootNoteRefCP = -1;
          footEndNoteEnv.curFootNoteRefCP = -1;
          // java footEndNoteEnv.footNoteRefCPs.resetQuick();
          footEndNoteEnv.footNoteRefCPs.resetQuick();
          // java this.mMainState.curPageTop = 0;
          self.mMainState.curPageTop = 0;
        }
        // java else if (pageIndex < this.mMainState.snapshot.getLayoutPageSize())
        else if pageIndex < self.mMainState.snapshot.getLayoutPageSize() {
          // java TypoSnapshot snapshot = this.mMainState.snapshot;
          let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
          // java pageViewLayoutState.pagePos = pageIndex;
          pageViewLayoutState.pagePos = pageIndex;
          // java int layoutPage = TypoList.get(pageIndex, snapshot.getLayoutPageList(), snapshot);
          let mut layoutPage: i32 = TypoList::get__int_int_typosnapshot(pageIndex, snapshot.getLayoutPageList(), snapshot);
          // java Pools pools = this.mDocEnv.pools;
          let mut pools: Pools = self.mDocEnv.pools;
          // java StateSnapshot stateSnapshot = pools.obtainStateSnapshot();
          let mut stateSnapshot: StateSnapshot = pools.obtainStateSnapshot();
          // java stateSnapshot.loadFromTypoMem(TypoLayoutPage.getStartState(layoutPage, snapshot), snapshot, pools);
          stateSnapshot.loadFromTypoMem(TypoLayoutPage::getStartState__int_typosnapshot(layoutPage, snapshot), snapshot, pools);
          // java pageViewLayoutState.getEnvoy().restoreFromSnapShot(stateSnapshot, this.mDocEnv);
          pageViewLayoutState.getEnvoy().restoreFromSnapShot(stateSnapshot, self.mDocEnv);
          // java pageViewLayoutState.setRangeEnd(pageViewLayoutState.document.getLength());
          pageViewLayoutState.setRangeEnd(pageViewLayoutState.document.getLength());
          // java pools.recycle(stateSnapshot);
          pools.recycle(stateSnapshot);
          // java this.mMainState.curPageTop = TypoLayoutPage.getTop(layoutPage, snapshot);
          self.mMainState.curPageTop = TypoLayoutPage::getTop(layoutPage, snapshot);
        }
        // java this.clearInvalidatePages(pageIndex);
        self.super_.clearInvalidatePages(pageIndex);
        // java if (!this.hasFinishIoLoading())
        if !self.super_.hasFinishIoLoading() {
          // java this.initGraphBuffForLoading();
          self.super_.initGraphBuffForLoading();
        }
        // java this.mHasProcessEnsureValidEndPage = false;
        self.mHasProcessEnsureValidEndPage = false;
      }
      */
    }

    // java public void processEvents(Rect layoutRect)
    pub fn processEvents(&mut self, layoutRect: Option<&Rect>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java LayoutState layoutState = this.mLayoutState;
      let mut layoutState: LayoutState = { let __x: LayoutState = todo!("no align strategy matched"); __x };
      // java layoutState.setRangeEnd(layoutState.document.getLength());
      layoutState.setRangeEnd(layoutState.document.getLength());
      // java this.getEditor();
      self.getEditor();
      // java int oldLayoutPageSize = this.mMainState.snapshot.getLayoutPageSize();
      let mut oldLayoutPageSize: i32 = self.mMainState.snapshot.getLayoutPageSize();
      // java int oldMinLeft = this.mMainState.snapshot.getPosVisitor().getTempMinLeft();
      let mut oldMinLeft: i32 = self.mMainState.snapshot.getPosVisitor().getTempMinLeft();
      // java if (this.mMainState.snapshot.getPosVisitor().getMinLeft() == 0)
      if self.mMainState.snapshot.getPosVisitor().getMinLeft() == 0 {
        // java this.mMainState.snapshot.getPosVisitor().setMinLeft(oldMinLeft);
        self.mMainState.snapshot.getPosVisitor().setMinLeft(oldMinLeft);
      }
      // java if (this.mEditor.flow(layoutRect, (PageViewLayoutState)layoutState))
      if self.mEditor.flow(layoutRect, (layoutState).downcast_ref::<PageViewLayoutState>().unwrap()) {
        // java if (this.mMainState.snapshot.getLayoutPageSize() != oldLayoutPageSize)
        if self.mMainState.snapshot.getLayoutPageSize() != oldLayoutPageSize {
          // java this.mHasProcessEnsureValidEndPage = false;
          self.mHasProcessEnsureValidEndPage = false;
        }
        // java this.onModified(layoutRect);
        self.super_.onModified(layoutRect);
        // java this.mLayoutServiceCache.setStartLocateCache((LocateResult)null, -1);
        self.mLayoutServiceCache.setStartLocateCache(None, -1);
        // java this.mLayoutServiceCache.setEndLocateCache((LocateResult)null, -1);
        self.mLayoutServiceCache.setEndLocateCache(None, -1);
        // java this.mDocEnv.pools.recycleEvents(this.mEvents);
        self.mDocEnv.pools.recycleEvents(self.mEvents);
      }
      // java else
      else {
        // java super.processEvents(layoutRect);
        self.super_.processEvents(layoutRect);
        // java this.mLayoutServiceCache.clearCache();
        self.mLayoutServiceCache.clearCache();
      }
      */
    }

    // java private void getEditor()
    pub fn getEditor(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mEditor)
      if self.mEditor.is_none() {
        // java this.mEditor = new Editor(this.mMainState, this.mDocEnv, this.mViewEnv, this.mEvents, this.mLayoutServiceCache);
        self.mEditor = Editor::new(self.mMainState, self.mDocEnv, self.mViewEnv, self.mEvents, self.mLayoutServiceCache);
      }
      */
    }

    // java protected void flowTextLine(LayoutState layoutState)
    pub fn flowTextLine(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (layoutState.cp != layoutState.paraEnv.curParaPropEnv.start || !layoutState.curPage.hasLines() || !this.processPageBreakBeforePara(layoutState))
      if layoutState.cp != layoutState.paraEnv.curParaPropEnv.start || !layoutState.curPage.hasLines() || !self.processPageBreakBeforePara(layoutState) {
        // java super.flowTextLine(layoutState);
        self.super_.flowTextLine(layoutState);
      }
      */
    }

    // java protected void doFlowLine(LayoutState layoutState)
    pub fn doFlowLine(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mHasProcessColumns)
      if !self.mHasProcessColumns {
        // java if (this.flowColumns(layoutState))
        if self.flowColumns(layoutState) {
          // java return;
          return;
        }
        // java this.mHasProcessColumns = true;
        self.mHasProcessColumns = true;
      }
      // java super.doFlowLine(layoutState);
      self.super_.doFlowLine(layoutState);
      */
    }

    // java private boolean processPageBreakBeforePara(LayoutState layoutState)
    pub fn processPageBreakBeforePara(&mut self, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (layoutState.paraEnv.curParaPropEnv.paraProperty.getBoolean(200, false))
      if layoutState.paraEnv.curParaPropEnv.paraProperty.getBoolean(200, false) {
        // java this.mEnableProcessPagination = false;
        self.mEnableProcessPagination = false;
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java private boolean flowColumns(LayoutState layoutState)
    pub fn flowColumns(&mut self, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mMainState.columns <= 1)
      if self.mMainState.columns <= 1 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (null == this.mColumnsLayouter)
        if self.mColumnsLayouter.is_none() {
          // java this.mColumnsLayouter = new ColumnsLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this);
          self.mColumnsLayouter = ColumnsLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv, self);
        }
        // java if (!this.mColumnsLayouter.flow((PageViewLayoutState)layoutState))
        if !self.mColumnsLayouter.flow({ let __x: &mut LayoutState = todo!("no align strategy matched"); __x }) {
          // java return false;
          return false;
        }
        // java else if (!layoutState.affectRect.isEmpty())
        else if !layoutState.affectRect.is_empty() {
          // java this.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
          self.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return true;
          return true;
        }
        // java else
        else {
          // java this.mSectionLayouter.end(((PageViewLayoutState)layoutState).getEnvoy());
          self.mSectionLayouter.end((layoutState).downcast_ref::<PageViewLayoutState>().unwrap().getEnvoy());
          // java this.mGraphBuff.clear(layoutState);
          self.mGraphBuff.clear(layoutState);
          // java if (!this.isComplete(layoutState) && layoutState.cp <= layoutState.pageEndLimitCP)
          if !self.isComplete({ let __x: &LayoutState = todo!("no align strategy matched"); __x }) && layoutState.cp <= layoutState.pageEndLimitCP {
            // java this.initGraphBuff(layoutState);
            self.super_.initGraphBuff__layoutstate(layoutState);
            // java if (0 == this.mMainState.secBreakCode && this.mDocEnv.canContinueFlowNextSec)
            if 0 == self.mMainState.secBreakCode && self.mDocEnv.canContinueFlowNextSec {
              // java this.mSectionLayouter.startForNoBreakPage(((PageViewLayoutState)layoutState).getEnvoy());
              self.mSectionLayouter.startForNoBreakPage((layoutState).downcast_ref::<PageViewLayoutState>().unwrap().getEnvoy());
            }
          }
          // java else
          else {
            // java layoutState.setFinishPage(true);
            layoutState.setFinishPage(true);
          }
          // java return true;
          return true;
        }
      }
      */
    }

    // java public void clearTypoDocument()
    pub fn clearTypoDocument(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.clearTypoDocument();
      self.super_.clearTypoDocument();
      // java PageViewLayoutState pageViewEnv = (PageViewLayoutState)this.mLayoutState;
      let mut pageViewEnv: PageViewLayoutState = (self.mLayoutState).downcast_ref::<PageViewLayoutState>().unwrap();
      // java pageViewEnv.pagePos = 0;
      pageViewEnv.pagePos = 0;
      */
    }

    // java protected void reset()
    pub fn reset(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSectionLayouter.reset();
      self.mSectionLayouter.reset();
      // java if (null != this.mColumnsLayouter)
      if let Some(mColumnsLayouter) = self.mColumnsLayouter.as_mut() {
        // java this.mColumnsLayouter.reset();
        mColumnsLayouter.reset();
      }
      // java super.reset();
      self.super_.reset();
      */
    }

    // java public void resetForInit()
    pub fn resetForInit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mHasProcessColumns = false;
      self.mHasProcessColumns = false;
      // java super.resetForInit();
      self.super_.resetForInit();
      // java this.clearTypoDocument();
      self.clearTypoDocument();
      */
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java protected void doOnIoLoadFinish()
    pub fn doOnIoLoadFinish(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.doOnIoLoadFinish();
      self.super_.doOnIoLoadFinish();
      // java LayoutState layoutEnv = this.mLayoutState;
      let mut layoutEnv: LayoutState = { let __x: LayoutState = todo!("no align strategy matched"); __x };
      // java ViewEnv viewEnv = this.mViewEnv;
      let mut viewEnv: ViewEnv = self.mViewEnv;
      // java Document doc = layoutEnv.document;
      let mut doc: Box<dyn Document> = layoutEnv.document;
      // java PLCSection sections = doc.getPLCSection();
      let mut sections: PLCSection = doc.getPLCSection();
      // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(doc);
      let mut propTool: Box<dyn IPropertyTool> = self.mDocEnv.propToolFactory.create(doc);
      // java PageViewEventHandler eventHandler = (PageViewEventHandler)this.mEventHandler;
      let mut eventHandler: PageViewEventHandler = (self.mEventHandler).downcast_ref::<PageViewEventHandler>().unwrap();
      // java PLCSection.SectionNode firstSection = sections.getFirstSection();
      let mut firstSection: PLCSection::SectionNode = sections.getFirstSection();
      // java int invalidateStartCP = Integer.MIN_VALUE;
      let mut invalidateStartCP: i32 = i32::MIN;
      // java PLCSection.SectionNode sectionNode = firstSection;
      let mut sectionNode: PLCSection::SectionNode = firstSection;
      // java int sectionIndex = 0;
      let mut sectionIndex: i32 = 0;
      // java while (null != sectionNode)
        // java if (!SectionTool.isHiddenSectionBreak(doc, propTool, sectionNode))
        if !SectionTool::isHiddenSectionBreak(doc, propTool, sectionNode) {
          // java boolean isBKCDifferent = false;
          let mut isBKCDifferent: bool = false;
          // java boolean isSectionPropDifferent = false;
          let mut isSectionPropDifferent: bool = false;
          // java PropertySet sectionProp = SectionTool.calcSectionPropertySet(viewEnv, sectionNode);
          let mut sectionProp: PropertySet = SectionTool::calcSectionPropertySet(viewEnv, sectionNode);
          // java if (2 != sectionProp.getInt(681, 2))
          if 2 != sectionProp.getInt(681, 2) {
            // java isBKCDifferent = true;
            isBKCDifferent = true;
          }
          // java else if (SectionTool.shouldReflowSection(layoutEnv.document, viewEnv, sectionNode))
          else if SectionTool::shouldReflowSection(layoutEnv.document, viewEnv, sectionNode) {
            // java isSectionPropDifferent = true;
            isSectionPropDifferent = true;
          }
          // java if (isBKCDifferent || isSectionPropDifferent)
          if isBKCDifferent || isSectionPropDifferent {
            // java PLCSection.SectionNode startNode = SectionTool.getSectionStartNode(doc, propTool, sectionNode);
            let mut startNode: PLCSection::SectionNode = SectionTool::getSectionStartNode(doc, propTool, sectionNode);
            // java invalidateStartCP = sections.getSectionStart(startNode);
            invalidateStartCP = sections.getSectionStart(startNode);
            // java if (isBKCDifferent)
            if isBKCDifferent {
              // java invalidateStartCP = Math.max(invalidateStartCP - 1, 0);
              invalidateStartCP = (invalidateStartCP - 1).max(0);
            }
            // java break;
            break;
          }
        }
        // java ++sectionIndex;
        { let _tmp = sectionIndex; sectionIndex += 1; _tmp };
        // java PLC.Node nextNode = sectionNode.getNext();
        let mut nextNode: Box<dyn Node> = sectionNode.getNext();
        // java if (null != nextNode && !sections.isHead(nextNode))
        if true && !sections.isHead(nextNode) {
          // java sectionNode = (PLCSection.SectionNode)nextNode;
          sectionNode = (nextNode).downcast_ref::<PLCSection::SectionNode>().unwrap();
        }
        // java else
        else {
          // java sectionNode = null;
          sectionNode = None;
        }
      // java if (invalidateStartCP >= 0)
      if invalidateStartCP >= 0 {
        // java if (sectionIndex > 0)
        if sectionIndex > 0 {
          // java sectionIndex = 0;
          sectionIndex = 0;
          // java invalidateStartCP = 0;
          invalidateStartCP = 0;
        }
        // java TypoSections.clean(sectionIndex, this.mMainState.snapshot.getSections(), this.mMainState.snapshot);
        TypoSections::clean(sectionIndex, self.mMainState.snapshot.getSections(), self.mMainState.snapshot);
        // java this.mLayoutServiceCache.clearCache();
        self.mLayoutServiceCache.clearCache();
        // java layoutEnv.paraEnv.init(this.mDocEnv.pools);
        layoutEnv.paraEnv.init(self.mDocEnv.pools);
        // java int pageIdx = eventHandler.invalidateForMainDoc(invalidateStartCP, doc.getLength(), this.mMainState.snapshot);
        let mut pageIdx: i32 = eventHandler.invalidateForMainDoc(invalidateStartCP, doc.getLength(), self.mMainState.snapshot);
        // java this.onModifyFirst(pageIdx, true);
        self.onModifyFirst(pageIdx, true);
        // java if (pageIdx == 0)
        if pageIdx == 0 {
          // java this.mMainState.snapshot.setModifyFrom(0);
          self.mMainState.snapshot.setModifyFrom(0);
          // java this.mMainState.snapshot.reset();
          self.mMainState.snapshot.reset();
          // java this.mMainState.snapshot.getPosVisitor().afterReset();
          self.mMainState.snapshot.getPosVisitor().afterReset();
        }
      }
      */
    }

    // java public void onAfterFirstRender()
    pub fn onAfterFirstRender(&mut self) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

    // java protected void clearFootNoteRefCPs(LayoutState layoutState)
    pub fn clearFootNoteRefCPs(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PageViewLayoutState pageViewLayoutEnv = (PageViewLayoutState)layoutState;
      let mut pageViewLayoutEnv: PageViewLayoutState = (layoutState).downcast_ref::<PageViewLayoutState>().unwrap();
      // java pageViewLayoutEnv.footEndNoteEnv.footNoteRefCPs.resetQuick();
      pageViewLayoutEnv.footEndNoteEnv.footNoteRefCPs.resetQuick();
      // java pageViewLayoutEnv.footEndNoteEnv.curFootNoteRefCP = -1;
      pageViewLayoutEnv.footEndNoteEnv.curFootNoteRefCP = -1;
      */
    }

    // java protected void clearDrawing(LayoutState layoutState)
    pub fn clearDrawing(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DrawingCacheTool.recycleDrawingCacheItems(this.mMainState.floatsCache, this.mDocEnv.pools, this.mMainState.snapshot);
      DrawingCacheTool::recycleDrawingCacheItems(self.mMainState.floatsCache, self.mDocEnv.pools, self.mMainState.snapshot);
      */
    }

    // java public void makeBalloonCaches()
    pub fn makeBalloonCaches(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int oldCurPageIndex = this.mMainState.curPageIndex;
      let mut oldCurPageIndex: i32 = self.mMainState.curPageIndex;
      // java int oldSecPageIndex = this.mMainState.curSecPageIndex;
      let mut oldSecPageIndex: i32 = self.mMainState.curSecPageIndex;
      // java super.makeBalloonCaches();
      self.super_.makeBalloonCaches();
      // java this.mMainState.curPageIndex = oldCurPageIndex;
      self.mMainState.curPageIndex = oldCurPageIndex;
      // java this.mMainState.curSecPageIndex = oldSecPageIndex;
      self.mMainState.curSecPageIndex = oldSecPageIndex;
      */
    }

    // java public void flowDocument(int maxPageCount, IInterruptState interrupt)
    pub fn flowDocument(&mut self, maxPageCount: i32, interrupt: Option<&IInterruptState>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.prepareFlowFromDocumentBegin();
      self.prepareFlowFromDocumentBegin();
      // java while (!this.isComplete())
      while !self.isComplete() {
        // java --maxPageCount;
        { let _tmp = maxPageCount; maxPageCount -= 1; _tmp };
        // java if (maxPageCount < 0 || interrupt != null && interrupt.checkInterruptedToExit())
        if maxPageCount < 0 || interrupt.is_some() && interrupt.checkInterruptedToExit() {
          // java break;
          break;
        }
        // java this.flowPage();
        self.flowPage();
      }
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      */
    }

    // java public void swapToLocal(boolean sync)
    pub fn swapToLocal(&mut self, sync: bool) {
      // java this.mPageViewBlocksSwap.swapToLocal(sync);
      if let Some(sw) = self.mPageViewBlocksSwap.as_mut() {
        sw.swapToLocal(sync);
      }
    }

    // java public boolean swapFromLocal()
    pub fn swapFromLocal(&mut self) -> bool {
      // java return this.mPageViewBlocksSwap.swapFromLocal();
      return self.mPageViewBlocksSwap.as_mut().map(|sw| sw.swapFromLocal()).unwrap_or(false);
    }

    // java protected boolean rollBackDropCap(LayoutState layoutState)
    pub fn rollBackDropCap(&mut self, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (TypoDrawing.getBottom(this.mTextFrameLayouter.getDrawing(), this.mMainState.snapshot) > layoutState.pageEnv.maxLayoutHeight)
      if TypoDrawing::getBottom(self.mTextFrameLayouter.getDrawing(), self.mMainState.snapshot) > layoutState.pageEnv.maxLayoutHeight {
        // java this.mMainState.getDrawingLayers(layoutState.document).removeDrawing(this.mTextFrameLayouter.getDrawing());
        self.mMainState.getDrawingLayers(layoutState.document).removeDrawing(self.mTextFrameLayouter.getDrawing());
        // java this.mDocEnv.cacheCleaner.recycleDrawing(this.mTextFrameLayouter.getDrawing());
        self.mDocEnv.cacheCleaner.recycleDrawing(self.mTextFrameLayouter.getDrawing());
        // java this.rollbackLine(layoutState.linePos - 1, layoutState);
        self.rollbackLine(layoutState.linePos - 1, layoutState);
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

}

impl std::ops::Deref for PageViewLayouter {
    type Target = MainLayouter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PageViewLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
