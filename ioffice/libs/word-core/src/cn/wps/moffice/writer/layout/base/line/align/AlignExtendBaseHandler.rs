// Java: cn.wps.moffice.writer.layout.base.line.align.AlignExtendBaseHandler
#![allow(non_snake_case, dead_code)]

use super::AlignBaseHandler::AlignBaseHandler;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public abstract class AlignExtendBaseHandler extends AlignBaseHandler
pub struct AlignExtendBaseHandler {
    pub super_: AlignBaseHandler,
    // Java: int mCurRunPos; float mSpacePerChar; float mRoundError;
    pub mCurRunPos: i32,
    pub mSpacePerChar: f32,
    pub mRoundError: f32,
    // Java: boolean mNeedExtend; int mExtendWidth;
    pub mNeedExtend: bool,
    pub mExtendWidth: i32,
    // Java: Graphs mFirstGraphForExtends; Graphs mLastGraphForExtends;
    pub mFirstGraphForExtends: Option<*mut Graphs>,
    pub mLastGraphForExtends: Option<*mut Graphs>,
    // Java: boolean mStartExtend;
    pub mStartExtend: bool,
}

impl AlignExtendBaseHandler {
    pub fn AlignExtendBaseHandler() -> Self {
        Self {
            super_: AlignBaseHandler {
                mSubLineEnv: std::ptr::null_mut(), mLayoutState: std::ptr::null_mut(),
                mCurOffset: 0, mMaxValidGraphIndex: 0, mAlignLeftOffset: 0,
                mAlignRightOffset: 0, mFindValidGraphs: false, mParaIsRTL: false,
                mNeedToCompressGraphs: false, mCompressedSpaceSum: 0,
            },
            mCurRunPos: 0, mSpacePerChar: 0.0, mRoundError: 0.0,
            mNeedExtend: false, mExtendWidth: 0, mFirstGraphForExtends: None,
            mLastGraphForExtends: None, mStartExtend: false,
        }
    }

    // Java: public void setSubLineEnv(SubLineEnv subLineEnv, LayoutState layoutState)
    pub fn setSubLineEnv(&mut self, subLineEnv: &mut SubLineEnv, layoutState: &mut LayoutState) {
        self.super_.setSubLineEnv(subLineEnv, layoutState);
        self.mCurRunPos = 0;
        self.mSpacePerChar = 0.0;
        self.mRoundError = 0.0;
        self.mFirstGraphForExtends = None;
        self.mLastGraphForExtends = None;
        self.mExtendWidth = 0;
        self.mNeedExtend = false;
    }

    // Java: final void resetCount()
    pub fn resetCount(&mut self) {
        let sub = unsafe { &mut *self.super_.mSubLineEnv };
        sub.charsCount = 0;
        sub.spaceCount = 0;
        sub.firstGraphForExtends = None;
        sub.lastGraphForExtends = None;
    }

    // Java: final void resetCountForRecalc()
    pub fn resetCountForRecalc(&mut self) {
        self.resetCount();
        unsafe { (*self.super_.mSubLineEnv).alignRightGraphs = None; }
    }

    // Java: protected void extendSpace(Graphs run, boolean ignoreLastChar)
    pub fn extendSpace(&mut self, run: &mut Graphs, ignoreLastChar: bool) {
        let extendLen = run.graphLength - i32::from(ignoreLastChar);
        if extendLen > 0 {
            // Java float-to-int truncates toward zero, including negative extension.
            let per = (self.mSpacePerChar + self.mRoundError / extendLen as f32 + 0.5) as i32;
            let extended = self.mSpacePerChar * extendLen as f32 + self.mRoundError;
            let rounded = (extended + 0.5) as i32;
            self.mRoundError = extended - rounded as f32;
            unsafe { (*self.super_.mLayoutState).extendWidths(run, per, run.graphIndex, extendLen); }
            run.mWidth += rounded;
        }
    }

    // Java: public void prepareRecalcForBidi()
    pub fn prepareRecalcForBidi(&mut self) {
        self.super_.prepareRecalcForBidi();
        self.resetCountForRecalc();
    }
}
