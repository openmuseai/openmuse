// Java: public interface Document extends Branch
pub trait Document {
    // Java: abstract Element getRootElement()
    fn getRootElement(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract void setRootElement(Element var1)
    fn setRootElement(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract Document addComment(String var1)
    fn addComment(&mut self, var1: &str);

    // Java: abstract Document addProcessingInstruction(String var1, String var2)
    fn addProcessingInstruction(&mut self, var1: &str, var2: &str);

    // Java: abstract Document addProcessingInstruction(String var1, Map<String, String> var2)
    fn addProcessingInstruction_with_map(&mut self, var1: &str, var2: std::collections::HashMap<String, String>);

    // Java: abstract Document addDocType(String var1, String var2, String var3)
    fn addDocType(&mut self, var1: &str, var2: &str, var3: &str);

    // Java: abstract DocumentType getDocType()
    fn getDocType(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract void setDocType(DocumentType var1)
    fn setDocType(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract EntityResolver getEntityResolver()
    fn getEntityResolver(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract void setEntityResolver(EntityResolver var1)
    fn setEntityResolver(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract String getXMLEncoding()
    fn getXMLEncoding(&self) -> Option<&str>;

    // Java: abstract void setXMLEncoding(String var1)
    fn setXMLEncoding(&mut self, var1: String);
}
