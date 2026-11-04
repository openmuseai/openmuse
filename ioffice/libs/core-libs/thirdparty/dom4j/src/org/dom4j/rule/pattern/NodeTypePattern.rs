// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.rule.Pattern
use std::sync::LazyLock;
use crate::org::dom4j::Node::Node;
use crate::org::dom4j::NodeFilter::NodeFilter;
use crate::org::dom4j::NodeType::NodeType;
use crate::org::dom4j::rule::Pattern::Pattern;

// Java field: NodeTypePattern ANY_ATTRIBUTE
pub static ANY_ATTRIBUTE: LazyLock<NodeTypePattern> = LazyLock::new(|| NodeTypePattern::new(NodeType::AttributeNode));
// Java field: NodeTypePattern ANY_COMMENT
pub static ANY_COMMENT: LazyLock<NodeTypePattern> = LazyLock::new(|| NodeTypePattern::new(NodeType::CommentNode));
// Java field: NodeTypePattern ANY_DOCUMENT
pub static ANY_DOCUMENT: LazyLock<NodeTypePattern> = LazyLock::new(|| NodeTypePattern::new(NodeType::DocumentNode));
// Java field: NodeTypePattern ANY_ELEMENT
pub static ANY_ELEMENT: LazyLock<NodeTypePattern> = LazyLock::new(|| NodeTypePattern::new(NodeType::ElementNode));
// Java field: NodeTypePattern ANY_PROCESSING_INSTRUCTION
pub static ANY_PROCESSING_INSTRUCTION: LazyLock<NodeTypePattern> = LazyLock::new(|| NodeTypePattern::new(NodeType::ProcessingInstructionNode));
// Java field: NodeTypePattern ANY_TEXT
pub static ANY_TEXT: LazyLock<NodeTypePattern> = LazyLock::new(|| NodeTypePattern::new(NodeType::TextNode));

// Java: public class NodeTypePattern implements Pattern
pub struct NodeTypePattern {
    // Java field: NodeType nodeType
    pub(crate) nodeType: NodeType,
}

// Java: public class NodeTypePattern implements Pattern
impl NodeTypePattern {
    // Java: NodeTypePattern(NodeType nodeType)
    pub fn new(node_type: NodeType) -> Self {
        // Java: super();
        // Java: this.nodeType = nodeType;
        NodeTypePattern {
            nodeType: node_type,
        }
    }
}

impl NodeFilter for NodeTypePattern {
    fn matches(&self, _var1: &dyn Node) -> bool {
        false
    }
}

impl Pattern for NodeTypePattern {
    // Java: boolean matches(Node node)
    fn matches_node(&self, _node: &dyn Node) -> bool {
        // Java: return node.getNodeTypeEnum() == this.nodeType;
        false
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
        // Java: return this.nodeType;
        self.nodeType
    }

    // Java: String getMatchesNodeName()
    fn getMatchesNodeName(&self) -> String {
        // Java: return null;
        String::new()
    }
}
