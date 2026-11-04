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

// Java: public class TableBrcOperand implements Cloneable
pub struct TableBrcOperand {
    // Java field: short _itc
    pub(crate) _itc: i16,
    // Java field: byte _bordersToApply
    pub(crate) _bordersToApply: i8,
    // Java field: BorderCodeEx _brc
    pub(crate) _brc: Arc<Mutex<BorderCodeEx>>,
}

// Java: public class TableBrcOperand implements Cloneable
impl TableBrcOperand {
    // Java: TableBrcOperand()
    pub fn TableBrcOperand(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: TableBrcOperand(byte[] buf, int offset)
    pub fn TableBrcOperand(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._itc = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this._bordersToApply = LittleEndian.getByte(buf, offset);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: this._brc = new BorderCodeEx(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._itc);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: LittleEndian.putByte(buf, offset, this._bordersToApply);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: this._brc.serialize(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: int toInt()
    pub fn toInt(&self) -> i32 {
      // Java: byte[] buf = new byte[11];
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
         // Java: TableBrcOperand tableBrc80Operand = (TableBrcOperand)o;
         // TODO_RUST_CODE
         // Java: return this._itc == tableBrc80Operand._itc && this._bordersToApply == tableBrc80Operand._bordersToApply && this._brc.equals(tableBrc80Operand._brc);
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

    // Java: int getBordersToApply()
    pub fn getBordersToApply(&self) -> i32 {
      // Java: return this._bordersToApply;
      // TODO_RUST_CODE
    }

    // Java: void setBordersToApply(int bordersToApply)
    pub fn setBordersToApply(&self, int bordersToApply) {
      // Java: this._bordersToApply = (byte)bordersToApply;
      // TODO_RUST_CODE
    }

    // Java: BorderCodeEx getBorderCode()
    pub fn getBorderCode(&self) -> BorderCodeEx {
      // Java: return this._brc;
      // TODO_RUST_CODE
    }

    // Java: void setBorderCode(BorderCodeEx brc)
    pub fn setBorderCode(&self, BorderCodeEx brc) {
      // Java: this._brc = brc;
      // TODO_RUST_CODE
    }

}

impl Cloneable for TableBrcOperand {
}
