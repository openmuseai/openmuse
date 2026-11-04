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

// Java: public class CellHideMarkOperand implements Cloneable
pub struct CellHideMarkOperand {
    // Java field: short _itc
    pub(crate) _itc: i16,
    // Java field: byte _bArg
    pub(crate) _bArg: i8,
}

// Java: public class CellHideMarkOperand implements Cloneable
impl CellHideMarkOperand {
    // Java: CellHideMarkOperand()
    pub fn CellHideMarkOperand(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: CellHideMarkOperand(byte[] buf, int offset)
    pub fn CellHideMarkOperand(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._itc = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this._bArg = LittleEndian.getByte(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._itc);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: LittleEndian.putByte(buf, offset, this._bArg);
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
         // Java: CellHideMarkOperand cellHideMarkOperand = (CellHideMarkOperand)o;
         // TODO_RUST_CODE
         // Java: return this._itc == cellHideMarkOperand._itc && this._bArg == cellHideMarkOperand._bArg;
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

    // Java: byte getbArg()
    pub fn getbArg(&self) -> i8 {
      // Java: return this._bArg;
      // TODO_RUST_CODE
    }

    // Java: void setbArg(byte bArg)
    pub fn setbArg(&self, byte bArg) {
      // Java: this._bArg = bArg;
      // TODO_RUST_CODE
    }

}

impl Cloneable for CellHideMarkOperand {
}
