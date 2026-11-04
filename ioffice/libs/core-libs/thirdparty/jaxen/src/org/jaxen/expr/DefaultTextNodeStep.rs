// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.expr.iter.IterableAxis
use crate::org::jaxen::expr::iter::IterableAxis::IterableAxis;
use crate::org::jaxen::Object;
use super::DefaultStep::DefaultStep;
use super::PredicateSet::PredicateSet;

// Java field: long serialVersionUID

// Java: public class DefaultTextNodeStep extends DefaultStep implements TextNodeStep
pub struct DefaultTextNodeStep {
    pub(crate) base: DefaultStep,
}

// Java: public class DefaultTextNodeStep extends DefaultStep implements TextNodeStep
impl DefaultTextNodeStep {
    // Java: DefaultTextNodeStep(IterableAxis axis, PredicateSet predicateSet)
    pub fn new(axis: IterableAxis, predicateSet: PredicateSet) -> Self {
      // Java: super(axis, predicateSet);
      DefaultTextNodeStep {
         base: DefaultStep::new(axis, predicateSet),
      }
    }

    // Java: boolean matches(Object node, ContextSupport support)
    pub fn matches(&self, node: &Object, support: &ContextSupport) -> bool {
      // Java: Navigator nav = support.getNavigator();
      let nav = support.getNavigator();
      // Java: return nav.isText(node);
      match nav {
         Some(n) => n.isText(node),
         None => false,
      }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.getAxisName() + "::text()" + super.getText();
      format!("{}::text(){}", self.base.getAxisName(), self.base.getText())
    }
}
