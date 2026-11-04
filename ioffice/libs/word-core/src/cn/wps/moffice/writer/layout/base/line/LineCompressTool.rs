// Java: import cn.wps.collections.KPList
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList;
// Java: import cn.wps.moffice.drawing.Shape
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.punctSpace.PunctTool
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::PunctTool::PunctTool;
// Java: import cn.wps.moffice.writer.layout.base.line.SubLineEnv
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
// Java: import cn.wps.moffice.writer.layout.base.line.LineRuleGraphsPos
use crate::cn::wps::moffice::writer::layout::base::line::LineRuleGraphsPos::LineRuleGraphsPos;
// Java: import cn.wps.moffice.writer.layout.base.line.LineCompressEnv
use crate::cn::wps::moffice::writer::layout::base::line::LineCompressEnv::LineCompressEnv;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;
use core::marker::PhantomData as _;

// Java 的 LayoutState 是产品排版状态；压缩必须读取同一实例的 graphList 和 typoPara。
#[derive(Default)]
pub struct Shape;
impl Shape {
    pub fn getShapeType(&self) -> i32 { 0 }
}
#[derive(Default)]
pub struct TypoDrawing;
impl TypoDrawing {
    pub fn isMath(_d: i32, _snap: ()) -> bool { false }
    pub fn getShape(_d: i32, _snap: ()) -> Option<Shape> { None }
    pub fn setHangEmbedWidth(_w: i32, _d: i32, _snap: ()) {}
}

// java private static final float PREV_GRAPH_NEED_SPACE_RATIO = 0.5F;
const PREV_GRAPH_NEED_SPACE_RATIO: f32 = 0.5f32;
// java private static final int MAX_WIDTH_BUF_SIZE = 32;
const MAX_WIDTH_BUF_SIZE: i32 = 32;

// java public class LineCompressTool
#[derive(Default)]
pub struct LineCompressTool {
    // java protected MainState mMainState;
    pub(crate) mMainState: Option<MainState>,
    // java protected DocumentEnv mDocEnv;
    pub(crate) mDocEnv: Option<DocumentEnv>,
    // java protected ViewEnv mViewEnv;
    pub(crate) mViewEnv: Option<ViewEnv>,
    // java protected SubLineEnv mSubLineEnv;
    pub mSubLineEnv: Option<SubLineEnv>,
    // rust: Java mSubLineEnv 与调用方共用对象；产品调用期间保留该节点地址。
    pub(crate) mSubLineEnvAlias: *mut SubLineEnv,
    // java protected LineRuleGraphsPos mTmpGraphsPos = new LineRuleGraphsPos();
    pub(crate) mTmpGraphsPos: Option<LineRuleGraphsPos>,
    // java protected LineRuleGraphsPos mNoSkipSpacePreGraphsPos = new LineRuleGraphsPos();
    pub(crate) mNoSkipSpacePreGraphsPos: Option<LineRuleGraphsPos>,
    // java private int mFirstExcessGraphIndex;
    pub(crate) mFirstExcessGraphIndex: i32,
    // java private int mFirstExcessGraphUncompressedLeftPos;
    mFirstExcessGraphUncompressedLeftPos: i32,
    // java private int[] mWidthBuff = new int[32];
    mWidthBuff: Vec<i32>,
    // java private int mWidthCacheStart;
    mWidthCacheStart: i32,
    // java private int mWidthCacheEnd;
    mWidthCacheEnd: i32,
}

// java public class LineCompressTool
impl LineCompressTool {
    // java public  LineCompressTool(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn LineCompressTool(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java this.mMainState = mainState;
      Self { mMainState: Some(mainState), mDocEnv: Some(docEnv), mViewEnv: Some(viewEnv), mSubLineEnv: Some(SubLineEnv::default()), mSubLineEnvAlias: std::ptr::null_mut(), mTmpGraphsPos: Some(LineRuleGraphsPos::default()), mNoSkipSpacePreGraphsPos: Some(LineRuleGraphsPos::default()), mFirstExcessGraphIndex: Default::default(), mFirstExcessGraphUncompressedLeftPos: Default::default(), mWidthBuff: vec![0; 32], mWidthCacheStart: Default::default(), mWidthCacheEnd: Default::default() }
      // java this.mDocEnv = docEnv;
      // java this.mViewEnv = viewEnv;
    }

    pub(crate) fn attachSubLineEnv(&mut self, env: *mut SubLineEnv) {
      self.mSubLineEnvAlias = env;
    }

    pub(crate) fn subLineEnv(&self) -> &SubLineEnv {
      if self.mSubLineEnvAlias.is_null() { self.mSubLineEnv.as_ref().unwrap() }
      else { unsafe { &*self.mSubLineEnvAlias } }
    }

    pub(crate) fn subLineEnv_mut(&mut self) -> &mut SubLineEnv {
      if self.mSubLineEnvAlias.is_null() { self.mSubLineEnv.as_mut().unwrap() }
      else { unsafe { &mut *self.mSubLineEnvAlias } }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mSubLineEnv = null;
      self.mSubLineEnv = None;
      // java if (null != this.mTmpGraphsPos)
      if let Some(mTmpGraphsPos) = self.mTmpGraphsPos.as_mut() {
        // java this.mTmpGraphsPos.dispose();
        mTmpGraphsPos.dispose();
        // java this.mTmpGraphsPos = null;
        self.mTmpGraphsPos = None;
      }
      // java this.mMainState = null;
      self.mMainState = None;
      // java this.mDocEnv = null;
      self.mDocEnv = None;
      // java this.mViewEnv = null;
      self.mViewEnv = None;
    }

    // java private int getPrevGraphCompressedWidth(LineCompressEnv lineRuleEnv, LineRuleGraphsPos prevGraphsPos, TypoPara typoPara)
    pub fn getPrevGraphCompressedWidth(&self, lineRuleEnv: &LineCompressEnv, prevGraphsPos: &LineRuleGraphsPos, typoPara: &TypoPara) -> i32 {
      // java Graphs prevGraphs = prevGraphsPos.getGraphs();
      let mut prevGraphs: Graphs = prevGraphsPos.getGraphs();
      // java int prevGraphIdx = prevGraphsPos.getGraphIndex();
      let mut prevGraphIdx: i32 = prevGraphsPos.getGraphIndex();
      // java if (!lineRuleEnv.isFirstCompress() && prevGraphIdx >= lineRuleEnv.getFirstCompressedGraphIndex())
      if !lineRuleEnv.isFirstCompress() && prevGraphIdx >= lineRuleEnv.getFirstCompressedGraphIndex() {
        // java int width = Math.abs(typoPara.getWidth(prevGraphIdx));
        let mut width: i32 = (typoPara.getWidth(prevGraphIdx)).abs();
        // java if (prevGraphIdx == prevGraphs.graphIndex)
        if prevGraphIdx == prevGraphs.graphIndex {
          // java width -= lineRuleEnv.getCompressBefore(prevGraphs.graphIndex);
          width = width - lineRuleEnv.getCompressBefore(prevGraphs.graphIndex);
        }
        // java if (prevGraphIdx == prevGraphs.getEndGraphIndex())
        if prevGraphIdx == prevGraphs.getEndGraphIndex() {
          // java width -= lineRuleEnv.getCompressAfter(prevGraphs.getEndGraphIndex());
          width = width - lineRuleEnv.getCompressAfter(prevGraphs.getEndGraphIndex());
        }
        // java return width;
        return width;
      }
      // java else
      else {
        // java return typoPara.getWidth(prevGraphIdx);
        return typoPara.getWidth(prevGraphIdx);
      }
    }

    // java private boolean isPrevGraphsFirstExcessSpace(LineRuleGraphsPos graphsPos, int firstExcessSpaceGraphIdx, KPList<Graphs> graphs)
    pub fn isPrevGraphsFirstExcessSpace(&self, graphsPos: &mut LineRuleGraphsPos, firstExcessSpaceGraphIdx: i32, graphs: &KPList<Graphs>) -> bool {
      // java Graphs graphsBak = graphsPos.getGraphs();
      let mut graphsBak: Graphs = graphsPos.getGraphs();
      // java int graphIdxBak = graphsPos.getGraphIndex();
      let mut graphIdxBak: i32 = graphsPos.getGraphIndex();
      // java boolean ret = false;
      let mut ret: bool = false;
      // java if (graphsPos.prev(false, graphs, this.mSubLineEnv))
      if graphsPos.prev(false, graphs, self.subLineEnv()) {
        // java ret = graphsPos.getGraphIndex() == firstExcessSpaceGraphIdx;
        ret = graphsPos.getGraphIndex() == firstExcessSpaceGraphIdx;
      }
      // java graphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
      graphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
      // java return ret;
      return ret;
    }

    // java private boolean canDoFirstCompress(LineCompressEnv lineRuleEnv, LineRuleGraphsPos prevGraphsPos, LineRuleGraphsPos noSkipSpacePreGraphsPos, LineRuleGraphsPos curGraphsPos, int compressedGraphLeft, int compressAfter, int subLineWidth, int needSpace, LayoutState layoutState)
    pub fn canDoFirstCompress(&self, lineRuleEnv: &LineCompressEnv, prevGraphsPos: &LineRuleGraphsPos, noSkipSpacePreGraphsPos: &LineRuleGraphsPos, curGraphsPos: &LineRuleGraphsPos, compressedGraphLeft: i32, compressAfter: i32, subLineWidth: i32, needSpace: i32, layoutState: &LayoutState) -> bool {
      // java if (lineRuleEnv.firstExcessPunctGraphIndex >= 0)
      if lineRuleEnv.firstExcessPunctGraphIndex >= 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Graphs prevGraphs = prevGraphsPos.getGraphs();
        let mut prevGraphs: Graphs = prevGraphsPos.getGraphs();
        // java if (prevGraphs.isEmbed())
        if prevGraphs.isEmbed() {
          // java return true;
          return true;
        }
        // java else
        else {
          // java int spaceGraphIdx = lineRuleEnv.firstExcessSpaceGraphIndex;
          let mut spaceGraphIdx: i32 = lineRuleEnv.firstExcessSpaceGraphIndex;
          // java if (spaceGraphIdx >= 0 && this.isPrevGraphsFirstExcessSpace(curGraphsPos, spaceGraphIdx, layoutState.getGraphList()))
          let mut isPrevFirstExcessSpace: bool = false;
          if spaceGraphIdx >= 0 {
            let mut tmpPos = LineRuleGraphsPos::default();
            tmpPos.copy(curGraphsPos);
            isPrevFirstExcessSpace = self.isPrevGraphsFirstExcessSpace(&mut tmpPos, spaceGraphIdx, &layoutState.graphList);
          }
          if spaceGraphIdx >= 0 && isPrevFirstExcessSpace {
              // java return true;
              return true;
          }
          // java else
          else {
            // java Graphs curGraphs = curGraphsPos.getGraphs();
            let mut curGraphs: Graphs = curGraphsPos.getGraphs();
            // java if (!TypoRun.isSpace(curGraphs.runType) && !curGraphs.isEmbed())
            if !TypoRun::isSpace(curGraphs.runType) && !curGraphs.isEmbed() {
              // java boolean isWordWrap = layoutState.paraEnv.wordWrap;
              let mut isWordWrap: bool = layoutState.paraEnv.wordWrap;
              // java if (isWordWrap && TypoRun.isSpace(noSkipSpacePreGraphsPos.getGraphs().runType))
              if isWordWrap && TypoRun::isSpace(noSkipSpacePreGraphsPos.getGraphs().runType) {
                // java return true;
                return true;
              }
              // java else if (PunctTool.isTailPunct(curGraphsPos.getChar(&layoutState.typoPara)))
              else if PunctTool::isTailPunct(curGraphsPos.getChar(&layoutState.typoPara)) {
                // java return true;
                return true;
              }
              // java else
              else {
                // java int compressLevel = this.mDocEnv.compressLevel;
                let mut compressLevel: i32 = self.mDocEnv.as_ref().map(|e| e.compressLevel).unwrap_or(0);
                // java if (isWordWrap && prevGraphsPos.isPartOfWord(compressLevel, layoutState.typoPara) && curGraphsPos.isPartOfWord(compressLevel, layoutState.typoPara))
                if isWordWrap && prevGraphsPos.isPartOfWord(compressLevel, &layoutState.typoPara) && curGraphsPos.isPartOfWord(compressLevel, &layoutState.typoPara) {
                  // java return true;
                  return true;
                }
                // java else if (layoutState.paraEnv.kinsoku && PunctTool.isHeadPunct(prevGraphsPos.getChar(&layoutState.typoPara)))
                else if layoutState.paraEnv.kinsoku && PunctTool::isHeadPunct(prevGraphsPos.getChar(&layoutState.typoPara)) {
                  // java return true;
                  return true;
                }
                // java else
                else {
                  // java int prevWidth = this.getPrevGraphCompressedWidth(lineRuleEnv, prevGraphsPos, layoutState.typoPara);
                  let mut prevWidth: i32 = self.getPrevGraphCompressedWidth(lineRuleEnv, prevGraphsPos, &layoutState.typoPara);
                  // java if (needSpace <= (int)((float)prevWidth * 0.5F))
                  if needSpace <= ((((prevWidth) as f32) * 0.5) as i32) {
                    // java return true;
                    return true;
                  }
                  // java else
                  else {
                    // java return (float)compressAfter != 0.0F && curGraphsPos.getGraphIndex() < curGraphs.getEndGraphIndex() && needSpace + compressAfter <= (int)((float)prevWidth * 0.5F);
                    return ((compressAfter) as f32) != 0.0 && curGraphsPos.getGraphIndex() < curGraphs.getEndGraphIndex() && needSpace + compressAfter <= ((((prevWidth) as f32) * 0.5) as i32);
                  }
                }
              }
            }
            // java else
            else {
              // java return compressedGraphLeft <= subLineWidth;
              return compressedGraphLeft <= subLineWidth;
            }
          }
        }
      }
    }

    // java private boolean canDoNextCompressIfPrevAndCurIsPartOfWord(LineRuleGraphsPos tmpGraphsPos, LineRuleGraphsPos curGraphsPos, LayoutState layoutState)
    pub fn canDoNextCompressIfPrevAndCurIsPartOfWord(&self, tmpGraphsPos: &mut LineRuleGraphsPos, curGraphsPos: &LineRuleGraphsPos, layoutState: &LayoutState) -> bool {
      // java boolean ret = false;
      let mut ret: bool = false;
      // java Graphs tmpGraphsBak = tmpGraphsPos.getGraphs();
      let mut tmpGraphsBak: Graphs = tmpGraphsPos.getGraphs();
      // java int tmpGraphIdxBak = tmpGraphsPos.getGraphIndex();
      let mut tmpGraphIdxBak: i32 = tmpGraphsPos.getGraphIndex();
      // java tmpGraphsPos.copy(curGraphsPos);
      tmpGraphsPos.copy(curGraphsPos);
      // java int compressLeve = this.mDocEnv.compressLevel;
      let mut compressLeve: i32 = self.mDocEnv.as_ref().map(|e| e.compressLevel).unwrap_or(0);
      // java TypoPara typoPara = layoutState.typoPara;
      let mut typoPara: TypoPara = layoutState.typoPara.clone();
      // java if (tmpGraphsPos.prev(false, layoutState.getGraphList(), this.mSubLineEnv) && tmpGraphsPos.isPartOfWord(compressLeve, &typoPara) && curGraphsPos.isPartOfWord(compressLeve, &typoPara))
      let has_prev = tmpGraphsPos.prev(false, &layoutState.graphList, self.subLineEnv());
      let prev_part = has_prev && tmpGraphsPos.isPartOfWord(compressLeve, &typoPara);
      let cur_part = prev_part && curGraphsPos.isPartOfWord(compressLeve, &typoPara);
      if std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp) {
        let prev_graph = tmpGraphsPos.getGraphs();
        eprintln!("compress_rule cp={} cur={} prev={} hasPrev={} prevPart={} curPart={} level={} prevType={} prevLang={} prevNode={} prevInvalid={} west={}",
          layoutState.cp, curGraphsPos.getGraphIndex(), if has_prev { tmpGraphsPos.getGraphIndex() } else { -1 },
          has_prev, prev_part, cur_part, compressLeve, prev_graph.runType, prev_graph.language,
          prev_graph.graphIndex, tmpGraphsPos.isInvalid(), TypoRun::isWestLangType(prev_graph.language));
      }
      if has_prev && prev_part && cur_part {
        // java ret = true;
        ret = true;
        // java if (!tmpGraphsPos.getGraphs().isEmbed() && !PunctTool.isTailPunct(tmpGraphsPos.getChar(&typoPara)))
        if !tmpGraphsPos.getGraphs().isEmbed() && !PunctTool::isTailPunct(tmpGraphsPos.getChar(&typoPara)) {
          // java Graphs prevGraphsBak = tmpGraphsPos.getGraphs();
          let mut prevGraphsBak: Graphs = tmpGraphsPos.getGraphs();
          // java int prevGraphIdxBak = tmpGraphsPos.getGraphIndex();
          let mut prevGraphIdxBak: i32 = tmpGraphsPos.getGraphIndex();
          // java if (tmpGraphsPos.prev(false, layoutState.getGraphList(), this.mSubLineEnv))
          if tmpGraphsPos.prev(false, &layoutState.graphList, self.subLineEnv()) {
            // java int prevPrevLang = tmpGraphsPos.getGraphs().language;
            let mut prevPrevLang: i32 = tmpGraphsPos.getGraphs().language;
            // java if (TypoRun.isFarEastLangType(prevPrevLang))
            if TypoRun::isFarEastLangType(prevPrevLang) {
              // java char prevPrevChar = tmpGraphsPos.getChar(&typoPara);
              let mut prevPrevChar: char = tmpGraphsPos.getChar(&typoPara);
              // java if (!layoutState.paraEnv.kinsoku || !PunctTool.isHeadPunct(prevPrevChar))
              if !layoutState.paraEnv.kinsoku || !PunctTool::isHeadPunct(prevPrevChar) {
                // java ret = false;
                ret = false;
              }
            }
          }
          // java tmpGraphsPos.setGraphsAndIndex(prevGraphsBak, prevGraphIdxBak);
          tmpGraphsPos.setGraphsAndIndex(prevGraphsBak, prevGraphIdxBak);
        }
      }
      // java tmpGraphsPos.setGraphsAndIndex(tmpGraphsBak, tmpGraphIdxBak);
      tmpGraphsPos.setGraphsAndIndex(tmpGraphsBak, tmpGraphIdxBak);
      // java return ret;
      return ret;
    }

    // java private boolean canDoNextCompress(LineCompressEnv lineRuleEnv, LineRuleGraphsPos prevGraphsPos, LineRuleGraphsPos curGraphsPos, LayoutState layoutState)
    pub fn canDoNextCompress(&self, lineRuleEnv: &LineCompressEnv, prevGraphsPos: &mut LineRuleGraphsPos, curGraphsPos: &LineRuleGraphsPos, layoutState: &LayoutState) -> bool {
      if std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp) {
        let para = &layoutState.typoPara;
        eprintln!("compress_rule cp={} prev={} prevType={} prevLang={} prevChar={} prevHead={} cur={} curType={} curLang={} curChar={} curTail={} kinsoku={} wordWrap={} firstPunct={}",
          layoutState.cp, prevGraphsPos.getGraphIndex(), prevGraphsPos.getGraphs().runType, prevGraphsPos.getGraphs().language,
          prevGraphsPos.getChar(para) as u32, PunctTool::isHeadPunct(prevGraphsPos.getChar(para)),
          curGraphsPos.getGraphIndex(), curGraphsPos.getGraphs().runType, curGraphsPos.getGraphs().language,
          curGraphsPos.getChar(para) as u32, PunctTool::isTailPunct(curGraphsPos.getChar(para)),
          layoutState.paraEnv.kinsoku, layoutState.paraEnv.wordWrap, lineRuleEnv.firstExcessPunctGraphIndex);
      }
      // java if (lineRuleEnv.firstExcessPunctGraphIndex >= 0)
      if lineRuleEnv.firstExcessPunctGraphIndex >= 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Graphs prevGraphs = prevGraphsPos.getGraphs();
        let mut prevGraphs: Graphs = prevGraphsPos.getGraphs();
        // java TypoPara typoPara = layoutState.typoPara;
        let mut typoPara: TypoPara = layoutState.typoPara.clone();
        // java if (TypoRun.isPunct(prevGraphs.runType) && PunctTool.isHeadPunct(prevGraphsPos.getChar(&typoPara)))
        if TypoRun::isPunct(prevGraphs.runType) && PunctTool::isHeadPunct(prevGraphsPos.getChar(&typoPara)) {
          // java return layoutState.paraEnv.kinsoku;
          return layoutState.paraEnv.kinsoku;
        }
        // java else
        else {
          // java Graphs curGraphs = curGraphsPos.getGraphs();
          let mut curGraphs: Graphs = curGraphsPos.getGraphs();
          // java if (TypoRun.isPunct(curGraphs.runType) && PunctTool.isTailPunct(curGraphsPos.getChar(&typoPara)))
          if TypoRun::isPunct(curGraphs.runType) && PunctTool::isTailPunct(curGraphsPos.getChar(&typoPara)) {
            // java return true;
            return true;
          }
          // java else if (!layoutState.paraEnv.wordWrap)
          else if !layoutState.paraEnv.wordWrap {
            // java return false;
            return false;
          }
          // java else
          else {
            // java return this.canDoNextCompressIfPrevAndCurIsPartOfWord(prevGraphsPos, curGraphsPos, layoutState);
            return self.canDoNextCompressIfPrevAndCurIsPartOfWord(prevGraphsPos, curGraphsPos, layoutState);
          }
        }
      }
    }

    // java private boolean canHangPunct(LineCompressEnv lineRuleEnv, LineRuleGraphsPos graphsPos, LayoutState layoutState)
    pub fn canHangPunct(&self, lineRuleEnv: &LineCompressEnv, graphsPos: &LineRuleGraphsPos, layoutState: &LayoutState) -> bool {
      // java Graphs graphs = graphsPos.getGraphs();
      let mut graphs: Graphs = graphsPos.getGraphs();
      // java if (lineRuleEnv.firstExcessSpaceGraphIndex >= 0)
      if lineRuleEnv.firstExcessSpaceGraphIndex >= 0 {
        // java return false;
        return false;
      }
      // java else if (layoutState.paraEnv.isRTL)
      else if layoutState.paraEnv.isRTL {
        // java return false;
        return false;
      }
      // java else if (layoutState.tablePeriLayer > 1)
      else if layoutState.tablePeriLayer > 1 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int docType = layoutState.document.getType();
        let docType: i32 = layoutState.document.map(|doc| unsafe { (&*doc).getType__on_IDocument() }).unwrap_or(0);
        // java switch (docType)
        match docType {
          // java case 1: case 4:
          1 | 4 => {
            // java return false;
            return false;
          }
          // java case 2: case 3: default:
          2 | 3 | _ => {
            // java if (!layoutState.paraEnv.overflowPunct)
            if !layoutState.paraEnv.overflowPunct {
              // java return false;
              return false;
            }
            // java else if (lineRuleEnv.firstExcessPunctGraphIndex >= 0)
            else if lineRuleEnv.firstExcessPunctGraphIndex >= 0 {
              // java return false;
              return false;
            }
            // java else
            else {
              // java PropertySet prop = graphs.charsRef.finalRunProp;
              let mut prop: PropertySet = graphs.charsRef.as_ref().and_then(|c| c.finalRunProp.clone()).unwrap_or_default();
              // java Integer lid = (Integer)prop.getObject(42, (Object)null);
              let lid: Option<i32> = if prop.contains(42) { Some(prop.getInt__int_int(42, 0)) } else { None };
              // java boolean useAltLineBreakRules = this.mDocEnv.useAltKinsokuLineBreakRules;
              let mut useAltLineBreakRules: bool = self.mDocEnv.as_ref().map(|e| e.useAltKinsokuLineBreakRules).unwrap_or(false);
              // java if (PunctTool.isHangablePunct(graphsPos.getChar(&layoutState.typoPara), lid, useAltLineBreakRules))
              if PunctTool::isHangablePunct(graphsPos.getChar(&layoutState.typoPara), lid, useAltLineBreakRules) {
                // java return true;
                return true;
              }
              // java return false;
              return false;
            }
          }
          // java case 5: case 6:
          5 | 6 => {
            // java return false;
            return false;
          }
        }
      }
    }

    // java private boolean canHangSpace(LineCompressEnv lineRuleEnv, LineRuleGraphsPos graphsPos, LayoutState layoutState)
    pub fn canHangSpace(&self, lineRuleEnv: &LineCompressEnv, graphsPos: &LineRuleGraphsPos, layoutState: &LayoutState) -> bool {
      // java Graphs graphs = graphsPos.getGraphs();
      let mut graphs: Graphs = graphsPos.getGraphs();
      // java if (!layoutState.paraEnv.wordWrap)
      if !layoutState.paraEnv.wordWrap {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return !this.mDocEnv.wrapTrailSpaces;
        return !self.mDocEnv.as_ref().map(|e| e.wrapTrailSpaces).unwrap_or(false);
      }
    }

    // java private boolean canHangEmbed(LineCompressEnv lineRuleEnv, LineRuleGraphsPos graphsPos, LayoutState layoutState)
    pub fn canHangEmbed(&self, lineRuleEnv: &LineCompressEnv, graphsPos: &mut LineRuleGraphsPos, layoutState: &mut LayoutState) -> bool {
      // java if (0 != this.mViewEnv.getViewMode())
      if 0 != self.mViewEnv.as_ref().map(|v| v.getViewMode()).unwrap_or(0) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Graphs run = graphsPos.getGraphs();
        let mut run: Graphs = graphsPos.getGraphs();
        // java SubLineEnv subLineEnv = this.mSubLineEnv;
        let subLineEnv = self.subLineEnv();
        // java if (run == subLineEnv.firstGraphs && run == subLineEnv.lastGraphs)
        if run.graphIndex == subLineEnv.firstGraphs.as_ref().map(|g| g.graphIndex).unwrap_or(i32::MIN)
            && run.graphIndex == subLineEnv.lastGraphs.as_ref().map(|g| g.graphIndex).unwrap_or(i32::MIN) {
          // java int embedDrawing = layoutState.getDrawing(run);
          let mut embedDrawing: i32 = layoutState.getDrawing(&run);
          // java if (0 != embedDrawing && !TypoDrawing.isMath(embedDrawing, this.mMainState.snapshot))
          if 0 != embedDrawing && !TypoDrawing::isMath(embedDrawing, ()) {
            // java boolean isFollowedByBreakChar = false;
            let mut isFollowedByBreakChar: bool = false;
            // java Graphs curGraphsBak = graphsPos.getGraphs();
            let mut curGraphsBak: Graphs = graphsPos.getGraphs();
            // java int curGraphIdxBak = graphsPos.getGraphIndex();
            let mut curGraphIdxBak: i32 = graphsPos.getGraphIndex();
            // java if (graphsPos.next(layoutState.getGraphList()))
            if graphsPos.next(&layoutState.graphList) {
              // java int nextRunType = graphsPos.getGraphs().runType;
              let mut nextRunType: i32 = graphsPos.getGraphs().runType;
              // java if (TypoRun.isLineBreak(nextRunType))
              if TypoRun::isLineBreak(nextRunType) {
                // java isFollowedByBreakChar = true;
                isFollowedByBreakChar = true;
              }
              // java graphsPos.setGraphsAndIndex(curGraphsBak, curGraphIdxBak);
              graphsPos.setGraphsAndIndex(curGraphsBak, curGraphIdxBak);
            }
            // java boolean isHandEmbed = false;
            let mut isHandEmbed: bool = false;
            // java if (isFollowedByBreakChar)
            if isFollowedByBreakChar {
              // java isHandEmbed = true;
              isHandEmbed = true;
            }
            // java else
            else {
              // java isHandEmbed = subLineEnv.wrapRectType == 1 && subLineEnv.isUniqueSubline;
              isHandEmbed = subLineEnv.wrapRectType == 1 && subLineEnv.isUniqueSubline;
            }
            // java if (isHandEmbed && !subLineEnv.enableRightIndent)
            if isHandEmbed && !subLineEnv.enableRightIndent {
              // java Shape shape = TypoDrawing.getShape(embedDrawing, this.mMainState.snapshot);
              let mut shape: Option<Shape> = TypoDrawing::getShape(embedDrawing, ());
              // java if (null != shape && shape.getShapeType() == 204)
              if shape.as_ref().map(|s| s.getShapeType() == 204).unwrap_or(false) {
                // java TypoDrawing.setHangEmbedWidth(subLineEnv.layoutRight - subLineEnv.layoutLeft, embedDrawing, this.mMainState.snapshot);
                TypoDrawing::setHangEmbedWidth(subLineEnv.layoutRight - subLineEnv.layoutLeft, embedDrawing, ());
              }
            }
            // java return isHandEmbed;
            return isHandEmbed;
          }
          // java else
          else {
            // java return false;
            return false;
          }
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
    }

    // java private boolean doCompressNormal(LineCompressEnv lineRuleEnv, LineRuleGraphsPos curGraphsPos, int uncompressedLeft, int subLineWidth, LayoutState layoutState)
    pub fn doCompressNormal(&mut self, curGraphsPos: &mut LineRuleGraphsPos, uncompressedLeft: i32, subLineWidth: i32, layoutState: &LayoutState) -> bool {
      // java LineRuleGraphsPos prevGraphsPos = this.mTmpGraphsPos;
        let mut prevGraphsPos = LineRuleGraphsPos::default();
        if let Some(tmp) = self.mTmpGraphsPos.as_ref() {
            prevGraphsPos.copy(tmp);
        }
      // java if (!layoutState.paraEnv.compressLine)
      if !layoutState.paraEnv.compressLine {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Graphs graphs = curGraphsPos.getGraphs();
        let mut graphs: Graphs = curGraphsPos.getGraphs();
        // java int lastGraphIdx = graphs.getEndGraphIndex();
        let mut lastGraphIdx: i32 = graphs.getEndGraphIndex();
        // java int compressAfter = lineRuleEnv.getCompressAfter(lastGraphIdx);
        let mut compressAfter: i32 = self.subLineEnv().lineCompressEnv.as_ref().unwrap().getCompressAfter(lastGraphIdx);
        // java TypoPara typoPara = layoutState.typoPara;
        let mut typoPara: TypoPara = layoutState.typoPara.clone();
        // java boolean needToSkipSpace = layoutState.paraEnv.wordWrap;
        let mut needToSkipSpace: bool = layoutState.paraEnv.wordWrap;
        // java boolean isFirstCompress = lineRuleEnv.isFirstCompress();
        let mut isFirstCompress: bool = self.subLineEnv().lineCompressEnv.as_ref().unwrap().isFirstCompress();
        // java boolean hasInitedPrevPos = false;
        let mut hasInitedPrevPos: bool = false;
        // java int graphIdx = curGraphsPos.getGraphIndex();
        let mut graphIdx: i32 = curGraphsPos.getGraphIndex();
        // java int compressedGraphIdx = Integer.MIN_VALUE;
        let mut compressedGraphIdx: i32 = i32::MIN;
        // java int graphLeft = uncompressedLeft;
        let mut graphLeft: i32 = uncompressedLeft;
        // java int[] widthBuff = this.mWidthBuff;
        let mut widthBuff: Vec<i32> = self.mWidthBuff.clone();
        // java if (graphIdx < this.mWidthCacheStart || graphIdx >= this.mWidthCacheEnd)
        if graphIdx < self.mWidthCacheStart || graphIdx >= self.mWidthCacheEnd {
          // java this.mWidthCacheStart = graphIdx;
          self.mWidthCacheStart = graphIdx;
          // java this.mWidthCacheEnd = graphIdx + Math.min(graphs.graphLength + graphs.graphIndex - graphIdx, 32);
          self.mWidthCacheEnd = graphIdx + (graphs.graphLength + graphs.graphIndex - graphIdx).min(32);
          // java typoPara.getWidths(graphIdx, this.mWidthCacheEnd - this.mWidthCacheStart, widthBuff, 0);
          let cache_len = self.mWidthCacheEnd - self.mWidthCacheStart;
          typoPara.getWidths(graphIdx, cache_len, &mut widthBuff, 0);
          self.mWidthBuff.clone_from(&widthBuff);
        }
        // java while (true)
        loop {
          // java int usedSum = lineRuleEnv.getUsedCompressableSpaceSum();
          let mut usedSum: i32 = self.subLineEnv().lineCompressEnv.as_ref().unwrap().getUsedCompressableSpaceSum();
          // java int leftedSum = lineRuleEnv.getLeftedCompressableSpaceSum();
          let mut leftedSum: i32 = self.subLineEnv().lineCompressEnv.as_ref().unwrap().getLeftedCompressableSpaceSum();
          // java int graphWidth = Math.abs(widthBuff[graphIdx - this.mWidthCacheStart]) + curGraphsPos.getBeginBdrWidth(graphIdx);
          let mut graphWidth: i32 = (widthBuff[(graphIdx - self.mWidthCacheStart) as usize]).abs() + curGraphsPos.getBeginBdrWidth(graphIdx);
          // java int compressedLeft = graphLeft - usedSum;
          let mut compressedLeft: i32 = graphLeft - usedSum;
          // java int compressedRight = compressedLeft + graphWidth;
          let mut compressedRight: i32 = compressedLeft + graphWidth;
          // java int needSpace = compressedRight - subLineWidth;
          let mut needSpace: i32 = compressedRight - subLineWidth;
          let trace_compress = std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp);
          if trace_compress {
            eprintln!("compress_step cp={} run={} graph={} left={} width={} used={} lefted={} need={} first={} wordWrap={}",
              layoutState.cp, graphs.graphIndex, graphIdx, graphLeft, graphWidth, usedSum, leftedSum, needSpace, isFirstCompress, needToSkipSpace);
          }
          // java if (needSpace > leftedSum)
          if needSpace > leftedSum {
            // java break;
            break;
          }
          // java boolean found = true;
          let mut found: bool = true;
          // java if (!hasInitedPrevPos)
          if !hasInitedPrevPos {
            // java prevGraphsPos.copy(curGraphsPos);
            prevGraphsPos.copy(curGraphsPos);
            // java found = prevGraphsPos.prev(needToSkipSpace, layoutState.getGraphList(), this.mSubLineEnv);
            found = prevGraphsPos.prev(needToSkipSpace, &layoutState.graphList, self.subLineEnv());
            // java if (found)
            if found {
              // java if (needToSkipSpace)
              if needToSkipSpace {
                // java this.mNoSkipSpacePreGraphsPos.copy(curGraphsPos);
                self.mNoSkipSpacePreGraphsPos.as_mut().unwrap().copy(curGraphsPos);
                // java this.mNoSkipSpacePreGraphsPos.prev(false, layoutState.getGraphList(), this.mSubLineEnv);
                let sub_line_env = self.subLineEnv() as *const SubLineEnv;
                self.mNoSkipSpacePreGraphsPos.as_mut().unwrap().prev(false, &layoutState.graphList, unsafe { &*sub_line_env });
              }
              // java else
              else {
                // java this.mNoSkipSpacePreGraphsPos.copy(prevGraphsPos);
                self.mNoSkipSpacePreGraphsPos.as_mut().unwrap().copy(&prevGraphsPos);
              }
            }
            // java hasInitedPrevPos = true;
            hasInitedPrevPos = true;
          }
          // java if (found)
          if found {
            if trace_compress {
              eprintln!("compress_step cp={} run={} graph={} prev={} prevType={} found={} first={}",
                layoutState.cp, graphs.graphIndex, graphIdx, prevGraphsPos.getGraphIndex(), prevGraphsPos.getGraphs().runType, found, isFirstCompress);
            }
            // java if (isFirstCompress)
            if isFirstCompress {
              // java if (!this.canDoFirstCompress(lineRuleEnv, prevGraphsPos, this.mNoSkipSpacePreGraphsPos, curGraphsPos, compressedLeft, compressAfter, subLineWidth, needSpace, layoutState))
              let can_first = self.canDoFirstCompress(self.subLineEnv().lineCompressEnv.as_ref().unwrap(), &prevGraphsPos, self.mNoSkipSpacePreGraphsPos.as_ref().unwrap(), curGraphsPos, compressedLeft, compressAfter, subLineWidth, needSpace, layoutState);
              if !can_first {
                if trace_compress { eprintln!("compress_step cp={} run={} decision=first_false", layoutState.cp, graphs.graphIndex); }
                // java break;
                break;
              }
            }
            // java else if (!this.canDoNextCompress(lineRuleEnv, prevGraphsPos, curGraphsPos, layoutState))
            else if !self.canDoNextCompress(self.subLineEnv().lineCompressEnv.as_ref().unwrap(), &mut prevGraphsPos, curGraphsPos, layoutState) {
              if trace_compress { eprintln!("compress_step cp={} run={} decision=next_false", layoutState.cp, graphs.graphIndex); }
              // java break;
              break;
            }
          }
          // java lineRuleEnv.consumeCompressableSpaces(graphIdx, needSpace);
          self.subLineEnv_mut().lineCompressEnv.as_mut().unwrap().consumeCompressableSpaces(graphIdx, needSpace);
          // java compressedGraphIdx = graphIdx;
          compressedGraphIdx = graphIdx;
          // java graphLeft += graphWidth;
          graphLeft = graphLeft + graphWidth;
          // java ++graphIdx;
          { let _tmp = graphIdx; graphIdx += 1; _tmp };
          // java if (graphIdx > lastGraphIdx)
          if graphIdx > lastGraphIdx {
            // java break;
            break;
          }
          // java if (graphIdx >= this.mWidthCacheEnd)
          if graphIdx >= self.mWidthCacheEnd {
            // java this.mWidthCacheStart = graphIdx;
            self.mWidthCacheStart = graphIdx;
            // java this.mWidthCacheEnd = graphIdx + Math.min(graphs.graphLength + graphs.graphIndex - graphIdx, 32);
            self.mWidthCacheEnd = graphIdx + (graphs.graphLength + graphs.graphIndex - graphIdx).min(32);
            // java typoPara.getWidths(graphIdx, this.mWidthCacheEnd - this.mWidthCacheStart, widthBuff, 0);
            let cache_len = self.mWidthCacheEnd - self.mWidthCacheStart;
            typoPara.getWidths(graphIdx, cache_len, &mut widthBuff, 0);
            self.mWidthBuff.clone_from(&widthBuff);
          }
          // java prevGraphsPos.copy(curGraphsPos);
          prevGraphsPos.copy(curGraphsPos);
          // java curGraphsPos.setGraphsAndIndex(graphs, graphIdx);
          curGraphsPos.setGraphsAndIndex(graphs.clone(), graphIdx);
          // java isFirstCompress = false;
          isFirstCompress = false;
        }
        if let Some(tmp) = self.mTmpGraphsPos.as_mut() {
            tmp.copy(&prevGraphsPos);
        }
        // java boolean hasCompressed = compressedGraphIdx > 0;
        let mut hasCompressed: bool = compressedGraphIdx > 0;
        // java if (hasCompressed)
        if hasCompressed {
          // java curGraphsPos.setGraphsAndIndex(graphs, compressedGraphIdx);
          curGraphsPos.setGraphsAndIndex(graphs.clone(), compressedGraphIdx);
        }
        // java return hasCompressed;
        return hasCompressed;
      }
    }

    // java private void getFirstExcessGraphIndexAndUncompressedLeftPos(Graphs graphs, TypoPara typoPara, int subLineWidth, int usedSpaceSum)
    pub fn getFirstExcessGraphIndexAndUncompressedLeftPos(&mut self, graphs: &Graphs, typoPara: &TypoPara, subLineWidth: i32, usedSpaceSum: i32) {
      // java int graphStartIdx = graphs.graphIndex;
      let mut graphStartIdx: i32 = graphs.graphIndex;
      // java int lastGraphIdx = graphs.getEndGraphIndex();
      let mut lastGraphIdx: i32 = graphs.getEndGraphIndex();
      // java int[] widthBuff = this.mWidthBuff;
      let mut widthBuff: Vec<i32> = self.mWidthBuff.clone();
      // java this.mWidthCacheStart = graphStartIdx;
      self.mWidthCacheStart = graphStartIdx;
      // java this.mWidthCacheEnd = graphStartIdx + Math.min(graphs.graphLength, 32);
      self.mWidthCacheEnd = graphStartIdx + (graphs.graphLength).min(32);
      // java typoPara.getWidths(graphStartIdx, this.mWidthCacheEnd - this.mWidthCacheStart, widthBuff, 0);
      let cache_len = self.mWidthCacheEnd - self.mWidthCacheStart;
      typoPara.getWidths(graphStartIdx, cache_len, &mut widthBuff, 0);
      self.mWidthBuff.clone_from(&widthBuff);
      // java int bdrBegin = graphs.hasBeginBdr() ? graphs.getBdrWidth() : 0;
      let mut bdrBegin: i32 = (if graphs.hasBeginBdr() { graphs.getBdrWidth() } else { 0 });
      // java int orgRight = graphs.getLeft() + Math.abs(widthBuff[graphStartIdx - this.mWidthCacheStart]) + bdrBegin;
      let mut orgRight: i32 = graphs.getLeft() + (widthBuff[(graphStartIdx - self.mWidthCacheStart) as usize]).abs() + bdrBegin;
      // java int orgCompressedRight = orgRight - usedSpaceSum;
      let mut orgCompressedRight: i32 = orgRight - usedSpaceSum;
      // java int graphIdx = graphs.graphIndex;
      let mut graphIdx: i32 = graphs.graphIndex;
      // java int right = orgCompressedRight;
      let mut right: i32 = orgCompressedRight;
      // java int graphWidth = widthBuff[graphIdx - this.mWidthCacheStart] + bdrBegin;
      let mut graphWidth: i32 = widthBuff[(graphIdx - self.mWidthCacheStart) as usize] + bdrBegin;
      // java this.mFirstExcessGraphIndex = Integer.MAX_VALUE;
      self.mFirstExcessGraphIndex = i32::MAX;
      self.mFirstExcessGraphUncompressedLeftPos = i32::MIN;
      // java for(this.mFirstExcessGraphUncompressedLeftPos = Integer.MIN_VALUE right < subLineWidth right += graphWidth)
      while right < subLineWidth {
        // java ++graphIdx;
        { let _tmp = graphIdx; graphIdx += 1; _tmp };
        // java if (graphIdx > lastGraphIdx)
        if graphIdx > lastGraphIdx {
          // java return;
          return;
        }
        // java if (graphIdx >= this.mWidthCacheEnd)
        if graphIdx >= self.mWidthCacheEnd {
          // java this.mWidthCacheStart = graphIdx;
          self.mWidthCacheStart = graphIdx;
          // java this.mWidthCacheEnd = graphIdx + Math.min(graphs.graphLength + graphs.graphIndex - graphIdx, 32);
          self.mWidthCacheEnd = graphIdx + (graphs.graphLength + graphs.graphIndex - graphIdx).min(32);
          // java typoPara.getWidths(graphIdx, this.mWidthCacheEnd - graphIdx, widthBuff, 0);
          typoPara.getWidths(graphIdx, self.mWidthCacheEnd - graphIdx, &mut widthBuff, 0);
          self.mWidthBuff.clone_from(&widthBuff);
        }
        // java graphWidth = Math.abs(widthBuff[graphIdx - this.mWidthCacheStart]);
        graphWidth = (widthBuff[(graphIdx - self.mWidthCacheStart) as usize]).abs();
        right += graphWidth;
      }
      // java int uncompressedRight = orgRight + right - orgCompressedRight;
      let mut uncompressedRight: i32 = orgRight + right - orgCompressedRight;
      // java int uncompressedLeft = uncompressedRight - graphWidth;
      let mut uncompressedLeft: i32 = uncompressedRight - graphWidth;
      // java this.mFirstExcessGraphIndex = graphIdx;
      self.mFirstExcessGraphIndex = graphIdx;
      // java this.mFirstExcessGraphUncompressedLeftPos = uncompressedLeft;
      self.mFirstExcessGraphUncompressedLeftPos = uncompressedLeft;
    }

    // java protected boolean compress(LineRuleGraphsPos graphsPos, int subLineWidth, LayoutState layoutState)
    pub fn compress(&mut self, graphsPos: &mut LineRuleGraphsPos, mut subLineWidth: i32, layoutState: &mut LayoutState) -> bool {
      // java SubLineEnv subLineEnv = this.mSubLineEnv;
      let graphs: Graphs = graphsPos.getGraphs();
      let usedSpaceSum: i32 = {
        let subLineEnv = self.subLineEnv();
        // java LineCompressEnv lineRuleEnv = this.mSubLineEnv.lineCompressEnv;
        let lineRuleEnv = subLineEnv.lineCompressEnv.as_ref().unwrap();
        lineRuleEnv.getUsedCompressableSpaceSum()
      };
      // java if (graphs.hasBdr() && !graphs.hasEndBdr())
      if graphs.hasBdr() && !graphs.hasEndBdr() {
        // java subLineWidth -= graphs.getBdrWidth();
        subLineWidth = subLineWidth - graphs.getBdrWidth();
      }
      // java this.getFirstExcessGraphIndexAndUncompressedLeftPos(graphs, layoutState.typoPara, subLineWidth, usedSpaceSum);
      self.getFirstExcessGraphIndexAndUncompressedLeftPos(&graphs, &layoutState.typoPara, subLineWidth, usedSpaceSum);
      // java int firstExcessGraphIdx = this.mFirstExcessGraphIndex;
      let mut firstExcessGraphIdx: i32 = self.mFirstExcessGraphIndex;
      // java int firstExcessGraphLeft = this.mFirstExcessGraphUncompressedLeftPos;
      let mut firstExcessGraphLeft: i32 = self.mFirstExcessGraphUncompressedLeftPos;
      // java int lastGraphIdx = graphs.getEndGraphIndex();
      let mut lastGraphIdx: i32 = graphs.getEndGraphIndex();
      if std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp) {
        eprintln!("compress cp={} graph={} firstExcess={} left={} last={} followed={} used={} max={}",
          layoutState.cp, graphs.graphIndex, firstExcessGraphIdx, firstExcessGraphLeft,
          lastGraphIdx, graphsPos.shouldBeFollowed(), usedSpaceSum, subLineWidth);
      }
      // java if (firstExcessGraphIdx > lastGraphIdx)
      if firstExcessGraphIdx > lastGraphIdx {
        // java return true;
        return true;
      }
      // java else if (graphsPos.shouldBeFollowed())
      else if graphsPos.shouldBeFollowed() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java boolean hasCompressed = false;
        let mut hasCompressed: bool = false;
        // java graphsPos.setGraphsAndIndex(graphs, firstExcessGraphIdx);
        graphsPos.setGraphsAndIndex(graphs.clone(), firstExcessGraphIdx);
        // java int runType = graphs.runType;
        let mut runType: i32 = graphs.runType;
        // java if (TypoRun.isSpace(runType))
        if TypoRun::isSpace(runType) {
          // java if (this.canHangSpace(lineRuleEnv, graphsPos, layoutState))
          if self.canHangSpace(self.subLineEnv().lineCompressEnv.as_ref().unwrap(), graphsPos, layoutState) {
            // java hasCompressed = true;
            hasCompressed = true;
            // java graphsPos.setGraphsAndIndex(graphs, lastGraphIdx);
            graphsPos.setGraphsAndIndex(graphs.clone(), lastGraphIdx);
            // java if (lineRuleEnv.firstExcessSpaceGraphIndex < 0)
            let env = self.subLineEnv_mut().lineCompressEnv.as_mut().unwrap();
            if env.firstExcessSpaceGraphIndex < 0 {
              // java lineRuleEnv.firstExcessSpaceGraphIndex = graphs.graphIndex;
              env.firstExcessSpaceGraphIndex = graphs.graphIndex;
            }
          }
          // java else
          else {
            // java hasCompressed = this.doCompressNormal(lineRuleEnv, graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
            hasCompressed = self.doCompressNormal(graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
          }
        }
        // java else if (18 == runType)
        else if 18 == runType {
          // java hasCompressed = this.doCompressNormal(lineRuleEnv, graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
          hasCompressed = self.doCompressNormal(graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
          // java if (!hasCompressed && this.canHangPunct(lineRuleEnv, graphsPos, layoutState))
          if !hasCompressed && self.canHangPunct(self.subLineEnv().lineCompressEnv.as_ref().unwrap(), graphsPos, layoutState) {
            // java hasCompressed = true;
            hasCompressed = true;
            // java graphsPos.setGraphsAndIndex(graphs, lastGraphIdx);
            graphsPos.setGraphsAndIndex(graphs.clone(), lastGraphIdx);
            // java if (lineRuleEnv.firstExcessPunctGraphIndex < 0)
            let env = self.subLineEnv_mut().lineCompressEnv.as_mut().unwrap();
            if env.firstExcessPunctGraphIndex < 0 {
              // java lineRuleEnv.firstExcessPunctGraphIndex = graphs.graphIndex;
              env.firstExcessPunctGraphIndex = graphs.graphIndex;
            }
          }
        }
        // java else if (6 == runType)
        else if 6 == runType {
          // java hasCompressed = this.doCompressNormal(lineRuleEnv, graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
          hasCompressed = self.doCompressNormal(graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
          // java if (!hasCompressed && this.canHangEmbed(lineRuleEnv, graphsPos, layoutState))
          if !hasCompressed && self.canHangEmbed(self.subLineEnv().lineCompressEnv.as_ref().unwrap(), graphsPos, layoutState) {
            // java hasCompressed = true;
            hasCompressed = true;
            // java graphsPos.setGraphsAndIndex(graphs, lastGraphIdx);
            graphsPos.setGraphsAndIndex(graphs.clone(), lastGraphIdx);
          }
        }
        // java else
        else {
          // java hasCompressed = this.doCompressNormal(lineRuleEnv, graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
          hasCompressed = self.doCompressNormal(graphsPos, firstExcessGraphLeft, subLineWidth, layoutState);
        }
        // java if (!hasCompressed)
        if !hasCompressed {
          // java graphsPos.init(graphs, firstExcessGraphIdx);
          graphsPos.init(graphs.clone(), firstExcessGraphIdx);
          // java boolean found = graphsPos.prev(false, layoutState.getGraphList(), this.mSubLineEnv);
          let mut found: bool = graphsPos.prev(false, &layoutState.graphList, self.subLineEnv());
          // java if (!found && subLineEnv.atLeastOneChar)
          if !found && self.subLineEnv().atLeastOneChar {
            // java Graphs firstGraphs = subLineEnv.firstGraphs;
            let mut firstGraphs: Graphs = self.subLineEnv().firstGraphs.clone().unwrap_or_else(Graphs::Graphs);
            // java graphsPos.setGraphsAndIndex(firstGraphs, firstGraphs.graphIndex);
            graphsPos.setGraphsAndIndex(firstGraphs.clone(), firstGraphs.graphIndex);
            // java if (!graphsPos.isBreakableIfAtleastOnChar())
            if !graphsPos.isBreakableIfAtleastOnChar() {
              // java graphsPos.setGraphsAndIndex(firstGraphs, firstGraphs.getEndGraphIndex());
              let end_idx = firstGraphs.getEndGraphIndex();
              graphsPos.setGraphsAndIndex(firstGraphs, end_idx);
            }
          }
          // java if (graphsPos.isInvalid())
          if graphsPos.isInvalid() {
            // java return false;
            return false;
          }
          // java if (graphsPos.shouldBeFollowed())
          if graphsPos.shouldBeFollowed() {
            // java Graphs graphsBak = graphsPos.getGraphs();
            let mut graphsBak: Graphs = graphsPos.getGraphs();
            // java int graphIdxBak = graphsPos.getGraphIndex();
            let mut graphIdxBak: i32 = graphsPos.getGraphIndex();
            // java if (!graphsPos.next(layoutState.getGraphList()))
            if !graphsPos.next(&layoutState.graphList) {
              // java graphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
              graphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
            }
          }
        }
        // java return graphsPos.getGraphIndex() == lastGraphIdx;
        return graphsPos.getGraphIndex() == lastGraphIdx;
      }
    }

}
