#![allow(non_snake_case)]

// Java: public class AdjustHandler
#[derive(Clone)]
pub struct AdjustHandler {
    // Java field: int type
    pub(crate) r#type: i32,
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float gdX
    pub(crate) gdX: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: float gdY
    pub(crate) gdY: f32,
}

// Java: public class AdjustHandler
impl AdjustHandler {
    // Java field: int INVALID
    pub const INVALID: i32 = 0;
    // Java field: int X
    pub const X: i32 = 1;
    // Java field: int Y
    pub const Y: i32 = 2;
    // Java field: int XY
    pub const XY: i32 = 3;
    // Java field: int ANGLE
    pub const ANGLE: i32 = 4;
    // Java field: int RADIUS
    pub const RADIUS: i32 = 5;
    // Java field: int ANGLERADIUS
    pub const ANGLERADIUS: i32 = 6;
    // Java field: int SINGLE
    pub const SINGLE: i32 = 7;
    // Java field: int COUPLE
    pub const COUPLE: i32 = 8;

    // Java: AdjustHandler(int type)
    pub fn new(r#type: i32) -> Self {
        let mut instance = Self {
            r#type: 0,
            x: 0.0,
            gdX: 0.0,
            y: 0.0,
            gdY: 0.0,
        };
        instance._init(r#type);
        instance
    }

    fn _init(&mut self, r#type: i32) {
        // Java: super();
        *self = Self { r#type: 0, x: 0.0, gdX: 0.0, y: 0.0, gdY: 0.0 };
        // Java: this.type = type;
        self.r#type = r#type; // self.type = type
    }

    // Java: int getCategory()
    pub fn getCategory(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: AdjustHandler clone()
    pub fn clone(&self) -> AdjustHandler {
        // Java: throw new Error("Need override");
        panic!("Need override")
    }
}
