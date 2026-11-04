// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import java.util.Locale
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathParseException
use crate::org::apache::commons::math3::exception::MathParseException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.CompositeFormat
use crate::org::apache::commons::math3::util::CompositeFormat;

use super::Complex::Complex;

// Java field: String DEFAULT_IMAGINARY_CHARACTER
pub(crate) const DEFAULT_IMAGINARY_CHARACTER: &str = "i";

// Java: public class ComplexFormat
pub struct ComplexFormat {
    // Java field: String imaginaryCharacter
    pub(crate) imaginaryCharacter: String,
    // Java field: NumberFormat imaginaryFormat
    // Java field: NumberFormat realFormat
}

// Java: public class ComplexFormat
impl ComplexFormat {
    // Java: ComplexFormat()
    pub fn ComplexFormat() -> Self {
        // Java: this.imaginaryCharacter = DEFAULT_IMAGINARY_CHARACTER;
        // Java: this.imaginaryFormat = CompositeFormat.getDefaultNumberFormat();
        // Java: this.realFormat = imaginaryFormat;
        ComplexFormat { imaginaryCharacter: DEFAULT_IMAGINARY_CHARACTER.to_string() }
    }

    // Java: ComplexFormat(NumberFormat format)
    pub fn ComplexFormat_fmt() -> Self {
        // Java: if (format == null)
        // Java: throw new NullArgumentException(LocalizedFormats.IMAGINARY_FORMAT);
        // Java: this.imaginaryCharacter = DEFAULT_IMAGINARY_CHARACTER;
        // Java: this.imaginaryFormat = format;
        // Java: this.realFormat = format;
        ComplexFormat { imaginaryCharacter: DEFAULT_IMAGINARY_CHARACTER.to_string() }
    }

    // Java: ComplexFormat(NumberFormat realFormat, NumberFormat imaginaryFormat)
    pub fn ComplexFormat_two_fmt() -> Self {
        ComplexFormat { imaginaryCharacter: DEFAULT_IMAGINARY_CHARACTER.to_string() }
    }

    // Java: ComplexFormat(String imaginaryCharacter)
    pub fn ComplexFormat_char(imaginary_character: String) -> Self {
        ComplexFormat { imaginaryCharacter: imaginary_character }
    }

    // Java: ComplexFormat(String imaginaryCharacter, NumberFormat format)
    pub fn ComplexFormat_char_fmt(imaginary_character: String) -> Self {
        ComplexFormat { imaginaryCharacter: imaginary_character }
    }

    // Java: ComplexFormat()
    pub fn ComplexFormat_full(imaginary_character: String) -> Self {
        ComplexFormat { imaginaryCharacter: imaginary_character }
    }

    // Java: static Locale[] getAvailableLocales()
    pub fn getAvailableLocales() -> Vec<String> {
        // Java: return NumberFormat.getAvailableLocales();
        vec!["en".to_string()]
    }

    // Java: String format(Complex c)
    pub fn format(&self, c: &Complex) -> String {
        // Java: return format(c, new StringBuffer(), new FieldPosition(0)).toString();
        let mut sb = String::new();
        sb.push_str(&format!("{}", c.getReal()));
        if c.getImaginary() >= 0.0 {
            sb.push_str(" + ");
            sb.push_str(&format!("{}", c.getImaginary()));
        } else {
            sb.push_str(" - ");
            sb.push_str(&format!("{}", -c.getImaginary()));
        }
        sb.push_str(&self.imaginaryCharacter);
        sb
    }

    // Java: String format(Double c)
    pub fn format_f64(&self, c: f64) -> String {
        // Java: return format(new Complex(c, 0), new StringBuffer(), new FieldPosition(0)).toString();
        self.format(&Complex::new(c, 0.0))
    }

    // Java: StringBuffer format()
    pub fn format_buf(&self, c: &Complex, buf: &mut String) -> String {
        buf.push_str(&format!("{}", c.getReal()));
        if c.getImaginary() >= 0.0 {
            buf.push_str(" + ");
            buf.push_str(&format!("{}", c.getImaginary()));
        } else {
            buf.push_str(" - ");
            buf.push_str(&format!("{}", -c.getImaginary()));
        }
        buf.push_str(&self.imaginaryCharacter);
        buf.clone()
    }

    // Java: StringBuffer formatImaginary()
    pub fn formatImaginary(&self, im: f64) -> String {
        format!("{}{}", im, &self.imaginaryCharacter)
    }

    // Java: StringBuffer format()
    pub fn format_obj(&self, c: &Complex) -> String {
        self.format(c)
    }

    // Java: String getImaginaryCharacter()
    pub fn getImaginaryCharacter(&self) -> &str {
        // Java: return imaginaryCharacter;
        &self.imaginaryCharacter
    }

    // Java: NumberFormat getImaginaryFormat()
    pub fn getImaginaryFormat(&self) {
        // Java: return imaginaryFormat;
    }

    // Java: static ComplexFormat getInstance()
    pub fn getInstance() -> ComplexFormat {
        // Java: return getInstance(Locale.getDefault());
        ComplexFormat::ComplexFormat()
    }

    // Java: static ComplexFormat getInstance(Locale locale)
    pub fn getInstance_locale(_locale: &str) -> ComplexFormat {
        // Java: NumberFormat f = CompositeFormat.getDefaultNumberFormat(locale);
        // Java: return new ComplexFormat(f);
        ComplexFormat::ComplexFormat()
    }

    // Java: static ComplexFormat getInstance(String imaginaryCharacter, Locale locale)
    pub fn getInstance_char_locale(imaginary_character: String, _locale: &str) -> ComplexFormat {
        ComplexFormat::ComplexFormat_char(imaginary_character)
    }

    // Java: NumberFormat getRealFormat()
    pub fn getRealFormat(&self) {
        // Java: return realFormat;
    }

    // Java: Complex parse(String source)
    pub fn parse(&self, source: &str) -> Complex {
        // Java: ParsePosition parsePosition = new ParsePosition(0);
        let mut pos = 0usize;
        // Java: Complex result = parse(source, parsePosition);
        let result = self.parse_pos(source, &mut pos);
        // Java: if (parsePosition.getIndex() == 0)
        if pos == 0 {
            // Java: throw new MathParseException(source,
                                         // Java: parsePosition.getErrorIndex(),
                                         // Java: Complex.class);
            panic!("MathParseException: cannot parse '{}'", source);
        }
        // Java: return result;
        result.unwrap()
    }

    // Java: Complex parse(String source, ParsePosition pos)
    pub fn parse_pos(&self, source: &str, pos: &mut usize) -> Option<Complex> {
        // Java: int initialIndex = pos.getIndex();
        let initial_index = *pos;
        // Java: // parse whitespace
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        CompositeFormat::CompositeFormat::parseAndIgnoreWhitespace(source, pos);
        // Java: // parse real
        // Java: Number re = CompositeFormat.parseNumber(source, getRealFormat(), pos);
        let re = CompositeFormat::CompositeFormat::parseNumber(source, pos);
        // Java: if (re == null)
        if re.is_none() {
            // Java: // invalid real number
            // Java: // set index back to initial, error index should already be set
            // Java: pos.setIndex(initialIndex);
            *pos = initial_index;
            // Java: return null;
            return None;
        }
        // Java: // parse sign
        // Java: int startIndex = pos.getIndex();
        let start_index = *pos;
        // Java: char c = CompositeFormat.parseNextCharacter(source, pos);
        let c = CompositeFormat::CompositeFormat::parseNextCharacter(source, pos);
        // Java: int sign = 0;
        let sign: i32;
        // Java: switch (c)
        match c {
        // Java: case 0 :
            '\0' => {
                // Java: // no sign
                // Java: // return real only complex number
                // Java: return new Complex(re.doubleValue(), 0.0);
                return Some(Complex::new(re.unwrap(), 0.0));
            }
        // Java: case '-' :
            '-' => {
                // Java: sign = -1;
                sign = -1;
                // Java: break;
            }
        // Java: case '+' :
            '+' => {
                // Java: sign = 1;
                sign = 1;
                // Java: break;
            }
        // Java: default :
            _ => {
                // Java: // invalid sign
                // Java: // set index back to initial, error index should be the last
                // Java: // character examined.
                // Java: pos.setIndex(initialIndex);
                *pos = initial_index;
                // Java: pos.setErrorIndex(startIndex);
                // Java: return null;
                return None;
            }
        }
        // Java: // parse whitespace
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        CompositeFormat::CompositeFormat::parseAndIgnoreWhitespace(source, pos);
        // Java: // parse imaginary
        // Java: Number im = CompositeFormat.parseNumber(source, getRealFormat(), pos);
        let im = CompositeFormat::CompositeFormat::parseNumber(source, pos);
        // Java: if (im == null)
        if im.is_none() {
            // Java: // invalid imaginary number
            // Java: // set index back to initial, error index should already be set
            // Java: pos.setIndex(initialIndex);
            *pos = initial_index;
            // Java: return null;
            return None;
        }
        // Java: // parse imaginary character
        // Java: if (!CompositeFormat.parseFixedstring(source, getImaginaryCharacter(), pos))
        if !CompositeFormat::CompositeFormat::parseFixedstring(source, self.getImaginaryCharacter(), pos) {
            // Java: return null;
            return None;
        }
        // Java: return new Complex(re.doubleValue(), im.doubleValue() * sign);
        Some(Complex::new(re.unwrap(), im.unwrap() * sign as f64))
    }
}
