// Java: cn.wps.moffice.writer.layout.base.line.style.UnderlineCollector
#![allow(dead_code, non_snake_case, unused_variables)]

use thirdparty_font::cn::wps::font::ITypeface::ITypeface;

use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
use crate::cn::wps::moffice::writer::cache::style::TypoLines::TypoLines;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

// java public class UnderlineCollector extends BaseCollector
pub struct UnderlineCollector {
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
    mLastRun: *mut Graphs,
}

impl Default for UnderlineCollector {
    fn default() -> Self { Self::new() }
}

impl UnderlineCollector {
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
            mLastRun: std::ptr::null_mut(),
        }
    }

    pub fn setCollectTrailSpace(&mut self, val: bool) {
        self.mFirstTrailSpace = std::ptr::null_mut();
        self.mCollectTrailSpace = val;
    }

    // java public void noForceInLineBoundary()
    pub fn noForceInLineBoundary(&mut self) {
        self.mForceInLineBoundary = false;
    }

    pub fn setLineBoundary(&mut self, left: i32, right: i32) {
        self.mForceInLineBoundary = true;
        self.mLayoutLeft = left;
        self.mLayoutRight = right;
    }

    // java public void init(TypoTextLine line, LayoutState layoutState)
    pub fn init(&mut self, line: &mut TypoTextLine, layoutState: &mut LayoutState) {
        let snap = line.mSnapshot;
        self.mTypolines.clearCache(snap);
        self.mLayoutState = layoutState as *mut LayoutState;
        self.mTextLine = line as *mut TypoTextLine;
        self.mEnable = true;
        self.mLastRun = std::ptr::null_mut();
        self.mLastLineIndex = -1;
        self.mForceInLineBoundary = false;
    }

    // java public void addRun(...)
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
                self.mLastRun = std::ptr::null_mut();
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
            20 | 21 | 22 | 23 | 24 | 25 => {
                self.mFirstTrailSpace = std::ptr::null_mut();
                if run.isAutoNum() {
                    self.mLastLineIndex = -1;
                    self.mLastRun = std::ptr::null_mut();
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
        let dfdFlag = run.getDfdFlag();
        let Some(fontStyle) = run.fontStyle.clone() else {
            self.mLastRun = std::ptr::null_mut();
            self.mLastLineIndex = -1;
            return;
        };
        let underlineType = fontStyle.getUnderline();
        if underlineType == 0 {
            self.mLastRun = std::ptr::null_mut();
            self.mLastLineIndex = -1;
            return;
        }
        let orgFontStyle = fontStyle.getOrgFontStyle();
        let Some(face) = orgFontStyle.typeface.as_ref() else {
            self.mLastRun = std::ptr::null_mut();
            self.mLastLineIndex = -1;
            return;
        };
        if orgFontStyle.emSize == 0.0 {
            self.mLastRun = std::ptr::null_mut();
            self.mLastLineIndex = -1;
            return;
        }
        let mut underLinePos = ((run.baseLine as f32) - ((face.getUnderlinePosition() as f32) * (orgFontStyle.fontSize as f32)) / orgFontStyle.emSize + 0.5) as i32;
        let mut stroke = (((orgFontStyle.fontSize as f32) * (face.getUnderlineThickness() as f32)) / orgFontStyle.emSize + 0.5) as i32;
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
        let mut needNewTypoLine = true;
        let can_merge = self.mLastLineIndex != -1
            && !self.mLastRun.is_null()
            && underlineType == self.mTypolines.getLineType(self.mLastLineIndex)
            && need_merge(unsafe { &*self.mLastRun }, run)
            && self.mTypolines.compareWithRightForBdr(self.mLastLineIndex, &run.mBdrInfo, runLeft)
            && self.mTypolines.getDfdFlag(self.mLastLineIndex) == dfdFlag;
        if can_merge {
            underLinePos = underLinePos.max(self.mTypolines.getYPos(self.mLastLineIndex));
            stroke = stroke.max(self.mTypolines.getStroke(self.mLastLineIndex));
            let index = self.mLastLineIndex;
            self.mTypolines.setYPos(index, underLinePos);
            self.mTypolines.setStroke(index, stroke);
            if fontStyle.underlineColor == self.mTypolines.getUnderlineColor(index)
                && revisionAuthor == self.mTypolines.getRevisionAuthor(index)
                && hyperlinkId == self.mTypolines.getHyperlinkId(index)
            {
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
                needNewTypoLine = false;
            }
        }
        if needNewTypoLine {
            let validLeft = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runLeft.max(self.mLayoutLeft) } else { runLeft };
            let validRight = if !self.mCollectTrailSpace && self.mForceInLineBoundary { runRight.min(self.mLayoutRight) } else { runRight };
            if validRight > validLeft {
                let shd = if fontStyle.shd.is_none() || self.mTextLine.is_null() {
                    0
                } else {
                    let snap = unsafe { (*self.mTextLine).mSnapshot };
                    if snap.is_null() { 0 } else { unsafe { (*snap).add_shd(fontStyle.shd.as_ref()) } }
                };
                self.mLastLineIndex = self.mTypolines.addUnderline(underlineType, runLeft, runRight, underLinePos, stroke, &fontStyle, shd, revisionAuthor, hyperlinkId, dfdFlag);
            } else {
                self.mLastLineIndex = -1;
            }
        }
        self.mLastRun = run_ptr;
    }

    pub fn processTrailSpace(&mut self) {
        if self.mEnable && self.mCollectTrailSpace && !self.mFirstTrailSpace.is_null() && !self.mLayoutState.is_null() {
            let state = unsafe { &*self.mLayoutState };
            if !state.tagForSpaceTrans {
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
    }

    pub fn commit(&mut self) {
        if !self.mTypolines.isEmpty() && !self.mTextLine.is_null() {
            let line = self.mTextLine;
            self.mTypolines.commit(unsafe { &mut *line });
        }
    }
}

fn hps_pos_down(run: &Graphs) -> bool {
    match run.fontStyle.as_ref() {
        None => false,
        Some(style) if style.hpsPos > 0 => true,
        Some(style) if style.hpsPos == 0 => style.isSubScript(),
        _ => false,
    }
}

// Java needMerge(cur, prev): the call site passes (mLastRun, run), so the
// first argument is the previous run and the second is the current run.
fn need_merge(cur: &Graphs, prev: &Graphs) -> bool {
    let cur_down = hps_pos_down(cur);
    let prev_down = hps_pos_down(prev);
    if cur_down && prev_down {
        let cur_pos = cur.fontStyle.as_ref().map(|s| s.hpsPos).unwrap_or(0);
        let prev_pos = prev.fontStyle.as_ref().map(|s| s.hpsPos).unwrap_or(0);
        cur_pos == prev_pos
    } else {
        !cur_down && !prev_down
    }
}
