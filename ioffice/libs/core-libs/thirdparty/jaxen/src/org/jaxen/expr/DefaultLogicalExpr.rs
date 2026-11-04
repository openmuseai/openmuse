use super::DefaultTruthExpr::DefaultTruthExpr;

// Java: public class DefaultLogicalExpr extends DefaultTruthExpr implements LogicalExpr
pub struct DefaultLogicalExpr {
    pub(crate) base: DefaultTruthExpr,
}

// Java: public class DefaultLogicalExpr extends DefaultTruthExpr implements LogicalExpr
impl DefaultLogicalExpr {
    // Java: DefaultLogicalExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultLogicalExpr {
         base: DefaultTruthExpr::new(lhs, rhs),
      }
    }
}
