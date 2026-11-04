use super::DefaultArithExpr::DefaultArithExpr;

// Java: public class DefaultMultiplicativeExpr extends DefaultArithExpr implements MultiplicativeExpr
pub struct DefaultMultiplicativeExpr {
    pub(crate) base: DefaultArithExpr,
}

// Java: public class DefaultMultiplicativeExpr extends DefaultArithExpr implements MultiplicativeExpr
impl DefaultMultiplicativeExpr {
    // Java: DefaultMultiplicativeExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultMultiplicativeExpr {
         base: DefaultArithExpr::new(lhs, rhs),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultMultiplicativeExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultMultiplicativeExpr): {}, {}]", self.base.base.getLHS().getText(), self.base.base.getRHS().getText())
    }
}
