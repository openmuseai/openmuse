use super::EscherSimpleProperty::EscherSimpleProperty;

// Java: public class EscherBoolProperty extends EscherSimpleProperty
pub struct EscherBoolProperty {
    pub(crate) base: EscherSimpleProperty,
}

// Java: public class EscherBoolProperty extends EscherSimpleProperty
impl EscherBoolProperty {
    // Java: EscherBoolProperty(short propertyNumber, int value)
    pub fn new(property_number: i16, value: i32) -> Self {
        // Java: super(propertyNumber, value);
        EscherBoolProperty {
            base: EscherSimpleProperty::new(property_number, value),
        }
    }

    // Java: boolean isTrue()
    pub fn isTrue(&self) -> bool {
        // Java: return this.propertyValue != 0;
        self.base.propertyValue != 0
    }

    // Java: boolean isFalse()
    pub fn isFalse(&self) -> bool {
        // Java: return this.propertyValue == 0;
        self.base.propertyValue == 0
    }
}
