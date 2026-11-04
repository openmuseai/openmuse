#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: public interface LearningFactorFunction
pub trait LearningFactorFunction {
    // Java: abstract double value(long numCall)
    fn value(&self, numCall: i64) -> f64;

}
