#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Arrays
// Java: import java.util.Comparator
// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.optim.PointValuePair
use crate::org::apache::commons::math3::optim::PointValuePair;
// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

use std::any::Any;

// Java: public class implements implements the simplex concept.
pub struct implements {}

// Java: public class AbstractSimplex implements OptimizationData
pub struct AbstractSimplex {
    // Java field: PointValuePair[] simplex
    pub(crate) simplex: Vec<Box<dyn Any>>,
    // Java field: double[][] startConfiguration
    startConfiguration: Option<Vec<Vec<f64>>>,
    // Java field: int dimension
    pub(crate) dimension: i32,
}

// Java: public class implements implements the simplex concept.
impl implements {}

// Java: public class AbstractSimplex implements OptimizationData
impl AbstractSimplex {
    // Java: AbstractSimplex(int n)
    pub fn new(n: i32) -> Self {
        // Java: this(n, 1d);
        Self::new_with_side_length(n, 1.0)
    }

    // Java: AbstractSimplex()
    pub fn new_with_side_length(n: i32, sideLength: f64) -> Self {
        let dimension = n;
        let mut startConfiguration = vec![vec![0.0; dimension as usize]; dimension as usize];
        for i in 0..dimension as usize {
            startConfiguration[i][i] = sideLength;
        }
        AbstractSimplex {
            simplex: Vec::new(),
            startConfiguration: Some(startConfiguration),
            dimension,
        }
    }

    // Java: AbstractSimplex(final double[] steps)
    pub fn new_with_steps(steps: &[f64]) -> Self {
        // Java: if (steps == null)
        // Java: throw new NullArgumentException();
        // Java: if (steps.length == 0)
        if steps.is_empty() {
            // Java: throw new ZeroException();
            panic!("ZeroException");
        }
        // Java: dimension = steps.length;
        let dimension = steps.len() as i32;
        // Java: // Only the relative position of the n final vertices with respect
        // Java: // to the first one are stored.
        // Java: startConfiguration = new double[dimension][dimension];
        let mut startConfiguration = vec![vec![0.0; dimension as usize]; dimension as usize];
        // Java: for (int i = 0; i < dimension; i++)
        for i in 0..dimension as usize {
            // Java: final double[] vertexI = startConfiguration[i];
            // Java: for (int j = 0; j < i + 1; j++)
            for j in 0..=i {
                // Java: if (steps[j] == 0)
                if steps[j] == 0.0 {
                    // Java: throw new ZeroException(LocalizedFormats.EQUAL_VERTICES_IN_SIMPLEX);
                    panic!("ZeroException: EQUAL_VERTICES_IN_SIMPLEX");
                }
                // Java: System.arraycopy(steps, 0, vertexI, 0, j + 1);
                startConfiguration[i][j] = steps[j];
            }
        }
        AbstractSimplex {
            simplex: Vec::new(),
            startConfiguration: Some(startConfiguration),
            dimension,
        }
    }

    // Java: AbstractSimplex(final double[][] referenceSimplex)
    pub fn new_with_reference(referenceSimplex: &[Vec<f64>]) -> Self {
        // Java: if (referenceSimplex.length <= 0)
        if referenceSimplex.is_empty() {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.SIMPLEX_NEED_ONE_POINT,
            // Java: referenceSimplex.length);
            panic!("NotStrictlyPositiveException: SIMPLEX_NEED_ONE_POINT");
        }
        // Java: dimension = referenceSimplex.length - 1;
        let dimension = (referenceSimplex.len() - 1) as i32;
        // Java: // Only the relative position of the n final vertices with respect
        // Java: // to the first one are stored.
        // Java: startConfiguration = new double[dimension][dimension];
        let mut startConfiguration = vec![vec![0.0; dimension as usize]; dimension as usize];
        // Java: final double[] ref0 = referenceSimplex[0];
        let ref0 = &referenceSimplex[0];
        // Java: // Loop over vertices.
        // Java: for (int i = 0; i < referenceSimplex.length; i++)
        for i in 0..referenceSimplex.len() {
            // Java: final double[] refI = referenceSimplex[i];
            let refI = &referenceSimplex[i];
            // Java: // Safety checks.
            // Java: if (refI.length != dimension)
            if refI.len() != dimension as usize {
                // Java: throw new DimensionMismatchException(refI.length, dimension);
                panic!("DimensionMismatchException: {} vs {}", refI.len(), dimension);
            }
            // Java: for (int j = 0; j < i; j++)
            for j in 0..i {
                // Java: final double[] refJ = referenceSimplex[j];
                let refJ = &referenceSimplex[j];
                // Java: boolean allEquals = true;
                let mut allEquals = true;
                // Java: for (int k = 0; k < dimension; k++)
                for k in 0..dimension as usize {
                    // Java: if (refI[k] != refJ[k])
                    if refI[k] != refJ[k] {
                        // Java: allEquals = false;
                        allEquals = false;
                        // Java: break;
                        break;
                    }
                }
                // Java: if (allEquals)
                if allEquals {
                    // Java: throw new MathIllegalArgumentException(LocalizedFormats.EQUAL_VERTICES_IN_SIMPLEX,
                    // Java: i, j);
                    panic!("MathIllegalArgumentException: EQUAL_VERTICES_IN_SIMPLEX {} {}", i, j);
                }
            }
            // Java: // Store vertex i position relative to vertex 0 position.
            // Java: if (i > 0)
            if i > 0 {
                // Java: final double[] confI = startConfiguration[i - 1];
                // Java: for (int k = 0; k < dimension; k++)
                for k in 0..dimension as usize {
                    // Java: confI[k] = refI[k] - ref0[k];
                    startConfiguration[i - 1][k] = refI[k] - ref0[k];
                }
            }
        }
        AbstractSimplex {
            simplex: Vec::new(),
            startConfiguration: Some(startConfiguration),
            dimension,
        }
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return dimension;
        self.dimension
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return simplex.length;
        self.simplex.len() as i32
    }

    // Java: abstract void iterate()
    pub fn iterate(&self) {
        todo!()
    }

    // Java: void build(final double[] startPoint)
    pub fn build(&mut self, startPoint: &[f64]) {
        // Java: if (dimension != startPoint.length)
        if self.dimension != startPoint.len() as i32 {
            // Java: throw new DimensionMismatchException(dimension, startPoint.length);
            panic!("DimensionMismatchException: {} vs {}", self.dimension, startPoint.len());
        }
        // Java: // Set first vertex.
        // Java: simplex = new PointValuePair[dimension + 1];
        self.simplex = Vec::with_capacity((self.dimension + 1) as usize);
        // Java: simplex[0] = new PointValuePair(startPoint, Double.NaN);
        self.simplex.push(Box::new((startPoint.to_vec(), f64::NAN)));
        // Java: // Set remaining vertices.
        // Java: for (int i = 0; i < dimension; i++)
        if let Some(ref config) = self.startConfiguration {
            for i in 0..self.dimension as usize {
                // Java: final double[] confI = startConfiguration[i];
                let confI = &config[i];
                // Java: final double[] vertexI = new double[dimension];
                let mut vertexI = vec![0.0; self.dimension as usize];
                // Java: for (int k = 0; k < dimension; k++)
                for k in 0..self.dimension as usize {
                    // Java: vertexI[k] = startPoint[k] + confI[k];
                    vertexI[k] = startPoint[k] + confI[k];
                }
                // Java: simplex[i + 1] = new PointValuePair(vertexI, Double.NaN);
                self.simplex.push(Box::new((vertexI, f64::NAN)));
            }
        }
    }

    // Java: void evaluate()
    pub fn evaluate(&self) {
        todo!()
    }

    // Java: void replaceWorstPoint()
    pub fn replaceWorstPoint(&self) {
        todo!()
    }

    // Java: PointValuePair[] getPoints()
    pub fn getPoints(&self) -> Vec<Box<dyn Any>> {
        // Java: final PointValuePair[] copy = new PointValuePair[simplex.length];
        // Java: System.arraycopy(simplex, 0, copy, 0, simplex.length);
        // Java: return copy;
        todo!()
    }

    // Java: PointValuePair getPoint(int index)
    pub fn getPoint(&self, index: i32) -> &Box<dyn Any> {
        // Java: if (index < 0 ||
        // Java: index >= simplex.length)
        if index < 0 || index >= self.simplex.len() as i32 {
            // Java: throw new OutOfRangeException(index, 0, simplex.length - 1);
            panic!("OutOfRangeException: {} not in [0, {}]", index, self.simplex.len() - 1);
        }
        // Java: return simplex[index];
        &self.simplex[index as usize]
    }

    // Java: void setPoint(int index, PointValuePair point)
    pub fn setPoint(&mut self, index: i32, point: Box<dyn Any>) {
        // Java: if (index < 0 ||
        // Java: index >= simplex.length)
        if index < 0 || index >= self.simplex.len() as i32 {
            // Java: throw new OutOfRangeException(index, 0, simplex.length - 1);
            panic!("OutOfRangeException: {} not in [0, {}]", index, self.simplex.len() - 1);
        }
        // Java: simplex[index] = point;
        self.simplex[index as usize] = point;
    }

    // Java: void setPoints(PointValuePair[] points)
    pub fn setPoints(&mut self, points: Vec<Box<dyn Any>>) {
        // Java: if (points.length != simplex.length)
        if points.len() != self.simplex.len() {
            // Java: throw new DimensionMismatchException(points.length, simplex.length);
            panic!("DimensionMismatchException: {} vs {}", points.len(), self.simplex.len());
        }
        // Java: simplex = points;
        self.simplex = points;
    }

    // Java: static double[] createHypercubeSteps()
    pub fn createHypercubeSteps(n: i32, sideLength: f64) -> Vec<f64> {
        vec![sideLength; n as usize]
    }
}
