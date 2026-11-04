use std::fmt;

// Java: public class IllegalAddException extends IllegalArgumentException
#[derive(Debug)]
pub struct IllegalAddException {
    pub(crate) message: String,
}

impl fmt::Display for IllegalAddException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IllegalAddException {}

// Java: public class IllegalAddException extends IllegalArgumentException
impl IllegalAddException {
    // Java: IllegalAddException(String reason)
    pub fn new(reason: String) -> Self {
        // Java: super(reason);
        IllegalAddException { message: reason }
    }

    // Java: IllegalAddException(Element parent, Node node, String reason)
    pub fn new_with_element(parent_name: &str, node: &str, reason: &str) -> Self {
        // Java: super("The node \"" + node.toString() + "\" could not be added to the element \"" + parent.getName() + "\" because: " + reason);
        IllegalAddException {
            message: format!(
                "The node \"{}\" could not be added to the element \"{}\" because: {}",
                node, parent_name, reason
            ),
        }
    }

    // Java: IllegalAddException(Branch parent, Node node, String reason)
    pub fn new_with_branch(parent_name: &str, node: &str, reason: &str) -> Self {
        // Java: super("The node \"" + node.toString() + "\" could not be added to the branch \"" + parent.getName() + "\" because: " + reason);
        IllegalAddException {
            message: format!(
                "The node \"{}\" could not be added to the branch \"{}\" because: {}",
                node, parent_name, reason
            ),
        }
    }
}
