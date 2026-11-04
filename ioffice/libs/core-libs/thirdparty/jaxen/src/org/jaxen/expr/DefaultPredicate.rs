// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use super::Expr::Expr;

// Java field: long serialVersionUID

// Java: public class DefaultPredicate implements Predicate
pub struct DefaultPredicate {
    // Java field: Expr expr
    pub(crate) expr: Box<dyn Expr>,
}

// Java: public class DefaultPredicate implements Predicate
impl DefaultPredicate {
    // Java: DefaultPredicate(Expr expr)
    pub fn new(expr: Box<dyn Expr>) -> Self {
      // Java: super();
      // Java: this.setExpr(expr);
      DefaultPredicate { expr }
    }

    // Java: Expr getExpr()
    pub fn getExpr(&self) -> &dyn Expr {
      // Java: return this.expr;
      &*self.expr
    }

    // Java: void setExpr(Expr expr)
    pub fn setExpr(&mut self, expr: Box<dyn Expr>) {
      // Java: this.expr = expr;
      self.expr = expr;
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return "[" + this.getExpr().getText() + "]";
      format!("[{}]", self.getExpr().getText())
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultPredicate): " + this.getExpr() + "]";
      format!("[(DefaultPredicate): {}]", self.getExpr().getText())
    }

    // Java: void simplify()
    pub fn simplify(&mut self) {
      // Java: this.setExpr(this.getExpr().simplify());
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: return this.getExpr().evaluate(context);
      todo!("DefaultPredicate::evaluate")
    }
}
