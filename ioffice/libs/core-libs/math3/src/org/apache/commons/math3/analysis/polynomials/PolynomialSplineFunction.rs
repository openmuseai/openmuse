// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.analysis.DifferentiableUnivariateFunction
use crate::org::apache::commons::math3::analysis::DifferentiableUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class PolynomialSplineFunction implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
pub struct PolynomialSplineFunction {
    // Java field: int n
    pub(crate) n: i32,
}

// Java: public class PolynomialSplineFunction implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction
impl PolynomialSplineFunction {
    // Java: PolynomialSplineFunction(double knots[], PolynomialFunction polynomials[])
    pub fn PolynomialSplineFunction(&self) {
    }

    // Java: double value(double v)
    pub fn value(&self) -> f64 {
        // Java: if (v < knots[0] || v > knots[n])
        {
            // Java: throw new OutOfRangeException(v, knots[0], knots[n]);
        }
        // Java: int i = Arrays.binarySearch(knots, v);
        // Java: if (i < 0)
        {
            // Java: i = -i - 2;
        }
        // Java: // This will handle the case where v is the last knot value
        // Java: // There are only n-1 polynomials, so if v is the last knot
        // Java: // then we will use the last polynomial to calculate the value.
        // Java: if ( i >= polynomials.length )
        {
            // Java: i--;
        }
        // Java: return polynomials[i].value(v - knots[i]);
        todo!()
    }

    // Java: UnivariateFunction derivative()
    pub fn derivative(&self) -> Box<dyn std::any::Any> {
        // Java: return polynomialSplineDerivative();
        todo!()
    }

    // Java: PolynomialSplineFunction polynomialSplineDerivative()
    pub fn polynomialSplineDerivative(&self) -> Box<dyn std::any::Any> {
        // Java: PolynomialFunction derivativePolynomials[] = new PolynomialFunction[n];
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: derivativePolynomials[i] = polynomials[i].polynomialDerivative();
        }
        // Java: return new PolynomialSplineFunction(knots, derivativePolynomials);
        todo!()
    }

    // Java: DerivativeStructure value(final DerivativeStructure t)
    pub fn value_2(&self) -> Box<dyn std::any::Any> {
        // Java: final double t0 = t.getValue();
        // Java: if (t0 < knots[0] || t0 > knots[n])
        {
            // Java: throw new OutOfRangeException(t0, knots[0], knots[n]);
        }
        // Java: int i = Arrays.binarySearch(knots, t0);
        // Java: if (i < 0)
        {
            // Java: i = -i - 2;
        }
        // Java: // This will handle the case where t is the last knot value
        // Java: // There are only n-1 polynomials, so if t is the last knot
        // Java: // then we will use the last polynomial to calculate the value.
        // Java: if ( i >= polynomials.length )
        {
            // Java: i--;
        }
        // Java: return polynomials[i].value(t.subtract(knots[i]));
        todo!()
    }

    // Java: int getN()
    pub fn getN(&self) -> i32 {
        // Java: return n;
        todo!()
    }

    // Java: PolynomialFunction[] getPolynomials()
    pub fn getPolynomials(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: PolynomialFunction p[] = new PolynomialFunction[n];
        // Java: System.arraycopy(polynomials, 0, p, 0, n);
        // Java: return p;
        todo!()
    }

    // Java: double[] getKnots()
    pub fn getKnots(&self) -> Vec<f64> {
        // Java: double out[] = new double[n + 1];
        // Java: System.arraycopy(knots, 0, out, 0, n + 1);
        // Java: return out;
        todo!()
    }

    // Java: boolean isValidPoint(double x)
    pub fn isValidPoint(&self) -> bool {
        // Java: if (x < knots[0] ||
            // Java: x > knots[n])
            {
            // Java: return false;
            }
        // Java: else
        {
            // Java: return true;
        }
        todo!()
    }

}


