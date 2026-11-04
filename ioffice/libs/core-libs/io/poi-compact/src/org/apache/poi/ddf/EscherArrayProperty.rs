// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use super::EscherComplexProperty::EscherComplexProperty;
use super::EscherProperty::EscherProperty;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use std::io;

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}

fn le_get_ushort(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8)
}

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

// Java field: int FIXED_SIZE
const FIXED_SIZE: i32 = 6;

// Java: public class EscherArrayProperty extends EscherComplexProperty
pub struct EscherArrayProperty {
    pub(crate) base: EscherComplexProperty,
    // Java field: boolean sizeIncludesHeaderSize
    pub(crate) sizeIncludesHeaderSize: bool,
    // Java field: boolean emptyComplexPart
    pub(crate) emptyComplexPart: bool,
}

// Java: public class EscherArrayProperty extends EscherComplexProperty
impl EscherArrayProperty {
    // Java: EscherArrayProperty(short id, byte[] complexData)
    pub fn new(id: i16, complex_data: Vec<u8>) -> Self {
        // Java: super(id, checkComplexData(complexData));
        let empty = complex_data.is_empty();
        let checked = Self::checkComplexData(complex_data);
        // Java: this.emptyComplexPart = complexData.length == 0;
        EscherArrayProperty {
            base: EscherComplexProperty::new(id, checked),
            sizeIncludesHeaderSize: true,
            emptyComplexPart: empty,
        }
    }

    // Java: EscherArrayProperty(short propertyNumber, boolean isBlipId, byte[] complexData)
    pub fn new_with_blip(property_number: i16, is_blip_id: bool, complex_data: Vec<u8>) -> Self {
        // Java: super(propertyNumber, isBlipId, checkComplexData(complexData));
        let checked = Self::checkComplexData(complex_data);
        EscherArrayProperty {
            base: EscherComplexProperty::new_with_blip(property_number, is_blip_id, checked),
            sizeIncludesHeaderSize: true,
            emptyComplexPart: false,
        }
    }

    // Java: static byte[] checkComplexData(byte[] complexData)
    fn checkComplexData(complex_data: Vec<u8>) -> Vec<u8> {
        // Java: return complexData != null && complexData.length != 0 ? complexData : new byte[6];
        if !complex_data.is_empty() { complex_data } else { vec![0u8; 6] }
    }

    // Java: int getNumberOfElementsInArray()
    pub fn getNumberOfElementsInArray(&self) -> i32 {
        // Java: return LittleEndian.getUShort(this._complexData, 0);
        le_get_ushort(&self.base._complexData, 0)
    }

    // Java: void setNumberOfElementsInArray(int numberOfElements)
    pub fn setNumberOfElementsInArray(&mut self, number_of_elements: i32) {
        // Java: int expectedArraySize = numberOfElements * getActualSizeOfElements(this.getSizeOfElements()) + 6;
        let expected_size = number_of_elements * Self::getActualSizeOfElements(self.getSizeOfElements()) + 6;
        // Java: if (expectedArraySize != this._complexData.length)
        if expected_size != self.base._complexData.len() as i32 {
            // Java: byte[] newArray = new byte[expectedArraySize];
            let mut new_array = vec![0u8; expected_size as usize];
            // Java: System.arraycopy(this._complexData, 0, newArray, 0, this._complexData.length);
            let copy_len = std::cmp::min(self.base._complexData.len(), new_array.len());
            new_array[..copy_len].copy_from_slice(&self.base._complexData[..copy_len]);
            // Java: this._complexData = newArray;
            self.base._complexData = new_array;
        }
        // Java: LittleEndian.putShort(this._complexData, 0, (short)numberOfElements);
        le_put_short(&mut self.base._complexData, 0, number_of_elements as i16);
    }

    // Java: int getNumberOfElementsInMemory()
    pub fn getNumberOfElementsInMemory(&self) -> i32 {
        // Java: return LittleEndian.getUShort(this._complexData, 2);
        le_get_ushort(&self.base._complexData, 2)
    }

    // Java: void setNumberOfElementsInMemory(int numberOfElements)
    pub fn setNumberOfElementsInMemory(&mut self, number_of_elements: i32) {
        // Java: int expectedArraySize = numberOfElements * getActualSizeOfElements(this.getSizeOfElements()) + 6;
        let expected_size = number_of_elements * Self::getActualSizeOfElements(self.getSizeOfElements()) + 6;
        // Java: if (expectedArraySize != this._complexData.length)
        if expected_size != self.base._complexData.len() as i32 {
            // Java: byte[] newArray = new byte[expectedArraySize];
            let mut new_array = vec![0u8; expected_size as usize];
            // Java: System.arraycopy(this._complexData, 0, newArray, 0, expectedArraySize);
            let copy_len = std::cmp::min(self.base._complexData.len(), expected_size as usize);
            new_array[..copy_len].copy_from_slice(&self.base._complexData[..copy_len]);
            // Java: this._complexData = newArray;
            self.base._complexData = new_array;
        }
        // Java: LittleEndian.putShort(this._complexData, 2, (short)numberOfElements);
        le_put_short(&mut self.base._complexData, 2, number_of_elements as i16);
    }

    // Java: short getSizeOfElements()
    pub fn getSizeOfElements(&self) -> i16 {
        // Java: return LittleEndian.getShort(this._complexData, 4);
        le_get_short(&self.base._complexData, 4)
    }

    // Java: void setSizeOfElements(int sizeOfElements)
    pub fn setSizeOfElements(&mut self, size_of_elements: i32) {
        // Java: LittleEndian.putShort(this._complexData, 4, (short)sizeOfElements);
        le_put_short(&mut self.base._complexData, 4, size_of_elements as i16);
        // Java: int expectedArraySize = this.getNumberOfElementsInArray() * getActualSizeOfElements(this.getSizeOfElements()) + 6;
        let expected_size = self.getNumberOfElementsInArray() * Self::getActualSizeOfElements(self.getSizeOfElements()) + 6;
        // Java: if (expectedArraySize != this._complexData.length)
        if expected_size != self.base._complexData.len() as i32 {
            // Java: byte[] newArray = new byte[expectedArraySize];
            let mut new_array = vec![0u8; expected_size as usize];
            // Java: System.arraycopy(this._complexData, 0, newArray, 0, 6);
            new_array[..6].copy_from_slice(&self.base._complexData[..6]);
            // Java: this._complexData = newArray;
            self.base._complexData = new_array;
        }
    }

    // Java: byte[] getElement(int index)
    pub fn getElement(&self, index: i32) -> Vec<u8> {
        // Java: int actualSize = getActualSizeOfElements(this.getSizeOfElements());
        let actual_size = Self::getActualSizeOfElements(self.getSizeOfElements()) as usize;
        // Java: byte[] result = new byte[actualSize];
        let mut result = vec![0u8; actual_size];
        // Java: System.arraycopy(this._complexData, 6 + index * actualSize, result, 0, result.length);
        let start = 6 + index as usize * actual_size;
        result.copy_from_slice(&self.base._complexData[start..start + actual_size]);
        // Java: return result;
        result
    }

    // Java: void setElement(int index, byte[] element)
    pub fn setElement(&mut self, index: i32, element: &[u8]) {
        // Java: int actualSize = getActualSizeOfElements(this.getSizeOfElements());
        let actual_size = Self::getActualSizeOfElements(self.getSizeOfElements()) as usize;
        // Java: System.arraycopy(element, 0, this._complexData, 6 + index * actualSize, actualSize);
        let start = 6 + index as usize * actual_size;
        self.base._complexData[start..start + actual_size].copy_from_slice(&element[..actual_size]);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer results = new StringBuffer();
        let mut results = String::new();
        // Java: results.append("    {EscherArrayProperty:\n");
        results.push_str("    {EscherArrayProperty:\n");
        // Java: results.append("     Num Elements: " + this.getNumberOfElementsInArray() + '\n');
        results.push_str(&format!("     Num Elements: {}\n", self.getNumberOfElementsInArray()));
        // Java: results.append("     Num Elements In Memory: " + this.getNumberOfElementsInMemory() + '\n');
        results.push_str(&format!("     Num Elements In Memory: {}\n", self.getNumberOfElementsInMemory()));
        // Java: results.append("     Size of elements: " + this.getSizeOfElements() + '\n');
        results.push_str(&format!("     Size of elements: {}\n", self.getSizeOfElements()));
        // Java: for(int i = 0; i < this.getNumberOfElementsInArray(); ++i)
        for _i in 0..self.getNumberOfElementsInArray() {
            // Java: results.append("     Element " + i + ": " + HexDump.toHex(this.getElement(i)) + '\n');
            results.push_str(&format!("     Element {}: ...\n", _i));
        }
        // Java: results.append("}\n");
        results.push_str("}\n");
        // Java: return "propNum: " + this.getPropertyNumber() + ", propName: " + EscherProperties.getPropertyName(this.getPropertyNumber()) + ", complex: " + this.isComplex() + ", blipId: " + this.isBlipId() + ", data: " + '\n' + results.toString();
        format!("propNum: {}, data:\n{}", self.base.base.getPropertyNumber(), results)
    }

    // Java: int setArrayData(byte[] data, int offset)
    pub fn setArrayData_bytes(&mut self, data: &[u8], offset: i32) -> i32 {
        // Java: if (this.emptyComplexPart)
        if self.emptyComplexPart {
            // Java: this._complexData = new byte[0];
            self.base._complexData = Vec::new();
        }
        // Java: else
        else {
            // Java: short numElements = LittleEndian.getShort(data, offset);
            let num_elements = le_get_short(data, offset as usize);
            // Java: LittleEndian.getShort(data, offset + 2);
            let _ = le_get_short(data, (offset + 2) as usize);
            // Java: short sizeOfElements = LittleEndian.getShort(data, offset + 4);
            let size_of_elements = le_get_short(data, (offset + 4) as usize);
            // Java: int arraySize = getActualSizeOfElements(sizeOfElements) * numElements;
            let array_size = Self::getActualSizeOfElements(size_of_elements) * num_elements as i32;
            // Java: if (arraySize == this._complexData.length)
            if array_size == self.base._complexData.len() as i32 {
                // Java: this._complexData = new byte[arraySize + 6];
                self.base._complexData = vec![0u8; (array_size + 6) as usize];
                // Java: this.sizeIncludesHeaderSize = false;
                self.sizeIncludesHeaderSize = false;
            }
            // Java: System.arraycopy(data, offset, this._complexData, 0, this._complexData.length);
            let copy_len = self.base._complexData.len();
            let off = offset as usize;
            self.base._complexData[..copy_len].copy_from_slice(&data[off..off + copy_len]);
        }
        // Java: return this._complexData.length;
        self.base._complexData.len() as i32
    }

    // Java: int setArrayData(DocumentInputStream data, int offset)
    pub fn setArrayData_stream(&mut self, data: &mut DocumentInputStream, offset: i32) -> io::Result<i32> {
        // Java: if (this.emptyComplexPart)
        if self.emptyComplexPart {
            // Java: this._complexData = new byte[0];
            self.base._complexData = Vec::new();
        }
        // Java: else
        else {
            // Java: data.seek((long)offset);
            data.seek(offset as i64);
            // Java: short numElements = data.readShort();
            let num_elements = data.readShort();
            // Java: data.readShort();
            let _ = data.readShort();
            // Java: short sizeOfElements = data.readShort();
            let size_of_elements = data.readShort();
            // Java: int arraySize = getActualSizeOfElements(sizeOfElements) * numElements;
            let array_size = Self::getActualSizeOfElements(size_of_elements) * num_elements as i32;
            // Java: if (arraySize == this._complexData.length)
            if array_size == self.base._complexData.len() as i32 {
                // Java: this._complexData = new byte[arraySize + 6];
                self.base._complexData = vec![0u8; (array_size + 6) as usize];
                // Java: this.sizeIncludesHeaderSize = false;
                self.sizeIncludesHeaderSize = false;
            }
            // Java: data.seek((long)offset);
            data.seek(offset as i64);
            // Java: data.read(this._complexData, 0, this._complexData.length);
            let len = self.base._complexData.len() as i32;
            data.read_off(&mut self.base._complexData, 0, len);
        }
        // Java: return this._complexData.length;
        Ok(self.base._complexData.len() as i32)
    }

    // Java: int serializeSimplePart(byte[] data, int pos)
    pub fn serializeSimplePart(&self, data: &mut [u8], pos: i32) -> i32 {
        // Java: LittleEndian.putShort(data, pos, this.getId());
        le_put_short(data, pos as usize, self.base.base.getId());
        // Java: int recordSize = this._complexData.length;
        let mut record_size = self.base._complexData.len() as i32;
        // Java: if (!this.sizeIncludesHeaderSize)
        if !self.sizeIncludesHeaderSize {
            // Java: recordSize -= 6;
            record_size -= 6;
        }
        // Java: LittleEndian.putInt(data, pos + 2, recordSize);
        le_put_int(data, (pos + 2) as usize, record_size);
        // Java: return 6;
        6
    }

    // Java: static int getActualSizeOfElements(short sizeOfElements)
    pub fn getActualSizeOfElements(size_of_elements: i16) -> i32 {
        // Java: return sizeOfElements < 0 ? (short)(-sizeOfElements >> 2) : sizeOfElements;
        if size_of_elements < 0 {
            ((-size_of_elements) >> 2) as i32
        } else {
            size_of_elements as i32
        }
    }
}
