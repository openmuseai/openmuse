#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

mod j2r_ty {
    // rust: ★wr-29 04 —— ITypeface/TypoFontStylePool 已用真实类型（thirdparty/trait +
    //       cache::style::TypoFontStyle），余下仅供 DEFERRED 停车体引用
    pub struct FontHost;
    pub struct IFontFamily;
    pub struct IFontTable;
    pub struct IMathFontFamily;
    pub struct UILanguage;
    pub struct SHD;
    pub struct FontTable;
    pub struct GraphAlignTool;
}

// Java: import cn.wps.font.FontHost
use self::j2r_ty::FontHost;
// Java: import cn.wps.font.IFontFamily
use self::j2r_ty::IFontFamily;
// Java: import cn.wps.font.IFontTable
use self::j2r_ty::IFontTable;
// Java: import cn.wps.font.IMathFontFamily
use self::j2r_ty::IMathFontFamily;
// Java: import cn.wps.font.ITypeface
// rust: ★wr-29 04 —— 真实 trait（thirdparty_font）+ 量宽实现（tools::TypefaceTool::WriterTypeface）
use std::sync::Arc;
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
// rust: ★wr-29 04 —— 真实 cache::style::TypoFontStyle（Java 同包名）
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::{TypoFontStyle, TypoFontStylePool};
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphAlignTool
// rust: ★wr-29 04 —— 真实 GraphAlignTool（graph 包）
use crate::cn::wps::moffice::writer::layout::base::graph::GraphAlignTool::GraphAlignTool;
// Java: import cn.wps.moffice.writer.layout.base.tools.TypefaceTool
use crate::cn::wps::moffice::writer::layout::base::tools::TypefaceTool::WriterTypeface;
// Java: import cn.wps.moffice.define.UILanguage
use self::j2r_ty::UILanguage;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.core.FontTable
use self::j2r_ty::FontTable;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphAlignTool

// java private static int MAX_FONT_SIZE = Integer.MAX_VALUE;
static MAX_FONT_SIZE: AtomicI32 = AtomicI32::new(i32::MAX);
// java private static boolean SmartFontSize = false;
static SmartFontSize: AtomicBool = AtomicBool::new(false);
// java private static boolean isInEmbedBalloonMode = false;
static isInEmbedBalloonMode: AtomicBool = AtomicBool::new(false);
// java private static int mEmedFontSize = 157;
static mEmedFontSize: AtomicI32 = AtomicI32::new(157);

// java public class FontStyleFactory
#[derive(Default)]
pub struct FontStyleFactory {
}

// java public static class StyleResult
#[derive(Default)]
pub struct StyleResult {
    // java public boolean hasHighlight = false;
    pub(crate) hasHighlight: bool,
    // java public boolean hasSHD = false;
    pub(crate) hasSHD: bool,
    // java public boolean hasBRC = false;
    pub(crate) hasBRC: bool,
    // java public boolean hasUnderline = false;
    pub(crate) hasUnderline: bool,
    // java public boolean hasStrikeThroughs = false;
    pub(crate) hasStrikeThroughs: bool,
    // java public boolean hasTypefaceWithKerning = false;
    pub(crate) hasTypefaceWithKerning: bool,
    // java public boolean ignoreBdr = false;
    pub(crate) ignoreBdr: bool,
    // java public boolean useBuildInFont = true;
    pub(crate) useBuildInFont: bool,
}

// java public class FontStyleFactory
impl FontStyleFactory {
    // java public static void setMaxFontSize(int size)
    pub fn setMaxFontSize(size: i32) {
      // java MAX_FONT_SIZE = size;
      MAX_FONT_SIZE.store(size, Ordering::SeqCst);
    }

    pub fn maxFontSize() -> i32 {
      MAX_FONT_SIZE.load(Ordering::SeqCst)
    }

    // java public static void setSmartFontSize(boolean smartFontSize)
    pub fn setSmartFontSize(smartFontSize: bool) {
      // java SmartFontSize = smartFontSize;
      SmartFontSize.store(smartFontSize, Ordering::SeqCst);
    }

    // java public static void setIsInEmbedBalloonMode(boolean isIn)
    pub fn setIsInEmbedBalloonMode(isIn: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java isInEmbedBalloonMode = isIn;
      isInEmbedBalloonMode = isIn;
      */
    }

    // java public static void setEmebedFontSize(int fontSize)
    pub fn setEmebedFontSize(fontSize: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java mEmedFontSize = fontSize;
      mEmedFontSize = fontSize;
      */
    }

    // java private static TypoFontStyle createFontStyle(PropertySet propSet, ITypeface typeface, int langType, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)
    // java private static TypoFontStyle createFontStyle(PropertySet propSet, ITypeface typeface, int langType, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)
    // java private static TypoFontStyle createFontStyle(PropertySet propSet, ITypeface typeface, int langType, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)
    pub fn createFontStyle(propSet: &PropertySet, typeface: Option<Arc<dyn ITypeface>>, mut langType: i32, mut styleResults: Option<&mut StyleResult>, mut typoFontStylePool: Option<&mut TypoFontStylePool>) -> TypoFontStyle {
      // java TypoFontStyle style = typoFontStylePool != null ? typoFontStylePool.obtain() : TypoFontStyle.newInstance();
      let mut style: TypoFontStyle = if let Some(pool) = typoFontStylePool.as_mut() {
        pool.obtain_empty()
      } else {
        TypoFontStyle::newInstance()
      };
      // java int hint = propSet.getInt(40, -1);
      let hint: i32 = propSet.getInt__int_int(40, -1);
      // java switch (hint)
      match hint {
        // java case 1: langType = 2; break;
        1 => { langType = 2; }
        // java case 2: if (langType != 6 && langType != 7) { langType = 5; }
        2 => { if langType != 6 && langType != 7 { langType = 5; } }
        // java default: break;
        _ => {}
      }
      // java style.typeface = typeface;
      style.typeface = typeface.clone();
      // java style.emSize = typeface.getWriterLayoutEMSize();
      style.emSize = if let Some(tf) = &typeface { tf.getWriterLayoutEMSize() } else { 0.0 };
      // java style.srcEmSize = typeface.getSrcEmSize();
      style.srcEmSize = if let Some(tf) = &typeface { tf.getSrcEmSize() } else { 0.0 };
      // java style.fontSize = getFontSize(propSet, langType);
      style.fontSize = FontStyleFactory::getFontSize(propSet, langType);
      // java style.fontSpace = propSet.getInt(24, 0);
      style.fontSpace = propSet.getInt__int_int(24, 0);
      // java if (isInEmbedBalloonMode)
      if isInEmbedBalloonMode.load(Ordering::SeqCst) {
        // java float originFontSize = (float)style.fontSize; float scale = (float)mEmedFontSize / originFontSize;
        let originFontSize: f32 = style.fontSize as f32;
        let scale: f32 = mEmedFontSize.load(Ordering::SeqCst) as f32 / originFontSize;
        // java style.fontSize = mEmedFontSize;
        style.fontSize = mEmedFontSize.load(Ordering::SeqCst);
        // java style.fontSpace = (int)((float)style.fontSpace * scale);
        style.fontSpace = (style.fontSpace as f32 * scale) as i32;
      }
      // java style.fontColor = propSet.getInt(11, -16777216);
      style.fontColor = propSet.getInt__int_int(11, -16777216);
      // java style.charScale = propSet.getFloat(23, 1.0F);
      style.charScale = propSet.getFloat__int_float(23, 1.0);
      // java if (style.charScale <= 0.0F) { style.charScale = 1.0F; }
      if style.charScale <= 0.0 { style.charScale = 1.0; }
      // java boolean hasOutResult = null != styleResults;
      let hasOutResult: bool = styleResults.is_some();
      // java Integer emphasisMark = (Integer)propSet.getValue(12);
      let emphasisMark: Option<&util::moffice::property::PropertySet::PropertyValue> = propSet.getValue(12);
      let emphasisMarkInt: Option<i32> = emphasisMark.and_then(|v| v.as_any().downcast_ref::<i32>().copied());
      // java if (null != emphasisMark && 0 != emphasisMark)
      if let Some(emphasisMarkInt) = emphasisMarkInt {
        if 0 != emphasisMarkInt {
          // java style.setEmphasisMark(emphasisMark);
          style.setEmphasisMark(emphasisMarkInt);
        }
      }
      // java Integer strikeThrough = (Integer)propSet.getValue(7);
      let strikeThrough: Option<&util::moffice::property::PropertySet::PropertyValue> = propSet.getValue(7);
      let strikeThroughInt: Option<i32> = strikeThrough.and_then(|v| v.as_any().downcast_ref::<i32>().copied());
      // java if (null != strikeThrough && 0 != strikeThrough)
      if let Some(strikeThroughInt) = strikeThroughInt {
        if 0 != strikeThroughInt {
          // java if (hasOutResult) { styleResults.hasStrikeThroughs = true; }
          if hasOutResult { if let Some(res) = styleResults.as_mut() { res.hasStrikeThroughs = true; } }
          // java style.setStrikeThrough(strikeThrough);
          style.setStrikeThrough(strikeThroughInt);
        }
      }
      // java Integer underline = (Integer)propSet.getValue(8);
      let underline: Option<&util::moffice::property::PropertySet::PropertyValue> = propSet.getValue(8);
      let underlineInt: Option<i32> = underline.and_then(|v| v.as_any().downcast_ref::<i32>().copied());
      // java if (null != underline && underline != 0)
      if let Some(underlineInt) = underlineInt {
        if underlineInt != 0 {
          // java if (hasOutResult) { styleResults.hasUnderline = true; }
          if hasOutResult { if let Some(res) = styleResults.as_mut() { res.hasUnderline = true; } }
          // java style.setUnderline(underline);
          style.setUnderline(underlineInt);
          // java style.underlineColor = propSet.getInt(9, -16777216);
          style.underlineColor = propSet.getInt__int_int(9, -16777216);
        }
      }
      // java Integer highlight = (Integer)propSet.getValue(15);
      let highlight: Option<&util::moffice::property::PropertySet::PropertyValue> = propSet.getValue(15);
      let highlightInt: Option<i32> = highlight.and_then(|v| v.as_any().downcast_ref::<i32>().copied());
      // java if (null != highlight && highlight != 0)
      if let Some(highlightInt) = highlightInt {
        if highlightInt != 0 {
          // java if (hasOutResult) { styleResults.hasHighlight = true; }
          if hasOutResult { if let Some(res) = styleResults.as_mut() { res.hasHighlight = true; } }
          // java style.highlightColor = highlight;
          style.highlightColor = highlightInt;
        }
      }
      // java style.shd = (SHD)propSet.getValue(14);
      let shd: Option<SHD> = propSet.getValue(14).and_then(|v| v.as_any().downcast_ref::<SHD>().cloned());
      // java if (null != style.shd && hasOutResult) { styleResults.hasSHD = true; }
      if let Some(shd) = shd {
        style.shd = Some(shd);
        if hasOutResult { if let Some(res) = styleResults.as_mut() { res.hasSHD = true; } }
      }
      // java BRC brc = (BRC)propSet.getValue(68);
      let brc: Option<BRC> = propSet.getValue(68).and_then(|v| v.as_any().downcast_ref::<BRC>().cloned());
      // java if (brc != null && brc.getBorderType() != 0)
      if let Some(brc) = &brc {
        if brc.getBorderType() != 0 {
          // java style.brc = brc;
          style.brc = Some(brc.clone());
          // java if (style.brc != null && hasOutResult) { styleResults.hasBRC = true; }
          if hasOutResult { if let Some(res) = styleResults.as_mut() { res.hasBRC = true; } }
        }
      }
      // java style.hpsPos = UnitUtil.PT2TWIPS(-propSet.getFloat(25, 0.0F));
      style.hpsPos = UnitUtil::PT2TWIPS(-propSet.getFloat__int_float(25, 0.0));
      // java int verticalAlign = propSet.getInt(13, 0);
      let verticalAlign: i32 = propSet.getInt__int_int(13, 0);
      // java switch (verticalAlign)
      match verticalAlign {
        // java case 1:
        1 => {
          // java style.setSupScript(true);
          style.setSupScript(true);
          // java style.hpsPos = (int)((float)style.hpsPos - (getScriptFontsizeScale(langType) - 0.0F) * (float)style.fontSize);
          style.hpsPos = (style.hpsPos as f32 - (FontStyleFactory::getScriptFontsizeScale(langType) - 0.0) * style.fontSize as f32) as i32;
          // java style.fontSize = getSubOrSuperScriptFontSize(style.fontSize, typeface.getWriterLayoutSuperscriptFactor());
          let factor: f32 = if let Some(tf) = &typeface { tf.getWriterLayoutSuperscriptFactor() } else { 1.0 };
          style.fontSize = FontStyleFactory::getSubOrSuperScriptFontSize(style.fontSize, factor);
        }
        // java case 2:
        2 => {
          // java style.setSubScript(true);
          style.setSubScript(true);
          // java style.hpsPos = (int)((float)style.hpsPos - getSubposition(langType) * (float)style.fontSize);
          style.hpsPos = (style.hpsPos as f32 - FontStyleFactory::getSubposition(langType) * style.fontSize as f32) as i32;
          // java style.fontSize = getSubOrSuperScriptFontSize(style.fontSize, typeface.getWriterLayoutSubscriptFactor());
          let factor: f32 = if let Some(tf) = &typeface { tf.getWriterLayoutSubscriptFactor() } else { 1.0 };
          style.fontSize = FontStyleFactory::getSubOrSuperScriptFontSize(style.fontSize, factor);
        }
        // java default: break;
        _ => {}
      }
      // java if (typeface.hasKerning() && hasOutResult) { styleResults.hasTypefaceWithKerning = true; }
      if let Some(tf) = &typeface {
        if tf.hasKerning() && hasOutResult {
          if let Some(res) = styleResults.as_mut() { res.hasTypefaceWithKerning = true; }
        }
      }
      // java return style;
      style
    }

    // java public static TypoFontStyle create(TextDocument textDoc, PropertySet propSet, int langType, boolean isWord97LineRule, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)
    pub fn create(textDoc: Option<&TextDocument>, propSet: &PropertySet, langType: i32, isWord97LineRule: bool, mut styleResults: Option<&mut StyleResult>, mut typoFontStylePool: Option<&mut TypoFontStylePool>) -> TypoFontStyle {
      // java ITypeface typeface = createTypeface(textDoc, propSet, langType, styleResults.useBuildInFont);
      let canApplyBuildInFont: bool = styleResults.as_ref().map(|r| r.useBuildInFont).unwrap_or(true);
      let typeface: Option<Arc<dyn ITypeface>> = FontStyleFactory::createTypeface(textDoc, propSet, langType, canApplyBuildInFont);
      // java calcAscentAndDescent(style, isWord97LineRule, styleResults.ignoreBdr);
      let ignoreBdr: bool = styleResults.as_ref().map(|r| r.ignoreBdr).unwrap_or(false);
      // java TypoFontStyle style = createFontStyle(propSet, typeface, langType, styleResults, typoFontStylePool);
      let mut style: TypoFontStyle = FontStyleFactory::createFontStyle(propSet, typeface, langType, styleResults.take(), typoFontStylePool.take());
      FontStyleFactory::calcAscentAndDescent(&mut style, isWord97LineRule, ignoreBdr);
      // java return style;
      style
    }

    // java private static float getScriptFontsizeScale(int langType)
    pub fn getScriptFontsizeScale(langType: i32) -> f32 {
      // java switch (langType)
      match langType {
        // java case 7:
        7 => {
          // java return 0.4F;
          return 0.4;
        }
        // java default:
        _ => {
          // java return 0.5F;
          return 0.5;
        }
      }
    }

    // java private static int getSubOrSuperScriptFontSize(int orgFontSize, float factor)
    // java private static int getSubOrSuperScriptFontSize(int orgFontSize, float factor)
    pub fn getSubOrSuperScriptFontSize(orgFontSize: i32, factor: f32) -> i32 {
      // java return (int)((float)orgFontSize * factor);
      (orgFontSize as f32 * factor) as i32
    }

    // java private static TypoFontStyle createFontStyle(PropertySet propSet, ITypeface typeface, int langType, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)


    // java public static TypoFontStyle create(TextDocument textDoc, PropertySet propSet, int langType, boolean isWord97LineRule, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)


    // java private static void calcAscentAndDescent(TypoFontStyle style, boolean isWord97LineRule, boolean egnoreBdr)
    // java private static void calcAscentAndDescent(TypoFontStyle style, boolean isWord97LineRule, boolean egnoreBdr)
    pub fn calcAscentAndDescent(style: &mut TypoFontStyle, isWord97LineRule: bool, egnoreBdr: bool) {
      // java ITypeface typeface = style.typeface;
      let typeface: Option<Arc<dyn ITypeface>> = style.typeface.clone();
      let Some(typeface) = typeface else { return };
      // java int fontSize = style.fontSize;
      let fontSize: i32 = style.fontSize;
      // java boolean isCJKFont = typeface.isCJKFont();
      let isCJKFont: bool = typeface.isCJKFont();
      // java float emSize = style.emSize;
      let emSize: f32 = style.emSize;
      // java float ascent = typeface.getWriterLayoutAscent();
      let mut ascent: f32 = typeface.getWriterLayoutAscent();
      // java float descent = typeface.getWriterLayoutDescent();
      let mut descent: f32 = typeface.getWriterLayoutDescent();
      // java float external = GraphAlignTool.getGraphExternal(typeface, ascent, descent, isCJKFont, false);
      let external: f32 = GraphAlignTool::getGraphExternal(&typeface, ascent, descent, isCJKFont, false);
      // java if (style.brc != null && !egnoreBdr)
      if style.brc.is_some() && !egnoreBdr {
        // java style.bdrWidth = GraphAlignTool.getLineWidth(style.brc) * 20.0F;
        style.bdrWidth = GraphAlignTool::getLineWidth(style.brc.as_ref().unwrap()) * 20.0;
        // java float emBdrWith = style.bdrWidth * emSize / (float)fontSize;
        let emBdrWith: f32 = style.bdrWidth * emSize / fontSize as f32;
        // java ascent += emBdrWith; descent += emBdrWith;
        ascent += emBdrWith;
        descent += emBdrWith;
      }
      // java style.maxAscent = (int)((ascent + external) * (float)fontSize / emSize + 0.5F);
      // java style.maxDescent = (int)(descent * (float)fontSize / emSize + 0.5F);
      // java style.external = (int)(external * (float)fontSize / emSize + 0.5F);
      style.maxAscent = ((ascent + external) * fontSize as f32 / emSize + 0.5) as i32;
      style.maxDescent = (descent * fontSize as f32 / emSize + 0.5) as i32;
      style.external = (external * fontSize as f32 / emSize + 0.5) as i32;
    }

    // java public static TypoFontStyle createMathFontStyle(PropertySet runProp, int langType, boolean isWord97LineRule, StyleResult styleResults, TypoFontStyle.TypoFontStylePool typoFontStylePool)
    pub fn createMathFontStyle(runProp: &PropertySet, langType: i32, isWord97LineRule: bool, styleResults: Option<&mut StyleResult>, typoFontStylePool: Option<&mut TypoFontStylePool>) -> TypoFontStyle {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ITypeface typeface = createMathTypeface(runProp, langType);
      let mut typeface: Box<dyn ITypeface> = { let __x: Box<Box<dyn ITypeface>> = todo!("no align strategy matched"); __x };
      // java TypoFontStyle style = createFontStyle(runProp, typeface, langType, styleResults, typoFontStylePool);
      let mut style: TypoFontStyle = Self::createFontStyle(runProp, typeface.as_ref(), langType, styleResults, typoFontStylePool);
      // java calcAscentAndDescent(style, isWord97LineRule, styleResults.ignoreBdr);
      Self::calcAscentAndDescent(&mut style, isWord97LineRule, styleResults.ignoreBdr);
      // java return style;
      return style;
      */
    }

    // java public static int getFontSize(PropertySet propSet, int langType)
    pub fn getFontSize(propSet: &PropertySet, langType: i32) -> i32 {
      // java int fontSize = -1;
      let mut fontSize: i32 = -1;
      // java if (isComplexScript(langType))
      if FontStyleFactory::isComplexScript(langType) {
        // java fontSize = UnitUtil.PT2TWIPS(propSet.getFloat(37, -1.0F));
        fontSize = UnitUtil::PT2TWIPS(propSet.getFloat__int_float(37, -1.0));
      }
      // java if (fontSize <= 0)
      if fontSize <= 0 {
        // java fontSize = UnitUtil.PT2TWIPS(propSet.getFloat(10, 10.0F));
        fontSize = UnitUtil::PT2TWIPS(propSet.getFloat__int_float(10, 10.0));
      }
      // java if (SmartFontSize)
      if SmartFontSize.load(Ordering::SeqCst) {
        // java fontSize = SmartLayout.getSmartFontSize(fontSize);
        // rust: no-stmt
      }
      // java else
      else {
        // java fontSize = Math.min(fontSize, MAX_FONT_SIZE);
        fontSize = fontSize.min(MAX_FONT_SIZE.load(Ordering::SeqCst));
      }
      // java if (fontSize <= 0)
      if fontSize <= 0 {
        // java fontSize = UnitUtil.PT2TWIPS(10.0F);
        fontSize = UnitUtil::PT2TWIPS(10.0);
      }
      // java return fontSize;
      return fontSize;
    }

    // java protected static boolean isComplexScript(int langType)
    pub fn isComplexScript(langType: i32) -> bool {
      // java return TypoRun.isComplexLangType(langType);
      return TypoRun::isComplexLangType(langType);
    }

    // java public static void updateStyleResult(TypoFontStyle style, StyleResult res)
    pub fn updateStyleResult(style: &TypoFontStyle, res: Option<&mut StyleResult>) {
      // java if (null != res)
      if let Some(res) = res {
        // java if (style.flag != 0)
        if style.flag != 0 {
          // java if (style.getStrikeThrough() != 0) { res.hasStrikeThroughs = true; }
          if style.getStrikeThrough() != 0 { res.hasStrikeThroughs = true; }
          // java if (style.getUnderline() != 0) { res.hasUnderline = true; }
          if style.getUnderline() != 0 { res.hasUnderline = true; }
        }
        // java if (style.highlightColor != 0) { res.hasHighlight = true; }
        if style.highlightColor != 0 { res.hasHighlight = true; }
        // java if (style.shd != null) { res.hasSHD = true; }
        if style.shd.is_some() { res.hasSHD = true; }
        // java if (style.brc != null) { res.hasBRC = true; }
        if style.brc.is_some() { res.hasBRC = true; }
        // java if (style.typeface.hasKerning()) { res.hasTypefaceWithKerning = true; }
        if style.typeface.as_ref().map(|t| t.hasKerning()).unwrap_or(false) { res.hasTypefaceWithKerning = true; }
      }
    }





    /// wr-10 I/O: min TypoFontStyle without FontHost (not a Java overload).
    /// rust: 用 env::TypoFontStyle（wr-10 布局最小面，非 Java 类；Java 无此方法）
    pub fn createMin(propSet: &PropertySet, langType: i32) -> super::TypoFontStyle::TypoFontStyle {
        let langType = super::FontNames::FontNames::applyHint(propSet, langType);
        let mut style = super::TypoFontStyle::TypoFontStyle::TypoFontStyle();
        style.ascii = super::FontNames::FontNames::getAscii(propSet);
        style.eastAsia = super::FontNames::FontNames::getEastAsia(propSet);
        style.fontName = super::FontNames::FontNames::getResolvedFontName(propSet, langType);
        style.fontSize = FontStyleFactory::getFontSize(propSet, langType);
        style.bold = super::RunFlags::RunFlags::getBold(propSet, langType);
        style.italic = super::RunFlags::RunFlags::getItalic(propSet, langType);
        style.fontColor = propSet.getInt__int_int(11, -16777216);
        style.underline = propSet.getInt__int_int(8, 0);
        style
    }

    /// wr-10 I/O: merge then createMin (not a Java method).
    pub fn createFromRun(
        chpx: &PropertySet,
        papx: &PropertySet,
        styles: Option<&crate::cn::wps::moffice::writer::core::KStyles::KStyles>,
        langType: i32,
    ) -> super::TypoFontStyle::TypoFontStyle {
        let merged = super::StyleMerge::StyleMerge::merge(chpx, papx, styles);
        Self::createMin(&merged, langType)
    }

}

// java public static class StyleResult
impl StyleResult {
    // java public void reset()
    pub fn reset(&mut self) {
      // java this.hasHighlight = false;
      self.hasHighlight = false;
      // java this.hasSHD = false;
      self.hasSHD = false;
      // java this.hasBRC = false;
      self.hasBRC = false;
      // java this.hasUnderline = false;
      self.hasUnderline = false;
      // java this.hasStrikeThroughs = false;
      self.hasStrikeThroughs = false;
      // java this.hasTypefaceWithKerning = false;
      self.hasTypefaceWithKerning = false;
      // java this.useBuildInFont = true;
      self.useBuildInFont = true;
    }

}


    impl FontStyleFactory {
// java private static float getSubposition(int langType)
    pub fn getSubposition(langType: i32) -> f32 {
      // java switch (langType)
      match langType {
        // java case 2:
        // java case 9:
        2 | 9 => {
          // java return 0.35F;
          return 0.35;
        }
        // java case 5:
        // java case 6:
        5 | 6 => {
          // java return 0.2F;
          return 0.2;
        }
        // java case 7:
        7 => {
          // java return 0.25F;
          return 0.25;
        }
        // java default:
        _ => {
          // java return 0.4F;
          return 0.4;
        }
      }
    }

    // java private static ITypeface createTypeface(TextDocument textDoc, PropertySet propSet, int langType, boolean canApplyBuildInFont)
    // rust: ★wr-29 04 —— FontHost.getFontTable() 以 Rust 静态表对等（layout::text::FontHost），
    //       family.getTypeface(fontStyle) = WriterTypeface（tools::TypefaceTool）
    pub fn createTypeface(_textDoc: Option<&TextDocument>, propSet: &PropertySet, langType: i32, canApplyBuildInFont: bool) -> Option<Arc<dyn ITypeface>> {
      // java int fontStyle = 0;
      let mut fontStyle: i32 = 0;
      // java boolean bold = false; boolean italic = false;
      let mut bold: bool = false;
      let mut italic: bool = false;
      // java if (isComplexScript(langType))
      if FontStyleFactory::isComplexScript(langType) {
        // java bold = propSet.getBoolean(38, false);
        bold = propSet.getBoolean__int_boolean(38, false);
        // java italic = propSet.getBoolean(39, false);
        italic = propSet.getBoolean__int_boolean(39, false);
      } else {
        // java bold = propSet.getBoolean(5, false);
        bold = propSet.getBoolean__int_boolean(5, false);
        // java italic = propSet.getBoolean(6, false);
        italic = propSet.getBoolean__int_boolean(6, false);
      }
      // java if (bold) { fontStyle |= 1; }
      if bold { fontStyle |= 1; }
      // java if (italic) { fontStyle |= 2; }
      if italic { fontStyle |= 2; }
      let _ = fontStyle;
      // java String fontName = "Times New Roman";
      let mut fontName: String = "Times New Roman".to_string();
      // java switch (langType)
      match langType {
        // java case 2:
        // java case 3:
        // java   fontName = propSet.getString(4, "宋体"); break;
        2 | 3 => { fontName = propSet.getString__int_string(4, "宋体"); }
        // java case 4:
        // java case 7:
        // java case 8:
        // java   fontName = propSet.getString(36, "Times New Roman"); break;
        // java case 5:
        // java   fontName = propSet.getString(36, "Times New Roman"); break;
        4 | 5 | 7 | 8 => { fontName = propSet.getString__int_string(36, "Times New Roman"); }
        // java case 6:
        6 => {
          // java if (propSet.contains(36)) { fontName = propSet.getString(36); }
          if propSet.contains(36) {
            fontName = propSet.getString(36);
          // java else if (propSet.contains(35)) { fontName = propSet.getString(35); }
          } else if propSet.contains(35) {
            fontName = propSet.getString(35);
          // java else { fontName = propSet.getString(3, "Times New Roman"); }
          } else {
            fontName = propSet.getString__int_string(3, "Times New Roman");
          }
        }
        // java case 9:
        // java   fontName = propSet.getString(35, "Times New Roman"); break;
        9 => { fontName = propSet.getString__int_string(35, "Times New Roman"); }
        // java default:
        // java   fontName = propSet.getString(3, "Times New Roman");
        _ => { fontName = propSet.getString__int_string(3, "Times New Roman"); }
      }
      // java if (canApplyBuildInFont && propSet.getValue(105) != null) { canApplyBuildInFont = false; }
      // java IFontTable fontHostFontTable = FontHost.getFontTable();
      // java IFontFamily fontFamily = fontHostFontTable.get(fontName, false);
      // java if (null == fontFamily) { ... per-lang default family ... }
      // rust: FontHost.get_font_table()（Rust 静态表），保留 Java 内置族名。
      let stack = crate::cn::wps::moffice::writer::layout::text::FontHost::get_font_table();
      let Some(stack) = stack else { return None; };
      // java if (canApplyBuildInFont) { fontFamily = fontHostFontTable.getBuildinFontFamily(fontName, fontFamily); }
      // java if (null == fontFamily) { fontFamily = fontHostFontTable.getDefault(); }
      // Keep a known Java built-in family name even when FontStack only has a
      // substitute glyph file; dropping it loses BuildinTTFHeader metrics.
      let is_system_family = matches!(fontName.as_str(), "serif" | "sans-serif" | "monospace");
      let resolved = if stack.name_hits_file(&fontName) || is_system_family || (canApplyBuildInFont
          && !propSet.contains(105)
          && thirdparty_font::cn::wps::font::r#impl::BuildinTTFHeader::BuildinTTFHeader::getFont(&fontName).is_some()) {
          fontName.clone()
      } else {
          String::new()
      };
      // java return fontFamily.getTypeface(fontStyle);
      Some(WriterTypeface::new(resolved, bold, italic, langType == 2 || langType == 3,
          canApplyBuildInFont && !propSet.contains(105)))
    }

    // java public static ITypeface createMathTypeface(PropertySet propSet, int langType)
    pub fn createMathTypeface(prop_set: &PropertySet, lang_type: i32) -> Option<Arc<dyn ITypeface>> {
      let mut font_style = 0;
      if prop_set.getBoolean__int_boolean(5, false) {
        font_style |= 1;
      }
      if prop_set.getBoolean__int_boolean(6, false) {
        font_style |= 2;
      }
      let font_name = match lang_type {
        2 | 3 => prop_set.getString__int_string(4, "宋体"),
        4 | 7 => prop_set.getString__int_string(35, "Times New Roman"),
        5 => prop_set.getString__int_string(36, "Times New Roman"),
        _ => prop_set.getString__int_string(3, "Times New Roman"),
      };
      let math_style = if prop_set.getBoolean__int_boolean(156, false) {
        0
      } else {
        match prop_set.getInt__int_int(152, 3) {
          1 => 2,
          2 => 4,
          3 => 3,
          _ => 1,
        }
      };
      let table = thirdparty_font::cn::wps::font::FontHost::FontHost::getFontTable();
      let family = thirdparty_font::cn::wps::font::IFontTable::IFontTable::getMathFontFamily(table.as_ref(), &font_name, true)?;
      thirdparty_font::cn::wps::font::IMathFontFamily::IMathFontFamily::getTypeface(family.as_ref(), math_style, font_style)
    }
}
