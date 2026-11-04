// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -3140080721715120745;

// Java: public class IterablePrecedingSiblingAxis extends IterableAxis
pub struct IterablePrecedingSiblingAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterablePrecedingSiblingAxis extends IterableAxis
impl IterablePrecedingSiblingAxis {
    // Java: IterablePrecedingSiblingAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterablePrecedingSiblingAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getPrecedingSiblingAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getPrecedingSiblingAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
