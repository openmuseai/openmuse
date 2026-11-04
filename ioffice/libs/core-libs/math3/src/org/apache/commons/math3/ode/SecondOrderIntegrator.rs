// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;

// Java: public interface represents
pub trait represents {
}

// Java: public interface SecondOrderIntegrator extends ODEIntegrator
// Java field: throws MathIllegalStateException, MathIllegalArgumentException
pub trait SecondOrderIntegrator {
    // Java: abstract void integrate()
    fn integrate(&self);

}
