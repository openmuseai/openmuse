#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.ml.neuralnet.twod.NeuronSquareMesh2D
use crate::org::apache::commons::math3::ml::neuralnet::twod::NeuronSquareMesh2D;

// Java: public interface MapDataVisualization
pub trait MapDataVisualization {
    // Java: abstract double[][] computeImage()
    fn computeImage(&self) -> Vec<Vec<f64>>;
}
