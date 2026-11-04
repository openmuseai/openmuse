// Java: public interface ICreateResume
pub trait ICreateResume {
    // Java: abstract void onCreateCallback(boolean var1)
    fn onCreateCallback(&self, _var1: bool);

    // Java: abstract void onRenderCallback(byte[] var1)
    fn onRenderCallback(&self, _var1: &[u8]);

}
