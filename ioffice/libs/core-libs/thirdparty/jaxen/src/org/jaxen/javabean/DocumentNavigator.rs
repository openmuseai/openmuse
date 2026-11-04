// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import org.jaxen.DefaultNavigator
use crate::org::jaxen::DefaultNavigator::DefaultNavigator;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
// Java: import org.jaxen.JaxenConstants
use crate::org::jaxen::JaxenConstants;
// Java: import org.jaxen.NamedAccessNavigator
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.XPath
use crate::org::jaxen::XPath::XPath;
// Java: import org.jaxen.saxpath.SAXPathException
use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
// Java: import org.jaxen.util.SingleObjectIterator
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use std::sync::OnceLock;

// Java field: long serialVersionUID

// Java field: Class[] EMPTY_CLASS_ARRAY
// Java field: Object[] EMPTY_OBJECT_ARRAY

// Java field: DocumentNavigator instance
static INSTANCE: OnceLock<DocumentNavigator> = OnceLock::new();

// Java: public class DocumentNavigator extends DefaultNavigator implements NamedAccessNavigator
pub struct DocumentNavigator {
    pub(crate) base: DefaultNavigator,
}

// Java: public class DocumentNavigator extends DefaultNavigator implements NamedAccessNavigator
impl DocumentNavigator {
    // Java: DocumentNavigator()
    pub fn new() -> Self {
      // Java: super();
      DocumentNavigator {
         base: DefaultNavigator::new(),
      }
    }

    // Java: static Navigator getInstance()
    pub fn getInstance() -> &'static DocumentNavigator {
      // Java: return instance;
      INSTANCE.get_or_init(|| DocumentNavigator::new())
    }

    // Java: boolean isElement(Object obj)
    pub fn isElement(&self, obj: &Object) -> bool {
      // Java: return obj instanceof Element;
      obj.downcast_ref::<Element>().is_some()
    }

    // Java: boolean isComment(Object obj)
    pub fn isComment(&self, _obj: &Object) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isText(Object obj)
    pub fn isText(&self, obj: &Object) -> bool {
      // Java: return obj instanceof String;
      obj.downcast_ref::<String>().is_some()
    }

    // Java: boolean isAttribute(Object obj)
    pub fn isAttribute(&self, _obj: &Object) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isProcessingInstruction(Object obj)
    pub fn isProcessingInstruction(&self, _obj: &Object) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isDocument(Object obj)
    pub fn isDocument(&self, _obj: &Object) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isNamespace(Object obj)
    pub fn isNamespace(&self, _obj: &Object) -> bool {
      // Java: return false;
      false
    }

    // Java: String getElementName(Object obj)
    pub fn getElementName(&self, _obj: &Object) -> String {
      // Java: return ((Element)obj).getName();
      todo!("DocumentNavigator::getElementName")
    }

    // Java: String getElementNamespaceUri(Object obj)
    pub fn getElementNamespaceUri(&self, _obj: &Object) -> String {
      // Java: return "";
      String::new()
    }

    // Java: String getElementQName(Object obj)
    pub fn getElementQName(&self, _obj: &Object) -> String {
      // Java: return "";
      String::new()
    }

    // Java: String getAttributeName(Object obj)
    pub fn getAttributeName(&self, _obj: &Object) -> String {
      // Java: return "";
      String::new()
    }

    // Java: String getAttributeNamespaceUri(Object obj)
    pub fn getAttributeNamespaceUri(&self, _obj: &Object) -> String {
      // Java: return "";
      String::new()
    }

    // Java: String getAttributeQName(Object obj)
    pub fn getAttributeQName(&self, _obj: &Object) -> String {
      // Java: return "";
      String::new()
    }

    // Java: Iterator getChildAxisIterator(Object contextNode)
    pub fn getChildAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      Ok(JaxenConstants::empty_iterator())
    }

    // Java: Iterator getChildAxisIterator(Object contextNode, String localName, String namespacePrefix, String namespaceURI)
    pub fn getChildAxisIterator_named(&self, _contextNode: &Object, _localName: &str, _namespacePrefix: &str, _namespaceURI: &str) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: Class cls = ((Element)contextNode).getObject().getClass();
      // Java: String methodName = this.javacase(localName);
      // Java: Method method = null;
      // Java: try
      // Java: method = cls.getMethod("get" + methodName, EMPTY_CLASS_ARRAY);
      // Java: catch (NoSuchMethodException var15)
      // Java: try
      // Java: method = cls.getMethod("get" + methodName + "s", EMPTY_CLASS_ARRAY);
      // Java: catch (NoSuchMethodException var14)
      // Java: try
      // Java: method = cls.getMethod(localName, EMPTY_CLASS_ARRAY);
      // Java: catch (NoSuchMethodException var13)
      // Java: method = null;
      // Java: if (method == null)
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      // Java: else
      // Java: try
      // Java: Object result = method.invoke(((Element)contextNode).getObject(), EMPTY_OBJECT_ARRAY);
      // Java: if (result == null)
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      // Java: if (result instanceof Collection)
      // Java: return new ElementIterator((Element)contextNode, localName, ((Collection)result).iterator());
      // Java: if (result.getClass().isArray())
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      // Java: return new SingleObjectIterator(new Element((Element)contextNode, localName, result));
      // Java: catch (IllegalAccessException var11)
      // Java: catch (InvocationTargetException var12)
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      todo!("DocumentNavigator::getChildAxisIterator_named")
    }

    // Java: Iterator getParentAxisIterator(Object contextNode)
    pub fn getParentAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return (Iterator)(contextNode instanceof Element ? new SingleObjectIterator(((Element)contextNode).getParent()) : JaxenConstants.EMPTY_ITERATOR);
      todo!("DocumentNavigator::getParentAxisIterator")
    }

    // Java: Iterator getAttributeAxisIterator(Object contextNode)
    pub fn getAttributeAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      Ok(JaxenConstants::empty_iterator())
    }

    // Java: Iterator getAttributeAxisIterator(Object contextNode, String localName, String namespacePrefix, String namespaceURI)
    pub fn getAttributeAxisIterator_named(&self, _contextNode: &Object, _localName: &str, _namespacePrefix: &str, _namespaceURI: &str) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      Ok(JaxenConstants::empty_iterator())
    }

    // Java: Iterator getNamespaceAxisIterator(Object contextNode)
    pub fn getNamespaceAxisIterator(&self, _contextNode: &Object) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return JaxenConstants.EMPTY_ITERATOR;
      Ok(JaxenConstants::empty_iterator())
    }

    // Java: Object getDocumentNode(Object contextNode)
    pub fn getDocumentNode(&self, _contextNode: &Object) -> Option<Object> {
      // Java: return null;
      None
    }

    // Java: Object getParentNode(Object contextNode)
    pub fn getParentNode(&self, _contextNode: &Object) -> Result<Object, UnsupportedAxisException> {
      // Java: return contextNode instanceof Element ? ((Element)contextNode).getParent() : JaxenConstants.EMPTY_ITERATOR;
      todo!("DocumentNavigator::getParentNode")
    }

    // Java: String getTextStringValue(Object obj)
    pub fn getTextStringValue(&self, _obj: &Object) -> String {
      // Java: return obj instanceof Element ? ((Element)obj).getObject().toString() : obj.toString();
      todo!("DocumentNavigator::getTextStringValue")
    }

    // Java: String getElementStringValue(Object obj)
    pub fn getElementStringValue(&self, _obj: &Object) -> String {
      // Java: return obj instanceof Element ? ((Element)obj).getObject().toString() : obj.toString();
      todo!("DocumentNavigator::getElementStringValue")
    }

    // Java: String getAttributeStringValue(Object obj)
    pub fn getAttributeStringValue(&self, _obj: &Object) -> String {
      // Java: return obj.toString();
      todo!("DocumentNavigator::getAttributeStringValue")
    }

    // Java: String getNamespaceStringValue(Object obj)
    pub fn getNamespaceStringValue(&self, _obj: &Object) -> String {
      // Java: return obj.toString();
      todo!("DocumentNavigator::getNamespaceStringValue")
    }

    // Java: String getNamespacePrefix(Object obj)
    pub fn getNamespacePrefix(&self, _obj: &Object) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: String getCommentStringValue(Object obj)
    pub fn getCommentStringValue(&self, _obj: &Object) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: String translateNamespacePrefixToUri(String prefix, Object context)
    pub fn translateNamespacePrefixToUri(&self, _prefix: &str, _context: &Object) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: short getNodeType(Object node)
    pub fn getNodeType(&self, _node: &Object) -> i16 {
      // Java: return 0;
      0
    }

    // Java: Object getDocument(String uri)
    pub fn getDocument(&self, _uri: &str) -> Result<Option<Object>, FunctionCallException> {
      // Java: return null;
      Ok(None)
    }

    // Java: String getProcessingInstructionTarget(Object obj)
    pub fn getProcessingInstructionTarget(&self, _obj: &Object) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: String getProcessingInstructionData(Object obj)
    pub fn getProcessingInstructionData(&self, _obj: &Object) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: XPath parseXPath(String xpath)
    pub fn parseXPath(&self, _xpath: &str) -> Result<Box<dyn XPath>, SAXPathException> {
      // Java: return new JavaBeanXPath(xpath);
      todo!("DocumentNavigator::parseXPath")
    }

    // Java: String javacase(String name)
    pub fn javacase(&self, name: &str) -> String {
      // Java: if (name.length() == 0)
      if name.is_empty() {
         // Java: return name;
         return name.to_string();
      }
      // Java: else
      else {
         // Java: return name.length() == 1 ? name.toUpperCase() : name.substring(0, 1).toUpperCase() + name.substring(1);
         if name.len() == 1 {
            name.to_uppercase()
         } else {
            let mut chars = name.chars();
            let first = chars.next().unwrap().to_uppercase().to_string();
            format!("{}{}", first, chars.as_str())
         }
      }
    }
}

pub struct Element {
    pub(crate) parent: Option<Object>,
    pub(crate) name: String,
    pub(crate) object: Object,
}
