#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.ml.neuralnet.twod.NeuronSquareMesh2D
use crate::org::apache::commons::math3::ml::neuralnet::twod::NeuronSquareMesh2D;

use std::any::Any;

// Java: public interface MapVisualization
pub trait MapVisualization {
    // Java: abstract double[][] computeImage(NeuronSquareMesh2D map)
    fn computeImage(&self, map: &dyn Any) -> Vec<Vec<f64>>;
}
