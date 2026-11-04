// Java: import cn.wps.base.assertion.IOAssert
use util::cn::wps::base::assertion::IOAssert;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int STEP
pub(crate) const STEP: i32 = /* TODO */;

// Java: public class BrcCvOperand implements Cloneable
pub struct BrcCvOperand {
    // Java field: int[] _rgcv
    pub(crate) _rgcv: Arc<Mutex<Vec<i32>>>,
}

// Java: public class BrcCvOperand implements Cloneable
impl BrcCvOperand {
    // Java: BrcCvOperand()
    pub fn BrcCvOperand(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: BrcCvOperand(byte[] buf, int offset, int sizeInBytes)
    pub fn BrcCvOperand(&self, byte[] buf, int offset, int sizeInBytes) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: IOAssert.assertEquals("sizeInBytes % STEP should be equal to 0!", 0, sizeInBytes % 4);
      // TODO_RUST_CODE
      // Java: int size = sizeInBytes / 4;
      // TODO_RUST_CODE
      // Java: this._rgcv = new int[size];
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: int value = LittleEndian.getInt(buf, offset + 4 * i);
         // TODO_RUST_CODE
         // Java: this._rgcv[i] = ShadingDescriptorEx.reverse(value);
         // TODO_RUST_CODE
      }
    }

    // Java: int[] getRgcv()
    pub fn getRgcv(&self) -> int[] {
      // Java: return this._rgcv;
      // TODO_RUST_CODE
    }

    // Java: void setRgcv(int[] rgcv)
    pub fn setRgcv(&self, int[] rgcv) {
      // Java: this._rgcv = rgcv;
      // TODO_RUST_CODE
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this._rgcv.length * 4;
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: int size = this._rgcv.length;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: if (-1 == this._rgcv[i])
            // TODO_RUST_CODE
         {
            // Java: LittleEndian.putInt(buf, offset, -16777216);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: LittleEndian.putInt(buf, offset, ShadingDescriptorEx.reverse(this._rgcv[i]));
            // TODO_RUST_CODE
         }
         // Java: offset += 4;
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      // TODO_RUST_CODE
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, Object o) -> bool {
      // Java: BrcCvOperand operand = (BrcCvOperand)o;
      // TODO_RUST_CODE
      // Java: if (null == operand)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (this._rgcv.length != operand._rgcv.length)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int size = this._rgcv.length;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < size; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (this._rgcv[i] != operand._rgcv[i])
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
         }
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

}

impl Cloneable for BrcCvOperand {
}
