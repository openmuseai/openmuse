// Java: cn.wps.moffice.writer.layout.base.line.align.AlignDistributeHandler
#![allow(non_snake_case, dead_code)]

use super::AlignExtendBaseHandler::AlignExtendBaseHandler;
use super::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public class AlignDistributeHandler extends AlignExtendBaseHandler
pub struct AlignDistributeHandler { pub super_: AlignExtendBaseHandler }

impl AlignDistributeHandler {
    pub fn AlignDistributeHandler() -> Self {
        Self { super_: AlignExtendBaseHandler::AlignExtendBaseHandler() }
    }

    fn sub(&mut self) -> &mut SubLineEnv { unsafe { &mut *self.super_.super_.mSubLineEnv } }
    fn state(&mut self) -> &mut LayoutState { unsafe { &mut *self.super_.super_.mLayoutState } }
    fn ptr(run: &mut Graphs) -> *mut Graphs { run as *mut Graphs }
    fn is_run(slot: Option<*mut Graphs>, run: &Graphs) -> bool {
        slot == Some(run as *const Graphs as *mut Graphs)
    }

    // Java: protected boolean canExtends(Graphs graphs)
    pub fn canExtends(&self, graphs: &Graphs) -> bool {
        !matches!(graphs.language, 4 | 5 | 6)
            && graphs.charsRef.as_ref().map(|chars| !chars.isInMath).unwrap_or(true)
    }

    // Java: public void setSubLineEnv(SubLineEnv subLineEnv, LayoutState layoutState)
    pub fn setSubLineEnv(&mut self, sub: &mut SubLineEnv, state: &mut LayoutState) {
        self.super_.setSubLineEnv(sub, state);
        if sub.layoutLeft >= sub.layoutRight { return; }
        if let Some(right) = sub.alignRightGraphs {
            if sub.charsCount >= 1 {
                let count = sub.charsCount - i32::from(self.canExtends(unsafe { &*right }));
                if count <= 0 { return; }
                self.super_.mExtendWidth = sub.layoutRight - sub.layoutLeft
                    - unsafe { (*right).getRight() } + self.super_.super_.mCompressedSpaceSum;
                self.super_.mSpacePerChar = self.super_.mExtendWidth as f32 / count as f32;
                self.super_.mFirstGraphForExtends = sub.firstGraphForExtends;
                self.super_.mLastGraphForExtends = sub.lastGraphForExtends;
                self.super_.mNeedExtend = true;
            }
        }
    }

    // Java: private void addNotExtendGraphs(Graphs run)
    pub fn addNotExtendGraphs(&mut self, run: &mut Graphs) {
        let sub = self.sub();
        if sub.alignRightGraphs.is_none() {
            sub.spaceCount = 0;
        } else {
            sub.charsCount += sub.spaceCount;
            sub.spaceCount = 0;
        }
        if sub.firstGraphForExtends.is_none() { sub.firstGraphForExtends = Some(Self::ptr(run)); }
        sub.lastGraphForExtends = Some(Self::ptr(run));
        sub.alignRightGraphs = Some(Self::ptr(run));
    }

    // Java: protected void addCount(Graphs run)
    pub fn addCount(&mut self, run: &mut Graphs) {
        let sub = self.sub();
        sub.charsCount += run.graphLength;
        if sub.alignRightGraphs.is_none() {
            sub.spaceCount = 0;
        } else {
            sub.charsCount += sub.spaceCount;
            sub.spaceCount = 0;
        }
        if sub.firstGraphForExtends.is_none() { sub.firstGraphForExtends = Some(Self::ptr(run)); }
        sub.lastGraphForExtends = Some(Self::ptr(run));
        sub.alignRightGraphs = Some(Self::ptr(run));
    }

    // Java: public void onAddRun(Graphs run)
    pub fn onAddRun(&mut self, run: &mut Graphs) {
        match run.runType {
            2 | 17 => {
                if !self.state().paraEnv.wordWrap && self.sub().alignRightGraphs.is_some() {
                    self.addCount(run);
                } else { self.sub().spaceCount += run.graphLength; }
            }
            3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => {}
            20..=25 => self.super_.resetCount(),
            _ => if self.canExtends(run) { self.addCount(run) } else { self.addNotExtendGraphs(run) },
        }
    }

    // Java: void onAddRunForRecalc(Graphs run)
    fn onAddRunForRecalc(&mut self, run: &mut Graphs) {
        match run.runType {
            2 | 17 => self.sub().spaceCount += run.graphLength,
            3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => {}
            20..=25 => self.super_.resetCountForRecalc(),
            _ => if self.canExtends(run) { self.addCount(run) } else { self.addNotExtendGraphs(run) },
        }
    }

    // Java: final void recalcAlign(Graphs endRun)
    fn recalcAlign(&mut self, end: *mut Graphs) {
        self.super_.resetCountForRecalc();
        let list: *mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList<Graphs> = self.state().getGraphList() as *mut _;
        let mut node = unsafe { (*list).firstElement() };
        while !node.is_null() && node != end {
            let next = unsafe { (*list).getNext(node) };
            self.onAddRunForRecalc(unsafe { &mut *node });
            node = next;
        }
    }

    // Java: public void onRemoveRun(Graphs run)
    pub fn onRemoveRun(&mut self, run: &mut Graphs) {
        match run.runType {
            2 | 17 => {
                if !self.state().paraEnv.wordWrap && self.sub().alignRightGraphs.is_some() {
                    self.sub().charsCount -= run.graphLength;
                } else { self.sub().spaceCount -= run.graphLength; }
                return;
            }
            3 | 4 => panic!("should not remove this char"),
            20..=25 => { self.recalcAlign(Self::ptr(run)); return; }
            _ => if self.canExtends(run) { self.sub().charsCount -= run.graphLength; },
        }
        let list: *mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList<Graphs> = self.state().getGraphList() as *mut _;
        if Self::ptr(run) == unsafe { (*list).firstElement() } { return; }
        let mut previous = unsafe { (*list).getPrev(Self::ptr(run)) };
        while !previous.is_null() {
            let prev = unsafe { &*previous };
            if prev.runType == 2 {
                self.sub().spaceCount += prev.graphLength;
                self.sub().charsCount -= prev.graphLength;
                previous = unsafe { (*list).getPrev(previous) };
            } else if (20..=25).contains(&prev.runType) {
                self.super_.resetCount();
                return;
            } else {
                self.sub().lastGraphForExtends = Some(previous);
                self.sub().alignRightGraphs = Some(previous);
                return;
            }
        }
        self.sub().alignRightGraphs = None;
    }

    // Java: protected boolean shouldAdjustSecBreakLine(Graphs run)
    fn shouldAdjustSecBreakLine(&mut self, run: &Graphs) -> bool {
        !self.super_.mNeedExtend
            && self.sub().firstGraphs.as_ref().map(|g| g.graphIndex) == Some(run.graphIndex)
            && self.super_.super_.shouldAdjustSecBreakLine(run)
    }

    // Java: public void adjustRun(Graphs run)
    pub fn adjustRun(&mut self, run: &mut Graphs) {
        if !self.super_.mNeedExtend { self.super_.super_.adjustRun(run); return; }
        if Self::is_run(self.super_.mFirstGraphForExtends, run) { self.super_.mStartExtend = true; }
        run.offsetXTo(self.super_.mCurRunPos);
        if self.super_.mStartExtend {
            if Self::is_run(self.super_.mLastGraphForExtends, run) { self.super_.mStartExtend = false; }
            let right = self.sub().alignRightGraphs;
            if self.canExtends(run) { self.super_.extendSpace(run, Self::is_run(right, run)); }
        }
        self.super_.mCurRunPos = run.getRight();
    }

    // Java: public void adjustLine(ITypoRect line)
    pub fn adjustLine(&mut self, line: &mut TypoRectCoreImpl) {
        let (left, right, count, aligned, last) = {
            let sub = self.sub();
            (sub.layoutLeft, sub.layoutRight, sub.charsCount, sub.alignRightGraphs,
                sub.lastGraphs.clone())
        };
        let aligned_right = aligned.map(|g| unsafe { (*g).getRight() });
        static TRACE_LINES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let cp = unsafe { (*self.super_.super_.mLayoutState).cp };
        if std::env::var_os("WORD_PARITY_TRACE_BOUNDS").is_some()
            && (TRACE_LINES.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 5
                || (997..=1035).contains(&cp)
                || (1350..=1380).contains(&cp)) {
            eprintln!("align cp={} left={} right={} count={} align_right={:?} last_clone_right={:?} last_type={:?} last_width={:?} last_lang={:?} last_font_size={:?} extend={} compress={}", cp, left, right, count, aligned_right, last.as_ref().map(|g| g.getRight()), last.as_ref().map(|g| g.runType), last.as_ref().map(|g| g.mWidth), last.as_ref().map(|g| g.language), last.as_ref().and_then(|g| g.fontStyle.as_ref().map(|f| f.fontSize)), self.super_.mExtendWidth, self.super_.super_.mCompressedSpaceSum);
        }
        if count == 0 || (count == 1 && aligned.map(|p| self.canExtends(unsafe { &*p })).unwrap_or(false)) {
            line.setLeft((left + right - aligned_right.unwrap_or(0)) / 2);
        } else { line.setLeft(left); }
        if let Some(last_run) = last.as_ref() {
            if self.shouldAdjustSecBreakLine(last_run) {
                line.setLeft(left);
                let width = (right - left - last_run.getLeft()).max(0);
                if let Some(real) = self.sub().lastGraphs.as_mut() { real.mWidth = width; }
                self.state().typoPara.putWidths(last_run.graphIndex, 1, &[width]);
            }
            let last_right = self.sub().lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0);
            line.setWidth(last_right + if self.super_.mNeedExtend { self.super_.mExtendWidth } else { 0 }
                - self.super_.super_.mCompressedSpaceSum);
        }
        self.super_.mStartExtend = false;
    }

    // Java: public boolean calcLineLeftAndRight(ITypoRect line)
    pub fn calcLineLeftAndRight(&mut self, line: &mut TypoRectCoreImpl) -> bool {
        let last = self.sub().lastGraphs.clone();
        if !self.super_.mNeedExtend && last.as_ref().map(|g| self.shouldAdjustSecBreakLine(g)).unwrap_or(false) {
            let sub = self.sub();
            let last = last.as_ref().unwrap();
            let width = (sub.layoutRight - sub.layoutLeft - last.getLeft()).max(0);
            line.setLeft(sub.layoutLeft);
            line.setWidth(last.getLeft() + width);
        } else { self.adjustLine(line); }
        true
    }

    // Java: public void onAddBidiRun(Graphs run, boolean isRTL)
    pub fn onAddBidiRun(&mut self, run: &mut Graphs, isRTL: bool) {
        self.super_.super_.mCurOffset += run.width();
        let run_right = self.super_.super_.mCurOffset;
        if TypoRun::isSpace(run.runType) {
            if self.super_.super_.mFindValidGraphs {
                if !self.state().paraEnv.wordWrap {
                    self.addCount(run);
                    self.super_.super_.mAlignRightOffset = run_right;
                } else {
                    self.sub().spaceCount += run.graphLength;
                    if self.super_.super_.mParaIsRTL != isRTL { self.super_.super_.mAlignRightOffset = run_right; }
                }
            } else if self.super_.super_.mParaIsRTL != isRTL {
                if self.sub().firstGraphForExtends.is_none() { self.sub().firstGraphForExtends = Some(Self::ptr(run)); }
                self.sub().charsCount += run.graphLength;
                self.sub().lastGraphForExtends = Some(Self::ptr(run));
            }
        } else {
            match run.runType {
                3 | 4 | 5 | 10 | 11 | 12 | 13 | 14 => {}
                20..=25 => {
                    self.super_.resetCount();
                    self.super_.super_.mAlignRightOffset = run_right;
                    self.super_.super_.mFindValidGraphs = true;
                }
                _ => {
                    self.super_.super_.mFindValidGraphs = true;
                    self.super_.super_.mAlignRightOffset = run_right;
                    if self.canExtends(run) { self.addCount(run) } else { self.addNotExtendGraphs(run) }
                }
            }
        }
    }

    // Java: public void adjustBidiLine(ITypoRect line)
    pub fn adjustBidiLine(&mut self, line: &mut TypoRectCoreImpl) {
        self.super_.mNeedExtend = false;
        let (left, right, count, valid) = {
            let sub = self.sub();
            (sub.layoutLeft, sub.layoutRight, sub.charsCount, sub.lastGraphForExtends)
        };
        let found = self.super_.super_.mFindValidGraphs;
        if left < right && found {
            if let Some(run) = valid {
                let div = count - i32::from(self.canExtends(unsafe { &*run }));
                if div > 0 {
                    let base = &self.super_.super_;
                    self.super_.mExtendWidth = right - left - base.mAlignRightOffset + base.mAlignLeftOffset;
                    self.super_.mSpacePerChar = self.super_.mExtendWidth as f32 / div as f32;
                    if base.mParaIsRTL {
                        self.super_.mFirstGraphForExtends = self.sub().lastGraphForExtends;
                        self.super_.mLastGraphForExtends = self.sub().firstGraphForExtends;
                    } else {
                        self.super_.mFirstGraphForExtends = self.sub().firstGraphForExtends;
                        self.super_.mLastGraphForExtends = self.sub().lastGraphForExtends;
                    }
                    self.super_.mNeedExtend = true;
                }
            }
        }
        let base = &self.super_.super_;
        let mid = (left + right - base.mAlignRightOffset + base.mAlignLeftOffset + base.mCompressedSpaceSum) / 2;
        if count != 0 && found {
            if count == 1 && valid.map(|p| self.canExtends(unsafe { &*p })).unwrap_or(false) {
                line.setLeft(mid);
            } else { line.setLeft(left + base.mAlignLeftOffset); }
        } else if valid.is_none() { line.setLeft((left + right) / 2); }
        else { line.setLeft(mid); }
        let last = self.sub().lastGraphs.clone();
        if let Some(run) = last.as_ref() {
            if !self.super_.mNeedExtend && self.shouldAdjustSecBreakLine(run) {
                line.setLeft(left);
                let width = (right - left - run.getLeft() + self.super_.super_.mAlignLeftOffset).max(0);
                if let Some(real) = self.sub().lastGraphs.as_mut() { real.mWidth = width; }
                self.state().typoPara.putWidths(run.graphIndex, 1, &[width]);
            }
            line.setWidth(self.sub().lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0)
                + if self.super_.mNeedExtend { self.super_.mExtendWidth } else { 0 }
                - self.super_.super_.mCompressedSpaceSum);
        }
        self.super_.mStartExtend = false;
    }
}

impl IHAlignHandler for AlignDistributeHandler {
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
