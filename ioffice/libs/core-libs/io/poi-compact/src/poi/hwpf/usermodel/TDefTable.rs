// Java: import cn.wps.base.assertion.IOAssert
use util::cn::wps::base::assertion::IOAssert;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java: public class TDefTable implements Cloneable
pub struct TDefTable {
    // Java field: byte _columnCount
    pub(crate) _columnCount: i8,
    // Java field: short[] _rgdxaCenter
    pub(crate) _rgdxaCenter: Arc<Mutex<Vec<i16>>>,
    // Java field: TableCellDescriptor[] _rgtc
    pub(crate) _rgtc: Arc<Mutex<Vec<TableCellDescriptor>>>,
}

// Java: public class TDefTable implements Cloneable
impl TDefTable {
    // Java: TDefTable()
    pub fn TDefTable(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: TDefTable(byte[] buf, int oldOffset, int size)
    pub fn TDefTable(&self, byte[] buf, int oldOffset, int size) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: int newOffset = oldOffset + this.read_columnCount(buf, oldOffset);
      // TODO_RUST_CODE
      // Java: this._columnCount = this._columnCount < 0 ? 0 : this._columnCount;
      // TODO_RUST_CODE
      // Java: newOffset += this.read_rgdxaCenter(buf, newOffset);
      // TODO_RUST_CODE
      // Java: int var10000 = newOffset + this.read_rgtc(buf, newOffset, size - (newOffset - oldOffset));
      // TODO_RUST_CODE
    }

    // Java: int read_columnCount(byte[] buf, int offset)
    pub fn read_columnCount(&self, byte[] buf, int offset) -> i32 {
      // Java: this._columnCount = LittleEndian.getByte(buf, offset);
      // TODO_RUST_CODE
      // Java: return 1;
      // TODO_RUST_CODE
    }

    // Java: int read_rgdxaCenter(byte[] buf, int oldOffset)
    pub fn read_rgdxaCenter(&self, byte[] buf, int oldOffset) -> i32 {
      // Java: int newOffset = oldOffset;
      // TODO_RUST_CODE
      // Java: this._rgdxaCenter = new short[this._columnCount + 1];
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < this._rgdxaCenter.length; ++i)
        // TODO_RUST_CODE
      {
         // Java: this._rgdxaCenter[i] = LittleEndian.getShort(buf, newOffset);
         // TODO_RUST_CODE
         // Java: newOffset += 2;
         // TODO_RUST_CODE
      }
      // Java: return newOffset - oldOffset;
      // TODO_RUST_CODE
    }

    // Java: int read_rgtc(byte[] buf, int oldOffset, int size)
    pub fn read_rgtc(&self, byte[] buf, int oldOffset, int size) -> i32 {
      // Java: IOAssert.assertTrue("0 == size % TableCellDescriptor.SIZE should be true!", 0 == size % 20);
      // TODO_RUST_CODE
      // Java: int newOffset = oldOffset;
      // TODO_RUST_CODE
      // Java: int sizeRgtc = Math.min(size / 20, this._columnCount);
      // TODO_RUST_CODE
      // Java: this._rgtc = new TableCellDescriptor[sizeRgtc];
      // TODO_RUST_CODE
      // Java: for(int j = 0; j < this._rgtc.length; ++j)
        // TODO_RUST_CODE
      {
         // Java: if (newOffset < buf.length && newOffset + 20 <= buf.length)
            // TODO_RUST_CODE
         {
            // Java: this._rgtc[j] = TableCellDescriptor.convertBytesToTC(buf, newOffset);
            // TODO_RUST_CODE
            // Java: newOffset += 20;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this._rgtc[j] = new TableCellDescriptor();
            // TODO_RUST_CODE
         }
      }
      // Java: return newOffset - oldOffset;
      // TODO_RUST_CODE
    }

    // Java: byte getColumnCount()
    pub fn getColumnCount(&self) -> i8 {
      // Java: return this._columnCount;
      // TODO_RUST_CODE
    }

    // Java: void setColumnCount(int count)
    pub fn setColumnCount(&self, int count) {
      // Java: this._columnCount = (byte)count;
      // TODO_RUST_CODE
    }

    // Java: short[] getRgdxaCenter()
    pub fn getRgdxaCenter(&self) -> short[] {
      // Java: return this._rgdxaCenter;
      // TODO_RUST_CODE
    }

    // Java: void setRgdxaCenter(short[] rgdxaCenter)
    pub fn setRgdxaCenter(&self, short[] rgdxaCenter) {
      // Java: this._rgdxaCenter = rgdxaCenter;
      // TODO_RUST_CODE
    }

    // Java: TableCellDescriptor[] getRgtc()
    pub fn getRgtc(&self) -> TableCellDescriptor[] {
      // Java: return this._rgtc;
      // TODO_RUST_CODE
    }

    // Java: void setRgtc(TableCellDescriptor[] rgtc)
    pub fn setRgtc(&self, TableCellDescriptor[] rgtc) {
      // Java: this._rgtc = rgtc;
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putByte(buf, offset, this._columnCount);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < this._rgdxaCenter.length; ++i)
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putShort(buf, offset, this._rgdxaCenter[i]);
         // TODO_RUST_CODE
         // Java: offset += 2;
         // TODO_RUST_CODE
      }
      // Java: for(int j = 0; j < this._rgtc.length; ++j)
        // TODO_RUST_CODE
      {
         // Java: this._rgtc[j].serialize(buf, offset);
         // TODO_RUST_CODE
         // Java: offset += 20;
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

}

impl Cloneable for TDefTable {
}
