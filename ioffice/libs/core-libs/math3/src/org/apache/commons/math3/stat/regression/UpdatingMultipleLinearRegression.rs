// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;

// Java: public interface for
pub trait r#for {
}

// Java: public interface UpdatingMultipleLinearRegression
pub trait UpdatingMultipleLinearRegression {
    // Java: abstract boolean hasIntercept()
    fn hasIntercept(&self) -> bool;

    // Java: abstract long getN()
    fn getN(&self) -> i64;

    // Java: abstract void addObservation(double[] x, double y)
    fn addObservation(&self);

    // Java: abstract void addObservations(double[][] x, double[] y)
    fn addObservations(&self);

    // Java: abstract void clear()
    fn clear(&self);

    // Java: abstract RegressionResults regress()
    fn regress(&self) -> Box<dyn std::any::Any>;

    // Java: abstract RegressionResults regress(int[] variablesToInclude)
    fn regress_2(&self) -> Box<dyn std::any::Any>;

}
