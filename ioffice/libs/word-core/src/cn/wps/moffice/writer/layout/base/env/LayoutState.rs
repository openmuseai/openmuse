// Java: import cn.wps.collections.KList
use util::collections::KList::{KList, RecycableNode, RecycableNodeAlloc, KListNode};
// Java: import cn.wps.collections.KPList
// CARGO_OFF_SURFACE use util::collections::KPList::KPList;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
// CARGO_OFF_SURFACE use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.drawing.Shape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet as RunPropertySet;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoRect
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRect::TypoRect;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.cache.TypoRectMemoryImpl
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IChpxTable
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCMathStart::PLCMathStart;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.layout.base.bidi.BidiTool
// CARGO_OFF_SURFACE // Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingResizeTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool;
// Java: import cn.wps.moffice.writer.layout.base.graph.CharsNode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.DrawingNode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::DrawingNode::DrawingNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphArrayDeque
use crate::cn::wps::moffice::writer::layout::base::graph::GraphArrayDeque::GraphArrayDeque;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.graph.OrgWidthCache
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::OrgWidthCache::OrgWidthCache;
// Java: import cn.wps.moffice.writer.layout.base.graph.RevisionNode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::RevisionNode::RevisionNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.SrcNode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.PredictSpacesInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::charSpace::PredictSpacesInfo::PredictSpacesInfo;
// Java: import cn.wps.moffice.writer.layout.base.line.docfield.DocFieldCollector
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::docfield::DocFieldCollector::DocFieldCollector;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.TableCellLayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.SignableRange
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::SignableRange::SignableRange;
// Java: import cn.wps.moffice.writer.layout.base.env.SuperEnvoy
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import org.apache.poi.poifs.filesystem.Entry
// CARGO_OFF_SURFACE use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;
// Java: import cn.wps.moffice.writer.layout.base.env.PhoneViewParam
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PhoneViewParam::PhoneViewParam;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;


// CARGO_OFF_SURFACE_TYPES
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// rust: ★wr-29 恢复 —— 真实类型面
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode;
use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
use crate::cn::wps::moffice::writer::layout::base::graph::DrawingNode::DrawingNode;
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::StyleResult;
use crate::cn::wps::moffice::writer::layout::base::graph::OrgWidthCache::OrgWidthCache;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
#[derive(Default)]
pub struct TableCellLayoutState;
#[derive(Default, Clone)]
pub struct OMathStartNode { pub start: i32, pub end: i32 }
#[derive(Default)]
pub struct RevisionNode {
    // java public int type;
    pub r#type: i32,
    // java public int start;
    pub start: i32,
    // java public int authorId;
    pub authorId: i32,
}
#[derive(Default)]
pub struct DocFieldCollector;

impl DocFieldCollector {
    // java public void collectDocFieldInGraph(Graphs graphs, boolean isRTL)
    pub fn collectDocFieldInGraph(&self, _graphs: &crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs, _isRTL: bool) {}

    // java public void dispose()
    pub fn dispose(&self) {}
}
#[derive(Default)]
pub struct PredictSpacesInfo;
#[derive(Default)]
pub struct SignableRange;
#[derive(Default)]
pub struct PLCMathStart;
#[derive(Default)]
pub struct Shape;
#[derive(Default)]
pub struct PhoneViewParam;
#[derive(Default)]
pub struct PropertySet;

// rust: GR-D1 (wr-29) 真实节点存储 —— Vec<Box<T>> 池；节点地址稳定（Box 不变更）
pub struct KPList<T: Default = ()> {
    pub nodes: Vec<Box<T>>,
    _t: core::marker::PhantomData<T>,
}
impl<T: Default> Default for KPList<T> {
    fn default() -> Self { Self { nodes: Vec::new(), _t: core::marker::PhantomData } }
}
impl<T: Default> KPList<T> {
    pub fn new(_a: ()) -> Self { Self::default() }
    pub fn isEmpty(&self) -> bool { self.nodes.is_empty() }
    pub fn is_empty(&self) -> bool { self.isEmpty() }
    pub fn clean(&mut self) { self.nodes.clear(); }
    pub fn size(&self) -> i32 { self.nodes.len() as i32 }
    pub fn get_at(&mut self, idx: usize) -> *mut T {
        if idx < self.nodes.len() { &mut *self.nodes[idx] as *mut T } else { std::ptr::null_mut() }
    }
    pub fn firstElement(&mut self) -> *mut T {
        if self.nodes.is_empty() { std::ptr::null_mut() } else { &mut *self.nodes[0] as *mut T }
    }
    pub fn lastElement(&mut self) -> *mut T {
        let n = self.nodes.len();
        if n == 0 { std::ptr::null_mut() } else { &mut *self.nodes[n - 1] as *mut T }
    }
    pub fn getNext(&mut self, node: *mut T) -> *mut T {
        let idx = self.nodes.iter().position(|n| &**n as *const T == node as *const T);
        match idx { Some(i) if i + 1 < self.nodes.len() => &mut *self.nodes[i+1] as *mut T, _ => std::ptr::null_mut() }
    }
    pub fn getPrev(&mut self, node: *mut T) -> *mut T {
        let idx = self.nodes.iter().position(|n| &**n as *const T == node as *const T);
        match idx { Some(i) if i >= 1 => &mut *self.nodes[i-1] as *mut T, _ => std::ptr::null_mut() }
    }
    // java public void remove(Node first, Node last) —— 含 first..last 闭区间（Java 链摘除）
    pub fn remove(&mut self, first: *mut T, last: *mut T) {
        if first.is_null() {
            return;
        }
        let i0 = self.nodes.iter().position(|n| &**n as *const T == first as *const T);
        let i1 = if last.is_null() {
            i0
        } else {
            self.nodes.iter().position(|n| &**n as *const T == last as *const T)
        };
        if let (Some(a), Some(b)) = (i0, i1) {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let _ = self.nodes.drain(lo..=hi);
        }
    }
    pub fn obtainAndAddLast(&mut self) -> *mut T {
        self.nodes.push(Box::new(T::default()));
        let n = self.nodes.len();
        &mut *self.nodes[n - 1] as *mut T
    }
    pub fn obtainAndAddFirst(&mut self) -> *mut T {
        self.nodes.insert(0, Box::new(T::default()));
        &mut *self.nodes[0] as *mut T
    }
    // java public T obtainAndInsertAfter(T node) —— 在 node 之后插入新池节点（Java KPList）
    pub fn obtainAndInsertAfter(&mut self, node: *mut T) -> *mut T {
        let idx = self
            .nodes
            .iter()
            .position(|n| &**n as *const T == node as *const T);
        match idx {
            Some(i) => {
                self.nodes.insert(i + 1, Box::new(T::default()));
                &mut *self.nodes[i + 1] as *mut T
            }
            None => self.obtainAndAddLast(),
        }
    }
}

// rust: ★wr-29 —— 真实游标（Java KPList.Iterator 对等；Vec 池索引游标）
#[derive(Default)]
pub struct GraphsIt {
    list: *mut KPList<Graphs>,
    idx: usize,
    end: bool,
}
impl GraphsIt {
    pub fn attach(&mut self, list: *mut KPList<Graphs>) { self.list = list; self.idx = 0; self.end = false; }
    pub fn seekBegin(&mut self) { self.idx = 0; self.end = false; }
    pub fn seekEnd(&mut self) { self.end = true; if !self.list.is_null() { self.idx = unsafe { (*self.list).nodes.len() }; } }
    pub fn seek(&mut self, node: *mut Graphs) {
        if self.list.is_null() || node.is_null() { return; }
        let nodes = unsafe { &(*self.list).nodes };
        if let Some(i) = nodes.iter().position(|n| &**n as *const Graphs == node as *const Graphs) {
            // java seek 停在该节点上；随后 Iterator.next 返回该节点
            self.idx = i; self.end = false;
        } else { self.end = true; }
    }
    pub fn valid(&self) -> bool { !self.end && !self.list.is_null() && self.idx < unsafe { (*self.list).nodes.len() } }
    pub fn value(&self) -> *mut Graphs {
        if !self.valid() {
            std::ptr::null_mut()
        } else {
            unsafe { &mut *(&mut (*self.list).nodes)[self.idx] as *mut Graphs }
        }
    }
    pub fn next(&mut self) -> *mut Graphs {
        // java Graphs graph = (Graphs)this.mGraphsIt.next(); 先取值再前进
        let v = self.value();
        if !v.is_null() {
            self.idx += 1;
            if self.list.is_null() || self.idx >= unsafe { (*self.list).nodes.len() } {
                self.end = true;
            }
        }
        v
    }
    pub fn prev(&mut self) -> *mut Graphs {
        if self.idx > 0 { self.idx -= 1; }
        self.value()
    }
    pub fn advance(&mut self) -> Option<Graphs> {
        let v = self.value();
        if v.is_null() { return None; }
        let g = Clone::clone(unsafe { &*v });
        self.idx += 1;
        Some(g)
    }
}

use crate::cn::wps::moffice::writer::layout::base::bidi::BidiTool::BidiTool;

/// Java `KList<TableCellLayoutState>`: insertion-order cells owned by the layout state.
#[derive(Default)]
pub struct CellStateList {
    cells: Vec<Box<crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState>>,
}

impl CellStateList {
    // java public boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.isEmpty()
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    // java public void clear()
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    // java KList drained into Pools.recycleTableCellLayoutStates
    pub fn drain(&mut self) -> Vec<crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState> {
        self.cells.drain(..).map(|cell| *cell).collect()
    }

    // java public void add(TableCellLayoutState cell)
    pub fn add(&mut self, cell: crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState) {
        self.cells.push(Box::new(cell));
    }

    pub fn ptr_at(&mut self, index: usize) -> *mut crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState {
        self.cells.get_mut(index).map(|cell| cell.as_mut() as *mut _).unwrap_or(std::ptr::null_mut())
    }
}

// java public class LayoutState extends KList.RecycableNode
pub struct LayoutState {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java public final PageEnv pageEnv = new PageEnv();
    pub(crate) pageEnv: PageEnv,
    // java public final ParaEnv paraEnv = new ParaEnv();
    pub paraEnv: ParaEnv,
    // java public int tablePeriLayer = 1;
    pub tablePeriLayer: i32,
    // java public int textFrameLayer = 0;
    pub textFrameLayer: i32,
    // java public boolean atLeastOneLine = true;
    pub(crate) atLeastOneLine: bool,
    // java public Document document;
    pub document: Option<*mut dyn Document>,
    // java private long range;
    range: i64,
    // java public final KList<TableCellLayoutState> cellStateList = new KList();
    pub cellStateList: CellStateList,
    // java public TypoPage curPage;
    pub(crate) curPage: Option<TypoPage>,
    // java public int linePos;
    pub linePos: i32,
    // java public int curLayoutHeight;
    pub(crate) curLayoutHeight: i32,
    // java public TypoRectCoreImpl affectRect = TypoRectCoreImpl.obtain();
    pub affectRect: TypoRectCoreImpl,
    // java private boolean finishPage;
    pub finishPage: bool,
    // java public boolean validLine;
    pub(crate) validLine: bool,
    // java public boolean hasFlowTableHeader;
    pub(crate) hasFlowTableHeader: bool,
    // java public boolean needCalcSectionBreakWidth;
    pub(crate) needCalcSectionBreakWidth: bool,
    // java public boolean isShowRevisionInsOrDelInBalloons;
    pub(crate) isShowRevisionInsOrDelInBalloons: bool,
    // java public int cp = 0;
    pub cp: i32,
    // java public long tableHeaderRange = 0L;
    pub(crate) tableHeaderRange: i64,
    // java public int maxWrapDrawingCP = Integer.MIN_VALUE;
    pub maxWrapDrawingCP: i32,
    // java public int pageEndLimitCP = Integer.MAX_VALUE;
    pub pageEndLimitCP: i32,
    // java public final TypoPara typoPara = TypoPara.newInstance();
    pub typoPara: TypoPara,
    // java public boolean isRevision;
    pub(crate) isRevision: bool,
    // java public final KPList<SrcNode> srcList = new KPList(new SrcNode.Alloc());
    pub srcList: KPList<SrcNode>,
    // java public final KPList<CharsNode> charsList = new KPList(new CharsNode.Alloc());
    pub charsList: KPList<CharsNode>,
    // java public KPList<DrawingNode> drawingList;
    pub(crate) drawingList: Option<KPList<DrawingNode>>,
    // java public final KPList<Graphs> graphList = new KPList(new Graphs.Alloc());
    pub graphList: KPList<Graphs>,
    // java public final FontStyleFactory.StyleResult styleResult = new FontStyleFactory.StyleResult();
    pub(crate) styleResult: StyleResult,
    // java KPList.Iterator<Graphs> mGraphsIt;
    pub(crate) mGraphsIt: GraphsIt,
    // java public int mFirstGraphIndex;
    pub(crate) mFirstGraphIndex: i32,
    // java public boolean hasBidiGraph;
    pub(crate) hasBidiGraph: bool,
    // java public boolean hasCoreCharsContent;
    pub(crate) hasCoreCharsContent: bool,
    // java public boolean hasReadEnd;
    pub hasReadEnd: bool,
    // java public boolean hasMaths;
    pub(crate) hasMaths: bool,
    // java public boolean hasEmbeds;
    pub(crate) hasEmbeds: bool,
    // java public boolean hasEquation;
    pub(crate) hasEquation: bool,
    // java public boolean paraIsOverflow;
    pub(crate) paraIsOverflow: bool,
    // java public boolean isInMathPara;
    pub(crate) isInMathPara: bool,
    // java public boolean isInEmbedBalloonMode;
    pub(crate) isInEmbedBalloonMode: bool,
    // java public boolean tagForSpaceTrans;
    pub(crate) tagForSpaceTrans: bool,
    // java protected boolean mIsAllowTransForSpace;
    pub(crate) mIsAllowTransForSpace: bool,
    // java public int mEmbedBallooFontSize;
    pub(crate) mEmbedBallooFontSize: i32,
    // java public float roundError;
    pub(crate) roundError: f32,
    // java private WIntArrayList mDrawPosOffsetIndexes;
    mDrawPosOffsetIndexes: WIntArrayList,
    // java private WIntArrayList mDrawPosOffsetValues;
    mDrawPosOffsetValues: WIntArrayList,
    // java private OrgWidthCache mOrgWidthCache;
    mOrgWidthCache: Option<OrgWidthCache>,
    // java private WIntArrayList mMathParaRange;
    mMathParaRange: Option<WIntArrayList>,
    // java private ArrayList<PLCMathStart.OMathStartNode> mMathStartNode;
    mMathStartNode: Option<Vec<crate::cn::wps::moffice::writer::data::PLCMathStart::OMathStartNode>>,
    // java public BidiTool bidiTool;
    pub(crate) bidiTool: Option<BidiTool>,
    // java public int readEndCP;
    pub(crate) readEndCP: i32,
    // java public GraphArrayDeque<Graphs> mGraphsForEmbed;
    // Java GraphArrayDeque compares Graphs by object identity. The graph pool
    // owns stable Box nodes, so retain their addresses instead of cloned values.
    pub mGraphsForEmbed: Option<GraphArrayDeque<*mut Graphs>>,
    // java public WIntArrayList mEmbedIndex;
    pub mEmbedIndex: Option<WIntArrayList>,
    /// Java TypoDrawing content size, layout padding, and run font ascent/border,
    /// keyed by source CP. The final two values come from setDrawing's Graph.
    /// The command bridge reads this instead of recomputing a shape box.
    pub embed_places: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32)>,
    // Java keeps adjusted graph widths on the committed TypoPara. The mirror
    // records each accepted line's (cp, width) pairs before the next paragraph
    // clears that para, so the product command bridge still reads the same plane.
    pub committed_line_advances: Vec<Vec<(i32, i32)>>,
    // Cell TextLayouter instances commit into their own LayoutState. Carry the
    // accepted graph widths back to the page command bridge by line CP range.
    pub table_line_advances: Vec<(i32, i32, Vec<(i32, i32)>)>,
    // java public KPList<RevisionNode> revisions;
    pub(crate) revisions: KPList<RevisionNode>,
    // java public DocFieldCollector mDocFieldsCollector;
    pub(crate) mDocFieldsCollector: Option<DocFieldCollector>,
    // java public PredictSpacesInfo mPredictSpacesInfo;
    pub(crate) mPredictSpacesInfo: Option<PredictSpacesInfo>,
    // java public SignableRange mSignableRangeToReflow;
    pub(crate) mSignableRangeToReflow: Option<SignableRange>,
    // java protected SuperEnvoy mSuperEnvoy;
    pub mSuperEnvoy: SuperEnvoy,
    // java PageViewLayoutState.pagePos — same object as LayoutState in the page-view subclass
    pub pagePos: i32,
    // java PageViewLayoutState.brcTop
    pub brcTop: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    // java PageViewLayoutState.brcLeft
    pub brcLeft: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    // java PageViewLayoutState.brcRight
    pub brcRight: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    // java PageViewLayoutState.brcBottom
    pub brcBottom: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>,
    // java PageViewLayoutState.pageBorderProp
    pub pageBorderProp: Option<crate::cn::wps::moffice::writer::data::value::PageBorderProp::PageBorderProp>,
    // java FootEndNoteEnv.curFootNoteRefCP = -1
    pub curFootNoteRefCP: i32,
    // java FootEndNoteEnv.curEndNoteRefCP = -1
    pub curEndNoteRefCP: i32,
    // java FootEndNoteEnv.hasInitSecProp
    pub hasInitSecProp: bool,
}

// java public abstract static class Alloc extends KList.RecycableNodeAlloc<LayoutState>
pub struct Alloc {
    // java KList.RecycableNodeAlloc<LayoutState> super_
    pub(crate) super_: RecycableNodeAlloc,
    // java protected Pools mPools;
    pub(crate) mPools: Option<Pools>,
}

impl Default for Alloc {
    fn default() -> Self {
        Self { super_: RecycableNodeAlloc::new(), mPools: None }
    }
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            super_: RecycableNode::new(),
            pageEnv: Default::default(),
            paraEnv: Default::default(),
            tablePeriLayer: 1,
            textFrameLayer: 0,
            atLeastOneLine: true,
            document: None,
            range: 0,
            cellStateList: CellStateList::default(),
            curPage: None,
            linePos: 0,
            curLayoutHeight: 0,
            affectRect: TypoRectCoreImpl::default(),
            finishPage: false,
            validLine: false,
            hasFlowTableHeader: false,
            needCalcSectionBreakWidth: false,
            isShowRevisionInsOrDelInBalloons: false,
            cp: 0,
            tableHeaderRange: 0,
            maxWrapDrawingCP: i32::MIN,
            pageEndLimitCP: i32::MAX,
            typoPara: Default::default(),
            isRevision: false,
            srcList: Default::default(),
            charsList: Default::default(),
            drawingList: None,
            graphList: Default::default(),
            styleResult: Default::default(),
            mGraphsIt: GraphsIt::default(),
            mFirstGraphIndex: 0,
            hasBidiGraph: false,
            hasCoreCharsContent: false,
            hasReadEnd: false,
            hasMaths: false,
            hasEmbeds: false,
            hasEquation: false,
            paraIsOverflow: false,
            isInMathPara: false,
            isInEmbedBalloonMode: false,
            tagForSpaceTrans: false,
            mIsAllowTransForSpace: false,
            mEmbedBallooFontSize: 157,
            roundError: 0.0,
            mDrawPosOffsetIndexes: WIntArrayList::new(),
            mDrawPosOffsetValues: WIntArrayList::new(),
            mOrgWidthCache: None,
            mMathParaRange: None,
            mMathStartNode: None,
            bidiTool: None,
            readEndCP: 0,
            mGraphsForEmbed: None,
            mEmbedIndex: None,
            embed_places: Vec::new(),
            committed_line_advances: Vec::new(),
            table_line_advances: Vec::new(),
            revisions: Default::default(),
            mDocFieldsCollector: None,
            mPredictSpacesInfo: None,
            mSignableRangeToReflow: None,
            mSuperEnvoy: SuperEnvoy::default(),
            pagePos: 0,
            brcTop: None,
            brcLeft: None,
            brcRight: None,
            brcBottom: None,
            pageBorderProp: None,
            curFootNoteRefCP: -1,
            curEndNoteRefCP: -1,
            hasInitSecProp: false,
        }
    }
}

// java public class LayoutState extends KList.RecycableNode
impl LayoutState {
    // java public void clearPredictSpaces()
    pub fn clearPredictSpaces(&mut self) {
      // java this.mPredictSpacesInfo = null;
      self.mPredictSpacesInfo = None;
    }

    // java public  LayoutState(SuperEnvoy superEnvoy)
    pub fn LayoutState(superEnvoy: Option<SuperEnvoy>) -> Self {
      // java this.mGraphsIt = this.graphList.iterator();
      let mut this = Self::default();
      // java this.tagForSpaceTrans = false;
      // java this.mIsAllowTransForSpace = false;
      // java this.mEmbedBallooFontSize = 157;
      // java this.mDrawPosOffsetIndexes = new WIntArrayList();
      // java this.mDrawPosOffsetValues = new WIntArrayList();
      // java this.revisions = new KPList(new RevisionNode.Alloc());
      // java this.mSuperEnvoy = superEnvoy != null ? superEnvoy : new SuperEnvoy();
      this.mSuperEnvoy = superEnvoy.unwrap_or_else(SuperEnvoy::default);
      // java this.mSuperEnvoy.setLayoutState(this);
      let p = &mut this as *mut LayoutState;
      let envoy = &mut this.mSuperEnvoy as *mut SuperEnvoy;
      unsafe { (*envoy).setLayoutState(p.cast()); }
      this
    }

    // java public int getEmbedSize()
    pub fn getEmbedSize(&self) -> i32 {
      // Java: return this.mGraphsForEmbed == null ? 0 : this.mGraphsForEmbed.size();
      self.mGraphsForEmbed.as_ref().map_or(0, GraphArrayDeque::size)
    }

    // java public SuperEnvoy getEnvoy()
    pub fn getEnvoy(&mut self) -> &mut SuperEnvoy {
      // java return this.mSuperEnvoy;
      // Rust moves LayoutState after its constructor; restore Java's stable
      // back-reference at the point the envoy is handed to a layouter.
      let state = self as *mut LayoutState;
      self.mSuperEnvoy.setLayoutState(state.cast());
      return &mut self.mSuperEnvoy;
    }

    // java public boolean isFinishPage()
    pub fn isFinishPage(&self) -> bool {
      // java return this.finishPage;
      return self.finishPage;
    }

    // java public void setFinishPage(boolean finishPage)
    pub fn setFinishPage(&mut self, finishPage: bool) {
      // java this.finishPage = finishPage;
      self.finishPage = finishPage;
    }

    // java public void setRange(int start, int end)
    pub fn setRange__int_int(&mut self, start: i32, end: i32) {
      // java this.range = RANGEFactory.getRANGE(start, end);
      self.range = RANGEFactory::getRANGE(start, end);
    }

    // java public void setRange(long range)
    pub fn setRange__long(&mut self, range: i64) {
      // java this.range = range;
      self.range = range;
    }

    // java public long getRange()
    pub fn getRange(&self) -> i64 {
      // java return this.range;
      self.range
    }

    // java public int getRangeEnd()
    pub fn getRangeEnd(&self) -> i32 {
      // java return RANGEFactory.getEnd(this.range);
      RANGEFactory::getEnd(self.range)
    }

    // java public int getRangeStart()
    pub fn getRangeStart(&self) -> i32 {
      // java return RANGEFactory.getStart(this.range);
      RANGEFactory::getStart(self.range)
    }

    // java public String getPosString()
    pub fn getPosString(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StringBuffer strBuff = new StringBuffer();
      let mut strBuff: String = { let __x: String = todo!("no align strategy matched"); __x };
      // java KPList.Iterator<SrcNode> srcIt = this.srcList.iterator();
      let mut srcIt: Vec<SrcNode> = self.srcList.iterator();
      // java srcIt.seekBegin();
      srcIt.seekBegin();
      // java SrcNode srcNode = null;
      let mut srcNode: SrcNode = Default::default();
      // java while (null != (srcNode = (SrcNode)srcIt.next()))
      while { srcNode = (srcIt.next()).downcast_ref::<SrcNode>().unwrap(); srcNode }.is_some() {
        // java if (0 == srcNode.type)
        if 0 == srcNode.r#type {
          // java strBuff.append(" ");
          strBuff.append(" ");
          // java strBuff.append(this.getDocumentName(srcNode.doc));
          strBuff.append(self.getDocumentName(srcNode.doc));
          // java strBuff.append(" cp=");
          strBuff.append(" cp=");
          // java strBuff.append(srcNode.cp);
          strBuff.append(srcNode.cp);
          // java break;
          break;
        }
        // java if (1 != srcNode.type)
        if 1 != srcNode.r#type {
          // java strBuff.append("BalloonInfo=");
          strBuff.append("BalloonInfo=");
          // java strBuff.append(srcNode.data);
          strBuff.append(srcNode.data);
          // java break;
          break;
        }
        // java strBuff.append("AutoNum=");
        strBuff.append("AutoNum=");
        // java strBuff.append(srcNode.data);
        strBuff.append(srcNode.data);
      }
      // java srcIt.recycle();
      srcIt.recycle();
      // java return strBuff.toString();
      return strBuff.toString();
      */
    }

    // java final String getDocumentName(Document document)
    pub fn getDocumentName(&self, document: &dyn Document) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (document.getType())
      match document.getType() {
        // java case 0:
        0 => {
          // java return "MAIN_DOCUMENT";
          return "MAIN_DOCUMENT";
        }
        // java case 1:
        1 => {
          // java return "FOOTNOTE_DOCUMENT";
          return "FOOTNOTE_DOCUMENT";
        }
        // java case 2:
        2 => {
          // java return "HEADER_DOCUMENT";
          return "HEADER_DOCUMENT";
        }
        // java case 3:
        3 => {
          // java return "COMMENT_DOCUMENT";
          return "COMMENT_DOCUMENT";
        }
        // java case 4:
        4 => {
          // java return "ENDNOTE_DOCUMENT";
          return "ENDNOTE_DOCUMENT";
        }
        // java case 5:
        5 => {
          // java return "TEXTBOX_DOCUMENT";
          return "TEXTBOX_DOCUMENT";
        }
        // java case 6:
        6 => {
          // java return "HEADERTEXTBOX_DOCUMENT";
          return "HEADERTEXTBOX_DOCUMENT";
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
      */
    }

    // java public KPList<Graphs> getGraphList()
    pub fn getGraphList(&mut self) -> &mut KPList<Graphs> {
      // java return this.graphList;
      &mut self.graphList
    }

    // java public Graphs nextGraphs()
    pub fn nextGraphs(&mut self) -> Graphs {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (Graphs)this.mGraphsIt.value();
      return (self.mGraphsIt.value()).downcast_ref::<Graphs>().unwrap();
      */
    }

    // java public int getDrawing(Graphs run)
    pub fn getDrawing(&mut self, run: &Graphs) -> i32 {
      // Java: if (null != this.mGraphsForEmbed) {
      if let Some(graphs_for_embed) = self.mGraphsForEmbed.as_ref() {
        // Java: int index = this.mGraphsForEmbed.indexOf(run);
        let index = graphs_for_embed.indexOf(&(run as *const Graphs as *mut Graphs));
        // Java: if (index >= 0 && null != this.mEmbedIndex && index < this.mEmbedIndex.size())
        if index >= 0 {
          if let Some(embed_index) = self.mEmbedIndex.as_ref() {
            if index < embed_index.size() {
              // Java: return this.typoPara.getEmbed(this.mEmbedIndex.get(index));
              return self.typoPara.getEmbed(embed_index.get(index));
            }
          }
        }
      }
      // Java: return 0;
      0
    }

    // java public int getIndexOfDrawing(Graphs graphs)
    pub fn getIndexOfDrawing(&mut self, graphs: &Graphs) -> i32 {
      // Java: int index = this.mGraphsForEmbed.indexOf(graphs);
      let index = self.mGraphsForEmbed.as_ref()
        .map_or(-1, |queue| queue.indexOf(&(graphs as *const Graphs as *mut Graphs)));
      // Java: if (index >= 0 && null != this.mEmbedIndex && index < this.mEmbedIndex.size())
      if index >= 0 {
        if let Some(indices) = self.mEmbedIndex.as_ref() {
          if index < indices.size() {
            // Java: return this.mEmbedIndex.get(index);
            return indices.get(index);
          }
        }
      }
      // Java: return -1;
      -1
    }

    // java public KPList<DrawingNode> getDrawings()
    pub fn getDrawings(&self) -> KPList {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.drawingList;
      return self.drawingList;
      */
    }

    // java public void push(Graphs firstRun)
    pub fn push__graphs(&mut self, firstRun: &Graphs) {
      // java this.push(firstRun, (Graphs)null);
      let p: *mut Graphs = self
        .graphList
        .nodes
        .iter_mut()
        .find(|n| n.graphIndex == firstRun.graphIndex && n.graphLength == firstRun.graphLength)
        .map(|n| &mut **n as *mut Graphs)
        .unwrap_or(std::ptr::null_mut());
      self.push_pair_ptr(p, std::ptr::null_mut());
    }

    // rust: Java push(Graphs) 节点引用；breakGraphs 插入后的半段用指针回指
    pub fn push_ptr(&mut self, firstRun: *mut Graphs) {
      self.push_pair_ptr(firstRun, std::ptr::null_mut());
    }

    // java public void push(Graphs firstRun, Graphs restoreFirst)
    pub fn push__graphs_graphs(&mut self, firstRun: Option<&Graphs>, restoreFirst: Option<&mut Graphs>) {
      let first = firstRun.map_or(std::ptr::null_mut(), |r| r as *const Graphs as *mut Graphs);
      let restore = restoreFirst.map_or(std::ptr::null_mut(), |r| r as *mut Graphs);
      self.push_pair_ptr(first, restore);
    }

    /// Java LayoutState.push(firstRun, restoreFirst), retaining graph identity.
    pub fn push_pair_ptr(&mut self, firstRun: *mut Graphs, restoreFirst: *mut Graphs) {
      if firstRun.is_null() {
        self.mGraphsIt.seekEnd();
        return;
      }
      let restoreFirst = if restoreFirst.is_null() { firstRun } else { restoreFirst };
      let restored = self.mOrgWidthCache.as_ref().is_some_and(|cache|
          cache.restore(unsafe { (*restoreFirst).graphIndex }, &mut self.typoPara));
      if restored {
        // Java's seekEnd()/value() names the final graph. GraphsIt uses a
        // sentinel there, so take the actual final node before walking back.
        let was_invalid = !self.mGraphsIt.valid();
        let mut cur = if was_invalid { self.graphList.lastElement() }
            else { self.mGraphsIt.value() };
        if was_invalid && !cur.is_null() {
          self.restore_graph_width(cur);
        }
        // Java: while (it.value() != restoreFirst) { it.prev();
        // curGraphs.changeWidthSum(...); ... }. This includes restoreFirst.
        while !cur.is_null() && cur != restoreFirst {
          cur = self.graphList.getPrev(cur);
          if !cur.is_null() { self.restore_graph_width(cur); }
        }
      }
      self.mGraphsIt.seek(firstRun);
    }

    fn restore_graph_width(&mut self, node: *mut Graphs) {
      let graph = unsafe { &mut *node };
      graph.changeWidthSum(self.typoPara.getSum(graph.graphIndex, graph.graphLength));
      if graph.autoSpaceType == 5 ||
          (TypoRun::isPunct(graph.runType) && self.getDrawPosOffset(graph.graphIndex) != 0) {
        self.addDrawPosOffsetX__int_int_boolean(graph.graphIndex, 0, false);
      }
    }

    // java public Graphs breakGraphs(Graphs run, boolean isRTL, int graphIndex, boolean pushBreak)
    pub fn breakGraphs(&mut self, run: &mut Graphs, isRTL: bool, graphIndex: i32, pushBreak: bool) -> Graphs {
      // java Graphs breakRun = (Graphs)this.graphList.obtainAndInsertAfter(run);
      let run_ptr: *mut Graphs = run as *mut Graphs;
      let break_ptr: *mut Graphs = self.graphList.obtainAndInsertAfter(run_ptr);
      // java breakRun.clone(run);
      unsafe { Graphs::clone(&mut (*break_ptr), &*run_ptr); }
      // java int runLen = graphIndex - run.graphIndex;
      let runLen: i32 = graphIndex - unsafe { (*run_ptr).graphIndex };
      // java int runWidth = 0;
      let mut runWidth: i32 = 0;
      // java if (run.autoSpaceType == 2 && !run.fontStyle.typeface.needBold())
      let auto_space = unsafe { (*run_ptr).autoSpaceType };
      let skip_hmtx = unsafe {
        (*run_ptr).fontStyle.as_ref().and_then(|fs| fs.typeface.as_ref()).map(|tf| !tf.needBold()).unwrap_or(false)
      };
      if auto_space == 2 && skip_hmtx {
        // java runWidth = run.getWithExcludeBdr() * runLen / run.graphLength;
        let glen = unsafe { (*run_ptr).graphLength };
        if glen != 0 {
          runWidth = unsafe { (*run_ptr).getWithExcludeBdr() } * runLen / glen;
        }
      } else {
        // java runWidth = isRTL ? this.typoPara.getAbsSum(run.graphIndex, runLen) : this.typoPara.getSum(run.graphIndex, runLen);
        let gi = unsafe { (*run_ptr).graphIndex };
        runWidth = if isRTL { self.typoPara.getAbsSum(gi, runLen) } else { self.typoPara.getSum(gi, runLen) };
      }
      // java run.graphLength = runLen;
      unsafe { (*run_ptr).graphLength = runLen; }
      // java int bdrWidth = breakRun.getBdrWidth();
      let bdrWidth: i32 = unsafe { (*break_ptr).getBdrWidth() };
      // java run.clearBdr();
      unsafe { (*run_ptr).clearBdr(); }
      // java run.mWidth = runWidth;
      unsafe { (*run_ptr).mWidth = runWidth; }
      // java if (breakRun.hasBeginBdr())
      if unsafe { (*break_ptr).hasBeginBdr() } {
        // java run.setBeginBdr(bdrWidth);
        unsafe { (*run_ptr).setBeginBdr(bdrWidth); }
      }
      // java breakRun.mWidth = breakRun.getWithExcludeBdr() - runWidth;
      unsafe { (*break_ptr).mWidth = (*break_ptr).getWithExcludeBdr() - runWidth; }
      // java breakRun.offsetX(run.width());
      unsafe { (*break_ptr).offsetX((*run_ptr).width()); }
      // java boolean hasEndBdr = breakRun.hasEndBdr();
      let hasEndBdr: bool = unsafe { (*break_ptr).hasEndBdr() };
      // java breakRun.clearBdr();
      unsafe { (*break_ptr).clearBdr(); }
      // java if (hasEndBdr)
      if hasEndBdr {
        // java breakRun.setEndBdr(bdrWidth);
        unsafe { (*break_ptr).setEndBdr(bdrWidth); }
      }
      // java breakRun.graphIndex = graphIndex;
      unsafe { (*break_ptr).graphIndex = graphIndex; }
      // java breakRun.graphLength -= runLen;
      unsafe { (*break_ptr).graphLength = (*break_ptr).graphLength - runLen; }
      // java if (pushBreak)
      if pushBreak {
        // java this.push(breakRun);
        self.push_ptr(break_ptr);
      }
      // java return breakRun;
      let mut out = Graphs::Graphs();
      unsafe { Graphs::clone(&mut out, &*break_ptr); }
      return out;
      /* DEFERRED_NON_FUNCTION 原模版体（指针版已接线；保留 Java 注释对照）
      // java Graphs breakRun = (Graphs)this.graphList.obtainAndInsertAfter(run);
      let mut breakRun: Graphs = (self.graphList.obtainAndInsertAfter(run)).downcast_ref::<Graphs>().unwrap();
      // java breakRun.clone(run);
      breakRun.clone(run);
      // java int runLen = graphIndex - run.graphIndex;
      let mut runLen: i32 = graphIndex - run.graphIndex;
      // java int runWidth = 0;
      let mut runWidth: i32 = 0;
      // java if (run.autoSpaceType == 2 && !run.fontStyle.typeface.needBold())
      if run.autoSpaceType == 2 && !run.fontStyle.typeface.needBold() {
        // java runWidth = run.getWithExcludeBdr() * runLen / run.graphLength;
        runWidth = run.getWithExcludeBdr() * runLen / run.graphLength;
      }
      // java else
      else {
        // java runWidth = isRTL ? this.typoPara.getAbsSum(run.graphIndex, runLen) : this.typoPara.getSum(run.graphIndex, runLen);
        runWidth = (if isRTL { self.typoPara.getAbsSum(run.graphIndex, runLen) } else { self.typoPara.getSum(run.graphIndex, runLen) });
      }
      // java run.graphLength = runLen;
      run.graphLength = runLen;
      // java int bdrWidth = breakRun.getBdrWidth();
      let mut bdrWidth: i32 = breakRun.getBdrWidth();
      // java run.clearBdr();
      run.clearBdr();
      // java run.mWidth = runWidth;
      run.mWidth = runWidth;
      // java if (breakRun.hasBeginBdr())
      if breakRun.hasBeginBdr() {
        // java run.setBeginBdr(bdrWidth);
        run.setBeginBdr(bdrWidth);
      }
      // java breakRun.mWidth = breakRun.getWithExcludeBdr() - runWidth;
      breakRun.mWidth = breakRun.getWithExcludeBdr() - runWidth;
      // java breakRun.offsetX(run.width());
      breakRun.offsetX(run.width());
      // java boolean hasEndBdr = breakRun.hasEndBdr();
      let mut hasEndBdr: bool = breakRun.hasEndBdr();
      // java breakRun.clearBdr();
      breakRun.clearBdr();
      // java if (hasEndBdr)
      if hasEndBdr {
        // java breakRun.setEndBdr(bdrWidth);
        breakRun.setEndBdr(bdrWidth);
      }
      // java breakRun.graphIndex = graphIndex;
      breakRun.graphIndex = graphIndex;
      // java breakRun.graphLength -= runLen;
      breakRun.graphLength = breakRun.graphLength - runLen;
      // java if (pushBreak)
      if pushBreak {
        // java this.push(breakRun);
        self.push__graphs(breakRun);
      }
      // java return breakRun;
      return breakRun;
      */
    }

    // java public int getAbsSum(Graphs run, int graphIndex, int length, boolean isRTL)
    pub fn getAbsSum(&self, run: &mut Graphs, graphIndex: i32, length: i32, isRTL: bool) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (length == 0)
      if length == 0 {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int sumWidth = 0;
        let mut sumWidth: i32 = 0;
        // java if (run.autoSpaceType == 2 && !run.fontStyle.typeface.needBold())
        if run.autoSpaceType == 2 && !run.fontStyle.typeface.needBold() {
          // java sumWidth = run.width() * length / run.graphLength;
          sumWidth = run.width() * length / run.graphLength;
        }
        // java else
        else {
          // java sumWidth = isRTL ? this.typoPara.getAbsSum(graphIndex, length) : this.typoPara.getSum(graphIndex, length);
          sumWidth = (if isRTL { self.typoPara.getAbsSum(graphIndex, length) } else { self.typoPara.getSum(graphIndex, length) });
        }
        // java return sumWidth;
        return sumWidth;
      }
      */
    }

    // java public Graphs get()
    pub fn get(&mut self) -> *mut Graphs {
      // java Graphs graph = (Graphs)this.mGraphsIt.next();
      if self.mGraphsIt.list.is_null() {
        self.mGraphsIt.attach(&mut self.graphList as *mut KPList<Graphs>);
      }
      let graph: *mut Graphs = self.mGraphsIt.next();
      // java return graph;
      graph
    }

    // java public Graphs getNextSectionBreak()
    pub fn getNextSectionBreak(&mut self) -> *mut Graphs {
      // java if (this.graphList.isEmpty())
      if self.graphList.is_empty() {
        // java return null;
        return std::ptr::null_mut();
      }
      // java Graphs rs = (Graphs)this.mGraphsIt.value();
      let rs = self.mGraphsIt.value();
      // java switch (rs.runType) { case 10: case 11: case 12: case 13: return this.get(); }
      if !rs.is_null() && matches!(unsafe { (*rs).runType }, 10 | 11 | 12 | 13) { self.get() } else { std::ptr::null_mut() }
    }

    // java public int getCP(int graphIndex)
    pub fn getCP(&self, graphIndex: i32) -> i32 {
      // java return this.typoPara.getCP(graphIndex);
      self.typoPara.getCP(graphIndex)
    }

    // java public int extendWidths(Graphs run, int spacePerChar, int graphIndex, int extendLen)
    pub fn extendWidths(&mut self, run: &Graphs, spacePerChar: i32, graphIndex: i32, extendLen: i32) -> i32 {
      let _ = run;
      // java this.typoPara.extendWidths(graphIndex, extendLen, spacePerChar);
      self.typoPara.extendWidths(graphIndex, extendLen, spacePerChar);
      // java int addWidthSum = spacePerChar * extendLen;
      let addWidthSum: i32 = spacePerChar * extendLen;
      // java return addWidthSum;
      addWidthSum
    }

    // java public int extendFloatWidths(Graphs run, float spacePerChar, int graphIndex, int extendLen)
    pub fn extendFloatWidths(&mut self, run: &Graphs, spacePerChar: f32, graphIndex: i32, extendLen: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.typoPara.extendFloatWidths(graphIndex, extendLen, spacePerChar);
      return self.typoPara.extendFloatWidths(graphIndex, extendLen, spacePerChar);
      */
    }

    // java public int scaleWidths(Graphs run, float scale, int graphIndex, int extendLen, int space)
    pub fn scaleWidths(&mut self, run: &Graphs, scale: f32, graphIndex: i32, extendLen: i32, space: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.typoPara.scaleWidths(graphIndex, extendLen, scale, space);
      return self.typoPara.scaleWidths(graphIndex, extendLen, scale, space);
      */
    }

    // java public boolean hasContents()
    pub fn hasContents(&self) -> bool {
      // java return !this.srcList.isEmpty();
      !self.srcList.isEmpty()
    }

    // java public boolean hasBidiGraph()
    pub fn hasBidiGraph(&self) -> bool {
      // java return this.hasBidiGraph;
      self.hasBidiGraph
    }

    // java public boolean hasCoreCharsContent()
    pub fn hasCoreCharsContent(&self) -> bool {
      // java return this.hasCoreCharsContent;
      self.hasCoreCharsContent
    }

    // java public PLCMathStart.OMathStartNode getOMathStart(int cp)
    pub fn getOMathStart(&self, cp: i32) -> Option<OMathStartNode> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java WIntArrayList mathParaRange = this.mMathParaRange;
      let mut mathParaRange: WIntArrayList = self.mMathParaRange.unwrap();
      // java if (null != mathParaRange)
      if true {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = (mathParaRange.len() as i32) / 2;
        // java for(int size = mathParaRange.size() / 2; i < size; ++i)
        while i < size {
          // java int start = mathParaRange.get(i * 2);
          let mut start: i32 = mathParaRange.get(i * 2);
          // java int end = mathParaRange.get(i * 2 + 1);
          let mut end: i32 = mathParaRange.get(i * 2 + 1);
          // java if (cp >= start && cp < end)
          if cp >= start && cp < end {
            // java return (PLCMathStart.OMathStartNode)this.mMathStartNode.get(i);
            return (self.mMathStartNode.get(i)).downcast_ref::<PLCMathStart::OMathStartNode>().unwrap();
          }
        }
      }
      // java return null;
      return None;
      */
    }

    // java public void addMathParaRange(int mathParaStart, int mathParaEnd, PLCMathStart.OMathStartNode startNode)
    pub fn addMathParaRange(&mut self, mathParaStart: i32, mathParaEnd: i32, startNode: &crate::cn::wps::moffice::writer::data::PLCMathStart::OMathStartNode) {
      // java if (null == this.mMathParaRange) this.mMathParaRange = new WIntArrayList();
      if self.mMathParaRange.is_none() {
        self.mMathParaRange = Some(WIntArrayList::new());
      }
      // java if (null == this.mMathStartNode) this.mMathStartNode = new ArrayList();
      if self.mMathStartNode.is_none() {
        self.mMathStartNode = Some(Vec::new());
      }
      if let Some(range) = self.mMathParaRange.as_mut() {
        range.add(mathParaStart);
        range.add(mathParaEnd);
      }
      if let Some(nodes) = self.mMathStartNode.as_mut() {
        nodes.push(startNode.clone());
      }
      // java this.isInMathPara = true;
      self.isInMathPara = true;
    }

    // java public void saveOrgWidth(Graphs graph)
    pub fn saveOrgWidth(&mut self, graph: &Graphs) {
      // Java: if (null == this.mOrgWidthCache) this.mOrgWidthCache = new OrgWidthCache();
      if self.mOrgWidthCache.is_none() { self.mOrgWidthCache = Some(OrgWidthCache::new()); }
      // Java: this.mOrgWidthCache.save(graph.graphIndex, graph.graphLength, this.typoPara);
      self.mOrgWidthCache.as_mut().unwrap().save(graph.graphIndex, graph.graphLength, &self.typoPara);
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mOrgWidthCache)
      if self.mOrgWidthCache.is_none() {
        // java this.mOrgWidthCache = new OrgWidthCache();
        self.mOrgWidthCache = OrgWidthCache::new();
      }
      // java this.mOrgWidthCache.save(graph.graphIndex, graph.graphLength, this.typoPara);
      self.mOrgWidthCache.save(graph.graphIndex, graph.graphLength, self.typoPara);
      */
    }

    // java public void checkSaveOrgWidth(Graphs graph)
    pub fn checkSaveOrgWidth(&mut self, graph: &Graphs) {
      // Java: if (null == this.mOrgWidthCache) this.mOrgWidthCache = new OrgWidthCache();
      if self.mOrgWidthCache.is_none() { self.mOrgWidthCache = Some(OrgWidthCache::new()); }
      // Java: this.mOrgWidthCache.checkSave(graph.graphIndex, graph.graphLength, this.typoPara);
      self.mOrgWidthCache.as_mut().unwrap().checkSave(graph.graphIndex, graph.graphLength, &self.typoPara);
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mOrgWidthCache)
      if self.mOrgWidthCache.is_none() {
        // java this.mOrgWidthCache = new OrgWidthCache();
        self.mOrgWidthCache = OrgWidthCache::new();
      }
      // java this.mOrgWidthCache.checkSave(graph.graphIndex, graph.graphLength, this.typoPara);
      self.mOrgWidthCache.checkSave(graph.graphIndex, graph.graphLength, self.typoPara);
      */
    }

    // java public void restoreToOrgWidth(Graphs graph)
    pub fn restoreToOrgWidth(&mut self, graph: &Graphs) {
      // Java: if (this.mOrgWidthCache != null) this.mOrgWidthCache.restore(graph.graphIndex, this.typoPara);
      if let Some(cache) = self.mOrgWidthCache.as_ref() { cache.restore(graph.graphIndex, &mut self.typoPara); }
      /* DEFERRED_NON_FUNCTION
      // java if (this.mOrgWidthCache != null)
      if let Some(mOrgWidthCache) = self.mOrgWidthCache.as_mut() {
        // java this.mOrgWidthCache.restore(graph.graphIndex, this.typoPara);
        mOrgWidthCache.restore(graph.graphIndex, self.typoPara);
      }
      */
    }

    // java public void addDrawPosOffsetX(int graphIndex, int offset)
    pub fn addDrawPosOffsetX__int_int(&mut self, graphIndex: i32, offset: i32) {
      // Java: this.addDrawPosOffsetX(graphIndex, offset, true);
      self.addDrawPosOffsetX__int_int_boolean(graphIndex, offset, true);
      /* DEFERRED_NON_FUNCTION
      // java this.addDrawPosOffsetX(graphIndex, offset, true);
      self.addDrawPosOffsetX__int_int_boolean(graphIndex, offset, true);
      */
    }

    // java public void addDrawPosOffsetX(int graphIndex, int offset, boolean addOffset)
    pub fn addDrawPosOffsetX__int_int_boolean(&mut self, graphIndex: i32, offset: i32, addOffset: bool) {
      // Java: insert or update the sorted offset pair for graphIndex.
      let indexes = &mut self.mDrawPosOffsetIndexes;
      let values = &mut self.mDrawPosOffsetValues;
      if !indexes.isEmpty() && indexes.get(indexes.size() - 1) >= graphIndex {
        let index = indexes.binarySearch(graphIndex);
        if index >= 0 {
          values.setQuick(index, if addOffset { values.get(index) + offset } else { offset });
        } else {
          let insertIndex = -index - 1;
          indexes.insert(insertIndex, graphIndex);
          values.insert(insertIndex, offset);
        }
      } else {
        indexes.add(graphIndex);
        values.add(offset);
      }
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mDrawPosOffsetIndexes.isEmpty() && this.mDrawPosOffsetIndexes.get(this.mDrawPosOffsetIndexes.size() - 1) >= graphIndex)
      if !self.mDrawPosOffsetIndexes.isEmpty() && self.mDrawPosOffsetIndexes.get(self.mDrawPosOffsetIndexes.size() - 1) >= graphIndex {
        // java int index = this.mDrawPosOffsetIndexes.binarySearch(graphIndex);
        let mut index: i32 = self.mDrawPosOffsetIndexes.binarySearch__int(graphIndex);
        // java if (index >= 0)
        if index >= 0 {
          // java this.mDrawPosOffsetValues.setQuick(index, addOffset ? this.mDrawPosOffsetValues.get(index) + offset : offset);
          self.mDrawPosOffsetValues.setQuick(index, (if addOffset { self.mDrawPosOffsetValues.get(index) + offset } else { offset }));
        }
        // java else
        else {
          // java int insertIndex = -index - 1;
          let mut insertIndex: i32 = -index - 1;
          // java this.mDrawPosOffsetIndexes.insert(insertIndex, graphIndex);
          self.mDrawPosOffsetIndexes.insert__int_int(insertIndex, graphIndex);
          // java this.mDrawPosOffsetValues.insert(insertIndex, offset);
          self.mDrawPosOffsetValues.insert__int_int(insertIndex, offset);
        }
      }
      // java else
      else {
        // java this.mDrawPosOffsetIndexes.add(graphIndex);
        self.mDrawPosOffsetIndexes.add__int(graphIndex);
        // java this.mDrawPosOffsetValues.add(offset);
        self.mDrawPosOffsetValues.add__int(offset);
      }
      */
    }

    // java public WIntArrayList getDrawPosOffsetIndexes()
    pub fn getDrawPosOffsetIndexes(&self) -> &WIntArrayList {
      // java return this.mDrawPosOffsetIndexes;
      &self.mDrawPosOffsetIndexes
    }

    // java public WIntArrayList getDrawPosOffsetValues()
    pub fn getDrawPosOffsetValues(&self) -> &WIntArrayList {
      // java return this.mDrawPosOffsetValues;
      &self.mDrawPosOffsetValues
    }

    // java public int getDrawPosOffset(int graphIndex)
    pub fn getDrawPosOffset(&self, graphIndex: i32) -> i32 {
      // java int index = this.mDrawPosOffsetIndexes.binarySearch(graphIndex);
      let index = self.mDrawPosOffsetIndexes.binarySearch(graphIndex);
      // java return index >= 0 ? this.mDrawPosOffsetValues.get(index) : 0;
      if index >= 0 { self.mDrawPosOffsetValues.get(index) } else { 0 }
    }

    // java public boolean hasMaths()
    pub fn hasMaths(&self) -> bool {
      // Java: return this.hasMaths;
      self.hasMaths
      /* DEFERRED_NON_FUNCTION
      // java return this.hasMaths;
      return self.hasMaths;
      */
    }

    // java public boolean hasEmbed()
    pub fn hasEmbed(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.hasEmbeds;
      return self.hasEmbeds;
      */
    }

    // java public boolean hasEquation()
    pub fn hasEquation(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.hasEquation;
      return self.hasEquation;
      */
    }

    // java public void resetDrawPosOffset()
    pub fn resetDrawPosOffset(&mut self) {
      // java this.mDrawPosOffsetIndexes.resetQuick();
      self.mDrawPosOffsetIndexes.resetQuick();
      // java this.mDrawPosOffsetValues.resetQuick();
      self.mDrawPosOffsetValues.resetQuick();
    }

    // java public void resetGraphsIt()
    pub fn resetGraphsIt(&mut self) {
      // java this.mGraphsIt.seekBegin();
      self.mGraphsIt.seekBegin();
    }

    // java public void resetMathPara()
    pub fn resetMathPara(&mut self) {
      // java if (null != this.mMathParaRange)
      if let Some(mMathParaRange) = self.mMathParaRange.as_mut() {
        // java this.mMathParaRange.resetQuick();
        mMathParaRange.resetQuick();
      }
      // java if (null != this.mMathStartNode)
      if let Some(mMathStartNode) = self.mMathStartNode.as_mut() {
        // java this.mMathStartNode.clear();
        mMathStartNode.clear();
      }
    }

    // java public void processRTLGraphWidth(Graphs run)
    pub fn processRTLGraphWidth(&mut self, run: &Graphs) {
      // java this.typoPara.processRTLGraphWidth(run.graphIndex, run.graphLength);
      self.typoPara.processRTLGraphWidth(run.graphIndex, run.graphLength);
    }

    // java public void clearWitchCache()
    pub fn clearWitchCache(&mut self) {
      // java if (null != this.mOrgWidthCache)
      if let Some(mOrgWidthCache) = self.mOrgWidthCache.as_mut() {
        // java this.mOrgWidthCache.clear();
        mOrgWidthCache.clear();
      }
    }

    // java public void consumeGraphsBefore(Graphs lastGraphs, boolean isEmbedOnlyInPhoneVeiw)
    pub fn consumeGraphsBefore(&mut self, lastGraphs: &Graphs, isEmbedOnlyInPhoneVeiw: bool) {
      // java this.clearWitchCache();
      self.clearWitchCache();
      // java this.mFirstGraphIndex = lastGraphs.graphIndex + lastGraphs.graphLength;
      self.mFirstGraphIndex = lastGraphs.graphIndex + lastGraphs.graphLength;
      // java this.cp = this.typoPara.getNextCP(this.mFirstGraphIndex - 1);
      self.cp = self.typoPara.getNextCP(self.mFirstGraphIndex - 1);
      // rust: Java lastGraphs 是 graphList 节点引用；Rust mLastRun 为值拷贝 → 按 graphIndex 回指池节点
      let last_ptr: *mut Graphs = self
        .graphList
        .nodes
        .iter_mut()
        .find(|n| n.graphIndex == lastGraphs.graphIndex)
        .map(|n| &mut **n as *mut Graphs)
        .unwrap_or(std::ptr::null_mut());
      // java Graphs next = (Graphs)this.graphList.getNext(lastGraphs);
      let next: *mut Graphs = if last_ptr.is_null() {
        std::ptr::null_mut()
      } else {
        self.graphList.getNext(last_ptr)
      };
      // java if (null != next)
      if !next.is_null() {
        // java boolean removeLastGraphOnly = false;
        let mut removeLastGraphOnly: bool = false;
        // java if (isEmbedOnlyInPhoneVeiw && lastGraphs.isEmbed())
        if isEmbedOnlyInPhoneVeiw && lastGraphs.isEmbed() {
          // java SrcNode firstSrcNode = (SrcNode)this.srcList.firstElement();
          let firstSrcNode: *mut crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode =
            self.srcList.firstElement();
          // java if (firstSrcNode != null && firstSrcNode.type == 1)
          if !firstSrcNode.is_null() && unsafe { (*firstSrcNode).r#type == 1 } {
            // java removeLastGraphOnly = true;
            removeLastGraphOnly = true;
          }
        }
        // java this.graphList.remove(this.graphList.firstElement(), lastGraphs);
        let first_graph = self.graphList.firstElement();
        self.graphList.remove(first_graph, last_ptr);
        // java if (!removeLastGraphOnly)
        if !removeLastGraphOnly {
          // java CharsNode charsRef = next.charsRef;
          let charsRef = unsafe { (*next).charsRef.as_ref() };
          // java if (null != charsRef)
          if let Some(charsRef) = charsRef {
            // java SrcNode srcNode = charsRef.srcRef;
            let srcNode = charsRef.srcRef.unwrap_or(std::ptr::null_mut());
            // java if (null != srcNode && srcNode != this.srcList.firstElement())
            if !srcNode.is_null() && srcNode != self.srcList.firstElement() {
              // java this.srcList.remove(this.srcList.firstElement(), this.srcList.getPrev(srcNode));
              let prev = self.srcList.getPrev(srcNode);
              let first_src = self.srcList.firstElement();
              self.srcList.remove(first_src, prev);
            }
            // java if (this.charsList.firstElement() != charsRef)
            let chars_ptr: *mut crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode = self
              .charsList
              .nodes
              .iter_mut()
              .find(|n| n.offsetOfRef == charsRef.offsetOfRef && n.lengthOfRef == charsRef.lengthOfRef)
              .map(|n| &mut **n as *mut _)
              .unwrap_or(std::ptr::null_mut());
            if !chars_ptr.is_null() && self.charsList.firstElement() != chars_ptr {
              // java this.charsList.remove(this.charsList.firstElement(), this.charsList.getPrev(charsRef));
              let prev = self.charsList.getPrev(chars_ptr);
              let first_chars = self.charsList.firstElement();
              self.charsList.remove(first_chars, prev);
            }
          }
        }
        // java this.mGraphsIt.seek(next);
        self.mGraphsIt.seek(next);
        // java this.cleanDrawPosOffsetBefore(this.mFirstGraphIndex);
        self.cleanDrawPosOffsetBefore(self.mFirstGraphIndex);
      } else {
        // java else
        // java CharsNode charsRef = lastGraphs.charsRef;
        let charsRef = lastGraphs.charsRef.as_ref();
        // java if (null != charsRef)
        if let Some(charsRef) = charsRef {
          // java SrcNode srcNode = charsRef.srcRef;
          let srcNode = charsRef.srcRef.unwrap_or(std::ptr::null_mut());
          // java if (null != srcNode)
          if !srcNode.is_null() {
            let src = unsafe { &mut *srcNode };
            // java if (0 != srcNode.type && 3 != srcNode.type)
            if 0 != src.r#type && 3 != src.r#type {
              // java this.srcList.remove(this.srcList.firstElement(), srcNode);
              let first_src = self.srcList.firstElement();
              self.srcList.remove(first_src, srcNode);
              // java this.charsList.clean();
              self.charsList.clean();
              // java this.graphList.clean();
              self.graphList.clean();
              // java this.hasReadEnd = false;
              self.hasReadEnd = false;
              // java return;
              return;
            }
            // java if (this.readEndCP < srcNode.cp + srcNode.length)
            if self.readEndCP < src.cp + src.length {
              // java int cpOffset = charsRef.offsetOfRef + charsRef.lengthOfRef;
              let cpOffset: i32 = charsRef.offsetOfRef + charsRef.lengthOfRef;
              // java srcNode.cp += cpOffset;
              src.cp += cpOffset;
              // java srcNode.length -= cpOffset;
              src.length -= cpOffset;
              // java if (srcNode.length > 0)
              if src.length > 0 {
                // java if (srcNode != this.srcList.firstElement())
                if srcNode != self.srcList.firstElement() {
                  // java this.srcList.remove(this.srcList.firstElement(), this.srcList.getPrev(srcNode));
                  let prev = self.srcList.getPrev(srcNode);
                  let first_src = self.srcList.firstElement();
                  self.srcList.remove(first_src, prev);
                }
                // java this.charsList.clean();
                self.charsList.clean();
                // java this.graphList.clean();
                self.graphList.clean();
                // java this.hasReadEnd = false;
                self.hasReadEnd = false;
                // java return;
                return;
              }
              // java if (this.srcList.lastElement() != srcNode)
              if self.srcList.lastElement() != srcNode {
                // java this.srcList.remove(this.srcList.firstElement(), srcNode);
                let first_src = self.srcList.firstElement();
                self.srcList.remove(first_src, srcNode);
                // java this.charsList.clean();
                self.charsList.clean();
                // java this.graphList.clean();
                self.graphList.clean();
                // java this.hasReadEnd = false;
                self.hasReadEnd = false;
                // java return;
                return;
              }
            }
          }
        }
        // java this.srcList.clean();
        self.srcList.clean();
        // java this.charsList.clean();
        self.charsList.clean();
        // java this.graphList.clean();
        self.graphList.clean();
        // java this.hasReadEnd = false;
        self.hasReadEnd = false;
      }
    }

    // java public boolean hasDrawing()
    pub fn hasDrawing(&self) -> bool {
      // java return null != this.drawingList && !this.drawingList.isEmpty();
      self.drawingList.as_ref().map(|l| !l.isEmpty()).unwrap_or(false)
    }

    // java public void addBidi(int graphIndex, char ch, DocumentEnv documentEnv)
    pub fn addBidi(&mut self, graphIndex: i32, ch: char, documentEnv: &DocumentEnv) {
      // java if (null == this.bidiTool) this.bidiTool = new BidiTool(documentEnv);
      if self.bidiTool.is_none() {
        self.bidiTool = Some(BidiTool::new(documentEnv as *const DocumentEnv as *mut DocumentEnv));
      }
      // java this.bidiTool.addBidi(graphIndex, ch);
      if let Some(tool) = self.bidiTool.as_mut() {
        tool.addBidi(graphIndex, ch);
      }
    }

    // java private void cleanDrawPosOffsetBefore(int graphIndex)
    pub fn cleanDrawPosOffsetBefore(&mut self, graphIndex: i32) {
      // java WIntArrayList drawPosOffsetIndex = this.mDrawPosOffsetIndexes;
      // java WIntArrayList drawPosOffsetValue = this.mDrawPosOffsetValues;
      // java while (drawPosOffsetIndex.size() > 0 && drawPosOffsetIndex.get(0) < graphIndex)
      while self.mDrawPosOffsetIndexes.size() > 0 && self.mDrawPosOffsetIndexes.get(0) < graphIndex {
        // java drawPosOffsetIndex.removeAt(0);
        self.mDrawPosOffsetIndexes.removeAt(0);
        // java drawPosOffsetValue.removeAt(0);
        self.mDrawPosOffsetValues.removeAt(0);
      }
    }

    // java public boolean isInMathParaLine()
    pub fn isInMathParaLine(&mut self) -> bool {
      // java return this.isInMathPara && ((Graphs)this.graphList.firstElement()).charsRef.isInMath;
      if !self.isInMathPara {
        return false;
      }
      let first = self.graphList.firstElement();
      if first.is_null() {
        return false;
      }
      unsafe { (*first).charsRef.as_ref().map(|c| c.isInMath).unwrap_or(false) }
    }

    // java public boolean isInMathPara()
    pub fn isInMathPara(&self) -> bool {
      // java return this.isInMathPara;
      self.isInMathPara
    }

    // java public boolean needFlowOMathInDrawing()
    pub fn needFlowOMathInDrawing(&mut self) -> bool {
      // java return false;
      false
    }

    // java public boolean isMathContentState()
    pub fn isMathContentState(&self) -> bool {
      // java return false;
      false
    }

    // java public final void addDrawing(Document mDocument, Shape shape, int cp)
    pub fn addDrawing(&mut self, mDocument: &dyn Document, shape: &Shape, cp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.drawingList)
      if self.drawingList.is_none() {
        // java this.drawingList = new KPList(new DrawingNode.Alloc());
        self.drawingList = KPList::new(Alloc::new());
      }
      // java DrawingNode drawing = (DrawingNode)this.drawingList.obtainAndAddLast();
      let mut drawing: DrawingNode = (self.drawingList.obtainAndAddLast()).downcast_ref::<DrawingNode>().unwrap();
      // java drawing.cp = cp;
      drawing.cp = cp;
      // java drawing.document = mDocument;
      drawing.document = mDocument;
      // java drawing.shape = shape;
      drawing.shape = shape;
      // java drawing.embed = false;
      drawing.embed = false;
      */
    }

    // java public void dump(StringBuilder sb)
    pub fn dump(&self, sb: &mut str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java sb.append(" cp=");
      sb.append(" cp=");
      // java sb.append(this.cp);
      sb.append(self.cp);
      // java sb.append(" range=[");
      sb.append(" range=[");
      // java sb.append(RANGEFactory.getStart(this.range));
      sb.append(RANGEFactory::getStart(self.range));
      // java sb.append(",");
      sb.append(",");
      // java sb.append(RANGEFactory.getEnd(this.range));
      sb.append(RANGEFactory::getEnd(self.range));
      // java sb.append(")");
      sb.append(")");
      // java if (null != this.document)
      if let Some(document) = self.document.as_ref() {
        // java sb.append("docType=" + this.document.getType());
        sb.append("docType=" + document.getType());
            // java sb.append(" docLen=" + this.document.getLength());
            sb.append(" docLen=" + document.getLength());
            // java Throwable var3
            todo!();
      }
      // java sb.append("\n");
      sb.append("\n");
      */
    }

    // java public void initCoreContent(int maxCP, boolean revisionFinal)
    pub fn initCoreContent(&mut self, maxCP: i32, revisionFinal: bool) {
      // java if (this.cp < maxCP)
      if self.cp < maxCP {
        // java SrcNode srcNode = (SrcNode)this.srcList.obtainAndAddLast();
        let srcNode = self.srcList.obtainAndAddLast();
        unsafe {
          // java srcNode.doc = this.document;
          (*srcNode).doc = self.document;
          // java srcNode.type = 0;
          (*srcNode).r#type = 0;
          // java srcNode.cp = this.cp;
          (*srcNode).cp = self.cp;
          // java srcNode.length = maxCP - this.cp;
          (*srcNode).length = maxCP - self.cp;
          // java srcNode.revisionFinal = revisionFinal;
          (*srcNode).revisionFinal = revisionFinal;
        }
      }
    }

    // java public void addBalloonInfoString(String str, PropertySet finalRunProp)
    pub fn addBalloonInfoString(&mut self, str: Option<&str>, finalRunProp: &RunPropertySet) {
      // java if (null != str && str.length() != 0)
      let Some(text) = str.filter(|s| !s.is_empty()) else { return };
      // java SrcNode srcNode = (SrcNode)this.srcList.obtainAndAddLast();
      let src_node = self.srcList.obtainAndAddLast();
      unsafe {
        // java srcNode.type = 2;
        (*src_node).r#type = 2;
        // java srcNode.length = str.length();
        (*src_node).length = text.chars().count() as i32;
        // java srcNode.data = str;
        (*src_node).data = Some(text.to_string());
        // java srcNode.finalRunProp = finalRunProp;
        (*src_node).finalRunProp = Some(finalRunProp.clone());
      }
    }

    // java public void addMathString(String str, Document doc, int cp)
    pub fn addMathString(&mut self, str: Option<&str>, doc: &dyn Document, cp: i32) {
      let _ = doc;
      let Some(text) = str.filter(|s| !s.is_empty()) else { return };
      let src_node = self.srcList.obtainAndAddLast();
      unsafe {
        (*src_node).r#type = 3;
        (*src_node).length = text.chars().count() as i32;
        (*src_node).data = Some(text.to_string());
        (*src_node).cp = cp;
        (*src_node).doc = self.document;
      }
    }

    // java public void extendsCoreContent(int maxCP, boolean revisionFinal)
    pub fn extendsCoreContent(&mut self, maxCP: i32, revisionFinal: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.srcList.isEmpty())
      if !self.srcList.is_empty() {
        // java SrcNode lastSrc = (SrcNode)this.srcList.lastElement();
        let mut lastSrc: SrcNode = (self.srcList.lastElement()).downcast_ref::<SrcNode>().unwrap();
        // java if (0 == lastSrc.type)
        if 0 == lastSrc.r#type {
          // java lastSrc.length = maxCP - lastSrc.cp;
          (lastSrc.len() as i32) = maxCP - lastSrc.cp;
          // java return;
          return;
        }
      }
      // java this.initCoreContent(maxCP, revisionFinal);
      self.initCoreContent(maxCP, revisionFinal);
      */
    }

    // java public void setDrawing(Graphs graphs, int pDrawing, int wordAlingFont, boolean useWord97LineBreakRules)
    pub fn setDrawing(&mut self, graphs: &mut Graphs, pDrawing: i32, wordAlingFont: i32, useWord97LineBreakRules: bool) {
      let _ = (wordAlingFont, useWord97LineBreakRules);
      if pDrawing == 0 || self.typoPara.snapshot.is_null() {
        return;
      }
      // java int embedIndex = this.typoPara.getEmbedSize(); this.typoPara.addEmbed(pDrawing);
      let embed_index = self.typoPara.getEmbedSize();
      self.typoPara.addEmbed(pDrawing);
      let snapshot = unsafe { &mut *self.typoPara.snapshot };
      // java DrawingResizeTool.expandRect(drawing, layoutPadding, rect); drawingWidth = rect.width();
      let content_height = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::height__int(pDrawing, snapshot);
      let raw_width = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::width__int(pDrawing, snapshot);
      let pad = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getLayoutRectPadding(pDrawing);
      let pl = crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot);
      let pt = crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot);
      let pr = crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot);
      let pb = crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot);
      let drawing_width = crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool::expanded_width(raw_width, pl, pr);
      let expanded_height = crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool::expanded_height(content_height, pt, pb);
      let (embed_ascent, embed_border) = if let Some(style) = graphs.fontStyle.as_mut() {
        let bdr = style.getBdrWidth();
        // java math align type 1 uses align line; otherwise the expanded rect height is the ascent.
        if crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isMath__int_typosnapshot(pDrawing, snapshot)
          && 1 == crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getAlignType__int_typosnapshot(pDrawing, snapshot)
        {
          style.maxAscent = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getAlignLine__int_typosnapshot(pDrawing, snapshot);
          style.maxDescent = content_height - style.maxAscent;
          style.maxAscent += bdr;
          style.maxDescent += bdr;
        } else {
          style.maxAscent = expanded_height + bdr;
          style.maxDescent = bdr;
        }
        (style.maxAscent, bdr)
      } else {
        (expanded_height, 0)
      };
      self.typoPara.putWidth(graphs.graphIndex, drawing_width);
      graphs.mWidth = drawing_width;
      if let Some(doc) = self.document {
        let cp = self.typoPara.getCP(graphs.graphIndex);
        crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::setCP__int(pDrawing, unsafe { &*doc }, snapshot, cp);
        self.embed_places.push((cp, raw_width, content_height, pl, pt, pr, pb, embed_ascent, embed_border));
      }
      // java if (null == this.mGraphsForEmbed) { new queues } this.mGraphsForEmbed.add(graphs); this.mEmbedIndex.add(embedIndex);
      if self.mGraphsForEmbed.is_none() {
        self.mGraphsForEmbed = Some(GraphArrayDeque::new());
        self.mEmbedIndex = Some(WIntArrayList::new());
      }
      if let Some(queue) = self.mGraphsForEmbed.as_mut() {
        queue.add(graphs as *mut Graphs);
      }
      if let Some(indices) = self.mEmbedIndex.as_mut() {
        indices.add(embed_index);
      }
      /* DEFERRED_NON_FUNCTION
      // java int embedIndex = this.typoPara.getEmbedSize();
      let mut embedIndex: i32 = self.typoPara.getEmbedSize();
      // java this.typoPara.addEmbed(pDrawing);
      self.typoPara.addEmbed(pDrawing);
      // java TypoSnapshot snapshot = this.typoPara.getSnapshot();
      let mut snapshot: TypoSnapshot = self.typoPara.getSnapshot();
      // java TypoToolsManager typoTools = snapshot.getTypoTools();
      let mut typoTools: TypoToolsManager = snapshot.getTypoTools();
      // java TypoRect rect = typoTools.obtainRect();
      let mut rect: TypoRect = typoTools.obtainRect();
      // java TypoDrawing drawing = typoTools.obtainTypoDrawing(pDrawing);
      let mut drawing: TypoDrawing = typoTools.obtainTypoDrawing(pDrawing);
      // java int pLayoutPadding = TypoDrawing.getLayoutRectPadding(pDrawing);
      let mut pLayoutPadding: i32 = TypoDrawing::getLayoutRectPadding(pDrawing);
      // java DrawingResizeTool.expandRect(drawing, TypoRectMemoryImpl.getLeft(pLayoutPadding, snapshot), TypoRectMemoryImpl.getTop(pLayoutPadding, snapshot), TypoRectMemoryImpl.getRight(pLayoutPadding, snapshot), TypoRectMemoryImpl.getBottom(pLayoutPadding, snapshot), rect);
      DrawingResizeTool::expandRect__ityporect_int_int_int_int_ityporect(drawing, TypoRectMemoryImpl::getLeft__int_typosnapshot(pLayoutPadding, snapshot), TypoRectMemoryImpl::getTop__int_typosnapshot(pLayoutPadding, snapshot), TypoRectMemoryImpl::getRight__int_typosnapshot(pLayoutPadding, snapshot), TypoRectMemoryImpl::getBottom__int_typosnapshot(pLayoutPadding, snapshot), rect);
      // java TypoFontStyle fontStyle = graphs.fontStyle;
      let mut fontStyle: TypoFontStyle = graphs.fontStyle;
      // java if (TypoDrawing.isMath(pDrawing, snapshot) && 1 == TypoDrawing.getAlignType(pDrawing, snapshot))
      if TypoDrawing::isMath__int_typosnapshot(pDrawing, snapshot) && 1 == TypoDrawing::getAlignType__int_typosnapshot(pDrawing, snapshot) {
        // java fontStyle.maxAscent = TypoDrawing.getAlignLine(pDrawing, snapshot);
        fontStyle.maxAscent = TypoDrawing::getAlignLine__int_typosnapshot(pDrawing, snapshot);
        // java fontStyle.maxDescent = drawing.height() - fontStyle.maxAscent;
        fontStyle.maxDescent = drawing.height() - fontStyle.maxAscent;
        // java fontStyle.maxAscent += fontStyle.getBdrWidth();
        fontStyle.maxAscent = fontStyle.maxAscent + fontStyle.getBdrWidth();
        // java fontStyle.maxDescent += fontStyle.getBdrWidth();
        fontStyle.maxDescent = fontStyle.maxDescent + fontStyle.getBdrWidth();
      }
      // java else
      else {
        // java fontStyle.maxAscent = rect.height() + fontStyle.getBdrWidth();
        fontStyle.maxAscent = rect.height() + fontStyle.getBdrWidth();
        // java fontStyle.maxDescent = fontStyle.getBdrWidth();
        fontStyle.maxDescent = fontStyle.getBdrWidth();
      }
      // java int drawingWidth = rect.width();
      let mut drawingWidth: i32 = rect.width();
      // java this.typoPara.putWidth(graphs.graphIndex, drawingWidth);
      self.typoPara.putWidth(graphs.graphIndex, drawingWidth);
      // java graphs.mWidth = drawingWidth;
      graphs.mWidth = drawingWidth;
      // java drawing.setCP(this.typoPara.getCP(graphs.graphIndex), this.document);
      drawing.setCP(self.typoPara.getCP(graphs.graphIndex), self.document);
      // java typoTools.recycle(rect);
      typoTools.recycle(rect);
      // java typoTools.recycle((TypoMemoryObj)drawing);
      typoTools.recycle((drawing).downcast_ref::<TypoMemoryObj>().unwrap());
      // java if (null == this.mGraphsForEmbed)
      if self.mGraphsForEmbed.is_none() {
        // java this.mGraphsForEmbed = new GraphArrayDeque<Graphs>();
        self.mGraphsForEmbed = GraphArrayDeque::new();
        // java this.mEmbedIndex = new WIntArrayList();
        self.mEmbedIndex = WIntArrayList::new();
      }
      // java this.mGraphsForEmbed.add(graphs);
      self.mGraphsForEmbed.add(graphs);
      // java this.mEmbedIndex.add(embedIndex);
      self.mEmbedIndex.add__int(embedIndex);
      */
    }

    // java public void setRangeStart(int start)
    pub fn setRangeStart(&mut self, start: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.range = RANGEFactory.getRANGE(start, this.getRangeEnd());
      self.range = RANGEFactory::getRANGE(start, self.getRangeEnd());
      */
    }

    // java public void setRangeEnd(int end)
    pub fn setRangeEnd(&mut self, end: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.range = RANGEFactory.getRANGE(this.getRangeStart(), end);
      self.range = RANGEFactory::getRANGE(self.getRangeStart(), end);
      */
    }

    // java public SrcNode getFirstCoreSrc()
    pub fn getFirstCoreSrc(&self) -> SrcNode {
      // java while (null != (node = it.next()) && node.type != 0 && node.type != 3)
      if let Some(n) = self.srcList.nodes.iter().find(|n| n.r#type == 0 || n.r#type == 3) {
        let mut out = SrcNode::SrcNode();
        out.r#type = n.r#type;
        out.revisionFinal = n.revisionFinal;
        out.doc = n.doc;
        out.cp = n.cp;
        out.length = n.length;
        out.data = n.data.clone();
        out.finalRunProp = n.finalRunProp.clone();
        out
      } else {
        SrcNode::SrcNode()
      }
    }

    // java public boolean skipSpaceBeforePara(int firstNormalCharCP)
    pub fn skipSpaceBeforePara(&mut self, firstNormalCharCP: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java SrcNode srcNode = this.getFirstCoreSrc();
      let mut srcNode: SrcNode = self.getFirstCoreSrc();
      // java if (srcNode != null && this.getGraphList().isEmpty() && firstNormalCharCP > srcNode.cp)
      if true && self.getGraphList().is_empty() && firstNormalCharCP > srcNode.cp {
        // java if (this.checkPropertyHasUnderLine(firstNormalCharCP - 1))
        if self.checkPropertyHasUnderLine(firstNormalCharCP - 1) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java srcNode.length -= firstNormalCharCP - srcNode.cp;
          (srcNode.len() as i32) = (srcNode.len() as i32) - firstNormalCharCP - srcNode.cp;
          // java srcNode.cp = firstNormalCharCP;
          srcNode.cp = firstNormalCharCP;
          // java return true;
          return true;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java private boolean checkPropertyHasUnderLine(int cp)
    pub fn checkPropertyHasUnderLine(&self, cp: i32) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IChpxTable.Entry entry = this.document.getChpxTable().seek(cp);
      let mut entry: Box<dyn Entry> = self.document.getChpxTable().seek(cp);
      // java if (entry != null && !entry.isHead())
      if true && !entry.isHead() {
        // java PropertySet propSet = entry.getProperty();
        let mut propSet: PropertySet = entry.getProperty();
        // java if (propSet != null)
        if true {
          // java Integer underline = (Integer)propSet.getValue(8);
          let mut underline: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
          // java return null != underline && underline != 0;
          return true && underline != 0;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean isIsolatedEmbed(Graphs run)
    pub fn isIsolatedEmbed(&mut self, run: &Graphs) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (run.runType == 6)
      if run.runType == 6 {
        // java if (this.typoPara.getChar(run.graphIndex) == '\b')
        if self.typoPara.getChar__int(run.graphIndex) == '\u{0008}' {
          // java return true;
          return true;
        }
        // java if (this.graphList.firstElement() == run)
        if self.graphList.firstElement() == run {
          // java Graphs next = (Graphs)this.graphList.getNext(run);
          let mut next: Graphs = (self.graphList.getNext(run)).downcast_ref::<Graphs>().unwrap();
          // java if (next != null && TypoRun.isLineBreak(next.runType))
          if true && TypoRun::isLineBreak(next.runType) {
            // java return true;
            return true;
          }
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean checkIsRtlToKeepSpace(PhoneViewParam param)
    pub fn checkIsRtlToKeepSpace(&self, param: &PhoneViewParam) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return param.isKeepSpaceForRTL() && (this.hasBidiGraph() || this.paraEnv.isRTL);
      return param.isKeepSpaceForRTL() && self.hasBidiGraph() || self.paraEnv.isRTL;
      */
    }

    // java public boolean isAllowTransForSpace()
    pub fn isAllowTransForSpace(&self) -> bool {
      // java return this.mIsAllowTransForSpace;
      self.mIsAllowTransForSpace
    }

    // java public void setAllowTransForSpace()
    pub fn setAllowTransForSpace(&mut self) {
      // java this.mIsAllowTransForSpace = true;
      self.mIsAllowTransForSpace = true;
    }

    // java public boolean checkIsColon(Graphs run)
    pub fn checkIsColon(&self, run: Option<&Graphs>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (run != null && TypoRun.isPunct(run.runType))
      if let Some(run) = run {
        if TypoRun::isPunct(run.runType) {
          // java char c = this.typoPara.getChar(run.graphIndex);
          let mut c: char = self.typoPara.getChar__int(run.graphIndex);
          // java if (c == '：' || c == ':')
          if c == '：' || c == ':' {
            // java return true;
            return true;
          }
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean needLineBreak(int graphIndex)
    pub fn needLineBreak(&mut self, graphIndex: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public boolean isUiCardStart(int graphIndex)
    pub fn isUiCardStart(&self, graphIndex: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public boolean isTitleEnd(int graphIndex)
    pub fn isTitleEnd(&self, graphIndex: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public boolean isUiCardEnd(int graphIndex, int length)
    pub fn isUiCardEnd(&self, graphIndex: i32, length: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public boolean isInUiCard()
    pub fn isInUiCard(&self) -> bool {
      // java return false;
      return false;
    }

    // java public boolean isUiCardStartCP(int startCP)
    pub fn isUiCardStartCP(&self, _startCP: i32) -> bool {
      // java return false;
      false
    }

    // java public boolean isUiCardEndCP(int endCP)
    pub fn isUiCardEndCP(&self, _endCP: i32) -> bool {
      // java return false;
      false
    }

    // java public void invalidCardState()
    pub fn invalidCardState(&mut self) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

}

impl std::ops::Deref for LayoutState {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for LayoutState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public abstract static class Alloc extends KList.RecycableNodeAlloc<LayoutState>
impl Alloc {
    // java public  Alloc(Pools pools)
    pub fn Alloc(pools: &Pools) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools = pools;
      Self { super_: RecycableNodeAlloc<LayoutState>::new(), mPools: pools }
      */
    }

    // java public void init(LayoutState env)
    pub fn init(&mut self, env: &mut LayoutState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.init(env);
      self.super_.init(env);
      // java env.pageEnv.init();
      env.pageEnv.init();
      // java env.paraEnv.init(this.mPools);
      env.paraEnv.init(self.mPools);
      // java env.cp = 0;
      env.cp = 0;
      // java this.mPools.recycleTableCellLayoutStates(env.cellStateList);
      self.mPools.recycleTableCellLayoutStates(env.cellStateList);
      // java env.curPage = null;
      env.curPage = None;
      // java env.typoPara.clear();
      env.typoPara.clear();
      // java env.curLayoutHeight = 0;
      env.curLayoutHeight = 0;
      // java env.affectRect.setEmpty();
      env.affectRect.setEmpty();
      // java env.finishPage = false;
      env.finishPage = false;
      // java env.validLine = false;
      env.validLine = false;
      // java env.atLeastOneLine = true;
      env.atLeastOneLine = true;
      // java env.document = null;
      env.document = None;
      // java env.hasFlowTableHeader = false;
      env.hasFlowTableHeader = false;
      // java env.maxWrapDrawingCP = Integer.MIN_VALUE;
      env.maxWrapDrawingCP = i32::MIN;
      // java env.pageEndLimitCP = Integer.MIN_VALUE;
      env.pageEndLimitCP = i32::MIN;
      // java env.needCalcSectionBreakWidth = false;
      env.needCalcSectionBreakWidth = false;
      // java env.linePos = 0;
      env.linePos = 0;
      // java if (null != env.mGraphsForEmbed)
      if let Some(env_mGraphsForEmbed) = env.mGraphsForEmbed.as_mut() {
        // java env.mGraphsForEmbed.clear();
        env_mGraphsForEmbed.clear();
      }
      // java if (null != env.mEmbedIndex)
      if let Some(env_mEmbedIndex) = env.mEmbedIndex.as_mut() {
        // java env.mEmbedIndex.clear();
        env_mEmbedIndex.clear();
      }
      // java env.srcList.clean();
      env.srcList.clean();
      // java env.charsList.clean();
      env.charsList.clean();
      // java env.graphList.clean();
      env.graphList.clean();
      // java if (env.drawingList != null)
      if let Some(env_drawingList) = env.drawingList.as_mut() {
        // java env.drawingList.clean();
        env_drawingList.clean();
      }
      */
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
