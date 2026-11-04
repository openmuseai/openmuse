// Java: import java.util.Map
// (Rust uses std::collections::HashMap)
// Java: import org.xml.sax.EntityResolver
// (not yet available in this crate)

use std::collections::HashMap;

use super::Branch::Branch;
use super::Element::Element;
use super::DocumentType::DocumentType;

// Java: public interface Document extends Branch
pub trait Document: Branch {
    // Java: abstract Element getRootElement()
    fn getRootElement(&self) -> Option<Box<dyn Element>>;

    // Java: abstract void setRootElement(Element var1)
    fn setRootElement(&mut self, var1: Box<dyn Element>);

    // Java: abstract Document addComment(String var1)
    fn addComment(&mut self, var1: &str) -> Box<dyn Document>;

    // Java: abstract Document addProcessingInstruction(String var1, String var2)
    fn addProcessingInstruction(&mut self, var1: &str, var2: &str) -> Box<dyn Document>;

    // Java: abstract Document addProcessingInstruction(String var1, Map<String, String> var2)
    fn addProcessingInstruction_with_map(&mut self, var1: &str, var2: HashMap<String, String>) -> Box<dyn Document>;

    // Java: abstract Document addDocType(String var1, String var2, String var3)
    fn addDocType(&mut self, var1: &str, var2: &str, var3: &str) -> Box<dyn Document>;

    // Java: abstract DocumentType getDocType()
    fn getDocType(&self) -> Option<Box<dyn DocumentType>>;

    // Java: abstract void setDocType(DocumentType var1)
    fn setDocType(&mut self, var1: Box<dyn DocumentType>);

    // Java: abstract EntityResolver getEntityResolver()
    // EntityResolver is from org.xml.sax, using a trait object placeholder
    fn getEntityResolver(&self) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract void setEntityResolver(EntityResolver var1)
    fn setEntityResolver(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract String getXMLEncoding()
    fn getXMLEncoding(&self) -> Option<String>;

    // Java: abstract void setXMLEncoding(String var1)
    fn setXMLEncoding(&mut self, var1: &str);
}
