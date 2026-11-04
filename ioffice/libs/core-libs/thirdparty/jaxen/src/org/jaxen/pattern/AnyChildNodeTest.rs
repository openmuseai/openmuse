// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use std::sync::OnceLock;

// Java field: AnyChildNodeTest instance
static INSTANCE: OnceLock<AnyChildNodeTest> = OnceLock::new();

// Java: public class AnyChildNodeTest extends NodeTest
pub struct AnyChildNodeTest {
}

// Java: public class AnyChildNodeTest extends NodeTest
impl AnyChildNodeTest {
    // Java: static AnyChildNodeTest getInstance()
    pub fn getInstance() -> &'static AnyChildNodeTest {
      // Java: return instance;
      INSTANCE.get_or_init(|| AnyChildNodeTest::new())
    }

    // Java: AnyChildNodeTest()
    pub fn new() -> Self {
      // Java: super();
      AnyChildNodeTest {}
    }
}

impl super::Pattern::Pattern for AnyChildNodeTest {
    // Java: boolean matches(Object node, Context context)
    fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: short type = context.getNavigator().getNodeType(node);
      // Java: return type == 1 || type == 3 || type == 8 || type == 7;
      todo!("AnyChildNodeTest::matches")
    }

    // Java: double getPriority()
    fn getPriority(&self) -> f64 {
      // Java: return (double)-0.5F;
      -0.5
    }

    // Java: short getMatchType()
    fn getMatchType(&self) -> i16 {
      // Java: return 0;
      0
    }

    // Java: String getText()
    fn getText(&self) -> String {
      // Java: return "*";
      "*".to_string()
    }
}
