// Java: import android.graphics.RectF
// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
// Java: import cn.wps.shaperecognize.ShapeRecognizeFactory
use crate::cn::wps::shaperecognize::ShapeRecognizeFactory::StrokePosition;
use super::GestureShape::{GestureShapeBase, GestureShapeTrait, RectF};
use super::Line::Line;
use super::Point::Point;
use super::ShapeBulge::ShapeBulgeBase;

// Java field: double MAX_HEAD_ANGLE
pub const MAX_HEAD_ANGLE: f64 = 1.4726215563702154;
// Java field: double MIN_HEAD_ANGLE
pub const MIN_HEAD_ANGLE: f64 = std::f64::consts::PI / 180.0;
// Java field: double MAX_HEAD_WING_ANGLE_DIFF
pub const MAX_HEAD_WING_ANGLE_DIFF: f64 = std::f64::consts::PI / 180.0;
// Java field: float MAX_POINT_INTERSECT_RATIO
pub const MAX_POINT_INTERSECT_RATIO: f32 = 0.2;
// Java field: float MAX_HEAD_LENGTH_DIFF_RATIO
pub const MAX_HEAD_LENGTH_DIFF_RATIO: f32 = 0.5;
// Java field: float NEIGHBOR_RADIUS_RATIO
pub const NEIGHBOR_RADIUS_RATIO: f32 = 0.04;
// Java field: float DF_SHORT_RATIO
pub const DF_SHORT_RATIO: f32 = 0.1;
// Java field: float DF_NORMAL_RATIO
pub const DF_NORMAL_RATIO: f32 = 0.05;
// Java field: float DF_LONG_RATIO
pub const DF_LONG_RATIO: f32 = 0.025;
// Java field: float SHORT_DISTANCE_MAX
pub const SHORT_DISTANCE_MAX: f32 = 30.0;
// Java field: float LONG_DISTANCE_MIN
pub const LONG_DISTANCE_MIN: f32 = 300.0;

// Java: public class ShapeArrow extends ShapeBulge
pub struct ShapeArrow {
    pub bulge: ShapeBulgeBase,
    // Java field: int tailInx
    tailInx: i32,
    // Java field: int headInx
    headInx: i32,
    // Java field: int wingInx1
    wingInx1: i32,
    // Java field: int wingInx2
    wingInx2: i32,
    // Java field: Point tailPoint
    tailPoint: Option<Point>,
    // Java field: Point headPoint
    headPoint: Option<Point>,
    // Java field: Point wingPoint1
    wingPoint1: Option<Point>,
    // Java field: Point wingPoint2
    wingPoint2: Option<Point>,
    // Java field: ShapeRecognizeFactory.StrokePosition tailStroke
    tailStroke: Option<StrokePosition>,
    // Java field: ShapeRecognizeFactory.StrokePosition headStroke
    headStroke: Option<StrokePosition>,
    // Java field: ShapeRecognizeFactory.StrokePosition wingStroke1
    wingStroke1: Option<StrokePosition>,
    // Java field: ShapeRecognizeFactory.StrokePosition wingStroke2
    wingStroke2: Option<StrokePosition>,
    // Java field: float neighborSquareRadius
    neighborSquareRadius: f32,
    // Java field: float headWingDst1
    headWingDst1: f32,
    // Java field: float headWingDst2
    headWingDst2: f32,
}

// Java: public class ShapeArrow extends ShapeBulge
impl ShapeArrow {
    // Java: ShapeArrow(List<Point> points, List<ShapeRecognizeFactory.StrokePosition> strokePositions, ShapeConfig config)
    pub fn new(points: &[Point], strokePositions: Vec<StrokePosition>, config: &ShapeConfig) -> Self {
        // Java: super(points, strokePositions, config);
        let mut bulge = ShapeBulgeBase::new(config.clone());
        bulge.base.init_with_points_and_strokes(points, strokePositions);
        let mut shape = ShapeArrow {
            bulge,
            tailInx: 0, headInx: 0, wingInx1: 0, wingInx2: 0,
            tailPoint: None, headPoint: None, wingPoint1: None, wingPoint2: None,
            tailStroke: None, headStroke: None, wingStroke1: None, wingStroke2: None,
            neighborSquareRadius: 0.0, headWingDst1: 0.0, headWingDst2: 0.0,
        };
        if !shape.bulge.base.points.is_empty() {
            let pts: Vec<Point> = shape.bulge.base.points.clone();
            if shape.isMatch(&pts) {
                shape.initType();
            }
        }
        shape
    }

    // Java: boolean isMatch(List<Point> points)
    fn isMatch_impl(&mut self, _points: &[Point]) -> bool {
        // Java: if (!this.config.supportArrow())
        if !self.bulge.base.config.supportArrow() {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: this.findBulges();
        self.bulge.findBulges();
        // Java: this.findMaxDstBulges();
        self.bulge.findMaxDstBulges();
        // Java: this.findMaxVerBulges();
        self.bulge.findMaxVerBulges();
        // Java: this.initBulgeIndices();
        self.bulge.initBulgeIndices();
        // Java: if (this.bulgeIndices.length < 4)
        if self.bulge.bulgeIndices.len() < 4 {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: ShapeRecognizeFactory.StrokePosition maxDstStartStroke = this.getStroke(this.maxDstStartInx);
        let maxDstStartStroke = self.bulge.base.getStroke(self.bulge.maxDstStartInx).cloned();
        // Java: ShapeRecognizeFactory.StrokePosition maxDstEndStroke = this.getStroke(this.maxDstEndInx);
        let maxDstEndStroke = self.bulge.base.getStroke(self.bulge.maxDstEndInx).cloned();
        // Java: ShapeRecognizeFactory.StrokePosition maxVerStartStroke = this.getStroke(this.maxVerStartInx);
        let maxVerStartStroke = self.bulge.base.getStroke(self.bulge.maxVerStartInx).cloned();
        // Java: ShapeRecognizeFactory.StrokePosition maxVerEndStroke = this.getStroke(this.maxVerEndInx);
        let maxVerEndStroke = self.bulge.base.getStroke(self.bulge.maxVerEndInx).cloned();

        if maxDstStartStroke.is_none() || maxDstEndStroke.is_none() || maxVerStartStroke.is_none() || maxVerEndStroke.is_none() {
            return false;
        }

        // Java: if (!this.getComponents(maxDstStartStroke, maxDstEndStroke, maxVerStartStroke, maxVerEndStroke))
        if !self.getComponents(&maxDstStartStroke.unwrap(), &maxDstEndStroke.unwrap(), &maxVerStartStroke.unwrap(), &maxVerEndStroke.unwrap()) {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: this.shapeRect = new RectF(this.tailPoint.x, this.tailPoint.y, this.headPoint.x, this.headPoint.y);
        self.bulge.base.shapeRect = Some(RectF::new(
            self.tailPoint.as_ref().unwrap().x,
            self.tailPoint.as_ref().unwrap().y,
            self.headPoint.as_ref().unwrap().x,
            self.headPoint.as_ref().unwrap().y,
        ));
        // Java: this.angle = (double)0.0F;
        self.bulge.base.angle = 0.0;
        // Java: return true;
        true
    }

    // Java: void initType()
    fn initType_impl(&mut self) {
        // Java: this.type = 2048;
        self.bulge.base.type_ = 2048;
    }

    // Java: int getNearestPoint(ShapeRecognizeFactory.StrokePosition strokePosition, Point point)
    fn getNearestPoint(&self, strokePosition: &StrokePosition, point: &Point) -> i32 {
        // Java: float minDst = Float.MAX_VALUE;
        let mut minDst = f32::MAX;
        // Java: int nearestPt = strokePosition.start;
        let mut nearestPt = strokePosition.start;

        // Java: for(int i = strokePosition.start; i < strokePosition.end; ++i)
        for i in strokePosition.start..strokePosition.end {
            // Java: float dst = point.getSquareDst((Point)this.points.get(i));
            let dst = point.getSquareDst(&self.bulge.base.points[i as usize]);
            // Java: if (dst < minDst)
            if dst < minDst {
                // Java: minDst = dst;
                minDst = dst;
                // Java: nearestPt = i;
                nearestPt = i;
            }
        }

        // Java: return nearestPt;
        nearestPt
    }

    // Java: int getNeighborPoint(int fromInx, int toInx, float squareRadius)
    fn getNeighborPoint(&self, fromInx: i32, toInx: i32, squareRadius: f32) -> i32 {
        // Java: Point point = (Point)this.points.get(toInx);
        let point = &self.bulge.base.points[toInx as usize];
        let mut i = fromInx;
        // Java: for(int i = fromInx; i != toInx; i = fromInx < toInx ? i + 1 : i - 1)
        while i != toInx {
            // Java: if (point.getSquareDst((Point)this.points.get(i)) < squareRadius)
            if point.getSquareDst(&self.bulge.base.points[i as usize]) < squareRadius {
                // Java: return i;
                return i;
            }
            i = if fromInx < toInx { i + 1 } else { i - 1 };
        }

        // Java: return toInx;
        toInx
    }

    // Java: Line getLine(ShapeRecognizeFactory.StrokePosition fromStroke, int fromInx, ShapeRecognizeFactory.StrokePosition toStroke, int toInx)
    fn getLineFromStrokes(&mut self, fromStroke: &StrokePosition, fromInx: i32, toStroke: &StrokePosition, toInx: i32) -> Option<Line> {
        // Java: Point fromPt = (Point)this.points.get(fromInx);
        let fromPt = &self.bulge.base.points[fromInx as usize];
        // Java: Point toPt = (Point)this.points.get(toInx);
        let toPt = &self.bulge.base.points[toInx as usize];
        // Java: float distance = Point.getDst(fromPt, toPt) / this.config.density;
        let distance = Point::getDst(fromPt, toPt) / self.bulge.base.config.density;
        // Java: float ratio = distance > 300.0F ? 0.025F : (distance <= 30.0F ? 0.1F : 0.05F);
        let ratio = if distance > LONG_DISTANCE_MIN { DF_LONG_RATIO }
            else if distance <= SHORT_DISTANCE_MAX { DF_SHORT_RATIO }
            else { DF_NORMAL_RATIO };
        // Java: df = distance * ratio;
        self.bulge.df = distance * ratio;
        // Java: return this.getLine(fromStroke, fromInx, toStroke, toInx, df);
        self.getLineFromStrokesWithDev(fromStroke, fromInx, toStroke, toInx, self.bulge.df)
    }

    // Java: Line getLine(ShapeRecognizeFactory.StrokePosition fromStroke, int fromInx, ShapeRecognizeFactory.StrokePosition toStroke, int toInx, float fitDeviation)
    fn getLineFromStrokesWithDev(&mut self, fromStroke: &StrokePosition, fromInx: i32, toStroke: &StrokePosition, toInx: i32, fitDeviation: f32) -> Option<Line> {
        // Java: Point fromPt = (Point)this.points.get(fromInx);
        // Java: Point toPt = (Point)this.points.get(toInx);
        // Java: if (fromStroke == toStroke)
        let sameStroke = fromStroke.start == toStroke.start && fromStroke.end == toStroke.end;
        if sameStroke {
            // Java: int startInx = Math.min(fromInx, toInx);
            let startInx = fromInx.min(toInx);
            // Java: int endInx = Math.max(fromInx, toInx);
            let endInx = fromInx.max(toInx);
            // Java: return Line.getAbsoluteFitLine(this.points, startInx, endInx + 1, fitDeviation);
            return Line::getAbsoluteFitLine(&self.bulge.base.points, startInx, endInx + 1, fitDeviation);
        }
        // Java: else
        // Java: int nearestInx = this.getNearestPoint(fromStroke, (Point)this.points.get(toInx));
        let nearestInx = self.getNearestPoint(fromStroke, &self.bulge.base.points[toInx as usize].clone());
        // Java: int startInx = Math.min(nearestInx, fromInx);
        let startInx = nearestInx.min(fromInx);
        // Java: int endInx = Math.max(nearestInx, fromInx);
        let endInx = nearestInx.max(fromInx);
        // Java: Point nearestPt = (Point)this.points.get(nearestInx);
        let nearestPt = &self.bulge.base.points[nearestInx as usize];
        let fromPt = &self.bulge.base.points[fromInx as usize];
        let toPt = &self.bulge.base.points[toInx as usize];
        // Java: if (nearestPt.getSquareDst(fromPt) < nearestPt.getSquareDst(toPt))
        if nearestPt.getSquareDst(fromPt) < nearestPt.getSquareDst(toPt) {
            // Java: return null;
            return None;
        }
        // Java: else

        let insert_pt = self.bulge.base.points[toInx as usize].clone();
        let mut nearestInx = nearestInx;
        // Java: if (nearestInx > fromInx)
        if nearestInx > fromInx {
            // Java: ++nearestInx;
            nearestInx += 1;
            // Java: this.points.add(nearestInx, (Point)this.points.get(toInx));
            self.bulge.base.points.insert(nearestInx as usize, insert_pt);
        // Java: else
        } else {
            // Java: nearestInx = nearestInx > 0 ? nearestInx - 1 : 0;
            nearestInx = if nearestInx > 0 { nearestInx - 1 } else { 0 };
            // Java: this.points.add(nearestInx, (Point)this.points.get(toInx));
            self.bulge.base.points.insert(nearestInx as usize, insert_pt);
        }

        // Java: Line line = Line.getAbsoluteFitLine(this.points, Math.min(nearestInx, fromInx), Math.max(nearestInx, fromInx) + 1, fitDeviation);
        let line = Line::getAbsoluteFitLine(
            &self.bulge.base.points,
            nearestInx.min(fromInx),
            nearestInx.max(fromInx) + 1,
            fitDeviation,
        );
        // Java: this.points.remove(nearestInx);
        self.bulge.base.points.remove(nearestInx as usize);
        // Java: if (null != line)
        if let Some(mut l) = line {
            // Java: line.setStartInx(startInx);
            l.setStartInx(startInx);
            // Java: line.setEndInx(endInx);
            l.setEndInx(endInx);
            Some(l)
        } else {
            None
        }

        // Java: return line;
    }

    // Java: Line getNeighborLine(int fromInx, int toInx, float squareRadius, float fitAvgDeviation)
    fn getNeighborLine(&mut self, fromInx: i32, toInx: i32, squareRadius: f32, fitAvgDeviation: f32) -> Option<Line> {
        // Java: int neighborInx = this.getNeighborPoint(fromInx, toInx, squareRadius);
        let neighborInx = self.getNeighborPoint(fromInx, toInx, squareRadius);
        // Java: Point toPoint = (Point)this.points.get(toInx);
        let toPoint = self.bulge.base.points[toInx as usize].clone();
        // Java: int startInx = Math.min(fromInx, neighborInx);
        let startInx = fromInx.min(neighborInx);
        // Java: int endInx = Math.max(fromInx, neighborInx);
        let endInx = fromInx.max(neighborInx);
        let mut neighborInx = neighborInx;

        // Java: if (fromInx < toInx)
        if fromInx < toInx {
            // Java: ++neighborInx;
            neighborInx += 1;
            // Java: this.points.add(neighborInx, toPoint);
            self.bulge.base.points.insert(neighborInx as usize, toPoint);
        // Java: else
        } else {
            // Java: List var10000 = this.points;
            // Java: int var10001;
            // Java: if (neighborInx > 0)
            if neighborInx > 0 {
                // Java: --neighborInx;
                neighborInx -= 1;
                // Java: var10001 = neighborInx;
            // Java: else
            } else {
                // Java: var10001 = 0;
                neighborInx = 0;
            }
            // Java: var10000.add(var10001, toPoint);
            self.bulge.base.points.insert(neighborInx as usize, toPoint);
        }

        // Java: Line line = Line.getAbsoluteFitLine(this.points, Math.min(fromInx, neighborInx), Math.max(fromInx, neighborInx) + 1, fitAvgDeviation);
        let line = Line::getAbsoluteFitLine(
            &self.bulge.base.points,
            fromInx.min(neighborInx),
            fromInx.max(neighborInx) + 1,
            fitAvgDeviation,
        );
        // Java: this.points.remove(neighborInx);
        self.bulge.base.points.remove(neighborInx as usize);
        // Java: if (null != line)
        if let Some(mut l) = line {
            // Java: line.setStartInx(startInx);
            l.setStartInx(startInx);
            // Java: line.setEndInx(endInx);
            l.setEndInx(endInx);
            Some(l)
        } else {
            None
        }

        // Java: return line;
    }

    // Java: boolean getComponents(ShapeRecognizeFactory.StrokePosition maxDstStartStroke, ShapeRecognizeFactory.StrokePosition maxDstEndStroke, ShapeRecognizeFactory.StrokePosition maxVerStartStroke, ShapeRecognizeFactory.StrokePosition maxVerEndStroke)
    fn getComponents(&mut self, maxDstStartStroke: &StrokePosition, maxDstEndStroke: &StrokePosition, maxVerStartStroke: &StrokePosition, maxVerEndStroke: &StrokePosition) -> bool {
        // Java: this.headInx = this.maxDstStartInx;
        self.headInx = self.bulge.maxDstStartInx;
        // Java: this.headStroke = maxDstStartStroke;
        self.headStroke = Some(maxDstStartStroke.clone());
        // Java: this.headPoint = this.maxDstStartPt;
        self.headPoint = self.bulge.maxDstStartPt.clone();
        // Java: this.tailInx = this.maxDstEndInx;
        self.tailInx = self.bulge.maxDstEndInx;
        // Java: this.tailStroke = maxDstEndStroke;
        self.tailStroke = Some(maxDstEndStroke.clone());
        // Java: this.tailPoint = this.maxDstEndPt;
        self.tailPoint = self.bulge.maxDstEndPt.clone();
        // Java: this.wingInx1 = this.maxVerStartInx;
        self.wingInx1 = self.bulge.maxVerStartInx;
        // Java: this.wingStroke1 = maxVerStartStroke;
        self.wingStroke1 = Some(maxVerStartStroke.clone());
        // Java: this.wingPoint1 = this.maxVerStartPt;
        self.wingPoint1 = self.bulge.maxVerStartPt.clone();
        // Java: this.wingInx2 = this.maxVerEndInx;
        self.wingInx2 = self.bulge.maxVerEndInx;
        // Java: this.wingStroke2 = maxVerEndStroke;
        self.wingStroke2 = Some(maxVerEndStroke.clone());
        // Java: this.wingPoint2 = this.maxVerEndPt;
        self.wingPoint2 = self.bulge.maxVerEndPt.clone();
        // Java: this.headWingDst1 = Point.getDst(this.headPoint, this.wingPoint1);
        self.headWingDst1 = Point::getDst(self.headPoint.as_ref().unwrap(), self.wingPoint1.as_ref().unwrap());
        // Java: this.headWingDst2 = Point.getDst(this.headPoint, this.wingPoint2);
        self.headWingDst2 = Point::getDst(self.headPoint.as_ref().unwrap(), self.wingPoint2.as_ref().unwrap());
        // Java: this.neighborSquareRadius = Math.max(this.headWingDst1, this.headWingDst2);
        self.neighborSquareRadius = self.headWingDst1.max(self.headWingDst2);
        // Java: this.neighborSquareRadius *= this.neighborSquareRadius * 0.04F;
        self.neighborSquareRadius *= self.neighborSquareRadius * NEIGHBOR_RADIUS_RATIO;
        // Java: if (this.checkComponents())
        if self.checkComponents() {
            // Java: return true;
            return true;
        }
        // Java: else
        // Java: this.headInx = this.maxDstEndInx;
        self.headInx = self.bulge.maxDstEndInx;
        // Java: this.headStroke = maxDstEndStroke;
        self.headStroke = Some(maxDstEndStroke.clone());
        // Java: this.headPoint = this.maxDstEndPt;
        self.headPoint = self.bulge.maxDstEndPt.clone();
        // Java: this.tailInx = this.maxDstStartInx;
        self.tailInx = self.bulge.maxDstStartInx;
        // Java: this.tailStroke = maxDstStartStroke;
        self.tailStroke = Some(maxDstStartStroke.clone());
        // Java: this.tailPoint = this.maxDstStartPt;
        self.tailPoint = self.bulge.maxDstStartPt.clone();
        // Java: this.headWingDst1 = Point.getDst(this.headPoint, this.wingPoint1);
        self.headWingDst1 = Point::getDst(self.headPoint.as_ref().unwrap(), self.wingPoint1.as_ref().unwrap());
        // Java: this.headWingDst2 = Point.getDst(this.headPoint, this.wingPoint2);
        self.headWingDst2 = Point::getDst(self.headPoint.as_ref().unwrap(), self.wingPoint2.as_ref().unwrap());
        // Java: this.neighborSquareRadius = Math.min(this.headWingDst1, this.headWingDst2);
        self.neighborSquareRadius = self.headWingDst1.min(self.headWingDst2);
        // Java: this.neighborSquareRadius *= this.neighborSquareRadius * 0.04F;
        self.neighborSquareRadius *= self.neighborSquareRadius * NEIGHBOR_RADIUS_RATIO;
        // Java: return this.checkComponents();
        self.checkComponents()
    }

    // Java: boolean checkComponents()
    fn checkComponents(&mut self) -> bool {
        // Java: Line wingLine1 = null;
        // Java: Line wingLine2 = null;
        let tail_stroke = match &self.tailStroke { Some(s) => s.clone(), None => return false };
        let head_stroke = match &self.headStroke { Some(s) => s.clone(), None => return false };
        let wing_stroke1 = match &self.wingStroke1 { Some(s) => s.clone(), None => return false };
        let wing_stroke2 = match &self.wingStroke2 { Some(s) => s.clone(), None => return false };

        let df = self.bulge.df;
        let nsqr = self.neighborSquareRadius;

        // Java: Line shaftLine = this.getLine(this.tailStroke, this.tailInx, this.headStroke, this.headInx);
        let mut shaftLine = self.getLineFromStrokes(&tail_stroke, self.tailInx, &head_stroke, self.headInx);
        // Java: if (null == shaftLine && this.tailStroke == this.headStroke)
        let same_tail_head = tail_stroke.start == head_stroke.start && tail_stroke.end == head_stroke.end;
        if shaftLine.is_none() && same_tail_head {
            // Java: shaftLine = this.getNeighborLine(this.tailInx, this.headInx, this.neighborSquareRadius, df);
            shaftLine = self.getNeighborLine(self.tailInx, self.headInx, nsqr, df);
        }

        // Java: if (null != shaftLine)
        if shaftLine.is_none() { return false; }
        let shaft = shaftLine.unwrap();

        // Java: wingLine1 = this.getLine(this.wingStroke1, this.wingInx1, this.headStroke, this.headInx);
        let mut wingLine1 = self.getLineFromStrokes(&wing_stroke1, self.wingInx1, &head_stroke, self.headInx);
        // Java: if (null == wingLine1 && this.wingStroke1 == this.headStroke)
        let same_wing1_head = wing_stroke1.start == head_stroke.start && wing_stroke1.end == head_stroke.end;
        if wingLine1.is_none() && same_wing1_head {
            // Java: wingLine1 = this.getNeighborLine(this.wingInx1, this.headInx, this.neighborSquareRadius, df);
            wingLine1 = self.getNeighborLine(self.wingInx1, self.headInx, nsqr, df);
        }

        // Java: if (null != wingLine1)
        if wingLine1.is_none() { return false; }
        let wl1 = wingLine1.unwrap();

        // Java: if ((float)shaftLine.getIntersectPointSize(wingLine1) / (float)Math.min(wingLine1.getPointSize(), shaftLine.getPointSize()) > 0.2F)
        if wl1.getIntersectPointSize(&shaft) as f32 / (wl1.getPointSize().min(shaft.getPointSize())) as f32 > MAX_POINT_INTERSECT_RATIO {
            // Java: return false;
            return false;
        }

        // Java: wingLine2 = this.getLine(this.wingStroke2, this.wingInx2, this.headStroke, this.headInx);
        let mut wingLine2 = self.getLineFromStrokes(&wing_stroke2, self.wingInx2, &head_stroke, self.headInx);
        // Java: if (null == wingLine2 && this.wingStroke2 == this.headStroke)
        let same_wing2_head = wing_stroke2.start == head_stroke.start && wing_stroke2.end == head_stroke.end;
        if wingLine2.is_none() && same_wing2_head {
            // Java: wingLine2 = this.getNeighborLine(this.wingInx2, this.headInx, this.neighborSquareRadius, df);
            wingLine2 = self.getNeighborLine(self.wingInx2, self.headInx, nsqr, df);
        }

        // Java: if (null != wingLine2 && Math.abs(this.headWingDst1 - this.headWingDst2) / Math.max(this.headWingDst1, this.headWingDst2) < 0.5F)
        if wingLine2.is_none() || (self.headWingDst1 - self.headWingDst2).abs() / self.headWingDst1.max(self.headWingDst2) >= MAX_HEAD_LENGTH_DIFF_RATIO {
            // Java: return false;
            return false;
        }
        let wl2 = wingLine2.unwrap();

        // Java: if ((float)shaftLine.getIntersectPointSize(wingLine2) / (float)Math.min(wingLine2.getPointSize(), shaftLine.getPointSize()) > 0.2F)
        if shaft.getIntersectPointSize(&wl2) as f32 / (wl2.getPointSize().min(shaft.getPointSize())) as f32 > MAX_POINT_INTERSECT_RATIO {
            // Java: return false;
            return false;
        }
        // Java: else

        let tail = self.tailPoint.as_ref().unwrap();
        let head = self.headPoint.as_ref().unwrap();
        let w1 = self.wingPoint1.as_ref().unwrap();
        let w2 = self.wingPoint2.as_ref().unwrap();

        // Java: shaftLine = Line.getLine(this.tailPoint, this.headPoint);
        let shaft_line2 = match Line::getLine(tail, head) { Some(l) => l, None => return false };
        // Java: wingLine1 = Line.getLine(this.wingPoint1, this.headPoint);
        let wing_line2_1 = match Line::getLine(w1, head) { Some(l) => l, None => return false };
        // Java: wingLine2 = Line.getLine(this.wingPoint2, this.headPoint);
        let wing_line2_2 = match Line::getLine(w2, head) { Some(l) => l, None => return false };
        // Java: double shaftAngle = Math.atan((double)shaftLine.getSlope());
        let shaftAngle = (shaft_line2.getSlope() as f64).atan();
        // Java: double wingAngle1 = Math.atan((double)wingLine1.getSlope());
        let wingAngle1 = (wing_line2_1.getSlope() as f64).atan();
        // Java: double wingAngle2 = Math.atan((double)wingLine2.getSlope());
        let wingAngle2 = (wing_line2_2.getSlope() as f64).atan();
        // Java: double hwAngle1 = Math.abs(wingAngle1 - shaftAngle);
        let mut hwAngle1 = (wingAngle1 - shaftAngle).abs();
        // Java: double hwAngle2 = Math.abs(wingAngle2 - shaftAngle);
        let mut hwAngle2 = (wingAngle2 - shaftAngle).abs();
        // Java: double headAngle = Math.abs(wingAngle1 - wingAngle2);
        let headAngle_raw = (wingAngle1 - wingAngle2).abs();

        // Java: float hwProduct1 = (this.wingPoint1.x - this.headPoint.x) * (this.tailPoint.x - this.headPoint.x) + (this.wingPoint1.y - this.headPoint.y) * (this.tailPoint.y - this.headPoint.y);
        let hwProduct1 = (w1.x - head.x) * (tail.x - head.x) + (w1.y - head.y) * (tail.y - head.y);
        // Java: float hwProduct2 = (this.wingPoint2.x - this.headPoint.x) * (this.tailPoint.x - this.headPoint.x) + (this.wingPoint2.y - this.headPoint.y) * (this.tailPoint.y - this.headPoint.y);
        let hwProduct2 = (w2.x - head.x) * (tail.x - head.x) + (w2.y - head.y) * (tail.y - head.y);
        // Java: float headProduct = (this.wingPoint1.x - this.headPoint.x) * (this.wingPoint2.x - this.headPoint.x) + (this.wingPoint1.y - this.headPoint.y) * (this.wingPoint2.y - this.headPoint.y);
        let headProduct = (w1.x - head.x) * (w2.x - head.x) + (w1.y - head.y) * (w2.y - head.y);

        // Java: hwAngle1 = hwProduct1 > 0.0F ? Math.min(hwAngle1, Math.PI - hwAngle1) : Math.max(hwAngle1, Math.PI - hwAngle2);
        hwAngle1 = if hwProduct1 > 0.0 { hwAngle1.min(std::f64::consts::PI - hwAngle1) } else { hwAngle1.max(std::f64::consts::PI - hwAngle2) };
        // Java: hwAngle2 = hwProduct2 > 0.0F ? Math.min(hwAngle2, Math.PI - hwAngle2) : Math.max(hwAngle2, Math.PI - hwAngle2);
        hwAngle2 = if hwProduct2 > 0.0 { hwAngle2.min(std::f64::consts::PI - hwAngle2) } else { hwAngle2.max(std::f64::consts::PI - hwAngle2) };
        // Java: headAngle = headProduct > 0.0F ? Math.min(headAngle, Math.PI - headAngle) : Math.max(headAngle, Math.PI - headAngle);
        let headAngle_val = if headProduct > 0.0 { headAngle_raw.min(std::f64::consts::PI - headAngle_raw) } else { headAngle_raw.max(std::f64::consts::PI - headAngle_raw) };

        // Java: if (!(hwAngle1 > 1.4726215563702154) && !(hwAngle2 > 1.4726215563702154))
        if hwAngle1 > MAX_HEAD_ANGLE || hwAngle2 > MAX_HEAD_ANGLE {
            // Java: return false;
            return false;
        }
        // Java: if (!(hwAngle1 < (Math.PI / 180D)) && !(hwAngle2 < (Math.PI / 180D)))
        if hwAngle1 < MIN_HEAD_ANGLE || hwAngle2 < MIN_HEAD_ANGLE {
            // Java: return false;
            return false;
        }
        // Java: return !(Math.abs(headAngle - hwAngle1 - hwAngle2) > (Math.PI / 180D));
        !((headAngle_val - hwAngle1 - hwAngle2).abs() > MAX_HEAD_WING_ANGLE_DIFF)
        // Java: else
        // Java: return false;
        // Java: else
        // Java: return false;
        // Java: else
        // Java: return false;
    }
}

impl GestureShapeTrait for ShapeArrow {
    fn base(&self) -> &GestureShapeBase { &self.bulge.base }
    fn base_mut(&mut self) -> &mut GestureShapeBase { &mut self.bulge.base }
    fn isMatch(&mut self, points: &[Point]) -> bool { self.isMatch_impl(points) }
    fn initType(&mut self) { self.initType_impl(); }
}
