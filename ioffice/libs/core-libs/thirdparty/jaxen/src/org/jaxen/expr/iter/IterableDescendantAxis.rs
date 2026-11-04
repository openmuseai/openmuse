// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 7286715505909806723;

// Java: public class IterableDescendantAxis extends IterableAxis
pub struct IterableDescendantAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterableDescendantAxis extends IterableAxis
impl IterableDescendantAxis {
    // Java: IterableDescendantAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterableDescendantAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getDescendantAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getDescendantAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
