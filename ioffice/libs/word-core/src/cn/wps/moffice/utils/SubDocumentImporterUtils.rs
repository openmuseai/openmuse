// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.util.KeepNotProguard
use util::util::KeepNotProguard::*;

// java @KeepNotProguard public class SubDocumentImporterUtils
#[derive(Default)]
pub struct SubDocumentImporterUtils {
}

// java @KeepNotProguard public class SubDocumentImporterUtils
impl SubDocumentImporterUtils {
    // java public static int toFieldType(String value)
    pub fn toFieldType(value: &mut str) -> i32 {
      // java IOAssert.assertNotNull("value should not be null!", value);
      IOAssert::assertNotNull__string_object(&"value should not be null!", Some(&value));
      // java KFieldType[] kFieldTypes = KFieldType.values();
      let mut kFieldTypes: Vec<KFieldType> = KFieldType::values();
      // java IOAssert.assertNotNull("kFieldTypes should not be null!", kFieldTypes);
      IOAssert::assertNotNull__string_object(&"kFieldTypes should not be null!", Some(&kFieldTypes));
      // java for(KFieldType kFieldType : kFieldTypes)
      for kFieldType in kFieldTypes.iter() {
        // java String strFieldType = kFieldType.toString().toUpperCase();
        let mut strFieldType: String = kFieldType.toString().toUpperCase();
        // java String newValue = value.toUpperCase();
        let mut newValue: String = value.toUpperCase();
        // java newValue = newValue.trim();
        newValue = newValue.trim();
        // java int index = newValue.indexOf(strFieldType);
        let mut index: i32 = newValue.indexOf(strFieldType);
        // java if (0 == index)
        if 0 == index {
          // java return kFieldType.getFlt();
          return kFieldType.getFlt();
        }
      }
      // java return KFieldType.FieldEmpty.getFlt();
      return KFieldType::FieldEmpty.getFlt();
    }

}
