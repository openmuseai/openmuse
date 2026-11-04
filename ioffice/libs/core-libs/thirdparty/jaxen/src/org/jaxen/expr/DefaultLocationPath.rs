// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.LinkedList
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::{Object, List};
use super::DefaultExpr::DefaultExpr;
use std::sync::Arc;

// Java: public class DefaultLocationPath extends DefaultExpr implements LocationPath
pub struct DefaultLocationPath {
    pub(crate) base: DefaultExpr,
    // Java field: List<Step> steps
    pub(crate) steps: Vec<Box<dyn super::Step::Step>>,
}

// Java: public class DefaultLocationPath extends DefaultExpr implements LocationPath
impl DefaultLocationPath {
    // Java: DefaultLocationPath()
    pub fn new() -> Self {
      // Java: super();
      DefaultLocationPath {
         base: DefaultExpr::new(),
         steps: Vec::new(),
      }
    }

    // Java: void addStep(Step step)
    pub fn addStep(&mut self, step: Box<dyn super::Step::Step>) {
      // Java: this.getSteps().add(step);
      self.steps.push(step);
    }

    // Java: List getSteps()
    pub fn getSteps(&self) -> &Vec<Box<dyn super::Step::Step>> {
      // Java: return this.steps;
      &self.steps
    }

    // Java: Expr simplify()
    pub fn simplify(&mut self) {
      // Java: Iterator stepIter = this.getSteps().iterator();
      // Java: Step eachStep = null;
      // Java: while(stepIter.hasNext())
      for step in self.steps.iter_mut() {
         // Java: eachStep = (Step)stepIter.next();
         // Java: eachStep.simplify();
         step.simplify();
      }
      // Java: return this;
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: Iterator stepIter = this.getSteps().iterator();
      let len = self.steps.len();
      // Java: while(stepIter.hasNext())
      for (i, step) in self.steps.iter().enumerate() {
         // Java: buf.append(((Step)stepIter.next()).getText());
         buf.push_str(&step.getText());
         // Java: if (stepIter.hasNext())
         if i + 1 < len {
            // Java: buf.append("/");
            buf.push('/');
         }
      }
      // Java: return buf.toString();
      buf
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: Iterator stepIter = this.getSteps().iterator();
      let len = self.steps.len();
      // Java: while(stepIter.hasNext())
      for (i, step) in self.steps.iter().enumerate() {
         // Java: buf.append(stepIter.next().toString());
         buf.push_str(&step.getText());
         // Java: if (stepIter.hasNext())
         if i + 1 < len {
            // Java: buf.append("/");
            buf.push('/');
         }
      }
      // Java: return buf.toString();
      buf
    }

    // Java: boolean isAbsolute()
    pub fn isAbsolute(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: List nodeSet = context.getNodeSet();
      // Java: List contextNodeSet = new ArrayList(nodeSet);
      // Java: ContextSupport support = context.getContextSupport();
      // Java: Context stepContext = new Context(support);
      // Java: for(Object eachStepObj : this.getSteps())
      // Java: Step eachStep = (Step)eachStepObj;
      // Java: stepContext.setNodeSet(contextNodeSet);
      // Java: contextNodeSet = eachStep.evaluate(stepContext);
      // Java: if (this.isReverseAxis(eachStep))
      // Java: Collections.reverse(contextNodeSet);
      // Java: if (this.getSteps().size() > 1 || nodeSet.size() > 1)
      // Java: Collections.sort(contextNodeSet, new NodeComparator(support.getNavigator()));
      // Java: return contextNodeSet;
      todo!("DefaultLocationPath::evaluate")
    }

    // Java: boolean isReverseAxis(Step step)
    fn isReverseAxis(&self, step: &dyn super::Step::Step) -> bool {
      // Java: int axis = step.getAxis();
      let axis = step.getAxis();
      // Java: return axis == 8 || axis == 6 || axis == 4 || axis == 13;
      axis == 8 || axis == 6 || axis == 4 || axis == 13
    }
}
