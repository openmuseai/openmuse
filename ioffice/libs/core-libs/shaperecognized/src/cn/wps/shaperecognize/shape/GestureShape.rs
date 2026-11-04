// Java: import android.graphics.Rect
// Java: import android.graphics.RectF
// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
// Java: import cn.wps.shaperecognize.ShapeRecognizeFactory
use crate::cn::wps::shaperecognize::ShapeRecognizeFactory::StrokePosition;
use super::Point::Point;

#[derive(Clone)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect { left, top, right, bottom }
    }
}

#[derive(Clone)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        RectF { left, top, right, bottom }
    }
}

// Java field: int SHAPE_NONE
pub const SHAPE_NONE: i32 = 0;
// Java field: int SHAPE_DELETE_LINE
pub const SHAPE_DELETE_LINE: i32 = 2;
// Java field: int SHAPE_DELETE_SLINE
pub const SHAPE_DELETE_SLINE: i32 = 4;
// Java field: int SHAPE_GENERAL_LINE
pub const SHAPE_GENERAL_LINE: i32 = 8;
// Java field: int SHAPE_NOTE_CIRCLE
pub const SHAPE_NOTE_CIRCLE: i32 = 16;
// Java field: int SHAPE_PERFECT_CIRCLE
pub const SHAPE_PERFECT_CIRCLE: i32 = 32;
// Java field: int SHAPE_GENERAL_OVAL
pub const SHAPE_GENERAL_OVAL: i32 = 64;
// Java field: int SHAPE_SWAP_LINE
pub const SHAPE_SWAP_LINE: i32 = 128;
// Java field: int SHAPE_INSERT_LINE
pub const SHAPE_INSERT_LINE: i32 = 256;
// Java field: int SHAPE_RECT
pub const SHAPE_RECT: i32 = 512;
// Java field: int SHAPE_TRANGLE
pub const SHAPE_TRANGLE: i32 = 1024;
// Java field: int SHAPE_ARROW
pub const SHAPE_ARROW: i32 = 2048;

// Java: public class GestureShape
pub struct GestureShapeBase {
    // Java field: int type
    pub type_: i32,
    // Java field: double angle
    pub angle: f64,
    // Java field: Point control
    pub control: Option<Point>,
    // Java field: ShapeConfig config
    pub config: ShapeConfig,
    // Java field: Rect wrapRect
    pub wrapRect: Option<Rect>,
    // Java field: RectF shapeRect
    pub shapeRect: Option<RectF>,
    // Java field: List<Point> points
    pub points: Vec<Point>,
    // Java field: List<ShapeRecognizeFactory.StrokePosition> strokePositions
    pub strokePositions: Option<Vec<StrokePosition>>,
}

impl GestureShapeBase {
    pub fn new(config: ShapeConfig) -> Self {
        GestureShapeBase {
            type_: 0,
            angle: 0.0,
            control: None,
            config,
            wrapRect: None,
            shapeRect: None,
            points: Vec::new(),
            strokePositions: None,
        }
    }

    // Java: GestureShape(List<Point> points, ShapeConfig config)
    pub fn init_with_points(&mut self, points: &[Point]) {
        // Java: super();
        // Java: if (null != points && points.size() > 0)
        if !points.is_empty() {
            // Java: this.points = points;
            self.points = points.to_vec();
            // Java: this.config = config;
            // Java: this.findWrapRect();
            self.findWrapRect();
            // Java: if (this.isMatch(points))
            // Java: this.initType();
        }
    }

    // Java: GestureShape(List<Point> points, List<ShapeRecognizeFactory.StrokePosition> strokePositions, ShapeConfig config)
    pub fn init_with_points_and_strokes(&mut self, points: &[Point], strokePositions: Vec<StrokePosition>) {
        // Java: super();
        // Java: if (null != points && points.size() > 0 && null != strokePositions && strokePositions.size() > 0)
        if !points.is_empty() && !strokePositions.is_empty() {
            // Java: this.points = points;
            self.points = points.to_vec();
            // Java: this.strokePositions = strokePositions;
            self.strokePositions = Some(strokePositions);
            // Java: this.config = config;
            // Java: this.findWrapRect();
            self.findWrapRect();
            // Java: if (this.isMatch(points))
            // Java: this.initType();
        }
    }

    // Java: void findWrapRect()
    fn findWrapRect(&mut self) {
        // Java: if (this.points != null && this.points.size() != 0)
        if self.points.is_empty() {
            return;
        }
        // Java: Point right;
        // Java: Point top;
        // Java: Point bottom;
        // Java: Point left = right = top = bottom = (Point)this.points.get(0);
        let mut left = self.points[0].clone();
        let mut right = self.points[0].clone();
        let mut top = self.points[0].clone();
        let mut bottom = self.points[0].clone();

        // Java: for(int i = 1; i < this.points.size(); ++i)
        for i in 1..self.points.len() {
            // Java: Point cur = (Point)this.points.get(i);
            let cur = &self.points[i];
            // Java: if (cur.x < left.x)
            if cur.x < left.x {
                // Java: left = cur;
                left = cur.clone();
            }
            // Java: if (cur.x > right.x)
            if cur.x > right.x {
                // Java: right = cur;
                right = cur.clone();
            }
            // Java: if (cur.y > bottom.y)
            if cur.y > bottom.y {
                // Java: bottom = cur;
                bottom = cur.clone();
            }
            // Java: if (cur.y < top.y)
            if cur.y < top.y {
                // Java: top = cur;
                top = cur.clone();
            }
        }

        // Java: this.wrapRect = new Rect((int)left.x, (int)top.y, (int)right.x, (int)bottom.y);
        self.wrapRect = Some(Rect::new(left.x as i32, top.y as i32, right.x as i32, bottom.y as i32));
    }

    // Java: Rect getWrapRect()
    pub fn getWrapRect(&self) -> &Option<Rect> {
        // Java: return this.wrapRect;
        &self.wrapRect
    }

    // Java: RectF getShapeRect()
    pub fn getShapeRect(&self) -> &Option<RectF> {
        // Java: return this.shapeRect;
        &self.shapeRect
    }

    // Java: Point getControl()
    pub fn getControl(&self) -> &Option<Point> {
        // Java: return this.control;
        &self.control
    }

    // Java: double getAngle()
    pub fn getAngle(&self) -> f64 {
        // Java: return this.angle;
        self.angle
    }

    // Java: abstract boolean isMatch(List<Point> var1)
    // Java: abstract void initType()

    // Java: boolean isCompleted()
    pub fn isCompleted(&self) -> bool {
        // Java: return this.type != 0;
        self.type_ != 0
    }

    // Java: List<Point> getPoints()
    pub fn getPoints(&self) -> &Vec<Point> {
        // Java: return this.points;
        &self.points
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
        // Java: return this.type;
        self.type_
    }

    // Java: StrokePosition getStroke(int inx)
    pub fn getStroke(&self, inx: i32) -> Option<&StrokePosition> {
        // Java: if (null != this.strokePositions && !this.strokePositions.isEmpty())
        if let Some(ref positions) = self.strokePositions {
            if positions.is_empty() {
                return None;
            }
            // Java: for(int i = 0; i < size; ++i)
            for position in positions.iter() {
                // Java: if (position.start <= inx && inx < position.end)
                if position.start <= inx && inx < position.end {
                    return Some(position);
                }
            }
            None
        // Java: else
        } else {
            // Java: return null;
            None
        }
    }
}

pub trait GestureShapeTrait {
    fn base(&self) -> &GestureShapeBase;
    fn base_mut(&mut self) -> &mut GestureShapeBase;
    fn isMatch(&mut self, points: &[Point]) -> bool;
    fn initType(&mut self);

    fn isCompleted(&self) -> bool {
        self.base().isCompleted()
    }

    fn getType(&self) -> i32 {
        self.base().getType()
    }
}
