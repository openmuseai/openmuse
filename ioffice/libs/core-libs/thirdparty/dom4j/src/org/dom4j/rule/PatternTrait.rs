// Java: import org.dom4j.Node
use crate::org::dom4j::Node::Node;
// Java: import org.dom4j.NodeFilter
use crate::org::dom4j::NodeFilter::NodeFilter;
// Java: import org.dom4j.NodeType
use crate::org::dom4j::NodeType::NodeType;

// Java field: double DEFAULT_PRIORITY
pub const DEFAULT_PRIORITY: f64 = 0.5;

// Java: public interface Pattern extends NodeFilter
pub trait Pattern: NodeFilter {
    // Java: abstract boolean matches(Node var1)
    fn matches_node(&self, var1: &dyn Node) -> bool;

    // Java: abstract double getPriority()
    fn getPriority(&self) -> f64;

    // Java: abstract Pattern[] getUnionPatterns()
    fn getUnionPatterns(&self) -> Vec<Box<dyn Pattern>>;

    // Java: abstract NodeType getMatchType()
    fn getMatchType(&self) -> NodeType;

    // Java: abstract String getMatchesNodeName()
    fn getMatchesNodeName(&self) -> String;
}
