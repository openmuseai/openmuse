#![allow(non_snake_case, dead_code)]

// Java: import java.util.Vector

use crate::cn::wps::graphics::gdi::Rectangle::Rectangle;

// Java: public class RgnData
pub struct RgnData {
    // Java field: Rectangle bounds
    pub(crate) bounds: Rectangle,
    // Java field: Vector<Rectangle> rects
    pub(crate) rects: Vec<Rectangle>,
}

// Java: public class RgnData
impl RgnData {
    // Java: RgnData(Rectangle bounds, Vector<Rectangle> rects)
    pub fn new(bounds: Rectangle, rects: Vec<Rectangle>) -> Self {
        // Java: super();
        // Java: this.bounds = bounds;
        // Java: this.rects = rects;
        RgnData { bounds, rects }
    }
}
