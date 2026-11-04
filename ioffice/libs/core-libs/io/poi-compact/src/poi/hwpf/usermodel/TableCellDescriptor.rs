// Java: import org.apache.poi.hwpf.model.types.TCAbstractType
use io_poi_compact::poi::hwpf::model::types::TCAbstractType;

// Java field: int SIZE
pub(crate) const SIZE: i32 = /* TODO */;

// Java: public class TableCellDescriptor extends TCAbstractType implements Cloneable
pub struct TableCellDescriptor {
}

// Java: public class TableCellDescriptor extends TCAbstractType implements Cloneable
impl TableCellDescriptor {
    // Java: TableCellDescriptor()
    pub fn TableCellDescriptor(&self) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.field_3_brcTop = new BorderCode();
      // TODO_RUST_CODE
      // Java: this.field_4_brcLeft = new BorderCode();
      // TODO_RUST_CODE
      // Java: this.field_5_brcBottom = new BorderCode();
      // TODO_RUST_CODE
      // Java: this.field_6_brcRight = new BorderCode();
      // TODO_RUST_CODE
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: TableCellDescriptor tc = (TableCellDescriptor)super.clone();
      // TODO_RUST_CODE
      // Java: tc.field_3_brcTop = (BorderCode)this.field_3_brcTop.clone();
      // TODO_RUST_CODE
      // Java: tc.field_4_brcLeft = (BorderCode)this.field_4_brcLeft.clone();
      // TODO_RUST_CODE
      // Java: tc.field_5_brcBottom = (BorderCode)this.field_5_brcBottom.clone();
      // TODO_RUST_CODE
      // Java: tc.field_6_brcRight = (BorderCode)this.field_6_brcRight.clone();
      // TODO_RUST_CODE
      // Java: return tc;
      // TODO_RUST_CODE
    }

    // Java: static TableCellDescriptor convertBytesToTC(byte[] buf, int offset)
    pub fn convertBytesToTC(&self, byte[] buf, int offset) -> TableCellDescriptor {
      // Java: TableCellDescriptor tc = new TableCellDescriptor();
      // TODO_RUST_CODE
      // Java: tc.fillFields(buf, offset);
      // TODO_RUST_CODE
      // Java: return tc;
      // TODO_RUST_CODE
    }

}

impl Cloneable for TableCellDescriptor {
}
