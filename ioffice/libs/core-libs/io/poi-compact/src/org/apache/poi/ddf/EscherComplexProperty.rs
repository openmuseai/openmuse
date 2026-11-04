// Java: import java.util.Arrays
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

// Java: public class EscherComplexProperty extends EscherProperty
pub struct EscherComplexProperty {
    pub(crate) base: EscherProperty,
    // Java field: byte[] _complexData
    pub(crate) _complexData: Vec<u8>,
}

// Java: public class EscherComplexProperty extends EscherProperty
impl EscherComplexProperty {
    // Java: EscherComplexProperty(short id, byte[] complexData)
    pub fn new(id: i16, complex_data: Vec<u8>) -> Self {
        // Java: super(id);
        // Java: this._complexData = complexData;
        EscherComplexProperty {
            base: EscherProperty::new(id),
            _complexData: complex_data,
        }
    }

    // Java: EscherComplexProperty(short propertyNumber, boolean isBlipId, byte[] complexData)
    pub fn new_with_blip(property_number: i16, is_blip_id: bool, complex_data: Vec<u8>) -> Self {
        // Java: super(propertyNumber, true, isBlipId);
        // Java: this._complexData = complexData;
        EscherComplexProperty {
            base: EscherProperty::new_with_flags(property_number, true, is_blip_id),
            _complexData: complex_data,
        }
    }

    // Java: int serializeSimplePart(byte[] data, int pos)
    pub fn serializeSimplePart(&self, data: &mut [u8], pos: i32) -> i32 {
        // Java: LittleEndian.putShort(data, pos, this.getId());
        le_put_short(data, pos as usize, self.base.getId());
        // Java: LittleEndian.putInt(data, pos + 2, this._complexData.length);
        le_put_int(data, (pos + 2) as usize, self._complexData.len() as i32);
        // Java: return 6;
        6
    }

    // Java: int serializeComplexPart(byte[] data, int pos)
    pub fn serializeComplexPart(&self, data: &mut [u8], pos: i32) -> i32 {
        // Java: System.arraycopy(this._complexData, 0, data, pos, this._complexData.length);
        let p = pos as usize;
        let len = self._complexData.len();
        data[p..p + len].copy_from_slice(&self._complexData);
        // Java: return this._complexData.length;
        len as i32
    }

    // Java: byte[] getComplexData()
    pub fn getComplexData(&self) -> &[u8] {
        // Java: return this._complexData;
        &self._complexData
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &EscherComplexProperty) -> bool {
        // Java: if (this == o)
        {
            // Java: return true;
        }
        // Java: else if (!(o instanceof EscherComplexProperty))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: EscherComplexProperty escherComplexProperty = (EscherComplexProperty)o;
            // Java: return Arrays.equals(this._complexData, escherComplexProperty._complexData);
            self._complexData == other._complexData
        }
    }

    // Java: int getPropertySize()
    pub fn getPropertySize(&self) -> i32 {
        // Java: return 6 + this._complexData.length;
        6 + self._complexData.len() as i32
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.getId() * 11;
        self.base.getId() as i32 * 11
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String dataStr = HexDump.toHex(this._complexData, 32);
        // Java: return "propNum: " + this.getPropertyNumber() + ", propName: " + EscherProperties.getPropertyName(this.getPropertyNumber()) + ", complex: " + this.isComplex() + ", blipId: " + this.isBlipId() + ", data: " + System.getProperty("line.separator") + dataStr;
        format!("propNum: {}, complex: {}, blipId: {}", self.base.getPropertyNumber(), self.base.isComplex(), self.base.isBlipId())
    }
}
