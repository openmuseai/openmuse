#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unused_assignments, unreachable_code, non_camel_case_types)]
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::property::PropertySet::PropertyValue;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.base.ViewMode
use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.IFilter
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;
// Java: import cn.wps.moffice.writer.layout.base.graph.IRunPropFilter
use crate::cn::wps::moffice::writer::layout::base::graph::IRunPropFilter::IRunPropFilter;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphReaderTool
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReaderTool::GraphReaderTool;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuff
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::GraphBuff;
// Java: import cn.wps.moffice.writer.layout.base.graph.CharsNode
use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.FilterArgs
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.graph.SrcNode
use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.FunctionSepField
use crate::cn::wps::moffice::writer::layout::base::graph::FunctionSepField::FunctionSepField;
// Java: import cn.wps.moffice.writer.layout.base.graph.FieldFilter
use crate::cn::wps::moffice::writer::layout::base::graph::FieldFilter::FieldFilter;
use crate::cn::wps::moffice::writer::layout::base::graph::MathFilter::MathFilter;
use crate::cn::wps::moffice::writer::layout::base::graph::OMathFilter::OMathFilter;
use crate::cn::wps::moffice::writer::layout::base::graph::PhoneViewOMathFilter::PhoneViewOMathFilter;
use crate::cn::wps::moffice::writer::layout::base::graph::PhoneViewBalloonDrawingFilter::PhoneViewBalloonDrawingFilter;
use crate::cn::wps::moffice::writer::data::PLCMathStart::OMathStartNode;
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
use util::moffice::define::FontName::FontName;
use crate::cn::wps::moffice::writer::layout::base::graph::FootEndNoteFilter::FootEndNoteFilter;
use crate::cn::wps::moffice::writer::layout::base::graph::FootEndRefCharFilter::FootEndRefCharFilter;
use crate::cn::wps::moffice::writer::layout::ShareView::ShareViewRunPropFilter::ShareViewRunPropFilter;
use crate::cn::wps::moffice::writer::cache::RevisionCache;
use crate::cn::wps::moffice::writer::data::PLC::Node;
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::MathEmbed;
use crate::cn::wps::moffice::writer::data::PLC::nodes_same;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::{is_math_start, MathStartNode};
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;

// Java GraphFactory overrides these abstract GraphReader methods.
// Function pointers stay thin so a null hook does not need dyn metadata.
pub(crate) type CreateNodesHook = fn(*mut (), &mut CharsNode, i32, i32, &mut LayoutState);
pub(crate) type CommitLayoutHook = fn(*mut (), &mut LayoutState);
pub(crate) type SetWordAlignFontHook = fn(*mut (), i32, &mut LayoutState);
pub(crate) type CreateEquationNodesHook = fn(*mut (), &mut CharsNode, i32, i32, i32, bool, bool, &mut LayoutState);
pub(crate) type CreateMathNodesHook = fn(*mut (), &mut CharsNode, i32, MathEmbed, &mut LayoutState);
pub(crate) type SetCalcGraphHeightHook = fn(*mut (), bool);
pub(crate) type SetMathUnderlineHook = fn(*mut (), i32, i32);
pub(crate) type CreateEmbedNodesHook = fn(*mut (), &mut CharsNode, Option<i32>, i32, i32, bool, &mut LayoutState);

// java abstract class GraphReader extends GraphBuff
pub struct GraphReader {
    // java GraphBuff super_
    pub(crate) super_: GraphBuff,
    // java private int mCurCP;
    pub(crate) mCurCP: i32,
    // java private int mMaxCP;
    mMaxCP: i32,
    // java protected boolean mEnableRevision;
    mEnableRevision: bool,
    // java boolean mRevisionFinal;
    pub(crate) mRevisionFinal: bool,
    // java private SrcNode mSrcNode;
    pub(crate) mSrcNode: Option<*mut SrcNode>,
    // java IPropertyTool mPropertyTool;
    pub(crate) mPropertyTool: Option<Box<dyn IPropertyTool>>,
    // java boolean mIsInField;
    pub(crate) mIsInField: bool,
    // java private boolean mIsInTable;
    mIsInTable: bool,
    // java int mHappylinkId = -1;
    pub(crate) mHappylinkId: i32,
    // java private ArrayList<IFilter> mFilters = new ArrayList();
    mFilters: Vec<Box<dyn IFilter>>,
    // java private ArrayList<IRunPropFilter> mRunPropFilters = new ArrayList();
    pub(crate) mRunPropFilters: Vec<Box<dyn IRunPropFilter>>,
    // java private FilterArgs mFilterArgs = new FilterArgs();
    mFilterArgs: FilterArgs,
    // java private int mCurStart;
    mCurStart: i32,
    // java private int mCurEnd;
    mCurEnd: i32,
    // java boolean mIsInMath;
    pub(crate) mIsInMath: bool,
    // java protected ISymbolConvert mSymbolConvert;
    pub(crate) mSymbolConvert: Option<()>,
    // java private boolean needReadVanishBreak;
    needReadVanishBreak: bool,
    // java protected int[] mBlockResult = new int[2];
    pub(crate) mBlockResult: [i32; 2],
    // java protected boolean mForceDrawingToEmbed;
    mForceDrawingToEmbed: bool,
    // java private int mMaxGraphSize = Integer.MAX_VALUE;
    mMaxGraphSize: i32,
    // java private RevisionCache.Item mItem;
    // rust: 修订 cache 未接线（GR-D3），占位
    mItem: Option<()>,
    // Rust split-phase bridge for GraphFactory's Java virtual createNodes.
    mPendingMapCommit: bool,
    // Java virtual GraphFactory; bound after the factory value stops moving.
    pub(crate) factory_self: *mut (),
    pub(crate) factory_create_nodes: Option<CreateNodesHook>,
    pub(crate) factory_commit_all: Option<CommitLayoutHook>,
    pub(crate) factory_commit_graph_node: Option<CommitLayoutHook>,
    pub(crate) factory_set_word_align_font: Option<SetWordAlignFontHook>,
    pub(crate) factory_create_equation_nodes: Option<CreateEquationNodesHook>,
    pub(crate) factory_create_math_nodes: Option<CreateMathNodesHook>,
    pub(crate) factory_set_calc_graph_height: Option<SetCalcGraphHeightHook>,
    pub(crate) factory_set_math_underline: Option<SetMathUnderlineHook>,
    pub(crate) factory_create_embed_nodes: Option<CreateEmbedNodesHook>,
    // java private int[] mPrevRunPropertyIds = new int[8];
    mPrevRunPropertyIds: Vec<i32>,
    // java private Object[] mPrevRunPropertyVales = new Object[8];
    mPrevRunPropertyVales: Vec<Option<PropertyValue>>,
    // java private int[] mRunPropertyIds = new int[8];
    mRunPropertyIds: Vec<i32>,
    // java private Object[] mRunPropertyVales = new Object[8];
    mRunPropertyVales: Vec<Option<PropertyValue>>,
}

// java abstract class GraphReader extends GraphBuff
impl GraphReader {
    // java public  GraphReader(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn GraphReader(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java super(mainState, docEnv, viewEnv);
      Self {
        super_: GraphBuff::GraphBuff(mainState, docEnv, viewEnv),
        mCurCP: -1,
        mMaxCP: 0,
        mEnableRevision: false,
        mRevisionFinal: false,
        mSrcNode: None,
        mPropertyTool: None,
        mIsInField: false,
        mIsInTable: false,
        mHappylinkId: -1,
        mFilters: Vec::new(),
        mRunPropFilters: Vec::new(),
        mFilterArgs: FilterArgs::default(),
        mCurStart: -1,
        mCurEnd: -1,
        mIsInMath: false,
        mSymbolConvert: None,
        needReadVanishBreak: false,
        mBlockResult: [0; 2],
        mForceDrawingToEmbed: false,
        mMaxGraphSize: i32::MAX,
        mItem: None,
        mPendingMapCommit: false,
        factory_self: std::ptr::null_mut(),
        factory_create_nodes: None,
        factory_commit_all: None,
        factory_commit_graph_node: None,
        factory_set_word_align_font: None,
        factory_create_equation_nodes: None,
        factory_create_math_nodes: None,
        factory_set_calc_graph_height: None,
        factory_set_math_underline: None,
        factory_create_embed_nodes: None,
        mPrevRunPropertyIds: vec![0; 8],
        mPrevRunPropertyVales: vec![None; 8],
        mRunPropertyIds: vec![0; 8],
        mRunPropertyVales: vec![None; 8],
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mSrcNode = null;
      self.mSrcNode = None;
      // java this.mPropertyTool = null;
      self.mPropertyTool = None;
      // java super.dispose();
      self.super_.dispose();
    }

    // java public void setMaxGraphSize(int maxGraphSize)
    pub fn setMaxGraphSize(&mut self, maxGraphSize: i32) {
      // java this.mMaxGraphSize = maxGraphSize;
      self.mMaxGraphSize = maxGraphSize;
    }

    // java protected void doCleanGraphReader()
    pub fn doCleanGraphReader(&mut self) {
      // java this.mCurCP = -1;
      self.mCurCP = -1;
      // java this.mMaxCP = 0;
      self.mMaxCP = 0;
      // java this.mSrcNode = null;
      self.mSrcNode = None;
      // java this.mIsInMath = false;
      self.mIsInMath = false;
    }

    // java protected final void doPrepareGraphs(int wordAlignFont, LayoutState layoutState)
    pub fn doPrepareGraphs(&mut self, wordAlignFont: i32, layoutState: &mut LayoutState) {
      // java KPList<SrcNode> srcList = layoutState.srcList;
      // java if (!srcList.isEmpty())
      if !layoutState.srcList.isEmpty() {
        // java int viewMode = this.mViewEnv.getViewMode();
        let viewMode: i32 = self.super_.aliased_view().map(|v| v.getViewMode()).unwrap_or(0);
        // java this.mForceDrawingToEmbed = ViewMode.isPhoneReadMode(viewMode) || 4 == viewMode;
        self.mForceDrawingToEmbed = ViewMode::isPhoneReadMode(viewMode) || 4 == viewMode;
        // java this.setWordAlignFont(wordAlignFont, layoutState);
        self.setWordAlignFont(wordAlignFont, layoutState);
        // java layoutState.hasBidiGraph = false;
        layoutState.hasBidiGraph = false;
        // java layoutState.hasCoreCharsContent = false;
        layoutState.hasCoreCharsContent = false;
        // java if (null != layoutState.bidiTool)
        if let Some(bidiTool) = layoutState.bidiTool.as_mut() {
          // java layoutState.bidiTool.clean();
          bidiTool.clean();
        }
        // java layoutState.styleResult.reset();
        layoutState.styleResult.reset();
        // java if (ViewMode.isPhoneReadMode(viewMode))
        if ViewMode::isPhoneReadMode(viewMode) {
          // java layoutState.styleResult.useBuildInFont = false;
          layoutState.styleResult.useBuildInFont = false;
          // java layoutState.tagForSpaceTrans = false;
          layoutState.tagForSpaceTrans = false;
        }
        // java for(this.mSrcNode = (SrcNode)srcList.firstElement(); null != this.mSrcNode; this.mSrcNode = (SrcNode)srcList.getNext(this.mSrcNode))
        self.mSrcNode = Some(layoutState.srcList.firstElement());
        while self.mSrcNode.is_some() {
          // java layoutState.roundError = 0.0F;
          layoutState.roundError = 0.0;
          // java this.read(layoutState);
          self.read(layoutState);
          // java if (layoutState.hasReadEnd || this.mSrcNode.type == 0 && this.mCurCP < this.mMaxCP)
          let srcNodeType: i32 = unsafe { (*self.mSrcNode.unwrap()).r#type };
          if layoutState.hasReadEnd || (srcNodeType == 0 && self.mCurCP < self.mMaxCP) {
            // java break;
            break;
          }
          // java this.mSrcNode = (SrcNode)srcList.getNext(this.mSrcNode);
          self.mSrcNode = Some(layoutState.srcList.getNext(self.mSrcNode.unwrap()));
        }
        // java if (this.mSrcNode == null)
        if self.mSrcNode.is_none() {
          // java layoutState.hasReadEnd = true;
          layoutState.hasReadEnd = true;
        }
        // java layoutState.readEndCP = this.mCurCP;
        layoutState.readEndCP = self.mCurCP;
      }
    }

    // java public boolean prepareGraphs(int wordAlignFont, LayoutState layoutState)
    pub fn prepareGraphs(&mut self, wordAlignFont: i32, layoutState: &mut LayoutState) -> bool {
      let prepared = self.prepareGraphsBeforeCommit(wordAlignFont, layoutState);
      if self.mPendingMapCommit {
        self.finishPrepareGraphs(layoutState);
      }
      prepared
    }

    // rust: GraphFactory host virtual-dispatch bridge. Java calls its
    // createNodes override inside doPrepareGraphs, before TypoPara.commit.
    // Rust product currently invokes that override explicitly at this split.
    pub fn prepareGraphsBeforeCommit(&mut self, wordAlignFont: i32, layoutState: &mut LayoutState) -> bool {
      self.mPendingMapCommit = false;
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
          self.super_.doClear(false, layoutState);
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
        if let Some(mMainState) = self.super_.aliased_main() {
          layoutState.typoPara.alloc(&mut mMainState.snapshot);
        }
        // java this.mTypoFontStylePool.reset();
        self.super_.mTypoFontStylePool.reset();
        // java layoutState.revisions.clean();
        layoutState.revisions.clean();
        // java this.doPrepareGraphs(wordAlignFont, layoutState);
        self.doPrepareGraphs(wordAlignFont, layoutState);
        self.mPendingMapCommit = true;
        // java layoutState.resetGraphsIt();
        layoutState.resetGraphsIt();
        // java return layoutState.hasEmbeds || layoutState.hasEquation || layoutState.hasMaths;
        return layoutState.hasEmbeds || layoutState.hasEquation || layoutState.hasMaths;
      }
    }

    pub fn finishPrepareGraphs(&mut self, layoutState: &mut LayoutState) {
      if !self.mPendingMapCommit { return; }
      // java layoutState.typoPara.commit(this.mMapData, this.mMapSize, layoutState.document, this.mRecommendCPRef);
      layoutState.typoPara.commit(&mut self.super_.mMapData, self.super_.mMapSize, layoutState.document, self.super_.mRecommendCPRef);
      // java this.mMapSize = 0;
      self.super_.mMapSize = 0;
      self.mPendingMapCommit = false;
    }

    // rust: ★测试辅助（03_function_graph_reader）—— mCurCP 推进证明
    pub fn current_cp(&self) -> i32 {
        self.mCurCP
    }

    // java protected abstract void setWordAlignFont(int var1, LayoutState var2)
    pub fn setWordAlignFont(&mut self, var1: i32, var2: &mut LayoutState) {
      let hook = self.factory_set_word_align_font;
      let this = self.factory_self;
      if let Some(hook) = hook {
        hook(this, var1, var2);
      }
    }

    // java private void read(LayoutState layoutState)
    fn read(&mut self, layoutState: &mut LayoutState) {
      // java SrcNode srcNode = this.mSrcNode;
      let srcNode: &mut SrcNode = unsafe { &mut *self.mSrcNode.unwrap() };
      // java if (0 == srcNode.type)
      if 0 == srcNode.r#type {
        // java this.readCoreNode(layoutState);
        self.readCoreNode(layoutState);
        // java this.commitAll(layoutState);
        self.commitAll(layoutState);
      }
      // java else if (3 == srcNode.type)
      else if 3 == srcNode.r#type {
        // java Document mDocument = srcNode.doc;
        let mDocument: *mut dyn Document = match srcNode.doc { Some(d) => d, None => return };
        // java this.mPropertyTool = this.mDocEnv.propToolFactory.create(mDocument);
        self.mPropertyTool = self.super_.aliased_doc()
          .and_then(|d| d.propToolFactory.as_mut())
          .and_then(|f| { let b: Box<Box<dyn IPropertyTool>> = f.create(unsafe { &*mDocument }); Some(*b) });
        // java this.mRevisionFinal = this.mSrcNode.revisionFinal;
        self.mRevisionFinal = srcNode.revisionFinal;
        // java this.mIsInField = false;
        self.mIsInField = false;
        // java this.mHappylinkId = -1;
        self.mHappylinkId = -1;
        // java this.mCurCP = srcNode.cp;
        self.mCurCP = srcNode.cp;
        // java IChpxTable.Entry run = mDocument.getChpxTable().seek(srcNode.cp);
        let run: Option<Box<dyn ChpxEntry>> = unsafe { (*mDocument).getChpxTable().and_then(|t| unsafe { IChpxTable::seek(&*t, srcNode.cp) }) };
        if let Some(run) = run {
          // java PropertySet finalRunProp = this.mPropertyTool.getFinalRunPropert(run, layoutState.paraEnv.curParaPropEnv.paraProperty, this.mRevisionFinal);
          let finalRunProp: Option<PropertySet> = self.mPropertyTool.as_mut().and_then(|pt| {
            layoutState.paraEnv.curParaPropEnv.as_ref()
              .and_then(|env| env.paraProperty.as_ref())
              .map(|pp| pt.getFinalRunPropert__entry_propertyset_boolean(&*run, pp, self.mRevisionFinal).clone())
          });
          let mut finalRunProp: Option<PropertySet> = finalRunProp;
          // java ArrayList<IRunPropFilter> runPropFilters = this.mRunPropFilters;
          // java for(int size = runPropFilters.size(); i < size; ++i)
          {
            let mut i: usize = 0;
            let size: usize = self.mRunPropFilters.len();
            while i < size {
              // java finalRunProp = filter.doFilter(finalRunProp, run, this.mRevisionFinal, layoutState);
              if let Some(prop) = finalRunProp.take() {
                finalRunProp = Some(self.mRunPropFilters[i].doFilter(&prop, &*run, self.mRevisionFinal, layoutState));
              }
              i += 1;
            }
          }
          // java if (this.isUnsupportMathChar(srcNode.data.charAt(0)))
          let data_first: char = srcNode.data.clone().and_then(|d| d.chars().next()).unwrap_or('\u{0}');
          if self.isUnsupportMathChar(data_first) {
            // java MutablePropertySet newMutablePropertyMap = new MutablePropertySet(finalRunProp);
            // java newMutablePropertyMap.setString(3, "Times New Roman");
            // java finalRunProp = newMutablePropertyMap.freeze();
            if let Some(prop) = finalRunProp.take() {
              let mut newMutablePropertyMap: MutablePropertySet = MutablePropertySet::new_from_ps(&prop);
              newMutablePropertyMap.setString(3, String::from("Times New Roman"));
              finalRunProp = Some(newMutablePropertyMap.freeze());
            }
          }
          // java srcNode.finalRunProp = finalRunProp;
          srcNode.finalRunProp = finalRunProp.clone();
          // java PLCField plcField = mDocument.getPLCField();
          let plcField: Option<*mut crate::cn::wps::moffice::writer::data::PLCField::PLCField> = unsafe { (*mDocument).getPLCField() };
          // java if (null != plcField && !plcField.isEmpty())
          if let Some(plcField) = plcField {
            // rust: ★wr-29 03 —— PLCField.isEmpty 未接线；以 fieldTriples(start..end) 空判等价（hello 无域）。
            let triples: Vec<crate::cn::wps::moffice::writer::data::PLCField::FieldTriple> = unsafe { (&mut *plcField).getFieldTriples__int_int(self.mCurCP, self.mCurCP + srcNode.length) };
            let _ = triples;
            // java this.mFilterArgs.start = this.mCurCP;
            // java this.mFilterArgs.end = this.mCurCP + srcNode.length;
            // java FieldFilter filter = FieldFilter.create(mDocument, this.mFilterArgs.start, this.mFilterArgs.end, this);
            // java if (null != filter) { filter.doFilter(this.mFilterArgs, layoutState); }
            // rust: DEFERRED —— FieldFilter 非 hello（GR-D3）
          }
          // java CharsNode charsNode = this.addCharsNode(0, srcNode.length, srcNode.finalRunProp, layoutState.charsList);
          let charsNode: *mut CharsNode = self.addCharsNode(0, srcNode.length, srcNode.finalRunProp.as_ref(), &mut layoutState.charsList) as *mut CharsNode;
          // java charsNode.srcRef = srcNode;
          let charsNode_ref: &mut CharsNode = unsafe { &mut *charsNode };
          charsNode_ref.srcRef = self.mSrcNode;
          // java charsNode.offsetOfRef = 0;
          charsNode_ref.offsetOfRef = 0;
          // java charsNode.lengthOfRef = srcNode.length;
          charsNode_ref.lengthOfRef = srcNode.length;
          // java charsNode.isInMath = true;
          charsNode_ref.isInMath = true;
          // java this.calcCharsNodeProp(charsNode, srcNode.finalRunProp);
          if let Some(prop) = srcNode.finalRunProp.as_ref() {
            self.calcCharsNodeProp(charsNode_ref, prop);
          }
          drop(charsNode_ref);
          // java TypoPara typoPara = layoutState.typoPara;
          // java int graphIndex = typoPara.allocGraphs(unsafe { (*charsNode).lengthOfRef });
          let graphIndex: i32 = layoutState.typoPara.allocGraphs(unsafe { (*charsNode).lengthOfRef });
          // java typoPara.putChars((char[])srcNode.data.toCharArray(), 0, charsNode.lengthOfRef, graphIndex);
          let mut dataChars: Vec<char> = srcNode.data.clone().unwrap_or_default().chars().collect();
          layoutState.typoPara.putChars__chararr_int_int_int(&mut dataChars, 0, unsafe { (*charsNode).lengthOfRef }, graphIndex);
          // java this.createNodes(charsNode, graphIndex, charsNode.lengthOfRef, layoutState);
          self.createNodes(unsafe { &mut *charsNode }, graphIndex, unsafe { (*charsNode).lengthOfRef }, layoutState);
          // java this.commitGraphNode(layoutState);
          self.commitGraphNode(layoutState);
        }
      }
      // java else if (srcNode.length > 0)
      else if srcNode.length > 0 {
        // java CharsNode charsNode = this.addCharsNode(0, srcNode.length, srcNode.finalRunProp, layoutState.charsList);
        let charsNode: *mut CharsNode = self.addCharsNode(0, srcNode.length, srcNode.finalRunProp.as_ref(), &mut layoutState.charsList) as *mut CharsNode;
        // java charsNode.srcRef = srcNode;
        let charsNode_ref: &mut CharsNode = unsafe { &mut *charsNode };
        charsNode_ref.srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = 0;
        charsNode_ref.offsetOfRef = 0;
        // java charsNode.lengthOfRef = srcNode.length;
        charsNode_ref.lengthOfRef = srcNode.length;
        // java this.calcCharsNodeProp(charsNode, srcNode.finalRunProp);
        if let Some(prop) = srcNode.finalRunProp.as_ref() {
          self.calcCharsNodeProp(charsNode_ref, prop);
        }
        drop(charsNode_ref);
        // java TypoPara typoPara = layoutState.typoPara;
        // java int graphIndex = typoPara.allocGraphs(unsafe { (*charsNode).lengthOfRef });
        let graphIndex: i32 = layoutState.typoPara.allocGraphs(unsafe { (*charsNode).lengthOfRef });
        // java typoPara.putChars((char[])srcNode.data.toCharArray(), 0, charsNode.lengthOfRef, graphIndex);
        let mut dataChars: Vec<char> = srcNode.data.clone().unwrap_or_default().chars().collect();
        layoutState.typoPara.putChars__chararr_int_int_int(&mut dataChars, 0, unsafe { (*charsNode).lengthOfRef }, graphIndex);
        // java this.createNodes(charsNode, graphIndex, charsNode.lengthOfRef, layoutState);
        self.createNodes(unsafe { &mut *charsNode }, graphIndex, unsafe { (*charsNode).lengthOfRef }, layoutState);
        // java this.commitAll(layoutState);
        self.commitAll(layoutState);
      }
    }

    // java private boolean isUnsupportMathChar(char c)
    fn isUnsupportMathChar(&self, c: char) -> bool {
      // java return c >= 10214 && c < 10224;
      return (c as i32) >= 10214 && (c as i32) < 10224;
    }

    // java protected abstract void commitGraphNode(LayoutState var1)
    pub fn commitGraphNode(&mut self, var1: &mut LayoutState) {
      let hook = self.factory_commit_graph_node;
      let this = self.factory_self;
      if let Some(hook) = hook {
        hook(this, var1);
      }
    }

    // java protected abstract void createNodes(CharsNode var1, int var2, int var3, LayoutState var4)
    pub fn createNodes(&mut self, var1: &mut CharsNode, var2: i32, var3: i32, var4: &mut LayoutState) {
      let hook = self.factory_create_nodes;
      let this = self.factory_self;
      if let Some(hook) = hook {
        hook(this, var1, var2, var3, var4);
      }
    }

    // java protected abstract void commitAll(LayoutState var1)
    pub fn commitAll(&mut self, var1: &mut LayoutState) {
      let hook = self.factory_commit_all;
      let this = self.factory_self;
      if let Some(hook) = hook {
        hook(this, var1);
      }
    }

    // java private void readCoreNode(LayoutState layoutState)
    fn readCoreNode(&mut self, layoutState: &mut LayoutState) {
      // java Document mDocument = this.mSrcNode.doc;
      let mDocument: *mut dyn Document = match unsafe { (*self.mSrcNode.unwrap()).doc } { Some(d) => d, None => return };
      // java this.mPropertyTool = this.mDocEnv.propToolFactory.create(mDocument);
      self.mPropertyTool = self.super_.aliased_doc()
        .and_then(|d| d.propToolFactory.as_mut())
        .and_then(|f| { let b: Box<Box<dyn IPropertyTool>> = f.create(unsafe { &*mDocument }); Some(*b) });
      // java this.mRevisionFinal = this.mViewEnv.isRevisionFinal();
      self.mRevisionFinal = self.super_.aliased_view().map(|v| v.isRevisionFinal()).unwrap_or(false);
      // java this.mCurCP = this.mSrcNode.cp;
      let srcNodeCp: i32 = unsafe { (*self.mSrcNode.unwrap()).cp };
      let srcNodeLen: i32 = unsafe { (*self.mSrcNode.unwrap()).length };
      self.mCurCP = srcNodeCp;
      // java this.mMaxCP = Math.min(this.mSrcNode.cp + this.mSrcNode.length, Math.min(mDocument.getLength(), layoutState.pageEndLimitCP));
      self.mMaxCP = (srcNodeCp + srcNodeLen).min(unsafe { (*mDocument).getLength() }.min(layoutState.pageEndLimitCP));
      // java this.mEnableRevision = this.mSrcNode.revisionFinal;
      self.mEnableRevision = unsafe { (*self.mSrcNode.unwrap()).revisionFinal };
      // java ParaPropEnv curParaPropEnv = layoutState.paraEnv.curParaPropEnv;
      // java if (this.mEnableRevision)
      if self.mEnableRevision {
        // java PropertySet paraProperty = curParaPropEnv.paraProperty;
        // java this.mIsInTable = paraProperty.getBoolean(223, false);
        if let Some(env) = layoutState.paraEnv.curParaPropEnv.as_ref() {
          if let Some(paraProperty) = env.paraProperty.as_ref() {
            self.mIsInTable = paraProperty.getBoolean__int_boolean(223, false);
          }
        }
      }
      // java this.mIsInField = false;
      self.mIsInField = false;
      // java this.mHappylinkId = -1;
      self.mHappylinkId = -1;
      // java while(this.mCurCP < this.mMaxCP)
      'corenode: while self.mCurCP < self.mMaxCP {
        // java this.mFilterArgs.init(this.mCurCP, this.mMaxCP);
        self.mFilterArgs.init(self.mCurCP, self.mMaxCP);
        // java boolean hasDoFilter = false;
        let mut hasDoFilter: bool = false;
        // java for(int size = this.mFilters.size(); i < size && !(hasDoFilter = ((IFilter)this.mFilters.get(i)).doFilter(this.mFilterArgs, layoutState)); ++i)
        {
          let mut i: usize = 0;
          let size: usize = self.mFilters.len();
          while i < size {
            // Java calls back into this reader from FunctionSepField. Take the filter
            // out so that callback can use the reader without aliasing mFilters.
            let mut filter = std::mem::replace(&mut self.mFilters[i], Box::new(IdleFilter));
            hasDoFilter = filter.doFilter(&mut self.mFilterArgs, layoutState);
            self.mFilters[i] = filter;
            i += 1;
            // java 循环条件：i < size && !hasDoFilter
            if hasDoFilter || i >= size { break; }
          }
        }
        // java if (hasDoFilter)
        if hasDoFilter {
          // java if (this.mFilterArgs.next > this.mCurCP)
          if self.mFilterArgs.next > self.mCurCP {
            // java this.mCurCP = this.mFilterArgs.next;
            self.mCurCP = self.mFilterArgs.next;
          }
          // java if (layoutState.hasReadEnd)
          if layoutState.hasReadEnd {
            // java break;
            break 'corenode;
          }
        }
        // java else
        else {
          // java if (!this.readCoreRangeDefault(this.mFilterArgs.start, this.mFilterArgs.end, layoutState))
          if !self.readCoreRangeDefault(self.mFilterArgs.start, self.mFilterArgs.end, layoutState) {
            // java break;
            break 'corenode;
          }
          // java if (this.mFilterArgs.next > this.mCurCP)
          if self.mFilterArgs.next > self.mCurCP {
            // java this.mCurCP = this.mFilterArgs.next;
            self.mCurCP = self.mFilterArgs.next;
          }
        }
      }
      // java if (0 == mDocument.getType() && this.mCurCP == this.mMainState.secEnd - 1 && this.mCurCP < this.mMaxCP && this.mCurCP != mDocument.getLength() - 1)
      if 0 == unsafe { (*mDocument).getType__on_IDocument() }
        && self.mCurCP == self.super_.aliased_main_ref().map(|m| m.secEnd - 1).unwrap_or(i32::MIN + 1)
        && self.mCurCP < self.mMaxCP
        && self.mCurCP != unsafe { (*mDocument).getLength() } - 1
      {
        // java this.readCoreRangeDefault(this.mCurCP, this.mCurCP + 1, layoutState);
        self.readCoreRangeDefault(self.mCurCP, self.mCurCP + 1, layoutState);
      }
    }

    // java private boolean readCoreRangeDefault(int start, int end, LayoutState layoutState)
    fn readCoreRangeDefault(&mut self, start: i32, end: i32, layoutState: &mut LayoutState) -> bool {
      // java if (start >= end)
      if start >= end {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Document mDocument = layoutState.document;
        let mDocument: *mut dyn Document = match layoutState.document { Some(d) => d, None => return false };
        // java IChpxTable.Entry run = mDocument.getChpxTable().seek(start);
        let run: Option<Box<dyn ChpxEntry>> = unsafe { (*mDocument).getChpxTable().and_then(|t| unsafe { IChpxTable::seek(&*t, start) }) };
        let mut run: Box<dyn ChpxEntry> = match run { Some(r) => r, None => return false };
        // java if (run.isEnd())
        if run.isEnd() {
          // java return false;
          return false;
        }
        // java this.mCurStart = -1;
        self.mCurStart = -1;
        // java int maxCP = Math.min(end, this.mMaxCP);
        let maxCP: i32 = end.min(self.mMaxCP);
        // java int curRunStart = run.position();
        let mut curRunStart: i32 = run.position();
        // java int paraRangeEnd = layoutState.paraEnv.curParaPropEnv.end;
        let mut paraRangeEnd: i32 = layoutState.paraEnv.curParaPropEnv.as_ref().map(|env| env.end).unwrap_or(i32::MAX);
        // java boolean needContinue = true;
        let mut needContinue: bool = true;
        // java this.needReadVanishBreak = false;
        self.needReadVanishBreak = false;
        // java KPList<CharsNode> charsList = layoutState.charsList;
        // java CharsNode lastCharsNodeBefore = (CharsNode)charsList.lastElement();
        let lastCharsNodeBefore: *mut CharsNode = if layoutState.charsList.isEmpty() { std::ptr::null_mut() } else { layoutState.charsList.lastElement() };
        // java CharsNode lastCharsNode = null;
        let mut lastCharsNode: *mut CharsNode = std::ptr::null_mut();
        // java int graphsRemain = this.getGraphsRemain(layoutState);
        let mut graphsRemain: i32 = self.getGraphsRemain(layoutState);
        // java PropertySet lastRunPropertySet = null;
        let mut lastRunPropertySetAddr: usize = 0;
        // java int prevRunPropertySize = 0;
        let mut prevRunPropertySize: i32 = 0;
        // java int[] prevRunPropertyIds = this.mPrevRunPropertyIds;
        let mut prevRunPropertyIds: Vec<i32> = std::mem::take(&mut self.mPrevRunPropertyIds);
        // java Object[] prevRunPropertyVales = this.mPrevRunPropertyVales;
        let mut prevRunPropertyVales: Vec<Option<PropertyValue>> = std::mem::take(&mut self.mPrevRunPropertyVales);
        // java int runPropertySize = 0;
        let mut runPropertySize: i32 = 0;
        // java int[] runPropertyIds = this.mRunPropertyIds;
        let mut runPropertyIds: Vec<i32> = std::mem::take(&mut self.mRunPropertyIds);
        // java Object[] runPropertyVales = this.mRunPropertyVales;
        let mut runPropertyVales: Vec<Option<PropertyValue>> = std::mem::take(&mut self.mRunPropertyVales);

        // java while(this.mCurCP < maxCP)
        'runloop: while self.mCurCP < maxCP {
          // java IChpxTable.Entry nextRun = run.getNext();
          let nextRun: Option<Box<dyn ChpxEntry>> = ChpxEntry::getNext(&*run);
          // java int nextRunStart = 0;
          let nextRunStart: i32 = match &nextRun {
            Some(nr) => {
              // java if (nextRun.isEnd())
              if nr.isEnd() {
                // java nextRunStart = mDocument.getLength();
                unsafe { (*mDocument).getLength() }
              } else {
                // java nextRunStart = nextRun.position();
                nr.position()
              }
            }
            None => unsafe { (*mDocument).getLength() },
          };
          // java label167: {
          'label167: {
            // java if (curRunStart >= paraRangeEnd)
            if curRunStart >= paraRangeEnd {
              // java if (null != lastCharsNode && lastCharsNode.srcRef.cp + lastCharsNode.offsetOfRef + lastCharsNode.lengthOfRef == paraRangeEnd)
              if !lastCharsNode.is_null() {
                let lcn: &CharsNode = unsafe { &*lastCharsNode };
                let lcnSrcCp: i32 = if let Some(sp) = lcn.srcRef { unsafe { (*sp).cp } } else { -1 };
                if lcnSrcCp + lcn.offsetOfRef + lcn.lengthOfRef == paraRangeEnd {
                  // java needContinue = false;
                  needContinue = false;
                  // java break;  // 跳出 label167
                  break 'label167;
                }
              }
              // java IPapxTable.Entry nextPara = mDocument.getPapxTable().seek(paraRangeEnd);
              let nextPara: Option<Box<dyn PapxEntry>> = unsafe { (*mDocument).getPapxTable().and_then(|t| unsafe { IPapxTable::seek(&*t, paraRangeEnd) }) };
              let nextPara: Box<dyn PapxEntry> = match nextPara {
                Some(np) => np,
                None => { needContinue = false; break 'label167; }
              };
              // java if (nextPara.isEnd())
              if nextPara.isEnd() {
                // java needContinue = false;
                needContinue = false;
                // java break;  // 跳出 label167
                break 'label167;
              }
              // java PropertySet finalNextParaProp = this.mPropertyTool.getFinalParaProp(nextPara, this.mRevisionFinal);
              // java if (GraphReaderTool.isInTextFrame(layoutState, finalNextParaProp, paraRangeEnd))
              let finalNextParaProp: &PropertySet = self.mPropertyTool.as_mut().unwrap().getFinalParaProp__entry_boolean(&*nextPara, self.mRevisionFinal);
              if GraphReaderTool::isInTextFrame__layoutstate_propertyset_int(layoutState, finalNextParaProp, paraRangeEnd) {
                // java break label167;
                break 'label167;
              }
              // java if (layoutState.tablePeriLayer <= 1)
              let insideTable: bool;
              if layoutState.tablePeriLayer <= 1 {
                // java if (finalNextParaProp.getBoolean(223, false))
                insideTable = finalNextParaProp.getBoolean__int_boolean(223, false);
              } else {
                // java else if (finalNextParaProp.getInt(224, 1) > layoutState.tablePeriLayer - 1)
                insideTable = finalNextParaProp.getInt__int_int(224, 1) > layoutState.tablePeriLayer - 1;
              }
              if insideTable {
                // java break label167;
                break 'label167;
              }
              // java if (charsList.isEmpty())
              if layoutState.charsList.isEmpty() {
                // java needContinue = false;
                needContinue = false;
                // java break;  // 跳出 label167
                break 'label167;
              }
              // java layoutState.paraEnv.curParaPropEnv.setLayoutRange(layoutState.cp, nextPara.endCp(), mDocument);
              if let Some(env) = layoutState.paraEnv.curParaPropEnv.as_mut() {
                env.setLayoutRange(layoutState.cp, nextPara.endCp(), unsafe { &*mDocument });
              }
              // java paraRangeEnd = layoutState.paraEnv.curParaPropEnv.end;
              paraRangeEnd = layoutState.paraEnv.curParaPropEnv.as_ref().map(|env| env.end).unwrap_or(paraRangeEnd);
            }
            // java PropertySet orgRunPropertySet = run.getProperty();
            let orgRunPropertySet: Option<&PropertySet> = ChpxEntry::getProperty(&*run);
            let Some(orgRunPropertySet) = orgRunPropertySet else { needContinue = false; break 'label167; };
            let orgRunPropertyAddr: usize = orgRunPropertySet as *const PropertySet as usize;
            // java runPropertySize = orgRunPropertySet.size();
            runPropertySize = orgRunPropertySet.size();
            // java if (runPropertySize > runPropertyIds.length)
            if (runPropertySize as usize) > runPropertyIds.len() {
              // java runPropertyIds = new int[runPropertySize];
              // java runPropertyVales = new Object[runPropertySize];
              runPropertyIds = vec![0; runPropertySize as usize];
              runPropertyVales = vec![None; runPropertySize as usize];
            }
            // java orgRunPropertySet.getAll(runPropertyIds, runPropertyVales);
            {
              let mut ids: Vec<i32> = vec![0; runPropertySize as usize];
              let mut vals: Vec<Option<PropertyValue>> = vec![None; runPropertySize as usize];
              let n = orgRunPropertySet.getAll(&mut ids, Some(&mut vals));
              let n = n as usize;
              let ids_len = runPropertyIds.len().min(n);
              runPropertyIds[..ids_len].copy_from_slice(&ids[..ids_len]);
              let vals_len = runPropertyVales.len().min(n);
              runPropertyVales[..vals_len].clone_from_slice(&vals[..vals_len]);
            }
            // java runPropertySize = this.removeRsids(runPropertyIds, runPropertyVales, runPropertySize);
            runPropertySize = self.removeRsids(&mut runPropertyIds, &mut runPropertyVales, runPropertySize);
            // java if (null != lastCharsNode && lastCharsNode.srcRef == this.mSrcNode && lastCharsNode.offsetOfRef + lastCharsNode.lengthOfRef == curRunStart - this.mSrcNode.cp && (!orgRunPropertySet.contains(16) || !orgRunPropertySet.getBoolean(16)))
            if !lastCharsNode.is_null() {
              let lcn: &CharsNode = unsafe { &*lastCharsNode };
              let srcNodeCp: i32 = self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(-1);
              if lcn.srcRef == self.mSrcNode
                && lcn.offsetOfRef + lcn.lengthOfRef == curRunStart - srcNodeCp
                && (!orgRunPropertySet.contains(16) || !orgRunPropertySet.getBoolean__int_boolean(16, false))
              {
                // java boolean propertySetEqual = lastRunPropertySet == orgRunPropertySet;
                let mut propertySetEqual: bool = lastRunPropertySetAddr == orgRunPropertyAddr;
                // java if (!propertySetEqual && prevRunPropertySize == runPropertySize)
                if !propertySetEqual && prevRunPropertySize == runPropertySize {
                  // java propertySetEqual = this.propertySetEqual(runPropertyIds, runPropertyVales, prevRunPropertyIds, prevRunPropertyVales, runPropertySize);
                  propertySetEqual = self.propertySetEqual(&runPropertyIds, &runPropertyVales, &prevRunPropertyIds, &prevRunPropertyVales, runPropertySize);
                }
                // java if (propertySetEqual)
                if propertySetEqual {
                  // java int lenOfRef = Math.min(Math.min(nextRunStart, maxCP) - curRunStart, graphsRemain);
                  let lenOfRef: i32 = (nextRunStart.min(maxCP) - curRunStart).min(graphsRemain);
                  // java end = curRunStart + lenOfRef;
                  let end: i32 = curRunStart + lenOfRef;
                  // java this.requestReadText(curRunStart, end, layoutState);
                  self.requestReadText(curRunStart, end, layoutState);
                  // java this.mCurCP = end;
                  self.mCurCP = end;
                  // java lastCharsNode.lengthOfRef += lenOfRef;
                  unsafe { (*lastCharsNode).lengthOfRef += lenOfRef };
                  // java graphsRemain = this.getGraphsRemain(layoutState);
                  graphsRemain = self.getGraphsRemain(layoutState);
                  // java if (graphsRemain <= 0)
                  if graphsRemain <= 0 {
                    // java break;  // 跳出 label167
                    break 'label167;
                  }
                  // java run = nextRun;
                  if let Some(nr) = nextRun { run = nr; }
                  // java curRunStart = nextRunStart;
                  curRunStart = nextRunStart;
                  // java continue;  // 继续 while
                  continue 'runloop;
                }
              }
            }
            // java lastCharsNode = this.readRun(run, Math.max(curRunStart, start), Math.min(nextRunStart, maxCP), layoutState);
            lastCharsNode = match self.readRun(&*run, curRunStart.max(start), nextRunStart.min(maxCP), layoutState) {
              Some(p) => p,
              None => std::ptr::null_mut(),
            };
            // java graphsRemain = this.getGraphsRemain(layoutState);
            graphsRemain = self.getGraphsRemain(layoutState);
            // java if (graphsRemain <= 0)
            if graphsRemain <= 0 {
              // java break;  // 跳出 label167
              break 'label167;
            }
            // java run = nextRun;
            if let Some(nr) = nextRun { run = nr; }
            // java curRunStart = nextRunStart;
            curRunStart = nextRunStart;
            // java lastRunPropertySet = orgRunPropertySet;
            lastRunPropertySetAddr = orgRunPropertyAddr;
            // java prevRunPropertySize = runPropertySize;
            prevRunPropertySize = runPropertySize;
            // java int[] tempIds = prevRunPropertyIds; prevRunPropertyIds = runPropertyIds; runPropertyIds = tempIds;
            std::mem::swap(&mut prevRunPropertyIds, &mut runPropertyIds);
            // java Object[] tempObjs = prevRunPropertyVales; prevRunPropertyVales = runPropertyVales; runPropertyVales = tempObjs;
            std::mem::swap(&mut prevRunPropertyVales, &mut runPropertyVales);
            // java continue;  // 继续 while
            continue 'runloop;
          }
          // java （label167 被 break 后到达此处）
          // java char lastChar = mDocument.getTextRope().charAt(paraRangeEnd - 1);
          let lastChar: char = {
            if paraRangeEnd - 1 < 0 { '\u{0}' } else {
              match unsafe { (*mDocument).getTextRope() } {
                Some(rope) => unsafe { &mut *rope }.charAt(paraRangeEnd - 1),
                None => '\u{0}',
              }
            }
          };
          // java if (lastChar == '\r' || lastChar == 7 || lastChar == 14)
          if lastChar == '\r' || lastChar == '\u{7}' || lastChar == '\u{e}' {
            // java this.needReadVanishBreak = true;
            self.needReadVanishBreak = true;
          }
          // java needContinue = false;
          needContinue = false;
          // java break;  // 跳出 while
          break 'runloop;
        }

        // java this.commitReadText(layoutState);
        self.commitReadText(layoutState);

        // java for(CharsNode charsNode = null == lastCharsNodeBefore ? (CharsNode)charsList.firstElement() : (CharsNode)charsList.getNext(lastCharsNodeBefore); null != charsNode; charsNode = (CharsNode)charsList.getNext(charsNode))
        let mut charsNode: *mut CharsNode = if lastCharsNodeBefore.is_null() {
          layoutState.charsList.firstElement()
        } else {
          layoutState.charsList.getNext(lastCharsNodeBefore)
        };
        while !charsNode.is_null() {
          // java int graphIndex = this.getGraphIndex(charsNode.offsetOfRef + charsNode.srcRef.cp);
          let cn: &CharsNode = unsafe { &*charsNode };
          let srcCp: i32 = if let Some(sp) = cn.srcRef { unsafe { (*sp).cp } } else { -1 };
          let graphIndex: i32 = self.super_.getGraphIndex(cn.offsetOfRef + srcCp);
          // java PropertySet runProp = charsNode.finalRunProp;
          let runProp: Option<PropertySet> = cn.finalRunProp.clone();
          // java Integer charSymbol = (Integer)runProp.getValue(20);
          // java String fontSymbol;
          let charSymbol: Option<&PropertyValue> = runProp.as_ref().and_then(|p| p.getValue(20));
          let fontSymbol: Option<&PropertyValue> = runProp.as_ref().and_then(|p| p.getValue(19));
          // java if (null != charSymbol && null != (fontSymbol = (String)runProp.getValue(19)))
          if charSymbol.is_some() && fontSymbol.is_some() {
            // rust: ★wr-29 03 —— 符号字体路径（props 20/19）非 hello CORECHARS；DEFERRED，见 Java GraphReader.java:530-540。
            todo!("DEFERRED_NON_FUNCTION");
            /* DEFERRED_NON_FUNCTION
            // java this.mSymbolConvert = this.getConvertFont(fontSymbol);
            // java char ch = (char)charSymbol.shortValue();
            // java if (null != this.mSymbolConvert) { ch = this.mSymbolConvert.convert(ch); }
            // java this.readSymbolCharNode(charsNode, graphIndex, ch, fontSymbol, layoutState);
            // java this.mSymbolConvert = null;
            */
          }
          // java else
          else {
            // java this.mSymbolConvert = this.getConvertFont(runProp.getString(3, (String)null));
            let fontName: String = runProp.as_ref().map(|p| p.getString__int_string(3, "")).unwrap_or_default();
            self.mSymbolConvert = self.getConvertFont(&fontName);
            // java this.createNodes(charsNode, graphIndex, charsNode.lengthOfRef, layoutState);
            self.createNodes(unsafe { &mut *charsNode }, graphIndex, cn.lengthOfRef, layoutState);
            // java this.mSymbolConvert = null;
            self.mSymbolConvert = None;
          }
          // java charsNode = (CharsNode)charsList.getNext(charsNode);
          charsNode = layoutState.charsList.getNext(charsNode);
        }

        // java if (this.needReadVanishBreak || !layoutState.hasCoreCharsContent() && layoutState.hasDrawing() && this.mCurCP >= layoutState.getRangeEnd() && 1 == layoutState.tablePeriLayer && 0 == layoutState.textFrameLayer && ViewMode.isWebMode(this.mViewEnv.getViewMode()))
        if self.needReadVanishBreak
          || (!layoutState.hasCoreCharsContent()
            && layoutState.hasDrawing()
            && self.mCurCP >= layoutState.getRangeEnd()
            && 1 == layoutState.tablePeriLayer
            && 0 == layoutState.textFrameLayer
            && ViewMode::isWebMode(self.super_.aliased_view().map(|v| v.getViewMode()).unwrap_or(0)))
        {
          // java this.readVanishBreak(layoutState);
          self.readVanishBreak(layoutState);
        }
        // java this.commitAll(layoutState);
        self.commitAll(layoutState);
        // java if (this.mCurCP >= paraRangeEnd)
        if self.mCurCP >= paraRangeEnd {
          // java layoutState.hasReadEnd = true;
          layoutState.hasReadEnd = true;
          // java needContinue = false;
          needContinue = false;
        }
        // java else if (graphsRemain <= 0)
        else if graphsRemain <= 0 {
          // java layoutState.paraIsOverflow = true;
          layoutState.paraIsOverflow = true;
          // java needContinue = false;
          needContinue = false;
        }
        // java return needContinue;
        return needContinue;
      }
    }

    // java private int removeRsids(int[] runPropertyIds, Object[] runPropertyVales, int runPropertySize)
    fn removeRsids(&mut self, runPropertyIds: &mut Vec<i32>, runPropertyVales: &mut Vec<Option<PropertyValue>>, mut runPropertySize: i32) -> i32 {
      // java int index = 0;
      let mut index: i32 = 0;
      // java while(index < runPropertySize)
      while index < runPropertySize {
        // java int id = runPropertyIds[index];
        let id: i32 = runPropertyIds[index as usize];
        // java if (id >= 50 && id <= 51)
        if id >= 50 && id <= 51 {
          // java if (index < runPropertySize - 1)
          if index < runPropertySize - 1 {
            // java System.arraycopy(runPropertyIds, index + 1, runPropertyIds, index, runPropertySize - index - 1);
            let ids = runPropertyIds.clone();
            runPropertyIds[(index as usize)..(runPropertySize as usize)].copy_from_slice(&ids[(index as usize + 1)..(runPropertySize as usize)]);
            // java System.arraycopy(runPropertyVales, index + 1, runPropertyVales, index, runPropertySize - index - 1);
            let vals = runPropertyVales.clone();
            runPropertyVales[(index as usize)..(runPropertySize as usize)].clone_from_slice(&vals[(index as usize + 1)..(runPropertySize as usize)]);
          }
          // java --runPropertySize;
          runPropertySize -= 1;
        }
        // java else
        else {
          // java ++index;
          index += 1;
        }
      }
      // java return runPropertySize;
      return runPropertySize;
    }

    // java private boolean propertySetEqual(int[] runPropertyIds, Object[] runPropertyVales, int[] prevRunPropertyIds, Object[] prevRunPropertyVales, int size)
    fn propertySetEqual(&mut self, runPropertyIds: &[i32], runPropertyVales: &[Option<PropertyValue>], prevRunPropertyIds: &[i32], prevRunPropertyVales: &[Option<PropertyValue>], size: i32) -> bool {
      // java for(int i = 0; i < size; ++i)
      let mut i: i32 = 0;
      while i < size {
        // java if (runPropertyIds[i] != prevRunPropertyIds[i])
        if runPropertyIds[i as usize] != prevRunPropertyIds[i as usize] {
          // java return false;
          return false;
        }
        // java if (runPropertyVales[i] != null)
        match (runPropertyVales.get(i as usize).and_then(|v| v.as_ref()), prevRunPropertyVales.get(i as usize).and_then(|v| v.as_ref())) {
          // java if (!runPropertyVales[i].equals(prevRunPropertyVales[i]))
          (Some(a), Some(b)) => {
            if !PropertySet::isArrayEqual(a, b) {
              // java return false;
              return false;
            }
          }
          // java else
          // java { if (prevRunPropertyVales[i] == null) { throw new RuntimeException("shound not reach here!"); }
          // java   if (!prevRunPropertyVales[i].equals(runPropertyVales[i])) { return false; } }
          (None, None) => {
            // java throw new RuntimeException("shound not reach here!");
            panic!("shound not reach here!");
          }
          (Some(_), None) => {
            // java runPropertyVales[i].equals(null) == false
            return false;
          }
          (None, Some(_)) => {
            // java prevRunPropertyVales[i].equals(null) == false
            return false;
          }
        }
        // java ++i;
        i += 1;
      }
      // java return true;
      return true;
    }

    // java private int getGraphsRemain(LayoutState layoutState)
    fn getGraphsRemain(&mut self, layoutState: &mut LayoutState) -> i32 {
      // java int remainSize = this.mMaxGraphSize - layoutState.typoPara.hasAlloc();
      let mut remainSize: i32 = self.mMaxGraphSize - layoutState.typoPara.hasAlloc();
      // java if (this.mCurStart >= 0)
      if self.mCurStart >= 0 {
        // java remainSize -= this.mCurEnd - this.mCurStart;
        remainSize -= self.mCurEnd - self.mCurStart;
      }
      // java return remainSize;
      return remainSize;
    }

    // java private void readVanishBreak(LayoutState layoutState)
    fn readVanishBreak(&mut self, layoutState: &mut LayoutState) {
      // java this.readBreak(this.mCurCP - 1, true, layoutState);
      self.readBreak(self.mCurCP - 1, true, layoutState);
    }

    // java void readBreak(int breakCP, boolean isVanish, LayoutState layoutState)
    pub(crate) fn readBreak(&mut self, breakCP: i32, isVanish: bool, layoutState: &mut LayoutState) {
      // java Document mDocument = layoutState.document;
      let mDocument: *mut dyn Document = match layoutState.document { Some(d) => d, None => return };
      // java char ch = mDocument.getTextRope().charAt(breakCP);
      let ch: char = match unsafe { (*mDocument).getTextRope() } {
        Some(rope) => unsafe { &mut *rope }.charAt(breakCP),
        None => return,
      };
      // java switch (ch)
      // java case '\u0007': case '\u000b': case '\f': case '\r': case '\u000e':
      if ch == '\u{7}' || ch == '\u{b}' || ch == '\u{c}' || ch == '\r' || ch == '\u{e}' {
        // java IChpxTable.Entry run = mDocument.getChpxTable().seek(breakCP);
        let run: Option<Box<dyn ChpxEntry>> = unsafe { (*mDocument).getChpxTable().and_then(|t| unsafe { IChpxTable::seek(&*t, breakCP) }) };
        let Some(run) = run else { return };
        // java PropertySet prop = this.mPropertyTool.getFinalRunPropert(run, layoutState.paraEnv.curParaPropEnv.paraProperty, this.mRevisionFinal);
        let prop: Option<PropertySet> = self.mPropertyTool.as_mut().and_then(|pt| {
          layoutState.paraEnv.curParaPropEnv.as_ref()
            .and_then(|env| env.paraProperty.as_ref())
            .map(|pp| pt.getFinalRunPropert__entry_propertyset_boolean(&*run, pp, self.mRevisionFinal).clone())
        });
        // java if (isVanish && !GraphReaderTool.isVanishedOrRevisionDeleted(prop))
        if isVanish && !prop.as_ref().map(|p| GraphReaderTool::isVanishedOrRevisionDeleted(p)).unwrap_or(false) {
          // java return;
          return;
        }
        let Some(prop) = prop else { return };
        // java CharsNode charsNode = (CharsNode)layoutState.charsList.obtainAndAddLast();
        let charsNode: &mut CharsNode = unsafe { &mut *layoutState.charsList.obtainAndAddLast() };
        // java charsNode.type = 0;
        charsNode.r#type = 0;
        // java charsNode.finalRunProp = prop;
        charsNode.finalRunProp = Some(prop.clone());
        // java charsNode.srcRef = this.mSrcNode;
        charsNode.srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = breakCP - this.mSrcNode.cp;
        charsNode.offsetOfRef = breakCP - self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
        // java charsNode.lengthOfRef = 1;
        charsNode.lengthOfRef = 1;
        // java charsNode.isVanishBreak = isVanish;
        charsNode.isVanishBreak = isVanish;
        // java TypoPara typoPara = layoutState.typoPara;
        // java int graphIndex = typoPara.allocGraphs(1);
        let graphIndex: i32 = layoutState.typoPara.allocGraphs(1);
        // java typoPara.putChar(graphIndex, ch);
        layoutState.typoPara.putChar(graphIndex, ch);
        // java this.calcCharsNodeProp(charsNode, prop);
        self.calcCharsNodeProp(charsNode, &prop);
        // java this.addMap(breakCP, 1, graphIndex, 1);
        self.super_.addMap(breakCP, 1, graphIndex, 1);
        // java this.createNodes(charsNode, graphIndex, 1, layoutState);
        self.createNodes(charsNode, graphIndex, 1, layoutState);
        // java this.commitVanishBreak(layoutState);
        self.commitVanishBreak(layoutState);
        // java return;
        return;
      }
      // java case '\b': case '\t': case '\n': default:
      // rust: 其它字符：Java default 无操作
    }

    // java protected abstract void commitVanishBreak(LayoutState var1)
    fn commitVanishBreak(&mut self, var1: &mut LayoutState) {
      // rust: Java abstract, no body（GraphFactory 04）
      let _ = var1;
    }

    // java protected boolean collectRevision(PropertySet prop, int start, int end, LayoutState layoutState)
    fn collectRevision(&mut self, prop: &PropertySet, start: i32, end: i32, layoutState: &mut LayoutState) -> bool {
      // java boolean isRunHiden = false;
      let mut isRunHiden: bool = false;
      // java Revision revision = (Revision)prop.getValue(45);
      if let Some(revision) = Self::revision_value(prop, 45) {
        // java if (this.mRevisionFinal) isRunHiden = true;
        if self.mRevisionFinal {
          isRunHiden = true;
        }
        // java this.addRevisionNode(start, end, 1, this.mDocEnv.typoDocument.getAuthorId(revision.getAuthor()), layoutState);
        let author_name = revision.getAuthor().unwrap_or_default();
        let author = self.author_id(&author_name);
        self.addRevisionNode(start, end, 1, author, layoutState);
      }
      // java revision = (Revision)prop.getValue(46);
      if let Some(revision) = Self::revision_value(prop, 46) {
        // java if (!this.mRevisionFinal) isRunHiden = true;
        if !self.mRevisionFinal {
          isRunHiden = true;
        }
        let author_name = revision.getAuthor().unwrap_or_default();
        let author = self.author_id(&author_name);
        self.addRevisionNode(start, end, 0, author, layoutState);
      }
      // java revision = (Revision)prop.getValue(48);
      if let Some(revision) = Self::revision_value(prop, 48) {
        let author_name = revision.getAuthor().unwrap_or_default();
        let author = self.author_id(&author_name);
        self.addRevisionNode(start, end, 2, author, layoutState);
      }
      // java return isRunHiden;
      isRunHiden
    }

    // java private CharsNode readRun(IChpxTable.Entry run, int start, int end, LayoutState layoutState)
    fn readRun(&mut self, run: &dyn ChpxEntry, start: i32, end_arg: i32, layoutState: &mut LayoutState) -> Option<*mut CharsNode> {
      // java long startGetRunProp = 0L;
      // java PropertySet prop = this.mPropertyTool.getFinalRunPropert(run, layoutState.paraEnv.curParaPropEnv.paraProperty, this.mRevisionFinal);
      let mut prop: PropertySet = match self.mPropertyTool.as_mut().and_then(|pt| {
        layoutState.paraEnv.curParaPropEnv.as_ref()
          .and_then(|env| env.paraProperty.as_ref())
          .map(|pp| pt.getFinalRunPropert__entry_propertyset_boolean(run, pp, self.mRevisionFinal).clone())
      }) {
        Some(p) => p,
        None => return None,
      };
      // java boolean isRevisionHidden = false;
      let mut isRevisionHidden: bool = false;
      // java if (this.mEnableRevision)
      if self.mEnableRevision {
        // java Revision revision = (Revision)prop.getValue(45);
        if let Some(revision) = Self::revision_value(&prop, 45) {
          let view = self.super_.aliased_view();
          // java if (this.mRevisionFinal)
          if self.mRevisionFinal {
            // java if (this.mViewEnv.getShowRevisionMakeup())
            if view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false) {
              // java if (this.mViewEnv.isShowRevisionInsOrDelInBalloons())
              if view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false) {
                isRevisionHidden = true;
              } else {
                // java new MutablePropertySet(prop).add(getRevisionDeleteProperty()).freeze()
                let makeup = view.and_then(|v| v.getRevisionDeleteProperty());
                prop = Self::merge_makeup(&prop, makeup);
              }
            } else {
              isRevisionHidden = true;
            }
          } else if !view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false)
            && view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false)
          {
            let makeup = view.and_then(|v| v.getRevisionDeleteProperty());
            prop = Self::merge_makeup(&prop, makeup);
          }
          let author_name = revision.getAuthor().unwrap_or_default();
          let author = self.author_id(&author_name);
          self.addRevisionNode(start, end_arg, 1, author, layoutState);
        }
        // java revision = (Revision)prop.getValue(46);
        if let Some(revision) = Self::revision_value(&prop, 46) {
          let view = self.super_.aliased_view();
          // java if (!this.mRevisionFinal)
          if !self.mRevisionFinal {
            if view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false) {
              if view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false) {
                isRevisionHidden = true;
              } else {
                let makeup = view.and_then(|v| v.getRevisionInsertProperty());
                prop = Self::merge_makeup(&prop, makeup);
              }
            } else {
              isRevisionHidden = true;
            }
          } else if !view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false)
            && view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false)
          {
            let makeup = view.and_then(|v| v.getRevisionInsertProperty());
            prop = Self::merge_makeup(&prop, makeup);
          }
          let author_name = revision.getAuthor().unwrap_or_default();
          let author = self.author_id(&author_name);
          self.addRevisionNode(start, end_arg, 0, author, layoutState);
        }
        // java revision = (Revision)prop.getValue(48); if (revision != null && prop.contains(48))
        if let Some(revision) = Self::revision_value(&prop, 48) {
          if prop.contains(48) {
            let author_name = revision.getAuthor().unwrap_or_default();
            let author = self.author_id(&author_name);
            self.addRevisionNode(start, end_arg, 2, author, layoutState);
          }
        }
      }
      // java if (!GraphReaderTool.isVanishedText(prop) && !isRevisionHidden)
      if !GraphReaderTool::isVanishedText(&prop) && !isRevisionHidden {
        // java ArrayList<IRunPropFilter> runPropFilters = this.mRunPropFilters;
        // java for(int size = runPropFilters.size(); i < size; ++i)
        {
          let mut i: usize = 0;
          let size: usize = self.mRunPropFilters.len();
          while i < size {
            // java prop = filter.doFilter(prop, run, this.mRevisionFinal, layoutState);
            prop = self.mRunPropFilters[i].doFilter(&prop, run, self.mRevisionFinal, layoutState);
            i += 1;
          }
        }
        // java i = start - this.mSrcNode.cp;
        let i: i32 = start - self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
        // java int graphsRemain = this.getGraphsRemain(layoutState);
        let graphsRemain: i32 = self.getGraphsRemain(layoutState);
        // java int lenOfRef = Math.min(end - start, graphsRemain);
        let mut end: i32 = end_arg;
        let lenOfRef: i32 = (end - start).min(graphsRemain);
        // java end = start + lenOfRef;
        end = start + lenOfRef;
        // java this.requestReadText(start, end, layoutState);
        self.requestReadText(start, end, layoutState);
        // java this.mCurCP = end;
        self.mCurCP = end;
        // java CharsNode charsNode = (CharsNode)layoutState.charsList.obtainAndAddLast();
        let charsNode: &mut CharsNode = unsafe { &mut *layoutState.charsList.obtainAndAddLast() };
        // java charsNode.type = 0;
        charsNode.r#type = 0;
        // java charsNode.finalRunProp = prop;
        charsNode.finalRunProp = Some(prop.clone());
        // java charsNode.srcRef = this.mSrcNode;
        charsNode.srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = i;
        charsNode.offsetOfRef = i;
        // java charsNode.lengthOfRef = lenOfRef;
        charsNode.lengthOfRef = lenOfRef;
        // java charsNode.isInMath = this.mIsInMath;
        charsNode.isInMath = self.mIsInMath;
        // java this.calcCharsNodeProp(charsNode, prop);
        self.calcCharsNodeProp(charsNode, &prop);
        // java if (!this.mIsInMath && this.mEnableRevision && this.mViewEnv.isShowRevision())
        if !self.mIsInMath && self.mEnableRevision && self.super_.aliased_view().map(|v| v.isShowRevision()).unwrap_or(false) {
          // java if (null != (revision = (Revision)prop.getValue(46)))
          if let Some(revision) = Self::revision_value(&prop, 46) {
            let author_name = revision.getAuthor().unwrap_or_default();
            charsNode.revisionInsAuthor = self.author_id(&author_name);
            // java charsNode.revisionAuthor = charsNode.revisionInsAuthor;
            charsNode.revisionAuthor = charsNode.revisionInsAuthor;
          } else if let Some(revision) = Self::revision_value(&prop, 45) {
            let author_name = revision.getAuthor().unwrap_or_default();
            charsNode.revisionDelAuthor = self.author_id(&author_name);
            charsNode.revisionAuthor = charsNode.revisionDelAuthor;
          }
        }
        // java if (prop.contains(57))
        if prop.contains(57) {
          // java layoutState.hasBidiGraph = true;
          layoutState.hasBidiGraph = true;
        }
        // java return charsNode;
        return Some(charsNode as *mut CharsNode);
      }
      // java else
      else {
        // java this.mCurCP = end;
        self.mCurCP = end_arg;
        // java if (end == layoutState.document.getLength() || end >= layoutState.getRangeEnd())
        if end_arg == layoutState.document.map(|d| unsafe { (*d).getLength() }).unwrap_or(i32::MIN) || end_arg >= layoutState.getRangeEnd() {
          // java this.needReadVanishBreak = true;
          self.needReadVanishBreak = true;
        }
        // java return null;
        return None;
      }
    }

    // java private void requestReadText(int start, int end, LayoutState layoutState)
    fn requestReadText(&mut self, start: i32, end: i32, layoutState: &mut LayoutState) {
      // java if (this.mCurStart < 0)
      if self.mCurStart < 0 {
        // java this.mCurStart = start;
        self.mCurStart = start;
        // java this.mCurEnd = end;
        self.mCurEnd = end;
      }
      // java else if (this.mCurEnd == start)
      else if self.mCurEnd == start {
        // java this.mCurEnd = end;
        self.mCurEnd = end;
      }
      // java else
      else {
        // java TypoPara typoPara = layoutState.typoPara;
        // java int graphIndex = typoPara.allocGraphs(this.mCurEnd - this.mCurStart);
        let graphIndex: i32 = layoutState.typoPara.allocGraphs(self.mCurEnd - self.mCurStart);
        // java int len = this.mCurEnd - this.mCurStart;
        let len: i32 = self.mCurEnd - self.mCurStart;
        // java typoPara.putChars(layoutState.document.getTextRope(), this.mCurStart, this.mCurEnd, graphIndex);
        if let Some(rope) = layoutState.document.and_then(|d| unsafe { (*d).getTextRope() }) {
          layoutState.typoPara.putChars__textrope_int_int_int(unsafe { &mut *rope }, self.mCurStart, self.mCurEnd, graphIndex);
        }
        // java this.addMap(this.mCurStart, len, graphIndex, len);
        self.super_.addMap(self.mCurStart, len, graphIndex, len);
        // rust: ★wr-29 05 —— TypoPara mirror 映射同步（Java 存 para snapshot）
        layoutState.typoPara.putMap(self.mCurStart, len, graphIndex, len);
        // java this.mCurStart = start;
        self.mCurStart = start;
        // java this.mCurEnd = end;
        self.mCurEnd = end;
      }
    }

    // java private void commitReadText(LayoutState layoutState)
    fn commitReadText(&mut self, layoutState: &mut LayoutState) {
      // java if (this.mCurStart >= 0)
      if self.mCurStart >= 0 {
        // java TypoPara typoPara = layoutState.typoPara;
        // java int graphIndex = typoPara.allocGraphs(this.mCurEnd - this.mCurStart);
        let graphIndex: i32 = layoutState.typoPara.allocGraphs(self.mCurEnd - self.mCurStart);
        // java int len = this.mCurEnd - this.mCurStart;
        let len: i32 = self.mCurEnd - self.mCurStart;
        // java typoPara.putChars(layoutState.document.getTextRope(), this.mCurStart, this.mCurEnd, graphIndex);
        if let Some(rope) = layoutState.document.and_then(|d| unsafe { (*d).getTextRope() }) {
          layoutState.typoPara.putChars__textrope_int_int_int(unsafe { &mut *rope }, self.mCurStart, self.mCurEnd, graphIndex);
        }
        // java this.addMap(this.mCurStart, len, graphIndex, len);
        self.super_.addMap(self.mCurStart, len, graphIndex, len);
        // rust: ★wr-29 05 —— TypoPara mirror 映射同步（Java 存 para snapshot）
        layoutState.typoPara.putMap(self.mCurStart, len, graphIndex, len);
        // java this.mCurStart = -1;
        self.mCurStart = -1;
      }
    }

    // java private void calcCharsNodeProp(CharsNode charsNode, PropertySet prop)
    fn calcCharsNodeProp(&mut self, charsNode: &mut CharsNode, prop: &PropertySet) {
      // java charsNode.isSnapToGrid = prop.getBoolean(27, true);
      charsNode.isSnapToGrid = prop.getBoolean__int_boolean(27, true);
      // java charsNode.isField = this.mIsInField;
      charsNode.isField = self.mIsInField;
      // java charsNode.hyperlinkIndex = this.mHappylinkId;
      charsNode.hyperlinkIndex = self.mHappylinkId;
      // java SrcNode srcNode = charsNode.srcRef;
      // java if (srcNode != null)
      if let Some(srcRef) = charsNode.srcRef {
        // java charsNode.isBalloonAuthor = charsNode.srcRef.type == 2;
        charsNode.isBalloonAuthor = unsafe { (*srcRef).r#type } == 2;
      }
      // java GeoText geoText = (GeoText)prop.getValue(105);
      // java if (geoText != null && !geoText.isEmpty())
      // rust: GeoText 未接线（STAGE5）；hello 无 prop 105 → 跳过
      // java try { charsNode.geoText = geoText.clone(); } catch ...
      // java charsNode.hpsKern = UnitUtil.PT2TWIPS(prop.getFloat(26, 0.0F));
      charsNode.hpsKern = UnitUtil::PT2TWIPS(prop.getFloat__int_float(26, 0.0));
      // java if (!this.mIsInMath)
      if !self.mIsInMath {
        // java if (null != (revision = (Revision)prop.getValue(46)))
        if let Some(revision) = Self::revision_value(prop, 46) {
          let author_name = revision.getAuthor().unwrap_or_default();
          charsNode.revisionInsAuthor = self.author_id(&author_name);
        } else if let Some(revision) = Self::revision_value(prop, 45) {
          let author_name = revision.getAuthor().unwrap_or_default();
          charsNode.revisionDelAuthor = self.author_id(&author_name);
        }
      }
    }

    // java final void readEmbed(int cp, Shape shape, PropertySet embedProp, int fieldStart, int fieldEnd, LayoutState layoutState)
    pub(crate) fn readEmbed(&mut self, cp: i32, shape_id: Option<i32>, embedProp: &PropertySet, fieldStart: i32, fieldEnd: i32, layoutState: &mut LayoutState) {
      let mut embedProp = embedProp.clone();
      // java if (this.mEnableRevision) { revision 45/46/48; if (isRevision) return; }
      if self.revise_embed_prop(&mut embedProp, fieldStart, fieldEnd, layoutState, true, true) {
        return;
      }
      // java if (!embedProp.getBoolean(18, false))
      if embedProp.getBoolean__int_boolean(18, false) {
        return;
      }
      let chars_node = self.addCharsNode(0, 1, Some(&embedProp), &mut layoutState.charsList) as *mut CharsNode;
      let src_cp = self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
      unsafe {
        // java charsNode.srcRef = this.mSrcNode;
        (*chars_node).srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = cp - this.mSrcNode.cp;
        (*chars_node).offsetOfRef = cp - src_cp;
        // java charsNode.lengthOfRef = 1;
        (*chars_node).lengthOfRef = 1;
        // java charsNode.isInMath = this.mIsInMath;
        (*chars_node).isInMath = self.mIsInMath;
      }
      // java this.calcCharsNodeProp(charsNode, embedProp);
      self.calcCharsNodeProp(unsafe { &mut *chars_node }, &embedProp);
      // java int graphIndex = typoPara.allocGraphs(1);
      let graph_index = layoutState.typoPara.allocGraphs(1);
      // java this.addMap(cp, charsNode.lengthOfRef, graphIndex, 1);
      self.super_.addMap(cp, 1, graph_index, 1);
      // java this.createEmbedNodes(charsNode, shape, graphIndex, cp, true, layoutState);
      self.createEmbedNodes(unsafe { &mut *chars_node }, shape_id, graph_index, cp, true, layoutState);
    }

    // java protected abstract Graphs createEmbedNodes(CharsNode, Shape, int graphIndex, int cp, boolean needFindAnchorShape, LayoutState)
    fn createEmbedNodes(&mut self, chars: &mut CharsNode, shape_id: Option<i32>, graph_index: i32, cp: i32, need_anchor: bool, layout: &mut LayoutState) {
      let hook = self.factory_create_embed_nodes;
      let this = self.factory_self;
      if let Some(hook) = hook {
        hook(this, chars, shape_id, graph_index, cp, need_anchor, layout);
      }
    }

    // java final void readFieldResult(FieldLayouter.FieldResult fieldResult, int fileStartCP, int fieldEndCP, PropertySet finalRunProp, LayoutState layoutState)
    pub(crate) fn readFieldResult(&mut self, result_str: &str, field_range: i64, fileStartCP: i32, fieldEndCP: i32, finalRunProp: &PropertySet, layoutState: &mut LayoutState) {
      let mut finalRunProp = finalRunProp.clone();
      // java if (this.mEnableRevision) { revision 45/46/48; if (isRevision) return; }
      if self.revise_embed_prop(&mut finalRunProp, fileStartCP, fieldEndCP, layoutState, true, true) {
        return;
      }
      // java if (!finalRunProp.getBoolean(18, false))
      if finalRunProp.getBoolean__int_boolean(18, false) {
        return;
      }
      // java if (this.mViewEnv.getForceUseShareViewRunProperty())
      if self.super_.aliased_view().map(|view| view.getForceUseShareViewRunProperty()).unwrap_or(false) {
        let share = self.super_.aliased_view().and_then(|view| view.getShareViewProperty());
        if let Some(share) = share {
          // java String oldFont = finalRunProp.getString(3, "Times New Roman");
          let old_font = finalRunProp.getString__int_string(3, "Times New Roman");
          let mut mutable = MutablePropertySet::new_from_ps(&finalRunProp);
          // java mutableProp.add(shareViewPropert);
          mutable.add(&share);
          // java if (2 == layoutState.curPage.getType())
          if layoutState.curPage.as_ref().map(|page| page.getType()).unwrap_or(0) == 2 {
            let font_size = self.super_.aliased_view().map(|view| view.getShareViewFontsize()).unwrap_or(0.0);
            mutable.setFloat(10, font_size);
            mutable.setFloat(37, font_size);
          }
          // java if (FontName.isSpecialFontName(oldFont)) mutableProp.setString(3, oldFont);
          if FontName::isSpecialFontName(&old_font) {
            mutable.setString(3, old_font);
          }
          finalRunProp = mutable.freeze();
        }
      }
      // java offsetOfResult = Math.max(offsetOfResult, 0);
      let offset_of_result: i32 = 0;
      let chars: Vec<char> = result_str.chars().collect();
      let data_len = chars.len() as i32 - offset_of_result;
      if data_len < 0 {
        return;
      }
      let chars_node = self.addCharsNode(1, data_len, Some(&finalRunProp), &mut layoutState.charsList) as *mut CharsNode;
      let src_cp = self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
      let field_start = RANGEFactory::getStart(field_range);
      unsafe {
        // java charsNode.srcRef = this.mSrcNode;
        (*chars_node).srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = RANGEFactory.getStart(fieldResult.fieldRange) - this.mSrcNode.cp;
        (*chars_node).offsetOfRef = field_start - src_cp;
        // java charsNode.lengthOfRef = RANGEFactory.getLength(fieldResult.fieldRange);
        (*chars_node).lengthOfRef = RANGEFactory::getLength(field_range);
        // java charsNode.isField = true; charsNode.isResult = true; charsNode.isInMath = this.mIsInMath;
        (*chars_node).isField = true;
        (*chars_node).isResult = true;
        (*chars_node).isInMath = self.mIsInMath;
      }
      // java this.calcCharsNodeProp(charsNode, finalRunProp);
      self.calcCharsNodeProp(unsafe { &mut *chars_node }, &finalRunProp);
      if let Some(revision) = Self::revision_value(&finalRunProp, 46) {
        let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
        unsafe {
          (*chars_node).revisionInsAuthor = author;
          (*chars_node).revisionAuthor = author;
          (*chars_node).isRevisionInsert = true;
        }
      }
      if let Some(revision) = Self::revision_value(&finalRunProp, 45) {
        let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
        unsafe {
          (*chars_node).revisionDelAuthor = author;
          (*chars_node).revisionAuthor = author;
          (*chars_node).isRevisionDelete = true;
        }
      }
      // java int graphIndex = typoPara.allocGraphs(dataLen);
      let graph_index = layoutState.typoPara.allocGraphs(data_len);
      // java typoPara.putChars(fieldResult.resultStr.toCharArray(), offsetOfResult, dataLen, graphIndex);
      layoutState.typoPara.putChars__chararr_int_int_int(&chars, offset_of_result, data_len, graph_index);
      // java this.addMap(RANGEFactory.getStart(fieldResult.fieldRange), charsNode.lengthOfRef, graphIndex, dataLen);
      let ref_len = unsafe { (*chars_node).lengthOfRef };
      self.super_.addMap(field_start, ref_len, graph_index, data_len);
      // java this.createNodes(charsNode, graphIndex, dataLen, layoutState);
      self.createNodes(unsafe { &mut *chars_node }, graph_index, data_len, layoutState);
    }

    // java private CharsNode addCharsNode(int type, int length, PropertySet prop, KPList<CharsNode> charsList)
    fn addCharsNode<'a>(&mut self, r#type: i32, length: i32, prop: Option<&PropertySet>, charsList: &'a mut KPList<CharsNode>) -> &'a mut CharsNode {
      // java CharsNode charsNode = (CharsNode)charsList.obtainAndAddLast();
      let charsNode: &'a mut CharsNode = unsafe { &mut *charsList.obtainAndAddLast() };
      // java charsNode.type = type;
      charsNode.r#type = r#type;
      // java charsNode.finalRunProp = prop;
      charsNode.finalRunProp = prop.cloned();
      // java return charsNode;
      return charsNode;
    }

    // java private void addRevisionNode(int startCP, int endCP, int nodeType, int authorId, LayoutState layoutState)
    fn addRevisionNode(&mut self, startCP: i32, _endCP: i32, nodeType: i32, authorId: i32, layoutState: &mut LayoutState) {
      // java if (!this.mIsInTable || !this.mViewEnv.isPhoneReadMode())
      let phone = self.mIsInTable && self.super_.aliased_view().map(|view| view.isPhoneReadMode()).unwrap_or(false);
      if phone || self.mIsInMath {
        return;
      }
      // java if (layoutState.document.getType() != 3)
      let doc_type = layoutState.document.filter(|doc| !doc.is_null()).map(|doc| unsafe { (*doc).getType__on_IDocument() }).unwrap_or(0);
      if doc_type == 3 {
        return;
      }
      let Some(document) = layoutState.document.filter(|doc| !doc.is_null()) else { return; };
      let document = unsafe { &*document };
      let snap = self.super_.aliased_main().map(|state| &mut state.snapshot as *mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot).unwrap_or(std::ptr::null_mut());
      let authors = self.super_.mDocEnv.as_mut().map(|env| &mut env.typoDocument as *mut crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument).unwrap_or(std::ptr::null_mut());
      if snap.is_null() {
        return;
      }
      let item = unsafe {
        RevisionCache::get(document, startCP, nodeType, &mut (*snap).revision_slots, &mut |name| {
          if authors.is_null() { 0 } else { (*authors).getAuthorId(name) }
        })
      };
      let Some(item) = item else { return; };
      // java if (item.start == startCP)
      if item.start == startCP {
        let node = layoutState.revisions.obtainAndAddLast();
        unsafe {
          (*node).r#type = nodeType;
          (*node).start = item.start;
          (*node).authorId = authorId;
        }
      } else if self.mIsInField {
        let last = layoutState.revisions.lastElement();
        if !last.is_null() {
          let same = unsafe { (*last).r#type == nodeType && (*last).start == item.start && (*last).authorId == authorId };
          if same {
            return;
          }
        }
        let node = layoutState.revisions.obtainAndAddLast();
        unsafe {
          (*node).r#type = nodeType;
          (*node).start = item.start;
          (*node).authorId = authorId;
        }
      }
    }

    fn final_run_prop(&mut self, run: &dyn ChpxEntry, layoutState: &LayoutState) -> Option<PropertySet> {
      let para = layoutState.paraEnv.curParaPropEnv.as_ref().and_then(|env| env.paraProperty.as_ref())?;
      let tool = self.mPropertyTool.as_mut()?;
      Some(tool.getFinalRunPropert__entry_propertyset_boolean(run, para, self.mRevisionFinal).clone())
    }

    // java final boolean readEndNote(int cp, PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endNoteRefNode, LayoutState layoutState)
    pub(crate) fn readEndNote(&mut self, cp: i32, endNoteRefNode: Node, layoutState: &mut LayoutState) -> bool {
      // java IChpxTable.Entry run = layoutState.document.getChpxTable().seek(cp);
      let run = layoutState.document.filter(|doc| !doc.is_null()).and_then(|doc| unsafe { (*doc).getChpxTable() }).filter(|table| !table.is_null()).and_then(|table| unsafe { IChpxTable::seek(&*table, cp) });
      let Some(run) = run else {
        self.mCurCP = cp + 1;
        return true;
      };
      let Some(final_run_prop) = self.final_run_prop(run.as_ref(), layoutState) else {
        self.mCurCP = cp + 1;
        return true;
      };
      // java this.mCurCP = cp + 1;
      self.mCurCP = cp + 1;
      // java if (this.readFootEndNoteRevision(cp, finalRunProp, layoutState)) return true;
      if self.readFootEndNoteRevision(cp, &final_run_prop, layoutState) {
        return true;
      }
      // java if (finalRunProp.getBoolean(18, false)) return true;
      if final_run_prop.getBoolean__int_boolean(18, false) {
        return true;
      }
      // java String endNoteStr = this.getEndNote(cp, endNoteRefNode);
      let end_node = FootnoteEndnoteRefNode::fromNode(endNoteRefNode);
      let Some(end_note_str) = self.getEndNote(cp, &end_node) else { return true; };
      // java this.setFootEndNoteRefType(9); this.readFootEndNoteStr(...); return true;
      self.setFootEndNoteRefType(9);
      self.readFootEndNoteStr(cp, &end_note_str, &final_run_prop, layoutState);
      true
    }

    // java protected abstract void setFootEndNoteRefType(int var1)
    fn setFootEndNoteRefType(&mut self, var1: i32) {
      // rust: Java abstract, no body（STAGE6）
      let _ = var1;
    }

    // java final boolean readFootNote(int cp, PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footNoteRefNode, LayoutState layoutState)
    pub(crate) fn readFootNote(&mut self, cp: i32, footNoteRefNode: Node, layoutState: &mut LayoutState) -> bool {
      let run = layoutState.document.filter(|doc| !doc.is_null()).and_then(|doc| unsafe { (*doc).getChpxTable() }).filter(|table| !table.is_null()).and_then(|table| unsafe { IChpxTable::seek(&*table, cp) });
      let Some(run) = run else {
        self.mCurCP = cp + 1;
        return true;
      };
      let Some(final_run_prop) = self.final_run_prop(run.as_ref(), layoutState) else {
        self.mCurCP = cp + 1;
        return true;
      };
      // java this.mCurCP = cp + 1;
      self.mCurCP = cp + 1;
      if self.readFootEndNoteRevision(cp, &final_run_prop, layoutState) {
        return true;
      }
      // java else if (finalRunProp.getBoolean(18, false)) return true;
      if final_run_prop.getBoolean__int_boolean(18, false) {
        return true;
      }
      // java FootNoteCacheItem footNote = this.getFootNote(cp, footNoteRefNode);
      let foot_node = FootnoteEndnoteRefNode::fromNode(footNoteRefNode);
      let Some(num_str) = self.getFootNote(cp, &foot_node) else { return true; };
      // java this.setFootEndNoteRefType(8); this.readFootEndNoteStr(cp, numStr, finalRunProp, layoutState); return true;
      self.setFootEndNoteRefType(8);
      self.readFootEndNoteStr(cp, &num_str, &final_run_prop, layoutState);
      true
    }

    fn revision_value(prop: &PropertySet, id: i32) -> Option<Revision> {
      prop.getValue(id).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>().cloned())
    }

    // java readEmbed / readFieldResult / readMath share the makeup branches.
    // `record` writes addRevisionNode. `format_rev` also reads property 48.
    // Returns true when Java sets isRevision and the caller returns.
    fn revise_embed_prop(&mut self, prop: &mut PropertySet, start: i32, end: i32, layout: &mut LayoutState, record: bool, format_rev: bool) -> bool {
      if !self.mEnableRevision {
        return false;
      }
      let mut is_revision = false;
      if let Some(revision) = Self::revision_value(prop, 45) {
        let view = self.super_.aliased_view();
        if self.mRevisionFinal {
          if view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false) {
            if view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false) {
              is_revision = true;
            } else {
              let makeup = view.and_then(|v| v.getRevisionDeleteProperty());
              *prop = Self::merge_makeup(prop, makeup);
            }
          } else {
            is_revision = true;
          }
        } else if !view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false)
          && view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false)
        {
          let makeup = view.and_then(|v| v.getRevisionDeleteProperty());
          *prop = Self::merge_makeup(prop, makeup);
        }
        if record {
          let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
          self.addRevisionNode(start, end, 1, author, layout);
        }
      }
      if let Some(revision) = Self::revision_value(prop, 46) {
        let view = self.super_.aliased_view();
        if !self.mRevisionFinal {
          if view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false) {
            if view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false) {
              is_revision = true;
            } else {
              let makeup = view.and_then(|v| v.getRevisionInsertProperty());
              *prop = Self::merge_makeup(prop, makeup);
            }
          } else {
            is_revision = true;
          }
        } else if !view.map(|v| v.isShowRevisionInsOrDelInBalloons()).unwrap_or(false)
          && view.map(|v| v.getShowRevisionMakeup()).unwrap_or(false)
        {
          let makeup = view.and_then(|v| v.getRevisionInsertProperty());
          *prop = Self::merge_makeup(prop, makeup);
        }
        if record {
          let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
          self.addRevisionNode(start, end, 0, author, layout);
        }
      }
      if format_rev {
        if let Some(revision) = Self::revision_value(prop, 48) {
          if record {
            let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
            self.addRevisionNode(start, end, 2, author, layout);
          }
        }
      }
      is_revision
    }

    // java new MutablePropertySet(prop).add(makeup).freeze()
    fn merge_makeup(prop: &PropertySet, makeup: Option<&PropertySet>) -> PropertySet {
      let mut map = MutablePropertySet::new_from_ps(prop);
      if let Some(makeup) = makeup {
        map.add(makeup);
      }
      map.freeze()
    }

    fn author_id(&mut self, author: &str) -> i32 {
      self.super_.mDocEnv.as_mut().map(|env| env.typoDocument.getAuthorId(author)).unwrap_or(0)
    }

    // java private boolean readFootEndNoteRevision(int cp, PropertySet finalRunProp, LayoutState layoutState)
    fn readFootEndNoteRevision(&mut self, cp: i32, finalRunProp: &PropertySet, layoutState: &mut LayoutState) -> bool {
      // java if (this.mEnableRevision)
      if !self.mEnableRevision {
        return false;
      }
      let mut is_revision = false;
      if let Some(revision) = Self::revision_value(finalRunProp, 45) {
        if self.mRevisionFinal {
          is_revision = true;
        }
        let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
        self.addRevisionNode(cp, cp + 1, 1, author, layoutState);
      }
      if let Some(revision) = Self::revision_value(finalRunProp, 46) {
        if !self.mRevisionFinal {
          is_revision = true;
        }
        let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
        self.addRevisionNode(cp, cp + 1, 0, author, layoutState);
      }
      if let Some(revision) = Self::revision_value(finalRunProp, 48) {
        let author = self.author_id(revision.getAuthor().as_deref().unwrap_or(""));
        self.addRevisionNode(cp, cp + 1, 2, author, layoutState);
      }
      is_revision
    }

    // java private void readFootEndNoteStr(int cp, String numStr, PropertySet finalRunProp, LayoutState layoutState)
    fn readFootEndNoteStr(&mut self, cp: i32, numStr: &str, finalRunProp: &PropertySet, layoutState: &mut LayoutState) {
      // java int length = numStr.length();
      let chars: Vec<char> = numStr.chars().collect();
      let len: i32 = chars.len() as i32;
      // java CharsNode charsNode = this.addCharsNode(1, numStr.length(), finalRunProp, layoutState.charsList);
      let chars_ptr: *mut CharsNode = {
        let charsNode = self.addCharsNode(1, len, Some(finalRunProp), &mut layoutState.charsList);
        // java charsNode.srcRef = this.mSrcNode;
        charsNode.srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = cp - this.mSrcNode.cp;
        charsNode.offsetOfRef = cp - self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
        // java charsNode.lengthOfRef = 1;
        charsNode.lengthOfRef = 1;
        charsNode as *mut CharsNode
      };
      let charsNode = unsafe { &mut *chars_ptr };
      // java this.calcCharsNodeProp(charsNode, finalRunProp);
      self.calcCharsNodeProp(charsNode, finalRunProp);
      // java charsNode.isFootEndNote = true;
      charsNode.isFootEndNote = true;
      // java charsNode.isResult = true;
      charsNode.isResult = true;
      // java charsNode.isInMath = this.mIsInMath;
      charsNode.isInMath = self.mIsInMath;
      // java if (numStr.length() > 0)
      if len > 0 {
        // java int graphIndex = typoPara.allocGraphs(numStr.length());
        let graphIndex: i32 = layoutState.typoPara.allocGraphs(len);
        // java typoPara.putChars(numStr.toCharArray(), 0, numStr.length(), graphIndex);
        layoutState.typoPara.putChars__chararr_int_int_int(&chars, 0, len, graphIndex);
        // java this.addMap(cp, 1, graphIndex, numStr.length());
        self.super_.addMap(cp, 1, graphIndex, len);
        // java this.createNodes(charsNode, graphIndex, numStr.length(), layoutState);
        self.createNodes(charsNode, graphIndex, len, layoutState);
        // java if (cp == layoutState.paraEnv.curParaPropEnv.end - 1)
        if layoutState.paraEnv.curParaPropEnv.as_ref().map(|env| cp == env.end - 1).unwrap_or(false) {
          layoutState.hasReadEnd = true;
        }
      }
    }

    // java final boolean readFootEndNoteRef(int cp, LayoutState layoutState)
    pub(crate) fn readFootEndNoteRef(&mut self, cp: i32, layoutState: &mut LayoutState) -> bool {
      let run = layoutState.document.filter(|doc| !doc.is_null()).and_then(|doc| unsafe { (*doc).getChpxTable() }).filter(|table| !table.is_null()).and_then(|table| unsafe { IChpxTable::seek(&*table, cp) });
      let Some(run) = run else { return true; };
      let Some(final_run_prop) = self.final_run_prop(run.as_ref(), layoutState) else { return true; };
      // java if (this.readFootEndNoteRevision(cp, finalRunProp, layoutState)) return true;
      if self.readFootEndNoteRevision(cp, &final_run_prop, layoutState) {
        return true;
      }
      // java else if (finalRunProp.getBoolean(18, false)) return true;
      if final_run_prop.getBoolean__int_boolean(18, false) {
        return true;
      }
      // java String numStr = this.getNumStrForFootEndDocument(cp, layoutState);
      if let Some(num_str) = self.getNumStrForFootEndDocument(cp, layoutState) {
        if !num_str.is_empty() {
          self.readFootEndNoteStr(cp, &num_str, &final_run_prop, layoutState);
          return true;
        }
      }
      true
    }

    // java private String getNumStrForFootEndDocument(int cp, LayoutState layoutState)
    fn getNumStrForFootEndDocument(&mut self, cp: i32, layoutState: &mut LayoutState) -> Option<String> {
      // java return null != footEndNoteNumLayouter ? footEndNoteNumLayouter.getNumByCP(layoutState.document, cp) : null;
      let doc_ptr = layoutState.document.filter(|p| !p.is_null())?;
      let doc = self.super_.aliased_doc()?;
      let layouter = doc.footEndNoteNumLayouter.as_mut()?;
      layouter.getNumByCP(unsafe { &*doc_ptr }, cp)
    }

    // java private String getEndNote(int cp, PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode endNoteRefNode)
    fn getEndNote(&mut self, _cp: i32, end_note_ref_node: &FootnoteEndnoteRefNode) -> Option<String> {
      // java return null != footEndNoteNumLayouter ? footEndNoteNumLayouter.getEndNoteNum(endNoteRefNode) : null;
      let doc = self.super_.aliased_doc()?;
      let layouter = doc.footEndNoteNumLayouter.as_mut()?;
      Some(layouter.getEndNoteNum(end_note_ref_node))
    }

    // java private FootNoteCacheItem getFootNote(int cp, PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footNoteRefNode)
    fn getFootNote(&mut self, cp: i32, foot_note_ref_node: &FootnoteEndnoteRefNode) -> Option<String> {
      // java if (null != footEndNoteNumLayouter) numStr = footEndNoteNumLayouter.getFootNoteNum(footNoteRefNode);
      let num = if let Some(doc) = self.super_.aliased_doc() {
        doc.footEndNoteNumLayouter.as_mut().map(|layouter| layouter.getFootNoteNum(foot_note_ref_node))
      } else {
        None
      };
      // java return null == this.mMainState.footEndNoteCache ? null : cache.addFootEndNoteCache(cp, node, numStr).numStr
      let main = self.super_.aliased_main()?;
      let cache = main.footEndNoteCache.as_mut()?;
      cache.addFootEndNoteCache(cp, foot_note_ref_node.clone(), num)
    }

    // java public void setIgnoreHpsAndVAlignRun(boolean val)
    pub fn setIgnoreHpsAndVAlignRun(&mut self, val: bool) {
      // java ArrayList<IRunPropFilter> runPropFilters = this.mRunPropFilters;
      // java if (val)
      if val {
        // java for(int size = runPropFilters.size(); i < size; ++i)
        let mut i: usize = 0;
        let size: usize = self.mRunPropFilters.len();
        while i < size {
          // java if (filter instanceof IgnoreHpsAndVAlignRunFilter)
          if self.mRunPropFilters[i].isIgnoreHpsAndVAlignRunFilter() {
            // java return;
            return;
          }
          i += 1;
        }
        // java runPropFilters.add(new IgnoreHpsAndVAlignRunFilter());
        self.mRunPropFilters.push(Box::new(IgnoreHpsAndVAlignRunFilter {}));
      }
      // java else
      else {
        // java for(int size = runPropFilters.size(); i < size; ++i)
        let mut i: usize = 0;
        let size: usize = self.mRunPropFilters.len();
        while i < size {
          // java if (filter instanceof IgnoreHpsAndVAlignRunFilter)
          if self.mRunPropFilters[i].isIgnoreHpsAndVAlignRunFilter() {
            // java runPropFilters.remove(i);
            self.mRunPropFilters.remove(i);
            // java return;
            return;
          }
          i += 1;
        }
      }
    }

    // java public void setIgnoreCapsAndSmallCaps(boolean val)
    pub fn setIgnoreCapsAndSmallCaps(&mut self, val: bool) {
      // java ArrayList<IRunPropFilter> runPropFilters = this.mRunPropFilters;
      // java if (val)
      if val {
        // java for(int size = runPropFilters.size(); i < size; ++i)
        let mut i: usize = 0;
        let size: usize = self.mRunPropFilters.len();
        while i < size {
          // java if (filter instanceof IgnoreCapsAndSmallCapsFilter)
          if self.mRunPropFilters[i].isIgnoreCapsAndSmallCapsFilter() {
            // java return;
            return;
          }
          i += 1;
        }
        // java runPropFilters.add(new IgnoreCapsAndSmallCapsFilter());
        self.mRunPropFilters.push(Box::new(IgnoreCapsAndSmallCapsFilter {}));
      }
      // java else
      else {
        // java for(int size = runPropFilters.size(); i < size; ++i)
        let mut i: usize = 0;
        let size: usize = self.mRunPropFilters.len();
        while i < size {
          // java if (filter instanceof IgnoreCapsAndSmallCapsFilter)
          if self.mRunPropFilters[i].isIgnoreCapsAndSmallCapsFilter() {
            // java runPropFilters.remove(i);
            self.mRunPropFilters.remove(i);
            // java return;
            return;
          }
          i += 1;
        }
      }
    }

    // java public final void initFilter(Document document, int start, int end, LayoutState layoutState)
    pub fn initFilter(&mut self, document: Option<*mut dyn Document>, start: i32, end: i32, layoutState: &mut LayoutState) {

      // java this.mFilters.clear();
      self.mFilters.clear();
      // java this.mRunPropFilters.clear();
      self.mRunPropFilters.clear();
      // java int viewMode = this.mViewEnv.getViewMode();
      let viewMode: i32 = self.super_.aliased_view().map(|v| v.getViewMode()).unwrap_or(0);
      // java int docType = document.getType();
      let docType: i32 = document.map(|d| unsafe { (*d).getType__on_IDocument() }).unwrap_or(0);
      // java int maxCp = layoutState.getRangeEnd();
      let maxCp: i32 = layoutState.getRangeEnd();
      // java if (0 != viewMode && (1 == docType || 4 == docType) || 3 == docType || layoutState.isRevision)
      if (0 != viewMode && (1 == docType || 4 == docType)) || 3 == docType || layoutState.isRevision {
        // java this.mFilters.add(new PhoneViewBalloonDrawingFilter(this, layoutState, start, maxCp, this.mViewEnv));
        // rust: DEFERRED —— PhoneView 气球过滤非 hello（GR-D3）
        let reader = self as *mut GraphReader;
        let view = self.super_.view_alias;
        self.mFilters.push(Box::new(PhoneViewBalloonDrawingFilter::new(
          reader,
          layoutState.isRevision,
          layoutState.isInEmbedBalloonMode,
          document,
          start,
          maxCp,
          view,
        )));
      }
      // java if (0 != viewMode && layoutState.needFlowOMathInDrawing())
      if 0 != viewMode && layoutState.needFlowOMathInDrawing() {
        // java PLC.Node mathNode = PhoneViewOMathFilter.getNextOMathNode(document, start, maxCp);
        if let Some(doc) = document.filter(|p| !p.is_null()) {
          if let Some(math_node) = PhoneViewOMathFilter::getNextOMathNode(unsafe { &*doc }, start, maxCp) {
            let reader = self as *mut GraphReader;
            // java this.mFilters.add(new PhoneViewOMathFilter(mathNode, start, maxCp, this));
            self.mFilters.push(Box::new(PhoneViewOMathFilter::new(math_node, maxCp, reader)));
          }
        }
      }
      // java else
      else {
        // java PLC.Node mathNode = OMathFilter.getNextOMathNode(document, start, maxCp);
        if let Some(doc) = document.filter(|p| !p.is_null()) {
          if let Some(edge) = OMathFilter::getNextOMathNode(unsafe { &*doc }, start, maxCp) {
            let reader = self as *mut GraphReader;
            self.mFilters.push(Box::new(OMathFilter::new(edge, maxCp, reader)));
          }
        }
        // java else if (layoutState.isMathContentState())
        else if layoutState.isMathContentState() {
          // java this.mIsInMath = true;
          self.mIsInMath = true;
        }
        if let Some(doc) = document.filter(|p| !p.is_null()) {
          if let Some(plc) = unsafe { (*doc).getPLCMathFunction() }.filter(|p| !p.is_null()) {
            let plc = unsafe { &*plc };
            let head = plc.super_.super_.getHead().as_node();
            let mut math_node = plc.super_.super_.findUpper(start);
            while !math_node.ptr.is_null() && !nodes_same(math_node, head) && !is_math_start(math_node) {
              math_node = math_node.getNext();
            }
            if !math_node.ptr.is_null() && !nodes_same(math_node, head) && math_node.getCP() < maxCp {
              let reader = self as *mut GraphReader;
              self.mFilters.push(Box::new(MathFilter::new(doc, math_node, start, maxCp, reader)));
            }
          }
        }
      }
      // java PLCField plcField = document.getPLCField();
      if let Some(doc) = document.filter(|p| !p.is_null()) {
        if let Some(plc) = unsafe { (*doc).getPLCField() }.filter(|p| !p.is_null()) {
          if !unsafe { (*plc).isEmpty() } {
            let reader = self as *mut GraphReader;
            if let Some(filter) = FieldFilter::create(doc, start, end, reader) {
              self.mFilters.push(Box::new(filter));
            }
          }
        }
      }
      // java if (0 == docType)
      if 0 == docType {
        if let Some(doc) = document.filter(|p| !p.is_null()) {
          // java PLCFootnoteEndnoteRef footNoteRef = document.getFootnoteRef();
          if let Some(plc) = unsafe { (*doc).getFootnoteRef() }.filter(|p| !p.is_null()) {
            let plc = unsafe { &*plc };
            let head = plc.getHead().as_node();
            let foot_node = plc.findUpper(start);
            if !foot_node.ptr.is_null() && !nodes_same(foot_node, head) && foot_node.getCP() < maxCp {
              let reader = self as *mut GraphReader;
              self.mFilters.push(Box::new(FootEndNoteFilter::new(foot_node, start, maxCp, true, reader)));
            }
          }
          // java PLCFootnoteEndnoteRef endNoteRef = document.getEndnoteRef();
          if let Some(plc) = unsafe { (*doc).getEndnoteRef() }.filter(|p| !p.is_null()) {
            let plc = unsafe { &*plc };
            let head = plc.getHead().as_node();
            let end_node = plc.findUpper(start);
            if !end_node.ptr.is_null() && !nodes_same(end_node, head) && end_node.getCP() < maxCp {
              let reader = self as *mut GraphReader;
              self.mFilters.push(Box::new(FootEndNoteFilter::new(end_node, start, maxCp, false, reader)));
            }
          }
        }
      }
      // java else if (1 == docType || 4 == docType)
      else if 1 == docType || 4 == docType {
        // java this.mFilters.add(new FootEndRefCharFilter(start, maxCp, this, layoutState));
        let reader = self as *mut GraphReader;
        self.mFilters.push(Box::new(FootEndRefCharFilter::new(start, maxCp, reader, layoutState)));
      }
      // java if (this.mViewEnv.getForceUseShareViewRunProperty())
      if self.super_.aliased_view().map(|v| v.getForceUseShareViewRunProperty()).unwrap_or(false) {
        // java this.mRunPropFilters.add(new ShareViewRunPropFilter(this.mViewEnv));
        if let Some(view) = self.super_.aliased_view() {
          self.mRunPropFilters.push(Box::new(ShareViewRunPropFilter::new(view)));
        }
      }
    }

    // java public void setIsInMath(boolean val, LayoutState layoutState)
    pub fn setIsInMath(&mut self, val: bool, layoutState: &mut LayoutState) {
      // java this.mIsInMath = val;
      self.mIsInMath = val;
      // java if (val)
      if val {
        // java layoutState.hasMaths = true;
        layoutState.hasMaths = true;
      }
    }

    // java private ISymbolConvert getConvertFont(String string)
    fn getConvertFont(&mut self, string: &str) -> Option<()> {
      // java if (null != string && string.length() > 0)
      if !string.is_empty() {
        // java switch (string.charAt(0))
        match string.chars().next().map(|c| c.to_ascii_lowercase()) {
          // java case 'K': case 'k':
          Some('k') => {
            // java if ("Kingsoft Confetti".equalsIgnoreCase(string)) { return this.mDocEnv.layoutToolsManager.getKingsoftConfettiConvert(); }
            // rust: DEFERRED —— 符号字体（Kingsoft 系列）非 hello（GR-D3）
            /* DEFERRED_NON_FUNCTION Kingsoft 符号字体转换 */
            // java break;
          }
          // java case 'M': case 'm':
          Some('m') => {
            // java if ("MT Extra".equalsIgnoreCase(string)) { return this.mDocEnv.layoutToolsManager.getExtraConvert(); }
            // rust: DEFERRED —— 符号字体（MT Extra）非 hello（GR-D3）
            /* DEFERRED_NON_FUNCTION MT Extra 符号字体转换 */
            // java break;
          }
          // java case 'S': case 's':
          Some('s') => {
            // java if ("Symbol".equalsIgnoreCase(string) || "WPS Special 3".equalsIgnoreCase(string)) { return this.mDocEnv.layoutToolsManager.getSymbolConvert(); }
            // java if ("WPS Special 1".equalsIgnoreCase(string)) { return this.mDocEnv.layoutToolsManager.getWingdingsConvert(); }
            // java if ("WPS Special 2".equalsIgnoreCase(string)) { return this.mDocEnv.layoutToolsManager.getExtraConvert(); }
            // rust: DEFERRED —— 符号字体（Symbol 系列）非 hello（GR-D3）
            /* DEFERRED_NON_FUNCTION Symbol 系列符号字体转换 */
            // java break;
          }
          // java case 'W': case 'w':
          Some('w') => {
            // java if ("Wingdings".equalsIgnoreCase(string)) { return this.mDocEnv.layoutToolsManager.getWingdingsConvert(); }
            // rust: DEFERRED —— 符号字体（Wingdings）非 hello（GR-D3）
            /* DEFERRED_NON_FUNCTION Wingdings 符号字体转换 */
          }
          _ => {
            // java default:
          }
        }
      }
      // java return null;
      return None;
    }

    // java private void readSymbolCharNode(CharsNode charsNode, int graphIndex, char ch, String fontSymbol, LayoutState layoutState)
    fn readSymbolCharNode(&mut self, charsNode: &mut CharsNode, graphIndex: i32, ch: char, fontSymbol: String, layoutState: &mut LayoutState) {
      // rust: DEFERRED —— 符号字体路径非 hello（GR-D3），Java 原文见 GraphReader.java:1397-1446
      let _ = (charsNode, graphIndex, ch, fontSymbol, layoutState);
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = graphIndex + charsNode.lengthOfRef;
      // java TypoPara typoPara = layoutState.typoPara;
      // java int curGraphIndex = graphIndex + 1;
      // java int curGraphStart = graphIndex;
      // java int curGraphLen = 1;
      // java int[] charsBuff = typoPara.getCharBlock(graphIndex, this.mBlockResult);
      // java int buffOffset = this.mBlockResult[0];
      // java int buffLimit = this.mBlockResult[1];
      // java for(curIsSymbol = charsBuff[buffOffset++] == 40; curGraphIndex < max; buffOffset += processThisTime) { ... }
      // java if (curGraphLen > 0) { if (curIsSymbol) { this.createSymbolNodes(...); } else { this.createNodes(...); } }
      */
    }

    // java protected abstract void createSymbolNodes(CharsNode var1, int var2, int var3, char var4, String var5, LayoutState var6)
    fn createSymbolNodes(&mut self, var1: &mut CharsNode, var2: i32, var3: i32, var4: char, var5: String, var6: &mut LayoutState) {
      // rust: Java abstract, no body（符号字体 STAGE）
      let _ = (var1, var2, var3, var4, var5, var6);
    }

    // java public void addFunctionSepField(int sepCP)
    pub fn addFunctionSepField(&mut self, sepCP: i32) {
      // java this.mFilters.add(new FunctionSepField(sepCP, this));
      let reader = self as *mut GraphReader;
      self.mFilters.push(Box::new(FunctionSepField::new(sepCP, reader)));
    }

    // java public void readMathChar(char ch, int cp, LayoutState layoutState)
    pub fn readMathChar(&mut self, ch: char, cp: i32, layoutState: &mut LayoutState) {
      // java SrcNode srcNode = this.mSrcNode;
      let srcNode = match self.mSrcNode { Some(p) => p, None => return };
      // java Document mDocument = srcNode.doc;
      let mDocument = match unsafe { (*srcNode).doc } { Some(d) => d, None => return };
      // java this.mPropertyTool = this.mDocEnv.propToolFactory.create(mDocument);
      self.mPropertyTool = self.super_.aliased_doc()
        .and_then(|d| d.propToolFactory.as_mut())
        .and_then(|f| { let b: Box<Box<dyn IPropertyTool>> = f.create(unsafe { &*mDocument }); Some(*b) });
      // java this.mRevisionFinal = this.mSrcNode.revisionFinal;
      self.mRevisionFinal = unsafe { (*srcNode).revisionFinal };
      // java this.mIsInField = false;
      self.mIsInField = false;
      // java this.mHappylinkId = -1;
      self.mHappylinkId = -1;
      // java this.mCurCP = cp;
      self.mCurCP = cp;
      // java IChpxTable.Entry run = mDocument.getChpxTable().seek(cp);
      let run = unsafe { (*mDocument).getChpxTable().and_then(|t| IChpxTable::seek(&*t, cp)) };
      let Some(run) = run else { return };
      // java PropertySet finalRunProp = this.mPropertyTool.getFinalRunPropert(run, layoutState.paraEnv.curParaPropEnv.paraProperty, this.mRevisionFinal);
      let mut finalRunProp: Option<PropertySet> = self.mPropertyTool.as_mut().and_then(|pt| {
        layoutState.paraEnv.curParaPropEnv.as_ref()
          .and_then(|env| env.paraProperty.as_ref())
          .map(|pp| pt.getFinalRunPropert__entry_propertyset_boolean(&*run, pp, self.mRevisionFinal).clone())
      });
      // java for(int size = runPropFilters.size(); i < size; ++i)
      {
        let mut i: usize = 0;
        let size: usize = self.mRunPropFilters.len();
        while i < size {
          // java finalRunProp = filter.doFilter(finalRunProp, run, this.mRevisionFinal, layoutState);
          if let Some(prop) = finalRunProp.take() {
            finalRunProp = Some(self.mRunPropFilters[i].doFilter(&prop, &*run, self.mRevisionFinal, layoutState));
          }
          i += 1;
        }
      }
      // java srcNode.finalRunProp = finalRunProp;
      unsafe { (*srcNode).finalRunProp = finalRunProp.clone(); }
      // java PLCField plcField = mDocument.getPLCField();
      let plcField = unsafe { (*mDocument).getPLCField() };
      // java if (null != plcField && !plcField.isEmpty())
      if let Some(plcField) = plcField {
        if !unsafe { (*plcField).super_.super_.isEmpty() } {
          // java this.mFilterArgs.start = this.mCurCP;
          self.mFilterArgs.start = self.mCurCP;
          // java this.mFilterArgs.end = this.mCurCP + 1;
          self.mFilterArgs.end = self.mCurCP + 1;
          // java FieldFilter filter = FieldFilter.create(mDocument, this.mFilterArgs.start, this.mFilterArgs.end, this);
          let reader = self as *mut GraphReader;
          if let Some(mut filter) = FieldFilter::create(mDocument, self.mFilterArgs.start, self.mFilterArgs.end, reader) {
            // java filter.doFilter(this.mFilterArgs, layoutState);
            filter.doFilter(&mut self.mFilterArgs, layoutState);
          }
        }
      }
      let src_len = unsafe { (*srcNode).length };
      let src_cp = unsafe { (*srcNode).cp };
      // java CharsNode charsNode = this.addCharsNode(0, srcNode.length, finalRunProp, layoutState.charsList);
      let charsNode: *mut CharsNode = self.addCharsNode(0, src_len, finalRunProp.as_ref(), &mut layoutState.charsList) as *mut CharsNode;
      // java charsNode.srcRef = srcNode;
      unsafe { (*charsNode).srcRef = self.mSrcNode; }
      // java charsNode.offsetOfRef = cp - srcNode.cp;
      unsafe { (*charsNode).offsetOfRef = cp - src_cp; }
      // java charsNode.lengthOfRef = 1;
      unsafe { (*charsNode).lengthOfRef = 1; }
      // java charsNode.isInMath = true;
      unsafe { (*charsNode).isInMath = true; }
      // java this.calcCharsNodeProp(charsNode, finalRunProp);
      if let Some(prop) = finalRunProp.as_ref() {
        self.calcCharsNodeProp(unsafe { &mut *charsNode }, prop);
      }
      // java int graphIndex = typoPara.allocGraphs(charsNode.lengthOfRef);
      let graphIndex = layoutState.typoPara.allocGraphs(unsafe { (*charsNode).lengthOfRef });
      // java typoPara.putChar(graphIndex, ch);
      layoutState.typoPara.putChar(graphIndex, ch);
      // java this.createNodes(charsNode, graphIndex, charsNode.lengthOfRef, layoutState);
      self.createNodes(unsafe { &mut *charsNode }, graphIndex, unsafe { (*charsNode).lengthOfRef }, layoutState);
      // java this.commitGraphNode(layoutState);
      self.commitGraphNode(layoutState);
    }

    // java final void readMath(PLC.Node mathNode, int endCP, LayoutState layoutState)
    pub(crate) fn readMath(&mut self, math_node: &MathStartNode, end_cp: i32, layout_state: &mut LayoutState) {
      let cp = math_node.getCP();
      let embed = MathEmbed::Function {
        math_type: math_node.getType(),
        picture_shape_id: math_node.getPicture().map(|shape| shape.getShapeID()),
        start_cp: cp,
        end_cp,
      };
      // java addCharsNode(mathNode instanceof MathStartNode ? 3 : 2, ...)
      self.read_math_at(3, embed, cp, end_cp, layout_state);
    }

    // java PhoneViewOMathFilter calls the same readMath with an OMathStartNode.
    pub(crate) fn readOMath(&mut self, start: &OMathStartNode, end_cp: i32, layout_state: &mut LayoutState) {
      let cp = start.getCP();
      let embed = MathEmbed::OMath { start_cp: cp, end_cp, in_para: start.isInOMathPara() };
      self.read_math_at(2, embed, cp, end_cp, layout_state);
    }

    fn read_math_at(&mut self, node_type: i32, embed: MathEmbed, cp: i32, end_cp: i32, layout_state: &mut LayoutState) {
      let Some(document) = layout_state.document.filter(|p| !p.is_null()) else { return };
      let Some(embed_run) = (unsafe { (*document).getChpxTable().and_then(|t| IChpxTable::seek(&*t, cp)) }) else { return };
      let embed_prop = self.mPropertyTool.as_mut().and_then(|pt| {
        layout_state.paraEnv.curParaPropEnv.as_ref()
          .and_then(|env| env.paraProperty.as_ref())
          .map(|pp| pt.getFinalRunPropert__entry_propertyset_boolean(&*embed_run, pp, self.mRevisionFinal).clone())
      });
      let Some(mut embed_prop) = embed_prop else { return };
      // java readMath revision makeup does not call addRevisionNode and does not read property 48.
      if self.revise_embed_prop(&mut embed_prop, cp, end_cp, layout_state, false, false) {
        return;
      }
      if embed_prop.getBoolean__int_boolean(18, false) {
        return;
      }
      let chars_node = self.addCharsNode(node_type, 1, Some(&embed_prop), &mut layout_state.charsList) as *mut CharsNode;
      let src_cp = self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
      unsafe {
        (*chars_node).srcRef = self.mSrcNode;
        (*chars_node).offsetOfRef = cp - src_cp;
        (*chars_node).lengthOfRef = end_cp - cp;
        (*chars_node).isInMath = true;
      }
      self.calcCharsNodeProp(unsafe { &mut *chars_node }, &embed_prop);
      let graph_index = layout_state.typoPara.allocGraphs(1);
      self.super_.addMap(cp, unsafe { (*chars_node).lengthOfRef }, graph_index, 1);
      layout_state.typoPara.putChar(graph_index, ' ');
      let hook = self.factory_create_math_nodes;
      let this = self.factory_self;
      if let Some(hook) = hook {
        hook(this, unsafe { &mut *chars_node }, graph_index, embed, layout_state);
      }
    }

    // java final void readEquation(PLCField.FieldTriple mFieldTriple, LayoutState layoutState)
    pub(crate) fn readEquation(&mut self, triple: &FieldTriple, layoutState: &mut LayoutState) {
      // java int start = mFieldTriple.start.getCP();
      let start = triple.getStartCP();
      // java int end = mFieldTriple.getEnd();
      let end = triple.getEnd();
      let Some(document) = layoutState.document.filter(|p| !p.is_null()) else { return };
      // java IChpxTable.Entry embedRun = layoutState.document.getChpxTable().seek(start);
      let Some(embedRun) = (unsafe { (*document).getChpxTable().and_then(|t| IChpxTable::seek(&*t, start)) }) else { return };
      // java PropertySet embedProp = this.mPropertyTool.getFinalRunPropert(...)
      let embedProp = self.mPropertyTool.as_mut().and_then(|pt| {
        layoutState.paraEnv.curParaPropEnv.as_ref()
          .and_then(|env| env.paraProperty.as_ref())
          .map(|pp| pt.getFinalRunPropert__entry_propertyset_boolean(&*embedRun, pp, self.mRevisionFinal).clone())
      });
      let Some(embedProp) = embedProp else { return };
      // java if (this.mEnableRevision) { ... if (isRevision) return; }
      if self.mEnableRevision && embedProp.getValue(45).is_some() && self.mRevisionFinal {
        return;
      }
      // java if (!embedProp.getBoolean(18, false))
      if embedProp.getBoolean__int_boolean(18, false) {
        return;
      }
      let Some(rope) = (unsafe { (*document).getTextRope() }) else { return };
      // java int contentStart = this.getEQContentStart(start, end, textRope);
      let contentStart = eq_content_start(start, end, unsafe { &mut *rope });
      if contentStart >= 0 && contentStart < end - 1 {
        // java CharsNode charsNode = this.addCharsNode(4, 1, embedProp, layoutState.charsList);
        let charsNode = self.addCharsNode(4, 1, Some(&embedProp), &mut layoutState.charsList) as *mut CharsNode;
        let src_cp = self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
        unsafe {
          // java charsNode.srcRef = this.mSrcNode;
          (*charsNode).srcRef = self.mSrcNode;
          // java charsNode.offsetOfRef = start - this.mSrcNode.cp;
          (*charsNode).offsetOfRef = start - src_cp;
          // java charsNode.lengthOfRef = end - start;
          (*charsNode).lengthOfRef = end - start;
        }
        // java this.calcCharsNodeProp(charsNode, embedProp);
        self.calcCharsNodeProp(unsafe { &mut *charsNode }, &embedProp);
        // java int graphIndex = typoPara.allocGraphs(1);
        let graphIndex = layoutState.typoPara.allocGraphs(1);
        // java this.addMap(start, charsNode.lengthOfRef, graphIndex, 1);
        self.super_.addMap(start, unsafe { (*charsNode).lengthOfRef }, graphIndex, 1);
        // java typoPara.putChar(graphIndex, ' ');
        layoutState.typoPara.putChar(graphIndex, ' ');
        // java this.createEquationNodes(..., separator == null ? end - 1 : getSepCP(), ...);
        let sep_cp = if triple.separator.is_none() { end - 1 } else { triple.getSepCP() };
        let hook = self.factory_create_equation_nodes;
        let this = self.factory_self;
        if let Some(hook) = hook {
          hook(this, unsafe { &mut *charsNode }, graphIndex, contentStart, sep_cp, self.mEnableRevision, self.mRevisionFinal, layoutState);
        }
      }
    }

    // java public abstract void setMathUnderline(int var1, int var2)
    pub fn setMathUnderline(&mut self, underline: i32, underline_color: i32) {
      if let Some(hook) = self.factory_set_math_underline {
        hook(self.factory_self, underline, underline_color);
      }
    }

    // java public abstract void setCalcGraphHeight(boolean val) on GraphFactory
    pub fn setCalcGraphHeight(&mut self, val: bool) {
      if let Some(hook) = self.factory_set_calc_graph_height {
        hook(self.factory_self, val);
      }
    }

    // java public void addRunPropFilter(IRunPropFilter filter)
    pub fn addRunPropFilter(&mut self, filter: Box<dyn IRunPropFilter>) {
      // java if (this.mRunPropFilters.indexOf(filter) < 0)
      if !self.mRunPropFilters.iter().any(|f| {
        std::ptr::eq(f.as_ref() as *const (dyn IRunPropFilter + '_), filter.as_ref() as *const (dyn IRunPropFilter + '_))
      }) {
        // java this.mRunPropFilters.add(filter);
        self.mRunPropFilters.push(filter);
      }
    }

    // java public void removeRunPropFilter(IRunPropFilter filter)
    pub fn removeRunPropFilter(&mut self, filter: &dyn IRunPropFilter) {
      // java int index = this.mRunPropFilters.indexOf(filter);
      let target: *const (dyn IRunPropFilter + '_) = filter;
      let mut index: Option<usize> = None;
      for (i, f) in self.mRunPropFilters.iter().enumerate() {
        if std::ptr::eq(f.as_ref() as *const (dyn IRunPropFilter + '_), target) {
          index = Some(i);
          break;
        }
      }
      // java if (index >= 0)
      if let Some(index) = index {
        // java this.mRunPropFilters.remove(index);
        self.mRunPropFilters.remove(index);
      }
    }

    // java final void readDrawingFlag(int start, int end, String str, PropertySet finalRunProp, LayoutState layoutState)
    pub(crate) fn readDrawingFlag(&mut self, start: i32, end: i32, str: &str, finalRunProp: &PropertySet, layoutState: &mut LayoutState) {
      let chars: Vec<char> = str.chars().collect();
      let len = chars.len() as i32;
      // java CharsNode charsNode = this.addCharsNode(0, str.length(), finalRunProp, layoutState.charsList);
      let chars_node = self.addCharsNode(0, len, Some(finalRunProp), &mut layoutState.charsList) as *mut CharsNode;
      let src_cp = self.mSrcNode.map(|p| unsafe { (*p).cp }).unwrap_or(0);
      unsafe {
        // java charsNode.srcRef = this.mSrcNode;
        (*chars_node).srcRef = self.mSrcNode;
        // java charsNode.offsetOfRef = start - this.mSrcNode.cp;
        (*chars_node).offsetOfRef = start - src_cp;
        // java charsNode.lengthOfRef = end - start;
        (*chars_node).lengthOfRef = end - start;
      }
      // java this.calcCharsNodeProp(charsNode, finalRunProp);
      self.calcCharsNodeProp(unsafe { &mut *chars_node }, finalRunProp);
      // java int graphIndex = typoPara.allocGraphs(str.length());
      let graph_index = layoutState.typoPara.allocGraphs(len);
      // java typoPara.putChars(str.toCharArray(), 0, str.length(), graphIndex);
      layoutState.typoPara.putChars__chararr_int_int_int(&chars, 0, len, graph_index);
      // java this.addMap(start, end - start, graphIndex, str.length());
      self.super_.addMap(start, end - start, graph_index, len);
      // java this.createNodes(charsNode, graphIndex, str.length(), layoutState);
      self.createNodes(unsafe { &mut *chars_node }, graph_index, len, layoutState);
      // java this.commitAll(layoutState);
      self.commitAll(layoutState);
    }
}

// java private class IgnoreHpsAndVAlignRunFilter implements IRunPropFilter
#[derive(Default)]
struct IgnoreHpsAndVAlignRunFilter {
}

// java public PropertySet doFilter(PropertySet prop, IChpxTable.Entry run, boolean revisionFinal, LayoutState layoutState)
impl IRunPropFilter for IgnoreHpsAndVAlignRunFilter {
    fn doFilter(&self, prop: &PropertySet, run: &dyn ChpxEntry, revisionFinal: bool, layoutState: &mut LayoutState) -> PropertySet {
      // java if (Float.compare(prop.getFloat(25, 0.0F), 0.0F) != 0 || prop.getInt(13, 0) != 0 || prop.getInt(24, 0) != 0 || prop.contains(37))
      if prop.getFloat__int_float(25, 0.0) != 0.0 || prop.getInt__int_int(13, 0) != 0 || prop.getInt__int_int(24, 0) != 0 || prop.contains(37) {
        // java MutablePropertySet newMutablePropertyMap = new MutablePropertySet(prop);
        let mut newMutablePropertyMap: MutablePropertySet = MutablePropertySet::new_from_ps(prop);
        // java newMutablePropertyMap.remove(25);
        newMutablePropertyMap.remove(25);
        // java newMutablePropertyMap.remove(13);
        newMutablePropertyMap.remove(13);
        // java newMutablePropertyMap.remove(24);
        newMutablePropertyMap.remove(24);
        // java newMutablePropertyMap.remove(37);
        newMutablePropertyMap.remove(37);
        // java prop = newMutablePropertyMap.freeze();
        return newMutablePropertyMap.freeze();
      }
      // java return prop;
      return prop.clone();
    }
    fn isIgnoreHpsAndVAlignRunFilter(&self) -> bool {
        true
    }
}

// java private class IgnoreCapsAndSmallCapsFilter implements IRunPropFilter
#[derive(Default)]
struct IgnoreCapsAndSmallCapsFilter {
}

// java public PropertySet doFilter(PropertySet prop, IChpxTable.Entry run, boolean revisionFinal, LayoutState layoutState)
impl IRunPropFilter for IgnoreCapsAndSmallCapsFilter {
    fn doFilter(&self, prop: &PropertySet, run: &dyn ChpxEntry, revisionFinal: bool, layoutState: &mut LayoutState) -> PropertySet {
      // java if (prop.getBoolean(17, false) || prop.getBoolean(16, false))
      if prop.getBoolean__int_boolean(17, false) || prop.getBoolean__int_boolean(16, false) {
        // java MutablePropertySet newMutablePropertyMap = new MutablePropertySet(prop);
        let mut newMutablePropertyMap: MutablePropertySet = MutablePropertySet::new_from_ps(prop);
        // java newMutablePropertyMap.remove(17);
        newMutablePropertyMap.remove(17);
        // java newMutablePropertyMap.remove(16);
        newMutablePropertyMap.remove(16);
        // java prop = newMutablePropertyMap.freeze();
        return newMutablePropertyMap.freeze();
      }
      // java return prop;
      return prop.clone();
    }
    fn isIgnoreCapsAndSmallCapsFilter(&self) -> bool {
        true
    }
}

// Placeholder so a filter can be taken out of mFilters while it calls back into GraphReader.
struct IdleFilter;

impl IFilter for IdleFilter {
    fn doFilter(&mut self, _args: &mut FilterArgs, _layoutState: &mut LayoutState) -> bool {
        false
    }
}

fn eq_content_start(start: i32, end: i32, text: &mut dyn ITextRope) -> i32 {
    // java private int getEQContentStart(int start, int end, ITextRope textRope)
    let max = end - 1;
    let mut cur = start + 1;
    if start >= end {
        return -1;
    }
    while cur < max {
        let ch = text.charAt(cur);
        cur += 1;
        if ch == 'e' || ch == 'E' {
            break;
        }
    }
    while cur < max {
        let ch = text.charAt(cur);
        cur += 1;
        if ch == 'q' || ch == 'Q' {
            break;
        }
    }
    while cur < max && text.charAt(cur) == ' ' {
        cur += 1;
    }
    cur
}
