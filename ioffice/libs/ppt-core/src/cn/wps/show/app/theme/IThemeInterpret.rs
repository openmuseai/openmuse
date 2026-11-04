#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;

// Java: public interface IThemeInterpret
pub trait IThemeInterpret {
    // Java: abstract Fill fillAt(int var1)
    fn fillAt(&self, var1: i32) -> Fill;

    // Java: abstract Outline outlineAt(int var1)
    fn outlineAt(&self, var1: i32) -> Outline;

    // Java: abstract void setSlide(ISlide var1)
    fn setSlide(&mut self, var1: &dyn ISlide);
}
