// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use std::sync::OnceLock;

// Java field: TextNodeTest SINGLETON
static SINGLETON: OnceLock<TextNodeTest> = OnceLock::new();

// Java: public class TextNodeTest extends NodeTest
pub struct TextNodeTest {
}

// Java: public class TextNodeTest extends NodeTest
impl TextNodeTest {
    pub fn singleton() -> &'static TextNodeTest {
      SINGLETON.get_or_init(|| TextNodeTest::new())
    }

    // Java: TextNodeTest()
    pub fn new() -> Self {
      // Java: super();
      TextNodeTest {}
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: return context.getNavigator().isText(node);
      todo!("TextNodeTest::matches")
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
      // Java: return (double)-0.5F;
      -0.5
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return 3;
      3
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return "text()";
      "text()".to_string()
    }
}
