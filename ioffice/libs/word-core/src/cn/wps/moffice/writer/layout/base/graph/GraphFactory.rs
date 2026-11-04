// Java: cn.wps.moffice.writer.layout.base.graph.GraphFactory
// rust: ★wr-29 04 —— 真实移植（Java 1160 行）：hello/ASCII 主路径全真实
//       （createNodes/addSpaceGraph/calcWidths 量宽、parseAscii/parseDefault 归类、
//       LangLayout 分配+度量）；数学/符号/绘图/脚注/评论/双向 DEFERRED 停车（GR-D3）。
//       继承无虚分派 → GraphFactory{ super_: GraphReader } 组合。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use std::sync::Arc;

// Java: import cn.wps.font.ITypeface
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.TypoPara
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.moffice.writer.core.CharParse
use crate::cn::wps::moffice::writer::core::CharParse::{CharParse, CharInfo};
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCache
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleCache::FontStyleCache;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.base.ViewMode
use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphReader
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.graph.CharsNode
use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
// Java: import cn.wps.moffice.writer.layout.base.languages.ILangLayout
use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::ILangLayout;
// Java: import cn.wps.moffice.writer.layout.base.languages.LangLayoutFactory
use crate::cn::wps::moffice::writer::layout::base::languages::LangLayoutFactory::LangLayoutFactory;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.punctSpace.PunctTool
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::PunctTool::PunctTool;

// java public final class GraphFactory extends GraphReader implements ICapsCallback
pub struct GraphFactory {
    // java GraphReader super_
    pub(crate) super_: GraphReader,
    // java private CharParse mCharParse = CharParse.getInstance();
    pub(crate) mCharParse: CharParse,
    // java private CharParse.CharInfo mCharInfo = new CharParse.CharInfo();
    pub(crate) mCharInfo: CharInfo,
    // java private int mCurLang = 0;
    pub(crate) mCurLang: i32,
    // java private int mFootEndNoteRefType;
    pub(crate) mFootEndNoteRefType: i32,
    // java private char mLastHighSurrogate;
    pub(crate) mLastHighSurrogate: char,
    // java private ILangLayout mLangLayout;
    pub(crate) mLangLayout: Box<dyn ILangLayout>,
    // java private ILangLayout.GraphOperationEnv mGraphOperationEnv;
    pub(crate) mGraphOperationEnv: crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::GraphOperationEnv,
    // java private int[] mSpaceWidth;
    pub(crate) mSpaceWidth: [i32; 1],
    // java private int mCacheSpaceEmSize;
    pub(crate) mCacheSpaceEmSize: i32,
    // java private ITypeface mCacheTypefaceForSpace;
    pub(crate) mCacheTypefaceForSpace: Option<Arc<dyn ITypeface>>,
}

fn factory_create_nodes(this: *mut (), chars: &mut CharsNode, graph_index: i32, graph_length: i32, layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.createNodes(chars, graph_index, graph_length, layout);
}
fn factory_commit_all(this: *mut (), layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.commitAll(layout);
}
fn factory_commit_graph_node(this: *mut (), layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.commitGraphNode(layout);
}
fn factory_set_word_align_font(this: *mut (), word_align_font: i32, layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.setWordAlignFont(word_align_font, layout);
}
fn factory_create_equation_nodes(this: *mut (), chars: &mut CharsNode, graph_index: i32, start: i32, end: i32, enable_revision: bool, revision_final: bool, layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.createEquationNodes(chars, graph_index, start, end, enable_revision, revision_final, layout);
}
fn factory_create_math_nodes(this: *mut (), chars: &mut CharsNode, graph_index: i32, math_node: crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::MathEmbed, layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.createMathNodes(chars, graph_index, math_node, layout);
}
fn factory_set_calc_graph_height(this: *mut (), val: bool) {
    unsafe { &mut *(this as *mut GraphFactory) }.setCalcGraphHeight(val);
}
fn factory_set_math_underline(this: *mut (), underline: i32, color: i32) {
    unsafe { &mut *(this as *mut GraphFactory) }.setMathUnderline(underline, color);
}
fn factory_create_embed_nodes(this: *mut (), chars: &mut CharsNode, shape_id: Option<i32>, graph_index: i32, cp: i32, need_anchor: bool, layout: &mut LayoutState) {
    unsafe { &mut *(this as *mut GraphFactory) }.createEmbedNodes(chars, shape_id, graph_index, cp, need_anchor, layout);
}

impl GraphFactory {
    // Java `this` for GraphReader.createNodes/commitAll after the value is stable.
    pub fn bind_factory_dispatch(&mut self) {
        self.super_.factory_self = self as *mut GraphFactory as *mut ();
        self.super_.factory_create_nodes = Some(factory_create_nodes);
        self.super_.factory_commit_all = Some(factory_commit_all);
        self.super_.factory_commit_graph_node = Some(factory_commit_graph_node);
        self.super_.factory_set_word_align_font = Some(factory_set_word_align_font);
        self.super_.factory_create_equation_nodes = Some(factory_create_equation_nodes);
        self.super_.factory_create_math_nodes = Some(factory_create_math_nodes);
        self.super_.factory_set_calc_graph_height = Some(factory_set_calc_graph_height);
        self.super_.factory_set_math_underline = Some(factory_set_math_underline);
        self.super_.factory_create_embed_nodes = Some(factory_create_embed_nodes);
    }

    // Java GraphBuffFactory.createGraphBuff shares the caller's envs. Owned Default copies stay empty.
    pub fn from_env_alias(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
      let mut this = Self::GraphFactory(MainState::default(), DocumentEnv::default(), ViewEnv::default());
      this.super_.super_.bind_env_alias(main, doc, view);
      if !view.is_null() && unsafe { (*view).isPhoneReadMode() } {
        crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory::setMaxFontSize(520);
      }
      this
    }

    // java public GraphFactory(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn GraphFactory(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java super(mainState, docEnv, viewEnv);
      let mut this = Self {
        super_: GraphReader::GraphReader(mainState, docEnv, viewEnv),
        mCharParse: CharParse::CharParse(),
        mCharInfo: CharInfo::default(),
        mCurLang: 0,
        mFootEndNoteRefType: 0,
        mLastHighSurrogate: '\0',
        mLangLayout: LangLayoutFactory::getInstance(0),
        mGraphOperationEnv: Default::default(),
        mSpaceWidth: [0],
        mCacheSpaceEmSize: 0,
        mCacheTypefaceForSpace: None,
      };
      // java this.mLangLayout = LangLayoutFactory.getInstance(this.mCurLang);
      this.mLangLayout = LangLayoutFactory::getInstance(0);
      // java this.mGraphOperationEnv = new ILangLayout.GraphOperationEnv();
      // java this.mSpaceWidth = new int[1];
      // rust: ★wr-29 04 —— documentEnv/typoFontStylePool 指针不在 ctor 捕获（Rust 构造后
      //       struct 整体移动 → 旧地址悬垂）。在 updateGraphOperationEnv（createNodes 入口）
      //       以最终位置刷新（Java 语义仍是对象引用，等价）。
      // java this.mGraphOperationEnv.blockResult = this.mBlockResult;
      // rust: opEnv 自持 blockResult（Java 为 mBlockResult 别名）
      // java this.mGraphOperationEnv.nonBMPChars = new char[2];
      // java this.mGraphOperationEnv.highSurrogateType = 30;
      this.mGraphOperationEnv.highSurrogateType = 30;
      // java this.mGraphOperationEnv.lowSurrogateType = 31;
      this.mGraphOperationEnv.lowSurrogateType = 31;
      // java this.mGraphOperationEnv.mathUnderlineColor = -16777216;
      this.mGraphOperationEnv.mathUnderlineColor = -16777216;
      // java this.mGraphOperationEnv.runType = 1;
      this.mGraphOperationEnv.runType = 1;
      // java this.mGraphOperationEnv.charInfo = this.mCharInfo;
      this.mGraphOperationEnv.charInfo = this.mCharInfo.clone();
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mLangLayout = null;
      self.mLangLayout = LangLayoutFactory::getInstance(0);
      // java if (null != this.mGraphOperationEnv) { this.mGraphOperationEnv.dispose(); this.mGraphOperationEnv = null; }
      // rust: opEnv 值成员；dispose 仅清 arabic
      self.mGraphOperationEnv.dispose();
    }

    // java public void setIgnoreLeading(boolean ingoreLeading)
    pub fn setIgnoreLeading(&mut self, ingoreLeading: bool) {
      // java this.mGraphOperationEnv.isIgnoreLeading = ingoreLeading;
      self.mGraphOperationEnv.isIgnoreLeading = ingoreLeading;
    }

    // java public void setMathContent(boolean val)
    pub fn setMathContent(&mut self, val: bool) {
      // java this.mGraphOperationEnv.isMathContent = val;
      self.mGraphOperationEnv.isMathContent = val;
    }

    // java protected void setWordAlignFont(int wordAlignFont, LayoutState layoutState)
    pub fn setWordAlignFont(&mut self, wordAlignFont: i32, layoutState: &mut LayoutState) {
      // java if (null != this.mGraphOperationEnv.arabicNodeList) { this.mGraphOperationEnv.arabicNodeList.clean(); }
      // rust: arabicNodeList 未接线（GR-D3）→ 恒空
      let _ = (wordAlignFont, layoutState);
    }

    // java public void scaleChar()
    pub fn scaleChar(&mut self) {
      // java this.mGraphOperationEnv.isScralForSmallCaps = true;
      self.mGraphOperationEnv.isScralForSmallCaps = true;
    }

    // java protected final void createNodes(CharsNode charsNode, int graphIndex, int graphLength, LayoutState layoutState)
    pub fn createNodes(&mut self, charsNode: &CharsNode, graphIndex: i32, graphLength: i32, layoutState: &mut LayoutState) {
      // java this.updateGraphOperationEnv(charsNode, layoutState);
      self.updateGraphOperationEnv(charsNode, layoutState);
      // java boolean isPageMode = ViewMode.isPageMode(this.mViewEnv.getViewMode());
      let isPageMode: bool = ViewMode::isPageMode(self.super_.super_.mViewEnv.as_ref().map(|v| v.getViewMode()).unwrap_or(0));
      // java int curGraphIndex = graphIndex;
      let mut curGraphIndex: i32 = graphIndex;
      // java int endGraphIndex = graphIndex + graphLength;
      let mut endGraphIndex: i32 = graphIndex + graphLength;
      // java TypoPara typoPara = layoutState.typoPara;
      // java int[] charsBuff = typoPara.getCharBlock(graphIndex, this.mBlockResult);
      let mut blockResult = self.super_.mBlockResult;
      // rust: 块拷贝为 Vec（Java 直接持 snapshot 数组引用；Rust 借用分割需 owned）
      let mut charsBuff: Vec<char> = layoutState.typoPara.getCharBlock(curGraphIndex, &mut blockResult).to_vec();
      // java int buffOffset = this.mBlockResult[0];
      let mut buffOffset: i32 = blockResult[0];
      // java int buffLimit = this.mBlockResult[1];
      let mut buffLimit: i32 = blockResult[1];

      // java while(curGraphIndex < endGraphIndex)
      while curGraphIndex < endGraphIndex {
        // java if (buffOffset >= buffLimit)
        if buffOffset >= buffLimit {
          // java charsBuff = typoPara.getCharBlock(curGraphIndex, this.mBlockResult);
          charsBuff = layoutState.typoPara.getCharBlock(curGraphIndex, &mut blockResult).to_vec();
          // java buffOffset = this.mBlockResult[0];
          buffOffset = blockResult[0];
          // java buffLimit = this.mBlockResult[1];
          buffLimit = blockResult[1];
        }
        // java int ch = charsBuff[buffOffset];
        let ch: i32 = charsBuff[buffOffset as usize] as u32 as i32;
        // java int prevRunType = this.mGraphOperationEnv.runType;
        let prevRunType: i32 = self.mGraphOperationEnv.runType;
        // java this.parseChar(ch, curGraphIndex, layoutState);
        self.parseChar(ch, curGraphIndex, layoutState);
        // java this.mGraphOperationEnv.isScralForSmallCaps = false;
        self.mGraphOperationEnv.isScralForSmallCaps = false;
        // java int graphLen = 1;
        let mut graphLen: i32 = 1;
        // java switch (this.mGraphOperationEnv.runType)
        match self.mGraphOperationEnv.runType {
          // java case 2: case 17:
          2 | 17 => {
            // java int nextGraphIndex = curGraphIndex + 1; int nextBlockOffset = buffOffset + 1; int nextBuffLimit = buffLimit;
            let mut nextGraphIndex: i32 = curGraphIndex + 1;
            let mut nextBlockOffset: i32 = buffOffset + 1;
            let mut nextBuffLimit: i32 = buffLimit;
            let mut nextCharsBuff: Vec<char> = charsBuff.clone();
            // java for(int[] nextCharsBuff = charsBuff; nextGraphIndex < endGraphIndex; ++nextBlockOffset)
            while nextGraphIndex < endGraphIndex && { let _ = nextBlockOffset; nextBlockOffset += 0; nextGraphIndex < endGraphIndex } {
              // java if (nextBlockOffset >= nextBuffLimit)
              if nextBlockOffset >= nextBuffLimit {
                nextCharsBuff = layoutState.typoPara.getCharBlock(nextGraphIndex, &mut blockResult).to_vec();
                nextBlockOffset = blockResult[0];
                nextBuffLimit = blockResult[1];
              }
              // java int curCh = nextCharsBuff[nextBlockOffset];
              let curCh: i32 = nextCharsBuff[nextBlockOffset as usize] as u32 as i32;
              // java if (curCh != ch) { break; }
              if curCh != ch { break; }
              // java ++graphLen;
              graphLen += 1;
              // java ++nextGraphIndex;
              nextGraphIndex += 1;
              // java ++nextBlockOffset;
              nextBlockOffset += 1;
            }
            // java this.addSpaceGraph(charsNode, curGraphIndex, graphLen, layoutState);
            self.addSpaceGraph(charsNode, curGraphIndex, graphLen, layoutState);
            // java curGraphIndex += graphLen;
            curGraphIndex += graphLen;
            // java buffOffset += graphLen;
            buffOffset += graphLen;
          }
          // java case 5: (页断 '\f')
          5 => {
            // java this.commitAll(layoutState);
            self.commitAll(layoutState);
            // java int cp = this.getCP(curGraphIndex);
            let cp: i32 = self.super_.super_.getCP(curGraphIndex);
            static BREAK_TRACE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            if std::env::var_os("WORD_PARITY_TRACE_BOUNDS").is_some()
              && BREAK_TRACE.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 12 {
              eprintln!("break-graph cp={} graph={} mapSize={} map={:?} secStart={} secEnd={} breakCode={} isPageMode={}", cp, curGraphIndex,
                self.super_.super_.mMapSize, &self.super_.super_.mMapData[..(self.super_.super_.mMapSize.max(0) as usize * 4).min(self.super_.super_.mMapData.len())],
                self.super_.super_.mMainState.as_ref().map(|m| m.secStart).unwrap_or(-1),
                self.super_.super_.mMainState.as_ref().map(|m| m.secEnd).unwrap_or(-1),
                self.super_.super_.mMainState.as_ref().map(|m| m.secBreakCode).unwrap_or(-1), isPageMode);
            }
            // java MainState mainState = this.mMainState;
            // java if (isPageMode && cp == mainState.secEnd - 1)
            if isPageMode && cp == self.super_.super_.mMainState.as_ref().map(|m| m.secEnd - 1).unwrap_or(i32::MIN) {
              // java switch (mainState.secBreakCode) { ... }
              match self.super_.super_.mMainState.as_ref().map(|m| m.secBreakCode).unwrap_or(0) {
                // java case 0: this.mGraphOperationEnv.runType = 11; break;
                0 => { self.mGraphOperationEnv.runType = 11; }
                // java case 1: case 2: this.mGraphOperationEnv.runType = 10; break;
                1 | 2 => { self.mGraphOperationEnv.runType = 10; }
                // java case 3: this.mGraphOperationEnv.runType = 13; break;
                3 => { self.mGraphOperationEnv.runType = 13; }
                // java case 4: this.mGraphOperationEnv.runType = 12;
                _ => { self.mGraphOperationEnv.runType = 12; }
              }
            }
            // java this.mLangLayout.addCommonGraph(curGraphIndex, this.mGraphOperationEnv);
            self.mLangLayout.addCommonGraph(curGraphIndex, &mut self.mGraphOperationEnv);
            // java ++curGraphIndex; ++buffOffset;
            curGraphIndex += 1;
            buffOffset += 1;
          }
          // java case 6: this.commitGraphNode; ShapeTool.findShape; createEmbedNodes/removeGraph.
          6 => {
            // Java: case 6 — commit, ShapeTool.findShape, createEmbedNodes or removeGraph.
            self.commitGraphNode(layoutState);
            let cp = self.super_.super_.getCP(curGraphIndex);
            let shape_id = layoutState.document.and_then(|doc| shape_id_at(unsafe { &mut *doc }, cp));
            if std::env::var_os("WORD_PARITY_TRACE_EMBED").is_some() {
              eprintln!("EMBED_GRAPH branch cp={cp} graph_index={curGraphIndex} shape_id={shape_id:?}");
            }
            if let Some(shape_id) = shape_id {
              // java createEmbedNodes(..., cp, false, layoutState); then ++curGraphIndex even if the node is hidden.
              self.createEmbedNodes(charsNode, Some(shape_id), curGraphIndex, cp, false, layoutState);
              curGraphIndex += 1;
              buffOffset += 1;
            } else {
              endGraphIndex -= 1;
              self.super_.super_.removeGraph(curGraphIndex, layoutState);
            }
          }
          // java case 15: (emoji)
          15 => {
            // rust: ★emoji DEFERRED（GR-D3），Java 原文见 case 15
            /* DEFERRED_NON_FUNCTION 加 commonGraph */
            self.commitAll(layoutState);
            curGraphIndex += 1;
            buffOffset += 1;
          }
          // java case 30/31/32/33/34/35/36/37: 非 hello（代理对/绘图/评论/双向/脚注/不可见/ZWSP）
          30..=37 => {
            // rust: ★wr-29 GR-D3 —— 非 hello 分支停车（Java 原文见 GraphFactory.java case 30-37；
            //       图中推进与默认一致）
            curGraphIndex += 1;
            buffOffset += 1;
          }
          // java case 3: case 4: case 7..29: default:
          _ => {
            // java int addLen = this.mLangLayout.addLangGraph(curGraphIndex, endGraphIndex, buffOffset, buffLimit, charsBuff, this.mGraphOperationEnv);
            let addLen: i32 = self.mLangLayout.addLangGraph(curGraphIndex, endGraphIndex, buffOffset, buffLimit, &charsBuff.iter().map(|c| *c as u32 as i32).collect::<Vec<_>>(), &mut self.mGraphOperationEnv);
            // java curGraphIndex += addLen;
            curGraphIndex += addLen;
            // java buffOffset += addLen;
            buffOffset += addLen;
          }
        }
      }
      // java if (null != this.mGraphOperationEnv.langGraphNode && !this.mLangLayout.hasMergeChar()) { this.mLangLayout.commitLangGraphNode(this.mGraphOperationEnv); }
      if !self.mGraphOperationEnv.langGraphNode.is_null() && !self.mLangLayout.hasMergeChar() {
        self.mLangLayout.commitLangGraphNode(&mut self.mGraphOperationEnv);
      }
      // java if (null != this.mGraphOperationEnv.commonGraphNode) { this.mLangLayout.commitCommonGraphNode(this.mGraphOperationEnv); }
      if !self.mGraphOperationEnv.commonGraphNode.is_null() {
        self.mLangLayout.commitCommonGraphNode(&mut self.mGraphOperationEnv);
      }
    }

    // java private void addSpaceGraph(CharsNode charsNode, int graphIndex, int graphLen, LayoutState layoutState)
    pub fn addSpaceGraph(&mut self, charsNode: &CharsNode, graphIndex: i32, graphLen: i32, layoutState: &mut LayoutState) {
      // java this.commitAll(layoutState);
      self.commitAll(layoutState);
      // java Graphs graphNode = this.mLangLayout.addGraphNode(graphIndex, graphLen, this.mGraphOperationEnv);
      let graphNode: *mut Graphs = self.mLangLayout.addGraphNode(graphIndex, graphLen, &mut self.mGraphOperationEnv);
      // java TypoFontStyle fontStyle = charsNode.fontStyles[this.mCurLang];
      let mut fontStyle: TypoFontStyle = match charsNode.fontStyles.get(self.mCurLang as usize) {
        Some(Some(f)) => f.clone(),
        _ => TypoFontStyle::newInstance(),
      };
      // java if (null == fontStyle)
      if charsNode.fontStyles.get(self.mCurLang as usize).map(|o| o.is_none()).unwrap_or(true) {
        // java PropertySet runProp = charsNode.finalRunProp;
        let runProp: PropertySet = charsNode.finalRunProp.clone().unwrap_or_default();
        // java FontStyleCache cache = this.getFontStyleCache(layoutState);
        // java fontStyle = cache.get(runProp, this.mCurLang, layoutState.styleResult, this.mTypoFontStylePool);
        // java charsNode.fontStyles[this.mCurLang] = fontStyle;
        // rust: 拆借 —— pool 以裸指针侧入（cache 与 pool 同属 self 的子字段，借用分割）
        let lang: i32 = self.mCurLang;
        let pool_ptr: *mut crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStylePool = {
            let gr = &mut self.super_;
            let gb = &mut gr.super_;
            &mut gb.mTypoFontStylePool as *mut _
        };
        if let Some(cache) = self.getFontStyleCache(layoutState) {
          fontStyle = cache.get(&runProp, lang, Some(&mut layoutState.styleResult), Some(unsafe { &mut *pool_ptr }));
          let _ = charsNode;
        }
      }
      // java if (this.mIsInMath)
      if self.super_.mIsInMath {
        // java fontStyle = this.mLangLayout.processMathUnderline(fontStyle, this.mGraphOperationEnv);
        fontStyle = self.mLangLayout.processMathUnderline(fontStyle, &mut self.mGraphOperationEnv);
      }
      // java if (this.mGraphOperationEnv.isCalcGraphHeight)
      if self.mGraphOperationEnv.isCalcGraphHeight {
        // rust: 05 行高阶段；Java 原文可得
        /* DEFERRED_NON_FUNCTION
        // java fontStyle = this.mTypoFontStylePool.obtain(fontStyle); fontStyle.maxAscent = fontStyle.fontSize / 2; fontStyle.maxDescent = 0;
        */
      }
      // java graphNode.fontStyle = fontStyle;
      if let Some(g) = unsafe { graphNode.as_mut() } {
        g.fontStyle = Some(fontStyle);
        // java float emsize = fontStyle.srcEmSize;
        let emsize: f32 = g.fontStyle.as_ref().unwrap().srcEmSize;
        // java float widthPerChar = emsize;
        let mut widthPerChar: f32 = emsize;
        // java if (17 != this.mGraphOperationEnv.runType)
        if 17 != self.mGraphOperationEnv.runType {
          // java widthPerChar = this.getSpaceEmSize(fontStyle.typeface);
          widthPerChar = self.getSpaceEmSize(g.fontStyle.as_ref().unwrap().typeface.clone());
        }
        // java widthPerChar = widthPerChar * fontStyle.charScale * (float)fontStyle.fontSize / emsize;
        let fontStyle = g.fontStyle.as_ref().unwrap();
        widthPerChar = widthPerChar * fontStyle.charScale * fontStyle.fontSize as f32 / emsize;
        // java if (fontStyle.fontSpace != 0)
        let mut fontSpace_extra: f32 = 0.0;
        if fontStyle.fontSpace != 0 {
          // java widthPerChar = Math.max(0.0F, widthPerChar + (float)fontStyle.fontSpace);
          widthPerChar = (widthPerChar + fontStyle.fontSpace as f32).max(0.0);
        }
        let _ = &mut fontSpace_extra;
        // java layoutState.typoPara.fillWidths(graphIndex, graphLen, (int)(widthPerChar + 0.5F));
        layoutState.typoPara.fillWidths(graphIndex, graphLen, (widthPerChar + 0.5) as i32);
        // java graphNode.mWidth = (int)(widthPerChar * (float)graphLen + 0.5F);
        g.mWidth = (widthPerChar * graphLen as f32 + 0.5) as i32;
        // java if (this.needTagForTransToSpace(layoutState)) { layoutState.tagForSpaceTrans = true; }
        if self.needTagForTransToSpace(layoutState) { layoutState.tagForSpaceTrans = true; }
      }
    }

    // java private float getSpaceEmSize(ITypeface typeface)
    pub fn getSpaceEmSize(&mut self, typeface: Option<Arc<dyn ITypeface>>) -> f32 {
      // java if (this.mCacheTypefaceForSpace == typeface) { return (float)this.mCacheSpaceEmSize; }
      let same = match (&self.mCacheTypefaceForSpace, &typeface) {
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
      };
      if same {
        return self.mCacheSpaceEmSize as f32;
      }
      // java else { ... typeface.getWriterLayoutAdavancesEM(nonBMPChars, 0, this.mSpaceWidth, 0, 1); }
      if let Some(tf) = &typeface {
        self.mCacheTypefaceForSpace = Some(tf.clone());
        self.mGraphOperationEnv.nonBMPChars[0] = ' ';
        let mut u16s = [0u16; 1];
        u16s[0] = ' ' as u16;
        self.mGraphOperationEnv.blockResult[0] = 0;
        tf.getWriterLayoutAdavancesEM_chars(&mut u16s, 0, &mut self.mSpaceWidth, 0, 1);
        self.mCacheSpaceEmSize = self.mSpaceWidth[0];
      }
      // java return (float)(this.mCacheSpaceEmSize = this.mSpaceWidth[0]);
      self.mCacheSpaceEmSize as f32
    }

    // java private void addZeroWidthSpaceGraph(CharsNode charsNode, int graphIndex, int graphLen, LayoutState layoutState)
    pub fn addZeroWidthSpaceGraph(&mut self, charsNode: &CharsNode, graphIndex: i32, graphLen: i32, layoutState: &mut LayoutState) {
      // java this.commitAll(layoutState);
      self.commitAll(layoutState);
      // java Graphs graphNode = this.mLangLayout.addGraphNode(graphIndex, graphLen, this.mGraphOperationEnv);
      let graphNode: *mut Graphs = self.mLangLayout.addGraphNode(graphIndex, graphLen, &mut self.mGraphOperationEnv);
      // java graphNode.runType = 1;
      if let Some(g) = unsafe { graphNode.as_mut() } { g.runType = 1; }
      // java TypoFontStyle fontStyle = charsNode.fontStyles[this.mCurLang];
      let mut fontStyle: TypoFontStyle = match charsNode.fontStyles.get(self.mCurLang as usize) {
        Some(Some(f)) => f.clone(),
        _ => TypoFontStyle::newInstance(),
      };
      // java if (null == fontStyle)
      if charsNode.fontStyles.get(self.mCurLang as usize).map(|o| o.is_none()).unwrap_or(true) {
        // java PropertySet runProp = charsNode.finalRunProp;
        let runProp: PropertySet = charsNode.finalRunProp.clone().unwrap_or_default();
        // java FontStyleCache cache = this.getFontStyleCache(layoutState);
        let lang: i32 = self.mCurLang;
        let pool_ptr: *mut crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStylePool = {
            let gr = &mut self.super_;
            let gb = &mut gr.super_;
            &mut gb.mTypoFontStylePool as *mut _
        };
        if let Some(cache) = self.getFontStyleCache(layoutState) {
          // java fontStyle = cache.get(runProp, this.mCurLang, layoutState.styleResult, this.mTypoFontStylePool);
          fontStyle = cache.get(&runProp, lang, Some(&mut layoutState.styleResult), Some(unsafe { &mut *pool_ptr }));
        }
      }
      // java graphNode.fontStyle = fontStyle;
      if let Some(g) = unsafe { graphNode.as_mut() } { g.fontStyle = Some(fontStyle); }
      // java layoutState.typoPara.fillWidths(graphIndex, graphLen, 0);
      layoutState.typoPara.fillWidths(graphIndex, graphLen, 0);
      // java graphNode.mWidth = 0;
      if let Some(g) = unsafe { graphNode.as_mut() } { g.mWidth = 0; }
    }

    // java protected Graphs createEmbedNodes(CharsNode charsNode, Shape shape, int graphIndex, int cp, boolean needFindAnchorShape, LayoutState layoutState)
    pub fn createEmbedNodes(&mut self, charsNode: &CharsNode, shape_id: Option<i32>, graphIndex: i32, cp: i32, needFindAnchorShape: bool, layoutState: &mut LayoutState) -> Option<*mut Graphs> {
      // java Document doc = charsNode.srcRef.doc; if (null == doc) return null;
      let doc = match charsNode.srcRef.and_then(|src| unsafe { (*src).doc }).filter(|p| !p.is_null()) {
        Some(doc) => unsafe { &mut *doc },
        None => return None,
      };
      // java else if (null != shape && !shape.getHidden())
      let Some(mut shape_id) = shape_id.filter(|id| !shape_is_hidden(doc, *id)) else { return None };
      // java if (cp > 0 && needFindAnchorShape) { field shape + char '\\b' replaces shape with the previous CP }
      if cp > 0 && needFindAnchorShape {
        match field_anchor_shape(doc, cp, shape_id) {
          Some(id) => shape_id = id,
          None => return None,
        }
      }
      // java shape = shape.cloneLightWeight(); CloneNotSupportedException returns null.
      if shape_clone_failed(doc, shape_id) {
        return None;
      }
      // Java: updateGraphOperationEnv; hasEmbeds; putChar(' '); addGraphNode; runType = 6.
      self.updateGraphOperationEnv(charsNode, layoutState);
      layoutState.hasEmbeds = true;
      layoutState.typoPara.putChar(graphIndex, ' ');
      let graphNode: *mut Graphs = self.mLangLayout.addGraphNode(graphIndex, 1, &mut self.mGraphOperationEnv);
      let fontStyle = embed_font_style(self, charsNode, layoutState);
      if let Some(g) = unsafe { graphNode.as_mut() } {
        g.runType = 6;
        g.autoSpaceType = 0;
        g.fontStyle = Some(fontStyle);
      }
      // Java: addEmbed(graphNode, shape) retains the original Graphs identity.
      // LayoutState.graphList owns this stable Box node until GraphBuff.clear.
      if !graphNode.is_null() {
        self.super_.super_.addEmbed(graphNode, shape_id);
      }
      Some(graphNode)
    }

    // java private void createFootEndNotes(...)
    pub fn createFootEndNotes(&mut self, _charsNode: &CharsNode, _graphIndex: i32, _cp: i32, _layoutState: &mut LayoutState) {
      // rust: ★wr-29 GR-D3 —— 脚注/尾注非 hello
      /* DEFERRED_NON_FUNCTION （Java 原文见 GraphFactory.java:469-507） */
    }

    // java protected void commitVanishBreak(LayoutState layoutState)
    pub fn commitVanishBreak(&mut self, layoutState: &mut LayoutState) {
      // java if (null != this.mGraphOperationEnv.commonGraphNode)
      if !self.mGraphOperationEnv.commonGraphNode.is_null() {
        // java Graphs graphNode = this.mGraphOperationEnv.commonGraphNode;
        let graphNode: *mut Graphs = self.mGraphOperationEnv.commonGraphNode;
        // java graphNode.mWidth = 0;
        if let Some(g) = unsafe { graphNode.as_mut() } { g.mWidth = 0; }
        // java TypoFontStyle fontStyle = graphNode.fontStyle;
        let mut fontStyle: TypoFontStyle = unsafe { &*graphNode }.fontStyle.clone().unwrap_or_default();
        let fontStyle_for_node: TypoFontStyle = fontStyle.clone();
        // java FontStyleCache fontStyleCache = this.getFontStyleCache(layoutState);
        // java if (fontStyleCache.contains(fontStyle)) { fontStyle = this.mTypoFontStylePool.obtain(fontStyle); graphNode.fontStyle = fontStyle; }
        if let Some(cache) = self.getFontStyleCache(layoutState) {
          if cache.contains_style(Some(&fontStyle)) {
            fontStyle = self.super_.super_.mTypoFontStylePool.obtain(Some(&fontStyle));
          }
        }
        // java fontStyle.fontSize = 0; fontStyle.maxAscent = 0; fontStyle.maxDescent = 0;
        fontStyle.fontSize = 0;
        fontStyle.maxAscent = 0;
        fontStyle.maxDescent = 0;
        let _ = fontStyle_for_node;
        if let Some(g) = unsafe { graphNode.as_mut() } { g.fontStyle = Some(fontStyle); }
        // java this.mGraphOperationEnv.commonGraphNode = null;
        self.mGraphOperationEnv.commonGraphNode = std::ptr::null_mut();
      }
    }

    // java private FontStyleCache getFontStyleCache(LayoutState layoutState)
    pub fn getFontStyleCache(&mut self, layoutState: &mut LayoutState) -> Option<&mut FontStyleCache> {
      // java FontStyleCache cache = this.mDocEnv.fontStyleCacheFactory.create(layoutState.document);
      // rust: borrow 经 mDocEnv.fontStyleCacheFactory（create_borrowed 别名）
      let de = self.super_.super_.aliased_doc()?;
      let factory = de.fontStyleCacheFactory.as_mut()?;
      Some(factory.create_borrowed(layoutState.document))
    }

    // java private void parseChar(int codePoint, int graphIndex, LayoutState layoutState)
    pub fn parseChar(&mut self, codePoint: i32, graphIndex: i32, layoutState: &mut LayoutState) {
      // java int oldLang = this.mCurLang;
      let oldLang: i32 = self.mCurLang;
      // java int oldRunType = this.mGraphOperationEnv.runType;
      let oldRunType: i32 = self.mGraphOperationEnv.runType;
      // java this.mGraphOperationEnv.isNoWidth = false;
      self.mGraphOperationEnv.isNoWidth = false;
      // java this.mGraphOperationEnv.autoSpaceType = 0;
      self.mGraphOperationEnv.autoSpaceType = 0;
      // java this.mGraphOperationEnv.runType = 1;
      self.mGraphOperationEnv.runType = 1;
      // java this.mGraphOperationEnv.isIsolate = false;
      self.mGraphOperationEnv.isIsolate = false;
      // java this.mCurLang = 0;
      self.mCurLang = 0;
      // java this.mCharParse.getCharInfoUseCache(this.mCharInfo, codePoint, true);
      self.mCharParse.getCharInfoUseCache(&mut self.mCharInfo, codePoint, true);
      // java int charType = this.mCharInfo.charType;
      let charType: i32 = self.mCharInfo.charType;
      // java switch (charType)
      match charType {
        // java case 1:
        1 => {
          // java this.parseAscii(codePoint, graphIndex, layoutState);
          self.parseAscii(codePoint, graphIndex, layoutState);
        }
        // java case 2:
        2 => {
          // java this.mCurLang = 2;
          self.mCurLang = 2;
          // java if (codePoint == 12288)
          if codePoint == 12288 {
            // java this.mGraphOperationEnv.runType = 17; this.mGraphOperationEnv.isIsolate = true;
            self.mGraphOperationEnv.runType = 17;
            self.mGraphOperationEnv.isIsolate = true;
          } else {
            // java switch (this.mCharInfo.unicodeCodeType)
            match self.mCharInfo.unicodeCodeType {
              // java case 106: case 160:
              106 | 160 => {
                // java if (PunctTool.isPunct(codePoint)) { this.mGraphOperationEnv.runType = 18; }
                if PunctTool::isPunct(codePoint) { self.mGraphOperationEnv.runType = 18; }
                // java this.mGraphOperationEnv.isIsolate = true;
                self.mGraphOperationEnv.isIsolate = true;
                // java if (codePoint == 65343 && this.needTagForTransToSpace(layoutState)) { this.mGraphOperationEnv.runType = 26; layoutState.tagForSpaceTrans = true; }
                if codePoint == 65343 && self.needTagForTransToSpace(layoutState) {
                  self.mGraphOperationEnv.runType = 26;
                  layoutState.tagForSpaceTrans = true;
                }
              }
              // java case 107: ... case 246:
              107 | 108 | 109 | 110 | 116 | 117 | 119 | 146 | 246 => {
                // java this.mGraphOperationEnv.autoSpaceType = 2;
                self.mGraphOperationEnv.autoSpaceType = 2;
              }
              // java case 148:
              148 => {
                // java this.mLastHighSurrogate = (char)codePoint;
                self.mLastHighSurrogate = char::from_u32(codePoint as u32).unwrap_or('\0');
                // java this.mGraphOperationEnv.runType = 30;
                self.mGraphOperationEnv.runType = 30;
                // java this.mCurLang = 2;
                self.mCurLang = 2;
              }
              // java case 149:
              149 => {
                // java this.mGraphOperationEnv.runType = 31;
                self.mGraphOperationEnv.runType = 31;
              }
              _ => {}
            }
          }
        }
        // java case 3: case 4:
        3 | 4 => {
          if self.mGraphOperationEnv.isProccessMathOp {
            // rust: MathOperatorCharacter 未接线（数学非 hello）—— 走 parseDefault 安全路径
          }
          // java this.parseDefault(codePoint, charType, graphIndex, layoutState);
          self.parseDefault(codePoint, charType, graphIndex, layoutState);
        }
        // java case 5:
        5 => {
          // rust: ★complex/泰文/阿拉伯 归类 DEFERRED；unicodeCodeType 表已接线。
          /* DEFERRED_NON_FUNCTION （Java 原文：isIndic/Thai/isRTL 分支） */
          let _ = self.mCharInfo.unicodeCodeType;
        }
        // java case 6: Unknown → parseDefault
        _ => {
          // java 默认归类 parseDefault（Java default: 同 case 3/4）
          if self.mGraphOperationEnv.isProccessMathOp {
            let _ = ();
          }
          self.parseDefault(codePoint, charType, graphIndex, layoutState);
        }
      }
      // java if (null != this.mLangLayout)
      // java if (oldLang != this.mCurLang) { if (31 != oldRunType) { this.commitAll(layoutState); } this.mLangLayout = LangLayoutFactory.getInstance(this.mCurLang); }
      if oldLang != self.mCurLang {
        if 31 != oldRunType {
          self.commitAll(layoutState);
        }
        self.mLangLayout = LangLayoutFactory::getInstance(self.mCurLang);
      }
    }

    // java private boolean needThaiPlaceHolder(int graphIndex, LayoutState layoutState)
    pub fn needThaiPlaceHolder(&mut self, _graphIndex: i32, _layoutState: &mut LayoutState) -> bool {
      // rust: ★泰文占位 DEFERRED（非 hello）
      /* DEFERRED_NON_FUNCTION （Java 原文见 GraphFactory.java:634-665） */
      false
    }

    // java private int getDefaultRunLang(int charType, CharsNode charsNode)
    pub fn getDefaultRunLang(&mut self, charType: i32, charsNode: &CharsNode) -> i32 {
      // java if (4 == charType)
      if 4 == charType {
        // java int idctHint = charsNode.finalRunProp.getInt(40, 0);
        let idctHint: i32 = charsNode.finalRunProp.as_ref().map(|p| p.getInt__int_int(40, 0)).unwrap_or(0);
        // java switch (idctHint)
        match idctHint {
          // java case 0: return 1;
          0 => { return 1; }
          // java case 1:
          1 => {
            // java Integer lidFe = (Integer)charsNode.finalRunProp.getValue(42);
            let lidFe: Option<&util::moffice::property::PropertySet::PropertyValue> = charsNode.finalRunProp.as_ref().and_then(|p| p.getValue(42));
            // java return null == lidFe || 1049 != lidFe && 2073 != lidFe ? 3 : 1;
            let lidFeInt: Option<i32> = lidFe.and_then(|v| v.as_any().downcast_ref::<i32>().copied());
            return match lidFeInt {
              None => 3,
              Some(v) => if v == 1049 || v == 2073 { 1 } else { 3 },
            };
          }
          // java default: return 4;
          _ => { return 4; }
        }
      }
      // java else { return 3 == charType ? 1 : 0; }
      if 3 == charType { 1 } else { 0 }
    }

    // java private void parseDefault(int codePoint, int charType, int graphIndex, LayoutState layoutState)
    pub fn parseDefault(&mut self, codePoint: i32, charType: i32, graphIndex: i32, layoutState: &mut LayoutState) {
      // java if (1 == this.mCharInfo.unicodeCodeType && codePoint < 160)
      if 1 == self.mCharInfo.unicodeCodeType && codePoint < 160 {
        // java switch (codePoint) { case 128: putChar '€'; ... case 151: putChar '—' }
        match codePoint {
          128 => { layoutState.typoPara.putChar(graphIndex, '€'); }
          133 => {
            layoutState.typoPara.putChar(graphIndex, '…');
            if self.needTagForTransToSpace(layoutState) { layoutState.tagForSpaceTrans = true; }
          }
          145 => { layoutState.typoPara.putChar(graphIndex, '‘'); }
          146 => { layoutState.typoPara.putChar(graphIndex, '’'); return; }
          147 => { layoutState.typoPara.putChar(graphIndex, '“'); }
          148 => { layoutState.typoPara.putChar(graphIndex, '”'); }
          149 => { layoutState.typoPara.putChar(graphIndex, '•'); }
          150 => { layoutState.typoPara.putChar(graphIndex, '–'); }
          151 => { layoutState.typoPara.putChar(graphIndex, '—'); }
          _ => {}
        }
        // java this.mGraphOperationEnv.runType = PunctTool.isPunct(codePoint) ? 18 : 1;
        self.mGraphOperationEnv.runType = if PunctTool::isPunct(codePoint) { 18 } else { 1 };
        // java this.mCurLang = this.getDefaultRunLang(charType, this.mGraphOperationEnv.charsNode);
        self.mCurLang = self.getDefaultRunLang(charType, unsafe { &*self.mGraphOperationEnv.charsNode });
        // java this.mGraphOperationEnv.isIsolate = true;
        self.mGraphOperationEnv.isIsolate = true;
      // java else if (70 == this.mCharInfo.unicodeCodeType)
      } else if 70 == self.mCharInfo.unicodeCodeType {
        // java if (codePoint != 8203 && codePoint != 8204)
        if codePoint != 8203 && codePoint != 8204 {
          // java if (PunctTool.isPunct(codePoint)) { this.mGraphOperationEnv.runType = 18; }
          if PunctTool::isPunct(codePoint) { self.mGraphOperationEnv.runType = 18; }
        } else {
          // java this.mGraphOperationEnv.runType = 37;
          self.mGraphOperationEnv.runType = 37;
        }
        // java this.mGraphOperationEnv.isIsolate = true;
        self.mGraphOperationEnv.isIsolate = true;
        // java this.mCurLang = this.getDefaultRunLang(charType, this.mGraphOperationEnv.charsNode);
        self.mCurLang = self.getDefaultRunLang(charType, unsafe { &*self.mGraphOperationEnv.charsNode });
        // java if (codePoint == 8230 && this.needTagForTransToSpace(layoutState)) { layoutState.tagForSpaceTrans = true; }
        if codePoint == 8230 && self.needTagForTransToSpace(layoutState) { layoutState.tagForSpaceTrans = true; }
      // java else if (this.isEmoji(codePoint, this.mCharInfo.unicodeCodeType))
      } else if self.isEmoji(codePoint, self.mCharInfo.unicodeCodeType) {
        // java this.mGraphOperationEnv.runType = 15;
        self.mGraphOperationEnv.runType = 15;
        // java this.mCurLang = 0;
        self.mCurLang = 0;
      // java else if (PunctTool.isPunct(codePoint))
      } else if PunctTool::isPunct(codePoint) {
        // java this.mGraphOperationEnv.runType = 18;
        self.mGraphOperationEnv.runType = 18;
        // java this.mGraphOperationEnv.isIsolate = true;
        self.mGraphOperationEnv.isIsolate = true;
        // java this.mCurLang = this.getDefaultRunLang(charType, this.mGraphOperationEnv.charsNode);
        self.mCurLang = self.getDefaultRunLang(charType, unsafe { &*self.mGraphOperationEnv.charsNode });
      // java else if (CharParse.getInstance().isSymbolChar(codePoint))
      } else if CharParse::getInstance().isSymbolChar(codePoint) {
        // java PropertySet finalProp = this.mGraphOperationEnv.charsNode.finalRunProp;
        let finalProp: Option<PropertySet> = unsafe { &*self.mGraphOperationEnv.charsNode }.finalRunProp.clone();
        // java Integer charSymbol = (Integer)finalProp.getValue(20);
        let charSymbol: Option<i32> = finalProp.as_ref().and_then(|p| p.getValue(20)).and_then(|v| v.as_any().downcast_ref::<i32>().copied());
        // java if (null != charSymbol && null != finalProp.getValue(19))
        if charSymbol.is_some() && finalProp.as_ref().map(|p| p.getValue(19).is_some()).unwrap_or(false) {
          // java this.mCurLang = this.getDefaultRunLang(charType, this.mGraphOperationEnv.charsNode);
          self.mCurLang = self.getDefaultRunLang(charType, unsafe { &*self.mGraphOperationEnv.charsNode });
        } else {
          // java int idctHint = this.mGraphOperationEnv.charsNode.finalRunProp.getInt(40, 0);
          let idctHint: i32 = unsafe { &*self.mGraphOperationEnv.charsNode }.finalRunProp.as_ref().map(|p| p.getInt__int_int(40, 0)).unwrap_or(0);
          // java if (4 == charType && idctHint == 0) { this.mCurLang = 9; } else { this.mCurLang = this.getDefaultRunLang(charType, this.mGraphOperationEnv.charsNode); }
          if 4 == charType && idctHint == 0 {
            self.mCurLang = 9;
          } else {
            self.mCurLang = self.getDefaultRunLang(charType, unsafe { &*self.mGraphOperationEnv.charsNode });
          }
        }
      } else {
        // java if (codePoint == 160 && this.needTagForTransToSpace(layoutState)) { this.mGraphOperationEnv.runType = 27; layoutState.tagForSpaceTrans = true; }
        if codePoint == 160 && self.needTagForTransToSpace(layoutState) {
          self.mGraphOperationEnv.runType = 27;
          layoutState.tagForSpaceTrans = true;
        }
        // java this.mCurLang = this.getDefaultRunLang(charType, this.mGraphOperationEnv.charsNode);
        self.mCurLang = self.getDefaultRunLang(charType, unsafe { &*self.mGraphOperationEnv.charsNode });
      }
    }

    // java private void parseAscii(int codePoint, int graphIndex, LayoutState layoutState)
    pub fn parseAscii(&mut self, codePoint: i32, graphIndex: i32, layoutState: &mut LayoutState) {
      // java this.mCurLang = 0;
      self.mCurLang = 0;
      // java CharsNode charsNode = this.mGraphOperationEnv.charsNode;
      let charsNode: *mut CharsNode = self.mGraphOperationEnv.charsNode;
      // java int runType = 1; boolean isIsolate = false; boolean isNoWidth = false; int autoSpaceType = 0;
      let mut runType: i32 = 1;
      let mut isIsolate: bool = false;
      let mut isNoWidth: bool = false;
      let mut autoSpaceType: i32 = 0;
      // java if ((codePoint < 97 || codePoint > 122) && (codePoint < 65 || codePoint > 90))
      if (codePoint < 97 || codePoint > 122) && (codePoint < 65 || codePoint > 90) {
        // java if (codePoint == 32)
        if codePoint == 32 {
          // java isIsolate = true; runType = 2;
          isIsolate = true;
          runType = 2;
        } else {
          // java if (codePoint < 32)
          if codePoint < 32 {
            // java switch (codePoint)
            match codePoint {
              // java case 1: isNoWidth = true; runType = 6;
              1 => {
                isIsolate = true;
                isNoWidth = true;
                runType = 6;
              }
              // java case 2: case 3: case 4:
              2 | 3 | 4 => {
                isIsolate = true;
                // java if (charsNode.isFootEndNote) { runType = 35; } else { isNoWidth = true; runType = 36; }
                if unsafe { &*charsNode }.isFootEndNote {
                  runType = 35;
                } else {
                  isNoWidth = true;
                  runType = 36;
                }
              }
              // java case 5:
              5 => {
                isIsolate = true;
                isNoWidth = true;
                runType = 33;
              }
              // java case 7: case 13:
              7 | 13 => {
                isIsolate = true;
                runType = 4;
              }
              // java case 8:
              8 => {
                isIsolate = true;
                isNoWidth = true;
                runType = 32;
              }
              // java case 9:
              9 => {
                isIsolate = true;
                isNoWidth = true;
                runType = 20;
                // java if (this.needTagForTransToSpace(layoutState)) { layoutState.tagForSpaceTrans = true; }
                if self.needTagForTransToSpace(layoutState) { layoutState.tagForSpaceTrans = true; }
              }
              // java case 10: case 11:
              10 | 11 => {
                isIsolate = true;
                runType = 3;
              }
              // java case 12:
              12 => {
                isIsolate = true;
                runType = 5;
              }
              // java case 14:
              14 => {
                isIsolate = true;
                runType = 14;
              }
              // java case 30:（soft hyphen——phone 自动断行分支 DEFERRED）
              // java case 31:（¬）
              // java default:
              _ => {
                // java layoutState.typoPara.putChar(graphIndex, ' ');
                layoutState.typoPara.putChar(graphIndex, ' ');
                // java isIsolate = true; runType = 2;
                isIsolate = true;
                runType = 2;
              }
            }
            // java this.mGraphOperationEnv.autoSpaceType = autoSpaceType;
            self.mGraphOperationEnv.autoSpaceType = autoSpaceType;
            // java this.mGraphOperationEnv.isIsolate = isIsolate;
            self.mGraphOperationEnv.isIsolate = isIsolate;
            // java this.mGraphOperationEnv.runType = runType;
            self.mGraphOperationEnv.runType = runType;
            // java this.mGraphOperationEnv.isNoWidth = isNoWidth;
            self.mGraphOperationEnv.isNoWidth = isNoWidth;
            // java return;
            return;
          }
          // java if (codePoint >= 48 && codePoint <= 57)
          if codePoint >= 48 && codePoint <= 57 {
            // java runType = 1; autoSpaceType = 4;
            runType = 1;
            autoSpaceType = 4;
          // java else if (codePoint == 62)
          } else if codePoint == 62 {
            // java isIsolate = true;
            isIsolate = true;
            // rust: MathOp autoSpaceType 5 —— 非数学忽略（isProccessMathOp false）
            let _ = ();
            // java else if (this.mGraphOperationEnv.isProccessMathOp && MathOperatorCharacter.isMathOperator((char)codePoint))
          } else if self.mGraphOperationEnv.isProccessMathOp {
            // rust: MathOperatorCharacter 未接线（数学非 hello）
            isIsolate = true;
          } else if PunctTool::isPunct(codePoint) {
            // java runType = 18; isIsolate = true;
            runType = 18;
            isIsolate = true;
            // java if (codePoint == 46 && this.needTagForTransToSpace(layoutState)) { layoutState.tagForSpaceTrans = true; }
            if codePoint == 46 && self.needTagForTransToSpace(layoutState) { layoutState.tagForSpaceTrans = true; }
          } else if codePoint == 95 && self.needTagForTransToSpace(layoutState) {
            // java runType = 26; layoutState.tagForSpaceTrans = true;
            runType = 26;
            layoutState.tagForSpaceTrans = true;
          }
        }
      } else {
        // java autoSpaceType = 1;
        autoSpaceType = 1;
      }
      // java this.mGraphOperationEnv.runType = runType;
      self.mGraphOperationEnv.runType = runType;
      // java this.mGraphOperationEnv.isIsolate = isIsolate;
      self.mGraphOperationEnv.isIsolate = isIsolate;
      // java this.mGraphOperationEnv.autoSpaceType = autoSpaceType;
      self.mGraphOperationEnv.autoSpaceType = autoSpaceType;
      // java this.mGraphOperationEnv.isNoWidth = isNoWidth;
      self.mGraphOperationEnv.isNoWidth = isNoWidth;
    }

    // java private boolean needTagForTransToSpace(LayoutState state)
    pub fn needTagForTransToSpace(&self, state: &LayoutState) -> bool {
      // java return state.isAllowTransForSpace() && ViewMode.isPhoneReadMode(this.mViewEnv.getViewMode());
      state.isAllowTransForSpace() && ViewMode::isPhoneReadMode(self.super_.super_.mViewEnv.as_ref().map(|v| v.getViewMode()).unwrap_or(0))
    }

    // java private boolean isEmoji(int codePoint, int unicodeCodeType)
    pub fn isEmoji(&self, codePoint: i32, unicodeCodeType: i32) -> bool {
      // java switch (unicodeCodeType) { case 74: ... case 238: return EmojiTool.isEmojiOfGoogleInputMethod(codePoint); default: return false; }
      // rust: EmojiTool 未接线（GR-D3 绘图）—— 恒 false（非 hello）
      let _ = codePoint;
      match unicodeCodeType {
        74 | 78 | 81 | 85 | 86 | 106 | 115 | 230 | 232 | 233 | 234 | 235 | 236 | 238 => {
          /* DEFERRED_NON_FUNCTION EmojiTool.isEmojiOfGoogleInputMethod */
          false
        }
        _ => false,
      }
    }

    // java private static boolean isRTL(int unicodeCodeType)
    pub fn isRTL(_unicodeCodeType: i32) -> bool {
      // java return isArabic(unicodeCodeType) || 11 == unicodeCodeType;
      GraphFactory::isArabic(_unicodeCodeType) || 11 == _unicodeCodeType
    }

    // java private static boolean isArabic(int unicodeCodeType)
    pub fn isArabic(unicodeCodeType: i32) -> bool {
      // java switch (unicodeCodeType) { case 12: case 14: case 153: case 159: return true; default: return false; }
      matches!(unicodeCodeType, 12 | 14 | 153 | 159)
    }

    // java private static boolean isIndic(int unicodeType)
    pub fn isIndic(_unicodeType: i32) -> bool {
      // rust: Indic 表未接线（GR-D3 复杂脚本）—— 恒 false
      false
    }

    // java public void createMathNodes(CharsNode charsNode, int graphIndex, PLC.Node mathNode, LayoutState layoutState)
    pub fn createMathNodes(&mut self, charsNode: &CharsNode, graphIndex: i32, mathNode: crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::MathEmbed, layoutState: &mut LayoutState) {
      // java this.updateGraphOperationEnv(charsNode, layoutState); this.commitAll(layoutState);
      self.updateGraphOperationEnv(charsNode, layoutState);
      self.commitAll(layoutState);
      let picture = match &mathNode {
        crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::MathEmbed::Function { picture_shape_id, .. } => *picture_shape_id,
        _ => None,
      };
      // java if (shape == null) layoutState.hasMaths = true;
      if picture.is_none() {
        layoutState.hasMaths = true;
      }
      let graphNode: *mut Graphs = self.mLangLayout.addGraphNode(graphIndex, 1, &mut self.mGraphOperationEnv);
      let base = embed_font_style(self, charsNode, layoutState);
      let mut fontStyle = self.super_.super_.mTypoFontStylePool.obtain(Some(&base));
      if self.super_.mIsInMath {
        fontStyle = self.mLangLayout.processMathUnderline(fontStyle, &mut self.mGraphOperationEnv);
      }
      fontStyle.hpsPos = 0;
      fontStyle.setSubScript(false);
      fontStyle.setSupScript(false);
      if let Some(g) = unsafe { graphNode.as_mut() } {
        g.runType = 6;
        g.autoSpaceType = 0;
        g.fontStyle = Some(fontStyle);
      }
      if !graphNode.is_null() {
        if let Some(shape_id) = picture {
          self.super_.super_.addEmbed(graphNode, shape_id);
        } else {
          self.super_.super_.addMath(graphNode, mathNode);
        }
      }
    }

    // java protected final void createEquationNodes(...)
    pub fn createEquationNodes(&mut self, charsNode: &CharsNode, graphIndex: i32, start: i32, end: i32, enableRevision: bool, revisionFinal: bool, layoutState: &mut LayoutState) {
      // java this.commitAll(layoutState); layoutState.hasEquation = true; this.updateGraphOperationEnv(...)
      self.commitAll(layoutState);
      layoutState.hasEquation = true;
      self.updateGraphOperationEnv(charsNode, layoutState);
      let graphNode: *mut Graphs = self.mLangLayout.addGraphNode(graphIndex, 1, &mut self.mGraphOperationEnv);
      let base = embed_font_style(self, charsNode, layoutState);
      let mut fontStyle = self.super_.super_.mTypoFontStylePool.obtain(Some(&base));
      fontStyle.hpsPos = 0;
      fontStyle.setSubScript(false);
      fontStyle.setSupScript(false);
      fontStyle.setUnderline(0);
      fontStyle.setStrikeThrough(0);
      if let Some(g) = unsafe { graphNode.as_mut() } {
        g.runType = 6;
        g.autoSpaceType = 0;
        g.fontStyle = Some(fontStyle);
      }
      if !graphNode.is_null() {
        self.super_.super_.addEqation(graphNode, start, end, enableRevision, revisionFinal);
      }
    }

    // java public void setCalcGraphHeight(boolean val)
    pub fn setCalcGraphHeight(&mut self, val: bool) {
      // java this.mGraphOperationEnv.isCalcGraphHeight = val;
      self.mGraphOperationEnv.isCalcGraphHeight = val;
    }

    // java public void setFootEndNoteRefType(int typeEndnote)
    pub fn setFootEndNoteRefType(&mut self, typeEndnote: i32) {
      // java this.mFootEndNoteRefType = typeEndnote;
      self.mFootEndNoteRefType = typeEndnote;
    }

    // java protected void createSymbolNodes(...)
    pub fn createSymbolNodes(&mut self, _charsNode: &CharsNode, _graphIndex: i32, _graphLen: i32, _symbolChar: char, _fontSymbol: &str, _layoutState: &mut LayoutState) {
      // rust: ★wr-29 GR-D3 —— 符号字体非 hello
      /* DEFERRED_NON_FUNCTION （Java 原文见 GraphFactory.java:1059-1128） */
    }

    // java protected void commitAll(LayoutState layoutState)
    pub fn commitAll(&mut self, layoutState: &mut LayoutState) {
      // java if (this.mLangLayout != null)
      // java this.mGraphOperationEnv.layoutState = layoutState;
      self.mGraphOperationEnv.layoutState = layoutState as *mut LayoutState;
      // java if (null != this.mGraphOperationEnv.langGraphNode || null != this.mGraphOperationEnv.arabicNodeList && !this.mGraphOperationEnv.arabicNodeList.isEmpty())
      if !self.mGraphOperationEnv.langGraphNode.is_null() {
        // java this.mLangLayout.commitLangGraphNode(this.mGraphOperationEnv);
        self.mLangLayout.commitLangGraphNode(&mut self.mGraphOperationEnv);
      }
      // java if (null != this.mGraphOperationEnv.commonGraphNode)
      if !self.mGraphOperationEnv.commonGraphNode.is_null() {
        // java this.mLangLayout.commitCommonGraphNode(this.mGraphOperationEnv);
        self.mLangLayout.commitCommonGraphNode(&mut self.mGraphOperationEnv);
      }
    }

    // java protected void commitGraphNode(LayoutState layoutState)
    pub fn commitGraphNode(&mut self, layoutState: &mut LayoutState) {
      // java this.commitAll(layoutState);
      self.commitAll(layoutState);
    }

    // java public void setMathUnderline(int underline, int underlineColor)
    pub fn setMathUnderline(&mut self, underline: i32, underlineColor: i32) {
      // java this.mGraphOperationEnv.mathUnderline = underline; this.mGraphOperationEnv.mathUnderlineColor = underlineColor;
      self.mGraphOperationEnv.mathUnderline = underline;
      self.mGraphOperationEnv.mathUnderlineColor = underlineColor;
    }

    // java private void updateGraphOperationEnv(CharsNode charsNode, LayoutState layoutState)
    pub fn updateGraphOperationEnv(&mut self, charsNode: &CharsNode, layoutState: &mut LayoutState) {
      // java this.mGraphOperationEnv.charsNode = charsNode;
      self.mGraphOperationEnv.charsNode = charsNode as *const CharsNode as *mut CharsNode;
      // rust: ★wr-29 04 —— 指针刷新（构造后位置；见 ctor 注释）
      if let Some(de) = self.super_.super_.aliased_doc() {
        self.mGraphOperationEnv.documentEnv = de as *mut DocumentEnv;
      }
      self.mGraphOperationEnv.typoFontStylePool = &mut self.super_.super_.mTypoFontStylePool as *mut _;
      // java this.mGraphOperationEnv.isProccessMathOp = charsNode.isInMath;
      self.mGraphOperationEnv.isProccessMathOp = charsNode.isInMath;
      // java this.mGraphOperationEnv.isInMath = this.mIsInMath;
      self.mGraphOperationEnv.isInMath = self.super_.mIsInMath;
      // java this.mGraphOperationEnv.layoutState = layoutState;
      self.mGraphOperationEnv.layoutState = layoutState as *mut LayoutState;
      // java this.mGraphOperationEnv.symbolConvert = this.mSymbolConvert;
      self.mGraphOperationEnv.symbolConvert = self.super_.mSymbolConvert;
      // java ICapsHandler capsHandler = this.mDocEnv.layoutToolsManager.getCapsHandler(charsNode.finalRunProp, this);
      // rust: caps（首字下沉）非 hello（GR-D3）→ 恒 None
      // java this.mGraphOperationEnv.capsHandler = capsHandler;
    }
}

fn with_shape<T>(doc: &mut dyn crate::cn::wps::moffice::writer::core::Document::Document, shape_id: i32, f: impl FnOnce(&mut drawing::cn::wps::moffice::drawing::Shape::Shape) -> T) -> Option<T> {
  let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
  let element = sub.getDrawingContainer()?.findShapeByID(shape_id)?;
  let shape = element.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>()?;
  let mut shape = shape.lock().expect("shape lock");
  Some(f(&mut shape))
}

fn shape_is_hidden(doc: &mut dyn crate::cn::wps::moffice::writer::core::Document::Document, shape_id: i32) -> bool {
  with_shape(doc, shape_id, |shape| shape.getHidden()).unwrap_or(false)
}

// java PLCField.getContainingField + '\\b' + ShapeTool.findShape(doc, cp - 1)
fn field_anchor_shape(doc: &mut dyn crate::cn::wps::moffice::writer::core::Document::Document, cp: i32, shape_id: i32) -> Option<i32> {
  let field = doc.getPLCField().and_then(|plc| unsafe { (*plc).getContainingField(cp) });
  let Some(field) = field else { return Some(shape_id); };
  let shape_field = crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType::FieldShape.getFlt();
  if field.getFieldType() == shape_field && doc.charAt(cp - 1) == '\u{8}' {
    // java preShape == null → Assert.fail and return null
    return shape_id_at(doc, cp - 1);
  }
  Some(shape_id)
}

fn shape_clone_failed(doc: &mut dyn crate::cn::wps::moffice::writer::core::Document::Document, shape_id: i32) -> bool {
  // java try { shape = shape.cloneLightWeight(); } catch (CloneNotSupportedException) { return null; }
  // A missing concrete shape keeps the existing id path; only a thrown clone aborts.
  match with_shape(doc, shape_id, |shape| shape.cloneLightWeight_default()) {
    Some(Err(_)) => true,
    _ => false,
  }
}

// Java: ShapeTool.findShape(document, cp) → PLCShapeAnchor.findByCP → findShapeByID.
fn shape_id_at(doc: &mut dyn crate::cn::wps::moffice::writer::core::Document::Document, cp: i32) -> Option<i32> {
  let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
  let anchor = sub.mShapeAnchors.as_mut()?.findByCP(cp)?;
  let id = anchor.getShapeId();
  let drawing = sub.getDrawingContainer()?;
  drawing.findShapeByID(id).map(|_| id)
}

// Java: DrawingShapeRectTool.calcDrawingShapeRect for an embed, then ShapeMetrics.transToLayout and scaleShapeRect.
pub(crate) fn embed_layout_size(doc: &mut dyn crate::cn::wps::moffice::writer::core::Document::Document, shape_id: i32) -> Option<(i32, i32)> {
  let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
  let element = sub.getDrawingContainer()?.findShapeByID(shape_id)?;
  let shape = element.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>()?;
  let mut shape = shape.lock().expect("shape lock");
  let pos = shape.getShapePos()?;
  let pos = pos.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>()?;
  let rect = pos.getShapeRect()?;
  let rect = rect.downcast_ref::<util::cn::wps::graphics::RectF::RectF>()?;
  let mut width = shape_metrics_round(rect.right * 20.0) - shape_metrics_round(rect.left * 20.0);
  let mut height = shape_metrics_round(rect.bottom * 20.0) - shape_metrics_round(rect.top * 20.0);
  let scale_h = pos.getScaleH();
  if scale_h != 1.0 { width = (width as f32 * scale_h) as i32; }
  let scale_v = pos.getScaleV();
  if scale_v != 1.0 { height = (height as f32 * scale_v) as i32; }
  Some((width, height))
}

fn shape_metrics_round(value: f32) -> i32 {
  if value >= 0.0 { (value + 0.5) as i32 } else { (value - 0.5) as i32 }
}

fn embed_font_style(factory: &mut GraphFactory, charsNode: &CharsNode, layoutState: &mut LayoutState) -> TypoFontStyle {
  // Java: charsNode.fontStyles[mCurLang], else FontStyleCache.get.
  if let Some(Some(style)) = charsNode.fontStyles.get(factory.mCurLang as usize) {
    return style.clone();
  }
  let run_prop = charsNode.finalRunProp.clone().unwrap_or_default();
  let lang = factory.mCurLang;
  let pool_ptr: *mut crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStylePool = {
    let gb = &mut factory.super_.super_;
    &mut gb.mTypoFontStylePool as *mut _
  };
  if let Some(cache) = factory.getFontStyleCache(layoutState) {
    cache.get(&run_prop, lang, Some(&mut layoutState.styleResult), Some(unsafe { &mut *pool_ptr }))
  } else {
    TypoFontStyle::newInstance()
  }
}

// ============================ 测试辅助（仅 04_function_graph_factory 单元测试使用） ============================
impl GraphFactory {
    pub fn test_op_runType(&self) -> i32 { self.mGraphOperationEnv.runType }
    pub fn test_op_mathUnderlineColor(&self) -> i32 { self.mGraphOperationEnv.mathUnderlineColor }
    pub fn test_op_highSurrogateType(&self) -> i32 { self.mGraphOperationEnv.highSurrogateType }
    pub fn test_op_documentEnv_ptr_nonnull(&self) -> bool { !self.mGraphOperationEnv.documentEnv.is_null() }
    pub fn test_op_pool_ptr_nonnull(&self) -> bool { !self.mGraphOperationEnv.typoFontStylePool.is_null() }
    pub fn test_op_isCalcGraphHeight(&self) -> bool { self.mGraphOperationEnv.isCalcGraphHeight }
    pub fn test_op_mathUnderline(&self) -> i32 { self.mGraphOperationEnv.mathUnderline }
    pub fn test_op_isScralForSmallCaps(&self) -> bool { self.mGraphOperationEnv.isScralForSmallCaps }
    pub fn test_op_isMathContent(&self) -> bool { self.mGraphOperationEnv.isMathContent }
    pub fn test_op_isIgnoreLeading(&self) -> bool { self.mGraphOperationEnv.isIgnoreLeading }
    pub fn test_footEndNoteRefType(&self) -> i32 { self.mFootEndNoteRefType }
    pub fn test_langGraphNode_null(&self) -> bool { self.mGraphOperationEnv.langGraphNode.is_null() }
    pub fn test_commonGraphNode_null(&self) -> bool { self.mGraphOperationEnv.commonGraphNode.is_null() }
}
