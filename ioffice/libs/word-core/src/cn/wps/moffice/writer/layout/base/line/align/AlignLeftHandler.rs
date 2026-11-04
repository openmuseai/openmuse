// Java: cn.wps.moffice.writer.layout.base.line.align.AlignLeftHandler
// rust: ★wr-29 05 —— 真实移植（Java 39 行）：jc=left 行盒调整
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public class AlignLeftHandler extends AlignBaseHandler
pub struct AlignLeftHandler {
    // java AlignBaseHandler super_
    pub super_: super::AlignBaseHandler::AlignBaseHandler,
}

impl AlignLeftHandler {
    pub fn AlignLeftHandler() -> AlignLeftHandler {
        AlignLeftHandler { super_: super::AlignBaseHandler::AlignBaseHandler {
            mSubLineEnv: std::ptr::null_mut(), mLayoutState: std::ptr::null_mut(), mCurOffset: 0,
            mMaxValidGraphIndex: 0, mAlignLeftOffset: 0, mAlignRightOffset: 0, mFindValidGraphs: false,
            mParaIsRTL: false, mNeedToCompressGraphs: false, mCompressedSpaceSum: 0,
        } }
    }

    // java public void adjustLine(ITypoRect line)
    pub fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) {
      // java if (this.mSubLineEnv.layoutLeft < this.mSubLineEnv.layoutRight)
      let sub = unsafe { &*self.super_.mSubLineEnv };
      if sub.layoutLeft < sub.layoutRight {
        // java line.setLeft(this.mSubLineEnv.layoutLeft);
        line.setLeft(sub.layoutLeft);
        // Java keeps lastGraphs as the graph-list node. Compression mutates that
        // node after SubLineEnv.addGraphs; its value snapshot is then stale.
        let lastRun = sub.lastGraphNode;
        // java if (this.shouldAdjustSecBreakLine(lastRun))
        if let Some(lastRun) = lastRun {
          let lastRun = unsafe { &mut *lastRun };
          if self.super_.shouldAdjustSecBreakLine(lastRun) {
            // java int width = Math.max(0, layoutRight - layoutLeft - lastRun.getLeft());
            let width: i32 = (sub.layoutRight - sub.layoutLeft - lastRun.getLeft()).max(0);
            // java lastRun.mWidth = width; this.mLayoutState.typoPara.putWidth(lastRun.graphIndex, width);
            lastRun.mWidth = width;
            let st = unsafe { &mut *self.super_.mLayoutState };
            st.typoPara.putWidths(lastRun.graphIndex, 1, &[width]);
          }
        }
        // java line.setWidth(lastRun.getRight() - this.mCompressedSpaceSum);
        let lastRight: i32 = lastRun.map(|g| unsafe { (*g).getRight() }).unwrap_or(0);
        line.setWidth(lastRight - self.super_.mCompressedSpaceSum);
      }
    }

    // java public boolean calcLineLeftAndRight(ITypoRect line)
    pub fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool {
      // java this.adjustLine(line); return true;
      self.adjustLine(line);
      true
    }
}

// java public class AlignLeftHandler extends AlignBaseHandler implements IHAlignHandler
impl crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler for AlignLeftHandler {
    fn setSubLineEnv(&mut self, var1: &mut SubLineEnv, var2: &mut LayoutState) {
        self.super_.setSubLineEnv(var1, var2);
    }
    fn onAddRun(&mut self, var1: &mut Graphs) {
        self.super_.onAddRun(var1);
    }
    fn onRemoveRun(&mut self, var1: &mut Graphs) {
        self.super_.onRemoveRun(var1);
    }
    fn adjustRun(&mut self, var1: &mut Graphs) {
        self.super_.adjustRun(var1);
    }
    fn adjustLine(&mut self, var1: &mut TypoRectCoreImpl) {
        self.adjustLine(var1);
    }
    fn calcLineLeftAndRight(&mut self, var1: &mut TypoRectCoreImpl) -> bool {
        self.calcLineLeftAndRight(var1)
    }
    fn prepareRecalcForBidi(&mut self) {
        self.super_.prepareRecalcForBidi();
    }
    fn onAddBidiRun(&mut self, var1: &mut Graphs, var2: bool) {
        self.super_.onAddBidiRun(var1, var2);
    }
    fn adjustBidiLine(&mut self, var1: &mut TypoRectCoreImpl) {
        // rust: 与非 RTL 同（Java AlignLeftHandler.adjustBidiLine）
        self.adjustLine(var1);
    }
    fn dispose(&mut self) {
        self.super_.dispose();
    }
}
