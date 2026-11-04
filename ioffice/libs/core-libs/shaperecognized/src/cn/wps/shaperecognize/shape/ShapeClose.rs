// Java: import android.graphics.Path
// Java: import android.graphics.PathMeasure
// Java: import android.graphics.RectF
// Java: import android.graphics.Region
// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
// Java: import cn.wps.shaperecognize.ShapeRecognizeFactory
use crate::cn::wps::shaperecognize::ShapeRecognizeFactory::StrokePosition;
use super::GestureShape::{GestureShapeTrait, GestureShapeBase, RectF};
use super::Line::Line;
use super::Point::Point;
use super::ShapeBulge::ShapeBulgeBase;

// Java field: float CIRCLE_GAP_PERCENT
pub const CIRCLE_GAP_PERCENT: f32 = 0.1;
// Java field: float TRANGLE_VERTICAL_DST_PERCENT
pub const TRANGLE_VERTICAL_DST_PERCENT: f32 = 0.05;
// Java field: double RECT_ANGLE_ERROR
pub const RECT_ANGLE_ERROR: f64 = std::f64::consts::PI / 6.0;
// Java field: float AVERAGE_PERFECT_CIRCLE_DEVIATION_LIMIT
pub const AVERAGE_PERFECT_CIRCLE_DEVIATION_LIMIT: f32 = 0.2;
// Java field: float MAX_PERFECT_CIRCLE_DEVIATION_LIMIT
pub const MAX_PERFECT_CIRCLE_DEVIATION_LIMIT: f32 = 0.8;
// Java field: float AVERAGE_OVAL_DEVIATION_LIMIT
pub const AVERAGE_OVAL_DEVIATION_LIMIT: f32 = 0.2;
// Java field: float MAX_OVAL_DEVIATION_LIMIT
pub const MAX_OVAL_DEVIATION_LIMIT: f32 = 0.8;
// Java field: float TRANGLE_PERIMETER_GIRTH_RATIO_LIMIT
pub const TRANGLE_PERIMETER_GIRTH_RATIO_LIMIT: f32 = 0.8;
// Java field: float TRANGLE_AREA_RATIO_LIMIT
pub const TRANGLE_AREA_RATIO_LIMIT: f32 = 0.8;
// Java field: float RECTANGEL_INNER_POINT_MIN_RATIO
pub const RECTANGEL_INNER_POINT_MIN_RATIO: f32 = 0.18;
// Java field: float DF_SHORT_RATIO
pub const DF_SHORT_RATIO: f32 = 0.06666667;
// Java field: float DF_NORMAL_RATIO
pub const DF_NORMAL_RATIO: f32 = 0.04;
// Java field: float DF_LONG_RATIO
pub const DF_LONG_RATIO: f32 = 0.02;
// Java field: float SHORT_DISTANCE_MAX
pub const SHORT_DISTANCE_MAX: f32 = 30.0;
// Java field: float LONG_DISTANCE_MIN
pub const LONG_DISTANCE_MIN: f32 = 300.0;

// Java: public class ShapeClose extends ShapeBulge
pub struct ShapeClose {
    pub bulge: ShapeBulgeBase,
    // Java field: Point centerPoint
    centerPoint: Option<Point>,
    // Java field: float radius
    radius: f32,
    // Java field: boolean centerInside
    centerInside: bool,
    // Java field: Path path
    // Java field: PathMeasure pathMeasure
    // Java field: float girth
    girth: f32,
    // Java field: float strokeGapLimit
    strokeGapLimit: f32,
    // Java field: float vTrangleDstLimit
    vTrangleDstLimit: f32,
    // Java field: int[] trangleIndices
    trangleIndices: Option<Vec<i32>>,
    // Java field: double[] rectAngles
    rectAngles: Option<Vec<f64>>,
    // Java field: Point[] rectVertexes
    rectVertexes: Option<Vec<Point>>,
    // Java field: Line[] rectBorders
    rectBorders: Option<Vec<Line>>,
    // Java field: Point crossPoint
    pub crossPoint: Option<Point>,
    circle_gap_percent: f32,
}

// Java: public class ShapeClose extends ShapeBulge
impl ShapeClose {
    // Java: ShapeClose(List<Point> points, ShapeConfig config)
    pub fn new(points: &[Point], config: &ShapeConfig) -> Self {
        // Java: super(points, config);
        let mut bulge = ShapeBulgeBase::new(config.clone());
        bulge.base.init_with_points(points);
        // Java: CIRCLE_GAP_PERCENT = config.circleGapPercent;
        let circle_gap = config.circleGapPercent;
        let mut shape = ShapeClose {
            bulge,
            centerPoint: None,
            radius: 0.0,
            centerInside: false,
            girth: 0.0,
            strokeGapLimit: 0.0,
            vTrangleDstLimit: 0.0,
            trangleIndices: None,
            rectAngles: None,
            rectVertexes: None,
            rectBorders: None,
            crossPoint: None,
            circle_gap_percent: circle_gap,
        };
        if !shape.bulge.base.points.is_empty() {
            let pts: Vec<Point> = shape.bulge.base.points.clone();
            if shape.isMatch(&pts) {
                shape.initType();
            }
        }
        shape
    }

    // Java: ShapeClose(List<Point> points, List<ShapeRecognizeFactory.StrokePosition> strokePositions, ShapeConfig config)
    pub fn new_with_strokes(points: &[Point], strokePositions: Vec<StrokePosition>, config: &ShapeConfig) -> Self {
        // Java: super(points, strokePositions, config);
        let mut bulge = ShapeBulgeBase::new(config.clone());
        bulge.base.init_with_points_and_strokes(points, strokePositions);
        // Java: CIRCLE_GAP_PERCENT = config.circleGapPercent;
        let circle_gap = config.circleGapPercent;
        let mut shape = ShapeClose {
            bulge,
            centerPoint: None,
            radius: 0.0,
            centerInside: false,
            girth: 0.0,
            strokeGapLimit: 0.0,
            vTrangleDstLimit: 0.0,
            trangleIndices: None,
            rectAngles: None,
            rectVertexes: None,
            rectBorders: None,
            crossPoint: None,
            circle_gap_percent: circle_gap,
        };
        if !shape.bulge.base.points.is_empty() {
            let pts: Vec<Point> = shape.bulge.base.points.clone();
            if shape.isMatch(&pts) {
                shape.initType();
            }
        }
        shape
    }

    // Java: Point getCenterPoint()
    pub fn getCenterPoint(&self) -> &Option<Point> {
        // Java: return this.centerPoint;
        &self.centerPoint
    }

    // Java: float getRadius()
    pub fn getRadius(&self) -> f32 {
        // Java: return this.radius;
        self.radius
    }

    // Java: boolean contain(RectF rect)
    pub fn contain_rect(&self, rect: &RectF) -> bool {
        // Java: return this.contain(rect.left, rect.top, rect.right, rect.bottom);
        self.contain(rect.left, rect.top, rect.right, rect.bottom)
    }

    // Java: boolean contain(float left, float top, float right, float bottom)
    pub fn contain(&self, left: f32, top: f32, right: f32, bottom: f32) -> bool {
        if let Some(ref center) = self.centerPoint {
            // Java: float ltDis = this.calcSquareDistance(left, top, this.centerPoint.x, this.centerPoint.y);
            let ltDis = self.calcSquareDistance(left, top, center.x, center.y);
            // Java: float rtDis = this.calcSquareDistance(right, top, this.centerPoint.x, this.centerPoint.y);
            let rtDis = self.calcSquareDistance(right, top, center.x, center.y);
            // Java: float lbDis = this.calcSquareDistance(left, bottom, this.centerPoint.x, this.centerPoint.y);
            let lbDis = self.calcSquareDistance(left, bottom, center.x, center.y);
            // Java: float rbDis = this.calcSquareDistance(right, bottom, this.centerPoint.x, this.centerPoint.y);
            let rbDis = self.calcSquareDistance(right, bottom, center.x, center.y);
            // Java: float radiusSquare = this.radius * this.radius;
            let radiusSquare = self.radius * self.radius;
            // Java: return ltDis <= radiusSquare && rtDis <= radiusSquare && lbDis <= radiusSquare && rbDis <= radiusSquare;
            ltDis <= radiusSquare && rtDis <= radiusSquare && lbDis <= radiusSquare && rbDis <= radiusSquare
        } else {
            false
        }
    }
}

impl GestureShapeTrait for ShapeClose {
    fn base(&self) -> &GestureShapeBase { &self.bulge.base }
    fn base_mut(&mut self) -> &mut GestureShapeBase { &mut self.bulge.base }

    // Java: boolean isMatch(List<Point> points)
    fn isMatch(&mut self, points: &[Point]) -> bool {
        // Java: if (points != null && points.size() != 0)
        if points.is_empty() {
            // Java: return false;
            return false;
        }

        // Java: this.path = this.createPath(points);
        // Java: this.pathMeasure = new PathMeasure(this.path, true);
        // Java: this.girth = this.pathMeasure.getLength() / this.config.density;
        self.girth = Self::calcPathLength(points) / self.bulge.base.config.density;
        // Java: this.strokeGapLimit = Math.max(this.girth * CIRCLE_GAP_PERCENT, this.config.strokeWidth);
        self.strokeGapLimit = (self.girth * self.circle_gap_percent).max(self.bulge.base.config.strokeWidth);
        // Java: this.vTrangleDstLimit = this.girth * 0.05F;
        self.vTrangleDstLimit = self.girth * TRANGLE_VERTICAL_DST_PERCENT;
        // Java: float ratio = this.girth > 300.0F ? 0.02F : (this.girth <= 30.0F ? 0.06666667F : 0.04F);
        let ratio = if self.girth > LONG_DISTANCE_MIN { DF_LONG_RATIO }
            else if self.girth <= SHORT_DISTANCE_MAX { DF_SHORT_RATIO }
            else { DF_NORMAL_RATIO };
        // Java: df = this.girth * ratio;
        self.bulge.df = self.girth * ratio;

        // Java: if (null == this.path)
        // Java: return false;
        // Java: else if (!this.isStrokesClosed())
        if !self.isStrokesClosed() {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: float sum_x = 0.0F;
        let mut sum_x: f32 = 0.0;
        // Java: float sum_y = 0.0F;
        let mut sum_y: f32 = 0.0;
        // Java: float sum_x2 = 0.0F;
        let mut sum_x2: f32 = 0.0;
        // Java: float sum_y2 = 0.0F;
        let mut sum_y2: f32 = 0.0;
        // Java: float sum_x3 = 0.0F;
        let mut sum_x3: f32 = 0.0;
        // Java: float sum_y3 = 0.0F;
        let mut sum_y3: f32 = 0.0;
        // Java: float sum_xy = 0.0F;
        let mut sum_xy: f32 = 0.0;
        // Java: float sum_x1y2 = 0.0F;
        let mut sum_x1y2: f32 = 0.0;
        // Java: float sum_x2y1 = 0.0F;
        let mut sum_x2y1: f32 = 0.0;
        // Java: float sumLen = 0.0F;
        let mut sumLen: f32 = 0.0;
        // Java: float maxx = Float.MIN_VALUE;
        let mut maxx = f32::MIN;
        // Java: float minx = Float.MAX_VALUE;
        let mut minx = f32::MAX;
        // Java: float maxy = Float.MIN_VALUE;
        let mut maxy = f32::MIN;
        // Java: float miny = Float.MAX_VALUE;
        let mut miny = f32::MAX;
        // Java: int len = points.size();
        let len = points.len();

        // Java: for(int i = 0; i < len; ++i)
        for i in 0..len {
            // Java: Point point1 = (Point)points.get(i);
            // Java: float x = point1.x;
            let x = points[i].x;
            // Java: float y = point1.y;
            let y = points[i].y;
            // Java: if (i < len - 1)
            if i < len - 1 {
                // Java: Point point2 = (Point)points.get(i + 1);
                // Java: float nx = point2.x;
                let nx = points[i + 1].x;
                // Java: float ny = point2.y;
                let ny = points[i + 1].y;
                // Java: sumLen = (float)((double)sumLen + Math.sqrt((double)((nx - x) * (nx - x) + (ny - y) * (ny - y))));
                sumLen += ((nx - x) * (nx - x) + (ny - y) * (ny - y)).sqrt();
            }

            // Java: if (x > maxx)
            if x > maxx {
                // Java: maxx = x;
                maxx = x;
            }
            // Java: if (x < minx)
            if x < minx {
                // Java: minx = x;
                minx = x;
            }
            // Java: if (y > maxy)
            if y > maxy {
                // Java: maxy = y;
                maxy = y;
            }
            // Java: if (y < miny)
            if y < miny {
                // Java: miny = y;
                miny = y;
            }

            // Java: float x2 = x * x;
            let x2 = x * x;
            // Java: float y2 = y * y;
            let y2 = y * y;
            // Java: sum_x += x;
            sum_x += x;
            // Java: sum_y += y;
            sum_y += y;
            // Java: sum_x2 += x2;
            sum_x2 += x2;
            // Java: sum_y2 += y2;
            sum_y2 += y2;
            // Java: sum_x3 += x2 * x;
            sum_x3 += x2 * x;
            // Java: sum_y3 += y2 * y;
            sum_y3 += y2 * y;
            // Java: sum_xy += x * y;
            sum_xy += x * y;
            // Java: sum_x1y2 += x * y2;
            sum_x1y2 += x * y2;
            // Java: sum_x2y1 += x2 * y;
            sum_x2y1 += x2 * y;
        }

        // Java: float C = (float)len * sum_x2 - sum_x * sum_x;
        let c_val = len as f32 * sum_x2 - sum_x * sum_x;
        // Java: float D = (float)len * sum_xy - sum_x * sum_y;
        let d_val = len as f32 * sum_xy - sum_x * sum_y;
        // Java: float E = (float)len * sum_x3 + (float)len * sum_x1y2 - (sum_x2 + sum_y2) * sum_x;
        let e_val = len as f32 * sum_x3 + len as f32 * sum_x1y2 - (sum_x2 + sum_y2) * sum_x;
        // Java: float G = (float)len * sum_y2 - sum_y * sum_y;
        let g_val = len as f32 * sum_y2 - sum_y * sum_y;
        // Java: float H = (float)len * sum_x2y1 + (float)len * sum_y3 - (sum_x2 + sum_y2) * sum_y;
        let h_val = len as f32 * sum_x2y1 + len as f32 * sum_y3 - (sum_x2 + sum_y2) * sum_y;
        // Java: float a = (H * D - E * G) / (C * G - D * D);
        let a = (h_val * d_val - e_val * g_val) / (c_val * g_val - d_val * d_val);
        // Java: float b = (H * C - E * D) / (D * D - G * C);
        let b = (h_val * c_val - e_val * d_val) / (d_val * d_val - g_val * c_val);
        // Java: float c = -(a * sum_x + b * sum_y + sum_x2 + sum_y2) / (float)len;
        let c = -(a * sum_x + b * sum_y + sum_x2 + sum_y2) / len as f32;
        // Java: float rx = a / -2.0F;
        let rx = a / -2.0;
        // Java: float ry = b / -2.0F;
        let ry = b / -2.0;

        // Java: if (!this.isInBound(rx, ry))
        if !self.isInBound(rx, ry) {
            // Java: this.centerInside = false;
            self.centerInside = false;
        // Java: else
        } else {
            // Java: this.centerInside = true;
            self.centerInside = true;
        }

        // Java: float ddx = (maxx + minx) / 2.0F;
        let ddx = (maxx + minx) / 2.0;
        // Java: float ddy = (maxy + miny) / 2.0F;
        let ddy = (maxy + miny) / 2.0;
        // Java: float dmax = sumLen * CIRCLE_GAP_PERCENT;
        let mut dmax = sumLen * self.circle_gap_percent;
        // Java: Point firstPoint = (Point)points.get(len - 1);
        let firstPoint = &points[len - 1];
        // Java: float xfst = firstPoint.x;
        let mut xfst = firstPoint.x;
        // Java: float yfst = firstPoint.y;
        let mut yfst = firstPoint.y;
        // Java: boolean isCircleClosed = false;
        let mut isCircleClosed = false;
        // Java: int i = 0;
        let mid = (len - 1) / 2;

        // Java: for(int mid = (len - 1) / 2; i < mid; ++i)
        for i in 0..mid {
            // Java: Point secondPoint = (Point)points.get(i);
            let secondPoint = &points[i];
            // Java: float xsnd = secondPoint.x;
            // Java: float ysnd = secondPoint.y;
            // Java: float fsdx = Math.abs(xsnd - xfst);
            let fsdx = (secondPoint.x - xfst).abs();
            // Java: float fsdy = Math.abs(ysnd - yfst);
            let fsdy = (secondPoint.y - yfst).abs();
            // Java: if (fsdx < dmax && fsdy < dmax)
            if fsdx < dmax && fsdy < dmax {
                // Java: isCircleClosed = true;
                isCircleClosed = true;
                // Java: break;
                break;
            }
        }

        // Java: if (isCircleClosed)
        if isCircleClosed {
            // Java: isCircleClosed = false;
            isCircleClosed = false;
            // Java: firstPoint = (Point)points.get(0);
            let firstPoint = &points[0];
            // Java: xfst = firstPoint.x;
            xfst = firstPoint.x;
            // Java: yfst = firstPoint.y;
            yfst = firstPoint.y;
            // Java: i = len - 1;

            // Java: for(int mid = (len - 1) / 2; i > mid; --i)
            for i in (mid + 1..len).rev() {
                // Java: Point secondPoint = (Point)points.get(i);
                let secondPoint = &points[i];
                // Java: float xsnd = secondPoint.x;
                // Java: float ysnd = secondPoint.y;
                // Java: float fsdx = Math.abs(xfst - xsnd);
                let fsdx = (xfst - secondPoint.x).abs();
                // Java: float fsdy = Math.abs(yfst - ysnd);
                let fsdy = (yfst - secondPoint.y).abs();
                // Java: if (fsdx < dmax && fsdy < dmax)
                if fsdx < dmax && fsdy < dmax {
                    // Java: isCircleClosed = true;
                    isCircleClosed = true;
                    // Java: break;
                    break;
                }
            }
        }

        // Java: if (!isCircleClosed)
        if !isCircleClosed {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: if (this.config.supportRect() || this.config.supportTrangle() || this.config.supportOvalCircle())
        if self.bulge.base.config.supportRect() || self.bulge.base.config.supportTrangle() || self.bulge.base.config.supportOvalCircle() {
            // Java: this.findBulges();
            self.bulge.findBulges();
            // Java: this.findMaxDstBulges();
            self.bulge.findMaxDstBulges();
            // Java: this.findMaxVerBulges();
            self.bulge.findMaxVerBulges();
            // Java: this.initBulgeIndices();
            self.bulge.initBulgeIndices();
        }

        // Java: if ((this.config.supportTrangle() || this.config.supportRect()) && this.matchRectOrTrangle())
        if (self.bulge.base.config.supportTrangle() || self.bulge.base.config.supportRect()) && self.matchRectOrTrangle() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.config.supportOvalCircle() && this.matchPerfectCircle())
        if self.bulge.base.config.supportOvalCircle() && self.matchPerfectCircle() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.config.supportOvalCircle() && this.matchOval())
        if self.bulge.base.config.supportOvalCircle() && self.matchOval() {
            // Java: return true;
            return true;
        }
        // Java: else

        // Java: dmax = sumLen * CIRCLE_GAP_PERCENT / 2.0F;
        dmax = sumLen * self.circle_gap_percent / 2.0;
        // Java: if (ddx < rx + dmax && ddx > rx - dmax && ddy > ry - dmax && ddy < ry + dmax)
        if ddx < rx + dmax && ddx > rx - dmax && ddy > ry - dmax && ddy < ry + dmax {
            // Java: this.radius = (float)(Math.sqrt((double)(a * a + b * b - 4.0F * c)) / (double)2.0F);
            self.radius = ((a * a + b * b - 4.0 * c) as f64).sqrt() as f32 / 2.0;
            // Java: this.centerPoint = new Point(rx, ry);
            self.centerPoint = Some(Point::new_with_xy(rx, ry));
            // Java: return true;
            return true;
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: return false;
    }

    // Java: void initType()
    fn initType(&mut self) {
        // Java: if (this.type == 0 && this.config.supportNoteCircle())
        if self.bulge.base.type_ == 0 && self.bulge.base.config.supportNoteCircle() {
            // Java: this.type = 16;
            self.bulge.base.type_ = 16;
        }
    }
}

impl ShapeClose {
    // Java: boolean isCenterInside()
    pub fn isCenterInside(&self) -> bool {
        // Java: return this.centerInside;
        self.centerInside
    }

    // Java: boolean isInBound(float rx, float ry)
    fn isInBound(&self, rx: f32, ry: f32) -> bool {
        // Java: RectF rect = new RectF();
        // Java: this.path.computeBounds(rect, true);
        // Java: Region re = new Region();
        // Java: re.setPath(this.path, new Region((int)rect.left, (int)rect.top, (int)rect.right, (int)rect.bottom));
        // Java: return re.contains((int)rx, (int)ry);
        let points = &self.bulge.base.points;
        if points.len() < 3 {
            return false;
        }
        let mut inside = false;
        let n = points.len();
        let mut j = n - 1;
        for i in 0..n {
            let yi = points[i].y;
            let xi = points[i].x;
            let yj = points[j].y;
            let xj = points[j].x;
            if ((yi > ry) != (yj > ry)) && (rx < (xj - xi) * (ry - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    // Java: Path createPath(List<Point> points)
    fn createPath(&self, points: &[Point]) -> bool {
        // Java: if (points != null && points.size() >= 2)
        if points.len() >= 2 {
            // Java: Path path = new Path();
            // Java: Point point0 = (Point)points.get(0);
            // Java: path.moveTo(point0.x, point0.y);
            // Java: float previousX = point0.x;
            // Java: float previousY = point0.y;
            // Java: int len = points.size();
            // Java: for(int i = 1; i < len; ++i)
            // Java: Point point = (Point)points.get(i);
            // Java: float dx = Math.abs(point.x - previousX);
            // Java: float dy = Math.abs(point.y - previousY);
            // Java: if (dx >= 3.0F || dy >= 3.0F)
            // Java: float cX = (point.x + previousX) / 2.0F;
            // Java: float cY = (point.y + previousY) / 2.0F;
            // Java: path.quadTo(previousX, previousY, cX, cY);
            // Java: previousX = point.x;
            // Java: previousY = point.y;
            // Java: return path;
            true
        // Java: else
        } else {
            // Java: return null;
            false
        }
    }

    fn calcPathLength(points: &[Point]) -> f32 {
        if points.len() < 2 { return 0.0; }
        let mut length: f32 = 0.0;
        for i in 0..points.len() - 1 {
            let dx = points[i + 1].x - points[i].x;
            let dy = points[i + 1].y - points[i].y;
            length += ((dx * dx + dy * dy) as f64).sqrt() as f32;
        }
        length
    }

    // Java: boolean checkCircleDeviation(float cx, float cy, float radius)
    fn checkCircleDeviation(&self, cx: f32, cy: f32, radius: f32) -> bool {
        // Java: int calCount = Math.min(360, this.points.size());
        let calCount = 360_usize.min(self.bulge.base.points.len());
        // Java: int step = this.points.size() / calCount;
        let step = if calCount > 0 { self.bulge.base.points.len() / calCount } else { 1 };
        let step = step.max(1);
        // Java: float sradius = radius * radius;
        let sradius = radius * radius;
        // Java: int size = this.points.size();
        let size = self.bulge.base.points.len();
        // Java: float totalErr = 0.0F;
        let mut totalErr: f32 = 0.0;
        // Java: float maxErr = 0.0F;
        let mut maxErr: f32 = 0.0;

        // Java: for(int i = 0; i < size; i += step)
        let mut i = 0;
        while i < size {
            // Java: Point point = (Point)this.points.get(i);
            let point = &self.bulge.base.points[i];
            // Java: float dx = point.x - cx;
            let dx = point.x - cx;
            // Java: float dy = point.y - cy;
            let dy = point.y - cy;
            // Java: float err = Math.abs((dx * dx + dy * dy) / sradius - 1.0F);
            let err = ((dx * dx + dy * dy) / sradius - 1.0).abs();
            // Java: maxErr = Math.max(err, maxErr);
            maxErr = err.max(maxErr);
            // Java: totalErr += err;
            totalErr += err;
            i += step;
        }

        // Java: return totalErr * (float)step / (float)size < 0.2F && maxErr < 0.8F;
        totalErr * step as f32 / (size as f32) < AVERAGE_PERFECT_CIRCLE_DEVIATION_LIMIT
            && maxErr < MAX_PERFECT_CIRCLE_DEVIATION_LIMIT
    }

    // Java: boolean checkOvalDeviation(float sqrmajor, float sqrminor, float cos, float sin, Point center)
    fn checkOvalDeviation(&self, sqrmajor: f32, sqrminor: f32, cos: f32, sin: f32, center: &Point) -> bool {
        // Java: int size = this.points.size();
        let size = self.bulge.base.points.len();
        // Java: int calCount = Math.min(360, size);
        let calCount = 360_usize.min(size);
        // Java: int step = this.points.size() / calCount;
        let step = if calCount > 0 { self.bulge.base.points.len() / calCount } else { 1 };
        let step = step.max(1);
        // Java: float totalErr = 0.0F;
        let mut totalErr: f32 = 0.0;
        // Java: float maxErr = 0.0F;
        let mut maxErr: f32 = 0.0;
        // Java: Point point = new Point();
        let mut point = Point::new();

        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: point.set((Point)this.points.get(i));
            point.set(&self.bulge.base.points[i]);
            // Java: this.rotatePoint(point, center, cos, sin);
            Self::rotatePoint_static(&mut point, center, cos, sin);
            // Java: float dx = point.x - center.x;
            let dx = point.x - center.x;
            // Java: float dy = point.y - center.y;
            let dy = point.y - center.y;
            // Java: float err = Math.abs(dx * dx / sqrmajor + dy * dy / sqrminor - 1.0F);
            let err = (dx * dx / sqrmajor + dy * dy / sqrminor - 1.0).abs();
            // Java: maxErr = Math.max(err, maxErr);
            maxErr = err.max(maxErr);
            // Java: totalErr += err;
            totalErr += err;
        }

        // Java: return totalErr * (float)step / (float)size < 0.2F && maxErr < 0.8F;
        totalErr * step as f32 / (size as f32) < AVERAGE_OVAL_DEVIATION_LIMIT
            && maxErr < MAX_OVAL_DEVIATION_LIMIT
    }

    // Java: boolean bulgeMatchRect(Point point)
    fn bulgeMatchRect(&self, point: &Option<Point>) -> bool {
        // Java: if (null != point && null != this.crossPoint)
        if let (Some(p), Some(ref cross)) = (point, &self.crossPoint) {
            // Java: float toMaxDst = Point.getDst(point, this.crossPoint);
            let toMaxDst = Point::getDst(p, cross);
            // Java: float halfLen = this.maxDst / 2.0F * 0.7F;
            let halfLen = self.bulge.maxDst / 2.0 * 0.7;
            // Java: return !(toMaxDst < halfLen);
            !(toMaxDst < halfLen)
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: boolean matchTrangle(Point point)
    fn matchTrangle(&mut self, point: &Option<Point>) -> bool {
        // Java: if (null != point)
        if let Some(ref p) = point {
            // Java: float vdst = Line.getVDstPToL(this.maxDstLine, point) / this.config.density;
            if let Some(ref max_dst_line) = self.bulge.maxDstLine {
                let vdst = Line::getVDstPToL(max_dst_line, p) / self.bulge.base.config.density;
                // Java: if (vdst > this.vTrangleDstLimit)
                if vdst > self.vTrangleDstLimit {
                    // Java: return false;
                    return false;
                }
            }
        }

        // Java: if (null == this.trangleIndices)
        if self.trangleIndices.is_none() {
            // Java: this.trangleIndices = new int[3];
            self.trangleIndices = Some(vec![0; 3]);
        }

        // Java: int[] trInxes = this.trangleIndices;
        let tr_inxes;
        // Java: if (this.bulgeIndices.length > 3)
        if self.bulge.bulgeIndices.len() > 3 {
            // Java: int excInx = this.points.indexOf(point);
            let excInx = point.as_ref().and_then(|p|
                self.bulge.base.points.iter().position(|pt| Point::equals(Some(pt), Some(p)))
            ).map(|i| i as i32).unwrap_or(-1);
            // Java: int trInx = 0;
            let mut new_inxes = Vec::new();
            // Java: for(int i = 0; i < this.bulgeIndices.length; ++i)
            for &idx in &self.bulge.bulgeIndices {
                // Java: if (this.bulgeIndices[i] != excInx)
                if idx != excInx {
                    // Java: trInxes[trInx++] = this.bulgeIndices[i];
                    new_inxes.push(idx);
                }
            }
            tr_inxes = new_inxes;
        // Java: else
        } else {
            // Java: if (this.bulgeIndices.length != 3)
            if self.bulge.bulgeIndices.len() != 3 {
                // Java: return false;
                return false;
            }
            // Java: trInxes = this.bulgeIndices;
            tr_inxes = self.bulge.bulgeIndices.clone();
        }

        if tr_inxes.len() < 3 { return false; }

        // Java: float bulgePerimeter = 0.0F;
        let mut bulgePerimeter: f32 = 0.0;
        // Java: for(int i = 0; i < trInxes.length; ++i)
        for i in 0..tr_inxes.len() {
            // Java: int j = i + 1 < trInxes.length ? i + 1 : 0;
            let j = if i + 1 < tr_inxes.len() { i + 1 } else { 0 };
            // Java: bulgePerimeter += Point.getDst((Point)this.points.get(trInxes[i]), (Point)this.points.get(trInxes[j]));
            bulgePerimeter += Point::getDst(
                &self.bulge.base.points[tr_inxes[i] as usize],
                &self.bulge.base.points[tr_inxes[j] as usize],
            );
        }

        // Java: bulgePerimeter /= this.config.density;
        bulgePerimeter /= self.bulge.base.config.density;
        // Java: if (bulgePerimeter / this.girth < 0.8F)
        if bulgePerimeter / self.girth < TRANGLE_PERIMETER_GIRTH_RATIO_LIMIT {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: int vertexInx = 0;
        let mut vertexInx = 0i32;
        let df = self.bulge.df;

        // Java: for(int i = 0; i < trInxes.length; ++i)
        for i in 0..tr_inxes.len() {
            // Java: int j = i + 1 < trInxes.length ? i + 1 : 0;
            let j = if i + 1 < tr_inxes.len() { i + 1 } else { 0 };
            // Java: if (trInxes[i] != this.maxDstStartInx && trInxes[i] != this.maxDstEndInx || trInxes[j] != this.maxDstStartInx && trInxes[j] != this.maxDstEndInx)
            if (tr_inxes[i] != self.bulge.maxDstStartInx && tr_inxes[i] != self.bulge.maxDstEndInx)
                || (tr_inxes[j] != self.bulge.maxDstStartInx && tr_inxes[j] != self.bulge.maxDstEndInx) {
                // Java: Line sideLine = Line.getAbsoluteFitLine(this.points, trInxes[i], trInxes[j] + 1, df);
                let sideLine = Line::getAbsoluteFitLine(
                    &self.bulge.base.points, tr_inxes[i], tr_inxes[j] + 1, df,
                );
                // Java: if (null == sideLine)
                if sideLine.is_none() {
                    // Java: return false;
                    return false;
                }
            // Java: else
            } else {
                // Java: Line baseLine = Line.getAbsoluteFitLine(this.points, trInxes[i], trInxes[j] + 1, df);
                let baseLine = Line::getAbsoluteFitLine(
                    &self.bulge.base.points, tr_inxes[i], tr_inxes[j] + 1, df,
                );
                // Java: if (null == baseLine)
                if baseLine.is_none() {
                    // Java: return false;
                    return false;
                }
            }

            // Java: if (trInxes[i] != this.maxDstStartInx && trInxes[i] != this.maxDstEndInx)
            if tr_inxes[i] != self.bulge.maxDstStartInx && tr_inxes[i] != self.bulge.maxDstEndInx {
                // Java: vertexInx = trInxes[i];
                vertexInx = tr_inxes[i];
            }
        }

        // Java: Point base1 = new Point(this.maxDstStartPt);
        let mut base1 = Point::new_from_point(self.bulge.maxDstStartPt.as_ref().unwrap());
        // Java: Point base2 = new Point(this.maxDstEndPt);
        let mut base2 = Point::new_from_point(self.bulge.maxDstEndPt.as_ref().unwrap());
        // Java: Point vertex = new Point((Point)this.points.get(vertexInx));
        let mut vertex = Point::new_from_point(&self.bulge.base.points[vertexInx as usize]);

        // Java: if (null != this.maxVerStartPt && null != this.maxVerEndPt)
        if self.bulge.maxVerStartPt.is_some() && self.bulge.maxVerEndPt.is_some() {
            let max_dst_line = self.bulge.maxDstLine.as_ref().unwrap();
            // Java: float b = Point.getDst(base1, base2);
            let b = Point::getDst(&base1, &base2);
            // Java: float h = Line.getVDstPToL(this.maxDstLine, vertex);
            let h = Line::getVDstPToL(max_dst_line, &vertex);
            // Java: float h1 = Line.getVDstPToL(this.maxDstLine, vertexInx == this.maxVerStartInx ? this.maxVerEndPt : this.maxVerStartPt);
            let other_pt = if vertexInx == self.bulge.maxVerStartInx {
                self.bulge.maxVerEndPt.as_ref().unwrap()
            } else {
                self.bulge.maxVerStartPt.as_ref().unwrap()
            };
            let h1 = Line::getVDstPToL(max_dst_line, other_pt);
            // Java: float s = b * h;
            let s = b * h;
            // Java: float s1 = b * h1;
            let s1 = b * h1;
            // Java: if (s / (s + s1) < 0.8F)
            if s / (s + s1) < TRANGLE_AREA_RATIO_LIMIT {
                // Java: return false;
                return false;
            }
        }

        // Java: this.angle = Math.atan((double)this.maxDstLine.getSlope());
        let max_dst_line = self.bulge.maxDstLine.as_ref().unwrap();
        self.bulge.base.angle = (max_dst_line.getSlope() as f64).atan();
        // Java: if (this.angle < (double)0.0F)
        if self.bulge.base.angle < 0.0 {
            // Java: this.angle += Math.PI;
            self.bulge.base.angle += std::f64::consts::PI;
        }

        // Java: float dx21 = base2.x - base1.x;
        let dx21 = base2.x - base1.x;
        // Java: float dy21 = base2.y - base1.y;
        let dy21 = base2.y - base1.y;
        // Java: float b1 = base1.x * dx21 + base1.y * dy21;
        let mut b1_val = base1.x * dx21 + base1.y * dy21;
        // Java: float b2 = vertex.x * dy21 - vertex.y * dx21;
        let b2_val = vertex.x * dy21 - vertex.y * dx21;
        // Java: float D = -dx21 * dx21 - dy21 * dy21;
        let d_val = -dx21 * dx21 - dy21 * dy21;
        // Java: if (Double.compare((double)D, (double)0.0F) == 0)
        if d_val == 0.0 {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: float D1 = -b1 * dx21 - b2 * dy21;
        let mut d1 = -b1_val * dx21 - b2_val * dy21;
        // Java: float D2 = dx21 * b2 - dy21 * b1;
        let mut d2 = dx21 * b2_val - dy21 * b1_val;
        // Java: Point vertex1 = new Point(D1 / D, D2 / D);
        let mut vertex1 = Point::new_with_xy(d1 / d_val, d2 / d_val);
        // Java: b1 = base2.x * dx21 + base2.y * dy21;
        b1_val = base2.x * dx21 + base2.y * dy21;
        // Java: D1 = -b1 * dx21 - b2 * dy21;
        d1 = -b1_val * dx21 - b2_val * dy21;
        // Java: D2 = dx21 * b2 - dy21 * b1;
        d2 = dx21 * b2_val - dy21 * b1_val;
        // Java: Point vertex2 = new Point(D1 / D, D2 / D);
        let mut vertex2 = Point::new_with_xy(d1 / d_val, d2 / d_val);
        // Java: Point center = new Point((vertex1.x + base2.x) / 2.0F, (vertex1.y + base2.y) / 2.0F);
        let center = Point::new_with_xy(
            (vertex1.x + base2.x) / 2.0,
            (vertex1.y + base2.y) / 2.0,
        );
        // Java: this.angle = -this.angle;
        self.bulge.base.angle = -self.bulge.base.angle;
        // Java: float cos = (float)Math.cos(this.angle);
        let mut cos = self.bulge.base.angle.cos() as f32;
        // Java: float sin = (float)Math.sin(this.angle);
        let mut sin = self.bulge.base.angle.sin() as f32;
        // Java: this.rotatePoint(vertex1, center, cos, sin);
        Self::rotatePoint_static(&mut vertex1, &center, cos, sin);
        // Java: this.rotatePoint(base1, center, cos, sin);
        Self::rotatePoint_static(&mut base1, &center, cos, sin);
        // Java: if (vertex1.y > base1.y)
        if vertex1.y > base1.y {
            // Java: cos = -1.0F;
            cos = -1.0;
            // Java: sin = 0.0F;
            sin = 0.0;
            // Java: this.rotatePoint(vertex1, center, cos, sin);
            Self::rotatePoint_static(&mut vertex1, &center, cos, sin);
            // Java: this.rotatePoint(base1, center, cos, sin);
            Self::rotatePoint_static(&mut base1, &center, cos, sin);
            // Java: this.angle += Math.PI;
            self.bulge.base.angle += std::f64::consts::PI;
            // Java: cos = (float)Math.cos(this.angle);
            cos = self.bulge.base.angle.cos() as f32;
            // Java: sin = (float)Math.sin(this.angle);
            sin = self.bulge.base.angle.sin() as f32;
        }

        // Java: this.rotatePoint(vertex2, center, cos, sin);
        Self::rotatePoint_static(&mut vertex2, &center, cos, sin);
        // Java: this.rotatePoint(base2, center, cos, sin);
        Self::rotatePoint_static(&mut base2, &center, cos, sin);
        // Java: this.rotatePoint(vertex, center, cos, sin);
        Self::rotatePoint_static(&mut vertex, &center, cos, sin);
        // Java: this.shapeRect = new RectF(Math.min(vertex1.x, vertex2.x), vertex.y, Math.max(vertex1.x, vertex2.x), base1.y);
        self.bulge.base.shapeRect = Some(RectF::new(
            vertex1.x.min(vertex2.x), vertex.y,
            vertex1.x.max(vertex2.x), base1.y,
        ));
        // Java: this.control = new Point(vertex.x, vertex.y);
        self.bulge.base.control = Some(Point::new_with_xy(vertex.x, vertex.y));
        // Java: this.angle = -this.angle;
        self.bulge.base.angle = -self.bulge.base.angle;
        // Java: this.type = 1024;
        self.bulge.base.type_ = 1024;

        // Java: if (point == this.maxVerStartPt)
        if point.as_ref().map_or(false, |p| {
            self.bulge.maxVerStartPt.as_ref().map_or(false, |s| Point::equals(Some(p), Some(s)))
        }) {
            // Java: this.maxVerStartPt = null;
            self.bulge.maxVerStartPt = None;
        }
        // Java: if (point == this.maxVerEndPt)
        if point.as_ref().map_or(false, |p| {
            self.bulge.maxVerEndPt.as_ref().map_or(false, |e| Point::equals(Some(p), Some(e)))
        }) {
            // Java: this.maxVerEndPt = null;
            self.bulge.maxVerEndPt = None;
        }

        // Java: return true;
        true
    }

    // Java: void rotatePoint(Point point, Point center, float cos, float sin)
    fn rotatePoint_static(point: &mut Point, center: &Point, cos: f32, sin: f32) {
        // Java: float x = point.x - center.x;
        let x = point.x - center.x;
        // Java: float y = point.y - center.y;
        let y = point.y - center.y;
        // Java: point.x = x * cos - y * sin + center.x;
        point.x = x * cos - y * sin + center.x;
        // Java: point.y = y * cos + x * sin + center.y;
        point.y = y * cos + x * sin + center.y;
    }

    // Java: boolean matchRect()
    fn matchRect(&mut self) -> bool {
        // Java: int borderNum = 4;
        // Java: if (this.config.supportRect() && this.bulgeIndices.length == 4)
        if !self.bulge.base.config.supportRect() || self.bulge.bulgeIndices.len() != 4 {
            // Java: return false;
            return false;
        }

        // Java: if (null == this.rectAngles)
        if self.rectAngles.is_none() {
            // Java: this.rectAngles = new double[4];
            self.rectAngles = Some(vec![0.0; 4]);
        }
        // Java: if (null == this.rectBorders)
        if self.rectBorders.is_none() {
            // Java: this.rectBorders = new Line[4];
            self.rectBorders = Some(Vec::new());
        }

        let df = self.bulge.df;
        let mut angles = vec![0.0f64; 4];
        let mut borders = Vec::new();

        // Java: for(int i = 0; i < 4; ++i)
        for i in 0..4 {
            // Java: int j = i + 1;
            let mut j = i + 1;
            // Java: if (j >= 4)
            if j >= 4 {
                // Java: j = 0;
                j = 0;
            }

            // Java: Line checkLine = Line.getAbsoluteFitLine(this.points, this.bulgeIndices[i], this.bulgeIndices[j] + 1, df);
            let checkLine = Line::getAbsoluteFitLine(
                &self.bulge.base.points,
                self.bulge.bulgeIndices[i],
                self.bulge.bulgeIndices[j] + 1,
                df,
            );
            // Java: if (null == checkLine)
            if checkLine.is_none() {
                // Java: return false;
                return false;
            }
            let cl = checkLine.unwrap();
            // Java: this.rectAngles[i] = Math.atan((double)checkLine.getSlope());
            angles[i] = (cl.getSlope() as f64).atan();
            // Java: this.rectBorders[i] = checkLine;
            borders.push(cl);
        }

        // Java: double angle0 = this.rectAngles[0];
        let angle0 = angles[0];
        // Java: this.angle = angle0;
        self.bulge.base.angle = angle0;

        // Java: for(int i = 0; i < this.rectAngles.length; ++i)
        for i in 0..angles.len() {
            // Java: if (Math.abs(this.rectAngles[i]) < Math.abs(this.angle))
            if angles[i].abs() < self.bulge.base.angle.abs() {
                // Java: this.angle = this.rectAngles[i];
                self.bulge.base.angle = angles[i];
            }

            // Java: int j = i + 1;
            let j = i + 1;
            // Java: if (j < this.rectAngles.length)
            if j < angles.len() {
                // Java: this.rectAngles[i] = this.rectAngles[j] - this.rectAngles[i];
                angles[i] = angles[j] - angles[i];
            // Java: else
            } else {
                // Java: this.rectAngles[i] = angle0 - this.rectAngles[i];
                angles[i] = angle0 - angles[i];
            }

            // Java: if (Math.abs(Math.abs(this.rectAngles[i]) - (Math.PI / 2D)) > (Math.PI / 6D))
            if (angles[i].abs() - std::f64::consts::FRAC_PI_2).abs() > RECT_ANGLE_ERROR {
                // Java: return false;
                return false;
            }
        }

        // Java: if (null == this.rectVertexes)
        // Java: this.rectVertexes = new Point[4];
        let mut rectVertexes = Vec::new();
        // Java: float sumX = 0.0F;
        let mut sumX: f32 = 0.0;
        // Java: float sumY = 0.0F;
        let mut sumY: f32 = 0.0;

        // Java: for(int i = 0; i < 4; ++i)
        for i in 0..4 {
            // Java: int j = i + 1 < 4 ? i + 1 : 0;
            let j = if i + 1 < 4 { i + 1 } else { 0 };
            // Java: this.rectVertexes[i] = Line.getIntersection(this.rectBorders[i], this.rectBorders[j]);
            if let Some(v) = Line::getIntersection(&borders[i], &borders[j]) {
                // Java: sumX += this.rectVertexes[i].x;
                sumX += v.x;
                // Java: sumY += this.rectVertexes[i].y;
                sumY += v.y;
                rectVertexes.push(v);
            } else {
                return false;
            }
        }

        // Java: int inCount = 0;
        let mut inCount = 0;
        // Java: int i = 0;
        // Java: for(int size = this.points.size(); i < size; ++i)
        for i in 0..self.bulge.base.points.len() {
            // Java: if (this.isInPolygon((Point)this.points.get(i)))
            if self.isInPolygon(&self.bulge.base.points[i]) {
                // Java: ++inCount;
                inCount += 1;
            }
        }

        // Java: if ((float)inCount / (float)this.points.size() < 0.18F)
        if inCount as f32 / (self.bulge.base.points.len() as f32) < RECTANGEL_INNER_POINT_MIN_RATIO {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: float cos = (float)Math.cos(-this.angle);
        let cos = (-self.bulge.base.angle).cos() as f32;
        // Java: float sin = (float)Math.sin(-this.angle);
        let sin = (-self.bulge.base.angle).sin() as f32;
        // Java: float left = Float.MAX_VALUE;
        let mut left = f32::MAX;
        // Java: float top = Float.MAX_VALUE;
        let mut top = f32::MAX;
        // Java: float right = Float.MIN_VALUE;
        let mut right = f32::MIN;
        // Java: float bottom = Float.MIN_VALUE;
        let mut bottom = f32::MIN;
        // Java: Point center = new Point(sumX / 4.0F, sumY / 4.0F);
        let center = Point::new_with_xy(sumX / 4.0, sumY / 4.0);

        // Java: for(i = 0; i < this.rectVertexes.length; ++i)
        for v in &mut rectVertexes {
            // Java: this.rotatePoint(this.rectVertexes[i], center, cos, sin);
            Self::rotatePoint_static(v, &center, cos, sin);
            // Java: left = Math.min(left, this.rectVertexes[i].x);
            left = left.min(v.x);
            // Java: right = Math.max(right, this.rectVertexes[i].x);
            right = right.max(v.x);
            // Java: top = Math.min(top, this.rectVertexes[i].y);
            top = top.min(v.y);
            // Java: bottom = Math.max(bottom, this.rectVertexes[i].y);
            bottom = bottom.max(v.y);
        }

        // Java: this.shapeRect = new RectF(left, top, right, bottom);
        self.bulge.base.shapeRect = Some(RectF::new(left, top, right, bottom));
        // Java: this.type = 512;
        self.bulge.base.type_ = 512;
        self.rectVertexes = Some(rectVertexes);
        // Java: return true;
        // Java: else
        // Java: return false;
        true
    }

    // Java: boolean isInPolygon(Point point)
    fn isInPolygon(&self, point: &Point) -> bool {
        // Java: boolean isLastLeft = false;
        let mut isLastLeft = false;
        // Java: int i = 0;
        let size = self.bulge.bulgeIndices.len();
        // Java: for(int size = this.bulgeIndices.length; i < size; ++i)
        for i in 0..size {
            // Java: int j = i + 1 < size ? i + 1 : 0;
            let j = if i + 1 < size { i + 1 } else { 0 };
            // Java: boolean isCurLeft = this.isOnLeft((Point)this.points.get(this.bulgeIndices[i]), (Point)this.points.get(this.bulgeIndices[j]), point);
            let isCurLeft = self.isOnLeft(
                &self.bulge.base.points[self.bulge.bulgeIndices[i] as usize],
                &self.bulge.base.points[self.bulge.bulgeIndices[j] as usize],
                point,
            );
            // Java: if (0 == i)
            if i == 0 {
                // Java: isLastLeft = isCurLeft;
                isLastLeft = isCurLeft;
            // Java: else
            } else {
                // Java: if (isLastLeft != isCurLeft)
                if isLastLeft != isCurLeft {
                    // Java: return false;
                    return false;
                }
                // Java: isLastLeft = isCurLeft;
                isLastLeft = isCurLeft;
            }
        }
        // Java: return true;
        true
    }

    // Java: float calcSquareDistance(float x1, float y1, float x2, float y2)
    fn calcSquareDistance(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        // Java: float xd = x1 - x2;
        let xd = x1 - x2;
        // Java: float yd = y1 - y2;
        let yd = y1 - y2;
        // Java: return xd * xd + yd * yd;
        xd * xd + yd * yd
    }

    // Java: boolean isOnLeft(Point lineStart, Point lineEnd, Point point)
    fn isOnLeft(&self, lineStart: &Point, lineEnd: &Point, point: &Point) -> bool {
        // Java: return lineStart.x * (lineEnd.y - point.y) + lineStart.y * (point.x - lineEnd.x) + lineEnd.x * point.y - lineEnd.y * point.x > 0.0F;
        lineStart.x * (lineEnd.y - point.y) + lineStart.y * (point.x - lineEnd.x)
            + lineEnd.x * point.y - lineEnd.y * point.x > 0.0
    }

    // Java: boolean matchRectOrTrangle()
    pub fn matchRectOrTrangle(&mut self) -> bool {
        // Java: if (!this.config.supportTrangle() && !this.config.supportRect())
        if !self.bulge.base.config.supportTrangle() && !self.bulge.base.config.supportRect() {
            // Java: return false;
            return false;
        }
        // Java: else if (null != this.maxDstLine && null != this.maxDstStartPt && null != this.maxDstEndPt && (null != this.maxVerStartPt || null != this.maxVerEndPt))
        if self.bulge.maxDstLine.is_none() || self.bulge.maxDstStartPt.is_none()
            || self.bulge.maxDstEndPt.is_none()
            || (self.bulge.maxVerStartPt.is_none() && self.bulge.maxVerEndPt.is_none()) {
            // Java: return false;
            return false;
        }

        // Java: if (null != this.maxVerStartPt && null != this.maxVerEndPt)
        if self.bulge.maxVerStartPt.is_some() && self.bulge.maxVerEndPt.is_some() {
            // Java: this.crossPoint = Line.getIntersection(this.maxDstLine, Line.getLine(this.maxVerStartPt, this.maxVerEndPt));
            let max_dst_line = self.bulge.maxDstLine.clone().unwrap();
            let ver_start = self.bulge.maxVerStartPt.clone().unwrap();
            let ver_end = self.bulge.maxVerEndPt.clone().unwrap();
            if let Some(ver_line) = Line::getLine(&ver_start, &ver_end) {
                self.crossPoint = Line::getIntersection(&max_dst_line, &ver_line);
            }
        }

        // Java: if (this.config.supportTrangle() && !this.bulgeMatchRect(this.maxVerEndPt) && this.matchTrangle(this.maxVerEndPt))
        if self.bulge.base.config.supportTrangle() && !self.bulgeMatchRect(&self.bulge.maxVerEndPt.clone()) {
            let ver_end = self.bulge.maxVerEndPt.clone();
            if self.matchTrangle(&ver_end) {
                // Java: return true;
                return true;
            }
        }
        // Java: else
        // Java: return this.config.supportTrangle() && !this.bulgeMatchRect(this.maxVerStartPt) && this.matchTrangle(this.maxVerStartPt) ? true : this.matchRect();
        if self.bulge.base.config.supportTrangle() && !self.bulgeMatchRect(&self.bulge.maxVerStartPt.clone()) {
            let ver_start = self.bulge.maxVerStartPt.clone();
            if self.matchTrangle(&ver_start) {
                return true;
            }
        }
        // Java: else
        // Java: return false;
        self.matchRect()
    }

    // Java: boolean matchPerfectCircle()
    pub fn matchPerfectCircle(&mut self) -> bool {
        // Java: if (null != this.maxDstStartPt && null != this.maxDstEndPt && null != this.maxVerStartPt && null != this.maxVerEndPt)
        if self.bulge.maxDstStartPt.is_none() || self.bulge.maxDstEndPt.is_none()
            || self.bulge.maxVerStartPt.is_none() || self.bulge.maxVerEndPt.is_none() {
            // Java: return false;
            return false;
        }
        let ver_start = self.bulge.maxVerStartPt.as_ref().unwrap();
        let ver_end = self.bulge.maxVerEndPt.as_ref().unwrap();
        // Java: this.centerPoint = new Point((this.maxVerStartPt.x + this.maxVerEndPt.x) / 2.0F, (this.maxVerStartPt.y + this.maxVerEndPt.y) / 2.0F);
        self.centerPoint = Some(Point::new_with_xy(
            (ver_start.x + ver_end.x) / 2.0,
            (ver_start.y + ver_end.y) / 2.0,
        ));
        // Java: this.radius = Point.getDst(this.maxVerStartPt, this.maxVerEndPt) / 2.0F;
        self.radius = Point::getDst(ver_start, ver_end) / 2.0;
        let cp = self.centerPoint.as_ref().unwrap();
        let cx = cp.x;
        let cy = cp.y;
        // Java: if (this.checkCircleDeviation(this.centerPoint.x, this.centerPoint.y, this.radius))
        if self.checkCircleDeviation(cx, cy, self.radius) {
            // Java: this.type = 32;
            self.bulge.base.type_ = 32;
            // Java: this.shapeRect = new RectF(this.centerPoint.x - this.radius, this.centerPoint.y - this.radius, this.centerPoint.x + this.radius, this.centerPoint.y + this.radius);
            self.bulge.base.shapeRect = Some(RectF::new(
                cx - self.radius, cy - self.radius,
                cx + self.radius, cy + self.radius,
            ));
            // Java: this.angle = (double)0.0F;
            self.bulge.base.angle = 0.0;
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

    // Java: boolean matchOval()
    pub fn matchOval(&mut self) -> bool {
        // Java: if (null != this.maxDstStartPt && null != this.maxDstEndPt && null != this.maxVerStartPt && null != this.maxVerEndPt)
        if self.bulge.maxDstStartPt.is_none() || self.bulge.maxDstEndPt.is_none()
            || self.bulge.maxVerStartPt.is_none() || self.bulge.maxVerEndPt.is_none() {
            // Java: return false;
            return false;
        }
        let max_dst_line = self.bulge.maxDstLine.as_ref().unwrap();
        // Java: this.angle = Math.atan((double)this.maxDstLine.getSlope());
        self.bulge.base.angle = (max_dst_line.getSlope() as f64).atan();
        let dst_start = self.bulge.maxDstStartPt.as_ref().unwrap();
        let dst_end = self.bulge.maxDstEndPt.as_ref().unwrap();
        // Java: Point center = new Point((this.maxDstStartPt.x + this.maxDstEndPt.x) / 2.0F, (this.maxDstStartPt.y + this.maxDstEndPt.y) / 2.0F);
        let center = Point::new_with_xy(
            (dst_start.x + dst_end.x) / 2.0,
            (dst_start.y + dst_end.y) / 2.0,
        );
        // Java: float cos = (float)Math.cos(-this.angle);
        let cos = (-self.bulge.base.angle).cos() as f32;
        // Java: float sin = (float)Math.sin(-this.angle);
        let sin = (-self.bulge.base.angle).sin() as f32;
        // Java: float major = Point.getDst(this.maxDstStartPt, this.maxDstEndPt) / 2.0F;
        let major = Point::getDst(dst_start, dst_end) / 2.0;
        let ver_start = self.bulge.maxVerStartPt.as_ref().unwrap();
        let ver_end = self.bulge.maxVerEndPt.as_ref().unwrap();
        // Java: float minor = (Line.getVDstPToL(this.maxDstLine, this.maxVerStartPt) + Line.getVDstPToL(this.maxDstLine, this.maxVerEndPt)) / 2.0F;
        let minor = (Line::getVDstPToL(max_dst_line, ver_start)
            + Line::getVDstPToL(max_dst_line, ver_end)) / 2.0;
        // Java: float smajor = major * major;
        let smajor = major * major;
        // Java: float sminor = minor * minor;
        let sminor = minor * minor;
        // Java: if (this.checkOvalDeviation(smajor, sminor, cos, sin, center))
        if self.checkOvalDeviation(smajor, sminor, cos, sin, &center) {
            // Java: this.type = 64;
            self.bulge.base.type_ = 64;
            // Java: this.shapeRect = new RectF(center.x - major, center.y - minor, center.x + major, center.y + minor);
            self.bulge.base.shapeRect = Some(RectF::new(
                center.x - major, center.y - minor,
                center.x + major, center.y + minor,
            ));
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

    // Java: boolean isStrokesClosed()
    pub fn isStrokesClosed(&self) -> bool {
        // Java: if (null != this.strokePositions && this.strokePositions.size() > 1)
        if let Some(ref positions) = self.bulge.base.strokePositions {
            if positions.len() <= 1 {
                // Java: return true;
                return true;
            }
            // Java: int i = 0;
            let size = positions.len();
            // Java: for(int size = this.strokePositions.size(); i < size; ++i)
            for i in 0..size {
                // Java: int j = i + 1 < size ? i + 1 : 0;
                let j = if i + 1 < size { i + 1 } else { 0 };
                // Java: ShapeRecognizeFactory.StrokePosition stroke1 = (ShapeRecognizeFactory.StrokePosition)this.strokePositions.get(i);
                let stroke1 = &positions[i];
                // Java: ShapeRecognizeFactory.StrokePosition stroke2 = (ShapeRecognizeFactory.StrokePosition)this.strokePositions.get(j);
                let stroke2 = &positions[j];
                // Java: Point point1 = (Point)this.points.get(stroke1.end - 1);
                let point1 = &self.bulge.base.points[(stroke1.end - 1) as usize];
                // Java: Point point2 = (Point)this.points.get(stroke2.start);
                let point2 = &self.bulge.base.points[stroke2.start as usize];
                // Java: if (Point.getDst(point1, point2) / this.config.density >= this.strokeGapLimit)
                if Point::getDst(point1, point2) / self.bulge.base.config.density >= self.strokeGapLimit {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return true;
            true
        }
    }

    // Java: List<Point> getBulges()
    pub fn getBulges(&self) -> &Vec<Point> {
        // Java: return this.bulgeList;
        &self.bulge.bulgeList
    }

    // Java: Point getMaxStartPoint()
    pub fn getMaxStartPoint(&self) -> &Option<Point> {
        // Java: return this.maxDstStartPt;
        &self.bulge.maxDstStartPt
    }

    // Java: Point getMaxEndPoint()
    pub fn getMaxEndPoint(&self) -> &Option<Point> {
        // Java: return this.maxDstEndPt;
        &self.bulge.maxDstEndPt
    }

    // Java: Point getMaxVerPtStart()
    pub fn getMaxVerPtStart(&self) -> &Option<Point> {
        // Java: return this.maxVerStartPt;
        &self.bulge.maxVerStartPt
    }

    // Java: Point getMaxVerPtEnd()
    pub fn getMaxVerPtEnd(&self) -> &Option<Point> {
        // Java: return this.maxVerEndPt;
        &self.bulge.maxVerEndPt
    }

    // Java: Point getTranBugle()
    pub fn getTranBugle(&self) -> Option<&Point> {
        // Java: if (this.type == 1024)
        if self.bulge.base.type_ == 1024 {
            // Java: return this.maxVerEndPt == null ? this.maxVerStartPt : this.maxVerEndPt;
            if self.bulge.maxVerEndPt.is_none() {
                self.bulge.maxVerStartPt.as_ref()
            } else {
                self.bulge.maxVerEndPt.as_ref()
            }
        // Java: else
        } else {
            // Java: return null;
            None
        }
    }
}
