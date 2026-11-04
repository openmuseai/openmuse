// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::{Object, List};
use super::Expr::Expr;
use super::PredicateSet::PredicateSet;
use super::Predicate::Predicate;

// Java field: long serialVersionUID

// Java: public class DefaultFilterExpr extends DefaultExpr implements FilterExpr, Predicated
pub struct DefaultFilterExpr {
    // Java field: Expr expr
    pub(crate) expr: Option<Box<dyn Expr>>,
    // Java field: PredicateSet predicates
    pub(crate) predicates: PredicateSet,
}

// Java: public class DefaultFilterExpr extends DefaultExpr implements FilterExpr, Predicated
impl DefaultFilterExpr {
    // Java: DefaultFilterExpr(PredicateSet predicateSet)
    pub fn new_with_predicates(predicateSet: PredicateSet) -> Self {
      // Java: super();
      // Java: this.predicates = predicateSet;
      DefaultFilterExpr {
         expr: None,
         predicates: predicateSet,
      }
    }

    // Java: DefaultFilterExpr(Expr expr, PredicateSet predicateSet)
    pub fn new(expr: Box<dyn Expr>, predicateSet: PredicateSet) -> Self {
      // Java: super();
      // Java: this.expr = expr;
      // Java: this.predicates = predicateSet;
      DefaultFilterExpr {
         expr: Some(expr),
         predicates: predicateSet,
      }
    }

    // Java: void addPredicate(Predicate predicate)
    pub fn addPredicate(&mut self, predicate: Box<dyn Predicate>) {
      // Java: this.predicates.addPredicate(predicate);
      self.predicates.addPredicate(predicate);
    }

    // Java: List getPredicates()
    pub fn getPredicates(&self) -> &Vec<Box<dyn Predicate>> {
      // Java: return this.predicates.getPredicates();
      self.predicates.getPredicates()
    }

    // Java: PredicateSet getPredicateSet()
    pub fn getPredicateSet(&self) -> &PredicateSet {
      // Java: return this.predicates;
      &self.predicates
    }

    // Java: Expr getExpr()
    pub fn getExpr(&self) -> Option<&dyn Expr> {
      // Java: return this.expr;
      self.expr.as_deref()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultFilterExpr): expr: " + this.expr + " predicates: " + this.predicates + " ]";
      format!("[(DefaultFilterExpr): expr: {:?} predicates: ... ]", self.expr.as_ref().map(|e| e.getText()))
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: String text = "";
      let mut text = String::new();
      // Java: if (this.expr != null)
      if let Some(ref expr) = self.expr {
         // Java: text = this.expr.getText();
         text = expr.getText();
      }
      // Java: text = text + this.predicates.getText();
      text.push_str(&self.predicates.getText());
      // Java: return text;
      text
    }

    // Java: Expr simplify()
    pub fn simplify(&mut self) {
      // Java: this.predicates.simplify();
      self.predicates.simplify();
      // Java: if (this.expr != null)
      // Java: this.expr = this.expr.simplify();
      // Java: return (Expr)(this.predicates.getPredicates().size() == 0 ? this.getExpr() : this);
    }

    // Java: boolean asBoolean(Context context)
    pub fn asBoolean(&self, _context: &Context) -> Result<bool, JaxenException> {
      // Java: Object results = null;
      // Java: if (this.expr != null)
      // Java: results = this.expr.evaluate(context);
      // Java: else
      // Java: List nodeSet = context.getNodeSet();
      // Java: ArrayList list = new ArrayList(nodeSet.size());
      // Java: list.addAll(nodeSet);
      // Java: results = list;
      // Java: if (results instanceof Boolean)
      // Java: Boolean b = (Boolean)results;
      // Java: return b;
      // Java: else
      // Java: return results instanceof List ? this.getPredicateSet().evaluateAsBoolean((List)results, context.getContextSupport()) : false;
      todo!("DefaultFilterExpr::asBoolean")
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Object results = this.getExpr().evaluate(context);
      // Java: if (results instanceof List)
      // Java: List newresults = this.getPredicateSet().evaluatePredicates((List)results, context.getContextSupport());
      // Java: results = newresults;
      // Java: return results;
      todo!("DefaultFilterExpr::evaluate")
    }
}
