// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.analysis.DifferentiableUnivariateFunction
use crate::org::apache::commons::math3::analysis::DifferentiableUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.ParametricUnivariateFunction
use crate::org::apache::commons::math3::analysis::ParametricUnivariateFunction;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class PolynomialFunction implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction, Serializable
pub struct PolynomialFunction {
    pub(crate) coefficients: Vec<f64>,
}

// Java: public class Parametric
pub struct PolynomialFunction_Parametric {
}

// Java: public class PolynomialFunction implements UnivariateDifferentiableFunction, DifferentiableUnivariateFunction, Serializable
impl PolynomialFunction {
    // Java: PolynomialFunction(double c[])
    pub fn new(c: Vec<f64>) -> Self {
        PolynomialFunction { coefficients: c }
    }

    // Java: double value(double x)
    pub fn value(&self, x: f64) -> f64 {
       // Java: return evaluate(coefficients, x);
       Self::evaluate(&self.coefficients, x)
    }

    // Java: int degree()
    pub fn degree(&self) -> i32 {
        // Java: return coefficients.length - 1;
        (self.coefficients.len() as i32) - 1
    }

    // Java: double[] getCoefficients()
    pub fn getCoefficients(&self) -> Vec<f64> {
        // Java: return coefficients.clone();
        self.coefficients.clone()
    }

    // Java: static double evaluate(double[] coefficients, double argument)
    pub fn evaluate(coefficients: &[f64], argument: f64) -> f64 {
        let n = coefficients.len();
        if n == 0 {
            return 0.0;
        }
        let mut result = coefficients[n - 1];
        for i in (0..n - 1).rev() {
            result = argument * result + coefficients[i];
        }
        result
    }

    // Java: DerivativeStructure value(final DerivativeStructure t)
    pub fn value_ds(&self, _t: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("PolynomialFunction::value_ds")
    }

    // Java: PolynomialFunction add(final PolynomialFunction p)
    pub fn add(&self, p: &PolynomialFunction) -> PolynomialFunction {
        // Java: // identify the lowest degree polynomial
        // Java: final int lowLength  = FastMath.min(coefficients.length, p.coefficients.length);
        let low_length = self.coefficients.len().min(p.coefficients.len());
        // Java: final int highLength = FastMath.max(coefficients.length, p.coefficients.length);
        let high_length = self.coefficients.len().max(p.coefficients.len());
        // Java: // build the coefficients array
        // Java: double[] newCoefficients = new double[highLength];
        let mut new_coefficients = vec![0.0; high_length];
        // Java: for (int i = 0; i < lowLength; ++i)
        for i in 0..low_length {
            // Java: newCoefficients[i] = coefficients[i] + p.coefficients[i];
            new_coefficients[i] = self.coefficients[i] + p.coefficients[i];
        }
        // Java: System.arraycopy((coefficients.length < p.coefficients.length) ?
                         // Java: p.coefficients : coefficients,
                         // Java: lowLength,
                         // Java: newCoefficients, lowLength,
                         // Java: highLength - lowLength);
        let source = if self.coefficients.len() < p.coefficients.len() { &p.coefficients } else { &self.coefficients };
        new_coefficients[low_length..high_length].copy_from_slice(&source[low_length..high_length]);
        // Java: return new PolynomialFunction(newCoefficients);
        PolynomialFunction::new(new_coefficients)
    }

    // Java: PolynomialFunction subtract(final PolynomialFunction p)
    pub fn subtract(&self, p: &PolynomialFunction) -> PolynomialFunction {
        // Java: // identify the lowest degree polynomial
        // Java: int lowLength  = FastMath.min(coefficients.length, p.coefficients.length);
        let low_length = self.coefficients.len().min(p.coefficients.len());
        // Java: int highLength = FastMath.max(coefficients.length, p.coefficients.length);
        let high_length = self.coefficients.len().max(p.coefficients.len());
        // Java: // build the coefficients array
        // Java: double[] newCoefficients = new double[highLength];
        let mut new_coefficients = vec![0.0; high_length];
        // Java: for (int i = 0; i < lowLength; ++i)
        for i in 0..low_length {
            // Java: newCoefficients[i] = coefficients[i] - p.coefficients[i];
            new_coefficients[i] = self.coefficients[i] - p.coefficients[i];
        }
        // Java: if (coefficients.length < p.coefficients.length)
        if self.coefficients.len() < p.coefficients.len() {
            // Java: for (int i = lowLength; i < highLength; ++i)
            for i in low_length..high_length {
                // Java: newCoefficients[i] = -p.coefficients[i];
                new_coefficients[i] = -p.coefficients[i];
            }
        }
        // Java: else
        else {
            // Java: System.arraycopy(coefficients, lowLength, newCoefficients, lowLength,
                             // Java: highLength - lowLength);
            new_coefficients[low_length..high_length].copy_from_slice(&self.coefficients[low_length..high_length]);
        }
        // Java: return new PolynomialFunction(newCoefficients);
        PolynomialFunction::new(new_coefficients)
    }

    // Java: PolynomialFunction negate()
    pub fn negate(&self) -> PolynomialFunction {
        // Java: double[] newCoefficients = new double[coefficients.length];
        let mut new_coefficients = vec![0.0; self.coefficients.len()];
        // Java: for (int i = 0; i < coefficients.length; ++i)
        for i in 0..self.coefficients.len() {
            // Java: newCoefficients[i] = -coefficients[i];
            new_coefficients[i] = -self.coefficients[i];
        }
        // Java: return new PolynomialFunction(newCoefficients);
        PolynomialFunction::new(new_coefficients)
    }

    // Java: PolynomialFunction multiply(final PolynomialFunction p)
    pub fn multiply(&self, p: &PolynomialFunction) -> PolynomialFunction {
        // Java: double[] newCoefficients = new double[coefficients.length + p.coefficients.length - 1];
        let mut new_coefficients = vec![0.0; self.coefficients.len() + p.coefficients.len() - 1];
        // Java: for (int i = 0; i < newCoefficients.length; ++i)
        for i in 0..new_coefficients.len() {
            // Java: newCoefficients[i] = 0.0;
            new_coefficients[i] = 0.0;
            // Java: for (int j = FastMath.max(0, i + 1 - p.coefficients.length);
                 // Java: j < FastMath.min(coefficients.length, i + 1);
                 // Java: ++j)
            let j_start = if i + 1 > p.coefficients.len() { i + 1 - p.coefficients.len() } else { 0 };
            let j_end = self.coefficients.len().min(i + 1);
            for j in j_start..j_end {
                // Java: newCoefficients[i] += coefficients[j] * p.coefficients[i-j];
                new_coefficients[i] += self.coefficients[j] * p.coefficients[i - j];
            }
        }
        // Java: return new PolynomialFunction(newCoefficients);
        PolynomialFunction::new(new_coefficients)
    }

    // Java: static double[] differentiate(double[] coefficients)
    pub fn differentiate(coefficients: &[f64]) -> Vec<f64> {
        let n = coefficients.len();
        if n <= 1 {
            return vec![0.0];
        }
        let mut result = vec![0.0; n - 1];
        for i in 1..n {
            result[i - 1] = (i as f64) * coefficients[i];
        }
        result
    }

    // Java: PolynomialFunction polynomialDerivative()
    pub fn polynomialDerivative(&self) -> PolynomialFunction {
        // Java: return new PolynomialFunction(differentiate(coefficients));
        PolynomialFunction::new(Self::differentiate(&self.coefficients))
    }

    // Java: UnivariateFunction derivative()
    pub fn derivative(&self) -> Box<dyn std::any::Any> {
        // Java: return polynomialDerivative();
        Box::new(self.polynomialDerivative())
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder s = new StringBuilder();
        let mut s = String::new();
        // Java: if (coefficients[0] == 0.0)
        if self.coefficients[0] == 0.0 {
            // Java: if (coefficients.length == 1)
            if self.coefficients.len() == 1 {
                // Java: return "0";
                return "0".to_string();
            }
        }
        // Java: else
        else {
            // Java: s.append(toString(coefficients[0]));
            s.push_str(&Self::toString_coeff(self.coefficients[0]));
        }
        // Java: for (int i = 1; i < coefficients.length; ++i)
        for i in 1..self.coefficients.len() {
            // Java: if (coefficients[i] != 0)
            if self.coefficients[i] != 0.0 {
                // Java: if (s.length() > 0)
                if !s.is_empty() {
                    // Java: if (coefficients[i] < 0)
                    if self.coefficients[i] < 0.0 {
                        // Java: s.append(" - ");
                        s.push_str(" - ");
                    }
                    // Java: else
                    else {
                        // Java: s.append(" + ");
                        s.push_str(" + ");
                    }
                }
                // Java: else
                else {
                    // Java: if (coefficients[i] < 0)
                    if self.coefficients[i] < 0.0 {
                        // Java: s.append("-");
                        s.push_str("-");
                    }
                }
                // Java: double absAi = FastMath.abs(coefficients[i]);
                let abs_ai = self.coefficients[i].abs();
                // Java: if ((absAi - 1) != 0)
                if (abs_ai - 1.0) != 0.0 {
                    // Java: s.append(toString(absAi));
                    s.push_str(&Self::toString_coeff(abs_ai));
                    // Java: s.append(' ');
                    s.push(' ');
                }
                // Java: s.append("x");
                s.push('x');
                // Java: if (i > 1)
                if i > 1 {
                    // Java: s.append('^');
                    s.push('^');
                    // Java: s.append(Integer.toString(i));
                    s.push_str(&i.to_string());
                }
            }
        }
        // Java: return s.toString();
        s
    }

    // Java: static String toString(double coeff)
    pub fn toString_coeff(coeff: f64) -> String {
        // Java: final String c = Double.toString(coeff);
        let c = coeff.to_string();
        // Java: if (c.endsWith(".0"))
        if c.ends_with(".0") {
            // Java: return c.substring(0, c.length() - 2);
            c[..c.len() - 2].to_string()
        }
        // Java: else
        else {
            // Java: return c;
            c
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: final int prime = 31;
        let prime: i32 = 31;
        // Java: int result = 1;
        let mut result: i32 = 1;
        // Java: result = prime * result + Arrays.hashCode(coefficients);
        let mut h: i32 = 1;
        for &c in &self.coefficients {
            h = 31i32.wrapping_mul(h).wrapping_add(c.to_bits() as i32);
        }
        result = prime.wrapping_mul(result).wrapping_add(h);
        // Java: return result;
        result
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, other: &PolynomialFunction) -> bool {
        // Java: if (this == obj)
        {
            // Java: return true;
        }
        // Java: if (!(obj instanceof PolynomialFunction))
        {
            // Java: return false;
        }
        // Java: PolynomialFunction other = (PolynomialFunction) obj;
        // Java: if (!Arrays.equals(coefficients, other.coefficients))
        if self.coefficients != other.coefficients {
            // Java: return false;
            return false;
        }
        // Java: return true;
        true
    }

}

// Java: public class Parametric
impl PolynomialFunction_Parametric {
    // Java: double[] gradient(double x, double ... parameters)
    pub fn gradient(&self, x: f64, parameters: &[f64]) -> Vec<f64> {
            // Java: final double[] gradient = new double[parameters.length];
            let mut gradient = vec![0.0; parameters.len()];
            // Java: double xn = 1.0;
            let mut xn = 1.0;
            // Java: for (int i = 0; i < parameters.length; ++i)
            for i in 0..parameters.len() {
                // Java: gradient[i] = xn;
                gradient[i] = xn;
                // Java: xn *= x;
                xn *= x;
            }
            // Java: return gradient;
            gradient
    }

    // Java: double value(final double x, final double ... parameters)
    pub fn value(&self, x: f64, parameters: &[f64]) -> f64 {
        PolynomialFunction::evaluate(parameters, x)
    }

}
