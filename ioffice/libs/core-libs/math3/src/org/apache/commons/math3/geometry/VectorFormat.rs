// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import java.util.Locale
// Java: import org.apache.commons.math3.util.CompositeFormat
use crate::org::apache::commons::math3::util::CompositeFormat;
// Java: import org.apache.commons.math3.exception.MathParseException
use crate::org::apache::commons::math3::exception::MathParseException;

// Java field: String DEFAULT_PREFIX
pub(crate) const DEFAULT_PREFIX: &str = "{";
// Java field: String DEFAULT_SUFFIX
pub(crate) const DEFAULT_SUFFIX: &str = "}";
// Java field: String DEFAULT_SEPARATOR
pub(crate) const DEFAULT_SEPARATOR: &str = "; ";

// Java: public class VectorFormat extends Space
pub struct VectorFormat {
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

// Java: public class VectorFormat extends Space
impl VectorFormat {
    // Java: VectorFormat()
    pub fn new() -> Self {
        // Java: this(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_SEPARATOR,
             // Java: CompositeFormat.getDefaultNumberFormat());
        Self::new_with_parts(DEFAULT_PREFIX.to_string(), DEFAULT_SUFFIX.to_string(),
                             DEFAULT_SEPARATOR.to_string(), Box::new(()))
    }

    // Java: VectorFormat(final NumberFormat format)
    pub fn new_with_format(format: Box<dyn std::any::Any>) -> Self {
        // Java: this(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_SEPARATOR, format);
        Self::new_with_parts(DEFAULT_PREFIX.to_string(), DEFAULT_SUFFIX.to_string(),
                             DEFAULT_SEPARATOR.to_string(), format)
    }

    // Java: VectorFormat()
    pub fn new_with_parts(prefix: String, suffix: String, separator: String, format: Box<dyn std::any::Any>) -> Self {
        let trimmed_prefix = prefix.trim().to_string();
        let trimmed_suffix = suffix.trim().to_string();
        let trimmed_separator = separator.trim().to_string();
        VectorFormat {
            prefix,
            suffix,
            separator,
            trimmedPrefix: trimmed_prefix,
            trimmedSuffix: trimmed_suffix,
            trimmedSeparator: trimmed_separator,
            format,
        }
    }

    // Java: VectorFormat()
    pub fn new_with_prefix_suffix_separator(prefix: String, suffix: String, separator: String) -> Self {
        Self::new_with_parts(prefix, suffix, separator, Box::new(()))
    }

    // Java: static Locale[] getAvailableLocales()
    pub fn getAvailableLocales() -> Vec<String> {
        // Java: return NumberFormat.getAvailableLocales();
        vec![]
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> &str {
        // Java: return prefix;
        &self.prefix
    }

    // Java: String getSuffix()
    pub fn getSuffix(&self) -> &str {
        // Java: return suffix;
        &self.suffix
    }

    // Java: String getSeparator()
    pub fn getSeparator(&self) -> &str {
        // Java: return separator;
        &self.separator
    }

    // Java: NumberFormat getFormat()
    pub fn getFormat(&self) -> &dyn std::any::Any {
        // Java: return format;
        &*self.format
    }

    // Java: String format(Vector<S> vector)
    pub fn format_vector(&self, _vector: &dyn std::any::Any) -> String {
        // Java: return format(vector, new StringBuffer(), new FieldPosition(0)).toString();
        todo!("VectorFormat::format_vector")
    }

    // Java: abstract StringBuffer format()
    pub fn format_to_buffer(&self, _vector: &dyn std::any::Any) -> String {
        todo!("VectorFormat::format_to_buffer")
    }

    // Java: StringBuffer format()
    pub fn format_coordinates(&self, _coords: &[f64]) -> String {
        todo!("VectorFormat::format_coordinates")
    }

    // Java: abstract Vector<S> parse(String source)
    pub fn parse(&self, _source: &str) -> Box<dyn std::any::Any> {
        todo!("VectorFormat::parse")
    }

    // Java: abstract Vector<S> parse(String source, ParsePosition pos)
    pub fn parse_with_pos(&self, _source: &str, _pos: usize) -> Box<dyn std::any::Any> {
        todo!("VectorFormat::parse_with_pos")
    }

    // Java: double[] parseCoordinates(int dimension, String source, ParsePosition pos)
    pub fn parseCoordinates(&self, _dimension: i32, _source: &str, _pos: usize) -> Option<Vec<f64>> {
        // Java: int initialIndex = pos.getIndex();
        // Java: double[] coordinates = new double[dimension];
        // Java: // parse prefix
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        // Java: if (!CompositeFormat.parseFixedstring(source, trimmedPrefix, pos))
        {
            // Java: return null;
        }
        // Java: for (int i = 0; i < dimension; ++i)
        {
            // Java: // skip whitespace
            // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
            // Java: // parse separator
            // Java: if (i > 0 && !CompositeFormat.parseFixedstring(source, trimmedSeparator, pos))
            {
                // Java: return null;
            }
            // Java: // skip whitespace
            // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
            // Java: // parse coordinate
            // Java: Number c = CompositeFormat.parseNumber(source, format, pos);
            // Java: if (c == null)
            {
                // Java: // invalid coordinate
                // Java: // set index back to initial, error index should already be set
                // Java: pos.setIndex(initialIndex);
                // Java: return null;
            }
            // Java: // store coordinate
            // Java: coordinates[i] = c.doubleValue();
        }
        // Java: // parse suffix
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        // Java: if (!CompositeFormat.parseFixedstring(source, trimmedSuffix, pos))
        {
            // Java: return null;
        }
        // Java: return coordinates;
        todo!("VectorFormat::parseCoordinates")
    }

}
