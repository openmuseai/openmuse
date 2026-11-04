// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::Object;
use std::sync::Arc;

// Java field: long serialVersionUID

// Java: public class DefaultNumberExpr extends DefaultExpr implements NumberExpr
pub struct DefaultNumberExpr {
    // Java field: Double number
    pub(crate) number: f64,
}

// Java: public class DefaultNumberExpr extends DefaultExpr implements NumberExpr
impl DefaultNumberExpr {
    // Java: DefaultNumberExpr(Double number)
    pub fn new(number: f64) -> Self {
      // Java: super();
      // Java: this.number = number;
      DefaultNumberExpr { number }
    }

    // Java: Number getNumber()
    pub fn getNumber(&self) -> f64 {
      // Java: return this.number;
      self.number
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultNumberExpr): " + this.getNumber() + "]";
      format!("[(DefaultNumberExpr): {}]", self.getNumber())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.getNumber().toString();
      self.getNumber().to_string()
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Object {
      // Java: return this.getNumber();
      Arc::new(self.getNumber())
    }
}
