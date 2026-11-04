// Rust stub for TextboxLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/drawing/textbox/TextboxLayouter.java
use crate::*;

pub struct TextboxLayouter {
    // TODO: fields
}

impl TextboxLayouter {
    pub fn new() -> Self { todo!() }
    pub fn createLayoutState(&self) -> LayoutState { todo!() }
    pub fn initLayouters(&self) { todo!() }
    pub fn initTableLayouter(&self) { todo!() }
    pub fn isInTextFrame(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn initDrawingLayouter(&self) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn flow(&self, layoutState: LayoutState) { todo!() }
    pub fn doFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn getLineCoreWidth(&self, textLine: TypoTextLine, layoutState: LayoutState) -> i32 { todo!() }
    pub fn markExcessedParaBrcs(&self, pParaBrc: i32, layoutState: LayoutState) { todo!() }
    pub fn markExcessedCellMargins(&self, row: TypoTableRow, layoutState: LayoutState) { todo!() }
    pub fn hasContentInCell(&self, cell: TypoTableCell) -> bool { todo!() }
    pub fn postFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn collectPageRange(&self, layoutState: LayoutState) { todo!() }
}
