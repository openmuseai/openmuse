// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.NamedAccessNavigator
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 1;

// Java: public class IterableChildAxis extends IterableAxis
pub struct IterableChildAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterableChildAxis extends IterableAxis
impl IterableChildAxis {
    // Java: IterableChildAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterableChildAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getChildAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getChildAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }

    // Java: Iterator namedAccessIterator(Object contextNode, ContextSupport support, String localName, String namespacePrefix, String namespaceURI)
    pub fn namedAccessIterator(&self, _contextNode: &Object, _support: &ContextSupport, _localName: &str, _namespacePrefix: &str, _namespaceURI: &str) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: NamedAccessNavigator nav = (NamedAccessNavigator)support.getNavigator();
      // Java: return nav.getChildAxisIterator(contextNode, localName, namespacePrefix, namespaceURI);
      Err(UnsupportedAxisException::new("Named access not supported via dyn Navigator".to_string()))
    }

    // Java: boolean supportsNamedAccess(ContextSupport support)
    pub fn supportsNamedAccess(&self, _support: &ContextSupport) -> bool {
      // Java: return support.getNavigator() instanceof NamedAccessNavigator;
      false
    }
}
