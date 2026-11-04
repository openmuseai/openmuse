#![allow(non_snake_case, dead_code)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import cn.wps.moffice.drawing.color.IColor

// Java: public interface IColorScheme
pub trait IColorScheme {
    // Java: abstract boolean hasName()
    fn hasName(&self) -> bool;

    // Java: abstract String name()
    fn name(&self) -> String;

    // Java: abstract IColor color(int var1)
    fn color(&self, var1: i32) -> Element;

}
