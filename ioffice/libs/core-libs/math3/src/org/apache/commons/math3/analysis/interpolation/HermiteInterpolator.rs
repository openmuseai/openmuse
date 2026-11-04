// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableVectorFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableVectorFunction;
// Java: import org.apache.commons.math3.analysis.polynomials.PolynomialFunction
use crate::org::apache::commons::math3::analysis::polynomials::PolynomialFunction;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.CombinatoricsUtils
use crate::org::apache::commons::math3::util::CombinatoricsUtils;

// Java: public class HermiteInterpolator implements UnivariateDifferentiableVectorFunction
pub struct HermiteInterpolator {
    // Java field: List<Double> abscissae
    abscissae: Vec<f64>,
    // Java field: List<double[]> topDiagonal
    topDiagonal: Vec<Vec<f64>>,
    // Java field: List<double[]> bottomDiagonal
    bottomDiagonal: Vec<Vec<f64>>,
}

// Java: public class HermiteInterpolator implements UnivariateDifferentiableVectorFunction
impl HermiteInterpolator {
    // Java: HermiteInterpolator()
    pub fn HermiteInterpolator(&self) {
        // Java: this.abscissae      = new ArrayList<Double>();
        // Java: this.topDiagonal    = new ArrayList<double[]>();
        // Java: this.bottomDiagonal = new ArrayList<double[]>();
    }

    // Java: void addSamplePoint(final double x, final double[] ... value)
    pub fn addSamplePoint(&self) {
    }

    // Java: PolynomialFunction[] getPolynomials()
    pub fn getPolynomials(&self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: double[] value(double x)
    pub fn value(&self) -> Vec<f64> {
        todo!()
    }

    // Java: DerivativeStructure[] value(final DerivativeStructure x)
    pub fn value_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void checkInterpolation()
    pub fn checkInterpolation(&self) {
        // Java: if (abscissae.isEmpty())
        {
            // Java: throw new NoDataException(LocalizedFormats.EMPTY_INTERPOLATION_SAMPLE);
        }
    }

    // Java: PolynomialFunction polynomial(double ... c)
    pub fn polynomial(&self) -> Box<dyn std::any::Any> {
        // Java: return new PolynomialFunction(c);
        todo!()
    }

}

