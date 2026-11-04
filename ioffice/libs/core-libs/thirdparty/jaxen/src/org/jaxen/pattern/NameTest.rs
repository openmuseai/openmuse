// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;

// Java: public class NameTest extends NodeTest
pub struct NameTest {
    // Java field: String name
    pub(crate) name: String,
    // Java field: short nodeType
    pub(crate) nodeType: i16,
}

// Java: public class NameTest extends NodeTest
impl NameTest {
    // Java: NameTest(String name, short nodeType)
    pub fn new(name: String, nodeType: i16) -> Self {
      // Java: super();
      // Java: this.name = name;
      // Java: this.nodeType = nodeType;
      NameTest { name, nodeType }
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: Navigator navigator = context.getNavigator();
      // Java: if (this.nodeType == 1)
      // Java: return navigator.isElement(node) && this.name.equals(navigator.getElementName(node));
      // Java: else if (this.nodeType != 2)
      // Java: if (navigator.isElement(node))
      // Java: return this.name.equals(navigator.getElementName(node));
      // Java: else
      // Java: return navigator.isAttribute(node) ? this.name.equals(navigator.getAttributeName(node)) : false;
      // Java: else
      // Java: return navigator.isAttribute(node) && this.name.equals(navigator.getAttributeName(node));
      todo!("NameTest::matches")
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
      // Java: return (double)0.0F;
      0.0
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return this.nodeType;
      self.nodeType
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.nodeType == 2 ? "@" + this.name : this.name;
      if self.nodeType == 2 {
         format!("@{}", self.name)
      } else {
         self.name.clone()
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + "[ name: " + this.name + " type: " + this.nodeType + " ]";
      format!("NameTest[ name: {} type: {} ]", self.name, self.nodeType)
    }
}
