// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use std::sync::OnceLock;

// Java field: NodeTypeTest DOCUMENT_TEST
static DOCUMENT_TEST: OnceLock<NodeTypeTest> = OnceLock::new();
// Java field: NodeTypeTest ELEMENT_TEST
static ELEMENT_TEST: OnceLock<NodeTypeTest> = OnceLock::new();
// Java field: NodeTypeTest ATTRIBUTE_TEST
static ATTRIBUTE_TEST: OnceLock<NodeTypeTest> = OnceLock::new();
// Java field: NodeTypeTest COMMENT_TEST
static COMMENT_TEST: OnceLock<NodeTypeTest> = OnceLock::new();
// Java field: NodeTypeTest TEXT_TEST
static TEXT_TEST: OnceLock<NodeTypeTest> = OnceLock::new();
// Java field: NodeTypeTest PROCESSING_INSTRUCTION_TEST
static PROCESSING_INSTRUCTION_TEST: OnceLock<NodeTypeTest> = OnceLock::new();
// Java field: NodeTypeTest NAMESPACE_TEST
static NAMESPACE_TEST: OnceLock<NodeTypeTest> = OnceLock::new();

// Java: public class NodeTypeTest extends NodeTest
pub struct NodeTypeTest {
    // Java field: short nodeType
    pub(crate) nodeType: i16,
}

// Java: public class NodeTypeTest extends NodeTest
impl NodeTypeTest {
    pub fn document_test() -> &'static NodeTypeTest { DOCUMENT_TEST.get_or_init(|| NodeTypeTest { nodeType: 9 }) }
    pub fn element_test() -> &'static NodeTypeTest { ELEMENT_TEST.get_or_init(|| NodeTypeTest { nodeType: 1 }) }
    pub fn attribute_test() -> &'static NodeTypeTest { ATTRIBUTE_TEST.get_or_init(|| NodeTypeTest { nodeType: 2 }) }
    pub fn comment_test() -> &'static NodeTypeTest { COMMENT_TEST.get_or_init(|| NodeTypeTest { nodeType: 8 }) }
    pub fn text_test() -> &'static NodeTypeTest { TEXT_TEST.get_or_init(|| NodeTypeTest { nodeType: 3 }) }
    pub fn processing_instruction_test() -> &'static NodeTypeTest { PROCESSING_INSTRUCTION_TEST.get_or_init(|| NodeTypeTest { nodeType: 7 }) }
    pub fn namespace_test() -> &'static NodeTypeTest { NAMESPACE_TEST.get_or_init(|| NodeTypeTest { nodeType: 13 }) }

    // Java: NodeTypeTest(short nodeType)
    pub fn new(nodeType: i16) -> Self {
      // Java: super();
      // Java: this.nodeType = nodeType;
      NodeTypeTest { nodeType }
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: return this.nodeType == context.getNavigator().getNodeType(node);
      todo!("NodeTypeTest::matches")
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
      // Java: return (double)-0.5F;
      -0.5
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return this.nodeType;
      self.nodeType
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: switch (this.nodeType)
      match self.nodeType {
         // Java: case 1:
         // Java: return "child()";
         1 => { "child()".to_string() }
         // Java: case 2:
         // Java: return "@*";
         2 => { "@*".to_string() }
         // Java: case 3:
         // Java: return "text()";
         3 => { "text()".to_string() }
         // Java: case 4:
         // Java: case 5:
         // Java: case 6:
         // Java: case 10:
         // Java: case 11:
         // Java: case 12:
         // Java: default:
         // Java: return "";
         // Java: case 7:
         // Java: return "processing-instruction()";
         7 => { "processing-instruction()".to_string() }
         // Java: case 8:
         // Java: return "comment()";
         8 => { "comment()".to_string() }
         // Java: case 9:
         // Java: return "/";
         9 => { "/".to_string() }
         // Java: case 13:
         // Java: return "namespace()";
         13 => { "namespace()".to_string() }
         _ => { String::new() }
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + "[ type: " + this.nodeType + " ]";
      format!("NodeTypeTest[ type: {} ]", self.nodeType)
    }
}

impl super::Pattern::Pattern for NodeTypeTest {
    fn matches(&self, node: &Object, context: &Context) -> Result<bool, JaxenException> {
      NodeTypeTest::matches(self, node, context)
    }

    fn getPriority(&self) -> f64 { NodeTypeTest::getPriority(self) }
    fn getMatchType(&self) -> i16 { NodeTypeTest::getMatchType(self) }
    fn getText(&self) -> String { NodeTypeTest::getText(self) }
}
