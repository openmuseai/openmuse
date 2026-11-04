// Java: import cn.wps.core.runtime.IDisposable
use util_adapter::cn::wps::core::runtime::IDisposable;

// Java field: int TYPE_START
pub const TYPE_START: i32 = 0;
// Java field: int TYPE_MOVE
pub const TYPE_MOVE: i32 = 1;
// Java field: int TYPE_FINISH
pub const TYPE_FINISH: i32 = 2;
// Java field: int TYPE_SHART
pub const TYPE_SHART: i32 = 3;

// Java: public interface IPointProcess extends IDisposable
pub trait IPointProcess {
    // Java: abstract void process(float var1, float var2, float var3, float var4, float var5, int var6)
    fn process(&mut self, var1: f32, var2: f32, var3: f32, var4: f32, var5: f32, var6: i32);

    // Java: abstract void clear()
    fn clear(&mut self);

}
