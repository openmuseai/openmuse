// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
use crate::org::jaxen::Object;
use super::DefaultAdditiveExpr::DefaultAdditiveExpr;

// Java field: long serialVersionUID

// Java: public class DefaultPlusExpr extends DefaultAdditiveExpr
pub struct DefaultPlusExpr {
    pub(crate) base: DefaultAdditiveExpr,
}

// Java: public class DefaultPlusExpr extends DefaultAdditiveExpr
impl DefaultPlusExpr {
    // Java: DefaultPlusExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultPlusExpr {
         base: DefaultAdditiveExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "+";
      "+".to_string()
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Number lhsValue = NumberFunction.evaluate(this.getLHS().evaluate(context), context.getNavigator());
      // Java: Number rhsValue = NumberFunction.evaluate(this.getRHS().evaluate(context), context.getNavigator());
      // Java: double result = lhsValue.doubleValue() + rhsValue.doubleValue();
      // Java: return new Double(result);
      todo!("DefaultPlusExpr::evaluate")
    }
}
