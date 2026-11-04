// Java: import java.math.BigInteger
// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;

// Java field: long serialVersionUID

// Java: public class ProperBigFractionFormat extends BigFractionFormat
pub struct ProperBigFractionFormat {
    // Java field: NumberFormat wholeFormat
    pub(crate) wholeFormat: Box<dyn std::any::Any>,
}

// Java: public class ProperBigFractionFormat extends BigFractionFormat
impl ProperBigFractionFormat {
    // Java: ProperBigFractionFormat()
    pub fn new() -> Self {
        // Java: this(getDefaultNumberFormat());
        ProperBigFractionFormat { wholeFormat: Box::new(()) }
    }

    // Java: ProperBigFractionFormat(final NumberFormat format)
    pub fn new_with_format(_format: Box<dyn std::any::Any>) -> Self {
        // Java: this(format, (NumberFormat)format.clone(), (NumberFormat)format.clone());
        ProperBigFractionFormat { wholeFormat: Box::new(()) }
    }

    // Java: ProperBigFractionFormat()
    pub fn new_with_three_formats(_whole_format: Box<dyn std::any::Any>, _numerator_format: Box<dyn std::any::Any>, _denominator_format: Box<dyn std::any::Any>) -> Self {
        ProperBigFractionFormat { wholeFormat: _whole_format }
    }

    // Java: StringBuffer format()
    pub fn format(&self) -> String {
        todo!("ProperBigFractionFormat::format")
    }

    // Java: NumberFormat getWholeFormat()
    pub fn getWholeFormat(&self) -> &dyn std::any::Any {
        // Java: return wholeFormat;
        &*self.wholeFormat
    }

    // Java: BigFraction parse(final String source, final ParsePosition pos)
    pub fn parse(&self, _source: &str, _pos: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // try to parse improper BigFraction
        // Java: BigFraction ret = super.parse(source, pos);
        // Java: if (ret != null)
        {
            // Java: return ret;
        }
        // Java: final int initialIndex = pos.getIndex();
        // Java: // parse whitespace
        // Java: parseAndIgnoreWhitespace(source, pos);
        // Java: // parse whole
        // Java: BigInteger whole = parseNextBigInteger(source, pos);
        // Java: if (whole == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: // parse whitespace
        // Java: parseAndIgnoreWhitespace(source, pos);
        // Java: // parse numerator
        // Java: BigInteger num = parseNextBigInteger(source, pos);
        // Java: if (num == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: if (num.compareTo(BigInteger.ZERO) < 0)
        {
            // Java: // minus signs should be leading, invalid expression
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: // parse '/'
        // Java: final int startIndex = pos.getIndex();
        // Java: final char c = parseNextCharacter(source, pos);
        // Java: switch (c)
        {
        // Java: case 0 :
            // Java: // no '/'
            // Java: // return num as a BigFraction
            // Java: return new BigFraction(num);
        // Java: case '/' :
            // Java: // found '/', continue parsing denominator
            // Java: break;
        // Java: default :
            // Java: // invalid '/'
            // Java: // set index back to initial, error index should be the last
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: pos.setErrorIndex(startIndex);
            // Java: return null;
        }
        // Java: // parse whitespace
        // Java: parseAndIgnoreWhitespace(source, pos);
        // Java: // parse denominator
        // Java: final BigInteger den = parseNextBigInteger(source, pos);
        // Java: if (den == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: if (den.compareTo(BigInteger.ZERO) < 0)
        {
            // Java: // minus signs must be leading, invalid
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: boolean wholeIsNeg = whole.compareTo(BigInteger.ZERO) < 0;
        // Java: if (wholeIsNeg)
        {
            // Java: whole = whole.negate();
        }
        // Java: num = whole.multiply(den).add(num);
        // Java: if (wholeIsNeg)
        {
            // Java: num = num.negate();
        }
        // Java: return new BigFraction(num, den);
        todo!("ProperBigFractionFormat::parse")
    }

    // Java: void setWholeFormat(final NumberFormat format)
    pub fn setWholeFormat(&mut self, format: Box<dyn std::any::Any>) {
        // Java: if (format == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.WHOLE_FORMAT);
        }
        // Java: this.wholeFormat = format;
        self.wholeFormat = format;
    }

}
