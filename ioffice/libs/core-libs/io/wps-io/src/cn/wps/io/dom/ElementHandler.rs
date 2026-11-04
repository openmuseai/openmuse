// Java: public interface ElementHandler
pub trait ElementHandler {
    // Java: abstract void onStart(ElementPath var1)
    fn onStart(&mut self, var1: &dyn super::ElementPath::ElementPath);

    // Java: abstract void onCharacters(String var1)
    fn onCharacters(&mut self, var1: &str);

    // Java: abstract void onEnd(ElementPath var1)
    fn onEnd(&mut self, var1: &dyn super::ElementPath::ElementPath);

    // Java: abstract boolean shouldNotifyChild()
    fn shouldNotifyChild(&self) -> bool;

    // Java: abstract void onStartChild(ElementPath var1)
    fn onStartChild(&mut self, var1: &dyn super::ElementPath::ElementPath);

    // Java: abstract void onEndChild(ElementPath var1)
    fn onEndChild(&mut self, var1: &dyn super::ElementPath::ElementPath);
}
