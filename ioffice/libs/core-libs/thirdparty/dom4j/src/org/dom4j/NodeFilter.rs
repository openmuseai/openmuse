use super::Node::Node;

// Java: public interface NodeFilter
pub trait NodeFilter {
    // Java: abstract boolean matches(Node var1)
    fn matches(&self, var1: &dyn Node) -> bool;
}
