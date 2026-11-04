// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
// Java: import cn.wps.shaperecognize.ShapeRecognizeFactory
use crate::cn::wps::shaperecognize::ShapeRecognizeFactory::StrokePosition;
// Java: import cn.wps.shaperecognize.algorithm.JarvisMarch
use crate::cn::wps::shaperecognize::algorithm::JarvisMarch::JarvisMarch;
use super::GestureShape::GestureShapeBase;
use super::Line::Line;
use super::Point::Point;

// Java: public class ShapeBulge extends GestureShape
pub struct ShapeBulgeBase {
    pub base: GestureShapeBase,
    // Java field: float df
    pub df: f32,
    // Java field: float DF_GIRTH_RATIO
    // Java field: List<Point> bulgeList
    pub bulgeList: Vec<Point>,
    // Java field: float maxDst
    pub maxDst: f32,
    // Java field: Point maxDstStartPt
    pub maxDstStartPt: Option<Point>,
    // Java field: Point maxDstEndPt
    pub maxDstEndPt: Option<Point>,
    // Java field: Point maxVerStartPt
    pub maxVerStartPt: Option<Point>,
    // Java field: Point maxVerEndPt
    pub maxVerEndPt: Option<Point>,
    // Java field: int maxDstStartInx
    pub maxDstStartInx: i32,
    // Java field: int maxDstEndInx
    pub maxDstEndInx: i32,
    // Java field: int maxVerStartInx
    pub maxVerStartInx: i32,
    // Java field: int maxVerEndInx
    pub maxVerEndInx: i32,
    // Java field: int[] bulgeIndices
    pub bulgeIndices: Vec<i32>,
    // Java field: Line maxDstLine
    pub maxDstLine: Option<Line>,
}

pub const DF_GIRTH_RATIO: f32 = 72.0;

// Java: public class ShapeBulge extends GestureShape
impl ShapeBulgeBase {
    // Java: ShapeBulge(List<Point> points, ShapeConfig config)
    pub fn new(config: ShapeConfig) -> Self {
        // Java: super(points, config);
        ShapeBulgeBase {
            base: GestureShapeBase::new(config),
            df: 10.0,
            bulgeList: Vec::new(),
            maxDst: 0.0,
            maxDstStartPt: None,
            maxDstEndPt: None,
            maxVerStartPt: None,
            maxVerEndPt: None,
            maxDstStartInx: 0,
            maxDstEndInx: 0,
            maxVerStartInx: 0,
            maxVerEndInx: 0,
            bulgeIndices: Vec::new(),
            maxDstLine: None,
        }
    }

    // Java: ShapeBulge(List<Point> points, List<ShapeRecognizeFactory.StrokePosition> strokePositions, ShapeConfig config)
    pub fn new_with_strokes(_config: ShapeConfig) -> Self {
        // Java: super(points, strokePositions, config);
        Self::new(_config)
    }

    // Java: void findBulges()
    pub fn findBulges(&mut self) {
        // Java: JarvisMarch jarvis = new JarvisMarch(this.points);
        let mut jarvis = JarvisMarch::new(&self.base.points);
        // Java: jarvis.calculateHull();
        jarvis.calculateHull();
        // Java: this.bulgeList = jarvis.getHullPointList();
        self.bulgeList = jarvis.getHullPointList().clone();
    }

    // Java: void findMaxDstBulges()
    pub fn findMaxDstBulges(&mut self) {
        // Java: this.maxDstStartPt = null;
        self.maxDstStartPt = None;
        // Java: this.maxDstEndPt = null;
        self.maxDstEndPt = None;
        // Java: this.maxDst = 0.0F;
        self.maxDst = 0.0;

        // Java: for(int i = 0; i < this.bulgeList.size() - 1; ++i)
        for i in 0..self.bulgeList.len().saturating_sub(1) {
            // Java: Point start = (Point)this.bulgeList.get(i);
            let start = &self.bulgeList[i];
            // Java: for(int j = i + 1; j < this.bulgeList.size(); ++j)
            for j in (i + 1)..self.bulgeList.len() {
                // Java: Point end = (Point)this.bulgeList.get(j);
                let end = &self.bulgeList[j];
                // Java: float dst = Point.getDst(start, end);
                let dst = Point::getDst(start, end);
                // Java: if (this.maxDst < dst)
                if self.maxDst < dst {
                    // Java: this.maxDst = dst;
                    self.maxDst = dst;
                    // Java: this.maxDstStartPt = start;
                    self.maxDstStartPt = Some(start.clone());
                    // Java: this.maxDstEndPt = end;
                    self.maxDstEndPt = Some(end.clone());
                }
            }
        }
    }

    // Java: void findMaxVerBulges()
    pub fn findMaxVerBulges(&mut self) {
        // Java: this.maxDstLine = Line.getLine(this.maxDstStartPt, this.maxDstEndPt);
        if let (Some(ref start), Some(ref end)) = (&self.maxDstStartPt, &self.maxDstEndPt) {
            self.maxDstLine = Line::getLine(start, end);
        }
        // Java: if (null != this.maxDstLine)
        let max_dst_line = match &self.maxDstLine {
            Some(l) => l.clone(),
            None => return,
        };

        // Java: float lmax = 0.0F;
        let mut lmax: f32 = 0.0;
        // Java: float rmax = 0.0F;
        let mut rmax: f32 = 0.0;
        // Java: float umax = 0.0F;
        let mut umax: f32 = 0.0;
        // Java: float dmax = 0.0F;
        let mut dmax: f32 = 0.0;
        // Java: int linx = -1;
        let mut linx: i32 = -1;
        // Java: int rinx = -1;
        let mut rinx: i32 = -1;
        // Java: int uinx = -1;
        let mut uinx: i32 = -1;
        // Java: int dinx = -1;
        let mut dinx: i32 = -1;

        // Java: for(int i = 0; i < this.bulgeList.size(); ++i)
        for i in 0..self.bulgeList.len() {
            // Java: Point curPoint = (Point)this.bulgeList.get(i);
            let curPoint = &self.bulgeList[i];
            let is_start = self.maxDstStartPt.as_ref().map_or(false, |p| Point::equals(Some(p), Some(curPoint)));
            let is_end = self.maxDstEndPt.as_ref().map_or(false, |p| Point::equals(Some(p), Some(curPoint)));
            // Java: if (curPoint != this.maxDstStartPt && curPoint != this.maxDstEndPt)
            if !is_start && !is_end {
                // Java: Point fixtPoint = Line.getFixPoint(this.maxDstLine, curPoint);
                if let Some(fixtPoint) = Line::getFixPoint(&max_dst_line, curPoint) {
                    // Java: float dst = fixtPoint.getSquareDst(curPoint);
                    let dst = fixtPoint.getSquareDst(curPoint);
                    // Java: if (fixtPoint.x > curPoint.x)
                    if fixtPoint.x > curPoint.x {
                        // Java: if (lmax < dst)
                        if lmax < dst {
                            // Java: lmax = dst;
                            lmax = dst;
                            // Java: linx = i;
                            linx = i as i32;
                        }
                    // Java: else if (fixtPoint.x < curPoint.x)
                    } else if fixtPoint.x < curPoint.x {
                        // Java: if (rmax < dst)
                        if rmax < dst {
                            // Java: rmax = dst;
                            rmax = dst;
                            // Java: rinx = i;
                            rinx = i as i32;
                        }
                    // Java: else if (fixtPoint.y > curPoint.y)
                    } else if fixtPoint.y > curPoint.y {
                        // Java: if (dmax > dst)
                        if dmax > dst {
                            // Java: dmax = dst;
                            dmax = dst;
                            // Java: dinx = i;
                            dinx = i as i32;
                        }
                    // Java: else if (fixtPoint.y < curPoint.y && umax > dst)
                    } else if fixtPoint.y < curPoint.y && umax > dst {
                        // Java: umax = dst;
                        umax = dst;
                        // Java: uinx = i;
                        uinx = i as i32;
                    }
                }
            }
        }

        // Java: if (umax + dmax > lmax + rmax)
        if umax + dmax > lmax + rmax {
            // Java: if (uinx >= 0)
            if uinx >= 0 {
                // Java: this.maxVerStartPt = (Point)this.bulgeList.get(uinx);
                self.maxVerStartPt = Some(self.bulgeList[uinx as usize].clone());
            }
            // Java: if (dinx >= 0)
            if dinx >= 0 {
                // Java: this.maxVerEndPt = (Point)this.bulgeList.get(dinx);
                self.maxVerEndPt = Some(self.bulgeList[dinx as usize].clone());
            }
        // Java: else
        } else {
            // Java: if (linx >= 0)
            if linx >= 0 {
                // Java: this.maxVerStartPt = (Point)this.bulgeList.get(linx);
                self.maxVerStartPt = Some(self.bulgeList[linx as usize].clone());
            }
            // Java: if (rinx >= 0)
            if rinx >= 0 {
                // Java: this.maxVerEndPt = (Point)this.bulgeList.get(rinx);
                self.maxVerEndPt = Some(self.bulgeList[rinx as usize].clone());
            }
        }
    }

    // Java: void initBulgeIndices()
    pub fn initBulgeIndices(&mut self) {
        // Java: this.maxDstStartInx = this.points.indexOf(this.maxDstStartPt);
        self.maxDstStartInx = self.maxDstStartPt.as_ref()
            .and_then(|p| self.base.points.iter().position(|pt| Point::equals(Some(pt), Some(p))))
            .map(|i| i as i32).unwrap_or(-1);
        // Java: this.maxDstEndInx = this.points.indexOf(this.maxDstEndPt);
        self.maxDstEndInx = self.maxDstEndPt.as_ref()
            .and_then(|p| self.base.points.iter().position(|pt| Point::equals(Some(pt), Some(p))))
            .map(|i| i as i32).unwrap_or(-1);
        // Java: this.maxVerStartInx = this.points.indexOf(this.maxVerStartPt);
        self.maxVerStartInx = self.maxVerStartPt.as_ref()
            .and_then(|p| self.base.points.iter().position(|pt| Point::equals(Some(pt), Some(p))))
            .map(|i| i as i32).unwrap_or(-1);
        // Java: this.maxVerEndInx = this.points.indexOf(this.maxVerEndPt);
        self.maxVerEndInx = self.maxVerEndPt.as_ref()
            .and_then(|p| self.base.points.iter().position(|pt| Point::equals(Some(pt), Some(p))))
            .map(|i| i as i32).unwrap_or(-1);

        // Java: List<Integer> bulgeIndicesList = new ArrayList();
        let mut bulgeIndicesList: Vec<i32> = Vec::new();
        // Java: if (this.maxDstStartInx != -1)
        if self.maxDstStartInx != -1 {
            // Java: bulgeIndicesList.add(this.maxDstStartInx);
            bulgeIndicesList.push(self.maxDstStartInx);
        }
        // Java: if (this.maxDstEndInx != -1)
        if self.maxDstEndInx != -1 {
            // Java: bulgeIndicesList.add(this.maxDstEndInx);
            bulgeIndicesList.push(self.maxDstEndInx);
        }
        // Java: if (this.maxVerStartInx != -1)
        if self.maxVerStartInx != -1 {
            // Java: bulgeIndicesList.add(this.maxVerStartInx);
            bulgeIndicesList.push(self.maxVerStartInx);
        }
        // Java: if (this.maxVerEndInx != -1)
        if self.maxVerEndInx != -1 {
            // Java: bulgeIndicesList.add(this.maxVerEndInx);
            bulgeIndicesList.push(self.maxVerEndInx);
        }

        // Java: this.bulgeIndices = new int[bulgeIndicesList.size()];
        // Java: for(int i = 0; i < bulgeIndicesList.size(); ++i)
        // Java: this.bulgeIndices[i] = (Integer)bulgeIndicesList.get(i);
        // Java: Arrays.sort(this.bulgeIndices);
        bulgeIndicesList.sort();
        self.bulgeIndices = bulgeIndicesList;
    }
}
