// Java: cn.wps.moffice.writer.layout.base.line.align.AlignRightHandler
#![allow(non_snake_case)]

use super::AlignBaseHandler::AlignBaseHandler;
use super::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public class AlignRightHandler extends AlignBaseHandler
pub struct AlignRightHandler { pub super_: AlignBaseHandler }

impl AlignRightHandler {
    pub fn AlignRightHandler() -> Self {
        Self { super_: AlignBaseHandler {
            mSubLineEnv: std::ptr::null_mut(), mLayoutState: std::ptr::null_mut(),
            mCurOffset: 0, mMaxValidGraphIndex: 0, mAlignLeftOffset: 0,
            mAlignRightOffset: 0, mFindValidGraphs: false, mParaIsRTL: false,
            mNeedToCompressGraphs: false, mCompressedSpaceSum: 0,
        } }
    }

    // Java: public void onAddRun(Graphs graphs)
    pub fn onAddRun(&mut self, graphs: &mut Graphs) {
        self.super_.onAddRun(graphs);
        let state = unsafe { &*self.super_.mLayoutState };
        if !state.paraEnv.wordWrap && TypoRun::isSpace(graphs.runType) {
            unsafe { (*self.super_.mSubLineEnv).alignRightGraphs = Some(graphs as *mut Graphs); }
        }
    }

    // Java: public void onAddBidiRun(Graphs graphs, boolean isRTL)
    pub fn onAddBidiRun(&mut self, graphs: &mut Graphs, isRTL: bool) {
        self.super_.onAddBidiRun(graphs, isRTL);
        let state = unsafe { &*self.super_.mLayoutState };
        if !state.paraEnv.wordWrap && TypoRun::isSpace(graphs.runType) {
            self.super_.mAlignRightOffset = self.super_.mCurOffset;
        }
    }

    // Java: protected boolean shouldAdjustSecBreakLine(Graphs run)
    pub fn shouldAdjustSecBreakLine(&self, run: &Graphs) -> bool {
        let sub = unsafe { &*self.super_.mSubLineEnv };
        // The remaining firstGraphs value holder identifies its graph-list node
        // by graphIndex until SubLineEnv's other references are migrated.
        sub.firstGraphs.as_ref().map(|first| first.graphIndex) == Some(run.graphIndex)
            && self.super_.shouldAdjustSecBreakLine(run)
    }

    fn adjust_section_break(&mut self, line: &mut TypoRectCoreImpl) {
        let (left, right, last) = {
            let sub = unsafe { &*self.super_.mSubLineEnv };
            (sub.layoutLeft, sub.layoutRight, sub.lastGraphs.clone())
        };
        if let Some(run) = last.as_ref() {
            if self.shouldAdjustSecBreakLine(run) {
                line.setLeft(left);
                let width = (right - left).max(0);
                unsafe { (*self.super_.mSubLineEnv).lastGraphs.as_mut().unwrap().mWidth = width; }
                unsafe { (*self.super_.mLayoutState).typoPara.putWidths(run.graphIndex, 1, &[width]); }
            }
            let last_right = unsafe { (*self.super_.mSubLineEnv).lastGraphs.as_ref().unwrap().getRight() };
            line.setWidth(last_right - self.super_.mCompressedSpaceSum);
        }
    }

    // Java: public void adjustLine(ITypoRect line)
    pub fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) {
        let sub = unsafe { &*self.super_.mSubLineEnv };
        if sub.layoutLeft < sub.layoutRight {
            let first_width = sub.firstGraphs.as_ref().map(Graphs::width).unwrap_or(0);
            if first_width > sub.layoutRight - sub.layoutLeft {
                line.setLeft(sub.layoutLeft);
            } else if let Some(graph) = sub.alignRightGraphs {
                line.setLeft(sub.layoutRight - unsafe { (*graph).getRight() } + self.super_.mCompressedSpaceSum);
            } else {
                line.setLeft(sub.layoutRight + self.super_.mCompressedSpaceSum);
            }
            self.adjust_section_break(line);
        }
    }

    // Java: public void adjustBidiLine(ITypoRect line)
    pub fn adjustBidiLine(&mut self, line: &mut TypoRectCoreImpl) {
        let sub = unsafe { &*self.super_.mSubLineEnv };
        if sub.layoutLeft < sub.layoutRight {
            if self.super_.mFindValidGraphs {
                line.setLeft(sub.layoutRight - self.super_.mAlignRightOffset
                    + self.super_.mAlignLeftOffset + self.super_.mCompressedSpaceSum);
            } else {
                line.setLeft(sub.layoutRight + self.super_.mCompressedSpaceSum);
            }
            self.adjust_section_break(line);
        }
    }

    // Java: public boolean calcLineLeftAndRight(ITypoRect line)
    pub fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool {
        self.adjustLine(line);
        true
    }
}

impl IHAlignHandler for AlignRightHandler {
    fn setSubLineEnv(&mut self, sub: &mut SubLineEnv, state: &mut LayoutState) { self.super_.setSubLineEnv(sub, state); }
    fn onAddRun(&mut self, run: &mut Graphs) { self.onAddRun(run); }
    fn onRemoveRun(&mut self, run: &mut Graphs) { self.super_.onRemoveRun(run); }
    fn adjustRun(&mut self, run: &mut Graphs) { self.super_.adjustRun(run); }
    fn onAddBidiRun(&mut self, run: &mut Graphs, rtl: bool) { self.onAddBidiRun(run, rtl); }
    fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) { self.adjustLine(line); }
    fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool { self.calcLineLeftAndRight(line) }
    fn prepareRecalcForBidi(&mut self) { self.super_.prepareRecalcForBidi(); }
    fn adjustBidiLine(&mut self, line: &mut TypoRectCoreImpl) { self.adjustBidiLine(line); }
    fn dispose(&mut self) { self.super_.dispose(); }
}
