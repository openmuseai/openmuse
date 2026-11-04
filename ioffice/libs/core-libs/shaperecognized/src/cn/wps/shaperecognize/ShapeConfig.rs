// Java: import cn.wps.shaperecognize.shape.ShapeClose
use crate::cn::wps::shaperecognize::shape::ShapeClose;
// Java: import cn.wps.shaperecognize.shape.ShapeDeleteLine
use crate::cn::wps::shaperecognize::shape::ShapeDeleteLine;

// Java field: int STROKE_CLEAR_WAIT_TIME
pub const STROKE_CLEAR_WAIT_TIME: i32 = 20000;
// Java field: int STROKE_LONG_WAIT_TIME
pub const STROKE_LONG_WAIT_TIME: i32 = 400;
// Java field: int STROKE_SHORT_WAIT_TIME
pub const STROKE_SHORT_WAIT_TIME: i32 = 250;
// Java field: int STROKE_SHORT_WAIT_SATISFY
pub const STROKE_SHORT_WAIT_SATISFY: i32 = 2;

// Java: public class ShapeConfig
#[derive(Clone)]
pub struct ShapeConfig {
    // Java field: float NA
    pub NA: f32,
    // Java field: float density
    pub density: f32,
    // Java field: float deviation
    pub deviation: f32,
    // Java field: float lineKMaxDvtion
    pub lineKMaxDvtion: f32,
    // Java field: float lintKMinDvtion
    pub lintKMinDvtion: f32,
    // Java field: float circleGapPercent
    pub circleGapPercent: f32,
    // Java field: float strokeWidth
    pub strokeWidth: f32,
    // Java field: int FLAG_RECT
    pub FLAG_RECT: i32,
    // Java field: int FLAG_TRAN
    pub FLAG_TRAN: i32,
    // Java field: int FLAG_LINE
    pub FLAG_LINE: i32,
    // Java field: int FLAG_NOTE_CIRCLE
    pub FLAG_NOTE_CIRCLE: i32,
    // Java field: int FLAG_OVAL_CIRCLE
    pub FLAG_OVAL_CIRCLE: i32,
    // Java field: int FLAG_INSERT
    pub FLAG_INSERT: i32,
    // Java field: int FLAG_SWAP
    pub FLAG_SWAP: i32,
    // Java field: int FLAG_ARROW
    pub FLAG_ARROW: i32,
    // Java field: int FLAG_SIMPLE_SHAPES
    pub FLAG_SIMPLE_SHAPES: i32,
    // Java field: int FLAG_RECOGNIZE_SHAPES
    pub FLAG_RECOGNIZE_SHAPES: i32,
    // Java field: int flag
    pub flag: i32,
}

// Java: public class ShapeConfig
impl ShapeConfig {
    // Java: ShapeConfig()
    pub fn new() -> Self {
        // Java: super();
        let flag_rect = 1;
        let flag_tran = 2;
        let flag_line = 4;
        let flag_note_circle = 8;
        let flag_oval_circle = 16;
        let flag_insert = 32;
        let flag_swap = 64;
        let flag_arrow = 128;
        ShapeConfig {
            NA: -65535.0,
            // Java: this.density = this.NA;
            density: -65535.0,
            // Java: this.deviation = ShapeDeleteLine.DEVIATION;
            deviation: ShapeDeleteLine::DEVIATION,
            // Java: this.lineKMaxDvtion = ShapeDeleteLine.MAXK;
            lineKMaxDvtion: ShapeDeleteLine::MAXK,
            // Java: this.lintKMinDvtion = ShapeDeleteLine.MINK;
            lintKMinDvtion: ShapeDeleteLine::MINK,
            // Java: this.circleGapPercent = ShapeClose.CIRCLE_GAP_PERCENT;
            circleGapPercent: ShapeClose::CIRCLE_GAP_PERCENT,
            strokeWidth: 0.0,
            // Java: this.FLAG_RECT = 1;
            FLAG_RECT: flag_rect,
            // Java: this.FLAG_TRAN = 2;
            FLAG_TRAN: flag_tran,
            // Java: this.FLAG_LINE = 4;
            FLAG_LINE: flag_line,
            // Java: this.FLAG_NOTE_CIRCLE = 8;
            FLAG_NOTE_CIRCLE: flag_note_circle,
            // Java: this.FLAG_OVAL_CIRCLE = 16;
            FLAG_OVAL_CIRCLE: flag_oval_circle,
            // Java: this.FLAG_INSERT = 32;
            FLAG_INSERT: flag_insert,
            // Java: this.FLAG_SWAP = 64;
            FLAG_SWAP: flag_swap,
            // Java: this.FLAG_ARROW = 128;
            FLAG_ARROW: flag_arrow,
            // Java: this.FLAG_SIMPLE_SHAPES = this.FLAG_LINE | this.FLAG_NOTE_CIRCLE;
            FLAG_SIMPLE_SHAPES: flag_line | flag_note_circle,
            // Java: this.FLAG_RECOGNIZE_SHAPES = this.FLAG_LINE | this.FLAG_OVAL_CIRCLE | this.FLAG_TRAN | this.FLAG_RECT | this.FLAG_ARROW;
            FLAG_RECOGNIZE_SHAPES: flag_line | flag_oval_circle | flag_tran | flag_rect | flag_arrow,
            flag: 0,
        }
    }

    // Java: void setDensity(float density)
    pub fn setDensity(&mut self, density: f32) {
        // Java: this.density = density;
        self.density = density;
    }

    // Java: void setLineDvtion(float dvtion)
    pub fn setLineDvtion(&mut self, dvtion: f32) {
        // Java: this.deviation = dvtion;
        self.deviation = dvtion;
    }

    // Java: void setLineKDvtion(float min, float max)
    pub fn setLineKDvtion(&mut self, min: f32, max: f32) {
        // Java: this.lineKMaxDvtion = max;
        self.lineKMaxDvtion = max;
        // Java: this.lintKMinDvtion = min;
        self.lintKMinDvtion = min;
    }

    // Java: void setCircleGapPercent(float perc)
    pub fn setCircleGapPercent(&mut self, perc: f32) {
        // Java: this.circleGapPercent = perc;
        self.circleGapPercent = perc;
    }

    // Java: void addRect()
    pub fn addRect(&mut self) {
        // Java: this.flag |= this.FLAG_RECT;
        self.flag |= self.FLAG_RECT;
    }

    // Java: void addTran()
    pub fn addTran(&mut self) {
        // Java: this.flag |= this.FLAG_TRAN;
        self.flag |= self.FLAG_TRAN;
    }

    // Java: void addCircl()
    pub fn addCircl(&mut self) {
        // Java: this.flag |= this.FLAG_NOTE_CIRCLE;
        self.flag |= self.FLAG_NOTE_CIRCLE;
    }

    // Java: void addLine()
    pub fn addLine(&mut self) {
        // Java: this.flag |= this.FLAG_LINE;
        self.flag |= self.FLAG_LINE;
    }

    // Java: void addInsert()
    pub fn addInsert(&mut self) {
        // Java: this.flag |= this.FLAG_INSERT;
        self.flag |= self.FLAG_INSERT;
    }

    // Java: void addSwap()
    pub fn addSwap(&mut self) {
        // Java: this.flag |= this.FLAG_SWAP;
        self.flag |= self.FLAG_SWAP;
    }

    // Java: void addArrow()
    pub fn addArrow(&mut self) {
        // Java: this.flag |= this.FLAG_ARROW;
        self.flag |= self.FLAG_ARROW;
    }

    // Java: void setSimpleShapes()
    pub fn setSimpleShapes(&mut self) {
        // Java: this.flag = this.FLAG_SIMPLE_SHAPES;
        self.flag = self.FLAG_SIMPLE_SHAPES;
    }

    // Java: void setRecognizeShapes()
    pub fn setRecognizeShapes(&mut self) {
        // Java: this.flag = this.FLAG_RECOGNIZE_SHAPES;
        self.flag = self.FLAG_RECOGNIZE_SHAPES;
    }

    // Java: boolean supportRect()
    pub fn supportRect(&self) -> bool {
        // Java: return (this.flag & this.FLAG_RECT) == this.FLAG_RECT;
        (self.flag & self.FLAG_RECT) == self.FLAG_RECT
    }

    // Java: boolean supportTrangle()
    pub fn supportTrangle(&self) -> bool {
        // Java: return (this.flag & this.FLAG_TRAN) == this.FLAG_TRAN;
        (self.flag & self.FLAG_TRAN) == self.FLAG_TRAN
    }

    // Java: boolean supportNoteCircle()
    pub fn supportNoteCircle(&self) -> bool {
        // Java: return (this.flag & this.FLAG_NOTE_CIRCLE) == this.FLAG_NOTE_CIRCLE;
        (self.flag & self.FLAG_NOTE_CIRCLE) == self.FLAG_NOTE_CIRCLE
    }

    // Java: boolean supportOvalCircle()
    pub fn supportOvalCircle(&self) -> bool {
        // Java: return (this.flag & this.FLAG_OVAL_CIRCLE) == this.FLAG_OVAL_CIRCLE;
        (self.flag & self.FLAG_OVAL_CIRCLE) == self.FLAG_OVAL_CIRCLE
    }

    // Java: boolean supportLine()
    pub fn supportLine(&self) -> bool {
        // Java: return (this.flag & this.FLAG_LINE) == this.FLAG_LINE;
        (self.flag & self.FLAG_LINE) == self.FLAG_LINE
    }

    // Java: boolean supportSwap()
    pub fn supportSwap(&self) -> bool {
        // Java: return (this.flag & this.FLAG_SWAP) == this.FLAG_SWAP;
        (self.flag & self.FLAG_SWAP) == self.FLAG_SWAP
    }

    // Java: boolean supportInsert()
    pub fn supportInsert(&self) -> bool {
        // Java: return (this.flag & this.FLAG_INSERT) == this.FLAG_INSERT;
        (self.flag & self.FLAG_INSERT) == self.FLAG_INSERT
    }

    // Java: boolean supportArrow()
    pub fn supportArrow(&self) -> bool {
        // Java: return (this.flag & this.FLAG_ARROW) == this.FLAG_ARROW;
        (self.flag & self.FLAG_ARROW) == self.FLAG_ARROW
    }
}
