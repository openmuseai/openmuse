#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.geotext.GeoText
// Java: import cn.wps.moffice.drawing.media.IMediaLib

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IWordArt
pub trait IWordArt {
    // Java: abstract Object getBitmap(Shape var1, RectF var2, RectF var3)
    fn getBitmap(&self, var1: Element, var2: &RectF, var3: &RectF) -> Element;

    // Java: abstract boolean draw(Object var1, Shape var2, RectF var3)
    fn draw(&self, var1: Element, var2: Element, var3: &RectF) -> bool;

    // Java: abstract RectF getTextRect(GeoText var1, RectF var2)
    fn getTextRect(&self, var1: Element, var2: &RectF) -> RectF;

    // Java: abstract Object getBitmap(GeoText var1, String var2, RectF var3, String var4)
    fn getBitmap_from_geotext(&self, var1: Element, var2: &str, var3: &RectF, var4: &str) -> Element;

    // Java: abstract boolean draw(Object var1, GeoText var2, String var3, RectF var4, String var5)
    fn draw_geotext(&self, var1: Element, var2: Element, var3: &str, var4: &RectF, var5: &str) -> bool;

    // Java: abstract void init(IMediaLib var1)
    fn init(&mut self, var1: Element);

}
