// Java: import java.io.Serializable
// Java: import java.math.BigDecimal
// Java: import java.math.BigInteger
// Java: import java.math.MathContext
// Java: import java.math.RoundingMode
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: BigReal ZERO
static ZERO: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: BigReal ONE
static ONE: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: long serialVersionUID

// Java: public class is
pub struct is {
}

// Java: public class BigReal implements FieldElement, Comparable, Serializable
pub struct BigReal {
    // Java field: BigDecimal d
    pub(crate) d: Box<dyn std::any::Any>,
    // Java field: RoundingMode roundingMode
    pub(crate) roundingMode: Box<dyn std::any::Any>,
    // Java field: int scale
    pub(crate) scale: i32,
}

// Java: public class is
impl is {
}

// Java: public class BigReal implements FieldElement, Comparable, Serializable
impl BigReal {
    // Java: BigReal(BigDecimal val)
    pub fn BigReal(&self) {
        // Java: d =  val;
    }

    // Java: BigReal(BigInteger val)
    pub fn BigReal_2(&self) {
        // Java: d = new BigDecimal(val);
    }

    // Java: BigReal(BigInteger unscaledVal, int scale)
    pub fn BigReal_3(&self) {
        // Java: d = new BigDecimal(unscaledVal, scale);
    }

    // Java: BigReal(BigInteger unscaledVal, int scale, MathContext mc)
    pub fn BigReal_4(&self) {
        // Java: d = new BigDecimal(unscaledVal, scale, mc);
    }

    // Java: BigReal(BigInteger val, MathContext mc)
    pub fn BigReal_5(&self) {
        // Java: d = new BigDecimal(val, mc);
    }

    // Java: BigReal(char[] in)
    pub fn BigReal_6(&self) {
        // Java: d = new BigDecimal(in);
    }

    // Java: BigReal(char[] in, int offset, int len)
    pub fn BigReal_7(&self) {
        // Java: d = new BigDecimal(in, offset, len);
    }

    // Java: BigReal(char[] in, int offset, int len, MathContext mc)
    pub fn BigReal_8(&self) {
        // Java: d = new BigDecimal(in, offset, len, mc);
    }

    // Java: BigReal(char[] in, MathContext mc)
    pub fn BigReal_9(&self) {
        // Java: d = new BigDecimal(in, mc);
    }

    // Java: BigReal(double val)
    pub fn BigReal_10(&self) {
        // Java: d = new BigDecimal(val);
    }

    // Java: BigReal(double val, MathContext mc)
    pub fn BigReal_11(&self) {
        // Java: d = new BigDecimal(val, mc);
    }

    // Java: BigReal(int val)
    pub fn BigReal_12(&self) {
        // Java: d = new BigDecimal(val);
    }

    // Java: BigReal(int val, MathContext mc)
    pub fn BigReal_13(&self) {
        // Java: d = new BigDecimal(val, mc);
    }

    // Java: BigReal(long val)
    pub fn BigReal_14(&self) {
        // Java: d = new BigDecimal(val);
    }

    // Java: BigReal(long val, MathContext mc)
    pub fn BigReal_15(&self) {
        // Java: d = new BigDecimal(val, mc);
    }

    // Java: BigReal(String val)
    pub fn BigReal_16(&self) {
        // Java: d = new BigDecimal(val);
    }

    // Java: BigReal(String val, MathContext mc)
    pub fn BigReal_17(&self) {
        // Java: d = new BigDecimal(val, mc);
    }

    // Java: RoundingMode getRoundingMode()
    pub fn getRoundingMode(&self) -> Box<dyn std::any::Any> {
        // Java: return roundingMode;
        todo!()
    }

    // Java: void setRoundingMode(RoundingMode roundingMode)
    pub fn setRoundingMode(&self) {
        // Java: this.roundingMode = roundingMode;
    }

    // Java: int getScale()
    pub fn getScale(&self) -> i32 {
        // Java: return scale;
        todo!()
    }

    // Java: void setScale(int scale)
    pub fn setScale(&self) {
        // Java: this.scale = scale;
    }

    // Java: BigReal add(BigReal a)
    pub fn add(&self) -> Box<dyn std::any::Any> {
        // Java: return new BigReal(d.add(a.d));
        todo!()
    }

    // Java: BigReal subtract(BigReal a)
    pub fn subtract(&self) -> Box<dyn std::any::Any> {
        // Java: return new BigReal(d.subtract(a.d));
        todo!()
    }

    // Java: BigReal negate()
    pub fn negate(&self) -> Box<dyn std::any::Any> {
        // Java: return new BigReal(d.negate());
        todo!()
    }

    // Java: BigReal divide(BigReal a)
    pub fn divide(&self) -> Box<dyn std::any::Any> {
        // Java: try
        {
            // Java: return new BigReal(d.divide(a.d, scale, roundingMode));
        }
        // Java: catch (ArithmeticException e)
        {
            // Java: // Division by zero has occurred
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NOT_ALLOWED);
        }
        todo!()
    }

    // Java: BigReal reciprocal()
    pub fn reciprocal(&self) -> Box<dyn std::any::Any> {
        // Java: try
        {
            // Java: return new BigReal(BigDecimal.ONE.divide(d, scale, roundingMode));
        }
        // Java: catch (ArithmeticException e)
        {
            // Java: // Division by zero has occurred
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NOT_ALLOWED);
        }
        todo!()
    }

    // Java: BigReal multiply(BigReal a)
    pub fn multiply(&self) -> Box<dyn std::any::Any> {
        // Java: return new BigReal(d.multiply(a.d));
        todo!()
    }

    // Java: BigReal multiply(final int n)
    pub fn multiply_2(&self) -> Box<dyn std::any::Any> {
        // Java: return new BigReal(d.multiply(new BigDecimal(n)));
        todo!()
    }

    // Java: int compareTo(BigReal a)
    pub fn compareTo(&self) -> i32 {
        // Java: return d.compareTo(a.d);
        todo!()
    }

    // Java: double doubleValue()
    pub fn doubleValue(&self) -> f64 {
        // Java: return d.doubleValue();
        todo!()
    }

    // Java: BigDecimal bigDecimalValue()
    pub fn bigDecimalValue(&self) -> Box<dyn std::any::Any> {
        // Java: return d;
        todo!()
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (other instanceof BigReal)
        {
            // Java: return d.equals(((BigReal) other).d);
        }
        // Java: return false;
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return d.hashCode();
        todo!()
    }

    // Java: Field<BigReal> getField()
    pub fn getField(&self) -> Box<dyn std::any::Any> {
        // Java: return BigRealField.getInstance();
        todo!()
    }

}



