#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Comparator
// Java: import org.apache.commons.math3.optim.PointValuePair
use crate::org::apache::commons::math3::optim::PointValuePair;
// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;

use std::any::Any;

// Java field: double DEFAULT_RHO
pub(crate) const DEFAULT_RHO: f64 = 1.0;
// Java field: double DEFAULT_KHI
pub(crate) const DEFAULT_KHI: f64 = 2.0;
// Java field: double DEFAULT_GAMMA
pub(crate) const DEFAULT_GAMMA: f64 = 0.5;
// Java field: double DEFAULT_SIGMA
pub(crate) const DEFAULT_SIGMA: f64 = 0.5;

// Java: public class implements implements the Nelder
pub struct implements {}

// Java: public class NelderMeadSimplex
pub struct NelderMeadSimplex {
    // Java field: double rho
    rho: f64,
    // Java field: double khi
    khi: f64,
    // Java field: double gamma
    gamma: f64,
    // Java field: double sigma
    sigma: f64,
}

// Java: public class implements implements the Nelder
impl implements {}

// Java: public class NelderMeadSimplex
impl NelderMeadSimplex {
    // Java: NelderMeadSimplex(final int n)
    pub fn new(n: i32) -> Self {
        // Java: this(n, 1d);
        Self::new_with_side_length(n, 1.0)
    }

    // Java: NelderMeadSimplex(final int n, double sideLength)
    pub fn new_with_side_length(n: i32, side_length: f64) -> Self {
        // Java: this(n, sideLength,
        // Java: DEFAULT_RHO, DEFAULT_KHI, DEFAULT_GAMMA, DEFAULT_SIGMA);
        Self::new_full(n, side_length, DEFAULT_RHO, DEFAULT_KHI, DEFAULT_GAMMA, DEFAULT_SIGMA)
    }

    // Java: NelderMeadSimplex()
    pub fn new_full(n: i32, _side_length: f64, rho: f64, khi: f64, gamma: f64, sigma: f64) -> Self {
        NelderMeadSimplex { rho, khi, gamma, sigma }
    }

    // Java: NelderMeadSimplex()
    pub fn new_full2(_n: i32, _side_length: f64, rho: f64, khi: f64, gamma: f64, sigma: f64) -> Self {
        NelderMeadSimplex { rho, khi, gamma, sigma }
    }

    // Java: NelderMeadSimplex(final double[] steps)
    pub fn new_from_steps(steps: &[f64]) -> Self {
        // Java: this(steps, DEFAULT_RHO, DEFAULT_KHI, DEFAULT_GAMMA, DEFAULT_SIGMA);
        Self::new_from_steps_with_params(steps, DEFAULT_RHO, DEFAULT_KHI, DEFAULT_GAMMA, DEFAULT_SIGMA)
    }

    // Java: NelderMeadSimplex()
    pub fn new_from_steps_with_params(_steps: &[f64], rho: f64, khi: f64, gamma: f64, sigma: f64) -> Self {
        NelderMeadSimplex { rho, khi, gamma, sigma }
    }

    // Java: NelderMeadSimplex(final double[][] referenceSimplex)
    pub fn new_from_reference(_reference_simplex: &[Vec<f64>]) -> Self {
        // Java: this(referenceSimplex, DEFAULT_RHO, DEFAULT_KHI, DEFAULT_GAMMA, DEFAULT_SIGMA);
        Self::new_from_reference_with_params(_reference_simplex, DEFAULT_RHO, DEFAULT_KHI, DEFAULT_GAMMA, DEFAULT_SIGMA)
    }

    // Java: NelderMeadSimplex()
    pub fn new_from_reference_with_params(_reference_simplex: &[Vec<f64>], rho: f64, khi: f64, gamma: f64, sigma: f64) -> Self {
        NelderMeadSimplex { rho, khi, gamma, sigma }
    }

    // Java: void iterate()
    pub fn iterate(&self) {
        todo!()
    }
}
