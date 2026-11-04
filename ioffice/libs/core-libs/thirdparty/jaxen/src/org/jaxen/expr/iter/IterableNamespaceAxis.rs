// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -8022585664651357087;

// Java: public class IterableNamespaceAxis extends IterableAxis
pub struct IterableNamespaceAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterableNamespaceAxis extends IterableAxis
impl IterableNamespaceAxis {
    // Java: IterableNamespaceAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterableNamespaceAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getNamespaceAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getNamespaceAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
