#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

use super::Vector3D::Vector3D;
use super::Line::Line;

// Java: public class Segment
pub struct Segment {
    // Java field: Vector3D start
    pub(crate) start: Box<dyn std::any::Any>,
    // Java field: Vector3D end
    pub(crate) end: Box<dyn std::any::Any>,
    // Java field: Line line
    pub(crate) line: Box<dyn std::any::Any>,
}

// Java: public class Segment
impl Segment {
    // Java: Segment(final Vector3D start, final Vector3D end, final Line line)
    pub fn new(start: Box<dyn std::any::Any>, end: Box<dyn std::any::Any>, line: Box<dyn std::any::Any>) -> Self {
        // Java: this.start  = start;
        // Java: this.end    = end;
        // Java: this.line   = line;
        Segment { start, end, line }
    }

    // Java: Vector3D getStart()
    pub fn getStart(&self) -> &Box<dyn std::any::Any> {
        // Java: return start;
        &self.start
    }

    // Java: Vector3D getEnd()
    pub fn getEnd(&self) -> &Box<dyn std::any::Any> {
        // Java: return end;
        &self.end
    }

    // Java: Line getLine()
    pub fn getLine(&self) -> &Box<dyn std::any::Any> {
        // Java: return line;
        &self.line
    }

}
