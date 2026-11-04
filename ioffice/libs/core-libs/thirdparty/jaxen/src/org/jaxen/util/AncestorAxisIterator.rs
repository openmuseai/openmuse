// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::Object;
use super::AncestorOrSelfAxisIterator::AncestorOrSelfAxisIterator;

// Java: public class AncestorAxisIterator extends AncestorOrSelfAxisIterator
pub struct AncestorAxisIterator {
    inner: AncestorOrSelfAxisIterator,
}

// Java: public class AncestorAxisIterator extends AncestorOrSelfAxisIterator
impl AncestorAxisIterator {
    // Java: AncestorAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super(contextNode, navigator);
        let mut inner = AncestorOrSelfAxisIterator::new(contextNode, navigator);
        // Java: this.next();
        inner.next_node();
        AncestorAxisIterator { inner }
    }
}

impl Iterator for AncestorAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
