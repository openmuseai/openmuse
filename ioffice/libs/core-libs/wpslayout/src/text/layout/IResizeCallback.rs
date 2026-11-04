// Java: public interface IResizeCallback
pub trait IResizeCallback {
    // Java: abstract void resize(ITextLayout var1, float var2, float var3, float var4, float var5)
    fn resize(&self, ITextLayout var1, float var2, float var3, float var4, float var5);

}
