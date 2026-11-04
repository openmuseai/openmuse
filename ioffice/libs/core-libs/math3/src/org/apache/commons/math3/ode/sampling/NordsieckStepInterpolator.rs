// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.ode.EquationsMapper
use crate::org::apache::commons::math3::ode::EquationsMapper;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID

// Java: public class implements implements an interpolator for integrators using Nordsieck representation.
pub struct implements {
}

// Java: public class NordsieckStepInterpolator
pub struct NordsieckStepInterpolator {
    // Java field: double[] stateVariation
    pub(crate) stateVariation: Box<dyn std::any::Any>,
    // Java field: double scalingH
    pub(crate) scalingH: f64,
    // Java field: double referenceTime
    pub(crate) referenceTime: f64,
    // Java field: double[] scaled
    pub(crate) scaled: Box<dyn std::any::Any>,
    // Java field: Array2DRowRealMatrix nordsieck
    pub(crate) nordsieck: Box<dyn std::any::Any>,
}

// Java: public class implements implements an interpolator for integrators using Nordsieck representation.
impl implements {
}

// Java: public class NordsieckStepInterpolator
impl NordsieckStepInterpolator {
    // Java: NordsieckStepInterpolator()
    pub fn NordsieckStepInterpolator(&self) {
    }

    // Java: NordsieckStepInterpolator(final NordsieckStepInterpolator interpolator)
    pub fn NordsieckStepInterpolator_2(&self) {
        // Java: super(interpolator);
        // Java: scalingH      = interpolator.scalingH;
        // Java: referenceTime = interpolator.referenceTime;
        // Java: if (interpolator.scaled != null)
        {
            // Java: scaled = interpolator.scaled.clone();
        }
        // Java: if (interpolator.nordsieck != null)
        {
            // Java: nordsieck = new Array2DRowRealMatrix(interpolator.nordsieck.getDataRef(), true);
        }
        // Java: if (interpolator.stateVariation != null)
        {
            // Java: stateVariation = interpolator.stateVariation.clone();
        }
    }

    // Java: StepInterpolator doCopy()
    pub fn doCopy(&self) -> Box<dyn std::any::Any> {
        // Java: return new NordsieckStepInterpolator(this);
        todo!()
    }

    // Java: void reinitialize()
    pub fn reinitialize(&self) {
    }

    // Java: void reinitialize()
    pub fn reinitialize_2(&self) {
    }

    // Java: void rescale(final double stepSize)
    pub fn rescale(&self) {
        // Java: final double ratio = stepSize / scalingH;
        // Java: for (int i = 0; i < scaled.length; ++i)
        {
            // Java: scaled[i] *= ratio;
        }
        // Java: final double[][] nData = nordsieck.getDataRef();
        // Java: double power = ratio;
        // Java: for (int i = 0; i < nData.length; ++i)
        {
            // Java: power *= ratio;
            // Java: final double[] nDataI = nData[i];
            // Java: for (int j = 0; j < nDataI.length; ++j)
            {
                // Java: nDataI[j] *= power;
            }
        }
        // Java: scalingH = stepSize;
    }

    // Java: double[] getInterpolatedStateVariation()
    pub fn getInterpolatedStateVariation(&self) -> Vec<f64> {
        // Java: // compute and ignore interpolated state
        // Java: // to make sure state variation is computed as a side effect
        // Java: getInterpolatedState();
        // Java: return stateVariation;
        todo!()
    }

    // Java: void computeInterpolatedStateAndDerivatives(final double theta, final double oneMinusThetaH)
    pub fn computeInterpolatedStateAndDerivatives(&self) {
        // Java: final double x = interpolatedTime - referenceTime;
        // Java: final double normalizedAbscissa = x / scalingH;
        // Java: Arrays.fill(stateVariation, 0.0);
        // Java: Arrays.fill(interpolatedDerivatives, 0.0);
        // Java: // apply Taylor formula from high order to low order,
        // Java: // for the sake of numerical accuracy
        // Java: final double[][] nData = nordsieck.getDataRef();
        // Java: for (int i = nData.length - 1; i >= 0; --i)
        {
            // Java: final int order = i + 2;
            // Java: final double[] nDataI = nData[i];
            // Java: final double power = FastMath.pow(normalizedAbscissa, order);
            // Java: for (int j = 0; j < nDataI.length; ++j)
            {
                // Java: final double d = nDataI[j] * power;
                // Java: stateVariation[j]          += d;
                // Java: interpolatedDerivatives[j] += order * d;
            }
        }
        // Java: for (int j = 0; j < currentState.length; ++j)
        {
            // Java: stateVariation[j] += scaled[j] * normalizedAbscissa;
            // Java: interpolatedState[j] = currentState[j] + stateVariation[j];
            // Java: interpolatedDerivatives[j] =
                // Java: (interpolatedDerivatives[j] + scaled[j] * normalizedAbscissa) / x;
        }
    }

    // Java: void writeExternal(final ObjectOutput out)
    pub fn writeExternal(&self) {
    }

    // Java: void readExternal(final ObjectInput in)
    pub fn readExternal(&self) {
    }

}

impl implements {
}
