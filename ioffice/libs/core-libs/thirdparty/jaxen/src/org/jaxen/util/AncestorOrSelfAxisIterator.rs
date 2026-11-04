// Java: import java.util.Iterator
// Java: import java.util.NoSuchElementException
// Java: import org.jaxen.JaxenRuntimeException
use crate::org::jaxen::JaxenRuntimeException::JaxenRuntimeException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::Object;

// Java: public class AncestorOrSelfAxisIterator implements Iterator
pub struct AncestorOrSelfAxisIterator {
    // Java field: Object contextNode
    pub(crate) contextNode: Option<Object>,
    // Java field: Navigator navigator
    navigator: Box<dyn Navigator>,
}

// Java: public class AncestorOrSelfAxisIterator implements Iterator
impl AncestorOrSelfAxisIterator {
    // Java: AncestorOrSelfAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super();
        // Java: this.contextNode = contextNode;
        // Java: this.navigator = navigator;
        AncestorOrSelfAxisIterator {
            contextNode: Some(contextNode),
            navigator,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.contextNode != null;
        self.contextNode.is_some()
    }

    // Java: Object next()
    pub fn next_node(&mut self) -> Object {
        // Java: try
        // Java: if (this.hasNext())
        if self.hasNext() {
            // Java: Object result = this.contextNode;
            let result = self.contextNode.clone().unwrap();
            // Java: this.contextNode = this.navigator.getParentNode(this.contextNode);
            self.contextNode = self.navigator.getParentNode(&result).ok();
            // Java: return result;
            result
        }
        // Java: else
        else {
            // Java: throw new NoSuchElementException("Exhausted ancestor-or-self axis");
            panic!("Exhausted ancestor-or-self axis");
        }
        // Java: catch (UnsupportedAxisException e)
        // Java: throw new JaxenRuntimeException(e);
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}

impl Iterator for AncestorOrSelfAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hasNext() {
            Some(self.next_node())
        } else {
            None
        }
    }
}
