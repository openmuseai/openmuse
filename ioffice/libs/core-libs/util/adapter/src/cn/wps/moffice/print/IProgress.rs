// Java: public interface IProgress
pub trait IProgress {
    // Java: abstract boolean isCanceled()
    fn isCanceled(&self) -> bool;

    // Java: abstract void setProgress(int var1)
    fn setProgress(&mut self, _var1: i32);

    // Java: abstract int getProgress()
    fn getProgress(&self) -> i32;

}
