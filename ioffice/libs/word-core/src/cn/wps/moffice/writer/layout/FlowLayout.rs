// Java: import cn.wps.collections.KList
// CARGO_OFF_SURFACE use util::collections::KList::KList;
// Java: import cn.wps.graphics.Rect
// CARGO_OFF_SURFACE use util::graphics::Rect::Rect;
// Java: import cn.wps.moffice.writer.cache.CacheVerify
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::CacheVerify::CacheVerify;
// Java: import cn.wps.moffice.writer.cache.TypoDocument
use crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.disk.BlocksSwapManager
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::disk::BlocksSwapManager::BlocksSwapManager;
// Java: import cn.wps.moffice.writer.cache.firstcache.FirstCacheBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::firstcache::FirstCacheBase::FirstCacheBase;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent;
// Java: import cn.wps.moffice.writer.core.IInterruptState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::IInterruptState::IInterruptState;
// Java: import cn.wps.moffice.writer.core.pool.Reusable
use crate::cn::wps::moffice::writer::core::pool::Reusable::Reusable;
// Java: import cn.wps.moffice.writer.data.table.grid.GridSizeCommit
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::grid::GridSizeCommit::GridSizeCommit;
// Java: import cn.wps.moffice.writer.layout.ShareView.ShareViewLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::ShareView::ShareViewLayouter::ShareViewLayouter;
// Java: import cn.wps.moffice.writer.layout.base.CacheCleaner
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::CacheCleaner::CacheCleaner;
// Java: import cn.wps.moffice.writer.layout.base.IViewListener
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::IViewListener::IViewListener;
// Java: import cn.wps.moffice.writer.layout.base.MainLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::MainLayouter::MainLayouter;
// Java: import cn.wps.moffice.writer.layout.base.balloons.footendnote.FootEndNoteBalloonLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::balloons::footendnote::FootEndNoteBalloonLayouter::FootEndNoteBalloonLayouter;
// Java: import cn.wps.moffice.writer.layout.base.edit.Event
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::Event::Event;
// Java: import cn.wps.moffice.writer.layout.base.edit.EventCollector
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::EventCollector::EventCollector;
// Java: import cn.wps.moffice.writer.layout.base.edit.GridSizeLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::GridSizeLayouter::GridSizeLayouter;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCacheFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FontStyleCacheFactory::FontStyleCacheFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.FootNoteCache
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FootNoteCache::FootNoteCache;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.PropertyToolFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.math.MathDrawingLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::math::MathDrawingLayouter::MathDrawingLayouter;
// Java: import cn.wps.moffice.writer.layout.base.smallerscale.IPageGrid
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::smallerscale::IPageGrid::IPageGrid;
// Java: import cn.wps.moffice.writer.layout.base.tools.LocksHelper
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::LocksHelper::LocksHelper;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::pageview::PageViewLayouter::PageViewLayouter;
// Java: import cn.wps.moffice.writer.layout.phoneview.PhoneViewLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::phoneview::PhoneViewLayouter::PhoneViewLayouter;
// Java: import cn.wps.moffice.writer.layout.phoneviewscroll.PhoneViewScrollLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::phoneviewscroll::PhoneViewScrollLayouter::PhoneViewScrollLayouter;
// Java: import cn.wps.moffice.writer.layout.webview.WebViewLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::webview::WebViewLayouter::WebViewLayouter;
// Java: import cn.wps.moffice.writer.render.base.IMathDrawer
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::render::base::IMathDrawer::IMathDrawer;
// Java: import cn.wps.moffice.writer.service.LayoutServiceCache
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::LayoutServiceCache::LayoutServiceCache;
// Java: import cn.wps.moffice.writer.service.LocateResult
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::LocateResult::LocateResult;
use core::marker::PhantomData as _;


// CARGO_OFF_SURFACE_TYPES
#[derive(Default)]
pub struct CacheCleaner;
#[derive(Default)]
pub struct LocksHelper;
#[derive(Default)]
pub struct Event;
#[derive(Default)]
pub struct EventCollector;
#[derive(Default)]
pub struct LayoutServiceCache;
#[derive(Default)]
pub struct PropertyToolFactory;
#[derive(Default)]
pub struct FontStyleCacheFactory;
#[derive(Default)]
pub struct MathDrawingLayouter;
#[derive(Default)]
pub struct FootEndNoteBalloonLayouter;
#[derive(Default)]
pub struct MainLayouter;
#[derive(Default)]
pub struct PageViewLayouter;
impl PageViewLayouter {
    pub fn PageViewLayouter(
        _mainState: &MainState,
        _docEnv: &DocumentEnv,
        _viewEnv: Option<&ViewEnv>,
        _locksHelper: &LocksHelper,
        _events: &KList,
        _layoutServiceCache: Option<&LayoutServiceCache>,
    ) -> Self {
        Self::default()
    }
    pub fn clearTypoDocument(&mut self) {}
    pub fn onSetTextDocument(&mut self) {}
}
#[derive(Default)]
pub struct KList;
#[derive(Default)]
pub struct Rect;
#[derive(Default)]
pub struct IInterruptState;
#[derive(Default)]
pub struct GridSizeCommit;
pub trait IMathDrawer {}
#[derive(Default)]
pub struct LocateResult;
impl MainLayouter {
    // java public static final int PAGEVIEW_LAYOUTER = 0;
    pub const PAGEVIEW_LAYOUTER: i32 = 0;
    // java public static final int WEBVIEW_LAYOUTER = 1;
    pub const WEBVIEW_LAYOUTER: i32 = 1;
    // java public static final int PHONEVIEW_LAYOUTER = 2;
    pub const PHONEVIEW_LAYOUTER: i32 = 2;
    // java public static final int PHONEVIEWSCROLL_LAYOUTER = 3;
    pub const PHONEVIEWSCROLL_LAYOUTER: i32 = 3;
    // java public static final int SHARE_LAYOUTER = 4;
    pub const SHARE_LAYOUTER: i32 = 4;
}
#[derive(Default)]
pub struct WebViewLayouter;
#[derive(Default)]
pub struct PhoneViewLayouter;
#[derive(Default)]
pub struct PhoneViewScrollLayouter;
#[derive(Default)]
pub struct ShareViewLayouter;
pub const PAGEVIEW_LAYOUTER: i32 = 0;
pub const WEBVIEW_LAYOUTER: i32 = 1;
pub const PHONEVIEW_LAYOUTER: i32 = 2;
pub const PHONEVIEWSCROLL_LAYOUTER: i32 = 3;
pub const SHARE_LAYOUTER: i32 = 4;

// java public class FlowLayout implements Reusable
#[derive(Default)]
pub struct FlowLayout {
    // java MainLayouter mMainLayouter;
    pub mMainLayouter: Option<PageViewLayouter>,
    // java TypoDocument mTypoDocument;
    pub(crate) mTypoDocument: Option<Box<TypoDocument>>,
    // java MainState mMainState;
    pub(crate) mMainState: MainState,
    // java DocumentEnv mDocEnv;
    pub(crate) mDocEnv: DocumentEnv,
    // java ViewEnv mViewEnv;
    pub(crate) mViewEnv: Option<Box<ViewEnv>>,
    // java CacheCleaner mCacheCleaner;
    pub(crate) mCacheCleaner: Option<Box<CacheCleaner>>,
    // java LocksHelper mLocksHelper;
    pub(crate) mLocksHelper: LocksHelper,
    // java TypoRectCoreImpl mLayoutRect;
    pub(crate) mLayoutRect: TypoRectCoreImpl,
    // java boolean mLayoutComplete;
    pub(crate) mLayoutComplete: bool,
    // java Pools mPools;
    pub(crate) mPools: Pools,
    // java KList<Event> mEvents;
    pub(crate) mEvents: KList,
    // java EventCollector mEventCollector;
    pub(crate) mEventCollector: EventCollector,
    // java LayoutServiceCache mLayoutServiceCache;
    pub(crate) mLayoutServiceCache: Option<Box<LayoutServiceCache>>,
    // java PropertyToolFactory mPropertyToolFactory;
    pub(crate) mPropertyToolFactory: PropertyToolFactory,
    // java FontStyleCacheFactory mFontStyleCacheFactory;
    pub(crate) mFontStyleCacheFactory: FontStyleCacheFactory,
    // java MathDrawingLayouter mMathDrawingLayouter;
    pub(crate) mMathDrawingLayouter: Option<Box<MathDrawingLayouter>>,
    // java boolean mHasInit;
    pub(crate) mHasInit: bool,
    // java boolean mHasProcessedIOLoadFinishEvent;
    pub(crate) mHasProcessedIOLoadFinishEvent: bool,
    // java FootEndNoteBalloonLayouter mFootEndNoteBalloonLayouter;
    pub(crate) mFootEndNoteBalloonLayouter: Option<Box<FootEndNoteBalloonLayouter>>,
    // java int mMode;
    pub(crate) mMode: i32,
    snapshot_usable: bool,
    version: i64,
}

// java public interface Reusable
impl Reusable for FlowLayout {
    // java public boolean reuseClean()
    fn reuseClean(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFontStyleCacheFactory.reuseClean();
      self.mFontStyleCacheFactory.reuseClean();
      // java this.mPropertyToolFactory.reuseClean();
      self.mPropertyToolFactory.reuseClean();
      // java this.mDocEnv.reuseClean();
      self.mDocEnv.reuseClean();
      // java this.mLocksHelper.reuseClean();
      self.mLocksHelper.reuseClean();
      // java this.mPools.recycleEvents(this.mEvents);
      self.mPools.recycleEvents(self.mEvents);
      // java this.mEventCollector.clear();
      self.mEventCollector.clear();
      // java if (null != this.mMainLayouter)
      if let Some(mMainLayouter) = self.mMainLayouter.as_mut() {
        // java this.mMainLayouter.dispose();
        mMainLayouter.dispose();
        // java this.mMainLayouter = null;
        mMainLayouter = None;
      }
      // java this.mMainState.reuseClean(this.mPools);
      self.mMainState.reuseClean__pools(self.mPools);
      // java if (null != this.mMathDrawingLayouter)
      if let Some(mMathDrawingLayouter) = self.mMathDrawingLayouter.as_mut() {
        // java this.mMathDrawingLayouter.dispose();
        mMathDrawingLayouter.dispose();
        // java this.mMathDrawingLayouter = null;
        mMathDrawingLayouter = None;
      }
      // java if (this.mFootEndNoteBalloonLayouter != null)
      if let Some(mFootEndNoteBalloonLayouter) = self.mFootEndNoteBalloonLayouter.as_mut() {
        // java this.mFootEndNoteBalloonLayouter.dispose();
        mFootEndNoteBalloonLayouter.dispose();
        // java this.mFootEndNoteBalloonLayouter = null;
        mFootEndNoteBalloonLayouter = None;
      }
      // java this.mHasProcessedIOLoadFinishEvent = false;
      self.mHasProcessedIOLoadFinishEvent = false;
      // java this.mHasInit = false;
      self.mHasInit = false;
      // java this.mPools.reuseClean();
      self.mPools.reuseClean();
      // java return true;
      return true;
      */
    }

    // java public void reuseInit()
    fn reuseInit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState.footEndNoteCache = new FootNoteCache(this.mPools);
      self.mMainState.footEndNoteCache = FootNoteCache::new(self.mPools);
      // java this.mLocksHelper.setTextDocument(this.mTypoDocument.getDocument());
      self.mLocksHelper.setTextDocument(self.mTypoDocument.getDocument());
      // java this.mDocEnv.reuseInit();
      self.mDocEnv.reuseInit();
      */
    }

}

// java public class FlowLayout implements Reusable
impl FlowLayout {
    // java public LocksHelper getLocksHelper()
    pub fn getLocksHelper(&self) -> &LocksHelper {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mLocksHelper;
      return &self.mLocksHelper;
      */
    }

    // java public  FlowLayout(TypoDocument typoDocument, ViewEnv viewEnv, LayoutServiceCache layoutServiceCache)
    pub fn FlowLayout(typoDocument: &TypoDocument, viewEnv: &ViewEnv, layoutServiceCache: &LayoutServiceCache) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mViewEnv = viewEnv;
      let mut this = Self { mMainLayouter: Default::default(), mTypoDocument: typoDocument, mMainState: MainState::new(), mDocEnv: Default::default(), mViewEnv: viewEnv, mCacheCleaner: Default::default(), mLocksHelper: Default::default(), mLayoutRect: TypoRectCoreImpl::new(), mLayoutComplete: Default::default(), mPools: Default::default(), mEvents: Default::default(), mEventCollector: Default::default(), mLayoutServiceCache: layoutServiceCache, mPropertyToolFactory: Default::default(), mFontStyleCacheFactory: Default::default(), mMathDrawingLayouter: Default::default(), mHasInit: Default::default(), mHasProcessedIOLoadFinishEvent: Default::default(), mFootEndNoteBalloonLayouter: Default::default(), mMode: viewEnv.getViewMode() };
      // java this.mLayoutRect = new TypoRectCoreImpl();
      // java this.mTypoDocument = typoDocument;
      // java this.mLayoutServiceCache = layoutServiceCache;
      // java this.mMainState = new MainState();
      // java this.mMode = viewEnv.getViewMode();
      // java this.doInit();
      this.doInit();
      this
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTypoDocument = null;
      self.mTypoDocument = None;
      // java this.mViewEnv = null;
      self.mViewEnv = None;
      // java this.mLayoutServiceCache = null;
      self.mLayoutServiceCache = None;
      // java if (this.mMainState.snapshot != null)
      if self.mMainState.snapshot.is_some() {
        // java this.mMainState.snapshot.release();
        self.mMainState.snapshot.release();
        // java this.mMainState.snapshot = null;
        self.mMainState.snapshot = None;
      }
      // java if (null != this.mMainLayouter)
      if let Some(mMainLayouter) = self.mMainLayouter.as_mut() {
        // java this.mMainLayouter.dispose();
        mMainLayouter.dispose();
        // java this.mMainLayouter = null;
        mMainLayouter = None;
      }
      // java this.mMainState.disposeGridSizeLayouter();
      self.mMainState.disposeGridSizeLayouter();
      // java if (null != this.mCacheCleaner)
      if let Some(mCacheCleaner) = self.mCacheCleaner.as_mut() {
        // java this.mCacheCleaner.dispose();
        mCacheCleaner.dispose();
        // java this.mCacheCleaner = null;
        mCacheCleaner = None;
      }
      // java if (null != this.mLocksHelper)
      if true {
        // java this.mLocksHelper.dispose();
        self.mLocksHelper.dispose();
        // java this.mLocksHelper = null;
        self.mLocksHelper = None;
      }
      // java if (null != this.mEventCollector)
      if true {
        // java this.mEventCollector.dispose();
        self.mEventCollector.dispose();
        // java this.mEventCollector = null;
        self.mEventCollector = None;
      }
      // java if (null != this.mEvents)
      if true {
        // java this.mEvents.dispose();
        self.mEvents.dispose();
        // java this.mEvents = null;
        self.mEvents = None;
      }
      // java if (null != this.mPools)
      if true {
        // java this.mPools.dispose();
        self.mPools.dispose();
        // java this.mPools = null;
        self.mPools = None;
      }
      // java if (null != this.mDocEnv)
      if true {
        // java this.mDocEnv.dispose();
        self.mDocEnv.dispose();
        // java this.mDocEnv = null;
        self.mDocEnv = None;
      }
      // java if (null != this.mPropertyToolFactory)
      if true {
        // java this.mPropertyToolFactory.dispose();
        self.mPropertyToolFactory.dispose();
        // java this.mPropertyToolFactory = null;
        self.mPropertyToolFactory = None;
      }
      // java if (null != this.mFontStyleCacheFactory)
      if true {
        // java this.mFontStyleCacheFactory.dispose();
        self.mFontStyleCacheFactory.dispose();
        // java this.mFontStyleCacheFactory = null;
        self.mFontStyleCacheFactory = None;
      }
      // java if (null != this.mMathDrawingLayouter)
      if let Some(mMathDrawingLayouter) = self.mMathDrawingLayouter.as_mut() {
        // java this.mMathDrawingLayouter.dispose();
        mMathDrawingLayouter.dispose();
        // java this.mMathDrawingLayouter = null;
        mMathDrawingLayouter = None;
      }
      // java if (null != this.mFootEndNoteBalloonLayouter)
      if let Some(mFootEndNoteBalloonLayouter) = self.mFootEndNoteBalloonLayouter.as_mut() {
        // java this.mFootEndNoteBalloonLayouter.dispose();
        mFootEndNoteBalloonLayouter.dispose();
        // java this.mFootEndNoteBalloonLayouter = null;
        mFootEndNoteBalloonLayouter = None;
      }
      */
    }

    // java public TypoDocument getTypoDocument()
    pub fn getTypoDocument(&self) -> Option<&TypoDocument> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mTypoDocument;
      return { let __x: Option<&TypoDocument> = todo!("no align strategy matched"); __x };
      */
    }

    // java public PropertyToolFactory getPropertyToolFactory()
    pub fn getPropertyToolFactory(&self) -> &PropertyToolFactory {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPropertyToolFactory;
      return &self.mPropertyToolFactory;
      */
    }

    // java public void changeMode()
    pub fn changeMode(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FontStyleFactory.setSmartFontSize(false);
      FontStyleFactory::setSmartFontSize(false);
      // java this.mFontStyleCacheFactory.clearCoreCharStyleCache();
      self.mFontStyleCacheFactory.clearCoreCharStyleCache();
      // java this.resetTypoLayoutState();
      self.resetTypoLayoutState();
      // java this.mTypoDocument.getTableViewport(false).clear();
      self.mTypoDocument.getTableViewport(false).clear();
      // java this.mMode = this.mViewEnv.getViewMode();
      self.mMode = self.mViewEnv.getViewMode();
      // java this.invalidCardState();
      self.invalidCardState();
      */
    }

    // java public int getMode()
    pub fn getMode(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mMode;
      return self.mMode;
      */
    }

    // java public void onModifyFirst(int pageIndex, boolean recycleAfter)
    pub fn onModifyFirst(&mut self, pageIndex: i32, recycleAfter: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainLayouter.onModifyFirst(pageIndex, recycleAfter);
      self.mMainLayouter.onModifyFirst(pageIndex, recycleAfter);
      */
    }

    // java public void resetTypoLayoutState()
    pub fn resetTypoLayoutState(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState.reset();
      self.mMainState.reset();
      // java BlocksSwapManager.cancleSwapTask();
      BlocksSwapManager::cancleSwapTask();
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java if (null != this.mMainLayouter)
      if let Some(mMainLayouter) = self.mMainLayouter.as_mut() {
        // java int viewMode = this.mViewEnv.getViewMode();
        let mut viewMode: i32 = self.mViewEnv.getViewMode();
        // java int layoutType = this.mMainLayouter.getType();
        let mut layoutType: i32 = mMainLayouter.getType();
        // java if (layoutType == 0 && viewMode == 0)
        if layoutType == 0 && viewMode == 0 {
          // java this.mMainLayouter.resetForInit();
          mMainLayouter.resetForInit();
          // java return;
          return;
        }
        // java if (layoutType == 3 && viewMode == 3)
        if layoutType == 3 && viewMode == 3 {
          // java this.mMainLayouter.resetForInit();
          mMainLayouter.resetForInit();
          // java return;
          return;
        }
        // java if (layoutType == 2 && viewMode == 2)
        if layoutType == 2 && viewMode == 2 {
          // java this.mMainLayouter.resetForInit();
          mMainLayouter.resetForInit();
          // java return;
          return;
        }
        // java if (layoutType == 1 && viewMode == 1)
        if layoutType == 1 && viewMode == 1 {
          // java this.mMainLayouter.resetForInit();
          mMainLayouter.resetForInit();
          // java return;
          return;
        }
        // java if (layoutType == 4 && viewMode == 4)
        if layoutType == 4 && viewMode == 4 {
          // java this.mMainLayouter.resetForInit();
          mMainLayouter.resetForInit();
          // java return;
          return;
        }
        // java this.mMainLayouter.dispose();
        mMainLayouter.dispose();
        // java this.mMainLayouter = null;
        mMainLayouter = None;
      }
      // java this.createDocLayouter(this.mViewEnv.getViewMode());
      self.createDocLayouter(self.mViewEnv.getViewMode());
      */
    }

    // java public void changeSize(boolean needClearFontStyleCache)
    pub fn changeSize(&mut self, needClearFontStyleCache: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (needClearFontStyleCache)
      if needClearFontStyleCache {
        // java this.mFontStyleCacheFactory.clearCoreCharStyleCache();
        self.mFontStyleCacheFactory.clearCoreCharStyleCache();
      }
      // java this.mMainLayouter.changePageSize();
      self.mMainLayouter.changePageSize();
      */
    }

    // java public synchronized boolean update(DocumentEvent e)
    pub fn update(&mut self, e: Option<&DocumentEvent>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mEventCollector.update(e);
      return self.mEventCollector.update(e);
      */
    }

    // java public synchronized void beforeUpdate(DocumentEvent e)
    pub fn beforeUpdate(&mut self, e: Option<&DocumentEvent>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mEventCollector.beforeUpdate(e);
      self.mEventCollector.beforeUpdate(e);
      */
    }

    // java public boolean isLayoutComplete()
    pub fn isLayoutComplete(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mLayoutComplete;
      return self.mLayoutComplete;
      */
    }

    // java  void setLayoutComplete(boolean value)
    pub fn setLayoutComplete(&mut self, value: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutComplete = value;
      self.mLayoutComplete = value;
      */
    }

    // java public boolean acceptCacheSnapShot(Rect rect, final boolean hasFinishedIO)
    pub fn acceptCacheSnapShot(&mut self, rect: Option<&Rect>, hasFinishedIO: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FirstCacheBase firstCacheBase = FirstCacheBase.getInstance();
      let mut firstCacheBase: FirstCacheBase = FirstCacheBase::getInstance();
      // java if (firstCacheBase == null)
      if false {
        // java return false;
        return false;
      }
      // java else
      else {
        // java long start = 0L;
        let mut start: i64 = 0;
        // java if (null != this.mMainState.snapshot && !this.mMainState.snapshot.isFirstCache())
        if self.mMainState.snapshot.is_some() && !self.mMainState.snapshot.isFirstCache() {
          // java this.mMainState.snapshot.release();
          self.mMainState.snapshot.release();
          // java this.mMainState.snapshot = this.mDocEnv.typoDocument.newSnapshot();
          self.mMainState.snapshot = self.mDocEnv.typoDocument.newSnapshot();
        }
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java this.mDocEnv.update();
        self.mDocEnv.update();
        // java boolean acceptOk = firstCacheBase.acceptFirstCache(snapshot, new FirstCacheBase.IAcceptCallback() { public int waitPageEnd(int maxCp) { return FlowLayout.this.mMainLayouter.waitRangeEnd(maxCp); } public boolean isFinishedIO() { return hasFinishedIO; } });
        let mut acceptOk: bool = firstCacheBase.acceptFirstCache(snapshot, IAcceptCallback::new());
        // java if (acceptOk)
        if acceptOk {
          // java acceptOk = this.mMainLayouter.acceptFirstCache(snapshot);
          acceptOk = self.mMainLayouter.acceptFirstCache(snapshot);
        }
        // java if (acceptOk)
        if acceptOk {
          // java rect.set(0, snapshot.getDocumentTop(), snapshot.getDocumentWidth(), snapshot.getDocumentBottom());
          rect.set__int_int_int_int(0, snapshot.getDocumentTop(), snapshot.getDocumentWidth(), snapshot.getDocumentBottom());
          // java int pFirsPage = TypoList.first(snapshot.getLayoutPageList(), snapshot);
          let mut pFirsPage: i32 = TypoList::first__int_typosnapshot(snapshot.getLayoutPageList(), snapshot);
          // java this.commitSnapshot(pFirsPage);
          self.commitSnapshot__int(pFirsPage);
        }
        // java return acceptOk;
        return acceptOk;
      }
      */
    }

    // java public DocumentEnv getDocumentEnv()
    pub fn getDocumentEnv(&self) -> &DocumentEnv {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mDocEnv;
      return &self.mDocEnv;
      */
    }

    // java public boolean flowPage(Rect rect)
    pub fn flowPage(&mut self, rect: Option<&Rect>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java TypoLayoutPage page = null;
      let mut page: TypoLayoutPage = Default::default();
      // java page = this.mMainLayouter.flowPage();
      page = self.mMainLayouter.flowPage();
      // java if (null != page && null != rect)
      if let Some(rect) = rect {
        if true {
          // java TypoRectCoreImpl pageRect = TypoRectCoreImpl.obtain();
          let mut pageRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
          // java if (rect.isEmpty())
          if rect.isEmpty() {
            // java this.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(pageRect, page.getPointer());
            self.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(pageRect, page.getPointer());
            // java rect.set(pageRect);
            rect.set__rect(pageRect);
          }
          // java else
          else {
            // java this.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(pageRect, page.getPointer());
            self.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(pageRect, page.getPointer());
            // java rect.top = Math.min(rect.top, pageRect.getTop());
            rect.top = (rect.top).min(pageRect.getTop());
            // java rect.bottom = Math.max(rect.bottom, pageRect.getBottom());
            rect.bottom = (rect.bottom).max(pageRect.getBottom());
          }
          // java pageRect.recycle();
          pageRect.recycle();
        }
      }
      // java if (this.checkLayoutComplete())
      if self.checkLayoutComplete() {
        // java this.mMainLayouter.ensureValidEndPage(rect);
        self.mMainLayouter.ensureValidEndPage(rect);
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java snapshot.setDocumentBottom(Math.max(this.mMainLayouter.getCurPageTop(), snapshot.getDocumentBottom()));
      snapshot.setDocumentBottom((self.mMainLayouter.getCurPageTop()).max(snapshot.getDocumentBottom()));
      // java this.commitSnapshot(null != page ? page.getPointer() : 0);
      self.commitSnapshot__int((if true { page.getPointer() } else { 0 }));
      // java this.mLocksHelper.unlockDocument();
      self.mLocksHelper.unlockDocument();
      // java return page != null;
      return true;
      */
    }

    // java private boolean checkLayoutComplete()
    pub fn checkLayoutComplete(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean complete = this.mMainLayouter.isComplete();
      let mut complete: bool = self.mMainLayouter.isComplete();
      // java this.setLayoutComplete(complete);
      self.setLayoutComplete(complete);
      // java return complete;
      return complete;
      */
    }

    // java private void createDocLayouter(int mode)
    pub fn createDocLayouter(&mut self, mode: i32) {
      // java switch (mode)
      match mode {
        // java case 1:
        1 => {
          // java this.mMainLayouter = new WebViewLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this.mLocksHelper, this.mEvents, this.mLayoutServiceCache);
          let _ = WebViewLayouter::default();
          todo!("DEFERRED_NON_FUNCTION");
        }
        // java case 2:
        2 => {
          // java this.mMainLayouter = new PhoneViewLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this.mLocksHelper, this.mEvents, this.mLayoutServiceCache);
          let _ = PhoneViewLayouter::default();
          todo!("DEFERRED_NON_FUNCTION");
        }
        // java case 3:
        3 => {
          // java this.mMainLayouter = new PhoneViewScrollLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this.mLocksHelper, this.mEvents, this.mLayoutServiceCache);
          let _ = PhoneViewScrollLayouter::default();
          todo!("DEFERRED_NON_FUNCTION");
        }
        // java case 4:
        4 => {
          // java this.mMainLayouter = new ShareViewLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this.mLocksHelper, this.mEvents, this.mLayoutServiceCache);
          let _ = ShareViewLayouter::default();
          todo!("DEFERRED_NON_FUNCTION");
        }
        // java case 0: default:
        0 | _ => {
          // java this.mMainLayouter = new PageViewLayouter(this.mMainState, this.mDocEnv, this.mViewEnv, this.mLocksHelper, this.mEvents, this.mLayoutServiceCache);
          self.mMainLayouter = Some(PageViewLayouter::PageViewLayouter(
            &self.mMainState,
            &self.mDocEnv,
            self.mViewEnv.as_deref(),
            &self.mLocksHelper,
            &self.mEvents,
            self.mLayoutServiceCache.as_deref(),
          ));
          // java this.mMainLayouter.clearTypoDocument();
          if let Some(lo) = self.mMainLayouter.as_mut() {
            lo.clearTypoDocument();
          }
        }
      }
      // java this.mMainLayouter.onSetTextDocument();
      if let Some(lo) = self.mMainLayouter.as_mut() {
        lo.onSetTextDocument();
      }
    }

    // java public void init()
    pub fn init(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools.recycleEvents(this.mEvents);
      self.mPools.recycleEvents(self.mEvents);
      // java this.mEventCollector.clear();
      self.mEventCollector.clear();
      // java this.mDocEnv.init();
      self.mDocEnv.init();
      // java this.changeMode();
      self.changeMode();
      // java this.mHasInit = true;
      self.mHasInit = true;
      // java this.mViewEnv.resetViewTop();
      self.mViewEnv.resetViewTop();
      // java this.mHasProcessedIOLoadFinishEvent = false;
      self.mHasProcessedIOLoadFinishEvent = false;
      */
    }

    // java public void showFormatChangeInBallons(boolean showFormatChangeInBallons)
    pub fn showFormatChangeInBallons(&mut self, showFormatChangeInBallons: bool) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

    // java public void onScroll(int scrollTop, Rect rect, TypoSnapshot snapshot)
    pub fn onScroll__int_rect_typosnapshot(&mut self, scrollTop: i32, rect: Option<&Rect>, snapshot: Option<&TypoSnapshot>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.onScroll(scrollTop, rect, false, snapshot);
      self.onScroll__int_rect_boolean_typosnapshot(scrollTop, rect, false, snapshot);
      */
    }

    // java public void onScroll(int scrollTop, Rect rect, boolean needNodifyListener, TypoSnapshot snapshot)
    pub fn onScroll__int_rect_boolean_typosnapshot(&mut self, scrollTop: i32, rect: Option<&Rect>, needNodifyListener: bool, snapshot: Option<&TypoSnapshot>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainLayouter.onScroll(scrollTop, needNodifyListener);
      self.mMainLayouter.onScroll__int_boolean(scrollTop, needNodifyListener);
      // java if (!this.mViewEnv.isWebLayoutMode())
      if !self.mViewEnv.isWebLayoutMode() {
        // java this.flowViewRect(rect);
        self.flowViewRect(rect);
      }
      // java else
      else {
        // java IViewListener viewListener = this.mViewEnv.getViewListener();
        let mut viewListener: Box<dyn IViewListener> = self.mViewEnv.getViewListener();
        // java if (null != viewListener)
        if true {
          // java viewListener.updateCPOfFirstLineOfView(snapshot);
          viewListener.updateCPOfFirstLineOfView(snapshot);
        }
      }
      */
    }

    // java public void flowCP(int cp, int endCp, int docType, Rect rect)
    pub fn flowCP(&mut self, cp: i32, endCp: i32, docType: i32, rect: Option<&Rect>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java if (!this.mEvents.isEmpty())
      if !self.mEvents.is_empty() {
        // java this.flowViewRect(rect);
        self.flowViewRect(rect);
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java this.mMainLayouter.flowCP(cp, endCp, docType);
      self.mMainLayouter.flowCP(cp, endCp, docType);
      // java this.commitSnapshot();
      self.commitSnapshot();
      // java if (this.mViewEnv.getViewMode() == 0)
      if self.mViewEnv.getViewMode() == 0 {
        // java int oldViewTop = this.mViewEnv.getViewTop();
        let mut oldViewTop: i32 = self.mViewEnv.getViewTop();
        // java TypoSnapshot snapshot = this.mTypoDocument.getSnapshot();
        let mut snapshot: TypoSnapshot = self.mTypoDocument.getSnapshot();
        // java this.onScroll(this.mMainLayouter.getCurPageTop() - 1440, this.mLayoutRect, snapshot);
        self.onScroll__int_rect_typosnapshot(self.mMainLayouter.getCurPageTop() - 1440, { let __x: Option<&Rect> = todo!("no align strategy matched"); __x }, Some(&snapshot));
        // java this.mViewEnv.onScroll(oldViewTop);
        self.mViewEnv.onScroll(oldViewTop);
        // java this.mLayoutServiceCache.updateTableInfoCache(this.mViewEnv, this.mTypoDocument);
        self.mLayoutServiceCache.updateTableInfoCache(self.mViewEnv, self.mTypoDocument);
        // java snapshot.release();
        snapshot.release();
      }
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      */
    }

    // java private void commitSnapshot(int pointer)
    pub fn commitSnapshot__int(&mut self, pointer: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (CacheVerify.ENABLE_VERIFY && 0 != pointer)
      if CacheVerify::ENABLE_VERIFY && 0 != pointer {
        // java CacheVerify.verifyLayoutPage(pointer, this.mMainState.snapshot);
        CacheVerify::verifyLayoutPage(pointer, self.mMainState.snapshot);
      }
      // java this.mTypoDocument.commitSnapshot(this.mMainState.snapshot);
      self.mTypoDocument.commitSnapshot(self.mMainState.snapshot);
      // java this.mMainState.snapshot = null;
      self.mMainState.snapshot = None;
      // java TypoSnapshot snapshot = this.mTypoDocument.getSnapshot();
      let mut snapshot: TypoSnapshot = self.mTypoDocument.getSnapshot();
      // java this.mLayoutServiceCache.commitUpdate(snapshot);
      self.mLayoutServiceCache.commitUpdate(snapshot);
      // java snapshot.release();
      snapshot.release();
      */
    }

    // java private void commitSnapshot()
    pub fn commitSnapshot(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (CacheVerify.ENABLE_VERIFY)
      if CacheVerify::ENABLE_VERIFY {
        // java CacheVerify.verify(this.mMainState.snapshot);
        CacheVerify::verify(self.mMainState.snapshot);
      }
      // java this.mTypoDocument.commitSnapshot(this.mMainState.snapshot);
      self.mTypoDocument.commitSnapshot(self.mMainState.snapshot);
      // java this.mMainState.snapshot = null;
      self.mMainState.snapshot = None;
      // java TypoSnapshot snapshot = this.mTypoDocument.getSnapshot();
      let mut snapshot: TypoSnapshot = self.mTypoDocument.getSnapshot();
      // java this.mLayoutServiceCache.commitUpdate(snapshot);
      self.mLayoutServiceCache.commitUpdate(snapshot);
      // java snapshot.release();
      snapshot.release();
      */
    }

    // java public int flowToPage(int pageIndex)
    pub fn flowToPage(&mut self, pageIndex: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java if (!this.mEvents.isEmpty())
      if !self.mEvents.is_empty() {
        // java this.flowViewRect((Rect)null);
        self.flowViewRect(None);
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java int cp = this.mMainLayouter.flowToPage(pageIndex);
      let mut cp: i32 = self.mMainLayouter.flowToPage(pageIndex);
      // java if (this.mViewEnv.getViewMode() == 0)
      if self.mViewEnv.getViewMode() == 0 {
        // java int oldViewTop = this.mViewEnv.getViewTop();
        let mut oldViewTop: i32 = self.mViewEnv.getViewTop();
        // java this.mViewEnv.onScroll(oldViewTop);
        self.mViewEnv.onScroll(oldViewTop);
        // java this.mLayoutServiceCache.updateTableInfoCache(this.mViewEnv, this.mTypoDocument);
        self.mLayoutServiceCache.updateTableInfoCache(self.mViewEnv, self.mTypoDocument);
      }
      // java this.commitSnapshot();
      self.commitSnapshot();
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      // java return cp;
      return cp;
      */
    }

    // java public boolean prepareToJumpToLastView()
    pub fn prepareToJumpToLastView(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java boolean ret = false;
      let mut ret: bool = false;
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java if (this.mMainLayouter.prepareToJumpToLastView())
      if self.mMainLayouter.prepareToJumpToLastView() {
        // java if (this.mViewEnv.getViewMode() == 0)
        if self.mViewEnv.getViewMode() == 0 {
          // java this.flowViewRect(this.mLayoutRect);
          self.flowViewRect({ let __x: Option<&Rect> = todo!("no align strategy matched"); __x });
        }
        // java ret = true;
        ret = true;
      }
      // java else
      else {
        // java ret = false;
        ret = false;
      }
      // java if (null != this.mMainState.snapshot)
      if self.mMainState.snapshot.is_some() {
        // java this.commitSnapshot();
        self.commitSnapshot();
      }
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      // java return ret;
      return ret;
      */
    }

    // java public GridSizeCommit doGridAutoResize(long changeRange, Document document, boolean reTypo)
    pub fn doGridAutoResize__long_document_boolean(&mut self, changeRange: i64, document: Option<&dyn Document>, reTypo: bool) -> GridSizeCommit {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.doGridAutoResize(changeRange, document, reTypo, -1);
      return self.doGridAutoResize__long_document_boolean_int(changeRange, document, reTypo, -1);
      */
    }

    // java public GridSizeCommit doGridAutoResize(long changeRange, Document document, boolean reTypo, int typoCoreWidth)
    pub fn doGridAutoResize__long_document_boolean_int(&mut self, changeRange: i64, document: Option<&dyn Document>, reTypo: bool, typoCoreWidth: i32) -> GridSizeCommit {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mMainState.mGridSizeLayouter == null)
      if self.mMainState.mGridSizeLayouter.is_none() {
        // java if (!this.hasInit())
        if !self.hasInit() {
          // java this.mDocEnv.init();
          self.mDocEnv.init();
        }
        // java this.mMainState.mGridSizeLayouter = new GridSizeLayouter(this.mDocEnv, this.mViewEnv);
        self.mMainState.mGridSizeLayouter = GridSizeLayouter::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mMainState.mGridSizeLayouter.doGridAutoResize(changeRange, document, reTypo, -1);
      return self.mMainState.mGridSizeLayouter.doGridAutoResize(changeRange, document, reTypo, -1);
      */
    }

    // java public boolean flowViewRect(Rect rect)
    pub fn flowViewRect(&mut self, rect: Option<&Rect>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.hasInit())
      if !self.hasInit() {
        // java this.init();
        self.init();
      }
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java if (null != rect)
      if let Some(rect) = rect {
        // java this.mLayoutRect.set((Rect)rect);
        self.mLayoutRect.set__typorectcoreimpl((rect).downcast_ref::<Rect>().unwrap());
      }
      // java else
      else {
        // java this.mLayoutRect.setEmpty();
        self.mLayoutRect.setEmpty();
      }
      // java float scrollTop = (float)this.mViewEnv.getViewTop();
      let mut scrollTop: f32 = ((self.mViewEnv.getViewTop()) as f32);
      // java boolean visibleChange = rect == null;
      let mut visibleChange: bool = rect.is_none();
      // java float destBottom = scrollTop + Math.max((float)this.mViewEnv.getViewHeight(), (float)this.mViewEnv.getViewWidth() * (visibleChange ? 2.0F : 1.1F));
      let mut destBottom: f32 = scrollTop + (((self.mViewEnv.getViewHeight()) as f32)).max(((self.mViewEnv.getViewWidth()) as f32) * (if visibleChange { 2 } else { 1.1 }));
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java if (this.mEvents.isEmpty())
      if self.mEvents.is_empty() {
        // java if ((!visibleChange && snapshot.getLayoutPageSize() != 0 || this.checkLayoutComplete()) && (float)snapshot.getDocumentBottom() >= destBottom)
        if !visibleChange && snapshot.getLayoutPageSize() != 0 || self.checkLayoutComplete() && ((snapshot.getDocumentBottom()) as f32) >= destBottom {
          // java if (this.mLocksHelper.isLockDocument())
          if self.mLocksHelper.isLockDocument() {
            // java this.mLocksHelper.unlockDocument();
            self.mLocksHelper.unlockDocument();
          }
          // java return false;
          return false;
        }
      }
      // java else
      else {
        // java this.mMainLayouter.processEvents(this.mLayoutRect);
        self.mMainLayouter.processEvents(self.mLayoutRect);
        // java if (CacheVerify.ENABLE_VERIFY)
        if CacheVerify::ENABLE_VERIFY {
          // java CacheVerify.verify(snapshot);
          CacheVerify::verify(snapshot);
        }
      }
      // java int curLayoutHeight = this.getCurLayoutHeight();
      let mut curLayoutHeight: i32 = self.getCurLayoutHeight();
      // java int lastPageSize = snapshot.getLayoutPageSize();
      let mut lastPageSize: i32 = snapshot.getLayoutPageSize();
      // java while (!this.checkLayoutComplete() && ((float)curLayoutHeight < destBottom || snapshot.getLayoutPageSize() <= 0 || this.checkContinueFlowOnMultiColumn((float)curLayoutHeight, destBottom)))
      while !self.checkLayoutComplete() && ((curLayoutHeight) as f32) < destBottom || snapshot.getLayoutPageSize() <= 0 || self.checkContinueFlowOnMultiColumn(((curLayoutHeight) as f32), destBottom) {
        // java TypoLayoutPage page = this.mMainLayouter.flowPage();
        let mut page: TypoLayoutPage = self.mMainLayouter.flowPage();
        // java if (page != null)
        if true {
          // java if (this.mMainState.endEmbedPage != 0)
          if self.mMainState.endEmbedPage != 0 {
            // java int endEmbedBottom = TypoLayoutPage.getBottom(this.mMainState.endEmbedPage, snapshot);
            let mut endEmbedBottom: i32 = TypoLayoutPage::getBottom(self.mMainState.endEmbedPage, snapshot);
            // java this.mMainState.endEmbedPage = 0;
            self.mMainState.endEmbedPage = 0;
            // java this.mLayoutRect.set(page.getLeft(), page.getTop(), page.getRight(), Math.max(endEmbedBottom, page.getBottom()));
            self.mLayoutRect.set__typorectcoreimpl(page.getLeft(), page.getTop(), page.getRight(), (endEmbedBottom).max(page.getBottom()));
          }
          // java else if (this.mLayoutRect.isEmpty())
          else if self.mLayoutRect.isEmpty() {
            // java this.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(this.mLayoutRect, page.getPointer());
            self.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(self.mLayoutRect, page.getPointer());
          }
          // java else
          else {
            // java int top = this.mLayoutRect.top;
            let mut top: i32 = self.mLayoutRect.top;
            // java int bottom = this.mLayoutRect.bottom;
            let mut bottom: i32 = self.mLayoutRect.bottom;
            // java this.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(this.mLayoutRect, page.getPointer());
            self.mMainState.snapshot.getPosVisitor().estimateAddDirtyRectForGlobal(self.mLayoutRect, page.getPointer());
            // java this.mLayoutRect.top = Math.min(this.mLayoutRect.top, top);
            self.mLayoutRect.top = (self.mLayoutRect.top).min(top);
            // java this.mLayoutRect.bottom = Math.max(this.mLayoutRect.bottom, bottom);
            self.mLayoutRect.bottom = (self.mLayoutRect.bottom).max(bottom);
          }
        }
        // java curLayoutHeight = this.getCurLayoutHeight();
        curLayoutHeight = self.getCurLayoutHeight();
        // java if (this.mViewEnv.isPageMode() && snapshot.getLayoutPageSize() - lastPageSize > 64)
        if self.mViewEnv.isPageMode() && snapshot.getLayoutPageSize() - lastPageSize > 64 {
          // java this.mMainLayouter.swapToLocal(true);
          self.mMainLayouter.swapToLocal(true);
          // java lastPageSize = snapshot.getLayoutPageSize();
          lastPageSize = snapshot.getLayoutPageSize();
        }
      }
      // java if (this.checkLayoutComplete())
      if self.checkLayoutComplete() {
        // java this.mMainLayouter.ensureValidEndPage(this.mLayoutRect);
        self.mMainLayouter.ensureValidEndPage(self.mLayoutRect);
      }
      // java if (!this.mViewEnv.isWebLayoutMode())
      if !self.mViewEnv.isWebLayoutMode() {
        // java snapshot.setDocumentBottom(Math.max(this.mMainLayouter.getCurPageTop(), snapshot.getDocumentBottom()));
        snapshot.setDocumentBottom((self.mMainLayouter.getCurPageTop()).max(snapshot.getDocumentBottom()));
      }
      // java if (!this.mLayoutRect.isEmpty() && this.checkLayoutComplete() && this.mViewEnv.isPhoneReadMode())
      if !self.mLayoutRect.isEmpty() && self.checkLayoutComplete() && self.mViewEnv.isPhoneReadMode() {
        // java snapshot.setDocumentBottom(Math.min(this.mMainLayouter.getCurPageTop(), snapshot.getDocumentBottom()));
        snapshot.setDocumentBottom((self.mMainLayouter.getCurPageTop()).min(snapshot.getDocumentBottom()));
      }
      // java this.commitSnapshot();
      self.commitSnapshot();
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      // java if (2 == this.mViewEnv.getViewMode() && this.isRunOutMemory(1200))
      if 2 == self.mViewEnv.getViewMode() && self.isRunOutMemory(1200) {
        // java this.mViewEnv.getViewSettings().onClearAndReflow();
        self.mViewEnv.getViewSettings().onClearAndReflow();
      }
      // java if (null != rect)
      if let Some(rect) = rect {
        // java rect.set(this.mLayoutRect);
        rect.set__rect(self.mLayoutRect);
        // java if (rect.height() > 0 && rect.width() == 0)
        if rect.height() > 0 && rect.width() == 0 {
          // java rect.right = rect.left + 1;
          rect.right = rect.left + 1;
        }
      }
      // java return !this.mLayoutRect.isEmpty();
      return !self.mLayoutRect.isEmpty();
      */
    }

    // java private boolean checkContinueFlowOnMultiColumn(float curLayoutHeight, float destBottom)
    pub fn checkContinueFlowOnMultiColumn(&self, curLayoutHeight: f32, destBottom: f32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mMainState.snapshot.getPosVisitor().hasPageGrid())
      if self.mMainState.snapshot.getPosVisitor().hasPageGrid() {
        // java IPageGrid pageGrid = this.mMainState.snapshot.getPosVisitor().getPageGrid();
        let mut pageGrid: Box<dyn IPageGrid> = self.mMainState.snapshot.getPosVisitor().getPageGrid();
        // java if (pageGrid.isTrendToMultiColumn())
        if pageGrid.isTrendToMultiColumn() {
          // java int layoutWidth = pageGrid.getLayoutWidth();
          let mut layoutWidth: i32 = pageGrid.getLayoutWidth();
          // java int minPageWidth = pageGrid.getPagePositionMap().getMinPageWidth();
          let mut minPageWidth: i32 = pageGrid.getPagePositionMap().getMinPageWidth();
          // java int maxPageHight = pageGrid.getPagePositionMap().getMaxPageHeight();
          let mut maxPageHight: i32 = pageGrid.getPagePositionMap().getMaxPageHeight();
          // java if (curLayoutHeight - destBottom - (float)pageGrid.getGap() < (float)maxPageHight && layoutWidth - this.mLayoutRect.right - pageGrid.getGap() > minPageWidth)
          if curLayoutHeight - destBottom - ((pageGrid.getGap()) as f32) < ((maxPageHight) as f32) && layoutWidth - self.mLayoutRect.right - pageGrid.getGap() > minPageWidth {
            // java return true;
            return true;
          }
          // java return false;
          return false;
        }
      }
      // java return false;
      return false;
      */
    }

    // java  int getCurLayoutHeight()
    pub fn getCurLayoutHeight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return !this.mViewEnv.isPageMode() ? this.mMainLayouter.getCurPageTop() : this.mMainState.snapshot.getPosVisitor().getContentRect().getBottom();
      return (if !self.mViewEnv.isPageMode() { self.mMainLayouter.getCurPageTop() } else { self.mMainState.snapshot.getPosVisitor().getContentRect().getBottom() });
      */
    }

    // java public boolean flowPageInScrollDirection(Rect rect)
    pub fn flowPageInScrollDirection(&mut self, rect: Option<&Rect>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java boolean result = false;
      let mut result: bool = false;
      // java if (this.mViewEnv.isWebLayoutMode() && this.mViewEnv.isScrollUp())
      if self.mViewEnv.isWebLayoutMode() && self.mViewEnv.isScrollUp() {
        // java result = ((WebViewLayouter)this.mMainLayouter).flowPageWhenScrollUp(rect);
        result = (self.mMainLayouter).downcast_ref::<WebViewLayouter>().unwrap().flowPageWhenScrollUp(rect);
      }
      // java else if (!this.mMainLayouter.swapFromLocal())
      else if !self.mMainLayouter.swapFromLocal() {
        // java int oldPageSize = this.mMainState.snapshot.getLayoutPageSize();
        let mut oldPageSize: i32 = self.mMainState.snapshot.getLayoutPageSize();
        // java if (!this.mMainLayouter.prepareFlowInScrollDirection())
        if !self.mMainLayouter.prepareFlowInScrollDirection() {
          // java int newPageSize = this.mMainState.snapshot.getLayoutPageSize();
          let mut newPageSize: i32 = self.mMainState.snapshot.getLayoutPageSize();
          // java if (newPageSize == oldPageSize)
          if newPageSize == oldPageSize {
            // java if (this.mLocksHelper.isLockDocument())
            if self.mLocksHelper.isLockDocument() {
              // java this.mLocksHelper.unlockDocument();
              self.mLocksHelper.unlockDocument();
            }
            // java return false;
            return false;
          }
        }
        // java if (this.checkLayoutComplete())
        if self.checkLayoutComplete() {
          // java this.mMainLayouter.ensureValidEndPage(rect);
          self.mMainLayouter.ensureValidEndPage(rect);
          // java if (null != this.mMainState.snapshot)
          if self.mMainState.snapshot.is_some() {
            // java this.commitSnapshot();
            self.commitSnapshot();
          }
          // java if (this.mLocksHelper.isLockDocument())
          if self.mLocksHelper.isLockDocument() {
            // java this.mLocksHelper.unlockDocument();
            self.mLocksHelper.unlockDocument();
          }
          // java return false;
          return false;
        }
        // java result = this.flowPage(rect);
        result = self.flowPage(rect);
      }
      // java this.checkLayoutComplete();
      self.checkLayoutComplete();
      // java if (null != this.mMainState.snapshot)
      if self.mMainState.snapshot.is_some() {
        // java this.commitSnapshot();
        self.commitSnapshot();
      }
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      // java return result;
      return result;
      */
    }

    // java public void flowToDocumentEnd(Rect rect)
    pub fn flowToDocumentEnd(&mut self, rect: Option<&Rect>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mLocksHelper.isLockDocument())
      if !self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.lockDocument();
        self.mLocksHelper.lockDocument();
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java if (this.mMainLayouter.prepareFlowToDocumentEnd())
      if self.mMainLayouter.prepareFlowToDocumentEnd() {
        // java if (this.checkLayoutComplete())
        if self.checkLayoutComplete() {
          // java if (this.mLocksHelper.isLockDocument())
          if self.mLocksHelper.isLockDocument() {
            // java this.mLocksHelper.unlockDocument();
            self.mLocksHelper.unlockDocument();
          }
          // java if (null != this.mMainState.snapshot)
          if self.mMainState.snapshot.is_some() {
            // java this.commitSnapshot();
            self.commitSnapshot();
          }
          // java return;
          return;
        }
        // java this.flowPage(rect);
        self.flowPage(rect);
      }
      // java this.checkLayoutComplete();
      self.checkLayoutComplete();
      // java if (null != this.mMainState.snapshot)
      if self.mMainState.snapshot.is_some() {
        // java this.commitSnapshot();
        self.commitSnapshot();
      }
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      */
    }

    // java public void prepareFlowFromDocumentBegin()
    pub fn prepareFlowFromDocumentBegin(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainLayouter.prepareFlowFromDocumentBegin();
      self.mMainLayouter.prepareFlowFromDocumentBegin();
      */
    }

    // java public boolean hasFlowToBegin()
    pub fn hasFlowToBegin(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mViewEnv.hasFlowToBegin();
      return self.mViewEnv.hasFlowToBegin();
      */
    }

    // java public boolean hasFlowToEnd()
    pub fn hasFlowToEnd(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mViewEnv.hasFlowToEnd();
      return self.mViewEnv.hasFlowToEnd();
      */
    }

    // java public boolean flowAndDrawMath(Document doc, int cp, IMathDrawer drawer, float zoom)
    pub fn flowAndDrawMath(&mut self, doc: Option<&dyn Document>, cp: i32, drawer: Option<&mut dyn IMathDrawer>, zoom: f32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (drawer == null)
      if drawer.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (null == this.mMathDrawingLayouter)
        if self.mMathDrawingLayouter.is_none() {
          // java this.mMathDrawingLayouter = new MathDrawingLayouter(this.mDocEnv, this.mViewEnv);
          self.mMathDrawingLayouter = MathDrawingLayouter::new(self.mDocEnv, self.mViewEnv);
        }
        // java else
        else {
          // java this.mMathDrawingLayouter.setTextDocument(this.mDocEnv.textDocument);
          self.mMathDrawingLayouter.setTextDocument(self.mDocEnv.textDocument);
        }
        // java int drawing = this.mMathDrawingLayouter.flowMathDrawing(doc, cp);
        let mut drawing: i32 = self.mMathDrawingLayouter.flowMathDrawing(doc, cp);
        // java boolean rs = drawer.draw(drawing, zoom, this.mMathDrawingLayouter.getSnapshot());
        let mut rs: bool = drawer.draw(drawing, zoom, self.mMathDrawingLayouter.getSnapshot());
        // java this.mMathDrawingLayouter.clearCache();
        self.mMathDrawingLayouter.clearCache();
        // java return rs;
        return rs;
      }
      */
    }

    // java public void swapToLocal(boolean sync)
    pub fn swapToLocal(&mut self, sync: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainLayouter.swapToLocal(sync);
      self.mMainLayouter.swapToLocal(sync);
      // java if (this.mMainState.snapshot != null)
      if self.mMainState.snapshot.is_some() {
        // java this.commitSnapshot();
        self.commitSnapshot();
      }
      */
    }

    // java public void swapFromLocal()
    pub fn swapFromLocal(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainLayouter.swapFromLocal();
      self.mMainLayouter.swapFromLocal();
      */
    }

    // java public void clear()
    pub fn clear(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mMainLayouter)
      if let Some(mMainLayouter) = self.mMainLayouter.as_mut() {
        // java this.mMainLayouter.clearTypoDocument();
        mMainLayouter.clearTypoDocument();
      }
      */
    }

    // java public void discardEvents()
    pub fn discardEvents(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools.recycleEvents(this.mEvents);
      self.mPools.recycleEvents(self.mEvents);
      // java this.mEventCollector.clear();
      self.mEventCollector.clear();
      */
    }

    // java public void onIoLoadFinish(boolean hasAudioComment)
    pub fn onIoLoadFinish(&mut self, hasAudioComment: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mHasProcessedIOLoadFinishEvent = true;
      self.mHasProcessedIOLoadFinishEvent = true;
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java if (null != this.mMainLayouter)
      if let Some(mMainLayouter) = self.mMainLayouter.as_mut() {
        // java this.mMainLayouter.onIoLoadFinish(hasAudioComment);
        mMainLayouter.onIoLoadFinish(hasAudioComment);
      }
      // java LocksHelper locksHelper = this.getLocksHelper();
      let mut locksHelper: LocksHelper = self.getLocksHelper().clone();
      // java if (locksHelper != null && locksHelper.isLockDocument())
      if true && locksHelper.isLockDocument() {
        // java locksHelper.unlockDocument();
        locksHelper.unlockDocument();
      }
      */
    }

    // java public void setNotMergeTypoText()
    pub fn setNotMergeTypoText(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocEnv.notMergeTypoText = true;
      self.mDocEnv.notMergeTypoText = true;
      */
    }

    // java public boolean hasUpdateMsg()
    pub fn hasUpdateMsg(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return !this.mEvents.isEmpty();
      return !self.mEvents.is_empty();
      */
    }

    // java public boolean hasInit()
    pub fn hasInit(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mHasInit;
      return self.mHasInit;
      */
    }

    // java public boolean isUseDefaultSec()
    pub fn isUseDefaultSec(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mMainState.firstUseDefaultSec;
      return self.mMainState.firstUseDefaultSec;
      */
    }

    // java public boolean hasProcessedIOLoadFinishEvent()
    pub fn hasProcessedIOLoadFinishEvent(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mHasProcessedIOLoadFinishEvent;
      return self.mHasProcessedIOLoadFinishEvent;
      */
    }

    // java public void onAfterFirstRender()
    pub fn onAfterFirstRender(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mMainLayouter)
      if let Some(mMainLayouter) = self.mMainLayouter.as_mut() {
        // java this.mMainLayouter.onAfterFirstRender();
        mMainLayouter.onAfterFirstRender();
      }
      */
    }

    // java public void unInit()
    pub fn unInit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mHasInit = false;
      self.mHasInit = false;
      // java this.mHasProcessedIOLoadFinishEvent = false;
      self.mHasProcessedIOLoadFinishEvent = false;
      */
    }

    // java private void doInit()
    pub fn doInit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoDocument typoDocument = this.mTypoDocument;
      let mut typoDocument: TypoDocument = { let __x: TypoDocument = todo!("no align strategy matched"); __x };
      // java FontStyleFactory.setSmartFontSize(false);
      FontStyleFactory::setSmartFontSize(false);
      // java this.mPools = new Pools();
      self.mPools = Pools::new();
      // java this.mCacheCleaner = new CacheCleaner();
      self.mCacheCleaner = CacheCleaner::new();
      // java this.mFontStyleCacheFactory = new FontStyleCacheFactory();
      self.mFontStyleCacheFactory = FontStyleCacheFactory::new();
      // java this.mPropertyToolFactory = new PropertyToolFactory();
      self.mPropertyToolFactory = PropertyToolFactory::new();
      // java this.mLocksHelper = new LocksHelper(typoDocument.getDocument());
      self.mLocksHelper = LocksHelper::new(typoDocument.getDocument());
      // java this.mDocEnv = new DocumentEnv(typoDocument, this.mPools, this.mCacheCleaner, this.mPropertyToolFactory, this.mFontStyleCacheFactory, this.mViewEnv);
      self.mDocEnv = DocumentEnv::new(typoDocument, self.mPools, self.mCacheCleaner, self.mPropertyToolFactory, self.mFontStyleCacheFactory, self.mViewEnv);
      // java this.mCacheCleaner.init(this.mMainState, this.mDocEnv);
      self.mCacheCleaner.init(self.mMainState, self.mDocEnv);
      // java this.mMainState.footEndNoteCache = new FootNoteCache(this.mPools);
      self.mMainState.footEndNoteCache = FootNoteCache::new(self.mPools);
      // java this.mEvents = new KList();
      self.mEvents = KList::new();
      // java this.mEventCollector = new EventCollector(this.mPools, this.mEvents, this.mPropertyToolFactory, this.mFontStyleCacheFactory);
      self.mEventCollector = EventCollector::new(self.mPools, self.mEvents, self.mPropertyToolFactory, self.mFontStyleCacheFactory);
      */
    }

    // java public void resetForCrash()
    pub fn resetForCrash(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mHasInit = false;
      self.mHasInit = false;
      // java this.mHasProcessedIOLoadFinishEvent = false;
      self.mHasProcessedIOLoadFinishEvent = false;
      // java this.mMathDrawingLayouter = null;
      self.mMathDrawingLayouter = None;
      // java this.mFootEndNoteBalloonLayouter = null;
      self.mFootEndNoteBalloonLayouter = None;
      // java this.mMainLayouter = null;
      self.mMainLayouter = None;
      // java this.doInit();
      self.doInit();
      */
    }

    // java public void onFontsChange()
    pub fn onFontsChange(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFontStyleCacheFactory.clearCoreCharStyleCache();
      self.mFontStyleCacheFactory.clearCoreCharStyleCache();
      */
    }

    // java public void dump(StringBuilder sb)
    pub fn dump(&self, sb: &mut str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java sb.append("flowlayout hasInit=");
      sb.append("flowlayout hasInit=");
      // java sb.append(this.mHasInit);
      sb.append(self.mHasInit);
      // java if (null == this.mLocksHelper)
      if false {
        // java sb.append(" locksHelper is null");
        sb.append(" locksHelper is null");
      }
      // java else
      else {
        // java this.mLocksHelper.dump(sb);
        self.mLocksHelper.dump(sb);
      }
      // java if (this.mTypoDocument != null)
      if let Some(mTypoDocument) = self.mTypoDocument.as_ref() {
        // java this.mTypoDocument.dump(sb);
        mTypoDocument.dump__stringbuilder(sb);
      }
      // java if (this.mMainState.snapshot != null)
      if self.mMainState.snapshot.is_some() {
        // java sb.append(" mainState:");
        sb.append(" mainState:");
        // java this.mMainState.snapshot.dump(sb, false);
        self.mMainState.snapshot.dump(sb, false);
      }
      // java BlocksSwapManager.dump(sb);
      BlocksSwapManager::dump(sb);
      // java RestoreEnableException.dump(sb);
      RestoreEnableException::dump(sb);
      // java sb.append("\n");
      sb.append("\n");
      */
    }

    // java public void flowDocument(int maxPageCount)
    pub fn flowDocument__int(&mut self, maxPageCount: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java this.mMainLayouter.flowDocument(maxPageCount);
      self.mMainLayouter.flowDocument__int(maxPageCount);
      // java this.checkLayoutComplete();
      self.checkLayoutComplete();
      // java if (null != this.mMainState.snapshot)
      if self.mMainState.snapshot.is_some() {
        // java this.commitSnapshot();
        self.commitSnapshot();
      }
      */
    }

    // java public void flowDocument()
    pub fn flowDocument(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.flowDocument(Integer.MAX_VALUE);
      self.flowDocument__int(i32::MAX);
      */
    }

    // java public void makeBalloonCache()
    pub fn makeBalloonCache(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mViewEnv.getViewMode() != 2)
      if self.mViewEnv.getViewMode() != 2 {
        // java if (null == this.mMainState.snapshot)
        if self.mMainState.snapshot.is_none() {
          // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
          self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
        }
        // java this.mMainLayouter.makeBalloonCaches();
        self.mMainLayouter.makeBalloonCaches();
        // java if (null != this.mMainState.snapshot)
        if self.mMainState.snapshot.is_some() {
          // java this.commitSnapshot();
          self.commitSnapshot();
        }
      }
      */
    }

    // java public boolean flowFootnoteBalloon(int refCP, int layoutPage, int width)
    pub fn flowFootnoteBalloon(&mut self, refCP: i32, layoutPage: i32, width: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.flowFootEndnoteBalloon(refCP, layoutPage, width, true);
      return self.flowFootEndnoteBalloon(refCP, layoutPage, width, true);
      */
    }

    // java public boolean flowEndnoteBalloon(int refCP, int layoutPage, int width)
    pub fn flowEndnoteBalloon(&mut self, refCP: i32, layoutPage: i32, width: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.flowFootEndnoteBalloon(refCP, layoutPage, width, false);
      return self.flowFootEndnoteBalloon(refCP, layoutPage, width, false);
      */
    }

    // java public boolean flowFootEndnoteBalloon(int refCP, int layoutPage, int width, boolean isFootNote)
    pub fn flowFootEndnoteBalloon(&mut self, refCP: i32, layoutPage: i32, width: i32, isFootNote: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mFootEndNoteBalloonLayouter == null)
      if self.mFootEndNoteBalloonLayouter.is_none() {
        // java this.mFootEndNoteBalloonLayouter = new FootEndNoteBalloonLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
        self.mFootEndNoteBalloonLayouter = FootEndNoteBalloonLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
      }
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java this.mFootEndNoteBalloonLayouter.flowFootEndNote(refCP, layoutPage, width, isFootNote);
      self.mFootEndNoteBalloonLayouter.flowFootEndNote(refCP, layoutPage, width, isFootNote);
      // java this.commitSnapshot();
      self.commitSnapshot();
      // java return true;
      return true;
      */
    }

    // java public void flowDocument(IInterruptState interrupt)
    pub fn flowDocument__iinterruptstate(&mut self, interrupt: Option<&IInterruptState>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mMainState.snapshot)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.mTypoDocument.newSnapshot();
        self.mMainState.snapshot = self.mTypoDocument.newSnapshot();
      }
      // java this.mMainLayouter.flowDocument(Integer.MAX_VALUE, interrupt);
      self.mMainLayouter.flowDocument__int_iinterruptstate(i32::MAX, interrupt);
      // java if (null != this.mMainState.snapshot)
      if self.mMainState.snapshot.is_some() {
        // java this.commitSnapshot();
        self.commitSnapshot();
      }
      */
    }

    // java public boolean isRunOutMemory(int limit)
    pub fn isRunOutMemory(&self, limit: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean pause = false;
      let mut pause: bool = false;
      // java TypoSnapshot snapshot = this.mTypoDocument.getSnapshot();
      let mut snapshot: TypoSnapshot = self.mTypoDocument.getSnapshot();
      // java if (snapshot.getBlockSize() > limit)
      if snapshot.getBlockSize() > limit {
        // java pause = true;
        pause = true;
      }
      // java snapshot.release();
      snapshot.release();
      // java return pause;
      return pause;
      */
    }

    // java public LocateResult locateEmbedCommentScroll(int embedCommentCp)
    pub fn locateEmbedCommentScroll(&self, embedCommentCp: i32) -> Option<LocateResult> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mMainLayouter.locateEmbedCommentScroll(embedCommentCp);
      return self.mMainLayouter.locateEmbedCommentScroll(embedCommentCp);
      */
    }

    // java public void invalidCardState()
    pub fn invalidCardState(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainLayouter.invalidCardState();
      self.mMainLayouter.invalidCardState();
      */
    }

    pub fn new() -> Self {
      Self::default()
    }

    pub fn layout(&mut self) {
      self.snapshot_usable = true;
      self.version += 1;
    }

    pub fn invalidate(&mut self) {
      self.snapshot_usable = false;
    }

    pub fn can_paint(&self) -> bool {
      self.snapshot_usable
    }

    pub fn version(&self) -> i64 {
      self.version
    }

}
