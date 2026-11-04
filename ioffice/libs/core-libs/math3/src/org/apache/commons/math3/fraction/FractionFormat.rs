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

// Java: public class FractionFormat extends AbstractFormat
pub struct FractionFormat {
}

// Java: public class FractionFormat extends AbstractFormat
impl FractionFormat {
    // Java: FractionFormat()
    pub fn new() -> Self {
        FractionFormat {}
    }

    // Java: FractionFormat(final NumberFormat format)
    pub fn new_with_format(_format: Box<dyn std::any::Any>) -> Self {
        // Java: super(format);
        FractionFormat {}
    }

    // Java: FractionFormat()
    pub fn new_with_two_formats(_numerator_format: Box<dyn std::any::Any>, _denominator_format: Box<dyn std::any::Any>) -> Self {
        FractionFormat {}
    }

    // Java: static Locale[] getAvailableLocales()
    pub fn getAvailableLocales() -> Box<dyn std::any::Any> {
        // Java: return NumberFormat.getAvailableLocales();
        todo!("FractionFormat::getAvailableLocales")
    }

    // Java: static String formatFraction(Fraction f)
    pub fn formatFraction(_f: &dyn std::any::Any) -> String {
        // Java: return getImproperInstance().format(f);
        todo!("FractionFormat::formatFraction")
    }

    // Java: static FractionFormat getImproperInstance()
    pub fn getImproperInstance() -> FractionFormat {
        // Java: return getImproperInstance(Locale.getDefault());
        todo!("FractionFormat::getImproperInstance")
    }

    // Java: static FractionFormat getImproperInstance(final Locale locale)
    pub fn getImproperInstance_locale(_locale: &dyn std::any::Any) -> FractionFormat {
        // Java: return new FractionFormat(getDefaultNumberFormat(locale));
        todo!("FractionFormat::getImproperInstance_locale")
    }

    // Java: static FractionFormat getProperInstance()
    pub fn getProperInstance() -> FractionFormat {
        // Java: return getProperInstance(Locale.getDefault());
        todo!("FractionFormat::getProperInstance")
    }

    // Java: static FractionFormat getProperInstance(final Locale locale)
    pub fn getProperInstance_locale(_locale: &dyn std::any::Any) -> FractionFormat {
        // Java: return new ProperFractionFormat(getDefaultNumberFormat(locale));
        todo!("FractionFormat::getProperInstance_locale")
    }

    // Java: static NumberFormat getDefaultNumberFormat()
    pub fn getDefaultNumberFormat() -> Box<dyn std::any::Any> {
        // Java: return getDefaultNumberFormat(Locale.getDefault());
        todo!("FractionFormat::getDefaultNumberFormat")
    }

    // Java: StringBuffer format()
    pub fn format(&self) -> String {
        todo!("FractionFormat::format")
    }

    // Java: StringBuffer format()
    pub fn format_long(&self) -> String {
        todo!("FractionFormat::format_long")
    }

    // Java: Fraction parse(final String source)
    pub fn parse(&self, _source: &str) -> Box<dyn std::any::Any> {
        // Java: final ParsePosition parsePosition = new ParsePosition(0);
        // Java: final Fraction result = parse(source, parsePosition);
        // Java: if (parsePosition.getIndex() == 0)
        {
            // Java: throw new MathParseException(source, parsePosition.getErrorIndex(), Fraction.class);
        }
        // Java: return result;
        todo!("FractionFormat::parse")
    }

    // Java: Fraction parse(final String source, final ParsePosition pos)
    pub fn parse_with_pos(&self, _source: &str, _pos: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final int initialIndex = pos.getIndex();
        // Java: // parse whitespace
        // Java: parseAndIgnoreWhitespace(source, pos);
        // Java: // parse numerator
        // Java: final Number num = getNumeratorFormat().parse(source, pos);
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
        // Java: final Number den = getDenominatorFormat().parse(source, pos);
        // Java: if (den == null)
        {
            // Java: // invalid integer number
            // Java: // set index back to initial, error index should already be set
            // Java: // character examined.
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: return new Fraction(num.intValue(), den.intValue());
        todo!("FractionFormat::parse_with_pos")
    }

}
