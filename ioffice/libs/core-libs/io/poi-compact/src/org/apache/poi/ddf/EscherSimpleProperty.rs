// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use super::EscherProperty::EscherProperty;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java: public class EscherSimpleProperty extends EscherProperty
pub struct EscherSimpleProperty {
    pub(crate) base: EscherProperty,
    // Java field: int propertyValue
    pub(crate) propertyValue: i32,
}

// Java: public class EscherSimpleProperty extends EscherProperty
impl EscherSimpleProperty {
    // Java: EscherSimpleProperty(short id, int propertyValue)
    pub fn new(id: i16, property_value: i32) -> Self {
        // Java: super(id);
        // Java: this.propertyValue = propertyValue;
        EscherSimpleProperty {
            base: EscherProperty::new(id),
            propertyValue: property_value,
        }
    }

    // Java: EscherSimpleProperty(short propertyNumber, boolean isComplex, boolean isBlipId, int propertyValue)
    pub fn new_with_flags(property_number: i16, is_complex: bool, is_blip_id: bool, property_value: i32) -> Self {
        // Java: super(propertyNumber, isComplex, isBlipId);
        // Java: this.propertyValue = propertyValue;
        EscherSimpleProperty {
            base: EscherProperty::new_with_flags(property_number, is_complex, is_blip_id),
            propertyValue: property_value,
        }
    }

    // Java: int serializeSimplePart(byte[] data, int offset)
    pub fn serializeSimplePart(&self, data: &mut [u8], offset: i32) -> i32 {
        // Java: LittleEndian.putShort(data, offset, this.getId());
        le_put_short(data, offset as usize, self.base.getId());
        // Java: LittleEndian.putInt(data, offset + 2, this.propertyValue);
        le_put_int(data, (offset + 2) as usize, self.propertyValue);
        // Java: return 6;
        6
    }

    // Java: int serializeComplexPart(byte[] data, int pos)
    pub fn serializeComplexPart(&self, _data: &mut [u8], _pos: i32) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getPropertyValue()
    pub fn getPropertyValue(&self) -> i32 {
        // Java: return this.propertyValue;
        self.propertyValue
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &EscherSimpleProperty) -> bool {
        // Java: if (this == o)
        {
            // Java: return true;
        }
        // Java: else if (!(o instanceof EscherSimpleProperty))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: EscherSimpleProperty escherSimpleProperty = (EscherSimpleProperty)o;
            // Java: if (this.propertyValue != escherSimpleProperty.propertyValue)
            if self.propertyValue != other.propertyValue {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: return this.getId() == escherSimpleProperty.getId();
                return self.base.getId() == other.base.getId();
            }
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.propertyValue;
        self.propertyValue
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "propNum: " + this.getPropertyNumber() + ", RAW: 0x" + HexDump.toHex(this.getId()) + ", propName: " + EscherProperties.getPropertyName(this.getPropertyNumber()) + ", complex: " + this.isComplex() + ", blipId: " + this.isBlipId() + ", value: " + this.propertyValue + " (0x" + HexDump.toHex(this.propertyValue) + ")";
        format!("propNum: {}, RAW: 0x{:04X}, complex: {}, blipId: {}, value: {} (0x{:08X})",
            self.base.getPropertyNumber(),
            self.base.getId(),
            self.base.isComplex(),
            self.base.isBlipId(),
            self.propertyValue,
            self.propertyValue)
    }
}
