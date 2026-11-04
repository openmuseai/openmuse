// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java: public class TCellBrcTypeOperand implements Cloneable
pub struct TCellBrcTypeOperand {
    // Java field: byte[] _rgBrcType
    pub(crate) _rgBrcType: Arc<Mutex<Vec<u8>>>,
}

// Java: public class TCellBrcTypeOperand implements Cloneable
impl TCellBrcTypeOperand {
    // Java: TCellBrcTypeOperand()
    pub fn TCellBrcTypeOperand(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: TCellBrcTypeOperand(byte[] buf, int offset, int size)
    pub fn TCellBrcTypeOperand(&self, byte[] buf, int offset, int size) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._rgBrcType = new byte[size];
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: this._rgBrcType[i] = LittleEndian.getByte(buf, offset);
         // TODO_RUST_CODE
         // Java: ++offset;
         // TODO_RUST_CODE
      }
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: int size = this._rgBrcType.length;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putByte(buf, offset, this._rgBrcType[i]);
         // TODO_RUST_CODE
         // Java: ++offset;
         // TODO_RUST_CODE
      }
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
         // Java: TCellBrcTypeOperand operand = (TCellBrcTypeOperand)o;
         // TODO_RUST_CODE
         // Java: int size = operand._rgBrcType.length;
         // TODO_RUST_CODE
         // Java: if (this._rgBrcType.length != size)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < size; ++i)
                // TODO_RUST_CODE
            {
               // Java: if (this._rgBrcType[i] != operand._rgBrcType[i])
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

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: byte[] getRgBrcType()
    pub fn getRgBrcType(&self) -> byte[] {
      // Java: return this._rgBrcType;
      // TODO_RUST_CODE
    }

    // Java: void setRgBrcType(byte[] rgBrcType)
    pub fn setRgBrcType(&self, byte[] rgBrcType) {
      // Java: this._rgBrcType = rgBrcType;
      // TODO_RUST_CODE
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this._rgBrcType.length;
      // TODO_RUST_CODE
    }

}

impl Cloneable for TCellBrcTypeOperand {
}
