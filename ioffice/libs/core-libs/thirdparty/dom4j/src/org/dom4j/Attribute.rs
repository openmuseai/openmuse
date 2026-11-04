use std::any::Any;

use super::Node::Node;
use super::QName::QName;
use super::Namespace::Namespace;

// Java: public interface Attribute extends Node
pub trait Attribute: Node {
    // Java: abstract QName getQName()
    fn getQName(&self) -> &QName;

    // Java: abstract Namespace getNamespace()
    fn getNamespace(&self) -> &Namespace;

    // Java: abstract void setNamespace(Namespace var1)
    fn setNamespace(&mut self, var1: Namespace);

    // Java: abstract String getNamespacePrefix()
    fn getNamespacePrefix(&self) -> &str;

    // Java: abstract String getNamespaceURI()
    fn getNamespaceURI(&self) -> &str;

    // Java: abstract String getQualifiedName()
    fn getQualifiedName(&self) -> &str;

    // Java: abstract String getValue()
    fn getValue(&self) -> &str;

    // Java: abstract void setValue(String var1)
    fn setValue(&mut self, var1: &str);

    // Java: abstract Object getData()
    fn getData(&self) -> &dyn Any;

    // Java: abstract void setData(Object var1)
    fn setData(&mut self, var1: Box<dyn Any>);
}
