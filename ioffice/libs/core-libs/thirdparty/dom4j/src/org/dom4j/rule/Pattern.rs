// Java: import org.dom4j.Node
use crate::org::dom4j::Node;
// Java: import org.dom4j.NodeFilter
use crate::org::dom4j::NodeFilter;
// Java: import org.dom4j.NodeType
use crate::org::dom4j::NodeType;

// Java: public interface Pattern extends NodeFilter
pub trait Pattern {
    // Java: abstract boolean matches(Node var1)
    fn matches(&self, Node var1) -> bool;

    // Java: abstract double getPriority()
    fn getPriority(&self) -> f64;

    // Java: abstract Pattern[] getUnionPatterns()
    fn getUnionPatterns(&self) -> Pattern[];

    // Java: abstract NodeType getMatchType()
    fn getMatchType(&self) -> NodeType;

    // Java: abstract String getMatchesNodeName()
    fn getMatchesNodeName(&self) -> String;

}
