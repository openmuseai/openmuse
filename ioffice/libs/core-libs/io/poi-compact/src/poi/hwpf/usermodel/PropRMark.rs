// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = 7;

// Java: public class PropRMark implements Cloneable
pub struct PropRMark {
    // Java field: boolean bPropRMark
    pub(crate) bPropRMark: bool,
    // Java field: short iAuthor
    pub(crate) iAuthor: i16,
    // Java field: DateAndTime dttm
    pub(crate) dttm: Arc<Mutex<DateAndTime>>,
}

// Java: public class PropRMark implements Cloneable
impl PropRMark {
    // Java: PropRMark()
    pub fn PropRMark(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: PropRMark(byte[] buf, int offset)
    pub fn PropRMark(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.bPropRMark = LittleEndian.getUnsignedByte(buf, offset) == 1;
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: this.iAuthor = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this.dttm = new DateAndTime(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: PropRMark(DocumentInputStream buf, int offset)
    pub fn PropRMark(&self, DocumentInputStream buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: buf.seek((long)offset);
      // TODO_RUST_CODE
      // Java: this.bPropRMark = buf.readUByte() == 1;
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: this.iAuthor = buf.readShort();
      // TODO_RUST_CODE
      // Java: offset += 2;
      // TODO_RUST_CODE
      // Java: this.dttm = new DateAndTime(buf, offset);
      // TODO_RUST_CODE
    }

    // Java: boolean isPropRMark()
    pub fn isPropRMark(&self) -> bool {
      // Java: return this.bPropRMark;
      // TODO_RUST_CODE
    }

    // Java: void setPropRMark(boolean bPropRMark)
    pub fn setPropRMark(&self, boolean bPropRMark) {
      // Java: this.bPropRMark = bPropRMark;
      // TODO_RUST_CODE
    }

    // Java: int getAuthor()
    pub fn getAuthor(&self) -> i32 {
      // Java: return this.iAuthor;
      // TODO_RUST_CODE
    }

    // Java: void setAuthor(int index)
    pub fn setAuthor(&self, int index) {
      // Java: this.iAuthor = (short)index;
      // TODO_RUST_CODE
    }

    // Java: DateAndTime getDTTM()
    pub fn getDTTM(&self) -> DateAndTime {
      // Java: return this.dttm;
      // TODO_RUST_CODE
    }

    // Java: void setDTTM(DateAndTime dttm)
    pub fn setDTTM(&self, DateAndTime dttm) {
      // Java: this.dttm = dttm;
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: byte[] dateTime = new byte[4];
      // TODO_RUST_CODE
      // Java: this.dttm.serialize(dateTime, 0);
      // TODO_RUST_CODE
      // Java: LittleEndian.putByte(buf, offset, this.bPropRMark ? 1 : 0);
      // TODO_RUST_CODE
      // Java: ++offset;
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset, this.iAuthor);
      // TODO_RUST_CODE
      // Java: System.arraycopy(dateTime, 0, buf, 3, 4);
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
         // Java: PropRMark propRMark = (PropRMark)o;
         // TODO_RUST_CODE
         // Java: return this.bPropRMark == propRMark.bPropRMark && this.iAuthor == propRMark.iAuthor && this.dttm.equals(propRMark.dttm);
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

}

impl Cloneable for PropRMark {
}
