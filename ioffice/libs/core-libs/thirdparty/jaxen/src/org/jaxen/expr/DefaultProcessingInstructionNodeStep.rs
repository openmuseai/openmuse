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

// Java: public class DefaultProcessingInstructionNodeStep extends DefaultStep implements ProcessingInstructionNodeStep
pub struct DefaultProcessingInstructionNodeStep {
    pub(crate) base: DefaultStep,
    // Java field: String name
    pub(crate) name: String,
}

// Java: public class DefaultProcessingInstructionNodeStep extends DefaultStep implements ProcessingInstructionNodeStep
impl DefaultProcessingInstructionNodeStep {
    // Java: DefaultProcessingInstructionNodeStep(IterableAxis axis, String name, PredicateSet predicateSet)
    pub fn new(axis: IterableAxis, name: String, predicateSet: PredicateSet) -> Self {
      // Java: super(axis, predicateSet);
      // Java: this.name = name;
      DefaultProcessingInstructionNodeStep {
         base: DefaultStep::new(axis, predicateSet),
         name,
      }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.name;
      &self.name
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: buf.append(this.getAxisName());
      buf.push_str(&self.base.getAxisName());
      // Java: buf.append("::processing-instruction(");
      buf.push_str("::processing-instruction(");
      // Java: String name = this.getName();
      let name = self.getName();
      // Java: if (name != null && name.length() != 0)
      if !name.is_empty() {
         // Java: buf.append("'");
         buf.push('\'');
         // Java: buf.append(name);
         buf.push_str(name);
         // Java: buf.append("'");
         buf.push('\'');
      }
      // Java: buf.append(")");
      buf.push(')');
      // Java: buf.append(super.getText());
      buf.push_str(&self.base.getText());
      // Java: return buf.toString();
      buf
    }

    // Java: boolean matches(Object node, ContextSupport support)
    pub fn matches(&self, node: &Object, support: &ContextSupport) -> bool {
      // Java: Navigator nav = support.getNavigator();
      let nav = support.getNavigator();
      // Java: if (nav.isProcessingInstruction(node))
      if let Some(nav) = nav {
         if nav.isProcessingInstruction(node) {
            // Java: String name = this.getName();
            let name = self.getName();
            // Java: return name != null && name.length() != 0 ? name.equals(nav.getProcessingInstructionTarget(node)) : true;
            if !name.is_empty() {
               name == nav.getProcessingInstructionTarget(node)
            } else {
               true
            }
         }
         // Java: else
         else {
            // Java: return false;
            false
         }
      } else {
         false
      }
    }
}
