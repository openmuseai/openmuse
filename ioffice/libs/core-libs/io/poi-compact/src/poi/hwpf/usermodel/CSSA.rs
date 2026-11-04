// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = /* TODO */;
// Java field: BitField _itcFirst
static _itcFirst: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _itcLim
static _itcLim: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();

// Java: public class CSSA implements Cloneable
pub struct CSSA {
    // Java field: short _itc
    pub(crate) _itc: i16,
    // Java field: byte _grfbrc
    pub(crate) _grfbrc: i8,
    // Java field: byte _ftsWidth
    pub(crate) _ftsWidth: i8,
    // Java field: int _wWidth
    pub(crate) _wWidth: i32,
}

// Java: public class CSSA implements Cloneable
impl CSSA {
    // Java: CSSA()
    pub fn CSSA(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: CSSA(byte[] buf, int offset)
    pub fn CSSA(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._itc = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this._grfbrc = LittleEndian.getByte(buf, offset);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: this._ftsWidth = LittleEndian.getByte(buf, offset);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: this._wWidth = LittleEndian.getUShort(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._itc);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: LittleEndian.putByte(buf, offset, this._grfbrc);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: LittleEndian.putByte(buf, offset, this._ftsWidth);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: LittleEndian.putUShort(buf, offset, this._wWidth);
      // TODO_RUST_CODE
    }

    // Java: int toInt()
    pub fn toInt(&self) -> i32 {
      // Java: byte[] buf = new byte[6];
      // TODO_RUST_CODE
      // Java: this.serialize(buf, 0);
      // TODO_RUST_CODE
      // Java: return LittleEndian.getInt(buf, 0);
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
         // Java: CSSA cssa = (CSSA)o;
         // TODO_RUST_CODE
         // Java: return this._itc == cssa._itc && this._grfbrc == cssa._grfbrc && this._ftsWidth == cssa._ftsWidth && this._wWidth == cssa._wWidth;
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: int getItc()
    pub fn getItc(&self) -> i32 {
      // Java: return this._itc;
      // TODO_RUST_CODE
    }

    // Java: void setItc(int itc)
    pub fn setItc(&self, int itc) {
      // Java: this._itc = (short)itc;
      // TODO_RUST_CODE
    }

    // Java: int getItcFirst()
    pub fn getItcFirst(&self) -> i32 {
      // Java: return _itcFirst.getShortValue(this._itc);
      // TODO_RUST_CODE
    }

    // Java: void setItcFirst(int itcFirst)
    pub fn setItcFirst(&self, int itcFirst) {
      // Java: this._itc = _itcFirst.setShortValue(this._itc, (short)itcFirst);
      // TODO_RUST_CODE
    }

    // Java: int getItcLim()
    pub fn getItcLim(&self) -> i32 {
      // Java: return _itcLim.getShortValue(this._itc);
      // TODO_RUST_CODE
    }

    // Java: void setItcLim(int itcLim)
    pub fn setItcLim(&self, int itcLim) {
      // Java: this._itc = _itcLim.setShortValue(this._itc, (short)itcLim);
      // TODO_RUST_CODE
    }

    // Java: int getBrcSide()
    pub fn getBrcSide(&self) -> i32 {
      // Java: return this._grfbrc;
      // TODO_RUST_CODE
    }

    // Java: void setBrcSide(int grfbrc)
    pub fn setBrcSide(&self, int grfbrc) {
      // Java: this._grfbrc = (byte)grfbrc;
      // TODO_RUST_CODE
    }

    // Java: int getMeasureType()
    pub fn getMeasureType(&self) -> i32 {
      // Java: return this._ftsWidth;
      // TODO_RUST_CODE
    }

    // Java: void setMeasureType(int ftsWidth)
    pub fn setMeasureType(&self, int ftsWidth) {
      // Java: this._ftsWidth = (byte)ftsWidth;
      // TODO_RUST_CODE
    }

    // Java: int getSpacing()
    pub fn getSpacing(&self) -> i32 {
      // Java: return this._wWidth;
      // TODO_RUST_CODE
    }

    // Java: void setSpacing(int wWidth)
    pub fn setSpacing(&self, int wWidth) {
      // Java: this._wWidth = wWidth;
      // TODO_RUST_CODE
    }

}

impl Cloneable for CSSA {
}
