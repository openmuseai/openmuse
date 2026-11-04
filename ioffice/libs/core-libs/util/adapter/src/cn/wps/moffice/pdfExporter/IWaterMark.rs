// Java: public interface IWaterMark
pub trait IWaterMark {
    // Java: abstract void draw(Object var1, float var2, float var3)
    fn draw(&self, _var1: &dyn std::any::Any, _var2: f32, _var3: f32);

}
