// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class ProperFractionFormat extends FractionFormat
pub struct ProperFractionFormat {
    // Java field: NumberFormat wholeFormat
    pub(crate) wholeFormat: Box<dyn std::any::Any>,
}

// Java: public class ProperFractionFormat extends FractionFormat
impl ProperFractionFormat {
    // Java: ProperFractionFormat()
    pub fn new() -> Self {
        // Java: this(getDefaultNumberFormat());
        ProperFractionFormat { wholeFormat: Box::new(()) }
    }

    // Java: ProperFractionFormat(NumberFormat format)
    pub fn new_with_format(_format: Box<dyn std::any::Any>) -> Self {
        // Java: this(format, (NumberFormat)format.clone(), (NumberFormat)format.clone());
        ProperFractionFormat { wholeFormat: Box::new(()) }
    }

    // Java: ProperFractionFormat()
    pub fn new_with_three_formats(_whole_format: Box<dyn std::any::Any>, _numerator_format: Box<dyn std::any::Any>, _denominator_format: Box<dyn std::any::Any>) -> Self {
        ProperFractionFormat { wholeFormat: _whole_format }
    }

    // Java: StringBuffer format()
    pub fn format(&self) -> String {
        todo!("ProperFractionFormat::format")
    }

    // Java: NumberFormat getWholeFormat()
    pub fn getWholeFormat(&self) -> &dyn std::any::Any {
        // Java: return wholeFormat;
        &*self.wholeFormat
    }

    // Java: Fraction parse(String source, ParsePosition pos)
    pub fn parse(&self, _source: &str, _pos: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // try to parse improper fraction
        // Java: Fraction ret = super.parse(source, pos);
        // Java: if (ret != null)
        {
            // Java: return ret;
        }
        // Java: int initialIndex = pos.getIndex();
        // Java: // parse whitespace
        // Java: parseAndIgnoreWhitespace(source, pos);
        // Java: // parse whole
        // Java: Number whole = getWholeFormat().parse(source, pos);
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
        // Java: Number num = getNumeratorFormat().parse(source, pos);
        // Java: if (num == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: if (num.intValue() < 0)
        {
            // Java: // minus signs should be leading, invalid expression
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: // parse '/'
        // Java: int startIndex = pos.getIndex();
        // Java: char c = parseNextCharacter(source, pos);
        // Java: switch (c)
        {
        // Java: case 0 :
            // Java: // no '/'
            // Java: // return num as a fraction
            // Java: return new Fraction(num.intValue(), 1);
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
        // Java: Number den = getDenominatorFormat().parse(source, pos);
        // Java: if (den == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: if (den.intValue() < 0)
        {
            // Java: // minus signs must be leading, invalid
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: int w = whole.intValue();
        // Java: int n = num.intValue();
        // Java: int d = den.intValue();
        // Java: return new Fraction(((FastMath.abs(w) * d) + n) * MathUtils.copySign(1, w), d);
        todo!("ProperFractionFormat::parse")
    }

    // Java: void setWholeFormat(NumberFormat format)
    pub fn setWholeFormat(&mut self, format: Box<dyn std::any::Any>) {
        // Java: if (format == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.WHOLE_FORMAT);
        }
        // Java: this.wholeFormat = format;
        self.wholeFormat = format;
    }

}
