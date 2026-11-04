// Java: cn.wps.moffice.writer.layout.base.line.charSpace.WhiteSpaceHandler
// rust: ★wr-29 05 —— 真实移植（Java 178 行）：空格压缩宽度/断行决策（wordWrap、diffSBMB、
//       半角/全角空格、压缩宽度写入 LineCompressEnv.slot）。池面不重做。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use core::cell::UnsafeCell;

// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.line.SubLineEnv
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public class WhiteSpaceHandler implements IWhiteSpaceHandler
pub struct WhiteSpaceHandler {
    // java protected LayoutState mLayoutState;
    pub(crate) mLayoutState: *mut LayoutState,
    // java protected boolean mDiffSBMB;
    pub(crate) mDiffSBMB: bool,
    // java protected boolean mWordWrap;
    pub(crate) mWordWrap: bool,
    // java private CharParse.CharInfo mCharInfo = new CharParse.CharInfo();
    mCharInfo: UnsafeCell<crate::cn::wps::moffice::writer::core::CharParse::CharInfo>,
    // java protected final DocumentEnv mDocEnv;
    pub(crate) mDocEnv: *mut DocumentEnv,
    // java protected final ViewEnv mViewEnv;
    pub(crate) mViewEnv: *mut ViewEnv,
    // java protected SubLineEnv mSubLineEnv;
    pub(crate) mSubLineEnv: *mut SubLineEnv,
}

impl WhiteSpaceHandler {
    // java public WhiteSpaceHandler(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn WhiteSpaceHandler(docEnv: *mut DocumentEnv, viewEnv: *mut ViewEnv) -> WhiteSpaceHandler {
      // java this.mDocEnv = docEnv; this.mViewEnv = viewEnv;
      WhiteSpaceHandler {
        mLayoutState: std::ptr::null_mut(),
        mDiffSBMB: false,
        mWordWrap: true,
        mCharInfo: UnsafeCell::new(crate::cn::wps::moffice::writer::core::CharParse::CharInfo { unicodeCodeType: 0, charType: 6, codePoint: 0 }),
        mDocEnv: docEnv,
        mViewEnv: viewEnv,
        mSubLineEnv: std::ptr::null_mut(),
      }
    }

    // java public void setEnv(LayoutState env)
    pub fn setEnv(&mut self, env: &LayoutState) {
      // java this.mLayoutState = env;
      self.mLayoutState = (env as *const LayoutState) as *mut LayoutState;
      // java this.mWordWrap = env.paraEnv.wordWrap;
      self.mWordWrap = env.paraEnv.wordWrap;
      // java this.mDiffSBMB = this.mDocEnv.balanceSingleByteDoubleByteWidth;
      self.mDiffSBMB = if self.mDocEnv.is_null() { false } else { unsafe { (*self.mDocEnv).balanceSingleByteDoubleByteWidth } };
    }

    // java public void setSubLineEnv(SubLineEnv subLineEnv)
    pub fn setSubLineEnv(&mut self, subLineEnv: &mut SubLineEnv) {
      // java this.mSubLineEnv = subLineEnv;
      self.mSubLineEnv = subLineEnv as *mut SubLineEnv;
    }

    // java private int getWordWrapWhiteSpaceMinWidth(TypoFontStyle fontStyle, boolean diffSBMB)
    fn getWordWrapWhiteSpaceMinWidth(&self, fontStyle: &crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle, diffSBMB: bool) -> i32 {
      // java float ratio = 4.0F;
      let mut ratio: f32 = 4.0;
      // java if (!diffSBMB) { ratio = fontStyle.typeface.getWriterLayoutCharSpaceRatio(); }
      if !diffSBMB {
        if let Some(typeface) = fontStyle.typeface.as_ref() {
          ratio = typeface.getWriterLayoutCharSpaceRatio();
        }
      }
      // java float scaledFontSize = fontStyle.charScale * (float)fontStyle.fontSize;
      let scaledFontSize: f32 = fontStyle.charScale * fontStyle.fontSize as f32;
      // java return (int)(scaledFontSize / ratio);
      (scaledFontSize / ratio) as i32
    }

    // java private boolean isLetterChar(Graphs graphs, boolean firstOrLast)
    fn isLetterChar(&self, graphs: &Graphs, firstOrLast: bool) -> bool {
      // java int autoSpaceType = graphs.autoSpaceType;
      let autoSpaceType: i32 = graphs.autoSpaceType;
      // java if (2 == autoSpaceType) { return false; }
      if 2 == autoSpaceType { return false; }
      // java else if (!TypoRun.isPunct(graphs.runType) && 1 != autoSpaceType && 4 != autoSpaceType)
      if !crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun::isPunct(graphs.runType) && 1 != autoSpaceType && 4 != autoSpaceType {
        // java int graphStartIdx = graphs.graphIndex;
        let graphStartIdx: i32 = graphs.graphIndex;
        // java int graphIdx = firstOrLast ? graphStartIdx : graphs.getEndGraphIndex();
        let graphIdx: i32 = if firstOrLast { graphStartIdx } else { graphs.getEndGraphIndex() };
        // java char ch = this.mLayoutState.typoPara.getChar(graphIdx);
        let ch: char = unsafe { (*self.mLayoutState).typoPara.getChar(graphIdx) };
        // java CharParse.getInstance().getCharInfoUseCache(charInfo, ch, true);
        let mut cp = crate::cn::wps::moffice::writer::core::CharParse::CharParse::getInstance();
        let charInfo = unsafe { &mut *self.mCharInfo.get() };
        cp.getCharInfoUseCache(charInfo, ch as u32 as i32, true);
        // java int charClass = charInfo.charType;
        let charClass: i32 = charInfo.charType;
        // java if (1 != charClass && 3 != charClass)
        if 1 != charClass && 3 != charClass {
          // java if (4 == charClass)
          if 4 == charClass {
            // java PropertySet prop = graphs.charsRef.finalRunProp;
            let prop: Option<&util::moffice::property::PropertySet::PropertySet> =
                graphs.charsRef.as_ref().and_then(|c| c.finalRunProp.as_ref());
            // java if (0 == prop.getInt(40, 0)) { return true; }
            if let Some(prop) = prop {
              if 0 == prop.getInt__int_int(40, 0) { return true; }
            }
          }
          // java return false;
          return false;
        } else {
          // java else { return true; }
          return true;
        }
      } else {
        // java else { return true; }
        return true;
      }
    }

    // java private boolean isLetterGraphs(Graphs graphs)
    fn isLetterGraphs(&self, graphs: &Graphs) -> bool {
      // java if (1 != graphs.runType && 18 != graphs.runType && 6 != graphs.language) { return false; }
      if 1 != graphs.runType && 18 != graphs.runType && 6 != graphs.language { return false; }
      // java else { return TypoRun.isWestLangType(graphs.language); }
      crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun::isWestLangType(graphs.language)
    }

    // java protected boolean isBetweenLetters(Graphs prev, Graphs cur, Graphs next)
    fn isBetweenLetters(&self, prev: Option<&Graphs>, cur: Option<&Graphs>, next: Option<&Graphs>) -> bool {
      // java if (null != prev && null != cur && cur.graphLength <= 1 && null != next)
      if let (Some(prev), Some(cur), Some(next)) = (prev, cur, next) {
        if cur.graphLength <= 1 {
          // java if (this.isLetterGraphs(prev) && this.isLetterGraphs(next))
          if self.isLetterGraphs(prev) && self.isLetterGraphs(next) {
            // java return this.isLetterChar(prev, true) && this.isLetterChar(next, false);
            return self.isLetterChar(prev, true) && self.isLetterChar(next, false);
          }
          // java else { return false; }
          return false;
        }
        // java else { return false; }
        return false;
      } else {
        // java else { return false; }
        return false;
      }
    }

    // java private void setCompactWidth(Graphs graphs, int fontSpace)
    fn setCompactWidth(&mut self, graphs: &mut Graphs, fontSpace: i32) {
      // java TypoFontStyle style = graphs.fontStyle;
      let Some(style) = graphs.fontStyle.clone() else { return };
      // java float scaledFontSize = style.charScale * (float)style.fontSize;
      let scaledFontSize: f32 = style.charScale * style.fontSize as f32;
      // java int fixedWidthPerChar = (int)(scaledFontSize / 2.0F + (float)fontSpace);
      let fixedWidthPerChar: i32 = (scaledFontSize / 2.0 + fontSpace as f32) as i32;
      // java int sumWidth = fixedWidthPerChar * graphs.graphLength;
      let sumWidth: i32 = fixedWidthPerChar * graphs.graphLength;
      // java if (sumWidth != graphs.width())
      if sumWidth != graphs.width() {
        // java this.mLayoutState.saveOrgWidth(graphs);
        unsafe { (*self.mLayoutState).saveOrgWidth(graphs); }
        // java graphs.mWidth = sumWidth;
        graphs.mWidth = sumWidth;
        // java this.mLayoutState.typoPara.fillWidths(graphs.graphIndex, graphs.graphLength, fixedWidthPerChar);
        unsafe { (*self.mLayoutState).typoPara.fillWidths(graphs.graphIndex, graphs.graphLength, fixedWidthPerChar); }
      }
    }

    // java protected void setCompressableWidth(Graphs graphs, boolean diffSBMB, int extraCompressableSpace)
    fn setCompressableWidth(&mut self, graphs: &Graphs, diffSBMB: bool, extraCompressableSpace: i32) {
      // java TypoFontStyle fontStyle = graphs.fontStyle;
      let Some(fontStyle) = graphs.fontStyle.clone() else { return };
      // java int minWidth = this.getWordWrapWhiteSpaceMinWidth(fontStyle, diffSBMB);
      let minWidth: i32 = self.getWordWrapWhiteSpaceMinWidth(&fontStyle, diffSBMB);
      // java int compressableWidthPerChar = graphs.mWidth / graphs.graphLength - minWidth + extraCompressableSpace;
      let compressableWidthPerChar: i32 = graphs.mWidth / graphs.graphLength - minWidth + extraCompressableSpace;
      // java if ((float)compressableWidthPerChar > 0.0F && this.mSubLineEnv != null)
      if compressableWidthPerChar as f32 > 0.0 && !self.mSubLineEnv.is_null() {
        // java LineCompressEnv lineCompressEnv = this.mSubLineEnv.lineCompressEnv;
        // java if (lineCompressEnv != null)
        if let Some(lineCompressEnv) = unsafe { (*self.mSubLineEnv).lineCompressEnv.as_mut() } {
          // java for(i = graphs.graphIndex; i < max; ++i) { lineCompressEnv.addSpaceCompressableSpace(i, compressableWidthPerChar); }
          let mut i: i32 = graphs.graphIndex;
          let max: i32 = i + graphs.graphLength;
          while i < max {
            lineCompressEnv.addSpaceCompressableSpace(i, compressableWidthPerChar);
            i += 1;
          }
        }
      }
    }

    // java public boolean handleSpace(Graphs prev, Graphs cur)
    pub fn handleSpace(&mut self, prev: Option<&mut Graphs>, cur: &mut Graphs) -> bool {
      // java int runType = cur.runType;
      let runType: i32 = cur.runType;
      // java boolean isNormalSpace = 2 == runType;
      let isNormalSpace: bool = 2 == runType;
      // java boolean isFullWidthSpace = 17 == runType;
      let isFullWidthSpace: bool = 17 == runType;
      // java if (!isNormalSpace && !isFullWidthSpace) { return true; }
      if !isNormalSpace && !isFullWidthSpace { return true; }
      // java boolean diffSBMB = this.mDiffSBMB;
      let diffSBMB: bool = self.mDiffSBMB;
      // java if (diffSBMB)
      if diffSBMB {
        // java if (isFullWidthSpace) { return true; }
        if isFullWidthSpace { return true; }
        // java if (this.mWordWrap)
        if self.mWordWrap {
          // java Graphs nextGraphs = layoutState.getGraphList().getNext(cur);
          let nextGraphs: *mut Graphs = unsafe { (*self.mLayoutState).getGraphList().getNext(cur as *mut Graphs) };
          let nextRef: Option<&Graphs> = if nextGraphs.is_null() { None } else { Some(unsafe { &*nextGraphs }) };
          // java if (this.isBetweenLetters(prev, cur, nextGraphs))
          if self.isBetweenLetters(prev.as_deref(), Some(cur), nextRef) {
            // java int extraCompressableSpace = 0;
            let mut extraCompressableSpace: i32 = 0;
            // java if (isNormalSpace) { extraCompressableSpace = (int)((float)cur.fontStyle.fontSpace / 2.0F); }
            if isNormalSpace {
              if let Some(fs) = cur.fontStyle.as_ref() {
                extraCompressableSpace = (fs.fontSpace as f32 / 2.0) as i32;
              }
            }
            // java this.setCompressableWidth(cur, diffSBMB, extraCompressableSpace);
            self.setCompressableWidth(cur, diffSBMB, extraCompressableSpace);
          } else {
            // java else { this.setCompactWidth(cur, cur.fontStyle.fontSpace * 2); }
            let fontSpace: i32 = cur.fontStyle.as_ref().map(|f| f.fontSpace).unwrap_or(0);
            self.setCompactWidth(cur, fontSpace * 2);
          }
        } else {
          // java else { this.setCompactWidth(cur, cur.fontStyle.fontSpace * 2); }
          let fontSpace: i32 = cur.fontStyle.as_ref().map(|f| f.fontSpace).unwrap_or(0);
          self.setCompactWidth(cur, fontSpace * 2);
        }
      }
      // java else if (this.mWordWrap)
      else if self.mWordWrap {
        // java this.setCompressableWidth(cur, diffSBMB, 0);
        self.setCompressableWidth(cur, diffSBMB, 0);
      }
      // java return true;
      true
    }

    // java public void endLine()
    pub fn endLine(&mut self) {
      // java （Java 空实现）
    }
}

impl crate::cn::wps::moffice::writer::layout::base::line::charSpace::IWhiteSpaceHandler::IWhiteSpaceHandler for WhiteSpaceHandler {
    fn setEnv(&mut self, var1: &LayoutState) {
        // rust: Java 引用语义（只读 paraEnv/docEnv）
        self.mLayoutState = (var1 as *const LayoutState) as *mut LayoutState;
        self.mWordWrap = var1.paraEnv.wordWrap;
        self.mDiffSBMB = if self.mDocEnv.is_null() { false } else { unsafe { (*self.mDocEnv).balanceSingleByteDoubleByteWidth } };
    }
    fn setSubLineEnv(&mut self, var1: &mut SubLineEnv) {
        self.mSubLineEnv = var1 as *mut SubLineEnv;
    }
    fn handleSpace(&mut self, var1: &mut Graphs, var2: &mut Graphs) -> bool {
        // rust: Java 传 prev 引用（hello prev 恒非空）
        self.handleSpace(Some(var1), var2)
    }
    fn endLine(&mut self) {
        self.endLine();
    }
}
