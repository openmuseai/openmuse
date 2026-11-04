// Java: import cn.wps.collections.KPList
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList;
// Java: import cn.wps.core.runtime.Assert
// CARGO_OFF_SURFACE use util::runtime::Assert::Assert;
// Java: import cn.wps.moffice.writer.base.ViewMode
use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.core.AsiaTypography
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::AsiaTypography::AsiaTypography;
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
// Java: import cn.wps.moffice.writer.layout.base.languages.ILangLayout
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::ILangLayout;
// Java: import cn.wps.moffice.writer.layout.base.line.align.IHAlignHandler
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.punctSpace.PunctTool
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::PunctTool::PunctTool;
// Java: import cn.wps.moffice.writer.layout.base.line.LineCompressTool
use crate::cn::wps::moffice::writer::layout::base::line::LineCompressTool::LineCompressTool;
// Java: import cn.wps.moffice.writer.layout.base.line.LineRuleGraphsPos
use crate::cn::wps::moffice::writer::layout::base::line::LineRuleGraphsPos::LineRuleGraphsPos;
// Java: import cn.wps.moffice.writer.layout.base.line.TabHandler
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::TabHandler::TabHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.SubLineEnv
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
// Java: import cn.wps.moffice.writer.layout.base.line.LineCompressEnv
use crate::cn::wps::moffice::writer::layout::base::line::LineCompressEnv::LineCompressEnv;

use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::layout::base::line::TabHandler::TabHandler;

#[derive(Default)]
pub struct RollBackResult {
    pub adjusted: bool,
    pub rollbackedAll: bool,
}
// rust: ★T7-5a —— 就地影子 `pub struct AsiaTypography`（自造 `isLineStartPunct` = `PunctTool::isHeadPunct`，
//   与 Java 的**文档级 kinsoku 表**语义不同）已删；改用真实 `core::AsiaTypography`（309 行，
//   `initKinsoku` 装载 5 组语言标点表 + 从文档属性读设置）。
use crate::cn::wps::moffice::writer::core::AsiaTypography::AsiaTypography;

// java public class SubLineLayouterBase extends LineCompressTool
#[derive(Default)]
pub struct SubLineLayouterBase {
    // java LineCompressTool super_
    pub super_: LineCompressTool,
    // java LineRuleGraphsPos mGraphsPos = new LineRuleGraphsPos();
    pub(crate) mGraphsPos: Option<LineRuleGraphsPos>,
    // java boolean mAdjusted;
    pub mAdjusted: bool,
    // java boolean mRollbackedAll;
    pub mRollbackedAll: bool,
    // java boolean mNextLineStartsWithHeadPunct;
    pub mNextLineStartsWithHeadPunct: bool,
    // java private ILangLayout.RollBackResult mRollBackResult = new ILangLayout.RollBackResult();
    mRollBackResult: Option<RollBackResult>,
}

// java public class SubLineLayouterBase extends LineCompressTool
impl SubLineLayouterBase {
    pub fn super_mut(&mut self) -> &mut LineCompressTool {
        &mut self.super_
    }
    pub fn super_ref(&self) -> &LineCompressTool {
        &self.super_
    }

    // java public  SubLineLayouterBase(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn SubLineLayouterBase(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
      // java super(mainState, docEnv, viewEnv);
      Self { super_: LineCompressTool::LineCompressTool(mainState, docEnv, viewEnv), mGraphsPos: Some(LineRuleGraphsPos::default()), mAdjusted: Default::default(), mRollbackedAll: Default::default(), mNextLineStartsWithHeadPunct: Default::default(), mRollBackResult: Some(RollBackResult::default()) }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mGraphsPos)
      if self.mGraphsPos.is_some() {
        // java this.mGraphsPos.dispose();
        self.mGraphsPos.as_mut().unwrap().dispose();
        // java this.mGraphsPos = null;
        self.mGraphsPos = None;
      }
      // java super.dispose();
      self.super_.dispose();
    }

    // java protected boolean doAddExcessGraphs(IHAlignHandler alignHandler, TabHandler tabHandler, Graphs excessGraphs, int subLineWidth, LayoutState layoutState)
    pub fn doAddExcessGraphs(&mut self, alignHandler: &mut dyn IHAlignHandler, tabHandler: Option<&mut TabHandler>, excessGraphs: *mut Graphs, subLineWidth: i32, layoutState: &mut LayoutState) -> bool {
      // java LineRuleGraphsPos graphsPos = this.mGraphsPos;
      let mut graphsPos = self.mGraphsPos.take().expect("mGraphsPos");
      // java graphsPos.init(excessGraphs, excessGraphs.graphIndex);
      graphsPos.init(unsafe { (*excessGraphs).clone() }, unsafe { (*excessGraphs).graphIndex });
      // java boolean hasCompressedWhole = this.compress(graphsPos, subLineWidth, layoutState);
      let hasCompressedWhole = self.super_.compress(&mut graphsPos, subLineWidth, layoutState);
      // java if (hasCompressedWhole)
      if hasCompressedWhole {
        // java alignHandler.onAddRun(excessGraphs);
        alignHandler.onAddRun(unsafe { &mut *excessGraphs });
        // java return true;
        self.mGraphsPos = Some(graphsPos);
        return true;
      }
      // java else
      else {
        // java boolean hasRollbackedAll = graphsPos.isInvalid();
        let mut hasRollbackedAll = graphsPos.isInvalid();
        // java if (!hasRollbackedAll) { this.setLastGraphsPosPrevLimit(...); hasRollbackedAll = this.rollback(...); }
        if !hasRollbackedAll {
          self.setLastGraphsPosPrevLimit(&mut graphsPos, &layoutState.graphList);
          hasRollbackedAll = self.rollback(&mut graphsPos, layoutState);
        }
        // java Graphs lastGraphs = hasRollbackedAll ? null : graphsPos.getGraphs();
        let last_ptr: *mut Graphs = if hasRollbackedAll { std::ptr::null_mut() } else {
          let last = graphsPos.getGraphs();
          layoutState.graphList.nodes.iter_mut()
            .find(|node| node.graphIndex == last.graphIndex && node.graphLength == last.graphLength)
            .map(|node| &mut **node as *mut Graphs)
            .unwrap_or(std::ptr::null_mut())
        };
        // java int lastGraphIdx = hasRollbackedAll ? Integer.MIN_VALUE : graphsPos.getGraphIndex();
        let lastGraphIdx: i32 = if hasRollbackedAll { i32::MIN } else { graphsPos.getGraphIndex() };
        // java int lastAligned = hasRollbackedAll ? Integer.MIN_VALUE : excessGraphs.graphIndex - 1;
        let lastAligned: i32 = if hasRollbackedAll { i32::MIN } else { unsafe { (*excessGraphs).graphIndex - 1 } };
        // java this.rollbackAndAlignGraphs(alignHandler, tabHandler, lastGraphs, lastGraphIdx, lastAligned, layoutState);
        self.rollbackAndAlignGraphs(alignHandler, tabHandler, last_ptr, lastGraphIdx, lastAligned, layoutState);
        // java return false;
        self.mGraphsPos = Some(graphsPos);
        return false;
      }
    }

    // java protected boolean rollbackWhileInMathPara(IHAlignHandler alignHandler, TabHandler tabHandler, Graphs excessGraphs, int subLineWidth, LayoutState layoutState)
    pub fn rollbackWhileInMathPara(&mut self, alignHandler: &mut dyn IHAlignHandler, tabHandler: Option<&mut TabHandler>, excessGraphs: Graphs, subLineWidth: i32, layoutState: &mut LayoutState) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean continueAddRun = true;
      let mut continueAddRun: bool = true;
      // java Graphs firstRollbackGraphs = MathParaSubLineTool.getFirstRollbackGraphs(layoutState, this.mSubLineEnv, excessGraphs);
      let mut firstRollbackGraphs: Graphs = MathParaSubLineTool::getFirstRollbackGraphs(layoutState, self.mSubLineEnv, excessGraphs);
      // java if (null == firstRollbackGraphs)
      if false {
        // java continueAddRun = this.doAddExcessGraphs(alignHandler, (TabHandler)null, excessGraphs, subLineWidth, layoutState);
        continueAddRun = self.doAddExcessGraphs(alignHandler, None, excessGraphs, subLineWidth, layoutState);
      }
      // java else
      else {
        // java KPList<Graphs> graphList = layoutState.getGraphList();
        let mut graphList: KPList = layoutState.getGraphList();
        // java if (excessGraphs != firstRollbackGraphs)
        if excessGraphs != firstRollbackGraphs {
          let mut var9: Graphs = (graphList.getPrev(excessGraphs)).downcast_ref::<Graphs>().unwrap();
          // java for(Graphs var9 = (Graphs)graphList.getPrev(excessGraphs); var9 != firstRollbackGraphs var9 = (Graphs)graphList.getPrev(var9))
          while var9 != firstRollbackGraphs {
            // java alignHandler.onRemoveRun(var9);
            alignHandler.onRemoveRun(var9);
          }
          // java alignHandler.onRemoveRun(firstRollbackGraphs);
          alignHandler.onRemoveRun(firstRollbackGraphs);
        }
        // java this.mSubLineEnv.lastGraphs = (Graphs)graphList.getPrev(firstRollbackGraphs);
        self.mSubLineEnv.lastGraphs = (graphList.getPrev(firstRollbackGraphs)).downcast_ref::<Graphs>().unwrap();
        // java layoutState.push(firstRollbackGraphs);
        layoutState.push__graphs(firstRollbackGraphs);
        // java continueAddRun = false;
        continueAddRun = false;
      }
      // java return continueAddRun;
      return continueAddRun;
      */
    }

    // java private void setLastGraphsPosPrevLimit(LineRuleGraphsPos lastGraphsPos, KPList<Graphs> graphList)
    pub fn setLastGraphsPosPrevLimit(&mut self, lastGraphsPos: &mut LineRuleGraphsPos, graphList: &KPList<Graphs>) {
      // java int graphIdxBak = lastGraphsPos.getGraphIndex();
      let graphIdxBak = lastGraphsPos.getGraphIndex();
      // java Graphs graphsBak = lastGraphsPos.getGraphs(); Graphs firstGraphs = this.mSubLineEnv.firstGraphs;
      let graphsBak = lastGraphsPos.getGraphs();
      let firstGraphs = self.super_.subLineEnv().firstGraphs.clone();
      if let Some(firstGraphs) = firstGraphs {
        lastGraphsPos.setGraphsAndIndex(firstGraphs.clone(), firstGraphs.graphIndex);
        // java lastGraphsPos.clearPrevLimitGraphsAndIndex();
        lastGraphsPos.clearPrevLimitGraphsAndIndex();
        // java for (hasLimit = false; lastGraphsPos.shouldBeFollowed(); hasLimit = true)
        let mut hasLimit = false;
        while lastGraphsPos.shouldBeFollowed() {
          let graphs = lastGraphsPos.getGraphs();
          lastGraphsPos.setGraphsAndIndex(graphs.clone(), graphs.getEndGraphIndex());
          lastGraphsPos.next(graphList);
          hasLimit = true;
        }
        if hasLimit {
          let env = self.super_.subLineEnv();
          lastGraphsPos.prev(false, graphList, env);
          let limitGraphIdx = lastGraphsPos.getGraphIndex();
          let limitGraphs = lastGraphsPos.getGraphs();
          lastGraphsPos.setPrevLimitGraphsAndIndex(Some(limitGraphs), limitGraphIdx);
        }
        // java lastGraphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
        lastGraphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
      }
      /* DEFERRED_NON_FUNCTION
      // java int graphIdxBak = lastGraphsPos.getGraphIndex();
      let mut graphIdxBak: i32 = lastGraphsPos.getGraphIndex();
      // java Graphs graphsBak = lastGraphsPos.getGraphs();
      let mut graphsBak: Graphs = lastGraphsPos.getGraphs();
      // java Graphs firstGraphs = this.mSubLineEnv.firstGraphs;
      let mut firstGraphs: Graphs = self.mSubLineEnv.firstGraphs;
      // java lastGraphsPos.setGraphsAndIndex(firstGraphs, firstGraphs.graphIndex);
      lastGraphsPos.setGraphsAndIndex(firstGraphs, firstGraphs.graphIndex);
      // java lastGraphsPos.clearPrevLimitGraphsAndIndex();
      lastGraphsPos.clearPrevLimitGraphsAndIndex();
      // java boolean hasLimit;
      let mut hasLimit: bool;
      hasLimit = false;
      // java for(hasLimit = false lastGraphsPos.shouldBeFollowed() hasLimit = true)
      while lastGraphsPos.shouldBeFollowed() {
        // java Graphs graphs = lastGraphsPos.getGraphs();
        let mut graphs: Graphs = lastGraphsPos.getGraphs();
        // java lastGraphsPos.setGraphsAndIndex(graphs, graphs.getEndGraphIndex());
        lastGraphsPos.setGraphsAndIndex(graphs, graphs.getEndGraphIndex());
        // java lastGraphsPos.next(graphList);
        lastGraphsPos.next(graphList);
      }
      // java if (hasLimit)
      if hasLimit {
        // java lastGraphsPos.prev(false, graphList, this.mSubLineEnv);
        lastGraphsPos.prev(false, graphList, self.mSubLineEnv);
        // java int limitGraphIdx = lastGraphsPos.getGraphIndex();
        let mut limitGraphIdx: i32 = lastGraphsPos.getGraphIndex();
        // java Graphs limitGraphs = lastGraphsPos.getGraphs();
        let mut limitGraphs: Graphs = lastGraphsPos.getGraphs();
        // java lastGraphsPos.setPrevLimitGraphsAndIndex(limitGraphs, limitGraphIdx);
        lastGraphsPos.setPrevLimitGraphsAndIndex(limitGraphs, limitGraphIdx);
      }
      // java lastGraphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
      lastGraphsPos.setGraphsAndIndex(graphsBak, graphIdxBak);
      */
    }

    // java private void rollbackAndAlignGraphs(IHAlignHandler alignHandler, TabHandler tabHandler, Graphs lastGraphs, int lastGraphIdx, int lastAlignedGraphIdx, LayoutState layoutState)
    pub fn rollbackAndAlignGraphs(&mut self, alignHandler: &mut dyn IHAlignHandler, tabHandler: Option<&mut TabHandler>, lastGraphs: *mut Graphs, lastGraphIdx: i32, lastAlignedGraphIdx: i32, layoutState: &mut LayoutState) {
      // java SubLineEnv subLineEnv = this.mSubLineEnv;
      let subLineEnv = self.super_.subLineEnv_mut();
      // java if (null == lastGraphs)
      if lastGraphs.is_null() {
        // java layoutState.push(subLineEnv.firstGraphs);
        if let Some(first) = subLineEnv.firstGraphs.as_ref() {
          layoutState.push__graphs(first);
        }
        // java subLineEnv.firstGraphs = subLineEnv.lastGraphs = null;
        subLineEnv.firstGraphs = None;
        subLineEnv.lastGraphs = None;
        subLineEnv.firstGraphNode = None;
        subLineEnv.lastGraphNode = None;
      } else {
        // Java: it.seek(subLineEnv.lastGraphs); walk backwards to lastGraphs.
        // The alignment handler has already accepted all runs up to the
        // excess run. Every rolled-back node must be removed from its counts.
        let mut run = subLineEnv.lastGraphNode.unwrap_or(std::ptr::null_mut());
        let mut nextRun: *mut Graphs = std::ptr::null_mut();
        while !run.is_null() {
          let isAlignedRun = unsafe { (*run).graphIndex <= lastAlignedGraphIdx };
          if run == lastGraphs {
            // Java: if (lastGraphIdx < run.getEndGraphIndex())
            if lastGraphIdx < unsafe { (*run).getEndGraphIndex() } {
              if isAlignedRun { alignHandler.onRemoveRun(unsafe { &mut *run }); }
              layoutState.breakGraphs(unsafe { &mut *run }, false, lastGraphIdx + 1, true);
              alignHandler.onAddRun(unsafe { &mut *run });
            } else if !nextRun.is_null() {
              // Java: restore from the retained non-space run when its
              // trailing graph was compressed; otherwise start at nextRun.
              let restoreFirst = if !TypoRun::isSpace(unsafe { (*run).runType })
                  && subLineEnv.lineCompressEnv.as_ref().is_some_and(|env|
                      env.getCompressAfter(unsafe { (*run).getEndGraphIndex() }) > 0) {
                run
              } else { nextRun };
              layoutState.push_pair_ptr(nextRun, restoreFirst);
            }
            break;
          }
          // Java: it.prev(); if (isAlignedRun) alignHandler.onRemoveRun(run).
          let prev = layoutState.getGraphList().getPrev(run);
          if isAlignedRun { alignHandler.onRemoveRun(unsafe { &mut *run }); }
          nextRun = run;
          run = prev;
        }
        let kept = if run.is_null() { nextRun } else { lastGraphs };
        subLineEnv.lastGraphNode = (!kept.is_null()).then_some(kept);
        subLineEnv.lastGraphs = (!kept.is_null()).then(|| unsafe { (*kept).clone() });
        if let Some(tabHandler) = tabHandler {
          if !kept.is_null() {
            tabHandler.onAddRun(unsafe { &mut *kept }, 0);
            tabHandler.commit();
          }
        }
      }
      /* DEFERRED_NON_FUNCTION 迭代器回退全链（数学/制表/phone）；正文溢出走上面 Java 同序 split
      // java SubLineEnv subLineEnv = this.mSubLineEnv;
      let mut subLineEnv: SubLineEnv = self.mSubLineEnv.unwrap();
      // java if (null == lastGraphs)
      if lastGraphs.is_none() {
        // java layoutState.push(subLineEnv.firstGraphs);
        layoutState.push__graphs(subLineEnv.firstGraphs);
        // java subLineEnv.firstGraphs = subLineEnv.lastGraphs = null;
        subLineEnv.firstGraphs = { subLineEnv.lastGraphs = None; subLineEnv.lastGraphs };
      }
      // java else
      else {
        // java Graphs run = null;
        let mut run: Graphs = Default::default();
        // java Graphs nextRun = null;
        let mut nextRun: Graphs = Default::default();
        // java KPList.Iterator<Graphs> it = layoutState.getGraphList().iterator();
        let mut it: Vec<Graphs> = layoutState.getGraphList().iterator();
        // java it.seek(subLineEnv.lastGraphs);
        it.seek(subLineEnv.lastGraphs);
        // java for(null != (run = (Graphs)it.value()) nextRun = run)
        while { run = (it.value()).downcast_ref::<Graphs>().unwrap(); run }.is_some() {
          // java boolean isAlignedRun = run.graphIndex <= lastAlignedGraphIdx;
          let mut isAlignedRun: bool = run.graphIndex <= lastAlignedGraphIdx;
          // java if (run == lastGraphs)
          if run == lastGraphs {
            // java if (lastGraphIdx < run.getEndGraphIndex())
            if lastGraphIdx < run.getEndGraphIndex() {
              // java if (isAlignedRun)
              if isAlignedRun {
                // java alignHandler.onRemoveRun(run);
                alignHandler.onRemoveRun(run);
              }
              // java layoutState.breakGraphs(run, false, lastGraphIdx + 1, true);
              layoutState.breakGraphs(run, false, lastGraphIdx + 1, true);
              // java alignHandler.onAddRun(run);
              alignHandler.onAddRun(run);
            }
            // java else if (null != nextRun)
            else if true {
              // java Graphs restoreFirst = nextRun;
              let mut restoreFirst: Graphs = nextRun;
              // java if (!TypoRun.isSpace(run.runType) && subLineEnv.lineCompressEnv.getCompressAfter(run.getEndGraphIndex()) > 0)
              if !TypoRun::isSpace(run.runType) && subLineEnv.lineCompressEnv.getCompressAfter(run.getEndGraphIndex()) > 0 {
                // java restoreFirst = run;
                restoreFirst = run;
              }
              // java layoutState.push(nextRun, restoreFirst);
              layoutState.push__graphs_graphs(nextRun, restoreFirst);
            }
            // java break;
            break;
          }
          // java it.prev();
          it.prev();
          // java if (isAlignedRun)
          if isAlignedRun {
            // java alignHandler.onRemoveRun(run);
            alignHandler.onRemoveRun(run);
            // java if (ViewMode.isPhoneReadMode(this.mViewEnv.getViewMode()) && run.runType == 6 && this.mViewEnv.getPhoneViewParam().isNoBothAlignAfterEmbedBreak() && !layoutState.checkIsRtlToKeepSpace(this.mViewEnv.getPhoneViewParam()))
            if ViewMode::isPhoneReadMode(self.mViewEnv.getViewMode()) && run.runType == 6 && self.mViewEnv.getPhoneViewParam().isNoBothAlignAfterEmbedBreak() && !layoutState.checkIsRtlToKeepSpace(self.mViewEnv.getPhoneViewParam()) {
              // java this.mSubLineEnv.mStopBothAlign = true;
              self.mSubLineEnv.mStopBothAlign = true;
            }
          }
        }
        // java if (null == run)
        if false {
          // java it.next();
          it.next();
          // java subLineEnv.lastGraphs = (Graphs)it.value();
          subLineEnv.lastGraphs = (it.value()).downcast_ref::<Graphs>().unwrap();
        }
        // java else
        else {
          // java subLineEnv.lastGraphs = lastGraphs;
          subLineEnv.lastGraphs = lastGraphs;
        }
        // java if (null != tabHandler && subLineEnv.hasGraphs())
        if let Some(tabHandler) = tabHandler {
          if subLineEnv.hasGraphs() {
            // java tabHandler.onAddRun(subLineEnv.lastGraphs, 0);
            tabHandler.onAddRun(subLineEnv.lastGraphs, 0);
            // java tabHandler.commit();
            tabHandler.commit();
          }
        }
        // java it.recycle();
        it.recycle();
      }
      */
    }

    // java private boolean rollback(LineRuleGraphsPos lastGraphsPos, LayoutState layoutState)
    pub fn rollback(&mut self, lastGraphsPos: &mut LineRuleGraphsPos, layoutState: &mut LayoutState) -> bool {
      // java if (lastGraphsPos.isInvalid())
      if lastGraphsPos.isInvalid() {
        // java return false;
        return false;
      }
      // java else if (TypoRun.isLineBreak(lastGraphsPos.getGraphs().runType))
      else if TypoRun::isLineBreak(lastGraphsPos.getGraphs().runType) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int lastGraphIdxBak = lastGraphsPos.getGraphIndex();
        let lastGraphIdxBak = lastGraphsPos.getGraphIndex();
        // java Graphs lastGraphsBak = lastGraphsPos.getGraphs();
        let lastGraphsBak = lastGraphsPos.getGraphs();
        // java boolean wordWrap = layoutState.paraEnv.wordWrap;
        let wordWrap = layoutState.paraEnv.wordWrap;
        // java boolean kinsoku = layoutState.paraEnv.kinsoku;
        let kinsoku = layoutState.paraEnv.kinsoku;
        // java AsiaTypography asiaTypography = this.mDocEnv.textDocument.getAsiaTypography();
        let doc_env = self.super_.mDocEnv.as_mut().expect("mDocEnv");
        let asia_ptr: *const AsiaTypography = if let Some(doc) = doc_env.textDocument.as_mut() {
          doc.getAsiaTypography()
        } else {
          doc_env.textDocumentAsiaTypography.as_ref().expect("DocumentEnv.updateFromDocument")
        };
        let asiaTypography = unsafe { &*asia_ptr };
        // java LineRuleGraphsPos nextGraphsPos = this.mTmpGraphsPos;
        let mut nextGraphsPos = self.super_.mTmpGraphsPos.take().expect("mTmpGraphsPos");
        // java nextGraphsPos.copy(lastGraphsPos);
        nextGraphsPos.copy(lastGraphsPos);
        // java KPList<Graphs> graphList = layoutState.getGraphList();
        let graph_list_ptr: *const KPList<Graphs> = &layoutState.graphList;
        let graphList = unsafe { &*graph_list_ptr };
        // java boolean hasNext = nextGraphsPos.next(graphList);
        let hasNext = nextGraphsPos.next(graphList);
        // java if (!hasNext) { return false; }
        if !hasNext {
          self.super_.mTmpGraphsPos = Some(nextGraphsPos);
          return false;
        }
        // java TypoPara typoPara = layoutState.typoPara;
        let typo_ptr: *const TypoPara = &layoutState.typoPara;
        let typoPara = unsafe { &*typo_ptr };
        // java if (this.canSkipRollback(asiaTypography, lastGraphsPos, nextGraphsPos, typoPara)) { return false; }
        if self.canSkipRollback(asiaTypography, lastGraphsPos, &nextGraphsPos, typoPara) {
          self.super_.mTmpGraphsPos = Some(nextGraphsPos);
          return false;
        }
        // java boolean hasAdjusted = false;
        // java boolean hasRollbackedAll = false;
        let mut hasAdjusted;
        let mut hasRollbackedAll = false;
        // java do { hasAdjusted = false; ... } while(hasAdjusted);
        loop {
          hasAdjusted = false;
          // java Graphs lastGraphs = lastGraphsPos.getGraphs();
          // java int lastRunType = lastGraphs.runType;
          let lastGraphs = lastGraphsPos.getGraphs();
          let lastRunType = lastGraphs.runType;
          if lastRunType != 15 && lastRunType != 16 {
            if wordWrap {
              // java this.mSubLineEnv.mAutoHyphenation = this.checkCanAutoWordBreak(layoutState);
              let auto_hyphenation = self.checkCanAutoWordBreak(layoutState);
              self.super_.subLineEnv_mut().mAutoHyphenation = auto_hyphenation;
              // java if (this.mSubLineEnv.mAutoHyphenation) { this.mSubLineEnv.mHyphenationZone = this.mViewEnv.getPhoneViewParam().getHyphenationZone(); }
              // rust: PhoneViewParam 是当前产品范围外的 deferred 面；此分支尚未验证。
              // java this.rollbackIfCurLineEndsWithPartOfWord(kinsoku, lastGraphsPos, nextGraphsPos, typoPara, graphList);
              self.rollbackIfCurLineEndsWithPartOfWord(kinsoku, lastGraphsPos, &mut nextGraphsPos, typoPara, graphList);
              hasAdjusted |= self.mAdjusted;
              hasRollbackedAll = self.mRollbackedAll;
              if hasRollbackedAll { break; }
            }
          // java else { int endGraphIndex = lastGraphs.getEndGraphIndex(); if (endGraphIndex != lastGraphsPos.getGraphIndex()) ... }
          } else if lastGraphs.getEndGraphIndex() != lastGraphsPos.getGraphIndex() {
            lastGraphsPos.setGraphsAndIndex(lastGraphs.clone(), lastGraphs.graphIndex);
            lastGraphsPos.prev(false, graphList, self.super_.subLineEnv());
            nextGraphsPos.setGraphsAndIndex(lastGraphs.clone(), lastGraphs.graphIndex);
            if lastGraphsPos.isInvalid() {
              if self.super_.subLineEnv().atLeastOneChar {
                lastGraphsPos.setGraphsAndIndex(lastGraphs.clone(), lastGraphs.getEndGraphIndex());
                nextGraphsPos.copy(lastGraphsPos);
                nextGraphsPos.next(graphList);
              } else { hasRollbackedAll = true; }
            }
            hasAdjusted = true;
          }
          // java if (kinsoku) { ... }
          if kinsoku {
            // java this.rollbackIfNextLineStartsWithHeadPuncts(wordWrap, lastGraphsPos, nextGraphsPos, typoPara, graphList);
            self.rollbackIfNextLineStartsWithHeadPuncts(wordWrap, lastGraphsPos, &mut nextGraphsPos, typoPara, graphList);
            hasAdjusted = self.mAdjusted;
            hasRollbackedAll = self.mRollbackedAll;
            if hasRollbackedAll { break; }
            // java this.rollbackIfNextLineStartsWithTailPuncts(wordWrap, asiaTypography, lastGraphsPos, nextGraphsPos, typoPara, graphList, layoutState);
            self.rollbackIfNextLineStartsWithTailPuncts(wordWrap, asiaTypography, lastGraphsPos, &mut nextGraphsPos, typoPara, graphList, layoutState);
            hasAdjusted |= self.mAdjusted;
            hasRollbackedAll = self.mRollbackedAll;
            if hasRollbackedAll { break; }
          }
          // java if (!hasAdjusted) { ... }
          if !hasAdjusted {
            // java this.doAlternativeRollback(wordWrap, kinsoku, asiaTypography, lastGraphsPos, nextGraphsPos, typoPara, graphList);
            self.doAlternativeRollback(wordWrap, kinsoku, asiaTypography, lastGraphsPos, &mut nextGraphsPos, typoPara, graphList);
            hasAdjusted |= self.mAdjusted;
            hasRollbackedAll |= self.mRollbackedAll;
            if hasRollbackedAll { break; }
          }
          if !hasAdjusted { break; }
        }
        // java if (!hasRollbackedAll) { ... }
        if !hasRollbackedAll {
          // java this.rollbackIfCurLineEndsWithTab(wordWrap, kinsoku, lastGraphsPos, nextGraphsPos, graphList);
          self.rollbackIfCurLineEndsWithTab(wordWrap, kinsoku, lastGraphsPos, &mut nextGraphsPos, graphList);
          hasRollbackedAll |= self.mRollbackedAll;
        }
        if hasRollbackedAll {
          // java if (!subLineEnv.atLeastOneChar) { return true; }
          if !self.super_.subLineEnv().atLeastOneChar {
            self.super_.mTmpGraphsPos = Some(nextGraphsPos);
            return true;
          }
          // java if (subLineEnv.lineCompressEnv.firstExcessSpaceGraphIndex >= 0) { boolean ret5 = this.rollbackIfHangingSpaces(lastGraphsPos, graphList); if (ret5) return false; }
          if self.super_.subLineEnv().lineCompressEnv.as_ref().is_some_and(|env| env.firstExcessSpaceGraphIndex >= 0)
            && self.rollbackIfHangingSpaces(lastGraphsPos, graphList) {
            self.super_.mTmpGraphsPos = Some(nextGraphsPos);
            return false;
          }
          lastGraphsPos.setGraphsAndIndex(lastGraphsBak, lastGraphIdxBak);
        }
        self.super_.mTmpGraphsPos = Some(nextGraphsPos);
        return false;
      }

    }

    // java private static void assertIsValidGraphsPos(LineRuleGraphsPos graphsPos)
    pub fn assertIsValidGraphsPos(graphsPos: &LineRuleGraphsPos) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Assert.assertFalse(graphsPos.isInvalid());
      Assert::assertFalse(graphsPos.isInvalid());
      */
    }

    // java private void rollbackIfCurLineEndsWithPartOfWord(boolean kinsoku, LineRuleGraphsPos lastGraphsPos, LineRuleGraphsPos nextGraphsPos, TypoPara typoPara, KPList<Graphs> graphsList)
    pub fn rollbackIfCurLineEndsWithPartOfWord(&mut self, kinsoku: bool, lastGraphsPos: &mut LineRuleGraphsPos, nextGraphsPos: &mut LineRuleGraphsPos, typoPara: &TypoPara, graphsList: &KPList<Graphs>) {
      let _ = kinsoku;
      // java ILangLayout langLayout = lastGraphsPos.getGraphs().mLangLayout;
      let _langLayout = lastGraphsPos.getGraphs().mLangLayout;
      // java langLayout.rollBackForPartialWord(lastGraphsPos, nextGraphsPos, this.mSubLineEnv, typoPara, graphsList, this.mDocEnv.compressLevel, this.mRollBackResult);
      let compressLevel: i32 = self.super_.mDocEnv.as_ref().map(|e| e.compressLevel).unwrap_or(0);
      {
        let result = self.mRollBackResult.as_mut().unwrap();
        result.adjusted = false;
        result.rollbackedAll = false;
        if lastGraphsPos.isPartOfWord(compressLevel, typoPara) && nextGraphsPos.isPartOfWord(compressLevel, typoPara) {
          let mut found: bool = false;
          loop {
            let graphs = lastGraphsPos.getGraphs();
            lastGraphsPos.setGraphsAndIndex(graphs.clone(), graphs.graphIndex);
            let hasPrev = lastGraphsPos.prev(false, graphsList, self.super_.subLineEnv());
            if !hasPrev {
              break;
            }
            if !lastGraphsPos.isPartOfWord(compressLevel, typoPara) {
              found = true;
              nextGraphsPos.copy(lastGraphsPos);
              nextGraphsPos.next(graphsList);
              break;
            }
          }
          let result = self.mRollBackResult.as_mut().unwrap();
          result.adjusted = true;
          if !found {
            result.rollbackedAll = true;
          }
        }
      }
      // java this.mAdjusted = this.mRollBackResult.adjusted;
      self.mAdjusted = self.mRollBackResult.as_ref().unwrap().adjusted;
      // java this.mRollbackedAll = this.mRollBackResult.rollbackedAll;
      self.mRollbackedAll = self.mRollBackResult.as_ref().unwrap().rollbackedAll;
    }

    // java  boolean checkCanAutoWordBreak(LayoutState state)
    pub fn checkCanAutoWordBreak(&self, state: &LayoutState) -> bool {
      // java return ViewMode.isPhoneReadMode(this.mViewEnv.getViewMode()) && this.mViewEnv.isAutoHyphenation() && !state.hasBidiGraph();
      let view = self.super_.mViewEnv.as_ref().unwrap();
      ViewMode::isPhoneReadMode(view.getViewMode()) && view.isAutoHyphenation() && !state.hasBidiGraph()

    }

    // java private void rollbackIfNextLineStartsWithHeadPuncts(boolean wordWrap, LineRuleGraphsPos lastGraphsPos, LineRuleGraphsPos nextGraphsPos, TypoPara typoPara, KPList<Graphs> graphsList)
    pub fn rollbackIfNextLineStartsWithHeadPuncts(&mut self, wordWrap: bool, lastGraphsPos: &mut LineRuleGraphsPos, nextGraphsPos: &mut LineRuleGraphsPos, typoPara: &TypoPara, graphsList: &KPList<Graphs>) {
      // java this.mAdjusted = false;
      self.mAdjusted = false;
      // java this.mRollbackedAll = false;
      self.mRollbackedAll = false;
      // java this.mNextLineStartsWithHeadPunct = false;
      self.mNextLineStartsWithHeadPunct = false;
      // java if (PunctTool.isHeadPunct(nextGraphsPos.getChar(typoPara)))
      if PunctTool::isHeadPunct(nextGraphsPos.getChar(typoPara)) {
        // java boolean hasValidPos = false;
        let mut hasValidPos: bool = false;
        // java int validGraphIdx = lastGraphsPos.getGraphIndex();
        let mut validGraphIdx: i32 = lastGraphsPos.getGraphIndex();
        // java Graphs validGraphs = lastGraphsPos.getGraphs();
        let mut validGraphs: Graphs = lastGraphsPos.getGraphs();
        // java if (PunctTool.isTailPunct(lastGraphsPos.getChar(typoPara)))
        if PunctTool::isTailPunct(lastGraphsPos.getChar(typoPara)) {
          // java this.mNextLineStartsWithHeadPunct = true;
          self.mNextLineStartsWithHeadPunct = true;
        }
        // java else
        else {
          // java boolean needToSkipSpaces = wordWrap;
          let mut needToSkipSpaces: bool = wordWrap;
          // java SubLineEnv subLineEnv = this.mSubLineEnv;
          // java while (true)
          loop {
            // java boolean found = lastGraphsPos.prev(needToSkipSpaces, graphsList, subLineEnv);
            let mut found: bool = lastGraphsPos.prev(needToSkipSpaces, graphsList, self.super_.subLineEnv());
            // java if (!found || !PunctTool.isHeadPunct(lastGraphsPos.getChar(typoPara)))
            if !found || !PunctTool::isHeadPunct(lastGraphsPos.getChar(typoPara)) {
              // java if (hasValidPos && lastGraphsPos.isInvalid())
              if hasValidPos && lastGraphsPos.isInvalid() {
                // java this.mAdjusted = true;
                self.mAdjusted = true;
                // java this.mRollbackedAll = true;
                self.mRollbackedAll = true;
              }
              // java else
              else {
                // java lastGraphsPos.setGraphsAndIndex(validGraphs, validGraphIdx);
                lastGraphsPos.setGraphsAndIndex(validGraphs, validGraphIdx);
                // java nextGraphsPos.copy(lastGraphsPos);
                nextGraphsPos.copy(lastGraphsPos);
                // java nextGraphsPos.next(graphsList);
                nextGraphsPos.next(graphsList);
              }
              // java this.mNextLineStartsWithHeadPunct = true;
              self.mNextLineStartsWithHeadPunct = true;
              // java return;
              return;
            }
            // java hasValidPos = true;
            hasValidPos = true;
            // java validGraphIdx = lastGraphsPos.getGraphIndex();
            validGraphIdx = lastGraphsPos.getGraphIndex();
            // java validGraphs = lastGraphsPos.getGraphs();
            validGraphs = lastGraphsPos.getGraphs();
          }
        }
      }
    }

    // java private void rollbackIfNextLineStartsWithTailPuncts(boolean wordWrap, AsiaTypography asiaTypography, LineRuleGraphsPos lastGraphsPos, LineRuleGraphsPos nextGraphsPos, TypoPara typoPara, KPList<Graphs> garphsList, LayoutState layoutState)
    pub fn rollbackIfNextLineStartsWithTailPuncts(&mut self, wordWrap: bool, asiaTypography: &AsiaTypography, lastGraphsPos: &mut LineRuleGraphsPos, nextGraphsPos: &mut LineRuleGraphsPos, typoPara: &TypoPara, garphsList: &KPList<Graphs>, layoutState: &mut LayoutState) {
      // java this.mAdjusted = false; this.mRollbackedAll = false;
      self.mAdjusted = false;
      self.mRollbackedAll = false;
      // java if (asiaTypography.isLineEndPunct(nextGraphsPos.getChar(typoPara)))
      if asiaTypography.isLineEndPunct(nextGraphsPos.getChar(typoPara)) {
        // java Graphs lastGraphs = lastGraphsPos.getGraphs(); int lastRunType = lastGraphs.runType;
        let lastGraphs = lastGraphsPos.getGraphs();
        if lastGraphs.runType == 15 || lastGraphs.runType == 16 {
          lastGraphsPos.setGraphsAndIndex(lastGraphs.clone(), lastGraphs.graphIndex);
        }
        // java boolean hasPrev = lastGraphsPos.prev(false, garphsList, subLineEnv);
        let mut hasPrev = lastGraphsPos.prev(false, garphsList, self.super_.subLineEnv());
        if hasPrev && lastGraphsPos.isPartOfComplexGraphs() {
          let graphs = lastGraphsPos.getGraphs();
          lastGraphsPos.setGraphsAndIndex(graphs.clone(), graphs.graphIndex);
          hasPrev = lastGraphsPos.prev(false, garphsList, self.super_.subLineEnv());
        }
        if hasPrev && wordWrap && self.isRealSpace(lastGraphsPos, layoutState) {
          // java boolean needToSkipSpaces = !this.mDocEnv.wrapTrailSpaces;
          let needToSkipSpaces = !self.super_.mDocEnv.as_ref().unwrap().wrapTrailSpaces;
          hasPrev = lastGraphsPos.prev(needToSkipSpaces, garphsList, self.super_.subLineEnv());
          hasPrev = hasPrev && lastGraphsPos.prev(false, garphsList, self.super_.subLineEnv());
        }
        if hasPrev {
          nextGraphsPos.copy(lastGraphsPos);
          nextGraphsPos.next(garphsList);
        }
        self.mAdjusted = true;
        if !hasPrev { self.mRollbackedAll = true; }
      }

    }

    // java private boolean isRealSpace(LineRuleGraphsPos lastGraphsPos, LayoutState layoutState)
    pub fn isRealSpace(&self, lastGraphsPos: &LineRuleGraphsPos, layoutState: &LayoutState) -> bool {
      // java if (!TypoRun.isSpace(lastGraphsPos.getGraphs().runType)) return false;
      if !TypoRun::isSpace(lastGraphsPos.getGraphs().runType) { return false; }
      // java if (layoutState.isAllowTransForSpace()) { ... }
      if layoutState.isAllowTransForSpace() {
        let cp = layoutState.typoPara.getCP(lastGraphsPos.getGraphIndex());
        if cp > 0 {
          if let Some(document) = layoutState.document {
            let c = unsafe { (*document).charAt(cp) };
            return c == ' ' || c == '\u{3000}';
          }
        }
      }
      // java return true;
      true

    }

    // java private void rollbackIfCurLineEndsWithTab(boolean wordWrap, boolean kinsoku, LineRuleGraphsPos lastGraphsPos, LineRuleGraphsPos nextGraphsPos, KPList<Graphs> graphsList)
    pub fn rollbackIfCurLineEndsWithTab(&mut self, wordWrap: bool, kinsoku: bool, lastGraphsPos: &mut LineRuleGraphsPos, nextGraphsPos: &mut LineRuleGraphsPos, graphsList: &KPList<Graphs>) {
      // java this.mAdjusted = false;
      self.mAdjusted = false;
      // java this.mRollbackedAll = false;
      self.mRollbackedAll = false;
      // java if (lastGraphsPos.getGraphs().isTab() && nextGraphsPos.getGraphs().isTab())
      if lastGraphsPos.getGraphs().isTab() && nextGraphsPos.getGraphs().isTab() {
        // java nextGraphsPos.copy(lastGraphsPos);
        nextGraphsPos.copy(lastGraphsPos);
        // java boolean hasPrev = lastGraphsPos.prev(false, graphsList, this.mSubLineEnv);
        let mut hasPrev: bool = lastGraphsPos.prev(false, graphsList, self.super_.subLineEnv());
        // java this.mAdjusted = true;
        self.mAdjusted = true;
        // java if (!hasPrev)
        if !hasPrev {
          // java this.mRollbackedAll = true;
          self.mRollbackedAll = true;
        }
      }
    }

    // java private void doAlternativeRollback(boolean wordWrap, boolean kinsoku, AsiaTypography asiaTypoGraphy, LineRuleGraphsPos lastGraphsPos, LineRuleGraphsPos nextGraphsPos, TypoPara typoPara, KPList<Graphs> graphsList)
    pub fn doAlternativeRollback(&mut self, wordWrap: bool, kinsoku: bool, asiaTypoGraphy: &AsiaTypography, lastGraphsPos: &mut LineRuleGraphsPos, nextGraphsPos: &mut LineRuleGraphsPos, typoPara: &TypoPara, graphsList: &KPList<Graphs>) {
      // java int lastGraphIdxBak = lastGraphsPos.getGraphIndex();
      let mut lastGraphIdxBak: i32 = lastGraphsPos.getGraphIndex();
      // java Graphs lastGraphsBak = lastGraphsPos.getGraphs();
      let mut lastGraphsBak: Graphs = lastGraphsPos.getGraphs();
      // java int nextGraphIdxBak = nextGraphsPos.getGraphIndex();
      let mut nextGraphIdxBak: i32 = nextGraphsPos.getGraphIndex();
      // java Graphs nextGraphsBak = nextGraphsPos.getGraphs();
      let mut nextGraphsBak: Graphs = nextGraphsPos.getGraphs();
      // java boolean hasAdjusted = false;
      let mut hasAdjusted: bool = false;
      // java boolean hasRollbackedAll = false;
      let mut hasRollbackedAll: bool = false;
      // java SubLineEnv subLineEnv = this.mSubLineEnv;
      let subLineEnv = self.super_.subLineEnv();
      // java boolean hasPrev1 = nextGraphsPos.prev(true, graphsList, subLineEnv);
      let mut hasPrev1: bool = nextGraphsPos.prev(true, graphsList, subLineEnv);
      // java if (hasPrev1 && asiaTypoGraphy.isLineStartPunct(nextGraphsPos.getChar(typoPara)))
      if hasPrev1 && asiaTypoGraphy.isLineStartPunct(nextGraphsPos.getChar(typoPara)) {
        // java lastGraphsPos.copy(nextGraphsPos);
        lastGraphsPos.copy(nextGraphsPos);
        // java boolean hasPrev2 = lastGraphsPos.prev(false, graphsList, this.mSubLineEnv);
        let mut hasPrev2: bool = lastGraphsPos.prev(false, graphsList, self.super_.subLineEnv());
        // java if (!hasPrev2)
        if !hasPrev2 {
          // java hasRollbackedAll = true;
          hasRollbackedAll = true;
        }
        // java else if (kinsoku)
        else if kinsoku {
          // java this.rollbackIfNextLineStartsWithHeadPuncts(wordWrap, lastGraphsPos, nextGraphsPos, typoPara, graphsList);
          self.rollbackIfNextLineStartsWithHeadPuncts(wordWrap, lastGraphsPos, nextGraphsPos, typoPara, graphsList);
          // java hasAdjusted = this.mNextLineStartsWithHeadPunct;
          hasAdjusted = self.mNextLineStartsWithHeadPunct;
          // java hasRollbackedAll = this.mRollbackedAll;
          hasRollbackedAll = self.mRollbackedAll;
        }
      }
      // java this.mAdjusted = false;
      self.mAdjusted = false;
      // java this.mRollbackedAll = false;
      self.mRollbackedAll = false;
      // java if (!hasAdjusted && !hasRollbackedAll)
      if !hasAdjusted && !hasRollbackedAll {
        // java lastGraphsPos.setGraphsAndIndex(lastGraphsBak, lastGraphIdxBak);
        lastGraphsPos.setGraphsAndIndex(lastGraphsBak, lastGraphIdxBak);
        // java nextGraphsPos.setGraphsAndIndex(nextGraphsBak, nextGraphIdxBak);
        nextGraphsPos.setGraphsAndIndex(nextGraphsBak, nextGraphIdxBak);
      }
      // java else
      else {
        // java if (hasAdjusted)
        if hasAdjusted {
          // java this.mAdjusted = true;
          self.mAdjusted = true;
        }
        // java if (hasRollbackedAll)
        if hasRollbackedAll {
          // java this.mRollbackedAll = true;
          self.mRollbackedAll = true;
        }
      }
    }

    // java private boolean isRealAsianGraphsPos(AsiaTypography asiaTypoGraphy, LineRuleGraphsPos graphsPos, TypoPara typoPara)
    pub fn isRealAsianGraphsPos(&self, asiaTypoGraphy: &AsiaTypography, graphsPos: &LineRuleGraphsPos, typoPara: &TypoPara) -> bool {
      // java Graphs graphs = graphsPos.getGraphs();
      let graphs = graphsPos.getGraphs();
      // java if (2 != graphs.autoSpaceType) return false;
      if graphs.autoSpaceType != 2 { return false; }
      // java else if (TypoRun.isPunct(graphs.runType)) return false;
      if TypoRun::isPunct(graphs.runType) { return false; }
      // java return !asiaTypoGraphy.isLineStartOrLineEndPunct(graphsPos.getChar(typoPara));
      !asiaTypoGraphy.isLineStartOrLineEndPunct(graphsPos.getChar(typoPara))

    }

    // java private boolean canSkipRollback(AsiaTypography asiaTypoGraphy, LineRuleGraphsPos lastGraphsPos, LineRuleGraphsPos nextGraphsPos, TypoPara typoPara)
    pub fn canSkipRollback(&self, asiaTypoGraphy: &AsiaTypography, lastGraphsPos: &LineRuleGraphsPos, nextGraphsPos: &LineRuleGraphsPos, typoPara: &TypoPara) -> bool {
      // java return this.isRealAsianGraphsPos(asiaTypoGraphy, lastGraphsPos, typoPara) && this.isRealAsianGraphsPos(asiaTypoGraphy, nextGraphsPos, typoPara);
      self.isRealAsianGraphsPos(asiaTypoGraphy, lastGraphsPos, typoPara) && self.isRealAsianGraphsPos(asiaTypoGraphy, nextGraphsPos, typoPara)

    }

    // java private boolean rollbackIfHangingSpaces(LineRuleGraphsPos lastGraphsPos, KPList<Graphs> graphsList)
    pub fn rollbackIfHangingSpaces(&mut self, lastGraphsPos: &mut LineRuleGraphsPos, graphsList: &KPList<Graphs>) -> bool {
      // java SubLineEnv subLineEnv = this.mSubLineEnv;
      let subLineEnv = self.super_.subLineEnv();
      // java LineCompressEnv lineCompressEnv = subLineEnv.lineCompressEnv;
      let lineCompressEnv = subLineEnv.lineCompressEnv.as_ref().expect("lineCompressEnv");
      // java int excessSpaceIdx = lineCompressEnv.firstExcessSpaceGraphIndex;
      let excessSpaceIdx = lineCompressEnv.firstExcessSpaceGraphIndex;
      // java Graphs lastGraphsOfSubLine = subLineEnv.lastGraphs;
      let lastGraphsOfSubLine = subLineEnv.lastGraphs.as_ref().expect("lastGraphs").clone();
      // java lastGraphsPos.setGraphsAndIndex(lastGraphsOfSubLine, lastGraphsOfSubLine.getEndGraphIndex());
      lastGraphsPos.setGraphsAndIndex(lastGraphsOfSubLine.clone(), lastGraphsOfSubLine.getEndGraphIndex());
      // java do { ... } while (hasPrev);
      loop {
        let graphs = lastGraphsPos.getGraphs();
        if graphs.graphIndex <= excessSpaceIdx && excessSpaceIdx <= graphs.getEndGraphIndex() {
          lastGraphsPos.setGraphsAndIndex(graphs, excessSpaceIdx);
          let mut hasPrev = lastGraphsPos.prev(false, graphsList, subLineEnv);
          if hasPrev {
            if lineCompressEnv.firstExcessPunctGraphIndex == lastGraphsPos.getGraphIndex() {
              hasPrev = lastGraphsPos.prev(false, graphsList, subLineEnv);
              if !hasPrev { return false; }
            }
            return true;
          }
        } else {
          lastGraphsPos.setGraphsAndIndex(graphs.clone(), graphs.graphIndex);
          if lastGraphsPos.prev(false, graphsList, subLineEnv) { continue; }
        }
        return false;
      }

    }

}

impl std::ops::Deref for SubLineLayouterBase {
    type Target = LineCompressTool;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SubLineLayouterBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
