// Java: import java.io.Serializable
// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.function.BooleanFunction
use crate::org::jaxen::function::BooleanFunction::BooleanFunction;
use crate::org::jaxen::{Object, List};
use super::Predicate::Predicate;

// Java field: long serialVersionUID

// Java: public class PredicateSet implements Serializable
pub struct PredicateSet {
    // Java field: List predicates
    pub(crate) predicates: Vec<Box<dyn Predicate>>,
}

// Java: public class PredicateSet implements Serializable
impl PredicateSet {
    // Java: PredicateSet()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.predicates = Collections.EMPTY_LIST;
      PredicateSet {
         predicates: Vec::new(),
      }
    }

    // Java: void addPredicate(Predicate predicate)
    pub fn addPredicate(&mut self, predicate: Box<dyn Predicate>) {
      // Java: if (this.predicates == Collections.EMPTY_LIST)
      // Java: this.predicates = new ArrayList();
      // Java: this.predicates.add(predicate);
      self.predicates.push(predicate);
    }

    // Java: List getPredicates()
    pub fn getPredicates(&self) -> &Vec<Box<dyn Predicate>> {
      // Java: return this.predicates;
      &self.predicates
    }

    // Java: void simplify()
    pub fn simplify(&mut self) {
      // Java: Iterator predIter = this.predicates.iterator();
      // Java: Predicate eachPred = null;
      // Java: while(predIter.hasNext())
      for pred in self.predicates.iter_mut() {
         // Java: eachPred = (Predicate)predIter.next();
         // Java: eachPred.simplify();
         pred.simplify();
      }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: Iterator predIter = this.predicates.iterator();
      // Java: Predicate eachPred = null;
      // Java: while(predIter.hasNext())
      for pred in self.predicates.iter() {
         // Java: eachPred = (Predicate)predIter.next();
         // Java: buf.append(eachPred.getText());
         buf.push_str(&pred.getText());
      }
      // Java: return buf.toString();
      buf
    }

    // Java: boolean evaluateAsBoolean(List contextNodeSet, ContextSupport support)
    pub fn evaluateAsBoolean(&self, _contextNodeSet: &List, _support: &ContextSupport) -> bool {
      // Java: return this.anyMatchingNode(contextNodeSet, support);
      todo!("PredicateSet::evaluateAsBoolean")
    }

    // Java: boolean anyMatchingNode(List contextNodeSet, ContextSupport support)
    pub fn anyMatchingNode(&self, _contextNodeSet: &List, _support: &ContextSupport) -> bool {
      // Java: if (this.predicates.size() == 0)
      if self.predicates.is_empty() {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: Iterator predIter = this.predicates.iterator();
         // Java: List nodes2Filter = contextNodeSet;
         // Java: while(predIter.hasNext())
         // Java: int nodes2FilterSize = nodes2Filter.size();
         // Java: Context predContext = new Context(support);
         // Java: List tempList = new ArrayList(1);
         // Java: predContext.setNodeSet(tempList);
         // Java: for(int i = 0; i < nodes2FilterSize; ++i)
         // Java: Object contextNode = nodes2Filter.get(i);
         // Java: tempList.clear();
         // Java: tempList.add(contextNode);
         // Java: predContext.setNodeSet(tempList);
         // Java: predContext.setPosition(i + 1);
         // Java: predContext.setSize(nodes2FilterSize);
         // Java: Object predResult = ((Predicate)predIter.next()).evaluate(predContext);
         // Java: if (predResult instanceof Number)
         // Java: int proximity = ((Number)predResult).intValue();
         // Java: if (proximity == i + 1)
         // Java: return true;
         // Java: else
         // Java: Boolean includes = BooleanFunction.evaluate(predResult, predContext.getNavigator());
         // Java: if (includes)
         // Java: return true;
         // Java: return false;
         todo!("PredicateSet::anyMatchingNode")
      }
    }

    // Java: List evaluatePredicates(List contextNodeSet, ContextSupport support)
    pub fn evaluatePredicates(&self, _contextNodeSet: &List, _support: &ContextSupport) -> Result<List, JaxenException> {
      // Java: if (this.predicates.size() == 0)
      if self.predicates.is_empty() {
         // Java: return contextNodeSet;
         return Ok(_contextNodeSet.clone());
      }
      // Java: else
      {
         // Java: Iterator predIter = this.predicates.iterator();
         // Java: List nodes2Filter;
         // Java: for(nodes2Filter = contextNodeSet; predIter.hasNext(); nodes2Filter = this.applyPredicate((Predicate)predIter.next(), nodes2Filter, support))
         // Java: return nodes2Filter;
         todo!("PredicateSet::evaluatePredicates")
      }
    }

    // Java: List applyPredicate(Predicate predicate, List nodes2Filter, ContextSupport support)
    pub fn applyPredicate(&self, _predicate: &dyn Predicate, _nodes2Filter: &List, _support: &ContextSupport) -> Result<List, JaxenException> {
      // Java: int nodes2FilterSize = nodes2Filter.size();
      // Java: List filteredNodes = new ArrayList(nodes2FilterSize);
      // Java: Context predContext = new Context(support);
      // Java: List tempList = new ArrayList(1);
      // Java: predContext.setNodeSet(tempList);
      // Java: for(int i = 0; i < nodes2FilterSize; ++i)
      // Java: Object contextNode = nodes2Filter.get(i);
      // Java: tempList.clear();
      // Java: tempList.add(contextNode);
      // Java: predContext.setNodeSet(tempList);
      // Java: predContext.setPosition(i + 1);
      // Java: predContext.setSize(nodes2FilterSize);
      // Java: Object predResult = predicate.evaluate(predContext);
      // Java: if (predResult instanceof Number)
      // Java: int proximity = ((Number)predResult).intValue();
      // Java: if (proximity == i + 1)
      // Java: filteredNodes.add(contextNode);
      // Java: else
      // Java: Boolean includes = BooleanFunction.evaluate(predResult, predContext.getNavigator());
      // Java: if (includes)
      // Java: filteredNodes.add(contextNode);
      // Java: return filteredNodes;
      todo!("PredicateSet::applyPredicate")
    }
}
