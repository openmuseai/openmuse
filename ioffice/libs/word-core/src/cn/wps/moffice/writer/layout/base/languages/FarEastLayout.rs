// Java: cn.wps.moffice.writer.layout.base.languages.FarEastLayout
#![allow(dead_code, non_snake_case, unused_variables)]

use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::{GraphOperationEnv, ILangLayout, RollBackResult};
use crate::cn::wps::moffice::writer::layout::base::languages::LangLayout::LangLayout;
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::PunctTool::PunctTool;

// java public class FarEastLayout extends LangLayout
pub struct FarEastLayout {
    pub(crate) super_: LangLayout,
}

impl FarEastLayout {
    // java public FarEastLayout(int language) { super(language); }
    pub fn FarEastLayout(language: i32) -> Self {
        Self { super_: LangLayout { language } }
    }

    // java public int addLangGraph(int startGraphIndex, int endGraphIndex, int bufOffset, int bufLimit, int[] charBuff, GraphOperationEnv graphOperationEnv)
    pub fn addLangGraph(&mut self, startGraphIndex: i32, endGraphIndex: i32, bufOffset: i32, bufLimit: i32, charBuff: &[i32], graphOperationEnv: &mut GraphOperationEnv) -> i32 {
        // java int autoSpaceType = graphOperationEnv.autoSpaceType;
        let autoSpaceType = graphOperationEnv.autoSpaceType;
        let mut nextGraphIndex = startGraphIndex + 1;
        let mut nextBlockOffset = bufOffset + 1;
        let mut nextBuffLimit = bufLimit;
        let mut nextCharsBuff = charBuff.to_vec();
        let mut graphLen = 1;
        // java if (2 == autoSpaceType) while(nextGraphIndex < endGraphIndex) ...
        if autoSpaceType == 2 {
            while nextGraphIndex < endGraphIndex {
                if nextBlockOffset >= nextBuffLimit {
                    let mut blockResult = [0; 2];
                    let chars = unsafe { &*graphOperationEnv.layoutState }
                        .typoPara.getCharBlock(nextGraphIndex, &mut blockResult);
                    nextCharsBuff = chars.iter().map(|ch| *ch as i32).collect();
                    nextBlockOffset = blockResult[0];
                    nextBuffLimit = blockResult[1];
                }
                // java if (!this.isRealAsian((char)nextCharsBuff[nextBlockOffset])) break;
                if !self.isRealAsian(char::from_u32(nextCharsBuff[nextBlockOffset as usize] as u32).unwrap_or('\0')) {
                    break;
                }
                graphLen += 1;
                nextGraphIndex += 1;
                nextBlockOffset += 1;
            }
        }
        // java this.addGraphAfterParse(startGraphIndex, startGraphIndex + graphLen, ...);
        self.super_.addGraphAfterParse(startGraphIndex, startGraphIndex + graphLen, bufOffset, bufLimit, charBuff, graphOperationEnv);
        graphLen
    }

    // java protected void calcWidths(Graphs graphNode, GraphOperationEnv graphOperationEnv)
    pub(crate) fn calcWidths(graphNode: &mut Graphs, graphOperationEnv: &mut GraphOperationEnv) {
        calc_default_east_widths(graphNode, graphOperationEnv, false);
    }

    // java private boolean isRealAsian(char c)
    fn isRealAsian(&self, c: char) -> bool {
        let c = c as u32;
        // java [19968,'鿏'] || [12352,12687] || [13056,19903] || ['가','\ud7af']
        (19968..=0x9FCF).contains(&c) || (12352..=12687).contains(&c)
            || (13056..=19903).contains(&c) || (0xAC00..=0xD7AF).contains(&c)
    }
}

// Java FarEastLayout.calcWidths and SharedFarEastLayout.calcWidths share this
// control flow; `shared` selects their original fontSpace and break rounding.
pub(crate) fn calc_default_east_widths(graphNode: &mut Graphs, graphOperationEnv: &mut GraphOperationEnv, shared: bool) {
    // java TypoFontStyle fontStyle = graphNode.fontStyle;
    let Some(fontStyle) = graphNode.fontStyle.as_ref().cloned() else { return; };
    // java LayoutState layoutState = graphOperationEnv.layoutState;
    // java DocumentEnv documentEnv = graphOperationEnv.documentEnv;
    let layoutState = unsafe { &mut *graphOperationEnv.layoutState };
    let documentEnv = unsafe { &*graphOperationEnv.documentEnv };
    let fontSize = fontStyle.fontSize;
    let graphIndex = graphNode.graphIndex;
    let graphLength = graphNode.graphLength;
    let runType = graphNode.runType;
    // java int fontSpace = balance ? fontStyle.fontSpace * 2 : fontStyle.fontSpace;
    let fontSpace = if shared || documentEnv.balanceSingleByteDoubleByteWidth {
        fontStyle.fontSpace * 2
    } else {
        fontStyle.fontSpace
    };
    // java float scaledFontSize = (float)fontSize * fontStyle.charScale;
    let mut scaledFontSize = fontSize as f32 * fontStyle.charScale;
    let mut widthError = layoutState.roundError;
    let mut boldAddWidth = 0.0f32;
    if let Some(typeface) = &fontStyle.typeface {
        // java if (typeface.needBold()) ... else if (charScale != 0 && != 1) ...
        if typeface.needBold() {
            boldAddWidth = if documentEnv.useWord97LineBreakRules {
                scaledFontSize / 1000.0
            } else {
                scaledFontSize / fontStyle.srcEmSize
            };
        } else if !shared && fontStyle.charScale != 0.0 && fontStyle.charScale != 1.0 {
            scaledFontSize *= (fontStyle.srcEmSize - 1.0) / fontStyle.srcEmSize;
        }
    }
    let extendSpace = fontSpace as f32 + boldAddWidth;
    let mut widthSum = 0;
    // java for(curGraphIndex = graphIndex; curGraphIndex < maxGraphIndex; curGraphIndex += fillThisTime)
    let mut curGraphIndex = graphIndex;
    while curGraphIndex < graphIndex + graphLength {
        let mut blockResult = [0; 2];
        let _ = layoutState.typoPara.getWidthBlock(curGraphIndex, &mut blockResult);
        let fillThisTime = (graphIndex + graphLength - curGraphIndex).min(blockResult[1] - blockResult[0]);
        if fillThisTime <= 0 { break; }
        let mut widths = Vec::with_capacity(fillThisTime as usize);
        for _ in 0..fillThisTime {
            // java float fwidth = (extendSpace != 0 ? max(0, defaultWidth + extendSpace) : defaultWidth) + widthError;
            let fwidth = if extendSpace != 0.0 {
                (scaledFontSize + extendSpace).max(0.0) + widthError
            } else {
                scaledFontSize + widthError
            };
            let width = (fwidth + 0.5) as i32;
            widthError = fwidth - width as f32;
            widths.push(width);
            widthSum += width;
        }
        // Java writes directly to TypoPara's width block.
        layoutState.typoPara.putWidths(curGraphIndex, fillThisTime, &widths);
        curGraphIndex += fillThisTime;
    }
    layoutState.roundError = widthError;
    // java if (TypoRun.isPunct(runType)) ... addDrawPosOffsetX(...)
    if TypoRun::isPunct(runType) {
        let ch = layoutState.typoPara.getChar(graphIndex);
        if let Some(typeface) = &fontStyle.typeface {
            if PunctTool::isSerifPunct(ch, typeface.as_ref()) {
                let offset = PunctTool::getFixSerifPunctOffset(ch, typeface.as_ref(), fontSize, widthSum, fontStyle.charScale);
                layoutState.addDrawPosOffsetX__int_int(graphIndex, offset);
            }
        }
    }
    // java if (0 == widthSum && TypoRun.isLineBreak(runType)) ...
    if widthSum == 0 && TypoRun::isLineBreak(runType) {
        if shared {
            let fwidth = scaledFontSize / 2.0 + layoutState.roundError / graphLength.max(1) as f32;
            layoutState.extendWidths(graphNode, (fwidth + 0.5) as i32, graphIndex, graphLength);
            let total = fwidth * graphLength as f32;
            widthSum = (total + 0.5) as i32;
            layoutState.roundError = total - widthSum as f32;
        } else {
            let width = (scaledFontSize / 2.0 + layoutState.roundError + 0.5) as i32;
            let total = scaledFontSize / 2.0 * graphLength as f32 + layoutState.roundError;
            widthSum = (total + 0.5) as i32;
            layoutState.roundError = total - widthSum as f32;
            layoutState.extendWidths(graphNode, width, graphIndex, graphLength);
        }
    }
    // java graphNode.mWidth = widthSum;
    graphNode.mWidth = widthSum;
}

impl ILangLayout for FarEastLayout {
    fn addGraphNode(&mut self, a: i32, b: i32, e: &mut GraphOperationEnv) -> *mut Graphs { self.super_.addGraphNode(a, b, e) }
    fn addLangGraph(&mut self, a: i32, b: i32, c: i32, d: i32, chars: &[i32], e: &mut GraphOperationEnv) -> i32 { self.addLangGraph(a, b, c, d, chars, e) }
    fn commitLangGraphNode(&mut self, e: &mut GraphOperationEnv) { self.super_.commitLangGraphNode(e) }
    fn addCommonGraph(&mut self, a: i32, e: &mut GraphOperationEnv) -> i32 { self.super_.addCommonGraph(a, e) }
    fn commitCommonGraphNode(&mut self, e: &mut GraphOperationEnv) { self.super_.commitCommonGraphNode(e) }
    fn processMathUnderline(&mut self, a: crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle, e: &mut GraphOperationEnv) -> crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle { self.super_.processMathUnderline(a, e) }
    fn rollBackForPartialWord(&mut self, a: i32, b: i32, c: i32, d: i32, f: i32, g: i32, r: &mut RollBackResult) { self.super_.rollBackForPartialWord(a, b, c, d, f, g, r) }
    fn hasMergeChar(&self) -> bool { self.super_.hasMergeChar() }
}
