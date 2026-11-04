// Java: import java.util.ArrayList

// Java: public interface ISlimListener
pub trait ISlimListener {
    // Java: abstract void onFindSlimItem()
    fn onFindSlimItem(&self);

    // Java: abstract void onSlimCheckFinish(ArrayList<SlimResult> var1)
    fn onSlimCheckFinish(&self, var1: Vec<Box<dyn std::any::Any>>);

    // Java: abstract void onStopFinish()
    fn onStopFinish(&self);

    // Java: abstract void onSlimFinish()
    fn onSlimFinish(&self);

    // Java: abstract void onSlimItemFinish(int var1, long var2)
    fn onSlimItemFinish(&self, var1: i32, var2: i64);

}
