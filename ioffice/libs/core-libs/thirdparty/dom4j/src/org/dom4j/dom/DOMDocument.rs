// Java: import java.util.ArrayList
// use std::collections::ArrayList;
// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.DocumentType
// use crate::org::dom4j::DocumentType;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.tree.DefaultDocument
// use crate::org::dom4j::tree::DefaultDocument;
// Java: import org.w3c.dom.Attr
// use crate::org::w3c::dom::Attr;
// Java: import org.w3c.dom.CDATASection
// use crate::org::w3c::dom::CDATASection;
// Java: import org.w3c.dom.Comment
// use io_wps_io::cn::wps::io::dom::Comment;
// Java: import org.w3c.dom.DOMConfiguration
// use crate::org::w3c::dom::DOMConfiguration;
// Java: import org.w3c.dom.DOMException
// use crate::org::w3c::dom::DOMException;
// Java: import org.w3c.dom.DOMImplementation
// use crate::org::w3c::dom::DOMImplementation;
// Java: import org.w3c.dom.Document
// use io_wps_io::cn::wps::io::dom::Document;
// Java: import org.w3c.dom.DocumentFragment
// use crate::org::w3c::dom::DocumentFragment;
// Java: import org.w3c.dom.EntityReference
// use crate::org::w3c::dom::EntityReference;
// Java: import org.w3c.dom.NamedNodeMap
// use crate::org::w3c::dom::NamedNodeMap;
// Java: import org.w3c.dom.Node
// use io_wps_io::cn::wps::io::dom::Node;
// Java: import org.w3c.dom.NodeList
// use crate::org::w3c::dom::NodeList;
// Java: import org.w3c.dom.ProcessingInstruction
// use io_wps_io::cn::wps::io::dom::ProcessingInstruction;
// Java: import org.w3c.dom.Text
// use io_wps_io::cn::wps::io::dom::Text;
// Java: import org.w3c.dom.UserDataHandler
// use crate::org::w3c::dom::UserDataHandler;

// Java field: DOMDocumentFactory DOCUMENT_FACTORY
// static DOCUMENT_FACTORY: i32 = 0;

// Java: public class DOMDocument extends DefaultDocument implements Document
pub struct DOMDocument {
}

// Java: public class DOMDocument extends DefaultDocument implements Document
impl DOMDocument {
    // Java: DOMDocument()
    pub fn new() -> i32 {
      // Java: super();
      // Java: this.init();
      todo!();
    }

    // Java: DOMDocument(String name)
    pub fn new_with_name(_name: &str) -> i32 {
      // Java: super(name);
      // Java: this.init();
      todo!();
    }

    // Java: DOMDocument(DOMElement rootElement)
    pub fn new_with_root(_rootElement: i32) -> i32 {
      // Java: super((Element)rootElement);
      // Java: this.init();
      todo!();
    }

    // Java: DOMDocument(DOMDocumentType docType)
    pub fn new_with_doctype(_docType: i32) -> i32 {
      // Java: super((DocumentType)docType);
      // Java: this.init();
      todo!();
    }

    // Java: DOMDocument(DOMElement rootElement, DOMDocumentType docType)
    pub fn new_with_root_doctype(_rootElement: i32, _docType: i32) -> i32 {
      // Java: super(rootElement, docType);
      // Java: this.init();
      todo!();
    }

    // Java: DOMDocument(String name, DOMElement rootElement, DOMDocumentType docType)
    pub fn new_with_name_root_doctype(_name: &str, _rootElement: i32, _docType: i32) -> i32 {
      // Java: super(name, rootElement, docType);
      // Java: this.init();
      todo!();
    }

    // Java: void init()
    pub fn init(&self) {
      // Java: this.setDocumentFactory(DOCUMENT_FACTORY);
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
      // Java: return "#document";
      todo!();
    }

    // Java: String getNodeValue()
    pub fn getNodeValue(&self) -> i32 {
      // Java: return null;
      todo!();
    }

    // Java: void setNodeValue(String nodeValue)
    pub fn setNodeValue(&self, _nodeValue: &str) {
      todo!();
    }

    // Java: Node getParentNode()
    pub fn getParentNode(&self) -> i32 {
      // Java: return DOMNodeHelper.getParentNode(this);
      todo!();
    }

    // Java: NodeList getChildNodes()
    pub fn getChildNodes(&self) -> i32 {
      // Java: return DOMNodeHelper.createNodeList(this.content());
      todo!();
    }

    // Java: Node getFirstChild()
    pub fn getFirstChild(&self) -> i32 {
      // Java: return DOMNodeHelper.asDOMNode(this.node(0));
      todo!();
    }

    // Java: Node getLastChild()
    pub fn getLastChild(&self) -> i32 {
      // Java: return DOMNodeHelper.asDOMNode(this.node(this.nodeCount() - 1));
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
      // Java: return null;
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
      // Java: int nodeType = newChild.getNodeType();
      // Java: if (nodeType != 1 && nodeType != 8 && nodeType != 7 && nodeType != 10)
      // Java: throw new DOMException((short)3, "Given node cannot be a child of document");
      todo!();
    }

    // Java: boolean hasChildNodes()
    pub fn hasChildNodes(&self) -> bool {
      // Java: return this.nodeCount() > 0;
      todo!();
    }

    // Java: Node cloneNode(boolean deep)
    pub fn cloneNode(&self, _deep: bool) -> i32 {
      // Java: return DOMNodeHelper.cloneNode(this, deep);
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

    // Java: NodeList getElementsByTagName(String name)
    pub fn getElementsByTagName(&self, _name: &str) -> i32 {
      // Java: ArrayList list = new ArrayList();
      // Java: DOMNodeHelper.appendElementsByTagName(list, this, name);
      // Java: return DOMNodeHelper.createNodeList(list);
      todo!();
    }

    // Java: NodeList getElementsByTagNameNS(String namespace, String name)
    pub fn getElementsByTagNameNS(&self, _namespace: &str, _name: &str) -> i32 {
      // Java: ArrayList list = new ArrayList();
      // Java: DOMNodeHelper.appendElementsByTagNameNS(list, this, namespace, name);
      // Java: return DOMNodeHelper.createNodeList(list);
      todo!();
    }

    // Java: DOMImplementation getImplementation()
    pub fn getImplementation(&self) -> i32 {
      // Java: return (DOMImplementation)(this.getDocumentFactory() instanceof DOMImplementation ? (DOMImplementation)this.getDocumentFactory() : DOCUMENT_FACTORY);
      todo!();
    }

    // Java: DocumentFragment createDocumentFragment()
    pub fn createDocumentFragment(&self) -> i32 {
      // Java: DOMNodeHelper.notSupported();
      // Java: return null;
      todo!();
    }

    // Java: Text createTextNode(String data)
    pub fn createTextNode(&self, _data: &str) -> i32 {
      // Java: return (Text)this.getDocumentFactory().createText(data);
      todo!();
    }

    // Java: Comment createComment(String data)
    pub fn createComment(&self, _data: &str) -> i32 {
      // Java: return (Comment)this.getDocumentFactory().createComment(data);
      todo!();
    }

    // Java: CDATASection createCDATASection(String data)
    pub fn createCDATASection(&self, _data: &str) -> i32 {
      // Java: return (CDATASection)this.getDocumentFactory().createCDATA(data);
      todo!();
    }

    // Java: ProcessingInstruction createProcessingInstruction(String target, String data)
    pub fn createProcessingInstruction(&self, _target: &str, _data: &str) -> i32 {
      // Java: return (ProcessingInstruction)this.getDocumentFactory().createProcessingInstruction(target, data);
      todo!();
    }

    // Java: Attr createAttribute(String name)
    pub fn createAttribute(&self, _name: &str) -> i32 {
      // Java: QName qname = this.getDocumentFactory().createQName(name);
      // Java: return (Attr)this.getDocumentFactory().createAttribute((Element)null, (QName)qname, "");
      todo!();
    }

    // Java: EntityReference createEntityReference(String name)
    pub fn createEntityReference(&self, _name: &str) -> i32 {
      // Java: return (EntityReference)this.getDocumentFactory().createEntity(name, (String)null);
      todo!();
    }

    // Java: Node importNode(Node importedNode, boolean deep)
    pub fn importNode(&self, _importedNode: i32, _deep: bool) -> i32 {
      // Java: DOMNodeHelper.notSupported();
      // Java: return null;
      todo!();
    }

    // Java: Attr createAttributeNS(String namespaceURI, String qualifiedName)
    pub fn createAttributeNS(&self, _namespaceURI: &str, _qualifiedName: &str) -> i32 {
      // Java: QName qname = this.getDocumentFactory().createQName(qualifiedName, namespaceURI);
      // Java: return (Attr)this.getDocumentFactory().createAttribute((Element)null, (QName)qname, (String)null);
      todo!();
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) -> i32 {
      // Java: return (DocumentFactory)(super.getDocumentFactory() == null ? DOCUMENT_FACTORY : super.getDocumentFactory());
      todo!();
    }

    // Java: String getInputEncoding()
    pub fn getInputEncoding(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getXmlEncoding()
    pub fn getXmlEncoding(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: boolean getXmlStandalone()
    pub fn getXmlStandalone(&self) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: void setXmlStandalone(boolean xmlStandalone)
    pub fn setXmlStandalone(&self, _xmlStandalone: bool) {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getXmlVersion()
    pub fn getXmlVersion(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: void setXmlVersion(String xmlVersion)
    pub fn setXmlVersion(&self, _xmlVersion: &str) {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: boolean getStrictErrorChecking()
    pub fn getStrictErrorChecking(&self) -> bool {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: void setStrictErrorChecking(boolean strictErrorChecking)
    pub fn setStrictErrorChecking(&self, _strictErrorChecking: bool) {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: String getDocumentURI()
    pub fn getDocumentURI(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: void setDocumentURI(String documentURI)
    pub fn setDocumentURI(&self, _documentURI: &str) {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: Node adoptNode(Node source)
    pub fn adoptNode(&self, _source: i32) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: DOMConfiguration getDomConfig()
    pub fn getDomConfig(&self) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: void normalizeDocument()
    pub fn normalizeDocument(&self) {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
      todo!();
    }

    // Java: Node renameNode(Node n, String namespaceURI, String qualifiedName)
    pub fn renameNode(&self, _n: i32, _namespaceURI: &str, _qualifiedName: &str) -> i32 {
      // Java: throw new UnsupportedOperationException("Not supported yet.");
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

// impl Document for DOMDocument { // }