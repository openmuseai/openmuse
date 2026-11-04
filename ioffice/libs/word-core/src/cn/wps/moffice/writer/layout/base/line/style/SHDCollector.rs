// Java: cn.wps.moffice.writer.layout.base.line.style.SHDCollector
#![allow(dead_code, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoSHDs::TypoSHDs;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;

// java public class SHDCollector extends BaseCollector
pub struct SHDCollector {
    mEnable: bool,
    mLayoutLeft: i32,
    mLayoutRight: i32,
    mFirstTrailSpace: *mut Graphs,
    mFirstTrailSpaceLeft: i32,
    mCollectTrailSpace: bool,
    mForceInLineBoundary: bool,
    mLayoutState: *mut LayoutState,
    mSHDs: TypoSHDs,
    mTextLine: *mut TypoTextLine,
    mLastSHDIndex: i32,
}

impl Default for SHDCollector {
    fn default() -> Self { Self::new() }
}

impl SHDCollector {
    pub fn new() -> Self {
        Self {
            mEnable: false,
            mLayoutLeft: 0,
            mLayoutRight: 0,
            mFirstTrailSpace: std::ptr::null_mut(),
            mFirstTrailSpaceLeft: 0,
            mCollectTrailSpace: false,
            mForceInLineBoundary: false,
            mLayoutState: std::ptr::null_mut(),
            mSHDs: TypoSHDs::newInstance(),
            mTextLine: std::ptr::null_mut(),
            mLastSHDIndex: -1,
        }
    }

    // java public void noForceInLineBoundary()
    pub fn noForceInLineBoundary(&mut self) {
        self.mForceInLineBoundary = false;
    }

    pub fn setCollectTrailSpace(&mut self, val: bool) {
        self.mFirstTrailSpace = std::ptr::null_mut();
        self.mCollectTrailSpace = val;
    }

    pub fn setLineBoundary(&mut self, left: i32, right: i32) {
        self.mForceInLineBoundary = true;
        self.mLayoutLeft = left;
        self.mLayoutRight = right;
    }

    pub fn init(&mut self, line: &mut TypoTextLine, layoutState: &mut LayoutState) {
        self.mSHDs.clearCache(line.mSnapshot);
        self.mLayoutState = layoutState as *mut LayoutState;
        self.mEnable = true;
        self.mTextLine = line as *mut TypoTextLine;
        self.mForceInLineBoundary = false;
        self.mLastSHDIndex = -1;
    }

    pub fn addRun(&mut self, run: &mut Graphs, mut runLeft: i32, mut runRight: i32, _revisionAuthor: i32, _hyperlinkId: i32) {
        if !self.mEnable { return; }
        let run_ptr = run as *mut Graphs;
        match run.runType {
            3 | 4 | 8 | 9 => {
                self.mLastSHDIndex = -1;
                return;
            }
            5 | 10 | 11 | 12 | 13 | 14 => {
                if !self.mLayoutState.is_null() && unsafe { (*self.mLayoutState).needCalcSectionBreakWidth } {
                    return;
                }
            }
            20 | 21 | 22 | 23 | 24 | 25 => {
                self.mFirstTrailSpace = std::ptr::null_mut();
                if run.isAutoNum() {
                    self.mLastSHDIndex = -1;
                    return;
                }
                if self.mFirstTrailSpace.is_null() {
                    self.mFirstTrailSpace = run_ptr;
                    self.mFirstTrailSpaceLeft = runLeft;
                }
            }
            2 | 17 => {
                if self.mFirstTrailSpace.is_null() {
                    self.mFirstTrailSpace = run_ptr;
                    self.mFirstTrailSpaceLeft = runLeft;
                }
            }
            _ => {
                self.mFirstTrailSpace = std::ptr::null_mut();
            }
        }
        let Some(fontStyle) = run.fontStyle.clone() else {
            self.mLastSHDIndex = -1;
            return;
        };
        let Some(shd) = fontStyle.shd.clone() else {
            self.mLastSHDIndex = -1;
            return;
        };
        if shd.getPattern() == SHD::patNil {
            self.mLastSHDIndex = -1;
            return;
        }
        let mut top = if self.mTextLine.is_null() { 0 } else { unsafe { (*self.mTextLine).getAscentPos() } };
        let mut bottom = if self.mTextLine.is_null() { 0 } else { unsafe { (*self.mTextLine).getDescentPos() } };
        if fontStyle.bdrWidth != 0.0 {
            top = (top as f32 + fontStyle.bdrWidth) as i32;
            bottom = (bottom as f32 - fontStyle.bdrWidth) as i32;
            if run.mBdrInfo.isRtl() {
                if run.hasBeginBdr() { runRight = (runRight as f32 - fontStyle.bdrWidth) as i32; }
                if run.hasEndBdr() && !run.mBdrInfo.isLeftMerged() { runLeft = (runLeft as f32 + fontStyle.bdrWidth) as i32; }
            } else {
                if run.hasBeginBdr() && !run.mBdrInfo.isLeftMerged() { runLeft = (runLeft as f32 + fontStyle.bdrWidth) as i32; }
                if run.hasEndBdr() { runRight = (runRight as f32 - fontStyle.bdrWidth) as i32; }
            }
        }
        let index = self.mLastSHDIndex;
        if index != -1 && self.mSHDs.equals(index, &shd) && self.mSHDs.compareWithRightForBdr(index, &run.mBdrInfo, runLeft) {
            if !self.mCollectTrailSpace && self.mForceInLineBoundary {
                let validRight = runRight.min(self.mLayoutRight);
                if validRight > self.mSHDs.getRight(index) {
                    self.mSHDs.setRight(index, validRight);
                } else {
                    self.mLastSHDIndex = -1;
                }
            } else {
                self.mSHDs.setRight(index, runRight);
            }
            if self.mLastSHDIndex != -1 {
                let index = self.mLastSHDIndex;
                if top < self.mSHDs.getTop(index) { self.mSHDs.setTop(index, top); }
                if bottom > self.mSHDs.getBottom(index) { self.mSHDs.setBottom(index, bottom); }
            }
        } else {
            let validLeft = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runLeft.max(self.mLayoutLeft) } else { runLeft };
            let validRight = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runRight.min(self.mLayoutRight) } else { runRight };
            if validRight > validLeft {
                if validRight < runRight && !self.mFirstTrailSpace.is_null() {
                    runRight = validRight;
                }
                self.mLastSHDIndex = self.mSHDs.add(runLeft, top, runRight, bottom, &shd);
            } else {
                self.mLastSHDIndex = -1;
            }
        }
    }

    pub fn processTrailSpace(&mut self) {
        if self.mEnable && self.mCollectTrailSpace && !self.mFirstTrailSpace.is_null() {
            let mut last = self.mSHDs.size() - 1;
            while last >= 0 && self.mSHDs.getRight(last) > self.mFirstTrailSpaceLeft {
                if self.mSHDs.getLeft(last) < self.mFirstTrailSpaceLeft {
                    self.mSHDs.setRight(last, self.mFirstTrailSpaceLeft);
                    break;
                }
                self.mSHDs.removeLast();
                last -= 1;
            }
            self.mLastSHDIndex = -1;
        }
    }

    pub fn commit(&mut self) {
        if !self.mSHDs.isEmpty() && !self.mTextLine.is_null() {
            let pointer = self.mSHDs.commit();
            let line = self.mTextLine;
            unsafe { (*line).setSHDs(pointer); }
        }
    }
}
