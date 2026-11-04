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
// Java field: String DEFAULT_ROW_PREFIX
pub(crate) const DEFAULT_ROW_PREFIX: &str = "{";
// Java field: String DEFAULT_ROW_SUFFIX
pub(crate) const DEFAULT_ROW_SUFFIX: &str = "}";
// Java field: String DEFAULT_ROW_SEPARATOR
pub(crate) const DEFAULT_ROW_SEPARATOR: &str = ",";
// Java field: String DEFAULT_COLUMN_SEPARATOR
pub(crate) const DEFAULT_COLUMN_SEPARATOR: &str = ",";

// Java: public class RealMatrixFormat
pub struct RealMatrixFormat {
    // Java field: String prefix
    pub(crate) prefix: String,
    // Java field: String suffix
    pub(crate) suffix: String,
    // Java field: String rowPrefix
    pub(crate) rowPrefix: String,
    // Java field: String rowSuffix
    pub(crate) rowSuffix: String,
    // Java field: String rowSeparator
    pub(crate) rowSeparator: String,
    // Java field: String columnSeparator
    pub(crate) columnSeparator: String,
    // Java field: NumberFormat format
    pub(crate) format: Box<dyn std::any::Any>,
}

// Java: public class RealMatrixFormat
impl RealMatrixFormat {
    // Java: RealMatrixFormat()
    pub fn RealMatrixFormat(&self) {
        // Java: this(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_ROW_PREFIX, DEFAULT_ROW_SUFFIX,
                // Java: DEFAULT_ROW_SEPARATOR, DEFAULT_COLUMN_SEPARATOR, CompositeFormat.getDefaultNumberFormat());
    }

    // Java: RealMatrixFormat(final NumberFormat format)
    pub fn RealMatrixFormat_1(&self, format: &dyn std::any::Any) {
        // Java: this(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_ROW_PREFIX, DEFAULT_ROW_SUFFIX,
                // Java: DEFAULT_ROW_SEPARATOR, DEFAULT_COLUMN_SEPARATOR, format);
    }

    // Java: RealMatrixFormat()
    pub fn RealMatrixFormat_2(&self) {
    }

    // Java: RealMatrixFormat()
    pub fn RealMatrixFormat_3(&self) {
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

    // Java: String getRowPrefix()
    pub fn getRowPrefix(&self) -> String {
        // Java: return rowPrefix;
        todo!();
    }

    // Java: String getRowSuffix()
    pub fn getRowSuffix(&self) -> String {
        // Java: return rowSuffix;
        todo!();
    }

    // Java: String getRowSeparator()
    pub fn getRowSeparator(&self) -> String {
        // Java: return rowSeparator;
        todo!();
    }

    // Java: String getColumnSeparator()
    pub fn getColumnSeparator(&self) -> String {
        // Java: return columnSeparator;
        todo!();
    }

    // Java: NumberFormat getFormat()
    pub fn getFormat(&self) -> Box<dyn std::any::Any> {
        // Java: return format;
        todo!();
    }

    // Java: static RealMatrixFormat getInstance()
    pub fn getInstance(&self) -> RealMatrixFormat {
        // Java: return getInstance(Locale.getDefault());
        todo!();
    }

    // Java: static RealMatrixFormat getInstance(final Locale locale)
    pub fn getInstance_1(&self, locale: &str) -> RealMatrixFormat {
        // Java: return new RealMatrixFormat(CompositeFormat.getDefaultNumberFormat(locale));
        todo!();
    }

    // Java: String format(RealMatrix m)
    pub fn format(&self, m: &dyn std::any::Any) -> String {
        // Java: return format(m, new StringBuffer(), new FieldPosition(0)).toString();
        todo!();
    }

    // Java: StringBuffer format()
    pub fn format_1(&self) -> String {
        todo!();
    }

    // Java: RealMatrix parse(String source)
    pub fn parse(&self, source: &str) -> Box<dyn std::any::Any> {
        // Java: final ParsePosition parsePosition = new ParsePosition(0);
        // Java: final RealMatrix result = parse(source, parsePosition);
        // Java: if (parsePosition.getIndex() == 0)
        {
            // Java: throw new MathParseException(source,
                                         // Java: parsePosition.getErrorIndex(),
                                         // Java: Array2DRowRealMatrix.class);
        }
        // Java: return result;
        todo!();
    }

    // Java: RealMatrix parse(String source, ParsePosition pos)
    pub fn parse_1(&self, source: &str, pos: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: int initialIndex = pos.getIndex();
        // Java: final String trimmedPrefix = prefix.trim();
        // Java: final String trimmedSuffix = suffix.trim();
        // Java: final String trimmedRowPrefix = rowPrefix.trim();
        // Java: final String trimmedRowSuffix = rowSuffix.trim();
        // Java: final String trimmedColumnSeparator = columnSeparator.trim();
        // Java: final String trimmedRowSeparator = rowSeparator.trim();
        // Java: // parse prefix
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        // Java: if (!CompositeFormat.parseFixedstring(source, trimmedPrefix, pos))
        {
            // Java: return null;
        }
        // Java: // parse components
        // Java: List<List<Number>> matrix = new ArrayList<List<Number>>();
        // Java: List<Number> rowComponents = new ArrayList<Number>();
        // Java: for (boolean loop = true; loop;)
        {
            // Java: if (!rowComponents.isEmpty())
            {
                // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
                // Java: if (!CompositeFormat.parseFixedstring(source, trimmedColumnSeparator, pos))
                {
                    // Java: if (trimmedRowSuffix.length() != 0 &&
                        // Java: !CompositeFormat.parseFixedstring(source, trimmedRowSuffix, pos))
                        {
                        // Java: return null;
                        }
                    // Java: else
                    {
                        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
                        // Java: if (CompositeFormat.parseFixedstring(source, trimmedRowSeparator, pos))
                        {
                            // Java: matrix.add(rowComponents);
                            // Java: rowComponents = new ArrayList<Number>();
                            // Java: continue;
                        }
                        // Java: else
                        {
                            // Java: loop = false;
                        }
                    }
                }
            }
            // Java: else
            {
                // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
                // Java: if (trimmedRowPrefix.length() != 0 &&
                    // Java: !CompositeFormat.parseFixedstring(source, trimmedRowPrefix, pos))
                    {
                    // Java: return null;
                    }
            }
            // Java: if (loop)
            {
                // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
                // Java: Number component = CompositeFormat.parseNumber(source, format, pos);
                // Java: if (component != null)
                {
                    // Java: rowComponents.add(component);
                }
                // Java: else
                {
                    // Java: if (rowComponents.isEmpty())
                    {
                        // Java: loop = false;
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
        }
        // Java: if (!rowComponents.isEmpty())
        {
            // Java: matrix.add(rowComponents);
        }
        // Java: // parse suffix
        // Java: CompositeFormat.parseAndIgnoreWhitespace(source, pos);
        // Java: if (!CompositeFormat.parseFixedstring(source, trimmedSuffix, pos))
        {
            // Java: return null;
        }
        // Java: // do not allow an empty matrix
        // Java: if (matrix.isEmpty())
        {
            // Java: pos.setIndex(initialIndex);
            // Java: return null;
        }
        // Java: // build vector
        // Java: double[][] data = new double[matrix.size()][];
        // Java: int row = 0;
        // Java: for (List<Number> rowList : matrix)
        {
            // Java: data[row] = new double[rowList.size()];
            // Java: for (int i = 0; i < rowList.size(); i++)
            {
                // Java: data[row][i] = rowList.get(i).doubleValue();
            }
            // Java: row++;
        }
        // Java: return MatrixUtils.createRealMatrix(data);
        todo!();
    }

}
