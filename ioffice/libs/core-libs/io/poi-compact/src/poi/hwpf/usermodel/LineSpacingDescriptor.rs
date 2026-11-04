// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java: public class LineSpacingDescriptor implements Cloneable
pub struct LineSpacingDescriptor {
    // Java field: short _dyaLine
    pub(crate) _dyaLine: i16,
    // Java field: short _fMultiLinespace
    pub(crate) _fMultiLinespace: i16,
}

// Java: public class LineSpacingDescriptor implements Cloneable
impl LineSpacingDescriptor {
    // Java: LineSpacingDescriptor()
    pub fn LineSpacingDescriptor(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: LineSpacingDescriptor(byte[] buf, int offset)
    pub fn LineSpacingDescriptor(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._dyaLine = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: this._fMultiLinespace = LittleEndian.getShort(buf, offset + 2);
      // TODO_RUST_CODE
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: void setMultiLinespace(short fMultiLinespace)
    pub fn setMultiLinespace(&self, short fMultiLinespace) {
      // Java: this._fMultiLinespace = fMultiLinespace;
      // TODO_RUST_CODE
    }

    // Java: short getMultiLinespace()
    pub fn getMultiLinespace(&self) -> i16 {
      // Java: return this._fMultiLinespace;
      // TODO_RUST_CODE
    }

    // Java: short getDyaLine()
    pub fn getDyaLine(&self) -> i16 {
      // Java: return this._dyaLine;
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
      // Java: LittleEndian.putShort(buf, offset, this._dyaLine);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset + 2, this._fMultiLinespace);
      // TODO_RUST_CODE
    }

    // Java: void setDyaLine(short dyaLine)
    pub fn setDyaLine(&self, short dyaLine) {
      // Java: this._dyaLine = dyaLine;
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
         // Java: LineSpacingDescriptor lspd = (LineSpacingDescriptor)o;
         // TODO_RUST_CODE
         // Java: return this._dyaLine == lspd._dyaLine && this._fMultiLinespace == lspd._fMultiLinespace;
         // TODO_RUST_CODE
      }
    }

}

impl Cloneable for LineSpacingDescriptor {
}
