#![allow(non_snake_case, dead_code, unused_imports)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IChartDevice
pub trait IChartDevice {
    // Java: abstract void draw(Object var1, float var2, float var3, boolean var4)
    fn draw(&self, var1: Element, var2: f32, var3: f32, var4: bool);

    // Java: abstract boolean draw(String var1, float var2, float var3, float var4, boolean var5)
    fn draw_from_path(&self, var1: &str, var2: f32, var3: f32, var4: f32, var5: bool) -> bool;

}
