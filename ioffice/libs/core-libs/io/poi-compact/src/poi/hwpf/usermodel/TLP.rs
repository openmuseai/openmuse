// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = /* TODO */;
// Java field: short itlNil
pub(crate) const itlNil: i16 = /* TODO */;
// Java field: short itlNone
pub(crate) const itlNone: i16 = /* TODO */;
// Java field: BitField _fatlBorders
static _fatlBorders: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlShading
static _fatlShading: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlFont
static _fatlFont: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlColor
static _fatlColor: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlBestFit
static _fatlBestFit: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlHdrRows
static _fatlHdrRows: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlLastRow
static _fatlLastRow: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlHdrCols
static _fatlHdrCols: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlLastCol
static _fatlLastCol: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlNoRowBands
static _fatlNoRowBands: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _fatlNoColBands
static _fatlNoColBands: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _padding
static _padding: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();

// Java: public class TLP implements Cloneable
pub struct TLP {
    // Java field: short _itl
    pub(crate) _itl: i16,
    // Java field: short _grfatl
    _grfatl: i16,
}

// Java: public class TLP implements Cloneable
impl TLP {
    // Java: TLP()
    pub fn TLP(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: TLP(byte[] buf, int offset)
    pub fn TLP(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._itl = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this._grfatl = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._itl);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset, this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: int toInt()
    pub fn toInt(&self) -> i32 {
      // Java: byte[] buf = new byte[4];
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
         // Java: TLP tlp = (TLP)o;
         // TODO_RUST_CODE
         // Java: return this._itl == tlp._itl && this._grfatl == tlp._grfatl;
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: short getItl()
    pub fn getItl(&self) -> i16 {
      // Java: return this._itl;
      // TODO_RUST_CODE
    }

    // Java: void setItl(short itl)
    pub fn setItl(&self, short itl) {
      // Java: this._itl = itl;
      // TODO_RUST_CODE
    }

    // Java: short getfatlBorders()
    pub fn getfatlBorders(&self) -> i16 {
      // Java: return _fatlBorders.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlBorders(short fatlBorders)
    pub fn setfatlBorders(&self, short fatlBorders) {
      // Java: this._grfatl = _fatlBorders.setShortValue(this._grfatl, fatlBorders);
      // TODO_RUST_CODE
    }

    // Java: short getfatlShading()
    pub fn getfatlShading(&self) -> i16 {
      // Java: return _fatlShading.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlShading(short fatlShading)
    pub fn setfatlShading(&self, short fatlShading) {
      // Java: this._grfatl = _fatlShading.setShortValue(this._grfatl, fatlShading);
      // TODO_RUST_CODE
    }

    // Java: short getfatlFont()
    pub fn getfatlFont(&self) -> i16 {
      // Java: return _fatlFont.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlFont(short fatlFont)
    pub fn setfatlFont(&self, short fatlFont) {
      // Java: this._grfatl = _fatlFont.setShortValue(this._grfatl, fatlFont);
      // TODO_RUST_CODE
    }

    // Java: short getfatlColor()
    pub fn getfatlColor(&self) -> i16 {
      // Java: return _fatlColor.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlColor(short fatlColor)
    pub fn setfatlColor(&self, short fatlColor) {
      // Java: this._grfatl = _fatlColor.setShortValue(this._grfatl, fatlColor);
      // TODO_RUST_CODE
    }

    // Java: short getfatlBestFit()
    pub fn getfatlBestFit(&self) -> i16 {
      // Java: return _fatlBestFit.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlBestFit(short fatlBestFit)
    pub fn setfatlBestFit(&self, short fatlBestFit) {
      // Java: this._grfatl = _fatlBestFit.setShortValue(this._grfatl, fatlBestFit);
      // TODO_RUST_CODE
    }

    // Java: short getfatlHdrRows()
    pub fn getfatlHdrRows(&self) -> i16 {
      // Java: return _fatlHdrRows.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlHdrRows(short fatlHdrRows)
    pub fn setfatlHdrRows(&self, short fatlHdrRows) {
      // Java: this._grfatl = _fatlHdrRows.setShortValue(this._grfatl, fatlHdrRows);
      // TODO_RUST_CODE
    }

    // Java: short getfatlLastRow()
    pub fn getfatlLastRow(&self) -> i16 {
      // Java: return _fatlLastRow.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlLastRow(short fatlLastRow)
    pub fn setfatlLastRow(&self, short fatlLastRow) {
      // Java: this._grfatl = _fatlLastRow.setShortValue(this._grfatl, fatlLastRow);
      // TODO_RUST_CODE
    }

    // Java: short getfatlHdrCols()
    pub fn getfatlHdrCols(&self) -> i16 {
      // Java: return _fatlHdrCols.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlHdrCols(short fatlHdrCols)
    pub fn setfatlHdrCols(&self, short fatlHdrCols) {
      // Java: this._grfatl = _fatlHdrCols.setShortValue(this._grfatl, fatlHdrCols);
      // TODO_RUST_CODE
    }

    // Java: short getfatlLastCol()
    pub fn getfatlLastCol(&self) -> i16 {
      // Java: return _fatlLastCol.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlLastCol(short fatlLastCol)
    pub fn setfatlLastCol(&self, short fatlLastCol) {
      // Java: this._grfatl = _fatlLastCol.setShortValue(this._grfatl, fatlLastCol);
      // TODO_RUST_CODE
    }

    // Java: short getfatlNoRowBands()
    pub fn getfatlNoRowBands(&self) -> i16 {
      // Java: return _fatlNoRowBands.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlNoRowBands(short fatlNoRowBands)
    pub fn setfatlNoRowBands(&self, short fatlNoRowBands) {
      // Java: this._grfatl = _fatlNoRowBands.setShortValue(this._grfatl, fatlNoRowBands);
      // TODO_RUST_CODE
    }

    // Java: short getfatlNoColBands()
    pub fn getfatlNoColBands(&self) -> i16 {
      // Java: return _fatlNoColBands.getShortValue(this._grfatl);
      // TODO_RUST_CODE
    }

    // Java: void setfatlNoColBands(short fatlNoColBands)
    pub fn setfatlNoColBands(&self, short fatlNoColBands) {
      // Java: this._grfatl = _fatlNoColBands.setShortValue(this._grfatl, fatlNoColBands);
      // TODO_RUST_CODE
    }

}

impl Cloneable for TLP {
}
