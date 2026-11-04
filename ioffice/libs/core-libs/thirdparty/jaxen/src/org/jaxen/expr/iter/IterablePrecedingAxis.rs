// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 587333938258540052;

// Java: public class IterablePrecedingAxis extends IterableAxis
pub struct IterablePrecedingAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterablePrecedingAxis extends IterableAxis
impl IterablePrecedingAxis {
    // Java: IterablePrecedingAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterablePrecedingAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getPrecedingAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getPrecedingAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
