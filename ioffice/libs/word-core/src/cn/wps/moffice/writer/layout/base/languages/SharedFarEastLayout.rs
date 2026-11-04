// Java: cn.wps.moffice.writer.layout.base.languages.SharedFarEastLayout
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::languages::FarEastLayout::calc_default_east_widths;
use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::{GraphOperationEnv, ILangLayout, RollBackResult};
use crate::cn::wps::moffice::writer::layout::base::languages::LangLayout::LangLayout;

// java public class SharedFarEastLayout extends LangLayout
pub struct SharedFarEastLayout {
    pub(crate) super_: LangLayout,
}

impl SharedFarEastLayout {
    // java public SharedFarEastLayout(int language) { super(language); }
    pub fn SharedFarEastLayout(language: i32) -> Self { Self { super_: LangLayout { language } } }

    // java public int addLangGraph(...)
    pub fn addLangGraph(&mut self, startGraphIndex: i32, _endGraphIndex: i32, bufOffset: i32, bufLimit: i32, charBuff: &[i32], graphOperationEnv: &mut GraphOperationEnv) -> i32 {
        // java this.addGraphAfterParse(startGraphIndex, startGraphIndex + 1, ...); return 1;
        self.super_.addGraphAfterParse(startGraphIndex, startGraphIndex + 1, bufOffset, bufLimit, charBuff, graphOperationEnv);
        1
    }

    // java protected void calcWidths(Graphs graphNode, GraphOperationEnv graphOperationEnv)
    pub(crate) fn calcWidths(graphNode: &mut Graphs, graphOperationEnv: &mut GraphOperationEnv) {
        calc_default_east_widths(graphNode, graphOperationEnv, true);
    }
}

impl ILangLayout for SharedFarEastLayout {
    fn addGraphNode(&mut self, a: i32, b: i32, e: &mut GraphOperationEnv) -> *mut Graphs { self.super_.addGraphNode(a, b, e) }
    fn addLangGraph(&mut self, a: i32, b: i32, c: i32, d: i32, chars: &[i32], e: &mut GraphOperationEnv) -> i32 { self.addLangGraph(a, b, c, d, chars, e) }
    fn commitLangGraphNode(&mut self, e: &mut GraphOperationEnv) { self.super_.commitLangGraphNode(e) }
    fn addCommonGraph(&mut self, a: i32, e: &mut GraphOperationEnv) -> i32 { self.super_.addCommonGraph(a, e) }
    fn commitCommonGraphNode(&mut self, e: &mut GraphOperationEnv) { self.super_.commitCommonGraphNode(e) }
    fn processMathUnderline(&mut self, a: crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle, e: &mut GraphOperationEnv) -> crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle { self.super_.processMathUnderline(a, e) }
    fn rollBackForPartialWord(&mut self, a: i32, b: i32, c: i32, d: i32, f: i32, g: i32, r: &mut RollBackResult) { self.super_.rollBackForPartialWord(a, b, c, d, f, g, r) }
    fn hasMergeChar(&self) -> bool { self.super_.hasMergeChar() }
}
