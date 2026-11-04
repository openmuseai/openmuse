// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public class to
pub struct to {
}

// Java: public class ParameterJacobianWrapper
pub struct ParameterJacobianWrapper {
    // Java field: FirstOrderDifferentialEquations fode
    fode: Box<dyn std::any::Any>,
    // Java field: ParameterizedODE pode
    pub(crate) pode: Box<dyn std::any::Any>,
    // Java field: Map<String, Double> hParam
    hParam: Box<dyn std::any::Any>,
}

// Java: public class to
impl to {
}

// Java: public class ParameterJacobianWrapper
impl ParameterJacobianWrapper {
    // Java: Collection<String> getParametersNames()
    pub fn getParametersNames(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return pode.getParametersNames();
        todo!()
    }

    // Java: boolean isSupported(String name)
    pub fn isSupported(&self) -> bool {
        // Java: return pode.isSupported(name);
        todo!()
    }

    // Java: void computeParameterJacobian()
    pub fn computeParameterJacobian(&self) {
    }

}
