// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -7521574185875636490;

// Java: public class IterableParentAxis extends IterableAxis
pub struct IterableParentAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterableParentAxis extends IterableAxis
impl IterableParentAxis {
    // Java: IterableParentAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterableParentAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getParentAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getParentAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
