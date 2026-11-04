// Java: import android.graphics.RectF
// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
use super::GestureShape::{GestureShapeBase, GestureShapeTrait, RectF};
use super::Line::Line;
use super::Point::Point;

// Java: public class ShapeGeneralLine extends GestureShape
pub struct ShapeGeneralLine {
    pub base: GestureShapeBase,
    // Java field: Point start
    start: Option<Point>,
    // Java field: Point end
    end: Option<Point>,
}

// Java field: float deviation
pub static deviation: f32 = 0.05;

// Java: public class ShapeGeneralLine extends GestureShape
impl ShapeGeneralLine {
    // Java: ShapeGeneralLine(List<Point> points, ShapeConfig config)
    pub fn new(points: &[Point], config: &ShapeConfig) -> Self {
        // Java: super(points, config);
        let mut base = GestureShapeBase::new(config.clone());
        base.init_with_points(points);
        let mut shape = ShapeGeneralLine {
            base,
            start: None,
            end: None,
        };
        if !shape.base.points.is_empty() {
            let pts: Vec<Point> = shape.base.points.clone();
            if shape.isMatch(&pts) {
                shape.initType();
            }
        }
        shape
    }

    // Java: Point getStart()
    pub fn getStart(&self) -> &Option<Point> {
        // Java: return this.start;
        &self.start
    }

    // Java: Point getEnd()
    pub fn getEnd(&self) -> &Option<Point> {
        // Java: return this.end;
        &self.end
    }
}

impl GestureShapeTrait for ShapeGeneralLine {
    fn base(&self) -> &GestureShapeBase { &self.base }
    fn base_mut(&mut self) -> &mut GestureShapeBase { &mut self.base }

    // Java: boolean isMatch(List<Point> points)
    fn isMatch(&mut self, points: &[Point]) -> bool {
        // Java: if (null != points && !points.isEmpty())
        if points.is_empty() { return false; }
        // Java: Line line = Line.getRelativeFitLine(points, deviation);
        let line = Line::getRelativeFitLine_with_dvtion(points, deviation);
        // Java: if (line != null)
        if line.is_some() {
            // Java: this.start = (Point)points.get(0);
            self.start = Some(points[0].clone());
            // Java: this.end = (Point)points.get(points.size() - 1);
            self.end = Some(points[points.len() - 1].clone());
            // Java: this.angle = (double)0.0F;
            self.base.angle = 0.0;
            // Java: this.shapeRect = new RectF(this.start.x, this.start.y, this.end.x, this.end.y);
            let s = self.start.as_ref().unwrap();
            let e = self.end.as_ref().unwrap();
            self.base.shapeRect = Some(RectF::new(s.x, s.y, e.x, e.y));
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
        // Java: else
        // Java: return false;
    }

    // Java: void initType()
    fn initType(&mut self) {
        // Java: this.type = 8;
        self.base.type_ = 8;
    }
}
