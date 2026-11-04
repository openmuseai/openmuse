// Java: import java.io.Serializable
// Java: import java.math.BigInteger
// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import java.util.Locale
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathParseException
use crate::org::apache::commons::math3::exception::MathParseException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID

// Java: public class BigFractionFormat extends AbstractFormat implements Serializable
pub struct BigFractionFormat {
}

// Java: public class BigFractionFormat extends AbstractFormat implements Serializable
impl BigFractionFormat {
    // Java: BigFractionFormat()
    pub fn new() -> Self {
        BigFractionFormat {}
    }

    // Java: BigFractionFormat(final NumberFormat format)
    pub fn new_with_format(_format: Box<dyn std::any::Any>) -> Self {
        // Java: super(format);
        BigFractionFormat {}
    }

    // Java: BigFractionFormat()
    pub fn new_with_two_formats(_numerator_format: Box<dyn std::any::Any>, _denominator_format: Box<dyn std::any::Any>) -> Self {
        BigFractionFormat {}
    }

    // Java: static Locale[] getAvailableLocales()
    pub fn getAvailableLocales() -> Box<dyn std::any::Any> {
        // Java: return NumberFormat.getAvailableLocales();
        todo!("BigFractionFormat::getAvailableLocales")
    }

    // Java: static String formatBigFraction(final BigFraction f)
    pub fn formatBigFraction(_f: &dyn std::any::Any) -> String {
        // Java: return getImproperInstance().format(f);
        todo!("BigFractionFormat::formatBigFraction")
    }

    // Java: static BigFractionFormat getImproperInstance()
    pub fn getImproperInstance() -> BigFractionFormat {
        // Java: return getImproperInstance(Locale.getDefault());
        todo!("BigFractionFormat::getImproperInstance")
    }

    // Java: static BigFractionFormat getImproperInstance(final Locale locale)
    pub fn getImproperInstance_locale(_locale: &dyn std::any::Any) -> BigFractionFormat {
        // Java: return new BigFractionFormat(getDefaultNumberFormat(locale));
        todo!("BigFractionFormat::getImproperInstance_locale")
    }

    // Java: static BigFractionFormat getProperInstance()
    pub fn getProperInstance() -> BigFractionFormat {
        // Java: return getProperInstance(Locale.getDefault());
        todo!("BigFractionFormat::getProperInstance")
    }

    // Java: static BigFractionFormat getProperInstance(final Locale locale)
    pub fn getProperInstance_locale(_locale: &dyn std::any::Any) -> BigFractionFormat {
        // Java: return new ProperBigFractionFormat(getDefaultNumberFormat(locale));
        todo!("BigFractionFormat::getProperInstance_locale")
    }

    // Java: StringBuffer format()
    pub fn format(&self) -> String {
        todo!("BigFractionFormat::format")
    }

    // Java: StringBuffer format()
    pub fn format_long(&self) -> String {
        todo!("BigFractionFormat::format_long")
    }

    // Java: BigFraction parse(final String source)
    pub fn parse(&self, _source: &str) -> Box<dyn std::any::Any> {
        // Java: final ParsePosition parsePosition = new ParsePosition(0);
        // Java: final BigFraction result = parse(source, parsePosition);
        // Java: if (parsePosition.getIndex() == 0)
        {
            // Java: throw new MathParseException(source, parsePosition.getErrorIndex(), BigFraction.class);
        }
        // Java: return result;
        todo!("BigFractionFormat::parse")
    }

    // Java: BigFraction parse(final String source, final ParsePosition pos)
    pub fn parse_with_pos(&self, _source: &str, _pos: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final int initialIndex = pos.getIndex();
        // Java: // parse whitespace
        // Java: parseAndIgnoreWhitespace(source, pos);
        // Java: // parse numerator
        // Java: final BigInteger num = parseNextBigInteger(source, pos);
        // Java: if (num == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
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
        // Java: return new BigFraction(num, den);
        todo!("BigFractionFormat::parse_with_pos")
    }

    // Java: BigInteger parseNextBigInteger()
    pub fn parseNextBigInteger(&self) -> Box<dyn std::any::Any> {
        todo!("BigFractionFormat::parseNextBigInteger")
    }

}
