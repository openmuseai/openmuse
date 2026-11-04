// Java: public interface StoppingCondition
pub trait StoppingCondition {
    // Java: abstract boolean isSatisfied(Population population)
    fn isSatisfied(&self, population: &dyn std::any::Any) -> bool;

}
