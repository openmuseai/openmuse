// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;

// Java: public class NamespaceTest extends NodeTest
pub struct NamespaceTest {
    // Java field: String prefix
    pub(crate) prefix: String,
    // Java field: short nodeType
    pub(crate) nodeType: i16,
}

// Java: public class NamespaceTest extends NodeTest
impl NamespaceTest {
    // Java: NamespaceTest(String prefix, short nodeType)
    pub fn new(prefix: Option<String>, nodeType: i16) -> Self {
      // Java: super();
      // Java: if (prefix == null)
      // Java: prefix = "";
      let prefix = prefix.unwrap_or_default();
      // Java: this.prefix = prefix;
      // Java: this.nodeType = nodeType;
      NamespaceTest { prefix, nodeType }
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: Navigator navigator = context.getNavigator();
      // Java: String uri = this.getURI(node, context);
      // Java: if (this.nodeType == 1)
      // Java: return navigator.isElement(node) && uri.equals(navigator.getElementNamespaceUri(node));
      // Java: else if (this.nodeType != 2)
      // Java: return false;
      // Java: else
      // Java: return navigator.isAttribute(node) && uri.equals(navigator.getAttributeNamespaceUri(node));
      todo!("NamespaceTest::matches")
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
      // Java: return (double)-0.25F;
      -0.25
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return this.nodeType;
      self.nodeType
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.prefix + ":";
      format!("{}:", self.prefix)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + "[ prefix: " + this.prefix + " type: " + this.nodeType + " ]";
      format!("NamespaceTest[ prefix: {} type: {} ]", self.prefix, self.nodeType)
    }

    // Java: String getURI(Object node, Context context)
    pub fn getURI(&self, _node: &Object, _context: &Context) -> String {
      // Java: String uri = context.getNavigator().translateNamespacePrefixToUri(this.prefix, node);
      // Java: if (uri == null)
      // Java: uri = context.getContextSupport().translateNamespacePrefixToUri(this.prefix);
      // Java: if (uri == null)
      // Java: uri = "";
      // Java: return uri;
      todo!("NamespaceTest::getURI")
    }
}
