// Java: import android.graphics.RectF
pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }

// Java: public interface IHolder
pub trait IHolder {
    // Java: abstract void setTextRect(RectF var1)
    fn setTextRect(&mut self, var1: RectF);

    // Java: abstract boolean hasFontSizeFit()
    fn hasFontSizeFit(&self) -> bool;

    // Java: abstract void clearFontSizeFit()
    fn clearFontSizeFit(&mut self);

    // Java: abstract boolean setFontScale(float var1, double var2)
    fn setFontScale(&mut self, var1: f32, var2: f64) -> bool;

    // Java: abstract Object getHolder()
    fn getHolder(&self) -> Option<&dyn std::any::Any>;

}
