// Java: import android.graphics.Rect
// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
use super::GestureShape::{GestureShapeBase, GestureShapeTrait, Rect};
use super::Line::Line;
use super::Point::Point;

// Java: public class ShapeSwap extends GestureShape
pub struct ShapeSwap {
    pub base: GestureShapeBase,
    // Java field: Point startPoint
    startPoint: Option<Point>,
    // Java field: Point endPoint
    endPoint: Option<Point>,
    // Java field: Point leftMaxPoint
    leftMaxPoint: Option<Point>,
    // Java field: Point rightMaxPoint
    rightMaxPoint: Option<Point>,
    // Java field: Point midFixPoint
    midFixPoint: Option<Point>,
}

// Java: public class ShapeSwap extends GestureShape
impl ShapeSwap {
    // Java: ShapeSwap(List<Point> points, ShapeConfig config)
    pub fn new(points: &[Point], config: &ShapeConfig) -> Self {
        // Java: super(points, config);
        let mut base = GestureShapeBase::new(config.clone());
        base.init_with_points(points);
        let mut shape = ShapeSwap {
            base,
            startPoint: None,
            endPoint: None,
            leftMaxPoint: None,
            rightMaxPoint: None,
            midFixPoint: None,
        };
        if !shape.base.points.is_empty() {
            let pts: Vec<Point> = shape.base.points.clone();
            if shape.isMatch(&pts) {
                shape.initType();
            }
        }
        shape
    }

    // Java: Rect getSwap1()
    pub fn getSwap1(&self) -> Rect {
        let sp = self.startPoint.as_ref().unwrap();
        let lp = self.leftMaxPoint.as_ref().unwrap();
        let rp = self.rightMaxPoint.as_ref().unwrap();
        let ep = self.endPoint.as_ref().unwrap();
        let mf = self.midFixPoint.as_ref().unwrap();
        // Java: int left = (int)this.startPoint.x;
        let left = sp.x as i32;
        // Java: int top = (int)(this.startPoint.y < this.leftMaxPoint.y ? this.startPoint.y : this.leftMaxPoint.y);
        let top = if sp.y < lp.y { sp.y } else { lp.y } as i32;
        // Java: int right = (int)this.midFixPoint.x;
        let right = mf.x as i32;
        // Java: int bottom = (int)(this.rightMaxPoint.y > this.endPoint.y ? this.rightMaxPoint.y : this.endPoint.y);
        let bottom = if rp.y > ep.y { rp.y } else { ep.y } as i32;
        // Java: return new Rect(left, top, right, bottom);
        Rect::new(left, top, right, bottom)
    }

    // Java: Rect getSwap2()
    pub fn getSwap2(&self) -> Rect {
        let sp = self.startPoint.as_ref().unwrap();
        let lp = self.leftMaxPoint.as_ref().unwrap();
        let rp = self.rightMaxPoint.as_ref().unwrap();
        let ep = self.endPoint.as_ref().unwrap();
        let mf = self.midFixPoint.as_ref().unwrap();
        // Java: int left = (int)this.midFixPoint.x;
        let left = mf.x as i32;
        // Java: int top = (int)(this.startPoint.y < this.leftMaxPoint.y ? this.startPoint.y : this.leftMaxPoint.y);
        let top = if sp.y < lp.y { sp.y } else { lp.y } as i32;
        // Java: int right = (int)this.endPoint.x;
        let right = ep.x as i32;
        // Java: int bottom = (int)(this.rightMaxPoint.y > this.endPoint.y ? this.rightMaxPoint.y : this.endPoint.y);
        let bottom = if rp.y > ep.y { rp.y } else { ep.y } as i32;
        // Java: return new Rect(left, top, right, bottom);
        Rect::new(left, top, right, bottom)
    }
}

impl GestureShapeTrait for ShapeSwap {
    fn base(&self) -> &GestureShapeBase { &self.base }
    fn base_mut(&mut self) -> &mut GestureShapeBase { &mut self.base }

    // Java: boolean isMatch(List<Point> points)
    fn isMatch(&mut self, points: &[Point]) -> bool {
        // Java: if (points != null && points.size() != 0)
        if points.is_empty() {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: this.startPoint = (Point)points.get(0);
        self.startPoint = Some(points[0].clone());
        // Java: this.endPoint = (Point)points.get(points.size() - 1);
        self.endPoint = Some(points[points.len() - 1].clone());

        // Java: if (this.endPoint.y < this.startPoint.y)
        if self.endPoint.as_ref().unwrap().y < self.startPoint.as_ref().unwrap().y {
            // Java: return false;
            return false;
        }
        // Java: else

        let sp = self.startPoint.as_ref().unwrap();
        let ep = self.endPoint.as_ref().unwrap();
        // Java: for(Point p : points)
        for p in points {
            // Java: if (p.x < this.startPoint.x)
            if p.x < sp.x {
                // Java: return false;
                return false;
            }
            // Java: if (p.x > this.endPoint.x)
            if p.x > ep.x {
                // Java: return false;
                return false;
            }
        }

        // Java: Line drLine = Line.getLine((Point)points.get(0), (Point)points.get(points.size() - 1));
        let drLine = match Line::getLine(&points[0], &points[points.len() - 1]) {
            Some(l) => l,
            None => return false,
        };

        // Java: int leftMaxPointIndex = -1;
        let mut leftMaxPointIndex: i32 = -1;
        // Java: int rightMaxPointIndex = -1;
        let mut rightMaxPointIndex: i32 = -1;
        // Java: float dt = -65536.0F;
        let mut dt: f32 = -65536.0;

        // Java: for(int i = 0; i < points.size(); ++i)
        for i in 0..points.len() {
            // Java: float curDt = Line.getVDstPToL(drLine, (Point)points.get(i));
            let curDt = Line::getVDstPToL(&drLine, &points[i]);
            // Java: if (!(dt <= curDt))
            if dt > curDt {
                // Java: leftMaxPointIndex = i;
                leftMaxPointIndex = i as i32;
                // Java: this.leftMaxPoint = (Point)points.get(i);
                self.leftMaxPoint = Some(points[i].clone());
                // Java: break;
                break;
            }
            // Java: dt = curDt;
            dt = curDt;
        }

        // Java: dt = -65536.0F;
        dt = -65536.0;

        // Java: for(int i = points.size() - 1; i >= 0; --i)
        for i in (0..points.len()).rev() {
            // Java: float curDt = Line.getVDstPToL(drLine, (Point)points.get(i));
            let curDt = Line::getVDstPToL(&drLine, &points[i]);
            // Java: if (!(dt <= curDt))
            if dt > curDt {
                // Java: rightMaxPointIndex = i;
                rightMaxPointIndex = i as i32;
                // Java: this.rightMaxPoint = (Point)points.get(i);
                self.rightMaxPoint = Some(points[i].clone());
                // Java: break;
                break;
            }
            // Java: dt = curDt;
            dt = curDt;
        }

        // Java: Line midLine = Line.getAbsoluteFitLine(points, leftMaxPointIndex, rightMaxPointIndex, 20.0F);
        let midLine = Line::getAbsoluteFitLine(points, leftMaxPointIndex, rightMaxPointIndex, 20.0);
        // Java: if (midLine != null)
        if let Some(midLine) = midLine {
            // Java: float mk = midLine.getSlope();
            let mk = midLine.getSlope();
            // Java: if (mk > 0.0F && mk < 2.0F || mk < 0.0F && mk > -2.0F)
            if (mk > 0.0 && mk < 2.0) || (mk < 0.0 && mk > -2.0) {
                // Java: return false;
                return false;
            }

            // Java: Point fixedPoint = Line.getIntersection(midLine, drLine);
            if let Some(fixedPoint) = Line::getIntersection(&midLine, &drLine) {
                let sp = self.startPoint.as_ref().unwrap();
                let ep = self.endPoint.as_ref().unwrap();
                let lp = self.leftMaxPoint.as_ref().unwrap();
                let rp = self.rightMaxPoint.as_ref().unwrap();

                // Java: if (fixedPoint.x > this.startPoint.x && fixedPoint.x < this.endPoint.x && fixedPoint.y > this.leftMaxPoint.y && fixedPoint.y < this.rightMaxPoint.y && this.rightMaxPoint.y > this.leftMaxPoint.y)
                if fixedPoint.x > sp.x && fixedPoint.x < ep.x
                    && fixedPoint.y > lp.y && fixedPoint.y < rp.y
                    && rp.y > lp.y
                {
                    // Java: this.midFixPoint = fixedPoint;
                    self.midFixPoint = Some(fixedPoint);
                    // Java: Line leftLine = Line.getAbsoluteFitLine(points, 0, leftMaxPointIndex, 20.0F);
                    let leftLine = Line::getAbsoluteFitLine(points, 0, leftMaxPointIndex, 20.0);
                    // Java: if (leftLine == null)
                    if leftLine.is_none() {
                        // Java: return false;
                        return false;
                    }

                    // Java: Line rightLine = Line.getAbsoluteFitLine(points, rightMaxPointIndex, points.size(), 20.0F);
                    let rightLine = Line::getAbsoluteFitLine(points, rightMaxPointIndex, points.len() as i32, 20.0);
                    // Java: if (rightLine == null)
                    if rightLine.is_none() {
                        // Java: return false;
                        return false;
                    }

                    // Java: float lk = leftLine.getSlope();
                    let lk = leftLine.unwrap().getSlope();
                    // Java: float rk = rightLine.getSlope();
                    let rk = rightLine.unwrap().getSlope();

                    // Java: if (!(lk < -0.2F) && !(lk > 0.2F))
                    if lk < -0.2 || lk > 0.2 {
                        // Java: return false;
                        return false;
                    }
                    // Java: if (!(rk < -0.2F) && !(rk > 0.2F))
                    if rk < -0.2 || rk > 0.2 {
                        // Java: return false;
                        return false;
                    }

                    // Java: return true;
                    return true;
                }
            }
        }

        // Java: return false;
        // Java: return false;
        // Java: return false;
        false
        // Java: else
        // Java: return false;
    }

    // Java: void initType()
    fn initType(&mut self) {
        // Java: this.type = 128;
        self.base.type_ = 128;
    }
}

impl ShapeSwap {
    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.startPoint + "-" + this.leftMaxPoint + "-" + this.midFixPoint + "-" + this.rightMaxPoint + "-" + this.endPoint;
        format!("{}-{}-{}-{}-{}",
            self.startPoint.as_ref().map_or("None".to_string(), |p| p.toString()),
            self.leftMaxPoint.as_ref().map_or("None".to_string(), |p| p.toString()),
            self.midFixPoint.as_ref().map_or("None".to_string(), |p| p.toString()),
            self.rightMaxPoint.as_ref().map_or("None".to_string(), |p| p.toString()),
            self.endPoint.as_ref().map_or("None".to_string(), |p| p.toString()),
        )
    }
}
