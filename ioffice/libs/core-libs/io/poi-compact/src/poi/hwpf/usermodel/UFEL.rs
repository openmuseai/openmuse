// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = 2;
// Java field: BitField _fTNY
static _fTNY: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fWarichu
static _fWarichu: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _iWarichuBracket
static _iWarichuBracket: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fTNYCompress
static _fTNYCompress: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();

// Java: public class UFEL
pub struct UFEL {
    // Java field: short _info
    pub(crate) _info: i16,
}

// Java: public class UFEL
impl UFEL {
    // Java: UFEL()
    pub fn UFEL(&self) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._info = 0;
      // TODO_RUST_CODE
    }

    // Java: UFEL(byte[] buf, int offset)
    pub fn UFEL(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._info = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: boolean isHorzVert()
    pub fn isHorzVert(&self) -> bool {
      // Java: return _fTNY.getValue(this._info) != 0;
      // TODO_RUST_CODE
    }

    // Java: void setHorzVert(boolean horzVert)
    pub fn setHorzVert(&self, boolean horzVert) {
      // Java: this._info = (short)_fTNY.setValue(this._info, horzVert ? 1 : 0);
      // TODO_RUST_CODE
    }

    // Java: boolean isCombinable()
    pub fn isCombinable(&self) -> bool {
      // Java: return _fWarichu.isSet(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setCombinable(boolean combine)
    pub fn setCombinable(&self, boolean combine) {
      // Java: this._info = (short)_fWarichu.setValue(this._info, combine ? 1 : 0);
      // TODO_RUST_CODE
    }

    // Java: int getCombineBracket()
    pub fn getCombineBracket(&self) -> i32 {
      // Java: return _iWarichuBracket.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setCombineBracket(int bracket)
    pub fn setCombineBracket(&self, int bracket) {
      // Java: this._info = (short)_iWarichuBracket.setValue(this._info, bracket);
      // TODO_RUST_CODE
    }

    // Java: boolean isHorzVertCompressed()
    pub fn isHorzVertCompressed(&self) -> bool {
      // Java: return _fTNYCompress.isSet(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setHorzVertCompressed(boolean vertCompressed)
    pub fn setHorzVertCompressed(&self, boolean vertCompressed) {
      // Java: this._info = (short)_fTNYCompress.setValue(this._info, vertCompressed ? 1 : 0);
      // TODO_RUST_CODE
    }

    // Java: short toShort()
    pub fn toShort(&self) -> i16 {
      // Java: return this._info;
      // TODO_RUST_CODE
    }

}
