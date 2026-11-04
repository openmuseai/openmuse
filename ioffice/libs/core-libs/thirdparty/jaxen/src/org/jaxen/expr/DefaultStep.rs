// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
// Java: import org.jaxen.expr.iter.IterableAxis
use crate::org::jaxen::expr::iter::IterableAxis::IterableAxis;
// Java: import org.jaxen.saxpath.Axis
use crate::org::jaxen::saxpath::Axis::Axis;
use crate::org::jaxen::{Object, List, NodeIterator};
use super::PredicateSet::PredicateSet;
use super::Predicate::Predicate;

// Java: public class DefaultStep implements Step
pub struct DefaultStep {
    // Java field: IterableAxis axis
    pub(crate) axis: IterableAxis,
    // Java field: PredicateSet predicates
    pub(crate) predicates: PredicateSet,
}

// Java: public class DefaultStep implements Step
impl DefaultStep {
    // Java: DefaultStep(IterableAxis axis, PredicateSet predicates)
    pub fn new(axis: IterableAxis, predicates: PredicateSet) -> Self {
      // Java: super();
      // Java: this.axis = axis;
      // Java: this.predicates = predicates;
      DefaultStep { axis, predicates }
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

    // Java: int getAxis()
    pub fn getAxis(&self) -> i32 {
      // Java: return this.axis.value();
      self.axis.value()
    }

    // Java: IterableAxis getIterableAxis()
    pub fn getIterableAxis(&self) -> &IterableAxis {
      // Java: return this.axis;
      &self.axis
    }

    // Java: String getAxisName()
    pub fn getAxisName(&self) -> String {
      // Java: return Axis.lookup(this.getAxis());
      Axis::lookup_by_num(self.getAxis())
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.predicates.getText();
      self.predicates.getText()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.getIterableAxis() + " " + super.toString();
      format!("IterableAxis({}) DefaultStep", self.axis.value())
    }

    // Java: void simplify()
    pub fn simplify(&mut self) {
      // Java: this.predicates.simplify();
      self.predicates.simplify();
    }

    // Java: Iterator axisIterator(Object contextNode, ContextSupport support)
    pub fn axisIterator(&self, _contextNode: &Object, _support: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException> {
      // Java: return this.getIterableAxis().iterator(contextNode, support);
      todo!("DefaultStep::axisIterator")
    }

    // Java: List evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<List, JaxenException> {
      // Java: List contextNodeSet = context.getNodeSet();
      // Java: IdentitySet unique = new IdentitySet();
      // Java: int contextSize = contextNodeSet.size();
      // Java: ArrayList interimSet = new ArrayList();
      // Java: ArrayList newNodeSet = new ArrayList();
      // Java: ContextSupport support = context.getContextSupport();
      // Java: for(int i = 0; i < contextSize; ++i)
      // Java: Object eachContextNode = contextNodeSet.get(i);
      // Java: Iterator axisNodeIter = this.axis.iterator(eachContextNode, support);
      // Java: while(axisNodeIter.hasNext())
      // Java: Object eachAxisNode = axisNodeIter.next();
      // Java: if (!unique.contains(eachAxisNode) && this.matches(eachAxisNode, support))
      // Java: unique.add(eachAxisNode);
      // Java: interimSet.add(eachAxisNode);
      // Java: newNodeSet.addAll(this.getPredicateSet().evaluatePredicates(interimSet, support));
      // Java: interimSet.clear();
      // Java: return newNodeSet;
      todo!("DefaultStep::evaluate")
    }
}
