// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java: public class TableWidth
pub struct TableWidth {
    // Java field: short _ftsWidth
    pub(crate) _ftsWidth: i16,
    // Java field: short _wWidth
    pub(crate) _wWidth: i16,
}

// Java: public class TableWidth
impl TableWidth {
    // Java: TableWidth()
    pub fn TableWidth(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: TableWidth(byte[] buf, int offset)
    pub fn TableWidth(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._ftsWidth = (short)LittleEndian.getByte(buf, offset);
      // TODO_RUST_CODE
      // Java: this._wWidth = LittleEndian.getShort(buf, offset + 1);
      // TODO_RUST_CODE
    }

    // Java: short getFtsWidth()
    pub fn getFtsWidth(&self) -> i16 {
      // Java: return this._ftsWidth;
      // TODO_RUST_CODE
    }

    // Java: void setFtsWidth(short ftsWidth)
    pub fn setFtsWidth(&self, short ftsWidth) {
      // Java: this._ftsWidth = ftsWidth;
      // TODO_RUST_CODE
    }

    // Java: int getWWidth()
    pub fn getWWidth(&self) -> i32 {
      // Java: return this._wWidth;
      // TODO_RUST_CODE
    }

    // Java: void setWWidth(short wWidth)
    pub fn setWWidth(&self, short wWidth) {
      // Java: this._wWidth = wWidth;
      // TODO_RUST_CODE
    }

    // Java: int toInt()
    pub fn toInt(&self) -> i32 {
      // Java: byte[] intHolder = new byte[4];
      // TODO_RUST_CODE
      // Java: this.serialize(intHolder, 0);
      // TODO_RUST_CODE
      // Java: return LittleEndian.getInt(intHolder);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._ftsWidth);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset + 1, this._wWidth);
      // TODO_RUST_CODE
    }

}
