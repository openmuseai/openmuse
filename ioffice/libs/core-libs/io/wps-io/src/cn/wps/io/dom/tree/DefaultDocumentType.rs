// Java: import cn.wps.io.dom.dtd.ExternalDeclaration
use crate::io::dom::dtd::ExternalDeclaration::ExternalDeclaration;
// Java: import cn.wps.io.dom.dtd.InternalDeclaration
use crate::io::dom::dtd::InternalDeclaration::InternalDeclaration;
// Java: import java.util.List

// Java: public class DefaultDocumentType extends AbstractDocumentType
pub struct DefaultDocumentType {
    // Java field: String elementName
    pub(crate) elementName: Option<Box<dyn std::any::Any>>,
    // Java field: String publicID
    pub(crate) publicID: Option<Box<dyn std::any::Any>>,
    // Java field: String systemID
    pub(crate) systemID: Option<Box<dyn std::any::Any>>,
    // Java field: List<InternalDeclaration> internalDeclarations
    internalDeclarations: Option<Vec<Box<dyn InternalDeclaration>>>,
    // Java field: List<ExternalDeclaration> externalDeclarations
    externalDeclarations: Option<Vec<Box<dyn ExternalDeclaration>>>,
}

// Java: public class DefaultDocumentType extends AbstractDocumentType
impl DefaultDocumentType {
    // Java: DefaultDocumentType()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: DefaultDocumentType(String elementName, String systemID)
    pub fn new_1() -> Self {
      // Java: super();
      todo!();
      // Java: this.elementName = elementName;
      todo!();
      // Java: this.systemID = systemID;
      todo!();
    }

    // Java: DefaultDocumentType(String elementName, String publicID, String systemID)
    pub fn new_2() -> Self {
      // Java: super();
      todo!();
      // Java: this.elementName = elementName;
      todo!();
      // Java: this.publicID = publicID;
      todo!();
      // Java: this.systemID = systemID;
      todo!();
    }

    // Java: String getElementName()
    pub fn getElementName(&mut self) -> String {
      // Java: return this.elementName;
      todo!();
    }

    // Java: void setElementName(String elementName)
    pub fn setElementName(&mut self) {
      // Java: this.elementName = elementName;
      todo!();
    }

    // Java: String getPublicID()
    pub fn getPublicID(&mut self) -> String {
      // Java: return this.publicID;
      todo!();
    }

    // Java: void setPublicID(String publicID)
    pub fn setPublicID(&mut self) {
      // Java: this.publicID = publicID;
      todo!();
    }

    // Java: String getSystemID()
    pub fn getSystemID(&mut self) -> String {
      // Java: return this.systemID;
      todo!();
    }

    // Java: void setSystemID(String systemID)
    pub fn setSystemID(&mut self) {
      // Java: this.systemID = systemID;
      todo!();
    }

}
