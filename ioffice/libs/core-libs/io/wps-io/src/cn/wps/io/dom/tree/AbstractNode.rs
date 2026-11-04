// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.Node

// Java: public abstract class AbstractNode implements Node, Cloneable, Serializable
pub struct AbstractNode {
}

// Java: public abstract class AbstractNode implements Node, Cloneable, Serializable
impl AbstractNode {
    // Java: AbstractNode()
    pub fn new() -> Self {
        AbstractNode {}
    }

    // Java: short getNodeType()
    pub fn getNodeType(&self) -> i16 {
        todo!()
    }

    // Java: String getNodeTypeName()
    pub fn getNodeTypeName(&self) -> String {
        todo!()
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> Option<String> {
        todo!()
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&mut self, document: Option<String>) {
        todo!()
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> Option<String> {
        None
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&mut self, parent: Option<String>) {
        todo!()
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
        false
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
        true
    }

    // Java: boolean hasContent()
    pub fn hasContent(&self) -> bool {
        false
    }

    // Java: AbstractNode clone()
    pub fn clone_node(&self) -> Self {
        todo!()
    }

    // Java: boolean detach()
    pub fn detach(&mut self) -> bool {
        todo!()
    }

    // Java: boolean detachRecursively()
    pub fn detachRecursively(&mut self) -> bool {
        todo!()
    }

    // Java: String getName()
    pub fn getName(&self) -> Option<String> {
        None
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: &str) {
        panic!("This node cannot be modified");
    }

    // Java: String getText()
    pub fn getText(&self) -> Option<String> {
        None
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> Option<String> {
        self.getText()
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: &str) {
        panic!("This node cannot be modified");
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        String::from("AbstractNode")
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString_builder(&self, builder: &mut String) {
        builder.push_str("AbstractNode");
    }
}
