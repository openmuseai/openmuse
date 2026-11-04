// Java: import java.util.List
// Java: import org.dom4j.dtd.ExternalDeclaration
// use crate::org::dom4j::dtd::ExternalDeclaration;
// Java: import org.dom4j.dtd.InternalDeclaration
// use crate::org::dom4j::dtd::InternalDeclaration;

// Java: public interface DocumentType extends Node
pub trait DocumentType {
    // Java: abstract String getElementName()
    fn getElementName(&self) -> String;

    // Java: abstract void setElementName(String var1)
    fn setElementName(&mut self, _var1: &str);

    // Java: abstract String getPublicID()
    fn getPublicID(&self) -> String;

    // Java: abstract void setPublicID(String var1)
    fn setPublicID(&mut self, _var1: &str);

    // Java: abstract String getSystemID()
    fn getSystemID(&self) -> String;

    // Java: abstract void setSystemID(String var1)
    fn setSystemID(&mut self, _var1: &str);

    // Java: abstract List<InternalDeclaration> getInternalDeclarations()
    fn getInternalDeclarations(&self) -> Vec<i32>;

    // Java: abstract void setInternalDeclarations(List<InternalDeclaration> var1)
    fn setInternalDeclarations(&mut self, _var1: Vec<i32>);

    // Java: abstract List<ExternalDeclaration> getExternalDeclarations()
    fn getExternalDeclarations(&self) -> Vec<i32>;

    // Java: abstract void setExternalDeclarations(List<ExternalDeclaration> var1)
    fn setExternalDeclarations(&mut self, _var1: Vec<i32>);

}
