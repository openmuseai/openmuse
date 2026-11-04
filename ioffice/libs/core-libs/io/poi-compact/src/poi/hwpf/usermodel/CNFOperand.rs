// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList
// Java: import org.apache.poi.hwpf.sprm.SprmIterator
use io_poi_compact::poi::hwpf::sprm::SprmIterator;
// Java: import org.apache.poi.hwpf.sprm.SprmOperation
use io_poi_compact::org::apache::poi::hwpf::sprm::SprmOperation;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int HeaderRow
pub(crate) const HeaderRow: i32 = 1;
// Java field: int FooterRow
pub(crate) const FooterRow: i32 = 2;
// Java field: int FirstColumn
pub(crate) const FirstColumn: i32 = 4;
// Java field: int LastColumn
pub(crate) const LastColumn: i32 = 8;
// Java field: int BandedColumns
pub(crate) const BandedColumns: i32 = 16;
// Java field: int EvenColumnBanding
pub(crate) const EvenColumnBanding: i32 = 32;
// Java field: int BandedRows
pub(crate) const BandedRows: i32 = 64;
// Java field: int EvenRowBanding
pub(crate) const EvenRowBanding: i32 = 128;
// Java field: int TopRightCell
pub(crate) const TopRightCell: i32 = 256;
// Java field: int TopLeftCell
pub(crate) const TopLeftCell: i32 = 512;
// Java field: int BottomRightCell
pub(crate) const BottomRightCell: i32 = 1024;
// Java field: int BottomLeftCell
pub(crate) const BottomLeftCell: i32 = 2048;

// Java: public class CNFOperand implements Cloneable
pub struct CNFOperand {
    // Java field: short _cnfc
    pub(crate) _cnfc: i16,
    // Java field: SprmOperation[] _grpprl
    pub(crate) _grpprl: Arc<Mutex<Vec<SprmOperation>>>,
}

// Java: public class CNFOperand implements Cloneable
impl CNFOperand {
    // Java: CNFOperand()
    pub fn CNFOperand(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: CNFOperand(byte[] buf, int offset, int size)
    pub fn CNFOperand(&self, byte[] buf, int offset, int size) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: int curSize = 0;
      // TODO_RUST_CODE
      // Java: this._cnfc = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: curSize += 2;
      // TODO_RUST_CODE
      // Java: ArrayList<SprmOperation> listSprmOperation = new ArrayList();
      // TODO_RUST_CODE
      // Java: SprmIterator sprmIt = new SprmIterator(buf, offset);
      // TODO_RUST_CODE
      // Java: while(sprmIt.hasNext())
        // TODO_RUST_CODE
      {
         // Java: SprmOperation sprm = sprmIt.next();
         // TODO_RUST_CODE
         // Java: SprmOperation newSprm = new SprmOperation();
         // TODO_RUST_CODE
         // Java: newSprm.init(buf, offset);
         // TODO_RUST_CODE
         // Java: curSize += sprm.size();
         // TODO_RUST_CODE
         // Java: offset += sprm.size();
         // TODO_RUST_CODE
         // Java: if (curSize > size)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
         // Java: listSprmOperation.add(newSprm);
         // TODO_RUST_CODE
      }
      // Java: this._grpprl = new SprmOperation[listSprmOperation.size()];
      // TODO_RUST_CODE
      // Java: listSprmOperation.toArray(this._grpprl);
      // TODO_RUST_CODE
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: int ret = 2;
      // TODO_RUST_CODE
      // Java: int size = this._grpprl.length;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: ret += this._grpprl[i].size();
         // TODO_RUST_CODE
      }
      // Java: return ret;
      // TODO_RUST_CODE
    }

    // Java: short getcnfc()
    pub fn getcnfc(&self) -> i16 {
      // Java: return this._cnfc;
      // TODO_RUST_CODE
    }

    // Java: void setcnfc(short cnfc)
    pub fn setcnfc(&self, short cnfc) {
      // Java: this._cnfc = cnfc;
      // TODO_RUST_CODE
    }

    // Java: SprmOperation[] getgrpprl()
    pub fn getgrpprl(&self) -> SprmOperation[] {
      // Java: return this._grpprl;
      // TODO_RUST_CODE
    }

    // Java: void setgrpprl(SprmOperation[] grpprl)
    pub fn setgrpprl(&self, SprmOperation[] grpprl) {
      // Java: this._grpprl = grpprl;
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._cnfc);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: for(int j = 0; j < this._grpprl.length; ++j)
        // TODO_RUST_CODE
      {
         // Java: this._grpprl[j].serialize(buf, offset);
         // TODO_RUST_CODE
         // Java: offset += this._grpprl[j].size();
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

}

impl Cloneable for CNFOperand {
}
