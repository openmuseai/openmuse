use std::fmt;

use super::Element::Element;
use super::Node::Node;
use super::Branch::Branch;

// Java: public class IllegalAddException extends IllegalArgumentException
#[derive(Debug)]
pub struct IllegalAddException {
    pub(crate) message: String,
}

// Java: public class IllegalAddException extends IllegalArgumentException
impl IllegalAddException {
    // Java: IllegalAddException(String reason)
    pub fn new(reason: &str) -> Self {
        // Java: super(reason);
        IllegalAddException {
            message: reason.to_string(),
        }
    }

    // Java: IllegalAddException(Element parent, Node node, String reason)
    pub fn new_with_element(parent: &dyn Element, node: &dyn Node, reason: &str) -> Self {
        // Java: super("The node \"" + node.toString() + "\" could not be added to the element \"" + parent.getQualifiedName() + "\" because: " + reason);
        IllegalAddException {
            message: format!(
                "The node \"{}\" could not be added to the element \"{}\" because: {}",
                node.getName(),
                parent.getQualifiedName(),
                reason
            ),
        }
    }

    // Java: IllegalAddException(Branch parent, Node node, String reason)
    pub fn new_with_branch(parent: &dyn Branch, node: &dyn Node, reason: &str) -> Self {
        // Java: super("The node \"" + node.toString() + "\" could not be added to the branch \"" + parent.getName() + "\" because: " + reason);
        IllegalAddException {
            message: format!(
                "The node \"{}\" could not be added to the branch \"{}\" because: {}",
                node.getName(),
                parent.getName(),
                reason
            ),
        }
    }
}

impl fmt::Display for IllegalAddException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IllegalAddException {}
