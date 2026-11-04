#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;

// Java: public class TransformMapEntry
pub struct TransformMapEntry {
    // Java field: DocumentInputStream mDocInputStream
    mDocInputStream: Option<Box<dyn std::any::Any>>,
    // Java field: int mLength
    mLength: i32,
    // Java field: int mTransformType
    mTransformType: i32,
    // Java field: int mTransformVersion
    mTransformVersion: i32,
    // Java field: String mTransformID
    mTransformID: String,
    // Java field: String mTransformName
    mTransformName: String,
    // Java field: String mTransformSource
    mTransformSource: String,
    // Java field: int mTransformItemType
    mTransformItemType: i32,
    // Java field: String mTransformItemName
    mTransformItemName: String,
}

// Java: public class TransformMapEntry
impl TransformMapEntry {
    // Java: TransformMapEntry(DocumentInputStream input)
    pub fn new(_input: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mDocInputStream = input;
      // Java: this.mLength = this.mDocInputStream.readInt();
      // Java: this.mTransformType = this.mDocInputStream.readInt();
      // Java: this.mTransformVersion = this.mDocInputStream.readInt();
      // Java: this.mTransformID = Tool.readUNICODELPP4(input);
      // Java: this.mTransformName = Tool.readUNICODELPP4(input);
      // Java: this.mTransformSource = Tool.readUNICODELPP4(input);
      // Java: this.mTransformItemType = this.mDocInputStream.readInt();
      // Java: this.mTransformItemName = Tool.readUNICODELPP4(input);
      todo!()
    }

    // Java: String getTransformItemName()
    pub fn getTransformItemName(&self) -> String {
      // Java: return this.mTransformItemName;
      self.mTransformItemName.clone()
    }

    // Java: String getTransformID()
    pub fn getTransformID(&self) -> String {
      // Java: return this.mTransformID;
      self.mTransformID.clone()
    }

}
