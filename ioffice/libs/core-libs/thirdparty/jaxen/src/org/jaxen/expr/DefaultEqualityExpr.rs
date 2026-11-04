// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.function.BooleanFunction
use crate::org::jaxen::function::BooleanFunction::BooleanFunction;
// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
// Java: import org.jaxen.function.StringFunction
use crate::org::jaxen::function::StringFunction::StringFunction;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;
use super::DefaultTruthExpr::DefaultTruthExpr;

// Java: public class DefaultEqualityExpr extends DefaultTruthExpr implements EqualityExpr
pub struct DefaultEqualityExpr {
    pub(crate) base: DefaultTruthExpr,
}

// Java: public class DefaultEqualityExpr extends DefaultTruthExpr implements EqualityExpr
impl DefaultEqualityExpr {
    // Java: DefaultEqualityExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultEqualityExpr {
         base: DefaultTruthExpr::new(lhs, rhs),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultEqualityExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultEqualityExpr): {}, {}]", self.base.base.getLHS().getText(), self.base.base.getRHS().getText())
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Object lhsValue = this.getLHS().evaluate(context);
      // Java: Object rhsValue = this.getRHS().evaluate(context);
      // Java: if (lhsValue != null && rhsValue != null)
      // Java: Navigator nav = context.getNavigator();
      // Java: if (this.bothAreSets(lhsValue, rhsValue))
      // Java: return this.evaluateSetSet((List)lhsValue, (List)rhsValue, nav);
      // Java: else if (this.isSet(lhsValue) && this.isBoolean(rhsValue))
      // Java: Boolean lhsBoolean = ((List)lhsValue).isEmpty() ? Boolean.FALSE : Boolean.TRUE;
      // Java: Boolean rhsBoolean = (Boolean)rhsValue;
      // Java: return this.evaluateObjectObject(lhsBoolean, rhsBoolean, nav);
      // Java: else if (this.isBoolean(lhsValue) && this.isSet(rhsValue))
      // Java: Boolean lhsBoolean = (Boolean)lhsValue;
      // Java: Boolean rhsBoolean = ((List)rhsValue).isEmpty() ? Boolean.FALSE : Boolean.TRUE;
      // Java: return this.evaluateObjectObject(lhsBoolean, rhsBoolean, nav);
      // Java: else if (this.eitherIsSet(lhsValue, rhsValue))
      // Java: return this.isSet(lhsValue) ? this.evaluateSetSet((List)lhsValue, convertToList(rhsValue), nav) : this.evaluateSetSet(convertToList(lhsValue), (List)rhsValue, nav);
      // Java: else
      // Java: return this.evaluateObjectObject(lhsValue, rhsValue, nav);
      // Java: else
      // Java: return Boolean.FALSE;
      todo!("DefaultEqualityExpr::evaluate")
    }

    // Java: Boolean evaluateSetSet(List lhsSet, List rhsSet, Navigator nav)
    pub fn evaluateSetSet(&self, _lhsSet: &List, _rhsSet: &List, _nav: Option<&dyn Navigator>) -> bool {
      // Java: if (!this.setIsEmpty(lhsSet) && !this.setIsEmpty(rhsSet))
      // Java: for(Object lhs : lhsSet)
      // Java: for(Object rhs : rhsSet)
      // Java: if (this.evaluateObjectObject(lhs, rhs, nav))
      // Java: return Boolean.TRUE;
      // Java: return Boolean.FALSE;
      // Java: else
      // Java: return Boolean.FALSE;
      todo!("DefaultEqualityExpr::evaluateSetSet")
    }

    // Java: boolean evaluateObjectObject(Object lhs, Object rhs, Navigator nav)
    pub fn evaluateObjectObject_with_nav(&self, _lhs: &Object, _rhs: &Object, _nav: Option<&dyn Navigator>) -> bool {
      // Java: if (this.eitherIsBoolean(lhs, rhs))
      // Java: return this.evaluateObjectObject(BooleanFunction.evaluate(lhs, nav), BooleanFunction.evaluate(rhs, nav));
      // Java: else
      // Java: return this.eitherIsNumber(lhs, rhs) ? this.evaluateObjectObject(NumberFunction.evaluate(lhs, nav), NumberFunction.evaluate(rhs, nav)) : this.evaluateObjectObject(StringFunction.evaluate(lhs, nav), StringFunction.evaluate(rhs, nav));
      todo!("DefaultEqualityExpr::evaluateObjectObject_with_nav")
    }

    // Java: abstract boolean evaluateObjectObject(Object var1, Object var2)
    pub fn evaluateObjectObject(&self, _var1: &Object, _var2: &Object) -> bool {
      todo!("DefaultEqualityExpr::evaluateObjectObject - abstract")
    }
}
