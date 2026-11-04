// Java: import java.io.Serializable
// Java: import java.util.Iterator
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator};

// Java: public class IterableAxis implements Serializable
pub struct IterableAxis {
    // Java field: int value
    pub(crate) value: i32,
}

// Java: public class IterableAxis implements Serializable
impl IterableAxis {
    // Java: IterableAxis(int axisValue)
    pub fn new(axisValue: i32) -> Self {
      // Java: super();
      // Java: this.value = axisValue;
      IterableAxis { value: axisValue }
    }

    // Java: int value()
    pub fn value(&self) -> i32 {
      // Java: return this.value;
      self.value
    }

    // Java: abstract Iterator iterator(Object var1, ContextSupport var2)
    pub fn iterator(&self, _var1: &Object, _var2: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      Err(UnsupportedAxisException::new("abstract".to_string()))
    }

    // Java: Iterator namedAccessIterator(Object contextNode, ContextSupport support, String localName, String namespacePrefix, String namespaceURI)
    pub fn namedAccessIterator(&self, _contextNode: &Object, _support: &ContextSupport, _localName: &str, _namespacePrefix: &str, _namespaceURI: &str) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: throw new UnsupportedOperationException("Named access unsupported");
      Err(UnsupportedAxisException::new("Named access unsupported".to_string()))
    }

    // Java: boolean supportsNamedAccess(ContextSupport support)
    pub fn supportsNamedAccess(&self, _support: &ContextSupport) -> bool {
      // Java: return false;
      false
    }
}
