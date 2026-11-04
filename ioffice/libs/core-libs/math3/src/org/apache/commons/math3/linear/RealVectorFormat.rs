// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import java.util.Locale
// Java: import org.apache.commons.math3.exception.MathParseException
use crate::org::apache::commons::math3::exception::MathParseException;
// Java: import org.apache.commons.math3.util.CompositeFormat
use crate::org::apache::commons::math3::util::CompositeFormat;

// Java field: String DEFAULT_PREFIX
pub(crate) const DEFAULT_PREFIX: &str = "{";
// Java field: String DEFAULT_SUFFIX
pub(crate) const DEFAULT_SUFFIX: &str = "}";
// Java field: String DEFAULT_SEPARATOR
pub(crate) const DEFAULT_SEPARATOR: &str = "; ";

// Java: public class RealVectorFormat
pub struct RealVectorFormat {
    // Java field: String prefix
    pub(crate) prefix: String,
    // Java field: String suffix
    pub(crate) suffix: String,
    // Java field: String separator
    pub(crate) separator: String,
    // Java field: String trimmedPrefix
    trimmedPrefix: String,
    // Java field: String trimmedSuffix
    trimmedSuffix: String,
    // Java field: String trimmedSeparator
    trimmedSeparator: String,
    // Java field: NumberFormat format
    pub(crate) format: Box<dyn std::any::Any>,
}

// Java: public class RealVectorFormat
impl RealVectorFormat {
    // Java: RealVectorFormat()
    pub fn RealVectorFormat(&self) {
        // Java: this(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_SEPARATOR,
             // Java: CompositeFormat.getDefaultNumberFormat());
    }

    // Java: RealVectorFormat(final NumberFormat format)
    pub fn RealVectorFormat_1(&self, format: &dyn std::any::Any) {
        // Java: this(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_SEPARATOR, format);
    }

    // Java: RealVectorFormat()
    pub fn RealVectorFormat_2(&self) {
    }

    // Java: RealVectorFormat()
    pub fn RealVectorFormat_3(&self) {
    }

    // Java: static Locale[] getAvailableLocales()
    pub fn getAvailableLocales(&self) -> Vec<String> {
        // Java: return NumberFormat.getAvailableLocales();
        todo!();
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> String {
        // Java: return prefix;
        todo!();
    }

    // Java: String getSuffix()
    pub fn getSuffix(&self) -> String {
        // Java: return suffix;
        todo!();
    }

    // Java: String getSeparator()
    pub fn getSeparator(&self) -> String {
        // Java: return separator;
        todo!();
    }

    // Java: NumberFormat getFormat()
    pub fn getFormat(&self) -> Box<dyn std::any::Any> {
        // Java: return format;
        todo!();
    }

    // Java: static RealVectorFormat getInstance()
    pub fn getInstance(&self) -> RealVectorFormat {
        // Java: return getInstance(Locale.getDefault());
        todo!();
    }

    // Java: static RealVectorFormat getInstance(final Locale locale)
    pub fn getInstance_1(&self, locale: &str) -> RealVectorFormat {
        // Java: return new RealVectorFormat(CompositeFormat.getDefaultNumberFormat(locale));
        todo!();
    }

    // Java: String format(RealVector v)
    pub fn format(&self, v: &dyn std::any::Any) -> String {
        // Java: return format(v, new StringBuffer(), new FieldPosition(0)).toString();
        todo!();
    }

    // Java: StringBuffer format()
    pub fn format_1(&self) -> String {
        todo!();
    }

    // Java: ArrayRealVector parse(String source)
    pub fn parse(&self, source: &str) -> Box<dyn std::any::Any> {
        // Java: final ParsePosition parsePosition = new ParsePosition(0);
        // Java: final ArrayRealVector result = parse(source, parsePosition);
        // Java: if (parsePosition.getIndex() == 0)
        {
            // Java: throw new MathParseException(source,
                                         // Java: parsePosition.getErrorIndex(),
                                         // Java: ArrayRealVector.class);
        }
        // Java: return result;
        todo!();
    }

    // Java: ArrayRealVector parse(String source, ParsePosition pos)
    pub fn parse_1(&self, source: &str, pos: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: int initialIndex = pos.getIndex();
        // Java: // parse prefix
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        // Java: if (!CompositeFormat.parseFixedstring(source, trimmedPrefix, pos))
        {
            // Java: return null;
        }
        // Java: // parse components
        // Java: List<Number> components = new ArrayList<Number>();
        // Java: for (boolean loop = true; loop;)
        {
            // Java: if (!components.isEmpty())
            {
                // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
                // Java: if (!CompositeFormat.parseFixedstring(source, trimmedSeparator, pos))
                {
                    // Java: loop = false;
                }
            }
            // Java: if (loop)
            {
                // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
                // Java: Number component = CompositeFormat.parseNumber(source, format, pos);
                // Java: if (component != null)
                {
                    // Java: components.add(component);
                }
                // Java: else
                {
                    // Java: // invalid component
                    // Java: // set index back to initial, error index should already be set
                    // Java: pos.setIndex(initialIndex);
                    // Java: return null;
                }
            }
        }
        // Java: // parse suffix
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        // Java: if (!CompositeFormat.parseFixedstring(source, trimmedSuffix, pos))
        {
            // Java: return null;
        }
        // Java: // build vector
        // Java: double[] data = new double[components.size()];
        // Java: for (int i = 0; i < data.length; ++i)
        {
            // Java: data[i] = components.get(i).doubleValue();
        }
        // Java: return new ArrayRealVector(data, false);
        todo!();
    }

}
