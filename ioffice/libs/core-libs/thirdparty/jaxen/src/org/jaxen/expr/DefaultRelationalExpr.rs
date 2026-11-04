// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;
use super::DefaultTruthExpr::DefaultTruthExpr;

// Java: public class DefaultRelationalExpr extends DefaultTruthExpr implements RelationalExpr
pub struct DefaultRelationalExpr {
    pub(crate) base: DefaultTruthExpr,
}

// Java: public class DefaultRelationalExpr extends DefaultTruthExpr implements RelationalExpr
impl DefaultRelationalExpr {
    // Java: DefaultRelationalExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultRelationalExpr {
         base: DefaultTruthExpr::new(lhs, rhs),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultRelationalExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultRelationalExpr): {}, {}]", self.base.base.getLHS().getText(), self.base.base.getRHS().getText())
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Object lhsValue = this.getLHS().evaluate(context);
      // Java: Object rhsValue = this.getRHS().evaluate(context);
      // Java: Navigator nav = context.getNavigator();
      // Java: if (this.bothAreSets(lhsValue, rhsValue))
      // Java: return this.evaluateSetSet((List)lhsValue, (List)rhsValue, nav);
      // Java: else if (this.eitherIsSet(lhsValue, rhsValue))
      // Java: return this.isSet(lhsValue) ? this.evaluateSetSet((List)lhsValue, convertToList(rhsValue), nav) : this.evaluateSetSet(convertToList(lhsValue), (List)rhsValue, nav);
      // Java: else
      // Java: return this.evaluateObjectObject(lhsValue, rhsValue, nav) ? Boolean.TRUE : Boolean.FALSE;
      todo!("DefaultRelationalExpr::evaluate")
    }

    // Java: Object evaluateSetSet(List lhsSet, List rhsSet, Navigator nav)
    pub fn evaluateSetSet(&self, _lhsSet: &List, _rhsSet: &List, _nav: Option<&dyn Navigator>) -> Object {
      // Java: if (!this.setIsEmpty(lhsSet) && !this.setIsEmpty(rhsSet))
      // Java: for(Object lhs : lhsSet)
      // Java: for(Object rhs : rhsSet)
      // Java: if (this.evaluateObjectObject(lhs, rhs, nav))
      // Java: return Boolean.TRUE;
      // Java: return Boolean.FALSE;
      // Java: else
      // Java: return Boolean.FALSE;
      todo!("DefaultRelationalExpr::evaluateSetSet")
    }

    // Java: boolean evaluateObjectObject(Object lhs, Object rhs, Navigator nav)
    pub fn evaluateObjectObject(&self, _lhs: &Object, _rhs: &Object, _nav: Option<&dyn Navigator>) -> bool {
      // Java: if (lhs != null && rhs != null)
      // Java: Double lhsNum = NumberFunction.evaluate(lhs, nav);
      // Java: Double rhsNum = NumberFunction.evaluate(rhs, nav);
      // Java: return !NumberFunction.isNaN(lhsNum) && !NumberFunction.isNaN(rhsNum) ? this.evaluateDoubleDouble(lhsNum, rhsNum) : false;
      // Java: else
      // Java: return false;
      todo!("DefaultRelationalExpr::evaluateObjectObject")
    }

    // Java: abstract boolean evaluateDoubleDouble(Double var1, Double var2)
    pub fn evaluateDoubleDouble(&self, _var1: f64, _var2: f64) -> bool {
      todo!("DefaultRelationalExpr::evaluateDoubleDouble - abstract")
    }
}
