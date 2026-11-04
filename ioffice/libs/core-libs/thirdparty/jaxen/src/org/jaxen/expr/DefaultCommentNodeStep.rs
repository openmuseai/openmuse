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

// Java: public class DefaultCommentNodeStep extends DefaultStep implements CommentNodeStep
pub struct DefaultCommentNodeStep {
    pub(crate) base: DefaultStep,
}

// Java: public class DefaultCommentNodeStep extends DefaultStep implements CommentNodeStep
impl DefaultCommentNodeStep {
    // Java: DefaultCommentNodeStep(IterableAxis axis, PredicateSet predicateSet)
    pub fn new(axis: IterableAxis, predicateSet: PredicateSet) -> Self {
      // Java: super(axis, predicateSet);
      DefaultCommentNodeStep {
         base: DefaultStep::new(axis, predicateSet),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultCommentNodeStep): " + this.getAxis() + "]";
      format!("[(DefaultCommentNodeStep): {}]", self.base.getAxis())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.getAxisName() + "::comment()";
      format!("{}::comment()", self.base.getAxisName())
    }

    // Java: boolean matches(Object node, ContextSupport contextSupport)
    pub fn matches(&self, node: &Object, contextSupport: &ContextSupport) -> bool {
      // Java: Navigator nav = contextSupport.getNavigator();
      let nav = contextSupport.getNavigator();
      // Java: return nav.isComment(node);
      match nav {
         Some(n) => n.isComment(node),
         None => false,
      }
    }
}
