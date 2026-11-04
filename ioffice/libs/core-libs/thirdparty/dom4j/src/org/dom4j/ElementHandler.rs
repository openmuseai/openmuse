use super::ElementPath::ElementPath;

// Java: public interface ElementHandler
pub trait ElementHandler {
    // Java: abstract void onStart(ElementPath var1)
    fn onStart(&self, var1: &dyn ElementPath);

    // Java: abstract void onCharacters(String var1)
    fn onCharacters(&self, var1: &str);

    // Java: abstract void onEnd(ElementPath var1)
    fn onEnd(&self, var1: &dyn ElementPath);

    // Java: abstract boolean shouldNotifyChild()
    fn shouldNotifyChild(&self) -> bool;

    // Java: abstract void onStartChild(ElementPath var1)
    fn onStartChild(&self, var1: &dyn ElementPath);

    // Java: abstract void onEndChild(ElementPath var1)
    fn onEndChild(&self, var1: &dyn ElementPath);
}
