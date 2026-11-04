use super::DefaultRelationalExpr::DefaultRelationalExpr;

// Java field: long serialVersionUID

// Java: public class DefaultLessThanEqualExpr extends DefaultRelationalExpr
pub struct DefaultLessThanEqualExpr {
    pub(crate) base: DefaultRelationalExpr,
}

// Java: public class DefaultLessThanEqualExpr extends DefaultRelationalExpr
impl DefaultLessThanEqualExpr {
    // Java: DefaultLessThanEqualExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultLessThanEqualExpr {
         base: DefaultRelationalExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "<=";
      "<=".to_string()
    }

    // Java: boolean evaluateDoubleDouble(Double lhs, Double rhs)
    pub fn evaluateDoubleDouble(&self, lhs: f64, rhs: f64) -> bool {
      // Java: return lhs.compareTo(rhs) <= 0;
      lhs <= rhs
    }
}
