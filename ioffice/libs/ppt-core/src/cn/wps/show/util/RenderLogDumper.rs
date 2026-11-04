// Rust stub for RenderLogDumper (auto-generated)
// Original: cn/wps/show/util/RenderLogDumper.java
use crate::*;

pub struct RenderLogDumper {
    // TODO: fields
}

impl RenderLogDumper {
    pub fn new() -> Self { todo!() }
    pub fn reset() { todo!() }
    pub fn setRenderLogDumperListener(listener: RenderLogDumperListener) { todo!() }
    pub fn sumTime() -> f32 { todo!() }
    pub fn start() { todo!() }
    pub fn increaseLayout(nstart: i64, nend: i64) { todo!() }
    pub fn increaseConvert(nstart: i64, nend: i64) { todo!() }
    pub fn dumpSlideTime(slide: KmoSlide, nstart: i64, nend: i64) { todo!() }
    pub fn calcFps(f: i32, nsStart: i64, nsEnd: i64) -> f32 { todo!() }
    pub fn nano2ms(nsStart: i64, nsEnd: i64) -> f32 { todo!() }
}

pub trait RenderLogDumperListener {}
