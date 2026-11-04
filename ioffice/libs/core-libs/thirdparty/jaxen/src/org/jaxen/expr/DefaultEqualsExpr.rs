// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
use crate::org::jaxen::Object;
use super::DefaultEqualityExpr::DefaultEqualityExpr;

// Java field: long serialVersionUID

// Java: public class DefaultEqualsExpr extends DefaultEqualityExpr
pub struct DefaultEqualsExpr {
    pub(crate) base: DefaultEqualityExpr,
}

// Java: public class DefaultEqualsExpr extends DefaultEqualityExpr
impl DefaultEqualsExpr {
    // Java: DefaultEqualsExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultEqualsExpr {
         base: DefaultEqualityExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "=";
      "=".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultEqualsExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultEqualsExpr): {}, {}]", self.base.base.base.getLHS().getText(), self.base.base.base.getRHS().getText())
    }

    // Java: boolean evaluateObjectObject(Object lhs, Object rhs)
    pub fn evaluateObjectObject(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: return !this.eitherIsNumber(lhs, rhs) || !NumberFunction.isNaN((Double)lhs) && !NumberFunction.isNaN((Double)rhs) ? lhs.equals(rhs) : false;
      if !self.base.base.eitherIsNumber(lhs, rhs) {
         return lhs.downcast_ref::<String>() == rhs.downcast_ref::<String>();
      }
      if let (Some(&l), Some(&r)) = (lhs.downcast_ref::<f64>(), rhs.downcast_ref::<f64>()) {
         if l.is_nan() || r.is_nan() {
            return false;
         }
         l == r
      } else {
         false
      }
    }
}
