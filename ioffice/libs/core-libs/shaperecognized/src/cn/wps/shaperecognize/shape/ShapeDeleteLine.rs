// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
use super::GestureShape::{GestureShapeBase, GestureShapeTrait};
use super::Line::Line;
use super::Point::Point;

// Java field: int LINE_TYPE_NONE
pub const LINE_TYPE_NONE: i32 = 0;
// Java field: int LINE_TYPE_HORIZONTAL
pub const LINE_TYPE_HORIZONTAL: i32 = 1;
// Java field: int LINE_TYPE_VERTICAL
pub const LINE_TYPE_VERTICAL: i32 = 2;
// Java field: int LINE_TYPE_LEFT
pub const LINE_TYPE_LEFT: i32 = 4;
// Java field: int LINE_TYPE_RIGHT
pub const LINE_TYPE_RIGHT: i32 = 8;

// Java field: float MINK
pub const MINK: f32 = -0.2;
// Java field: float MAXK
pub const MAXK: f32 = 0.2;
// Java field: float DEVIATION
pub const DEVIATION: f32 = 5.0;

// Java: public class ShapeDeleteLine extends GestureShape
pub struct ShapeDeleteLine {
    pub base: GestureShapeBase,
    // Java field: int shapeType
    pub shapeType: i32,
    // Java field: float[] leftRange
    leftRange: [f32; 2],
    // Java field: float[] rightRange
    rightRange: [f32; 2],
    // Java field: float[] verticalRange
    verticalRange: [f32; 2],
    // Java field: float[] horizontalRange
    horizontalRange: [f32; 2],
    // Java field: Point start
    start: Option<Point>,
    // Java field: Point end
    end: Option<Point>,
}

// Java: public class ShapeDeleteLine extends GestureShape
impl ShapeDeleteLine {
    // Java: ShapeDeleteLine(List<Point> points, ShapeConfig config)
    pub fn new(points: &[Point], config: &ShapeConfig) -> Self {
        // Java: super(points, config);
        let mut base = GestureShapeBase::new(config.clone());
        base.init_with_points(points);
        // Java: MINK = config.lintKMinDvtion;
        let mink = config.lintKMinDvtion;
        // Java: MAXK = config.lineKMaxDvtion;
        let maxk = config.lineKMaxDvtion;
        let mut shape = ShapeDeleteLine {
            base,
            shapeType: 0,
            leftRange: [-3.0, -0.4],
            rightRange: [0.4, 3.0],
            verticalRange: [-9.0, 9.0],
            horizontalRange: [mink, maxk],
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

impl GestureShapeTrait for ShapeDeleteLine {
    fn base(&self) -> &GestureShapeBase { &self.base }
    fn base_mut(&mut self) -> &mut GestureShapeBase { &mut self.base }

    // Java: boolean isMatch(List<Point> points)
    fn isMatch(&mut self, points: &[Point]) -> bool {
        // Java: Line line = Line.getAbsoluteFitLine(points, DEVIATION);
        let line = Line::getAbsoluteFitLine_with_dvtion(points, DEVIATION);
        // Java: if (line != null)
        if let Some(line) = line {
            // Java: float k = line.getSlope();
            let k = line.getSlope();
            // Java: if (this.isHorizontal(k) || this.isVerticalLine(k) || this.isLeftLine(k) || this.isRightLine(k))
            if self.isHorizontal(k) || self.isVerticalLine(k) || self.isLeftLine(k) || self.isRightLine(k) {
                // Java: this.start = (Point)points.get(0);
                self.start = Some(points[0].clone());
                // Java: this.end = (Point)points.get(points.size() - 1);
                self.end = Some(points[points.len() - 1].clone());
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: void initType()
    fn initType(&mut self) {
        // Java: this.type = 2;
        self.base.type_ = 2;
    }
}

impl ShapeDeleteLine {
    // Java: boolean isHorizontal(float k)
    pub fn isHorizontal(&mut self, k: f32) -> bool {
        // Java: if (k > horizontalRange[0] && k < horizontalRange[1])
        if k > self.horizontalRange[0] && k < self.horizontalRange[1] {
            // Java: this.shapeType = 1;
            self.shapeType = LINE_TYPE_HORIZONTAL;
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: boolean isLeftLine(float k)
    pub fn isLeftLine(&mut self, k: f32) -> bool {
        // Java: if (k > leftRange[0] && k < leftRange[1])
        if k > self.leftRange[0] && k < self.leftRange[1] {
            // Java: this.shapeType = 4;
            self.shapeType = LINE_TYPE_LEFT;
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: boolean isRightLine(float k)
    pub fn isRightLine(&mut self, k: f32) -> bool {
        // Java: if (k > rightRange[0] && k < rightRange[1])
        if k > self.rightRange[0] && k < self.rightRange[1] {
            // Java: this.shapeType = 8;
            self.shapeType = LINE_TYPE_RIGHT;
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: boolean isVerticalLine(float k)
    pub fn isVerticalLine(&mut self, k: f32) -> bool {
        // Java: if (!(k > verticalRange[1]) && !(k < verticalRange[0]))
        if !(k > self.verticalRange[1]) && !(k < self.verticalRange[0]) {
            // Java: return false;
            false
        // Java: else
        } else {
            // Java: this.shapeType = 2;
            self.shapeType = LINE_TYPE_VERTICAL;
            // Java: return true;
            true
        }
    }
}
