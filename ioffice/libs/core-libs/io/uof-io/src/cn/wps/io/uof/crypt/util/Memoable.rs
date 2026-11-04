// Java: public interface Memoable
pub trait Memoable {
    // Java: abstract Memoable copy()
    fn copy(&self) -> Box<dyn Memoable>;

    // Java: abstract void reset(Memoable var1)
    fn reset_from(&mut self, var1: &dyn Memoable);

}
