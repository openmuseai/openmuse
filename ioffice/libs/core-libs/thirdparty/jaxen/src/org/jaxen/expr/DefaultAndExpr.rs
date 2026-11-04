// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.function.BooleanFunction
use crate::org::jaxen::function::BooleanFunction::BooleanFunction;
use crate::org::jaxen::Object;
use std::sync::Arc;
use super::DefaultLogicalExpr::DefaultLogicalExpr;

// Java field: long serialVersionUID

// Java: public class DefaultAndExpr extends DefaultLogicalExpr
pub struct DefaultAndExpr {
    pub(crate) base: DefaultLogicalExpr,
}

// Java: public class DefaultAndExpr extends DefaultLogicalExpr
impl DefaultAndExpr {
    // Java: DefaultAndExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultAndExpr {
         base: DefaultLogicalExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "and";
      "and".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultAndExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultAndExpr): {}, {}]", self.base.base.base.getLHS().getText(), self.base.base.base.getRHS().getText())
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Navigator nav = context.getNavigator();
      // Java: Boolean lhsValue = BooleanFunction.evaluate(this.getLHS().evaluate(context), nav);
      // Java: if (!lhsValue)
      // Java: return Boolean.FALSE;
      // Java: else
      // Java: Boolean rhsValue = BooleanFunction.evaluate(this.getRHS().evaluate(context), nav);
      // Java: return !rhsValue ? Boolean.FALSE : Boolean.TRUE;
      todo!("DefaultAndExpr::evaluate")
    }
}
