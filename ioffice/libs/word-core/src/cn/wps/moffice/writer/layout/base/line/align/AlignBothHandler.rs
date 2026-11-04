// Java: cn.wps.moffice.writer.layout.base.line.align.AlignBothHandler
#![allow(non_snake_case, dead_code)]

use super::AlignExtendBaseHandler::AlignExtendBaseHandler;
use super::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::{KPList, LayoutState};
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public class AlignBothHandler extends AlignExtendBaseHandler
pub struct AlignBothHandler {
    pub super_: AlignExtendBaseHandler,
    // Java: Graphs mPrevGraphs — identity of the node in LayoutState.graphList.
    mPrevGraphs: Option<*mut Graphs>,
}

impl AlignBothHandler {
    pub fn AlignBothHandler() -> Self {
        Self { super_: AlignExtendBaseHandler::AlignExtendBaseHandler(), mPrevGraphs: None }
    }

    fn sub(&self) -> &SubLineEnv { unsafe { &*self.super_.super_.mSubLineEnv } }
    fn sub_mut(&mut self) -> &mut SubLineEnv { unsafe { &mut *self.super_.super_.mSubLineEnv } }
    fn state(&mut self) -> &mut LayoutState { unsafe { &mut *self.super_.super_.mLayoutState } }
    fn ptr(run: &mut Graphs) -> *mut Graphs { run as *mut Graphs }
    fn is_run(slot: Option<*mut Graphs>, run: &Graphs) -> bool {
        slot == Some(run as *const Graphs as *mut Graphs)
    }
    fn prev_needs_space(&self, recalc: bool) -> bool {
        self.mPrevGraphs.is_some_and(|p| {
            let prev = unsafe { &*p };
            let kind = prev.runType;
            prev.language == 6
                || ((kind == 1 || (!recalc && kind == 18))
                    && !TypoRun::isFarEastLangType(prev.language))
        })
    }
    fn count_previous_boundary(&mut self, recalc: bool) {
        if self.prev_needs_space(recalc) {
            let end = unsafe { (*self.mPrevGraphs.unwrap()).getEndGraphIndex() };
            let sub = self.sub_mut();
            if let Some(list) = sub.needExtendsLastGraphsForBothAlign.as_mut() { list.add(end); }
            sub.spaceCount += 1;
        }
    }
    // Java: final void addCount(Graphs run)
    fn addCount(&mut self, run: &mut Graphs) {
        let ptr = Self::ptr(run);
        let sub = self.sub_mut();
        sub.charsCount += run.graphLength;
        if sub.alignRightGraphs.is_none() {
            sub.spaceCount = 0;
            sub.firstGraphForExtends = Some(ptr);
        } else {
            if sub.firstGraphForExtends.is_none() { sub.firstGraphForExtends = Some(ptr); }
            sub.charsCount += sub.spaceCount;
            sub.spaceCount = 0;
        }
        sub.lastGraphForExtends = Some(ptr);
        sub.alignRightGraphs = Some(ptr);
    }
    // Java: final void setAlignRight(Graphs run)
    fn setAlignRight(&mut self, run: &mut Graphs) {
        let ptr = Self::ptr(run);
        let sub = self.sub_mut();
        if sub.alignRightGraphs.is_none() { sub.spaceCount = 0; }
        if sub.firstGraphForExtends.is_none() { sub.firstGraphForExtends = Some(ptr); }
        sub.alignRightGraphs = Some(ptr);
        sub.lastGraphForExtends = Some(ptr);
        if sub.spaceCount > 0 {
            sub.charsCount += sub.spaceCount;
            sub.spaceCount = 0;
        }
    }
    fn resetCount(&mut self) { self.super_.resetCount(); }
    fn resetCountForRecalc(&mut self) { self.super_.resetCountForRecalc(); }

    // Java: public boolean reuseClean()
    pub fn reuseClean(&mut self) -> bool {
        self.mPrevGraphs = None;
        self.super_ = AlignExtendBaseHandler::AlignExtendBaseHandler();
        true
    }
    // Java: public void setSubLineEnv(SubLineEnv subLineEnv, LayoutState layoutState)
    pub fn setSubLineEnv(&mut self, sub: &mut SubLineEnv, state: &mut LayoutState) {
        self.super_.setSubLineEnv(sub, state);
        self.mPrevGraphs = None;
        if sub.hasGraphs() {
            // Java: this.mCurRunPos = subLineEnv.firstGraphs.getLeft();
            // firstGraphs aliases the node later mutated by alignment. The
            // compatibility holder is only a snapshot of that node.
            self.super_.mCurRunPos = sub.firstGraphNode
                .map_or(0, |node| unsafe { (*node).getLeft() });
        }
        self.super_.mNeedExtend = false;
        if sub.layoutLeft >= sub.layoutRight || sub.mStopBothAlign { return; }
        if let Some(right) = sub.alignRightGraphs {
            if sub.charsCount == 0 { return; }
            let run = unsafe { &*right };
            if run.getRight() - self.super_.super_.mCompressedSpaceSum
                > sub.layoutRight - sub.layoutLeft { return; }
            let mut count = sub.charsCount;
            if TypoRun::isFarEastLangType(run.language)
                || (run.runType == 6 && !run.charsRef.as_ref().is_some_and(|c| c.isInMath))
                || run.isFootEndNote() { count -= 1; }
            if count <= 0 { return; }
            self.super_.mNeedExtend = true;
            self.super_.mExtendWidth = sub.layoutRight - sub.layoutLeft - run.getRight()
                + self.super_.super_.mCompressedSpaceSum;
            self.super_.mSpacePerChar = self.super_.mExtendWidth as f32 / count as f32;
            self.super_.mFirstGraphForExtends = sub.firstGraphForExtends;
            self.super_.mLastGraphForExtends = sub.lastGraphForExtends;
        }
    }

    // Java: public void onAddRun(Graphs run)
    pub fn onAddRun(&mut self, run: &mut Graphs) {
        match run.runType {
            2 | 17 => {
                if !self.state().paraEnv.wordWrap && self.sub().alignRightGraphs.is_some() {
                    self.addCount(run);
                } else { self.sub_mut().spaceCount += run.graphLength; }
            }
            3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => {
                if !run.charsRef.as_ref().is_some_and(|c| c.isVanishBreak) { self.resetCount(); }
            }
            6 if run.charsRef.as_ref().is_some_and(|c| c.isInMath) => self.setAlignRight(run),
            6 | 8 | 9 => { self.count_previous_boundary(false); self.addCount(run); }
            20..=25 => self.resetCount(),
            _ => {
                if TypoRun::isFarEastLangType(run.language) {
                    self.count_previous_boundary(false);
                    self.addCount(run);
                } else { self.setAlignRight(run); }
            }
        }
        self.mPrevGraphs = Some(Self::ptr(run));
    }
    // Java: void onAddRunForRecalc(Graphs run)
    fn onAddRunForRecalc(&mut self, run: &mut Graphs) {
        match run.runType {
            2 | 17 => self.sub_mut().spaceCount += run.graphLength,
            3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => self.resetCountForRecalc(),
            6 | 8 | 9 => { self.count_previous_boundary(true); self.addCount(run); }
            20..=25 => self.resetCountForRecalc(),
            _ => {
                if TypoRun::isFarEastLangType(run.language) {
                    self.count_previous_boundary(true);
                    self.addCount(run);
                } else { self.setAlignRight(run); }
            }
        }
        self.mPrevGraphs = Some(Self::ptr(run));
    }
    fn recalcAlign(&mut self, end: *mut Graphs) {
        self.resetCountForRecalc();
        self.mPrevGraphs = None;
        let list: *mut KPList<Graphs> = self.state().getGraphList() as *mut _;
        let mut node = unsafe { (*list).firstElement() };
        while !node.is_null() && node != end {
            let next = unsafe { (*list).getNext(node) };
            self.onAddRunForRecalc(unsafe { &mut *node });
            node = next;
        }
    }
    // Java: public void onRemoveRun(Graphs run)
    pub fn onRemoveRun(&mut self, run: &mut Graphs) {
        if self.sub().needExtendsLastGraphsForBothAlign.as_ref().is_some_and(|v| !v.isEmpty()) {
            self.mPrevGraphs = None;
            if let Some(v) = self.sub_mut().needExtendsLastGraphsForBothAlign.as_mut() { v.resetQuick(); }
            self.recalcAlign(Self::ptr(run));
            return;
        }
        match run.runType {
            2 | 17 => {
                if !self.state().paraEnv.wordWrap && self.sub().alignRightGraphs.is_some() {
                    self.sub_mut().charsCount -= run.graphLength;
                } else { self.sub_mut().spaceCount -= run.graphLength; }
                return;
            }
            3 | 4 => panic!("should not remove this char"),
            // Java: case 6 breaks without changing charsCount when charsRef.isInMath.
            6 if run.charsRef.as_ref().is_some_and(|c| c.isInMath) => {},
            6 => self.sub_mut().charsCount -= 1,
            8 | 9 => self.sub_mut().charsCount -= 1,
            20..=25 => { self.recalcAlign(Self::ptr(run)); return; }
            _ if TypoRun::isFarEastLangType(run.language) => self.sub_mut().charsCount -= run.graphLength,
            _ => {}
        }
        let list: *mut KPList<Graphs> = self.state().getGraphList() as *mut _;
        if Self::ptr(run) == unsafe { (*list).firstElement() } { return; }
        let mut prev = unsafe { (*list).getPrev(Self::ptr(run)) };
        while !prev.is_null() {
            let node = unsafe { &*prev };
            match node.runType {
                2 | 17 => {
                    let sub = self.sub_mut();
                    sub.spaceCount += node.graphLength;
                    sub.charsCount -= node.graphLength;
                    if sub.charsCount < 0 { self.recalcAlign(Self::ptr(run)); return; }
                    prev = unsafe { (*list).getPrev(prev) };
                }
                20..=25 => { self.resetCount(); return; }
                _ => {
                    self.sub_mut().lastGraphForExtends = Some(prev);
                    self.sub_mut().alignRightGraphs = Some(prev);
                    return;
                }
            }
        }
        self.sub_mut().alignRightGraphs = None;
    }
    // Java: protected boolean shouldAdjustSecBreakLine(Graphs run)
    fn shouldAdjustSecBreakLine(&self, run: &Graphs) -> bool {
        !self.super_.mNeedExtend && self.super_.super_.shouldAdjustSecBreakLine(run)
    }

    // Java: public void adjustRun(Graphs run)
    pub fn adjustRun(&mut self, run: &mut Graphs) {
        if !self.super_.mNeedExtend {
            if self.super_.super_.mNeedToCompressGraphs { self.super_.super_.adjustRun(run); }
            return;
        }
        run.offsetXTo(self.super_.mCurRunPos);
        if Self::is_run(self.super_.mFirstGraphForExtends, run) { self.super_.mStartExtend = true; }
        if self.super_.mStartExtend {
            if Self::is_run(self.super_.mLastGraphForExtends, run) { self.super_.mStartExtend = false; }
            let end = run.getEndGraphIndex();
            let extra_boundary = self.sub().needExtendsLastGraphsForBothAlign.as_ref()
                .is_some_and(|list| list.contains(end));
            let aligned = Self::is_run(self.sub().alignRightGraphs, run);
            let in_math = run.charsRef.as_ref().is_some_and(|c| c.isInMath);
            // Java: the text run cases extend CJK characters, but apply the
            // boundary extension only in the non-CJK branch. Math and break
            // cases leave both branches untouched.
            let east_text = matches!(run.runType, 1 | 7 | 18 | 19)
                && TypoRun::isFarEastLangType(run.language);
            let excluded = matches!(run.runType, 5 | 10 | 11 | 12 | 13 | 14)
                || (run.runType == 6 && in_math);
            let extend = !excluded && (east_text || !matches!(run.runType, 1 | 7 | 18 | 19 | 15));
            let extra = extra_boundary && !excluded && !east_text;
            if extend { self.super_.extendSpace(run, aligned); }
            if extra {
                let f_right = run.getRight() as f32 + self.super_.mSpacePerChar + self.super_.mRoundError;
                let i_right = (f_right + 0.5) as i32;
                let i_space = (self.super_.mSpacePerChar + self.super_.mRoundError + 0.5) as i32;
                self.super_.mRoundError = f_right - i_right as f32;
                run.setRight(i_right);
                self.state().extendWidths(run, i_space, end, 1);
            }
        }
        self.super_.mCurRunPos = run.getRight();
    }
    // Java: public void adjustLine(ITypoRect line)
    pub fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) {
        let (left, last_node) = (self.sub().layoutLeft, self.sub().lastGraphNode);
        if std::env::var_os("WORD_PARITY_ALIGN_TRACE").is_some() {
            let cp = unsafe { (*self.super_.super_.mLayoutState).cp };
            if (1038..=1142).contains(&cp) || std::env::var("WORD_PARITY_ALIGN_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(cp) {
                eprintln!("align both cp={} last_right={:?} first_compress={:?} final_compress={:?} compressed={} extend={} chars={} right={}",
                    cp, last_node.map(|r| unsafe { (*r).getRight() }),
                    self.sub().lineCompressEnv.as_ref().map(|e| e.isFirstCompress()),
                    self.sub().lineCompressEnv.as_ref().map(|e| e.getFinalCompress()),
                    self.super_.super_.mCompressedSpaceSum, self.super_.mExtendWidth,
                    self.sub().charsCount, self.sub().layoutRight);
            }
        }
        line.setLeft(left);
        if let Some(last_node) = last_node {
            let last = unsafe { &mut *last_node };
            if self.shouldAdjustSecBreakLine(last) {
                let width = if self.sub().firstGraphNode == Some(last_node) {
                    (self.sub().layoutRight - left).max(0)
                } else { (self.sub().layoutRight - left - last.getLeft()).max(0) };
                last.mWidth = width;
                self.state().typoPara.putWidth(last.graphIndex, width);
            }
            line.setWidth(last.getRight() + if self.super_.mNeedExtend { self.super_.mExtendWidth } else { 0 }
                - self.super_.super_.mCompressedSpaceSum);
        }
        self.super_.mStartExtend = false;
    }
    // Java: public boolean calcLineLeftAndRight(ITypoRect line)
    pub fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool {
        let (last, first, left, right) = (self.sub().lastGraphNode, self.sub().firstGraphNode,
            self.sub().layoutLeft, self.sub().layoutRight);
        if !self.super_.mNeedExtend && last.is_some_and(|r| self.shouldAdjustSecBreakLine(unsafe { &*r }))
            && last == first {
            let last = unsafe { &*last.unwrap() };
            line.setLeft(left);
            line.setWidth(last.getLeft() + (right - left).max(0));
        } else { self.adjustLine(line); }
        true
    }
    // Java: public void onAddBidiRun(Graphs run, boolean isRTL)
    pub fn onAddBidiRun(&mut self, run: &mut Graphs, rtl: bool) {
        if self.sub().mStopBothAlign { self.super_.super_.onAddBidiRun(run, rtl); return; }
        self.super_.super_.mCurOffset += run.width();
        let right = self.super_.super_.mCurOffset;
        if TypoRun::isSpace(run.runType) {
            if self.super_.super_.mFindValidGraphs {
                if !self.state().paraEnv.wordWrap { self.addCount(run); self.super_.super_.mAlignRightOffset = right; }
                else {
                    self.sub_mut().spaceCount += run.graphLength;
                    if self.super_.super_.mParaIsRTL != rtl { self.super_.super_.mAlignRightOffset = right; }
                }
            } else if self.super_.super_.mParaIsRTL != rtl { self.super_.super_.mAlignLeftOffset = right; }
        } else {
            self.super_.super_.mFindValidGraphs = true;
            match run.runType {
                3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => {
                    if !run.charsRef.as_ref().is_some_and(|c| c.isVanishBreak) { self.resetCount(); }
                }
                6 | 8 | 9 => { self.super_.super_.mAlignRightOffset = right; self.addCount(run); }
                20..=25 => { self.resetCount(); self.super_.super_.mAlignRightOffset = right; }
                _ => {
                    self.super_.super_.mAlignRightOffset = right;
                    if TypoRun::isFarEastLangType(run.language) { self.addCount(run); }
                    else { self.setAlignRight(run); }
                }
            }
        }
    }
    // Java: public void adjustBidiLine(ITypoRect line)
    pub fn adjustBidiLine(&mut self, line: &mut TypoRectCoreImpl) {
        let (left, right, count, stop, aligned, last) = {
            let sub = self.sub();
            (sub.layoutLeft, sub.layoutRight, sub.charsCount, sub.mStopBothAlign,
                sub.alignRightGraphs, sub.lastGraphs.clone())
        };
        let base = &self.super_.super_;
        let left_offset = base.mAlignLeftOffset;
        let right_offset = base.mAlignRightOffset;
        line.setLeft(left - left_offset);
        self.super_.mNeedExtend = false;
        if left < right && !stop && right_offset - left_offset <= right - left && base.mFindValidGraphs && count != 0 {
            if let Some(aligned) = aligned {
                let run = unsafe { &*aligned };
                let mut count = count;
                if (TypoRun::isFarEastLangType(run.language) || run.runType == 6 || run.isFootEndNote())
                    && run.graphLength != 1 { count -= 1; }
                if count > 0 {
                    self.super_.mNeedExtend = true;
                    self.super_.mExtendWidth = right - left - right_offset + left_offset;
                    self.super_.mSpacePerChar = self.super_.mExtendWidth as f32 / count as f32;
                    if base.mParaIsRTL {
                        self.super_.mFirstGraphForExtends = self.sub().lastGraphForExtends;
                        self.super_.mLastGraphForExtends = self.sub().firstGraphForExtends;
                    } else {
                        self.super_.mFirstGraphForExtends = self.sub().firstGraphForExtends;
                        self.super_.mLastGraphForExtends = self.sub().lastGraphForExtends;
                    }
                }
            }
        }
        if let Some(last) = last.as_ref() {
            if self.shouldAdjustSecBreakLine(last) {
                let width = if self.sub().firstGraphs.as_ref().is_some_and(|f| f.graphIndex == last.graphIndex) {
                    (right - left + left_offset).max(0)
                } else { (right - left - last.getLeft() + left_offset).max(0) };
                if let Some(real) = self.sub_mut().lastGraphs.as_mut() { real.mWidth = width; }
                self.state().typoPara.putWidth(last.graphIndex, width);
            }
            let right = self.sub().lastGraphs.as_ref().map_or(0, |r| r.getRight());
            line.setWidth(right + if self.super_.mNeedExtend { self.super_.mExtendWidth } else { 0 }
                - self.super_.super_.mCompressedSpaceSum);
        }
        self.super_.mStartExtend = false;
    }
}

impl IHAlignHandler for AlignBothHandler {
    fn setSubLineEnv(&mut self, sub: &mut SubLineEnv, state: &mut LayoutState) { self.setSubLineEnv(sub, state); }
    fn onAddRun(&mut self, run: &mut Graphs) { self.onAddRun(run); }
    fn onRemoveRun(&mut self, run: &mut Graphs) { self.onRemoveRun(run); }
    fn adjustRun(&mut self, run: &mut Graphs) { self.adjustRun(run); }
    fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) { self.adjustLine(line); }
    fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool { self.calcLineLeftAndRight(line) }
    fn prepareRecalcForBidi(&mut self) { self.super_.prepareRecalcForBidi(); }
    fn onAddBidiRun(&mut self, run: &mut Graphs, rtl: bool) { self.onAddBidiRun(run, rtl); }
    fn adjustBidiLine(&mut self, line: &mut TypoRectCoreImpl) { self.adjustBidiLine(line); }
    fn dispose(&mut self) { self.super_.super_.dispose(); }
}
