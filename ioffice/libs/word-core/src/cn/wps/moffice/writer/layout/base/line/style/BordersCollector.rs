// Java: cn.wps.moffice.writer.layout.base.line.style.BordersCollector
#![allow(dead_code, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoBorders::TypoBorders;
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

// java public class BordersCollector extends BaseCollector
pub struct BordersCollector {
    mEnable: bool,
    mLayoutLeft: i32,
    mLayoutRight: i32,
    mFirstTrailSpace: *mut Graphs,
    mFirstTrailSpaceLeft: i32,
    mCollectTrailSpace: bool,
    mForceInLineBoundary: bool,
    mLayoutState: *mut LayoutState,
    mBrcs: TypoBorders,
    mTextLine: *mut TypoTextLine,
    mLastBrcIndex: i32,
}

impl Default for BordersCollector {
    fn default() -> Self { Self::new() }
}

impl BordersCollector {
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
            mBrcs: TypoBorders::newInstance(),
            mTextLine: std::ptr::null_mut(),
            mLastBrcIndex: -1,
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
        self.mBrcs.clearCache(line.mSnapshot);
        self.mLayoutState = layoutState as *mut LayoutState;
        self.mEnable = true;
        self.mTextLine = line as *mut TypoTextLine;
        self.mForceInLineBoundary = false;
        self.mLastBrcIndex = -1;
    }

    pub fn addRun(&mut self, run: &mut Graphs, runLeft: i32, runRight: i32, _revisionAuthor: i32, _hyperlinkId: i32) {
        if !self.mEnable { return; }
        let run_ptr = run as *mut Graphs;
        match run.runType {
            2 | 17 => {
                if self.mFirstTrailSpace.is_null() {
                    self.mFirstTrailSpace = run_ptr;
                    self.mFirstTrailSpaceLeft = runLeft;
                }
            }
            3 | 4 | 8 | 9 => {
                self.mLastBrcIndex = -1;
                return;
            }
            5 | 10 | 11 | 12 | 13 | 14 => {
                if !self.mLayoutState.is_null() && unsafe { (*self.mLayoutState).needCalcSectionBreakWidth } {
                    return;
                }
            }
            20 | 21 | 22 | 23 | 24 | 25 => {
                self.mFirstTrailSpace = std::ptr::null_mut();
            }
            _ => {
                self.mFirstTrailSpace = std::ptr::null_mut();
            }
        }
        let Some(fontStyle) = run.fontStyle.clone() else {
            self.mLastBrcIndex = -1;
            return;
        };
        if !run.hasBdr() {
            self.mLastBrcIndex = -1;
            return;
        }
        let mut brc: Option<BRC> = fontStyle.brc.clone();
        if self.mLastBrcIndex >= 0 && !run.hasBeginOrEndBdr() {
            brc = self.mBrcs.getBRC(self.mLastBrcIndex);
        }
        let adjustBaseLine = run.baseLine - fontStyle.hpsPos;
        let mut top = adjustBaseLine - fontStyle.getAscentBeforeSupOrSubScript(true, true);
        let mut bottom = adjustBaseLine + fontStyle.getDescentBeforeSupOrSubScript(true, true) - 10;
        if !self.mTextLine.is_null() {
            let line = unsafe { &*self.mTextLine };
            let topLimit = 0.max(line.getAscentPos());
            if top < topLimit {
                top = topLimit;
                if TypoRun::isWeakBdrForHeight(run.runType) {
                    run.mBdrInfo.setClipVer(true);
                }
            }
            let bottomLimit = line.getDescentPos().min(line.getTextRectHeight());
            if bottom > bottomLimit {
                bottom = bottomLimit;
                if TypoRun::isWeakBdrForHeight(run.runType) {
                    run.mBdrInfo.setClipVer(true);
                }
            }
        }
        let index = self.mLastBrcIndex;
        if index != -1 && self.mBrcs.equals(index, brc.as_ref()) && runLeft == self.mBrcs.getRight(index) {
            if !self.mCollectTrailSpace && self.mForceInLineBoundary {
                let validRight = runRight.min(self.mLayoutRight);
                if validRight >= self.mBrcs.getRight(index) {
                    self.mBrcs.setRight(index, validRight);
                    run.mBdrInfo.setLeftMerged(true);
                } else {
                    self.mLastBrcIndex = -1;
                }
            } else {
                self.mBrcs.setRight(index, runRight);
                run.mBdrInfo.setLeftMerged(true);
            }
            if self.mLastBrcIndex != -1 {
                let index = self.mLastBrcIndex;
                if top < self.mBrcs.getTop(index) { self.mBrcs.setTop(index, top); }
                if bottom > self.mBrcs.getBottom(index) { self.mBrcs.setBottom(index, bottom); }
            }
        } else {
            let validLeft = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runLeft.max(self.mLayoutLeft) } else { runLeft };
            let validRight = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runRight.min(self.mLayoutRight) } else { runRight };
            if validRight > validLeft {
                self.mLastBrcIndex = self.mBrcs.add(runLeft, top, runRight, bottom, brc.as_ref());
            } else {
                self.mLastBrcIndex = -1;
            }
        }
    }

    pub fn processTrailSpace(&mut self) {
        if self.mEnable && self.mCollectTrailSpace && !self.mFirstTrailSpace.is_null() {
            let mut last = self.mBrcs.size() - 1;
            while last >= 0 && self.mBrcs.getRight(last) > self.mFirstTrailSpaceLeft {
                if self.mBrcs.getLeft(last) < self.mFirstTrailSpaceLeft {
                    self.mBrcs.setRight(last, self.mFirstTrailSpaceLeft);
                    break;
                }
                self.mBrcs.removeLast();
                last -= 1;
            }
            self.mLastBrcIndex = -1;
        }
    }

    pub fn commit(&mut self) {
        if !self.mBrcs.isEmpty() && !self.mTextLine.is_null() {
            let pointer = self.mBrcs.commit();
            let line = self.mTextLine;
            unsafe { (*line).setBorders__int(pointer); }
        }
    }
}
