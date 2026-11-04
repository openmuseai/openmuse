// Java: import java.io.Serializable
// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import java.util.Locale
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID

// Java: public class AbstractFormat extends NumberFormat implements Serializable
pub struct AbstractFormat {
    // Java field: NumberFormat denominatorFormat
    pub(crate) denominatorFormat: Box<dyn std::any::Any>,
    // Java field: NumberFormat numeratorFormat
    pub(crate) numeratorFormat: Box<dyn std::any::Any>,
}

// Java: public class AbstractFormat extends NumberFormat implements Serializable
impl AbstractFormat {
    // Java: AbstractFormat()
    pub fn new() -> Self {
        // Java: this(getDefaultNumberFormat());
        AbstractFormat {
            denominatorFormat: Box::new(()),
            numeratorFormat: Box::new(()),
        }
    }

    // Java: AbstractFormat(final NumberFormat format)
    pub fn new_with_format(_format: Box<dyn std::any::Any>) -> Self {
        // Java: this(format, (NumberFormat) format.clone());
        AbstractFormat {
            denominatorFormat: Box::new(()),
            numeratorFormat: Box::new(()),
        }
    }

    // Java: AbstractFormat()
    pub fn new_with_two_formats(_numerator_format: Box<dyn std::any::Any>, _denominator_format: Box<dyn std::any::Any>) -> Self {
        AbstractFormat {
            denominatorFormat: _denominator_format,
            numeratorFormat: _numerator_format,
        }
    }

    // Java: static NumberFormat getDefaultNumberFormat()
    pub fn getDefaultNumberFormat() -> Box<dyn std::any::Any> {
        // Java: return getDefaultNumberFormat(Locale.getDefault());
        todo!("AbstractFormat::getDefaultNumberFormat")
    }

    // Java: static NumberFormat getDefaultNumberFormat(final Locale locale)
    pub fn getDefaultNumberFormat_locale(_locale: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final NumberFormat nf = NumberFormat.getNumberInstance(locale);
        // Java: nf.setMaximumFractionDigits(0);
        // Java: nf.setParseIntegerOnly(true);
        // Java: return nf;
        todo!("AbstractFormat::getDefaultNumberFormat_locale")
    }

    // Java: NumberFormat getDenominatorFormat()
    pub fn getDenominatorFormat(&self) -> &dyn std::any::Any {
        // Java: return denominatorFormat;
        &*self.denominatorFormat
    }

    // Java: NumberFormat getNumeratorFormat()
    pub fn getNumeratorFormat(&self) -> &dyn std::any::Any {
        // Java: return numeratorFormat;
        &*self.numeratorFormat
    }

    // Java: void setDenominatorFormat(final NumberFormat format)
    pub fn setDenominatorFormat(&mut self, format: Box<dyn std::any::Any>) {
        // Java: if (format == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.DENOMINATOR_FORMAT);
        }
        // Java: this.denominatorFormat = format;
        self.denominatorFormat = format;
    }

    // Java: void setNumeratorFormat(final NumberFormat format)
    pub fn setNumeratorFormat(&mut self, format: Box<dyn std::any::Any>) {
        // Java: if (format == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.NUMERATOR_FORMAT);
        }
        // Java: this.numeratorFormat = format;
        self.numeratorFormat = format;
    }

    // Java: static void parseAndIgnoreWhitespace()
    pub fn parseAndIgnoreWhitespace() {
        todo!("AbstractFormat::parseAndIgnoreWhitespace")
    }

    // Java: static char parseNextCharacter()
    pub fn parseNextCharacter() -> char {
        todo!("AbstractFormat::parseNextCharacter")
    }

    // Java: StringBuffer format()
    pub fn format(&self) -> String {
        todo!("AbstractFormat::format")
    }

    // Java: StringBuffer format()
    pub fn format_long(&self) -> String {
        todo!("AbstractFormat::format_long")
    }

}
