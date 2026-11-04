// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.expr.iter.IterableAxis
use crate::org::jaxen::expr::iter::IterableAxis::IterableAxis;
use crate::org::jaxen::Object;
use super::DefaultStep::DefaultStep;
use super::PredicateSet::PredicateSet;

// Java field: long serialVersionUID

// Java: public class DefaultAllNodeStep extends DefaultStep implements AllNodeStep
pub struct DefaultAllNodeStep {
    pub(crate) base: DefaultStep,
}

// Java: public class DefaultAllNodeStep extends DefaultStep implements AllNodeStep
impl DefaultAllNodeStep {
    // Java: DefaultAllNodeStep(IterableAxis axis, PredicateSet predicateSet)
    pub fn new(axis: IterableAxis, predicateSet: PredicateSet) -> Self {
      // Java: super(axis, predicateSet);
      DefaultAllNodeStep {
         base: DefaultStep::new(axis, predicateSet),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultAllNodeStep): " + this.getAxisName() + "]";
      format!("[(DefaultAllNodeStep): {}]", self.base.getAxisName())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.getAxisName() + "::node()" + super.getText();
      format!("{}::node(){}", self.base.getAxisName(), self.base.getText())
    }

    // Java: boolean matches(Object node, ContextSupport contextSupport)
    pub fn matches(&self, _node: &Object, _contextSupport: &ContextSupport) -> bool {
      // Java: return true;
      true
    }
}
