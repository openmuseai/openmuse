// Java: import cn.wps.shaperecognize.shape.GestureShape
use crate::cn::wps::shaperecognize::shape::GestureShape::{GestureShapeBase, GestureShapeTrait};
// Java: import cn.wps.shaperecognize.shape.Point
use crate::cn::wps::shaperecognize::shape::Point::Point;
// Java: import cn.wps.shaperecognize.shape.ShapeArrow
use crate::cn::wps::shaperecognize::shape::ShapeArrow;
// Java: import cn.wps.shaperecognize.shape.ShapeClose
use crate::cn::wps::shaperecognize::shape::ShapeClose;
// Java: import cn.wps.shaperecognize.shape.ShapeDeleteLine
use crate::cn::wps::shaperecognize::shape::ShapeDeleteLine;
// Java: import cn.wps.shaperecognize.shape.ShapeGeneralLine
use crate::cn::wps::shaperecognize::shape::ShapeGeneralLine;
// Java: import cn.wps.shaperecognize.shape.ShapeInsert
use crate::cn::wps::shaperecognize::shape::ShapeInsert;
// Java: import cn.wps.shaperecognize.shape.ShapeSwap
use crate::cn::wps::shaperecognize::shape::ShapeSwap;
use super::ShapeConfig::ShapeConfig;

// Java: public class StrokePosition
#[derive(Clone)]
pub struct StrokePosition {
    // Java field: int start
    pub start: i32,
    // Java field: int end
    pub end: i32,
}

impl StrokePosition {
    // Java: StrokePosition(int start, int end)
    pub fn new(start: i32, end: i32) -> Self {
        // Java: super();
        // Java: this.start = start;
        // Java: this.end = end;
        StrokePosition { start, end }
    }
}

// Java: public class ShapeRecognizeFactory
// Java field: ShapeRecognizeFactory instance
// Java field: float density
pub struct ShapeRecognizeFactory {
    // Java field: ShapeConfig config
    pub config: ShapeConfig,
    // Java field: List<List<Point>> strokes
    strokes: Vec<Vec<Point>>,
    // Java field: List<Point> comboPoints
    comboPoints: Vec<Point>,
    // Java field: List<StrokePosition> strokePositions
    strokePositions: Vec<StrokePosition>,
    // Java field: long startTime
    startTime: i64,
    // Java field: long waitTime
    waitTime: i64,
}

pub static mut density: f32 = 2.5;

fn current_time_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

impl ShapeRecognizeFactory {
    // Java: ShapeRecognizeFactory()
    fn new_private() -> Self {
        // Java: super();
        ShapeRecognizeFactory {
            config: ShapeConfig::new(),
            strokes: Vec::new(),
            comboPoints: Vec::new(),
            strokePositions: Vec::new(),
            startTime: 0,
            waitTime: 0,
        }
    }

    // Java: static ShapeRecognizeFactory getInstance(float density)
    pub fn getInstance_with_density(d: f32) -> Self {
        // Java: ShapeConfig config = new ShapeConfig();
        let mut config = ShapeConfig::new();
        // Java: config.density = density;
        config.density = d;
        // Java: ShapeRecognizeFactory.density = density;
        unsafe { density = d; }
        // Java: config.setSimpleShapes();
        config.setSimpleShapes();
        // Java: return getInstance(config);
        Self::getInstance(config.clone())
    }

    // Java: static ShapeRecognizeFactory getInstance(ShapeConfig config)
    pub fn getInstance(config: ShapeConfig) -> Self {
        // Java: if (instance == null)
        // Java: instance = new ShapeRecognizeFactory();
        let mut instance = ShapeRecognizeFactory::new_private();
        // Java: if (config == null)
        // Java: config = new ShapeConfig();
        // Java: config.density = density;
        // Java: config.setSimpleShapes();
        // Java: instance.config = config;
        instance.config = config;
        // Java: return instance;
        instance
    }

    // Java: GestureShape getResult(List<Point> points)
    pub fn getResult(&self, points: &[Point]) -> Option<Box<dyn GestureShapeTrait>> {
        // Java: GestureShape result = null;

        // Java: if (this.config.supportSwap() && (result == null || !result.isCompleted()))
        if self.config.supportSwap() {
            // Java: result = new ShapeSwap(points, this.config);
            let shape = ShapeSwap::ShapeSwap::new(points, &self.config);
            if shape.isCompleted() {
                return Some(Box::new(shape));
            }
        }

        // Java: if (this.config.supportLine() && (result == null || !result.isCompleted()))
        if self.config.supportLine() {
            // Java: result = new ShapeDeleteLine(points, this.config);
            let shape = ShapeDeleteLine::ShapeDeleteLine::new(points, &self.config);
            if shape.isCompleted() {
                return Some(Box::new(shape));
            }
        }

        // Java: if ((this.config.supportNoteCircle() || this.config.supportRect() || this.config.supportTrangle()) && (result == null || !result.isCompleted()))
        if self.config.supportNoteCircle() || self.config.supportRect() || self.config.supportTrangle() {
            // Java: result = new ShapeClose(points, this.config);
            let shape = ShapeClose::ShapeClose::new(points, &self.config);
            if shape.isCompleted() {
                return Some(Box::new(shape));
            }
        }

        // Java: if (this.config.supportInsert() && (result == null || !result.isCompleted()))
        if self.config.supportInsert() {
            // Java: result = new ShapeInsert(points, this.config);
            let shape = ShapeInsert::ShapeInsert::new(points, &self.config);
            if shape.isCompleted() {
                return Some(Box::new(shape));
            }
        }

        // Java: return result;
        None
    }

    // Java: GestureShape getStrokesResult(List<Point> stroke)
    pub fn getStrokesResult(&mut self, stroke: Option<Vec<Point>>) -> Option<Box<dyn GestureShapeTrait>> {
        // Java: GestureShape result = null;
        let mut result: Option<Box<dyn GestureShapeTrait>> = None;
        // Java: List<Point> var4 = null != stroke && !stroke.isEmpty() ? new ArrayList(stroke) : null;
        let var4 = stroke.filter(|s| !s.is_empty());

        // Java: if (this.strokes.size() == 0)
        if self.strokes.is_empty() {
            // Java: if (null != var4)
            if let Some(v) = var4 {
                // Java: this.strokes.add(var4);
                self.strokes.push(v);
            }
            // Java: return result;
            return result;
        // Java: else
        } else {
            // Java: ShapeConfig var10001 = this.config;
            // Java: boolean isSameShape = this.waitTime <= 400L;
            let isSameShape = self.waitTime <= 400;
            // Java: if (null != var4 && isSameShape)
            if let Some(ref v) = var4 {
                if isSameShape {
                    // Java: this.strokes.add(var4);
                    self.strokes.push(v.clone());
                }
            }

            // Java: label118:
            // Java: if (null != var4)
            let should_recognize = var4.is_none()
                || self.waitTime >= 400
                || (self.waitTime >= 250 && self.strokes.len() >= 2);

            // Java: var10001 = this.config;
            // Java: if (this.waitTime < 400L)
            // Java: var10001 = this.config;
            // Java: if (this.waitTime < 250L)
            // Java: break label118;
            // Java: int var10000 = this.strokes.size();
            // Java: var10001 = this.config;
            // Java: if (var10000 < 2)
            // Java: break label118;

            if should_recognize {
                // Java: this.comboStrokes();
                self.comboStrokes();

                // Java: if ((this.config.supportTrangle() || this.config.supportRect() || this.config.supportOvalCircle()) && (null == result || !result.isCompleted()))
                if self.config.supportTrangle() || self.config.supportRect() || self.config.supportOvalCircle() {
                    // Java: result = new ShapeClose(this.comboPoints, this.strokePositions, this.config);
                    let shape = ShapeClose::ShapeClose::new_with_strokes(
                        &self.comboPoints,
                        self.strokePositions.clone(),
                        &self.config,
                    );
                    if shape.isCompleted() {
                        result = Some(Box::new(shape));
                    }
                }

                // Java: if (this.config.supportArrow() && (null == result || !result.isCompleted()))
                if self.config.supportArrow() && result.as_ref().map_or(true, |r| !r.isCompleted()) {
                    // Java: result = new ShapeArrow(this.comboPoints, this.strokePositions, this.config);
                    let shape = ShapeArrow::ShapeArrow::new(
                        &self.comboPoints,
                        self.strokePositions.clone(),
                        &self.config,
                    );
                    if shape.isCompleted() {
                        result = Some(Box::new(shape));
                    }
                }

                // Java: if (this.config.supportLine() && (null == result || !result.isCompleted()))
                if self.config.supportLine() && result.as_ref().map_or(true, |r| !r.isCompleted()) {
                    // Java: result = new ShapeGeneralLine(this.comboPoints, this.config);
                    let shape = ShapeGeneralLine::ShapeGeneralLine::new(&self.comboPoints, &self.config);
                    if shape.isCompleted() {
                        result = Some(Box::new(shape));
                    }
                }

                // Java: if (null != result)
                // Java: switch (result.getType())
                // Java: case 8:
                // Java: case 16:
                // Java: case 32:
                // Java: case 64:
                // Java: case 512:
                // Java: case 1024:
                // Java: case 2048:

                // Java: this.comboPoints.clear();
                self.comboPoints.clear();
                // Java: this.strokePositions.clear();
                self.strokePositions.clear();
            }

            // Java: if (!isSameShape && null != var4)
            if !isSameShape {
                if let Some(v) = var4 {
                    // Java: this.strokes.add(var4);
                    self.strokes.push(v);
                }
            }

            // Java: return result;
            result
        }
    }

    // Java: boolean shouldClear()
    pub fn shouldClear(&self) -> bool {
        // Java: ShapeConfig var10001 = this.config;
        // Java: return this.waitTime > 20000L;
        self.waitTime > 20000
    }

    // Java: boolean isStrokesEmpty()
    pub fn isStrokesEmpty(&self) -> bool {
        // Java: return this.strokes.isEmpty();
        self.strokes.is_empty()
    }

    // Java: boolean isTimeout()
    pub fn isTimeout(&self) -> bool {
        // Java: long var10000 = System.currentTimeMillis() - this.startTime;
        // Java: ShapeConfig var10001 = this.config;
        // Java: return var10000 >= 400L;
        current_time_millis() - self.startTime >= 400
    }

    // Java: void clearStrokes()
    pub fn clearStrokes(&mut self) {
        // Java: while(!this.strokes.isEmpty())
        while !self.strokes.is_empty() {
            // Java: ((List)this.strokes.remove(0)).clear();
            self.strokes.remove(0);
        }
    }

    // Java: void resetTime()
    pub fn resetTime(&mut self) {
        // Java: this.startTime = System.currentTimeMillis();
        self.startTime = current_time_millis();
    }

    // Java: void calcWaitTime()
    pub fn calcWaitTime(&mut self) {
        // Java: this.waitTime = System.currentTimeMillis() - this.startTime;
        self.waitTime = current_time_millis() - self.startTime;
    }

    // Java: void setStrokeWidth(float width)
    pub fn setStrokeWidth(&mut self, width: f32) {
        // Java: this.config.strokeWidth = width;
        self.config.strokeWidth = width;
    }

    // Java: void comboStrokes()
    fn comboStrokes(&mut self) {
        // Java: this.comboPoints.addAll((Collection)this.strokes.remove(0));
        let first_stroke = self.strokes.remove(0);
        self.comboPoints.extend(first_stroke);
        // Java: this.strokePositions.add(new StrokePosition(0, this.comboPoints.size()));
        self.strokePositions.push(StrokePosition::new(0, self.comboPoints.len() as i32));

        // Java: while(this.strokes.size() > 0)
        while !self.strokes.is_empty() {
            // Java: int strokeInx = 0;
            let mut strokeInx: usize = 0;
            // Java: boolean startHead = true;
            let mut startHead = true;
            // Java: Point lastPt = (Point)this.comboPoints.get(this.comboPoints.size() - 1);
            let lastPt = self.comboPoints.last().unwrap().clone();
            // Java: float minSqrDst = Float.MAX_VALUE;
            let mut minSqrDst = f32::MAX;

            // Java: for(int i = 0; i < this.strokes.size(); ++i)
            for i in 0..self.strokes.len() {
                // Java: List<Point> stroke = (List)this.strokes.get(i);
                let stroke = &self.strokes[i];
                // Java: float sqrDst = lastPt.getSquareDst((Point)stroke.get(0));
                let sqrDst = lastPt.getSquareDst(&stroke[0]);
                // Java: if (sqrDst < minSqrDst)
                if sqrDst < minSqrDst {
                    // Java: minSqrDst = sqrDst;
                    minSqrDst = sqrDst;
                    // Java: strokeInx = i;
                    strokeInx = i;
                    // Java: startHead = true;
                    startHead = true;
                }

                // Java: sqrDst = lastPt.getSquareDst((Point)stroke.get(stroke.size() - 1));
                let sqrDst = lastPt.getSquareDst(stroke.last().unwrap());
                // Java: if (sqrDst < minSqrDst)
                if sqrDst < minSqrDst {
                    // Java: minSqrDst = sqrDst;
                    minSqrDst = sqrDst;
                    // Java: strokeInx = i;
                    strokeInx = i;
                    // Java: startHead = false;
                    startHead = false;
                }
            }

            // Java: List<Point> stroke = (List)this.strokes.remove(strokeInx);
            let stroke = self.strokes.remove(strokeInx);
            // Java: this.strokePositions.add(new StrokePosition(this.comboPoints.size(), this.comboPoints.size() + stroke.size()));
            self.strokePositions.push(StrokePosition::new(
                self.comboPoints.len() as i32,
                (self.comboPoints.len() + stroke.len()) as i32,
            ));
            // Java: if (startHead)
            if startHead {
                // Java: this.comboPoints.addAll(stroke);
                self.comboPoints.extend(stroke);
            // Java: else
            } else {
                // Java: for(int i = stroke.size() - 1; i >= 0; --i)
                for i in (0..stroke.len()).rev() {
                    // Java: this.comboPoints.add((Point)stroke.get(i));
                    self.comboPoints.push(stroke[i].clone());
                }
            }
        }
    }
}
