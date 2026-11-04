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
// Java: import org.apache.commons.math3.optimization.PointValuePair
use crate::org::apache::commons::math3::optimization::PointValuePair;
// Java: import org.apache.commons.math3.optimization.OptimizationData
use crate::org::apache::commons::math3::optimization::OptimizationData;

// Java: public class implements implements the simplex concept.
pub struct implements {
}

// Java: public class AbstractSimplex implements OptimizationData
pub struct AbstractSimplex {
    // Java field: PointValuePair[] simplex
    pub(crate) simplex: Box<dyn std::any::Any>,
    // Java field: double[][] startConfiguration
    startConfiguration: Option<Vec<Vec<f64>>>,
    // Java field: int dimension
    pub(crate) dimension: i32,
}

// Java: public class implements implements the simplex concept.
impl implements {
}

impl implements {
}

// Java: public class AbstractSimplex implements OptimizationData
impl AbstractSimplex {
    // Java: AbstractSimplex(int n)
    pub fn AbstractSimplex(&self) {
        // Java: this(n, 1d);
    }

    // Java: AbstractSimplex()
    pub fn AbstractSimplex_2(&self) {
    }

    // Java: AbstractSimplex(final double[] steps)
    pub fn AbstractSimplex_3(&self) {
        // Java: if (steps == null)
        {
            // Java: throw new NullArgumentException();
        }
        // Java: if (steps.length == 0)
        {
            // Java: throw new ZeroException();
        }
        // Java: dimension = steps.length;
        // Java: // Only the relative position of the n final vertices with respect
        // Java: // to the first one are stored.
        // Java: startConfiguration = new double[dimension][dimension];
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: final double[] vertexI = startConfiguration[i];
            // Java: for (int j = 0; j < i + 1; j++)
            {
                // Java: if (steps[j] == 0)
                {
                    // Java: throw new ZeroException(LocalizedFormats.EQUAL_VERTICES_IN_SIMPLEX);
                }
                // Java: System.arraycopy(steps, 0, vertexI, 0, j + 1);
            }
        }
    }

    // Java: AbstractSimplex(final double[][] referenceSimplex)
    pub fn AbstractSimplex_4(&self) {
        // Java: if (referenceSimplex.length <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.SIMPLEX_NEED_ONE_POINT,
                                                   // Java: referenceSimplex.length);
        }
        // Java: dimension = referenceSimplex.length - 1;
        // Java: // Only the relative position of the n final vertices with respect
        // Java: // to the first one are stored.
        // Java: startConfiguration = new double[dimension][dimension];
        // Java: final double[] ref0 = referenceSimplex[0];
        // Java: // Loop over vertices.
        // Java: for (int i = 0; i < referenceSimplex.length; i++)
        {
            // Java: final double[] refI = referenceSimplex[i];
            // Java: // Safety checks.
            // Java: if (refI.length != dimension)
            {
                // Java: throw new DimensionMismatchException(refI.length, dimension);
            }
            // Java: for (int j = 0; j < i; j++)
            {
                // Java: final double[] refJ = referenceSimplex[j];
                // Java: boolean allEquals = true;
                // Java: for (int k = 0; k < dimension; k++)
                {
                    // Java: if (refI[k] != refJ[k])
                    {
                        // Java: allEquals = false;
                        // Java: break;
                    }
                }
                // Java: if (allEquals)
                {
                    // Java: throw new MathIllegalArgumentException(LocalizedFormats.EQUAL_VERTICES_IN_SIMPLEX,
                                                           // Java: i, j);
                }
            }
            // Java: // Store vertex i position relative to vertex 0 position.
            // Java: if (i > 0)
            {
                // Java: final double[] confI = startConfiguration[i - 1];
                // Java: for (int k = 0; k < dimension; k++)
                {
                    // Java: confI[k] = refI[k] - ref0[k];
                }
            }
        }
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return dimension;
        todo!()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return simplex.length;
        todo!()
    }

    // Java: abstract void iterate()
    pub fn iterate(&self) {
    }

    // Java: void build(final double[] startPoint)
    pub fn build(&self) {
        // Java: if (dimension != startPoint.length)
        {
            // Java: throw new DimensionMismatchException(dimension, startPoint.length);
        }
        // Java: // Set first vertex.
        // Java: simplex = new PointValuePair[dimension + 1];
        // Java: simplex[0] = new PointValuePair(startPoint, Double.NaN);
        // Java: // Set remaining vertices.
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: final double[] confI = startConfiguration[i];
            // Java: final double[] vertexI = new double[dimension];
            // Java: for (int k = 0; k < dimension; k++)
            {
                // Java: vertexI[k] = startPoint[k] + confI[k];
            }
            // Java: simplex[i + 1] = new PointValuePair(vertexI, Double.NaN);
        }
    }

    // Java: void evaluate()
    pub fn evaluate(&self) {
    }

    // Java: void replaceWorstPoint()
    pub fn replaceWorstPoint(&self) {
    }

    // Java: PointValuePair[] getPoints()
    pub fn getPoints(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final PointValuePair[] copy = new PointValuePair[simplex.length];
        // Java: System.arraycopy(simplex, 0, copy, 0, simplex.length);
        // Java: return copy;
        todo!()
    }

    // Java: PointValuePair getPoint(int index)
    pub fn getPoint(&self) -> Box<dyn std::any::Any> {
        // Java: if (index < 0 ||
            // Java: index >= simplex.length)
            {
            // Java: throw new OutOfRangeException(index, 0, simplex.length - 1);
            }
        // Java: return simplex[index];
        todo!()
    }

    // Java: void setPoint(int index, PointValuePair point)
    pub fn setPoint(&self) {
        // Java: if (index < 0 ||
            // Java: index >= simplex.length)
            {
            // Java: throw new OutOfRangeException(index, 0, simplex.length - 1);
            }
        // Java: simplex[index] = point;
    }

    // Java: void setPoints(PointValuePair[] points)
    pub fn setPoints(&self) {
        // Java: if (points.length != simplex.length)
        {
            // Java: throw new DimensionMismatchException(points.length, simplex.length);
        }
        // Java: simplex = points;
    }

    // Java: static double[] createHypercubeSteps()
    pub fn createHypercubeSteps(&self) -> Vec<f64> {
        todo!()
    }

}

