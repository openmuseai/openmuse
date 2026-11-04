// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};
use super::IterableAxis::IterableAxis;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 4412705219546610009;

// Java: public class IterableFollowingSiblingAxis extends IterableAxis
pub struct IterableFollowingSiblingAxis {
    pub(crate) base: IterableAxis,
}

// Java: public class IterableFollowingSiblingAxis extends IterableAxis
impl IterableFollowingSiblingAxis {
    // Java: IterableFollowingSiblingAxis(int value)
    pub fn new(value: i32) -> Self {
      // Java: super(value);
      IterableFollowingSiblingAxis { base: IterableAxis::new(value) }
    }

    // Java: Iterator iterator(Object contextNode, ContextSupport support)
    pub fn iterator(&self, contextNode: &Object, support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return support.getNavigator().getFollowingSiblingAxisIterator(contextNode);
      match support.getNavigator() {
          Some(nav) => nav.getFollowingSiblingAxisIterator(contextNode),
          None => Err(UnsupportedAxisException::new("No navigator installed".to_string())),
      }
    }
}
