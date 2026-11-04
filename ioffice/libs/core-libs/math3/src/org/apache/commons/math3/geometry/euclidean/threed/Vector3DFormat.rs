#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.text.ParsePosition
// Java: import java.util.Locale
// Java: import org.apache.commons.math3.exception.MathParseException
use crate::org::apache::commons::math3::exception::MathParseException;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;
// Java: import org.apache.commons.math3.geometry.VectorFormat
use crate::org::apache::commons::math3::geometry::VectorFormat;
// Java: import org.apache.commons.math3.util.CompositeFormat
use crate::org::apache::commons::math3::util::CompositeFormat;

use super::Vector3D::Vector3D;

// Java: public class Vector3DFormat extends VectorFormat
pub struct Vector3DFormat {
}

// Java: public class Vector3DFormat extends VectorFormat
impl Vector3DFormat {
    // Java: Vector3DFormat()
    pub fn new() -> Self {
        // Java: super(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_SEPARATOR,
        // Java: CompositeFormat.getDefaultNumberFormat());
        Vector3DFormat {}
    }

    // Java: Vector3DFormat(final NumberFormat format)
    pub fn new_with_format(_format: &dyn std::any::Any) -> Self {
        // Java: super(DEFAULT_PREFIX, DEFAULT_SUFFIX, DEFAULT_SEPARATOR, format);
        Vector3DFormat {}
    }

    // Java: Vector3DFormat()
    pub fn new_default_1() -> Self {
        Vector3DFormat {}
    }

    // Java: Vector3DFormat()
    pub fn new_default_2() -> Self {
        Vector3DFormat {}
    }

    // Java: static Vector3DFormat getInstance()
    pub fn getInstance() -> Vector3DFormat {
        // Java: return getInstance(Locale.getDefault());
        Vector3DFormat::new()
    }

    // Java: static Vector3DFormat getInstance(final Locale locale)
    pub fn getInstance_locale(_locale: &dyn std::any::Any) -> Vector3DFormat {
        // Java: return new Vector3DFormat(CompositeFormat.getDefaultNumberFormat(locale));
        Vector3DFormat::new()
    }

    // Java: StringBuffer format()
    pub fn format(&self, v: &Vector3D) -> String {
        format!("{{{}, {}, {}}}", v.getX(), v.getY(), v.getZ())
    }

    // Java: Vector3D parse(final String source)
    pub fn parse(&self, _source: &str) -> Vector3D {
        // Java: ParsePosition parsePosition = new ParsePosition(0);
        // Java: Vector3D result = parse(source, parsePosition);
        // Java: if (parsePosition.getIndex() == 0)
        // Java: throw new MathParseException(source,
        // Java: parsePosition.getErrorIndex(),
        // Java: Vector3D.class);
        // Java: return result;
        todo!()
    }

    // Java: Vector3D parse(final String source, final ParsePosition pos)
    pub fn parse_with_pos(&self, _source: &str, _pos: &dyn std::any::Any) -> Option<Vector3D> {
        // Java: final double[] coordinates = parseCoordinates(3, source, pos);
        // Java: if (coordinates == null)
        // Java: return null;
        // Java: return new Vector3D(coordinates[0], coordinates[1], coordinates[2]);
        todo!()
    }

}
