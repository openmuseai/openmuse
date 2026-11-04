#![allow(non_snake_case)]

// Java field: int PATH_NONE
pub const PATH_NONE: i32 = -1;
// Java field: int PATH_MOVE
pub const PATH_MOVE: i32 = 0;
// Java field: int PATH_LINE
pub const PATH_LINE: i32 = 1;
// Java field: int PATH_ARC
pub const PATH_ARC: i32 = 2;
// Java field: int PATH_QUAD
pub const PATH_QUAD: i32 = 3;
// Java field: int PATH_CUBIC
pub const PATH_CUBIC: i32 = 4;
// Java field: int PATH_CLOSE
pub const PATH_CLOSE: i32 = 5;
// Java field: int PATH_END
pub const PATH_END: i32 = 6;

#[derive(Clone)]
// Java: public class PathElem
pub struct PathElem {
    // Java field: int elemType
    pub elemType: i32,
    // Java field: float[] pos
    pub pos: Vec<f32>,
}

// Java: public class PathElem
impl PathElem {
    // Java: PathElem()
    pub fn new() -> Self {
        // Java: super();
        PathElem {
            elemType: -1,
            pos: Vec::new(),
        }
    }

    // Java: PathElem(int type)
    pub fn with_type(elem_type: i32) -> Self {
        // Java: super();
        let mut instance = PathElem::new();
        instance.init_elem_type(elem_type);
        instance
    }

    fn init_elem_type(&mut self, elem_type: i32) {
        // Java: this.elemType = type;
        self.elemType = elem_type;
    }

    // Java: float[] getPoints()
    pub fn getPoints(&self) -> &Vec<f32> {
        // Java: return this.pos;
        &self.pos
    }

    // Java: void setLineTo(float x, float y)
    pub fn setLineTo(&mut self, x: f32, y: f32) {
        // Java: this.elemType = 1;
        self.elemType = 1;
        // Java: this.pos = new float[2];
        self.pos = vec![0.0; 2];
        // Java: this.pos[0] = x;
        self.pos[0] = x;
        // Java: this.pos[1] = y;
        self.pos[1] = y;
    }

    // Java: static PathElem createMoveElem(float x, float y)
    pub fn createMoveElem(x: f32, y: f32) -> PathElem {
        // Java: PathElem elem = new PathElem(0);
        let mut elem = PathElem::with_type(0);
        // Java: elem.pos = new float[]{x, y};
        elem.pos = vec![x, y];
        // Java: return elem;
        elem
    }

    // Java: static PathElem createLineElem(float x, float y)
    pub fn createLineElem(x: f32, y: f32) -> PathElem {
        // Java: PathElem elem = new PathElem(1);
        let mut elem = PathElem::with_type(1);
        // Java: elem.pos = new float[]{x, y};
        elem.pos = vec![x, y];
        // Java: return elem;
        elem
    }

    // Java: static PathElem createArcElem(float w, float h, float startAngle, float swingAngle)
    pub fn createArcElem(w: f32, h: f32, startAngle: f32, swingAngle: f32) -> PathElem {
        // Java: PathElem elem = new PathElem(2);
        let mut elem = PathElem::with_type(2);
        // Java: elem.pos = new float[]{w, h, startAngle, swingAngle};
        elem.pos = vec![w, h, startAngle, swingAngle];
        // Java: return elem;
        elem
    }

    // Java: static PathElem createQuadElem(float ctrlX, float ctrlY, float anchorX, float anchorY)
    pub fn createQuadElem(ctrlX: f32, ctrlY: f32, anchorX: f32, anchorY: f32) -> PathElem {
        // Java: PathElem elem = new PathElem(3);
        let mut elem = PathElem::with_type(3);
        // Java: elem.pos = new float[]{ctrlX, ctrlY, anchorX, anchorY};
        elem.pos = vec![ctrlX, ctrlY, anchorX, anchorY];
        // Java: return elem;
        elem
    }

    // Java: static PathElem createCubicElem(float ctrlX1, float ctrlY1, float ctrlX2, float ctrlY2, float anchorX, float anchorY)
    pub fn createCubicElem(ctrlX1: f32, ctrlY1: f32, ctrlX2: f32, ctrlY2: f32, anchorX: f32, anchorY: f32) -> PathElem {
        // Java: PathElem elem = new PathElem(4);
        let mut elem = PathElem::with_type(4);
        // Java: elem.pos = new float[]{ctrlX1, ctrlY1, ctrlX2, ctrlY2, anchorX, anchorY};
        elem.pos = vec![ctrlX1, ctrlY1, ctrlX2, ctrlY2, anchorX, anchorY];
        // Java: return elem;
        elem
    }

    // Java: static PathElem createCloseElem()
    pub fn createCloseElem() -> PathElem {
        // Java: return new PathElem(5);
        PathElem::with_type(5)
    }

    // Java: static PathElem createEndElem()
    pub fn createEndElem() -> PathElem {
        // Java: return new PathElem(6);
        PathElem::with_type(6)
    }
}
