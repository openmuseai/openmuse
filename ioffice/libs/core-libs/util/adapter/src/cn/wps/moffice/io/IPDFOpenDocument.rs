// Java: public interface IPDFOpenDocument
pub trait IPDFOpenDocument {
    // Java: abstract boolean checkPassword(String var1)
    fn checkPassword(&self, _var1: &str) -> bool;

    // Java: abstract void closeDocument()
    fn closeDocument(&mut self);

    // Java: abstract boolean hasPermission()
    fn hasPermission(&self) -> bool;

    // Java: abstract int getOpenPageCount()
    fn getOpenPageCount(&self) -> i32;

}
