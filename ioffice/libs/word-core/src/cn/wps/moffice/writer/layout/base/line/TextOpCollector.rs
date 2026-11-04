// Java: cn.wps.moffice.writer.layout.base.line.TextOpCollector
#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

use std::sync::Arc;

use thirdparty_font::cn::wps::font::ITypeface::{FontMetrics, ITypeface};

use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoText::TypoText;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::PunctTool::PunctTool;

// java static final int OP_BUF_SIZE = 512;
const OP_BUF_SIZE: usize = 512;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DrawPosKind {
    None,
    Fast,
    Normal,
    Rotation,
}

// java public class TextOpCollector implements IDrawPosCallback, Reusable
pub struct TextOpCollector {
    mEnable: bool,
    mDrawSpace: bool,
    mDrawParaBreak: bool,
    mDrawNewLine: bool,
    // java private boolean mDrawVanishTab; never assigned in init, stays false
    mDrawVanishTab: bool,
    mAllowMergeTypoText: bool,
    mSpecialGraphs: Vec<*mut Graphs>,
    mSpecialRTL: Vec<bool>,
    mHLGraphs: Vec<*mut Graphs>,
    mHLRTL: Vec<bool>,
    mTextLine: *mut TypoTextLine,
    mCurText: TypoText,
    mDrawPosKind: DrawPosKind,
    mNextDrawPosOffsetIndex: i32,
    mNextDrawPosOffstGraphIndex: i32,
    mHasSetDrawPos: bool,
    mHasSetDrawPosY: bool,
    mHasSetRoutation: bool,
    mLastCharsNode: *const CharsNode,
    mDocEnv: *mut DocumentEnv,
    mViewEnv: *const ViewEnv,
    mLayoutState: *mut LayoutState,
    mOpSize: i32,
    mCache: Vec<i32>,
}

impl Default for TextOpCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl TextOpCollector {
    pub fn new() -> Self {
        Self {
            mEnable: false,
            mDrawSpace: false,
            mDrawParaBreak: false,
            mDrawNewLine: false,
            mDrawVanishTab: false,
            mAllowMergeTypoText: false,
            mSpecialGraphs: Vec::new(),
            mSpecialRTL: Vec::new(),
            mHLGraphs: Vec::new(),
            mHLRTL: Vec::new(),
            mTextLine: std::ptr::null_mut(),
            mCurText: TypoText::default(),
            mDrawPosKind: DrawPosKind::None,
            mNextDrawPosOffsetIndex: 0,
            mNextDrawPosOffstGraphIndex: -1,
            mHasSetDrawPos: false,
            mHasSetDrawPosY: false,
            mHasSetRoutation: false,
            mLastCharsNode: std::ptr::null(),
            mDocEnv: std::ptr::null_mut(),
            mViewEnv: std::ptr::null(),
            mLayoutState: std::ptr::null_mut(),
            mOpSize: 0,
            mCache: vec![0; OP_BUF_SIZE],
        }
    }

    // java public TextOpCollector(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn bind(&mut self, docEnv: *mut DocumentEnv, viewEnv: *const ViewEnv) {
        self.mDocEnv = docEnv;
        self.mViewEnv = viewEnv;
    }

    // java public boolean reuseClean()
    pub fn reuseClean(&mut self) -> bool {
        self.mLayoutState = std::ptr::null_mut();
        self.mSpecialGraphs.clear();
        self.mSpecialRTL.clear();
        self.mHLGraphs.clear();
        self.mHLRTL.clear();
        self.mTextLine = std::ptr::null_mut();
        self.mCurText.init();
        self.mDrawPosKind = DrawPosKind::None;
        self.mLastCharsNode = std::ptr::null();
        true
    }

    // java public void reuseInit()
    pub fn reuseInit(&mut self) {}

    // java public void dispose()
    pub fn dispose(&mut self) {
        self.reuseClean();
    }

    // java public void init(TypoTextLine textLine, LayoutState layoutState, int lastGraphIndex)
    pub fn init(&mut self, textLine: &mut TypoTextLine, layoutState: &mut LayoutState, lastGraphIndex: i32) {
        self.mOpSize = 0;
        self.mLayoutState = layoutState as *mut LayoutState;
        self.mTextLine = textLine as *mut TypoTextLine;
        self.mEnable = true;
        if !self.mViewEnv.is_null() {
            let viewEnv = unsafe { &*self.mViewEnv };
            self.mDrawSpace = viewEnv.isDrawSpace();
            self.mDrawParaBreak = viewEnv.isDrawParaBreak();
            self.mDrawNewLine = viewEnv.isDrawNewLine();
        } else {
            self.mDrawSpace = false;
            self.mDrawParaBreak = false;
            self.mDrawNewLine = false;
        }
        self.mAllowMergeTypoText = if self.mDocEnv.is_null() {
            true
        } else {
            !unsafe { (*self.mDocEnv).notMergeTypoText }
        };
        let _ = self.mAllowMergeTypoText;
        self.mSpecialGraphs.clear();
        self.mSpecialRTL.clear();
        self.mHLGraphs.clear();
        self.mHLRTL.clear();
        self.mCurText.init();
        self.mLastCharsNode = std::ptr::null();
        self.initDrawPosHandler(layoutState, lastGraphIndex);
    }

    // java public void initDrawPosHandler(LayoutState layoutState, int maxGraphIndex)
    pub fn initDrawPosHandler(&mut self, layoutState: &LayoutState, maxGraphIndex: i32) {
        let textFlow = layoutState.pageEnv.textFlow;
        let rotationFarEast = textFlow == 4 || textFlow == 1;
        let drawPosOffsetIndex = layoutState.getDrawPosOffsetIndexes();
        if rotationFarEast {
            self.mDrawPosKind = DrawPosKind::Rotation;
        } else if !drawPosOffsetIndex.isEmpty() && drawPosOffsetIndex.get(0) <= maxGraphIndex {
            if !layoutState.paraEnv.isRTL && !layoutState.hasBidiGraph() {
                self.mDrawPosKind = DrawPosKind::Fast;
            } else {
                self.mDrawPosKind = DrawPosKind::Normal;
            }
        } else {
            self.mDrawPosKind = DrawPosKind::None;
        }
        self.mHasSetDrawPos = false;
        self.mHasSetDrawPosY = false;
        self.mHasSetRoutation = false;
        if self.mDrawPosKind == DrawPosKind::Fast {
            self.mNextDrawPosOffsetIndex = 0;
            self.mNextDrawPosOffstGraphIndex = drawPosOffsetIndex.get(0);
        }
    }

    // java public void commit()
    pub fn commit(&mut self) {
        if !self.mEnable || self.mTextLine.is_null() {
            return;
        }
        let special: Vec<*mut Graphs> = self.mSpecialGraphs.clone();
        let special_rtl: Vec<bool> = self.mSpecialRTL.clone();
        for i in 0..special.len() {
            let ptr = special[i];
            if !ptr.is_null() {
                let run = unsafe { &mut *ptr };
                let run_type = run.runType;
                self.addGraphs(run, run_type, special_rtl[i]);
            }
        }
        if !self.mHLGraphs.is_empty() {
            self.ensureMoreCapacitySize(2);
            self.put(19);
            self.put(0);
            let hl: Vec<*mut Graphs> = self.mHLGraphs.clone();
            let hl_rtl: Vec<bool> = self.mHLRTL.clone();
            for i in 0..hl.len() {
                let ptr = hl[i];
                if !ptr.is_null() {
                    let run = unsafe { &mut *ptr };
                    let run_type = run.runType;
                    self.addGraphs(run, run_type, hl_rtl[i]);
                }
            }
        }
        let op_size = self.mOpSize;
        let cache = self.mCache.clone();
        let line = self.mTextLine;
        unsafe { (*line).commitOp(&cache, op_size); }
    }

    // java public void addRun(Graphs run, int runLeft, int runRight, boolean isRTL)
    pub fn addRun(&mut self, run: &mut Graphs, layoutState: &mut LayoutState, runLeft: i32, runRight: i32, isRTL: bool) {
        if !self.mEnable {
            return;
        }
        if TypoRun::isTab(run.runType) {
            layoutState.typoPara.putWidth(run.graphIndex, run.getRight() - run.getLeft());
        }
        run.offsetXTo(runLeft);
        let _ = runRight;
        if run.hasBdr() {
            let bdrWidth = if isRTL { -run.getBdrWidth() } else { run.getBdrWidth() };
            if run.hasBeginBdr() {
                layoutState.extendWidths(run, bdrWidth, run.graphIndex, 1);
            }
            if run.hasEndBdr() {
                layoutState.extendWidths(run, bdrWidth, run.getEndGraphIndex(), 1);
            }
        }
        if let Some(collector) = layoutState.mDocFieldsCollector.as_ref() {
            collector.collectDocFieldInGraph(run, isRTL);
        }
        match run.runType {
            1 | 7 | 18 | 19 => self.addGraphs(run, run.runType, isRTL),
            2 | 17 => {
                if self.mDrawSpace {
                    self.addSpecialText(run);
                    self.mSpecialRTL.push(isRTL);
                } else {
                    self.newHLText(run, isRTL);
                }
            }
            3 => {
                if self.mDrawNewLine {
                    self.addSpecialText(run);
                    self.mSpecialRTL.push(isRTL);
                } else {
                    self.newHLText(run, isRTL);
                }
            }
            4 => {
                let vanish = run.charsRef.as_ref().map(|c| c.isVanishBreak).unwrap_or(false);
                if self.mDrawParaBreak && !vanish {
                    self.addSpecialText(run);
                    self.mSpecialRTL.push(isRTL);
                } else {
                    self.newHLText(run, isRTL);
                }
            }
            5 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => {
                self.addSpecialText(run);
                self.mSpecialRTL.push(isRTL);
            }
            6 => {
                self.ensureMoreCapacitySize(2);
                let drawing = layoutState.getIndexOfDrawing(run);
                self.put(18);
                self.put(drawing);
                self.addGraphs(run, run.runType, isRTL);
            }
            20 => {
                if !self.mDrawVanishTab && run.runType <= 20 {
                    self.newHLText(run, isRTL);
                } else {
                    self.addSpecialText(run);
                    self.mSpecialRTL.push(isRTL);
                }
            }
            21 | 22 | 23 | 24 | 25 => {
                self.addSpecialText(run);
                self.mSpecialRTL.push(isRTL);
            }
            _ => {}
        }
    }

    fn addSpecialText(&mut self, run: &mut Graphs) {
        self.mSpecialGraphs.push(run as *mut Graphs);
    }

    fn newHLText(&mut self, run: &mut Graphs, isRTL: bool) {
        self.mHLGraphs.push(run as *mut Graphs);
        self.mHLRTL.push(isRTL);
    }

    fn addGraphs(&mut self, run: &mut Graphs, runType: i32, isRTL: bool) {
        // Graphs.mDfdFlagsIt only carries the flag; it does not split the run.
        self.doAddGraphs(run, runType, isRTL);
    }

    fn doAddGraphs(&mut self, run: &mut Graphs, runType: i32, isRTL: bool) {
        if self.mDrawPosKind != DrawPosKind::None && !self.mLayoutState.is_null() {
            let state = self.mLayoutState;
            self.process_draw_pos(run, unsafe { &*state });
        }
        self.ensureMoreCapacitySize(72);
        self.emit_font_style(run);
        if runType != self.mCurText.runType {
            self.mCurText.runType = runType;
            self.put(4);
            self.put(runType);
        }
        if run.language != self.mCurText.language {
            self.mCurText.language = run.language;
            self.put(20);
            self.put(run.language);
        }
        if run.baseLine != self.mCurText.baseline {
            self.mCurText.baseline = run.baseLine;
            self.put(3);
            self.put(run.baseLine);
        }
        let dfdFlag = run.getDfdFlag();
        if dfdFlag != self.mCurText.dfdFlag {
            self.mCurText.dfdFlag = dfdFlag;
            self.put(33);
            self.put(dfdFlag);
        }
        let bdrFlag = run.mBdrInfo.getFlag();
        if bdrFlag != self.mCurText.bdrFlag {
            self.mCurText.bdrFlag = bdrFlag;
            self.put(34);
            self.put(bdrFlag);
        }
        if isRTL != self.mCurText.isRTL {
            self.mCurText.isRTL = isRTL;
            self.put(10);
            self.put(if isRTL { 1 } else { 0 });
        }
        if run.mHyphenFlag != self.mCurText.hyphenFlag {
            self.mCurText.hyphenFlag = run.mHyphenFlag;
            self.put(35);
            self.put(self.mCurText.hyphenFlag);
        }
        let chars_ptr = run.charsRef.as_ref().map(|c| c as *const CharsNode).unwrap_or(std::ptr::null());
        if chars_ptr != self.mLastCharsNode {
            if let Some(charsRef) = run.charsRef.as_ref() {
                if charsRef.isField != self.mCurText.isInField {
                    self.mCurText.isInField = charsRef.isField;
                    self.put(12);
                    self.put(if self.mCurText.isInField { 1 } else { 0 });
                }
                if charsRef.isBalloonAuthor != self.mCurText.isBalloonAuthor {
                    self.mCurText.isBalloonAuthor = charsRef.isBalloonAuthor;
                    self.put(32);
                    self.put(if self.mCurText.isBalloonAuthor { 1 } else { 0 });
                }
                let isInMath = charsRef.isInMath;
                let hyperlinkIndex = if isInMath { -1 } else { charsRef.hyperlinkIndex };
                if (hyperlinkIndex >= 0) != self.mCurText.isHyperlink {
                    self.mCurText.isHyperlink = hyperlinkIndex >= 0;
                    self.put(13);
                    self.put(if self.mCurText.isHyperlink { 1 } else { 0 });
                }
                if hyperlinkIndex != self.mCurText.hyperlinkId {
                    self.mCurText.hyperlinkId = hyperlinkIndex;
                    self.put(7);
                    self.put(hyperlinkIndex);
                }
                if isInMath != self.mCurText.isMath {
                    self.mCurText.isMath = isInMath;
                    self.put(15);
                    self.put(if isInMath { 1 } else { 0 });
                }
                if charsRef.geoText != self.mCurText.geoText {
                    self.mCurText.geoText = charsRef.geoText;
                    let index = self.geo_index(self.mCurText.geoText);
                    self.put(14);
                    self.put(index);
                }
                if charsRef.revisionAuthor != self.mCurText.revisionAuthor {
                    self.mCurText.revisionAuthor = charsRef.revisionAuthor;
                    self.put(6);
                    self.put(charsRef.revisionAuthor);
                }
            } else {
                if self.mCurText.isInField {
                    self.mCurText.isInField = false;
                    self.put(12);
                    self.put(0);
                }
                if self.mCurText.isHyperlink {
                    self.mCurText.isHyperlink = false;
                    self.put(13);
                    self.put(0);
                }
                if self.mCurText.isMath {
                    self.mCurText.isMath = false;
                    self.put(15);
                    self.put(0);
                }
                if self.mCurText.geoText.is_some() {
                    self.mCurText.geoText = None;
                    self.put(14);
                    self.put(0);
                }
                if self.mCurText.revisionAuthor != -1 {
                    self.mCurText.revisionAuthor = -1;
                    self.put(6);
                    self.put(-1);
                }
                if self.mCurText.hyperlinkId != -1 {
                    self.mCurText.hyperlinkId = -1;
                    self.put(7);
                    self.put(-1);
                }
            }
            self.mLastCharsNode = chars_ptr;
        }
        self.put(16);
        self.put(run.mXPos);
        self.put(17);
        self.put(run.getRight());
        self.put(1);
        self.put(run.graphIndex);
        self.put(2);
        self.put(run.graphLength);
    }

    fn emit_font_style(&mut self, run: &Graphs) {
        // Java compares TypoFontStyle by reference, then each field against the previous
        // style or TypoFontStyle.sDefault when the previous style is null.
        let same = match (&run.fontStyle, &self.mCurText.typoFontStyle) {
            (None, None) => true,
            (Some(a), Some(b)) => font_style_same(a, b),
            _ => false,
        };
        if same {
            return;
        }
        let new_style = run.fontStyle.clone().unwrap_or_else(TypoFontStyle::newInstance);
        let old_style = self.mCurText.typoFontStyle.clone().unwrap_or_else(TypoFontStyle::newInstance);
        if new_style.maxAscent != old_style.maxAscent {
            self.put(21);
            self.put(new_style.maxAscent);
        }
        if new_style.maxDescent != old_style.maxDescent {
            self.put(22);
            self.put(new_style.maxDescent);
        }
        if new_style.external != old_style.external {
            self.put(23);
            self.put(new_style.external);
        }
        if new_style.fontSize != old_style.fontSize {
            self.put(24);
            self.put(new_style.fontSize);
        }
        if new_style.shd != old_style.shd {
            let index = self.shd_index(new_style.shd.as_ref());
            self.put(25);
            self.put(index);
        }
        if new_style.fontColor != old_style.fontColor {
            self.put(26);
            self.put(new_style.fontColor);
        }
        if new_style.highlightColor != old_style.highlightColor {
            self.put(27);
            self.put(new_style.highlightColor);
        }
        if !typeface_same(&new_style.typeface, &old_style.typeface) {
            let index = self.typeface_index(&new_style.typeface);
            self.put(28);
            self.put(index);
        }
        if new_style.charScale != old_style.charScale {
            self.put(29);
            self.put(new_style.charScale.to_bits() as i32);
        }
        if new_style.flag != old_style.flag {
            self.put(30);
            self.put(new_style.flag);
        }
        if new_style.bdrWidth != old_style.bdrWidth {
            self.put(31);
            self.put(new_style.bdrWidth.to_bits() as i32);
        }
        self.mCurText.typoFontStyle = run.fontStyle.clone();
    }

    fn snapshot(&self) -> *mut TypoSnapshot {
        if self.mTextLine.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { (*self.mTextLine).mSnapshot }
        }
    }

    fn typeface_index(&self, face: &Option<Arc<dyn ITypeface>>) -> i32 {
        let snap = self.snapshot();
        if snap.is_null() {
            0
        } else {
            unsafe { (*snap).add_typeface(face) }
        }
    }

    fn shd_index(&self, shd: Option<&drawing::cn::wps::moffice::drawing::graphics::SHD::SHD>) -> i32 {
        let snap = self.snapshot();
        if snap.is_null() { 0 } else { unsafe { (*snap).add_shd(shd) } }
    }

    fn geo_index(&self, geo: Option<()>) -> i32 {
        let snap = self.snapshot();
        if snap.is_null() { 0 } else { unsafe { (*snap).add_geo_text(geo) } }
    }

    fn process_draw_pos(&mut self, run: &Graphs, layout_state: &LayoutState) {
        match self.mDrawPosKind {
            DrawPosKind::None => {}
            DrawPosKind::Fast => {
                let indexes = layout_state.getDrawPosOffsetIndexes();
                let values = layout_state.getDrawPosOffsetValues();
                if run.graphIndex == self.mNextDrawPosOffstGraphIndex {
                    let x = values.get(self.mNextDrawPosOffsetIndex);
                    self.mNextDrawPosOffsetIndex += 1;
                    self.setDrawPosOffsetX(x);
                    if self.mNextDrawPosOffsetIndex >= indexes.size() {
                        self.mNextDrawPosOffstGraphIndex = -1;
                    } else {
                        self.mNextDrawPosOffstGraphIndex = indexes.get(self.mNextDrawPosOffsetIndex);
                    }
                    self.mHasSetDrawPos = true;
                } else if self.mHasSetDrawPos {
                    self.setDrawPosOffsetX(0);
                    self.mHasSetDrawPos = false;
                }
            }
            DrawPosKind::Normal => {
                let drawPosX = layout_state.getDrawPosOffset(run.graphIndex);
                if drawPosX != 0 {
                    self.setDrawPosOffsetX(drawPosX);
                    self.mHasSetDrawPos = true;
                } else if self.mHasSetDrawPos {
                    self.setDrawPosOffsetX(0);
                    self.mHasSetDrawPos = false;
                }
            }
            DrawPosKind::Rotation => self.process_rotation(run, layout_state),
        }
    }

    fn process_rotation(&mut self, run: &Graphs, layout_state: &LayoutState) {
        let drawPosX = layout_state.getDrawPosOffset(run.graphIndex);
        let style = run.fontStyle.as_ref();
        let face = style.and_then(|s| s.typeface.as_ref());
        if run.language == 2 && face.is_some() && style.is_some() {
            let style = style.unwrap();
            let face = face.unwrap();
            let mut metrics = FontMetrics::new();
            face.getFontMetrics(style.fontSize as f32, &mut metrics);
            let mut drawPosOffsetY = metrics.mMaxAscent as i32;
            let rect = face.getDefaultFareastCharRect(style.fontSize as f32);
            let vBaseLineOffet = (-(rect.bottom.max(0.0) + rect.left.max(0.0))) as i32;
            let mut drawPosOffsetX = vBaseLineOffet;
            let mut rouation = 1;
            if run.runType == 18 {
                let ch = layout_state.typoPara.getChar(run.graphIndex);
                let charRect = face.getTextRect_char(style.fontSize as f32, ch as u16);
                if !PunctTool::isRotatedPunct(ch as i32, 2) {
                    rouation = 0;
                }
                if rouation == 0 {
                    drawPosOffsetX = drawPosX;
                    drawPosOffsetY = 0;
                } else {
                    if PunctTool::needAlignRightWhileRotation(ch) {
                        drawPosOffsetX = (vBaseLineOffet as f32 + (rect.right - charRect.right)) as i32;
                    } else if PunctTool::needAlignCenterWhileRotation(ch) {
                        drawPosOffsetX = (vBaseLineOffet as f32 + (rect.left + rect.right - charRect.left - charRect.right) / 2.0) as i32;
                    }
                    if PunctTool::needAlignTopWhileRotation(ch) {
                        drawPosOffsetY = (drawPosOffsetY as f32 - (metrics.mMaxAscent - charRect.top.abs() - (run.width() as f32) * 0.2)) as i32;
                    }
                }
            }
            self.setDrawPosOffsetX(drawPosOffsetX);
            self.setDrawPosOffsetY(drawPosOffsetY);
            self.setRotation(rouation);
            self.mHasSetDrawPos = true;
            self.mHasSetDrawPosY = true;
            self.mHasSetRoutation = true;
        } else {
            if drawPosX != 0 {
                self.setDrawPosOffsetX(drawPosX);
                self.mHasSetDrawPos = true;
            } else if self.mHasSetDrawPos {
                self.setDrawPosOffsetX(0);
                self.mHasSetDrawPos = false;
            }
            if self.mHasSetDrawPosY {
                self.setDrawPosOffsetY(0);
                self.mHasSetDrawPosY = false;
            }
            if self.mHasSetRoutation {
                self.setRotation(0);
                self.mHasSetRoutation = false;
            }
        }
    }

    // java public void setDrawPosOffsetX(int xOffset)
    pub fn setDrawPosOffsetX(&mut self, xOffset: i32) {
        if xOffset != self.mCurText.drawPosOffsetX {
            self.mCurText.drawPosOffsetX = xOffset;
            self.ensureMoreCapacitySize(2);
            self.put(8);
            self.put(xOffset);
        }
    }

    // java public void setDrawPosOffsetY(int yOffset)
    pub fn setDrawPosOffsetY(&mut self, yOffset: i32) {
        if yOffset != self.mCurText.drawPosOffsetY {
            self.mCurText.drawPosOffsetY = yOffset;
            self.ensureMoreCapacitySize(2);
            self.put(9);
            self.put(yOffset);
        }
    }

    // java public void setRotation(int rotation)
    pub fn setRotation(&mut self, rotation: i32) {
        if rotation != self.mCurText.rotation {
            self.mCurText.rotation = rotation;
            self.ensureMoreCapacitySize(2);
            self.put(11);
            self.put(rotation);
        }
    }

    fn ensureMoreCapacitySize(&mut self, size: i32) {
        let need = self.mOpSize + size;
        if need > self.mCache.len() as i32 {
            let mut new_len = (self.mCache.len() as i32).max(1) << 1;
            while new_len < need {
                new_len <<= 1;
            }
            self.mCache.resize(new_len as usize, 0);
        }
    }

    fn put(&mut self, value: i32) {
        let index = self.mOpSize as usize;
        if index >= self.mCache.len() {
            self.mCache.push(value);
        } else {
            self.mCache[index] = value;
        }
        self.mOpSize += 1;
    }
}

fn typeface_same(a: &Option<Arc<dyn ITypeface>>, b: &Option<Arc<dyn ITypeface>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        _ => false,
    }
}

fn font_style_same(a: &TypoFontStyle, b: &TypoFontStyle) -> bool {
    a.maxAscent == b.maxAscent
        && a.maxDescent == b.maxDescent
        && a.external == b.external
        && a.fontSize == b.fontSize
        && a.shd == b.shd
        && a.fontColor == b.fontColor
        && a.highlightColor == b.highlightColor
        && typeface_same(&a.typeface, &b.typeface)
        && a.charScale == b.charScale
        && a.flag == b.flag
        && a.bdrWidth == b.bdrWidth
}
