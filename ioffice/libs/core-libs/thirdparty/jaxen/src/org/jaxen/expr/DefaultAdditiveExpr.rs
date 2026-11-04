use super::DefaultArithExpr::DefaultArithExpr;
use super::Expr::Expr;

// Java: public class DefaultAdditiveExpr extends DefaultArithExpr implements AdditiveExpr
pub struct DefaultAdditiveExpr {
    pub(crate) base: DefaultArithExpr,
}

// Java: public class DefaultAdditiveExpr extends DefaultArithExpr implements AdditiveExpr
impl DefaultAdditiveExpr {
    // Java: DefaultAdditiveExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn Expr>, rhs: Box<dyn Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultAdditiveExpr {
         base: DefaultArithExpr::new(lhs, rhs),
      }
    }

    // Java: String toString()
    pub fn toString(&self, class_name: &str) -> String {
      // Java: return "[(" + this.getClass().getName() + "): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[({}): {}, {}]", class_name, self.base.base.getLHS().getText(), self.base.base.getRHS().getText())
    }
}
