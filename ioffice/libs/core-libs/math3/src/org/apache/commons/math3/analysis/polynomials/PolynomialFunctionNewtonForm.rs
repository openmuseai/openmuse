// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class PolynomialFunctionNewtonForm implements UnivariateDifferentiableFunction
pub struct PolynomialFunctionNewtonForm {
    // Java field: boolean coefficientsComputed
    coefficientsComputed: bool,
}

// Java: public class PolynomialFunctionNewtonForm implements UnivariateDifferentiableFunction
impl PolynomialFunctionNewtonForm {
    // Java: PolynomialFunctionNewtonForm(double a[], double c[])
    pub fn PolynomialFunctionNewtonForm(&self) {
    }

    // Java: double value(double z)
    pub fn value(&self) -> f64 {
       // Java: return evaluate(a, c, z);
        todo!()
    }

    // Java: DerivativeStructure value(final DerivativeStructure t)
    pub fn value_2(&self) -> Box<dyn std::any::Any> {
        // Java: verifyInputArray(a, c);
        // Java: final int n = c.length;
        // Java: DerivativeStructure value = new DerivativeStructure(t.getFreeParameters(), t.getOrder(), a[n]);
        // Java: for (int i = n - 1; i >= 0; i--)
        {
            // Java: value = t.subtract(c[i]).multiply(value).add(a[i]);
        }
        // Java: return value;
        todo!()
    }

    // Java: int degree()
    pub fn degree(&self) -> i32 {
        // Java: return c.length;
        todo!()
    }

    // Java: double[] getNewtonCoefficients()
    pub fn getNewtonCoefficients(&self) -> Vec<f64> {
        // Java: double[] out = new double[a.length];
        // Java: System.arraycopy(a, 0, out, 0, a.length);
        // Java: return out;
        todo!()
    }

    // Java: double[] getCenters()
    pub fn getCenters(&self) -> Vec<f64> {
        // Java: double[] out = new double[c.length];
        // Java: System.arraycopy(c, 0, out, 0, c.length);
        // Java: return out;
        todo!()
    }

    // Java: double[] getCoefficients()
    pub fn getCoefficients(&self) -> Vec<f64> {
        // Java: if (!coefficientsComputed)
        {
            // Java: computeCoefficients();
        }
        // Java: double[] out = new double[coefficients.length];
        // Java: System.arraycopy(coefficients, 0, out, 0, coefficients.length);
        // Java: return out;
        todo!()
    }

    // Java: static double evaluate(double a[], double c[], double z)
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: void computeCoefficients()
    pub fn computeCoefficients(&self) {
        // Java: final int n = degree();
        // Java: coefficients = new double[n+1];
        // Java: for (int i = 0; i <= n; i++)
        {
            // Java: coefficients[i] = 0.0;
        }
        // Java: coefficients[0] = a[n];
        // Java: for (int i = n-1; i >= 0; i--)
        {
            // Java: for (int j = n-i; j > 0; j--)
            {
                // Java: coefficients[j] = coefficients[j-1] - c[i] * coefficients[j];
            }
            // Java: coefficients[0] = a[i] - c[i] * coefficients[0];
        }
        // Java: coefficientsComputed = true;
    }

    // Java: static void verifyInputArray(double a[], double c[])
    pub fn verifyInputArray(&self) {
    }

}

