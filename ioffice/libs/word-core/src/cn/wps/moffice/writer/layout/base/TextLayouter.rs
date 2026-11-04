// Java: import cn.wps.base.assertion.Assert
// CARGO_OFF_SURFACE use util::base::assertion::Assert::Assert;
// Java: import cn.wps.collections.KList
// CARGO_OFF_SURFACE use util::collections::KList::KList;
// Java: import cn.wps.collections.KPList
// CARGO_OFF_SURFACE use util::collections::KPList::KPList;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
// CARGO_OFF_SURFACE use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.drawing.Shape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.property.PropertySet
// CARGO_OFF_SURFACE use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.base.ViewMode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.cache.LocateTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::LocateTool::LocateTool;
// Java: import cn.wps.moffice.writer.cache.TypoColumns
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoPara
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoRangeBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
// Java: import cn.wps.moffice.writer.cache.TypoTextFrame
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTextFrame::TypoTextFrame;
// Java: import cn.wps.moffice.writer.cache.TypoTextLine
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.data.PLC
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLC::PLC;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCMathStart::PLCMathStart;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfo::TableInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfos
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
// Java: import cn.wps.moffice.writer.data.value.MathProperty
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::MathProperty::MathProperty;
// Java: import cn.wps.moffice.writer.data.value.Revision
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingAffectTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingAffectTool::DrawingAffectTool;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingCacheTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCacheTool::DrawingCacheTool;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::DrawingLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingPositionTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingPositionTool::DrawingPositionTool;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingResizeTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool;
// Java: import cn.wps.moffice.writer.layout.base.drawing.TextFrameLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::TextFrameLayouter::TextFrameLayouter;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.DrawingCacheItem
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaPropEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;
// Java: import cn.wps.moffice.writer.layout.base.env.SuperEnvoy
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::footendnote::IFootEndNoteNumLayouter::IFootEndNoteNumLayouter;
// Java: import cn.wps.moffice.writer.layout.base.equation.EquationException
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::equation::EquationException::EquationException;
// Java: import cn.wps.moffice.writer.layout.base.equation.EquationLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::EquationLayouter;
// Java: import cn.wps.moffice.writer.layout.base.footendnote.FootEndNoteCacheLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::footendnote::FootEndNoteCacheLayouter::FootEndNoteCacheLayouter;
// Java: import cn.wps.moffice.writer.layout.base.footendnote.FootEndNoteLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::footendnote::FootEndNoteLayouter::FootEndNoteLayouter;
// Java: import cn.wps.moffice.writer.layout.base.graph.CharsNode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuff
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::GraphBuff;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuffFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuffFactory::GraphBuffFactory;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphReaderTool
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReaderTool::GraphReaderTool;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.line.LineLayouter
use crate::cn::wps::moffice::writer::layout::base::line::LineLayouter::LineLayouter;
// Java: import cn.wps.moffice.writer.layout.base.math.GroupCenterAlignTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::math::GroupCenterAlignTool::GroupCenterAlignTool;
// Java: import cn.wps.moffice.writer.layout.base.math.MathLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::math::MathLayouter::MathLayouter;
// Java: import cn.wps.moffice.writer.layout.base.page.PageLayouter
use crate::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
// Java: import cn.wps.moffice.writer.layout.base.page.PageRangesCollector
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::page::PageRangesCollector::PageRangesCollector;
// Java: import cn.wps.moffice.writer.layout.base.table.TableCellBrcTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableCellBrcTool::TableCellBrcTool;
// Java: import cn.wps.moffice.writer.layout.base.table.TableLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::TableLayouter;
// Java: import cn.wps.moffice.writer.layout.base.table.TableRowLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableRowLayouter::TableRowLayouter;
// Java: import cn.wps.moffice.writer.layout.base.table.Tools
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::Tools::Tools;
// Java: import cn.wps.moffice.writer.layout.base.tools.PaginationTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::tools::PaginationTool::PaginationTool;
// Java: import cn.wps.util.PrimitivePackTool
// CARGO_OFF_SURFACE use thirdparty_font::cn::wps::util::PrimitivePackTool::PrimitivePackTool;
// Java: import cn.wps.moffice.writer.layout.base.ParaLayouter
use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
// Java: import org.apache.poi.poifs.filesystem.Entry
// CARGO_OFF_SURFACE use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;
// Java: import cn.wps.moffice.writer.layout.base.CacheCleaner
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::CacheCleaner::CacheCleaner;
// Java: import java.lang.Object
// CARGO_OFF_SURFACE use stubs::java::lang::Object::Object;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;
use core::marker::PhantomData;

// CARGO_OFF_SURFACE_TYPES
use crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::{ParentLayouterTask, TableLayouter};
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::StateSnapshot as LayoutStateSnapshot;
// Java field type is DrawingLayouter. PageViewLayouter.initDrawingLayouter constructs it.
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::DrawingLayouter as BaseDrawingLayouter;
#[derive(Default)]
pub struct TextFrameLayouter;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::{EmbedArg, GraphBuff};
use crate::cn::wps::moffice::writer::layout::base::graph::GraphFactory::GraphFactory;
#[derive(Default)]
pub struct PageRangesCollector;
#[derive(Default)]
pub struct FootEndNoteLayouter;
pub struct StateSnapshot {
    // java getInt(KEY_START_CP, -1)
    start_cp: i32,
    // java getObject(KEY_FIRSTMERGECELL)
    first_merge: Option<Box<StateSnapshot>>,
    // java KList<StateSnapshot> mCellStates
    cells: Vec<StateSnapshot>,
}

impl Default for StateSnapshot {
    fn default() -> Self {
        Self { start_cp: -1, first_merge: None, cells: Vec::new() }
    }
}

impl StateSnapshot {
    // java public int getStartCp()
    pub fn getStartCp(&self) -> i32 { self.start_cp }

    // java public StateSnapshot getFirstMergeCell()
    pub fn getFirstMergeCell(&self) -> Option<&StateSnapshot> { self.first_merge.as_deref() }

    // java public KList<StateSnapshot> getCellStates()
    pub fn getCellStates(&self) -> &[StateSnapshot] { &self.cells }
}

#[derive(Default)]
pub struct Object;
#[derive(Default)]
pub struct TypoPage;
#[derive(Default)]
pub struct WIntArrayList;
#[derive(Default)]
pub struct TypoDrawing;
#[derive(Default)]
pub struct TableInfo;
pub trait IStateEnvoy {}
struct CrashesLog;
impl CrashesLog {
    fn addEnv(_s: &LayoutState) {}
    fn removeEnv(_s: &LayoutState) {}
}

// java private static final String TAG = null;
static TAG: std::sync::LazyLock<Option<&str>> =
                    std::sync::LazyLock::new(|| None);

// java public abstract class TextLayouter implements TableLayouter.ParentLayouterTask
#[derive(Default)]
pub struct TextLayouter {
    // java protected final MainState mMainState;
    pub mMainState: MainState,
    // java protected final DocumentEnv mDocEnv;
    pub(crate) mDocEnv: DocumentEnv,
    // java protected final ViewEnv mViewEnv;
    pub(crate) mViewEnv: ViewEnv,
    // java protected PageLayouter mPageLayouter;
    pub mPageLayouter: Option<PageLayouter>,
    // java protected ParaLayouter mParaLayouter;
    pub mParaLayouter: Option<ParaLayouter>,
    // java protected LineLayouter mLineLayouter;
    pub(crate) mLineLayouter: Option<LineLayouter>,
    // java protected TableLayouter mTableLayouter;
    pub(crate) mTableLayouter: Option<TableLayouter>,
    // java protected DrawingLayouter mDrawingLayouter;
    pub(crate) mDrawingLayouter: Option<BaseDrawingLayouter>,
    // java protected TextFrameLayouter mTextFrameLayouter;
    pub(crate) mTextFrameLayouter: Option<TextFrameLayouter>,
    // java protected GraphBuff mGraphBuff;
    // Java stores a GraphFactory (GraphBuff subclass). Calls go through GraphReader.
    pub(crate) mGraphBuff: Option<GraphFactory>,
    // java protected int mStartCP;
    pub mStartCP: i32,
    // java protected int mRef;
    pub mRef: i32,
    // java protected boolean mNeedCollectRangeForFirstRow;
    pub mNeedCollectRangeForFirstRow: bool,
    // java protected boolean mNeedCollectRangeForLastRow;
    pub(crate) mNeedCollectRangeForLastRow: bool,
    // java protected boolean mHasFlowTableHeader;
    pub(crate) mHasFlowTableHeader: bool,
    // java protected boolean mHasFlowNormalTable;
    pub mHasFlowNormalTable: bool,
    // java protected long mTableHeaderRange = 0L;
    pub(crate) mTableHeaderRange: i64,
    // java protected int mReflowCount;
    pub(crate) mReflowCount: i32,
    // java protected PageRangesCollector mPageRangesCollector;
    pub(crate) mPageRangesCollector: Option<PageRangesCollector>,
    // java protected FootEndNoteLayouter mFootEndNoteLayouter;
    pub(crate) mFootEndNoteLayouter: Option<FootEndNoteLayouter>,
    // java protected StateSnapshot mStateSnapshot;
    pub(crate) mStateSnapshot: Option<LayoutStateSnapshot>,
    // java protected boolean mHasBreakRow;
    pub(crate) mHasBreakRow: bool,
    // java protected boolean mEnableProcessPagination;
    pub(crate) mEnableProcessPagination: bool,
}

// Restores the Java-shared MainState/DocumentEnv/ViewEnv if a drawing call unwinds.
struct DrawingEnvSwap {
    drawing: *mut BaseDrawingLayouter,
    main: *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState,
    doc: *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv,
    view: *mut crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv,
}

impl Drop for DrawingEnvSwap {
    fn drop(&mut self) {
        unsafe {
            std::mem::swap(&mut (*self.drawing).mMainState, &mut *self.main);
            std::mem::swap(&mut (*self.drawing).mDocEnv, &mut *self.doc);
            std::mem::swap(&mut (*self.drawing).mViewEnv, &mut *self.view);
        }
    }
}

// java public abstract class TextLayouter implements TableLayouter.ParentLayouterTask
unsafe fn end_para_from_table(
  parent: *mut (),
  envoy: *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy,
) {
  if parent.is_null() || envoy.is_null() { return; }
  (*parent.cast::<TextLayouter>()).endParaLayouter(&mut *envoy);
}

unsafe fn adjust_wrap_table_from_table(
  parent: *mut (),
  drawing: *mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing,
  table: *const crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::TableInfo,
  para_top: i32,
  need_cut: bool,
  state: *mut LayoutState,
) {
  if parent.is_null() || drawing.is_null() || table.is_null() || state.is_null() { return; }
  (*parent.cast::<TextLayouter>()).adjustWrapTable(&mut *drawing, &*table, para_top, need_cut, &mut *state);
}

impl TextLayouter {
    // java public  TextLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn TextLayouter(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java this.mMainState = mainState;
      // rust: ★wr-29 08 —— 子 layouters 装配（Java this.mLineLayouter = new LineLayouter(...);
      //       this.mGraphBuff = new GraphBuff(...)）
      // rust: env 无 Clone（镜像 owned）—— 子 layouters 以默认实例装配（Java 引用语义；
      //       MainLayouter 全链路接线（06）时以真实 env 重建）
      let line_layouter: LineLayouter = LineLayouter::LineLayouter(MainState::default(), DocumentEnv::default(), ViewEnv::default());
      // java this.mGraphBuff = GraphBuffFactory.createGraphBuff(...) which returns GraphFactory.
      let graph_buff: GraphFactory = GraphFactory::GraphFactory(MainState::default(), DocumentEnv::default(), ViewEnv::default());
      let mut this = Self { mMainState: mainState, mDocEnv: docEnv, mViewEnv: viewEnv, mPageLayouter: Default::default(), mParaLayouter: Default::default(), mLineLayouter: Some(line_layouter), mTableLayouter: Default::default(), mDrawingLayouter: Default::default(), mTextFrameLayouter: Default::default(), mGraphBuff: Some(graph_buff), mStartCP: Default::default(), mRef: Default::default(), mNeedCollectRangeForFirstRow: Default::default(), mNeedCollectRangeForLastRow: Default::default(), mHasFlowTableHeader: Default::default(), mHasFlowNormalTable: Default::default(), mTableHeaderRange: Default::default(), mReflowCount: Default::default(), mPageRangesCollector: Default::default(), mFootEndNoteLayouter: Default::default(), mStateSnapshot: Default::default(), mHasBreakRow: Default::default(), mEnableProcessPagination: Default::default() };
      // java this.mDocEnv = docEnv;
      // java this.mViewEnv = viewEnv;
      // java this.init();
      // rust: tests construct via Default + mPageLayouter; init() stays DEFERRED_NON_FUNCTION
      this
    }

    // java protected LayoutState createLayoutState()
    pub fn createLayoutState(&self) -> LayoutState {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new LayoutState((SuperEnvoy)null);
      return LayoutState::new(None);
      */
    }

    // java protected void init()
    pub fn init(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mGraphBuff = GraphBuffFactory.createGraphBuff(this.mMainState, this.mDocEnv, this.mViewEnv);
      self.mGraphBuff = GraphBuffFactory::createGraphBuff(self.mMainState, self.mDocEnv, self.mViewEnv);
      // java this.initLayouters();
      self.initLayouters();
      // java this.mPageRangesCollector = new PageRangesCollector();
      self.mPageRangesCollector = PageRangesCollector::new();
      // java this.mStateSnapshot = this.mDocEnv.pools.obtainStateSnapshot();
      self.mStateSnapshot = self.mDocEnv.pools.obtainStateSnapshot();
      */
    }

    // java protected abstract void initLayouters()
    pub fn initLayouters(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("TextLayouter::initLayouters not implemented");
      */
    }

    // java protected abstract void initTableLayouter()
    pub fn initTableLayouter(&mut self) {
      // java this.mTableLayouter = new TableLayouter(this, this.mDocEnv, this.mViewEnv, this.mMainState);
      let doc = &mut self.mDocEnv as *mut DocumentEnv;
      let view = &mut self.mViewEnv as *mut ViewEnv;
      let main = &mut self.mMainState as *mut MainState;
      let mut table = TableLayouter::TableLayouter(
        ParentLayouterTask::default(),
        DocumentEnv::default(),
        ViewEnv::default(),
        MainState::default(),
      );
      table.bind_env(doc, view, main);
      let parent = self as *mut TextLayouter as *mut ();
      table.bind_parent(parent, end_para_from_table, adjust_wrap_table_from_table);
      self.mTableLayouter = Some(table);
    }

    // java protected abstract void initDrawingLayouter()
    // PageViewLayouter overrides this. The hook is the Rust stand-in for that virtual call.
    pub fn initDrawingLayouter(&mut self) {
      if self.mDrawingLayouter.is_some() {
        return;
      }
      let hook = self.mPageLayouter.as_ref().and_then(|page| {
        let (owner, init_drawing) = page.page_view_init_drawing();
        init_drawing.map(|init_drawing| (owner, init_drawing))
      });
      if let Some((owner, init_drawing)) = hook {
        if !owner.is_null() {
          init_drawing(owner);
          return;
        }
      }
      self.install_drawing_layouter();
    }

    // java new DrawingLayouter(mMainState, mDocEnv, mViewEnv, mLineLayouter)
    // The three env objects are the same Java instances. Rust swaps them in for each drawing call.
    pub fn install_drawing_layouter(&mut self) {
      if self.mDrawingLayouter.is_some() {
        return;
      }
      self.mDrawingLayouter = Some(BaseDrawingLayouter::DrawingLayouter(
        crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState::default(),
        crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv::default(),
        crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv::default(),
        None,
      ));
    }

    // Java passes one MainState/DocumentEnv/ViewEnv into DrawingLayouter. Swap them for the call.
    fn with_drawing_envs<R>(&mut self, body: impl FnOnce(&mut BaseDrawingLayouter) -> R) -> R {
      let mut drawing = self.mDrawingLayouter.take().expect("mDrawingLayouter");
      let guard = DrawingEnvSwap {
        drawing: &mut drawing as *mut BaseDrawingLayouter,
        main: &mut self.mMainState as *mut _,
        doc: &mut self.mDocEnv as *mut _,
        view: &mut self.mViewEnv as *mut _,
      };
      unsafe {
        std::mem::swap(&mut (*guard.drawing).mMainState, &mut *guard.main);
        std::mem::swap(&mut (*guard.drawing).mDocEnv, &mut *guard.doc);
        std::mem::swap(&mut (*guard.drawing).mViewEnv, &mut *guard.view);
      }
      let result = body(&mut drawing);
      drop(guard);
      self.mDrawingLayouter = Some(drawing);
      result
    }

    // java protected void initTextFrameLayouter()
    pub fn initTextFrameLayouter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTextFrameLayouter = new TextFrameLayouter(this.mDocEnv, this.mViewEnv, this.mMainState, this.getDrawingLayouter());
      self.mTextFrameLayouter = TextFrameLayouter::new(self.mDocEnv, self.mViewEnv, self.mMainState, self.getDrawingLayouter());
      */
    }

    // java protected void initFootEndNoteLayouter()
    pub fn initFootEndNoteLayouter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteLayouter = new FootEndNoteCacheLayouter(this.mMainState, this.mDocEnv, this.mViewEnv);
      self.mFootEndNoteLayouter = FootEndNoteCacheLayouter::new(self.mMainState, self.mDocEnv, self.mViewEnv);
      */
    }

    // java public DrawingLayouter getDrawingLayouter()
    pub fn getDrawingLayouter(&self) -> Option<&BaseDrawingLayouter> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mDrawingLayouter)
      if self.mDrawingLayouter.is_none() {
        // java this.initDrawingLayouter();
        self.initDrawingLayouter();
      }
      // java return this.mDrawingLayouter;
      return self.mDrawingLayouter.as_ref();
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mPageLayouter != null)
      if let Some(mPageLayouter) = self.mPageLayouter.as_mut() {
        // java this.mPageLayouter.dispose();
        mPageLayouter.dispose();
        // java this.mPageLayouter = null;
        mPageLayouter = None;
      }
      // java if (this.mParaLayouter != null)
      if let Some(mParaLayouter) = self.mParaLayouter.as_mut() {
        // java this.mParaLayouter.dispose();
        mParaLayouter.dispose();
        // java this.mParaLayouter = null;
        mParaLayouter = None;
      }
      // java if (this.mLineLayouter != null)
      if let Some(mLineLayouter) = self.mLineLayouter.as_mut() {
        // java this.mLineLayouter.dispose();
        mLineLayouter.dispose();
        // java this.mLineLayouter = null;
        mLineLayouter = None;
      }
      // java if (this.mTableLayouter != null)
      if let Some(mTableLayouter) = self.mTableLayouter.as_mut() {
        // java this.mTableLayouter.dispose();
        mTableLayouter.dispose();
        // java this.mTableLayouter = null;
        mTableLayouter = None;
      }
      // java if (this.mDrawingLayouter != null)
      if let Some(mDrawingLayouter) = self.mDrawingLayouter.as_mut() {
        // java this.mDrawingLayouter.dispose();
        mDrawingLayouter.dispose();
        // java this.mDrawingLayouter = null;
        mDrawingLayouter = None;
      }
      // java if (null != this.mTextFrameLayouter)
      if let Some(mTextFrameLayouter) = self.mTextFrameLayouter.as_mut() {
        // java this.mTextFrameLayouter.dispose();
        mTextFrameLayouter.dispose();
        // java this.mTextFrameLayouter = null;
        mTextFrameLayouter = None;
      }
      // java if (this.mFootEndNoteLayouter != null)
      if let Some(mFootEndNoteLayouter) = self.mFootEndNoteLayouter.as_mut() {
        // java this.mFootEndNoteLayouter.dispose();
        mFootEndNoteLayouter.dispose();
        // java this.mFootEndNoteLayouter = null;
        mFootEndNoteLayouter = None;
      }
      // java if (null != this.mGraphBuff)
      if let Some(mGraphBuff) = self.mGraphBuff.as_mut() {
        // java this.mGraphBuff.dispose();
        mGraphBuff.dispose();
        // java this.mGraphBuff = null;
        mGraphBuff = None;
      }
      // java if (null != this.mStateSnapshot)
      if let Some(mStateSnapshot) = self.mStateSnapshot.as_mut() {
        // java this.mDocEnv.pools.recycle(this.mStateSnapshot);
        self.mDocEnv.pools.recycle(mStateSnapshot);
        // java this.mStateSnapshot = null;
        mStateSnapshot = None;
      }
      */
    }

    // rust: 测试面
    pub fn m_line_layouter_is_some(&self) -> bool {
        self.mLineLayouter.is_some()
    }

    // rust: ★wr-29 06 —— 行 layouter 及其行高叶的 env 重绑到宿主真实实例
    //       （Java 引用语义：TextLayouter 与其子层共享同一 docEnv/viewEnv）
    pub fn rebase_line_env(&mut self) {
        let main_ptr = &mut self.mMainState as *mut MainState;
        let doc_ptr = &mut self.mDocEnv as *mut DocumentEnv;
        let view_ptr = &mut self.mViewEnv as *mut ViewEnv;
        if let Some(ll) = self.mLineLayouter.as_mut() {
            ll.bindTextWrapEnvs(main_ptr, doc_ptr, view_ptr);
            // 写入 owned 槽（Java 引用语义等价：子层共享同一实例）
            let base = ll.super_mut().super_mut().super_mut();
            base.mDocEnv = Some(DocumentEnv::default());
            base.mViewEnv = Some(ViewEnv::default());
            let doc_ptr: *mut DocumentEnv = base.mDocEnv.as_mut().map(|d| d as *mut DocumentEnv).unwrap_or(std::ptr::null_mut());
            let view_ptr: *mut ViewEnv = base.mViewEnv.as_mut().map(|v| v as *mut ViewEnv).unwrap_or(std::ptr::null_mut());
            let lh: &mut crate::cn::wps::moffice::writer::layout::base::line::LineHeightLayouter::LineHeightLayouter =
                ll.mLineHeightLayouter_mut();
            lh.set_env(doc_ptr, view_ptr);
        }
    }

    // java GraphBuff mGraphBuff is a GraphFactory. Rebind after the owner is in place.
    pub fn bind_graph_factory(&mut self) {
      if let Some(gf) = self.mGraphBuff.as_mut() {
        // java GraphBuffFactory.createGraphBuff(this.mMainState, this.mDocEnv, this.mViewEnv)
        // retains the same environment objects; Rust must refresh their aliases
        // after TextLayouter has reached its final owner address.
        gf.super_.super_.bind_env_alias(
          &mut self.mMainState as *mut MainState,
          &mut self.mDocEnv as *mut DocumentEnv,
          &mut self.mViewEnv as *mut ViewEnv,
        );
        gf.bind_factory_dispatch();
      }
      // java PageViewFootEndNoteNumLayouter holds the same MainState, DocumentEnv, ViewEnv and mStateSnapshot.
      let main = &mut self.mMainState as *mut MainState as *mut ();
      let view = &mut self.mViewEnv as *mut ViewEnv as *mut ();
      let snap = self.mStateSnapshot.as_mut().map(|s| s as *mut LayoutStateSnapshot as *mut ()).unwrap_or(std::ptr::null_mut());
      let doc = &mut self.mDocEnv as *mut DocumentEnv as *mut ();
      if let Some(layouter) = self.mDocEnv.footEndNoteNumLayouter.as_mut() {
        layouter.rebind_ptrs(main, doc, view, snap);
      }
    }

    // java collectors allocated by TextLayouter.init after initLayouters().
    pub fn finish_java_init(&mut self) {
      self.mPageRangesCollector = Some(PageRangesCollector::default());
      self.mStateSnapshot = Some(LayoutStateSnapshot::default());
      if let Some(gf) = self.mGraphBuff.as_mut() {
        // java PageViewLayouter ctor: this.mGraphBuff.setMaxGraphSize(8192);
        gf.super_.setMaxGraphSize(8192);
      }
    }

    // java public void flow(LayoutState layoutState)
    pub fn flow(&mut self, layoutState: &mut LayoutState) {
      self.bind_graph_factory();
      // java this.mReflowCount = 0;
      self.mReflowCount = 0;
      // java layoutState.setFinishPage(false);
      layoutState.setFinishPage(false);
      // java this.mNeedCollectRangeForFirstRow = false;
      self.mNeedCollectRangeForFirstRow = false;
      // java this.mNeedCollectRangeForLastRow = false;
      self.mNeedCollectRangeForLastRow = false;
      // java this.mHasFlowTableHeader = false;
      self.mHasFlowTableHeader = false;
      // java this.mHasFlowNormalTable = false;
      self.mHasFlowNormalTable = false;
      // java this.mHasBreakRow = !layoutState.cellStateList.isEmpty();
      self.mHasBreakRow = !layoutState.cellStateList.isEmpty();
      // java this.initGraphBuff(layoutState);
      self.initGraphBuff(layoutState);
      // java this.makeSnapshot(layoutState);
      self.makeSnapshot(layoutState);
      // java CrashesLog.addEnv(layoutState);
      CrashesLog::addEnv(layoutState);
      // java this.preFlow(layoutState);
      self.preFlow(layoutState);
      // java while (!layoutState.isFinishPage() && !this.isComplete(layoutState))
      while !layoutState.isFinishPage() && !self.isComplete(layoutState) {
        // java this.flowLine(layoutState);
        self.flowLine(layoutState);
      }
      // java this.postFlow(layoutState);
      self.postFlow(layoutState);
      // java CrashesLog.removeEnv(layoutState);
      CrashesLog::removeEnv(layoutState);
    }

    // java protected boolean isComplete(LayoutState layoutState)
    pub fn isComplete(&self, layoutState: &LayoutState) -> bool {
      // java return layoutState.srcList.isEmpty();
      return layoutState.srcList.isEmpty();
    }

    // java protected void preFlow(LayoutState layoutState)
    pub fn preFlow(&mut self, layoutState: &mut LayoutState) {
      // java PageViewLayouter.preFlow runs its section/page alloc before super.preFlow.
      let (owner, prefix) = if let Some(page) = self.mPageLayouter.as_ref() {
        page.page_view_prefix()
      } else {
        (std::ptr::null_mut(), None)
      };
      if let Some(prefix) = prefix {
        if !owner.is_null() {
          prefix(owner, layoutState as *mut LayoutState);
        }
      }
      // java this.mReflowCount = 0;
      self.mReflowCount = 0;
      // java layoutState.setFinishPage(false);
      layoutState.setFinishPage(false);
      // java this.mNeedCollectRangeForFirstRow = false;
      self.mNeedCollectRangeForFirstRow = false;
      // java this.mNeedCollectRangeForLastRow = false;
      self.mNeedCollectRangeForLastRow = false;
      // java this.mHasFlowTableHeader = false;
      self.mHasFlowTableHeader = false;
      // java this.mHasFlowNormalTable = false;
      self.mHasFlowNormalTable = false;
      // java if (this.mLineLayouter != null)
      if let Some(mLineLayouter) = self.mLineLayouter.as_mut() {
        // java this.mLineLayouter.clearTextWrapLayouter();
        mLineLayouter.clearTextWrapLayouter();
      }
      // java this.mPageLayouter.start(layoutState.getEnvoy());
      let (page_view, on_start) = if let Some(page) = self.mPageLayouter.as_mut() {
        page.begin_page_view()
      } else {
        (std::ptr::null_mut(), None)
      };
      if let Some(on_start) = on_start {
        if !page_view.is_null() {
          on_start(page_view, layoutState as *mut LayoutState);
        }
      }
      // java layoutState.linePos = 0;
      layoutState.linePos = 0;
      // java this.mStartCP = layoutState.cp;
      self.mStartCP = layoutState.cp;
      // java int paraStart = this.mStartCP;
      let mut paraStart: i32 = self.mStartCP;
      // java ParaPropEnv curParaPropEnv = layoutState.paraEnv.curParaPropEnv;
      let curParaPropEnv = layoutState.paraEnv.curParaPropEnv.as_ref();
      // java if (paraStart != 0)
      if paraStart != 0 {
        // java if (null != curParaPropEnv && paraStart > curParaPropEnv.start && paraStart < curParaPropEnv.end)
        if let Some(curParaPropEnv) = curParaPropEnv {
          if paraStart > curParaPropEnv.start && paraStart < curParaPropEnv.end {
            // java paraStart = curParaPropEnv.start;
            paraStart = curParaPropEnv.start;
          } else {
            // java paraStart = layoutState.document.getPapxTable().seek(paraStart).position();
            paraStart = unsafe {
              let doc = layoutState.document.expect("document");
              let papx = (*doc).getPapxTable().expect("papx");
              IPapxTable::seek(&*papx, paraStart).expect("seek").position()
            };
          }
        } else {
          // java paraStart = layoutState.document.getPapxTable().seek(paraStart).position();
          paraStart = unsafe {
            let doc = layoutState.document.expect("document");
            let papx = (*doc).getPapxTable().expect("papx");
            IPapxTable::seek(&*papx, paraStart).expect("seek").position()
          };
        }
      }
      // java this.mRef = this.mMainState.snapshot.addCPRef(layoutState.document, paraStart);
      self.mRef = self.mMainState.snapshot.addCPRef(
        layoutState.document.map(|p| unsafe { &*p }),
        paraStart,
      );
      // java this.mNeedCollectRangeForFirstRow = !layoutState.cellStateList.isEmpty();
      self.mNeedCollectRangeForFirstRow = !layoutState.cellStateList.isEmpty();
      // java layoutState.affectRect.setEmpty();
      layoutState.affectRect.setEmpty();
      // java layoutState.maxWrapDrawingCP = Integer.MIN_VALUE;
      layoutState.maxWrapDrawingCP = i32::MIN;
      // java layoutState.pageEndLimitCP = Integer.MAX_VALUE;
      layoutState.pageEndLimitCP = i32::MAX;
      // java PageViewLayouter.preFlow continues after super.preFlow.
      let (owner, suffix) = if let Some(page) = self.mPageLayouter.as_ref() {
        page.page_view_suffix()
      } else {
        (std::ptr::null_mut(), None)
      };
      if let Some(suffix) = suffix {
        if !owner.is_null() {
          suffix(owner, layoutState as *mut LayoutState);
        }
      }
    }

    // java protected void postFlow(LayoutState layoutState)
    pub fn postFlow(&mut self, layoutState: &mut LayoutState) {
      // java this.collectPageRange(layoutState);
      self.collectPageRange(layoutState);
      // java SuperEnvoy envoy = layoutState.getEnvoy();
      // java this.mParaLayouter.end(envoy);
      if let Some(para) = self.mParaLayouter.as_mut() {
        para.end(layoutState.getEnvoy());
      }
      // java this.mPageLayouter.end(envoy);
      let (page_view, on_end) = if let Some(page) = self.mPageLayouter.as_mut() {
        page.finish_page_view()
      } else {
        (std::ptr::null_mut(), None)
      };
      if let Some(on_end) = on_end {
        if !page_view.is_null() {
          on_end(page_view, layoutState as *mut LayoutState);
        }
      }
      // java this.clearDrawing(layoutState);
      self.clearDrawing(layoutState);
      // java if (this.mHasFlowNormalTable)
      if self.mHasFlowNormalTable {
        // java TableCellBrcTool.removeOverlappedBrcs(this.mMainState.snapshot, layoutState.curPage.getLines());
        // rust: TableCellBrcTool off-surface (wr-27)
      }
      // java this.mLineLayouter.clearTextWrapLayouter();
      if let Some(line) = self.mLineLayouter.as_mut() {
        line.clearTextWrapLayouter();
      }
    }

    // java protected void clearDrawing(LayoutState layoutState)
    pub fn clearDrawing(&mut self, layoutState: &mut LayoutState) {
      let _ = layoutState;
    }

    // java protected void doFlowLine(LayoutState layoutState)
    pub fn doFlowLine(&mut self, layoutState: &mut LayoutState) {
      // Java ParaLayouter.mMainState is the same object PageViewSectionLayouter writes.
      let sec_start = self.mMainState.secStart;
      let sec_end = self.mMainState.secEnd;
      let sec_break = self.mMainState.secBreakCode;
      if let Some(para) = self.mParaLayouter.as_mut() {
        para.mMainState.secStart = sec_start;
        para.mMainState.secEnd = sec_end;
        para.mMainState.secBreakCode = sec_break;
      }
      // java if (this.mParaLayouter.hasStart())
      if self.mParaLayouter.as_ref().map(|p| p.hasStart()).unwrap_or(false) {
        // java this.flowTextLine(layoutState);
        self.flowTextLine(layoutState);
      }
      // java else
      else {
        // java this.mParaLayouter.start(layoutState.getEnvoy());
        if let Some(para) = self.mParaLayouter.as_mut() {
          para.start(layoutState.getEnvoy());
        }
        // java PropertySet paraProperty = layoutState.paraEnv.curParaPropEnv.paraProperty;
        let paraProperty = layoutState.paraEnv.curParaPropEnv.as_ref().and_then(|c| c.paraProperty.as_ref());
        // java if (this.isInTextFrame(layoutState))
        if self.isInTextFrame(layoutState) {
          // java this.flowTextFrame(layoutState);
          self.flowTextFrame(layoutState);
        }
        // java else if (GraphReaderTool.isInTable(paraProperty, layoutState.tablePeriLayer))
        else if paraProperty.map(|p| GraphReaderTool::isInTable(p, layoutState.tablePeriLayer)).unwrap_or(false) {
          // java this.flowTable(layoutState);
          self.flowTable(layoutState);
        }
        // java else
        else {
          // java this.flowTextLine(layoutState);
          self.flowTextLine(layoutState);
        }
      }
    }

    // java protected boolean isInTextFrame(LayoutState layoutState)
    pub fn isInTextFrame(&self, layoutState: &LayoutState) -> bool {
      // java return GraphReaderTool.isInTextFrame(layoutState, TAG, true);
      GraphReaderTool::isInTextFrame__layoutstate_string_boolean(
        layoutState,
        (*TAG).unwrap_or(""),
        true,
      )
    }

    // java protected final int getLastValidLineInPage(TypoPage page)
    pub fn getLastValidLineInPage__typopage(&self, page: Option<&crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage>) -> i32 {
      // Java: null page or absent line list has no valid line.
      let Some(page) = page else { return 0; };
      let p_list = page.getLines();
      if p_list == 0 { return 0; }
      let snapshot = &self.mMainState.snapshot;
      // Java: walk the snapshot list backwards, ignoring page ornaments,
      // deleted table rows, and repeated table header rows.
      for index in (0..TypoList::size__int_typosnapshot(p_list, snapshot)).rev() {
        let line = TypoList::get__int_int_typosnapshot(index, p_list, snapshot);
        let line_type = TypoMemoryObj::getType__int_typosnapshot(line, snapshot);
        let valid = self.getLastValidLineInPage__int_int_typosnapshot(line, line_type, snapshot);
        if valid != 0 { return valid; }
      }
      0
    }

    // java protected int getLastValidLineInPage(int curLine, int lineType, TypoSnapshot snapshot)
    pub fn getLastValidLineInPage__int_int_typosnapshot(&self, curLine: i32, lineType: i32, snapshot: &TypoSnapshot) -> i32 {
      // Java: line types 11, 4, and 10 are not visible content lines.
      if lineType == 11 || lineType == 4 || lineType == 10 { return 0; }
      if lineType == 5 {
        // Java: deleted rows and continuation headers do not end the page.
        if TypoTableRow::isRevisionDelete__int_typosnapshot(curLine, snapshot)
          || (TypoTableRow::isTableHeader__int_typosnapshot(curLine, snapshot)
            && !TypoTableRow::isFirstHeader__int_typosnapshot(curLine, snapshot)) {
          return 0;
        }
      }
      curLine
    }

    // java protected void doAfterFlowLine(LayoutState layoutState)
    pub fn doAfterFlowLine(&mut self, layoutState: &mut LayoutState) {
      // Java: only the break-row flag update is needed when this line has not
      // finished its page. The finish-page branch below remains a separate
      // dependency closure.
      if self.mHasBreakRow {
        // Java: int pList = layoutState.curPage.getLines();
        let p_list = layoutState.curPage.as_ref().map_or(0, |page| page.getLines());
        if p_list != 0 {
          let snapshot = &self.mMainState.snapshot;
          if crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(p_list, snapshot) > 0 {
            let last_line = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::last__int_typosnapshot(p_list, snapshot);
            let last_type = crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(last_line, snapshot);
            if last_type == 5 {
              if !TypoTableRow::isTableHeader__int_typosnapshot(last_line, snapshot)
                || TypoTableRow::isFirstHeader__int_typosnapshot(last_line, snapshot) {
                self.mHasBreakRow = false;
              }
            } else if last_type != 11 {
              self.mHasBreakRow = false;
            }
          }
        }
      }
      // Preserve the existing product guard until the Java finish-page
      // dependency closure below is implemented. This is not a parity pass.
      if !self.mHasBreakRow || !layoutState.isFinishPage() {
        return;
      }
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mHasBreakRow)
      if self.mHasBreakRow {
        // java int pList = layoutState.curPage.getLines();
        let mut pList: i32 = layoutState.curPage.getLines();
        // java if (0 != pList)
        if 0 != pList {
          // java TypoSnapshot snapshot = this.mMainState.snapshot;
          let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
          // java if (TypoList.size(pList, snapshot) > 0)
          if TypoList::size__int_typosnapshot(pList, snapshot) > 0 {
            // java int lastLine = TypoList.last(pList, snapshot);
            let mut lastLine: i32 = TypoList::last__int_typosnapshot(pList, snapshot);
            // java int lastLineType = TypoMemoryObj.getType(lastLine, snapshot);
            let mut lastLineType: i32 = TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot);
            // java if (5 == lastLineType)
            if 5 == lastLineType {
              // java if (!TypoTableRow.isTableHeader(lastLine, snapshot) || TypoTableRow.isFirstHeader(lastLine, snapshot))
              if !TypoTableRow::isTableHeader__int_typosnapshot(lastLine, snapshot) || TypoTableRow::isFirstHeader__int_typosnapshot(lastLine, snapshot) {
                // java this.mHasBreakRow = false;
                self.mHasBreakRow = false;
              }
            }
            // java else if (11 != lastLineType)
            else if 11 != lastLineType {
              // java this.mHasBreakRow = false;
              self.mHasBreakRow = false;
            }
          }
        }
      }
      // java if (layoutState.isFinishPage())
      if layoutState.isFinishPage() {
        // java if (layoutState.cellStateList.isEmpty())
        if layoutState.cellStateList.is_empty() {
          // java this.rollbackTrailingHiddenParagraphs(layoutState);
          self.rollbackTrailingHiddenParagraphs(layoutState);
        }
        // java if (layoutState.isInMathPara())
        if layoutState.isInMathPara() {
          // java this.processMathPara(layoutState);
          self.processMathPara(layoutState);
        }
        // java if (this.mEnableProcessPagination)
        if self.mEnableProcessPagination {
          // java this.processPagination(layoutState);
          self.processPagination(layoutState);
        }
        // java long res = this.clearRedundantDrawings(layoutState);
        let mut res: i64 = self.clearRedundantDrawings(layoutState);
        // java boolean hasInvalidateWrap = PrimitivePackTool.unpackFirstBoolean(res);
        let mut hasInvalidateWrap: bool = PrimitivePackTool::unpackFirstBoolean(res);
        // java int redundantWrapDrawingLimitCP = PrimitivePackTool.unpackSecondInt(res);
        let mut redundantWrapDrawingLimitCP: i32 = PrimitivePackTool::unpackSecondInt(res);
        // java if (redundantWrapDrawingLimitCP > 0)
        if redundantWrapDrawingLimitCP > 0 {
          // java if (hasInvalidateWrap)
          if hasInvalidateWrap {
            // java layoutState.pageEndLimitCP = Integer.MAX_VALUE;
            layoutState.pageEndLimitCP = i32::MAX;
          }
          // java else
          else {
            // java layoutState.pageEndLimitCP = Math.min(layoutState.pageEndLimitCP, redundantWrapDrawingLimitCP);
            layoutState.pageEndLimitCP = (layoutState.pageEndLimitCP).min(redundantWrapDrawingLimitCP);
          }
          // java this.mLineLayouter.clearTextWrapLayouter();
          self.mLineLayouter.clearTextWrapLayouter();
          // java this.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
          self.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
          // java boolean needCheckRangeEnd = layoutState.curPage instanceof TypoLayoutPage;
          let mut needCheckRangeEnd: bool = layoutState.curPage.downcast_ref::<TypoLayoutPage>().is_some();
          // java if (needCheckRangeEnd)
          if needCheckRangeEnd {
            // java int prevRangeEnd = layoutState.getRangeEnd();
            let mut prevRangeEnd: i32 = layoutState.getRangeEnd();
            // java this.rollbackLines(layoutState);
            self.rollbackLines(layoutState);
            // java if (layoutState.getRangeEnd() < prevRangeEnd)
            if layoutState.getRangeEnd() < prevRangeEnd {
              // java layoutState.setRangeEnd(prevRangeEnd);
              layoutState.setRangeEnd(prevRangeEnd);
            }
          }
          // java else
          else {
            // java this.rollbackLines(layoutState);
            self.rollbackLines(layoutState);
          }
        }
        // java else
        else {
          // java layoutState.setFinishPage(true);
          layoutState.setFinishPage(true);
          // java this.clearRedundantEmbeds(layoutState);
          self.clearRedundantEmbeds(layoutState);
        }
      }
      */
    }

    // java private void rollbackTrailingHiddenParagraphs(LayoutState layoutState)
    pub fn rollbackTrailingHiddenParagraphs(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int lastLine = this.getLastValidLineInPage(layoutState.curPage);
      let mut lastLine: i32 = self.getLastValidLineInPage__typopage(&layoutState.curPage);
      // java if (0 != lastLine)
      if 0 != lastLine {
        // java if (layoutState.cp != layoutState.document.getLength())
        if layoutState.cp != layoutState.document.getLength() {
          // java if (layoutState.cp != layoutState.getRangeEnd())
          if layoutState.cp != layoutState.getRangeEnd() {
            // java TypoSnapshot snapshot = this.mMainState.snapshot;
            let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
            // java int lastLineEndCp = TypoMemoryObj.getType(lastLine, snapshot) == 3 ? TypoTextLine.getEndCP(lastLine, snapshot) : TypoRangeBase.getEndCP(lastLine, snapshot);
            let mut lastLineEndCp: i32 = (if TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot) == 3 { TypoTextLine::getEndCP__int_typosnapshot(lastLine, snapshot) } else { TypoRangeBase::getEndCP__int_typosnapshot(lastLine, snapshot) });
            // java if (lastLineEndCp != layoutState.cp)
            if lastLineEndCp != layoutState.cp {
              // java int lines = layoutState.curPage.getLines();
              let mut lines: i32 = layoutState.curPage.getLines();
              // java int lastIndex = TypoList.indexOf(lastLine, lines, snapshot);
              let mut lastIndex: i32 = TypoList::indexOf(lastLine, lines, snapshot);
              // java int size = TypoList.size(lines, snapshot);
              let mut size: i32 = TypoList::size__int_typosnapshot(lines, snapshot);
              // java int i;
              let mut i: i32;
              i = lastIndex + 1;
              // java for(i = lastIndex + 1 i < size ++i)
              while i < size {
                // java int nextLine = TypoList.quickGet(i, lines, snapshot);
                let mut nextLine: i32 = TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
                // java if (11 != TypoMemoryObj.getType(nextLine, snapshot))
                if 11 != TypoMemoryObj::getType__int_typosnapshot(nextLine, snapshot) {
                  // java break;
                  break;
                }
              }
              // java layoutState.cp = lastLineEndCp;
              layoutState.cp = lastLineEndCp;
              // java layoutState.linePos = i;
              layoutState.linePos = i;
              // java if (i != size)
              if i != size {
                // java this.mDocEnv.cacheCleaner.cleanAfter(layoutState);
                self.mDocEnv.cacheCleaner.cleanAfter(layoutState);
              }
            }
          }
        }
      }
      */
    }

    // java protected void clearRedundantEmbeds(LayoutState layoutState)
    pub fn clearRedundantEmbeds(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ParaPropEnv paraPropEnv = layoutState.paraEnv.curParaPropEnv;
      let mut paraPropEnv: ParaPropEnv = layoutState.paraEnv.curParaPropEnv;
      // java if (paraPropEnv != null && layoutState.cp < paraPropEnv.end - 1 && layoutState.hasEmbed() && layoutState.typoPara.getEmbedSize() > 0)
      if true && layoutState.cp < paraPropEnv.end - 1 && layoutState.hasEmbed() && layoutState.typoPara.getEmbedSize() > 0 {
        // java layoutState.typoPara.clearRedundantEmbeds();
        layoutState.typoPara.clearRedundantEmbeds();
      }
      */
    }

    // java protected void processPagination(LayoutState layoutState)
    pub fn processPagination(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (layoutState.cp < layoutState.getRangeEnd())
      if layoutState.cp < layoutState.getRangeEnd() {
        // java int pLines = layoutState.curPage.getLines();
        let mut pLines: i32 = layoutState.curPage.getLines();
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java if (TypoList.size(pLines, snapshot) > 0)
        if TypoList::size__int_typosnapshot(pLines, snapshot) > 0 {
          // java int lastLinePos = TypoList.size(pLines, snapshot) - 1;
          let mut lastLinePos: i32 = TypoList::size__int_typosnapshot(pLines, snapshot) - 1;
          // java int lastLine = TypoList.last(pLines, snapshot);
          let mut lastLine: i32 = TypoList::last__int_typosnapshot(pLines, snapshot);
          // java int lastLineType = TypoMemoryObj.getType(lastLine, snapshot);
          let mut lastLineType: i32 = TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot);
          // java if (3 == lastLineType)
          if 3 == lastLineType {
            // java if (!TypoTextLine.isLastLineOfPara(lastLine, snapshot))
            if !TypoTextLine::isLastLineOfPara__int_typosnapshot(lastLine, snapshot) {
              // java PropertySet paraProperty = null == layoutState.paraEnv.curParaPropEnv ? null : layoutState.paraEnv.curParaPropEnv.paraProperty;
              let mut paraProperty: PropertySet = (if layoutState.paraEnv.curParaPropEnv.is_none() { PropertySet::default() } else { layoutState.paraEnv.curParaPropEnv.paraProperty });
              // java if (null == paraProperty)
              if false {
                // java int startCP = TypoTextLine.getStartCP(lastLine, this.mMainState.snapshot);
                let mut startCP: i32 = TypoTextLine::getStartCP__int_typosnapshot(lastLine, self.mMainState.snapshot);
                // java if (startCP >= 0)
                if startCP >= 0 {
                  // java IPapxTable.Entry para = layoutState.document.getPapxTable().seek(startCP);
                  let mut para: Box<dyn Entry> = layoutState.document.getPapxTable().seek(startCP);
                  // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(layoutState.document);
                  let mut propTool: Box<dyn IPropertyTool> = self.mDocEnv.propToolFactory.create(layoutState.document);
                  // java paraProperty = propTool.getFinalParaProp(para, this.mViewEnv.isRevisionFinal());
                  paraProperty = propTool.getFinalParaProp(para, self.mViewEnv.isRevisionFinal());
                }
              }
              // java if (null != paraProperty && paraProperty.getBoolean(197, false))
              if true && paraProperty.getBoolean(197, false) {
                // java int oldPageLimitCP = layoutState.pageEndLimitCP;
                let mut oldPageLimitCP: i32 = layoutState.pageEndLimitCP;
                // java int oldCP = layoutState.cp;
                let mut oldCP: i32 = layoutState.cp;
                // java layoutState.pageEndLimitCP = Integer.MAX_VALUE;
                layoutState.pageEndLimitCP = i32::MAX;
                // java if (!this.flowTextLineForCheckWidow(layoutState))
                if !self.flowTextLineForCheckWidow(layoutState) {
                  // java layoutState.cp = oldCP;
                  layoutState.cp = oldCP;
                  // java layoutState.pageEndLimitCP = oldPageLimitCP;
                  layoutState.pageEndLimitCP = oldPageLimitCP;
                  // java return;
                  return;
                }
                // java pLines = layoutState.curPage.getLines();
                pLines = layoutState.curPage.getLines();
                // java int nextLine = TypoList.last(pLines, snapshot);
                let mut nextLine: i32 = TypoList::last__int_typosnapshot(pLines, snapshot);
                // java if (TypoTextLine.isLastLineOfPara(nextLine, snapshot))
                if TypoTextLine::isLastLineOfPara__int_typosnapshot(nextLine, snapshot) {
                  // java --lastLinePos;
                  { let _tmp = lastLinePos; lastLinePos -= 1; _tmp };
                }
                // java this.rollbackLine(TypoList.size(pLines, snapshot) - 1, layoutState);
                self.rollbackLine(TypoList::size__int_typosnapshot(pLines, snapshot) - 1, layoutState);
                // java layoutState.setFinishPage(true);
                layoutState.setFinishPage(true);
              }
            }
            // java int firstRollbackLinePos = PaginationTool.findRollbackLine(lastLinePos, layoutState, this.mMainState.snapshot, this.mDocEnv.propToolFactory.create(layoutState.document), this.mViewEnv.isRevisionFinal(), layoutState.atLeastOneLine);
            let mut firstRollbackLinePos: i32 = PaginationTool::findRollbackLine(lastLinePos, layoutState, self.mMainState.snapshot, self.mDocEnv.propToolFactory.create(layoutState.document), self.mViewEnv.isRevisionFinal(), layoutState.atLeastOneLine);
            // java if (0 <= firstRollbackLinePos)
            if 0 <= firstRollbackLinePos {
              // java this.rollbackLine(firstRollbackLinePos, layoutState);
              self.rollbackLine(firstRollbackLinePos, layoutState);
              // java this.clearFootNoteRefCPs(layoutState);
              self.clearFootNoteRefCPs(layoutState);
            }
          }
        }
      }
      */
    }

    // java protected void processMathPara(LayoutState layoutState)
    pub fn processMathPara(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int pLines = layoutState.curPage.getLines();
      let mut pLines: i32 = layoutState.curPage.getLines();
      // java if (0 != pLines)
      if 0 != pLines {
        // java int size = TypoList.size(pLines, snapshot);
        let mut size: i32 = TypoList::size__int_typosnapshot(pLines, snapshot);
        // java if (size > 0)
        if size > 0 {
          // java int lastLine = TypoList.last(pLines, snapshot);
          let mut lastLine: i32 = TypoList::last__int_typosnapshot(pLines, snapshot);
          // java int lastLineType = TypoMemoryObj.getType(lastLine, snapshot);
          let mut lastLineType: i32 = TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot);
          // java if (3 == lastLineType)
          if 3 == lastLineType {
            // java if (TypoTextLine.isInMathPara(lastLine, snapshot) && !TypoTextLine.isLastLineOfMathPara(lastLine, snapshot))
            if TypoTextLine::isInMathPara(lastLine, snapshot) && !TypoTextLine::isLastLineOfMathPara(lastLine, snapshot) {
              // java int curLinePos = size - 1;
              let mut curLinePos: i32 = size - 1;
              // java int firstLineOfMathParaPos;
              let mut firstLineOfMathParaPos: i32;
              firstLineOfMathParaPos = -1;
              // java for(firstLineOfMathParaPos = -1 curLinePos >= 0 --curLinePos)
              while curLinePos >= 0 {
                // java if (TypoTextLine.isFirstLineOfMathPara(TypoList.get(curLinePos, pLines, snapshot), snapshot))
                if TypoTextLine::isFirstLineOfMathPara__int_typosnapshot(TypoList::get__int_int_typosnapshot(curLinePos, pLines, snapshot), snapshot) {
                  // java firstLineOfMathParaPos = curLinePos;
                  firstLineOfMathParaPos = curLinePos;
                  // java break;
                  break;
                }
              }
              // java if (firstLineOfMathParaPos < 0 || firstLineOfMathParaPos == 0 && layoutState.atLeastOneLine)
              if firstLineOfMathParaPos < 0 || firstLineOfMathParaPos == 0 && layoutState.atLeastOneLine {
                // java int mathParaJc = 1;
                let mut mathParaJc: i32 = 1;
                // java MathProperty mathProperty = (MathProperty)this.mDocEnv.textDocument.getProperty().getValue(642);
                let mut mathProperty: MathProperty = (self.mDocEnv.textDocument.getProperty().getValue(642)).downcast_ref::<MathProperty>().unwrap();
                // java if (null != mathProperty)
                if true {
                  // java PropertySet property = mathProperty.getMathPropertySet();
                  let mut property: PropertySet = mathProperty.getMathPropertySet();
                  // java mathParaJc = property.getInt(605, mathParaJc);
                  mathParaJc = property.getInt(605, mathParaJc);
                }
                // java PLCMathStart.OMathStartNode oMathStartNode = layoutState.getOMathStart(layoutState.cp);
                let mut oMathStartNode: PLCMathStart::OMathStartNode = layoutState.getOMathStart(layoutState.cp);
                // java if (null != oMathStartNode)
                if true {
                  // java Integer jc = oMathStartNode.getOMathParaPrJc();
                  let mut jc: i32 = oMathStartNode.getOMathParaPrJc();
                  // java if (null != jc)
                  if true {
                    // java mathParaJc = jc;
                    mathParaJc = jc;
                  }
                }
                // java if (mathParaJc == 1 && null != oMathStartNode)
                if mathParaJc == 1 && true {
                  // java int width = GroupCenterAlignTool.getMathWidth(layoutState.document, oMathStartNode.getCP(), this.mDocEnv, this.mViewEnv);
                  let mut width: i32 = GroupCenterAlignTool::getMathWidth(layoutState.document, oMathStartNode.getCP(), self.mDocEnv, self.mViewEnv);
                  // java ParaPropEnv curParaPropEnv = layoutState.paraEnv.curParaPropEnv;
                  let mut curParaPropEnv: ParaPropEnv = layoutState.paraEnv.curParaPropEnv;
                  // java if (null == curParaPropEnv)
                  if false {
                    // java curParaPropEnv = layoutState.paraEnv.prevParaPropEnv;
                    curParaPropEnv = layoutState.paraEnv.prevParaPropEnv;
                  }
                  // java if (null == curParaPropEnv)
                  if false {
                    // java return;
                    return;
                  }
                  // java int left = 0;
                  let mut left: i32 = 0;
                  // java if (curParaPropEnv.leftIndent > 0)
                  if curParaPropEnv.leftIndent > 0 {
                    // java left += curParaPropEnv.leftIndent;
                    left = left + curParaPropEnv.leftIndent;
                  }
                  // java if (curParaPropEnv.firstLineIndent < 0)
                  if curParaPropEnv.firstLineIndent < 0 {
                    // java left += curParaPropEnv.firstLineIndent;
                    left = left + curParaPropEnv.firstLineIndent;
                  }
                  // java int right = layoutState.pageEnv.getCoreWidth();
                  let mut right: i32 = layoutState.pageEnv.getCoreWidth();
                  // java if (curParaPropEnv.rightIndent > 0)
                  if curParaPropEnv.rightIndent > 0 {
                    // java right -= curParaPropEnv.rightIndent;
                    right = right - curParaPropEnv.rightIndent;
                  }
                  // java int offset = (right - left - width) / 2 - left;
                  let mut offset: i32 = right - left - width / 2 - left;
                  // java GroupCenterAlignTool.offset(Math.max(firstLineOfMathParaPos, 0), size - 1, offset, layoutState, snapshot);
                  GroupCenterAlignTool::offset__int_int_int_layoutstate_typosnapshot((firstLineOfMathParaPos).max(0), size - 1, offset, layoutState, snapshot);
                }
              }
              // java else
              else {
                // java this.rollbackLine(firstLineOfMathParaPos, layoutState);
                self.rollbackLine(firstLineOfMathParaPos, layoutState);
                // java this.clearFootNoteRefCPs(layoutState);
                self.clearFootNoteRefCPs(layoutState);
              }
            }
          }
        }
      }
      */
    }

    // java protected final void flowLine(LayoutState layoutState)
    pub fn flowLine(&mut self, layoutState: &mut LayoutState) {
      // java if (layoutState.cp < layoutState.pageEndLimitCP && layoutState.cp < layoutState.document.getLength())
      let doc_len = layoutState.document.map(|p| unsafe { (*p).getLength() }).unwrap_or(0);
      if layoutState.cp < layoutState.pageEndLimitCP && layoutState.cp < doc_len {
        // java this.doFlowLine(layoutState);
        self.doFlowLine(layoutState);
      }
      // java else
      else {
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
      // java this.doAfterFlowLine(layoutState);
      self.doAfterFlowLine(layoutState);
    }

    // java protected void initGraphBuff(LayoutState layoutState)
    pub fn initGraphBuff(&mut self, layoutState: &mut LayoutState) {
      // java this.mGraphBuff.clear(layoutState); GraphReader.doCleanGraphReader is the override.
      if let Some(buff) = self.mGraphBuff.as_mut() {
        buff.super_.super_.clear(layoutState);
        buff.super_.doCleanGraphReader();
      }
      // java layoutState.initCoreContent(layoutState.getRangeEnd(), this.mViewEnv.isRevisionFinal());
      layoutState.initCoreContent(layoutState.getRangeEnd(), self.mViewEnv.isRevisionFinal());
      // java this.mGraphBuff.initFilter — GraphReader override, not the empty GraphBuff body.
      if let Some(buff) = self.mGraphBuff.as_mut() {
        buff.super_.initFilter(layoutState.document, layoutState.cp, layoutState.getRangeEnd(), layoutState);
      }
    }

    // java protected boolean rollBackDropCap(LayoutState layoutState)
    pub fn rollBackDropCap(&mut self, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java protected void flowTextFrame(LayoutState layoutState)
    pub fn flowTextFrame(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mTextFrameLayouter)
      if self.mTextFrameLayouter.is_none() {
        // java this.initTextFrameLayouter();
        self.initTextFrameLayouter();
      }
      // java this.mTextFrameLayouter.flow(layoutState.getEnvoy());
      self.mTextFrameLayouter.flow(&mut layoutState.getEnvoy());
      // java this.mParaLayouter.end(layoutState.getEnvoy());
      self.mParaLayouter.end(layoutState.getEnvoy());
      // java if (layoutState.validLine)
      if layoutState.validLine {
        // java if (!this.mTextFrameLayouter.drawingHasInPage() && layoutState.curPage.getType() != 9)
        if !self.mTextFrameLayouter.drawingHasInPage() && layoutState.curPage.getType() != 9 {
          // java this.mLineLayouter.onAddWrapDrawing(this.mTextFrameLayouter.getDrawing(), layoutState.getEnvoy());
          self.mLineLayouter.onAddWrapDrawing(self.mTextFrameLayouter.getDrawing(), layoutState.getEnvoy());
        }
        // java if (this.mTextFrameLayouter.isDropCap() && layoutState.linePos > 1 && this.rollBackDropCap(layoutState))
        if self.mTextFrameLayouter.isDropCap() && layoutState.linePos > 1 && self.rollBackDropCap(layoutState) {
          // java return;
          return;
        }
        // java if (!layoutState.affectRect.isEmpty())
        if !layoutState.affectRect.is_empty() {
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return;
          return;
        }
        // java if (this.flowFootEndNotes(layoutState))
        if self.flowFootEndNotes(layoutState) {
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return;
          return;
        }
      }
      // java this.initGraphBuff(layoutState);
      self.initGraphBuff(layoutState);
      // java if (layoutState.cp >= layoutState.pageEndLimitCP)
      if layoutState.cp >= layoutState.pageEndLimitCP {
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
      // java if (this.needFinishPageForFootNotes(layoutState))
      if self.needFinishPageForFootNotes({ let __x: &LayoutState = todo!("no align strategy matched"); __x }) {
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
      */
    }

    // java protected void flowTable(LayoutState layoutState)
    pub fn flowTable(&mut self, layoutState: &mut LayoutState) {
      // java if (null == this.mTableLayouter)
      if self.mTableLayouter.is_none() {
        // java this.initTableLayouter();
        self.initTableLayouter();
      }
      // java int traceTableIdx = resolveTraceTableIndex(layoutState);
      let traceTableIdx = Self::resolveTraceTableIndex(layoutState);
      let _ = traceTableIdx;
      let trace_table = std::env::var_os("WORD_PARITY_TABLE_FLOW_TRACE").is_some()
        && (6334..=6653).contains(&layoutState.cp);
      if trace_table {
        eprintln!("table_flow_start cp={} layer={} cur_y={} max_h={} line_pos={}",
          layoutState.cp, layoutState.tablePeriLayer, layoutState.curLayoutHeight,
          layoutState.pageEnv.maxLayoutHeight, layoutState.linePos);
      }
      // java this.mTableLayouter.flow(this.mHasBreakRow, layoutState);
      if let Some(table) = self.mTableLayouter.as_mut() {
        table.flow(self.mHasBreakRow, layoutState);
      }
      if trace_table {
        eprintln!("table_flow_end cp={} layer={} cur_y={} max_h={} line_pos={} finish={} valid={}",
          layoutState.cp, layoutState.tablePeriLayer, layoutState.curLayoutHeight,
          layoutState.pageEnv.maxLayoutHeight, layoutState.linePos,
          layoutState.isFinishPage(), layoutState.validLine);
      }
      // java if (!layoutState.validLine)
      if !layoutState.validLine {
        // java if (this.flowFootEndNotes(layoutState))
        if self.flowFootEndNotes(layoutState) {
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return;
          return;
        }
      } else {
        let need_rollback = false;
        let p_row = {
          let lines = layoutState.curPage.as_ref().map(|page| page.getLines()).unwrap_or(0);
          let line_pos = layoutState.linePos;
          crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(
            line_pos, lines, &mut self.mMainState.snapshot,
          )
        };
        let is_wrap = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::isWrap__int_typosnapshot(
          p_row, &self.mMainState.snapshot,
        );
        if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
          eprintln!("wrap_table cp={} row={} is_wrap={} y={} line_pos={}", layoutState.cp, p_row, is_wrap, layoutState.curLayoutHeight, layoutState.linePos);
        }
        let mut need_rollback = need_rollback || (is_wrap && !layoutState.affectRect.isEmpty());
        // Java TextLayouter.flowTable: the committed wrap row makes its
        // drawing visible to the following text line immediately.
        if is_wrap {
          if let Some(line_layouter) = self.mLineLayouter.as_mut() {
            line_layouter.bindTextWrapEnvs(
              &mut self.mMainState as *mut MainState,
              &mut self.mDocEnv as *mut DocumentEnv,
              &mut self.mViewEnv as *mut ViewEnv,
            );
            line_layouter.initTextWrapLayouter(layoutState.getEnvoy());
          }
        }
        if !self.mMainState.floatsCache.is_empty() {
          if self.mDrawingLayouter.is_none() {
            self.initDrawingLayouter();
          }
          // java needRollback |= this.mDrawingLayouter.flowForTableRow(pRow, layoutState.getEnvoy());
          let row_rollback = self.with_drawing_envs(|drawing| {
            drawing.flowForTableRow(p_row, layoutState.getEnvoy())
          });
          need_rollback |= row_rollback;
        }
        // java if (needRollback && (!layoutState.isFinishPage() || layoutState.linePos != 0))
        if need_rollback && (!layoutState.isFinishPage() || layoutState.linePos != 0) {
          self.rollbackLines(layoutState);
          return;
        }
        let finish_merge = {
          let finished = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::hasFinishVertMergeCell(
            p_row, &self.mMainState.snapshot,
          );
          layoutState.isFinishPage() || finished
        };
        if finish_merge && self.flowFootEndNotes(layoutState) {
          self.rollbackLines(layoutState);
          return;
        }
        if layoutState.hasFlowTableHeader {
          self.mHasFlowTableHeader = true;
          self.mTableHeaderRange = layoutState.tableHeaderRange;
        }
        self.mHasFlowNormalTable = true;
        layoutState.linePos += 1;
      }
      // java this.initGraphBuff(layoutState);
      self.initGraphBuff(layoutState);
      // java if (layoutState.cp >= layoutState.pageEndLimitCP)
      if layoutState.cp >= layoutState.pageEndLimitCP {
        layoutState.setFinishPage(true);
      }
      // java if (this.needFinishPageForFootNotes(layoutState))
      if self.needFinishPageForFootNotes(layoutState) {
        layoutState.setFinishPage(true);
      }
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mTableLayouter)
      if self.mTableLayouter.is_none() {
        // java this.initTableLayouter();
        self.initTableLayouter();
      }
      // java int traceTableIdx = resolveTraceTableIndex(layoutState);
      let mut traceTableIdx: i32 = Self::resolveTraceTableIndex({ let __x: &LayoutState = todo!("no align strategy matched"); __x });
      // java LayoutTraceCollector.onTableStart(traceTableIdx, layoutState.curLayoutHeight);
      LayoutTraceCollector::onTableStart(traceTableIdx, layoutState.curLayoutHeight);
      // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"table_start\",\"cur_y\":" + layoutState.curLayoutHeight + ",\"cp\":" + layoutState.cp + "}");
      android.util.Log.d("WPS_PIPE", "{\"phase\":\"table_start\",\"cur_y\":" + layoutState.curLayoutHeight + ",\"cp\":" + layoutState.cp + "}");
      // java this.mTableLayouter.flow(this.mHasBreakRow, layoutState);
      self.mTableLayouter.flow({ let __x: &mut LayoutState = todo!("no align strategy matched"); __x }, layoutState);
      // java if (!layoutState.validLine)
      if !layoutState.validLine {
        // java if (this.flowFootEndNotes(layoutState))
        if self.flowFootEndNotes(layoutState) {
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return;
          return;
        }
      }
      // java else
      else {
        // java boolean needRollback = false;
        let mut needRollback: bool = false;
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int pRow = TypoList.get(layoutState.linePos, layoutState.curPage.getLines(), snapshot);
        let mut pRow: i32 = TypoList::get__int_int_typosnapshot(layoutState.linePos, layoutState.curPage.getLines(), snapshot);
        // java boolean isWrap = TypoTableRow.isWrap(pRow, snapshot);
        let mut isWrap: bool = TypoTableRow::isWrap__int_typosnapshot(pRow, snapshot);
        // java needRollback |= isWrap && !layoutState.affectRect.isEmpty();
        needRollback = needRollback | isWrap && !layoutState.affectRect.is_empty();
        // java if (isWrap)
        if isWrap {
          // java this.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
          self.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
        }
        // java if (!this.mMainState.floatsCache.isEmpty())
        if !self.mMainState.floatsCache.is_empty() {
          // java if (null == this.mDrawingLayouter)
          if self.mDrawingLayouter.is_none() {
            // java this.initDrawingLayouter();
            self.initDrawingLayouter();
          }
          // java needRollback |= this.mDrawingLayouter.flowForTableRow(pRow, layoutState.getEnvoy());
          needRollback = needRollback | self.mDrawingLayouter.flowForTableRow(pRow, layoutState.getEnvoy());
        }
        // java if (needRollback && (!layoutState.isFinishPage() || layoutState.linePos != 0))
        if needRollback && !layoutState.isFinishPage() || layoutState.linePos != 0 {
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return;
          return;
        }
        // java if ((layoutState.isFinishPage() || TypoTableRow.hasFinishVertMergeCell(pRow, snapshot)) && this.flowFootEndNotes(layoutState))
        if layoutState.isFinishPage() || TypoTableRow::hasFinishVertMergeCell(pRow, snapshot) && self.flowFootEndNotes(layoutState) {
          // java this.rollbackLines(layoutState);
          self.rollbackLines(layoutState);
          // java return;
          return;
        }
        // java if (layoutState.hasFlowTableHeader)
        if layoutState.hasFlowTableHeader {
          // java this.mHasFlowTableHeader = true;
          self.mHasFlowTableHeader = true;
          // java this.mTableHeaderRange = layoutState.tableHeaderRange;
          self.mTableHeaderRange = layoutState.tableHeaderRange;
        }
        // java this.mHasFlowNormalTable = true;
        self.mHasFlowNormalTable = true;
        // java ++layoutState.linePos;
        { let _tmp = layoutState.linePos; layoutState.linePos += 1; _tmp };
      }
      // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"table_end\",\"cur_y\":" + layoutState.curLayoutHeight + ",\"cp\":" + layoutState.cp + "}");
      android.util.Log.d("WPS_PIPE", "{\"phase\":\"table_end\",\"cur_y\":" + layoutState.curLayoutHeight + ",\"cp\":" + layoutState.cp + "}");
      // java LayoutTraceCollector.onTableEnd(traceTableIdx, layoutState.curLayoutHeight);
      LayoutTraceCollector::onTableEnd(traceTableIdx, layoutState.curLayoutHeight);
      // java this.initGraphBuff(layoutState);
      self.initGraphBuff(layoutState);
      // java if (layoutState.cp >= layoutState.pageEndLimitCP)
      if layoutState.cp >= layoutState.pageEndLimitCP {
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
      // java if (this.needFinishPageForFootNotes(layoutState))
      if self.needFinishPageForFootNotes({ let __x: &LayoutState = todo!("no align strategy matched"); __x }) {
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
      */
    }

    // java protected boolean needReflowLine(LayoutState layoutState)
    pub fn needReflowLine(&self, layoutState: &LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java private boolean doFlowTextLine(LayoutState layoutState)
    pub fn doFlowTextLine(&mut self, layoutState: &mut LayoutState) -> bool {
      // java long lineStart = 0L;
      // java int paraEnd = layoutState.paraEnv.curParaPropEnv.end;
      let paraEnd: i32 = layoutState.paraEnv.curParaPropEnv.as_ref().map(|e| e.end).unwrap_or(i32::MAX);
      // java if (layoutState.hasDrawing()) { ... mDrawingLayouter.addDrawings ... } —— drawing 非 hello（GR-D3）
      // java this.mLineLayouter.flow(layoutState);
      if let Some(ll) = self.mLineLayouter.as_mut() {
        ll.flow(layoutState);
      }
      // java if (this.needReflowLine(layoutState)) { this.mParaLayouter.end(layoutState.getEnvoy()); return false; }
      // rust: needReflowLine 恒 false
      // java boolean isParaEnd = layoutState.cp >= paraEnd;
      let isParaEnd: bool = layoutState.cp >= paraEnd;
      // java if (isParaEnd) { this.mParaLayouter.end(layoutState.getEnvoy()); }
      if isParaEnd {
        if let Some(para) = self.mParaLayouter.as_mut() {
          para.end(layoutState.getEnvoy());
        }
        // java if (null != this.mDrawingLayouter && this.doFlowDrawings(layoutState)) { return false; }
      }
      // java if (!layoutState.validLine)
      if !layoutState.validLine {
        // java layoutState.setFinishPage(true); return true;
        layoutState.setFinishPage(true);
        return true;
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      // java int lines = layoutState.curPage.getLines();
      // java if (layoutState.linePos >= TypoList.size(lines, snapshot)) { return false; }
      // A wrapped table inserts a type-5 anchor row in the snapshot list. The
      // product mirror contains text lines only, so its index is not linePos.
      let lines = layoutState.curPage.as_ref().map(|p| p.getLines()).unwrap_or(0);
      let page_lines: i32 = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(
        lines, &self.mMainState.snapshot,
      );
      if layoutState.linePos >= page_lines {
        return false;
      }
      // java int textLine = TypoList.quickGet(layoutState.linePos, lines, snapshot);
      // java float lineTop = TypoObjBase.getTop(textLine, snapshot);
      // java float lineHeight = TypoObjBase.getBottom(textLine, snapshot) - lineTop;
      // java int lineLeft = TypoObjBase.getLeft(textLine, snapshot);
      // java int lineWidth = TypoObjBase.getRight(textLine, snapshot) - lineLeft;
      // Java selects the snapshot slot, including a preceding type-5 anchor
      // row that the product text mirror does not contain.
      let line_ptr = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::quickGet__int_int_typosnapshot(
        layoutState.linePos, lines, &mut self.mMainState.snapshot,
      );
      let snapshot = &self.mMainState.snapshot;
      let graph_start = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getGraphIndexStart__int_typosnapshot(line_ptr, snapshot);
      let graph_end = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getGraphIndexEnd__int_typosnapshot(line_ptr, snapshot);
      let advances = if graph_end >= graph_start {
        (graph_start..=graph_end).filter_map(|gi| {
          let cp = layoutState.typoPara.getCP(gi);
          (cp >= 0).then_some((cp, layoutState.typoPara.getWidth(gi)))
        }).collect()
      } else {
        Vec::new()
      };
      layoutState.committed_line_advances.push(advances);
      let lineTop: i32 = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(line_ptr, snapshot);
      let lineHeight: i32 = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(line_ptr, snapshot).wrapping_sub(lineTop);
      let lineLeft: i32 = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(line_ptr, snapshot);
      let lineWidth: i32 = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getRight__int_typosnapshot(line_ptr, snapshot).wrapping_sub(lineLeft);
      // java layoutState.curLayoutHeight = TypoObjBase.getBottom(textLine, snapshot) + TypoTextLine.getLineAfter(textLine, snapshot);
      layoutState.curLayoutHeight = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(line_ptr, snapshot)
        .wrapping_add(crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getLineAfter__int_typosnapshot(line_ptr, snapshot));
      // java int lineStartCp = TypoTextLine.getStartCP(textLine, snapshot);
      let lineStartCp: i32 = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getStartCP__int_typosnapshot(line_ptr, snapshot);
      // java int lineEndCp = layoutState.cp;
      let lineEndCp: i32 = layoutState.cp;
      // java boolean lineFitsPageLimit = layoutState.cp <= layoutState.pageEndLimitCP;
      let lineFitsPageLimit: bool = layoutState.cp <= layoutState.pageEndLimitCP;
      // java if (!lineFitsPageLimit) { this.rollbackLine(layoutState.linePos, layoutState); layoutState.setFinishPage(true); }
      if !lineFitsPageLimit {
        // rust: rollbackLine（镜像：行回退 06 产品桥；此处线页限内不触发）
        layoutState.setFinishPage(true);
      }
      // java if (this.flowFootEndNotes(layoutState)) { this.rollbackLine(...); setFinishPage; return true; }
      // rust: 脚注非 hello（GR-D3）
      // java else
      // java LayoutTraceCollector.onStage2Line(documentType, linePos,
      //      lineStartCp, lineEndCp, lineLeft, lineTop, lineWidth, lineHeight);
      if lineFitsPageLimit {
        crate::cn::wps::moffice::writer::layout::base::LayoutTraceCollector::LayoutTraceCollector::onStage2Line(
          layoutState.document.map(|doc| unsafe { (*doc).getType__on_IDocument() }).unwrap_or(-1),
          layoutState.linePos,
          lineStartCp,
          lineEndCp,
          lineLeft,
          lineTop,
          lineWidth,
          lineHeight,
        );
      }
      // java ++layoutState.linePos;
      layoutState.linePos += 1;
      // java if (this.needFinishPageForFootNotes(layoutState)) { layoutState.setFinishPage(true); return true; }
      if self.needFinishPageForFootNotes(layoutState) {
        layoutState.setFinishPage(true);
        return true;
      }
      // java if (layoutState.cp == layoutState.pageEndLimitCP) { layoutState.setFinishPage(true); }
      if layoutState.cp == layoutState.pageEndLimitCP {
        layoutState.setFinishPage(true);
      }
      // java this.finishPage(textLine, layoutState); textLine is the snapshot pointer.
      self.finishPage__int_layoutstate(line_ptr, layoutState);
      // java return layoutState.isFinishPage();
      let _ = (lineEndCp, lineStartCp, lineHeight, lineTop);
      layoutState.isFinishPage()
    }

    // java protected final boolean hasGraphs(LayoutState layoutState)
    pub fn hasGraphs(&self, layoutState: &LayoutState) -> bool {
      // java return !layoutState.getGraphList().isEmpty();
      !layoutState.graphList.isEmpty()
    }

    // java protected void flowTextLine(LayoutState layoutState)
    pub fn flowTextLine(&mut self, layoutState: &mut LayoutState) {
      // java long prepareGraphStart = 0L;
      // java this.prepareGraphs(layoutState);
      self.prepareGraphs(layoutState);
      // java if (this.hasGraphs(layoutState))
      if self.hasGraphs(layoutState) {
        // java KPList<Graphs> graphList = layoutState.graphList;
        // java TypoPara typoPara = layoutState.typoPara;
        // java Graphs graphs = (Graphs)graphList.lastElement();
        // java Graphs graphs = (Graphs)graphList.lastElement();
        let auto_num: bool = {
          let graphList = &mut layoutState.graphList;
          if graphList.is_empty() {
            false
          } else {
            let lastElement: *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs = graphList.lastElement();
            unsafe { (*lastElement).isAutoNum() }
          }
        };
        // java if (graphs.isAutoNum()) { graphList.clean(); typoPara.clear(); }
        if auto_num {
          // rust: 自动编号图清空（非 hello，GR-D3；Java 原文见 TextLayouter.java:788-793）
          layoutState.graphList.clean();
          layoutState.typoPara.clear();
        }
      }
      // java if (!this.hasGraphs(layoutState))
      if !self.hasGraphs(layoutState) {
        // java if (!layoutState.revisions.isEmpty()) { ... mMainState.revisions.addRunRevisions ... }
        // rust: 修订收集（hello 无修订；Java 原文见 TextLayouter.java:795-810）
        // java ParaEnv mParaEnv = layoutState.paraEnv; firstLineOfPara ...
        let firstLineOfPara: bool = layoutState.paraEnv.isFirstLineOfPara || layoutState.paraEnv.firstLineIsSecBreak;
        // java Revision revision = (Revision)mParaEnv.curParaPropEnv.paraProperty.getValue(237);
        let revision = layoutState.paraEnv.curParaPropEnv.as_ref()
          .and_then(|e| e.paraProperty.as_ref())
          .and_then(|p| p.getValue(237));
        // java if (null != revision) { ... addRevisionChangePara ... }
        if revision.is_some() {
          // rust: 修订（D：GR-D3；Java 原文见 TextLayouter.java:811-819）
          let _ = firstLineOfPara;
        }
        // java float beforeY = layoutState.curLayoutHeight;
        // java this.onEmptyGraphSize(layoutState);
        self.onEmptyGraphSize(layoutState);
        // java LayoutTraceCollector.onEmptyParagraph(beforeY, afterY - beforeY);
      }
      // java else
      else {
        // java boolean isFinishPage = this.doFlowTextLine(layoutState);
        let isFinishPage: bool = self.doFlowTextLine(layoutState);
        // java if (isFinishPage && this.shouldFlowDrawingsBeforeFinishPage(layoutState) && null != this.mDrawingLayouter)
        // rust: drawing 分支（GR-D3）
        let _ = isFinishPage;
      }
    }

    // java private boolean doFlowDrawings(LayoutState layoutState)
    pub fn doFlowDrawings(&mut self, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int maxAnchorCP = layoutState.cp;
      let mut maxAnchorCP: i32 = layoutState.cp;
      // java maxAnchorCP = Math.min(maxAnchorCP, layoutState.pageEndLimitCP);
      maxAnchorCP = (maxAnchorCP).min(layoutState.pageEndLimitCP);
      // java int lastRow = 0;
      let mut lastRow: i32 = 0;
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java if (layoutState.isFinishPage())
      if layoutState.isFinishPage() {
        // java int lastLine = this.getLastValidLineInPage(layoutState.curPage);
        let mut lastLine: i32 = self.getLastValidLineInPage__typopage(&layoutState.curPage);
        // java if (0 != lastLine)
        if 0 != lastLine {
          // java switch (TypoMemoryObj.getType(lastLine, snapshot))
          match TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot) {
            // java case 3:
            3 => {
              // java maxAnchorCP = Math.min(maxAnchorCP, TypoTextLine.getEndCP(lastLine, snapshot));
              maxAnchorCP = (maxAnchorCP).min(TypoTextLine::getEndCP__int_typosnapshot(lastLine, snapshot));
            }
            // java case 5:
            5 => {
              // java lastRow = lastLine;
              lastRow = lastLine;
            }
            // java default:
            _ => {
              // java maxAnchorCP = Math.min(maxAnchorCP, TypoRangeBase.getEndCP(lastLine, snapshot));
              maxAnchorCP = (maxAnchorCP).min(TypoRangeBase::getEndCP__int_typosnapshot(lastLine, snapshot));
            }
          }
        }
      }
      // java int oldWrapSize = this.mMainState.getWrapDrawingCount(layoutState.document);
      let mut oldWrapSize: i32 = self.mMainState.getWrapDrawingCount(layoutState.document);
      // java boolean isRollbacked = 0 != lastRow ? this.mDrawingLayouter.flowForTableRow(lastRow, layoutState.getEnvoy()) : this.mDrawingLayouter.flowBeforeCP(maxAnchorCP, layoutState.getEnvoy());
      let mut isRollbacked: bool = (if 0 != lastRow { self.mDrawingLayouter.flowForTableRow(lastRow, layoutState.getEnvoy()) } else { self.mDrawingLayouter.flowBeforeCP(maxAnchorCP, layoutState.getEnvoy()) });
      // java if (this.mMainState.getWrapDrawingCount(layoutState.document) != oldWrapSize)
      if self.mMainState.getWrapDrawingCount(layoutState.document) != oldWrapSize {
        // java this.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
        self.mLineLayouter.initTextWrapLayouter(layoutState.getEnvoy());
      }
      // java if (isRollbacked)
      if isRollbacked {
        // java this.rollbackLines(layoutState);
        self.rollbackLines(layoutState);
        // java if (null != this.mFootEndNoteLayouter)
        if let Some(mFootEndNoteLayouter) = self.mFootEndNoteLayouter.as_mut() {
          // java this.mFootEndNoteLayouter.adjustMaxLayoutHeight(layoutState);
          mFootEndNoteLayouter.adjustMaxLayoutHeight(layoutState);
        }
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

    // java protected boolean shouldFlowDrawingsBeforeFinishPage(LayoutState layoutState)
    pub fn shouldFlowDrawingsBeforeFinishPage(&self, layoutState: &LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return !this.mMainState.floatsCache.isEmpty();
      return !self.mMainState.floatsCache.is_empty();
      */
    }

    // java protected void prepareGraphs(LayoutState layoutState)
    pub fn prepareGraphs(&mut self, layoutState: &mut LayoutState) {
      // java int wordAlingFont = layoutState.paraEnv.wordAlignFont;
      let mut wordAlingFont: i32 = layoutState.paraEnv.wordAlignFont;
      // Java GraphBuff.mMainState is the same MainState PageViewSectionLayouter writes.
      let sec_start = self.mMainState.secStart;
      let sec_end = self.mMainState.secEnd;
      let sec_break = self.mMainState.secBreakCode;
      if let Some(gf) = self.mGraphBuff.as_mut() {
        if let Some(ms) = gf.super_.super_.mMainState.as_mut() {
          ms.secStart = sec_start;
          ms.secEnd = sec_end;
          ms.secBreakCode = sec_break;
        }
      }
      // java int textFlow = layoutState.pageEnv.textFlow;
      let textFlow: i32 = layoutState.pageEnv.textFlow;
      // java if (4 == wordAlingFont && 0 != textFlow && 4 != textFlow)
      if 4 == wordAlingFont && 0 != textFlow && 4 != textFlow {
        // java wordAlingFont = 1;
        wordAlingFont = 1;
      }
      // java if (this.mGraphBuff.prepareGraphs(wordAlingFont, layoutState)) { this.prepareEmbeds(wordAlingFont, layoutState); }
      let prepared: bool = if let Some(gb) = self.mGraphBuff.as_mut() {
        gb.super_.prepareGraphs(wordAlingFont, layoutState)
      } else {
        false
      };
      // java if (this.mGraphBuff.prepareGraphs(...)) { this.prepareEmbeds(wordAlingFont, layoutState); }
      if prepared {
        self.prepareEmbeds(wordAlingFont, layoutState);
      }
    }

    // java protected void initSmartFontSize(LayoutState layoutState)
    pub fn initSmartFontSize(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean smartFontSize = this.mViewEnv.isSmartFontSize() && layoutState.curPage.isBodyPage();
      let mut smartFontSize: bool = self.mViewEnv.isSmartFontSize() && layoutState.curPage.isBodyPage();
      // java FontStyleFactory.setSmartFontSize(smartFontSize);
      FontStyleFactory::setSmartFontSize(smartFontSize);
      */
    }

    // java protected final void onEmptyGraphSize(LayoutState layoutState)
    pub fn onEmptyGraphSize(&mut self, layoutState: &mut LayoutState) {
      // java int nextParaStart = layoutState.paraEnv.curParaPropEnv.end;
      let mut nextParaStart: i32 = layoutState.paraEnv.curParaPropEnv.as_ref().map(|e| e.end).unwrap_or(0);
      // java if (layoutState.pageEndLimitCP < nextParaStart)
      if layoutState.pageEndLimitCP < nextParaStart {
        // java layoutState.setFinishPage(true); this.mGraphBuff.clear(layoutState);
        layoutState.setFinishPage(true);
        if let Some(gb) = self.mGraphBuff.as_mut() {
          gb.super_.super_.clear(layoutState);
          gb.super_.doCleanGraphReader();
        }
      }
      // java else
      else {
        // java this.mParaLayouter.end(layoutState.getEnvoy());
        if let Some(para) = self.mParaLayouter.as_mut() {
          para.end(layoutState.getEnvoy());
        }
        // java layoutState.paraEnv.clearBrcAndShdAdjustInfo(this.mDocEnv.pools);
        // rust: 段边框调整收集（非 hello，GR-D3）
        // java nextParaStart = Math.max(nextParaStart, layoutState.cp);
        nextParaStart = nextParaStart.max(layoutState.cp);
        // java layoutState.cp = nextParaStart;
        layoutState.cp = nextParaStart;
        // java if (nextParaStart < layoutState.document.getLength() && nextParaStart < layoutState.getRangeEnd())
        let doc_len: i32 = layoutState.document.map(|d| unsafe { (*d).getLength() }).unwrap_or(-1);
        if nextParaStart < doc_len && nextParaStart < layoutState.getRangeEnd() {
          // java this.initGraphBuff(layoutState);
          self.initGraphBuff(layoutState);
        } else {
          // java layoutState.setFinishPage(true); this.mGraphBuff.clear(layoutState);
          layoutState.setFinishPage(true);
          if let Some(gb) = self.mGraphBuff.as_mut() {
            gb.super_.super_.clear(layoutState);
            gb.super_.doCleanGraphReader();
          }
        }
      }
    }

    // java protected void rollbackLines(LayoutState layoutState)
    pub fn rollbackLines(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.rewindLineIteratorBeforeRollback(layoutState);
      self.rewindLineIteratorBeforeRollback(layoutState);
      // java this.doRollbackLines(layoutState, true);
      self.doRollbackLines(layoutState, true);
      */
    }

    // java private void rewindLineIteratorBeforeRollback(LayoutState layoutState)
    pub fn rewindLineIteratorBeforeRollback(&mut self, layoutState: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean hasFootEndNote = false;
      let mut hasFootEndNote: bool = false;
      // java int firstRollbackLineIndex = -1;
      let mut firstRollbackLineIndex: i32 = -1;
      // java layoutState.linePos = 0;
      layoutState.linePos = 0;
      // java int affectTop = layoutState.affectRect.top;
      let mut affectTop: i32 = layoutState.affectRect.top;
      // java int pLines = layoutState.curPage.getLines();
      let mut pLines: i32 = layoutState.curPage.getLines();
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int size;
      let mut size: i32;
      size = TypoList::size__int_typosnapshot(pLines, snapshot);
      // java for(size = TypoList.size(pLines, snapshot) layoutState.linePos < size ++layoutState.linePos)
      while layoutState.linePos < size {
        // java int pLine = TypoList.get(layoutState.linePos, pLines, snapshot);
        let mut pLine: i32 = TypoList::get__int_int_typosnapshot(layoutState.linePos, pLines, snapshot);
        // java if (-1 == firstRollbackLineIndex && TypoObjBase.getBottom(pLine, snapshot) > affectTop)
        if -1 == firstRollbackLineIndex && TypoObjBase::getBottom__int_typosnapshot(pLine, snapshot) > affectTop {
          // java firstRollbackLineIndex = layoutState.linePos;
          firstRollbackLineIndex = layoutState.linePos;
          // java if (TypoMemoryObj.getType(pLine, snapshot) == 11)
          if TypoMemoryObj::getType__int_typosnapshot(pLine, snapshot) == 11 {
            // java hasFootEndNote = true;
            hasFootEndNote = true;
          }
          // java break;
          break;
        }
      }
      // java if (-1 != firstRollbackLineIndex && size - 1 != firstRollbackLineIndex)
      if -1 != firstRollbackLineIndex && size - 1 != firstRollbackLineIndex {
        let mut it: i32 = firstRollbackLineIndex + 1;
        // java for(int it = firstRollbackLineIndex + 1; it < size ++it)
        while it < size {
          // java int pLine = TypoList.get(it, pLines, snapshot);
          let mut pLine: i32 = TypoList::get__int_int_typosnapshot(it, pLines, snapshot);
          // java if (TypoMemoryObj.getType(pLine, snapshot) == 11)
          if TypoMemoryObj::getType__int_typosnapshot(pLine, snapshot) == 11 {
            // java hasFootEndNote = true;
            hasFootEndNote = true;
            // java break;
            break;
          }
        }
      }
      // java if (!hasFootEndNote && -1 != firstRollbackLineIndex)
      if !hasFootEndNote && -1 != firstRollbackLineIndex {
        // java while (-1 != firstRollbackLineIndex && 5 == TypoMemoryObj.getType(TypoList.get(firstRollbackLineIndex, pLines, snapshot), snapshot))
        while -1 != firstRollbackLineIndex && 5 == TypoMemoryObj::getType__int_typosnapshot(TypoList::get__int_int_typosnapshot(firstRollbackLineIndex, pLines, snapshot), snapshot) {
          // java int prveLineIndex = firstRollbackLineIndex - 1;
          let mut prveLineIndex: i32 = firstRollbackLineIndex - 1;
          // java if (prveLineIndex == -1)
          if prveLineIndex == -1 {
            // java break;
            break;
          }
          // java int prevLine = TypoList.get(prveLineIndex, pLines, snapshot);
          let mut prevLine: i32 = TypoList::get__int_int_typosnapshot(prveLineIndex, pLines, snapshot);
          // java int prevTableRow = 5 == TypoMemoryObj.getType(prevLine, snapshot) ? prevLine : 0;
          let mut prevTableRow: i32 = (if 5 == TypoMemoryObj::getType__int_typosnapshot(prevLine, snapshot) { prevLine } else { 0 });
          // java if (prevTableRow != 0 && TypoTableRow.isTableHeader(prevTableRow, snapshot) && !TypoTableRow.isFirstHeader(prevTableRow, snapshot))
          if prevTableRow != 0 && TypoTableRow::isTableHeader__int_typosnapshot(prevTableRow, snapshot) && !TypoTableRow::isFirstHeader__int_typosnapshot(prevTableRow, snapshot) {
            // java firstRollbackLineIndex = prveLineIndex - 1;
            firstRollbackLineIndex = prveLineIndex - 1;
          }
          // java else
          else {
            // java int tableRow = TypoList.get(firstRollbackLineIndex, pLines, snapshot);
            let mut tableRow: i32 = TypoList::get__int_int_typosnapshot(firstRollbackLineIndex, pLines, snapshot);
            // java if (!TypoTableRow.hasVertMergeCell(tableRow, snapshot) && (!TypoTableRow.isTableHeader(tableRow, snapshot) || TypoTableRow.isFirstHeader(tableRow, snapshot)))
            if !TypoTableRow::hasVertMergeCell(tableRow, snapshot) && !TypoTableRow::isTableHeader__int_typosnapshot(tableRow, snapshot) || TypoTableRow::isFirstHeader__int_typosnapshot(tableRow, snapshot) {
              // java break;
              break;
            }
            // java firstRollbackLineIndex = prveLineIndex;
            firstRollbackLineIndex = prveLineIndex;
          }
        }
        // java if (-1 != firstRollbackLineIndex)
        if -1 != firstRollbackLineIndex {
          // java layoutState.linePos = firstRollbackLineIndex;
          layoutState.linePos = firstRollbackLineIndex;
        }
        // java else
        else {
          // java layoutState.linePos = 0;
          layoutState.linePos = 0;
        }
      }
      // java else
      else {
        // java layoutState.linePos = 0;
        layoutState.linePos = 0;
      }
      */
    }

    // java private void doRollbackLines(LayoutState layoutState, boolean canClearFootNoteRefCPs)
    pub fn doRollbackLines(&mut self, layoutState: &mut LayoutState, canClearFootNoteRefCPs: bool) {
      // java layoutState.setFinishPage(false);
      layoutState.setFinishPage(false);
      // java this.mParaLayouter.end(layoutState.getEnvoy());
      if let Some(para) = self.mParaLayouter.as_mut() {
        para.end(layoutState.getEnvoy());
      }
      // java this.mDocEnv.pools.recycleTableCellLayoutStates(layoutState.cellStateList);
      let drained = layoutState.cellStateList.drain();
      self.mDocEnv.pools.recycleTableCellLayoutStates(drained);
      // java if (layoutState.linePos == 0)
      let mut rebuild_row: i32 = 0;
      if layoutState.linePos == 0 {
        // java this.loadSnapshot(layoutState);
        self.loadSnapshot(layoutState);
        // java this.mHasBreakRow = !layoutState.cellStateList.isEmpty();
        self.mHasBreakRow = !layoutState.cellStateList.isEmpty();
        // java layoutState.curLayoutHeight = layoutState.pageEnv.minLayoutHeight;
        layoutState.curLayoutHeight = layoutState.pageEnv.minLayoutHeight;
      } else {
        // java if (canClearFootNoteRefCPs) this.clearFootNoteRefCPs(layoutState);
        if canClearFootNoteRefCPs {
          self.clearFootNoteRefCPs(layoutState);
        }
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let lines = layoutState.curPage.as_ref().map(|page| page.getLines()).unwrap_or(0);
        let snapshot = &self.mMainState.snapshot;
        // java int prevLine = TypoList.get(layoutState.linePos - 1, layoutState.curPage.getLines(), snapshot);
        let prev_line = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(layoutState.linePos - 1, lines, snapshot);
        // java layoutState.curLayoutHeight = TypoObjBase.getBottom(prevLine, snapshot);
        layoutState.curLayoutHeight = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(prev_line, snapshot);
        // java switch (TypoMemoryObj.getType(prevLine, snapshot))
        // case 3 falls through empty cases into default break; it does not enter case 5.
        match crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(prev_line, snapshot) {
          3 => {
            // java layoutState.cp = TypoTextLine.getEndCP(prevLine, snapshot);
            layoutState.cp = crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getEndCP__int_typosnapshot(prev_line, snapshot);
            // java layoutState.curLayoutHeight += TypoTextLine.getLineAfter(prevLine, snapshot);
            layoutState.curLayoutHeight += crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getLineAfter__int_typosnapshot(prev_line, snapshot);
          }
          // java case 5: Tools.createCellEnvsForTableRow(layoutState, prevLine, true, true, this.mDocEnv, this.mMainState);
          5 => rebuild_row = prev_line,
          // java case 10: TypoTextFrame.getEndCP; case 13: TypoColumns.getEndCP — both TypoRangeBase.getEndCP
          10 | 13 => {
            layoutState.cp = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getEndCP__int_typosnapshot(prev_line, snapshot);
          }
          // java case 11: Assert.assertTrue(..., false)
          11 => {
            debug_assert!(false, "由于限制了存在尾注的页面必要整页重排。所以不应该走到这里");
          }
          _ => {}
        }
      }
      if rebuild_row != 0 {
        crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::Tools::createCellEnvsForTableRow(
          layoutState,
          rebuild_row,
          true,
          true,
          &mut self.mDocEnv,
          &mut self.mMainState,
        );
      }
      // java this.mDocEnv.cacheCleaner.cleanAfter(layoutState);
      let main = &mut self.mMainState as *mut MainState;
      let doc = &mut self.mDocEnv as *mut DocumentEnv;
      unsafe {
        (*doc).cacheCleaner.bind_env(main, doc);
        (*doc).cacheCleaner.cleanAfter(layoutState);
      }
      // java this.initGraphBuff(layoutState);
      self.initGraphBuff(layoutState);
      // java layoutState.affectRect.setEmpty();
      layoutState.affectRect.setEmpty();
      // java layoutState.paraEnv.clearBrcAndShdAdjustInfo(this.mDocEnv.pools);
      layoutState.paraEnv.clearBrcAndShdAdjustInfo(&mut self.mDocEnv.pools);
    }

    // java protected void clearFootNoteRefCPs(LayoutState layoutState)
    pub fn clearFootNoteRefCPs(&mut self, layoutState: &mut LayoutState) {
      // java body is empty
      let _ = layoutState;
    }

    // java protected void rollbackLine(int textLinePos, LayoutState layoutState)
    pub fn rollbackLine(&mut self, textLinePos: i32, layoutState: &mut LayoutState) {
      // java layoutState.linePos = textLinePos;
      layoutState.linePos = textLinePos;
      // java this.doRollbackLines(layoutState, true);
      self.doRollbackLines(layoutState, true);
    }

    // java protected boolean isLastLineOfPage(int line)
    pub fn isLastLineOfPage(&self, line: i32) -> bool {
      // java switch (TypoMemoryObj.getType(line, snapshot)) case 3
      let snapshot = &self.mMainState.snapshot;
      let typ = if line == 0 { -1 } else { snapshot.get__int(line) };
      if typ == 3 {
        crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::isLastLineOfSection__int_typosnapshot(line, snapshot)
          || crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::isLastLineOfPage(line, snapshot)
          || crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::isLastLineOfColumn(line, snapshot)
      } else {
        false
      }
    }

    // java protected void finishPage(int line, LayoutState layoutState)
    pub fn finishPage__int_layoutstate(&mut self, line: i32, layoutState: &mut LayoutState) {
      // java if (this.isLastLineOfPage(line) && this.finishPage(layoutState))
      if self.isLastLineOfPage(line) && self.finishPage__layoutstate(layoutState) {
        // java layoutState.setFinishPage(true);
        layoutState.setFinishPage(true);
      }
    }

    // java protected boolean finishPage(LayoutState layoutState)
    pub fn finishPage__layoutstate(&mut self, layoutState: &mut LayoutState) -> bool {
      // java return false; PageViewLayouter overrides this.
      let (owner, finish) = if let Some(page) = self.mPageLayouter.as_ref() {
        page.page_view_finish()
      } else {
        (std::ptr::null_mut(), None)
      };
      if let Some(finish) = finish {
        if !owner.is_null() {
          return finish(owner, layoutState as *mut LayoutState);
        }
      }
      false
    }

    // java protected boolean shouldInvalidateRedundantWrapDrawing(int lastVisibleLine, int drawing, int anchorCP, int limitCP, LayoutState layoutState)
    pub fn shouldInvalidateRedundantWrapDrawing(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32, limitCP: i32, layoutState: &LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java private boolean doClearRedundantDrawings(int lastVisibleLine, WIntArrayList redundantDrawings, int maxRedundantWrapDrawingCP, int maxLimitCP, LayoutState layoutState)
    pub fn doClearRedundantDrawings(&mut self, lastVisibleLine: i32, redundantDrawings: &WIntArrayList, maxRedundantWrapDrawingCP: i32, maxLimitCP: i32, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoRectCoreImpl affectRect = layoutState.affectRect;
      let mut affectRect: TypoRectCoreImpl = layoutState.affectRect;
      // java boolean hasInvalidatedWrap = false;
      let mut hasInvalidatedWrap: bool = false;
      // java if (redundantDrawings.isEmpty())
      if redundantDrawings.isEmpty() {
        // java return hasInvalidatedWrap;
        return hasInvalidatedWrap;
      }
      // java else
      else {
        // java Pools pools = this.mDocEnv.pools;
        let mut pools: Pools = self.mDocEnv.pools;
        // java CacheCleaner cleaner = this.mDocEnv.cacheCleaner;
        let mut cleaner: CacheCleaner = self.mDocEnv.cacheCleaner;
        // java Document doc = layoutState.document;
        let mut doc: Box<dyn Document> = layoutState.document;
        // java KList<DrawingCacheItem> floatsCache = this.mMainState.floatsCache;
        let mut floatsCache: KList = self.mMainState.floatsCache;
        // java int redundantDrawingCount = redundantDrawings.size();
        let mut redundantDrawingCount: i32 = redundantDrawings.size();
        // java TypoRectCoreImpl layoutRect = TypoRectCoreImpl.obtain();
        let mut layoutRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java TypoDrawingLayers drawingLayers = this.mMainState.drawingLayers;
        let mut drawingLayers: TypoDrawingLayers = self.mMainState.drawingLayers;
        // java TypoDrawing drawing = snapshot.getTypoTools().obtainTypoDrawing();
        let mut drawing: TypoDrawing = snapshot.getTypoTools().obtainTypoDrawing();
        let mut i: i32 = 0;
        // java for(int i = 0; i < redundantDrawingCount; ++i)
        while i < redundantDrawingCount {
          // java drawing.assignment(redundantDrawings.getQuick(i), snapshot);
          drawing.assignment(redundantDrawings.getQuick(i), snapshot);
          // java int anchorCP = drawing.getCP();
          let mut anchorCP: i32 = drawing.getCP();
          // java if (anchorCP >= maxRedundantWrapDrawingCP)
          if anchorCP >= maxRedundantWrapDrawingCP {
            // java if (drawing.isWrap())
            if drawing.isWrap() {
              // java drawing.layoutRectForPageCore(layoutRect);
              drawing.layoutRectForPageCore(layoutRect);
              // java DrawingAffectTool.unionAffectRect(affectRect, layoutRect);
              DrawingAffectTool::unionAffectRect(affectRect, layoutRect);
              // java if (this.shouldInvalidateRedundantWrapDrawing(lastVisibleLine, drawing.getPointer(), anchorCP, maxLimitCP, layoutState))
              if self.shouldInvalidateRedundantWrapDrawing(lastVisibleLine, drawing.getPointer(), anchorCP, maxLimitCP, { let __x: &LayoutState = todo!("no align strategy matched"); __x }) {
                // java hasInvalidatedWrap = true;
                hasInvalidatedWrap = true;
                // java drawing.setWrap(false);
                drawing.setWrap(false);
              }
              // java if (DrawingPositionTool.isWrapTableOrTextFrame(drawing.getPointer(), snapshot))
              if DrawingPositionTool::isWrapTableOrTextFrame(drawing.getPointer(), snapshot) {
                // java int line = LocateTool.quickFindRangeObj(layoutState.curPage.getPointer(), anchorCP, true, snapshot);
                let mut line: i32 = LocateTool::quickFindRangeObj(layoutState.curPage.getPointer(), anchorCP, true, snapshot);
                // java if (0 != line)
                if 0 != line {
                  // java TypoRectCoreImpl tmpRect = TypoRectCoreImpl.obtain();
                  let mut tmpRect: TypoRectCoreImpl = TypoRectCoreImpl::obtain();
                  // java TypoObjBase objBase = (TypoObjBase)snapshot.getTypoTools().obtain(line);
                  let mut objBase: TypoObjBase = (snapshot.getTypoTools().obtain(line)).downcast_ref::<TypoObjBase>().unwrap();
                  // java objBase.rectForPageCore(tmpRect);
                  objBase.rectForPageCore(tmpRect);
                  // java affectRect.top = Math.min(affectRect.top, tmpRect.top - 1);
                  affectRect.top = (affectRect.top).min(tmpRect.top - 1);
                  // java affectRect.bottom = Math.min(affectRect.bottom, tmpRect.bottom + 1);
                  affectRect.bottom = (affectRect.bottom).min(tmpRect.bottom + 1);
                  // java snapshot.getTypoTools().recycle((TypoMemoryObj)objBase);
                  snapshot.getTypoTools().recycle((objBase).downcast_ref::<TypoMemoryObj>().unwrap());
                  // java tmpRect.recycle();
                  tmpRect.recycle();
                }
              }
            }
            // java drawingLayers.removeDrawing(drawing.getPointer());
            drawingLayers.removeDrawing(drawing.getPointer());
            // java DrawingCacheTool.addToSortedCacheItems(floatsCache, pools, cleaner, doc, drawing.getPointer(), drawingLayers, snapshot);
            DrawingCacheTool::addToSortedCacheItems__klistdrawingcacheitem_pools_cachecleaner_document_int_typodrawinglayers_typosnapshot(floatsCache, pools, cleaner, doc, drawing.getPointer(), drawingLayers, snapshot);
          }
        }
        // java snapshot.getTypoTools().recycle((TypoMemoryObj)drawing);
        snapshot.getTypoTools().recycle((drawing).downcast_ref::<TypoMemoryObj>().unwrap());
        // java layoutRect.recycle();
        layoutRect.recycle();
        // java redundantDrawings.resetQuick();
        redundantDrawings.resetQuick();
        // java return hasInvalidatedWrap;
        return hasInvalidatedWrap;
      }
      */
    }

    // java private int getMaxWrapDrawingCP(WIntArrayList wrapDrawingCPs, int maxRedundantWrapDrawingCP)
    pub fn getMaxWrapDrawingCP(&self, wrapDrawingCPs: &WIntArrayList, maxRedundantWrapDrawingCP: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int maxWrapDrawingCP = Integer.MIN_VALUE;
      let mut maxWrapDrawingCP: i32 = i32::MIN;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = wrapDrawingCPs.size();
      // java for(int n = wrapDrawingCPs.size(); i < n; ++i)
      while i < n {
        // java int cp = wrapDrawingCPs.get(i);
        let mut cp: i32 = wrapDrawingCPs.get(i);
        // java if (cp != maxRedundantWrapDrawingCP)
        if cp != maxRedundantWrapDrawingCP {
          // java maxWrapDrawingCP = Math.max(maxWrapDrawingCP, cp);
          maxWrapDrawingCP = (maxWrapDrawingCP).max(cp);
        }
      }
      // java return maxWrapDrawingCP;
      return maxWrapDrawingCP;
      */
    }

    // java protected int getLastValidLineBeforeClearDrawings(LayoutState layoutState)
    pub fn getLastValidLineBeforeClearDrawings(&self, layoutState: &LayoutState) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getLastValidLineInPage(layoutState.curPage);
      return self.getLastValidLineInPage__typopage(&layoutState.curPage);
      */
    }

    // java private long findMaxRedundantWrapDrawingCP(int lastVisibleLine, WIntArrayList redundantDrawings, WIntArrayList wrapDrawingCPs, LayoutState layoutState)
    pub fn findMaxRedundantWrapDrawingCP(&self, lastVisibleLine: i32, redundantDrawings: &WIntArrayList, wrapDrawingCPs: &WIntArrayList, layoutState: &LayoutState) -> i64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int maxLimitCP = Integer.MIN_VALUE;
      let mut maxLimitCP: i32 = i32::MIN;
      // java int maxRedundantWrapDrawingCP = Integer.MIN_VALUE;
      let mut maxRedundantWrapDrawingCP: i32 = i32::MIN;
      // java TypoDrawingLayers drawingLayers = this.mMainState.getDrawingLayers(layoutState.document);
      let mut drawingLayers: TypoDrawingLayers = self.mMainState.getDrawingLayers__document(layoutState.document);
      // java if (drawingLayers != null)
      if true {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java WIntArrayList floatList = this.mMainState.snapshot.getTypoTools().obtainIntArrayList();
        let mut floatList: WIntArrayList = self.mMainState.snapshot.getTypoTools().obtainIntArrayList();
        // java if (drawingLayers.getFloats(floatList, layoutState.curPage.getPointer()) > 0)
        if drawingLayers.getFloats(floatList, layoutState.curPage.getPointer()) > 0 {
          // java int i = 0;
          let mut i: i32 = 0;
          let mut size: i32 = (floatList.len() as i32);
          // java for(int size = floatList.size(); i < size; ++i)
          while i < size {
            // java int iterDrawing = floatList.get(i);
            let mut iterDrawing: i32 = floatList.get(i);
            // java int anchorCP = TypoDrawing.getCP(iterDrawing, snapshot);
            let mut anchorCP: i32 = TypoDrawing::getCP__int_typosnapshot(iterDrawing, snapshot);
            // java boolean isWrap = TypoDrawing.isWrap(iterDrawing, snapshot);
            let mut isWrap: bool = TypoDrawing::isWrap__int_typosnapshot(iterDrawing, snapshot);
            // java if (isWrap)
            if isWrap {
              // java wrapDrawingCPs.add(anchorCP);
              wrapDrawingCPs.add__int(anchorCP);
            }
            // java int limitCP = this.getRedundantDrawingLimitCP(lastVisibleLine, iterDrawing, anchorCP, layoutState);
            let mut limitCP: i32 = self.getRedundantDrawingLimitCP(lastVisibleLine, iterDrawing, anchorCP, layoutState);
            // java if (limitCP >= 0)
            if limitCP >= 0 {
              // java redundantDrawings.add(iterDrawing);
              redundantDrawings.add__int(iterDrawing);
              // java if (isWrap)
              if isWrap {
                // java maxLimitCP = Math.max(maxLimitCP, limitCP);
                maxLimitCP = (maxLimitCP).max(limitCP);
                // java maxRedundantWrapDrawingCP = Math.max(maxRedundantWrapDrawingCP, anchorCP);
                maxRedundantWrapDrawingCP = (maxRedundantWrapDrawingCP).max(anchorCP);
              }
            }
          }
        }
        // java this.mMainState.snapshot.getTypoTools().recycle(floatList);
        self.mMainState.snapshot.getTypoTools().recycle(floatList);
      }
      // java return PrimitivePackTool.packIntAndInt(maxLimitCP, maxRedundantWrapDrawingCP);
      return PrimitivePackTool::packIntAndInt(maxLimitCP, maxRedundantWrapDrawingCP);
      */
    }

    // java protected long clearRedundantDrawings(LayoutState layoutState)
    pub fn clearRedundantDrawings(&mut self, layoutState: &mut LayoutState) -> i64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int lastVisibleLine = this.getLastValidLineBeforeClearDrawings(layoutState);
      let mut lastVisibleLine: i32 = self.getLastValidLineBeforeClearDrawings({ let __x: &LayoutState = todo!("no align strategy matched"); __x });
      // java TypoToolsManager typoTools = this.mMainState.snapshot.getTypoTools();
      let mut typoTools: TypoToolsManager = self.mMainState.snapshot.getTypoTools();
      // java WIntArrayList wrapDrawingCPs = typoTools.obtainIntArrayList();
      let mut wrapDrawingCPs: WIntArrayList = typoTools.obtainIntArrayList();
      // java WIntArrayList redundantDrawings = typoTools.obtainIntArrayList();
      let mut redundantDrawings: WIntArrayList = typoTools.obtainIntArrayList();
      // java long res = this.findMaxRedundantWrapDrawingCP(lastVisibleLine, redundantDrawings, wrapDrawingCPs, layoutState);
      let mut res: i64 = self.findMaxRedundantWrapDrawingCP(lastVisibleLine, &redundantDrawings, &wrapDrawingCPs, { let __x: &LayoutState = todo!("no align strategy matched"); __x });
      // java int maxLimitCP = PrimitivePackTool.unpackFirstInt(res);
      let mut maxLimitCP: i32 = PrimitivePackTool::unpackFirstInt(res);
      // java int maxRedundantWrapDrawingCP = PrimitivePackTool.unpackSecondInt(res);
      let mut maxRedundantWrapDrawingCP: i32 = PrimitivePackTool::unpackSecondInt(res);
      // java boolean hasInvalidatedWrap = this.doClearRedundantDrawings(lastVisibleLine, redundantDrawings, maxRedundantWrapDrawingCP, maxLimitCP, layoutState);
      let mut hasInvalidatedWrap: bool = self.doClearRedundantDrawings(lastVisibleLine, &redundantDrawings, maxRedundantWrapDrawingCP, maxLimitCP, layoutState);
      // java layoutState.maxWrapDrawingCP = this.getMaxWrapDrawingCP(wrapDrawingCPs, maxRedundantWrapDrawingCP);
      layoutState.maxWrapDrawingCP = self.getMaxWrapDrawingCP(&wrapDrawingCPs, maxRedundantWrapDrawingCP);
      // java typoTools.recycle(wrapDrawingCPs);
      typoTools.recycle(wrapDrawingCPs);
      // java typoTools.recycle(redundantDrawings);
      typoTools.recycle(redundantDrawings);
      // java boolean hasClearedOrInvalidated = maxRedundantWrapDrawingCP >= 0 || hasInvalidatedWrap;
      let mut hasClearedOrInvalidated: bool = maxRedundantWrapDrawingCP >= 0 || hasInvalidatedWrap;
      // java if (!hasClearedOrInvalidated && layoutState.atLeastOneLine && 0 == lastVisibleLine)
      if !hasClearedOrInvalidated && layoutState.atLeastOneLine && 0 == lastVisibleLine {
        // java DrawingCacheItem item = DrawingCacheTool.getFirstFlowedWrapDrawingCacheItem(this.mMainState.floatsCache, this.mMainState.snapshot);
        let mut item: DrawingCacheItem = DrawingCacheTool::getFirstFlowedWrapDrawingCacheItem(self.mMainState.floatsCache, self.mMainState.snapshot);
        // java if (null != item)
        if true {
          // java TypoDrawing.setWrap(false, item.drawing, this.mMainState.snapshot);
          TypoDrawing::setWrap__boolean_int_typosnapshot(false, item.drawing, self.mMainState.snapshot);
          // java return PrimitivePackTool.packBooleanAndInt(true, Integer.MAX_VALUE);
          return PrimitivePackTool::packBooleanAndInt(true, i32::MAX);
        }
      }
      // java return PrimitivePackTool.packBooleanAndInt(hasInvalidatedWrap, maxLimitCP);
      return PrimitivePackTool::packBooleanAndInt(hasInvalidatedWrap, maxLimitCP);
      */
    }

    // java protected int getRedundantDrawingLimitCP(int lastVisibleLine, int drawing, int anchorCP, LayoutState layoutState)
    pub fn getRedundantDrawingLimitCP(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32, layoutState: &LayoutState) -> i32 {
      // Java: the base TextLayouter uses page-end CP; page-view subclasses may
      // override this policy when their own layout surface is wired.
      let _ = (lastVisibleLine, drawing);
      if anchorCP >= layoutState.cp { anchorCP } else { i32::MIN }
    }

    // java protected void collectPageRange(LayoutState layoutState)
    pub fn collectPageRange(&mut self, layoutState: &mut LayoutState) {
      // java this.mNeedCollectRangeForLastRow = !layoutState.cellStateList.isEmpty();
      self.mNeedCollectRangeForLastRow = !layoutState.cellStateList.isEmpty();
      // java this.mPageRangesCollector.collectRanges(...): no break-cell row
      // takes the direct page.setRange(ref, start, end) branch.
      if !self.mNeedCollectRangeForFirstRow && !self.mNeedCollectRangeForLastRow {
        if let Some(page) = layoutState.curPage.as_mut() {
          // java page.removeExtendRanges();
          page.removeExtendRanges();
          // java page.setRange(ref, start, end);
          crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::setRange__int_int_int_int_typosnapshot(
            self.mRef, self.mStartCP, layoutState.cp, page.snapshot_pointer, &mut self.mMainState.snapshot,
          );
        }
      }
      /* DEFERRED_NON_FUNCTION
      // java if (this.mHasFlowTableHeader && RANGEFactory.getLength(this.mTableHeaderRange) > 0 && RANGEFactory.getStart(this.mTableHeaderRange) >= 0)
      if self.mHasFlowTableHeader && RANGEFactory::getLength(self.mTableHeaderRange) > 0 && RANGEFactory::getStart(self.mTableHeaderRange) >= 0 {
        // java layoutState.curPage.addExtendRange(layoutState.document, this.mTableHeaderRange);
        layoutState.curPage.addExtendRange(layoutState.document, self.mTableHeaderRange);
      }
      // java this.mNeedCollectRangeForLastRow = !layoutState.cellStateList.isEmpty();
      self.mNeedCollectRangeForLastRow = !layoutState.cellStateList.is_empty();
      // java int endCP = layoutState.cp;
      let mut endCP: i32 = layoutState.cp;
      // java this.mPageRangesCollector.collectRanges(layoutState.document, layoutState.curPage, this.mStartCP, endCP, this.mRef, this.mNeedCollectRangeForFirstRow, this.mNeedCollectRangeForLastRow, this.mMainState.snapshot, this.mMainState.drawingLayers);
      self.mPageRangesCollector.collectRanges__document_typopage_int_int_int_boolean_boolean_typosnapshot_typodrawinglayers(layoutState.document, layoutState.curPage, self.mStartCP, endCP, self.mRef, self.mNeedCollectRangeForFirstRow, self.mNeedCollectRangeForLastRow, self.mMainState.snapshot, self.mMainState.drawingLayers);
      */
    }

    // java protected void makeSnapshot(LayoutState layoutState)
    pub fn makeSnapshot(&mut self, layoutState: &mut LayoutState) {
      let _ = layoutState;
      /* DEFERRED_NON_FUNCTION
      // java this.mStateSnapshot.clear(this.mDocEnv.pools);
      self.mStateSnapshot.clear(self.mDocEnv.pools);
      // java layoutState.getEnvoy().saveToSnapShot(this.mStateSnapshot, this.mDocEnv);
      layoutState.getEnvoy().saveToSnapShot(self.mStateSnapshot, self.mDocEnv);
      */
    }

    // java protected void loadSnapshot(LayoutState layoutState)
    pub fn loadSnapshot(&mut self, layoutState: &mut LayoutState) {
      // java layoutState.getEnvoy().restoreFromSnapShot(this.mStateSnapshot, this.mDocEnv);
      let Some(snap) = self.mStateSnapshot.as_mut() else { return; };
      let doc = &mut self.mDocEnv as *mut _;
      layoutState.getEnvoy().restoreFromSnapShot(snap, unsafe { &mut *doc });
    }

    // java protected boolean flowFootEndNotes(LayoutState layoutState)
    pub fn flowFootEndNotes(&mut self, layoutState: &mut LayoutState) -> bool {
      // java 脚注布局（非 hello，GR-D3；Java 原文见 TextLayouter.java:1206-1216）
      let _ = layoutState;
      // java return false;
      false
    }

    // java private boolean needFinishPageForFootNotes(LayoutState layoutState)
    pub fn needFinishPageForFootNotes(&self, layoutState: &LayoutState) -> bool {
      // java 脚注页结束判定（非 hello，GR-D3）
      let _ = layoutState;
      // java return false;
      false
    }

    // java private boolean flowTextLineForCheckWidow(LayoutState layoutState)
    pub fn flowTextLineForCheckWidow(&mut self, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PageEnv pageEnv = layoutState.pageEnv;
      let mut pageEnv: PageEnv = layoutState.pageEnv;
      // java boolean oldAllowOverCoreHeight = pageEnv.allowOverCoreHeight;
      let mut oldAllowOverCoreHeight: bool = pageEnv.allowOverCoreHeight;
      // java pageEnv.allowOverCoreHeight = true;
      pageEnv.allowOverCoreHeight = true;
      // java SuperEnvoy envoy = layoutState.getEnvoy();
      let mut envoy: SuperEnvoy = layoutState.getEnvoy();
      // java this.mParaLayouter.start(envoy);
      self.mParaLayouter.start(envoy);
      // java while (true)
        // java this.prepareGraphs(layoutState);
        self.prepareGraphs(layoutState);
        // java if (this.hasGraphs(layoutState))
        if self.hasGraphs({ let __x: &LayoutState = todo!("no align strategy matched"); __x }) {
          // java TypoSnapshot snapshot = this.mMainState.snapshot;
          let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
          // java int oldLineSize = TypoList.size(layoutState.curPage.getLines(), snapshot);
          let mut oldLineSize: i32 = TypoList::size__int_typosnapshot(layoutState.curPage.getLines(), snapshot);
          // java do (layoutState.validLine && oldLineSize == TypoList.size(layoutState.curPage.getLines(), snapshot))
          // TODO block {
              // java this.mLineLayouter.flow(layoutState);
              self.mLineLayouter.flow(layoutState);
          }
          // java pageEnv.allowOverCoreHeight = oldAllowOverCoreHeight;
          pageEnv.allowOverCoreHeight = oldAllowOverCoreHeight;
          // java return true;
          return true;
        }
        // java this.onEmptyGraphSize(layoutState);
        self.onEmptyGraphSize(layoutState);
        // java if (layoutState.cp >= layoutState.getRangeEnd())
        if layoutState.cp >= layoutState.getRangeEnd() {
          // java return false;
          return false;
        }
        // java this.mParaLayouter.start(envoy);
        self.mParaLayouter.start(envoy);
      */
    }

    fn scale_share_embed(drawing: i32, core_width: i32, graphs: *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot, share_font: f32) {
      let font = if graphs.is_null() { 10.0 } else {
        unsafe { &*graphs }.charsRef.as_ref().and_then(|c| c.finalRunProp.as_ref()).map(|p| p.getFloat__int_float(10, 10.0)).unwrap_or(10.0)
      };
      let mut zoom = if font == 0.0 { 1.0 } else { share_font / font };
      let width = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::width__int(drawing, snapshot);
      if width > 0 && zoom * width as f32 > core_width as f32 {
        zoom = core_width as f32 / width as f32;
      }
      crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
    }

    fn scale_math_embed(drawing: i32, force_share: bool, phone: bool, core_width: i32, graphs: *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot, share_font: f32) {
      let zoom_now = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getZoom__int_typosnapshot(drawing, snapshot);
      if force_share && zoom_now == 1.0 {
        Self::scale_share_embed(drawing, core_width, graphs, snapshot, share_font);
      } else if phone {
        let width = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::width__int(drawing, snapshot);
        if width > core_width && width > 0 {
          let zoom = core_width as f32 / width as f32;
          crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
        }
      }
    }

    fn mark_revision_del_embed(graphs: *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs, drawing: i32, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) {
      if graphs.is_null() { return; }
      let chars = match unsafe { &*graphs }.charsRef.as_ref() { Some(c) => c, None => return };
      let revision_final = chars.srcRef.map(|src| unsafe { (*src).revisionFinal }).unwrap_or(false);
      let has_del = chars.finalRunProp.as_ref().map(|p| p.contains(45)).unwrap_or(false);
      if revision_final && has_del {
        crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::setIsRevisionDelEmbed__boolean_int_typosnapshot(true, drawing, snapshot);
        crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::setRevisionDelId(chars.revisionDelAuthor, drawing, snapshot);
      }
    }

    // java protected void prepareEmbeds(int wordAlingFont, LayoutState layoutState)
    pub fn prepareEmbeds(&mut self, wordAlingFont: i32, layoutState: &mut LayoutState) {
      // java boolean useWord97LineBreakRules = this.mDocEnv.useWord97LineBreakRules;
      let use_word97 = self.mDocEnv.useWord97LineBreakRules;
      // java int viewMode = this.mViewEnv.getViewMode();
      let view_mode = self.mViewEnv.getViewMode();
      // java boolean forceUseShareViewRunProperty = this.mViewEnv.getForceUseShareViewRunProperty();
      let force_share = self.mViewEnv.getForceUseShareViewRunProperty();
      // java int curEmbedIndex = layoutState.getEmbedSize();
      let mut cur = layoutState.getEmbedSize();
      loop {
        // java int argsSize = mGraphBuff.getArgsForEmbedSize();
        let args_size = self.mGraphBuff.as_ref().map(|g| g.super_.super_.getArgsForEmbedSize()).unwrap_or(0);
        if cur >= args_size {
          break;
        }
        let arg = self.mGraphBuff.as_ref().and_then(|g| g.super_.super_.getArgsForEmbed(cur).cloned());
        let graphs = self.mGraphBuff.as_ref().and_then(|g| g.super_.super_.getGraphsForEmbed(cur)).unwrap_or(std::ptr::null_mut());
        let Some(arg) = arg else { break; };
        // java extendWidth from card margins when showing a UI card.
        let extend_width = if self.mViewEnv.isShowCardInfo() && self.mMainState.isInUiCard {
          self.mViewEnv.getCardMarginLeft() + self.mViewEnv.getCardMarginRight()
        } else {
          0
        };
        let core_width = layoutState.pageEnv.getCoreWidth() - extend_width;
        let page_type = layoutState.curPage.as_ref().map(|page| page.getType()).unwrap_or(2);
        let drawing = match &arg {
          EmbedArg::Shape(shape_id) => {
            // java if (null == this.mDrawingLayouter) this.initDrawingLayouter();
            if self.mDrawingLayouter.is_none() {
              self.initDrawingLayouter();
            }
            // java drawing = this.mDrawingLayouter.getEmbed((Shape)args, layoutState.getEnvoy());
            let id = *shape_id;
            self.mGraphBuff.as_mut().map(|g| g.super_.super_.shape_embed_pointer(id, layoutState)).unwrap_or(0)
          }
          EmbedArg::Math(node) => {
            crate::cn::wps::moffice::writer::layout::base::math::MathLayouter::MathLayouter::getMath(
              &mut self.mMainState, &mut self.mDocEnv, &mut self.mViewEnv, layoutState, node,
            )
          }
          EmbedArg::Eq(eq) => {
            match crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::EquationLayouter::getEquation__mainstate_documentenv_viewenv_document_int_int_boolean_boolean(
              &mut self.mMainState, &mut self.mDocEnv, &mut self.mViewEnv, layoutState.document, eq.start, eq.end, eq.enableRevision, eq.revisionFinal,
            ) {
              Ok(pointer) => pointer,
              Err(_) => {
                let prop = if graphs.is_null() { None } else { unsafe { &*graphs }.charsRef.as_ref().and_then(|c| c.finalRunProp.as_ref()) };
                crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::EquationLayouter::getExceptionEquation(
                  &mut self.mMainState, &self.mDocEnv, &self.mViewEnv, layoutState.document, prop,
                )
              }
            }
          }
        };
        if drawing != 0 {
          let phone = crate::cn::wps::moffice::writer::base::ViewMode::ViewMode::isPhoneReadMode(view_mode);
          {
          let snapshot = &mut self.mMainState.snapshot;
          match &arg {
            EmbedArg::Shape(_) if phone => {
              // java if (zoom * drawingWidth > coreWidth) scale; zoom starts at 1.
              let width = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::width__int(drawing, snapshot);
              if width > 0 && width > core_width {
                let zoom = core_width as f32 / width as f32;
                crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
              }
            }
            EmbedArg::Math(_) if page_type == 2 && !graphs.is_null() => {
              Self::scale_math_embed(drawing, force_share, phone, core_width, graphs, snapshot, self.mViewEnv.getShareViewFontsize());
            }
            EmbedArg::Eq(_) if page_type == 2 && force_share && !graphs.is_null() => {
              Self::scale_share_embed(drawing, core_width, graphs, snapshot, self.mViewEnv.getShareViewFontsize());
            }
            _ => {}
          }
          }
          if !graphs.is_null() {
            layoutState.setDrawing(unsafe { &mut *graphs }, drawing, wordAlingFont, use_word97);
            Self::mark_revision_del_embed(graphs, drawing, &mut self.mMainState.snapshot);
          }
          cur += 1;
        } else {
          if let Some(buff) = self.mGraphBuff.as_mut() {
            buff.super_.super_.removeEmbedArgsAt(cur);
          }
          if !graphs.is_null() {
            let next = layoutState.getGraphList().getNext(graphs);
            if layoutState.getGraphList().firstElement() == graphs {
              layoutState.push_ptr(next);
            }
            layoutState.getGraphList().remove(graphs, graphs);
          }
        }
      }
      /* DEFERRED_NON_FUNCTION
      // java boolean useWord97LineBreakRules = this.mDocEnv.useWord97LineBreakRules;
      let mut useWord97LineBreakRules: bool = self.mDocEnv.useWord97LineBreakRules;
      // java GraphBuff mGraphBuff = this.mGraphBuff;
      let mut mGraphBuff: GraphBuff = self.mGraphBuff.unwrap();
      // java int curEmbedIndex = layoutState.getEmbedSize();
      let mut curEmbedIndex: i32 = layoutState.getEmbedSize();
      // java int argsSize = mGraphBuff.getArgsForEmbedSize();
      let mut argsSize: i32 = mGraphBuff.getArgsForEmbedSize();
      // java int viewMode = this.mViewEnv.getViewMode();
      let mut viewMode: i32 = self.mViewEnv.getViewMode();
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java boolean forceUseShareViewRunProperty = this.mViewEnv.getForceUseShareViewRunProperty();
      let mut forceUseShareViewRunProperty: bool = self.mViewEnv.getForceUseShareViewRunProperty();
      // java while (curEmbedIndex < argsSize)
      while curEmbedIndex < argsSize {
        // java Graphs curGraphs = mGraphBuff.getGraphsForEmbed(curEmbedIndex);
        let mut curGraphs: Graphs = mGraphBuff.getGraphsForEmbed(curEmbedIndex);
        // java Object args = mGraphBuff.getArgsForEmbed(curEmbedIndex);
        let mut args: Box<dyn Object> = mGraphBuff.getArgsForEmbed(curEmbedIndex);
        // java int extendWidth = 0;
        let mut extendWidth: i32 = 0;
        // java if (this.mViewEnv.isShowCardInfo() && this.mMainState.isInUiCard)
        if self.mViewEnv.isShowCardInfo() && self.mMainState.isInUiCard {
          // java extendWidth = this.mViewEnv.getCardMarginLeft() + this.mViewEnv.getCardMarginRight();
          extendWidth = self.mViewEnv.getCardMarginLeft() + self.mViewEnv.getCardMarginRight();
        }
        // java int coreWidth = layoutState.pageEnv.getCoreWidth() - extendWidth;
        let mut coreWidth: i32 = layoutState.pageEnv.getCoreWidth() - extendWidth;
        // java int drawing;
        let mut drawing: i32;
        // java if (args instanceof Shape)
        if args.downcast_ref::<Shape>().is_some() {
          // java if (null == this.mDrawingLayouter)
          if self.mDrawingLayouter.is_none() {
            // java this.initDrawingLayouter();
            self.initDrawingLayouter();
          }
          // java drawing = this.mDrawingLayouter.getEmbed((Shape)args, layoutState.getEnvoy());
          drawing = self.mDrawingLayouter.getEmbed((args).downcast_ref::<Shape>().unwrap(), layoutState.getEnvoy());
          // java if (0 != drawing && ViewMode.isPhoneReadMode(viewMode))
          if 0 != drawing && ViewMode::isPhoneReadMode(viewMode) {
            // java float zoom = 1.0F;
            let mut zoom: f32 = 1;
            // java int drawingWidth = TypoDrawing.width(drawing, snapshot);
            let mut drawingWidth: i32 = TypoDrawing::width(drawing, snapshot);
            // java if (zoom * (float)drawingWidth > (float)coreWidth)
            if zoom * ((drawingWidth) as f32) > ((coreWidth) as f32) {
              // java zoom = (float)coreWidth / (float)drawingWidth;
              zoom = ((coreWidth) as f32) / ((drawingWidth) as f32);
              // java DrawingResizeTool.scaleDrawing(drawing, zoom, true, true, snapshot);
              DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
            }
            // java else if (this.mViewEnv.getPhoneViewParam().isKeepDrawingNormalWhenShrink() && layoutState.isIsolatedEmbed(curGraphs) && drawingWidth < coreWidth)
            else if self.mViewEnv.getPhoneViewParam().isKeepDrawingNormalWhenShrink() && layoutState.isIsolatedEmbed(curGraphs) && drawingWidth < coreWidth {
              // java float curZoom = this.mViewEnv.getViewSettings().getZoom();
              let mut curZoom: f32 = self.mViewEnv.getViewSettings().getZoom();
              // java float normalZoom = this.mViewEnv.getViewSettings().getWebModeManager().getNormalZoom();
              let mut normalZoom: f32 = self.mViewEnv.getViewSettings().getWebModeManager().getNormalZoom();
              // java if (normalZoom > curZoom)
              if normalZoom > curZoom {
                // java float adjustZoom = Math.min(normalZoom / curZoom, (float)coreWidth * 1.0F / (float)drawingWidth);
                let mut adjustZoom: f32 = (normalZoom / curZoom).min(((coreWidth) as f32) * 1 / ((drawingWidth) as f32));
                // java DrawingResizeTool.scaleDrawing(drawing, adjustZoom, true, false, snapshot);
                DrawingResizeTool::scaleDrawing(drawing, adjustZoom, true, false, snapshot);
              }
            }
          }
        }
        // java else if (args instanceof PLC.Node)
        else if args.downcast_ref::<Box<dyn Node>>().is_some() {
          // java drawing = MathLayouter.getMath(this.mMainState, this.mDocEnv, this.mViewEnv, layoutState, (PLC.Node)args);
          drawing = MathLayouter::getMath(&mut self.mMainState, &mut self.mDocEnv, &mut self.mViewEnv, layoutState, todo!("cast → Box<dyn Node>"));
          // java if (0 != drawing && layoutState.curPage.getType() == 2)
          if 0 != drawing && layoutState.curPage.getType() == 2 {
            // java if (forceUseShareViewRunProperty && TypoDrawing.getZoom(drawing, snapshot) == 1.0F)
            if forceUseShareViewRunProperty && TypoDrawing::getZoom__int_typosnapshot(drawing, snapshot) == 1 {
              // java float zoom = this.mViewEnv.getShareViewFontsize() / curGraphs.charsRef.finalRunProp.getFloat(10, 10.0F);
              let mut zoom: f32 = self.mViewEnv.getShareViewFontsize() / curGraphs.charsRef.finalRunProp.getFloat(10, 10);
              // java int drawingWidth = TypoDrawing.width(drawing, snapshot);
              let mut drawingWidth: i32 = TypoDrawing::width(drawing, snapshot);
              // java if (zoom * (float)drawingWidth > (float)coreWidth)
              if zoom * ((drawingWidth) as f32) > ((coreWidth) as f32) {
                // java zoom = (float)coreWidth / (float)drawingWidth;
                zoom = ((coreWidth) as f32) / ((drawingWidth) as f32);
              }
              // java DrawingResizeTool.scaleDrawing(drawing, zoom, true, true, snapshot);
              DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
            }
            // java else if (ViewMode.isPhoneReadMode(viewMode))
            else if ViewMode::isPhoneReadMode(viewMode) {
              // java int drawingWidth = TypoDrawing.width(drawing, snapshot);
              let mut drawingWidth: i32 = TypoDrawing::width(drawing, snapshot);
              // java if (drawingWidth > coreWidth)
              if drawingWidth > coreWidth {
                // java float zoom = (float)coreWidth / (float)drawingWidth;
                let mut zoom: f32 = ((coreWidth) as f32) / ((drawingWidth) as f32);
                // java DrawingResizeTool.scaleDrawing(drawing, zoom, true, true, snapshot);
                DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
              }
            }
          }
        }
        // java else if (args instanceof GraphBuff.EqArg)
        else if args.downcast_ref::<GraphBuff::EqArg>().is_some() {
          // java GraphBuff.EqArg eqArg = (GraphBuff.EqArg)args;
          let mut eqArg: GraphBuff::EqArg = { let __x: GraphBuff::EqArg = todo!("no align strategy matched"); __x };
              // java drawing = EquationLayouter.getEquation(this.mMainState, this.mDocEnv, this.mViewEnv, layoutState.document, eqArg.start, eqArg.end, eqArg.enableRevision, eqArg.revisionFinal);
              drawing = EquationLayouter::getEquation__mainstate_documentenv_viewenv_document_int_int_boolean_boolean(self.mMainState, self.mDocEnv, self.mViewEnv, layoutState.document, eqArg.start, eqArg.end, eqArg.enableRevision, eqArg.revisionFinal);
              // java EquationException var20
              todo!();
                // java drawing = EquationLayouter.getExceptionEquation(this.mMainState, this.mDocEnv, this.mViewEnv, layoutState.document, this.mViewEnv.getEquationExceptionProperty());
                drawing = EquationLayouter::getExceptionEquation(self.mMainState, self.mDocEnv, self.mViewEnv, layoutState.document, self.mViewEnv.getEquationExceptionProperty());
          // java if (layoutState.curPage.getType() == 2 && forceUseShareViewRunProperty)
          if layoutState.curPage.getType() == 2 && forceUseShareViewRunProperty {
            // java float zoom = this.mViewEnv.getShareViewFontsize() / curGraphs.charsRef.finalRunProp.getFloat(10, 10.0F);
            let mut zoom: f32 = self.mViewEnv.getShareViewFontsize() / curGraphs.charsRef.finalRunProp.getFloat(10, 10);
            // java int drawingWidth = TypoDrawing.width(drawing, snapshot);
            let mut drawingWidth: i32 = TypoDrawing::width(drawing, snapshot);
            // java if (zoom * (float)drawingWidth > (float)coreWidth)
            if zoom * ((drawingWidth) as f32) > ((coreWidth) as f32) {
              // java zoom = (float)coreWidth / (float)drawingWidth;
              zoom = ((coreWidth) as f32) / ((drawingWidth) as f32);
            }
            // java DrawingResizeTool.scaleDrawing(drawing, zoom, true, true, snapshot);
            DrawingResizeTool::scaleDrawing(drawing, zoom, true, true, snapshot);
          }
        }
        // java else
        else {
          // java Assert.fail();
          Assert::fail();
          // java drawing = 0;
          drawing = 0;
        }
        // java if (0 != drawing)
        if 0 != drawing {
          // java layoutState.setDrawing(curGraphs, drawing, wordAlingFont, useWord97LineBreakRules);
          layoutState.setDrawing(curGraphs, drawing, wordAlingFont, useWord97LineBreakRules);
          // java ++curEmbedIndex;
          { let _tmp = curEmbedIndex; curEmbedIndex += 1; _tmp };
          // java CharsNode charNode = curGraphs.charsRef;
          let mut charNode: CharsNode = curGraphs.charsRef;
          // java if (null != charNode.srcRef && charNode.srcRef.revisionFinal && charNode.finalRunProp.contains(45))
          if charNode.srcRef.is_some() && charNode.srcRef.revisionFinal && charNode.finalRunProp.contains(45) {
            // java TypoDrawing.setIsRevisionDelEmbed(true, drawing, snapshot);
            TypoDrawing::setIsRevisionDelEmbed__boolean_int_typosnapshot(true, drawing, snapshot);
            // java TypoDrawing.setRevisionDelId(charNode.revisionDelAuthor, drawing, snapshot);
            TypoDrawing::setRevisionDelId(charNode.revisionDelAuthor, drawing, snapshot);
          }
        }
        // java else
        else {
          // java mGraphBuff.removeEmbedArgsAt(curEmbedIndex);
          mGraphBuff.removeEmbedArgsAt(curEmbedIndex);
          // java KPList<Graphs> graphs = layoutState.getGraphList();
          let mut graphs: KPList = layoutState.getGraphList();
          // java if (graphs.firstElement() == curGraphs)
          if graphs.firstElement() == curGraphs {
            // java layoutState.push((Graphs)graphs.getNext(curGraphs));
            layoutState.push__graphs((graphs.getNext(curGraphs)).downcast_ref::<Graphs>().unwrap());
          }
          // java graphs.remove(curGraphs);
          graphs.remove(curGraphs);
          // java --argsSize;
          { let _tmp = argsSize; argsSize -= 1; _tmp };
        }
      }
      */
    }

    // java public void endParaLayouter(TableLayouter.IStateEnvoy stateEnvoy)
    pub fn endParaLayouter(&mut self, envoy: &mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy) {
      // java if (stateEnvoy instanceof ParaLayouter.IStateEnvoy) this.mParaLayouter.end(...)
      if let Some(para) = self.mParaLayouter.as_mut() {
        para.end(envoy);
      }
    }

    // java public void adjustWrapTable(TypoDrawing drawing, TableInfo tableInfo, int paraTop, boolean needCutTable, TableLayouter.IStateEnvoy stateEnvoy)
    pub fn adjustWrapTable(&mut self, drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing, tableInfo: &crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::TableInfo, paraTop: i32, needCutTable: bool, stateEnvoy: &mut LayoutState) {
      // Java PageViewLayoutState implements DrawingLayouter.IStateEnvoy.
      self.initDrawingLayouter();
      let main = &mut self.mMainState as *mut MainState;
      let doc = &mut self.mDocEnv as *mut DocumentEnv;
      if let Some(layouter) = self.mDrawingLayouter.as_mut() {
        layouter.bind_live_env(main, doc);
        layouter.adjustWrapTable(drawing, tableInfo, paraTop, needCutTable, stateEnvoy);
      }
    }

    // java public boolean postFlowTableHeader(int row, TableRowLayouter.IStateEnvoy stateEnvoy)
    pub fn postFlowTableHeader(&mut self, row: i32, stateEnvoy: &mut dyn IStateEnvoy) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return stateEnvoy instanceof DrawingLayouter.IStateEnvoy ? this.getDrawingLayouter().flowForTableRow(row, (DrawingLayouter.IStateEnvoy)stateEnvoy) : false;
      return (if stateEnvoy.downcast_ref::<DrawingLayouter::IStateEnvoy>().is_some() { self.getDrawingLayouter().flowForTableRow(row, (stateEnvoy).downcast_ref::<DrawingLayouter::IStateEnvoy>().unwrap()) } else { false });
      */
    }

    // java private static int resolveTraceTableIndex(LayoutState layoutState)
    pub fn resolveTraceTableIndex(layoutState: &LayoutState) -> i32 {
      // java TableInfos tins = layoutState.document.getTableInfos();
      let Some(doc) = layoutState.document else { return 0 };
      let Some(tins) = (unsafe { (*doc).getTableInfos() }) else { return 0 };
      let tins = unsafe { &*tins };
      // java TableInfo ti = tins.getTableInfoByLevel(layoutState.cp, layoutState.tablePeriLayer);
      let Some(ti) = tins.getTableInfoByLevel(layoutState.cp, layoutState.tablePeriLayer) else {
        // java if (ti == null) return 0;
        return 0;
      };
      // java int n = tins.size();
      let n = tins.size();
      let mut i = 0;
      // java for (int i = 0; i < n; ++i)
      while i < n {
        // java if (tins.getTableByIndex(i) == ti) return i;
        // TableInfoNode is cloned out of the cache, so identity is start CP + level.
        if let Some(other) = tins.getTableByIndex(i) {
          if other.getStartCP() == ti.getStartCP() && other.getLevel() == ti.getLevel() {
            return i;
          }
        }
        i += 1;
      }
      // java return 0;
      0
    }

}
