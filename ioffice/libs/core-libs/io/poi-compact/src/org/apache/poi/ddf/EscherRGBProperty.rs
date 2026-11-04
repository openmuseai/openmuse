use super::EscherSimpleProperty::EscherSimpleProperty;

// Java: public class EscherRGBProperty extends EscherSimpleProperty
pub struct EscherRGBProperty {
    pub(crate) base: EscherSimpleProperty,
}

// Java: public class EscherRGBProperty extends EscherSimpleProperty
impl EscherRGBProperty {
    // Java: EscherRGBProperty(short propertyNumber, int rgbColor)
    pub fn new(property_number: i16, rgb_color: i32) -> Self {
        // Java: super(propertyNumber, rgbColor);
        EscherRGBProperty {
            base: EscherSimpleProperty::new(property_number, rgb_color),
        }
    }

    // Java: int getRgbColor()
    pub fn getRgbColor(&self) -> i32 {
        // Java: return this.propertyValue;
        self.base.propertyValue
    }

    // Java: byte getRed()
    pub fn getRed(&self) -> i8 {
        // Java: return (byte)(this.propertyValue & 255);
        (self.base.propertyValue & 255) as i8
    }

    // Java: byte getGreen()
    pub fn getGreen(&self) -> i8 {
        // Java: return (byte)(this.propertyValue >> 8 & 255);
        ((self.base.propertyValue >> 8) & 255) as i8
    }

    // Java: byte getBlue()
    pub fn getBlue(&self) -> i8 {
        // Java: return (byte)(this.propertyValue >> 16 & 255);
        ((self.base.propertyValue >> 16) & 255) as i8
    }
}
