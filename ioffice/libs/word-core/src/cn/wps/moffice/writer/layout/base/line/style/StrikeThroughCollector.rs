// Java: cn.wps.moffice.writer.layout.base.line.style.StrikeThroughCollector
#![allow(dead_code, non_snake_case, unused_variables)]

use thirdparty_font::cn::wps::font::ITypeface::ITypeface;

use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoLines::TypoLines;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

// java public class StrikeThroughCollector extends BaseCollector
pub struct StrikeThroughCollector {
    mEnable: bool,
    mLayoutLeft: i32,
    mLayoutRight: i32,
    mFirstTrailSpace: *mut Graphs,
    mFirstTrailSpaceLeft: i32,
    mCollectTrailSpace: bool,
    mForceInLineBoundary: bool,
    mLayoutState: *mut LayoutState,
    mTextLine: *mut TypoTextLine,
    mTypolines: TypoLines,
    mLastLineIndex: i32,
}

impl Default for StrikeThroughCollector {
    fn default() -> Self { Self::new() }
}

impl StrikeThroughCollector {
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
            mTextLine: std::ptr::null_mut(),
            mTypolines: TypoLines::newInstance(),
            mLastLineIndex: -1,
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
        self.mTypolines.clearCache(line.mSnapshot);
        self.mTextLine = line as *mut TypoTextLine;
        self.mLayoutState = layoutState as *mut LayoutState;
        self.mEnable = true;
        self.mForceInLineBoundary = false;
        self.mLastLineIndex = -1;
    }

    pub fn addRun(&mut self, run: &mut Graphs, runLeft: i32, runRight: i32, revisionAuthor: i32, hyperlinkId: i32) {
        if self.mEnable {
            self.doAddRun(run, runLeft, runRight, revisionAuthor, hyperlinkId);
        }
    }

    pub fn doAddRun(&mut self, run: &mut Graphs, mut runLeft: i32, mut runRight: i32, revisionAuthor: i32, hyperlinkId: i32) {
        if !self.mEnable {
            return;
        }
        let run_ptr = run as *mut Graphs;
        match run.runType {
            3 | 4 | 8 | 9 => {
                self.mLastLineIndex = -1;
                return;
            }
            5 | 10 | 11 | 12 | 13 | 14 => {
                if !self.mLayoutState.is_null() {
                    let state = unsafe { &*self.mLayoutState };
                    if state.needCalcSectionBreakWidth && (state.isShowRevisionInsOrDelInBalloons || revisionAuthor == -1) {
                        return;
                    }
                }
            }
            6 => {
                let mut need_collect = false;
                if run.charsRef.as_ref().map(|c| c.isInMath).unwrap_or(false) && !self.mLayoutState.is_null() && !self.mTextLine.is_null() {
                    let state = self.mLayoutState;
                    let drawing = unsafe { (*state).getDrawing(run) };
                    let snap = unsafe { (*self.mTextLine).mSnapshot };
                    if drawing != 0 && !snap.is_null() && TypoDrawing::getDrawingType__int_typosnapshot(drawing, unsafe { &*snap }) == 13 {
                        need_collect = true;
                    }
                }
                if !need_collect {
                    self.mLastLineIndex = -1;
                    return;
                }
                self.mFirstTrailSpace = std::ptr::null_mut();
            }
            20 | 21 | 22 | 23 | 24 | 25 => {
                self.mFirstTrailSpace = std::ptr::null_mut();
                if run.isAutoNum() {
                    self.mLastLineIndex = -1;
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
        let dfdFlag = run.getDfdFlag();
        let Some(fontStyle) = run.fontStyle.clone() else {
            self.mLastLineIndex = -1;
            return;
        };
        let strikeThrough = fontStyle.getStrikeThrough();
        if strikeThrough == 0 {
            self.mLastLineIndex = -1;
            return;
        }
        let fontStyle = fontStyle.getOrgFontStyle();
        let Some(face) = fontStyle.typeface.as_ref() else {
            self.mLastLineIndex = -1;
            return;
        };
        if fontStyle.emSize == 0.0 {
            self.mLastLineIndex = -1;
            return;
        }
        let mut maxAscent = fontStyle.maxAscent;
        let mut maxDescent = fontStyle.maxDescent;
        let mut strokeWidth = (((fontStyle.fontSize as f32) * (face.getYStrikeoutSize() as f32)) / fontStyle.emSize + 0.5) as i32;
        if fontStyle.isScralForSmallCaps() {
            maxAscent = (maxAscent as f32 / 0.8) as i32;
            maxDescent = (maxDescent as f32 / 0.8) as i32;
            strokeWidth = (strokeWidth as f32 / 0.8) as i32;
        }
        let _ = (maxAscent, maxDescent);
        let linePos = ((run.baseLine as f32) - ((face.getYStrikeoutPosition() as f32) * (fontStyle.fontSize as f32)) / fontStyle.emSize + 0.5) as i32;
        if fontStyle.bdrWidth != 0.0 {
            if run.mBdrInfo.isRtl() {
                if run.hasBeginBdr() {
                    runRight = (runRight as f32 - fontStyle.bdrWidth) as i32;
                }
                if run.hasEndBdr() && !run.mBdrInfo.isLeftMerged() {
                    runLeft = (runLeft as f32 + fontStyle.bdrWidth) as i32;
                }
            } else {
                if run.hasBeginBdr() && !run.mBdrInfo.isLeftMerged() {
                    runLeft = (runLeft as f32 + fontStyle.bdrWidth) as i32;
                }
                if run.hasEndBdr() {
                    runRight = (runRight as f32 - fontStyle.bdrWidth) as i32;
                }
            }
        }
        let index = self.mLastLineIndex;
        let can_merge = index != -1
            && self.mTypolines.getFontColor(index) == fontStyle.fontColor
            && self.mTypolines.getRevisionAuthor(index) == revisionAuthor
            && linePos == self.mTypolines.getYPos(index)
            && self.mTypolines.compareWithRightForBdr(index, &run.mBdrInfo, runLeft)
            && self.mTypolines.getStroke(index) == strokeWidth
            && strikeThrough == -self.mTypolines.getLineType(index)
            && hyperlinkId == self.mTypolines.getHyperlinkId(index)
            && self.mTypolines.getDfdFlag(index) == dfdFlag;
        if can_merge {
            if !self.mCollectTrailSpace && self.mForceInLineBoundary {
                let validRight = runRight.min(self.mLayoutRight);
                if validRight > self.mTypolines.getRight(index) {
                    self.mTypolines.setRight(index, validRight);
                } else {
                    self.mLastLineIndex = -1;
                }
            } else {
                self.mTypolines.setRight(index, runRight);
            }
        } else {
            let validLeft = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runLeft.max(self.mLayoutLeft) } else { runLeft };
            let validRight = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runRight.min(self.mLayoutRight) } else { runRight };
            if validRight > validLeft {
                let shd = if fontStyle.shd.is_none() || self.mTextLine.is_null() {
                    0
                } else {
                    let snap = unsafe { (*self.mTextLine).mSnapshot };
                    if snap.is_null() { 0 } else { unsafe { (*snap).add_shd(fontStyle.shd.as_ref()) } }
                };
                self.mLastLineIndex = self.mTypolines.addStrikeThrough(strikeThrough, runLeft, runRight, linePos, strokeWidth, &fontStyle, shd, revisionAuthor, hyperlinkId, dfdFlag);
            } else {
                self.mLastLineIndex = -1;
            }
        }
    }

    pub fn processTrailSpace(&mut self) {
        if self.mEnable && self.mCollectTrailSpace && !self.mFirstTrailSpace.is_null() {
            let mut lastLineIndex = self.mTypolines.size() - 1;
            while lastLineIndex >= 0 && self.mTypolines.getRight(lastLineIndex) > self.mFirstTrailSpaceLeft {
                if self.mTypolines.getLeft(lastLineIndex) < self.mFirstTrailSpaceLeft {
                    self.mTypolines.setRight(lastLineIndex, self.mFirstTrailSpaceLeft);
                    break;
                }
                self.mTypolines.removeLast();
                lastLineIndex -= 1;
            }
            self.mLastLineIndex = -1;
        }
    }

    pub fn commit(&mut self) {
        if !self.mTypolines.isEmpty() && !self.mTextLine.is_null() {
            let line = self.mTextLine;
            self.mTypolines.commit(unsafe { &mut *line });
        }
    }
}
