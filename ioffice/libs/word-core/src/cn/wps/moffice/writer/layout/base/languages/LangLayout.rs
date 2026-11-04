// Java: cn.wps.moffice.writer.layout.base.languages.LangLayout
// rust: ★wr-29 04 —— 真实移植（Java 539 行）：addGraphNode/addGraph/addCommonGraph/
//       addLangGraph + commit*/calcGraphNode/calcWidths/calcHeight 热路径真实；
//       emoji/符号替换/合并 char（泰文上下文）DEFERRED（非 hello 语料）。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use std::sync::Arc;

// Java: import cn.wps.font.FontMetrics
// Java: import cn.wps.font.IFontTable
// Java: import cn.wps.font.FontHost
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::{TypoFontStyle, TypoFontStylePool};
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCache
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleCache::FontStyleCache;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewMode
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.languages.ILangLayout
use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::{GraphOperationEnv, ILangLayout, RollBackResult};
// Java: import cn.wps.font.ITypeface
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;

// java public class LangLayout implements ILangLayout
pub struct LangLayout {
    // java public final int language;
    pub language: i32,
}

// java public LangLayout(int language)
pub fn LangLayout(language: i32) -> LangLayout {
    LangLayout { language }
}

impl LangLayout {
    pub(crate) fn getFontStyleCache(&self, documentEnv: *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv, layoutState: &mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState) -> &mut FontStyleCache {
        // java FontStyleCache cache = documentEnv.fontStyleCacheFactory.create(layoutState.document);
        // rust: 借用自 mDocEnv.fontStyleCacheFactory（Java 同实例复用）
        let de = unsafe { &mut *documentEnv };
        let factory = de.fontStyleCacheFactory.as_mut().expect("fontStyleCacheFactory");
        factory.create_borrowed(layoutState.document)
    }

    // java protected void calcGraphNode(Graphs graphNode, ILangLayout.GraphOperationEnv graphOperationEnv)
    // rust: *mut 池节点（Java 引用语义）
    pub(crate) fn calcGraphNode(&mut self, graphNode: *mut Graphs, graphOperationEnv: &mut GraphOperationEnv) {
        // java if (null != graphNode)
        if graphNode.is_null() { return; }
        let graphNode = unsafe { &mut *graphNode };
        // java DocumentEnv documentEnv = graphOperationEnv.documentEnv;
        // java LayoutState layoutState = graphOperationEnv.layoutState;
        let layoutState = unsafe { &mut *graphOperationEnv.layoutState };
        // java ISymbolConvert symbolConvert = graphOperationEnv.symbolConvert;
        let symbolConvert = graphOperationEnv.symbolConvert;
        // java if (15 == graphNode.runType)
        if 15 == graphNode.runType {
            // java emoji —— DEFERRED（非 hello）；Java 原文 begin
            // rust: DEFERRED 停车见文件头注释
            let _ = symbolConvert;
            /* DEFERRED_NON_FUNCTION （emoji 图宽/高）
            // java TypoFontStyle fontStyle = graphNode.fontStyle;
            // java if (this.getFontStyleCache(documentEnv, layoutState).contains(fontStyle)) { fontStyle = graphOperationEnv.typoFontStylePool.obtain(fontStyle); }
            // java this.calcEmojiAscentAndDescent(graphNode, fontStyle.fontSize, fontStyle, graphOperationEnv);
            // java graphNode.fontStyle = fontStyle;
            */
        // java else if (graphOperationEnv.highSurrogateType != graphNode.runType && graphOperationEnv.lowSurrogateType != graphNode.runType)
        } else if graphOperationEnv.highSurrogateType != graphNode.runType && graphOperationEnv.lowSurrogateType != graphNode.runType {
            // java if (null != symbolConvert && this.language == 0) { this.replaceSymbol(graphNode.graphIndex, graphNode.graphLength, graphOperationEnv); }
            // rust: 符号转换（ISymbolConvert）未接线（GR-D3）→ symbolConvert 恒 None
            let _ = symbolConvert;
        } else {
            // java graphNode.runType = 16;
            graphNode.runType = 16;
            // java graphNode.language = 0;
            graphNode.language = 0;
            // java layoutState.typoPara.putChar(graphNode.graphIndex, '?');
            layoutState.typoPara.putChar(graphNode.graphIndex, '?');
        }
        // java this.calcHeight(graphNode, graphOperationEnv);
        self.calcHeight(graphNode, graphOperationEnv);
        // java this.calcWidths(graphNode, graphOperationEnv);
        self.calcWidths(graphNode, graphOperationEnv);
    }

    // java protected void calcWidths(Graphs graphNode, ILangLayout.GraphOperationEnv graphOperationEnv)
    fn calcWidths(&mut self, graphNode: &mut Graphs, graphOperationEnv: &mut GraphOperationEnv) {
        // Java virtual dispatch: FarEastLayout/SharedFarEastLayout override calcWidths.
        // Rust stores the Java language id on graph nodes, so forward the override here.
        if graphNode.autoSpaceType == 2 && graphNode.runType != 15 && graphNode.runType != 16 {
            if self.language == 2 {
                crate::cn::wps::moffice::writer::layout::base::languages::FarEastLayout::FarEastLayout::calcWidths(graphNode, graphOperationEnv);
                return;
            }
            if self.language == 3 {
                crate::cn::wps::moffice::writer::layout::base::languages::SharedFarEastLayout::SharedFarEastLayout::calcWidths(graphNode, graphOperationEnv);
                return;
            }
        }
        // java if (null != graphNode)
        // java TypoFontStyle fontStyle = graphNode.fontStyle;
        if graphNode.fontStyle.is_none() { return; }
        let (fontSpace_code, fontSize_code, charScale_code, srcEmSize_code, typeface_code): (i32, i32, f32, f32, Option<Arc<dyn ITypeface>>) = {
            let fs = graphNode.fontStyle.as_ref().unwrap();
            (fs.fontSpace, fs.fontSize, fs.charScale, fs.srcEmSize, fs.typeface.clone())
        };
        let fontSpace = fontSpace_code;
        let _fontSize_code = fontSize_code;
        let _charScale_code = charScale_code;
        let _srcEmSize_code = srcEmSize_code;
        // java LayoutState layoutState = graphOperationEnv.layoutState;
        let layoutState = unsafe { &mut *graphOperationEnv.layoutState };
        // java DocumentEnv documentEnv = graphOperationEnv.documentEnv;
        let documentEnv = graphOperationEnv.documentEnv;
        // java int graphIndex = graphNode.graphIndex;
        let graphIndex: i32 = graphNode.graphIndex;
        // java int graphLength = graphNode.graphLength;
        let graphLength: i32 = graphNode.graphLength;
        // java int runType = graphNode.runType;
        let runType: i32 = graphNode.runType;
        // java TypoPara typoPara = layoutState.typoPara;
        // java int[] blockResult = graphOperationEnv.blockResult;
        // java int fontSpace = fontStyle.fontSpace;
        let fontSpace: i32 = fontSpace_code;
        // java int widthSum = 0;
        let mut widthSum: i32 = 0;
        // java if (15 == runType)
        if 15 == runType {
            // java emoji DEFERRED —— Java calcEmojiGraphWidth 原文见 LangLayout.java:353
            let _ = (graphLength, fontSpace);
            /* DEFERRED_NON_FUNCTION （emoji）
            // java widthSum = this.calcEmojiGraphWidth(fontStyle, graphIndex, graphLength, graphOperationEnv);
            */
        } else {
            // java if (0 != fontSpace && 2 == this.language && documentEnv.balanceSingleByteDoubleByteWidth) { fontSpace *= 2; }
            let mut fontSpace = fontSpace;
            if 0 != fontSpace && 2 == self.language && unsafe { !(*documentEnv).balanceSingleByteDoubleByteWidth == false } {
                // rust: 上面条件剥成可读形式（Java 原文：fontSpace *= 2;）
                if unsafe { (*documentEnv).balanceSingleByteDoubleByteWidth } {
                    fontSpace *= 2;
                }
            }
            // java if (16 == runType)
            if 16 == runType {
                // java calcNonBMPGraphWidth —— DEFERRED（非 hello）
                let _ = (fontSpace, graphIndex, graphLength);
                /* DEFERRED_NON_FUNCTION （Non-BMP）
                // java widthSum = this.calcNonBMPGraphWidth(graphNode, fontSpace, graphIndex, graphLength, layoutState);
                */
            } else {
                // java ITypeface typeface = fontStyle.typeface;
                let typeface: Option<Arc<dyn ITypeface>> = typeface_code;
                let Some(typeface) = typeface else { return };
                // java float scaledFontSize = (float)fontStyle.fontSize * fontStyle.charScale;
                let scaledFontSize: f32 = fontSize_code as f32 * charScale_code;
                // java float emsize = fontStyle.srcEmSize;
                let emsize: f32 = srcEmSize_code;
                // java float scale = scaledFontSize / emsize;
                let scale: f32 = scaledFontSize / emsize;
                // java float widthError = layoutState.roundError;
                let mut widthError: f32 = layoutState.roundError;
                // java float boldAddWidth = 0.0F;
                let mut boldAddWidth: f32 = 0.0;
                // java if (typeface.needBold())
                if typeface.needBold() {
                    // java if (documentEnv.useWord97LineBreakRules) { boldAddWidth = scaledFontSize / 1000.0F; }
                    if unsafe { (*documentEnv).useWord97LineBreakRules } {
                        boldAddWidth = scaledFontSize / 1000.0;
                    } else {
                        // java else { boldAddWidth = scaledFontSize / emsize; }
                        boldAddWidth = scaledFontSize / emsize;
                    }
                }
                // java float extendSpace = (float)fontSpace + boldAddWidth;
                let extendSpace: f32 = fontSpace as f32 + boldAddWidth;
                // Read-only metric probe, disabled in normal layout. It keeps
                // the Java raw-EM -> rounded-twip boundary observable.
                let trace_codepoint = std::env::var("WORD_PARITY_FONT_TRACE_CODEPOINT")
                    .ok().and_then(|value| value.parse::<u32>().ok());
                // java int curGraphIndex = graphIndex;
                let mut curGraphIndex: i32 = graphIndex;
                // java int fillThisTime;
                // loop over blocks until maxGraphIndex
                let maxGraphIndex: i32 = graphIndex + graphLength;
                loop {
                    // java for(int maxGraphIndex = graphIndex + graphLength; curGraphIndex < maxGraphIndex; curGraphIndex += fillThisTime)
                    if curGraphIndex >= maxGraphIndex {
                        break;
                    }
                    // java int[] charBuff = typoPara.getCharBlock(curGraphIndex, blockResult);
                    let charBuff: &[char] = layoutState.typoPara.getCharBlock(curGraphIndex, &mut graphOperationEnv.blockResult);
                    // java int charOffset = blockResult[0];
                    let charOffset: i32 = graphOperationEnv.blockResult[0];
                    // java int charLimit = blockResult[1];
                    let charLimit: i32 = graphOperationEnv.blockResult[1];
                    // java int[] widthBuff = typoPara.getWidthBlock(curGraphIndex, blockResult);
                    let widthBuff: &[i32] = layoutState.typoPara.getWidthBlock(curGraphIndex, &mut graphOperationEnv.blockResult);
                    // java int widthOffset = blockResult[0];
                    let widthOffset: i32 = graphOperationEnv.blockResult[0];
                    // java int widthLimit = blockResult[1];
                    let widthLimit: i32 = graphOperationEnv.blockResult[1];
                    // java fillThisTime = Math.min(maxGraphIndex - curGraphIndex, Math.min(charLimit - charOffset, widthLimit - widthOffset));
                    let fillThisTime: i32 = (maxGraphIndex - curGraphIndex).min((charLimit - charOffset).min(widthLimit - widthOffset));
                    // java fontStyle.typeface.getWriterLayoutAdavancesEM(charBuff, charOffset, widthBuff, widthOffset, fillThisTime);
                    let mut codepoints: Vec<i32> = Vec::with_capacity(fillThisTime as usize);
                    for i in 0..fillThisTime {
                        codepoints.push(charBuff[(charOffset + i) as usize] as i32);
                    }
                    // rust: widths 平面 &[i32] 只读 —— 先算出目标宽再回写（fillWidths 语义按块）
                    let mut advances: Vec<i32> = vec![0; fillThisTime as usize];
                    // Java int[] charBuff selects the int[] overload; any
                    // narrowing belongs inside AndroidTypeface's overload.
                    typeface.getWriterLayoutAdavancesEM_ints(&codepoints, 0, &mut advances, 0, fillThisTime);
                    // java if (0.0F != extendSpace)
                    if 0.0 != extendSpace {
                        // java int i = widthOffset; for(int max = widthOffset + fillThisTime; i < max; ++i) { float fvalue = Math.max(0.0F, (float)widthBuff[i] * scale + extendSpace) + widthError; widthBuff[i] = (int)(fvalue + 0.5F); widthError = fvalue - (float)widthBuff[i]; }
                        for i in 0..fillThisTime {
                            let raw_em = advances[i as usize];
                            let fvalue: f32 = (advances[i as usize] as f32 * scale + extendSpace).max(0.0) + widthError;
                            advances[i as usize] = (fvalue + 0.5) as i32;
                            widthError = fvalue - advances[i as usize] as f32;
                            if trace_codepoint == Some(charBuff[(charOffset + i) as usize] as u32) {
                                eprintln!("font_advance graph={} code_point={} language={} run_type={} raw_em={} scale={} extend_space={} final_twips={}",
                                    curGraphIndex + i, trace_codepoint.unwrap(), self.language, runType,
                                    raw_em, scale, extendSpace, advances[i as usize]);
                            }
                        }
                    } else {
                        // java else { ... widthBuff[i] = (int)(fvalue + 0.5F); widthError = fvalue - widthBuff[i]; }
                        for i in 0..fillThisTime {
                            let raw_em = advances[i as usize];
                            let fvalue: f32 = advances[i as usize] as f32 * scale + widthError;
                            advances[i as usize] = (fvalue + 0.5) as i32;
                            widthError = fvalue - advances[i as usize] as f32;
                            if trace_codepoint == Some(charBuff[(charOffset + i) as usize] as u32) {
                                eprintln!("font_advance graph={} code_point={} language={} run_type={} raw_em={} scale={} extend_space={} final_twips={}",
                                    curGraphIndex + i, trace_codepoint.unwrap(), self.language, runType,
                                    raw_em, scale, extendSpace, advances[i as usize]);
                            }
                        }
                    }
                    // java widthSum += Tools.getSum(widthBuff, widthOffset, fillThisTime);
                    for i in 0..fillThisTime {
                        widthSum += advances[i as usize];
                    }
                    // rust: 回写宽度平面（compile-surface：以逐块 fillWidths 等价；Java 直接写 snapshot 数组）
                    layoutState.typoPara.putWidths(curGraphIndex, fillThisTime, &advances);
                    // java curGraphIndex += fillThisTime;
                    curGraphIndex += fillThisTime;
                }
                // java layoutState.roundError = widthError;
                layoutState.roundError = widthError;
            }
        }
        // java if (0 == widthSum && TypoRun.isLineBreak(runType))
        if 0 == widthSum && TypoRun::isLineBreak(runType) {
            // java int scaledFontSize = (int)((float)fontStyle.fontSize * fontStyle.charScale);
            let scaledFontSize: i32 = (fontSize_code as f32 * charScale_code) as i32;
            // java widthSum = scaledFontSize / 2;
            widthSum = scaledFontSize / 2;
            // java layoutState.extendWidths(graphNode, widthSum, graphIndex, graphLength);
            layoutState.extendWidths(graphNode, widthSum, graphIndex, graphLength);
        }
        // java graphNode.mWidth = widthSum;
        graphNode.mWidth = widthSum;
    }

    // java protected void calcHeight(Graphs graphNode, ILangLayout.GraphOperationEnv graphOperationEnv)
    fn calcHeight(&mut self, graphNode: &mut Graphs, graphOperationEnv: &mut GraphOperationEnv) {
        // java if (null != graphNode)
        // java TypoFontStyle fontStyle = graphNode.fontStyle;
        let fontStyle: Option<TypoFontStyle> = graphNode.fontStyle.clone();
        let Some(mut fontStyle) = fontStyle else { return };
        // java int runType = graphNode.runType;
        let runType: i32 = graphNode.runType;
        // java boolean isCalcGraphHeight = graphOperationEnv.isCalcGraphHeight;
        let isCalcGraphHeight: bool = graphOperationEnv.isCalcGraphHeight;
        // java boolean isMathContent = graphOperationEnv.isMathContent;
        // java boolean ignoreLeading = graphOperationEnv.isIgnoreLeading;
        // java LayoutState layoutState = graphOperationEnv.layoutState;
        // java if (isCalcGraphHeight && !TypoRun.isWeakHeightGraphs(runType))
        if isCalcGraphHeight && !TypoRun::isWeakHeightGraphs(runType) {
            // rust: ★05(行高) —— 高计算在 flowTextLine 阶段需要；当前保持 Java 原文（DEFERRED 停车）
            let _ = (runType, fontStyle);
            /* DEFERRED_NON_FUNCTION（05 行高阶段解）
            // java fontStyle = graphOperationEnv.typoFontStylePool.obtain(fontStyle);
            // java ... typeface.getWriterLayoutAscent() ... getTextRect ...
            */
            return;
        }
        // java graphNode.fontStyle = fontStyle;
        graphNode.fontStyle = Some(fontStyle);
    }

    // java private int addGraph(int startGraphIndex, boolean isLanguageSpecial, ILangLayout.GraphOperationEnv graphOperationEnv)
    pub(crate) fn addGraph(&mut self, startGraphIndex: i32, _isLanguageSpecial: bool, graphOperationEnv: &mut GraphOperationEnv) -> i32 {
        // java int addLen = 1;
        let addLen: i32 = 1;
        // java Graphs graphNode = this.addGraphNode(startGraphIndex, 1, graphOperationEnv);
        let graphNode: *mut Graphs = self.addGraphNode(startGraphIndex, 1, graphOperationEnv);
        // java if (isLanguageSpecial) { graphOperationEnv.langGraphNode = graphNode; } else { graphOperationEnv.commonGraphNode = graphNode; }
        if _isLanguageSpecial {
            graphOperationEnv.langGraphNode = graphNode;
        } else {
            graphOperationEnv.commonGraphNode = graphNode;
        }
        // java CharsNode charsNode = graphOperationEnv.charsNode;
        let charsNode: *mut crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode = graphOperationEnv.charsNode;
        // java LayoutState layoutState = graphOperationEnv.layoutState;
        let layoutState = unsafe { &mut *graphOperationEnv.layoutState };
        // java int runType = graphOperationEnv.runType;
        let runType: i32 = graphOperationEnv.runType;
        // java TypoFontStyle cachedFontStyle = charsNode.fontStyles[this.language];
        let cachedFontStyle: Option<TypoFontStyle> = unsafe { &*charsNode }.fontStyles[self.language as usize].clone();
        // java TypoFontStyle fontStyle = cachedFontStyle;
        let mut fontStyle: TypoFontStyle = if let Some(c) = &cachedFontStyle { c.clone() } else { TypoFontStyle::newInstance() };
        // java if (null == cachedFontStyle)
        if cachedFontStyle.is_none() {
            // java PropertySet runProp = charsNode.finalRunProp;
            let runProp: util::moffice::property::PropertySet::PropertySet = unsafe { &*charsNode }.finalRunProp.clone().unwrap_or_default();
            // java FontStyleCache cache = this.getFontStyleCache(graphOperationEnv.documentEnv, layoutState);
            // rust: 用 GraphFactory 同理的 FontStyleCacheFactory.create —— 此处经 documentEnv
            // // java if (graphOperationEnv.isProccessMathOp) { fontStyle = cache.getMathFontStyle(...); }
            // java else { fontStyle = cache.get(runProp, this.language, layoutState.styleResult, graphOperationEnv.typoFontStylePool); charsNode.fontStyles[this.language] = fontStyle; }
            if graphOperationEnv.isProccessMathOp {
                // rust: 数学 → getMathFontStyle（createMathFontStyle 停 DEFERRED —— 数学非 hello）
                fontStyle = FontStyleFactory::createMathFontStyle(
                    &runProp,
                    self.language,
                    false,
                    Some(&mut layoutState.styleResult),
                    Some(unsafe { &mut *graphOperationEnv.typoFontStylePool }),
                );
            } else {
                let cache = self.getFontStyleCache(graphOperationEnv.documentEnv, layoutState);
                fontStyle = cache.get(
                    &runProp,
                    self.language,
                    Some(&mut layoutState.styleResult),
                    Some(unsafe { &mut *graphOperationEnv.typoFontStylePool }),
                );
                unsafe { &mut *charsNode }.fontStyles[self.language as usize] = Some(fontStyle.clone());
            }
        }
        // java if (graphOperationEnv.isScralForSmallCaps)
        if graphOperationEnv.isScralForSmallCaps {
            // rust: 小型大写字母 —— 非 hello（GR-D3）；Java 原文见 LangLayout.java:337-342
            /* DEFERRED_NON_FUNCTION
            // java fontStyle = graphOperationEnv.typoFontStylePool.obtain(fontStyle);
            // java fontStyle.fontSize = (int)((float)fontStyle.fontSize * 0.8F);
            // java fontStyle.maxAscent = (int)((float)fontStyle.maxAscent * 0.8F);
            // java fontStyle.maxDescent = (int)((float)fontStyle.maxDescent * 0.8F);
            // java fontStyle.setScralForSmallCaps(true);
            */
        }
        // java if (!isLanguageSpecial)
        if !_isLanguageSpecial {
            // java if (TypoRun.isTypoPageBreak(runType))
            if TypoRun::isTypoPageBreak(runType) {
                // rust: ★页断（runType 5）—— 9pt serif 字体：FontHost 表 + FontMetrics
                // java if (fontStyle == cachedFontStyle) { fontStyle = graphOperationEnv.typoFontStylePool.obtain(fontStyle); }
                // rust: Rust 值语义无 == 引用比较 → 恒 obtain 拷贝（语义等价）
                if let Some(c) = &cachedFontStyle {
                    fontStyle = c.clone();
                }
                // java fontStyle.fontSize = UnitUtil.PT2TWIPS(9.0F);
                fontStyle.fontSize = UnitUtil::PT2TWIPS(9.0);
                // rust: Java `fontTable.get("serif", true)`；宿主注入同名字体面。
                // java IFontTable fontTable = FontHost.getFontTable();
                // java IFontFamily fontFamily = fontTable.get("serif", true);
                // java fontStyle.typeface = fontFamily.getTypeface(0);
                if let Some(stack) = crate::cn::wps::moffice::writer::layout::text::FontHost::get_font_table() {
                    fontStyle.typeface = Some(crate::cn::wps::moffice::writer::layout::base::tools::TypefaceTool::WriterTypeface::new(
                        "serif".to_string(), false, false, false, false,
                    ));
                    // java fontStyle.emSize = fontStyle.typeface.getWriterLayoutEMSize();
                    if let Some(tf) = &fontStyle.typeface {
                        fontStyle.emSize = tf.getWriterLayoutEMSize();
                        // java fontStyle.srcEmSize = fontStyle.typeface.getSrcEmSize();
                        fontStyle.srcEmSize = tf.getSrcEmSize();
                        // java FontMetrics fontMetrics = ...; fontStyle.typeface.getFontMetrics(fontSize, fontMetrics);
                        if graphOperationEnv.fontMetrics.is_none() {
                            graphOperationEnv.fontMetrics = Some(thirdparty_font::cn::wps::font::ITypeface::FontMetrics::new());
                        }
                        let mut fm = thirdparty_font::cn::wps::font::ITypeface::FontMetrics::new();
                        tf.getFontMetrics(fontStyle.fontSize as f32, &mut fm);
                        // java fontStyle.maxAscent = (int)fontMetrics.getBaseline();
                        fontStyle.maxAscent = fm.mBaseline as i32;
                        // java fontStyle.maxDescent = (int)fontMetrics.getMaxDescent();
                        fontStyle.maxDescent = fm.mMaxDescent as i32;
                    }
                }
            // java else if (15 == runType)
            } else if 15 == runType {
                // rust: emoji —— DEFERRED（非 hello）
                /* DEFERRED_NON_FUNCTION
                // java this.calcEmojiAscentAndDescent(graphNode, fontStyle.fontSize, fontStyle, graphOperationEnv);
                */
            }
        }
        // java if (graphOperationEnv.isInMath)
        if graphOperationEnv.isInMath {
            // rust: 数学下划线 —— DEFERRED（非 hello）
            /* DEFERRED_NON_FUNCTION
            // java fontStyle = this.processMathUnderline(fontStyle, graphOperationEnv);
            */
        }
        // java if (layoutState.styleResult.hasBRC && TypoRun.isLineBreak(graphNode.runType) && fontStyle.hasBdr())
        if layoutState.styleResult.hasBRC && TypoRun::isLineBreak(runType) && fontStyle.hasBdr() {
            // rust: BRC 行断 —— 非 hello（无边）；Java 原文可得
            /* DEFERRED_NON_FUNCTION
            // java fontStyle = graphOperationEnv.typoFontStylePool.obtain(fontStyle);
            // java fontStyle.deleteBdr();
            */
        }
        // java graphNode.fontStyle = fontStyle;
        unsafe { &mut *graphNode }.fontStyle = Some(fontStyle);
        // java graphOperationEnv.isScralForTmpGrpahNode = graphOperationEnv.isScralForSmallCaps;
        graphOperationEnv.isScralForTmpGrpahNode = graphOperationEnv.isScralForSmallCaps;
        // java if (graphOperationEnv.isIsolate)
        if graphOperationEnv.isIsolate {
            // java if (isLanguageSpecial) { this.commitLangGraphNode(graphOperationEnv); } else { this.commitCommonGraphNode(graphOperationEnv); }
            if _isLanguageSpecial {
                self.commitLangGraphNode(graphOperationEnv);
            } else {
                self.commitCommonGraphNode(graphOperationEnv);
            }
        }
        // java return 1;
        addLen
    }

    // java public void addGraphAfterParse(...)
    // rust: ★wr-29 04 —— java2rust 完整方法当随 calcWidths 一起解（05 组合）；当前仅 hello 需要
    //       的 addGraph/commit 已真实，此方法 DEFERRED。
    fn _addGraphAfterParse_parked(&mut self) {
        /* DEFERRED_NON_FUNCTION （LangLayout.java:333 addGraphAfterParse —— 泰文/爪哇语合并）
        */
    }

    // java public boolean hasMergeChar()
    pub fn hasMergeChar(&self) -> bool {
        // java return false;
        false
    }

    // java protected void addGraphAfterParse(int startGraphIndex, int endGraphIndex, int bufOffset, int bufLimit, int[] charBuf, ILangLayout.GraphOperationEnv graphOperationEnv)
    // rust: ★wr-29 04 —— 真实（WestLayout 字母合并的落图口）
    pub(crate) fn addGraphAfterParse(&mut self, startGraphIndex: i32, endGraphIndex: i32, _bufOffset: i32, _bufLimit: i32, _charBuf: &[i32], graphOperationEnv: &mut GraphOperationEnv) {
      // java if (this.shouldCommitGraphsAfterParse(graphOperationEnv)) { this.commitLangGraphNode(graphOperationEnv); }
      if self.shouldCommitGraphsAfterParse(graphOperationEnv) {
        self.commitLangGraphNode(graphOperationEnv);
      }
      // java int graphLen = endGraphIndex - startGraphIndex;
      let graphLen: i32 = endGraphIndex - startGraphIndex;
      // java if (null != graphOperationEnv.langGraphNode)
      if !graphOperationEnv.langGraphNode.is_null() {
        // java var10000.graphLength += graphLen;
        unsafe { (*graphOperationEnv.langGraphNode).graphLength += graphLen; }
      } else {
        // java this.addGraph(startGraphIndex, true, graphOperationEnv);
        self.addGraph(startGraphIndex, true, graphOperationEnv);
        // java --graphLen;
        let mut graphLen = graphLen - 1;
        // java if (null != graphOperationEnv.langGraphNode) { var9.graphLength += graphLen; }
        if !graphOperationEnv.langGraphNode.is_null() {
          unsafe { (*graphOperationEnv.langGraphNode).graphLength += graphLen; }
        }
      }
    }

    // java private boolean shouldCommitGraphsAfterParse(ILangLayout.GraphOperationEnv graphOperationEnv)
    pub fn shouldCommitGraphsAfterParse(&self, graphOperationEnv: &GraphOperationEnv) -> bool {
      // java Graphs graphs = graphOperationEnv.langGraphNode;
      let graphs: *mut Graphs = graphOperationEnv.langGraphNode;
      // java return null != graphs && (isIsolate || isNoWidth || scral-flag 变 || language != || autoSpaceType != || TypoRun.needTransToSpace(runType)&&runType 变);
      if graphs.is_null() {
        return false;
      }
      let g = unsafe { &*graphs };
      return graphOperationEnv.isIsolate
        || graphOperationEnv.isNoWidth
        || graphOperationEnv.isScralForSmallCaps != graphOperationEnv.isScralForTmpGrpahNode
        || g.language != self.language
        || g.autoSpaceType != graphOperationEnv.autoSpaceType
        || (crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun::needTransToSpace(g.runType) && g.runType != graphOperationEnv.runType);
    }

    // java protected boolean needSplitGraphs(int graphIndex, ILangLayout.GraphOperationEnv graphOperationEnv)
    pub fn needSplitGraphs(&self, _graphIndex: i32, _graphOperationEnv: &GraphOperationEnv) -> bool {
      // java return false;
      false
    }

    // java protected void addSameStyleGraph(...)
    // rust: ★wr-29 04 —— FarEast 合图共用；hello 的合并走 addGraphAfterParse——保持 Java 原文（DEFERRED 停车）
    pub fn addSameStyleGraph(&mut self, _startGraphIndex: i32, _endGraphIndex: i32, _bufOffset: i32, _bufLimit: i32, _charBuf: &[i32], _needSplit: bool, _graphOperationEnv: &mut GraphOperationEnv) {
      /* DEFERRED_NON_FUNCTION
      // java Graphs graphNode = graphOperationEnv.langGraphNode;
      // java ...（Java 原文见 LangLayout.java:282-304：hasSameFontStyle 合并/分图）
      */
    }
}

impl ILangLayout for LangLayout {
    // java public Graphs addGraphNode(int graphIndex, int length, ILangLayout.GraphOperationEnv graphOperationEnv)
    fn addGraphNode(&mut self, graphIndex: i32, length: i32, graphOperationEnv: &mut GraphOperationEnv) -> *mut Graphs {
        // java SrcNode srcNode = graphOperationEnv.charsNode.srcRef;
        let srcNode: Option<*mut crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode> =
            unsafe { &*graphOperationEnv.charsNode }.srcRef;
        // java LayoutState layoutState = graphOperationEnv.layoutState;
        let layoutState = unsafe { &mut *graphOperationEnv.layoutState };
        // java if (null != srcNode && 0 == srcNode.type)
        if let Some(srcNode) = srcNode {
            if 0 == unsafe { (*srcNode).r#type } {
                // java layoutState.hasCoreCharsContent = true;
                layoutState.hasCoreCharsContent = true;
            }
        }
        // java Graphs graphNode = (Graphs)layoutState.getGraphList().obtainAndAddLast();
        let graphNode: *mut Graphs = layoutState.getGraphList().obtainAndAddLast();
        let graphNodeRef = unsafe { &mut *graphNode };
        // java graphNode.autoSpaceType = graphOperationEnv.autoSpaceType;
        graphNodeRef.autoSpaceType = graphOperationEnv.autoSpaceType;
        // java graphNode.runType = graphOperationEnv.runType;
        graphNodeRef.runType = graphOperationEnv.runType;
        // java graphNode.graphIndex = graphIndex;
        graphNodeRef.graphIndex = graphIndex;
        // java graphNode.graphLength = length;
        graphNodeRef.graphLength = length;
        // java graphNode.mLangLayout = this;
        // rust: Graphs.mLangLayout 以语言 id 承载（Java 对象引用 → 语言标识；05 经 Factory 重建）
        graphNodeRef.mLangLayout = Some(self.language);
        // java graphNode.charsRef = graphOperationEnv.charsNode;
        graphNodeRef.charsRef = Some(crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode::clone(unsafe { &*graphOperationEnv.charsNode }));
        // java graphNode.language = this.language;
        graphNodeRef.language = self.language;
        // java return graphNode;
        graphNode
    }

    // java public int addLangGraph(...)
    fn addLangGraph(&mut self, var1: i32, _var2: i32, _var3: i32, _var4: i32, _var5: &[i32], var6: &mut GraphOperationEnv) -> i32 {
        // java return this.addGraph(startGraphIndex, true, graphOperationEnv);
        self.addGraph(var1, true, var6)
    }

    // java public void commitLangGraphNode(...)
    fn commitLangGraphNode(&mut self, var1: &mut GraphOperationEnv) {
        // java this.calcGraphNode(graphOperationEnv.langGraphNode, graphOperationEnv);
        let node = var1.langGraphNode;
        self.calcGraphNode(node, var1);
        // java graphOperationEnv.langGraphNode = null;
        var1.langGraphNode = std::ptr::null_mut();
    }

    // java public int addCommonGraph(...)
    fn addCommonGraph(&mut self, var1: i32, var2: &mut GraphOperationEnv) -> i32 {
        // java return this.addGraph(graphIndex, false, graphOperationEnv);
        self.addGraph(var1, false, var2)
    }

    // java public void commitCommonGraphNode(...)
    fn commitCommonGraphNode(&mut self, var1: &mut GraphOperationEnv) {
        // java this.calcGraphNode(graphOperationEnv.commonGraphNode, graphOperationEnv);
        let node = var1.commonGraphNode;
        self.calcGraphNode(node, var1);
        // java graphOperationEnv.commonGraphNode = null;
        var1.commonGraphNode = std::ptr::null_mut();
    }

    // java public TypoFontStyle processMathUnderline(...)
    fn processMathUnderline(&mut self, var1: TypoFontStyle, _var2: &mut GraphOperationEnv) -> TypoFontStyle {
        // rust: ★DEFERRED（数学下划线 非 hello）；Java 原文见 LangLayout.java:103
        var1
    }

    // java public void rollBackForPartialWord(...)
    fn rollBackForPartialWord(&mut self, _var1: i32, _var2: i32, _var3: i32, _var4: i32, _var5: i32, _var6: i32, var7: &mut RollBackResult) {
        // rust: DEFERRED（05 压缩）；Java 原文见 LangLayout.java:73
        var7.adjusted = false;
        var7.rollbackedAll = false;
    }

    // java public boolean hasMergeChar()
    fn hasMergeChar(&self) -> bool {
        self.hasMergeChar()
    }
}
