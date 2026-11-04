// Java: public interface Attribute extends Node
pub trait Attribute {
    // Java: abstract void recycle()
    fn recycle(&mut self);

    // Java: abstract String getNamespacePrefix()
    fn getNamespacePrefix(&self) -> &str;

    // Java: abstract String getNamespaceURI()
    fn getNamespaceURI(&self) -> &str;

    // Java: abstract String getQualifiedName()
    fn getQualifiedName(&self) -> String;

    // Java: abstract String getValue()
    fn getValue(&self) -> &str;

    // Java: abstract void setValue(String var1)
    fn setValue(&mut self, var1: String);

    // Java: abstract Object getData()
    fn getData(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract void setData(Object var1)
    fn setData(&mut self, var1: Box<dyn std::any::Any>);
}
