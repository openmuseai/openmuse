#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Comparator
// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.optim.PointValuePair
use crate::org::apache::commons::math3::optim::PointValuePair;

use std::any::Any;

// Java field: double DEFAULT_KHI
pub(crate) const DEFAULT_KHI: f64 = 2.0;
// Java field: double DEFAULT_GAMMA
pub(crate) const DEFAULT_GAMMA: f64 = 0.5;

// Java: public class implements implements the multi
pub struct implements {}

// Java: public class MultiDirectionalSimplex
pub struct MultiDirectionalSimplex {
    // Java field: double khi
    khi: f64,
    // Java field: double gamma
    gamma: f64,
}

// Java: public class implements implements the multi
impl implements {}

// Java: public class MultiDirectionalSimplex
impl MultiDirectionalSimplex {
    // Java: MultiDirectionalSimplex(final int n)
    pub fn new(n: i32) -> Self {
        // Java: this(n, 1d);
        Self::new_with_side_length(n, 1.0)
    }

    // Java: MultiDirectionalSimplex(final int n, double sideLength)
    pub fn new_with_side_length(n: i32, sideLength: f64) -> Self {
        // Java: this(n, sideLength, DEFAULT_KHI, DEFAULT_GAMMA);
        Self::new_with_all(n, sideLength, DEFAULT_KHI, DEFAULT_GAMMA)
    }

    // Java: MultiDirectionalSimplex()
    pub fn new_with_coefficients(n: i32, khi: f64, gamma: f64) -> Self {
        Self::new_with_all(n, 1.0, khi, gamma)
    }

    // Java: MultiDirectionalSimplex()
    pub fn new_with_all(n: i32, sideLength: f64, khi: f64, gamma: f64) -> Self {
        MultiDirectionalSimplex { khi, gamma }
    }

    // Java: MultiDirectionalSimplex(final double[] steps)
    pub fn new_with_steps(steps: &[f64]) -> Self {
        // Java: this(steps, DEFAULT_KHI, DEFAULT_GAMMA);
        Self::new_with_steps_and_coefficients(steps, DEFAULT_KHI, DEFAULT_GAMMA)
    }

    // Java: MultiDirectionalSimplex()
    pub fn new_with_steps_and_coefficients(steps: &[f64], khi: f64, gamma: f64) -> Self {
        MultiDirectionalSimplex { khi, gamma }
    }

    // Java: MultiDirectionalSimplex(final double[][] referenceSimplex)
    pub fn new_with_reference(referenceSimplex: &[Vec<f64>]) -> Self {
        // Java: this(referenceSimplex, DEFAULT_KHI, DEFAULT_GAMMA);
        Self::new_with_reference_and_coefficients(referenceSimplex, DEFAULT_KHI, DEFAULT_GAMMA)
    }

    // Java: MultiDirectionalSimplex()
    pub fn new_with_reference_and_coefficients(referenceSimplex: &[Vec<f64>], khi: f64, gamma: f64) -> Self {
        MultiDirectionalSimplex { khi, gamma }
    }

    // Java: void iterate()
    pub fn iterate(&self) {
        todo!()
    }

    // Java: PointValuePair evaluateNewSimplex()
    pub fn evaluateNewSimplex(&self) -> Box<dyn Any> {
        todo!()
    }
}
