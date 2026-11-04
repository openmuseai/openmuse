// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeFilter
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.rule.Pattern
use crate::org::dom4j::Node::Node;
use crate::org::dom4j::NodeFilter::NodeFilter;
use crate::org::dom4j::NodeType::NodeType;
use crate::org::dom4j::rule::Pattern::Pattern;

// Java: public class DefaultPattern implements Pattern
pub struct DefaultPattern {
    // Java field: NodeFilter filter
    pub(crate) filter: Box<dyn NodeFilter>,
}

// Java: public class DefaultPattern implements Pattern
impl DefaultPattern {
    // Java: DefaultPattern(NodeFilter filter)
    pub fn new(filter: Box<dyn NodeFilter>) -> Self {
        // Java: super();
        // Java: this.filter = filter;
        DefaultPattern { filter }
    }
}

impl NodeFilter for DefaultPattern {
    fn matches(&self, node: &dyn Node) -> bool {
        self.filter.matches(node)
    }
}

impl Pattern for DefaultPattern {
    // Java: boolean matches(Node node)
    fn matches_node(&self, node: &dyn Node) -> bool {
        // Java: return this.filter.matches(node);
        self.filter.matches(node)
    }

    // Java: double getPriority()
    fn getPriority(&self) -> f64 {
        // Java: return (double)0.5F;
        0.5
    }

    // Java: Pattern[] getUnionPatterns()
    fn getUnionPatterns(&self) -> Vec<Box<dyn Pattern>> {
        // Java: return null;
        Vec::new()
    }

    // Java: NodeType getMatchType()
    fn getMatchType(&self) -> NodeType {
        // Java: return NodeType.ANY_NODE;
        NodeType::AnyNode
    }

    // Java: String getMatchesNodeName()
    fn getMatchesNodeName(&self) -> String {
        // Java: return null;
        String::new()
    }
}
