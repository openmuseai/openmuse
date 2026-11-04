// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::List;
use super::Expr::Expr;

// Java field: long serialVersionUID

// Java: public class DefaultXPathExpr implements XPathExpr
pub struct DefaultXPathExpr {
    // Java field: Expr rootExpr
    pub(crate) rootExpr: Box<dyn Expr>,
}

// Java: public class DefaultXPathExpr implements XPathExpr
impl DefaultXPathExpr {
    // Java: DefaultXPathExpr(Expr rootExpr)
    pub fn new(rootExpr: Box<dyn Expr>) -> Self {
      // Java: super();
      // Java: this.rootExpr = rootExpr;
      DefaultXPathExpr { rootExpr }
    }

    // Java: Expr getRootExpr()
    pub fn getRootExpr(&self) -> &dyn Expr {
      // Java: return this.rootExpr;
      &*self.rootExpr
    }

    // Java: void setRootExpr(Expr rootExpr)
    pub fn setRootExpr(&mut self, rootExpr: Box<dyn Expr>) {
      // Java: this.rootExpr = rootExpr;
      self.rootExpr = rootExpr;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultXPath): " + this.getRootExpr() + "]";
      format!("[(DefaultXPath): {}]", self.getRootExpr().getText())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.getRootExpr().getText();
      self.getRootExpr().getText()
    }

    // Java: void simplify()
    pub fn simplify(&mut self) {
      // Java: this.setRootExpr(this.getRootExpr().simplify());
    }

    // Java: List asList(Context context)
    pub fn asList(&self, _context: &Context) -> Result<List, JaxenException> {
      // Java: Expr expr = this.getRootExpr();
      // Java: Object value = expr.evaluate(context);
      // Java: List result = DefaultExpr.convertToList(value);
      // Java: return result;
      todo!("DefaultXPathExpr::asList")
    }
}
