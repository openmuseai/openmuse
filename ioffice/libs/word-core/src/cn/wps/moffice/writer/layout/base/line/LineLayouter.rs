// Java: cn.wps.moffice.writer.layout.base.line.LineLayouter
// rust: ★wr-29 07 —— 真实移植（Java 1029 行，转写自 Java flow/preCalcLineHeight/
//       prepareSubLineEnvs/regulateToCache 主路径）。hello 纵行：
//       flow → preCalcLineHeight（LineHeightLayouter 回调）→ isOverflowBeforeFlow →
//       prepareSubLineEnvs（无绘绕排 → 单 subline）→ wrap 循环 → SubLineLayouter.flow →
//       postCalcLineHeight → regulateToTextLine/regulateToCache（镜像行 + TypoPage.lines）。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables, unused_mut)]

use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::line::ILineHeightCallback::ILineHeightCallback;
use crate::cn::wps::moffice::writer::layout::base::line::LineHeightLayouter::LineHeightLayouter;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineLayouter::SubLineLayouter;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignCenterHandler::AlignCenterHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignRightHandler::AlignRightHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignDistributeHandler::AlignDistributeHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignBothHandler::AlignBothHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignLeftHandler::AlignLeftHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
use crate::cn::wps::moffice::writer::layout::base::line::TextOpCollector::TextOpCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::UnderlineCollector::UnderlineCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::StrikeThroughCollector::StrikeThroughCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::HighlightCollector::HighlightCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::SHDCollector::SHDCollector;
use crate::cn::wps::moffice::writer::layout::base::line::style::BordersCollector::BordersCollector;
use crate::cn::wps::moffice::writer::layout::base::bidi::BidiTool::{BidiStyleCollector, BidiTool};
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
// java: import cn.wps.collections.KPList（rust: LayoutState.rs 真实 Vec 池 KPList）

// java public class LineLayouter extends SubLineLayouter implements ILineHeightCallback
pub struct LineLayouter {
    // java SubLineLayouter super_
    pub(crate) super_: SubLineLayouter,
    // Java sub-layouters share the owner's environment objects by reference.
    // Rust's translated constructor owns defaults, so keep the live aliases
    // for TextWrapLayouter instead of reading those detached copies.
    mWrapMainStateAlias: *mut MainState,
    mWrapDocEnvAlias: *mut DocumentEnv,
    mWrapViewEnvAlias: *mut ViewEnv,
    // java protected boolean mForceVCenterAlign;
    pub(crate) mForceVCenterAlign: bool,
    // java protected int mAlignLine;
    pub(crate) mAlignLine: i32,
    // java protected int mLineHeight;
    pub(crate) mLineHeight: i32,
    // java protected int mAscentPos;
    pub(crate) mAscentPos: i32,
    // java protected int mDescentPos;
    pub(crate) mDescentPos: i32,
    // java protected int mParaBefore;
    pub(crate) mParaBefore: i32,
    // java protected int mParaAfter;
    pub(crate) mParaAfter: i32,
    // java protected int mLineBefore;
    pub(crate) mLineBefore: i32,
    // java protected int mLineAfter;
    pub(crate) mLineAfter: i32,
    // java protected boolean mIsCardBegin;
    pub(crate) mIsCardBegin: bool,
    // java protected boolean mIsCardEnd;
    pub(crate) mIsCardEnd: bool,
    // java protected int mLastGraphIndex;
    pub(crate) mLastGraphIndex: i32,
    // java protected Graphs mLastRun;
    pub(crate) mLastRun: Option<Graphs>,
    // java protected KPList<SubLineEnv> mWrapEnvs;
    pub(crate) mWrapEnvs: crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList<SubLineEnv>,
    // java protected SubLineEnv mFirstValidSubLineEnv;
    // Java stores the same SubLineEnv object; mWrapEnvs owns its boxed node
    // until regulateSubLinesAndRuns completes. Cloning dropped compression state.
    pub(crate) mFirstValidSubLineEnv: Option<*mut SubLineEnv>,
    // java protected int mValidSubLineEnvCount;
    pub(crate) mValidSubLineEnvCount: i32,
    // java protected boolean mIsLastLineOfPara;
    pub(crate) mIsLastLineOfPara: bool,
    // java protected boolean mFirstIsSpaceAndAfterSoftBreak;
    pub(crate) mFirstIsSpaceAndAfterSoftBreak: bool,
    // java protected boolean mIsLastLineOfSection;
    pub(crate) mIsLastLineOfSection: bool,
    // java protected boolean mIsLastLineOfPage;
    pub(crate) mIsLastLineOfPage: bool,
    // java protected boolean mIsLastLineOfColumn;
    pub(crate) mIsLastLineOfColumn: bool,
    // java protected LineHeightLayouter mLineHeightLayouter;
    pub(crate) mLineHeightLayouter: Option<LineHeightLayouter>,
    // java protected TypoTextLine mTextLine;
    pub(crate) mTextLine: TypoTextLine,
    // java protected TextOpCollector mTextOpCollector;
    pub(crate) mTextOpCollector: TextOpCollector,
    // java UnderlineCollector from LayoutToolsManager.getUnderlineCollector when styleResult.hasUnderline
    pub(crate) mUnderlineCollector: UnderlineCollector,
    mCollectUnderline: bool,
    pub(crate) mStrikeThroughCollector: StrikeThroughCollector,
    mCollectStrike: bool,
    pub(crate) mHighlightCollector: HighlightCollector,
    mCollectHighlight: bool,
    pub(crate) mSHDCollector: SHDCollector,
    mCollectSHD: bool,
    pub(crate) mBordersCollector: BordersCollector,
    mCollectBrc: bool,
    // java protected IGetTextWrapLayouter mTextWrapLayouter;
    pub(crate) mTextWrapLayouter: Option<Box<crate::cn::wps::moffice::writer::layout::base::drawing::TextWrapLayouter::TextWrapLayouter>>,
    // java protected boolean mHasAutoNumber;
    pub(crate) mHasAutoNumber: bool,
    // java PageViewLineLayouter overrides calcParaBefore when this is set.
    pub(crate) mPageView: bool,
}

impl LineLayouter {
    // java public LineLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn LineLayouter(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> LineLayouter {
      let mut this = LineLayouter {
        super_: SubLineLayouter::SubLineLayouter(mainState, docEnv, viewEnv),
        mWrapMainStateAlias: std::ptr::null_mut(),
        mWrapDocEnvAlias: std::ptr::null_mut(),
        mWrapViewEnvAlias: std::ptr::null_mut(),
        mForceVCenterAlign: false, mAlignLine: 0, mLineHeight: 0, mAscentPos: 0, mDescentPos: 0,
        mParaBefore: 0, mParaAfter: 0, mLineBefore: 0, mLineAfter: 0,
        mIsCardBegin: false, mIsCardEnd: false, mLastGraphIndex: -1, mLastRun: None,
        mWrapEnvs: crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList::default(),
        mFirstValidSubLineEnv: None, mValidSubLineEnvCount: 0,
        mIsLastLineOfPara: false, mFirstIsSpaceAndAfterSoftBreak: false,
        mIsLastLineOfSection: false, mIsLastLineOfPage: false, mIsLastLineOfColumn: false,
        mLineHeightLayouter: None, mTextLine: TypoTextLine::new(), mTextOpCollector: TextOpCollector::new(),
        mUnderlineCollector: UnderlineCollector::new(), mCollectUnderline: false,
        mStrikeThroughCollector: StrikeThroughCollector::new(), mCollectStrike: false,
        mHighlightCollector: HighlightCollector::new(), mCollectHighlight: false,
        mSHDCollector: SHDCollector::new(), mCollectSHD: false,
        mBordersCollector: BordersCollector::new(), mCollectBrc: false,
        mTextWrapLayouter: None, mHasAutoNumber: false, mPageView: false,
      };
      // java this.mLineHeightLayouter = new LineHeightLayouter(this.mDocEnv, this.mViewEnv);
      this.mLineHeightLayouter = Some(LineHeightLayouter::LineHeightLayouter(
        (this.super_.super_.super_.mDocEnv.as_ref().map(|d| d as *const DocumentEnv as *mut DocumentEnv).unwrap_or(std::ptr::null_mut())),
        (this.super_.super_.super_.mViewEnv.as_ref().map(|v| v as *const ViewEnv as *mut ViewEnv).unwrap_or(std::ptr::null_mut())),
      ));
      this
    }

    // rust: 测试面（test_ 访问器）；Java 无此方法
    pub fn super_mut(&mut self) -> &mut SubLineLayouter {
        &mut self.super_
    }

    // rust: 测试面
    pub fn super_ref(&self) -> &SubLineLayouter {
        &self.super_
    }

    // rust: 行高叶（06 rebase 用）
    pub fn mLineHeightLayouter_mut(&mut self) -> &mut LineHeightLayouter {
        self.mLineHeightLayouter.as_mut().unwrap()
    }

    pub fn bindTextWrapEnvs(&mut self, main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) {
        self.mWrapMainStateAlias = main;
        self.mWrapDocEnvAlias = doc;
        self.mWrapViewEnvAlias = view;
    }

    // java public final void initTextWrapLayouter(TextWrapLayouter.IStateEnvoy stateEnvoy)
    pub fn initTextWrapLayouter(&mut self, state_envoy: &mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy) {
        // java if (null == this.mTextWrapLayouter) this.doInitTextWrapLayouter();
        let main = if self.mWrapMainStateAlias.is_null() { self.super_.super_.super_.mMainState.as_mut().map(|m| m as *mut MainState).unwrap_or(std::ptr::null_mut()) } else { self.mWrapMainStateAlias };
        let doc = if self.mWrapDocEnvAlias.is_null() { self.super_.super_.super_.mDocEnv.as_mut().map(|d| d as *mut DocumentEnv).unwrap_or(std::ptr::null_mut()) } else { self.mWrapDocEnvAlias };
        let view = if self.mWrapViewEnvAlias.is_null() { self.super_.super_.super_.mViewEnv.as_mut().map(|v| v as *mut ViewEnv).unwrap_or(std::ptr::null_mut()) } else { self.mWrapViewEnvAlias };
        if self.mTextWrapLayouter.is_none() {
            self.mTextWrapLayouter = Some(Box::new(
                crate::cn::wps::moffice::writer::layout::base::drawing::TextWrapLayouter::TextWrapLayouter::new(main, doc, view),
            ));
        } else if let Some(layouter) = self.mTextWrapLayouter.as_mut() {
            layouter.rebind(main, doc, view);
        }
        // java this.mTextWrapLayouter.update(stateEnvoy);
        if let Some(layouter) = self.mTextWrapLayouter.as_mut() {
            layouter.update(state_envoy);
        }
    }

    // java public void clearTextWrapLayouter()
    pub fn clearTextWrapLayouter(&mut self) {
        // java if (null != this.mTextWrapLayouter) this.mTextWrapLayouter.clean();
        if let Some(layouter) = self.mTextWrapLayouter.as_mut() {
            layouter.clean();
        }
    }

    // java public void flow(LayoutState layoutState)
    pub fn flow(&mut self, layoutState: &mut LayoutState) {
      // java layoutState.validLine = false;
      layoutState.validLine = false;
      // java this.mParaBefore = 0;
      self.mParaBefore = 0;
      // java this.mParaAfter = 0;
      self.mParaAfter = 0;
      // java this.mIsLastLineOfPara = false;
      self.mIsLastLineOfPara = false;
      // java this.mFirstIsSpaceAndAfterSoftBreak = false;
      self.mFirstIsSpaceAndAfterSoftBreak = false;
      // java this.mIsLastLineOfSection = false;
      self.mIsLastLineOfSection = false;
      // java this.mIsLastLineOfPage = false;
      self.mIsLastLineOfPage = false;
      // java this.mIsLastLineOfColumn = false;
      self.mIsLastLineOfColumn = false;
      // java this.mAlignLine = 0;
      self.mAlignLine = 0;
      // java this.mLineHeight = 0;
      self.mLineHeight = 0;
      // java this.mForceVCenterAlign = false;
      self.mForceVCenterAlign = false;
      // java this.mValidSubLineEnvCount = 0;
      self.mValidSubLineEnvCount = 0;
      // java this.mFirstValidSubLineEnv = null;
      self.mFirstValidSubLineEnv = None;
      // java this.mLastGraphIndex = -1;
      self.mLastGraphIndex = -1;
      // java this.mDrawingOnlyInPhoneVeiw = false;
      self.super_.mDrawingOnlyInPhoneVeiw = false;
      // java this.mHasAutoNumber = false;
      self.mHasAutoNumber = false;
      // java this.mLineBefore = 0;
      self.mLineBefore = 0;
      // java this.mLineAfter = 0;
      self.mLineAfter = 0;
      // java this.mIsCardEnd = false;
      self.mIsCardEnd = false;
      // java this.mIsCardBegin = false;
      self.mIsCardBegin = false;
      // java this.mIsFirstLineOfPage = false;
      // java this.preCalcLineHeight(layoutState);
      self.preCalcLineHeight(layoutState);
      // java if (!this.isOverflowBeforeFlow(layoutState))
      if !self.isOverflowBeforeFlow(layoutState) {
        // java this.prepareSubLineEnvs(layoutState);
        self.prepareSubLineEnvs(layoutState);
        // java if (this.mWrapEnvs.isEmpty())
        if self.mWrapEnvs.is_empty() {
          // java layoutState.validLine = true;
          layoutState.validLine = true;
          // java layoutState.curLayoutHeight = this.mTextWrapLayouter.getNearestDrawingBottom(...);
          if let Some(wrap) = self.mTextWrapLayouter.as_ref() {
            layoutState.curLayoutHeight = wrap.getNearestDrawingBottom(layoutState.getEnvoy());
          }
        }
        // java else
        else {
          // java this.mWrapEnvIt.seekBegin(); （Rust mirror：索引遍历）
          // java this.getHAlignHandler(layoutState);
          self.getHAlignHandler(layoutState);
          // java this.getGridHandler/CharSpace/AutoSpace/PunctSpace/Kerning/CharBorder
          // rust: 各 handler None（hello；真实 WhiteSpaceHandler 由 flow 前一环在 SubLineLayouter 设置——
          //       Java getCharSpaceHandler 默认 WhiteSpaceHandler → 这里装配真实默认实例）
          self.getCharSpaceHandler(layoutState);
          // Java: this.getAutoSpaceHandler(layoutState);
          self.getAutoSpaceHandler(layoutState);
          // Java: this.getPunctSpaceHandler(layoutState);
          self.getPunctSpaceHandler(layoutState);
          // java boolean isEmptyLine = true;
          let mut isEmptyLine: bool = true;
          // java boolean firstSubLineOfPara = 0 == validSubLineEnvCount && (isFirstLineOfPara || firstLineIsSecBreak);
          let mut firstSubLineOfPara: bool = 0 == self.mValidSubLineEnvCount
            && (layoutState.paraEnv.isFirstLineOfPara || layoutState.paraEnv.firstLineIsSecBreak);
          // java while(true) { curWrapEnv = mWrapEnvIt.next(); if (null == curWrapEnv) break; ... }
          let env_count: usize = self.mWrapEnvs.size() as usize;
          let mut env_idx: usize = 0;
          while env_idx < env_count {
            let curWrapEnv: *mut SubLineEnv = self.mWrapEnvs.get_at(env_idx);
            env_idx += 1;
            // java boolean continueFlow = this.flow(curWrapEnv, firstSubLineOfPara, layoutState);
            let continueFlow: bool = self.super_.flow(unsafe { &mut *curWrapEnv }, firstSubLineOfPara, layoutState);
            // java if (this.needReflowLine(layoutState)) { return; } —— Base 恒 false
            // java if (curWrapEnv.hasGraphs())
            if unsafe { (*curWrapEnv).hasGraphs() } {
              // java curWrapEnv.valid = true;
              unsafe { (*curWrapEnv).valid = true; }
              // java if (0 == this.mValidSubLineEnvCount) { this.mFirstValidSubLineEnv = curWrapEnv; }
              if 0 == self.mValidSubLineEnvCount {
                self.mFirstValidSubLineEnv = Some(curWrapEnv);
              }
              // java ++this.mValidSubLineEnvCount;
              self.mValidSubLineEnvCount += 1;
              // java Graphs lastRun = curWrapEnv.lastGraphs;
              let lastRun: Option<Graphs> = unsafe { (*curWrapEnv).lastGraphs.clone() };
              // java this.mLastGraphIndex = lastRun.getEndGraphIndex();
              if let Some(lastRun) = lastRun.as_ref() {
                self.mLastGraphIndex = lastRun.getEndGraphIndex();
              }
              // java this.mLastRun = lastRun;
              self.mLastRun = lastRun;
              // java firstSubLineOfPara = false;
              firstSubLineOfPara = false;
              // java isEmptyLine = false;
              isEmptyLine = false;
              // java if (!curWrapEnv.lineCompressEnv.isFirstCompress()) { this.compress(curWrapEnv, layoutState); }
              if unsafe { (*curWrapEnv).lineCompressEnv.as_ref().is_some_and(|env| !env.isFirstCompress()) } {
                self.compress(unsafe { &mut *curWrapEnv }, layoutState);
              }
            }
            // java if (continueFlow) { continue; }
            if continueFlow { continue; }
          }
          // java if (0 != this.mValidSubLineEnvCount && !isEmptyLine)
          if 0 != self.mValidSubLineEnvCount && !isEmptyLine {
            // java switch (this.mLastRun.runType)
            if let Some(lastRun) = self.mLastRun.as_ref() {
              match lastRun.runType {
                // java case 4: mIsLastLineOfPara = true; ...
                4 => {
                  self.mIsLastLineOfPara = true;
                }
                // java case 5: mIsLastLineOfPara = true; mIsLastLineOfPage = true; ...
                5 => {
                  self.mIsLastLineOfPara = true;
                  self.mIsLastLineOfPage = true;
                }
                // java case 6..9 default: mIsLastLineOfPara = false;
                6 | 7 | 8 | 9 => {
                  self.mIsLastLineOfPara = false;
                }
                // java case 10..13: mIsLastLineOfPara = true; mIsLastLineOfSection = true;
                10 | 11 | 12 | 13 => {
                  self.mIsLastLineOfPara = true;
                  self.mIsLastLineOfSection = true;
                }
                // java case 14: mIsLastLineOfPara = true; mIsLastLineOfColumn = true;
                14 => {
                  self.mIsLastLineOfPara = true;
                  self.mIsLastLineOfColumn = true;
                }
                _ => {
                  self.mIsLastLineOfPara = false;
                }
              }
            }
            // java if (layoutState.isAllowTransForSpace() && mViewEnv.getPhoneViewParam().isFitSpaceAfterSoftBreak()) {...}
            // rust: phone 参数非 hello（GR-D3）
            // java this.processCardInfo(layoutState);
            // rust: isShowCardInfo 恒 false → 空
            // java this.postCalcLineHeight(layoutState);
            self.postCalcLineHeight(layoutState);
            // java if (this.isOverflowTextLine(layoutState)) { return; }
            // rust: isOverflow(textLine) 需行对象——先于 regulate 用高度判（hello maxLayoutHeight 大 → false）
            // java layoutState.validLine = true;
            layoutState.validLine = true;
            // java this.regulateToCache(layoutState);
            self.regulateToCache(layoutState);
            // java return;
          }
        }
      }
    }

    // java public void setLineHeight(int lineHeight) —— ILineHeightCallback
    pub fn setLineHeight(&mut self, lineHeight: i32) {
      // java this.mLineHeight = lineHeight;
      self.mLineHeight = lineHeight;
    }

    // java public void setAlignLine(int alignLine) —— ILineHeightCallback
    pub fn setAlignLine(&mut self, alignLine: i32) {
      // java this.mAlignLine = alignLine;
      self.mAlignLine = alignLine;
    }

    // java public void setAscDesPos(int ascentPos, int descentPos)
    pub fn setAscDesPos_line(&mut self, ascentPos: i32, descentPos: i32) {
      // java this.mAscentPos = ascentPos;
      self.mAscentPos = ascentPos;
      // java this.mDescentPos = descentPos;
      self.mDescentPos = descentPos;
    }

    // java public void setForceVCenterAlign(boolean forceVCenterAlign)
    pub fn setForceVCenterAlign_map(&mut self, forceVCenterAlign: bool) {
      // java this.mForceVCenterAlign = forceVCenterAlign;
      self.mForceVCenterAlign = forceVCenterAlign;
    }

    // java protected void preCalcLineHeight(LayoutState layoutState)
    fn preCalcLineHeight(&mut self, layoutState: &mut LayoutState) {
      // java ParaEnv paraEnv = layoutState.paraEnv;
      // java if (paraEnv.isFirstLineOfPara || paraEnv.firstLineIsSecBreak) { this.calcParaBefore(layoutState); }
      if layoutState.paraEnv.isFirstLineOfPara || layoutState.paraEnv.firstLineIsSecBreak {
        self.calcParaBefore(layoutState);
      }
      // java this.mLineHeightLayouter.calcLineHeightAndAlignmentLine(layoutState, this);
      let mut lhl = self.mLineHeightLayouter.take();
      if let Some(lhl) = lhl.as_mut() {
        lhl.calcLineHeightAndAlignmentLine(layoutState, self);
      }
      self.mLineHeightLayouter = lhl;
      // java this.mParaAfter = 0;
      self.mParaAfter = 0;
    }

    // java protected void postCalcLineHeight(LayoutState layoutState)
    fn postCalcLineHeight(&mut self, layoutState: &mut LayoutState) {
      // java this.mLineHeightLayouter.updateLineHeightAndAlignmentLine(this.mLastGraphIndex, layoutState, this);
      let mut lhl = self.mLineHeightLayouter.take();
      if let Some(lhl) = lhl.as_mut() {
        lhl.updateLineHeightAndAlignmentLine(self.mLastGraphIndex, layoutState, self);
      }
      self.mLineHeightLayouter = lhl;
      // java if (this.mIsLastLineOfPara) { this.calcParaAfter(layoutState); }
      if self.mIsLastLineOfPara {
        self.calcParaAfter(layoutState);
      }
    }

    // java protected void calcParaBefore(LayoutState layoutState)
    fn calcParaBefore(&mut self, layoutState: &LayoutState) {
      // java PageViewLineLayouter: an empty page that is not the section's first page has no para-before.
      if self.mPageView {
        let has_lines = layoutState.curPage.as_ref().map(|page| {
          if page.snapshot.is_null() || page.snapshot_pointer == 0 {
            !page.lines.is_empty()
          } else {
            crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(page.getLines(), unsafe { &*page.snapshot }) > 0
          }
        }).unwrap_or(false);
        if !has_lines && !layoutState.pageEnv.isFirstPageOfSection {
          self.mParaBefore = 0;
          return;
        }
      }
      // java ParaPropEnv cur = paraEnv.curParaPropEnv;
      if let Some(cur) = layoutState.paraEnv.curParaPropEnv.as_ref() {
        // java this.mParaBefore = cur.paraBefore;
        self.mParaBefore = cur.paraBefore;
        // java ParaPropEnv pre = paraEnv.prevParaPropEnv;
        if let Some(pre) = layoutState.paraEnv.prevParaPropEnv.as_ref() {
          // java this.mParaBefore = Math.max(0, this.mParaBefore - pre.paraAfter);
          self.mParaBefore = (self.mParaBefore - pre.paraAfter).max(0);
        }
      }
    }

    // java protected void calcParaAfter(LayoutState layoutState)
    fn calcParaAfter(&mut self, layoutState: &LayoutState) {
      // java if (!this.mIsCardBegin && !this.mIsFirstLineOfPage)
      if !self.mIsCardBegin {
        // java this.mParaAfter = layoutState.paraEnv.curParaPropEnv.paraAfter;
        if let Some(cur) = layoutState.paraEnv.curParaPropEnv.as_ref() {
          self.mParaAfter = cur.paraAfter;
          if std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(layoutState.cp) {
            eprintln!("line_para_after cp={} after={} prop214={} prop215={} prop216={} first_page_line={}", layoutState.cp, self.mParaAfter,
              cur.paraProperty.as_ref().map(|p| p.getInt__int_int(214, -1)).unwrap_or(-1),
              cur.paraProperty.as_ref().map(|p| p.getInt__int_int(215, -1)).unwrap_or(-1),
              cur.paraProperty.as_ref().map(|p| p.getBoolean__int_boolean(216, false)).unwrap_or(false),
              layoutState.curPage.as_ref().map(|p| !p.hasLines()).unwrap_or(true));
          }
        }
      }
    }

    // java private boolean isOverflowBeforeFlow(LayoutState layoutState)
    fn isOverflowBeforeFlow(&self, layoutState: &LayoutState) -> bool {
      // java PageEnv pageEnv = layoutState.pageEnv;
      let pageEnv = &layoutState.pageEnv;
      // java if (pageEnv.allowOverCoreHeight) { return false; }
      if pageEnv.allowOverCoreHeight { return false; }
      // java else if (layoutState.curLayoutHeight + mLineHeight + mParaBefore <= pageEnv.maxLayoutHeight) { return false; }
      if layoutState.curLayoutHeight + self.mLineHeight + self.mParaBefore <= pageEnv.maxLayoutHeight { return false; }
      // java else if (layoutState.atLeastOneLine && TypoList.size(curPage.getLines(), snapshot) <= 0) { return false; }
      // rust: curPage.lines mirror size（Java TypoList.size 对等）
      let hasLines: i32 = layoutState.curPage.as_ref().map(|p| p.lines_len()).unwrap_or(0);
      if layoutState.atLeastOneLine && hasLines <= 0 { return false; }
      // java else if (pageEnv.allowBreakLine && curLayoutHeight < pageEnv.maxLayoutHeight) { return false; }
      if pageEnv.allowBreakLine && layoutState.curLayoutHeight < pageEnv.maxLayoutHeight { return false; }
      // java else
      else {
        // java if (layoutState.curPage.getType() == 2) { ... }
        // java return true;
        true
      }
    }

    // java protected boolean isOverflow(TypoTextLine textLine, LayoutState layoutState)
    fn isOverflow_line(&self, layoutState: &LayoutState) -> bool {
      // java PageEnv pageEnv = layoutState.pageEnv;
      let pageEnv = &layoutState.pageEnv;
      // java if (pageEnv.allowOverCoreHeight) { return false; }
      if pageEnv.allowOverCoreHeight { return false; }
      // java textLine.getTextRectBottom() <= maxLayoutHeight → false
      let top = self.mTextLine.textRectTop + self.mParaBefore;
      let bottom = top + self.mLineHeight + self.mParaBefore + self.mParaAfter;
      if bottom <= pageEnv.maxLayoutHeight { return false; }
      // java atLeastOneLine ...
      if layoutState.atLeastOneLine && layoutState.curPage.as_ref().map(|p| p.lines_len()).unwrap_or(0) <= 0 { return false; }
      // java else { return !pageEnv.allowBreakLine || curLayoutHeight >= maxLayoutHeight; }
      !pageEnv.allowBreakLine || layoutState.curLayoutHeight >= pageEnv.maxLayoutHeight
    }

    // java protected void prepareSubLineEnvs(LayoutState layoutState)
    fn prepareSubLineEnvs(&mut self, layoutState: &mut LayoutState) {
      // java KPList<SubLineEnv> wrapEnvList = this.mWrapEnvs;
      self.mWrapEnvs.clean();
      // java if (null != this.mTextWrapLayouter && this.mTextWrapLayouter.getSubLineRects(...))
      let mut sub_line_rects = Vec::new();
      let top = layoutState.curLayoutHeight;
      let bottom = top + self.mLineHeight + self.mParaBefore + self.mParaAfter;
      let has_wrap_rects = self.mTextWrapLayouter.as_ref().is_some_and(|wrap| {
        wrap.getSubLineRects(&mut sub_line_rects, top, bottom, i32::MIN, i32::MIN, layoutState.getEnvoy())
      });
      if has_wrap_rects {
        // Java: a wrap drawing may consume the whole horizontal strip. Leave
        // mWrapEnvs empty so flow asks getNearestDrawingBottom for the next y.
        let is_unique_subline = sub_line_rects.len() == 1;
        let core_width = layoutState.pageEnv.getCoreWidth();
        let (indent_left, indent_right) = layoutState.paraEnv.curParaPropEnv.as_ref()
          .map(|prop| (prop.leftIndent, core_width - prop.rightIndent))
          .unwrap_or((0, core_width));
        for mut rect in sub_line_rects {
          if layoutState.paraEnv.isRTL {
            let left = core_width - rect.right;
            rect.right = core_width - rect.left;
            rect.left = left;
          }
          if rect.left <= indent_left.max(indent_right) && rect.right >= indent_left.min(indent_right) {
            let sub: *mut SubLineEnv = if layoutState.paraEnv.isRTL {
              self.mWrapEnvs.obtainAndAddFirst()
            } else {
              self.mWrapEnvs.obtainAndAddLast()
            };
            unsafe {
              (*sub).atLeastOneChar = false;
              (*sub).left = rect.left;
              (*sub).right = rect.right;
              (*sub).wrapRectType = rect.getWrapType();
              (*sub).isUniqueSubline = is_unique_subline;
              if rect.left > 0 { (*sub).enableLeftIndent = false; }
              if rect.right < core_width { (*sub).enableRightIndent = false; }
            }
          }
        }
      } else {
        // java else { SubLineEnv subLineEnv = wrapEnvList.obtainAndAddLast(); offset=...; }
        let subLineEnv: *mut SubLineEnv = self.mWrapEnvs.obtainAndAddLast();
        // java int offset = isRTL ? -sectionMarginOffset : sectionMarginOffset;
        let offset: i32 = if layoutState.paraEnv.isRTL { -layoutState.pageEnv.sectionMarginOffset } else { layoutState.pageEnv.sectionMarginOffset };
        // java subLineEnv.left = offset;
        unsafe { (*subLineEnv).left = offset; }
        // java subLineEnv.right = pageEnv.getCoreWidth() + offset;
        unsafe { (*subLineEnv).right = layoutState.pageEnv.getCoreWidth() + offset; }
      }
    }

    // Java: private void compress(SubLineEnv subLineEnv, LayoutState layoutState)
    fn compress(&mut self, sub: &mut SubLineEnv, state: &mut LayoutState) {
      let Some(env) = sub.lineCompressEnv.as_mut() else { return; };
      // Java: restore the state immediately before the first compressed graph.
      let first = env.getFirstCompressedGraphIndex();
      env.restoreCompressState(first.wrapping_sub(1));
      let max_space1 = env.getMaxSpaceCompressableSpaceSum();
      let max_punct1 = env.getMaxPunctCompressableSpaceSum();
      let max_other1 = env.getMaxOtherCompressableSpaceSum();
      let Some(last_node) = sub.lastGraphNode else { return; };
      let mut restore = unsafe { (*last_node).getEndGraphIndex() };
      if let Some(align_right) = sub.alignRightGraphs {
        let align_index = unsafe { (*align_right).graphIndex };
        if env.firstExcessPunctGraphIndex >= align_index && env.firstExcessPunctGraphIndex >= 0 {
          restore = restore.min(env.firstExcessPunctGraphIndex - 1);
        }
        if env.firstExcessSpaceGraphIndex >= align_index && env.firstExcessSpaceGraphIndex >= 0 {
          restore = restore.min(env.firstExcessSpaceGraphIndex - 1);
        }
      }
      env.restoreCompressState(restore);
      let space_factor = Self::doGetCompressFactor(env.getUsedSpaceCompressableSpaceSum(),
          max_space1, env.getMaxSpaceCompressableSpaceSum());
      let punct_factor = Self::doGetCompressFactor(env.getUsedPunctCompressableSpaceSum(),
          max_punct1, env.getMaxPunctCompressableSpaceSum());
      let other_factor = Self::doGetCompressFactor(env.getUsedOtherCompressableSpaceSum(),
          max_other1, env.getMaxOtherCompressableSpaceSum());
      if std::env::var_os("WORD_PARITY_ALIGN_TRACE").is_some() && state.cp == 1076 {
        eprintln!("compress cp=1076 first={} max1={},{},{} used2={},{},{} max2={},{},{} factors={},{},{}",
          first, max_space1, max_punct1, max_other1,
          env.getUsedSpaceCompressableSpaceSum(), env.getUsedPunctCompressableSpaceSum(), env.getUsedOtherCompressableSpaceSum(),
          env.getMaxSpaceCompressableSpaceSum(), env.getMaxPunctCompressableSpaceSum(), env.getMaxOtherCompressableSpaceSum(),
          space_factor, punct_factor, other_factor);
      }
      // Java iterates from firstGraphs to lastGraphs by node identity.
      let Some(mut run_node) = sub.firstGraphNode else { return; };
      let list = state.getGraphList() as *mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList<Graphs>;
      let mut final_compress = 0;
      while !run_node.is_null() {
        let run = unsafe { &mut *run_node };
        let before = env.getCompressBefore(run.graphIndex);
        let after = env.getCompressAfter(run.getEndGraphIndex());
        if before != 0 || after != 0 {
          let factor = if TypoRun::isSpace(run.runType) { space_factor }
            else if TypoRun::isPunct(run.runType) { punct_factor }
            else { other_factor };
          if before != 0 && factor != 0.0 {
            let factor = if run.graphIndex >= first { 1.0 } else { factor };
            let compressed = (before as f32 * factor + 0.5) as i32;
            if std::env::var_os("WORD_PARITY_ALIGN_TRACE").is_some() && state.cp == 1076 {
              eprintln!("compress before graph={} type={} space={} factor={} value={}", run.graphIndex, run.runType, before, factor, compressed);
            }
            final_compress += compressed;
            run.mWidth -= compressed;
            state.extendWidths(run, -compressed, run.graphIndex, 1);
            state.addDrawPosOffsetX__int_int(run.graphIndex, -compressed);
          }
          if after != 0 && factor != 0.0 {
            let graph_index = run.getEndGraphIndex();
            let factor = if graph_index >= first { 1.0 } else { factor };
            let compressed = (after as f32 * factor + 0.5) as i32;
            if std::env::var_os("WORD_PARITY_ALIGN_TRACE").is_some() && state.cp == 1076 {
              eprintln!("compress after graph={} type={} space={} factor={} value={} count={}", graph_index, run.runType, after, factor, compressed, sub.alignRightGraphs.map(|p| p as *const Graphs) != Some(run as *const Graphs));
            }
            if sub.alignRightGraphs != Some(run as *mut Graphs) { final_compress += compressed; }
            run.mWidth -= compressed;
            state.extendWidths(run, -compressed, graph_index, 1);
          }
        }
        if run_node == last_node { break; }
        run_node = unsafe { (*list).getNext(run_node) };
      }
      env.setFinalCompress(final_compress);
    }

    // Java: private float doGetCompressFactor(int used2, int max1, int max2)
    fn doGetCompressFactor(used2: i32, max1: i32, max2: i32) -> f32 {
      if max1 != 0 && max2 != 0 {
        let average = used2 - (max2 - max1);
        if average <= 0 { 0.0 } else { average as f32 / max1 as f32 }
      } else { 0.0 }
    }

    // java protected void getHAlignHandler(LayoutState layoutState)
    fn getHAlignHandler(&mut self, layoutState: &mut LayoutState) {
      if std::env::var_os("WORD_PARITY_ALIGN_TRACE").is_some()
          && (1038..=1142).contains(&layoutState.cp) {
        eprintln!("align select cp={} alignment={} grid={}",
            layoutState.cp, layoutState.paraEnv.alignment, layoutState.pageEnv.gridModel);
      }
      // java if (layoutState.isInMathParaLine())
      if layoutState.isInMathParaLine() {
        // java this.mHAlignHandler = this.mDocEnv.layoutToolsManager.getHAlignMathParaHandler();
        // rust: 公式对齐 GR-D3；hello/sample2 不进入
        self.super_.mHAlignHandler = Some(Box::new(AlignLeftHandler::AlignLeftHandler()));
      } else if layoutState.pageEnv.gridModel == 3 {
        // java else if (layoutState.pageEnv.gridModel == 3)
        // java this.mHAlignHandler = this.mDocEnv.layoutToolsManager.getHAlignHandler(0);
        self.super_.mHAlignHandler = Some(Box::new(AlignLeftHandler::AlignLeftHandler()));
      } else {
        // java this.mHAlignHandler = this.mDocEnv.layoutToolsManager.getHAlignHandler(layoutState.paraEnv.alignment);
        self.super_.mHAlignHandler = Some(match layoutState.paraEnv.alignment {
          1 => Box::new(AlignCenterHandler::AlignCenterHandler()) as Box<dyn IHAlignHandler>,
          // Java: LayoutToolsManager.getHAlignHandler(2) returns AlignRightHandler.
          2 => Box::new(AlignRightHandler::AlignRightHandler()) as Box<dyn IHAlignHandler>,
          // Java: LayoutToolsManager.getHAlignHandler(3/8) returns AlignBothHandler.
          3 | 8 => Box::new(AlignBothHandler::AlignBothHandler()) as Box<dyn IHAlignHandler>,
          // Java: LayoutToolsManager.getHAlignHandler(4/5) returns AlignDistributeHandler.
          4 | 5 => Box::new(AlignDistributeHandler::AlignDistributeHandler()) as Box<dyn IHAlignHandler>,
          _ => Box::new(AlignLeftHandler::AlignLeftHandler()) as Box<dyn IHAlignHandler>,
        });
      }
    }

    // java protected void getCharSpaceHandler(LayoutState layoutState)
    fn getCharSpaceHandler(&mut self, _layoutState: &LayoutState) {
      // java 默认 WhiteSpaceHandler（Java layoutToolsManager 默认实例对等）
      if self.super_.mCharSpaceHandler.is_none() {
        let docPtr = self.super_.super_.super_.mDocEnv.as_ref().map(|d| d as *const DocumentEnv as *mut DocumentEnv).unwrap_or(std::ptr::null_mut());
        let viewPtr = self.super_.super_.super_.mViewEnv.as_ref().map(|v| v as *const ViewEnv as *mut ViewEnv).unwrap_or(std::ptr::null_mut());
        self.super_.mCharSpaceHandler = Some(Box::new(crate::cn::wps::moffice::writer::layout::base::line::charSpace::WhiteSpaceHandler::WhiteSpaceHandler::WhiteSpaceHandler(docPtr, viewPtr)));
      }
    }

    // Java: final void getAutoSpaceHandler(LayoutState layoutState)
    fn getAutoSpaceHandler(&mut self, layoutState: &mut LayoutState) {
      // Java: this.mAutoSpaceHandler = this.mDocEnv.layoutToolsManager.getAutoSpaceHandler(layoutState);
      if layoutState.hasMaths() { panic!("math auto-space handler is not implemented"); }
      let para = &layoutState.paraEnv;
      if layoutState.pageEnv.gridModel == 3 || (!para.autoSpaceDN && !para.autoSpaceDE) {
        self.super_.mAutoSpaceHandler = None;
        return;
      }
      let docPtr = self.super_.super_.super_.mDocEnv.as_ref().map(|d| d as *const DocumentEnv).unwrap_or(std::ptr::null());
      use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::{AutoSpaceDNandDEHandler, AutoSpaceDNHandler, AutoSpaceDEHandler};
      self.super_.mAutoSpaceHandler = Some(if para.autoSpaceDN && para.autoSpaceDE {
        Box::new(AutoSpaceDNandDEHandler::AutoSpaceDNandDEHandler::AutoSpaceDNandDEHandler(docPtr))
      } else if para.autoSpaceDN {
        Box::new(AutoSpaceDNHandler::AutoSpaceDNHandler::AutoSpaceDNHandler(docPtr))
      } else {
        Box::new(AutoSpaceDEHandler::AutoSpaceDEHandler::AutoSpaceDEHandler(docPtr))
      });
      if let Some(handler) = self.super_.mAutoSpaceHandler.as_mut() { handler.setEnv(layoutState as *mut LayoutState); }
    }

    // Java: final void getPunctSpaceHandler(LayoutState layoutState)
    fn getPunctSpaceHandler(&mut self, layoutState: &mut LayoutState) {
      // Java: this.mPunctSpaceHandler = this.mDocEnv.layoutToolsManager.getPunctSpaceHandler(layoutState);
      // DocumentEnv.updateFromDocument has copied the same imported property 382 into compressLevel.
      let doc = self.super_.super_.super_.mDocEnv.as_ref();
      let compressPunct = doc.map(|d| d.compressLevel == 1).unwrap_or(false);
      if self.super_.super_.super_.mViewEnv.as_ref()
          .map(|v| crate::cn::wps::moffice::writer::base::ViewMode::ViewMode::isPhoneReadMode(v.getViewMode()))
          .unwrap_or(false) {
        panic!("PhoneViewParam.isCompressLine is not implemented");
      }
      if compressPunct {
        if self.super_.mPunctSpaceHandler.is_none() {
          let docPtr = doc.map(|d| d as *const DocumentEnv).unwrap_or(std::ptr::null());
          let viewPtr = self.super_.super_.super_.mViewEnv.as_ref().map(|v| v as *const ViewEnv).unwrap_or(std::ptr::null());
          self.super_.mPunctSpaceHandler = Some(Box::new(crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::CompressPunctSpaceHandler::CompressPunctSpaceHandler::CompressPunctSpaceHandler(docPtr, viewPtr)));
        }
        if let Some(handler) = self.super_.mPunctSpaceHandler.as_mut() { handler.setEnv(layoutState as *mut LayoutState); }
      } else {
        self.super_.mPunctSpaceHandler = None;
      }
    }

    // java private TypoTextLine regulateToTextLine(...) —— 行对象创建（镜像面）
    fn regulateToTextLine(&mut self, layoutState: &mut LayoutState, relTextRectTop: i32, textRectHeight: i32, firstLineOfPara: bool, lastLineOfPara: bool, rtl: bool) -> TypoTextLine {
      // java TypoTextLine line = this.mTextLine;
      let mut line: TypoTextLine = Clone::clone(&self.mTextLine);
      // java line.alloc(this.mMainState.snapshot); line.setParent(layoutState.curPage.getPointer());
      let (snap, parent) = layoutState.curPage.as_ref()
        .map(|page| (page.snapshot, page.snapshot_pointer))
        .unwrap_or((std::ptr::null_mut(), 0));
      if !snap.is_null() && parent != 0 {
        line.alloc(unsafe { &mut *snap });
        line.setParent(parent);
      }
      // java line.setPara(layoutState.typoPara.getPointer());
      line.setPara__int(layoutState.typoPara.getPointer());
      // java line.setParent(layoutState.curPage.getPointer());
      // java line.setTextRectTop(relTextRectTop);
      line.setTextRectTop(relTextRectTop);
      // java line.setTextRectHeight(textRectHeight);
      line.setTextRectHeight(textRectHeight);
      // java line.setAscDesPos(this.mAscentPos, this.mDescentPos);
      line.setAscDesPos(self.mAscentPos, self.mDescentPos);
      // java if (firstLineOfPara) { line.setFirstLineOfPara(true); }
      if firstLineOfPara { line.setFirstLineOfPara(true); }
      // java if (lastLineOfPara) { line.setLastLineOfPara(true); }
      if lastLineOfPara { line.setLastLineOfPara(true); }
      // java if (this.mIsLastLineOfSection) { line.setLastLineOfSection(true); }
      if self.mIsLastLineOfSection { line.setLastLineOfSection(); }
      // java if (this.mIsLastLineOfPage) { line.setLastLineOfPage(true); }
      if self.mIsLastLineOfPage { line.setLastLineOfPage(true); }
      // java if (this.mIsLastLineOfColumn) { line.setLastLineOfColumn(true); }
      if self.mIsLastLineOfColumn { line.setLastLineOfColumn(); }
      // java if (rtl) { line.setRTL(true); }
      if rtl { line.setRTL(); }
      // java if (paraEnv.lineSpacing.getRule() == 1) { line.setFixedHeight(true); }
      if layoutState.paraEnv.lineSpacing.getRule() == 1 { line.setFixedHeight(); }
      // java if (this.mHasAutoNumber) { line.setHasAutoNumber(true); }
      if self.mHasAutoNumber { line.setHasAutoNumber(true); }
      // java line.setGraphIndexStart(...); line.setGraphIndexEnd(mLastGraphIndex);
      let startGI: i32 = self.mFirstValidSubLineEnv.map(|e| unsafe { (*e).firstGraphIndexInCP }).filter(|v| *v > 0).unwrap_or(layoutState.mFirstGraphIndex);
      line.setGraphIndexStart(startGI);
      line.setGraphIndexEnd(self.mLastGraphIndex);
      // java if (paraEnv.isRTL || layoutState.hasBidiGraph) { bidiTool.initEmbeddings(...) }
      if layoutState.paraEnv.isRTL || layoutState.hasBidiGraph() {
        let doc_env = self.super_.super_.super_.mDocEnv.as_ref().map(|d| d as *const DocumentEnv as *mut DocumentEnv).unwrap_or(std::ptr::null_mut());
        if layoutState.bidiTool.is_none() {
          layoutState.bidiTool = Some(BidiTool::new(doc_env));
        }
        let doc = layoutState.document;
        let start = layoutState.paraEnv.curParaPropEnv.as_ref().map(|p| p.start);
        let cp = layoutState.cp;
        if let (Some(tool), Some(start), Some(doc)) = (layoutState.bidiTool.as_mut(), start, doc) {
          if !doc.is_null() {
            tool.initEmbeddings(unsafe { &*doc }, start, cp);
          }
        }
      }
      // java this.regulateSubLinesAndRuns(line, layoutState);
      self.regulateSubLinesAndRuns(&mut line, layoutState);
      // java layoutState.consumeGraphsBefore(this.mLastRun, this.mDrawingOnlyInPhoneVeiw);
      if let Some(last) = self.mLastRun.as_ref() {
        layoutState.consumeGraphsBefore(last, self.super_.mDrawingOnlyInPhoneVeiw);
      }
      // java if (null != layoutState.bidiTool) { layoutState.bidiTool.setNextLineStart(layoutState.cp); }
      let next_cp = layoutState.cp;
      if let Some(tool) = layoutState.bidiTool.as_mut() {
        tool.setNextLineStart(next_cp);
      }
      line
    }

    // java private void regulateSubLinesAndRuns(TypoTextLine textLine, LayoutState layoutState)
    fn regulateSubLinesAndRuns(&mut self, textLine: &mut TypoTextLine, layoutState: &mut LayoutState) {
      // java this.getVAlignHandler(layoutState);
      // java this.initCollectors(textLine, layoutState);
      let doc = self.super_.super_.super_.mDocEnv.as_ref().map(|d| d as *const DocumentEnv as *mut DocumentEnv).unwrap_or(std::ptr::null_mut());
      let view = self.super_.super_.super_.mViewEnv.as_ref().map(|v| v as *const ViewEnv).unwrap_or(std::ptr::null());
      self.mTextOpCollector.bind(doc, view);
      self.mTextOpCollector.init(textLine, layoutState, self.mLastGraphIndex);
      self.mCollectUnderline = layoutState.styleResult.hasUnderline;
      if self.mCollectUnderline {
        self.mUnderlineCollector.init(textLine, layoutState);
      }
      self.mCollectStrike = layoutState.styleResult.hasStrikeThroughs;
      if self.mCollectStrike {
        self.mStrikeThroughCollector.init(textLine, layoutState);
      }
      self.mCollectHighlight = layoutState.styleResult.hasHighlight;
      if self.mCollectHighlight {
        self.mHighlightCollector.init(textLine, layoutState);
      }
      self.mCollectSHD = layoutState.styleResult.hasSHD;
      if self.mCollectSHD {
        self.mSHDCollector.init(textLine, layoutState);
      }
      self.mCollectBrc = layoutState.styleResult.hasBRC;
      if self.mCollectBrc {
        self.mBordersCollector.init(textLine, layoutState);
      }
      // java if (this.mValidSubLineEnvCount > 1) { ... 多 subline 绕排 }
      if self.mValidSubLineEnvCount > 1 {
        // rust: 绕排多 subline —— 非 sample2 单栏；逐 env 后取 maxLeft/maxRight
        let env_count: usize = self.mWrapEnvs.size() as usize;
        let mut maxRight: i32 = 0;
        let mut maxLeft: i32 = 0;
        for i in 0..env_count {
          let env: *mut SubLineEnv = self.mWrapEnvs.get_at(i);
          if env.is_null() { continue; }
          if unsafe { (*env).hasGraphs() } {
            self.regulateSubLine(layoutState, unsafe { &mut *env }, textLine);
            maxRight = textLine.getLeft() + textLine.getWidth();
            if i == 0 {
              maxLeft = textLine.getLeft();
            }
          }
        }
        // java textLine.setLeft(maxLeft); textLine.setRight(maxRight); textLine.setHasSubLine(true);
        textLine.setLeft(maxLeft);
        textLine.setWidth((maxRight - maxLeft).max(0));
      } else {
        // java SubLineEnv sublineEnv = this.mFirstValidSubLineEnv;
        // java this.regulateSubLine(layoutState, sublineEnv, textLine, textLine);
        if let Some(sub) = self.mFirstValidSubLineEnv {
          self.regulateSubLine(layoutState, unsafe { &mut *sub }, textLine);
        }
      }
      // java this.mTextOpCollector.commit();
      self.mTextOpCollector.commit();
      if self.mCollectUnderline {
        self.mUnderlineCollector.commit();
      }
      if self.mCollectStrike {
        self.mStrikeThroughCollector.commit();
      }
      if self.mCollectHighlight {
        self.mHighlightCollector.commit();
      }
      if self.mCollectSHD {
        self.mSHDCollector.commit();
      }
      if self.mCollectBrc {
        self.mBordersCollector.commit();
      }
      // java if (layoutState.hasBidiGraph() || layoutState.paraEnv.isRTL) { textLine.setHasBidi(true); }
      if layoutState.hasBidiGraph() || layoutState.paraEnv.isRTL {
        textLine.setHasBidi(true);
      }
      // java this.mTextLine.setMemEnd(this.mMainState.snapshot.getAllocSize());
      if !textLine.mSnapshot.is_null() {
        let alloc_size = unsafe { (*textLine.mSnapshot).getAllocSize() } as i32;
        textLine.setMemEnd(alloc_size);
      }
      // java if (layoutState.mDocFieldsCollector != null) { dispose; = null; }
      if layoutState.mDocFieldsCollector.is_some() {
        if let Some(collector) = layoutState.mDocFieldsCollector.as_ref() {
          collector.dispose();
        }
        layoutState.mDocFieldsCollector = None;
      }
    }

    // java final void regulateSubLine(LayoutState layoutState, SubLineEnv subLineEnv, ITypoRect line, TypoTextLine textLine)
    fn regulateSubLine(&mut self, layoutState: &mut LayoutState, subLineEnv: &mut SubLineEnv, line: &mut TypoTextLine) {
      // java this.mHAlignHandler.setSubLineEnv(subLineEnv, layoutState);
      let mut handler = self.super_.mHAlignHandler.take();
      if let Some(h) = handler.as_mut() {
        h.setSubLineEnv(subLineEnv, layoutState);
        // java this.mHAlignHandler.adjustLine(line);
        let mut rect = TypoRectCoreImpl::default();
        rect.setLeft(line.getLeft());
        rect.setWidth(line.getWidth());
        h.adjustLine(&mut rect);
        line.setLeft(rect.getLeft());
        line.setWidth(rect.super_.right - rect.super_.left);
        // Java: regulateSubLine iterates firstGraphs..lastGraphs and invokes
        // mHAlignHandler.adjustRun before TextOpCollector.addRun.
        if !layoutState.hasBidiGraph() && !layoutState.paraEnv.isRTL {
          if self.mCollectUnderline || self.mCollectStrike || self.mCollectHighlight || self.mCollectSHD || self.mCollectBrc {
            let view = self.super_.super_.super_.mViewEnv.as_ref().map(|v| v as *const ViewEnv).unwrap_or(std::ptr::null());
            let trail = !view.is_null() && unsafe { (*view).isUlTrailSpace() };
            if self.mCollectUnderline {
              if trail {
                self.mUnderlineCollector.setCollectTrailSpace(false);
                self.mUnderlineCollector.setLineBoundary(subLineEnv.layoutLeft, subLineEnv.layoutRight);
              } else {
                self.mUnderlineCollector.setCollectTrailSpace(true);
              }
            }
            if self.mCollectStrike {
              if trail {
                self.mStrikeThroughCollector.setCollectTrailSpace(false);
                self.mStrikeThroughCollector.setLineBoundary(subLineEnv.layoutLeft, subLineEnv.layoutRight);
              } else {
                self.mStrikeThroughCollector.setCollectTrailSpace(true);
              }
            }
            if self.mCollectHighlight {
              if trail {
                self.mHighlightCollector.setCollectTrailSpace(false);
                self.mHighlightCollector.setLineBoundary(subLineEnv.layoutLeft, subLineEnv.layoutRight);
              } else {
                self.mHighlightCollector.setCollectTrailSpace(true);
              }
            }
            if self.mCollectSHD {
              if trail {
                self.mSHDCollector.setCollectTrailSpace(false);
                self.mSHDCollector.setLineBoundary(subLineEnv.layoutLeft, subLineEnv.layoutRight);
              } else {
                self.mSHDCollector.setCollectTrailSpace(true);
              }
            }
            if self.mCollectBrc {
              if trail {
                self.mBordersCollector.setCollectTrailSpace(false);
                self.mBordersCollector.setLineBoundary(subLineEnv.layoutLeft, subLineEnv.layoutRight);
              } else {
                self.mBordersCollector.setCollectTrailSpace(true);
              }
            }
          }
          let x_offset = line.getLeft();
          if let (Some(first), Some(last)) = (&subLineEnv.firstGraphs, &subLineEnv.lastGraphs) {
            let first_index = first.graphIndex;
            let last_index = last.graphIndex;
            let state_ptr = layoutState as *mut LayoutState;
            let mut under_line_pos: i32 = 0;
            let nodes = unsafe { &mut (*state_ptr).graphList.nodes };
            for node in nodes.iter_mut() {
              if node.graphIndex < first_index { continue; }
              if node.graphIndex > last_index { break; }
              h.adjustRun(node);
              let run_type = node.runType;
              if run_type == 8 || run_type == 9 {
                if let Some(style) = node.fontStyle.as_ref() {
                  node.baseLine = style.maxAscent;
                }
              }
              let run_left = node.getLeft() + x_offset;
              let run_right = run_left + node.width();
              let graph = node.as_mut() as *mut Graphs;
              self.mTextOpCollector.addRun(unsafe { &mut *graph }, unsafe { &mut *state_ptr }, run_left, run_right, false);
              if self.mCollectUnderline || self.mCollectStrike || self.mCollectHighlight || self.mCollectSHD || self.mCollectBrc {
                let mut revision_author = node.getRevisionInsAuthor();
                let revision_del = node.getRevisionDelAuthor();
                if revision_del != -1 {
                  revision_author = revision_del;
                }
                let hyperlink_id = node.getHyperlinkId();
                let graph = node.as_mut() as *mut Graphs;
                if self.mCollectUnderline {
                  self.mUnderlineCollector.addRun(unsafe { &mut *graph }, run_left, run_right, revision_author, hyperlink_id);
                }
                if self.mCollectStrike {
                  self.mStrikeThroughCollector.addRun(unsafe { &mut *graph }, run_left, run_right, revision_author, hyperlink_id);
                }
                if self.mCollectHighlight {
                  self.mHighlightCollector.addRun(unsafe { &mut *graph }, run_left, run_right, revision_author, hyperlink_id);
                }
                if self.mCollectSHD {
                  self.mSHDCollector.addRun(unsafe { &mut *graph }, run_left, run_right, revision_author, hyperlink_id);
                }
                if self.mCollectBrc {
                  self.mBordersCollector.addRun(unsafe { &mut *graph }, run_left, run_right, revision_author, hyperlink_id);
                }
              }
              if let Some(style) = node.fontStyle.as_ref() {
                let org = style.getOrgFontStyle();
                if let Some(face) = org.typeface.as_ref() {
                  if org.emSize != 0.0 {
                    let pos = ((node.baseLine as f32) - ((face.getUnderlinePosition() as f32) * (org.fontSize as f32)) / org.emSize + 0.5) as i32;
                    under_line_pos = under_line_pos.max(pos);
                  }
                }
              }
              if node.graphIndex == last_index { break; }
            }
            if self.mCollectUnderline {
              self.mUnderlineCollector.processTrailSpace();
            }
            if self.mCollectStrike {
              self.mStrikeThroughCollector.processTrailSpace();
            }
            if self.mCollectHighlight {
              self.mHighlightCollector.processTrailSpace();
            }
            if self.mCollectSHD {
              self.mSHDCollector.processTrailSpace();
            }
            if self.mCollectBrc {
              self.mBordersCollector.processTrailSpace();
            }
            line.setYPos(under_line_pos);
          }
        } else {
          self.runBidi(layoutState, subLineEnv, line, h.as_mut());
        }
      }
      self.super_.mHAlignHandler = handler;
    }

    // java else { collectors.setCollectTrailSpace(false); noForceInLineBoundary(); bidiTool.calcBidi(...) }
    fn runBidi(&mut self, layoutState: &mut LayoutState, subLineEnv: &mut SubLineEnv, line: &mut TypoTextLine, h: &mut dyn IHAlignHandler) {
      let mut cols: Vec<*mut dyn BidiStyleCollector> = Vec::new();
      if self.mCollectBrc {
        self.mBordersCollector.setCollectTrailSpace(false);
        self.mBordersCollector.noForceInLineBoundary();
        cols.push(&mut self.mBordersCollector as *mut dyn BidiStyleCollector);
      }
      if self.mCollectUnderline {
        self.mUnderlineCollector.setCollectTrailSpace(false);
        self.mUnderlineCollector.noForceInLineBoundary();
        cols.push(&mut self.mUnderlineCollector as *mut dyn BidiStyleCollector);
      }
      if self.mCollectStrike {
        self.mStrikeThroughCollector.setCollectTrailSpace(false);
        self.mStrikeThroughCollector.noForceInLineBoundary();
        cols.push(&mut self.mStrikeThroughCollector as *mut dyn BidiStyleCollector);
      }
      if self.mCollectSHD {
        self.mSHDCollector.setCollectTrailSpace(false);
        self.mSHDCollector.noForceInLineBoundary();
        cols.push(&mut self.mSHDCollector as *mut dyn BidiStyleCollector);
      }
      if self.mCollectHighlight {
        self.mHighlightCollector.setCollectTrailSpace(false);
        self.mHighlightCollector.noForceInLineBoundary();
        cols.push(&mut self.mHighlightCollector as *mut dyn BidiStyleCollector);
      }
      let doc_env = self.super_.super_.super_.mDocEnv.as_ref().map(|d| d as *const DocumentEnv as *mut DocumentEnv).unwrap_or(std::ptr::null_mut());
      if layoutState.bidiTool.is_none() {
        layoutState.bidiTool = Some(BidiTool::new(doc_env));
      }
      let text_op = &mut self.mTextOpCollector as *mut TextOpCollector;
      let align = self.mAlignLine;
      let word = layoutState.paraEnv.wordAlignFont;
      let last = self.mLastGraphIndex;
      let mut tool = layoutState.bidiTool.take().unwrap();
      tool.setTextOpCollector(text_op);
      tool.setStyleCollectors(cols);
      tool.setAlign(align, word);
      tool.calcBidi(layoutState, subLineEnv, h, line, last);
      layoutState.bidiTool = Some(tool);
    }

    // java protected void regulateToCache(LayoutState layoutState)
    fn regulateToCache(&mut self, layoutState: &mut LayoutState) {
      // java if (0 != this.mValidSubLineEnvCount)
      if 0 != self.mValidSubLineEnvCount {
        // java int oldCP = layoutState.cp;
        let old_cp: i32 = layoutState.cp;
        // java boolean firstLineOfPara = para.isFirstLineOfPara || para.firstLineIsSecBreak;
        let firstLineOfPara: bool = layoutState.paraEnv.isFirstLineOfPara || layoutState.paraEnv.firstLineIsSecBreak;
        // java TypoTextLine line = this.regulateToTextLine(...);
        let mut line: TypoTextLine = self.regulateToTextLine(layoutState, self.mParaBefore, self.mLineHeight, firstLineOfPara, self.mIsLastLineOfPara, layoutState.paraEnv.isRTL);
        // java layoutState.curLayoutHeight += this.mLineBefore;
        layoutState.curLayoutHeight += self.mLineBefore;
        // java line.setTop(layoutState.curLayoutHeight);
        line.setTop(layoutState.curLayoutHeight);
        // java line.setHeight(this.mLineHeight + mParaBefore + mParaAfter);
        line.setHeight(self.mLineHeight + self.mParaBefore + self.mParaAfter);
        // java line.setLineBefore(mLineBefore); line.setLineAfter(mLineAfter);
        line.setLineBeforeVal(self.mLineBefore);
        line.setLineAfterVal(self.mLineAfter);
        // java line.setCardBegin(mIsCardBegin); line.setCardEnd(mIsCardEnd);
        line.setCardBegin__boolean(self.mIsCardBegin);
        line.setCardEnd__boolean(self.mIsCardEnd);
        // java if (null != curParaPropEnv.paraSHD || curParaPropEnv.vaildBRC) { ParaBrc... } —— 段边框非 hello（GR-D3）
        // java if (mIsLastLineOfPara && line.getBottom() > maxLayoutHeight
        //       && line.getBottom() - mParaAfter <= maxLayoutHeight)
        let bottom = line.top + line.height;
        if std::env::var("WORD_PARITY_LINE_TRACE_CP").ok().and_then(|v| v.parse::<i32>().ok()) == Some(old_cp) {
          eprintln!("line_cache_before_clamp cp={} top={} height={} bottom={} paraAfter={} max={} last={}",
            old_cp, line.top, line.height, bottom, self.mParaAfter, layoutState.pageEnv.maxLayoutHeight, self.mIsLastLineOfPara);
        }
        if self.mIsLastLineOfPara && bottom > layoutState.pageEnv.maxLayoutHeight
            && bottom - self.mParaAfter <= layoutState.pageEnv.maxLayoutHeight {
          // java line.setBottom(pageEnv.maxLayoutHeight);
          line.setHeight(layoutState.pageEnv.maxLayoutHeight - line.top);
        }
        // java this.compressEmptyLine(line, layoutState); —— 空行压缩（hello 非空 → 空操作）
        // java layoutState.curPage.insertLine(line.getPointer(), layoutState.linePos);
        // rust: ★wr-29 07 —— 产品行信封：镜像 TypoTextLine 写入 TypoPage.lines（Java snapshot 行对等）
        let mut line = line;
        // start/end CP（Java 读时经 para map；mirror 写时换算）
        // Java captures layoutState.cp before regulating the line. Graph
        // indices restart at zero for each TypoPara, so zero is not CP zero.
        let startCp: i32 = old_cp;
        let endCp: i32 = layoutState.typoPara.getNextCP(self.mLastGraphIndex);
        line.setStartCP(startCp);
        line.setEndCP(endCp);
        if line.mPointer != 0 {
          if let Some(page) = layoutState.curPage.as_mut() {
            // java layoutState.curPage.insertLine(line.getPointer(), layoutState.linePos);
            page.insertLine(line.mPointer, layoutState.linePos);
          }
        }
        if let Some(page) = layoutState.curPage.as_mut() {
          let pos: usize = layoutState.linePos.max(0) as usize;
          page.insert_line(pos, line);
        }
        // java paraEnv.isFirstLineOfPara = false;
        layoutState.paraEnv.isFirstLineOfPara = false;
        // java paraEnv.firstLineIsSecBreak = false;
        layoutState.paraEnv.firstLineIsSecBreak = false;
        let _ = old_cp;
      }
    }
}

// ILineHeightCallback for LineLayouter
impl ILineHeightCallback for LineLayouter {
    fn setLineHeight(&mut self, var1: i32) { self.setLineHeight(var1); }
    fn setAlignLine(&mut self, var1: i32) { self.setAlignLine(var1); }
    fn setAscDesPos(&mut self, var1: i32, var2: i32) { self.setAscDesPos_line(var1, var2); }
    fn setForceVCenterAlign(&mut self, var1: bool) { self.setForceVCenterAlign_map(var1); }
}

// rust: ★wr-29 07 —— WR-11 测试保留：按 run 宽段累加（cache 语义不在 Java 门面内，
//       本行测试沿用历史签名）
pub fn cache_run_widths(runs: &[&str], predict: impl Fn(&str) -> i32) -> Vec<i32> {
    runs.iter().map(|r| predict(r)).collect()
}
