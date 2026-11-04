// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
use crate::org::jaxen::Object;
use super::DefaultEqualityExpr::DefaultEqualityExpr;

// Java field: long serialVersionUID

// Java: public class DefaultNotEqualsExpr extends DefaultEqualityExpr
pub struct DefaultNotEqualsExpr {
    pub(crate) base: DefaultEqualityExpr,
}

// Java: public class DefaultNotEqualsExpr extends DefaultEqualityExpr
impl DefaultNotEqualsExpr {
    // Java: DefaultNotEqualsExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultNotEqualsExpr {
         base: DefaultEqualityExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "!=";
      "!=".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultNotEqualsExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultNotEqualsExpr): {}, {}]", self.base.base.base.getLHS().getText(), self.base.base.base.getRHS().getText())
    }

    // Java: boolean evaluateObjectObject(Object lhs, Object rhs)
    pub fn evaluateObjectObject(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: if (!this.eitherIsNumber(lhs, rhs) || !NumberFunction.isNaN((Double)lhs) && !NumberFunction.isNaN((Double)rhs))
      if !self.base.base.eitherIsNumber(lhs, rhs) ||
         (lhs.downcast_ref::<f64>().map_or(false, |v| !v.is_nan()) &&
          rhs.downcast_ref::<f64>().map_or(false, |v| !v.is_nan())) {
         // Java: return !lhs.equals(rhs);
         if let (Some(&l), Some(&r)) = (lhs.downcast_ref::<f64>(), rhs.downcast_ref::<f64>()) {
            return l != r;
         }
         return lhs.downcast_ref::<String>() != rhs.downcast_ref::<String>();
      }
      // Java: else
      else {
         // Java: return true;
         true
      }
    }
}
