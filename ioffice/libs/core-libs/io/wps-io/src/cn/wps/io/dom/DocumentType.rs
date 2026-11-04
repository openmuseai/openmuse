// Java: public interface DocumentType extends Node
pub trait DocumentType {
    // Java: abstract String getElementName()
    fn getElementName(&self) -> &str;

    // Java: abstract void setElementName(String var1)
    fn setElementName(&mut self, var1: String);

    // Java: abstract String getPublicID()
    fn getPublicID(&self) -> &str;

    // Java: abstract void setPublicID(String var1)
    fn setPublicID(&mut self, var1: String);

    // Java: abstract String getSystemID()
    fn getSystemID(&self) -> &str;

    // Java: abstract void setSystemID(String var1)
    fn setSystemID(&mut self, var1: String);
}
