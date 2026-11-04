// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::Object;
use std::sync::Arc;

// Java field: long serialVersionUID

// Java: public class DefaultLiteralExpr extends DefaultExpr implements LiteralExpr
pub struct DefaultLiteralExpr {
    // Java field: String literal
    pub(crate) literal: String,
}

// Java: public class DefaultLiteralExpr extends DefaultExpr implements LiteralExpr
impl DefaultLiteralExpr {
    // Java: DefaultLiteralExpr(String literal)
    pub fn new(literal: String) -> Self {
      // Java: super();
      // Java: this.literal = literal;
      DefaultLiteralExpr { literal }
    }

    // Java: String getLiteral()
    pub fn getLiteral(&self) -> &str {
      // Java: return this.literal;
      &self.literal
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultLiteralExpr): " + this.getLiteral() + "]";
      format!("[(DefaultLiteralExpr): {}]", self.getLiteral())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.literal.indexOf(34) == -1 ? "\"" + this.getLiteral() + "\"" : "'" + this.getLiteral() + "'";
      if !self.literal.contains('"') {
         format!("\"{}\"", self.getLiteral())
      } else {
         format!("'{}'", self.getLiteral())
      }
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Object {
      // Java: return this.getLiteral();
      Arc::new(self.literal.clone())
    }
}
