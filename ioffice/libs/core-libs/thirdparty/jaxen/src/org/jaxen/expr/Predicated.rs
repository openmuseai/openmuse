// Java: import java.io.Serializable
// Java: import java.util.List
use crate::org::jaxen::List;

// Java: public interface Predicated extends Serializable
pub trait Predicated {
    // Java: abstract void addPredicate(Predicate var1)
    fn addPredicate(&mut self, var1: Box<dyn super::Predicate::Predicate>);

    // Java: abstract List getPredicates()
    fn getPredicates(&self) -> &Vec<Box<dyn super::Predicate::Predicate>>;

    // Java: abstract PredicateSet getPredicateSet()
    fn getPredicateSet(&self) -> &Vec<Box<dyn super::Predicate::Predicate>>;
}
