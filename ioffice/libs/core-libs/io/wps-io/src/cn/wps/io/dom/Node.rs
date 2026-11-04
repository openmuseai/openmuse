// Java: public interface Node extends Cloneable
pub trait Node {
    // Java: abstract boolean supportsParent()
    fn supportsParent(&self) -> bool;

    // Java: abstract Element getParent()
    fn getParent(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract void setParent(Element var1)
    fn setParent(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract Document getDocument()
    fn getDocument(&self) -> Option<&dyn std::any::Any>;

    // Java: abstract void setDocument(Document var1)
    fn setDocument(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract boolean isReadOnly()
    fn isReadOnly(&self) -> bool;

    // Java: abstract boolean hasContent()
    fn hasContent(&self) -> bool;

    // Java: abstract String getName()
    fn getName(&self) -> &str;

    // Java: abstract void setName(String var1)
    fn setName(&mut self, var1: String);

    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract void setText(String var1)
    fn setText(&mut self, var1: String);

    // Java: abstract String getStringValue()
    fn getStringValue(&self) -> String;

    // Java: abstract String asXML()
    fn asXML(&self) -> String;

    // Java: abstract NodeType getNodeTypeEnum()
    fn getNodeTypeEnum(&self) -> super::NodeType::NodeType;

    // Java: abstract short getNodeType()
    fn getNodeType(&self) -> i16;

    // Java: abstract String getNodeTypeName()
    fn getNodeTypeName(&self) -> String;

    // Java: abstract boolean detach()
    fn detach(&mut self) -> bool;

    // Java: abstract boolean detachRecursively()
    fn detachRecursively(&mut self) -> bool;

    // Java: abstract Object clone()
    fn clone_node(&self) -> Box<dyn Node>;
}
