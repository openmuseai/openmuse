#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IDocumentImporter

// Java field: int HEADER_EVEN
pub const HEADER_EVEN: i32 = 0;
// Java field: int HEADER_ODD
pub const HEADER_ODD: i32 = 1;
// Java field: int FOOTER_EVEN
pub const FOOTER_EVEN: i32 = 2;
// Java field: int FOOTER_ODD
pub const FOOTER_ODD: i32 = 3;
// Java field: int HEADER_FIRST
pub const HEADER_FIRST: i32 = 4;
// Java field: int FOOTER_FIRST
pub const FOOTER_FIRST: i32 = 5;

// Java: public interface IXml03Importer extends IDocumentImporter
pub trait IXml03Importer {
    // Java: abstract void initHeaderFooterNode(int var1)
    fn initHeaderFooterNode(&self, _var1: i32);

    // Java: abstract void onImportPicBinData(String var1, byte[] var2, String var3)
    fn onImportPicBinData(&self, _var1: &str, _var2: &[u8], _var3: &str);

    // Java: abstract void onImportOleBinData(String var1, byte[] var2, String var3)
    fn onImportOleBinData(&self, _var1: &str, _var2: &[u8], _var3: &str);

    // Java: abstract void onImportExtendedProperties(int var1, String var2)
    fn onImportExtendedProperties(&self, _var1: i32, _var2: &str);

    // Java: abstract void onImportCustomPropertyEnd(int var1, String var2, String var3, String var4)
    fn onImportCustomPropertyEnd(&self, _var1: i32, _var2: &str, _var3: &str, _var4: &str);
}
