// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.util.SingleObjectIterator
// Java: import org.jaxen.util.SingletonList
use crate::org::jaxen::{Object, List, NodeIterator};
use std::sync::Arc;
use std::any::Any;

// Java: public class DefaultExpr implements Expr
pub struct DefaultExpr {
}

// Java: public class DefaultExpr implements Expr
impl DefaultExpr {
    // Java: DefaultExpr()
    pub fn new() -> Self {
      // Java: super();
      DefaultExpr {}
    }

    // Java: Expr simplify()
    // Note: simplify() in Java returns `this`. In Rust, this is handled by concrete types.
    // Java: return this;

    // Java: static Iterator convertToIterator(Object obj)
    pub fn convertToIterator(obj: &Object) -> NodeIterator {
      // Java: if (obj instanceof Iterator)
      // Java: return (Iterator)obj;
      // Java: else
      // Java: return (Iterator)(obj instanceof List ? ((List)obj).iterator() : new SingleObjectIterator(obj));
      if let Some(list) = obj.downcast_ref::<List>() {
         Box::new(list.clone().into_iter())
      } else {
         Box::new(std::iter::once(Arc::clone(obj)))
      }
    }

    // Java: static List convertToList(Object obj)
    pub fn convertToList(obj: &Object) -> List {
      // Java: return (List)(obj instanceof List ? (List)obj : new SingletonList(obj));
      if let Some(list) = obj.downcast_ref::<List>() {
         list.clone()
      } else {
         vec![Arc::clone(obj)]
      }
    }
}
