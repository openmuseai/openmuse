use super::DefaultExpr::DefaultExpr;
use super::Expr::Expr;

// Java: public class DefaultBinaryExpr extends DefaultExpr implements BinaryExpr
pub struct DefaultBinaryExpr {
    pub(crate) base: DefaultExpr,
    // Java field: Expr lhs
    pub(crate) lhs: Box<dyn Expr>,
    // Java field: Expr rhs
    pub(crate) rhs: Box<dyn Expr>,
}

// Java: public class DefaultBinaryExpr extends DefaultExpr implements BinaryExpr
impl DefaultBinaryExpr {
    // Java: DefaultBinaryExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn Expr>, rhs: Box<dyn Expr>) -> Self {
      // Java: super();
      // Java: this.lhs = lhs;
      // Java: this.rhs = rhs;
      DefaultBinaryExpr {
         base: DefaultExpr::new(),
         lhs,
         rhs,
      }
    }

    // Java: Expr getLHS()
    pub fn getLHS(&self) -> &dyn Expr {
      // Java: return this.lhs;
      &*self.lhs
    }

    // Java: Expr getRHS()
    pub fn getRHS(&self) -> &dyn Expr {
      // Java: return this.rhs;
      &*self.rhs
    }

    // Java: void setLHS(Expr lhs)
    pub fn setLHS(&mut self, lhs: Box<dyn Expr>) {
      // Java: this.lhs = lhs;
      self.lhs = lhs;
    }

    // Java: void setRHS(Expr rhs)
    pub fn setRHS(&mut self, rhs: Box<dyn Expr>) {
      // Java: this.rhs = rhs;
      self.rhs = rhs;
    }

    // Java: abstract String getOperator()

    // Java: String getText()
    pub fn getText(&self, operator: &str) -> String {
      // Java: return "(" + this.getLHS().getText() + " " + this.getOperator() + " " + this.getRHS().getText() + ")";
      format!("({} {} {})", self.getLHS().getText(), operator, self.getRHS().getText())
    }

    // Java: String toString()
    pub fn toString(&self, class_name: &str) -> String {
      // Java: return "[" + this.getClass().getName() + ": " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[{}: {}, {}]", class_name, self.getLHS().getText(), self.getRHS().getText())
    }

    // Java: Expr simplify()
    // Java: this.setLHS(this.getLHS().simplify());
    // Java: this.setRHS(this.getRHS().simplify());
    // Java: return this;
}
