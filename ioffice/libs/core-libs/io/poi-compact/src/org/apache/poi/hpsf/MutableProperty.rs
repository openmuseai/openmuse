#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.OutputStream
use super::Property::{Property, PropertyValue};

// Java: public class MutableProperty extends Property
pub struct MutableProperty {
    pub(crate) base: Property,
}

// Java: public class MutableProperty extends Property
impl MutableProperty {
    // Java: MutableProperty()
    pub fn new() -> Self {
        // Java: super();
        Self { base: Property::new_default() }
    }

    // Java: MutableProperty(Property p)
    pub fn new_from_property(p: &Property) -> Self {
        // Java: super();
        let mut mp = Self { base: Property::new_default() };
        // Java: this.setID(p.getID());
        mp.setID(p.getID());
        // Java: this.setType(p.getType());
        mp.setType(p.getType());
        // Java: this.setValue(p.getValue());
        mp.base.value = p.getValue().clone();
        mp
    }

    // Java: void setID(long id)
    pub fn setID(&mut self, id: i64) {
        // Java: this.id = id;
        self.base.id = id;
    }

    // Java: void setType(long type)
    pub fn setType(&mut self, type_: i64) {
        // Java: this.type = type;
        self.base.type_ = type_;
    }

    // Java: void setValue(Object value)
    pub fn setValue(&mut self, value: PropertyValue) {
        // Java: this.value = value;
        self.base.value = value;
    }

    pub fn getID(&self) -> i64 { self.base.getID() }
    pub fn getType(&self) -> i64 { self.base.getType() }
    pub fn getValue(&self) -> &PropertyValue { self.base.getValue() }

    // Java: int write(OutputStream out, int codepage)
    pub fn write(&self, out: &mut Vec<u8>, codepage: i32) -> i32 {
        // Java: int length = 0;
        let mut length: i32 = 0;
        // Java: long variantType = this.getType();
        let mut variant_type = self.getType();
        // Java: if (codepage == 1200 && variantType == 30L)
        if codepage == 1200 && variant_type == 30
        {
            // Java: variantType = 31L;
            variant_type = 31;
        }
        // Java: length += TypeWriter.writeUIntToStream(out, variantType);
        out.extend_from_slice(&(variant_type as u32).to_le_bytes());
        length += 4;
        // Java: length += VariantSupport.write(out, variantType, this.getValue(), codepage);
        // Java: return length;
        length
    }
}
