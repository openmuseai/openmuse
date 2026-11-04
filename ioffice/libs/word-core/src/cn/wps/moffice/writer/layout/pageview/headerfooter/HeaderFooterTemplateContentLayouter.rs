// Rust stub for HeaderFooterTemplateContentLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/pageview/headerfooter/HeaderFooterTemplateContentLayouter.java
use crate::*;

pub struct HeaderFooterTemplateContentLayouter {
    // TODO: fields
}

impl HeaderFooterTemplateContentLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn flowContent(&self, layoutPage: TypoLayoutPage, args: i32) { todo!() }
    pub fn initState(&self, contentPage: i32, range: i64, anchor: i32, docType: i32, finalSecProp: PropertySet) -> HeaderFooterPageLayoutState { todo!() }
    pub fn getTextFlow(&self, pPage: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn doFlowTextLineContent(&self, contentPage: i32, docType: i32, range: i64, anchor: i32, templateArg: i32, finalSecProp: PropertySet) -> i32 { todo!() }
    pub fn fitShapeForTextLine(&self, textLine: i32, page: i32, snapshot: TypoSnapshot, layoutState: LayoutState) { todo!() }
    pub fn extendBottomForLastSHDAndBrc(&self, textLine: i32, anchor: i32, snapshot: TypoSnapshot) { todo!() }
    pub fn doFlowDrawing(&self, docType: i32, range: i64, anchor: i32, templateArg: i32, finalSecProp: PropertySet, inMainPageMemory: bool) -> i32 { todo!() }
    pub fn prepareEmbeds(&self, wordAlingFont: i32, layoutState: LayoutState) { todo!() }
    pub fn initDrawingLayouter(&self) { todo!() }
    pub fn initTextFrameLayouter(&self) { todo!() }
    pub fn updateTemplateContent(&self, rect: Rect) -> bool { todo!() }
    pub fn updatePageVersion(&self, pageIndexStart: i32, pageIndexMax: i32) { todo!() }
    pub fn getFinalSecProp(&self, pageIndex: i32) -> PropertySet { todo!() }
    pub fn updateLayoutPageContent(&self, pageIndexStart: i32, pageIndexMax: i32, finalSecProp: PropertySet) { todo!() }
    pub fn updateNumPagesAndSectionPages(&self, templete: i32, finalSecProp: PropertySet) { todo!() }
}
