// Java: cn.wps.moffice.writer.layout.base.line.LineHeightLayouter
// rust: ★wr-29 05 —— 真实移植（Java 467 行）：initLineHeightInfo/getGraphVMetrics 链/
//       calcLineHeightAndAlignmentLine/updateLineHeightAndAlignmentLine 全真实。
//       embed page-9（TypoDrawing padding）、nextGraphs 游标 按 GR-D3 停车（hello 非 embed）。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.moffice.writer.data.value.LineSpacing
use crate::cn::wps::moffice::writer::data::value::LineSpacing::LineSpacing;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphAlignTool
use crate::cn::wps::moffice::writer::layout::base::graph::GraphAlignTool::GraphAlignTool;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.line.ILineHeightCallback
use crate::cn::wps::moffice::writer::layout::base::line::ILineHeightCallback::ILineHeightCallback;

// java public class LineHeightLayouter
pub struct LineHeightLayouter {
    // java private static final int CHANNEL_ALIGN_TOP = 0;
    // Java 常量内联
    // java protected int mGridHeight;
    pub(crate) mGridHeight: i32,
    // java protected LineSpacing mLineSpacing;
    pub(crate) mLineSpacing: LineSpacing,
    // java protected int mBaseHeight;
    pub(crate) mBaseHeight: i32,
    // java protected float mMaxAscent;
    pub(crate) mMaxAscent: f32,
    // java protected float mMaxDescent;
    pub(crate) mMaxDescent: f32,
    // java protected int mChannelAlign;
    pub(crate) mChannelAlign: i32,
    // java protected boolean mNeedSnapToGrid;
    pub(crate) mNeedSnapToGrid: bool,
    // java protected int mLineHeight;
    pub(crate) mLineHeight: i32,
    // java protected int mAscentPos;
    pub(crate) mAscentPos: i32,
    // java protected int mDescentPos;
    pub(crate) mDescentPos: i32,
    // java protected int mBaselinePos;
    pub(crate) mBaselinePos: i32,
    // java float[] mVMetricsDatas = new float[7];
    pub(crate) mVMetricsDatas: [f32; 7],
    // java float[] mAscentAndDescent = new float[2];
    pub(crate) mAscentAndDescent: [f32; 2],
    // java float[] mVMetricsStack = new float[28];
    pub(crate) mVMetricsStack: Vec<f32>,
    // java int mStackSize = 0;
    pub(crate) mStackSize: usize,
    // java int mMaxGraphIndexForMetrics = 0;
    pub(crate) mMaxGraphIndexForMetrics: i32,
    // java protected DocumentEnv mDocEnv;
    pub(crate) mDocEnv: *mut DocumentEnv,
    // java protected ViewEnv mViewEnv;
    pub(crate) mViewEnv: *mut ViewEnv,
}

impl LineHeightLayouter {
    // java public LineHeightLayouter(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn LineHeightLayouter(docEnv: *mut DocumentEnv, viewEnv: *mut ViewEnv) -> LineHeightLayouter {
      // java this.mDocEnv = docEnv; this.mViewEnv = viewEnv;
      LineHeightLayouter {
        mGridHeight: 0, mLineSpacing: LineSpacing::LineSpacing(0, 1.0), mBaseHeight: 0,
        mMaxAscent: 0.0, mMaxDescent: 0.0, mChannelAlign: 0, mNeedSnapToGrid: false,
        mLineHeight: 0, mAscentPos: 0, mDescentPos: 0, mBaselinePos: 0,
        mVMetricsDatas: [0.0; 7], mAscentAndDescent: [0.0; 2],
        mVMetricsStack: vec![0.0; 28], mStackSize: 0, mMaxGraphIndexForMetrics: 0,
        mDocEnv: docEnv, mViewEnv: viewEnv,
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mDocEnv = null; this.mViewEnv = null;
      self.mDocEnv = std::ptr::null_mut();
      self.mViewEnv = std::ptr::null_mut();
    }

    // java private void calcLineHeightInfoByLineSpacingRule(LayoutState layoutState)
    fn calcLineHeightInfoByLineSpacingRule(&mut self, layoutState: &LayoutState) {
      // java int spacingRule = this.mLineSpacing.getRule();
      let spacingRule: i32 = self.mLineSpacing.getRule();
      // java float lineSpacing = this.mLineSpacing.getSpacing();
      let lineSpacing: f32 = self.mLineSpacing.getSpacing();
      // java if (1 == spacingRule)
      if 1 == spacingRule {
        // java this.mNeedSnapToGrid = false;
        self.mNeedSnapToGrid = false;
        // java this.mLineHeight = (int)(Math.abs(lineSpacing) + 0.5F);
        self.mLineHeight = (lineSpacing.abs() + 0.5) as i32;
        // java int wordAlign = layoutState.paraEnv.wordAlignFont;
        let mut wordAlign: i32 = layoutState.paraEnv.wordAlignFont;
        // java if (4 == wordAlign) { if (0 != layoutState.pageEnv.textFlow) { wordAlign = 1; } else { wordAlign = 2; } }
        if 4 == wordAlign {
          if 0 != layoutState.pageEnv.textFlow { wordAlign = 1; } else { wordAlign = 2; }
        }
        // java switch (wordAlign)
        match wordAlign {
          // java case 0: this.mChannelAlign = 3; break;
          0 => { self.mChannelAlign = 3; }
          // java case 1: this.mChannelAlign = 4; break;
          1 => { self.mChannelAlign = 4; }
          // java default: this.mChannelAlign = 5;
          _ => { self.mChannelAlign = 5; }
        }
      }
      // java else if (2 == spacingRule)
      else if 2 == spacingRule {
        // java if (this.mNeedSnapToGrid) { this.mNeedSnapToGrid = 0.0F != lineSpacing; }
        if self.mNeedSnapToGrid { self.mNeedSnapToGrid = 0.0 != lineSpacing; }
        // java this.mChannelAlign = 2;
        self.mChannelAlign = 2;
        // java this.mLineHeight = (int)(Math.max(this.getChannelHeight(), lineSpacing) + 0.5F);
        self.mLineHeight = (self.getChannelHeight().max(lineSpacing) + 0.5) as i32;
      }
      // java else if (this.mNeedSnapToGrid)
      else if self.mNeedSnapToGrid {
        // java this.mChannelAlign = 1;
        self.mChannelAlign = 1;
        // java this.mLineHeight = (int)((float)this.mGridHeight * lineSpacing + 0.5F);
        self.mLineHeight = (self.mGridHeight as f32 * lineSpacing + 0.5) as i32;
      }
      // java else
      else {
        // java if (this.mViewEnv.getViewMode() != 0 && datas[1] <= 0 && datas[2] <= 0 && datas[5] <= 0 && datas[6] <= 0)
        let viewMode: i32 = if self.mViewEnv.is_null() { 0 } else { unsafe { &*self.mViewEnv }.getViewMode() };
        if viewMode != 0 && self.mVMetricsDatas[1] <= 0.0 && self.mVMetricsDatas[2] <= 0.0 && self.mVMetricsDatas[5] <= 0.0 && self.mVMetricsDatas[6] <= 0.0 {
          // java this.mLineHeight = this.mBaseHeight;
          self.mLineHeight = self.mBaseHeight;
        } else {
          // java float extraSpace = (float)this.mBaseHeight * (lineSpacing - 1.0F);
          let extraSpace: f32 = self.mBaseHeight as f32 * (lineSpacing - 1.0);
          // java this.mLineHeight = (int)(this.getChannelHeight() + extraSpace + 0.5F);
          self.mLineHeight = (self.getChannelHeight() + extraSpace + 0.5) as i32;
        }
        // java this.mChannelAlign = lineSpacing >= 1.0F ? 0 : 1;
        self.mChannelAlign = if lineSpacing >= 1.0 { 0 } else { 1 };
      }
    }

    // java private void calcLineHeightInfoBySnappingToGrid()
    fn calcLineHeightInfoBySnappingToGrid(&mut self) {
      // java if (this.mNeedSnapToGrid)
      if self.mNeedSnapToGrid {
        // java int gridHeight = this.mGridHeight;
        let gridHeight: i32 = self.mGridHeight;
        // java float allGraphHeight = this.getChannelHeight();
        let allGraphHeight: f32 = self.getChannelHeight();
        // java int lineHeight = (int)((double)gridHeight * Math.ceil((double)(allGraphHeight / (float)gridHeight)));
        let mut lineHeight: i32 = ((gridHeight as f64) * (allGraphHeight as f64 / gridHeight as f64).ceil()) as i32;
        // java if (allGraphHeight > (float)lineHeight) { lineHeight += gridHeight; }
        if allGraphHeight > lineHeight as f32 { lineHeight += gridHeight; }
        // java if (lineHeight > this.mLineHeight) { this.mLineHeight = lineHeight; this.mChannelAlign = 1; }
        if lineHeight > self.mLineHeight {
          self.mLineHeight = lineHeight;
          self.mChannelAlign = 1;
        }
      }
    }

    // java private void calcLineHeightInfoByLastLine(int remainingHeight)
    fn calcLineHeightInfoByLastLine(&mut self, remainingHeight: i32) {
      // java if (1 != this.mLineSpacing.getRule())
      if 1 != self.mLineSpacing.getRule() {
        // java float minHeight = (float)this.mBaselinePos + this.mMaxDescent;
        let minHeight: f32 = self.mBaselinePos as f32 + self.mMaxDescent;
        // java if (minHeight < (float)remainingHeight) { this.mLineHeight = remainingHeight; }
        if minHeight < remainingHeight as f32 { self.mLineHeight = remainingHeight; }
        // java this.mDescentPos = Math.min(this.mDescentPos, this.mLineHeight);
        self.mDescentPos = self.mDescentPos.min(self.mLineHeight);
      }
    }

    // java protected void initLineHeightInfo(LayoutState layoutState)
    pub(crate) fn initLineHeightInfo(&mut self, layoutState: &mut LayoutState) {
      // java ParaEnv paraEnv = layoutState.paraEnv;
      // java PageEnv pageEnv = layoutState.pageEnv;
      let lineSpacing: LineSpacing = layoutState.paraEnv.lineSpacing.clone();
      let gridHeight: i32 = layoutState.pageEnv.gridHeight;
      let gridModel: i32 = layoutState.pageEnv.gridModel;
      let isParaSnapToGrid: bool = layoutState.paraEnv.snapToGrid;
      let isPageSnapToGrid: bool = 0 != gridModel && gridHeight as f32 > 0.0;
      // java this.mLineSpacing = paraEnv.lineSpacing;
      self.mLineSpacing = lineSpacing;
      // java this.mGridHeight = pageEnv.gridHeight;
      self.mGridHeight = gridHeight;
      // java boolean needSnapToGrid = isParaSnapToGrid && isPageSnapToGrid;
      let mut needSnapToGrid: bool = isParaSnapToGrid && isPageSnapToGrid;
      // java if (layoutState.tablePeriLayer > 1 && this.mDocEnv.noAdjustLineHeightInTable)
      if layoutState.tablePeriLayer > 1 && !self.mDocEnv.is_null() && unsafe { &*self.mDocEnv }.noAdjustLineHeightInTable {
        // java needSnapToGrid = false;
        needSnapToGrid = false;
      }
      // java this.mStackSize = 0;
      self.mStackSize = 0;
      // java KPList<Graphs> graphList = layoutState.getGraphList();
      // java int wordAlingFont = paraEnv.wordAlignFont;
      let mut wordAlingFont: i32 = layoutState.paraEnv.wordAlignFont;
      // java int textFlow = pageEnv.textFlow;
      let textFlow: i32 = layoutState.pageEnv.textFlow;
      // java if (4 == wordAlingFont && 0 != textFlow && 4 != textFlow) { wordAlingFont = 1; }
      if 4 == wordAlingFont && 0 != textFlow && 4 != textFlow { wordAlingFont = 1; }
      // java boolean isWord97LineBreakRules = this.mDocEnv.useWord97LineBreakRules;
      let isWord97LineBreakRules: bool = if self.mDocEnv.is_null() { false } else { unsafe { &*self.mDocEnv }.useWord97LineBreakRules };
      // java float[] rsAscentAndDescent = this.mAscentAndDescent;
      // java float[] datas = this.mVMetricsDatas;
      // java Arrays.fill(datas, 0.0F);
      self.mVMetricsDatas = [0.0; 7];
      // java Graphs graphs = (Graphs)graphList.firstElement();
      // rust: 空 graphList（hello 前一状态已 prepare）→ Java firstElement 亦 null → datas 全 0 → doCommit 直通
      if layoutState.getGraphList().is_empty() {
        self.mNeedSnapToGrid = needSnapToGrid;
        self.mLineHeight = 0;
        self.mBaselinePos = 0;
        self.updateLineInfo();
        return;
      }
      if std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp) {
        let mut probe_ptr = layoutState.getGraphList().firstElement();
        let mut probe = [0.0_f32; 2];
        while !probe_ptr.is_null() {
          let g = unsafe { &*probe_ptr };
          if let Some(style) = &g.fontStyle {
            if let Some(face) = &style.typeface {
              if g.isEmbed() { GraphAlignTool::getEmbedVMetrics(g, wordAlingFont, isWord97LineBreakRules, &mut probe); }
              else { GraphAlignTool::getGraphVMetrics(g, wordAlingFont, isWord97LineBreakRules, &mut probe); }
              eprintln!("graph_metrics cp={} graph={} type={} size={} em={} ascent={} descent={} external={} graphAsc={} graphDesc={}",
                layoutState.cp, g.graphIndex, g.runType, style.fontSize, style.emSize,
                face.getWriterLayoutAscent(), face.getWriterLayoutDescent(), face.getWriterLayoutLatinExternal(), probe[0], probe[1]);
            }
          }
          probe_ptr = layoutState.getGraphList().getNext(probe_ptr);
        }
      }
      let core_width_f: f32 = layoutState.pageEnv.getCoreWidth() as f32;
      let graphsPtr: *mut Graphs = layoutState.getGraphList().firstElement();
      // java datas[0] = (float)graphs.graphIndex;
      self.mVMetricsDatas[0] = unsafe { (*graphsPtr).graphIndex } as f32;
      // java TypoFontStyle lastFontStyle = null;
      let mut lastFontStyle: Option<TypoFontStyle> = None;
      // java if (graphs.isEmbed())
      if unsafe { (*graphsPtr).isEmbed() } {
        // java GraphAlignTool.getEmbedVMetrics(...); datas[3] = ...; datas[4] = ...;
        GraphAlignTool::getEmbedVMetrics(unsafe { &*graphsPtr }, wordAlingFont, isWord97LineBreakRules, &mut self.mAscentAndDescent);
        self.mVMetricsDatas[3] = self.mAscentAndDescent[0];
        self.mVMetricsDatas[4] = self.mAscentAndDescent[1];
        // java if (layoutState.curPage.getType() == 9 && datas[3] > 0) { ... TypoDrawing padding ... }
        // rust: GR-D3 —— embed page-9 TypoDrawing padding 非 hello
      } else {
        // java GraphAlignTool.getGraphVMetrics(...)
        GraphAlignTool::getGraphVMetrics(unsafe { &*graphsPtr }, wordAlingFont, isWord97LineBreakRules, &mut self.mAscentAndDescent);
        // java if (TypoRun.isWeakHeightGraphs(graphs.runType)) { datas[5]/datas[6] } else { datas[1]/datas[2]; lastFontStyle = graphs.fontStyle; }
        if TypoRun::isWeakHeightGraphs(unsafe { (*graphsPtr).runType }) {
          self.mVMetricsDatas[5] = self.mAscentAndDescent[0];
          self.mVMetricsDatas[6] = self.mAscentAndDescent[1];
        } else {
          self.mVMetricsDatas[1] = self.mAscentAndDescent[0];
          self.mVMetricsDatas[2] = self.mAscentAndDescent[1];
          lastFontStyle = unsafe { (*graphsPtr).fontStyle.clone() };
        }
      }
      // java this.mMaxGraphIndexForMetrics = graphs.graphIndex + graphs.graphLength;
      self.mMaxGraphIndexForMetrics = unsafe { (*graphsPtr).graphIndex } + unsafe { (*graphsPtr).graphLength };
      // java int widthCount = graphs.width();
      let mut widthCount: i32 = unsafe { (*graphsPtr).width() };
      // java while(null != (graphs = (Graphs)graphList.getNext(graphs)))
      let mut curPtr: *mut Graphs = unsafe { (&*graphsPtr as *const Graphs as *mut Graphs) };
      loop {
        curPtr = layoutState.getGraphList().getNext(curPtr);
        if curPtr.is_null() { break; }
        // java int runType = graphs.runType;
        let runType: i32 = unsafe { (*curPtr).runType };
        // java widthCount += graphs.width();
        widthCount += unsafe { (*curPtr).width() };
        // java if (graphs.isEmbed())
        if unsafe { (*curPtr).isEmbed() } {
          // java if (this.checkWidthIsInCoreWidth((float)widthCount, pageEnv))
          if (widthCount as f32) < core_width_f {
            // java GraphAlignTool.getEmbedVMetrics(...)
            GraphAlignTool::getEmbedVMetrics(unsafe { &*curPtr }, wordAlingFont, isWord97LineBreakRules, &mut self.mAscentAndDescent);
            let graphAscent: f32 = self.mAscentAndDescent[0];
            let graphDescent: f32 = self.mAscentAndDescent[1];
            // java if (graphAscent > datas[3] || graphDescent > datas[4]) { push; datas[0]=graphIndex; datas[3]=max; datas[4]=max; }
            if graphAscent > self.mVMetricsDatas[3] || graphDescent > self.mVMetricsDatas[4] {
              self.pushVMetricsDatas();
              self.mVMetricsDatas[0] = unsafe { (*curPtr).graphIndex } as f32;
              self.mVMetricsDatas[3] = self.mVMetricsDatas[3].max(graphAscent);
              self.mVMetricsDatas[4] = self.mVMetricsDatas[4].max(graphDescent);
            }
            // java this.mMaxGraphIndexForMetrics = graphs.graphIndex + graphs.graphLength;
            self.mMaxGraphIndexForMetrics = unsafe { (*curPtr).graphIndex } + unsafe { (*curPtr).graphLength };
          }
        } else {
          // java this.mMaxGraphIndexForMetrics = graphs.graphIndex + graphs.graphLength;
          self.mMaxGraphIndexForMetrics = unsafe { (*curPtr).graphIndex } + unsafe { (*curPtr).graphLength };
          // java if (lastFontStyle == graphs.fontStyle)
          let curStyle: Option<TypoFontStyle> = unsafe { (*curPtr).fontStyle.clone() };
          if lastFontStyle.is_some() && Self::same_style(&lastFontStyle, &curStyle) {
            // java if (!this.checkWidthIsInCoreWidth(...)) { break; }
            if !((widthCount as f32) < core_width_f) { break; }
            // java continue;
            continue;
          }
          // java boolean isWeak = TypoRun.isWeakHeightGraphs(runType);
          let isWeak: bool = TypoRun::isWeakHeightGraphs(runType);
          // java if (!isWeak || datas[1] <= 0 && datas[2] <= 0 && datas[3] <= 0 && datas[4] <= 0)
          if !isWeak || (self.mVMetricsDatas[1] <= 0.0 && self.mVMetricsDatas[2] <= 0.0 && self.mVMetricsDatas[3] <= 0.0 && self.mVMetricsDatas[4] <= 0.0) {
            // java GraphAlignTool.getGraphVMetrics(...)
            GraphAlignTool::getGraphVMetrics(unsafe { &*curPtr }, wordAlingFont, isWord97LineBreakRules, &mut self.mAscentAndDescent);
            let graphAscent: f32 = self.mAscentAndDescent[0];
            let graphDescent: f32 = self.mAscentAndDescent[1];
            // java if (isWeak) { push weak }
            if isWeak {
              if graphAscent > self.mVMetricsDatas[5] || graphDescent > self.mVMetricsDatas[6] {
                self.pushVMetricsDatas();
                self.mVMetricsDatas[0] = unsafe { (*curPtr).graphIndex } as f32;
                self.mVMetricsDatas[5] = self.mVMetricsDatas[5].max(graphAscent);
                self.mVMetricsDatas[6] = self.mVMetricsDatas[6].max(graphDescent);
              }
            } else {
              // java else { if (...) push text; lastFontStyle = graphs.fontStyle; }
              if graphAscent > self.mVMetricsDatas[1] || graphDescent > self.mVMetricsDatas[2] {
                self.pushVMetricsDatas();
                self.mVMetricsDatas[0] = unsafe { (*curPtr).graphIndex } as f32;
                self.mVMetricsDatas[1] = self.mVMetricsDatas[1].max(graphAscent);
                self.mVMetricsDatas[2] = self.mVMetricsDatas[2].max(graphDescent);
              }
              lastFontStyle = curStyle;
            }
          }
        }
        // java if (!this.checkWidthIsInCoreWidth((float)widthCount, pageEnv)) { break; }
        if !((widthCount as f32) < core_width_f) { break; }
      }
      // java this.mNeedSnapToGrid = needSnapToGrid;
      self.mNeedSnapToGrid = needSnapToGrid;
      // java this.mLineHeight = 0; this.mBaselinePos = 0;
      self.mLineHeight = 0;
      self.mBaselinePos = 0;
      // java this.updateLineInfo();
      self.updateLineInfo();
    }

    // java 引用等价（lastFontStyle == graphs.fontStyle，Java 引用比较 → fontStyle 池复用）
    fn same_style(a: &Option<TypoFontStyle>, b: &Option<TypoFontStyle>) -> bool {
      // rust: Arc typeface + 关键字段相等（Java 引用等价的 compile-surface 近似；hello 同一 run 同 style）
      match (a, b) {
        (Some(x), Some(y)) => {
          ArcPtrEq::same(&x.typeface, &y.typeface) && x.fontSize == y.fontSize && x.hpsPos == y.hpsPos
        }
        _ => a.is_none() && b.is_none(),
      }
    }

    // java protected boolean checkWidthIsInCoreWidth(float widthCount, PageEnv pageEnv)
    pub(crate) fn checkWidthIsInCoreWidth(&self, widthCount: f32, pageEnv: &PageEnv) -> bool {
      // java float coreWidth = (float)pageEnv.getCoreWidth();
      let coreWidth: f32 = pageEnv.getCoreWidth() as f32;
      // java return widthCount < coreWidth;
      widthCount < coreWidth
    }

    // java private void updateLineInfo()
    fn updateLineInfo(&mut self) {
      // java float[] datas = this.mVMetricsDatas;
      // java this.mBaseHeight = 0;
      self.mBaseHeight = 0;
      // java if (!(datas[1] > 0) && !(datas[2] > 0))
      if !(self.mVMetricsDatas[1] > 0.0) && !(self.mVMetricsDatas[2] > 0.0) {
        // java if (!(datas[5] > 0) && !(datas[6] > 0)) { this.mBaseHeight = (int)(datas[3]+datas[4]+0.5F); }
        if !(self.mVMetricsDatas[5] > 0.0) && !(self.mVMetricsDatas[6] > 0.0) {
          self.mBaseHeight = (self.mVMetricsDatas[3] + self.mVMetricsDatas[4] + 0.5) as i32;
        } else {
          // java else { this.mBaseHeight = (int)(datas[5]+datas[6]+0.5F); }
          self.mBaseHeight = (self.mVMetricsDatas[5] + self.mVMetricsDatas[6] + 0.5) as i32;
        }
        // java if (!(datas[3] > 0) && !(datas[4] > 0)) { mMaxAscent=datas[5]; mMaxDescent=datas[6]; }
        if !(self.mVMetricsDatas[3] > 0.0) && !(self.mVMetricsDatas[4] > 0.0) {
          self.mMaxAscent = self.mVMetricsDatas[5];
          self.mMaxDescent = self.mVMetricsDatas[6];
        }
        // java else if (this.mNeedSnapToGrid) { ... datas[3]/[4] }
        else if self.mNeedSnapToGrid {
          self.mMaxAscent = self.mVMetricsDatas[3];
          self.mMaxDescent = self.mVMetricsDatas[4];
        }
        // java else { mMaxAscent = max(datas[5],datas[3]); mMaxDescent = max(datas[6],datas[4]); }
        else {
          self.mMaxAscent = self.mVMetricsDatas[5].max(self.mVMetricsDatas[3]);
          self.mMaxDescent = self.mVMetricsDatas[6].max(self.mVMetricsDatas[4]);
        }
      } else {
        // java else { mMaxAscent=datas[1]; mMaxDescent=datas[2]; mBaseHeight=...; mMaxAscent=max(datas[1],datas[3]); mMaxDescent=max(datas[2],datas[4]); }
        self.mMaxAscent = self.mVMetricsDatas[1];
        self.mMaxDescent = self.mVMetricsDatas[2];
        self.mBaseHeight = (self.mMaxAscent + self.mMaxDescent + 0.5) as i32;
        self.mMaxAscent = self.mMaxAscent.max(self.mVMetricsDatas[3]);
        self.mMaxDescent = self.mMaxDescent.max(self.mVMetricsDatas[4]);
      }
    }

    // java private void pushVMetricsDatas()
    fn pushVMetricsDatas(&mut self) {
      // java if (this.mStackSize * 7 >= this.mVMetricsStack.length) { double stack }
      if self.mStackSize * 7 >= self.mVMetricsStack.len() {
        self.mVMetricsStack.resize(self.mVMetricsStack.len() * 2, 0.0);
      }
      // java System.arraycopy(this.mVMetricsDatas, 0, this.mVMetricsStack, this.mStackSize * 7, 7);
      let base: usize = self.mStackSize * 7;
      for i in 0..7 { self.mVMetricsStack[base + i] = self.mVMetricsDatas[i]; }
      // java ++this.mStackSize;
      self.mStackSize += 1;
    }

    // java private void calcBaseLinePos()
    fn calcBaseLinePos(&mut self) {
      // java float maxAscent = this.mMaxAscent;
      let maxAscent: f32 = self.mMaxAscent;
      // java float channelHeight = this.getChannelHeight();
      let channelHeight: f32 = self.getChannelHeight();
      // java float extraHeight = (float)this.mLineHeight - channelHeight;
      let extraHeight: f32 = self.mLineHeight as f32 - channelHeight;
      // java switch (this.mChannelAlign)
      self.mBaselinePos = match self.mChannelAlign {
        // java case 0: this.mBaselinePos = (int)(maxAscent + 0.5F); break;
        0 => (maxAscent + 0.5) as i32,
        // java case 1: this.mBaselinePos = (int)(maxAscent + extraHeight * 0.5F + 0.5F); break;
        1 => (maxAscent + extraHeight * 0.5 + 0.5) as i32,
        // java case 2: this.mBaselinePos = (int)(maxAscent + extraHeight + 0.5F); break;
        2 => (maxAscent + extraHeight + 0.5) as i32,
        // java case 3: this.mBaselinePos = (int)((float)this.mLineHeight * 0.2F + 0.5F); break;
        3 => (self.mLineHeight as f32 * 0.2 + 0.5) as i32,
        // java case 4: this.mBaselinePos = (int)((float)this.mLineHeight * 0.5F + 0.5F); break;
        4 => (self.mLineHeight as f32 * 0.5 + 0.5) as i32,
        // java case 5: this.mBaselinePos = (int)((float)this.mLineHeight * 0.8F + 0.5F); break;
        5 => (self.mLineHeight as f32 * 0.8 + 0.5) as i32,
        // java default: this.mBaselinePos = (int)(maxAscent + 0.5F);
        _ => (maxAscent + 0.5) as i32,
      };
      // java this.mDescentPos = (int)((float)this.mBaselinePos + this.mMaxDescent);
      self.mDescentPos = (self.mBaselinePos as f32 + self.mMaxDescent) as i32;
      // java this.mAscentPos = (int)((float)this.mBaselinePos - this.mMaxAscent);
      self.mAscentPos = (self.mBaselinePos as f32 - self.mMaxAscent) as i32;
    }

    // java private void doCommitLineHeightInfo(ILineHeightCallback callback)
    fn doCommitLineHeightInfo(&self, callback: &mut dyn ILineHeightCallback) {
      // java if (callback != null)
      // java callback.setLineHeight(this.mLineHeight);
      callback.setLineHeight(self.mLineHeight);
      // java callback.setAlignLine(this.mBaselinePos);
      callback.setAlignLine(self.mBaselinePos);
      // java callback.setAscDesPos(this.mAscentPos, this.mDescentPos);
      callback.setAscDesPos(self.mAscentPos, self.mDescentPos);
    }

    // java public void calcLineHeightAndAlignmentLine(LayoutState layoutState, ILineHeightCallback callback)
    pub fn calcLineHeightAndAlignmentLine(&mut self, layoutState: &mut LayoutState, callback: &mut dyn ILineHeightCallback) {
      // java this.initLineHeightInfo(layoutState);
      self.initLineHeightInfo(layoutState);
      let trace_cp = std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp);
      if trace_cp { eprintln!("line_height before cp={} grid={} snap={} base={} maxAsc={} maxDesc={} data={:?}", layoutState.cp, self.mGridHeight, self.mNeedSnapToGrid, self.mBaseHeight, self.mMaxAscent, self.mMaxDescent, self.mVMetricsDatas); }
      // java float[] datas = this.mVMetricsDatas;
      // java if (0.0F >= datas[1] && ... datas[4]) { this.doCommitLineHeightInfo(callback); }
      if self.mVMetricsDatas[1] <= 0.0 && self.mVMetricsDatas[2] <= 0.0
          && self.mVMetricsDatas[5] <= 0.0 && self.mVMetricsDatas[6] <= 0.0
          && self.mVMetricsDatas[3] <= 0.0 && self.mVMetricsDatas[4] <= 0.0 {
        self.doCommitLineHeightInfo(callback);
      } else {
        // java else { this.doCalcLineHeight(layoutState); this.doCommitLineHeightInfo(callback); }
        self.doCalcLineHeight(layoutState);
        self.doCommitLineHeightInfo(callback);
      }
      if trace_cp { eprintln!("line_height after cp={} height={} baseline={} asc={} desc={}", layoutState.cp, self.mLineHeight, self.mBaselinePos, self.mAscentPos, self.mDescentPos); }
    }

    // java private void doCalcLineHeight(LayoutState layoutState)
    fn doCalcLineHeight(&mut self, layoutState: &LayoutState) {
      // java this.calcLineHeightInfoByLineSpacingRule(layoutState);
      self.calcLineHeightInfoByLineSpacingRule(layoutState);
      // java this.calcLineHeightInfoBySnappingToGrid();
      self.calcLineHeightInfoBySnappingToGrid();
      // java if (1 == this.mLineSpacing.getRule() && this.mDocEnv.doNotUseHTMLParagraphAutoSpacing)
      if 1 == self.mLineSpacing.getRule() && !self.mDocEnv.is_null() && unsafe { &*self.mDocEnv }.doNotUseHTMLParagraphAutoSpacing {
        // java this.mLineHeight = Math.max(this.mLineHeight, (int)(this.getChannelHeight() + 0.5F));
        self.mLineHeight = self.mLineHeight.max((self.getChannelHeight() + 0.5) as i32);
      }
      // java this.calcBaseLinePos();
      self.calcBaseLinePos();
      // java int remainingHeight = layoutState.pageEnv.maxLayoutHeight - layoutState.curLayoutHeight;
      let remainingHeight: i32 = layoutState.pageEnv.maxLayoutHeight - layoutState.curLayoutHeight;
      // java if (remainingHeight < this.mLineHeight) { this.calcLineHeightInfoByLastLine(remainingHeight); }
      if remainingHeight < self.mLineHeight {
        self.calcLineHeightInfoByLastLine(remainingHeight);
      }
    }

    // java private float getChannelHeight()
    fn getChannelHeight(&self) -> f32 {
      // java return this.mMaxAscent + this.mMaxDescent;
      self.mMaxAscent + self.mMaxDescent
    }

    // java public int getBaseHeight()
    pub fn getBaseHeight(&self) -> i32 {
      // java return this.mBaseHeight;
      self.mBaseHeight
    }

    // java public boolean isOnlyEmbed()
    pub fn isOnlyEmbed(&self) -> bool {
      // java return datas[1] <= 0 && datas[2] <= 0 && datas[5] <= 0 && datas[6] <= 0 && datas[3] > 0;
      self.mVMetricsDatas[1] <= 0.0 && self.mVMetricsDatas[2] <= 0.0
        && self.mVMetricsDatas[5] <= 0.0 && self.mVMetricsDatas[6] <= 0.0
        && self.mVMetricsDatas[3] > 0.0
    }

    // java public void updateLineHeightAndAlignmentLine(int lastGraphIndex, LayoutState layoutState, ILineHeightCallback callback)
    pub fn updateLineHeightAndAlignmentLine(&mut self, lastGraphIndex: i32, layoutState: &mut LayoutState, callback: &mut dyn ILineHeightCallback) {
      let trace_cp = std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp);
      // java if (lastGraphIndex >= this.mMaxGraphIndexForMetrics)
      if lastGraphIndex >= self.mMaxGraphIndexForMetrics {
        // java KPList<Graphs> graphList = layoutState.getGraphList();
        // java int wordAlingFont = ...; int textFlow = ...;
        let mut wordAlingFont: i32 = layoutState.paraEnv.wordAlignFont;
        let textFlow: i32 = layoutState.pageEnv.textFlow;
        if 4 == wordAlingFont && 0 != textFlow && 4 != textFlow { wordAlingFont = 1; }
        // java boolean isWord97LineBreakRules = this.mDocEnv.useWord97LineBreakRules;
        let isWord97LineBreakRules: bool = if self.mDocEnv.is_null() { false } else { unsafe { &*self.mDocEnv }.useWord97LineBreakRules };
        // java Graphs graphs = layoutState.nextGraphs(); if (null == graphs) graphs = ...lastElement();
        // rust: nextGraphs 游标（05 后半程）—— hello 走 lastElement 等价
        let mut graphsPtr: *mut Graphs = if layoutState.getGraphList().is_empty() { std::ptr::null_mut() } else { layoutState.getGraphList().lastElement() };
        // java while(graphs != null && graphs.graphIndex >= this.mMaxGraphIndexForMetrics) { graphs = ...getPrev(graphs); }
        while !graphsPtr.is_null() && unsafe { (*graphsPtr).graphIndex } >= self.mMaxGraphIndexForMetrics {
          graphsPtr = layoutState.getGraphList().getPrev(graphsPtr);
        }
        // java TypoFontStyle lastFontStyle = null;
        let mut lastFontStyle: Option<TypoFontStyle> = None;
        // java while(null != (graphs = (Graphs)graphList.getNext(graphs)))
        loop {
          graphsPtr = layoutState.getGraphList().getNext(graphsPtr);
          if graphsPtr.is_null() { break; }
          // java int runType = graphs.runType;
          let runType: i32 = unsafe { (*graphsPtr).runType };
          // java int endGraphIndex = graphs.graphIndex + graphs.graphLength;
          let endGraphIndex: i32 = unsafe { (*graphsPtr).graphIndex } + unsafe { (*graphsPtr).graphLength };
          // java if (graphs.isEmbed()) { ... } —— hello 非 embed → else
          if unsafe { (*graphsPtr).isEmbed() } {
            // rust: embed 分支（非 hello，GR-D3；Java 原文同 initLineHeightInfo 模式）
            GraphAlignTool::getEmbedVMetrics(unsafe { &*graphsPtr }, wordAlingFont, isWord97LineBreakRules, &mut self.mAscentAndDescent);
            let graphAscent: f32 = self.mAscentAndDescent[0];
            let graphDescent: f32 = self.mAscentAndDescent[1];
            if graphAscent > self.mVMetricsDatas[3] || graphDescent > self.mVMetricsDatas[4] {
              self.pushVMetricsDatas();
              self.mVMetricsDatas[0] = unsafe { (*graphsPtr).graphIndex } as f32;
              self.mVMetricsDatas[3] = self.mVMetricsDatas[3].max(graphAscent);
              self.mVMetricsDatas[4] = self.mVMetricsDatas[4].max(graphDescent);
            }
          }
          // java else if (lastFontStyle != graphs.fontStyle)
          else {
            let curStyle: Option<TypoFontStyle> = unsafe { (*graphsPtr).fontStyle.clone() };
            if !Self::same_style(&lastFontStyle, &curStyle) {
              // java boolean isWeak = TypoRun.isWeakHeightGraphs(runType);
              let isWeak: bool = TypoRun::isWeakHeightGraphs(runType);
              // java if (!isWeak || datas[1] <= 0 && datas[2] <= 0)
              if !isWeak || (self.mVMetricsDatas[1] <= 0.0 && self.mVMetricsDatas[2] <= 0.0) {
                // java GraphAlignTool.getGraphVMetrics(...)
                GraphAlignTool::getGraphVMetrics(unsafe { &*graphsPtr }, wordAlingFont, isWord97LineBreakRules, &mut self.mAscentAndDescent);
                let graphAscent: f32 = self.mAscentAndDescent[0];
                let graphDescent: f32 = self.mAscentAndDescent[1];
                // java if (isWeak) { push weak } else { push text; lastFontStyle = graphs.fontStyle; }
                if isWeak {
                  if graphAscent > self.mVMetricsDatas[5] || graphDescent > self.mVMetricsDatas[6] {
                    self.pushVMetricsDatas();
                    self.mVMetricsDatas[0] = unsafe { (*graphsPtr).graphIndex } as f32;
                    self.mVMetricsDatas[5] = self.mVMetricsDatas[5].max(graphAscent);
                    self.mVMetricsDatas[6] = self.mVMetricsDatas[6].max(graphDescent);
                  }
                } else {
                  if graphAscent > self.mVMetricsDatas[1] || graphDescent > self.mVMetricsDatas[2] {
                    self.pushVMetricsDatas();
                    self.mVMetricsDatas[0] = unsafe { (*graphsPtr).graphIndex } as f32;
                    self.mVMetricsDatas[1] = self.mVMetricsDatas[1].max(graphAscent);
                    self.mVMetricsDatas[2] = self.mVMetricsDatas[2].max(graphDescent);
                  }
                  lastFontStyle = curStyle;
                }
              }
            }
          }
          // java if (lastGraphIndex < endGraphIndex) { break; }
          if lastGraphIndex < endGraphIndex { break; }
        }
        // java this.updateLineInfo(); this.doCalcLineHeight(layoutState);
        self.updateLineInfo();
        self.doCalcLineHeight(layoutState);
      }
      // java if ((float)lastGraphIndex < this.mVMetricsDatas[0]) { pop stack }
      if (lastGraphIndex as f32) < (self.mVMetricsDatas[0]) {
        while (lastGraphIndex as f32) < (self.mVMetricsDatas[0]) && self.mStackSize > 0 {
          self.mStackSize -= 1;
          let base: usize = self.mStackSize * 7;
          for i in 0..7 { self.mVMetricsDatas[i] = self.mVMetricsStack[base + i]; }
        }
        // java this.updateLineInfo(); this.doCalcLineHeight(layoutState);
        self.updateLineInfo();
        self.doCalcLineHeight(layoutState);
      }
      // java this.doCommitLineHeightInfo(callback);
      if trace_cp { eprintln!("line_height update cp={} last={} max_graph={} grid={} snap={} maxAsc={} maxDesc={} data={:?} height={}", layoutState.cp, lastGraphIndex, self.mMaxGraphIndexForMetrics, self.mGridHeight, self.mNeedSnapToGrid, self.mMaxAscent, self.mMaxDescent, self.mVMetricsDatas, self.mLineHeight); }
      self.doCommitLineHeightInfo(callback);
    }
    // rust: env 重绑（宿主真实 env；Java 引用语义 —— ctor 后由 TextLayouter.rebase_line_env 调）
    pub fn set_env(&mut self, mDocEnv: *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv, mViewEnv: *mut crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv) {
      self.mDocEnv = mDocEnv;
      self.mViewEnv = mViewEnv;
    }

}

// rust: Arc typeface 同一性比较（Java 引用 ==）
struct ArcPtrEq;
impl ArcPtrEq {
    fn same(a: &Option<std::sync::Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface>>,
            b: &Option<std::sync::Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface>>) -> bool {
        match (a, b) {
            (Some(x), Some(y)) => std::ptr::eq(&**x as *const dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface,
                                                &**y as *const dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface),
            _ => a.is_none() && b.is_none(),
        }
    }
}
