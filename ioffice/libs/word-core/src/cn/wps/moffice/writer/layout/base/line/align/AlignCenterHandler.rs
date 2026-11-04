// Java: cn.wps.moffice.writer.layout.base.line.align.AlignCenterHandler
// rust: ★wr-29 06 —— 真实移植（Java 63 行）：jc=center 行盒调整
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignBaseHandler::AlignBaseHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public class AlignCenterHandler extends AlignBaseHandler
pub struct AlignCenterHandler {
    // java AlignBaseHandler super_
    pub super_: AlignBaseHandler,
}

impl AlignCenterHandler {
    pub fn AlignCenterHandler() -> AlignCenterHandler {
        AlignCenterHandler {
            super_: AlignBaseHandler {
                mSubLineEnv: std::ptr::null_mut(), mLayoutState: std::ptr::null_mut(), mCurOffset: 0,
                mMaxValidGraphIndex: 0, mAlignLeftOffset: 0, mAlignRightOffset: 0, mFindValidGraphs: false,
                mParaIsRTL: false, mNeedToCompressGraphs: false, mCompressedSpaceSum: 0,
            },
        }
    }

    // java public void onAddRun(Graphs graphs)
    pub fn onAddRun_center(&mut self, graphs: &mut Graphs) {
        // java super.onAddRun(graphs);
        self.super_.onAddRun(graphs);
        // java if (!this.mLayoutState.paraEnv.wordWrap && TypoRun.isSpace(graphs.runType))
        let word_wrap: bool = unsafe { (*self.super_.mLayoutState).paraEnv.wordWrap };
        let is_space: bool = TypoRun::isSpace(graphs.runType);
        if !word_wrap && is_space {
            // java this.mSubLineEnv.alignRightGraphs = graphs;
            unsafe { (*self.super_.mSubLineEnv).alignRightGraphs = Some(graphs as *mut Graphs); }
        }
    }

    // java public void onAddBidiRun(Graphs graphs, boolean isRTL)
    pub fn onAddBidiRun_center(&mut self, graphs: &mut Graphs, isRTL: bool) {
        // java super.onAddBidiRun(graphs, isRTL);
        self.super_.onAddBidiRun(graphs, isRTL);
        // java if (!this.mLayoutState.paraEnv.wordWrap && TypoRun.isSpace(graphs.runType))
        if !unsafe { (*self.super_.mLayoutState).paraEnv.wordWrap } && TypoRun::isSpace(graphs.runType) {
            // java this.mAlignRightOffset = this.mCurOffset;
            self.super_.mAlignRightOffset = self.super_.mCurOffset;
        }
    }

    // java public void adjustLine(ITypoRect line)
    pub fn adjustLine_center(&mut self, line: &mut TypoRectCoreImpl) {
        // java if (this.mSubLineEnv.layoutLeft < this.mSubLineEnv.layoutRight)
        let sub = unsafe { &*self.super_.mSubLineEnv };
        if sub.layoutLeft < sub.layoutRight {
            // java Graphs firstRun = this.mSubLineEnv.firstGraphs;
            let firstRun: Option<Graphs> = sub.firstGraphs.clone();
            // java if (firstRun.width() > this.mSubLineEnv.layoutRight - this.mSubLineEnv.layoutLeft)
            let first_width: i32 = firstRun.as_ref().map(|g| g.mWidth).unwrap_or(0);
            if first_width > sub.layoutRight - sub.layoutLeft {
                // java line.setLeft(this.mSubLineEnv.layoutLeft);
                line.setLeft(sub.layoutLeft);
            } else if sub.alignRightGraphs.is_none() {
                // java else if (this.mSubLineEnv.alignRightGraphs == null)
                // java line.setLeft((this.mSubLineEnv.layoutLeft + this.mSubLineEnv.layoutRight) / 2);
                line.setLeft((sub.layoutLeft + sub.layoutRight) / 2);
            } else {
                // java line.setLeft((this.mSubLineEnv.layoutLeft + this.mSubLineEnv.layoutRight - (this.mSubLineEnv.alignRightGraphs.getRight() - this.mCompressedSpaceSum)) / 2);
                let arRight: i32 = unsafe { (*self.super_.mSubLineEnv).alignRightGraphs.map(|g| (*g).getRight()).unwrap_or(0) };
                let right_val: i32 = arRight - self.super_.mCompressedSpaceSum;
                line.setLeft((sub.layoutLeft + sub.layoutRight - right_val) / 2);
            }
            // java Graphs lastRun = this.mSubLineEnv.lastGraphs;
            let lastRun: Option<Graphs> = unsafe { (*self.super_.mSubLineEnv).lastGraphs.clone() };
            // java if (this.shouldAdjustSecBreakLine(lastRun))
            if let Some(lastRun) = lastRun.as_ref() {
                if self.super_.shouldAdjustSecBreakLine(lastRun) {
                    // java int width = 0;
                    let mut width: i32 = 0;
                    // java if (lastRun == this.mSubLineEnv.firstGraphs)
                    if Some(lastRun.graphIndex) == unsafe { (*self.super_.mSubLineEnv).firstGraphs.as_ref().map(|g| g.graphIndex) } {
                        // java width = Math.max(0, this.mSubLineEnv.layoutRight - this.mSubLineEnv.layoutLeft);
                        // java line.setLeft(this.mSubLineEnv.layoutLeft);
                        width = (sub.layoutRight - sub.layoutLeft).max(0);
                        line.setLeft(sub.layoutLeft);
                    } else {
                        // java width = Math.max(0, (this.mSubLineEnv.layoutRight - this.mSubLineEnv.layoutLeft - lastRun.getLeft()) / 2);
                        width = ((sub.layoutRight - sub.layoutLeft - lastRun.getLeft()) / 2).max(0);
                    }
                    // java lastRun.mWidth = width;
                    unsafe { (*self.super_.mSubLineEnv).lastGraphs.as_mut().unwrap().mWidth = width; }
                    // java this.mLayoutState.typoPara.putWidth(lastRun.graphIndex, width);
                    unsafe { (*self.super_.mLayoutState).typoPara.putWidths(lastRun.graphIndex, 1, &[width]); }
                }
            }
            // java line.setWidth(lastRun.getRight() - this.mCompressedSpaceSum);
            let lastRight: i32 = unsafe { (*self.super_.mSubLineEnv).lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0) };
            line.setWidth(lastRight - self.super_.mCompressedSpaceSum);
        }
    }

    // java public void adjustBidiLine(ITypoRect line)
    pub fn adjustBidiLine_center(&mut self, line: &mut TypoRectCoreImpl) {
      // java if (this.mSubLineEnv.layoutLeft < this.mSubLineEnv.layoutRight)
      let sub = unsafe { &*self.super_.mSubLineEnv };
      if sub.layoutLeft < sub.layoutRight {
        // java if (this.mFindValidGraphs)
        if self.super_.mFindValidGraphs {
          // java line.setLeft((this.mSubLineEnv.layoutLeft + this.mSubLineEnv.layoutRight - this.mAlignLeftOffset - this.mAlignRightOffset + this.mCompressedSpaceSum) / 2);
          line.setLeft((sub.layoutLeft + sub.layoutRight - self.super_.mAlignLeftOffset - self.super_.mAlignRightOffset + self.super_.mCompressedSpaceSum) / 2);
        } else {
          // java line.setLeft((this.mSubLineEnv.layoutLeft + this.mSubLineEnv.layoutRight) / 2);
          line.setLeft((sub.layoutLeft + sub.layoutRight) / 2);
        }
        // java Graphs lastRun = this.mSubLineEnv.lastGraphs;
        let lastRun: Option<Graphs> = unsafe { (*self.super_.mSubLineEnv).lastGraphs.clone() };
        // java if (this.shouldAdjustSecBreakLine(lastRun))
        if let Some(lastRun) = lastRun.as_ref() {
          if self.super_.shouldAdjustSecBreakLine(lastRun) {
            // java int width = 0;
            let mut width: i32 = 0;
            // java if (lastRun == this.mSubLineEnv.firstGraphs)
            if Some(lastRun.graphIndex) == unsafe { (*self.super_.mSubLineEnv).firstGraphs.as_ref().map(|g| g.graphIndex) } {
              // java width = Math.max(0, this.mSubLineEnv.layoutRight - this.mSubLineEnv.layoutLeft);
              // java line.setLeft(this.mSubLineEnv.layoutLeft);
              width = (sub.layoutRight - sub.layoutLeft).max(0);
              line.setLeft(sub.layoutLeft);
            } else {
              // java width = Math.max(0, (this.mSubLineEnv.layoutRight - this.mSubLineEnv.layoutLeft - lastRun.getLeft() + this.mAlignLeftOffset) / 2);
              width = ((sub.layoutRight - sub.layoutLeft - lastRun.getLeft() + self.super_.mAlignLeftOffset) / 2).max(0);
            }
            // java lastRun.mWidth = width;
            unsafe { (*self.super_.mSubLineEnv).lastGraphs.as_mut().unwrap().mWidth = width; }
            // java this.mLayoutState.typoPara.putWidth(lastRun.graphIndex, width);
            unsafe { (*self.super_.mLayoutState).typoPara.putWidths(lastRun.graphIndex, 1, &[width]); }
          }
        }
        // java line.setWidth(lastRun.getRight() - this.mCompressedSpaceSum);
        let lastRight: i32 = unsafe { (*self.super_.mSubLineEnv).lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0) };
        line.setWidth(lastRight - self.super_.mCompressedSpaceSum);
      }
    }

    // java public boolean calcLineLeftAndRight(ITypoRect line)
    pub fn calcLineLeftAndRight_center(&mut self, line: &mut TypoRectCoreImpl) -> bool {
        // java this.adjustLine(line); return true;
        self.adjustLine_center(line);
        true
    }
}

// java implements IHAlignHandler
impl IHAlignHandler for AlignCenterHandler {
    fn setSubLineEnv(&mut self, var1: &mut SubLineEnv, var2: &mut LayoutState) {
        self.super_.setSubLineEnv(var1, var2);
    }
    fn onAddRun(&mut self, graphs: &mut Graphs) { self.onAddRun_center(graphs); }
    fn onRemoveRun(&mut self, var1: &mut Graphs) { self.super_.onRemoveRun(var1); }
    fn adjustRun(&mut self, var1: &mut Graphs) { self.super_.adjustRun(var1); }
    fn onAddBidiRun(&mut self, graphs: &mut Graphs, isRTL: bool) { self.onAddBidiRun_center(graphs, isRTL); }
    fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) { self.adjustLine_center(line); }
    fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool { self.calcLineLeftAndRight_center(line) }
    fn prepareRecalcForBidi(&mut self) { self.super_.prepareRecalcForBidi(); }
    fn adjustBidiLine(&mut self, line: &mut TypoRectCoreImpl) { self.adjustBidiLine_center(line); }
    fn dispose(&mut self) { self.super_.dispose(); }
}
