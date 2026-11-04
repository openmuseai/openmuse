use std::sync::{OnceLock, Mutex};

// Java field: int RECORD_BEGIN
pub(crate) const RECORD_BEGIN: i32 = 0;
// Java field: int RECORD_END
pub(crate) const RECORD_END: i32 = 1;
// Java field: int RECORD_ATTR
pub(crate) const RECORD_ATTR: i32 = 2;
// Java field: int RECORD_ATTR_INT
pub(crate) const RECORD_ATTR_INT: i32 = 3;
// Java field: int RECORD_ATTR_UBYTE
pub(crate) const RECORD_ATTR_UBYTE: i32 = 4;
// Java field: int RECORD_TEXT
pub(crate) const RECORD_TEXT: i32 = 5;
// Java field: int RECORD_TEXT_INT
pub(crate) const RECORD_TEXT_INT: i32 = 6;
// Java field: int RECORD_TEXT_UBYTE
pub(crate) const RECORD_TEXT_UBYTE: i32 = 7;
// Java field: int VALUE_LEN_BYTE
pub(crate) const VALUE_LEN_BYTE: i32 = 0;
// Java field: int VALUE_LEN_SHORT
pub(crate) const VALUE_LEN_SHORT: i32 = 1;
// Java field: int VALUE_LEN_INT
pub(crate) const VALUE_LEN_INT: i32 = 2;
// Java field: int INTER_OK
pub(crate) const INTER_OK: i32 = 0;
// Java field: int INTER_UNKNOWN
pub(crate) const INTER_UNKNOWN: i32 = 1;
// Java field: int INTER_INTERFACE
pub(crate) const INTER_INTERFACE: i32 = 2;
// Java field: int INTER_LINK_ERROR
pub(crate) const INTER_LINK_ERROR: i32 = 3;
// Java field: int INTER_LOAD_ERROR
pub(crate) const INTER_LOAD_ERROR: i32 = 4;
// Java field: boolean bLoadLibrary
static B_LOAD_LIBRARY: OnceLock<Mutex<bool>> = OnceLock::new();

fn get_bLoadLibrary() -> &'static Mutex<bool> {
    B_LOAD_LIBRARY.get_or_init(|| Mutex::new(false))
}

// Java: public class CppXmlReader
pub struct CppXmlReader {
}

// Java: public class CppXmlReader
impl CppXmlReader {
    // Java: CppXmlReader()
    pub fn new() -> Self {
      // Java: super();
      CppXmlReader {}
    }

    // Java: static private static native int convertXmlToMiddleLayer(String var0, int var1, int var2, CppXmlInteraction var3)
    fn convertXmlToMiddleLayer(_var0: &str, _var1: i32, _var2: i32, _var3: &dyn std::any::Any) -> i32 {
        todo!()
    }

    // Java: static private static native int sheetMIndex(String var0, CppSheetInteraction var1)
    fn sheetMIndex(_var0: &str, _var1: &dyn std::any::Any) -> i32 {
        todo!()
    }

    // Java: static private static native int stringsMIndex(String var0, CppStringsInteraction var1)
    fn stringsMIndex(_var0: &str, _var1: &dyn std::any::Any) -> i32 {
        todo!()
    }

    // Java: static int convertXmlToMiddleLayerX(String srcPath, int beginPos, int endPos, CppXmlInteraction inter)
    pub fn convertXmlToMiddleLayerX(_srcPath: &str, _beginPos: i32, _endPos: i32, _inter: &dyn std::any::Any) -> i32 {
      // Java: if (!bLoadLibrary)
      let loaded = *get_bLoadLibrary().lock().unwrap();
      if !loaded
      {
         // Java: return 4;
         return 4;
      }
      // Java: else
      else
      {
         // Java: try
         // Java: return convertXmlToMiddleLayer(srcPath, beginPos, endPos, inter);
         // Java: catch (UnsatisfiedLinkError e)
         // Java: e.printStackTrace();
         // Java: return 3;
         todo!()
      }
    }

    // Java: static int sheetMIndexX(String srcPath, CppSheetInteraction inter)
    pub fn sheetMIndexX(_srcPath: &str, _inter: &dyn std::any::Any) -> i32 {
      // Java: if (!bLoadLibrary)
      let loaded = *get_bLoadLibrary().lock().unwrap();
      if !loaded
      {
         // Java: return 4;
         return 4;
      }
      // Java: else
      else
      {
         // Java: try
         // Java: return sheetMIndex(srcPath, inter);
         // Java: catch (UnsatisfiedLinkError e)
         // Java: e.printStackTrace();
         // Java: return 3;
         todo!()
      }
    }

    // Java: static int stringsMIndexX(String srcPath, CppStringsInteraction inter)
    pub fn stringsMIndexX(_srcPath: &str, _inter: &dyn std::any::Any) -> i32 {
      // Java: if (!bLoadLibrary)
      let loaded = *get_bLoadLibrary().lock().unwrap();
      if !loaded
      {
         // Java: return 4;
         return 4;
      }
      // Java: else
      else
      {
         // Java: try
         // Java: return stringsMIndex(srcPath, inter);
         // Java: catch (UnsatisfiedLinkError e)
         // Java: e.printStackTrace();
         // Java: return 3;
         todo!()
      }
    }

}
