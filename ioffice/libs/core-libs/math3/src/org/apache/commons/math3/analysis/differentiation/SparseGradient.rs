// Java: import java.io.Serializable
// Java: import java.util.Collections
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: long serialVersionUID

// Java: public class plays
pub struct plays {
}

// Java: public class should
pub struct should {
}

// Java: public class SparseGradient implements RealFieldElement, Serializable
pub struct SparseGradient {
    // Java field: double value
    pub(crate) value: f64,
    // Java field: Map<Integer, Double> derivatives
    pub(crate) derivatives: Box<dyn std::any::Any>,
}

// Java: public class plays
impl plays {
}

// Java: public class should
impl should {
}

// Java: public class SparseGradient implements RealFieldElement, Serializable
impl SparseGradient {
    // Java: SparseGradient(final double value, final Map<Integer, Double> derivatives)
    pub fn SparseGradient(&self) {
        // Java: this.value = value;
        // Java: this.derivatives = new HashMap<Integer, Double>();
        // Java: if (derivatives != null)
        {
            // Java: this.derivatives.putAll(derivatives);
        }
    }

    // Java: SparseGradient()
    pub fn SparseGradient_2(&self) {
    }

    // Java: static SparseGradient createConstant(final double value)
    pub fn createConstant(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(value, Collections.<Integer, Double> emptyMap());
        todo!()
    }

    // Java: static SparseGradient createVariable(final int idx, final double value)
    pub fn createVariable(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(value, Collections.singletonMap(idx, 1.0));
        todo!()
    }

    // Java: int numVars()
    pub fn numVars(&self) -> i32 {
        // Java: return derivatives.size();
        todo!()
    }

    // Java: double getDerivative(final int index)
    pub fn getDerivative(&self) -> f64 {
        // Java: final Double out = derivatives.get(index);
        // Java: return (out == null) ? 0.0 : out;
        todo!()
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
        // Java: return value;
        todo!()
    }

    // Java: double getReal()
    pub fn getReal(&self) -> f64 {
        // Java: return value;
        todo!()
    }

    // Java: SparseGradient add(final SparseGradient a)
    pub fn add(&self) -> Box<dyn std::any::Any> {
        // Java: final SparseGradient out = new SparseGradient(value + a.value, derivatives);
        // Java: for (Map.Entry<Integer, Double> entry : a.derivatives.entrySet())
        {
            // Java: final int id = entry.getKey();
            // Java: final Double old = out.derivatives.get(id);
            // Java: if (old == null)
            {
                // Java: out.derivatives.put(id, entry.getValue());
            }
            // Java: else
            {
                // Java: out.derivatives.put(id, old + entry.getValue());
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: void addInPlace(final SparseGradient a)
    pub fn addInPlace(&self) {
        // Java: value += a.value;
        // Java: for (final Map.Entry<Integer, Double> entry : a.derivatives.entrySet())
        {
            // Java: final int id = entry.getKey();
            // Java: final Double old = derivatives.get(id);
            // Java: if (old == null)
            {
                // Java: derivatives.put(id, entry.getValue());
            }
            // Java: else
            {
                // Java: derivatives.put(id, old + entry.getValue());
            }
        }
    }

    // Java: SparseGradient add(final double c)
    pub fn add_2(&self) -> Box<dyn std::any::Any> {
        // Java: final SparseGradient out = new SparseGradient(value + c, derivatives);
        // Java: return out;
        todo!()
    }

    // Java: SparseGradient subtract(final SparseGradient a)
    pub fn subtract(&self) -> Box<dyn std::any::Any> {
        // Java: final SparseGradient out = new SparseGradient(value - a.value, derivatives);
        // Java: for (Map.Entry<Integer, Double> entry : a.derivatives.entrySet())
        {
            // Java: final int id = entry.getKey();
            // Java: final Double old = out.derivatives.get(id);
            // Java: if (old == null)
            {
                // Java: out.derivatives.put(id, -entry.getValue());
            }
            // Java: else
            {
                // Java: out.derivatives.put(id, old - entry.getValue());
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: SparseGradient subtract(double c)
    pub fn subtract_2(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(value - c, derivatives);
        todo!()
    }

    // Java: SparseGradient multiply(final SparseGradient a)
    pub fn multiply(&self) -> Box<dyn std::any::Any> {
        // Java: final SparseGradient out =
            // Java: new SparseGradient(value * a.value, Collections.<Integer, Double> emptyMap());
        // Java: // Derivatives.
        // Java: for (Map.Entry<Integer, Double> entry : derivatives.entrySet())
        {
            // Java: out.derivatives.put(entry.getKey(), a.value * entry.getValue());
        }
        // Java: for (Map.Entry<Integer, Double> entry : a.derivatives.entrySet())
        {
            // Java: final int id = entry.getKey();
            // Java: final Double old = out.derivatives.get(id);
            // Java: if (old == null)
            {
                // Java: out.derivatives.put(id, value * entry.getValue());
            }
            // Java: else
            {
                // Java: out.derivatives.put(id, old + value * entry.getValue());
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: void multiplyInPlace(final SparseGradient a)
    pub fn multiplyInPlace(&self) {
        // Java: // Derivatives.
        // Java: for (Map.Entry<Integer, Double> entry : derivatives.entrySet())
        {
            // Java: derivatives.put(entry.getKey(), a.value * entry.getValue());
        }
        // Java: for (Map.Entry<Integer, Double> entry : a.derivatives.entrySet())
        {
            // Java: final int id = entry.getKey();
            // Java: final Double old = derivatives.get(id);
            // Java: if (old == null)
            {
                // Java: derivatives.put(id, value * entry.getValue());
            }
            // Java: else
            {
                // Java: derivatives.put(id, old + value * entry.getValue());
            }
        }
        // Java: value *= a.value;
    }

    // Java: SparseGradient multiply(final double c)
    pub fn multiply_2(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(value * c, c, derivatives);
        todo!()
    }

    // Java: SparseGradient multiply(final int n)
    pub fn multiply_3(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(value * n, n, derivatives);
        todo!()
    }

    // Java: SparseGradient divide(final SparseGradient a)
    pub fn divide(&self) -> Box<dyn std::any::Any> {
        // Java: final SparseGradient out = new SparseGradient(value / a.value, Collections.<Integer, Double> emptyMap());
        // Java: // Derivatives.
        // Java: for (Map.Entry<Integer, Double> entry : derivatives.entrySet())
        {
            // Java: out.derivatives.put(entry.getKey(), entry.getValue() / a.value);
        }
        // Java: for (Map.Entry<Integer, Double> entry : a.derivatives.entrySet())
        {
            // Java: final int id = entry.getKey();
            // Java: final Double old = out.derivatives.get(id);
            // Java: if (old == null)
            {
                // Java: out.derivatives.put(id, -out.value / a.value * entry.getValue());
            }
            // Java: else
            {
                // Java: out.derivatives.put(id, old - out.value / a.value * entry.getValue());
            }
        }
        // Java: return out;
        todo!()
    }

    // Java: SparseGradient divide(final double c)
    pub fn divide_2(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(value / c, 1.0 / c, derivatives);
        todo!()
    }

    // Java: SparseGradient negate()
    pub fn negate(&self) -> SparseGradient {
        // Java: return new SparseGradient(-value, -1.0, derivatives);
        todo!()
    }

    // Java: Field<SparseGradient> getField()
    pub fn getField(&self) -> Box<dyn std::any::Any> {
        // Java: return new Field<SparseGradient>()
        {
            // Java: /** {@inheritDoc} */
            // Java: public SparseGradient getZero()
            {
                // Java: return createConstant(0);
            }
            // Java: /** {@inheritDoc} */
            // Java: public SparseGradient getOne()
            {
                // Java: return createConstant(1);
            }
            // Java: /** {@inheritDoc} */
            // Java: public Class<? extends FieldElement<SparseGradient>> getRuntimeClass()
            {
                // Java: return SparseGradient.class;
            }
        }
        todo!()
    }

    // Java: SparseGradient remainder(final double a)
    pub fn remainder(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(FastMath.IEEEremainder(value, a), derivatives);
        todo!()
    }

    // Java: SparseGradient remainder(final SparseGradient a)
    pub fn remainder_2(&self) -> Box<dyn std::any::Any> {
        // Java: // compute k such that lhs % rhs = lhs - k rhs
        // Java: final double rem = FastMath.IEEEremainder(value, a.value);
        // Java: final double k   = FastMath.rint((value - rem) / a.value);
        // Java: return subtract(a.multiply(k));
        todo!()
    }

    // Java: SparseGradient abs()
    pub fn abs(&self) -> SparseGradient {
        // Java: if (Double.doubleToLongBits(value) < 0)
        {
            // Java: // we use the bits representation to also handle -0.0
            // Java: return negate();
        }
        // Java: else
        {
            // Java: return this;
        }
        todo!()
    }

    // Java: SparseGradient ceil()
    pub fn ceil(&self) -> SparseGradient {
        // Java: return createConstant(FastMath.ceil(value));
        todo!()
    }

    // Java: SparseGradient floor()
    pub fn floor(&self) -> SparseGradient {
        // Java: return createConstant(FastMath.floor(value));
        todo!()
    }

    // Java: SparseGradient rint()
    pub fn rint(&self) -> SparseGradient {
        // Java: return createConstant(FastMath.rint(value));
        todo!()
    }

    // Java: long round()
    pub fn round(&self) -> i64 {
        // Java: return FastMath.round(value);
        todo!()
    }

    // Java: SparseGradient signum()
    pub fn signum(&self) -> SparseGradient {
        // Java: return createConstant(FastMath.signum(value));
        todo!()
    }

    // Java: SparseGradient copySign(final SparseGradient sign)
    pub fn copySign(&self) -> Box<dyn std::any::Any> {
        // Java: final long m = Double.doubleToLongBits(value);
        // Java: final long s = Double.doubleToLongBits(sign.value);
        // Java: if ((m >= 0 && s >= 0) || (m < 0 && s < 0)) { // Sign is currently OK
            // Java: return this;
        // Java: return negate(); // flip sign
        todo!()
    }

    // Java: SparseGradient copySign(final double sign)
    pub fn copySign_2(&self) -> Box<dyn std::any::Any> {
        // Java: final long m = Double.doubleToLongBits(value);
        // Java: final long s = Double.doubleToLongBits(sign);
        // Java: if ((m >= 0 && s >= 0) || (m < 0 && s < 0)) { // Sign is currently OK
            // Java: return this;
        // Java: return negate(); // flip sign
        todo!()
    }

    // Java: SparseGradient scalb(final int n)
    pub fn scalb(&self) -> Box<dyn std::any::Any> {
        // Java: final SparseGradient out = new SparseGradient(FastMath.scalb(value, n), Collections.<Integer, Double> emptyMap());
        // Java: for (Map.Entry<Integer, Double> entry : derivatives.entrySet())
        {
            // Java: out.derivatives.put(entry.getKey(), FastMath.scalb(entry.getValue(), n));
        }
        // Java: return out;
        todo!()
    }

    // Java: SparseGradient hypot(final SparseGradient y)
    pub fn hypot(&self) -> Box<dyn std::any::Any> {
        // Java: if (Double.isInfinite(value) || Double.isInfinite(y.value))
        {
            // Java: return createConstant(Double.POSITIVE_INFINITY);
        }
        // Java: else if (Double.isNaN(value) || Double.isNaN(y.value))
        {
            // Java: return createConstant(Double.NaN);
        }
        // Java: else
        {
            // Java: final int expX = FastMath.getExponent(value);
            // Java: final int expY = FastMath.getExponent(y.value);
            // Java: if (expX > expY + 27)
            {
                // Java: // y is negligible with respect to x
                // Java: return abs();
            }
            // Java: else if (expY > expX + 27)
            {
                // Java: // x is negligible with respect to y
                // Java: return y.abs();
            }
            // Java: else
            {
                // Java: // find an intermediate scale to avoid both overflow and underflow
                // Java: final int middleExp = (expX + expY) / 2;
                // Java: // scale parameters without losing precision
                // Java: final SparseGradient scaledX = scalb(-middleExp);
                // Java: final SparseGradient scaledY = y.scalb(-middleExp);
                // Java: // compute scaled hypotenuse
                // Java: final SparseGradient scaledH =
                        // Java: scaledX.multiply(scaledX).add(scaledY.multiply(scaledY)).sqrt();
                // Java: // remove scaling
                // Java: return scaledH.scalb(middleExp);
            }
        }
        todo!()
    }

    // Java: static SparseGradient hypot(final SparseGradient x, final SparseGradient y)
    pub fn hypot_2(&self) -> Box<dyn std::any::Any> {
        // Java: return x.hypot(y);
        todo!()
    }

    // Java: SparseGradient reciprocal()
    pub fn reciprocal(&self) -> SparseGradient {
        // Java: return new SparseGradient(1.0 / value, -1.0 / (value * value), derivatives);
        todo!()
    }

    // Java: SparseGradient sqrt()
    pub fn sqrt(&self) -> SparseGradient {
        // Java: final double sqrt = FastMath.sqrt(value);
        // Java: return new SparseGradient(sqrt, 0.5 / sqrt, derivatives);
        todo!()
    }

    // Java: SparseGradient cbrt()
    pub fn cbrt(&self) -> SparseGradient {
        // Java: final double cbrt = FastMath.cbrt(value);
        // Java: return new SparseGradient(cbrt, 1.0 / (3 * cbrt * cbrt), derivatives);
        todo!()
    }

    // Java: SparseGradient rootN(final int n)
    pub fn rootN(&self) -> Box<dyn std::any::Any> {
        // Java: if (n == 2)
        {
            // Java: return sqrt();
        }
        // Java: else if (n == 3)
        {
            // Java: return cbrt();
        }
        // Java: else
        {
            // Java: final double root = FastMath.pow(value, 1.0 / n);
            // Java: return new SparseGradient(root, 1.0 / (n * FastMath.pow(root, n - 1)), derivatives);
        }
        todo!()
    }

    // Java: SparseGradient pow(final double p)
    pub fn pow(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(FastMath.pow(value,  p), p * FastMath.pow(value,  p - 1), derivatives);
        todo!()
    }

    // Java: SparseGradient pow(final int n)
    pub fn pow_2(&self) -> Box<dyn std::any::Any> {
        // Java: if (n == 0)
        {
            // Java: return getField().getOne();
        }
        // Java: else
        {
            // Java: final double valueNm1 = FastMath.pow(value,  n - 1);
            // Java: return new SparseGradient(value * valueNm1, n * valueNm1, derivatives);
        }
        todo!()
    }

    // Java: SparseGradient pow(final SparseGradient e)
    pub fn pow_3(&self) -> Box<dyn std::any::Any> {
        // Java: return log().multiply(e).exp();
        todo!()
    }

    // Java: static SparseGradient pow(final double a, final SparseGradient x)
    pub fn pow_4(&self) -> Box<dyn std::any::Any> {
        // Java: if (a == 0)
        {
            // Java: if (x.value == 0)
            {
                // Java: return x.compose(1.0, Double.NEGATIVE_INFINITY);
            }
            // Java: else if (x.value < 0)
            {
                // Java: return x.compose(Double.NaN, Double.NaN);
            }
            // Java: else
            {
                // Java: return x.getField().getZero();
            }
        }
        // Java: else
        {
            // Java: final double ax = FastMath.pow(a, x.value);
            // Java: return new SparseGradient(ax, ax * FastMath.log(a), x.derivatives);
        }
        todo!()
    }

    // Java: SparseGradient exp()
    pub fn exp(&self) -> SparseGradient {
        // Java: final double e = FastMath.exp(value);
        // Java: return new SparseGradient(e, e, derivatives);
        todo!()
    }

    // Java: SparseGradient expm1()
    pub fn expm1(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.expm1(value), FastMath.exp(value), derivatives);
        todo!()
    }

    // Java: SparseGradient log()
    pub fn log(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.log(value), 1.0 / value, derivatives);
        todo!()
    }

    // Java: SparseGradient log10()
    pub fn log10(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.log10(value), 1.0 / (FastMath.log(10.0) * value), derivatives);
        todo!()
    }

    // Java: SparseGradient log1p()
    pub fn log1p(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.log1p(value), 1.0 / (1.0 + value), derivatives);
        todo!()
    }

    // Java: SparseGradient cos()
    pub fn cos(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.cos(value), -FastMath.sin(value), derivatives);
        todo!()
    }

    // Java: SparseGradient sin()
    pub fn sin(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.sin(value), FastMath.cos(value), derivatives);
        todo!()
    }

    // Java: SparseGradient tan()
    pub fn tan(&self) -> SparseGradient {
        // Java: final double t = FastMath.tan(value);
        // Java: return new SparseGradient(t, 1 + t * t, derivatives);
        todo!()
    }

    // Java: SparseGradient acos()
    pub fn acos(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.acos(value), -1.0 / FastMath.sqrt(1 - value * value), derivatives);
        todo!()
    }

    // Java: SparseGradient asin()
    pub fn asin(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.asin(value), 1.0 / FastMath.sqrt(1 - value * value), derivatives);
        todo!()
    }

    // Java: SparseGradient atan()
    pub fn atan(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.atan(value), 1.0 / (1 + value * value), derivatives);
        todo!()
    }

    // Java: SparseGradient atan2(final SparseGradient x)
    pub fn atan2(&self) -> Box<dyn std::any::Any> {
        // Java: // compute r = sqrt(x^2+y^2)
        // Java: final SparseGradient r = multiply(this).add(x.multiply(x)).sqrt();
        // Java: final SparseGradient a;
        // Java: if (x.value >= 0)
        {
            // Java: // compute atan2(y, x) = 2 atan(y / (r + x))
            // Java: a = divide(r.add(x)).atan().multiply(2);
        }
        // Java: else
        {
            // Java: // compute atan2(y, x) = +/- pi - 2 atan(y / (r - x))
            // Java: final SparseGradient tmp = divide(r.subtract(x)).atan().multiply(-2);
            // Java: a = tmp.add(tmp.value <= 0 ? -FastMath.PI : FastMath.PI);
        }
        // Java: // fix value to take special cases (+0/+0, +0/-0, -0/+0, -0/-0, +/-infinity) correctly
        // Java: a.value = FastMath.atan2(value, x.value);
        // Java: return a;
        todo!()
    }

    // Java: static SparseGradient atan2(final SparseGradient y, final SparseGradient x)
    pub fn atan2_2(&self) -> Box<dyn std::any::Any> {
        // Java: return y.atan2(x);
        todo!()
    }

    // Java: SparseGradient cosh()
    pub fn cosh(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.cosh(value), FastMath.sinh(value), derivatives);
        todo!()
    }

    // Java: SparseGradient sinh()
    pub fn sinh(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.sinh(value), FastMath.cosh(value), derivatives);
        todo!()
    }

    // Java: SparseGradient tanh()
    pub fn tanh(&self) -> SparseGradient {
        // Java: final double t = FastMath.tanh(value);
        // Java: return new SparseGradient(t, 1 - t * t, derivatives);
        todo!()
    }

    // Java: SparseGradient acosh()
    pub fn acosh(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.acosh(value), 1.0 / FastMath.sqrt(value * value - 1.0), derivatives);
        todo!()
    }

    // Java: SparseGradient asinh()
    pub fn asinh(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.asinh(value), 1.0 / FastMath.sqrt(value * value + 1.0), derivatives);
        todo!()
    }

    // Java: SparseGradient atanh()
    pub fn atanh(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.atanh(value), 1.0 / (1.0 - value * value), derivatives);
        todo!()
    }

    // Java: SparseGradient toDegrees()
    pub fn toDegrees(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.toDegrees(value), FastMath.toDegrees(1.0), derivatives);
        todo!()
    }

    // Java: SparseGradient toRadians()
    pub fn toRadians(&self) -> SparseGradient {
        // Java: return new SparseGradient(FastMath.toRadians(value), FastMath.toRadians(1.0), derivatives);
        todo!()
    }

    // Java: double taylor(final double ... delta)
    pub fn taylor(&self) -> f64 {
        // Java: double y = value;
        // Java: for (int i = 0; i < delta.length; ++i)
        {
            // Java: y += delta[i] * getDerivative(i);
        }
        // Java: return y;
        todo!()
    }

    // Java: SparseGradient compose(final double f0, final double f1)
    pub fn compose(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseGradient(f0, f1, derivatives);
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination(&self) -> SparseGradient {
        todo!()
    }

    // Java: SparseGradient linearCombination(final double[] a, final SparseGradient[] b)
    pub fn linearCombination_2(&self) -> Box<dyn std::any::Any> {
        // Java: // compute a simple value, with all partial derivatives
        // Java: SparseGradient out = b[0].getField().getZero();
        // Java: for (int i = 0; i < a.length; ++i)
        {
            // Java: out = out.add(b[i].multiply(a[i]));
        }
        // Java: // recompute an accurate value, taking care of cancellations
        // Java: final double[] bDouble = new double[b.length];
        // Java: for (int i = 0; i < b.length; ++i)
        {
            // Java: bDouble[i] = b[i].getValue();
        }
        // Java: out.value = MathArrays.linearCombination(a, bDouble);
        // Java: return out;
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination_3(&self) -> SparseGradient {
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination_4(&self) -> SparseGradient {
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination_5(&self) -> SparseGradient {
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination_6(&self) -> SparseGradient {
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination_7(&self) -> SparseGradient {
        todo!()
    }

    // Java: SparseGradient linearCombination()
    pub fn linearCombination_8(&self) -> SparseGradient {
        todo!()
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (other instanceof SparseGradient)
        {
            // Java: final SparseGradient rhs = (SparseGradient)other;
            // Java: if (!Precision.equals(value, rhs.value, 1))
            {
                // Java: return false;
            }
            // Java: if (derivatives.size() != rhs.derivatives.size())
            {
                // Java: return false;
            }
            // Java: for (final Map.Entry<Integer, Double> entry : derivatives.entrySet())
            {
                // Java: if (!rhs.derivatives.containsKey(entry.getKey()))
                {
                    // Java: return false;
                }
                // Java: if (!Precision.equals(entry.getValue(), rhs.derivatives.get(entry.getKey()), 1))
                {
                    // Java: return false;
                }
            }
            // Java: return true;
        }
        // Java: return false;
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return 743 + 809 * MathUtils.hash(value) + 167 * derivatives.hashCode();
        todo!()
    }

}
