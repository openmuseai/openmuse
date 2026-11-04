#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.effects.Glow
// Java: import cn.wps.moffice.drawing.effects.Shadow
// Java: import cn.wps.moffice.drawing.graphics.exception.IGraphicsException
// Java: import cn.wps.moffice.drawing.line.LineProperty
// Java: import cn.wps.moffice.drawing.media.IMediaLib

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IGraphics
pub trait IGraphics {
    // Java: abstract void setRoatation(float var1)
    fn setRoatation(&mut self, var1: f32);

    // Java: abstract void setShadow(Shadow var1)
    fn setShadow(&mut self, var1: Element);

    // Java: abstract void setGlow(Glow var1)
    fn setGlow(&mut self, var1: Element);

    // Java: abstract void setFill(FillBase var1)
    fn setFill(&mut self, var1: Element);

    // Java: abstract void setStroke(LineProperty var1)
    fn setStroke(&mut self, var1: Element);

    // Java: abstract void setFlip(boolean var1, boolean var2)
    fn setFlip(&mut self, var1: bool, var2: bool);

    // Java: abstract void init(boolean var1, boolean var2, boolean var3)
    fn init(&mut self, var1: bool, var2: bool, var3: bool);

    // Java: abstract void reset()
    fn reset(&mut self);

    // Java: abstract void drawPicture(Picture var1, float var2, float var3, float var4, float var5, float var6)
    fn drawPicture(&mut self, var1: Element, var2: f32, var3: f32, var4: f32, var5: f32, var6: f32);

    // Java: abstract void drawPath(GraphicsPath var1, float var2)
    fn drawPath(&mut self, var1: Element, var2: f32);

    // Java: abstract void drawPath(GraphicsPath var1, float var2, RectF var3)
    fn drawPath_with_rect(&mut self, var1: Element, var2: f32, var3: &RectF);

    // Java: abstract void clipPath(Object var1)
    fn clipPath(&mut self, var1: Element);

    // Java: abstract void drawBitmap(Object var1, Object var2, Object var3, Object var4)
    fn drawBitmap(&mut self, var1: Element, var2: Element, var3: Element, var4: Element);

    // Java: abstract void drawRect(Object var1, Object var2)
    fn drawRect(&mut self, var1: Element, var2: Element);

    // Java: abstract void drawPath(GraphicsPath var1, float var2, RectF var3, boolean var4)
    fn drawPath_with_rect_flag(&mut self, var1: Element, var2: f32, var3: &RectF, var4: bool);

    // Java: abstract IGraphicsException getException()
    fn getException(&self) -> Element;

    // Java: abstract void save()
    fn save(&mut self);

    // Java: abstract void restore()
    fn restore(&mut self);

    // Java: abstract void translate(float var1, float var2)
    fn translate(&mut self, var1: f32, var2: f32);

    // Java: abstract void scale(float var1, float var2)
    fn scale(&mut self, var1: f32, var2: f32);

    // Java: abstract void scale(float var1, float var2, float var3, float var4)
    fn scale_with_pivot(&mut self, var1: f32, var2: f32, var3: f32, var4: f32);

    // Java: abstract void rotate(float var1)
    fn rotate(&mut self, var1: f32);

    // Java: abstract void rotate(float var1, float var2, float var3)
    fn rotate_with_pivot(&mut self, var1: f32, var2: f32, var3: f32);

    // Java: abstract void skew(float var1, float var2)
    fn skew(&mut self, var1: f32, var2: f32);

    // Java: abstract void concat(float[] var1)
    fn concat(&mut self, var1: &[f32]);

    // Java: abstract Object getCanvas()
    fn getCanvas(&self) -> Element;

    // Java: abstract boolean isInstractRender()
    fn isInstractRender(&self) -> bool;

    // Java: abstract void setPatternScale(float var1)
    fn setPatternScale(&mut self, var1: f32);

    // Java: abstract float getPatternScale()
    fn getPatternScale(&self) -> f32;

    // Java: abstract void setPrintMode(boolean var1)
    fn setPrintMode(&mut self, var1: bool);

    // Java: abstract boolean getPrintMode()
    fn getPrintMode(&self) -> bool;

    // Java: abstract void setFastDraw(boolean var1)
    fn setFastDraw(&mut self, var1: bool);

    // Java: abstract void setDrawHigh(boolean var1)
    fn setDrawHigh(&mut self, var1: bool);

    // Java: abstract boolean isFastDraw()
    fn isFastDraw(&self) -> bool;

    // Java: abstract void setVisibleRect(RectF var1)
    fn setVisibleRect(&mut self, var1: &RectF);

    // Java: abstract IMediaLib getMediaLib()
    fn getMediaLib(&self) -> Element;

}
