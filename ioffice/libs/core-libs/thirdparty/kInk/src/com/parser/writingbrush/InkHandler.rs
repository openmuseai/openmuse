// Java: import android.view.MotionEvent
// (MotionEvent is an Android type - using a stub)

pub struct MotionEvent;

// Java: public interface InkHandler
pub trait InkHandler {
    // Java: abstract void start(MotionEvent var1)
    fn start(&mut self, var1: &MotionEvent);

    // Java: abstract void move(MotionEvent var1)
    fn r#move(&mut self, var1: &MotionEvent);

    // Java: abstract void end(MotionEvent var1)
    fn end(&mut self, var1: &MotionEvent);

}
