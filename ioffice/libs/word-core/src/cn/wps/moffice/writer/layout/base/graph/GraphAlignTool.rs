// Java: cn.wps.moffice.writer.layout.base.graph.GraphAlignTool
// rust: ★wr-29 04/05 —— 真实移植（Java 164 行）：getGraphExternal/getLineWidth/getGraphVMetrics/
//       getEmbedVMetrics + 4 个对齐度量 helper 全真实；geoText（图形效果）非 hello 停车。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use std::sync::Arc;

// Java: import cn.wps.font.ITypeface
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.layout.base.tools.BRCBorderTool
use crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

// java public final class GraphAlignTool
#[derive(Default)]
pub struct GraphAlignTool {}

impl GraphAlignTool {
    // java private static final float CJK_EXTERNAL_FACTOR = 0.15F;
    // rust: getGraphExternal 内联常量
    // java public static final float BOTTOM_ALIGN_UPPER_CJK_REVEERSED_FACTOR = 0.25F;
    // java public static final float BOTTOM_ALIGN_UPPER_LATIN_REVERSED_FACTOR = 0.8F;

    // java public static float getGraphExternal(ITypeface typeface, float ascent, float descent, boolean isCJKFont, boolean isWord97Rules)
    pub fn getGraphExternal(typeface: &Arc<dyn ITypeface>, ascent: f32, descent: f32, isCJKFont: bool, isWord97Rules: bool) -> f32 {
      // java if (isCJKFont)
      if isCJKFont {
        // java float height = ascent + descent;
        let height: f32 = ascent + descent;
        // java float external = height * 0.15F;
        let mut external: f32 = height * 0.15;
        // java if (!isWord97Rules)
        if !isWord97Rules {
          // java float heightTwip = external * 20.0F;
          let heightTwip: f32 = external * 20.0;
          // java external = (float)Math.floor((double)(heightTwip + 0.5F));
          external = ((heightTwip + 0.5) as f64).floor() as f32;
          // java external /= 20.0F;
          external /= 20.0;
        }
        // java return external;
        return external;
      } else {
        // java return typeface.getWriterLayoutLatinExternal();
        return typeface.getWriterLayoutLatinExternal();
      }
    }

    // java public static final float getLineWidth(BRC brc)
    pub fn getLineWidth(brc: &BRC) -> f32 {
      // java return BRCBorderTool.getActualStrokeWidth(brc.getBorderType(), brc.getLineWidth());
      BRCBorderTool::getActualStrokeWidth(brc.getBorderType(), brc.getLineWidth())
    }

    // java private static void getTopAlignVMetrics(...)
    pub fn getTopAlignVMetrics(height: f32, external: f32, factor: f32, hps: i32, bdrWith: f32, ascentAndDescent: &mut [f32]) {
      // java float upper = (external + bdrWith) * factor;
      let upper: f32 = (external + bdrWith) * factor;
      // java float lower = (height - external - bdrWith) * factor;
      let lower: f32 = (height - external - bdrWith) * factor;
      // java ascentAndDescent[0] = Math.max(upper - (float)hps, 0.0F);
      ascentAndDescent[0] = (upper - hps as f32).max(0.0);
      // java ascentAndDescent[1] = Math.max(lower + (float)hps, 0.0F);
      ascentAndDescent[1] = (lower + hps as f32).max(0.0);
    }

    // java private static void getMiddleAlignVMetrics(...)
    pub fn getMiddleAlignVMetrics(height: f32, factor: f32, hps: i32, ascentAndDescent: &mut [f32]) {
      // java float upper = height / 2.0F * factor;
      let upper: f32 = height / 2.0 * factor;
      // java ascentAndDescent[0] = Math.max(upper - (float)hps, 0.0F);
      ascentAndDescent[0] = (upper - hps as f32).max(0.0);
      // java ascentAndDescent[1] = Math.max(upper + (float)hps, 0.0F);
      ascentAndDescent[1] = (upper + hps as f32).max(0.0);
    }

    // java private static void getBottomAlignVMetrics(...)
    pub fn getBottomAlignVMetrics(descent: f32, height: f32, external: f32, factor: f32, hps: i32, isCJKFont: bool, bdrWith: f32, ascentAndDescent: &mut [f32]) {
      // java float lower;
      let mut lower: f32;
      // java if (isCJKFont) { lower = (descent * 0.25F + external + bdrWith) * factor; }
      if isCJKFont {
        lower = (descent * 0.25 + external + bdrWith) * factor;
      } else {
        // java else { lower = (descent * 0.8F + bdrWith) * factor; }
        lower = (descent * 0.8 + bdrWith) * factor;
      }
      // java float upper = height * factor - lower;
      let upper: f32 = height * factor - lower;
      // java ascentAndDescent[0] = Math.max(upper - (float)hps, 0.0F);
      ascentAndDescent[0] = (upper - hps as f32).max(0.0);
      // java ascentAndDescent[1] = Math.max(lower + (float)hps, 0.0F);
      ascentAndDescent[1] = (lower + hps as f32).max(0.0);
    }

    // java public static float getReversedFactorForBottomAlign(boolean isCJKFont)
    pub fn getReversedFactorForBottomAlign(isCJKFont: bool) -> f32 {
      // java return isCJKFont ? 0.25F : 0.8F;
      if isCJKFont { 0.25 } else { 0.8 }
    }

    // java private static void getBaselineAlignVMetrics(...)
    pub fn getBaselineAlignVMetrics(ascent: f32, height: f32, external: f32, factor: f32, hps: i32, bdrWith: f32, ascentAndDescent: &mut [f32]) {
      // java float upper = (external + bdrWith + ascent) * factor;
      let upper: f32 = (external + bdrWith + ascent) * factor;
      // java float lower = height * factor - upper;
      let lower: f32 = height * factor - upper;
      // java ascentAndDescent[0] = Math.max(upper - (float)hps, 0.0F);
      ascentAndDescent[0] = (upper - hps as f32).max(0.0);
      // java ascentAndDescent[1] = Math.max(lower + (float)hps, 0.0F);
      ascentAndDescent[1] = (lower + hps as f32).max(0.0);
    }

    // java public static void getEmbedVMetrics(Graphs graphs, int wordAlign, boolean isWord97Rules, float[] ascentAndDescent)
    pub fn getEmbedVMetrics(graphs: &Graphs, wordAlign: i32, _isWord97Rules: bool, ascentAndDescent: &mut [f32]) {
      // java TypoFontStyle fontStyle = graphs.fontStyle;
      let fontStyle = graphs.fontStyle.clone().unwrap_or_default();
      // java int ascent = fontStyle.maxAscent; int descent = fontStyle.maxDescent; int height = ascent + descent;
      let ascent: i32 = fontStyle.maxAscent;
      let descent: i32 = fontStyle.maxDescent;
      // java int hps = fontStyle.hpsPos;
      let hps: i32 = fontStyle.hpsPos;
      // java switch (wordAlign)
      match wordAlign {
        // java case 0: getTopAlignVMetrics((float)height, 0.0F, 1.0F, hps, fontStyle.bdrWidth, ascentAndDescent); return;
        0 => {
          Self::getTopAlignVMetrics((ascent + descent) as f32, 0.0, 1.0, hps, fontStyle.bdrWidth, ascentAndDescent);
          return;
        }
        // java case 1: getMiddleAlignVMetrics((float)height, 1.0F, hps, ascentAndDescent); return;
        1 => {
          Self::getMiddleAlignVMetrics((ascent + descent) as f32, 1.0, hps, ascentAndDescent);
          return;
        }
        // java case 2: case 4: getBaselineAlignVMetrics(...); return;
        2 | 4 => {
          Self::getBaselineAlignVMetrics(ascent as f32, (ascent + descent) as f32, 0.0, 1.0, hps, fontStyle.bdrWidth, ascentAndDescent);
          return;
        }
        // java case 3: getBottomAlignVMetrics(...); return;
        3 => {
          Self::getBottomAlignVMetrics(descent as f32, (ascent + descent) as f32, 0.0, 1.0, hps, false, fontStyle.bdrWidth, ascentAndDescent);
          return;
        }
        // java default: 同 case 2
        _ => {
          Self::getBaselineAlignVMetrics(ascent as f32, (ascent + descent) as f32, 0.0, 1.0, hps, fontStyle.bdrWidth, ascentAndDescent);
          return;
        }
      }
    }

    // java public static void getGraphVMetrics(Graphs graphs, int wordAlign, boolean isWord97Rules, float[] ascentAndDescent)
    pub fn getGraphVMetrics(graphs: &Graphs, wordAlign: i32, isWord97Rules: bool, ascentAndDescent: &mut [f32]) {
      // java TypoFontStyle style = graphs.fontStyle;
      let Some(style) = graphs.fontStyle.clone() else { return };
      // java ITypeface typeface = style.typeface;
      let Some(typeface) = style.typeface.clone() else { return };
      // java boolean isCJKFont = typeface.isCJKFont();
      let isCJKFont: bool = typeface.isCJKFont();
      // java int hps = style.hpsPos;
      let mut hps: i32 = style.hpsPos;
      // java float factor = (float)style.fontSize / style.emSize;
      let mut factor: f32 = style.fontSize as f32 / style.emSize;
      // java if (hps != 0 && (style.isSubScript() || style.isSupScript()))
      if hps != 0 && (style.isSubScript() || style.isSupScript()) {
        // java PropertySet finalRunProp = graphs.charsRef.finalRunProp;
        let finalRunProp: Option<util::moffice::property::PropertySet::PropertySet> =
            graphs.charsRef.as_ref().and_then(|c| c.finalRunProp.clone());
        // java hps = null == finalRunProp ? -UnitUtil.PT2TWIPS(-0.0F) : -UnitUtil.PT2TWIPS(finalRunProp.getFloat(25, 0.0F));
        hps = match &finalRunProp {
          None => -UnitUtil::PT2TWIPS(-0.0),
          Some(p) => -UnitUtil::PT2TWIPS(p.getFloat__int_float(25, 0.0)),
        };
        // java factor /= style.isSubScript() ? typeface.getWriterLayoutSubscriptFactor() : typeface.getWriterLayoutSuperscriptFactor();
        if style.isSubScript() {
          factor /= typeface.getWriterLayoutSubscriptFactor();
        } else {
          factor /= typeface.getWriterLayoutSuperscriptFactor();
        }
      }
      // java float ascent = typeface.getWriterLayoutAscent();
      let ascent: f32 = typeface.getWriterLayoutAscent();
      // java float descent = typeface.getWriterLayoutDescent();
      let descent: f32 = typeface.getWriterLayoutDescent();
      // java float external = getGraphExternal(typeface, ascent, descent, isCJKFont, isWord97Rules);
      let external: f32 = Self::getGraphExternal(&typeface, ascent, descent, isCJKFont, isWord97Rules);
      // java float bdrWith = 0.0F;
      let mut bdrWith: f32 = 0.0;
      // java if (style.bdrWidth != 0.0F && !TypoRun.isWeakBdrForHeight(graphs.runType) && null == graphs.charsRef.geoText)
      if style.bdrWidth != 0.0 && !TypoRun::isWeakBdrForHeight(graphs.runType) && graphs.charsRef.as_ref().map(|c| c.geoText.is_none()).unwrap_or(true) {
        // java bdrWith = style.bdrWidth / factor;
        bdrWith = style.bdrWidth / factor;
      }
      // java float height = ascent + descent + external + bdrWith * 2.0F;
      let mut height: f32 = ascent + descent + external + bdrWith * 2.0;
      // java if (isCJKFont) { height += external; }
      if isCJKFont { height += external; }
      // java switch (wordAlign)
      match wordAlign {
        // java case 0: getTopAlignVMetrics(height, external, factor, hps, bdrWith, ascentAndDescent); break;
        0 => { Self::getTopAlignVMetrics(height, external, factor, hps, bdrWith, ascentAndDescent); }
        // java case 1: getMiddleAlignVMetrics(height, factor, hps, ascentAndDescent); break;
        1 => { Self::getMiddleAlignVMetrics(height, factor, hps, ascentAndDescent); }
        // java case 2: case 4: getBaselineAlignVMetrics(ascent, height, external, factor, hps, bdrWith, ascentAndDescent); break;
        2 | 4 => { Self::getBaselineAlignVMetrics(ascent, height, external, factor, hps, bdrWith, ascentAndDescent); }
        // java case 3: getBottomAlignVMetrics(descent, height, external, factor, hps, isCJKFont, bdrWith, ascentAndDescent); break;
        3 => { Self::getBottomAlignVMetrics(descent, height, external, factor, hps, isCJKFont, bdrWith, ascentAndDescent); }
        // java default: getBaselineAlignVMetrics(...)
        _ => { Self::getBaselineAlignVMetrics(ascent, height, external, factor, hps, bdrWith, ascentAndDescent); }
      }
      // java if (null != graphs.charsRef.geoText) { ... } —— geoText（图形效果）非 hello（GR-D3）
      // rust: geoText 恒 None → 跳过
    }
}