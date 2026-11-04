// Java: import java.util.Map
// Java: import org.dom4j.Element
use crate::org::dom4j::Element;
// Java: import org.dom4j.tree.DefaultProcessingInstruction
use crate::org::dom4j::tree::DefaultProcessingInstruction;
// Java: import org.w3c.dom.DOMException
// use crate::org::w3c::dom::DOMException;
// Java: import org.w3c.dom.Document
// use io_wps_io::cn::wps::io::dom::Document;
// Java: import org.w3c.dom.NamedNodeMap
// use crate::org::w3c::dom::NamedNodeMap;
// Java: import org.w3c.dom.Node
// use io_wps_io::cn::wps::io::dom::Node;
// Java: import org.w3c.dom.NodeList
// use crate::org::w3c::dom::NodeList;
// Java: import org.w3c.dom.ProcessingInstruction
// use io_wps_io::cn::wps::io::dom::ProcessingInstruction;
// Java: import org.w3c.dom.UserDataHandler
// use crate::org::w3c::dom::UserDataHandler;

// Java: public class DOMProcessingInstruction extends DefaultProcessingInstruction implements ProcessingInstruction
pub struct DOMProcessingInstruction {
}

// Java: public class DOMProcessingInstruction extends DefaultProcessingInstruction implements ProcessingInstruction
impl DOMProcessingInstruction {
    // Java: DOMProcessingInstruction(String target, Map values)
    pub fn DOMProcessingInstruction(&self, _target: &str, _values: i32) {
      // Java: super(target, values);
      todo!();
    }

    // Java: DOMProcessingInstruction(String target, String values)
    pub fn DOMProcessingInstruction_2(&self, _target: &str, _values: &str) {
      // Java: super(target, values);
      todo!();
    }

    // Java: DOMProcessingInstruction(Element parent, String target, String val)
    pub fn DOMProcessingInstruction_3(&self, _parent: i32, _target: &str, _val: &str) {
      // Java: super(parent, target, val);
      todo!();
    }

    // Java: boolean supports(String feature, String version)
    pub fn supports(&self, _feature: &str, _version: &str) -> bool {
      // Java: return DOMNodeHelper.supports(this, feature, version);
      todo!();
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> i32 {
      // Java: return DOMNodeHelper.getNamespaceURI(this);
      todo!();
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> i32 {
      // Java: return DOMNodeHelper.getPrefix(this);
      todo!();
    }

    // Java: void setPrefix(String prefix)
    pub fn setPrefix(&self, _prefix: &str) {
      // Java: DOMNodeHelper.setPrefix(this, prefix);
      todo!();
    }

    // Java: String getLocalName()
    pub fn getLocalName(&self) -> i32 {
      // Java: return DOMNodeHelper.getLocalName(this);
      todo!();
    }

    // Java: String getNodeName()
    pub fn getNodeName(&self) -> i32 {
      // Java: return this.getName();
      todo!();
    }

    // Java: String getNodeValue()
    pub fn getNodeValue(&self) -> i32 {
      // Java: return DOMNodeHelper.getNodeValue(this);
      todo!();
    }

    // Java: void setNodeValue(String nodeValue)
    pub fn setNodeValue(&self, _nodeValue: &str) {
      // Java: DOMNodeHelper.setNodeValue(this, nodeValue);
      todo!();
    }

    // Java: Node getParentNode()
    pub fn getParentNode(&self) -> i32 {
      // Java: return DOMNodeHelper.getParentNode(this);
      todo!();
    }

    // Java: NodeList getChildNodes()
    pub fn getChildNodes(&self) -> i32 {
      // Java: return DOMNodeHelper.getChildNodes(this);
      todo!();
    }

    // Java: Node getFirstChild()
    pub fn getFirstChild(&self) -> i32 {
      // Java: return DOMNodeHelper.getFirstChild(this);
      todo!();
    }

    // Java: Node getLastChild()
    pub fn getLastChild(&self) -> i32 {
      // Java: return DOMNodeHelper.getLastChild(this);
      todo!();
    }

    // Java: Node getPreviousSibling()
    pub fn getPreviousSibling(&self) -> i32 {
      // Java: return DOMNodeHelper.getPreviousSibling(this);
      todo!();
    }

    // Java: Node getNextSibling()
    pub fn getNextSibling(&self) -> i32 {
      // Java: return DOMNodeHelper.getNextSibling(this);
      todo!();
    }

    // Java: NamedNodeMap getAttributes()
    pub fn getAttributes(&self) -> i32 {
      // Java: return null;
      todo!();
    }

    // Java: Document getOwnerDocument()
    pub fn getOwnerDocument(&self) -> i32 {
      // Java: return DOMNodeHelper.getOwnerDocument(this);
      todo!();
    }

    // Java: Node insertBefore(Node newChild, Node refChild)
    pub fn insertBefore(&self, _newChild: i32, _refChild: i32) -> i32 {
      // Java: this.checkNewChildNode(newChild);
      // Java: return DOMNodeHelper.insertBefore(this, newChild, refChild);
      todo!();
    }

    // Java: Node replaceChild(Node newChild, Node oldChild)
    pub fn replaceChild(&self, _newChild: i32, _oldChild: i32) -> i32 {
      // Java: this.checkNewChildNode(newChild);
      // Java: return DOMNodeHelper.replaceChild(this, newChild, oldChild);
      todo!();
    }

    // Java: Node removeChild(Node oldChild)
    pub fn removeChild(&self, _oldChild: i32) -> i32 {
      // Java: return DOMNodeHelper.removeChild(this, oldChild);
      todo!();
    }

    // Java: Node appendChild(Node newChild)
    pub fn appendChild(&self, _newChild: i32) -> i32 {
      // Java: this.checkNewChildNode(newChild);
      // Java: return DOMNodeHelper.appendChild(this, newChild);
      todo!();
    }

    // Java: void checkNewChildNode(Node newChild)
    pub fn checkNewChildNode(&self, _newChild: i32) {
      // Java: throw new DOMException((short)3, "PI nodes cannot have children");
      todo!();
    }

    // Java: boolean hasChildNodes()
    pub fn hasChildNodes(&self) -> bool {
      // Java: return DOMNodeHelper.hasChildNodes(this);
      todo!();
    }

    // Java: Node cloneNode(boolean deep)
    pub fn cloneNode(&self, _deep: bool) -> i32 {
      // Java: return DOMNodeHelper.cloneNode(this, deep);
      todo!();
    }

    // Java: void normalize()
    pub fn normalize(&self) {
      // Java: DOMNodeHelper.normalize(this);
      todo!();
    }

    // Java: boolean isSupported(String feature, String version)
    pub fn isSupported(&self, _feature: &str, _version: &str) -> bool {
      // Java: return DOMNodeHelper.isSupported(this, feature, version);
      todo!();
    }

    // Java: boolean hasAttributes()
    pub fn hasAttributes(&self) -> bool {
      // Java: return DOMNodeHelper.hasAttributes(this);
      todo!();
    }

    // Java: String getData()
    pub fn getData(&self) -> i32 {
      // Java: return this.getText();
      todo!();
    }

    // Java: void setData(String data)
    pub fn setData(&self, _data: &str) {
      // Java: if (this.isReadOnly())
      // Java: throw new DOMException((short)7, "This ProcessingInstruction is read only");
      // Java: else
      // Java: this.setText(data);
      todo!();
    }

    // Java: String getBaseURI()
    pub fn getBaseURI(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: short compareDocumentPosition(Node other)
    pub fn compareDocumentPosition(&self, _other: i32) -> i16 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getTextContent()
    pub fn getTextContent(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: void setTextContent(String textContent)
    pub fn setTextContent(&self, _textContent: &str) {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: boolean isSameNode(Node other)
    pub fn isSameNode(&self, _other: i32) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String lookupPrefix(String namespaceURI)
    pub fn lookupPrefix(&self, _namespaceURI: &str) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: boolean isDefaultNamespace(String namespaceURI)
    pub fn isDefaultNamespace(&self, _namespaceURI: &str) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String lookupNamespaceURI(String prefix)
    pub fn lookupNamespaceURI(&self, _prefix: &str) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: boolean isEqualNode(Node arg)
    pub fn isEqualNode(&self, _arg: i32) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: Object getFeature(String feature, String version)
    pub fn getFeature(&self, _feature: &str, _version: &str) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: Object setUserData(String key, Object data, UserDataHandler handler)
    pub fn setUserData(&self, _key: &str, _data: i32, _handler: i32) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: Object getUserData(String key)
    pub fn getUserData(&self, _key: &str) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

}

// impl ProcessingInstruction for DOMProcessingInstruction {