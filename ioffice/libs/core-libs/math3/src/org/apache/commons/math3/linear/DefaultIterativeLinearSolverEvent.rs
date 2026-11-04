// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;

// Java field: long serialVersionUID

// Java: public class DefaultIterativeLinearSolverEvent extends IterativeLinearSolverEvent
pub struct DefaultIterativeLinearSolverEvent {
    // Java field: RealVector b
    pub(crate) b: Box<dyn std::any::Any>,
    // Java field: RealVector r
    pub(crate) r: Option<Box<dyn std::any::Any>>,
    // Java field: double rnorm
    pub(crate) rnorm: f64,
    // Java field: RealVector x
    pub(crate) x: Box<dyn std::any::Any>,
}

// Java: public class DefaultIterativeLinearSolverEvent extends IterativeLinearSolverEvent
impl DefaultIterativeLinearSolverEvent {
    // Java: DefaultIterativeLinearSolverEvent()
    pub fn DefaultIterativeLinearSolverEvent(&self) {
    }

    // Java: DefaultIterativeLinearSolverEvent()
    pub fn DefaultIterativeLinearSolverEvent_2(&self) {
    }

    // Java: double getNormOfResidual()
    pub fn getNormOfResidual(&self) -> f64 {
        // Java: return rnorm;
        todo!()
    }

    // Java: RealVector getResidual()
    pub fn getResidual(&self) -> Box<dyn std::any::Any> {
        // Java: if (r != null)
        {
            // Java: return r;
        }
        // Java: throw new MathUnsupportedOperationException();
        todo!()
    }

    // Java: RealVector getRightHandSideVector()
    pub fn getRightHandSideVector(&self) -> Box<dyn std::any::Any> {
        // Java: return b;
        todo!()
    }

    // Java: RealVector getSolution()
    pub fn getSolution(&self) -> Box<dyn std::any::Any> {
        // Java: return x;
        todo!()
    }

    // Java: boolean providesResidual()
    pub fn providesResidual(&self) -> bool {
        // Java: return r != null;
        todo!()
    }

}
