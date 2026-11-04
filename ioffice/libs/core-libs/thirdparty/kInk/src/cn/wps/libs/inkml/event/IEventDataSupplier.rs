// Java: public interface IEventDataSupplier
pub trait IEventDataSupplier {
    // Java: abstract void postEvent(MotionEvent var1, boolean var2)
    fn postEvent(&mut self, var1: i64, var2: bool);

    // Java: abstract void setCallback(EventDtaCallback var1)
    fn setCallback(&mut self, var1: Box<dyn EventDtaCallback>);
}

// Java: public interface EventDtaCallback
pub trait EventDtaCallback {
    // Java: abstract void onData(int var1, float[] var2, float[] var3, float[] var4, int var5)
    fn onData(&self, var1: i32, var2: &[f32], var3: &[f32], var4: &[f32], var5: i32);
}
