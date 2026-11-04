// Java: cn.wps.moffice.writer.layout.base.line.align.AlignBaseHandler
// rust: ★wr-29 05 —— 真实移植（Java 143 行）
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public abstract class AlignBaseHandler implements IHAlignHandler
pub struct AlignBaseHandler {
    // java protected SubLineEnv mSubLineEnv;
    pub(crate) mSubLineEnv: *mut SubLineEnv,
    // java protected LayoutState mLayoutState;
    pub(crate) mLayoutState: *mut LayoutState,
    // java int mCurOffset;
    pub(crate) mCurOffset: i32,
    // java int mMaxValidGraphIndex;
    pub(crate) mMaxValidGraphIndex: i32,
    // java protected int mAlignLeftOffset;
    pub(crate) mAlignLeftOffset: i32,
    // java protected int mAlignRightOffset;
    pub(crate) mAlignRightOffset: i32,
    // java protected boolean mFindValidGraphs;
    pub(crate) mFindValidGraphs: bool,
    // java boolean mParaIsRTL;
    pub(crate) mParaIsRTL: bool,
    // java boolean mNeedToCompressGraphs;
    pub(crate) mNeedToCompressGraphs: bool,
    // java int mCompressedSpaceSum;
    pub(crate) mCompressedSpaceSum: i32,
}

impl AlignBaseHandler {
    // java public void setSubLineEnv(SubLineEnv subLineEnv, LayoutState layoutState)
    pub fn setSubLineEnv(&mut self, subLineEnv: &mut SubLineEnv, layoutState: &mut LayoutState) {
      // java this.mSubLineEnv = subLineEnv;
      self.mSubLineEnv = subLineEnv as *mut SubLineEnv;
      // java this.mParaIsRTL = layoutState.paraEnv.isRTL;
      self.mParaIsRTL = layoutState.paraEnv.isRTL;
      // java this.mCurOffset = subLineEnv.hasGraphs() ? subLineEnv.firstGraphs.getLeft() : 0;
      self.mCurOffset = if subLineEnv.hasGraphs() {
        subLineEnv.firstGraphs.as_ref().map(|g| g.getLeft()).unwrap_or(0)
      } else { 0 };
      // java this.mNeedToCompressGraphs = null != subLineEnv.lastGraphs && !subLineEnv.lineCompressEnv.isFirstCompress();
      self.mNeedToCompressGraphs = subLineEnv.lastGraphs.is_some()
        && subLineEnv.lineCompressEnv.as_ref().map(|l| !l.isFirstCompress()).unwrap_or(false);
      // java this.mCompressedSpaceSum = this.mNeedToCompressGraphs ? subLineEnv.lineCompressEnv.getFinalCompress() : 0;
      self.mCompressedSpaceSum = if self.mNeedToCompressGraphs {
        subLineEnv.lineCompressEnv.as_ref().map(|l| l.getFinalCompress()).unwrap_or(0)
      } else { 0 };
      // java this.mLayoutState = layoutState;
      self.mLayoutState = layoutState as *mut LayoutState;
    }

    // java public void onAddRun(Graphs graphs)
    pub fn onAddRun(&mut self, graphs: &mut Graphs) {
      // java switch (graphs.runType)
      match graphs.runType {
        // java case 2: case 17: case 3: case 4: case 5: case 10: case 11: case 12: case 13: case 14: break;
        2 | 3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 | 17 => {}
        // java case 6: case 7: case 8: case 9: case 15: case 16: default: this.mSubLineEnv.alignRightGraphs = graphs;
        _ => {
          unsafe { (*self.mSubLineEnv).alignRightGraphs = Some(graphs as *const Graphs as *mut Graphs); }
        }
      }
    }

    // java public void onRemoveRun(Graphs run)
    pub fn onRemoveRun(&mut self, run: &mut Graphs) {
      // java KPList<Graphs> graphList = this.mLayoutState.getGraphList();
      // java this.mSubLineEnv.alignRightGraphs = null;
      unsafe { (*self.mSubLineEnv).alignRightGraphs = None; }
      // java if (run != graphList.firstElement())
      let first: *mut Graphs = unsafe { (*self.mLayoutState).getGraphList().firstElement() };
      if run as *mut Graphs != first {
        // java for(prevGraphs = graphList.getPrev(run); null != prevGraphs; prevGraphs = getPrev(prevGraphs))
        let mut prev: *mut Graphs = unsafe { (*self.mLayoutState).getGraphList().getPrev(run as *mut Graphs) };
        while !prev.is_null() {
          // java switch (prevGraphs.runType) { case 2: case 17: default: alignRightGraphs = prevGraphs; return; }
          match unsafe { (*prev).runType } {
            _ => {
              unsafe { (*self.mSubLineEnv).alignRightGraphs = Some(prev); }
              return;
            }
          }
        }
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mSubLineEnv = null; this.mLayoutState = null;
      self.mSubLineEnv = std::ptr::null_mut();
      self.mLayoutState = std::ptr::null_mut();
    }

    // java public void prepareRecalcForBidi()
    pub fn prepareRecalcForBidi(&mut self) {
      // java this.mCurOffset = 0;
      self.mCurOffset = 0;
      // java this.mMaxValidGraphIndex = this.mSubLineEnv.lastGraphs.getEndGraphIndex();
      self.mMaxValidGraphIndex = unsafe { (*self.mSubLineEnv).lastGraphs.as_ref().map(|g| g.getEndGraphIndex()).unwrap_or(0) };
      // java Graphs alignRightGraphs = this.mSubLineEnv.alignRightGraphs;
      // java if (null != alignRightGraphs) { this.mMaxValidGraphIndex = alignRightGraphs.getEndGraphIndex(); }
      if let Some(alignRightGraphs) = unsafe { (*self.mSubLineEnv).alignRightGraphs.as_ref() } {
        self.mMaxValidGraphIndex = unsafe { (**alignRightGraphs).getEndGraphIndex() };
      }
      // java this.mAlignLeftOffset = 0; this.mAlignRightOffset = 0; this.mFindValidGraphs = false;
      self.mAlignLeftOffset = 0;
      self.mAlignRightOffset = 0;
      self.mFindValidGraphs = false;
    }

    // java public void onAddBidiRun(Graphs graphs, boolean isRTL)
    pub fn onAddBidiRun(&mut self, graphs: &mut Graphs, isRTL: bool) {
      // java this.mCurOffset += graphs.width();
      self.mCurOffset += graphs.width();
      // java int runRight = this.mCurOffset;
      let runRight: i32 = self.mCurOffset;
      // java switch (graphs.runType)
      match graphs.runType {
        // java case 2: case 17: if (this.mParaIsRTL != isRTL || this.mParaIsRTL) { this.mAlignRightOffset = runRight; }
        2 | 17 => {
          if self.mParaIsRTL != isRTL || self.mParaIsRTL {
            self.mAlignRightOffset = runRight;
          }
        }
        // java case 3: case 4: case 5: case 10..14: break;
        3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => {}
        // java default: this.mFindValidGraphs = true; alignRightGraphs = graphs; mAlignRightOffset = runRight;
        _ => {
          self.mFindValidGraphs = true;
          unsafe { (*self.mSubLineEnv).alignRightGraphs = Some(graphs as *const Graphs as *mut Graphs); }
          self.mAlignRightOffset = runRight;
        }
      }
    }

    // java protected boolean shouldAdjustSecBreakLine(Graphs run)
    pub fn shouldAdjustSecBreakLine(&self, run: &Graphs) -> bool {
      // java return this.mLayoutState.needCalcSectionBreakWidth && TypoRun.isTypoPageBreak(run.runType);
      !self.mLayoutState.is_null()
        && unsafe { (*self.mLayoutState).needCalcSectionBreakWidth }
        && crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun::isTypoPageBreak(run.runType)
    }

    // java public void adjustRun(Graphs run)
    pub fn adjustRun(&mut self, run: &mut Graphs) {
      // java run.offsetXTo(this.mCurOffset);
      run.offsetXTo(self.mCurOffset);
      // java this.mCurOffset = run.getRight();
      self.mCurOffset = run.getRight();
    }
}
