use super::DefaultBinaryExpr::DefaultBinaryExpr;
use super::Expr::Expr;

// Java: public class DefaultArithExpr extends DefaultBinaryExpr
pub struct DefaultArithExpr {
    pub(crate) base: DefaultBinaryExpr,
}

// Java: public class DefaultArithExpr extends DefaultBinaryExpr
impl DefaultArithExpr {
    // Java: DefaultArithExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn Expr>, rhs: Box<dyn Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultArithExpr {
         base: DefaultBinaryExpr::new(lhs, rhs),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultArithExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultArithExpr): {}, {}]", self.base.getLHS().getText(), self.base.getRHS().getText())
    }
}
