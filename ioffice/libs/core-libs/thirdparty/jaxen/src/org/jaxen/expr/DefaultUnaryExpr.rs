// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
use crate::org::jaxen::Object;
use super::Expr::Expr;

// Java field: long serialVersionUID

// Java: public class DefaultUnaryExpr extends DefaultExpr implements UnaryExpr
pub struct DefaultUnaryExpr {
    // Java field: Expr expr
    pub(crate) expr: Box<dyn Expr>,
}

// Java: public class DefaultUnaryExpr extends DefaultExpr implements UnaryExpr
impl DefaultUnaryExpr {
    // Java: DefaultUnaryExpr(Expr expr)
    pub fn new(expr: Box<dyn Expr>) -> Self {
      // Java: super();
      // Java: this.expr = expr;
      DefaultUnaryExpr { expr }
    }

    // Java: Expr getExpr()
    pub fn getExpr(&self) -> &dyn Expr {
      // Java: return this.expr;
      &*self.expr
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultUnaryExpr): " + this.getExpr() + "]";
      format!("[(DefaultUnaryExpr): {}]", self.getExpr().getText())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return "-(" + this.getExpr().getText() + ")";
      format!("-({})", self.getExpr().getText())
    }

    // Java: Expr simplify()
    pub fn simplify(&mut self) {
      // Java: this.expr = this.expr.simplify();
      // Java: return this;
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Number number = NumberFunction.evaluate(this.getExpr().evaluate(context), context.getNavigator());
      // Java: return new Double(number.doubleValue() * (double)-1.0F);
      todo!("DefaultUnaryExpr::evaluate")
    }
}
