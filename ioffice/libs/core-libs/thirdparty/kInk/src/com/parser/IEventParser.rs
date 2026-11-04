// Java: public interface IEventParser extends IDisposable
pub trait IEventParser {
    // Java: abstract void parseEvent(MotionEvent var1)
    fn parseEvent(&mut self, var1: i64);

    // Java: abstract void touchCancel()
    fn touchCancel(&mut self);

    // Java: abstract void clear()
    fn clear(&mut self);
}
