// Java: import android.graphics.Canvas

// Java: public interface WatermarkCallback
pub trait WatermarkCallback {
    // Java: abstract void afterDrawGridView(Canvas var1)
    fn afterDrawGridView(&self, _var1: &dyn std::any::Any);

}
