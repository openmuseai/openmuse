// Java: public class FactoryEstimate
pub trait FactoryEstimate {
    // Java: FactoryEstimate()
    // Java: super();

    // Java: boolean openDebug()
    fn openDebug(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: abstract boolean isEngineRuntimeAvailable(Context var1)
    fn isEngineRuntimeAvailable(&self, var1: i64) -> bool;

    // Java: abstract void addPoint(MotionEvent var1, long var2)
    fn addPoint_event(&mut self, var1: i64, var2: i64);

    // Java: abstract void addPoint(float var1, float var2, float var3)
    fn addPoint(&mut self, var1: f32, var2: f32, var3: f32);

    // Java: abstract void addHistoryPoint(MotionEvent var1)
    fn addHistoryPoint(&mut self, var1: i64);

    // Java: abstract List<PointF> getEstimatePoints()
    fn getEstimatePoints(&self) -> Vec<(f32, f32)>;

    // Java: abstract boolean isParamsOn()
    fn isParamsOn(&self) -> bool;

    // Java: abstract void onTouchDown(MotionEvent var1)
    fn onTouchDown(&mut self, var1: i64);

    // Java: abstract void clear()
    fn clear(&mut self);
}

// Java: static boolean isParamsOn(String key)
pub fn isParamsOn_static(_key: &str) -> bool {
    // Java: ISharedInfo sharedInfo = Platform.getSharedInfo();
    // Java: return sharedInfo != null && sharedInfo.isParamsOn(key);
    false
}
