// Java: import cn.wps.io.dom.NodeType
// Java: import cn.wps.io.dom.Text

// Java: public class AbstractText extends AbstractCharacterData implements Text
pub struct AbstractText {
}

// Java: public class AbstractText extends AbstractCharacterData implements Text
impl AbstractText {
    // Java: AbstractText()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&mut self) -> Option<super::super::NodeType::NodeType> {
      // Java: return NodeType.TEXT_NODE;
      todo!();
    }

    // Java: String toString()
    pub fn toString(&mut self) -> String {
      // Java: return super.toString() + " [Text: \"" + this.getText() + "\"]";
      todo!();
    }

    // Java: String asXML()
    pub fn asXML(&mut self) -> String {
      // Java: return this.getText();
      todo!();
    }

}
