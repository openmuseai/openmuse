#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;

// Java: public class TransformMap
pub struct TransformMap {
    // Java field: DocumentInputStream mDocInputStream
    mDocInputStream: Option<Box<dyn std::any::Any>>,
    // Java field: int mHeaderLength
    mHeaderLength: i32,
    // Java field: int mEntryCount
    mEntryCount: i32,
    // Java field: ArrayList<TransformMapEntry> mMapEntries
    mMapEntries: Vec<Box<dyn std::any::Any>>,
}

// Java: public class TransformMap
impl TransformMap {
    // Java: TransformMap(DocumentInputStream input)
    pub fn new(_input: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mDocInputStream = input;
      // Java: this.mHeaderLength = this.mDocInputStream.readInt();
      // Java: this.mEntryCount = this.mDocInputStream.readInt();
      // Java: this.mMapEntries = new ArrayList(this.mEntryCount);
      // Java: this.mMapEntries.add(new TransformMapEntry(input));
      todo!()
    }

    // Java: ArrayList<TransformMapEntry> getTransformMapEntry()
    pub fn getTransformMapEntry(&self) -> &Vec<Box<dyn std::any::Any>> {
      // Java: return this.mMapEntries;
      &self.mMapEntries
    }

}
