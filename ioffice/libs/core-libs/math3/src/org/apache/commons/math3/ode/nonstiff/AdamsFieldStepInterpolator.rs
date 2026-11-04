// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.linear.Array2DRowFieldMatrix
use crate::org::apache::commons::math3::linear::Array2DRowFieldMatrix;
// Java: import org.apache.commons.math3.ode.FieldEquationsMapper
use crate::org::apache::commons::math3::ode::FieldEquationsMapper;
// Java: import org.apache.commons.math3.ode.FieldODEStateAndDerivative
use crate::org::apache::commons::math3::ode::FieldODEStateAndDerivative;
// Java: import org.apache.commons.math3.ode.sampling.AbstractFieldStepInterpolator
use crate::org::apache::commons::math3::ode::sampling::AbstractFieldStepInterpolator;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class implements implements an interpolator for Adams integrators using Nordsieck representation.
// Java: public class AdamsFieldStepInterpolator
pub struct AdamsFieldStepInterpolator {
    // Java field: T scalingH
    scalingH: Option<Box<dyn std::any::Any>>,
    // Java field: FieldODEStateAndDerivative<T> reference
    reference: Option<Box<dyn std::any::Any>>,
    // Java field: T[] scaled
    scaled: Vec<Box<dyn std::any::Any>>,
    // Java field: Array2DRowFieldMatrix<T> nordsieck
    nordsieck: Option<Box<dyn std::any::Any>>,
}

// Java: public class AdamsFieldStepInterpolator
impl AdamsFieldStepInterpolator {
    // Java: AdamsFieldStepInterpolator()
    pub fn AdamsFieldStepInterpolator(&self) {
    }

    // Java: AdamsFieldStepInterpolator<T> create()
    pub fn create(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldODEStateAndDerivative<T> computeInterpolatedStateAndDerivatives()
    pub fn computeInterpolatedStateAndDerivatives(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static public static <S extends RealFieldElement<S>> FieldODEStateAndDerivative<S> taylor()
    pub fn taylor(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}
