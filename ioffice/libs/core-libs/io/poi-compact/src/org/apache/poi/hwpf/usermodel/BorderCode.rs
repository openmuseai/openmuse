// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}
fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}
fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8;
}

struct BitField { mask: i32, shift: i32 }
impl BitField {
    fn getValue(&self, holder: i32) -> i32 { (holder & self.mask) >> self.shift }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self.mask) | ((value << self.shift) & self.mask) }
    fn getShortValue(&self, holder: i16) -> i16 { ((holder as i32 & self.mask) >> self.shift) as i16 }
}

const DPT_LINE_WIDTH: BitField = BitField { mask: 255, shift: 0 };
const BRC_TYPE: BitField = BitField { mask: 65280, shift: 8 };
const ICO: BitField = BitField { mask: 255, shift: 0 };
const DPT_SPACE: BitField = BitField { mask: 7936, shift: 8 };
const F_SHADOW: BitField = BitField { mask: 8192, shift: 13 };
const F_FRAME: BitField = BitField { mask: 16384, shift: 14 };

// Java: public class BorderCode implements Cloneable
pub struct BorderCode {
    // Java field: int SIZE
    // Java field: short _info
    _info: i16,
    // Java field: BitField _dptLineWidth
    // Java field: BitField _brcType
    // Java field: short _info2
    _info2: i16,
    // Java field: BitField _ico
    // Java field: BitField _dptSpace
    // Java field: BitField _fShadow
    // Java field: BitField _fFrame
}

pub const SIZE: i32 = 4;

// Java: public class BorderCode implements Cloneable
impl BorderCode {
    // Java: BorderCode()
    pub fn new() -> Self {
        // Java: super();
        BorderCode { _info: 0, _info2: 0 }
    }

    // Java: BorderCode(byte[] buf, int offset)
    pub fn new_with_buf(buf: &[u8], offset: i32) -> Self {
        // Java: super();
        let off = offset as usize;
        // Java: this._info = LittleEndian.getShort(buf, offset);
        let _info = le_get_short(buf, off);
        // Java: this._info2 = LittleEndian.getShort(buf, offset + 2);
        let _info2 = le_get_short(buf, off + 2);
        BorderCode { _info, _info2 }
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, buf: &mut [u8], offset: i32) {
        let off = offset as usize;
        // Java: LittleEndian.putShort(buf, offset, this._info);
        le_put_short(buf, off, self._info);
        // Java: LittleEndian.putShort(buf, offset + 2, this._info2);
        le_put_short(buf, off + 2, self._info2);
    }

    // Java: int toInt()
    pub fn toInt(&self) -> i32 {
        // Java: byte[] buf = new byte[4];
        let mut buf = [0u8; 4];
        // Java: this.serialize(buf, 0);
        self.serialize(&mut buf, 0);
        // Java: return LittleEndian.getInt(buf);
        le_get_int(&buf, 0)
    }

    // Java: boolean isNoBorder()
    pub fn isNoBorder(&self) -> bool {
        // Java: return this._info == -1 && this._info2 == -1;
        self._info == -1 && self._info2 == -1
    }

    // Java: void setNoBorder()
    pub fn setNoBorder(&mut self) {
        // Java: this._info = -1;
        self._info = -1;
        // Java: this._info2 = -1;
        self._info2 = -1;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let ptr = self as *const _ as usize;
        ptr as i32
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&BorderCode>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: BorderCode brc = (BorderCode)o;
            let brc = o.unwrap();
            // Java: return this._info == brc._info && this._info2 == brc._info2;
            return self._info == brc._info && self._info2 == brc._info2;
        }
    }

    // Java: Object clone()
    // Java: return super.clone();

    // Java: int getLineWidth()
    pub fn getLineWidth(&self) -> i32 {
        // Java: return _dptLineWidth.getShortValue(this._info);
        DPT_LINE_WIDTH.getShortValue(self._info) as i32
    }

    // Java: void setLineWidth(int lineWidth)
    pub fn setLineWidth(&mut self, lineWidth: i32) {
        // Java: this._info = (short)_dptLineWidth.setValue(this._info, lineWidth);
        self._info = DPT_LINE_WIDTH.setValue(self._info as i32, lineWidth) as i16;
    }

    // Java: int getBorderType()
    pub fn getBorderType(&self) -> i32 {
        // Java: return _brcType.getShortValue(this._info);
        BRC_TYPE.getShortValue(self._info) as i32
    }

    // Java: void setBorderType(int borderType)
    pub fn setBorderType(&mut self, borderType: i32) {
        // Java: this._info = (short)_brcType.setValue(this._info, borderType);
        self._info = BRC_TYPE.setValue(self._info as i32, borderType) as i16;
    }

    // Java: short getICo()
    pub fn getICo(&self) -> i16 {
        // Java: return _ico.getShortValue(this._info2);
        ICO.getShortValue(self._info2)
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
        // Java: this._info2 = (short)_ico.setValue(this._info2, color);
        self._info2 = ICO.setValue(self._info2 as i32, color) as i16;
    }

    // Java: int getSpace()
    pub fn getSpace(&self) -> i32 {
        // Java: return _dptSpace.getShortValue(this._info2);
        DPT_SPACE.getShortValue(self._info2) as i32
    }

    // Java: void setSpace(int space)
    pub fn setSpace(&mut self, space: i32) {
        // Java: this._info2 = (short)_dptSpace.setValue(this._info2, space);
        self._info2 = DPT_SPACE.setValue(self._info2 as i32, space) as i16;
    }

    // Java: boolean isShadow()
    pub fn isShadow(&self) -> bool {
        // Java: return _fShadow.getValue(this._info2) != 0;
        F_SHADOW.getValue(self._info2 as i32) != 0
    }

    // Java: void setShadow(boolean shadow)
    pub fn setShadow(&mut self, shadow: bool) {
        // Java: this._info2 = (short)_fShadow.setValue(this._info2, shadow ? 1 : 0);
        self._info2 = F_SHADOW.setValue(self._info2 as i32, if shadow { 1 } else { 0 }) as i16;
    }

    // Java: boolean isFrame()
    pub fn isFrame(&self) -> bool {
        // Java: return _fFrame.getValue(this._info2) != 0;
        F_FRAME.getValue(self._info2 as i32) != 0
    }

    // Java: void setFrame(boolean frame)
    pub fn setFrame(&mut self, frame: bool) {
        // Java: this._info2 = (short)_fFrame.setValue(this._info2, frame ? 1 : 0);
        self._info2 = F_FRAME.setValue(self._info2 as i32, if frame { 1 } else { 0 }) as i16;
    }
}
