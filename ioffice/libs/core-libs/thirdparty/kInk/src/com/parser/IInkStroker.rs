// Java: public interface IInkStroker
pub trait IInkStroker {
    // Java: abstract void onStart(float var1, float var2, float var3)
    fn onStart(&mut self, var1: f32, var2: f32, var3: f32);

    // Java: abstract void onMove(float var1, float var2, float var3)
    fn onMove(&mut self, var1: f32, var2: f32, var3: f32);

    // Java: abstract void onFinish()
    fn onFinish(&mut self);

    // Java: abstract float getStrokeWidth()
    fn getStrokeWidth(&self) -> f32;
}
