// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use std::sync::OnceLock;

// Java field: NoNodeTest instance
static INSTANCE: OnceLock<NoNodeTest> = OnceLock::new();

// Java: public class NoNodeTest extends NodeTest
pub struct NoNodeTest {
}

// Java: public class NoNodeTest extends NodeTest
impl NoNodeTest {
    // Java: static NoNodeTest getInstance()
    pub fn getInstance() -> &'static NoNodeTest {
      // Java: return instance;
      INSTANCE.get_or_init(|| NoNodeTest::new())
    }

    // Java: NoNodeTest()
    pub fn new() -> Self {
      // Java: super();
      NoNodeTest {}
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: return false;
      Ok(false)
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
      // Java: return (double)-0.5F;
      -0.5
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return 14;
      14
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return "";
      String::new()
    }
}
