// Java: import org.apache.commons.math3.analysis.polynomials.PolynomialFunction
use crate::org::apache::commons::math3::analysis::polynomials::PolynomialFunction;
// Java: import org.apache.commons.math3.complex.Complex
use crate::org::apache::commons::math3::complex::Complex;
// Java: import org.apache.commons.math3.complex.ComplexUtils
use crate::org::apache::commons::math3::complex::ComplexUtils;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_ABSOLUTE_ACCURACY: f64 = 0.0;

// Java: public class LaguerreSolver extends AbstractPolynomialSolver
pub struct LaguerreSolver {
    // Java field: ComplexSolver complexSolver
    complexSolver: ComplexSolver,
}

// Java: public class ComplexSolver
pub struct ComplexSolver {
}

// Java: public class LaguerreSolver extends AbstractPolynomialSolver
impl LaguerreSolver {
    // Java: LaguerreSolver()
    pub fn LaguerreSolver(&self) {
        // Java: this(DEFAULT_ABSOLUTE_ACCURACY);
    }

    // Java: LaguerreSolver(double absoluteAccuracy)
    pub fn LaguerreSolver_2(&self) {
        // Java: super(absoluteAccuracy);
    }

    // Java: LaguerreSolver()
    pub fn LaguerreSolver_3(&self) {
    }

    // Java: LaguerreSolver()
    pub fn LaguerreSolver_4(&self) {
    }

    // Java: double doSolve()
    pub fn doSolve(&self) -> f64 {
        todo!()
    }

    // Java: double laguerre()
    pub fn laguerre(&self) -> f64 {
        todo!()
    }

    // Java: Complex[] solveAllComplex()
    pub fn solveAllComplex(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: Complex[] solveAllComplex()
    pub fn solveAllComplex_2(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: Complex solveComplex()
    pub fn solveComplex(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: Complex solveComplex()
    pub fn solveComplex_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class ComplexSolver
impl ComplexSolver {
    // Java: boolean isRoot(double min, double max, Complex z)
    pub fn isRoot(&self) -> bool {
            // Java: if (isSequence(min, z.getReal(), max))
            {
                // Java: double tolerance = FastMath.max(getRelativeAccuracy() * z.abs(), getAbsoluteAccuracy());
                // Java: return (FastMath.abs(z.getImaginary()) <= tolerance) ||
                     // Java: (z.abs() <= getFunctionValueAccuracy());
            }
            // Java: return false;
        todo!()
    }

    // Java: Complex[] solveAll(Complex coefficients[], Complex initial)
    pub fn solveAll(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: Complex solve(Complex coefficients[], Complex initial)
    pub fn solve(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}
