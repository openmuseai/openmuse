#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::core::CharParse::{CharInfo, CharParse};
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use super::IPunctSpaceHandler::IPunctSpaceHandler;
use super::PunctTool::PunctTool;

// Java: public final class CompressPunctSpaceHandler implements IPunctSpaceHandler, Reusable
pub struct CompressPunctSpaceHandler {
    // Java: private LayoutState mLayoutState;
    mLayoutState: *mut LayoutState,
    // Java: private SubLineEnv mSubLineEnv;
    mSubLineEnv: *mut SubLineEnv,
    // Java: private CharParse.CharInfo mCharInfo;
    mCharInfo: Option<CharInfo>,
    // Java: private boolean mCanAdjustPunctKerning;
    mCanAdjustPunctKerning: bool,
    // Java: private DocumentEnv mDocEnv;
    mDocEnv: *const DocumentEnv,
    // Java: private ViewEnv mViewEnv;
    mViewEnv: *const ViewEnv,
}

impl CompressPunctSpaceHandler {
    // Java: public CompressPunctSpaceHandler(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn CompressPunctSpaceHandler(docEnv: *const DocumentEnv, viewEnv: *const ViewEnv) -> Self {
        Self { mLayoutState: std::ptr::null_mut(), mSubLineEnv: std::ptr::null_mut(), mCharInfo: None,
            mCanAdjustPunctKerning: false, mDocEnv: docEnv, mViewEnv: viewEnv }
    }

    // Java: public boolean reuseClean()
    pub fn reuseClean(&mut self) -> bool { self.mLayoutState = std::ptr::null_mut(); self.mSubLineEnv = std::ptr::null_mut(); true }
    // Java: public void reuseInit()
    pub fn reuseInit(&mut self) {}
    // Java: public void dispose()
    pub fn dispose(&mut self) { self.mLayoutState = std::ptr::null_mut(); self.mCharInfo = None; }

    // Java: public void setEnv(LayoutState env)
    pub fn setEnv(&mut self, env: *mut LayoutState) {
        self.mLayoutState = env;
        self.mCanAdjustPunctKerning = self.canAdjustPunctKerning();
    }
    // Java: public void setSubLineEnv(SubLineEnv subLineEnv)
    pub fn setSubLineEnv(&mut self, subLineEnv: *mut SubLineEnv) { self.mSubLineEnv = subLineEnv; }

    // Java: private boolean canAdjustPunctKerning(LayoutState env)
    fn canAdjustPunctKerning(&self) -> bool {
        let Some(doc) = (unsafe { self.mDocEnv.as_ref() }) else { return false; };
        let Some(env) = (unsafe { self.mLayoutState.as_ref() }) else { return false; };
        // Java: if (this.mDocEnv.forbidPunctuationKerning) return false;
        if doc.forbidPunctuationKerning { return false; }
        // Java: return 3 != env.pageEnv.gridModel || !this.mDocEnv.forbidEastAsianBreakRules;
        env.pageEnv.gridModel != 3 || !doc.forbidEastAsianBreakRules
    }

    // Java: private boolean isPunctKerningCharClass(Graphs graphs, char ch)
    fn isPunctKerningCharClass(&mut self, graphs: &Graphs, ch: char) -> bool {
        // Java: if (null == this.mCharInfo) this.mCharInfo = new CharParse.CharInfo();
        let info = self.mCharInfo.get_or_insert_with(CharInfo::default);
        // Java: CharParse.getInstance().getCharInfoUseCache(charInfo, ch, true);
        CharParse::getInstance().getCharInfoUseCache(info, ch as i32, true);
        // Java: if (2 == charType) return true;
        if info.charType == 2 { return true; }
        // Java: if (4 == charType) { if (!paraEnv.wordWrap) return true; if (fontHint == 1) return true; }
        if info.charType == 4 {
            let env = unsafe { &*self.mLayoutState };
            if !env.paraEnv.wordWrap { return true; }
            let fontHint = graphs.charsRef.as_ref().and_then(|c| c.finalRunProp.as_ref())
                .map(|p| p.getInt__int_int(40, 0)).unwrap_or(0);
            if fontHint == 1 { return true; }
        }
        false
    }

    // Java: private Integer getFarEastLanguageID(Graphs graphs)
    fn getFarEastLanguageID(&self, graphs: &Graphs) -> Option<i32> {
        graphs.charsRef.as_ref().and_then(|c| c.finalRunProp.as_ref())
            .and_then(|p| if p.contains(42) { Some(p.getInt__int_int(42, i32::MIN)) } else { None })
    }

    // Java: private boolean canAdjustHeadPunctKerning(Graphs prevGraphs, Graphs curGraphs)
    fn canAdjustHeadPunctKerning(&mut self, prev: &Graphs, cur: &Graphs) -> bool {
        let prevCh = unsafe { &*self.mLayoutState }.typoPara.getChar(prev.graphIndex);
        // Java: PREV_UNADJUSTABLE_HEAD_KERNING_PUNCTS contains prevCh => false.
        if "·〝﹙﹛﹝＄￡￥".contains(prevCh) { return false; }
        // Java: if (PunctTool.isTailPunct(prevCh)) only traditional Chinese tail set is adjustable.
        if PunctTool::isTailPunct(prevCh) {
            return self.getFarEastLanguageID(prev) == Some(1028) && "、。，．：；".contains(prevCh);
        }
        if !self.isPunctKerningCharClass(prev, prevCh) { return false; }
        let curCh = unsafe { &*self.mLayoutState }.typoPara.getChar(cur.graphIndex);
        match self.getFarEastLanguageID(cur) {
            Some(1028) => !"‘“〖".contains(curCh),
            Some(1042) => !"‘“〈《「『【〔〖（［｛".contains(curCh),
            _ => true,
        }
    }

    // Java: private boolean canAdjustTailPunctKerning(Graphs curGraphs, Graphs nextGraphs)
    fn canAdjustTailPunctKerning(&mut self, cur: &Graphs, next: &Graphs) -> bool {
        let nextCh = unsafe { &*self.mLayoutState }.typoPara.getChar(next.graphIndex);
        if "〝﹙﹛﹝＄￡￥".contains(nextCh) { return false; }
        if "¨°ˇˉ―‖…‰′″›℃∶〃〞︶︺︾﹀﹄﹚﹜﹞＂％？｀｜～￠".contains(nextCh) { return false; }
        if !self.isPunctKerningCharClass(next, nextCh) { return false; }
        let curCh = unsafe { &*self.mLayoutState }.typoPara.getChar(cur.graphIndex);
        match self.getFarEastLanguageID(cur) {
            Some(1028) => !"·’”、。〗！＇，．：；".contains(curCh),
            Some(1042) => !"·’”、。〉》」』】〕〗！＇），．：；］｝".contains(curCh),
            _ => true,
        }
    }

    // Java: private void doCompactPunctGraphs(Graphs graphs, boolean needToFixDrawOffset)
    fn doCompactPunctGraphs(&mut self, graphs: &mut Graphs, needToFixDrawOffset: bool) {
        // Java: int shrink = (int)((float)graphs.mWidth / 2.0F);
        let shrink = (graphs.mWidth as f32 / 2.0) as i32;
        let env = unsafe { &mut *self.mLayoutState };
        // Java: layoutState.saveOrgWidth(graphs);
        env.saveOrgWidth(graphs);
        // Java: graphs.extendsWidth(-shrink); layoutState.extendWidths(graphs, -shrink, graphs.graphIndex, 1);
        graphs.extendsWidth(-shrink);
        env.extendWidths(graphs, -shrink, graphs.graphIndex, 1);
        // Java: if (needToFixDrawOffset) layoutState.addDrawPosOffsetX(graphs.graphIndex, -shrink);
        if needToFixDrawOffset { env.addDrawPosOffsetX__int_int(graphs.graphIndex, -shrink); }
    }

    // Java: private float getCompactPuntWidth(Graphs graphs)
    fn getCompactPuntWidth(&self, graphs: &Graphs) -> f32 { -(graphs.mWidth as f32 / 2.0) }
    // Java: private int getKernFontSize(Graphs graphs)
    fn getKernFontSize(&self, graphs: &Graphs) -> i32 { graphs.charsRef.as_ref().map(|c| c.hpsKern).unwrap_or(0) }
    // Java: private int getScaledFontSize(Graphs graphs)
    fn getScaledFontSize(&self, graphs: &Graphs) -> i32 { graphs.fontStyle.as_ref().map(|f| (f.charScale * f.fontSize as f32) as i32).unwrap_or(0) }

    // Java: private boolean isForceCompressPunt()
    fn isForceCompressPunt(&self) -> bool {
        // Java: ViewMode.isPhoneReadMode(mViewEnv.getViewMode()) && mViewEnv.getPhoneViewParam().isCompressLine().
        // Java short-circuits for the print view. The phone branch requires its separate
        // PhoneViewParam milestone and must never silently assume a value.
        let phone = unsafe { self.mViewEnv.as_ref() }
            .map(|v| ViewMode::isPhoneReadMode(v.getViewMode())).unwrap_or(false);
        if phone { panic!("PhoneViewParam.isCompressLine is not implemented"); }
        false
    }

    // Java: private boolean doAdjustPunctKerning(Graphs prevGraphs, Graphs nextGraphs, boolean needToAdjPrev)
    fn doAdjustPunctKerning(&mut self, prev: *mut Graphs, next: *mut Graphs, needToAdjPrev: bool) -> bool {
        if !self.mCanAdjustPunctKerning || prev.is_null() || next.is_null() { return false; }
        let (prev, next) = unsafe { (&mut *prev, &mut *next) };
        if !TypoRun::isPunct(prev.runType) || !TypoRun::isPunct(next.runType) { return false; }
        if !self.isForceCompressPunt() {
            let prevKern = self.getKernFontSize(prev);
            let nextKern = self.getKernFontSize(next);
            if prevKern == 0 || prevKern > self.getScaledFontSize(prev) || nextKern == 0 || nextKern > self.getScaledFontSize(next) { return false; }
        }
        let prevCh = unsafe { &*self.mLayoutState }.typoPara.getChar(prev.graphIndex);
        let nextCh = unsafe { &*self.mLayoutState }.typoPara.getChar(next.graphIndex);
        if needToAdjPrev {
            if !PunctTool::isAfterCompressablePunct(prevCh) || !self.canAdjustTailPunctKerning(prev, next) { return false; }
            self.doCompactPunctGraphs(prev, false);
        } else {
            if !PunctTool::isBeforeCompresssablePunct(nextCh) || !self.canAdjustHeadPunctKerning(prev, next) { return false; }
            self.doCompactPunctGraphs(next, true);
        }
        true
    }

    // Java: private float getPunctKerning(Graphs prevGraphs, Graphs nextGraphs, boolean needToAdjPrev)
    fn getPunctKerning(&mut self, prev: *mut Graphs, next: *mut Graphs, needToAdjPrev: bool) -> f32 {
        if !self.mCanAdjustPunctKerning || prev.is_null() || next.is_null() { return 0.0; }
        let (prev, next) = unsafe { (&*prev, &*next) };
        if !TypoRun::isPunct(prev.runType) || !TypoRun::isPunct(next.runType) { return 0.0; }
        if !self.isForceCompressPunt() {
            let prevKern = self.getKernFontSize(prev);
            let nextKern = self.getKernFontSize(next);
            if prevKern == 0 || prevKern > self.getScaledFontSize(prev) || nextKern == 0 || nextKern > self.getScaledFontSize(next) { return 0.0; }
        }
        let prevCh = unsafe { &*self.mLayoutState }.typoPara.getChar(prev.graphIndex);
        let nextCh = unsafe { &*self.mLayoutState }.typoPara.getChar(next.graphIndex);
        if needToAdjPrev {
            if PunctTool::isAfterCompressablePunct(prevCh) && self.canAdjustTailPunctKerning(prev, next) { self.getCompactPuntWidth(prev) } else { 0.0 }
        } else {
            if PunctTool::isBeforeCompresssablePunct(nextCh) && self.canAdjustHeadPunctKerning(prev, next) { self.getCompactPuntWidth(next) } else { 0.0 }
        }
    }

    // Java: public void onAddRun(Graphs prevRun, Graphs run)
    pub fn onAddRun(&mut self, prevRun: *mut Graphs, run: *mut Graphs) {
        if run.is_null() || !TypoRun::isPunct(unsafe { (*run).runType }) { return; }
        let run_ref = unsafe { &mut *run };
        // Java: if (TypoRun.isFarEastLangType(run.language))
        if !TypoRun::isFarEastLangType(run_ref.language) { return; }
        let ch = unsafe { &*self.mLayoutState }.typoPara.getChar(run_ref.graphIndex);
        let canCompressBefore = PunctTool::isBeforeCompresssablePunct(ch);
        let canCompressAfter = PunctTool::isAfterCompressablePunct(ch);
        if !canCompressBefore && !canCompressAfter { return; }
        // Java: if (!canCompressBefore || null != prevRun)
        if canCompressBefore && prevRun.is_null() { return; }
        let adjusted = if canCompressBefore {
            self.doAdjustPunctKerning(prevRun, run, false)
        } else {
            let next = unsafe { &mut *self.mLayoutState }.getGraphList().getNext(run);
            self.doAdjustPunctKerning(run, next, true)
        };
        // Java: if (!hasAdjustedCur) addPunctCompressableSpace(graphIndex, maxBefore, maxAfter, maxCompress).
        if !adjusted {
            let maxCompress = (run_ref.width() as f32 / 2.0) as i32;
            let before = if canCompressBefore { maxCompress } else { 0 };
            let after = if canCompressAfter { maxCompress } else { 0 };
            if let Some(subLine) = unsafe { self.mSubLineEnv.as_mut() } {
                if let Some(compressEnv) = subLine.lineCompressEnv.as_mut() {
                    compressEnv.addPunctCompressableSpace(run_ref.graphIndex, before, after, maxCompress);
                }
            }
        }
    }

    // Java: public float getCompressWidth(Graphs prevRun, Graphs run)
    pub fn getCompressWidth(&mut self, prevRun: *mut Graphs, run: *mut Graphs) -> f32 {
        if run.is_null() { return 0.0; }
        let run_ref = unsafe { &*run };
        if !TypoRun::isPunct(run_ref.runType) || !TypoRun::isFarEastLangType(run_ref.language) { return 0.0; }
        let ch = unsafe { &*self.mLayoutState }.typoPara.getChar(run_ref.graphIndex);
        let before = PunctTool::isBeforeCompresssablePunct(ch);
        let after = PunctTool::isAfterCompressablePunct(ch);
        if !before && !after || before && prevRun.is_null() { return 0.0; }
        if before { self.getPunctKerning(prevRun, run, false) }
        else if after {
            let next = unsafe { &mut *self.mLayoutState }.getGraphList().getNext(run);
            self.getPunctKerning(run, next, true)
        } else { 0.0 }
    }
}

impl IPunctSpaceHandler for CompressPunctSpaceHandler {
    fn setEnv(&mut self, env: *mut LayoutState) { CompressPunctSpaceHandler::setEnv(self, env); }
    fn setSubLineEnv(&mut self, env: *mut SubLineEnv) { CompressPunctSpaceHandler::setSubLineEnv(self, env); }
    fn onAddRun(&mut self, prev: *mut Graphs, run: *mut Graphs) { CompressPunctSpaceHandler::onAddRun(self, prev, run); }
    fn getCompressWidth(&mut self, prev: *mut Graphs, run: *mut Graphs) -> f32 { CompressPunctSpaceHandler::getCompressWidth(self, prev, run) }
    fn dispose(&mut self) { CompressPunctSpaceHandler::dispose(self); }
}
