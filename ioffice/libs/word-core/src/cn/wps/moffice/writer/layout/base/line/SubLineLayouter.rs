// Java: cn.wps.moffice.writer.layout.base.line.SubLineLayouter
// rust: ★wr-29 05/06 —— 真实移植（Java 508 行 flow/addRun/doAddRun）
#![allow(dead_code, unused_imports, non_snake_case, unused_variables, unused_mut)]

use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::IWhiteSpaceHandler::IWhiteSpaceHandler;
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::IAutoSpaceHandler::IAutoSpaceHandler;
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::CompressTopLinePunctSpaceHandler::CompressTopLinePunctSpaceHandler;
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::IPunctSpaceHandler::IPunctSpaceHandler;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineLayouterBase::SubLineLayouterBase;
use crate::cn::wps::moffice::writer::layout::base::line::TabHandler::TabHandler;
use crate::cn::wps::moffice::writer::layout::base::wordbreaker::HyphenFlag::HyphenFlag;

// java abstract class SubLineLayouter extends SubLineLayouterBase
pub struct SubLineLayouter {
    // java SubLineLayouterBase super_
    pub super_: SubLineLayouterBase,
    // java boolean mContinueFlow;
    pub mContinueFlow: bool,
    // java int mMaxWidth;
    pub mMaxWidth: i32,
    // java protected IHAlignHandler mHAlignHandler;
    pub mHAlignHandler: Option<Box<dyn IHAlignHandler>>,
    // java protected IGridHandler mGridHandler;
    pub mGridHandler: Option<()>,
    // java protected IWhiteSpaceHandler mCharSpaceHandler;
    pub mCharSpaceHandler: Option<Box<dyn IWhiteSpaceHandler>>,
    // java protected IAutoSpaceHandler mAutoSpaceHandler;
    pub mAutoSpaceHandler: Option<Box<dyn IAutoSpaceHandler>>,
    // java protected IPunctSpaceHandler mPunctSpaceHandler;
    pub mPunctSpaceHandler: Option<Box<dyn IPunctSpaceHandler>>,
    // java TabHandler mTabHandler;
    pub mTabHandler: Option<TabHandler>,
    // java protected KerningHandler mKerningHandler;
    pub mKerningHandler: Option<()>,
    // java protected CharBorderHandler mCharBoderHandler;
    pub mCharBoderHandler: Option<()>,
    // java protected boolean mDrawingOnlyInPhoneVeiw = false;
    pub mDrawingOnlyInPhoneVeiw: bool,
    // java protected boolean mHasAutoNumber = false;
    pub mHasAutoNumber: bool,
}

impl SubLineLayouter {
    pub fn super_mut(&mut self) -> &mut SubLineLayouterBase {
        &mut self.super_
    }
    pub fn super_ref(&self) -> &SubLineLayouterBase {
        &self.super_
    }

    // java public SubLineLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn SubLineLayouter(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
        // java super(mainState, docEnv, viewEnv);
        Self {
            super_: SubLineLayouterBase::SubLineLayouterBase(mainState, docEnv, viewEnv),
            mContinueFlow: false,
            mMaxWidth: 0,
            mHAlignHandler: None,
            mGridHandler: None,
            mCharSpaceHandler: None,
            mAutoSpaceHandler: None,
            mPunctSpaceHandler: None,
            mTabHandler: None,
            mKerningHandler: None,
            mCharBoderHandler: None,
            mDrawingOnlyInPhoneVeiw: false,
            mHasAutoNumber: false,
        }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
        // java this.mHAlignHandler = null;
        self.mHAlignHandler = None;
        // java this.mGridHandler = null;
        self.mGridHandler = None;
        // java this.mCharSpaceHandler = null;
        self.mCharSpaceHandler = None;
        // java this.mAutoSpaceHandler = null;
        self.mAutoSpaceHandler = None;
        // java this.mTabHandler = null;
        self.mTabHandler = None;
        // java super.dispose();
        self.super_.dispose();
    }

    fn view_env(&self) -> Option<&ViewEnv> {
        self.super_.super_.mViewEnv.as_ref()
    }
    fn view_mode(&self) -> i32 {
        self.view_env().map(|v| v.getViewMode()).unwrap_or(0)
    }

    // java protected final boolean flow(SubLineEnv subLineEnv, boolean firstSubLine, LayoutState layoutState)
    pub fn flow(&mut self, subLineEnv: &mut SubLineEnv, firstSubLine: bool, layoutState: &mut LayoutState) -> bool {
        // java this.mSubLineEnv = subLineEnv;
        // rust: 调用方别名（测试 SL1 回读同一 SubLineEnv）
        let env_ptr: *mut SubLineEnv = subLineEnv as *mut SubLineEnv;
        self.super_.super_.attachSubLineEnv(env_ptr);
        // java ParaEnv paraEnv = layoutState.paraEnv;
        // java this.mHAlignHandler.setSubLineEnv(subLineEnv, layoutState);
        if let Some(h) = self.mHAlignHandler.as_mut() {
            h.setSubLineEnv(subLineEnv, layoutState);
        }
        // java IGridHandler gridHandler = this.mGridHandler;
        // java if (null != gridHandler) { gridHandler.setSubLineEnv(subLineEnv); }
        let _gridHandler = self.mGridHandler;
        // java this.mTabHandler = null;
        self.mTabHandler = None;
        // java subLineEnv.lineCompressEnv.reset();
        if let Some(lc) = subLineEnv.lineCompressEnv.as_mut() {
            lc.reset();
        }
        // java subLineEnv.mBdrEnv.reset();
        if let Some(bdr) = subLineEnv.mBdrEnv.as_mut() {
            bdr.reset();
        }
        // java subLineEnv.layoutLeft = subLineEnv.left;
        subLineEnv.layoutLeft = subLineEnv.left;
        // java subLineEnv.layoutRight = subLineEnv.right;
        subLineEnv.layoutRight = subLineEnv.right;
        // java ParaPropEnv paraPropEnv = paraEnv.curParaPropEnv;
        let (leftIndent, firstLineIndent, rightIndent, hasAutoNums) = layoutState
            .paraEnv
            .curParaPropEnv
            .as_ref()
            .map(|p| (p.leftIndent, p.firstLineIndent, p.rightIndent, p.hasAutoNums))
            .unwrap_or((0, 0, 0, false));
        // java if (subLineEnv.enableLeftIndent)
        if subLineEnv.enableLeftIndent {
            // java subLineEnv.layoutLeft += paraPropEnv.leftIndent;
            subLineEnv.layoutLeft += leftIndent;
            // java if (firstSubLine)
            if firstSubLine {
                // java subLineEnv.layoutLeft += paraPropEnv.firstLineIndent;
                subLineEnv.layoutLeft += firstLineIndent;
            }
        } else if hasAutoNums {
            // java else if (paraEnv.curParaPropEnv.hasAutoNums)
            // java subLineEnv.layoutLeft += paraPropEnv.leftIndent;
            subLineEnv.layoutLeft += leftIndent;
            // java if (firstSubLine && paraPropEnv.firstLineIndent > 0)
            if firstSubLine && firstLineIndent > 0 {
                // java subLineEnv.layoutLeft += paraPropEnv.firstLineIndent;
                subLineEnv.layoutLeft += firstLineIndent;
            }
        } else {
            // java subLineEnv.layoutLeft = Math.max(subLineEnv.layoutLeft, paraPropEnv.leftIndent);
            subLineEnv.layoutLeft = subLineEnv.layoutLeft.max(leftIndent);
            // java if (firstSubLine && paraPropEnv.firstLineIndent > 0)
            if firstSubLine && firstLineIndent > 0 {
                // java subLineEnv.layoutLeft += paraPropEnv.firstLineIndent;
                subLineEnv.layoutLeft += firstLineIndent;
            }
        }
        // java if (this.mViewEnv.isShowCardInfo() && layoutState.isInUiCard())
        if self.view_env().map(|v| v.isShowCardInfo()).unwrap_or(false) && layoutState.isInUiCard() {
            // java this.mMainState.isInUiCard = true;
            if let Some(ms) = self.super_.super_.mMainState.as_mut() {
                ms.isInUiCard = true;
            }
            // java subLineEnv.layoutLeft += this.mViewEnv.getCardMarginLeft();
            subLineEnv.layoutLeft += self.view_env().map(|v| v.getCardMarginLeft()).unwrap_or(0);
            // java subLineEnv.layoutRight -= this.mViewEnv.getCardMarginRight();
            subLineEnv.layoutRight -= self.view_env().map(|v| v.getCardMarginRight()).unwrap_or(0);
        } else {
            // java this.mMainState.isInUiCard = false;
            if let Some(ms) = self.super_.super_.mMainState.as_mut() {
                ms.isInUiCard = false;
            }
        }
        // java if (subLineEnv.enableRightIndent)
        if subLineEnv.enableRightIndent {
            // java subLineEnv.layoutRight -= paraPropEnv.rightIndent;
            subLineEnv.layoutRight -= rightIndent;
            // java subLineEnv.layoutRight = Math.max(subLineEnv.layoutLeft, subLineEnv.layoutRight);
            subLineEnv.layoutRight = subLineEnv.layoutLeft.max(subLineEnv.layoutRight);
        }
        if std::env::var_os("WORD_PARITY_TRACE_BOUNDS").is_some()
            && (layoutState.cp < 100 || (1038..=1142).contains(&layoutState.cp)
                || std::env::var("WORD_PARITY_TRACE_BOUNDS_CP").ok()
                    .and_then(|cp| cp.parse::<i32>().ok()) == Some(layoutState.cp)) {
            eprintln!("bounds cp={} page_width={} margin_l={} margin_r={} core={} sub_right={} right_indent={} layout_right={} section_offset={}", layoutState.cp, layoutState.pageEnv.width, layoutState.pageEnv.marginLeft, layoutState.pageEnv.marginRight, layoutState.pageEnv.getCoreWidth(), subLineEnv.right, rightIndent, subLineEnv.layoutRight, layoutState.pageEnv.sectionMarginOffset);
        }
        // java this.mContinueFlow = true;
        self.mContinueFlow = true;
        // java this.mMaxWidth = subLineEnv.layoutRight - subLineEnv.layoutLeft;
        self.mMaxWidth = subLineEnv.layoutRight - subLineEnv.layoutLeft;
        // java Graphs run = layoutState.get();
        let mut run: *mut Graphs = layoutState.get();
        // java int viewMode = this.mViewEnv.getViewMode();
        let viewMode: i32 = self.view_mode();
        // java boolean needCheckAnchorDrawing = ViewMode.isPhoneReadMode(viewMode) || viewMode == 4;
        let needCheckAnchorDrawing: bool = ViewMode::isPhoneReadMode(viewMode) || viewMode == 4;
        // java this.mDrawingOnlyInPhoneVeiw = false;
        self.mDrawingOnlyInPhoneVeiw = false;
        // java if (needCheckAnchorDrawing && run.runType == 6 && layoutState.typoPara.getChar(run.graphIndex) == '\b')
        if needCheckAnchorDrawing && !run.is_null() {
            let rt = unsafe { (*run).runType };
            let gi = unsafe { (*run).graphIndex };
            if rt == 6 && layoutState.typoPara.getChar(gi) == '\u{0008}' {
                // java this.mDrawingOnlyInPhoneVeiw = true;
                self.mDrawingOnlyInPhoneVeiw = true;
            }
        }
        // java if (null == run)
        if run.is_null() {
            // java return this.mContinueFlow;
            return self.mContinueFlow;
        }
        // java CharBorderHandler charBorderHandler = this.mCharBoderHandler;
        // java if (charBorderHandler != null) { charBorderHandler.setSubLineEnv(subLineEnv); }
        // java if (run.isAutoNum())
        if unsafe { (*run).isAutoNum() } {
            // java run = this.flowAutoNum(run, layoutState);
            run = self.flowAutoNum(run, layoutState);
            // java subLineEnv.firstGraphIndexInCP = run.isAutoNum() ? run.graphIndex + 1 : run.graphIndex;
            if !run.is_null() {
                subLineEnv.firstGraphIndexInCP = if unsafe { (*run).isAutoNum() } {
                    unsafe { (*run).graphIndex + 1 }
                } else {
                    unsafe { (*run).graphIndex }
                };
            }
            // java this.mHasAutoNumber = true;
            self.mHasAutoNumber = true;
        }
        // java IPunctSpaceHandler punctSpaceHandler = this.mPunctSpaceHandler;
        if let Some(handler) = self.mPunctSpaceHandler.as_mut() {
            handler.setEnv(layoutState as *mut LayoutState);
            handler.setSubLineEnv(env_ptr);
        }
        // java CompressTopLinePunctSpaceHandler.compressPunct(run, layoutState);
        if !run.is_null() {
            CompressTopLinePunctSpaceHandler::compressPunct(unsafe { &mut *run }, layoutState);
        }
        // java IWhiteSpaceHandler charSpaceHandler = this.mCharSpaceHandler;
        // java if (null != charSpaceHandler) { charSpaceHandler.setSubLineEnv(subLineEnv); }
        if let Some(cs) = self.mCharSpaceHandler.as_mut() {
            cs.setSubLineEnv(subLineEnv);
            cs.setEnv(layoutState);
        }
        // java IAutoSpaceHandler autoSpaceHandler = this.mAutoSpaceHandler;
        if let Some(handler) = self.mAutoSpaceHandler.as_mut() { handler.setSubLineEnv(env_ptr); }
        // java KerningHandler kerningHandler = this.mKerningHandler;
        // java Graphs prevRun = null;
        let mut prevRun: *mut Graphs = std::ptr::null_mut();
        // java boolean isFinishLine = false;
        let mut isFinishLine: bool = false;
        // java while(null != run)
        while !run.is_null() {
            let runType: i32 = unsafe { (*run).runType };
            match runType {
                // java case 2: case 17:
                2 | 17 => {
                    let mut space_ok = true;
                    if let Some(cs) = self.mCharSpaceHandler.as_mut() {
                        if prevRun.is_null() {
                            space_ok = cs.handleSpace(unsafe { &mut *run }, unsafe { &mut *run });
                        } else {
                            space_ok = cs.handleSpace(unsafe { &mut *prevRun }, unsafe { &mut *run });
                        }
                    }
                    // java if (!charSpaceHandler.handleSpace(prevRun, run))
                    if !space_ok {
                        // java if (this.mSubLineEnv.lastGraphs == run && this.mHAlignHandler != null)
                        let last_is_run = unsafe {
                            (*env_ptr).lastGraphs.as_ref().map(|g| g.graphIndex == (*run).graphIndex).unwrap_or(false)
                        };
                        if last_is_run && self.mHAlignHandler.is_some() {
                            // java boolean oldWordWrap = layoutState.paraEnv.wordWrap;
                            let oldWordWrap: bool = layoutState.paraEnv.wordWrap;
                            // java layoutState.paraEnv.wordWrap = false;
                            layoutState.paraEnv.wordWrap = false;
                            // java this.mHAlignHandler.onAddRun(run);
                            if let Some(h) = self.mHAlignHandler.as_mut() {
                                h.onAddRun(unsafe { &mut *run });
                            }
                            // java layoutState.paraEnv.wordWrap = oldWordWrap;
                            layoutState.paraEnv.wordWrap = oldWordWrap;
                        }
                        // java run = null;
                        run = std::ptr::null_mut();
                        // java this.mContinueFlow = false;
                        self.mContinueFlow = false;
                        // java isFinishLine = true;
                        isFinishLine = true;
                    }
                }
                // java case 3: case 4:
                3 | 4 => {
                    // java this.doAddRun(run);
                    self.doAddRun(run, env_ptr);
                    // java this.mContinueFlow = false;
                    self.mContinueFlow = false;
                    // java if (null != this.mTabHandler) { this.mTabHandler.commit(); }
                    if let Some(tab) = self.mTabHandler.as_mut() {
                        tab.commit();
                    }
                    // java this.mHAlignHandler.onAddRun(run);
                    if let Some(h) = self.mHAlignHandler.as_mut() {
                        h.onAddRun(unsafe { &mut *run });
                    }
                    // java isFinishLine = true;
                    isFinishLine = true;
                    // java run = null;
                    run = std::ptr::null_mut();
                }
                // java case 5:
                5 => {
                    // java this.mContinueFlow = false;
                    self.mContinueFlow = false;
                    // java case 10..14: isFinishLine = true;  fall through
                    isFinishLine = true;
                }
                10 | 11 | 12 | 13 | 14 => {
                    // java isFinishLine = true;
                    isFinishLine = true;
                }
                // java case 18:
                18 => {
                    // java if (null != punctSpaceHandler) { punctSpaceHandler.onAddRun(prevRun, run); }
                    if let Some(handler) = self.mPunctSpaceHandler.as_mut() { handler.onAddRun(prevRun, run); }
                }
                // java case 20..25 tab
                20 | 21 | 22 | 23 | 24 | 25 => {
                    // java if (this.tryTransForSpace(run, layoutState)) { continue; }
                    if self.tryTransForSpace(unsafe { &mut *run }, layoutState) {
                        run = layoutState.get();
                        continue;
                    }
                    // java if (this.doAddTab(run, layoutState))
                    if self.doAddTab(run, env_ptr, layoutState) {
                        // java prevRun = run;
                        prevRun = run;
                        // java run = layoutState.get();
                        run = layoutState.get();
                        continue;
                    }
                    // java run = null; this.mContinueFlow = false; isFinishLine = true;
                    run = std::ptr::null_mut();
                    self.mContinueFlow = false;
                    isFinishLine = true;
                }
                26 | 27 => {
                    // java if (this.tryTransForSpace(run, layoutState)) { continue; }
                    if self.tryTransForSpace(unsafe { &mut *run }, layoutState) {
                        run = layoutState.get();
                        continue;
                    }
                }
                28 => {
                    // java if (run.graphLength == 1)
                    if unsafe { (*run).graphLength } == 1 {
                        let gi = unsafe { (*run).graphIndex };
                        // java layoutState.typoPara.putWidth(run.graphIndex, 0);
                        layoutState.typoPara.putWidth(gi, 0);
                        // java run.mWidth = 0;
                        unsafe { (*run).mWidth = 0; }
                        // java run.mHyphenFlag = HyphenFlag.setIncludeSoftHyphen(...)
                        unsafe {
                            (*run).mHyphenFlag = HyphenFlag::setIncludeSoftHyphen((*run).mHyphenFlag, true);
                        }
                    }
                    // java run.runType = 1;
                    unsafe { (*run).runType = 1; }
                }
                29 => {
                    if unsafe { (*run).graphLength } == 1 {
                        unsafe {
                            (*run).mHyphenFlag = HyphenFlag::setIncludeNoBreakHyphen((*run).mHyphenFlag, true);
                        }
                    }
                    unsafe { (*run).runType = 1; }
                }
                // java case 6..9, 15, 16, 19, default
                _ => {}
            }
            // java if (isFinishLine)
            if isFinishLine {
                // java if (charSpaceHandler != null) { charSpaceHandler.endLine(); }
                if let Some(cs) = self.mCharSpaceHandler.as_mut() {
                    cs.endLine();
                }
                // java break;
                break;
            }
            // Java: if (null != autoSpaceHandler && prevRun != null) autoSpaceHandler.dealAutoSpace(prevRun, run);
            if !prevRun.is_null() {
                if let Some(handler) = self.mAutoSpaceHandler.as_mut() { handler.dealAutoSpace(prevRun, run); }
            }
            // java prevRun = run;
            prevRun = run;
            // java if (!this.addRun(run, layoutState))
            if !self.addRun(run, env_ptr, layoutState) {
                // java run = null;
                run = std::ptr::null_mut();
                if let Some(cs) = self.mCharSpaceHandler.as_mut() {
                    cs.endLine();
                }
                // java break;
                break;
            }
            // java if (this.mDrawingOnlyInPhoneVeiw) { run = null; break; }
            if self.mDrawingOnlyInPhoneVeiw {
                run = std::ptr::null_mut();
                break;
            }
            // java run = layoutState.get();
            run = layoutState.get();
            // java if (this.needBreakLineOnShowCardInfo(run, layoutState)) { run = null; }
            if self.needBreakLineOnShowCardInfo(run, layoutState) {
                run = std::ptr::null_mut();
            }
            let _ = needCheckAnchorDrawing;
        }
        // java if (null != run && TypoRun.isTypoPageBreak(run.runType))
        if !run.is_null() && TypoRun::isTypoPageBreak(unsafe { (*run).runType }) {
            // java layoutState.needCalcSectionBreakWidth = true;
            // The following Java fix-section-break branch can suppress this
            // only for a type-5 page break followed by a paragraph mark.
            // Type 10..13 section breaks always keep it enabled.
            layoutState.needCalcSectionBreakWidth = true;
            // java this.doAddRun(run);
            self.doAddRun(run, env_ptr);
            if let Some(tab) = self.mTabHandler.as_mut() {
                tab.commit();
            }
            if let Some(h) = self.mHAlignHandler.as_mut() {
                h.onAddRun(unsafe { &mut *run });
            }
        } else if unsafe { (*env_ptr).lastGraphNode.is_some_and(|g| (*g).runType == 4) } {
            // java else if (lastGraphs.runType == 4 && null != (run = layoutState.getNextSectionBreak()))
            let section_break = layoutState.getNextSectionBreak();
            if !section_break.is_null() {
                // java layoutState.needCalcSectionBreakWidth = true;
                layoutState.needCalcSectionBreakWidth = true;
                // java this.doAddRun(run);
                self.doAddRun(section_break, env_ptr);
            }
        }
        // java return this.mContinueFlow;
        self.mContinueFlow
    }

    // java private boolean needBreakLineOnShowCardInfo(Graphs run, LayoutState layoutState)
    fn needBreakLineOnShowCardInfo(&self, run: *mut Graphs, layoutState: &mut LayoutState) -> bool {
        // java if (!this.mViewEnv.isShowCardInfo())
        if !self.view_env().map(|v| v.isShowCardInfo()).unwrap_or(false) {
            // java return false;
            return false;
        }
        if run.is_null() {
            return false;
        }
        // java else if (null != run && layoutState.needLineBreak(run.graphIndex))
        if layoutState.needLineBreak(unsafe { (*run).graphIndex }) {
            // java this.mSubLineEnv.mStopBothAlign = true;
            // java return true;
            return true;
        }
        // java return false;
        false
    }

    // java protected boolean tryTransForSpace(Graphs run, LayoutState layoutState)
    pub fn tryTransForSpace(&mut self, run: &mut Graphs, layoutState: &mut LayoutState) -> bool {
        let _ = (run, layoutState);
        // java return false;
        false
    }

    // java final Graphs flowAutoNum(Graphs firstRun, LayoutState layoutState)
    pub fn flowAutoNum(&mut self, firstRun: *mut Graphs, layoutState: &mut LayoutState) -> *mut Graphs {
        // java int pos = 0;
        let mut pos = 0;
        // java for(run = firstRun; null != run; run = layoutState.get())
        let mut run = firstRun;
        while !run.is_null() {
            // java run.offsetXTo(pos);
            unsafe { (*run).offsetXTo(pos); }
            // java if (run.isTab() || !run.isAutoNum()) break;
            if unsafe { (*run).isTab() || !(*run).isAutoNum() } {
                break;
            }
            // java if (this.mCharBoderHandler != null) this.mCharBoderHandler.handleRun(run);
            // rust: CharBorderHandler is not constructed on this layout path.
            // java pos += run.width();
            pos += unsafe { (*run).width() };
            // java this.mSubLineEnv.addGraphs(run);
            self.super_.super_.subLineEnv_mut().addGraphs(run);
            run = layoutState.get();
        }
        // java switch (layoutState.paraEnv.curParaPropEnv.autoNumsJC)
        let jc = layoutState.paraEnv.curParaPropEnv.as_ref().map(|p| p.autoNumsJC).unwrap_or(0);
        let offset = match jc {
            // java case 1: offset = -pos / 2;
            1 => -pos / 2,
            // java case 2: offset = -pos;
            2 => -pos,
            _ => 0,
        };
        // java if (0 != offset) { for (Graphs runPos : layoutState.getGraphList()) runPos.offsetX(offset); }
        if offset != 0 {
            let graph_list = layoutState.getGraphList();
            let count = graph_list.size() as usize;
            for index in 0..count {
                let graph = graph_list.get_at(index);
                if !graph.is_null() {
                    unsafe { (*graph).offsetX(offset); }
                }
            }
        }
        // java return run;
        run
    }

    // java private void doAddRun(Graphs run)
    fn doAddRun(&mut self, run: *mut Graphs, env_ptr: *mut SubLineEnv) {
        // java int runPos = 0;
        let mut runPos: i32 = 0;
        // java if (this.mSubLineEnv.hasGraphs())
        unsafe {
            if (*env_ptr).hasGraphs() {
                // java runPos = this.mSubLineEnv.lastGraphs.getRight();
                // Java lastGraphs aliases the same Graphs node that TabHandler.setRight mutates.
                runPos = (*env_ptr).lastGraphNode.map(|g| (*g).getRight()).unwrap_or(0);
            }
            // java run.offsetXTo(runPos);
            (*run).offsetXTo(runPos);
            // java this.mSubLineEnv.addGraphs(run);
            (*env_ptr).addGraphs(run);
        }
    }

    // java private boolean doAddTab(Graphs run, LayoutState layoutState)
    fn doAddTab(&mut self, run: *mut Graphs, env_ptr: *mut SubLineEnv, layoutState: &mut LayoutState) -> bool {
        // java this.doAddRun(run);
        self.doAddRun(run, env_ptr);
        // java if (null == this.mTabHandler) { mDocEnv.layoutToolsManager.getTabHandler(); ... }
        if self.mTabHandler.is_none() {
            let doc = self.super_.super_.mDocEnv.as_mut().expect("TabHandler needs DocumentEnv");
            let view = self.super_.super_.mViewEnv.as_mut().expect("TabHandler needs ViewEnv");
            let mut tab = TabHandler::with_env(doc, view);
            tab.setEnv(layoutState);
            tab.setSubLineEnv(unsafe { &mut *env_ptr });
            self.mTabHandler = Some(tab);
        }
        // java boolean continueAddRun = this.mTabHandler.onAddTab(run);
        let mut continueAddRun = self.mTabHandler.as_mut().unwrap().onAddTab(unsafe { &mut *run });
        // java if (continueAddRun) { this.mHAlignHandler.onAddRun(run); }
        if continueAddRun {
            if let Some(handler) = self.mHAlignHandler.as_mut() { handler.onAddRun(unsafe { &mut *run }); }
        } else if layoutState.isInMathParaLine() {
            // java rollbackWhileInMathPara(...)
            continueAddRun = false;
        } else if self.mTabHandler.as_ref().unwrap().needRollBackTab() {
            let prev = layoutState.graphList.getPrev(run);
            unsafe { (*env_ptr).lastGraphNode = (!prev.is_null()).then_some(prev); }
        } else if let Some(handler) = self.mHAlignHandler.as_mut() {
            self.super_.super_.attachSubLineEnv(env_ptr);
            continueAddRun = self.super_.doAddExcessGraphs(handler.as_mut(), self.mTabHandler.as_mut(), run, self.mMaxWidth, layoutState);
        }
        continueAddRun
    }

    // java private boolean addRun(Graphs run, LayoutState layoutState)
    fn addRun(&mut self, run: *mut Graphs, env_ptr: *mut SubLineEnv, layoutState: &mut LayoutState) -> bool {
        // java int runPos = 0;
        let mut runPos: i32 = 0;
        unsafe {
            // java if (this.mSubLineEnv.hasGraphs())
            if (*env_ptr).hasGraphs() {
                // java runPos = this.mSubLineEnv.lastGraphs.getRight();
                // Java lastGraphs aliases the same Graphs node that TabHandler.setRight mutates.
                runPos = (*env_ptr).lastGraphNode.map(|g| (*g).getRight()).unwrap_or(0);
            }
            // java run.offsetXTo(runPos);
            (*run).offsetXTo(runPos);
            // java this.mSubLineEnv.addGraphs(run);
            (*env_ptr).addGraphs(run);
        }
        let trace_line = std::env::var("WORD_PARITY_LINE_TRACE_CP").ok()
            .and_then(|value| value.parse::<i32>().ok()) == Some(layoutState.cp);
        if trace_line {
            let node = unsafe { &*run };
            eprintln!("line_add before cp={} graph={} length={} type={} width={} right={} max={}",
                layoutState.cp, node.graphIndex, node.graphLength, node.runType,
                node.width(), node.getRightWithBdr(), self.mMaxWidth);
        }
        // java boolean continueAddRun = true;
        let mut continueAddRun: bool = true;
        // java if (null != this.mTabHandler)
        if let Some(tab) = self.mTabHandler.as_mut() {
            // java if (!this.mTabHandler.onAddRun(run, 0) && run.getRight() > this.mMaxWidth)
            if !tab.onAddRun(unsafe { &mut *run }, 0) && unsafe { (*run).getRight() } > self.mMaxWidth {
                // java continueAddRun = false;
                continueAddRun = false;
            }
        } else if unsafe { (*run).getRightWithBdr() } - self.mMaxWidth >= 2 {
            // java else if (run.getRightWithBdr() - this.mMaxWidth >= 2)
            // java continueAddRun = false;
            continueAddRun = false;
        }
        // java if (continueAddRun)
        if continueAddRun {
            // java this.mHAlignHandler.onAddRun(run);
            if let Some(h) = self.mHAlignHandler.as_mut() {
                h.onAddRun(unsafe { &mut *run });
            }
        } else if layoutState.isInMathParaLine() {
            // java else if (layoutState.isInMathParaLine())
            // java continueAddRun = this.rollbackWhileInMathPara(...);
            // rust: sample2 非数学行；数学回退仍 DEFERRED
        } else {
            // java else { continueAddRun = this.doAddExcessGraphs(this.mHAlignHandler, this.mTabHandler, run, this.mMaxWidth, layoutState); }
            self.super_.super_.attachSubLineEnv(env_ptr);
            if let Some(h) = self.mHAlignHandler.as_mut() {
                continueAddRun = self.super_.doAddExcessGraphs(
                    h.as_mut(),
                    self.mTabHandler.as_mut(),
                    run,
                    self.mMaxWidth,
                    layoutState,
                );
            } else {
                continueAddRun = false;
            }
        }
        // java if (!continueAddRun && ViewMode.isPhoneReadMode(this.mViewEnv.getViewMode())) { ... hyphen ... }
        if trace_line {
            eprintln!("line_add after cp={} graph={} continue={} last={}",
                layoutState.cp, unsafe { (*run).graphIndex }, continueAddRun,
                unsafe { (*env_ptr).lastGraphNode.map(|last| (*last).graphIndex).unwrap_or(-1) });
        }
        return continueAddRun;
    }
}
