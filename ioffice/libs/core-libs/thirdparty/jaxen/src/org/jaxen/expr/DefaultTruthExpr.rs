// Java: import java.util.List
use crate::org::jaxen::{Object, List};
use super::DefaultBinaryExpr::DefaultBinaryExpr;

// Java: public class DefaultTruthExpr extends DefaultBinaryExpr
pub struct DefaultTruthExpr {
    pub(crate) base: DefaultBinaryExpr,
}

// Java: public class DefaultTruthExpr extends DefaultBinaryExpr
impl DefaultTruthExpr {
    // Java: DefaultTruthExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultTruthExpr {
         base: DefaultBinaryExpr::new(lhs, rhs),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultTruthExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultTruthExpr): {}, {}]", self.base.getLHS().getText(), self.base.getRHS().getText())
    }

    // Java: boolean bothAreSets(Object lhs, Object rhs)
    pub fn bothAreSets(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: return lhs instanceof List && rhs instanceof List;
      lhs.downcast_ref::<List>().is_some() && rhs.downcast_ref::<List>().is_some()
    }

    // Java: boolean eitherIsSet(Object lhs, Object rhs)
    pub fn eitherIsSet(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: return lhs instanceof List || rhs instanceof List;
      lhs.downcast_ref::<List>().is_some() || rhs.downcast_ref::<List>().is_some()
    }

    // Java: boolean isSet(Object obj)
    pub fn isSet(&self, obj: &Object) -> bool {
      // Java: return obj instanceof List;
      obj.downcast_ref::<List>().is_some()
    }

    // Java: boolean isBoolean(Object obj)
    pub fn isBoolean(&self, obj: &Object) -> bool {
      // Java: return obj instanceof Boolean;
      obj.downcast_ref::<bool>().is_some()
    }

    // Java: boolean setIsEmpty(List set)
    pub fn setIsEmpty(&self, set: &List) -> bool {
      // Java: return set == null || set.size() == 0;
      set.is_empty()
    }

    // Java: boolean eitherIsBoolean(Object lhs, Object rhs)
    pub fn eitherIsBoolean(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: return lhs instanceof Boolean || rhs instanceof Boolean;
      lhs.downcast_ref::<bool>().is_some() || rhs.downcast_ref::<bool>().is_some()
    }

    // Java: boolean bothAreBoolean(Object lhs, Object rhs)
    pub fn bothAreBoolean(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: return lhs instanceof Boolean && rhs instanceof Boolean;
      lhs.downcast_ref::<bool>().is_some() && rhs.downcast_ref::<bool>().is_some()
    }

    // Java: boolean eitherIsNumber(Object lhs, Object rhs)
    pub fn eitherIsNumber(&self, lhs: &Object, rhs: &Object) -> bool {
      // Java: return lhs instanceof Number || rhs instanceof Number;
      lhs.downcast_ref::<f64>().is_some() || rhs.downcast_ref::<f64>().is_some()
    }
}
