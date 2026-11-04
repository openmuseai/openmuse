// Java: import java.util.Iterator
// Java: import org.jaxen.util.AncestorAxisIterator
use crate::org::jaxen::util::AncestorAxisIterator::AncestorAxisIterator;
// Java: import org.jaxen.util.AncestorOrSelfAxisIterator
use crate::org::jaxen::util::AncestorOrSelfAxisIterator::AncestorOrSelfAxisIterator;
// Java: import org.jaxen.util.DescendantAxisIterator
use crate::org::jaxen::util::DescendantAxisIterator::DescendantAxisIterator;
// Java: import org.jaxen.util.DescendantOrSelfAxisIterator
use crate::org::jaxen::util::DescendantOrSelfAxisIterator::DescendantOrSelfAxisIterator;
// Java: import org.jaxen.util.FollowingAxisIterator
use crate::org::jaxen::util::FollowingAxisIterator::FollowingAxisIterator;
// Java: import org.jaxen.util.FollowingSiblingAxisIterator
use crate::org::jaxen::util::FollowingSiblingAxisIterator::FollowingSiblingAxisIterator;
// Java: import org.jaxen.util.PrecedingAxisIterator
use crate::org::jaxen::util::PrecedingAxisIterator::PrecedingAxisIterator;
// Java: import org.jaxen.util.PrecedingSiblingAxisIterator
use crate::org::jaxen::util::PrecedingSiblingAxisIterator::PrecedingSiblingAxisIterator;
// Java: import org.jaxen.util.SelfAxisIterator
use crate::org::jaxen::util::SelfAxisIterator::SelfAxisIterator;

use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::{Object, NodeIterator};

// Java: public class DefaultNavigator implements Navigator
pub struct DefaultNavigator {
}

// Java: public class DefaultNavigator implements Navigator
impl DefaultNavigator {
    // Java: DefaultNavigator()
    pub fn new() -> Self {
      // Java: super();
      DefaultNavigator {}
    }

    // Java: Iterator getChildAxisIterator(Object contextNode)
    pub fn getChildAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: throw new UnsupportedAxisException("child");
      Err(UnsupportedAxisException::new("child".to_string()))
    }

    // Java: Iterator getDescendantAxisIterator(Object contextNode)
    pub fn getDescendantAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new DescendantAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("descendant".to_string()))
    }

    // Java: Iterator getParentAxisIterator(Object contextNode)
    pub fn getParentAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: throw new UnsupportedAxisException("parent");
      Err(UnsupportedAxisException::new("parent".to_string()))
    }

    // Java: Iterator getAncestorAxisIterator(Object contextNode)
    pub fn getAncestorAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new AncestorAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("ancestor".to_string()))
    }

    // Java: Iterator getFollowingSiblingAxisIterator(Object contextNode)
    pub fn getFollowingSiblingAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new FollowingSiblingAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("following-sibling".to_string()))
    }

    // Java: Iterator getPrecedingSiblingAxisIterator(Object contextNode)
    pub fn getPrecedingSiblingAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new PrecedingSiblingAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("preceding-sibling".to_string()))
    }

    // Java: Iterator getFollowingAxisIterator(Object contextNode)
    pub fn getFollowingAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new FollowingAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("following".to_string()))
    }

    // Java: Iterator getPrecedingAxisIterator(Object contextNode)
    pub fn getPrecedingAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new PrecedingAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("preceding".to_string()))
    }

    // Java: Iterator getAttributeAxisIterator(Object contextNode)
    pub fn getAttributeAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: throw new UnsupportedAxisException("attribute");
      Err(UnsupportedAxisException::new("attribute".to_string()))
    }

    // Java: Iterator getNamespaceAxisIterator(Object contextNode)
    pub fn getNamespaceAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: throw new UnsupportedAxisException("namespace");
      Err(UnsupportedAxisException::new("namespace".to_string()))
    }

    // Java: Iterator getSelfAxisIterator(Object contextNode)
    pub fn getSelfAxisIterator(&self, contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new SelfAxisIterator(contextNode);
      Ok(Box::new(SelfAxisIterator::new(contextNode.clone())))
    }

    // Java: Iterator getDescendantOrSelfAxisIterator(Object contextNode)
    pub fn getDescendantOrSelfAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new DescendantOrSelfAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("descendant-or-self".to_string()))
    }

    // Java: Iterator getAncestorOrSelfAxisIterator(Object contextNode)
    pub fn getAncestorOrSelfAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return new AncestorOrSelfAxisIterator(contextNode, this);
      Err(UnsupportedAxisException::new("ancestor-or-self".to_string()))
    }

    // Java: Object getDocumentNode(Object contextNode)
    pub fn getDocumentNode(&self, _contextNode: &Object) -> Object {
      // Java: return null;
      std::sync::Arc::new(())
    }

    // Java: String translateNamespacePrefixToUri(String prefix, Object element)
    pub fn translateNamespacePrefixToUri(&self, _prefix: &str, _element: &Object) -> String {
      // Java: return null;
      String::new()
    }

    // Java: String getProcessingInstructionTarget(Object obj)
    pub fn getProcessingInstructionTarget(&self, _obj: &Object) -> String {
      // Java: return null;
      String::new()
    }

    // Java: String getProcessingInstructionData(Object obj)
    pub fn getProcessingInstructionData(&self, _obj: &Object) -> String {
      // Java: return null;
      String::new()
    }

    // Java: short getNodeType(Object node)
    pub fn getNodeType(&self, node: &Object) -> i16 {
      // Java: if (this.isElement(node))
      if self.isElement(node) {
         // Java: return 1;
         return 1;
      }
      // Java: else if (this.isAttribute(node))
      else if self.isAttribute(node) {
         // Java: return 2;
         return 2;
      }
      // Java: else if (this.isText(node))
      else if self.isText(node) {
         // Java: return 3;
         return 3;
      }
      // Java: else if (this.isComment(node))
      else if self.isComment(node) {
         // Java: return 8;
         return 8;
      }
      // Java: else if (this.isDocument(node))
      else if self.isDocument(node) {
         // Java: return 9;
         return 9;
      }
      // Java: else if (this.isProcessingInstruction(node))
      else if self.isProcessingInstruction(node) {
         // Java: return 7;
         return 7;
      }
      // Java: else
      else {
         // Java: return (short)(this.isNamespace(node) ? 13 : 14);
         if self.isNamespace(node) { 13 } else { 14 }
      }
    }

    // Java: Object getParentNode(Object contextNode)
    pub fn getParentNode(&self, contextNode: &Object) -> Result<Object, UnsupportedAxisException> {
      // Java: Iterator iter = this.getParentAxisIterator(contextNode);
      let mut iter = self.getParentAxisIterator(contextNode)?;
      // Java: return iter != null && iter.hasNext() ? iter.next() : null;
      match iter.next() {
          Some(obj) => Ok(obj),
          None => Err(UnsupportedAxisException::new("no parent".to_string())),
      }
    }

    // Java: Object getDocument(String url)
    pub fn getDocument(&self, _url: &str) -> Result<Object, FunctionCallException> {
      // Java: return null;
      Err(FunctionCallException::new("not supported".to_string()))
    }

    // Java: Object getElementById(Object contextNode, String elementId)
    pub fn getElementById(&self, _contextNode: &Object, _elementId: &str) -> Object {
      // Java: return null;
      std::sync::Arc::new(())
    }

    fn isElement(&self, _node: &Object) -> bool { false }
    fn isAttribute(&self, _node: &Object) -> bool { false }
    fn isText(&self, _node: &Object) -> bool { false }
    fn isComment(&self, _node: &Object) -> bool { false }
    fn isDocument(&self, _node: &Object) -> bool { false }
    fn isProcessingInstruction(&self, _node: &Object) -> bool { false }
    fn isNamespace(&self, _node: &Object) -> bool { false }
}

impl Navigator for DefaultNavigator {
    fn getChildAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getChildAxisIterator(self, c) }
    fn getDescendantAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getDescendantAxisIterator(self, c) }
    fn getParentAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getParentAxisIterator(self, c) }
    fn getAncestorAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getAncestorAxisIterator(self, c) }
    fn getFollowingSiblingAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getFollowingSiblingAxisIterator(self, c) }
    fn getPrecedingSiblingAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getPrecedingSiblingAxisIterator(self, c) }
    fn getFollowingAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getFollowingAxisIterator(self, c) }
    fn getPrecedingAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getPrecedingAxisIterator(self, c) }
    fn getAttributeAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getAttributeAxisIterator(self, c) }
    fn getNamespaceAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getNamespaceAxisIterator(self, c) }
    fn getSelfAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getSelfAxisIterator(self, c) }
    fn getDescendantOrSelfAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getDescendantOrSelfAxisIterator(self, c) }
    fn getAncestorOrSelfAxisIterator(&self, c: &Object) -> Result<NodeIterator, UnsupportedAxisException> { DefaultNavigator::getAncestorOrSelfAxisIterator(self, c) }
    fn getDocumentNode(&self, c: &Object) -> Object { DefaultNavigator::getDocumentNode(self, c) }
    fn translateNamespacePrefixToUri(&self, p: &str, e: &Object) -> String { DefaultNavigator::translateNamespacePrefixToUri(self, p, e) }
    fn getProcessingInstructionTarget(&self, o: &Object) -> String { DefaultNavigator::getProcessingInstructionTarget(self, o) }
    fn getProcessingInstructionData(&self, o: &Object) -> String { DefaultNavigator::getProcessingInstructionData(self, o) }
    fn getNodeType(&self, n: &Object) -> i16 { DefaultNavigator::getNodeType(self, n) }
    fn getParentNode(&self, c: &Object) -> Result<Object, UnsupportedAxisException> { DefaultNavigator::getParentNode(self, c) }
    fn getDocument(&self, u: &str) -> Result<Object, FunctionCallException> { DefaultNavigator::getDocument(self, u) }
    fn getElementById(&self, c: &Object, e: &str) -> Object { DefaultNavigator::getElementById(self, c, e) }
    fn getElementNamespaceUri(&self, _v: &Object) -> String { String::new() }
    fn getElementName(&self, _v: &Object) -> String { String::new() }
    fn getElementQName(&self, _v: &Object) -> String { String::new() }
    fn getAttributeNamespaceUri(&self, _v: &Object) -> String { String::new() }
    fn getAttributeName(&self, _v: &Object) -> String { String::new() }
    fn getAttributeQName(&self, _v: &Object) -> String { String::new() }
    fn isDocument(&self, _v: &Object) -> bool { false }
    fn isElement(&self, _v: &Object) -> bool { false }
    fn isAttribute(&self, _v: &Object) -> bool { false }
    fn isNamespace(&self, _v: &Object) -> bool { false }
    fn isComment(&self, _v: &Object) -> bool { false }
    fn isText(&self, _v: &Object) -> bool { false }
    fn isProcessingInstruction(&self, _v: &Object) -> bool { false }
    fn getCommentStringValue(&self, _v: &Object) -> String { String::new() }
    fn getElementStringValue(&self, _v: &Object) -> String { String::new() }
    fn getAttributeStringValue(&self, _v: &Object) -> String { String::new() }
    fn getNamespaceStringValue(&self, _v: &Object) -> String { String::new() }
    fn getTextStringValue(&self, _v: &Object) -> String { String::new() }
    fn getNamespacePrefix(&self, _v: &Object) -> String { String::new() }
    fn parseXPath(&self, _v: &str) -> Result<Box<dyn crate::org::jaxen::XPath::XPath>, crate::org::jaxen::saxpath::SAXPathException::SAXPathException> {
        Err(crate::org::jaxen::saxpath::SAXPathException::SAXPathException::new("not supported".to_string()))
    }
}
