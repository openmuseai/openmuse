// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = 8;
// Java field: BitField _dptLineWidth
static _dptLineWidth: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _brcType
static _brcType: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _dptSpace
static _dptSpace: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fShadow
static _fShadow: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fFrame
static _fFrame: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();

// Java: public class BorderCodeEx implements Cloneable
pub struct BorderCodeEx {
    // Java field: int _color
    pub(crate) _color: i32,
    // Java field: short _info1
    _info1: i16,
    // Java field: short _info2
    _info2: i16,
}

// Java: public class BorderCodeEx implements Cloneable
impl BorderCodeEx {
    // Java: BorderCodeEx()
    pub fn BorderCodeEx(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: BorderCodeEx(byte[] buf, int offset)
    pub fn BorderCodeEx(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._color = LittleEndian.getInt(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 4;
      // TODO_RUST_CODE
      // Java: this._info1 = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this._info2 = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putInt(buf, offset, this._color);
      // TODO_RUST_CODE
      // Java: offset += 4;
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset, this._info1);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset, this._info2);
      // TODO_RUST_CODE
    }

    // Java: long toInt()
    pub fn toInt(&self) -> i64 {
      // Java: byte[] buf = new byte[8];
      // TODO_RUST_CODE
      // Java: this.serialize(buf, 0);
      // TODO_RUST_CODE
      // Java: return LittleEndian.getLong(buf, 0);
      // TODO_RUST_CODE
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      // TODO_RUST_CODE
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, Object o) -> bool {
      // Java: if (o == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: BorderCodeEx brc = (BorderCodeEx)o;
         // TODO_RUST_CODE
         // Java: return this._color == brc._color && this._info1 == brc._info1 && this._info2 == brc._info2;
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: boolean isNoBorder()
    pub fn isNoBorder(&self) -> bool {
      // Java: return this._color == -1 && this._info1 == -1 && this._info2 == -1;
      // TODO_RUST_CODE
    }

    // Java: void setNoBorder()
    pub fn setNoBorder(&self) {
      // Java: this._color = -1;
      // TODO_RUST_CODE
      // Java: this._info1 = -1;
      // TODO_RUST_CODE
      // Java: this._info2 = -1;
      // TODO_RUST_CODE
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
      // Java: return this._color;
      // TODO_RUST_CODE
    }

    // Java: void setColor(int color)
    pub fn setColor(&self, int color) {
      // Java: this._color = color;
      // TODO_RUST_CODE
    }

    // Java: int getLineWidth()
    pub fn getLineWidth(&self) -> i32 {
      // Java: return _dptLineWidth.getValue(this._info1);
      // TODO_RUST_CODE
    }

    // Java: void setLineWidth(int lineWidth)
    pub fn setLineWidth(&self, int lineWidth) {
      // Java: this._info1 = (short)_dptLineWidth.setValue(this._info1, lineWidth);
      // TODO_RUST_CODE
    }

    // Java: int getBorderType()
    pub fn getBorderType(&self) -> i32 {
      // Java: return _brcType.getValue(this._info1);
      // TODO_RUST_CODE
    }

    // Java: void setBorderType(int borderType)
    pub fn setBorderType(&self, int borderType) {
      // Java: this._info1 = (short)_brcType.setValue(this._info1, borderType);
      // TODO_RUST_CODE
    }

    // Java: int getSpace()
    pub fn getSpace(&self) -> i32 {
      // Java: return _dptSpace.getShortValue(this._info2);
      // TODO_RUST_CODE
    }

    // Java: void setSpace(int space)
    pub fn setSpace(&self, int space) {
      // Java: this._info2 = _dptSpace.setShortValue(this._info2, (short)space);
      // TODO_RUST_CODE
    }

    // Java: boolean isShadow()
    pub fn isShadow(&self) -> bool {
      // Java: return _fShadow.getValue(this._info2) != 0;
      // TODO_RUST_CODE
    }

    // Java: void setShadow(boolean shadow)
    pub fn setShadow(&self, boolean shadow) {
      // Java: this._info2 = (short)_fShadow.setValue(this._info2, shadow ? 1 : 0);
      // TODO_RUST_CODE
    }

    // Java: boolean isFrame()
    pub fn isFrame(&self) -> bool {
      // Java: return _fFrame.getValue(this._info2) != 0;
      // TODO_RUST_CODE
    }

    // Java: void setFrame(boolean frame)
    pub fn setFrame(&self, boolean frame) {
      // Java: this._info2 = (short)_fFrame.setValue(this._info2, frame ? 1 : 0);
      // TODO_RUST_CODE
    }

}

impl Cloneable for BorderCodeEx {
}
