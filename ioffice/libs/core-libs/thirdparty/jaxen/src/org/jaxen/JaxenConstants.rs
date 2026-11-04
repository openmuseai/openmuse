// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.ListIterator
use crate::org::jaxen::{Object, NodeIterator};

// Java field: Iterator EMPTY_ITERATOR
pub fn empty_iterator() -> NodeIterator {
    Box::new(std::iter::empty())
}

// Java field: ListIterator EMPTY_LIST_ITERATOR
pub fn empty_list_iterator() -> NodeIterator {
    Box::new(std::iter::empty())
}

// Java: public class JaxenConstants
pub struct JaxenConstants {
}

// Java: public class JaxenConstants
impl JaxenConstants {
    // Java: JaxenConstants()
    pub fn new() -> Self {
        // Java: super();
        JaxenConstants {}
    }
}
