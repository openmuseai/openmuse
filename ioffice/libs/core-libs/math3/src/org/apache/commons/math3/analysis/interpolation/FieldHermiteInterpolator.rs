// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class FieldHermiteInterpolator extends FieldElement
pub struct FieldHermiteInterpolator {
    // Java field: List<T> abscissae
    abscissae: Vec<Box<dyn std::any::Any>>,
    // Java field: List<T[]> topDiagonal
    topDiagonal: Vec<Vec<Box<dyn std::any::Any>>>,
    // Java field: List<T[]> bottomDiagonal
    bottomDiagonal: Vec<Vec<Box<dyn std::any::Any>>>,
}

// Java: public class FieldHermiteInterpolator extends FieldElement
impl FieldHermiteInterpolator {
    // Java: FieldHermiteInterpolator()
    pub fn FieldHermiteInterpolator(&self) {
        // Java: this.abscissae      = new ArrayList<T>();
        // Java: this.topDiagonal    = new ArrayList<T[]>();
        // Java: this.bottomDiagonal = new ArrayList<T[]>();
    }

    // Java: void addSamplePoint(final T x, final T[] ... value)
    pub fn addSamplePoint(&self) {
    }

    // Java: T[] value(T x)
    pub fn value(&self) -> Box<dyn std::any::Any> {
        // Java: // safety check
        // Java: MathUtils.checkNotNull(x);
        // Java: if (abscissae.isEmpty())
        {
            // Java: throw new NoDataException(LocalizedFormats.EMPTY_INTERPOLATION_SAMPLE);
        }
        // Java: final T[] value = MathArrays.buildArray(x.getField(), topDiagonal.get(0).length);
        // Java: T valueCoeff = x.getField().getOne();
        // Java: for (int i = 0; i < topDiagonal.size(); ++i)
        {
            // Java: T[] dividedDifference = topDiagonal.get(i);
            // Java: for (int k = 0; k < value.length; ++k)
            {
                // Java: value[k] = value[k].add(dividedDifference[k].multiply(valueCoeff));
            }
            // Java: final T deltaX = x.subtract(abscissae.get(i));
            // Java: valueCoeff = valueCoeff.multiply(deltaX);
        }
        // Java: return value;
        todo!()
    }

    // Java: T[][] derivatives(T x, int order)
    pub fn derivatives(&self) -> Box<dyn std::any::Any> {
        // Java: // safety check
        // Java: MathUtils.checkNotNull(x);
        // Java: if (abscissae.isEmpty())
        {
            // Java: throw new NoDataException(LocalizedFormats.EMPTY_INTERPOLATION_SAMPLE);
        }
        // Java: final T zero = x.getField().getZero();
        // Java: final T one  = x.getField().getOne();
        // Java: final T[] tj = MathArrays.buildArray(x.getField(), order + 1);
        // Java: tj[0] = zero;
        // Java: for (int i = 0; i < order; ++i)
        {
            // Java: tj[i + 1] = tj[i].add(one);
        }
        // Java: final T[][] derivatives =
                // Java: MathArrays.buildArray(x.getField(), order + 1, topDiagonal.get(0).length);
        // Java: final T[] valueCoeff = MathArrays.buildArray(x.getField(), order + 1);
        // Java: valueCoeff[0] = x.getField().getOne();
        // Java: for (int i = 0; i < topDiagonal.size(); ++i)
        {
            // Java: T[] dividedDifference = topDiagonal.get(i);
            // Java: final T deltaX = x.subtract(abscissae.get(i));
            // Java: for (int j = order; j >= 0; --j)
            {
                // Java: for (int k = 0; k < derivatives[j].length; ++k)
                {
                    // Java: derivatives[j][k] =
                            // Java: derivatives[j][k].add(dividedDifference[k].multiply(valueCoeff[j]));
                }
                // Java: valueCoeff[j] = valueCoeff[j].multiply(deltaX);
                // Java: if (j > 0)
                {
                    // Java: valueCoeff[j] = valueCoeff[j].add(tj[j].multiply(valueCoeff[j - 1]));
                }
            }
        }
        // Java: return derivatives;
        todo!()
    }

}
