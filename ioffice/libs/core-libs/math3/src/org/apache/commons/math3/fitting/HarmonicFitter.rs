// Java: import org.apache.commons.math3.optim.nonlinear.vector.MultivariateVectorOptimizer
use crate::org::apache::commons::math3::optim::nonlinear::vector::MultivariateVectorOptimizer;
// Java: import org.apache.commons.math3.analysis.function.HarmonicOscillator
use crate::org::apache::commons::math3::analysis::function::HarmonicOscillator;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class HarmonicFitter extends CurveFitter
pub struct HarmonicFitter {
}

// Java: public class ParameterGuesser
pub struct HarmonicFitter_ParameterGuesser {
    // Java field: double a
    pub(crate) a: f64,
    // Java field: double omega
    pub(crate) omega: f64,
    // Java field: double phi
    pub(crate) phi: f64,
}

// Java: public class HarmonicFitter extends CurveFitter
impl HarmonicFitter {
    // Java: HarmonicFitter(final MultivariateVectorOptimizer optimizer)
    pub fn new(_optimizer: MultivariateVectorOptimizer::MultivariateVectorOptimizer) -> Self {
        // Java: super(optimizer);
        HarmonicFitter {}
    }

    // Java: double[] fit(double[] initialGuess)
    pub fn fit(&self, _initial_guess: &[f64]) -> Vec<f64> {
        // Java: return fit(new HarmonicOscillator.Parametric(), initialGuess);
        todo!("HarmonicFitter::fit")
    }

    // Java: double[] fit()
    pub fn fit_default(&self) -> Vec<f64> {
        // Java: return fit((new ParameterGuesser(getObservations())).guess());
        todo!("HarmonicFitter::fit_default")
    }

}

// Java: public class ParameterGuesser
impl HarmonicFitter_ParameterGuesser {
    // Java: ParameterGuesser(WeightedObservedPoint[] observations)
    pub fn new(_observations: &dyn std::any::Any) -> Self {
            // Java: if (observations.length < 4)
            {
                // Java: throw new NumberIsTooSmallException(LocalizedFormats.INSUFFICIENT_OBSERVED_POINTS_IN_SAMPLE,
                                                    // Java: observations.length, 4, true);
            }
            // Java: final WeightedObservedPoint[] sorted = sortObservations(observations);
            // Java: final double aOmega[] = guessAOmega(sorted);
            // Java: a = aOmega[0];
            // Java: omega = aOmega[1];
            // Java: phi = guessPhi(sorted);
        HarmonicFitter_ParameterGuesser { a: 0.0, omega: 0.0, phi: 0.0 }
    }

    // Java: double[] guess()
    pub fn guess(&self) -> Vec<f64> {
            // Java: return new double[] { a, omega, phi };
            vec![self.a, self.omega, self.phi]
    }

    // Java: WeightedObservedPoint[] sortObservations(WeightedObservedPoint[] unsorted)
    pub fn sortObservations(&self, _unsorted: &dyn std::any::Any) -> Box<dyn std::any::Any> {
            // Java: final WeightedObservedPoint[] observations = unsorted.clone();
            // Java: // Since the samples are almost always already sorted, this
            // Java: // method is implemented as an insertion sort that reorders the
            // Java: // elements in place. Insertion sort is very efficient in this case.
            // Java: WeightedObservedPoint curr = observations[0];
            // Java: for (int j = 1; j < observations.length; ++j)
            {
                // Java: WeightedObservedPoint prec = curr;
                // Java: curr = observations[j];
                // Java: if (curr.getX() < prec.getX())
                {
                    // Java: // the current element should be inserted closer to the beginning
                    // Java: int i = j - 1;
                    // Java: WeightedObservedPoint mI = observations[i];
                    // Java: while ((i >= 0) && (curr.getX() < mI.getX()))
                    {
                        // Java: observations[i + 1] = mI;
                        // Java: if (i-- != 0)
                        {
                            // Java: mI = observations[i];
                        }
                    }
                    // Java: observations[i + 1] = curr;
                    // Java: curr = observations[j];
                }
            }
            // Java: return observations;
        todo!("ParameterGuesser::sortObservations")
    }

    // Java: double[] guessAOmega(WeightedObservedPoint[] observations)
    pub fn guessAOmega(&self, _observations: &dyn std::any::Any) -> Vec<f64> {
            // Java: final double[] aOmega = new double[2];
            // Java: // initialize the sums for the linear model between the two integrals
            // Java: double sx2 = 0;
            // Java: double sy2 = 0;
            // Java: double sxy = 0;
            // Java: double sxz = 0;
            // Java: double syz = 0;
            // Java: double currentX = observations[0].getX();
            // Java: double currentY = observations[0].getY();
            // Java: double f2Integral = 0;
            // Java: double fPrime2Integral = 0;
            // Java: final double startX = currentX;
            // Java: for (int i = 1; i < observations.length; ++i)
            {
                // Java: // one step forward
                // Java: final double previousX = currentX;
                // Java: final double previousY = currentY;
                // Java: currentX = observations[i].getX();
                // Java: currentY = observations[i].getY();
                // Java: // update the integrals of f<sup>2</sup> and f'<sup>2</sup>
                // Java: // considering a linear model for f (and therefore constant f')
                // Java: final double dx = currentX - previousX;
                // Java: final double dy = currentY - previousY;
                // Java: final double f2StepIntegral =
                    // Java: dx * (previousY * previousY + previousY * currentY + currentY * currentY) / 3;
                // Java: final double fPrime2StepIntegral = dy * dy / dx;
                // Java: final double x = currentX - startX;
                // Java: f2Integral += f2StepIntegral;
                // Java: fPrime2Integral += fPrime2StepIntegral;
                // Java: sx2 += x * x;
                // Java: sy2 += f2Integral * f2Integral;
                // Java: sxy += x * f2Integral;
                // Java: sxz += x * fPrime2Integral;
                // Java: syz += f2Integral * fPrime2Integral;
            }
            // Java: // compute the amplitude and pulsation coefficients
            // Java: double c1 = sy2 * sxz - sxy * syz;
            // Java: double c2 = sxy * sxz - sx2 * syz;
            // Java: double c3 = sx2 * sy2 - sxy * sxy;
            // Java: if ((c1 / c2 < 0) || (c2 / c3 < 0))
            {
                // Java: final int last = observations.length - 1;
                // Java: // Range of the observations, assuming that the
                // Java: // observations are sorted.
                // Java: final double xRange = observations[last].getX() - observations[0].getX();
                // Java: if (xRange == 0)
                {
                    // Java: throw new ZeroException();
                }
                // Java: aOmega[1] = 2 * Math.PI / xRange;
                // Java: double yMin = Double.POSITIVE_INFINITY;
                // Java: double yMax = Double.NEGATIVE_INFINITY;
                // Java: for (int i = 1; i < observations.length; ++i)
                {
                    // Java: final double y = observations[i].getY();
                    // Java: if (y < yMin)
                    {
                        // Java: yMin = y;
                    }
                    // Java: if (y > yMax)
                    {
                        // Java: yMax = y;
                    }
                }
                // Java: aOmega[0] = 0.5 * (yMax - yMin);
            }
            // Java: else
            {
                // Java: if (c2 == 0)
                {
                    // Java: // In some ill-conditioned cases (cf. MATH-844), the guesser
                    // Java: // procedure cannot produce sensible results.
                    // Java: throw new MathIllegalStateException(LocalizedFormats.ZERO_DENOMINATOR);
                }
                // Java: aOmega[0] = FastMath.sqrt(c1 / c2);
                // Java: aOmega[1] = FastMath.sqrt(c2 / c3);
            }
            // Java: return aOmega;
        todo!("ParameterGuesser::guessAOmega")
    }

    // Java: double guessPhi(WeightedObservedPoint[] observations)
    pub fn guessPhi(&self, _observations: &dyn std::any::Any) -> f64 {
            // Java: // initialize the means
            // Java: double fcMean = 0;
            // Java: double fsMean = 0;
            // Java: double currentX = observations[0].getX();
            // Java: double currentY = observations[0].getY();
            // Java: for (int i = 1; i < observations.length; ++i)
            {
                // Java: // one step forward
                // Java: final double previousX = currentX;
                // Java: final double previousY = currentY;
                // Java: currentX = observations[i].getX();
                // Java: currentY = observations[i].getY();
                // Java: final double currentYPrime = (currentY - previousY) / (currentX - previousX);
                // Java: double omegaX = omega * currentX;
                // Java: double cosine = FastMath.cos(omegaX);
                // Java: double sine = FastMath.sin(omegaX);
                // Java: fcMean += omega * currentY * cosine - currentYPrime * sine;
                // Java: fsMean += omega * currentY * sine + currentYPrime * cosine;
            }
            // Java: return FastMath.atan2(-fsMean, fcMean);
        todo!("ParameterGuesser::guessPhi")
    }

}
