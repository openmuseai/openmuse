use super::EscherSimpleProperty::EscherSimpleProperty;

// Java field: int LINE_OF_STRAIGHT_SEGMENTS
pub const LINE_OF_STRAIGHT_SEGMENTS: i32 = 0;
// Java field: int CLOSED_POLYGON
pub const CLOSED_POLYGON: i32 = 1;
// Java field: int CURVES
pub const CURVES: i32 = 2;
// Java field: int CLOSED_CURVES
pub const CLOSED_CURVES: i32 = 3;
// Java field: int COMPLEX
pub const COMPLEX: i32 = 4;

// Java: public class EscherShapePathProperty extends EscherSimpleProperty
pub struct EscherShapePathProperty {
    pub(crate) base: EscherSimpleProperty,
}

// Java: public class EscherShapePathProperty extends EscherSimpleProperty
impl EscherShapePathProperty {
    // Java: EscherShapePathProperty(short propertyNumber, int shapePath)
    pub fn new(property_number: i16, shape_path: i32) -> Self {
        // Java: super(propertyNumber, false, false, shapePath);
        EscherShapePathProperty {
            base: EscherSimpleProperty::new_with_flags(property_number, false, false, shape_path),
        }
    }
}
