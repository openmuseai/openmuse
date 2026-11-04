// Java: public interface IDocumentLoaderListener
pub trait IDocumentLoaderListener {
    // Java: abstract void onLoadFinish(IDocument var1)
    fn onLoadFinish(&self, _var1: &dyn std::any::Any);

    // Java: abstract void onNeedPassword(IDocument var1)
    fn onNeedPassword(&self, _var1: &dyn std::any::Any);

    // Java: abstract void onOpenForceQuit()
    fn onOpenForceQuit(&self);

}
