#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.pro.security.read.Tool
use crate::cn::wps::pro::security::read::Tool::Tool;
// Java: import org.apache.poi.util.LittleEndianByteArrayOutputStream
// use util::org::apache::poi::util::LittleEndianByteArrayOutputStream;

// Java: public class TransformMapEntry
pub struct TransformMapEntry {
    // Java field: int mTransformType
    mTransformType: i32,
    // Java field: int mTransformVersion
    mTransformVersion: i32,
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
    // Java: TransformMapEntry()
    pub fn new() -> Self {
      // Java: super();
      TransformMapEntry {
          mTransformType: 1,
          mTransformVersion: 1,
          mTransformName: String::from("Wps Personal Security Document"),
          mTransformSource: String::from("WpsContent"),
          mTransformItemType: 1,
          mTransformItemName: String::from("WpsSecurityInfo"),
      }
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return 12 + Tool.getUNICODELPP4("{64419DEF-FF3A-40CB-A659-B9CD7E922CBD}") + Tool.getUNICODELPP4("Wps Personal Security Document") + Tool.getUNICODELPP4("WpsContent") + 4 + Tool.getUNICODELPP4("WpsSecurityInfo");
      12 + Tool::getUNICODELPP4_size("{64419DEF-FF3A-40CB-A659-B9CD7E922CBD}")
         + Tool::getUNICODELPP4_size("Wps Personal Security Document")
         + Tool::getUNICODELPP4_size("WpsContent")
         + 4
         + Tool::getUNICODELPP4_size("WpsSecurityInfo")
    }

    // Java: void write(LittleEndianByteArrayOutputStream outputStream)
    pub fn write(&self, _outputStream: &mut dyn std::any::Any) {
      // Java: outputStream.writeInt(this.getSize());
      // Java: outputStream.writeInt(1);
      // Java: outputStream.writeInt(1);
      // Java: Tool.writeUNICODELPP4(outputStream, "{64419DEF-FF3A-40CB-A659-B9CD7E922CBD}");
      // Java: Tool.writeUNICODELPP4(outputStream, "Wps Personal Security Document");
      // Java: Tool.writeUNICODELPP4(outputStream, "WpsContent");
      // Java: outputStream.writeInt(1);
      // Java: Tool.writeUNICODELPP4(outputStream, "WpsSecurityInfo");
      todo!()
    }

}
