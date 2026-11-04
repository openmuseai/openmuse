// Java: import java.util.Arrays
// Java: import java.util.Comparator
// Java: import org.apache.commons.math3.analysis.function.Gaussian
use crate::org::apache::commons::math3::analysis::function::Gaussian;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.optimization.DifferentiableMultivariateVectorOptimizer
use crate::org::apache::commons::math3::optimization::DifferentiableMultivariateVectorOptimizer;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class GaussianFitter extends CurveFitter
pub struct GaussianFitter {
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

// Java: public class GaussianFitter extends CurveFitter
impl GaussianFitter {
    // Java: GaussianFitter(DifferentiableMultivariateVectorOptimizer optimizer)
    pub fn GaussianFitter(&self) {
        // Java: super(optimizer);
    }

    // Java: double[] fit(double[] initialGuess)
    pub fn fit(&self) -> Vec<f64> {
        // Java: final Gaussian.Parametric f = new Gaussian.Parametric()
        {
                // Java: /** {@inheritDoc} */
                // Java: @Override
                // Java: public double value(double x, double ... p)
                {
                    // Java: double v = Double.POSITIVE_INFINITY;
                    // Java: try
                    {
                        // Java: v = super.value(x, p);
                    }
                        // Java: // Do nothing.
                    // Java: return v;
                }
                // Java: /** {@inheritDoc} */
                // Java: @Override
                // Java: public double[] gradient(double x, double ... p)
                {
                    // Java: double[] v = { Double.POSITIVE_INFINITY,
                                   // Java: Double.POSITIVE_INFINITY,
                                   // Java: Double.POSITIVE_INFINITY };
                    // Java: try
                    {
                        // Java: v = super.gradient(x, p);
                    }
                        // Java: // Do nothing.
                    // Java: return v;
                }
        }
        // Java: return fit(f, initialGuess);
        todo!()
    }

    // Java: double[] fit()
    pub fn fit_2(&self) -> Vec<f64> {
        // Java: final double[] guess = (new ParameterGuesser(getObservations())).guess();
        // Java: return fit(guess);
        todo!()
    }

}

// Java: public class ParameterGuesser
impl ParameterGuesser {
    // Java: ParameterGuesser(WeightedObservedPoint[] observations)
    pub fn ParameterGuesser(&self) {
            // Java: if (observations == null)
            {
                // Java: throw new NullArgumentException(LocalizedFormats.INPUT_ARRAY);
            }
            // Java: if (observations.length < 3)
            {
                // Java: throw new NumberIsTooSmallException(observations.length, 3, true);
            }
            // Java: final WeightedObservedPoint[] sorted = sortObservations(observations);
            // Java: final double[] params = basicGuess(sorted);
            // Java: norm = params[0];
            // Java: mean = params[1];
            // Java: sigma = params[2];
    }

    // Java: double[] guess()
    pub fn guess(&self) -> Vec<f64> {
            // Java: return new double[] { norm, mean, sigma };
        todo!()
    }

    // Java: WeightedObservedPoint[] sortObservations(WeightedObservedPoint[] unsorted)
    pub fn sortObservations(&self) -> Box<dyn std::any::Any> {
            // Java: final WeightedObservedPoint[] observations = unsorted.clone();
            // Java: final Comparator<WeightedObservedPoint> cmp
                // Java: = new Comparator<WeightedObservedPoint>()
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
            // Java: Arrays.sort(observations, cmp);
            // Java: return observations;
        todo!()
    }

    // Java: double[] basicGuess(WeightedObservedPoint[] points)
    pub fn basicGuess(&self) -> Vec<f64> {
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
        todo!()
    }

    // Java: int findMaxY(WeightedObservedPoint[] points)
    pub fn findMaxY(&self) -> i32 {
            // Java: int maxYIdx = 0;
            // Java: for (int i = 1; i < points.length; i++)
            {
                // Java: if (points[i].getY() > points[maxYIdx].getY())
                {
                    // Java: maxYIdx = i;
                }
            }
            // Java: return maxYIdx;
        todo!()
    }

    // Java: double interpolateXAtY()
    pub fn interpolateXAtY(&self) -> f64 {
        todo!()
    }

    // Java: WeightedObservedPoint[] getInterpolationPointsForY()
    pub fn getInterpolationPointsForY(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: boolean isBetween()
    pub fn isBetween(&self) -> bool {
        todo!()
    }

}
