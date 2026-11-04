// Java: public interface ElementPath
pub trait ElementPath {
    // Java: abstract boolean getOnlyUseName()
    fn getOnlyUseName(&self) -> bool;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract Element getElement(int var1)
    fn getElement(&self, var1: i32) -> Option<&dyn std::any::Any>;

    // Java: abstract String getPath()
    fn getPath(&self) -> String;

    // Java: abstract Element getCurrent()
    fn getCurrent(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract String getCurrentName()
    fn getCurrentName(&self) -> String;

    // Java: abstract void addHandler(String var1, ElementHandler var2)
    fn addHandler(&mut self, var1: &str, var2: Box<dyn super::ElementHandler::ElementHandler>);

    // Java: abstract void removeHandler(String var1)
    fn removeHandler(&mut self, var1: &str);
}
