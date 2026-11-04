// Java: import java.util.Iterator
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::Object;
use super::DescendantAxisIterator::DescendantAxisIterator;
use super::SingleObjectIterator::SingleObjectIterator;

// Java: public class DescendantOrSelfAxisIterator extends DescendantAxisIterator
pub struct DescendantOrSelfAxisIterator {
    inner: DescendantAxisIterator,
}

// Java: public class DescendantOrSelfAxisIterator extends DescendantAxisIterator
impl DescendantOrSelfAxisIterator {
    // Java: DescendantOrSelfAxisIterator(Object contextNode, Navigator navigator)
    pub fn new(contextNode: Object, navigator: Box<dyn Navigator>) -> Self {
        // Java: super((Navigator)navigator, (Iterator)(new SingleObjectIterator(contextNode)));
        let iter = Box::new(SingleObjectIterator::new(contextNode));
        let inner = DescendantAxisIterator::new_with_iterator(navigator, iter);
        DescendantOrSelfAxisIterator { inner }
    }
}

impl Iterator for DescendantOrSelfAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
