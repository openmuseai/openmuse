// use std::sync::{Arc, Mutex};
// use crate::org::dom4j::dom::DOMElement::DOMElement;
// Java: import org.w3c.dom.Attr
// use crate::org::w3c::dom::Attr;
// Java: import org.w3c.dom.DOMException
// use crate::org::w3c::dom::DOMException;
// Java: import org.w3c.dom.NamedNodeMap
// use crate::org::w3c::dom::NamedNodeMap;
// Java: import org.w3c.dom.Node
// use io_wps_io::cn::wps::io::dom::Node;

// Java: public class DOMAttributeNodeMap implements NamedNodeMap
pub struct DOMAttributeNodeMap {
    // Java field: DOMElement element
    pub(crate) element: i32,
}

// Java: public class DOMAttributeNodeMap implements NamedNodeMap
impl DOMAttributeNodeMap {
    // Java: DOMAttributeNodeMap(DOMElement element)
    pub fn new(_element: i32) -> Self {
      // Java: super();
      // Java: this.element = element;
      Self { element: _element }
    }

    // Java: void foo()
    pub fn foo(&self) {
      // Java: DOMNodeHelper.notSupported();
    }

    // Java: Node getNamedItem(String name)
    pub fn getNamedItem(&self, _name: &str) -> i32 {
      // Java: return this.element.getAttributeNode(name);
      0
    }

    // Java: Node setNamedItem(Node arg)
    pub fn setNamedItem(&self, _arg: i32) -> i32 {
      // Java: if (arg instanceof Attr)
      // Java: return this.element.setAttributeNode((Attr)arg);
      // Java: else
      // Java: throw new DOMException((short)9, "Node is not an Attr: " + arg);
      0
    }

    // Java: Node removeNamedItem(String name)
    pub fn removeNamedItem(&self, _name: &str) -> i32 {
      // Java: Attr attr = this.element.getAttributeNode(name);
      // Java: if (attr == null)
      // Java: throw new DOMException((short)8, "No attribute named " + name);
      // Java: else
      // Java: return this.element.removeAttributeNode(attr);
      0
    }

    // Java: Node item(int index)
    pub fn item(&self, _index: i32) -> i32 {
      // Java: return DOMNodeHelper.asDOMAttr(this.element.attribute(index));
      0
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
      // Java: return this.element.attributeCount();
      0
    }

    // Java: Node getNamedItemNS(String namespaceURI, String localName)
    pub fn getNamedItemNS(&self, _namespaceURI: &str, _localName: &str) -> i32 {
      // Java: return this.element.getAttributeNodeNS(namespaceURI, localName);
      0
    }

    // Java: Node setNamedItemNS(Node arg)
    pub fn setNamedItemNS(&self, _arg: i32) -> i32 {
      // Java: if (arg instanceof Attr)
      // Java: return this.element.setAttributeNodeNS((Attr)arg);
      // Java: else
      // Java: throw new DOMException((short)9, "Node is not an Attr: " + arg);
      0
    }

    // Java: Node removeNamedItemNS(String namespaceURI, String localName)
    pub fn removeNamedItemNS(&self, _namespaceURI: &str, _localName: &str) -> i32 {
      // Java: Attr attr = this.element.getAttributeNodeNS(namespaceURI, localName);
      // Java: return attr != null ? this.element.removeAttributeNode(attr) : attr;
      0
    }

}

// impl NamedNodeMap for DOMAttributeNodeMap { // }
