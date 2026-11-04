#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.IOException
// Java: import java.util.ArrayList

use super::SecurityWriterBase::SecurityWriterBase;
// TransformMapEntry not yet available in write module
// use super::TransformMapEntry::TransformMapEntry;

// Java: public class TransformMap extends SecurityWriterBase
pub struct TransformMap {
    // Java field: int mHeaderLength
    mHeaderLength: i32,
    // Java field: ArrayList<TransformMapEntry> mMapEntries
    mMapEntries: Vec<Box<dyn std::any::Any>>,
    pub base: SecurityWriterBase,
}

// Java: public class TransformMap extends SecurityWriterBase
impl TransformMap {
    // Java: TransformMap()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mMapEntries.add(new TransformMapEntry());
      todo!()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: int size = 0;
      let mut size = 0i32;
      // Java: for(int i = 0; i < this.mMapEntries.size(); ++i)
      for _i in 0..self.mMapEntries.len() {
         // Java: size += ((TransformMapEntry)this.mMapEntries.get(i)).getSize();
         todo!()
      }
      // Java: return 8 + size;
      8 + size
    }

    // Java: void writeValue()
    pub fn writeValue(&mut self) {
      // Java: this.outputStream.writeInt(8);
      // Java: this.outputStream.writeInt(this.mMapEntries.size());
      // Java: for(int i = 0; i < this.mMapEntries.size(); ++i)
      {
         // Java: ((TransformMapEntry)this.mMapEntries.get(i)).write(this.outputStream);
      }
      todo!()
    }

}
