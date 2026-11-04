#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.base.assertion.Assert
// CARGO_OFF_SURFACE use util::base::assertion::Assert::Assert;
// Java: import cn.wps.base.log.Log
// CARGO_OFF_SURFACE use util::base::log::Log::Log;
// Java: import cn.wps.collections.KList
// CARGO_OFF_SURFACE use util::collections::KList::KList;
// Java: import cn.wps.collections.textrope.ITextRope
// CARGO_OFF_SURFACE use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
// Java: import cn.wps.graphics.Rect
// CARGO_OFF_SURFACE use util::graphics::Rect::Rect;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.LocateTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::LocateTool::LocateTool;
// Java: import cn.wps.moffice.writer.cache.TypoCPRefUsage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoCPRefUsage::TypoCPRefUsage;
// Java: import cn.wps.moffice.writer.cache.TypoColumnPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoColumnPage::TypoColumnPage;
// Java: import cn.wps.moffice.writer.cache.TypoColumns
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns;
// Java: import cn.wps.moffice.drawing.Shape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoMemHelper
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoMemHelper::TypoMemHelper;
// Java: import cn.wps.moffice.writer.cache.TypoMemTag
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoMemTag::TypoMemTag;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
// Java: import cn.wps.moffice.writer.cache.TypoTextLine
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
// Java: import cn.wps.moffice.writer.cache.disk.CrackRanges
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::disk::CrackRanges::CrackRanges;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.ErrorCollector
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::ErrorCollector::ErrorCollector;
// Java: import cn.wps.moffice.writer.core.IInterruptState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::IInterruptState::IInterruptState;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.data.IChpxTable
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
// Java: import cn.wps.moffice.writer.data.IPapxTable
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkEnd
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::PLCBookmarkEnd;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkStart
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::PLCBookmarkStart;
// Java: import cn.wps.moffice.writer.data.PLCField
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCField::PLCField;
// Java: import cn.wps.moffice.writer.data.RANGE
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::RANGE::RANGE;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::CellInfo::CellInfo;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::RowInfo::RowInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfo::TableInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfos
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
// Java: import cn.wps.moffice.writer.data.value.FileFormat
// CARGO_OFF_SURFACE use util::moffice::writer::data::value::FileFormat::FileFormat;
// Java: import cn.wps.moffice.writer.layout.base.balloons.BalloonCacheLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::balloons::BalloonCacheLayouter::BalloonCacheLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingPositionTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingPositionTool::DrawingPositionTool;
// Java: import cn.wps.moffice.writer.layout.base.edit.Event
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::Event::Event;
// Java: import cn.wps.moffice.writer.layout.base.edit.EventHandler
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::EventHandler::EventHandler;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.PhoneViewFitInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PhoneViewFitInfo::PhoneViewFitInfo;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.SignableRange
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::SignableRange::SignableRange;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuff
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::GraphBuff;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.PredictSpacesInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::charSpace::PredictSpacesInfo::PredictSpacesInfo;
// Java: import cn.wps.moffice.writer.layout.base.tools.EnvTools
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::EnvTools::EnvTools;
// Java: import cn.wps.moffice.writer.layout.base.tools.LocksHelper
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::LocksHelper::LocksHelper;
// Java: import cn.wps.moffice.writer.layout.base.tools.MaxCp
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::MaxCp::MaxCp;
// Java: import cn.wps.moffice.writer.layout.phoneview.LineBreakAnalyzer
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::phoneview::LineBreakAnalyzer::LineBreakAnalyzer;
// Java: import cn.wps.moffice.writer.layout.webview.WebViewLayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::webview::WebViewLayoutState::WebViewLayoutState;
// Java: import cn.wps.moffice.writer.service.LayoutServiceCache
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::LayoutServiceCache::LayoutServiceCache;
// Java: import cn.wps.moffice.writer.service.LocateResult
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::service::LocateResult::LocateResult;
// Java: import cn.wps.moffice.writer.layout.base.TextLayouter
use crate::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;
// Java: import cn.wps.moffice.writer.layout.base.ILayouterWhileLoading
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::ILayouterWhileLoading::ILayouterWhileLoading;
// Java: import org.apache.poi.poifs.filesystem.Entry
// CARGO_OFF_SURFACE use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;


use core::marker::PhantomData as _;
// CARGO_OFF_SURFACE_TYPES
#[derive(Default)]
pub struct KList;
#[derive(Default)]
pub struct Rect;
#[derive(Default)]
pub struct LocksHelper;
#[derive(Default)]
pub struct Event;
#[derive(Default)]
pub struct LayoutServiceCache;
impl LayoutServiceCache {
    pub fn clearCache(&mut self) {}
}
#[derive(Default)]
pub struct EventHandler;
#[derive(Default)]
pub struct MaxCp;
#[derive(Default)]
pub struct BalloonCacheLayouter;
#[derive(Default)]
pub struct IPageEnumCallback;
#[derive(Default)]
pub struct TypoMemTag;
#[derive(Default)]
pub struct TypoCPRefUsage;
#[derive(Default)]
pub struct IInterruptState;
#[derive(Default)]
pub struct LocateResult;
#[derive(Default)]
pub struct Entry;
#[derive(Default)]
pub struct RowInfo;
#[derive(Default)]
pub struct StateSnapshot;
pub trait IContextHelper {}

// java public static final String TAG = null;
pub(crate) const TAG: &str = "";
// java public static final int PAGEVIEW_LAYOUTER = 0;
pub const PAGEVIEW_LAYOUTER: i32 = 0;
// java public static final int WEBVIEW_LAYOUTER = 1;
pub(crate) const WEBVIEW_LAYOUTER: i32 = 1;
// java public static final int PHONEVIEW_LAYOUTER = 2;
pub(crate) const PHONEVIEW_LAYOUTER: i32 = 2;
// java public static final int PHONEVIEWSCROLL_LAYOUTER = 3;
pub(crate) const PHONEVIEWSCROLL_LAYOUTER: i32 = 3;
// java public static final int SHARE_LAYOUTER = 4;
pub(crate) const SHARE_LAYOUTER: i32 = 4;

// java public abstract class MainLayouter extends TextLayouter implements EventHandler.CallBack, ILayouterWhileLoading
#[derive(Default)]
pub struct MainLayouter {
    // java TextLayouter super_
    pub(crate) super_: TextLayouter,
    // java protected boolean mFinishLoading;
    pub mFinishLoading: bool,
    // java protected boolean mFindIOError;
    pub(crate) mFindIOError: bool,
    // java protected LocksHelper mLocksHelper;
    pub(crate) mLocksHelper: Option<Box<LocksHelper>>,
    // java protected KList<Event> mEvents;
    pub(crate) mEvents: KList,
    // java protected LayoutServiceCache mLayoutServiceCache;
    pub(crate) mLayoutServiceCache: Option<Box<LayoutServiceCache>>,
    // java protected EventHandler mEventHandler;
    pub(crate) mEventHandler: Option<Box<EventHandler>>,
    // java protected MaxCp mMaxCp = new MaxCp();
    pub(crate) mMaxCp: MaxCp,
    // java protected LayoutState mLayoutState;
    pub mLayoutState: Option<Box<LayoutState>>,
    // java protected BalloonCacheLayouter balloonCacheLayouter;
    pub(crate) balloonCacheLayouter: Option<Box<BalloonCacheLayouter>>,
}

// java private class EnumPagesMakeBalloon extends CrackRanges.IPageEnumCallback
#[derive(Default)]
pub struct EnumPagesMakeBalloon {
    // java CrackRanges.IPageEnumCallback super_
    pub(crate) super_: IPageEnumCallback,
    // java final TypoMemTag mMemTag;
    pub(crate) mMemTag: TypoMemTag,
    // java protected TypoCPRefUsage mCPRefUsage;
    pub(crate) mCPRefUsage: Option<TypoCPRefUsage>,
}

// java public abstract class MainLayouter extends TextLayouter implements EventHandler.CallBack, ILayouterWhileLoading
impl MainLayouter {
    // java public  MainLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv, LocksHelper locksHelper, KList<Event> events, LayoutServiceCache layoutServiceCache)
    pub fn MainLayouter(mainState: &MainState, docEnv: &DocumentEnv, viewEnv: &ViewEnv, locksHelper: &LocksHelper, events: &KList, layoutServiceCache: &LayoutServiceCache) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super(mainState, docEnv, viewEnv);
      // java this.mEvents = events;
      Self { super_: TextLayouter::new(mainState, docEnv, viewEnv), mFinishLoading: Default::default(), mFindIOError: Default::default(), mLocksHelper: locksHelper, mEvents: events, mLayoutServiceCache: layoutServiceCache, mEventHandler: Default::default(), mMaxCp: Default::default(), mLayoutState: Default::default(), balloonCacheLayouter: Default::default() }
      // java this.mLocksHelper = locksHelper;
      // java this.mLayoutServiceCache = layoutServiceCache;
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.dispose();
      self.super_.dispose();
      // java this.mLocksHelper = null;
      self.mLocksHelper = None;
      // java this.mEvents = null;
      self.mEvents = None;
      // java this.mMaxCp = null;
      self.mMaxCp = None;
      // java if (this.mEventHandler != null)
      if let Some(mEventHandler) = self.mEventHandler.as_mut() {
        // java this.mEventHandler.dispose();
        mEventHandler.dispose();
        // java this.mEventHandler = null;
        mEventHandler = None;
      }
      // java if (this.balloonCacheLayouter != null)
      if let Some(balloonCacheLayouter) = self.balloonCacheLayouter.as_mut() {
        // java this.balloonCacheLayouter.dispose();
        balloonCacheLayouter.dispose();
        // java this.balloonCacheLayouter = null;
        balloonCacheLayouter = None;
      }
      */
    }

    // java public abstract TypoLayoutPage flowPage()
    pub fn flowPage(&mut self) -> Option<TypoLayoutPage> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::flowPage not implemented");
      */
    }

    // java public int waitRangeEnd(int maxCp)
    pub fn waitRangeEnd(&mut self, maxCp: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasFinishIoLoading())
      if self.hasFinishIoLoading() {
        // java return this.mLayoutState.document.getLength();
        return self.mLayoutState.document.getLength();
      }
      // java else
      else {
        // java this.mLayoutState.setRange(0L);
        self.mLayoutState.setRange__long(0);
        // java int rangeEnd = this.mMaxCp.waitIoCp(maxCp, this.mLayoutState, this.mDocEnv.textDocument, this.mLocksHelper);
        let mut rangeEnd: i32 = self.mMaxCp.waitIoCp(maxCp, self.mLayoutState, self.mDocEnv.textDocument, self.mLocksHelper);
        // java return rangeEnd;
        return rangeEnd;
      }
      */
    }

    // java public boolean acceptFirstCache(TypoSnapshot snapshot)
    pub fn acceptFirstCache(&self, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public abstract void flowCP(int var1, int var2, int var3)
    pub fn flowCP(&mut self, var1: i32, var2: i32, var3: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::flowCP not implemented");
      */
    }

    // java public abstract int flowToPage(int var1)
    pub fn flowToPage(&mut self, var1: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::flowToPage not implemented");
      */
    }

    // java public void flowDocument(int maxPageCount)
    pub fn flowDocument__int(&mut self, maxPageCount: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.prepareFlowFromDocumentBegin();
      self.prepareFlowFromDocumentBegin();
      // java LayoutTraceCollector.clear();
      LayoutTraceCollector::clear();
      // java LayoutTraceCollector.setTraceDocument(this.mLayoutState.document);
      LayoutTraceCollector::setTraceDocument(self.mLayoutState.document);
      // java int pageIndex = 0;
      let mut pageIndex: i32 = 0;
      // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":0}");
      android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":0}");
      // java LayoutTraceCollector.onPageStart(0, this.mLayoutState.pageEnv);
      LayoutTraceCollector::onPageStart__int_pageenv(0, self.mLayoutState.pageEnv);
      // java while (!this.isComplete(this.mLayoutState))
      while !self.isComplete(self.mLayoutState) {
        // java --maxPageCount;
        { let _tmp = maxPageCount; maxPageCount -= 1; _tmp };
        // java if (maxPageCount < 0)
        if maxPageCount < 0 {
          // java break;
          break;
        }
        // java this.flowPage();
        self.flowPage();
        // java ++pageIndex;
        { let _tmp = pageIndex; pageIndex += 1; _tmp };
        // java if (!this.isComplete(this.mLayoutState))
        if !self.isComplete(self.mLayoutState) {
          // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":" + pageIndex + "}");
          android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":" + pageIndex + "}");
          // java LayoutTraceCollector.onPageStart(pageIndex, this.mLayoutState.pageEnv);
          LayoutTraceCollector::onPageStart__int_pageenv(pageIndex, self.mLayoutState.pageEnv);
        }
      }
      // java if (this.isComplete(this.mLayoutState))
      if self.isComplete(self.mLayoutState) {
        // java this.ensureValidEndPage((Rect)null);
        self.ensureValidEndPage(None);
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int lastPage = TypoList.last(snapshot.getLayoutPageList(), snapshot);
      let mut lastPage: i32 = TypoList::last__int_typosnapshot(snapshot.getLayoutPageList(), snapshot);
      // java snapshot.setDocumentBottom(TypoLayoutPage.getBottom(lastPage, snapshot));
      snapshot.setDocumentBottom(TypoLayoutPage::getBottom(lastPage, snapshot));
      // java collectDrawingsForTrace(snapshot);
      Self::collectDrawingsForTrace(&snapshot);
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      */
    }

    // java public void flowDocument(int maxPageCount, IInterruptState interrupt)
    pub fn flowDocument__int_iinterruptstate(&mut self, maxPageCount: i32, interrupt: Option<&IInterruptState>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.prepareFlowFromDocumentBegin();
      self.prepareFlowFromDocumentBegin();
      // java LayoutTraceCollector.clear();
      LayoutTraceCollector::clear();
      // java LayoutTraceCollector.setTraceDocument(this.mLayoutState.document);
      LayoutTraceCollector::setTraceDocument(self.mLayoutState.document);
      // java int pageIndex2 = 0;
      let mut pageIndex2: i32 = 0;
      // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":0}");
      android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":0}");
      // java LayoutTraceCollector.onPageStart(0, this.mLayoutState.pageEnv);
      LayoutTraceCollector::onPageStart__int_pageenv(0, self.mLayoutState.pageEnv);
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
        // java ++pageIndex2;
        { let _tmp = pageIndex2; pageIndex2 += 1; _tmp };
        // java if (!this.isComplete())
        if !self.isComplete() {
          // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":" + pageIndex2 + "}");
          android.util.Log.d("WPS_PIPE", "{\"phase\":\"page_start\",\"page_index\":" + pageIndex2 + "}");
          // java LayoutTraceCollector.onPageStart(pageIndex2, this.mLayoutState.pageEnv);
          LayoutTraceCollector::onPageStart__int_pageenv(pageIndex2, self.mLayoutState.pageEnv);
        }
      }
      // java if (this.isComplete(this.mLayoutState))
      if self.isComplete(self.mLayoutState) {
        // java this.ensureValidEndPage((Rect)null);
        self.ensureValidEndPage(None);
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int lastPage = TypoList.last(snapshot.getLayoutPageList(), snapshot);
      let mut lastPage: i32 = TypoList::last__int_typosnapshot(snapshot.getLayoutPageList(), snapshot);
      // java snapshot.setDocumentBottom(TypoLayoutPage.getBottom(lastPage, snapshot));
      snapshot.setDocumentBottom(TypoLayoutPage::getBottom(lastPage, snapshot));
      // java collectDrawingsForTrace(snapshot);
      Self::collectDrawingsForTrace(&snapshot);
      // java if (this.mLocksHelper.isLockDocument())
      if self.mLocksHelper.isLockDocument() {
        // java this.mLocksHelper.unlockDocument();
        self.mLocksHelper.unlockDocument();
      }
      */
    }

    // java private static void collectDrawingsForTrace(TypoSnapshot snapshot)
    pub fn collectDrawingsForTrace(snapshot: &TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int layoutPageList = snapshot.getLayoutPageList();
      let mut layoutPageList: i32 = snapshot.getLayoutPageList();
      // java int pageCount = TypoList.size(layoutPageList, snapshot);
      let mut pageCount: i32 = TypoList::size__int_typosnapshot(layoutPageList, snapshot);
      let mut pi: i32 = 0;
      // java for(int pi = 0; pi < pageCount pi++)
      while pi < pageCount {
        // java int pLayoutPage = TypoList.quickGet(pi, layoutPageList, snapshot);
        let mut pLayoutPage: i32 = TypoList::quickGet__int_int_typosnapshot(pi, layoutPageList, snapshot);
        // java int pDrawingLayers = TypoLayoutPage.getDrawingLayers(pLayoutPage, snapshot);
        let mut pDrawingLayers: i32 = TypoLayoutPage::getDrawingLayers__int_typosnapshot(pLayoutPage, snapshot);
        // java if (pDrawingLayers == 0)
        if pDrawingLayers == 0 {
        }
        // java int pDrawings = TypoDrawingLayers.getDrawings(pDrawingLayers, snapshot);
        let mut pDrawings: i32 = TypoDrawingLayers::getDrawings(pDrawingLayers, snapshot);
        // java if (pDrawings == 0)
        if pDrawings == 0 {
        }
        // java int drawCount = TypoList.size(pDrawings, snapshot);
        let mut drawCount: i32 = TypoList::size__int_typosnapshot(pDrawings, snapshot);
        let mut di: i32 = 0;
        // java for(int di = 0; di < drawCount di++)
        while di < drawCount {
          // java int pDrawing = TypoList.quickGet(di, pDrawings, snapshot);
          let mut pDrawing: i32 = TypoList::quickGet__int_int_typosnapshot(di, pDrawings, snapshot);
          // java if (pDrawing == 0)
          if pDrawing == 0 {
          }
          // java float left = TypoObjBase.getLeft(pDrawing, snapshot);
          let mut left: f32 = TypoObjBase::getLeft__int_typosnapshot(pDrawing, snapshot);
          // java float top = TypoObjBase.getTop(pDrawing, snapshot);
          let mut top: f32 = TypoObjBase::getTop__int_typosnapshot(pDrawing, snapshot);
          // java float w = TypoObjBase.getRight(pDrawing, snapshot) - left;
          let mut w: f32 = TypoObjBase::getRight__int_typosnapshot(pDrawing, snapshot) - left;
          // java float h = TypoObjBase.getBottom(pDrawing, snapshot) - top;
          let mut h: f32 = TypoObjBase::getBottom__int_typosnapshot(pDrawing, snapshot) - top;
          // java boolean isEmbed = TypoDrawing.isEmbed(pDrawing, snapshot);
          let mut isEmbed: bool = TypoDrawing::isEmbed__int_typosnapshot(pDrawing, snapshot);
          // java Shape shape = TypoDrawing.getShape(pDrawing, snapshot);
          let mut shape: Shape = TypoDrawing::getShape__int_typosnapshot(pDrawing, snapshot);
          // java boolean behind = false;
          let mut behind: bool = false;
          // java String imageInfo = "cp:" + TypoDrawing.getCP(pDrawing, snapshot);
          let mut imageInfo: String = "cp:" + TypoDrawing::getCP__int_typosnapshot(pDrawing, snapshot);
          // java if (shape != null)
          if true {
            // java behind = shape.getBehindDocument();
            behind = shape.getBehindDocument();
            // java imageInfo = "shape:" + shape.getShapeID();
            imageInfo = "shape:" + shape.getShapeID();
          }
          // java String dtype;
          let mut dtype: String = String::new();
          // java if (isEmbed)
          if isEmbed {
            // java dtype = "inline";
            dtype = "inline";
          }
          // java else if (behind)
          else if behind {
            // java dtype = "anchor_behind";
            dtype = "anchor_behind";
          }
          // java else
          else {
            // java dtype = "anchor_float";
            dtype = "anchor_float";
          }
          // java LayoutTraceCollector.onDrawing(pi, left, top, w, h, dtype, imageInfo);
          LayoutTraceCollector::onDrawing(pi, left, top, w, h, dtype, imageInfo);
        }
      }
      */
    }

    // java public abstract boolean prepareFlowFromDocumentBegin()
    pub fn prepareFlowFromDocumentBegin(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::prepareFlowFromDocumentBegin not implemented");
      */
    }

    // java public abstract boolean prepareFlowInScrollDirection()
    pub fn prepareFlowInScrollDirection(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::prepareFlowInScrollDirection not implemented");
      */
    }

    // java public abstract boolean prepareFlowToDocumentEnd()
    pub fn prepareFlowToDocumentEnd(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::prepareFlowToDocumentEnd not implemented");
      */
    }

    // java public abstract boolean prepareToJumpToLastView()
    pub fn prepareToJumpToLastView(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::prepareToJumpToLastView not implemented");
      */
    }

    // java public void changePageSize()
    pub fn changePageSize(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.clearTypoDocument();
      self.clearTypoDocument();
      // java PageEnv pageEnv = this.mLayoutState.pageEnv;
      let mut pageEnv: PageEnv = self.mLayoutState.pageEnv;
      // java ViewEnv viewEnv = this.mViewEnv;
      let mut viewEnv: ViewEnv = self.mViewEnv;
      // java pageEnv.width = viewEnv.getViewWidth();
      pageEnv.width = viewEnv.getViewWidth();
      // java pageEnv.marginLeft = viewEnv.getWebViewMarginLeft();
      pageEnv.marginLeft = viewEnv.getWebViewMarginLeft();
      // java pageEnv.marginRight = viewEnv.getWebViewMarginRight();
      pageEnv.marginRight = viewEnv.getWebViewMarginRight();
      // java pageEnv.marginTop = viewEnv.getWebViewMarginTop();
      pageEnv.marginTop = viewEnv.getWebViewMarginTop();
      // java pageEnv.marginBottom = viewEnv.getWebViewMarginBottom();
      pageEnv.marginBottom = viewEnv.getWebViewMarginBottom();
      // java pageEnv.height = viewEnv.getViewHeight();
      pageEnv.height = viewEnv.getViewHeight();
      // java if (this.mDocEnv != null)
      if true {
        // java this.mDocEnv.clearFitInfo();
        self.mDocEnv.clearFitInfo();
      }
      */
    }

    // java public abstract boolean ensureValidEndPage(Rect var1)
    pub fn ensureValidEndPage(&mut self, var1: Option<&Rect>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("MainLayouter::ensureValidEndPage not implemented");
      */
    }

    // java public final boolean hasFinishIoLoading()
    pub fn hasFinishIoLoading(&self) -> bool {
      // java return this.mFinishLoading;
      return self.mFinishLoading;
    }

    // java public int getCurPageTop()
    pub fn getCurPageTop(&self) -> i32 {
      // java return this.mMainState.curPageTop;
      return self.super_.mMainState.curPageTop;
    }

    // java public int getPageHeight()
    pub fn getPageHeight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pageHeight = this.mLayoutState.pageEnv.height;
      let mut pageHeight: i32 = self.mLayoutState.pageEnv.height;
      // java if (pageHeight <= 0)
      if pageHeight <= 0 {
        // java pageHeight = this.mViewEnv.getViewHeight();
        pageHeight = self.mViewEnv.getViewHeight();
      }
      // java return pageHeight;
      return pageHeight;
      */
    }

    // java public void swapToLocal(boolean sync)
    pub fn swapToLocal(&mut self, _sync: bool) {
      // rust: no-stmt
    }

    // java public boolean swapFromLocal()
    pub fn swapFromLocal(&self) -> bool {
      // java return false;
      return false;
    }

    // java public void clearRevisionCache()
    pub fn clearRevisionCache(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.balloonCacheLayouter)
      if let Some(balloonCacheLayouter) = self.balloonCacheLayouter.as_mut() {
        // java this.balloonCacheLayouter.clear();
        balloonCacheLayouter.clear();
      }
      */
    }

    // java public void onScroll(int scrollTop)
    pub fn onScroll__int(&mut self, scrollTop: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.onScroll(scrollTop, false);
      self.onScroll__int_boolean(scrollTop, false);
      */
    }

    // java public void onScroll(int scrollTop, boolean needNodifyListener)
    pub fn onScroll__int_boolean(&mut self, scrollTop: i32, needNodifyListener: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mViewEnv.onScroll(scrollTop);
      self.mViewEnv.onScroll(scrollTop);
      */
    }

    // java public void clearTypoDocument()
    pub fn clearTypoDocument(&mut self) {
      // java if (null == this.mMainState.snapshot)
      // rust: TypoSnapshot is a value type on this compile surface (Java null → owned Default)
      if false {
        // java this.mMainState.snapshot = this.mDocEnv.typoDocument.newSnapshot();
        self.super_.mMainState.snapshot = Default::default();
      }
      // java this.mMainState.snapshot.reset();
      self.super_.mMainState.snapshot.reset();
      // java this.mMainState.curPageTop = 0;
      self.super_.mMainState.curPageTop = 0;
      // java this.mViewEnv.beforeClearDocument();
      self.mViewEnv.beforeClearDocument();
      // java if (null != this.mLayoutServiceCache)
      if let Some(mLayoutServiceCache) = self.mLayoutServiceCache.as_mut() {
        // java this.mLayoutServiceCache.clearCache();
        mLayoutServiceCache.clearCache();
      }
    }

    // java public void clearForStructure()
    pub fn clearForStructure(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.clearTypoDocument();
      self.clearTypoDocument();
      // java this.onModifyFirst(0, true);
      self.onModifyFirst(0, true);
      */
    }

    // java protected void postFlow(LayoutState layoutState)
    pub fn postFlow(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java EnvTools.clearPageForCellEnvList(layoutState.cellStateList, this.mDocEnv.pools);
      EnvTools::clearPageForCellEnvList(layoutState.cellStateList, self.mDocEnv.pools);
      // java super.postFlow(layoutState);
      self.super_.postFlow(layoutState);
      */
    }

    // java private static boolean isInTable(PropertySet paraProp)
    pub fn isInTable(paraProp: &PropertySet) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return paraProp.getBoolean(223, false);
      return paraProp.getBoolean__int_boolean(223, false);
      */
    }

    // java public void processEvents(Rect layoutRect)
    pub fn processEvents(&mut self, layoutRect: Option<&Rect>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Pools pools = this.mDocEnv.pools;
      let mut pools: Pools = self.mDocEnv.pools;
      // java if (this.mEventHandler != null)
      if let Some(mEventHandler) = self.mEventHandler.as_mut() {
        // java this.mEventHandler.onBeforeFirst();
        mEventHandler.onBeforeFirst();
      }
      // java while (!this.mEvents.isEmpty())
      while !self.mEvents.is_empty() {
        // java Event event = (Event)this.mEvents.removeFirst();
        let mut event: Event = (self.mEvents.removeFirst()).downcast_ref::<Event>().unwrap();
        // java this.mEventHandler.update(event, this.mMainState.snapshot);
        self.mEventHandler.update(event, self.mMainState.snapshot);
        // java pools.recycleEvent(event);
        pools.recycleEvent(event);
        // java if (4 == event.type)
        if 4 == event.r#type {
          // java pools.recycleEvents(this.mEvents);
          pools.recycleEvents(self.mEvents);
        }
      }
      */
    }

    // java protected int getBeginFlowCP(IPapxTable.Entry para)
    pub fn getBeginFlowCP__entry(&self, para: &Entry) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getBeginFlowCP(para, -1);
      return self.getBeginFlowCP__entry_int(para, -1);
      */
    }

    // java protected int getBeginFlowCP(IPapxTable.Entry para, int flowCp)
    pub fn getBeginFlowCP__entry_int(&self, para: &Entry, flowCp: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java while (true)
        // java if (!para.isHead())
        if !para.isHead() {
          // java int paraStart = para.position();
          let mut paraStart: i32 = para.position();
          // java if (paraStart == 0)
          if paraStart == 0 {
            // java if (flowCp > paraStart && this.allowCutFlow(para, flowCp))
            if flowCp > paraStart && self.allowCutFlow(para, flowCp) {
              // java return flowCp;
              return flowCp;
            }
            // java return 0;
            return 0;
          }
          // java if (this.needSkip(para, paraStart))
          if self.needSkip(para, paraStart) {
            // java para = para.getPrev();
            para = para.getPrev();
            // java flowCp = -1;
            flowCp = -1;
            // java continue;
            continue;
          }
          // java if (flowCp > paraStart && this.allowCutFlow(para, flowCp))
          if flowCp > paraStart && self.allowCutFlow(para, flowCp) {
            // java return flowCp;
            return flowCp;
          }
          // java return paraStart;
          return paraStart;
        }
        // java return 0;
        return 0;
      */
    }

    // java private boolean allowCutFlow(IPapxTable.Entry para, int flowCp)
    pub fn allowCutFlow(&self, para: &Entry, flowCp: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (isInTable(para.getProperty()))
      if Self::isInTable(&para.getProperty()) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PLCField.FieldTriple fieldTriple = this.mLayoutState.document.getPLCField().getContainingField(flowCp);
        let mut fieldTriple: PLCField::FieldTriple = self.mLayoutState.document.getPLCField().getContainingField(flowCp);
        // java if (null != fieldTriple)
        if true {
          // java return false;
          return false;
        }
        // java else
        else {
          // java int paraStart = para.position();
          let mut paraStart: i32 = para.position();
          // java ITextRope textRope = this.mLayoutState.document.getTextRope();
          let mut textRope: Box<dyn ITextRope> = self.mLayoutState.document.getTextRope();
          // java if (this.hasAntherChar(textRope.charAt(flowCp)))
          if self.hasAntherChar(textRope.charAt(flowCp)) {
            // java return false;
            return false;
          }
          // java else
          else {
            // java if (flowCp > paraStart)
            if flowCp > paraStart {
              // java --flowCp;
              { let _tmp = flowCp; flowCp -= 1; _tmp };
              // java if (this.hasAntherChar(textRope.charAt(flowCp)))
              if self.hasAntherChar(textRope.charAt(flowCp)) {
                // java return false;
                return false;
              }
            }
            // java return true;
            return true;
          }
        }
      }
      */
    }

    // java private boolean hasAntherChar(char ch)
    pub fn hasAntherChar(&self, ch: u16) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean result = ch == 7 || ch == '\b' || ch == 5 || ch == 1 || ch == '\r' || ch == 19 || ch == 21 || ch == '\f' || ch == 11 || ch == '\n' || ch == 14;
      let mut result: bool = ch == 7 || ch == '\u{0008}' || ch == 5 || ch == 1 || ch == '\r' || ch == 19 || ch == 21 || ch == '\u{000c}' || ch == 11 || ch == '\n' || ch == 14;
      // java return result;
      return result;
      */
    }

    // java protected boolean needSkip(IPapxTable.Entry para, int paraStart)
    pub fn needSkip(&self, para: &Entry, paraStart: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IPapxTable.Entry prevPara = para.getPrev();
      let mut prevPara: Box<dyn Entry> = para.getPrev();
      // java if (prevPara.isHead())
      if prevPara.isHead() {
        // java return false;
        return false;
      }
      // java else if (isInTable(para.getProperty()))
      else if Self::isInTable(&para.getProperty()) {
        // java return isInTable(prevPara.getProperty());
        return Self::isInTable(&prevPara.getProperty());
      }
      // java else if (isInTable(prevPara.getProperty()))
      else if Self::isInTable(&prevPara.getProperty()) {
        // java return false;
        return false;
      }
      // java else if (this.isRevisionDelPara(paraStart - 1))
      else if self.isRevisionDelPara(paraStart - 1) {
        // java return true;
        return true;
      }
      // java else if (para.length() == 1 && prevPara.length() == 1)
      else if para.length() == 1 && (prevPara.len() as i32) == 1 {
        // java return true;
        return true;
      }
      // java else
      else {
        // java PLCField.FieldTriple fieldTriple = this.mLayoutState.document.getPLCField().getContainingField(paraStart);
        let mut fieldTriple: PLCField::FieldTriple = self.mLayoutState.document.getPLCField().getContainingField(paraStart);
        // java if (null != fieldTriple && fieldTriple.getStartCP() < paraStart)
        if true && fieldTriple.getStartCP() < paraStart {
          // java return true;
          return true;
        }
        // java else
        else {
          // java SignableRange range = (SignableRange)this.mDocEnv.getPhoneViewFitInfo().getItemByStartCp(paraStart, SignableRange.class);
          let mut range: SignableRange = (self.mDocEnv.getPhoneViewFitInfo().getItemByStartCp(paraStart, todo!())).downcast_ref::<SignableRange>().unwrap();
          // java return range != null && paraStart > range.mStartCp;
          return true && paraStart > range.mStartCp;
        }
      }
      */
    }

    // java private boolean isRevisionDelPara(int prepParaEndCP)
    pub fn isRevisionDelPara(&self, prepParaEndCP: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (prepParaEndCP < 0)
      if prepParaEndCP < 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java IChpxTable.Entry run = this.mLayoutState.document.getChpxTable().seek(prepParaEndCP);
        let mut run: Box<dyn Entry> = self.mLayoutState.document.getChpxTable().seek(prepParaEndCP);
        // java PropertySet prop = run.getProperty();
        let mut prop: PropertySet = run.getProperty();
        // java return null != prop && prop.contains(45);
        return true && prop.contains(45);
      }
      */
    }

    // java protected void onNewRangeEnd(int lastRangeEnd, int rangeEnd)
    pub fn onNewRangeEnd(&mut self, lastRangeEnd: i32, rangeEnd: i32) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

    // java public int calcMaxValidRangeEnd(int curCP)
    pub fn calcMaxValidRangeEnd(&self, curCP: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int rangeEnd = 0;
      let mut rangeEnd: i32 = 0;
      // java int lastRangeEnd = this.mLayoutState.getRangeEnd();
      let mut lastRangeEnd: i32 = self.mLayoutState.getRangeEnd();
      // java Document doc = this.mLayoutState.document;
      let mut doc: Box<dyn Document> = self.mLayoutState.document;
      // java TextDocument textDocument = this.mDocEnv.textDocument;
      let mut textDocument: TextDocument = self.mDocEnv.textDocument;
      // java if (!textDocument.isInIOState())
      if !textDocument.isInIOState() {
        // java this.mLocksHelper.extendLockDocument();
        self.mLocksHelper.extendLockDocument();
        // java rangeEnd = doc.getLength();
        rangeEnd = doc.getLength();
        // java this.mFinishLoading = true;
        self.mFinishLoading = true;
        // java this.onNewRangeEnd(lastRangeEnd, rangeEnd);
        self.onNewRangeEnd(lastRangeEnd, rangeEnd);
        // java this.mLayoutState.setRangeEnd(rangeEnd);
        self.mLayoutState.setRangeEnd(rangeEnd);
        // java return rangeEnd;
        return rangeEnd;
      }
      // java else if (curCP < lastRangeEnd && textDocument.isInIOState())
      else if curCP < lastRangeEnd && textDocument.isInIOState() {
        // java return lastRangeEnd;
        return lastRangeEnd;
      }
      // java else if (this.mFindIOError)
      else if self.mFindIOError {
        // java return lastRangeEnd;
        return lastRangeEnd;
      }
      // java else
      else {
        // java rangeEnd = this.mMaxCp.waitIoCp(lastRangeEnd, this.mLayoutState, textDocument, this.mLocksHelper);
        rangeEnd = self.mMaxCp.waitIoCp(lastRangeEnd, self.mLayoutState, textDocument, self.mLocksHelper);
        // java ErrorCollector errorCollector = doc.getErrorCollector();
        let mut errorCollector: ErrorCollector = doc.getErrorCollector();
        // java if (errorCollector.hasError())
        if errorCollector.hasError() {
          // java rangeEnd = Math.min(errorCollector.getMinErrorCp(), rangeEnd);
          rangeEnd = (errorCollector.getMinErrorCp()).min(rangeEnd);
          // java this.mFindIOError = true;
          self.mFindIOError = true;
        }
        // java this.onNewRangeEnd(lastRangeEnd, rangeEnd);
        self.onNewRangeEnd(lastRangeEnd, rangeEnd);
        // java if (!textDocument.isInIOState() && !this.mFinishLoading && rangeEnd == doc.getLength())
        if !textDocument.isInIOState() && !self.mFinishLoading && rangeEnd == doc.getLength() {
          // java Log.i(TAG, "NOT in IOState, the last loading, rangeEnd: " + rangeEnd);
          Log::i__string_string(TAG, "NOT in IOState, the last loading, rangeEnd: " + rangeEnd);
          // java this.mFinishLoading = true;
          self.mFinishLoading = true;
        }
        // java if (rangeEnd > 0)
        if rangeEnd > 0 {
          // java this.mLayoutState.setRangeEnd(rangeEnd);
          self.mLayoutState.setRangeEnd(rangeEnd);
        }
        // java return rangeEnd;
        return rangeEnd;
      }
      */
    }

    // java public int waitToCp(int waitCp)
    pub fn waitToCp(&mut self, waitCp: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mMaxCp.waitIoCp(waitCp, this.mLayoutState, this.mDocEnv.textDocument, this.mLocksHelper);
      return self.mMaxCp.waitIoCp(waitCp, self.mLayoutState, self.mDocEnv.textDocument, self.mLocksHelper);
      */
    }

    // java protected void initGraphBuffForLoading()
    pub fn initGraphBuffForLoading(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int rangeEnd = this.calcMaxValidRangeEnd(this.mLayoutState.cp);
      let mut rangeEnd: i32 = self.calcMaxValidRangeEnd(self.mLayoutState.cp);
      // java GraphBuff graphBuff = this.mGraphBuff;
      let mut graphBuff: GraphBuff = self.mGraphBuff.unwrap();
      // java if (rangeEnd > 0)
      if rangeEnd > 0 {
        // java this.mLayoutState.setRangeEnd(rangeEnd);
        self.mLayoutState.setRangeEnd(rangeEnd);
        // java this.mLayoutState.extendsCoreContent(rangeEnd, this.mViewEnv.isRevisionFinal());
        self.mLayoutState.extendsCoreContent(rangeEnd, self.mViewEnv.isRevisionFinal());
        // java graphBuff.initFilter(this.mLayoutState.document, this.mLayoutState.cp, rangeEnd, this.mLayoutState);
        graphBuff.initFilter(self.mLayoutState.document, self.mLayoutState.cp, rangeEnd, self.mLayoutState);
      }
      */
    }

    // java protected void rollbackLines(LayoutState layoutState)
    pub fn rollbackLines(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DrawingPositionTool.updateRelParaQuasiFloatsVertPos(this.mMainState.layoutPageRelParaQuasiFloats, this.mMainState);
      DrawingPositionTool::updateRelParaQuasiFloatsVertPos(self.mMainState.layoutPageRelParaQuasiFloats, self.mMainState);
      // java super.rollbackLines(layoutState);
      self.super_.rollbackLines(layoutState);
      */
    }

    // java private boolean reflowComments(boolean hasAudioComment)
    pub fn reflowComments(&mut self, hasAudioComment: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TextDocument textDoc = this.mDocEnv.textDocument;
      let mut textDoc: TextDocument = self.mDocEnv.textDocument;
      // java if (hasAudioComment || FileFormat.FF_DOC == textDoc.ff && !textDoc.getTrackChange())
      if hasAudioComment || FileFormat::FF_DOC == textDoc.ff && !textDoc.getTrackChange() {
        // java Document mainDoc = textDoc.getMainDocument();
        let mut mainDoc: Box<dyn Document> = textDoc.getMainDocument();
        // java PLCBookmarkStart commentStarts = mainDoc.getCommentStarts();
        let mut commentStarts: PLCBookmarkStart = mainDoc.getCommentStarts();
        // java PLCBookmarkEnd commentEnds = mainDoc.getCommentEnds();
        let mut commentEnds: PLCBookmarkEnd = mainDoc.getCommentEnds();
        // java return !commentStarts.isEmpty() && !commentEnds.isEmpty();
        return !commentStarts.is_empty() && !commentEnds.is_empty();
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java protected void reset()
    pub fn reset(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPageLayouter.reset();
      self.mPageLayouter.reset();
      // java this.mParaLayouter.reset();
      self.mParaLayouter.reset();
      // java this.mGraphBuff.clear(this.mLayoutState);
      self.mGraphBuff.clear(self.mLayoutState);
      // java this.mDocEnv.pools.recycleTableCellLayoutStates(this.mLayoutState.cellStateList);
      self.mDocEnv.pools.recycleTableCellLayoutStates(self.mLayoutState.cellStateList);
      */
    }

    // java public void resetForInit()
    pub fn resetForInit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFinishLoading = false;
      self.mFinishLoading = false;
      // java this.mFindIOError = false;
      self.mFindIOError = false;
      // java this.mLayoutState.setRange(0, 0);
      self.mLayoutState.setRange__int_int(0, 0);
      // java this.reset();
      self.reset();
      // java this.changePageSize();
      self.changePageSize();
      // java this.mGraphBuff.resetForInit();
      self.mGraphBuff.resetForInit();
      */
    }

    // java public abstract int getType()
    pub fn getType(&self) -> i32 {
      panic!("MainLayouter::getType not implemented");
    }

    // java protected void doOnIoLoadFinish()
    pub fn doOnIoLoadFinish(&mut self) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

    // java public void onIoLoadFinish(boolean hasAudioComment)
    pub fn onIoLoadFinish(&mut self, hasAudioComment: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mEvents.addAll(this.mMainState.eventsForReflowField);
      self.mEvents.addAll(self.mMainState.eventsForReflowField);
      // java if (this.mFindIOError || this.mDocEnv.textDocument.hasDocField())
      if self.mFindIOError || self.mDocEnv.textDocument.hasDocField() {
        // java Event even = this.mDocEnv.pools.obtainEvent();
        let mut even: Event = self.mDocEnv.pools.obtainEvent();
        // java even.cp = 0;
        even.cp = 0;
        // java even.document = this.mLayoutState.document;
        even.document = self.mLayoutState.document;
        // java even.length = even.document.getLength();
        (even.len() as i32) = even.document.getLength();
        // java even.type = 4;
        even.r#type = 4;
        // java this.mEvents.add(even);
        self.mEvents.add(even);
      }
      // java this.doOnIoLoadFinish();
      self.doOnIoLoadFinish();
      // java this.reflowComments(hasAudioComment);
      self.reflowComments(hasAudioComment);
      */
    }

    // java public boolean hasFindIOError()
    pub fn hasFindIOError(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mFindIOError;
      return self.mFindIOError;
      */
    }

    // java public void onAfterFirstRender()
    pub fn onAfterFirstRender(&mut self) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

    // java private boolean isFirstLineTableHeader(int line)
    pub fn isFirstLineTableHeader(&self, line: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (0 == line)
      if 0 == line {
        // java return false;
        return false;
      }
      // java else
      else {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java if (5 != TypoMemoryObj.getType(line, snapshot))
        if 5 != TypoMemoryObj::getType__int_typosnapshot(line, snapshot) {
          // java return false;
          return false;
        }
        // java else if (!TypoTableRow.isTableHeader(line, snapshot))
        else if !TypoTableRow::isTableHeader__int_typosnapshot(line, snapshot) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java int linesPage = TypoObjBase.getPage(line, snapshot);
          let mut linesPage: i32 = TypoObjBase::getPage__int_typosnapshot(line, snapshot);
          // java if (TypoList.first(TypoPage.getLines(linesPage, snapshot), snapshot) != line)
          if TypoList::first__int_typosnapshot(TypoPage::getLines__int_typosnapshot(linesPage, snapshot), snapshot) != line {
            // java return false;
            return false;
          }
          // java else
          else {
            // java if (14 == TypoMemoryObj.getType(linesPage, snapshot))
            if 14 == TypoMemoryObj::getType__int_typosnapshot(linesPage, snapshot) {
              // java int columns = TypoPage.getParent(linesPage, snapshot);
              let mut columns: i32 = TypoPage::getParent(linesPage, snapshot);
              // java if (TypoColumns.getColumnPage(0, columns, snapshot) != linesPage)
              if TypoColumns::getColumnPage__int_int_typosnapshot(0, columns, snapshot) != linesPage {
                // java return false;
                return false;
              }
            }
            // java return true;
            return true;
          }
        }
      }
      */
    }

    // java private boolean isNullOrFirstLineTableHeader(int line)
    pub fn isNullOrFirstLineTableHeader(&self, line: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (0 == line)
      if 0 == line {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return this.isFirstLineTableHeader(line);
        return self.isFirstLineTableHeader(line);
      }
      */
    }

    // java protected int getLastValidLineInPage(int line, int lineType, TypoSnapshot snapshot)
    pub fn getLastValidLineInPage(&self, line: i32, lineType: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (0 != line && 13 == TypoMemoryObj.getType(line, snapshot))
      if 0 != line && 13 == TypoMemoryObj::getType__int_typosnapshot(line, snapshot) {
        // java TypoColumnPage columnPage = null;
        let mut columnPage: TypoColumnPage = Default::default();
        // java int pColumnsLine = line;
        let mut pColumnsLine: i32 = line;
        let mut index: i32 = TypoColumns::getColumnsCount__int_typosnapshot(line, snapshot) - 1;
        // java for(int index = TypoColumns.getColumnsCount(line, snapshot) - 1; index >= 0 --index)
        while index >= 0 {
          // java int column = TypoColumns.getColumnPage(index, pColumnsLine, snapshot);
          let mut column: i32 = TypoColumns::getColumnPage__int_int_typosnapshot(index, pColumnsLine, snapshot);
          // java if (null == columnPage)
          if false {
            // java columnPage = snapshot.getTypoTools().obtainTypoColumnPage(column);
            columnPage = snapshot.getTypoTools().obtainTypoColumnPage(column);
          }
          // java else
          else {
            // java columnPage.assignment(column, snapshot);
            columnPage.assignment(column, snapshot);
          }
          // java line = this.getLastValidLineInPage(columnPage);
          line = self.getLastValidLineInPage({ let __x: i32 = todo!("no align strategy matched"); __x });
          // java if (0 != line)
          if 0 != line {
            // java return line;
            return line;
          }
        }
        // java if (null != columnPage)
        if true {
          // java snapshot.getTypoTools().recycle((TypoMemoryObj)columnPage);
          snapshot.getTypoTools().recycle((columnPage).downcast_ref::<TypoMemoryObj>().unwrap());
        }
      }
      // java return super.getLastValidLineInPage(line, lineType, snapshot);
      return self.super_.getLastValidLineInPage__int_int_typosnapshot(line, lineType, snapshot);
      */
    }

    // java protected int getRedundantDrawingLimitCP(int lastVisibleLine, int drawing, int anchorCP, LayoutState layoutState)
    pub fn getRedundantDrawingLimitCP(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32, layoutState: &LayoutState) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.isRedundantDrawing(lastVisibleLine, drawing, anchorCP))
      if !self.isRedundantDrawing(lastVisibleLine, drawing, anchorCP) {
        // java return Integer.MIN_VALUE;
        return i32::MIN;
      }
      // java else
      else {
        // java return TypoDrawing.isLayoutNotInCell(drawing, this.mMainState.snapshot) ? this.getLayoutNotInCellDrawingLimitCP(anchorCP) : anchorCP;
        return (if TypoDrawing::isLayoutNotInCell__int_typosnapshot(drawing, self.mMainState.snapshot) { self.getLayoutNotInCellDrawingLimitCP(anchorCP) } else { anchorCP });
      }
      */
    }

    // java private boolean isRedundantDrawing(int lastVisibleLine, int drawing, int anchorCP)
    pub fn isRedundantDrawing(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pageEndCP = this.mLayoutState.cp;
      let mut pageEndCP: i32 = self.mLayoutState.cp;
      // java if (this.mLayoutState.cellStateList.isEmpty())
      if self.mLayoutState.cellStateList.is_empty() {
        // java return anchorCP >= pageEndCP;
        return anchorCP >= pageEndCP;
      }
      // java else
      else {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java if (anchorCP == pageEndCP && 7 == TypoDrawing.getDrawingType(drawing, snapshot))
        if anchorCP == pageEndCP && 7 == TypoDrawing::getDrawingType__int_typosnapshot(drawing, snapshot) {
          // java return false;
          return false;
        }
        // java else if (this.isNullOrFirstLineTableHeader(lastVisibleLine))
        else if self.isNullOrFirstLineTableHeader(lastVisibleLine) {
          // java return true;
          return true;
        }
        // java else if (TypoDrawing.isLayoutNotInCell(drawing, snapshot))
        else if TypoDrawing::isLayoutNotInCell__int_typosnapshot(drawing, snapshot) {
          // java int line = LocateTool.quickFindRangeObj(this.mLayoutState.curPage.getPointer(), anchorCP, false, snapshot);
          let mut line: i32 = LocateTool::quickFindRangeObj(self.mLayoutState.curPage.getPointer(), anchorCP, false, snapshot);
          // java return 0 != line;
          return 0 != line;
        }
        // java else
        else {
          // java int lastVisibleLineType = TypoMemoryObj.getType(lastVisibleLine, snapshot);
          let mut lastVisibleLineType: i32 = TypoMemoryObj::getType__int_typosnapshot(lastVisibleLine, snapshot);
          // java if (lastVisibleLineType == 3 && anchorCP < TypoTextLine.getStartCP(lastVisibleLine, snapshot))
          if lastVisibleLineType == 3 && anchorCP < TypoTextLine::getStartCP__int_typosnapshot(lastVisibleLine, snapshot) {
            // java return false;
            return false;
          }
          // java else if (anchorCP < TypoTableRow.getStartCP(lastVisibleLine, snapshot))
          else if anchorCP < TypoTableRow::getStartCP(lastVisibleLine, snapshot) {
            // java return false;
            return false;
          }
          // java else
          else {
            // java return 5 != lastVisibleLineType || !TableCellRangesTool.isInTableRow(lastVisibleLine, anchorCP, (RANGE)null, true, snapshot);
            return 5 != lastVisibleLineType || !TableCellRangesTool::isInTableRow(lastVisibleLine, anchorCP, None, true, snapshot);
          }
        }
      }
      */
    }

    // java protected boolean shouldInvalidateRedundantWrapDrawing(int lastVisibleLine, int drawing, int anchorCP, int limitCP, LayoutState layoutState)
    pub fn shouldInvalidateRedundantWrapDrawing(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32, limitCP: i32, layoutState: &LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java if (!TypoDrawing.isLayoutNotInCell(drawing, snapshot))
      if !TypoDrawing::isLayoutNotInCell__int_typosnapshot(drawing, snapshot) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (layoutState.atLeastOneLine)
        if layoutState.atLeastOneLine {
          // java if (this.isNullOrFirstLineTableHeader(lastVisibleLine))
          if self.isNullOrFirstLineTableHeader(lastVisibleLine) {
            // java return true;
            return true;
          }
          // java int line = LocateTool.quickFindRangeObj(layoutState.curPage.getPointer(), limitCP - 1, false, snapshot);
          let mut line: i32 = LocateTool::quickFindRangeObj(layoutState.curPage.getPointer(), limitCP - 1, false, snapshot);
          // java if (this.isNullOrFirstLineTableHeader(line))
          if self.isNullOrFirstLineTableHeader(line) {
            // java return true;
            return true;
          }
        }
        // java return false;
        return false;
      }
      */
    }

    // java private boolean hasVMergeCells(RowInfo rowInfo)
    pub fn hasVMergeCells(&self, rowInfo: &RowInfo) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      let mut cellInfo: CellInfo = rowInfo.getCellByIndex(0);
      // java for(CellInfo cellInfo = rowInfo.getCellByIndex(0); null != cellInfo cellInfo = cellInfo.getNextCell())
      while true {
        // java if (1 == cellInfo.getVertMerge())
        if 1 == cellInfo.getVertMerge() {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java private int getLayoutNotInCellDrawingLimitCP(int wrapDrawingCP)
    pub fn getLayoutNotInCellDrawingLimitCP(&self, wrapDrawingCP: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java RowInfo rowInfo = null;
      let mut rowInfo: RowInfo = Default::default();
      // java TableInfos tableInfos = this.mLayoutState.document.getTableInfos();
      let mut tableInfos: Box<dyn TableInfos> = self.mLayoutState.document.getTableInfos();
      // java TableInfo tableInfo = tableInfos.getTableInfoByLevel(wrapDrawingCP, this.mLayoutState.tablePeriLayer);
      let mut tableInfo: TableInfo = tableInfos.getTableInfoByLevel(wrapDrawingCP, self.mLayoutState.tablePeriLayer);
      // java if (null != tableInfo)
      if true {
        // java rowInfo = tableInfo.getRowByCP(wrapDrawingCP);
        rowInfo = tableInfo.getRowByCP(wrapDrawingCP);
        // java if (null != rowInfo)
        if true {
          // java RowInfo prevRow = rowInfo;
          let mut prevRow: RowInfo = rowInfo;
          // java while (null != prevRow)
            // java rowInfo = prevRow;
            rowInfo = prevRow;
            // java prevRow = prevRow.getPrevRow();
            prevRow = prevRow.getPrevRow();
            // java if (!this.hasVMergeCells(rowInfo) && rowInfo.isRevisionDelete())
            if !self.hasVMergeCells(&rowInfo) && rowInfo.isRevisionDelete() {
              // java while (true)
                // java RowInfo prev = rowInfo.getPrevRow();
                let mut prev: RowInfo = rowInfo.getPrevRow();
                // java if (null == prev || !prev.isRevisionDelete())
                if false || !prev.isRevisionDelete() {
                  // java return null != rowInfo ? rowInfo.getStartCP() : wrapDrawingCP;
                  return (if true { rowInfo.getStartCP() } else { wrapDrawingCP });
                }
                // java rowInfo = prev;
                rowInfo = prev;
            }
        }
      }
      // java return null != rowInfo ? rowInfo.getStartCP() : wrapDrawingCP;
      return (if true { rowInfo.getStartCP() } else { wrapDrawingCP });
      */
    }

    // java protected void onModified(Rect changeRect)
    pub fn onModified(&mut self, changeRect: Option<&Rect>) {
      // java this.mViewEnv.onPageModified(changeRect);
      let _ = changeRect;
      self.super_.mViewEnv.onPageModified(None);
    }

    // java public DocumentEnv getDocEnv()
    pub fn getDocEnv(&self) -> &DocumentEnv {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mDocEnv;
      return &self.mDocEnv;
      */
    }

    // java public ViewEnv getViewEnv()
    pub fn getViewEnv(&self) -> &ViewEnv {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mViewEnv;
      return &self.mViewEnv;
      */
    }

    // java public boolean isComplete()
    pub fn isComplete(&self) -> bool {
      // java return this.isComplete(this.mLayoutState);
      match self.mLayoutState.as_ref() {
        None => true,
        Some(st) => TextLayouter::isComplete(&self.super_, st),
      }
    }

    // java public void onSetTextDocument()
    pub fn onSetTextDocument(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TextDocument textDocument = this.mDocEnv.textDocument;
      let mut textDocument: TextDocument = self.mDocEnv.textDocument;
      // java this.mLayoutState.document = textDocument.getMainDocument();
      self.mLayoutState.document = textDocument.getMainDocument();
      */
    }

    // java protected void clearInvalidatePages(int pageId)
    pub fn clearInvalidatePages(&mut self, pageId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pageId >= 0)
      if pageId >= 0 {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int layoutPageList = snapshot.getLayoutPageList();
        let mut layoutPageList: i32 = snapshot.getLayoutPageList();
        // java int size = TypoList.size(layoutPageList, snapshot);
        let mut size: i32 = TypoList::size__int_typosnapshot(layoutPageList, snapshot);
        let mut i: i32 = pageId;
        // java for(int i = pageId; i < size; ++i)
        while i < size {
          // java int page = TypoList.quickGet(i, layoutPageList, snapshot);
          let mut page: i32 = TypoList::quickGet__int_int_typosnapshot(i, layoutPageList, snapshot);
          // java this.mViewEnv.onPageDeleted(page);
          self.mViewEnv.onPageDeleted(page);
          // java if (null != this.mLayoutServiceCache)
          if let Some(mLayoutServiceCache) = self.mLayoutServiceCache.as_mut() {
            // java this.mLayoutServiceCache.onRemove(page);
            mLayoutServiceCache.onRemove(page);
          }
        }
        // java if (size > pageId)
        if size > pageId {
          // java TypoMemHelper.freeLayoutPages(pageId, size, layoutPageList, snapshot);
          TypoMemHelper::freeLayoutPages(pageId, size, layoutPageList, snapshot);
          // java if (this.mViewEnv.isPageMode())
          if self.mViewEnv.isPageMode() {
            // java snapshot.setModifyRange(pageId, size - 1);
            snapshot.setModifyRange(pageId, size - 1);
          }
          // java if (this.mViewEnv.isPhoneReadMode())
          if self.mViewEnv.isPhoneReadMode() {
            // java this.mViewEnv.setHasFlowToEnd(false);
            self.mViewEnv.setHasFlowToEnd(false);
          }
        }
        // java if (this.mDocEnv != null)
        if true {
          // java this.mDocEnv.clearFitInfo();
          self.mDocEnv.clearFitInfo();
        }
      }
      */
    }

    // java public void invalidCardState()
    pub fn invalidCardState(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocEnv.setCardPaperDirty(true);
      self.mDocEnv.setCardPaperDirty(true);
      // java this.mLayoutState.invalidCardState();
      self.mLayoutState.invalidCardState();
      */
    }

    // java public void makeBalloonCaches()
    pub fn makeBalloonCaches(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java EnumPagesMakeBalloon enumPagesMakeBalloon = new EnumPagesMakeBalloon();
      let mut enumPagesMakeBalloon: EnumPagesMakeBalloon = EnumPagesMakeBalloon::new();
      // java enumPagesMakeBalloon.enumAllPages(snapshot);
      enumPagesMakeBalloon.enumAllPages(snapshot);
      */
    }

    // java protected static void commitMemTag(int pointer, int layoutPage, TypoSnapshot snapshot)
    pub fn commitMemTag(pointer: i32, layoutPage: i32, snapshot: &TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int lastMemTag = TypoLayoutPage.getNextMemTag(layoutPage, snapshot);
      let mut lastMemTag: i32 = TypoLayoutPage::getNextMemTag__int_typosnapshot(layoutPage, snapshot);
      // java if (0 == lastMemTag)
      if 0 == lastMemTag {
        // java lastMemTag = layoutPage;
        lastMemTag = layoutPage;
        // java TypoLayoutPage.setNextMemTag(pointer, layoutPage, snapshot);
        TypoLayoutPage::setNextMemTag__int_int_typosnapshot(pointer, layoutPage, snapshot);
      }
      // java else
      else {
        let mut nextMemTag: i32 = TypoMemTag::getNextMemTag(lastMemTag, snapshot);
        // java for(int nextMemTag = TypoMemTag.getNextMemTag(lastMemTag, snapshot); 0 != nextMemTag nextMemTag = TypoMemTag.getNextMemTag(nextMemTag, snapshot))
        while 0 != nextMemTag {
          // java lastMemTag = nextMemTag;
          lastMemTag = nextMemTag;
        }
        // java TypoMemTag.setNextMemTag(pointer, lastMemTag, snapshot);
        TypoMemTag::setNextMemTag(pointer, lastMemTag, snapshot);
      }
      // java TypoMemTag.setPrevMemTag(lastMemTag, pointer, snapshot);
      TypoMemTag::setPrevMemTag(lastMemTag, pointer, snapshot);
      */
    }

    // java protected void loadStartStateFromLastVersion(StateSnapshot stateSnapshot, int pLayoutPage, Pools pools)
    pub fn loadStartStateFromLastVersion(&mut self, stateSnapshot: &StateSnapshot, pLayoutPage: i32, pools: &Pools) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mDocEnv.typoDocument.getSnapshot();
      let mut snapshot: TypoSnapshot = self.mDocEnv.typoDocument.getSnapshot();
      // java stateSnapshot.loadFromTypoMem(TypoLayoutPage.getStartState(pLayoutPage, snapshot), snapshot, pools);
      stateSnapshot.loadFromTypoMem__int_typosnapshot_pools(TypoLayoutPage::getStartState__int_typosnapshot(pLayoutPage, snapshot), snapshot, pools);
      // java snapshot.release();
      snapshot.release();
      */
    }

    // java public LocateResult locateEmbedCommentScroll(int embedCommentCp)
    pub fn locateEmbedCommentScroll(&self, embedCommentCp: i32) -> Option<LocateResult> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return null;
      return None;
      */
    }

    // java protected void initGraphBuff(LayoutState layoutState)
    pub fn initGraphBuff__layoutstate(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isLayoutOnePara = false;
      let mut isLayoutOnePara: bool = false;
      // java if (layoutState instanceof WebViewLayoutState)
      if layoutState.downcast_ref::<WebViewLayoutState>().is_some() {
        // java isLayoutOnePara = ((WebViewLayoutState)layoutState).layoutOnePara;
        isLayoutOnePara = (layoutState).downcast_ref::<WebViewLayoutState>().unwrap().layoutOnePara;
      }
      // java if (layoutState.cp >= layoutState.getRangeEnd() && !isLayoutOnePara)
      if layoutState.cp >= layoutState.getRangeEnd() && !isLayoutOnePara {
        // java layoutState.setRangeEnd(this.calcMaxValidRangeEnd(layoutState.cp));
        layoutState.setRangeEnd(self.calcMaxValidRangeEnd(layoutState.cp));
      }
      // java super.initGraphBuff(layoutState);
      self.super_.initGraphBuff(layoutState);
      */
    }

    // java protected void initGraphBuff(LayoutState layoutState, int endCp)
    pub fn initGraphBuff__layoutstate_int(&mut self, layoutState: &mut LayoutState, endCp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isLayoutOnePara = false;
      let mut isLayoutOnePara: bool = false;
      // java if (layoutState instanceof WebViewLayoutState)
      if layoutState.downcast_ref::<WebViewLayoutState>().is_some() {
        // java isLayoutOnePara = ((WebViewLayoutState)layoutState).layoutOnePara;
        isLayoutOnePara = (layoutState).downcast_ref::<WebViewLayoutState>().unwrap().layoutOnePara;
      }
      // java if (endCp >= layoutState.getRangeEnd() && !isLayoutOnePara)
      if endCp >= layoutState.getRangeEnd() && !isLayoutOnePara {
        // java layoutState.setRangeEnd(this.calcMaxValidRangeEnd(endCp));
        layoutState.setRangeEnd(self.calcMaxValidRangeEnd(endCp));
      }
      // java super.initGraphBuff(layoutState);
      self.super_.initGraphBuff(layoutState);
      */
    }

    // java protected PredictSpacesInfo.IContextHelper getPhoneFitHelper()
    pub fn getPhoneFitHelper(&self) -> Option<&dyn IContextHelper> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PredictSpacesInfo.IContextHelper helper = new PredictSpacesInfo.IContextHelper() { public PhoneViewFitInfo getFitInfo() { return MainLayouter.this.mDocEnv.getPhoneViewFitInfo(); } public LineBreakAnalyzer getLineBreakAnalyzer() { if (MainLayouter.this.mDocEnv.mLineBreakAnalyzer == null) { MainLayouter.this.mDocEnv.mLineBreakAnalyzer = new LineBreakAnalyzer(MainLayouter.this.mDocEnv, MainLayouter.this.mViewEnv); } MainLayouter.this.mDocEnv.mLineBreakAnalyzer.setLayouterWhileLoading(MainLayouter.this); return MainLayouter.this.mDocEnv.mLineBreakAnalyzer; } public boolean isInAnalyzer() { return false; } };
      let mut helper: PredictSpacesInfo::IContextHelper = { let __x: PredictSpacesInfo::IContextHelper = todo!("no align strategy matched"); __x };
      // java return helper;
      return { let __x: Option<&IContextHelper> = todo!("no align strategy matched"); __x };
      */
    }

}

impl std::ops::Deref for MainLayouter {
    type Target = TextLayouter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for MainLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class EnumPagesMakeBalloon extends CrackRanges.IPageEnumCallback
impl EnumPagesMakeBalloon {
    // java private  EnumPagesMakeBalloon()
    pub fn EnumPagesMakeBalloon() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMemTag = new TypoMemTag();
      Self { super_: IPageEnumCallback::new(), mMemTag: TypoMemTag::new(), mCPRefUsage: TypoCPRefUsage::new() }
      // java this.mCPRefUsage = new TypoCPRefUsage();
      */
    }

    // java public boolean onPage(int index, int pPage, TypoSnapshot snapshot)
    pub fn onPage(&mut self, index: i32, pPage: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java MainState mainState = MainLayouter.this.mMainState;
      let mut mainState: MainState = self.mMainState;
      // java mainState.layoutPage.assignment(pPage, snapshot);
      mainState.layoutPage.assignment(pPage, snapshot);
      // java int revisons = mainState.layoutPage.getRevisions();
      let mut revisons: i32 = mainState.layoutPage.getRevisions();
      // java if (0 != revisons)
      if 0 != revisons {
        // java mainState.revisions.assignment(revisons, snapshot);
        mainState.revisions.assignment(revisons, snapshot);
      }
      // java else
      else {
        // java mainState.revisions.clean();
        mainState.revisions.clean();
      }
      // java if (null == MainLayouter.this.balloonCacheLayouter)
      if self.balloonCacheLayouter.is_none() {
        // java MainLayouter.this.balloonCacheLayouter = new BalloonCacheLayouter(mainState, MainLayouter.this.mDocEnv, MainLayouter.this.mViewEnv);
        self.balloonCacheLayouter = BalloonCacheLayouter::new(mainState, self.mDocEnv, self.mViewEnv);
      }
      // java this.mMemTag.alloc(snapshot);
      self.mMemTag.alloc(snapshot);
      // java this.mCPRefUsage.init(snapshot);
      self.mCPRefUsage.init(snapshot);
      // java int pBalloonCache = TypoLayoutPage.getBalloonCache(pPage, snapshot);
      let mut pBalloonCache: i32 = TypoLayoutPage::getBalloonCache__int_typosnapshot(pPage, snapshot);
      // java int pCommentCache = TypoLayoutPage.getCommentData(pPage, snapshot);
      let mut pCommentCache: i32 = TypoLayoutPage::getCommentData__int_typosnapshot(pPage, snapshot);
      // java MainLayouter.this.balloonCacheLayouter.makeBalloonCache(pPage);
      self.balloonCacheLayouter.makeBalloonCache(pPage);
      // java if (pBalloonCache != TypoLayoutPage.getBalloonCache(pPage, snapshot) || pCommentCache != TypoLayoutPage.getCommentData(pPage, snapshot))
      if pBalloonCache != TypoLayoutPage::getBalloonCache__int_typosnapshot(pPage, snapshot) || pCommentCache != TypoLayoutPage::getCommentData__int_typosnapshot(pPage, snapshot) {
        // java this.setPageModifyTag(index);
        self.setPageModifyTag(index);
      }
      // java int pCPUsage = this.mCPRefUsage.commit();
      let mut pCPUsage: i32 = self.mCPRefUsage.commit();
      // java int memEnd = snapshot.getAllocSize();
      let mut memEnd: i32 = snapshot.getAllocSize();
      // java if (memEnd == this.mMemTag.getPointer() + 6)
      if memEnd == self.mMemTag.getPointer() + 6 {
        // java Assert.assertTrue(pCPUsage == 0);
        Assert::assertTrue__boolean(pCPUsage == 0);
        // java snapshot.free(this.mMemTag.getPointer(), 6);
        snapshot.free(self.mMemTag.getPointer(), 6);
      }
      // java else
      else {
        // java if (pCPUsage != 0)
        if pCPUsage != 0 {
          // java this.mMemTag.setCPRefUsage(pCPUsage);
          self.mMemTag.setCPRefUsage__int(pCPUsage);
        }
        // java this.mMemTag.setMemEnd(memEnd);
        self.mMemTag.setMemEnd__int(memEnd);
        // java MainLayouter.commitMemTag(this.mMemTag.getPointer(), pPage, snapshot);
        MainLayouter::commitMemTag(self.mMemTag.getPointer(), pPage, snapshot);
        // java TypoLayoutPage.updateModifyVerion(pPage, snapshot);
        TypoLayoutPage::updateModifyVerion__int_typosnapshot(pPage, snapshot);
      }
      // java return true;
      return true;
      */
    }

}

impl std::ops::Deref for EnumPagesMakeBalloon {
    type Target = IPageEnumCallback;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EnumPagesMakeBalloon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
