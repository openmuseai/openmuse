// Java: public interface ITEListener
pub trait ITEListener {
    // Java: abstract void onExtractFinish(String var1)
    fn onExtractFinish(&self, _var1: &str);

    // Java: abstract void onNeedPassword()
    fn onNeedPassword(&self);

}
