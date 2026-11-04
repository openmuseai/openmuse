// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.List
// Java: import org.apache.commons.math3.analysis.function.Gaussian
use crate::org::apache::commons::math3::analysis::function::Gaussian;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresBuilder
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresBuilder;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem;
// Java: import org.apache.commons.math3.linear.DiagonalMatrix
use crate::org::apache::commons::math3::linear::DiagonalMatrix;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class GaussianCurveFitter extends AbstractCurveFitter
pub struct GaussianCurveFitter {
    // Java field: double[] initialGuess
    initialGuess: Vec<f64>,
    // Java field: int maxIter
    maxIter: i32,
}

// Java: public class ParameterGuesser
pub struct ParameterGuesser {
    // Java field: double norm
    pub(crate) norm: f64,
    // Java field: double mean
    pub(crate) mean: f64,
    // Java field: double sigma
    pub(crate) sigma: f64,
}

// Java: public class GaussianCurveFitter extends AbstractCurveFitter
impl GaussianCurveFitter {
    // Java: GaussianCurveFitter()
    pub fn new(initial_guess: Vec<f64>, max_iter: i32) -> Self {
        GaussianCurveFitter {
            initialGuess: initial_guess,
            maxIter: max_iter,
        }
    }

    // Java: static GaussianCurveFitter create()
    pub fn create() -> GaussianCurveFitter {
        // Java: return new GaussianCurveFitter(null, Integer.MAX_VALUE);
        GaussianCurveFitter::new(Vec::new(), i32::MAX)
    }

    // Java: GaussianCurveFitter withStartPoint(double[] newStart)
    pub fn withStartPoint(&self, new_start: Vec<f64>) -> GaussianCurveFitter {
        // Java: return new GaussianCurveFitter(newStart.clone(),
                                       // Java: maxIter);
        GaussianCurveFitter::new(new_start, self.maxIter)
    }

    // Java: GaussianCurveFitter withMaxIterations(int newMaxIter)
    pub fn withMaxIterations(&self, new_max_iter: i32) -> GaussianCurveFitter {
        // Java: return new GaussianCurveFitter(initialGuess,
                                       // Java: newMaxIter);
        GaussianCurveFitter::new(self.initialGuess.clone(), new_max_iter)
    }

    // Java: LeastSquaresProblem getProblem(Collection<WeightedObservedPoint> observations)
    pub fn getProblem(&self, _observations: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // Prepare least-squares problem.
        // Java: final int len = observations.size();
        // Java: final double[] target  = new double[len];
        // Java: final double[] weights = new double[len];
        // Java: int i = 0;
        // Java: for (WeightedObservedPoint obs : observations)
        {
            // Java: target[i]  = obs.getY();
            // Java: weights[i] = obs.getWeight();
            // Java: ++i;
        }
        // Java: final AbstractCurveFitter.TheoreticalValuesFunction model =
                // Java: new AbstractCurveFitter.TheoreticalValuesFunction(FUNCTION, observations);
        // Java: final double[] startPoint = initialGuess != null ?
            // Java: initialGuess :
            // Java: // Compute estimation.
            // Java: new ParameterGuesser(observations).guess();
        // Java: // Return a new least squares problem set up to fit a Gaussian curve to the
        // Java: // observed points.
        // Java: return new LeastSquaresBuilder().
                // Java: maxEvaluations(Integer.MAX_VALUE).
                // Java: maxIterations(maxIter).
                // Java: start(startPoint).
                // Java: target(target).
                // Java: weight(new DiagonalMatrix(weights)).
                // Java: model(model.getModelFunction(), model.getModelFunctionJacobian()).
                // Java: build();
        todo!("GaussianCurveFitter::getProblem")
    }

}

// Java: public class ParameterGuesser
impl ParameterGuesser {
    // Java: ParameterGuesser(Collection<WeightedObservedPoint> observations)
    pub fn new(_observations: &dyn std::any::Any) -> Self {
            // Java: if (observations == null)
            {
                // Java: throw new NullArgumentException(LocalizedFormats.INPUT_ARRAY);
            }
            // Java: if (observations.size() < 3)
            {
                // Java: throw new NumberIsTooSmallException(observations.size(), 3, true);
            }
            // Java: final List<WeightedObservedPoint> sorted = sortObservations(observations);
            // Java: final double[] params = basicGuess(sorted.toArray(new WeightedObservedPoint[0]));
            // Java: norm = params[0];
            // Java: mean = params[1];
            // Java: sigma = params[2];
        ParameterGuesser { norm: 0.0, mean: 0.0, sigma: 0.0 }
    }

    // Java: double[] guess()
    pub fn guess(&self) -> Vec<f64> {
            // Java: return new double[] { norm, mean, sigma };
            vec![self.norm, self.mean, self.sigma]
    }

    // Java: List<WeightedObservedPoint> sortObservations(Collection<WeightedObservedPoint> unsorted)
    pub fn sortObservations(&self, _unsorted: &dyn std::any::Any) -> Box<dyn std::any::Any> {
            // Java: final List<WeightedObservedPoint> observations = new ArrayList<WeightedObservedPoint>(unsorted);
            // Java: final Comparator<WeightedObservedPoint> cmp = new Comparator<WeightedObservedPoint>()
            {
                // Java: /** {@inheritDoc} */
                // Java: public int compare(WeightedObservedPoint p1,
                                   // Java: WeightedObservedPoint p2)
                                   {
                    // Java: if (p1 == null && p2 == null)
                    {
                        // Java: return 0;
                    }
                    // Java: if (p1 == null)
                    {
                        // Java: return -1;
                    }
                    // Java: if (p2 == null)
                    {
                        // Java: return 1;
                    }
                    // Java: final int cmpX = Double.compare(p1.getX(), p2.getX());
                    // Java: if (cmpX < 0)
                    {
                        // Java: return -1;
                    }
                    // Java: if (cmpX > 0)
                    {
                        // Java: return 1;
                    }
                    // Java: final int cmpY = Double.compare(p1.getY(), p2.getY());
                    // Java: if (cmpY < 0)
                    {
                        // Java: return -1;
                    }
                    // Java: if (cmpY > 0)
                    {
                        // Java: return 1;
                    }
                    // Java: final int cmpW = Double.compare(p1.getWeight(), p2.getWeight());
                    // Java: if (cmpW < 0)
                    {
                        // Java: return -1;
                    }
                    // Java: if (cmpW > 0)
                    {
                        // Java: return 1;
                    }
                    // Java: return 0;
                                   }
            }
            // Java: Collections.sort(observations, cmp);
            // Java: return observations;
        todo!("ParameterGuesser::sortObservations")
    }

    // Java: double[] basicGuess(WeightedObservedPoint[] points)
    pub fn basicGuess(&self, _points: &dyn std::any::Any) -> Vec<f64> {
            // Java: final int maxYIdx = findMaxY(points);
            // Java: final double n = points[maxYIdx].getY();
            // Java: final double m = points[maxYIdx].getX();
            // Java: double fwhmApprox;
            // Java: try
            {
                // Java: final double halfY = n + ((m - n) / 2);
                // Java: final double fwhmX1 = interpolateXAtY(points, maxYIdx, -1, halfY);
                // Java: final double fwhmX2 = interpolateXAtY(points, maxYIdx, 1, halfY);
                // Java: fwhmApprox = fwhmX2 - fwhmX1;
            }
            // Java: catch (OutOfRangeException e)
            {
                // Java: // TODO: Exceptions should not be used for flow control.
                // Java: fwhmApprox = points[points.length - 1].getX() - points[0].getX();
            }
            // Java: final double s = fwhmApprox / (2 * FastMath.sqrt(2 * FastMath.log(2)));
            // Java: return new double[] { n, m, s };
        todo!("ParameterGuesser::basicGuess")
    }

    // Java: int findMaxY(WeightedObservedPoint[] points)
    pub fn findMaxY(&self, _points: &dyn std::any::Any) -> i32 {
            // Java: int maxYIdx = 0;
            // Java: for (int i = 1; i < points.length; i++)
            {
                // Java: if (points[i].getY() > points[maxYIdx].getY())
                {
                    // Java: maxYIdx = i;
                }
            }
            // Java: return maxYIdx;
        todo!("ParameterGuesser::findMaxY")
    }

    // Java: double interpolateXAtY()
    pub fn interpolateXAtY(&self) -> f64 {
        todo!("ParameterGuesser::interpolateXAtY")
    }

    // Java: WeightedObservedPoint[] getInterpolationPointsForY()
    pub fn getInterpolationPointsForY(&self) -> Box<dyn std::any::Any> {
        todo!("ParameterGuesser::getInterpolationPointsForY")
    }

    // Java: boolean isBetween()
    pub fn isBetween(&self) -> bool {
        todo!("ParameterGuesser::isBetween")
    }

}
