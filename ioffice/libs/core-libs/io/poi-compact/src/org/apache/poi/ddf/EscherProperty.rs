pub use super::EscherProperties::EscherProperties;

// Java: public class EscherProperty
#[derive(Clone)]
pub struct EscherProperty {
    // Java field: short _id
    pub(crate) _id: i16,
}

// Java: public class EscherProperty
impl EscherProperty {
    // Java: EscherProperty(short id)
    pub fn new(id: i16) -> Self {
        // Java: super();
        // Java: this._id = id;
        EscherProperty { _id: id }
    }

    // Java: EscherProperty(short propertyNumber, boolean isComplex, boolean isBlipId)
    pub fn new_with_flags(property_number: i16, is_complex: bool, is_blip_id: bool) -> Self {
        // Java: super();
        // Java: this._id = (short)(propertyNumber + (isComplex ? '耀' : 0) + (isBlipId ? 16384 : 0));
        let complex_flag: i16 = if is_complex { 0x8000u16 as i16 } else { 0 };
        let blip_flag: i16 = if is_blip_id { 16384 } else { 0 };
        EscherProperty {
            _id: property_number.wrapping_add(complex_flag).wrapping_add(blip_flag),
        }
    }

    // Java: short getId()
    pub fn getId(&self) -> i16 {
        // Java: return this._id;
        self._id
    }

    // Java: short getPropertyNumber()
    pub fn getPropertyNumber(&self) -> i16 {
        // Java: return (short)(this._id & 16383);
        self._id & 16383
    }

    // Java: boolean isComplex()
    pub fn isComplex(&self) -> bool {
        // Java: return (this._id & Short.MIN_VALUE) != 0;
        (self._id & i16::MIN) != 0
    }

    // Java: boolean isBlipId()
    pub fn isBlipId(&self) -> bool {
        // Java: return (this._id & 16384) != 0;
        (self._id & 16384) != 0
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return EscherProperties.getPropertyName(this._id);
        EscherProperties::getPropertyName(self._id)
    }

    // Java: int getPropertySize()
    pub fn getPropertySize(&self) -> i32 {
        // Java: return 6;
        6
    }

    // Java: abstract int serializeSimplePart(byte[] var1, int var2)
    pub fn serializeSimplePart(&self, _data: &mut [u8], _offset: i32) -> i32 {
        0
    }

    // Java: abstract int serializeComplexPart(byte[] var1, int var2)
    pub fn serializeComplexPart(&self, _data: &mut [u8], _offset: i32) -> i32 {
        0
    }
}
