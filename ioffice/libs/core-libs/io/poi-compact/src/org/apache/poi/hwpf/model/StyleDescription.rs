// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

use super::UPX::UPX;
use super::HDFType::HDFType;

struct BitField { mask: i32 }
impl BitField {
    fn new(mask: i32) -> Self { BitField { mask } }
    fn getValue(&self, holder: i16) -> i32 {
        let shift = self.mask.trailing_zeros();
        ((holder as i32) & self.mask) >> shift
    }
    fn setShortValue(&self, holder: i16, value: i16) -> i16 {
        let shift = self.mask.trailing_zeros();
        let cleared = (holder as i32) & !self.mask;
        (cleared | (((value as i32) << shift) & self.mask)) as i16
    }
    fn setBoolean(&self, holder: i16, flag: bool) -> i32 {
        if flag { (holder as i32) | self.mask } else { (holder as i32) & !self.mask }
    }
}

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}
fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

// Java field: int NIL_STI
pub(crate) const NIL_STI: i32 = -1;
// Java field: int NIL_STYLE
pub(crate) const NIL_STYLE: i32 = 4095;
// Java field: int USER_STYLE
pub(crate) const USER_STYLE: i32 = 4094;
// Java field: int PARAGRAPH_STYLE
pub(crate) const PARAGRAPH_STYLE: i32 = 1;
// Java field: int CHARACTER_STYLE
pub(crate) const CHARACTER_STYLE: i32 = 2;
// Java field: int TABLE_STYLE
pub(crate) const TABLE_STYLE: i32 = 3;
// Java field: int NUMBERING_STYLE
pub(crate) const NUMBERING_STYLE: i32 = 4;

// Java field: BitField _sti
fn bf_sti() -> BitField { BitField::new(4095) }
// Java field: BitField _fInvalHeight
fn bf_fInvalHeight() -> BitField { BitField::new(8192) }
// Java field: BitField _fHasUpe
fn bf_fHasUpe() -> BitField { BitField::new(16384) }
// Java field: BitField _styleTypeCode
fn bf_styleTypeCode() -> BitField { BitField::new(15) }
// Java field: BitField _baseStyle
fn bf_baseStyle() -> BitField { BitField::new(65520) }
// Java field: BitField _numUPX
fn bf_numUPX() -> BitField { BitField::new(15) }
// Java field: BitField _nextStyle
fn bf_nextStyle() -> BitField { BitField::new(65520) }
// Java field: BitField _linkStyle
fn bf_linkStyle() -> BitField { BitField::new(4095) }

// Java: public class StyleDescription implements HDFType
pub struct StyleDescription {
    // Java field: int _istd
    _istd: i32,
    // Java field: int _baseLength
    _baseLength: i32,
    // Java field: short _infoShort
    _infoShort: i16,
    // Java field: short _infoShort2
    _infoShort2: i16,
    // Java field: short _infoShort3
    _infoShort3: i16,
    // Java field: short _bchUpe
    _bchUpe: i16,
    // Java field: short _infoShort4
    _infoShort4: i16,
    // Java field: short _infoShort5
    _infoShort5: i16,
    // Java field: UPX[] _upxs
    pub _upxs: Vec<UPX>,
    // Java field: String _name
    pub _name: Option<String>,
}

// Java: public class StyleDescription implements HDFType
impl StyleDescription {
    // Java: StyleDescription()
    pub fn new() -> Self {
        // Java: super();
        StyleDescription {
            _istd: 0, _baseLength: 0, _infoShort: 0, _infoShort2: 0,
            _infoShort3: 0, _bchUpe: 0, _infoShort4: 0, _infoShort5: 0,
            _upxs: Vec::new(), _name: None,
        }
    }

    // Java: StyleDescription(int baseLength)
    pub fn new_with_base_length(base_length: i32) -> Self {
        // Java: super();
        // Java: this._baseLength = baseLength;
        StyleDescription {
            _istd: 0, _baseLength: base_length, _infoShort: 0, _infoShort2: 0,
            _infoShort3: 0, _bchUpe: 0, _infoShort4: 0, _infoShort5: 0,
            _upxs: Vec::new(), _name: None,
        }
    }

    // Java: StyleDescription(DocumentInputStream std, int baseLength, int offset, boolean word9)
    pub fn new_from_stream(stream: &mut DocumentInputStream, base_length: i32, offset: i32, word9: bool) -> Self {
        // Java: super();
        // Java: this._baseLength = baseLength;
        // Java: int nameStart = offset + baseLength;
        let name_start = offset + base_length;
        // Java: std.seek((long)offset);
        stream.seek(offset as i64);
        // Java: byte[] tmp = new byte[12];
        let mut tmp = [0u8; 12];
        // Java: std.readFully(tmp);
        stream.read(&mut tmp);
        // Java: int offs = 0;
        let mut offs: usize = 0;
        // Java: this._infoShort = LittleEndian.getShort(tmp, offs);
        let _infoShort = le_get_short(&tmp, offs);
        // Java: offs += 2;
        offs += 2;
        // Java: this._infoShort2 = LittleEndian.getShort(tmp, offs);
        let _infoShort2 = le_get_short(&tmp, offs);
        // Java: offs += 2;
        offs += 2;
        // Java: this._infoShort3 = LittleEndian.getShort(tmp, offs);
        let _infoShort3 = le_get_short(&tmp, offs);
        // Java: offs += 2;
        offs += 2;
        // Java: this._bchUpe = LittleEndian.getShort(tmp, offs);
        let _bchUpe = le_get_short(&tmp, offs);
        // Java: offs += 2;
        offs += 2;
        // Java: this._infoShort4 = LittleEndian.getShort(tmp, offs);
        let _infoShort4 = le_get_short(&tmp, offs);
        // Java: offs += 2;
        offs += 2;
        // Java: this._infoShort5 = LittleEndian.getShort(tmp, offs);
        let _infoShort5 = le_get_short(&tmp, offs);
        // Java: offs += 2;

        // Java: std.seek((long)nameStart);
        stream.seek(name_start as i64);
        // Java: int nameBytes = 0;
        let name_bytes: i32;
        // Java: int grupxStart;
        let grupx_start: i32;
        // Java: if (word9)
        if word9 {
            // Java: nameBytes = std.readUShort() << 1;
            name_bytes = (stream.readUShort() as i32) << 1;
            // Java: grupxStart = nameStart + nameBytes + 4;
            grupx_start = name_start + name_bytes + 4;
        }
        // Java: else
        else {
            // Java: nameBytes = std.readByte();
            name_bytes = stream.readByte() as i32;
            // Java: grupxStart = nameStart + nameBytes + 1;
            grupx_start = name_start + name_bytes + 1;
        }

        let mut _name: Option<String> = None;
        // Java: if (nameBytes != 0)
        if name_bytes != 0 {
            // Java: byte[] bytes = new byte[nameBytes];
            let mut bytes = vec![0u8; name_bytes as usize];
            // Java: std.read(bytes, 0, nameBytes);
            stream.read(&mut bytes);
            // Java: int n = nameBytes >> 1;
            let n = (name_bytes >> 1) as usize;
            // Java: char[] chars = new char[n];
            let mut chars: Vec<u16> = Vec::with_capacity(n);
            // Java: for(int i = 0; i < n; ++i)
            for i in 0..n {
                // Java: chars[i] = (char)((bytes[i * 2 + 1] & 255) << 8 | bytes[i * 2] & 255);
                let ch = ((bytes[i * 2 + 1] as u16) << 8) | (bytes[i * 2] as u16);
                chars.push(ch);
            }
            // Java: this._name = new String(chars);
            _name = Some(String::from_utf16_lossy(&chars));
        }

        // Java: int numUPX = _numUPX.getValue(this._infoShort3);
        let num_upx = bf_numUPX().getValue(_infoShort3);
        // Java: this._upxs = new UPX[numUPX];
        let mut _upxs: Vec<UPX> = Vec::with_capacity(num_upx as usize);
        // Java: std.seek((long)grupxStart);
        stream.seek(grupx_start as i64);
        // Java: for(int x = 0; x < numUPX; ++x)
        for _x in 0..num_upx {
            // Java: int upxSize = std.readUShort();
            let upx_size = stream.readUShort() as i32;
            // Java: byte[] upx = new byte[upxSize];
            let mut upx = vec![0u8; upx_size as usize];
            // Java: std.read(upx, 0, upxSize);
            stream.read(&mut upx);
            // Java: this._upxs[x] = new UPX(upx);
            _upxs.push(UPX::new(upx));
            // Java: if (upxSize % 2 != 0)
            if upx_size % 2 != 0 {
                // Java: std.readByte();
                stream.readByte();
            }
        }

        StyleDescription {
            _istd: 0, _baseLength: base_length, _infoShort, _infoShort2,
            _infoShort3, _bchUpe, _infoShort4, _infoShort5,
            _upxs, _name,
        }
    }

    // Java: int getId()
    pub fn getId(&self) -> i32 {
        // Java: return _sti.getValue(this._infoShort);
        bf_sti().getValue(self._infoShort)
    }

    // Java: void setId(int id)
    pub fn setId(&mut self, id: i32) {
        // Java: this._infoShort = _sti.setShortValue(this._infoShort, (short)id);
        self._infoShort = bf_sti().setShortValue(self._infoShort, id as i16);
    }

    // Java: int getBaseStyle()
    pub fn getBaseStyle(&self) -> i32 {
        // Java: return _baseStyle.getValue(this._infoShort2);
        bf_baseStyle().getValue(self._infoShort2)
    }

    // Java: void setBaseStyle(int base)
    pub fn setBaseStyle(&mut self, base: i32) {
        // Java: this._infoShort2 = _baseStyle.setShortValue(this._infoShort2, (short)base);
        self._infoShort2 = bf_baseStyle().setShortValue(self._infoShort2, base as i16);
    }

    // Java: int getNextStyle()
    pub fn getNextStyle(&self) -> i32 {
        // Java: return _nextStyle.getValue(this._infoShort3);
        bf_nextStyle().getValue(self._infoShort3)
    }

    // Java: void setNextStyle(int next)
    pub fn setNextStyle(&mut self, next: i32) {
        // Java: this._infoShort3 = _nextStyle.setShortValue(self._infoShort3, (short)next);
        self._infoShort3 = bf_nextStyle().setShortValue(self._infoShort3, next as i16);
    }

    // Java: int getLinkStyle()
    pub fn getLinkStyle(&self) -> i32 {
        // Java: return _linkStyle.getValue(this._infoShort5);
        bf_linkStyle().getValue(self._infoShort5)
    }

    // Java: void setLinkStyle(int link)
    pub fn setLinkStyle(&mut self, link: i32) {
        // Java: this._infoShort5 = _linkStyle.setShortValue(this._infoShort5, (short)link);
        self._infoShort5 = bf_linkStyle().setShortValue(self._infoShort5, link as i16);
    }

    // Java: void setInvalHeight(boolean invalHeight)
    pub fn setInvalHeight(&mut self, inval_height: bool) {
        // Java: this._infoShort = (short)_fInvalHeight.setBoolean(this._infoShort, invalHeight);
        self._infoShort = bf_fInvalHeight().setBoolean(self._infoShort, inval_height) as i16;
    }

    // Java: void setHasUpe(boolean hasUpe)
    pub fn setHasUpe(&mut self, has_upe: bool) {
        // Java: this._infoShort = (short)_fHasUpe.setBoolean(this._infoShort, hasUpe);
        self._infoShort = bf_fHasUpe().setBoolean(self._infoShort, has_upe) as i16;
    }

    // Java: void setBchUpe()
    pub fn setBchUpe(&mut self) {
        // Java: this._bchUpe = (short)this.toByteArray().length;
        self._bchUpe = self.toByteArray().len() as i16;
    }

    // Java: void setNumUPX(int numUPX)
    pub fn setNumUPX(&mut self, num_upx: i32) {
        // Java: this._infoShort3 = _numUPX.setShortValue(this._infoShort3, (short)numUPX);
        self._infoShort3 = bf_numUPX().setShortValue(self._infoShort3, num_upx as i16);
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
        // Java: return _styleTypeCode.getValue(this._infoShort2);
        bf_styleTypeCode().getValue(self._infoShort2)
    }

    // Java: void setType(int type)
    pub fn setType(&mut self, style_type: i32) {
        // Java: this._infoShort2 = _styleTypeCode.setShortValue(this._infoShort2, (short)type);
        self._infoShort2 = bf_styleTypeCode().setShortValue(self._infoShort2, style_type as i16);
    }

    // Java: byte[] getCHPX()
    pub fn getCHPX(&self) -> Option<Vec<u8>> {
        // Java: switch (_styleTypeCode.getValue(this._infoShort2))
        match bf_styleTypeCode().getValue(self._infoShort2) {
            // Java: case 1:
            1 => {
                // Java: if (this._upxs.length > 1)
                if self._upxs.len() > 1 {
                    // Java: return this._upxs[1].getUPX();
                    Some(self._upxs[1].getUPX().to_vec())
                } else {
                    // Java: return null;
                    None
                }
            }
            // Java: case 2:
            2 => {
                // Java: return this._upxs[0].getUPX();
                Some(self._upxs[0].getUPX().to_vec())
            }
            // Java: case 3:
            3 => {
                // Java: if (this._upxs.length > 2)
                if self._upxs.len() > 2 {
                    // Java: return this._upxs[2].getUPX();
                    Some(self._upxs[2].getUPX().to_vec())
                } else {
                    None
                }
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: byte[] getPAPX()
    pub fn getPAPX(&self) -> Option<Vec<u8>> {
        // Java: switch (_styleTypeCode.getValue(this._infoShort2))
        match bf_styleTypeCode().getValue(self._infoShort2) {
            // Java: case 1:
            1 => {
                // Java: return this._upxs[0].getUPX();
                Some(self._upxs[0].getUPX().to_vec())
            }
            // Java: case 3:
            3 => {
                // Java: if (this._upxs.length > 1)
                if self._upxs.len() > 1 {
                    // Java: return this._upxs[1].getUPX();
                    Some(self._upxs[1].getUPX().to_vec())
                } else {
                    None
                }
            }
            // Java: case 4:
            4 => {
                // Java: return this._upxs[0].getUPX();
                Some(self._upxs[0].getUPX().to_vec())
            }
            // Java: case 2:
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: byte[] getTAPX()
    pub fn getTAPX(&self) -> Option<Vec<u8>> {
        // Java: switch (_styleTypeCode.getValue(this._infoShort2))
        match bf_styleTypeCode().getValue(self._infoShort2) {
            // Java: case 3:
            3 => {
                // Java: return this._upxs[0].getUPX();
                Some(self._upxs[0].getUPX().to_vec())
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: void setUPX(UPX[] upxs)
    pub fn setUPX(&mut self, upxs: Vec<UPX>) {
        // Java: this._upxs = upxs;
        self._upxs = upxs;
    }

    // Java: String getName()
    pub fn getName(&self) -> Option<&String> {
        // Java: return this._name;
        self._name.as_ref()
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this._name = name;
        self._name = Some(name);
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
        // Java: int size;
        let mut size: usize;
        // Java: if (this._name != null)
        if let Some(ref name) = self._name {
            // Java: size = this._baseLength + 2 + (this._name.length() + 1) * 2;
            size = self._baseLength as usize + 2 + (name.len() + 1) * 2;
        }
        // Java: else
        else {
            // Java: size = this._baseLength + 2 + 2;
            size = self._baseLength as usize + 2 + 2;
        }
        // Java: size += this._upxs[0].size() + 2;
        if !self._upxs.is_empty() {
            size += self._upxs[0].size() as usize + 2;
            // Java: for(int x = 1; x < this._upxs.length; ++x)
            for x in 1..self._upxs.len() {
                // Java: size += this._upxs[x - 1].size() % 2;
                size += (self._upxs[x - 1].size() % 2) as usize;
                // Java: size += this._upxs[x].size() + 2;
                size += self._upxs[x].size() as usize + 2;
            }
            // Java: if (this._upxs.length == 1)
            if self._upxs.len() == 1 {
                // Java: size += this._upxs[0].size() % 2;
                size += (self._upxs[0].size() % 2) as usize;
            }
            // Java: else
            else {
                // Java: size += this._upxs[this._upxs.length - 1].size() % 2;
                size += (self._upxs[self._upxs.len() - 1].size() % 2) as usize;
            }
        }

        // Java: byte[] buf = new byte[size];
        let mut buf = vec![0u8; size];
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: LittleEndian.putShort(buf, offset, this._infoShort);
        le_put_short(&mut buf, offset, self._infoShort);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, this._infoShort2);
        le_put_short(&mut buf, offset, self._infoShort2);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, this._infoShort3);
        le_put_short(&mut buf, offset, self._infoShort3);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, this._bchUpe);
        le_put_short(&mut buf, offset, self._bchUpe);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, this._infoShort4);
        le_put_short(&mut buf, offset, self._infoShort4);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(buf, offset, this._infoShort5);
        le_put_short(&mut buf, offset, self._infoShort5);
        // Java: offset = this._baseLength;
        offset = self._baseLength as usize;
        // Java: if (this._name != null)
        if let Some(ref name) = self._name {
            // Java: char[] letters = this._name.toCharArray();
            let letters: Vec<u16> = name.encode_utf16().collect();
            // Java: LittleEndian.putShort(buf, this._baseLength, (short)letters.length);
            le_put_short(&mut buf, self._baseLength as usize, letters.len() as i16);
            // Java: offset += 2;
            offset += 2;
            // Java: for(int x = 0; x < letters.length; ++x)
            for x in 0..letters.len() {
                // Java: LittleEndian.putShort(buf, offset, (short)letters[x]);
                le_put_short(&mut buf, offset, letters[x] as i16);
                // Java: offset += 2;
                offset += 2;
            }
        }
        // Java: else
        else {
            // Java: LittleEndian.putShort(buf, this._baseLength, (short)0);
            le_put_short(&mut buf, self._baseLength as usize, 0);
            // Java: offset += 2;
            offset += 2;
        }
        // Java: offset += 2;
        offset += 2;
        // Java: for(int x = 0; x < this._upxs.length; ++x)
        for x in 0..self._upxs.len() {
            // Java: int upxSize = this._upxs[x].size();
            let upx_size = self._upxs[x].size() as usize;
            // Java: LittleEndian.putShort(buf, offset, (short)upxSize);
            le_put_short(&mut buf, offset, upx_size as i16);
            // Java: offset += 2;
            offset += 2;
            // Java: System.arraycopy(this._upxs[x].getUPX(), 0, buf, offset, upxSize);
            let upx_bytes = self._upxs[x].getUPX();
            buf[offset..offset + upx_size].copy_from_slice(&upx_bytes[..upx_size]);
            // Java: offset += upxSize + upxSize % 2;
            offset += upx_size + upx_size % 2;
        }
        // Java: return buf;
        buf
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int prime = 31;
        // Java: int result = 1;
        let mut result: i32 = 1;
        // Java: result = 31 * result + this._baseLength;
        result = 31i32.wrapping_mul(result).wrapping_add(self._baseLength);
        // Java: result = 31 * result + this._bchUpe;
        result = 31i32.wrapping_mul(result).wrapping_add(self._bchUpe as i32);
        // Java: result = 31 * result + this._infoShort;
        result = 31i32.wrapping_mul(result).wrapping_add(self._infoShort as i32);
        // Java: result = 31 * result + this._infoShort2;
        result = 31i32.wrapping_mul(result).wrapping_add(self._infoShort2 as i32);
        // Java: result = 31 * result + this._infoShort3;
        result = 31i32.wrapping_mul(result).wrapping_add(self._infoShort3 as i32);
        // Java: result = 31 * result + this._infoShort4;
        result = 31i32.wrapping_mul(result).wrapping_add(self._infoShort4 as i32);
        // Java: result = 31 * result + this._infoShort5;
        result = 31i32.wrapping_mul(result).wrapping_add(self._infoShort5 as i32);
        // Java: result = 31 * result + this._istd;
        result = 31i32.wrapping_mul(result).wrapping_add(self._istd);
        // Java: result = 31 * result + (this._name == null ? 0 : this._name.hashCode());
        // Java: result = 31 * result + Arrays.hashCode(this._upxs);
        // Java: return result;
        result
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, other: &StyleDescription) -> bool {
        // Java: if (this == obj)
        // Java: return true;
        // Java: else if (obj == null)
        // Java: return false;
        // Java: else if (this.getClass() != obj.getClass())
        // Java: return false;
        // Java: else
        {
            // Java: StyleDescription other = (StyleDescription)obj;
            // Java: if (this._baseLength != other._baseLength)
            if self._baseLength != other._baseLength {
                // Java: return false;
                return false;
            }
            // Java: else if (this._bchUpe != other._bchUpe)
            else if self._bchUpe != other._bchUpe {
                // Java: return false;
                return false;
            }
            // Java: else if (this._infoShort != other._infoShort)
            else if self._infoShort != other._infoShort {
                // Java: return false;
                return false;
            }
            // Java: else if (this._infoShort2 != other._infoShort2)
            else if self._infoShort2 != other._infoShort2 {
                // Java: return false;
                return false;
            }
            // Java: else if (this._infoShort3 != other._infoShort3)
            else if self._infoShort3 != other._infoShort3 {
                // Java: return false;
                return false;
            }
            // Java: else if (this._infoShort4 != other._infoShort4)
            else if self._infoShort4 != other._infoShort4 {
                // Java: return false;
                return false;
            }
            // Java: else if (this._infoShort5 != other._infoShort5)
            else if self._infoShort5 != other._infoShort5 {
                // Java: return false;
                return false;
            }
            // Java: else if (this._istd != other._istd)
            else if self._istd != other._istd {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: if (this._name == null)
                if self._name.is_none() {
                    // Java: if (other._name != null)
                    if other._name.is_some() {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: else if (!this._name.equals(other._name))
                else if self._name != other._name {
                    // Java: return false;
                    return false;
                }
                // Java: return Arrays.equals(this._upxs, other._upxs);
                self._upxs.len() == other._upxs.len()
            }
        }
    }
}

impl HDFType for StyleDescription {}
