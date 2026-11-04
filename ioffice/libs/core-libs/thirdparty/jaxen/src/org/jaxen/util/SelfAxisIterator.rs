use crate::org::jaxen::Object;
use super::SingleObjectIterator::SingleObjectIterator;

// Java: public class SelfAxisIterator extends SingleObjectIterator
pub struct SelfAxisIterator {
    inner: SingleObjectIterator,
}

// Java: public class SelfAxisIterator extends SingleObjectIterator
impl SelfAxisIterator {
    // Java: SelfAxisIterator(Object node)
    pub fn new(node: Object) -> Self {
        // Java: super(node);
        SelfAxisIterator {
            inner: SingleObjectIterator::new(node),
        }
    }
}

impl Iterator for SelfAxisIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
