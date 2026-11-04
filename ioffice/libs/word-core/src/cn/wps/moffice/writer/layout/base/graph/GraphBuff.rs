#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unused_assignments, unreachable_code)]
// Java: import cn.wps.moffice.drawing.Shape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.PLC
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLC::PLC;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphArrayDeque
use crate::cn::wps::moffice::writer::layout::base::graph::GraphArrayDeque::GraphArrayDeque;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import java.lang.Object
// CARGO_OFF_SURFACE use stubs::java::lang::Object::Object;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;

// java private static final String TAG = null;
const TAG: i32 = 0;
// java private static final int MIN_MAP_ITEM = 4;
const MIN_MAP_ITEM: i32 = 4;
// java private static final int MAPITEM_OFFSET_CP_START = 0;
const MAPITEM_OFFSET_CP_START: i32 = 0;
// java private static final int MAPITEM_OFFSET_CP_END = 1;
const MAPITEM_OFFSET_CP_END: i32 = 1;
// java private static final int MAPITEM_OFFSET_GRAPH_INDEX_START = 2;
const MAPITEM_OFFSET_GRAPH_INDEX_START: i32 = 2;
// java private static final int MAPITEM_OFFSET_GRAPH_INDEX_END = 3;
const MAPITEM_OFFSET_GRAPH_INDEX_END: i32 = 3;
// java private static final int MAPITEM_OFFSET_MAX = 4;
const MAPITEM_OFFSET_MAX: i32 = 4;

// rust: ★wr-29 04 —— TypoFontStylePool 去影子：用真实 cache::style::TypoFontStyle::TypoFontStylePool
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStylePool;

// Java PLC.Node carried by addMath: MathStartNode (optional picture) or OMathStartNode.
#[derive(Clone, Debug, PartialEq)]
pub enum MathEmbed {
    Function { math_type: i32, picture_shape_id: Option<i32>, start_cp: i32, end_cp: i32 },
    OMath { start_cp: i32, end_cp: i32, in_para: bool },
    Other,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EmbedArg {
    Shape(i32),
    Math(MathEmbed),
    Eq(EqArg),
}

// java public abstract class GraphBuff
pub struct GraphBuff {
    // java MainState mMainState;
    pub(crate) mMainState: Option<MainState>,
    // java DocumentEnv mDocEnv;
    pub(crate) mDocEnv: Option<DocumentEnv>,
    // java ViewEnv mViewEnv;
    pub(crate) mViewEnv: Option<ViewEnv>,
    // Java shares one MainState/DocumentEnv/ViewEnv. A by-value copy deep-copies TypoSnapshot.heap.
    pub(crate) main_alias: *mut MainState,
    pub(crate) doc_alias: *mut DocumentEnv,
    pub(crate) view_alias: *mut ViewEnv,
    // java private GraphArrayDeque<Graphs> mGraphsForEmbed;
    // Java retains the same Graphs object; graph nodes live in stable Box slots.
    mGraphsForEmbed: Option<GraphArrayDeque<*mut Graphs>>,
    // java private GraphArrayDeque<Object> mArgsForEmbed;
    mArgsForEmbed: Option<GraphArrayDeque<EmbedArg>>,
    // java private int[] mMapData = new int[16];
    pub(crate) mMapData: Vec<i32>,
    // java private int mMapSize = 0;
    pub(crate) mMapSize: i32,
    // java private int mLastReadMapItem = 0;
    mLastReadMapItem: i32,
    // java private int mRecommendCPRef = 0;
    pub(crate) mRecommendCPRef: i32,
    // java public TypoFontStyle.TypoFontStylePool mTypoFontStylePool = new TypoFontStyle.TypoFontStylePool();
    pub(crate) mTypoFontStylePool: TypoFontStylePool,
}

impl Default for GraphBuff {
    fn default() -> Self {
        Self {
            mMainState: None,
            mDocEnv: None,
            mViewEnv: None,
            main_alias: std::ptr::null_mut(),
            doc_alias: std::ptr::null_mut(),
            view_alias: std::ptr::null_mut(),
            mGraphsForEmbed: None,
            mArgsForEmbed: None,
            mMapData: vec![0; 16],
            mMapSize: 0,
            mLastReadMapItem: 0,
            mRecommendCPRef: 0,
            mTypoFontStylePool: TypoFontStylePool::default(),
        }
    }
}

// java public static class EqArg
#[derive(Default, Clone, Debug, PartialEq)]
pub struct EqArg {
    // java public int start;
    pub start: i32,
    // java public int end;
    pub end: i32,
    // java public boolean enableRevision;
    pub enableRevision: bool,
    // java public boolean revisionFinal;
    pub revisionFinal: bool,
}

// java public abstract class GraphBuff
impl GraphBuff {
    // java public  GraphBuff(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn GraphBuff(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java this.mMainState = mainState;
      let phone = viewEnv.isPhoneReadMode();
      let mut this = Self {
        mMainState: Some(mainState),
        mDocEnv: Some(docEnv),
        mViewEnv: Some(viewEnv),
        main_alias: std::ptr::null_mut(),
        doc_alias: std::ptr::null_mut(),
        view_alias: std::ptr::null_mut(),
        mGraphsForEmbed: None,
        mArgsForEmbed: None,
        mMapData: vec![0; 16],
        mMapSize: 0,
        mLastReadMapItem: 0,
        mRecommendCPRef: 0,
        mTypoFontStylePool: TypoFontStylePool::default(),
      };
      // java this.mDocEnv = docEnv;
      // java this.mViewEnv = viewEnv;
      // java if (viewEnv.isPhoneReadMode())
      if phone {
        // java FontStyleFactory.setMaxFontSize(520);
        FontStyleFactory::setMaxFontSize(520);
      }
      // java else
      else {
        // java FontStyleFactory.setMaxFontSize(Integer.MAX_VALUE);
        FontStyleFactory::setMaxFontSize(i32::MAX);
        // java FontStyleFactory.setSmartFontSize(false);
        FontStyleFactory::setSmartFontSize(false);
      }
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mMainState = null;
      self.mMainState = None;
      // java this.mDocEnv = null;
      self.mDocEnv = None;
      // java this.mViewEnv = null;
      self.mViewEnv = None;
    }

    // java public abstract void setMaxGraphSize(int var1)
    pub fn setMaxGraphSize(&mut self, var1: i32) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public void clear(LayoutState layoutState)
    pub fn clear(&mut self, layoutState: &mut LayoutState) {
      // java this.mMapSize = 0;
      self.mMapSize = 0;
      // java this.mLastReadMapItem = 0;
      self.mLastReadMapItem = 0;
      // java layoutState.srcList.clean();
      layoutState.srcList.clean();
      // java layoutState.styleResult.reset();
      layoutState.styleResult.reset();
      // java this.doClear(true, layoutState);
      self.doClear(true, layoutState);
      // java layoutState.hasBidiGraph = false;
      layoutState.hasBidiGraph = false;
      // java layoutState.hasCoreCharsContent = false;
      layoutState.hasCoreCharsContent = false;
      // java layoutState.hasReadEnd = false;
      layoutState.hasReadEnd = false;
      // java layoutState.hasMaths = false;
      layoutState.hasMaths = false;
      // java layoutState.hasEquation = false;
      layoutState.hasEquation = false;
      // java layoutState.typoPara.clear();
      layoutState.typoPara.clear();
      // java layoutState.hasEmbeds = false;
      layoutState.hasEmbeds = false;
      // java layoutState.revisions.clean();
      layoutState.revisions.clean();
      // java layoutState.tagForSpaceTrans = false;
      layoutState.tagForSpaceTrans = false;
    }

    // java public void setRecommendCPRef(int cpRef)
    pub fn setRecommendCPRef(&mut self, cpRef: i32) {
      // java this.mRecommendCPRef = cpRef;
      self.mRecommendCPRef = cpRef;
    }

    // java public int getRecommendCPRef()
    pub fn getRecommendCPRef(&self) -> i32 {
      // java return this.mRecommendCPRef;
      return self.mRecommendCPRef;
    }

    // java private void doClear(boolean needClearDrawings, LayoutState layoutState)
    pub fn doClear(&mut self, needClearDrawings: bool, layoutState: &mut LayoutState) {
      // java if (needClearDrawings && null != layoutState.drawingList)
      if needClearDrawings && layoutState.drawingList.is_some() {
        // java layoutState.drawingList.clean();
        if let Some(drawingList) = layoutState.drawingList.as_mut() {
          drawingList.clean();
        }
      }
      // java if (null != this.mGraphsForEmbed)
      if self.mGraphsForEmbed.is_some() {
        // java this.mGraphsForEmbed.clear();
        if let Some(mGraphsForEmbed) = self.mGraphsForEmbed.as_mut() {
          mGraphsForEmbed.clear();
        }
        // java this.mArgsForEmbed.clear();
        if let Some(mArgsForEmbed) = self.mArgsForEmbed.as_mut() {
          mArgsForEmbed.clear();
        }
      }
      // java if (null != layoutState.mEmbedIndex)
      if layoutState.mEmbedIndex.is_some() {
        // java layoutState.mEmbedIndex.resetQuick();
        if let Some(mEmbedIndex) = layoutState.mEmbedIndex.as_mut() {
          mEmbedIndex.resetQuick();
        }
        // java layoutState.mGraphsForEmbed.clear();
        if let Some(mGraphsForEmbed) = layoutState.mGraphsForEmbed.as_mut() {
          mGraphsForEmbed.clear();
        }
      }
      // java layoutState.charsList.clean();
      layoutState.charsList.clean();
      // java layoutState.graphList.clean();
      layoutState.graphList.clean();
      // java layoutState.clearPredictSpaces();
      layoutState.clearPredictSpaces();
      // java this.doCleanGraphReader();
      self.doCleanGraphReader();
      // java layoutState.hasMaths = false;
      layoutState.hasMaths = false;
      // java layoutState.hasEmbeds = false;
      layoutState.hasEmbeds = false;
      // java layoutState.hasEquation = false;
      layoutState.hasEquation = false;
      // java this.mMapSize = 0;
      self.mMapSize = 0;
      // java layoutState.resetDrawPosOffset();
      layoutState.resetDrawPosOffset();
      // java layoutState.revisions.clean();
      layoutState.revisions.clean();
      // java layoutState.clearWitchCache();
      layoutState.clearWitchCache();
    }

    // java protected abstract void doCleanGraphReader()
    pub fn doCleanGraphReader(&mut self) {
      // rust: Java abstract, no body
    }

    // Java GraphBuff holds the caller's MainState. Rust stores a pointer so the snapshot heap is not cloned.
    pub fn bind_env_alias(&mut self, main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) {
      self.main_alias = main;
      self.doc_alias = doc;
      self.view_alias = view;
    }

    pub(crate) fn aliased_main(&mut self) -> Option<&mut MainState> {
      if !self.main_alias.is_null() {
        Some(unsafe { &mut *self.main_alias })
      } else {
        self.mMainState.as_mut()
      }
    }

    pub(crate) fn aliased_main_ref(&self) -> Option<&MainState> {
      if !self.main_alias.is_null() {
        Some(unsafe { &*self.main_alias })
      } else {
        self.mMainState.as_ref()
      }
    }

    pub(crate) fn aliased_doc(&mut self) -> Option<&mut DocumentEnv> {
      if !self.doc_alias.is_null() {
        Some(unsafe { &mut *self.doc_alias })
      } else {
        self.mDocEnv.as_mut()
      }
    }

    pub(crate) fn aliased_view(&self) -> Option<&ViewEnv> {
      if !self.view_alias.is_null() {
        Some(unsafe { &*self.view_alias })
      } else {
        self.mViewEnv.as_ref()
      }
    }

    // java public boolean prepareGraphs(int wordAlignFont, LayoutState layoutState)
    pub fn prepareGraphs(&mut self, wordAlignFont: i32, layoutState: &mut LayoutState) -> bool {
      // java if (!layoutState.hasContents())
      if !layoutState.hasContents() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (!layoutState.graphList.isEmpty() && !layoutState.charsList.isEmpty())
        if !layoutState.graphList.isEmpty() && !layoutState.charsList.isEmpty() {
          // java if (layoutState.hasReadEnd || layoutState.paraIsOverflow)
          if layoutState.hasReadEnd || layoutState.paraIsOverflow {
            // java layoutState.resetGraphsIt();
            layoutState.resetGraphsIt();
            // java return false;
            return false;
          }
        }
        // java else
        else {
          // java this.doClear(false, layoutState);
          self.doClear(false, layoutState);
        }
        // java if (layoutState.graphList.isEmpty())
        if layoutState.graphList.isEmpty() {
          // java layoutState.mFirstGraphIndex = 0;
          layoutState.mFirstGraphIndex = 0;
        }
        // java layoutState.hasReadEnd = false;
        layoutState.hasReadEnd = false;
        // java layoutState.isInMathPara = false;
        layoutState.isInMathPara = false;
        // java layoutState.paraIsOverflow = false;
        layoutState.paraIsOverflow = false;
        // java layoutState.resetMathPara();
        layoutState.resetMathPara();
        // java layoutState.typoPara.alloc(this.mMainState.snapshot);
        if let Some(mMainState) = self.aliased_main() {
          layoutState.typoPara.alloc(&mut mMainState.snapshot);
        }
        // java this.mTypoFontStylePool.reset();
        self.mTypoFontStylePool.reset();
        // java layoutState.revisions.clean();
        layoutState.revisions.clean();
        // java this.doPrepareGraphs(wordAlignFont, layoutState);
        self.doPrepareGraphs(wordAlignFont, layoutState);
        // java layoutState.resetGraphsIt();
        layoutState.resetGraphsIt();
        // java layoutState.typoPara.commit(this.mMapData, this.mMapSize, layoutState.document, this.mRecommendCPRef);
        layoutState.typoPara.commit(&mut self.mMapData, self.mMapSize, layoutState.document, self.mRecommendCPRef);
        // java this.mMapSize = 0;
        self.mMapSize = 0;
        // java return layoutState.hasEmbeds || layoutState.hasEquation || layoutState.hasMaths;
        return layoutState.hasEmbeds || layoutState.hasEquation || layoutState.hasMaths;
      }
    }

    // java protected abstract void doPrepareGraphs(int var1, LayoutState var2)
    pub fn doPrepareGraphs(&mut self, var1: i32, var2: &mut LayoutState) {
      // rust: Java abstract, no body (GraphReader 03)
      let _ = (var1, var2);
    }

    // java public abstract void setCalcGraphHeight(boolean var1)
    pub fn setCalcGraphHeight(&mut self, var1: bool) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public abstract void setIsInMath(boolean var1, LayoutState var2)
    pub fn setIsInMath(&mut self, var1: bool, var2: &mut LayoutState) {
      // rust: Java abstract, no body
      let _ = (var1, var2);
    }

    // java protected void addBidi(int graphIndex, char ch, LayoutState layoutState)
    pub fn addBidi(&mut self, graphIndex: i32, ch: char, layoutState: &mut LayoutState) {
      // java layoutState.addBidi(graphIndex, ch, this.mDocEnv);
      if let Some(mDocEnv) = self.mDocEnv.as_ref() {
        layoutState.addBidi(graphIndex, ch, mDocEnv);
      }
    }

    // java public abstract void setIgnoreHpsAndVAlignRun(boolean var1)
    pub fn setIgnoreHpsAndVAlignRun(&mut self, var1: bool) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public abstract void setIgnoreCapsAndSmallCaps(boolean var1)
    pub fn setIgnoreCapsAndSmallCaps(&mut self, var1: bool) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public void resetForInit()
    pub fn resetForInit(&self) {
    }

    // java private void ensureGraphsForEmbed()
    pub fn ensureGraphsForEmbed(&mut self) {
      // java if (null == this.mGraphsForEmbed)
      if self.mGraphsForEmbed.is_none() {
        // java this.mGraphsForEmbed = new GraphArrayDeque<Graphs>();
        self.mGraphsForEmbed = Some(GraphArrayDeque::new());
      }
      // java if (null == this.mArgsForEmbed)
      if self.mArgsForEmbed.is_none() {
        // java this.mArgsForEmbed = new GraphArrayDeque<Object>();
        self.mArgsForEmbed = Some(GraphArrayDeque::new());
      }
    }

    // java  void addEmbed(Graphs graphs, Shape shape)
    pub fn addEmbed(&mut self, graphs: *mut Graphs, shape_id: i32) {
      // java this.ensureGraphsForEmbed();
      self.ensureGraphsForEmbed();
      // java this.mGraphsForEmbed.add(graphs);
      if let Some(dq) = self.mGraphsForEmbed.as_mut() {
        dq.add(graphs);
      }
      // java this.mArgsForEmbed.add(shape);
      if let Some(dq) = self.mArgsForEmbed.as_mut() {
        dq.add(EmbedArg::Shape(shape_id));
      }
    }

    // java  void addMath(Graphs graphs, PLC.Node mathNode)
    pub fn addMath(&mut self, graphs: *mut Graphs, mathNode: MathEmbed) {
      // java this.ensureGraphsForEmbed();
      self.ensureGraphsForEmbed();
      // java this.mGraphsForEmbed.add(graphs);
      if let Some(dq) = self.mGraphsForEmbed.as_mut() {
        dq.add(graphs);
      }
      // java this.mArgsForEmbed.add(mathNode);
      if let Some(dq) = self.mArgsForEmbed.as_mut() {
        dq.add(EmbedArg::Math(mathNode));
      }
    }

    // java  void addEqation(Graphs graphs, int start, int end, boolean enableRevision, boolean revisionFinal)
    pub fn addEqation(&mut self, graphs: *mut Graphs, start: i32, end: i32, enableRevision: bool, revisionFinal: bool) {
      // java this.ensureGraphsForEmbed();
      self.ensureGraphsForEmbed();
      // java this.mGraphsForEmbed.add(graphs);
      if let Some(dq) = self.mGraphsForEmbed.as_mut() {
        dq.add(graphs);
      }
      // java this.mArgsForEmbed.add(new EqArg(start, end, enableRevision, revisionFinal));
      if let Some(dq) = self.mArgsForEmbed.as_mut() {
        dq.add(EmbedArg::Eq(EqArg::EqArg(start, end, enableRevision, revisionFinal)));
      }
    }

    // java public int getArgsForEmbedSize()
    pub fn getArgsForEmbedSize(&self) -> i32 {
      // java return null == this.mArgsForEmbed ? 0 : this.mArgsForEmbed.size();
      return if self.mArgsForEmbed.is_none() { 0 } else { self.mArgsForEmbed.as_ref().map(|a| a.size()).unwrap_or(0) };
    }

    // java public int getEmbedSize()
    pub fn getEmbedSize(&self) -> i32 {
      // java return null == this.mGraphsForEmbed ? 0 : this.mGraphsForEmbed.size();
      return if self.mGraphsForEmbed.is_none() { 0 } else { self.mGraphsForEmbed.as_ref().map(|a| a.size()).unwrap_or(0) };
    }

    // java public Graphs getGraphsForEmbed(int index)
    pub fn getGraphsForEmbed(&self, index: i32) -> Option<*mut Graphs> {
      // java return null == this.mGraphsForEmbed ? null : (Graphs)this.mGraphsForEmbed.get(index);
      self.mGraphsForEmbed.as_ref().and_then(|queue| queue.get(index).copied())
    }

    // java public Object getArgsForEmbed(int index)
    pub fn getArgsForEmbed(&self, index: i32) -> Option<&EmbedArg> {
      // java return null == this.mArgsForEmbed ? null : this.mArgsForEmbed.get(index);
      return if self.mArgsForEmbed.is_none() { None } else { self.mArgsForEmbed.as_ref().and_then(|a| a.get(index)) };
    }

    // Java DrawingLayouter.getEmbed: parent page, core width, indents, text flow, then preflow.
    pub fn shape_embed_pointer(&mut self, shape_id: i32, layoutState: &mut LayoutState) -> i32 {
      let (snap, page_index) = match self.aliased_main() {
        Some(state) => (&mut state.snapshot as *mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot, state.curPageIndex),
        None => return 0,
      };
      layoutState.typoPara.snapshot = snap;
      let shape = layoutState.document.and_then(|doc| shape_arc_at(unsafe { &mut *doc }, shape_id));
      let core_width = layoutState.pageEnv.getCoreWidth();
      let (left_indent, right_indent) = layoutState.paraEnv.curParaPropEnv.as_ref().map(|env| (env.leftIndent, env.rightIndent)).unwrap_or((0, 0));
      let textflow = layoutState.pageEnv.textFlow;
      let parent_pointer = layoutState.curPage.as_mut().map(|page| {
        let snapshot = unsafe { &mut *snap };
        page.ensure_snapshot_pointer(snapshot)
      }).unwrap_or(0);
      match shape.as_ref() {
        Some(shape) => {
          let snapshot = unsafe { &mut *snap };
          crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::DrawingLayouter::preflow_embed_pointer(
            snapshot, shape, parent_pointer, page_index, core_width, left_indent, right_indent, textflow,
          )
        }
        None => 0,
      }
    }

    // Java TextLayouter.prepareEmbeds Shape branch: DrawingLayouter.getEmbed then LayoutState.setDrawing.
    pub fn prepareShapeEmbeds(&mut self, layoutState: &mut LayoutState) {
      let count = self.getArgsForEmbedSize();
      let mut index = 0;
      while index < count {
        let shape_id = match self.getArgsForEmbed(index) {
          Some(EmbedArg::Shape(id)) => *id,
          _ => { index += 1; continue; }
        };
        let graphs = self.getGraphsForEmbed(index).unwrap_or(std::ptr::null_mut());
        if graphs.is_null() {
          index += 1;
          continue;
        }
        let pointer = self.shape_embed_pointer(shape_id, layoutState);
        // java if (0 != drawing) setDrawing; else remove the embed graph.
        if pointer != 0 && !graphs.is_null() {
          let word_align = layoutState.paraEnv.wordAlignFont;
          layoutState.setDrawing(unsafe { &mut *graphs }, pointer, word_align, false);
          index += 1;
        } else {
          self.removeEmbedArgsAt(index);
          if !graphs.is_null() {
            let next = layoutState.getGraphList().getNext(graphs);
            if layoutState.getGraphList().firstElement() == graphs {
              layoutState.push_ptr(next);
            }
            layoutState.getGraphList().remove(graphs, graphs);
          }
        }
      }
    }

    // java public void removeEmbedArgsAt(int curEmbedIndex)
    pub fn removeEmbedArgsAt(&mut self, curEmbedIndex: i32) {
      // java this.mGraphsForEmbed.removeAt(curEmbedIndex);
      if let Some(dq) = self.mGraphsForEmbed.as_mut() {
        dq.removeAt(curEmbedIndex);
      }
      // java this.mArgsForEmbed.removeAt(curEmbedIndex);
      if let Some(dq) = self.mArgsForEmbed.as_mut() {
        dq.removeAt(curEmbedIndex);
      }
    }

    // java final void addMap(int cpStart, int cpLen, int graphIndex, int graphLen)
    pub fn addMap(&mut self, cpStart: i32, cpLen: i32, graphIndex: i32, graphLen: i32) {
      // java int startOffset = (this.mMapSize - 1) * 4;
      let mut startOffset: i32 = (self.mMapSize - 1) * 4;
      // java int[] mapData = this.mMapData;
      let mapData = &self.mMapData;
      let _ = mapData;
      // java if (this.mMapSize == 0)
      if self.mMapSize == 0 {
        // java mapData[0] = cpStart;
        self.mMapData[0] = cpStart;
        // java mapData[1] = cpStart + cpLen;
        self.mMapData[1] = cpStart + cpLen;
        // java mapData[2] = graphIndex;
        self.mMapData[2] = graphIndex;
        // java mapData[3] = graphIndex + graphLen;
        self.mMapData[3] = graphIndex + graphLen;
        // java ++this.mMapSize;
        self.mMapSize += 1;
      }
      // java else if (cpLen == graphLen && mapData[startOffset + 1] - mapData[startOffset + 0] == mapData[startOffset + 3] - mapData[startOffset + 2] && cpStart == mapData[startOffset + 1] && graphIndex == mapData[startOffset + 3])
      else if cpLen == graphLen
        && self.mMapData[(startOffset + 1) as usize] - self.mMapData[(startOffset + 0) as usize]
          == self.mMapData[(startOffset + 3) as usize] - self.mMapData[(startOffset + 2) as usize]
        && cpStart == self.mMapData[(startOffset + 1) as usize]
        && graphIndex == self.mMapData[(startOffset + 3) as usize]
      {
        // java mapData[startOffset + 1] = cpStart + cpLen;
        self.mMapData[(startOffset + 1) as usize] = cpStart + cpLen;
        // java mapData[startOffset + 3] = graphIndex + graphLen;
        self.mMapData[(startOffset + 3) as usize] = graphIndex + graphLen;
      }
      // java else
      else {
        // java startOffset = this.mMapSize * 4;
        startOffset = self.mMapSize * 4;
        // java if (startOffset >= this.mMapData.length)
        if startOffset >= (self.mMapData.len() as i32) {
          // java int[] newMapData = new int[this.mMapData.length * 2];
          let mut newMapData: Vec<i32> = vec![0; self.mMapData.len() * 2];
          // java System.arraycopy(this.mMapData, 0, newMapData, 0, this.mMapData.length);
          newMapData[..self.mMapData.len()].copy_from_slice(&self.mMapData);
          // java this.mMapData = newMapData;
          self.mMapData = newMapData;
          // java mapData = newMapData;
          let mapData = &self.mMapData;
          let _ = mapData;
        }
        // java mapData[startOffset + 0] = cpStart;
        self.mMapData[(startOffset + 0) as usize] = cpStart;
        // java mapData[startOffset + 1] = cpStart + cpLen;
        self.mMapData[(startOffset + 1) as usize] = cpStart + cpLen;
        // java mapData[startOffset + 2] = graphIndex;
        self.mMapData[(startOffset + 2) as usize] = graphIndex;
        // java mapData[startOffset + 3] = graphIndex + graphLen;
        self.mMapData[(startOffset + 3) as usize] = graphIndex + graphLen;
        // java ++this.mMapSize;
        self.mMapSize += 1;
      }
    }

    // java final int getGraphIndex(int cp)
    pub fn getGraphIndex(&mut self, cp: i32) -> i32 {
      // java int curItemIndex = this.mLastReadMapItem;
      let mut curItemIndex: i32 = self.mLastReadMapItem;
      // java int[] mapData = this.mMapData;
      let mapData = &self.mMapData;
      let _ = mapData;
      // java if (curItemIndex >= this.mMapSize)
      if curItemIndex >= self.mMapSize {
        // java curItemIndex = 0;
        curItemIndex = 0;
      }
      // java int itemStart = curItemIndex * 4;
      let mut itemStart: i32 = curItemIndex * 4;
      // java int minIndex = 0;
      let mut minIndex: i32 = 0;
      // java int maxIndex = this.mMapSize;
      let mut maxIndex: i32 = self.mMapSize;
      // java if (cp >= mapData[itemStart + 0])
      if cp >= self.mMapData[(itemStart + 0) as usize] {
        // java if (cp < mapData[itemStart + 1])
        if cp < self.mMapData[(itemStart + 1) as usize] {
          // java this.mLastReadMapItem = curItemIndex;
          self.mLastReadMapItem = curItemIndex;
          // java return mapData[itemStart + 2] + cp - mapData[itemStart + 0];
          return self.mMapData[(itemStart + 2) as usize] + cp - self.mMapData[(itemStart + 0) as usize];
        }
        // java ++curItemIndex;
        curItemIndex += 1;
        // java if (curItemIndex >= this.mMapSize)
        if curItemIndex >= self.mMapSize {
          // java return -1;
          return -1;
        }
        // java itemStart += 4;
        itemStart += 4;
        // java if (cp >= mapData[itemStart + 0] && cp < mapData[itemStart + 1])
        if cp >= self.mMapData[(itemStart + 0) as usize] && cp < self.mMapData[(itemStart + 1) as usize] {
          // java this.mLastReadMapItem = curItemIndex;
          self.mLastReadMapItem = curItemIndex;
          // java return mapData[itemStart + 2] + cp - mapData[itemStart + 0];
          return self.mMapData[(itemStart + 2) as usize] + cp - self.mMapData[(itemStart + 0) as usize];
        }
        // java minIndex = curItemIndex + 1;
        minIndex = curItemIndex + 1;
      }
      // java else
      else {
        // java maxIndex = curItemIndex;
        maxIndex = curItemIndex;
      }
      // java while(minIndex < maxIndex)
      while minIndex < maxIndex {
        // java curItemIndex = minIndex + maxIndex >> 1;
        curItemIndex = (minIndex + maxIndex) >> 1;
        // java itemStart = curItemIndex * 4;
        itemStart = curItemIndex * 4;
        // java if (cp >= mapData[itemStart + 0])
        if cp >= self.mMapData[(itemStart + 0) as usize] {
          // java if (cp < mapData[itemStart + 1])
          if cp < self.mMapData[(itemStart + 1) as usize] {
            // java this.mLastReadMapItem = curItemIndex;
            self.mLastReadMapItem = curItemIndex;
            // java return mapData[itemStart + 2] + cp - mapData[itemStart + 0];
            return self.mMapData[(itemStart + 2) as usize] + cp - self.mMapData[(itemStart + 0) as usize];
          }
          // java minIndex = curItemIndex + 1;
          minIndex = curItemIndex + 1;
        }
        // java else
        else {
          // java maxIndex = curItemIndex;
          maxIndex = curItemIndex;
        }
      }
      // java return -1;
      return -1;
    }

    // java public final int getCP(int graphIndex)
    pub fn getCP(&mut self, graphIndex: i32) -> i32 {
      // java if (this.mMapSize <= 0)
      if self.mMapSize <= 0 {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java int curItemIndex = this.mLastReadMapItem;
        let mut curItemIndex: i32 = self.mLastReadMapItem;
        // java int[] mapData = this.mMapData;
        let mapData = &self.mMapData;
        let _ = mapData;
        // java if (curItemIndex >= this.mMapSize)
        if curItemIndex >= self.mMapSize {
          // java curItemIndex = 0;
          curItemIndex = 0;
        }
        // java int itemStart = curItemIndex * 4;
        let mut itemStart: i32 = curItemIndex * 4;
        // java int minIndex = 0;
        let mut minIndex: i32 = 0;
        // java int maxIndex = this.mMapSize;
        let mut maxIndex: i32 = self.mMapSize;
        // java if (graphIndex >= mapData[itemStart + 2])
        if graphIndex >= self.mMapData[(itemStart + 2) as usize] {
          // java if (graphIndex < mapData[itemStart + 3])
          if graphIndex < self.mMapData[(itemStart + 3) as usize] {
            // java this.mLastReadMapItem = curItemIndex;
            self.mLastReadMapItem = curItemIndex;
            // java return mapData[itemStart + 0] + graphIndex - mapData[itemStart + 2];
            return self.mMapData[(itemStart + 0) as usize] + graphIndex - self.mMapData[(itemStart + 2) as usize];
          }
          // java ++curItemIndex;
          curItemIndex += 1;
          // java if (curItemIndex >= this.mMapSize)
          if curItemIndex >= self.mMapSize {
            // java return -1;
            return -1;
          }
          // java itemStart += 4;
          itemStart += 4;
          // java if (graphIndex >= mapData[itemStart + 2] && graphIndex < mapData[itemStart + 3])
          if graphIndex >= self.mMapData[(itemStart + 2) as usize] && graphIndex < self.mMapData[(itemStart + 3) as usize] {
            // java this.mLastReadMapItem = curItemIndex;
            self.mLastReadMapItem = curItemIndex;
            // java return mapData[itemStart + 0] + graphIndex - mapData[itemStart + 2];
            return self.mMapData[(itemStart + 0) as usize] + graphIndex - self.mMapData[(itemStart + 2) as usize];
          }
          // java minIndex = curItemIndex + 1;
          minIndex = curItemIndex + 1;
        }
        // java else
        else {
          // java maxIndex = curItemIndex;
          maxIndex = curItemIndex;
        }
        // java while(minIndex < maxIndex)
        while minIndex < maxIndex {
          // java curItemIndex = minIndex + maxIndex >> 1;
          curItemIndex = (minIndex + maxIndex) >> 1;
          // java itemStart = curItemIndex * 4;
          itemStart = curItemIndex * 4;
          // java if (graphIndex >= mapData[itemStart + 2])
          if graphIndex >= self.mMapData[(itemStart + 2) as usize] {
            // java if (graphIndex < mapData[itemStart + 3])
            if graphIndex < self.mMapData[(itemStart + 3) as usize] {
              // java this.mLastReadMapItem = curItemIndex;
              self.mLastReadMapItem = curItemIndex;
              // java return mapData[itemStart + 0] + graphIndex - mapData[itemStart + 2];
              return self.mMapData[(itemStart + 0) as usize] + graphIndex - self.mMapData[(itemStart + 2) as usize];
            }
            // java minIndex = curItemIndex + 1;
            minIndex = curItemIndex + 1;
          }
          // java else
          else {
            // java maxIndex = curItemIndex;
            maxIndex = curItemIndex;
          }
        }
        // java return -1;
        return -1;
      }
    }

    // java final void removeGraph(int graphIndex, LayoutState layoutState)
    pub fn removeGraph(&mut self, graphIndex: i32, layoutState: &mut LayoutState) {
      // java layoutState.typoPara.removeGraphs(graphIndex, 1);
      layoutState.typoPara.removeGraphs(graphIndex, 1);
      // java int cp = this.getCP(graphIndex);
      let mut cp: i32 = self.getCP(graphIndex);
      // java if (cp >= 0)
      if cp >= 0 {
        // java int[] mapData = this.mMapData;
        let mapData = &self.mMapData;
        let _ = mapData;
        // java int curMapItem = this.mLastReadMapItem;
        let mut curMapItem: i32 = self.mLastReadMapItem;
        // java int itemStart = curMapItem * 4;
        let mut itemStart: i32 = curMapItem * 4;
        // java if (graphIndex == mapData[itemStart + 2])
        if graphIndex == self.mMapData[(itemStart + 2) as usize] {
          // java --mapData[itemStart + 3];
          self.mMapData[(itemStart + 3) as usize] -= 1;
          // java if (mapData[itemStart + 2] == mapData[itemStart + 3])
          if self.mMapData[(itemStart + 2) as usize] == self.mMapData[(itemStart + 3) as usize] {
            // java --this.mMapSize;
            self.mMapSize -= 1;
            // java if (curMapItem < this.mMapSize)
            if curMapItem < self.mMapSize {
              // java System.arraycopy(mapData, itemStart + 4, mapData, itemStart, this.mMapSize * 4 - itemStart);
              let n = (self.mMapSize * 4 - itemStart) as usize;
              let src = (itemStart + 4) as usize;
              let dst = itemStart as usize;
              if n > 0 && src + n <= self.mMapData.len() {
                let tmp = self.mMapData[src..src + n].to_vec();
                self.mMapData[dst..dst + n].copy_from_slice(&tmp);
              }
            }
          }
          // java else
          else {
            // java ++mapData[itemStart + 0];
            self.mMapData[(itemStart + 0) as usize] += 1;
            // java ++curMapItem;
            curMapItem += 1;
          }
        }
        // java else if (graphIndex == mapData[itemStart + 3] - 1)
        else if graphIndex == self.mMapData[(itemStart + 3) as usize] - 1 {
          // java --mapData[itemStart + 3];
          self.mMapData[(itemStart + 3) as usize] -= 1;
          // java --mapData[itemStart + 1];
          self.mMapData[(itemStart + 1) as usize] -= 1;
          // java ++curMapItem;
          curMapItem += 1;
        }
        // java else
        else {
          // java if (this.mMapSize * 4 >= mapData.length)
          if self.mMapSize * 4 >= (self.mMapData.len() as i32) {
            // java mapData = new int[mapData.length * 2];
            let mut mapData: Vec<i32> = vec![0; self.mMapData.len() * 2];
            // java System.arraycopy(this.mMapData, 0, mapData, 0, (curMapItem + 1) * 4);
            let n0 = ((curMapItem + 1) * 4) as usize;
            mapData[..n0].copy_from_slice(&self.mMapData[..n0]);
            // java System.arraycopy(this.mMapData, curMapItem * 4, mapData, (curMapItem + 1) * 4, (this.mMapSize - curMapItem) * 4);
            let src = (curMapItem * 4) as usize;
            let dst = ((curMapItem + 1) * 4) as usize;
            let n1 = ((self.mMapSize - curMapItem) * 4) as usize;
            mapData[dst..dst + n1].copy_from_slice(&self.mMapData[src..src + n1]);
            // java this.mMapData = mapData;
            self.mMapData = mapData;
          }
          // java else
          else {
            // java System.arraycopy(mapData, curMapItem * 4, mapData, (curMapItem + 1) * 4, (this.mMapSize - curMapItem) * 4);
            let src = (curMapItem * 4) as usize;
            let dst = ((curMapItem + 1) * 4) as usize;
            let n1 = ((self.mMapSize - curMapItem) * 4) as usize;
            let tmp = self.mMapData[src..src + n1].to_vec();
            self.mMapData[dst..dst + n1].copy_from_slice(&tmp);
          }
          // java mapData[itemStart + 1] = mapData[itemStart + 0] + graphIndex - mapData[itemStart + 2];
          self.mMapData[(itemStart + 1) as usize] = self.mMapData[(itemStart + 0) as usize] + graphIndex - self.mMapData[(itemStart + 2) as usize];
          // java mapData[itemStart + 3] = graphIndex;
          self.mMapData[(itemStart + 3) as usize] = graphIndex;
          // java ++curMapItem;
          curMapItem += 1;
          // java itemStart += 4;
          itemStart += 4;
          // java mapData[itemStart + 0] += graphIndex - mapData[itemStart + 2] + 1;
          self.mMapData[(itemStart + 0) as usize] += graphIndex - self.mMapData[(itemStart + 2) as usize] + 1;
          // java mapData[itemStart + 2] = graphIndex + 1;
          self.mMapData[(itemStart + 2) as usize] = graphIndex + 1;
          // java ++this.mMapSize;
          self.mMapSize += 1;
        }
        // java itemStart = curMapItem * 4;
        itemStart = curMapItem * 4;
        // java for(int max = this.mMapSize * 4; itemStart < max; itemStart += 4)
        let mut max: i32 = self.mMapSize * 4;
        while itemStart < max {
          // java --mapData[itemStart + 2];
          self.mMapData[(itemStart + 2) as usize] -= 1;
          // java --mapData[itemStart + 3];
          self.mMapData[(itemStart + 3) as usize] -= 1;
          itemStart += 4;
        }
      }
    }

    // java public abstract void addFunctionSepField(int var1)
    pub fn addFunctionSepField(&mut self, var1: i32) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public abstract void setIgnoreLeading(boolean var1)
    pub fn setIgnoreLeading(&mut self, var1: bool) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public abstract void setMathContent(boolean var1)
    pub fn setMathContent(&mut self, var1: bool) {
      // rust: Java abstract, no body
      let _ = var1;
    }

    // java public abstract void initFilter(Document var1, int var2, int var3, LayoutState var4)
    pub fn initFilter(&mut self, var1: Option<*mut dyn Document>, var2: i32, var3: i32, var4: &mut LayoutState) {
      // rust: Java abstract, no body (GraphReader 03)
      let _ = (var1, var2, var3, var4);
    }

    // java BalloonContentLayouter.prepareEmbeds: picture returns 1, ink returns 2.
    pub fn comment_shape_kind(&mut self, shape_id: i32, doc: *mut dyn Document) -> i32 {
      if doc.is_null() {
        return 0;
      }
      let Some(shape) = shape_arc_at(unsafe { &mut *doc }, shape_id) else { return 0; };
      let mut guard = shape.lock().unwrap_or_else(|err| err.into_inner());
      if guard.isPicture() {
        return 1;
      }
      if guard.getInk().is_some() {
        return 2;
      }
      0
    }

}

fn shape_arc_at(doc: &mut dyn Document, shape_id: i32) -> Option<std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>> {
  use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
  let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
  let element = IDrawingContainer::findShapeByID(sub.drawingContainer()?, shape_id)?;
  element.downcast::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>().ok()
}

// java public static class EqArg
impl EqArg {
    // java public  EqArg(int start, int end, boolean enableRevision, boolean revisionFinal)
    pub fn EqArg(start: i32, end: i32, enableRevision: bool, revisionFinal: bool) -> Self {
      // java this.start = start;
      Self { start: start, end: end, enableRevision: enableRevision, revisionFinal: revisionFinal }
      // java this.end = end;
      // java this.enableRevision = enableRevision;
      // java this.revisionFinal = revisionFinal;
    }

}
