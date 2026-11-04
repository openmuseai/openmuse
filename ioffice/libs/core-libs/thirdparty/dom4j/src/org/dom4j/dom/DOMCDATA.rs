// Java: import org.dom4j.CDATA
use crate::org::dom4j::CDATA;
// Java: import org.dom4j.Element
use crate::org::dom4j::Element;
// Java: import org.dom4j.tree.DefaultCDATA
use crate::org::dom4j::tree::DefaultCDATA;
// Java: import org.w3c.dom.CDATASection
// use crate::org::w3c::dom::CDATASection;
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
// Java: import org.w3c.dom.Text
// use io_wps_io::cn::wps::io::dom::Text;
// Java: import org.w3c.dom.UserDataHandler
// use crate::org::w3c::dom::UserDataHandler;

// Java: public class DOMCDATA extends DefaultCDATA implements CDATASection
pub struct DOMCDATA {
}

// Java: public class DOMCDATA extends DefaultCDATA implements CDATASection
impl DOMCDATA {
    // Java: DOMCDATA(String text)
    pub fn DOMCDATA(&self, _text: &str) {
      // Java: super(text);
      todo!();
    }

    // Java: DOMCDATA(Element parent, String text)
    pub fn DOMCDATA_2(&self, _parent: i32, _text: &str) {
      // Java: super(parent, text);
      todo!();
    }

    // Java: boolean supports(String feature, String version)
    pub fn supports(&self, _feature: &str, _version: &str) -> bool {
      // Java: return DOMNodeHelper.supports(this, feature, version);
      todo!();
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> String {
      // Java: return DOMNodeHelper.getNamespaceURI(this);
      todo!();
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> String {
      // Java: return DOMNodeHelper.getPrefix(this);
      todo!();
    }

    // Java: void setPrefix(String prefix)
    pub fn setPrefix(&self, _prefix: &str) {
      // Java: DOMNodeHelper.setPrefix(this, prefix);
      todo!();
    }

    // Java: String getLocalName()
    pub fn getLocalName(&self) -> String {
      // Java: return DOMNodeHelper.getLocalName(this);
      todo!();
    }

    // Java: String getNodeName()
    pub fn getNodeName(&self) -> String {
      // Java: return "#cdata-section";
      todo!();
    }

    // Java: String getNodeValue()
    pub fn getNodeValue(&self) -> String {
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
      // Java: throw new DOMException((short)3, "CDATASection nodes cannot have children");
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
    pub fn getData(&self) -> String {
      // Java: return DOMNodeHelper.getData(this);
      todo!();
    }

    // Java: void setData(String data)
    pub fn setData(&self, _data: &str) {
      // Java: DOMNodeHelper.setData(this, data);
      todo!();
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
      // Java: return DOMNodeHelper.getLength(this);
      todo!();
    }

    // Java: String substringData(int offset, int count)
    pub fn substringData(&self, _offset: i32, _count: i32) -> String {
      // Java: return DOMNodeHelper.substringData(this, offset, count);
      todo!();
    }

    // Java: void appendData(String arg)
    pub fn appendData(&self, _arg: &str) {
      // Java: DOMNodeHelper.appendData(this, arg);
      todo!();
    }

    // Java: void insertData(int offset, String arg)
    pub fn insertData(&self, _offset: i32, _arg: &str) {
      // Java: DOMNodeHelper.insertData(this, offset, arg);
      todo!();
    }

    // Java: void deleteData(int offset, int count)
    pub fn deleteData(&self, _offset: i32, _count: i32) {
      // Java: DOMNodeHelper.deleteData(this, offset, count);
      todo!();
    }

    // Java: void replaceData(int offset, int count, String arg)
    pub fn replaceData(&self, _offset: i32, _count: i32, _arg: &str) {
      // Java: DOMNodeHelper.replaceData(this, offset, count, arg);
      todo!();
    }

    // Java: Text splitText(int offset)
    pub fn splitText(&self, _offset: i32) -> i32 {
      // Java: if (this.isReadOnly())
      // Java: throw new DOMException((short)7, "CharacterData node is read only: " + this);
      // Java: else
      // Java: String text = this.getText();
      // Java: int length = text != null ? text.length() : 0;
      // Java: if (offset >= 0 && offset < length)
      // Java: String start = text.substring(0, offset);
      // Java: String rest = text.substring(offset);
      // Java: this.setText(start);
      // Java: Element parent = this.getParent();
      // Java: CDATA newText = this.createCDATA(rest);
      // Java: if (parent != null)
      // Java: parent.add(newText);
      // Java: return DOMNodeHelper.asDOMText(newText);
      // Java: else
      // Java: throw new DOMException((short)1, "No text at offset: " + offset);
      todo!();
    }

    // Java: CDATA createCDATA(String text)
    pub fn createCDATA(&self, _text: &str) -> i32 {
      // Java: return new DOMCDATA(text);
      todo!();
    }

    // Java: boolean isElementContentWhitespace()
    pub fn isElementContentWhitespace(&self) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getWholeText()
    pub fn getWholeText(&self) -> String {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: Text replaceWholeText(String content)
    pub fn replaceWholeText(&self, _content: &str) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getBaseURI()
    pub fn getBaseURI(&self) -> String {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: short compareDocumentPosition(Node other)
    pub fn compareDocumentPosition(&self, _other: i32) -> i16 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getTextContent()
    pub fn getTextContent(&self) -> String {
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
    pub fn lookupPrefix(&self, _namespaceURI: &str) -> String {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: boolean isDefaultNamespace(String namespaceURI)
    pub fn isDefaultNamespace(&self, _namespaceURI: &str) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String lookupNamespaceURI(String prefix)
    pub fn lookupNamespaceURI(&self, _prefix: &str) -> String {
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

// impl CDATASection for DOMCDATA {