// Java: public interface InkMLViewListener
pub trait InkMLViewListener {
    // Java: abstract void invalidate()
    fn invalidate(&self);

    // Java: abstract void postInvalidate()
    fn postInvalidate(&self);

}
