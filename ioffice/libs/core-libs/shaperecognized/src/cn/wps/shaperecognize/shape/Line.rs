// Java: import cn.wps.shaperecognize.ShapeRecognizeFactory
use crate::cn::wps::shaperecognize::ShapeRecognizeFactory;
use super::Point::Point;

// Java field: float MAX_DVTION
pub const MAX_DVTION: f32 = 65536.0;
// Java field: float MAX_DEVIATION_RATIO
pub const MAX_DEVIATION_RATIO: f32 = 10.0;
// Java field: float MAX_K_DVTION
pub const MAX_K_DVTION: f32 = f32::MAX;

// Java: public class Line
#[derive(Clone)]
pub struct Line {
    // Java field: float a
    pub(crate) a: f32,
    // Java field: float b
    pub(crate) b: f32,
    // Java field: float c
    pub(crate) c: f32,
    // Java field: int startInx
    startInx: i32,
    // Java field: int endInx
    endInx: i32,
}

// Java: public class Line
impl Line {
    // Java: static Line getAbsoluteFitLine(List<Point> points)
    pub fn getAbsoluteFitLine_default(points: &[Point]) -> Option<Line> {
        // Java: return getAbsoluteFitLine(points, 65536.0F);
        Line::getAbsoluteFitLine_with_dvtion(points, MAX_DVTION)
    }

    // Java: static Line getAbsoluteFitLine(List<Point> points, float fitAvgDvtion)
    pub fn getAbsoluteFitLine_with_dvtion(points: &[Point], fitAvgDvtion: f32) -> Option<Line> {
        // Java: return getAbsoluteFitLine(points, 0, points.size(), fitAvgDvtion);
        Line::getAbsoluteFitLine(points, 0, points.len() as i32, fitAvgDvtion)
    }

    // Java: static Line getAbsoluteFitLine(List<Point> points, int start, int end, float fitAvgDvtion)
    pub fn getAbsoluteFitLine(points: &[Point], start: i32, end: i32, fitAvgDvtion: f32) -> Option<Line> {
        // Java: if (points != null && points.size() != 0)
        if points.is_empty() {
            // Java: return null;
            return None;
        }
        // Java: int size = points.size();
        let size = points.len() as i32;
        // Java: float maxDeviation = fitAvgDvtion * 10.0F;
        let maxDeviation = fitAvgDvtion * MAX_DEVIATION_RATIO;
        let mut end = end;
        // Java: if (start > end)
        if start > end {
            // Java: end += size;
            end += size;
        }

        // Java: float sumxx = 0.0F;
        let mut sumxx: f32 = 0.0;
        // Java: float sumx = 0.0F;
        let mut sumx: f32 = 0.0;
        // Java: float sumxy = 0.0F;
        let mut sumxy: f32 = 0.0;
        // Java: float sumy = 0.0F;
        let mut sumy: f32 = 0.0;

        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: int inx = i >= size ? i - size : i;
            let inx = if i >= size { i - size } else { i };
            // Java: Point point = (Point)points.get(inx);
            let point = &points[inx as usize];
            // Java: sumxx += point.x * point.x;
            sumxx += point.x * point.x;
            // Java: sumx += point.x;
            sumx += point.x;
            // Java: sumxy += point.x * point.y;
            sumxy += point.x * point.y;
            // Java: sumy += point.y;
            sumy += point.y;
        }

        let n = (end - start) as f32;
        // Java: float r = sumxx * (float)(end - start) - sumx * sumx;
        let r = sumxx * n - sumx * sumx;
        // Java: float k = (sumxy * (float)(end - start) - sumx * sumy) / r;
        let k = (sumxy * n - sumx * sumy) / r;
        // Java: float l = (sumxx * sumy - sumxy * sumx) / r;
        let l = (sumxx * sumy - sumxy * sumx) / r;
        // Java: float sumDy = 0.0F;
        let mut sumDy: f32 = 0.0;
        // Java: float maxDy = 0.0F;
        let mut maxDy: f32 = 0.0;
        let dens = unsafe { ShapeRecognizeFactory::density };
        // Java: if (k != Float.NEGATIVE_INFINITY && k != Float.POSITIVE_INFINITY)
        if k != f32::NEG_INFINITY && k != f32::INFINITY {
            // Java: for(int i = start; i < end; ++i)
            for i in start..end {
                // Java: int inx = i >= size ? i - size : i;
                let inx = if i >= size { i - size } else { i };
                // Java: Point point = (Point)points.get(inx);
                let point = &points[inx as usize];
                // Java: float dy = Math.abs(point.y - (k * point.x + l));
                let dy = (point.y - (k * point.x + l)).abs();
                // Java: maxDy = Math.max(dy, maxDy);
                maxDy = dy.max(maxDy);
                // Java: sumDy += dy;
                sumDy += dy;
            }

            // Java: if (sumDy / (float)(end - start) / ShapeRecognizeFactory.density < fitAvgDvtion && maxDy / ShapeRecognizeFactory.density < maxDeviation)
            if sumDy / n / dens < fitAvgDvtion && maxDy / dens < maxDeviation {
                // Java: return new Line(k, -1.0F, l, start, end - 1);
                return Some(Line::new(k, -1.0, l, start, end - 1));
            }
        }

        // Java: float sumyy = 0.0F;
        let mut sumyy: f32 = 0.0;
        // Java: sumx = 0.0F;
        sumx = 0.0;
        // Java: sumxy = 0.0F;
        sumxy = 0.0;
        // Java: sumy = 0.0F;
        sumy = 0.0;

        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: int inx = i >= size ? i - size : i;
            let inx = if i >= size { i - size } else { i };
            // Java: Point p = (Point)points.get(inx);
            let p = &points[inx as usize];
            // Java: sumyy += p.y * p.y;
            sumyy += p.y * p.y;
            // Java: sumy += p.y;
            sumy += p.y;
            // Java: sumxy += p.x * p.y;
            sumxy += p.x * p.y;
            // Java: sumx += p.x;
            sumx += p.x;
        }

        // Java: r = sumyy * (float)(end - start) - sumy * sumy;
        let r = sumyy * n - sumy * sumy;
        // Java: k = (sumxy * (float)(end - start) - sumx * sumy) / r;
        let k = (sumxy * n - sumx * sumy) / r;
        // Java: l = (sumyy * sumx - sumxy * sumy) / r;
        let l = (sumyy * sumx - sumxy * sumy) / r;
        // Java: float sumDx = 0.0F;
        let mut sumDx: f32 = 0.0;
        // Java: float maxDx = 0.0F;
        let mut maxDx: f32 = 0.0;

        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: int inx = i >= size ? i - size : i;
            let inx = if i >= size { i - size } else { i };
            // Java: Point point = (Point)points.get(inx);
            let point = &points[inx as usize];
            // Java: float dx = Math.abs(point.x - (k * point.y + l));
            let dx = (point.x - (k * point.y + l)).abs();
            // Java: maxDx = Math.max(dx, maxDx);
            maxDx = dx.max(maxDx);
            // Java: sumDx += dx;
            sumDx += dx;
        }

        // Java: if (sumDx / (float)(end - start) / ShapeRecognizeFactory.density < fitAvgDvtion && maxDx / ShapeRecognizeFactory.density < maxDeviation)
        if sumDx / n / dens < fitAvgDvtion && maxDx / dens < maxDeviation {
            // Java: return new Line(1.0F, -k, -l, start, end - 1);
            Some(Line::new(1.0, -k, -l, start, end - 1))
        // Java: else
        } else {
            // Java: return null;
            None
        }
        // Java: else
        // Java: return null;
    }

    // Java: static Line getRelativeFitLine(List<Point> points, float fitAvgDvtion)
    pub fn getRelativeFitLine_with_dvtion(points: &[Point], fitAvgDvtion: f32) -> Option<Line> {
        // Java: return getRelativeFitLine(points, 0, points.size(), fitAvgDvtion);
        Line::getRelativeFitLine(points, 0, points.len() as i32, fitAvgDvtion)
    }

    // Java: static Line getRelativeFitLine(List<Point> points, int start, int end, float fitAvgDvtion)
    pub fn getRelativeFitLine(points: &[Point], start: i32, end: i32, fitAvgDvtion: f32) -> Option<Line> {
        // Java: if (points != null && points.size() != 0)
        if points.is_empty() {
            // Java: return null;
            return None;
        }
        // Java: int size = points.size();
        let size = points.len() as i32;
        let mut end = end;
        // Java: if (start > end)
        if start > end {
            // Java: end += size;
            end += size;
        }
        // Java: if (end - start < 2)
        if end - start < 2 {
            // Java: return null;
            return None;
        }
        // Java: else

        // Java: float sumxx = 0.0F;
        let mut sumxx: f32 = 0.0;
        // Java: float sumx = 0.0F;
        let mut sumx: f32 = 0.0;
        // Java: float sumxy = 0.0F;
        let mut sumxy: f32 = 0.0;
        // Java: float sumy = 0.0F;
        let mut sumy: f32 = 0.0;

        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: int inx = i >= size ? i - size : i;
            let inx = if i >= size { i - size } else { i };
            // Java: Point point = (Point)points.get(inx);
            let point = &points[inx as usize];
            // Java: sumxx += point.x * point.x;
            sumxx += point.x * point.x;
            // Java: sumx += point.x;
            sumx += point.x;
            // Java: sumxy += point.x * point.y;
            sumxy += point.x * point.y;
            // Java: sumy += point.y;
            sumy += point.y;
        }

        let n = (end - start) as f32;
        // Java: float r = sumxx * (float)(end - start) - sumx * sumx;
        let r = sumxx * n - sumx * sumx;
        // Java: float k = (sumxy * (float)(end - start) - sumx * sumy) / r;
        let k = (sumxy * n - sumx * sumy) / r;
        // Java: float l = (sumxx * sumy - sumxy * sumx) / r;
        let l = (sumxx * sumy - sumxy * sumx) / r;
        // Java: float dy = 0.0F;
        let mut dy: f32 = 0.0;
        // Java: float lineLen = Point.getDst((Point)points.get(start), (Point)points.get(end - 1 < size ? end - 1 : end - 1 - size));
        let end_inx = if end - 1 < size { end - 1 } else { end - 1 - size };
        let lineLen = Point::getDst(&points[start as usize], &points[end_inx as usize]);
        // Java: if (lineLen == 0.0F)
        if lineLen == 0.0 {
            // Java: return null;
            return None;
        }
        // Java: else

        // Java: if (k != Float.NEGATIVE_INFINITY && k != Float.POSITIVE_INFINITY)
        if k != f32::NEG_INFINITY && k != f32::INFINITY {
            // Java: for(int i = start; i < end; ++i)
            for i in start..end {
                // Java: int inx = i >= size ? i - size : i;
                let inx = if i >= size { i - size } else { i };
                // Java: Point point = (Point)points.get(inx);
                let point = &points[inx as usize];
                // Java: dy += Math.abs(point.y - (k * point.x + l));
                dy += (point.y - (k * point.x + l)).abs();
            }

            // Java: if (dy / lineLen / (float)(end - start) < fitAvgDvtion)
            if dy / lineLen / n < fitAvgDvtion {
                // Java: return new Line(k, -1.0F, l, start, end - 1);
                return Some(Line::new(k, -1.0, l, start, end - 1));
            }
        }

        // Java: float sumyy = 0.0F;
        let mut sumyy: f32 = 0.0;
        // Java: sumx = 0.0F;
        sumx = 0.0;
        // Java: sumxy = 0.0F;
        sumxy = 0.0;
        // Java: sumy = 0.0F;
        sumy = 0.0;

        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: int inx = i >= size ? i - size : i;
            let inx = if i >= size { i - size } else { i };
            // Java: Point p = (Point)points.get(inx);
            let p = &points[inx as usize];
            // Java: sumyy += p.y * p.y;
            sumyy += p.y * p.y;
            // Java: sumy += p.y;
            sumy += p.y;
            // Java: sumxy += p.x * p.y;
            sumxy += p.x * p.y;
            // Java: sumx += p.x;
            sumx += p.x;
        }

        // Java: r = sumyy * (float)(end - start) - sumy * sumy;
        let r = sumyy * n - sumy * sumy;
        // Java: k = (sumxy * (float)(end - start) - sumx * sumy) / r;
        let k = (sumxy * n - sumx * sumy) / r;
        // Java: l = (sumyy * sumx - sumxy * sumy) / r;
        let l = (sumyy * sumx - sumxy * sumy) / r;
        // Java: float dx = 0.0F;
        let mut dx: f32 = 0.0;

        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: int inx = i >= size ? i - size : i;
            let inx = if i >= size { i - size } else { i };
            // Java: Point point = (Point)points.get(inx);
            let point = &points[inx as usize];
            // Java: dx += Math.abs(point.x - (k * point.y + l));
            dx += (point.x - (k * point.y + l)).abs();
        }

        // Java: if (dx / lineLen / (float)(end - start) < fitAvgDvtion)
        if dx / lineLen / n < fitAvgDvtion {
            // Java: return new Line(1.0F, -k, -l, start, end - 1);
            Some(Line::new(1.0, -k, -l, start, end - 1))
        // Java: else
        } else {
            // Java: return null;
            None
        }
        // Java: else
        // Java: return null;
    }

    // Java: static Point getIntersection(Line line1, Line line2)
    pub fn getIntersection(line1: &Line, line2: &Line) -> Option<Point> {
        // Java: if (line1 != null && line2 != null)
        // Java: float D = line1.a * line2.b - line2.a * line1.b;
        let d = line1.a * line2.b - line2.a * line1.b;
        // Java: if (D == 0.0F)
        if d == 0.0 {
            // Java: return null;
            return None;
        }
        // Java: else
        // Java: float D1 = line1.b * line2.c - line2.b * line1.c;
        let d1 = line1.b * line2.c - line2.b * line1.c;
        // Java: float D2 = line2.a * line1.c - line1.a * line2.c;
        let d2 = line2.a * line1.c - line1.a * line2.c;
        // Java: return new Point(D1 / D, D2 / D);
        Some(Point::new_with_xy(d1 / d, d2 / d))
        // Java: else
        // Java: return null;
    }

    // Java: static Line getLine(Point start, Point end)
    pub fn getLine(start: &Point, end: &Point) -> Option<Line> {
        // Java: if (null != start && null != end && !Point.equals(start, end))
        if Point::equals(Some(start), Some(end)) {
            // Java: return null;
            return None;
        }
        // Java: if (Math.abs(start.x - end.x) > Math.abs(start.y - end.y))
        if (start.x - end.x).abs() > (start.y - end.y).abs() {
            // Java: float k = (start.y - end.y) / (start.x - end.x);
            let k = (start.y - end.y) / (start.x - end.x);
            // Java: float l = start.y - k * start.x;
            let l = start.y - k * start.x;
            // Java: return new Line(k, -1.0F, l);
            Some(Line::new_abc(k, -1.0, l))
        // Java: else
        } else {
            // Java: float k = (start.x - end.x) / (start.y - end.y);
            let k = (start.x - end.x) / (start.y - end.y);
            // Java: float l = start.x - k * start.y;
            let l = start.x - k * start.y;
            // Java: return new Line(1.0F, -k, -l);
            Some(Line::new_abc(1.0, -k, -l))
        }
        // Java: else
        // Java: return null;
    }

    // Java: Line(float a, float b, float c, int startInx, int endInx)
    pub fn new(a: f32, b: f32, c: f32, startInx: i32, endInx: i32) -> Self {
        // Java: super();
        // Java: this.a = a;
        // Java: this.b = b;
        // Java: this.c = c;
        // Java: this.startInx = startInx;
        // Java: this.endInx = endInx;
        Line { a, b, c, startInx, endInx }
    }

    // Java: Line(float a, float b, float c)
    pub fn new_abc(a: f32, b: f32, c: f32) -> Self {
        // Java: super();
        // Java: this.a = a;
        // Java: this.b = b;
        // Java: this.c = c;
        // Java: this.startInx = 0;
        // Java: this.endInx = 0;
        Line { a, b, c, startInx: 0, endInx: 0 }
    }

    // Java: void setStartInx(int inx)
    pub fn setStartInx(&mut self, inx: i32) {
        // Java: this.startInx = inx;
        self.startInx = inx;
    }

    // Java: int getStartInx()
    pub fn getStartInx(&self) -> i32 {
        // Java: return this.startInx;
        self.startInx
    }

    // Java: void setEndInx(int inx)
    pub fn setEndInx(&mut self, inx: i32) {
        // Java: this.endInx = inx;
        self.endInx = inx;
    }

    // Java: int getEndInx()
    pub fn getEndInx(&self) -> i32 {
        // Java: return this.endInx;
        self.endInx
    }

    // Java: int getPointSize()
    pub fn getPointSize(&self) -> i32 {
        // Java: return this.endInx - this.startInx + 1;
        self.endInx - self.startInx + 1
    }

    // Java: int getIntersectPointSize(Line line)
    pub fn getIntersectPointSize(&self, line: &Line) -> i32 {
        // Java: int count = Math.min(line.endInx, this.endInx) - Math.max(line.startInx, this.startInx) + 1;
        let count = line.endInx.min(self.endInx) - line.startInx.max(self.startInx) + 1;
        // Java: return Math.max(0, count);
        count.max(0)
    }

    // Java: float getSlope()
    pub fn getSlope(&self) -> f32 {
        // Java: return -this.a / this.b;
        -self.a / self.b
    }

    // Java: static float getVDstPToL(Line line, Point p)
    pub fn getVDstPToL(line: &Line, p: &Point) -> f32 {
        // Java: float vk = -1.0F / line.getSlope();
        let vk = -1.0 / line.getSlope();
        // Java: Line vLine;
        let vLine;
        // Java: if (vk != Float.NEGATIVE_INFINITY && vk != Float.POSITIVE_INFINITY)
        if vk != f32::NEG_INFINITY && vk != f32::INFINITY {
            // Java: float vl = p.y - vk * p.x;
            let vl = p.y - vk * p.x;
            // Java: vLine = new Line(vk, -1.0F, vl);
            vLine = Line::new_abc(vk, -1.0, vl);
        // Java: else
        } else {
            // Java: vLine = new Line(1.0F, 0.0F, -p.x);
            vLine = Line::new_abc(1.0, 0.0, -p.x);
        }

        // Java: Point fixedPoint = getIntersection(vLine, line);
        let fixedPoint = Line::getIntersection(&vLine, line).unwrap();
        // Java: return (float)Math.sqrt(Math.pow((double)(fixedPoint.x - p.x), (double)2.0F) + Math.pow((double)(fixedPoint.y - p.y), (double)2.0F));
        (((fixedPoint.x - p.x) as f64).powi(2) + ((fixedPoint.y - p.y) as f64).powi(2)).sqrt() as f32
    }

    // Java: static Point getFixPoint(Line line, Point p)
    pub fn getFixPoint(line: &Line, p: &Point) -> Option<Point> {
        // Java: float vk = -1.0F / line.getSlope();
        let vk = -1.0 / line.getSlope();
        // Java: Line vLine;
        let vLine;
        // Java: if (vk != Float.POSITIVE_INFINITY && vk != Float.NEGATIVE_INFINITY)
        if vk != f32::INFINITY && vk != f32::NEG_INFINITY {
            // Java: float vl = p.y - vk * p.x;
            let vl = p.y - vk * p.x;
            // Java: vLine = new Line(vk, -1.0F, vl);
            vLine = Line::new_abc(vk, -1.0, vl);
        // Java: else
        } else {
            // Java: vLine = new Line(1.0F, 0.0F, -p.x);
            vLine = Line::new_abc(1.0, 0.0, -p.x);
        }

        // Java: return getIntersection(vLine, line);
        Line::getIntersection(&vLine, line)
    }
}
