// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use std::sync::OnceLock;

// Java field: AnyNodeTest instance
static INSTANCE: OnceLock<AnyNodeTest> = OnceLock::new();

// Java: public class AnyNodeTest extends NodeTest
pub struct AnyNodeTest {
}

// Java: public class AnyNodeTest extends NodeTest
impl AnyNodeTest {
    // Java: static AnyNodeTest getInstance()
    pub fn getInstance() -> &'static AnyNodeTest {
      // Java: return instance;
      INSTANCE.get_or_init(|| AnyNodeTest::new())
    }

    // Java: AnyNodeTest()
    pub fn new() -> Self {
      // Java: super();
      AnyNodeTest {}
    }
}

impl super::Pattern::Pattern for AnyNodeTest {
    // Java: boolean matches(Object node, Context context)
    fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: return true;
      Ok(true)
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
