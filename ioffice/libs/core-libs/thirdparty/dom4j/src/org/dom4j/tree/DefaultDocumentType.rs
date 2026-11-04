// Java: import java.util.List
// Java: import org.dom4j.dtd.ExternalDeclaration
// Java: import org.dom4j.dtd.InternalDeclaration

// Java: public class DefaultDocumentType extends AbstractDocumentType
pub struct DefaultDocumentType {
    // Java field: String elementName
    pub(crate) elementName: String,
    // Java field: String publicID
    pub(crate) publicID: String,
    // Java field: String systemID
    pub(crate) systemID: String,
    // Java field: List<InternalDeclaration> internalDeclarations
    pub(crate) internalDeclarations: Vec<String>,
    // Java field: List<ExternalDeclaration> externalDeclarations
    pub(crate) externalDeclarations: Vec<String>,
}

// Java: public class DefaultDocumentType extends AbstractDocumentType
impl DefaultDocumentType {
    // Java: DefaultDocumentType()
    pub fn new() -> Self {
        // Java: super();
        DefaultDocumentType {
            elementName: String::new(),
            publicID: String::new(),
            systemID: String::new(),
            internalDeclarations: Vec::new(),
            externalDeclarations: Vec::new(),
        }
    }

    // Java: DefaultDocumentType(String elementName, String systemID)
    pub fn new_with_system(element_name: String, system_id: String) -> Self {
        // Java: super();
        // Java: this.elementName = elementName;
        // Java: this.systemID = systemID;
        DefaultDocumentType {
            elementName: element_name,
            publicID: String::new(),
            systemID: system_id,
            internalDeclarations: Vec::new(),
            externalDeclarations: Vec::new(),
        }
    }

    // Java: DefaultDocumentType(String elementName, String publicID, String systemID)
    pub fn new_with_public(element_name: String, public_id: String, system_id: String) -> Self {
        // Java: super();
        // Java: this.elementName = elementName;
        // Java: this.publicID = publicID;
        // Java: this.systemID = systemID;
        DefaultDocumentType {
            elementName: element_name,
            publicID: public_id,
            systemID: system_id,
            internalDeclarations: Vec::new(),
            externalDeclarations: Vec::new(),
        }
    }

    // Java: String getElementName()
    pub fn getElementName(&self) -> String {
        // Java: return this.elementName;
        self.elementName.clone()
    }

    // Java: void setElementName(String elementName)
    pub fn setElementName(&mut self, element_name: String) {
        // Java: this.elementName = elementName;
        self.elementName = element_name;
    }

    // Java: String getPublicID()
    pub fn getPublicID(&self) -> String {
        // Java: return this.publicID;
        self.publicID.clone()
    }

    // Java: void setPublicID(String publicID)
    pub fn setPublicID(&mut self, public_id: String) {
        // Java: this.publicID = publicID;
        self.publicID = public_id;
    }

    // Java: String getSystemID()
    pub fn getSystemID(&self) -> String {
        // Java: return this.systemID;
        self.systemID.clone()
    }

    // Java: void setSystemID(String systemID)
    pub fn setSystemID(&mut self, system_id: String) {
        // Java: this.systemID = systemID;
        self.systemID = system_id;
    }

    // Java: List<InternalDeclaration> getInternalDeclarations()
    pub fn getInternalDeclarations(&self) -> Vec<String> {
        // Java: return this.internalDeclarations;
        self.internalDeclarations.clone()
    }

    // Java: void setInternalDeclarations(List<InternalDeclaration> internalDeclarations)
    pub fn setInternalDeclarations(&mut self, internal_declarations: Vec<String>) {
        // Java: this.internalDeclarations = internalDeclarations;
        self.internalDeclarations = internal_declarations;
    }

    // Java: List<ExternalDeclaration> getExternalDeclarations()
    pub fn getExternalDeclarations(&self) -> Vec<String> {
        // Java: return this.externalDeclarations;
        self.externalDeclarations.clone()
    }

    // Java: void setExternalDeclarations(List<ExternalDeclaration> externalDeclarations)
    pub fn setExternalDeclarations(&mut self, external_declarations: Vec<String>) {
        // Java: this.externalDeclarations = externalDeclarations;
        self.externalDeclarations = external_declarations;
    }
}
