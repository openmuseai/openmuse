use super::{Object, NodeIterator};
use super::UnsupportedAxisException::UnsupportedAxisException;
use super::FunctionCallException::FunctionCallException;
use super::saxpath::SAXPathException::SAXPathException;
use super::XPath::XPath;

// Java: public interface Navigator extends Serializable
pub trait Navigator {
    // Java: abstract Iterator getChildAxisIterator(Object var1)
    fn getChildAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getDescendantAxisIterator(Object var1)
    fn getDescendantAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getParentAxisIterator(Object var1)
    fn getParentAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getAncestorAxisIterator(Object var1)
    fn getAncestorAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getFollowingSiblingAxisIterator(Object var1)
    fn getFollowingSiblingAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getPrecedingSiblingAxisIterator(Object var1)
    fn getPrecedingSiblingAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getFollowingAxisIterator(Object var1)
    fn getFollowingAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getPrecedingAxisIterator(Object var1)
    fn getPrecedingAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getAttributeAxisIterator(Object var1)
    fn getAttributeAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getNamespaceAxisIterator(Object var1)
    fn getNamespaceAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getSelfAxisIterator(Object var1)
    fn getSelfAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getDescendantOrSelfAxisIterator(Object var1)
    fn getDescendantOrSelfAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getAncestorOrSelfAxisIterator(Object var1)
    fn getAncestorOrSelfAxisIterator(&self, var1: &Object) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Object getDocument(String var1)
    fn getDocument(&self, var1: &str) -> Result<Object, FunctionCallException>;

    // Java: abstract Object getDocumentNode(Object var1)
    fn getDocumentNode(&self, var1: &Object) -> Object;

    // Java: abstract Object getParentNode(Object var1)
    fn getParentNode(&self, var1: &Object) -> Result<Object, UnsupportedAxisException>;

    // Java: abstract String getElementNamespaceUri(Object var1)
    fn getElementNamespaceUri(&self, var1: &Object) -> String;

    // Java: abstract String getElementName(Object var1)
    fn getElementName(&self, var1: &Object) -> String;

    // Java: abstract String getElementQName(Object var1)
    fn getElementQName(&self, var1: &Object) -> String;

    // Java: abstract String getAttributeNamespaceUri(Object var1)
    fn getAttributeNamespaceUri(&self, var1: &Object) -> String;

    // Java: abstract String getAttributeName(Object var1)
    fn getAttributeName(&self, var1: &Object) -> String;

    // Java: abstract String getAttributeQName(Object var1)
    fn getAttributeQName(&self, var1: &Object) -> String;

    // Java: abstract String getProcessingInstructionTarget(Object var1)
    fn getProcessingInstructionTarget(&self, var1: &Object) -> String;

    // Java: abstract String getProcessingInstructionData(Object var1)
    fn getProcessingInstructionData(&self, var1: &Object) -> String;

    // Java: abstract boolean isDocument(Object var1)
    fn isDocument(&self, var1: &Object) -> bool;

    // Java: abstract boolean isElement(Object var1)
    fn isElement(&self, var1: &Object) -> bool;

    // Java: abstract boolean isAttribute(Object var1)
    fn isAttribute(&self, var1: &Object) -> bool;

    // Java: abstract boolean isNamespace(Object var1)
    fn isNamespace(&self, var1: &Object) -> bool;

    // Java: abstract boolean isComment(Object var1)
    fn isComment(&self, var1: &Object) -> bool;

    // Java: abstract boolean isText(Object var1)
    fn isText(&self, var1: &Object) -> bool;

    // Java: abstract boolean isProcessingInstruction(Object var1)
    fn isProcessingInstruction(&self, var1: &Object) -> bool;

    // Java: abstract String getCommentStringValue(Object var1)
    fn getCommentStringValue(&self, var1: &Object) -> String;

    // Java: abstract String getElementStringValue(Object var1)
    fn getElementStringValue(&self, var1: &Object) -> String;

    // Java: abstract String getAttributeStringValue(Object var1)
    fn getAttributeStringValue(&self, var1: &Object) -> String;

    // Java: abstract String getNamespaceStringValue(Object var1)
    fn getNamespaceStringValue(&self, var1: &Object) -> String;

    // Java: abstract String getTextStringValue(Object var1)
    fn getTextStringValue(&self, var1: &Object) -> String;

    // Java: abstract String getNamespacePrefix(Object var1)
    fn getNamespacePrefix(&self, var1: &Object) -> String;

    // Java: abstract String translateNamespacePrefixToUri(String var1, Object var2)
    fn translateNamespacePrefixToUri(&self, var1: &str, var2: &Object) -> String;

    // Java: abstract XPath parseXPath(String var1)
    fn parseXPath(&self, var1: &str) -> Result<Box<dyn XPath>, SAXPathException>;

    // Java: abstract Object getElementById(Object var1, String var2)
    fn getElementById(&self, var1: &Object, var2: &str) -> Object;

    // Java: abstract short getNodeType(Object var1)
    fn getNodeType(&self, var1: &Object) -> i16;
}
