// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.UnresolvableException
use crate::org::jaxen::UnresolvableException::UnresolvableException;
use crate::org::jaxen::Object;

// Java field: long serialVersionUID

// Java: public class DefaultVariableReferenceExpr extends DefaultExpr implements VariableReferenceExpr
pub struct DefaultVariableReferenceExpr {
    // Java field: String prefix
    pub(crate) prefix: String,
    // Java field: String localName
    pub(crate) localName: String,
}

// Java: public class DefaultVariableReferenceExpr extends DefaultExpr implements VariableReferenceExpr
impl DefaultVariableReferenceExpr {
    // Java: DefaultVariableReferenceExpr(String prefix, String variableName)
    pub fn new(prefix: String, variableName: String) -> Self {
      // Java: super();
      // Java: this.prefix = prefix;
      // Java: this.localName = variableName;
      DefaultVariableReferenceExpr {
         prefix,
         localName: variableName,
      }
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> &str {
      // Java: return this.prefix;
      &self.prefix
    }

    // Java: String getVariableName()
    pub fn getVariableName(&self) -> &str {
      // Java: return this.localName;
      &self.localName
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultVariableReferenceExpr): " + this.getQName() + "]";
      format!("[(DefaultVariableReferenceExpr): {}]", self.getQName())
    }

    // Java: String getQName()
    fn getQName(&self) -> String {
      // Java: return "".equals(this.prefix) ? this.localName : this.prefix + ":" + this.localName;
      if self.prefix.is_empty() {
         self.localName.clone()
      } else {
         format!("{}:{}", self.prefix, self.localName)
      }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return "$" + this.getQName();
      format!("${}", self.getQName())
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, UnresolvableException> {
      // Java: String namespaceURI = context.translateNamespacePrefixToUri(this.getPrefix());
      // Java: return context.getVariableValue(namespaceURI, this.prefix, this.localName);
      todo!("DefaultVariableReferenceExpr::evaluate")
    }
}
