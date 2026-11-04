// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 8292222516706760134;

// Java: public class IterableSelfAxis extends IterableAxis
pub struct IterableSelfAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterableSelfAxis extends IterableAxis
impl IterableSelfAxis {
    // Java: IterableSelfAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterableSelfAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getSelfAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getSelfAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
