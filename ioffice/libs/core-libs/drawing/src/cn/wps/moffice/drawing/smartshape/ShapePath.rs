#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.graphics.FillBase
use crate::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use crate::cn::wps::moffice::drawing::line::LineProperty::LineProperty;

use super::Segment::Segment;

// Java: public class ShapePath
pub struct ShapePath {
    // Java field: LineProperty lineProperty
    pub lineProperty: Option<LineProperty>,
    // Java field: FillBase fill
    pub fill: Option<FillBase>,
    // Java field: Segment[] segments
    pub segments: Option<Vec<Segment>>,
}

// Java: public class ShapePath
impl ShapePath {
    // Java: ShapePath()
    pub fn new() -> Self {
        // Java: super();
        ShapePath {
            // Java: LineProperty lineProperty = null
            lineProperty: None,
            // Java: FillBase fill = null
            fill: None,
            // Java: Segment[] segments = null
            segments: None,
        }
    }

    // Java: void setLineProperty(LineProperty lineProperty)
    pub fn setLineProperty(&mut self, lineProperty: Option<LineProperty>) {
        // Java: this.lineProperty = lineProperty;
        self.lineProperty = lineProperty;
    }

    // Java: LineProperty getLineProperty()
    pub fn getLineProperty(&self) -> Option<&LineProperty> {
        // Java: return this.lineProperty;
        self.lineProperty.as_ref()
    }

    // Java: void setFill(FillBase fill)
    pub fn setFill(&mut self, fill: Option<FillBase>) {
        // Java: this.fill = fill;
        self.fill = fill;
    }

    // Java: FillBase getFill()
    pub fn getFill(&self) -> Option<&FillBase> {
        // Java: return this.fill;
        self.fill.as_ref()
    }

    // Java: void setSegment(Segment[] segments)
    pub fn setSegment(&mut self, segments: Option<Vec<Segment>>) {
        // Java: this.segments = segments;
        self.segments = segments;
    }

    // Java: Segment[] getSegment()
    pub fn getSegment(&self) -> Option<&Vec<Segment>> {
        // Java: return this.segments;
        self.segments.as_ref()
    }
}
