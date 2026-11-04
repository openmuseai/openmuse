// Java: cn.wps.moffice.writer.layout.base.line.style.HighlightCollector
#![allow(dead_code, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoHighlights::TypoHighlights;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

// java public class HighlightCollector extends BaseCollector
pub struct HighlightCollector {
    mEnable: bool,
    mLayoutLeft: i32,
    mLayoutRight: i32,
    mFirstTrailSpace: *mut Graphs,
    mFirstTrailSpaceLeft: i32,
    mCollectTrailSpace: bool,
    mForceInLineBoundary: bool,
    mLayoutState: *mut LayoutState,
    mHighlight: TypoHighlights,
    mTextLine: *mut TypoTextLine,
    mLastHighlightIndex: i32,
}

impl Default for HighlightCollector {
    fn default() -> Self { Self::new() }
}

impl HighlightCollector {
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
            mHighlight: TypoHighlights::newInstance(),
            mTextLine: std::ptr::null_mut(),
            mLastHighlightIndex: -1,
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
        self.mHighlight.clearCache(line.mSnapshot);
        self.mLayoutState = layoutState as *mut LayoutState;
        self.mEnable = true;
        self.mTextLine = line as *mut TypoTextLine;
        self.mForceInLineBoundary = false;
        self.mLastHighlightIndex = -1;
    }

    pub fn addRun(&mut self, run: &mut Graphs, mut runLeft: i32, mut runRight: i32, _revisionAuthor: i32, _hyperlinkId: i32) {
        if !self.mEnable { return; }
        let run_ptr = run as *mut Graphs;
        match run.runType {
            3 | 4 | 8 | 9 => {
                self.mLastHighlightIndex = -1;
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
                    self.mLastHighlightIndex = -1;
                    return;
                }
                self.mFirstTrailSpace = run_ptr;
                self.mFirstTrailSpaceLeft = runLeft;
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
        let Some(fontStyle) = run.fontStyle.as_ref() else {
            self.mLastHighlightIndex = -1;
            return;
        };
        let highlight = fontStyle.highlightColor;
        if highlight == 0 {
            self.mLastHighlightIndex = -1;
            return;
        }
        let mut top = 0;
        let mut bottom = if self.mTextLine.is_null() { 0 } else { unsafe { (*self.mTextLine).getDescentPos() } };
        let isInMath = run.charsRef.as_ref().map(|c| c.isInMath).unwrap_or(false);
        if isInMath && run.runType != 6 {
            top = run.baseLine - fontStyle.maxAscent;
            bottom = run.baseLine + fontStyle.maxDescent;
        }
        if fontStyle.bdrWidth != 0.0 {
            if run.mBdrInfo.isRtl() {
                if run.hasBeginBdr() { runRight = (runRight as f32 - fontStyle.bdrWidth) as i32; }
                if run.hasEndBdr() && !run.mBdrInfo.isLeftMerged() { runLeft = (runLeft as f32 + fontStyle.bdrWidth) as i32; }
            } else {
                if run.hasBeginBdr() && !run.mBdrInfo.isLeftMerged() { runLeft = (runLeft as f32 + fontStyle.bdrWidth) as i32; }
                if run.hasEndBdr() { runRight = (runRight as f32 - fontStyle.bdrWidth) as i32; }
            }
        }
        let index = self.mLastHighlightIndex;
        if index != -1 && self.mHighlight.getHighlightColor(index) == highlight && self.mHighlight.compareWithRightForBdr(index, &run.mBdrInfo, runLeft) {
            if !self.mCollectTrailSpace && self.mForceInLineBoundary {
                let validRight = runRight.min(self.mLayoutRight);
                if validRight > self.mHighlight.getRight(index) {
                    self.mHighlight.setRight(index, validRight);
                    if isInMath {
                        self.mHighlight.setTop(index, self.mHighlight.getTop(index).min(top));
                        self.mHighlight.setBottom(index, self.mHighlight.getBottom(index).max(bottom));
                    }
                } else {
                    self.mLastHighlightIndex = -1;
                }
            } else {
                self.mHighlight.setRight(index, runRight);
                if isInMath {
                    self.mHighlight.setTop(index, self.mHighlight.getTop(index).min(top));
                    self.mHighlight.setBottom(index, self.mHighlight.getBottom(index).max(bottom));
                }
            }
        } else {
            let validLeft = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runLeft.max(self.mLayoutLeft) } else { runLeft };
            let validRight = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runRight.min(self.mLayoutRight) } else { runRight };
            if validRight > validLeft {
                self.mLastHighlightIndex = self.mHighlight.add(runLeft, top, runRight, bottom, highlight);
            } else {
                self.mLastHighlightIndex = -1;
            }
        }
    }

    pub fn processTrailSpace(&mut self) {
        if self.mEnable && self.mCollectTrailSpace && !self.mFirstTrailSpace.is_null() {
            let mut last = self.mHighlight.size() - 1;
            while last >= 0 && self.mHighlight.getRight(last) > self.mFirstTrailSpaceLeft {
                if self.mHighlight.getLeft(last) < self.mFirstTrailSpaceLeft {
                    self.mHighlight.setRight(last, self.mFirstTrailSpaceLeft);
                    break;
                }
                self.mHighlight.removeLast();
                last -= 1;
            }
            self.mLastHighlightIndex = -1;
        }
    }

    pub fn commit(&mut self) {
        if !self.mHighlight.isEmpty() && !self.mTextLine.is_null() {
            let pointer = self.mHighlight.commit();
            let line = self.mTextLine;
            unsafe { (*line).setHighlights(pointer); }
        }
    }
}
