// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: BitField _type
static _type: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _lines
static _lines: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();

// Java: public class DropCapSpecifier
pub struct DropCapSpecifier {
    // Java field: short _info
    pub(crate) _info: i16,
}

// Java: public class DropCapSpecifier
impl DropCapSpecifier {
    // Java: DropCapSpecifier()
    pub fn DropCapSpecifier(&self) {
      // Java: this((short)0);
      // TODO_RUST_CODE
    }

    // Java: DropCapSpecifier(short info)
    pub fn DropCapSpecifier(&self, short info) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._info = info;
      // TODO_RUST_CODE
    }

    // Java: DropCapSpecifier(byte[] buf, int offset)
    pub fn DropCapSpecifier(&self, byte[] buf, int offset) {
      // Java: this(LittleEndian.getShort(buf, offset));
      // TODO_RUST_CODE
    }

    // Java: short toShort()
    pub fn toShort(&self) -> i16 {
      // Java: return this._info;
      // TODO_RUST_CODE
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return _type.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setType(int type)
    pub fn setType(&self, int type) {
      // Java: this._info = (short)_type.setValue(this._info, type);
      // TODO_RUST_CODE
    }

    // Java: int getLines()
    pub fn getLines(&self) -> i32 {
      // Java: return _lines.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setLines(int lines)
    pub fn setLines(&self, int lines) {
      // Java: this._info = (short)_lines.setValue(this._info, lines);
      // TODO_RUST_CODE
    }

}
