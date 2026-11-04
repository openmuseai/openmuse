use super::DefaultRelationalExpr::DefaultRelationalExpr;

// Java field: long serialVersionUID

// Java: public class DefaultLessThanExpr extends DefaultRelationalExpr
pub struct DefaultLessThanExpr {
    pub(crate) base: DefaultRelationalExpr,
}

// Java: public class DefaultLessThanExpr extends DefaultRelationalExpr
impl DefaultLessThanExpr {
    // Java: DefaultLessThanExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultLessThanExpr {
         base: DefaultRelationalExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "<";
      "<".to_string()
    }

    // Java: boolean evaluateDoubleDouble(Double lhs, Double rhs)
    pub fn evaluateDoubleDouble(&self, lhs: f64, rhs: f64) -> bool {
      // Java: return lhs.compareTo(rhs) < 0;
      lhs < rhs
    }
}
