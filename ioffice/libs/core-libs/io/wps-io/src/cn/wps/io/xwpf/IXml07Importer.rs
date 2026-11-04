#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.xwpf.element_handler.SectHdrFdr07
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes

use super::IDocumentImporter::IDocumentImporter;

// Java: public interface IXml07Importer extends IDocumentImporter
pub trait IXml07Importer: IDocumentImporter {
    // Java: abstract void onImportRelationStart(String var1, String var2, String var3, String var4)
    fn onImportRelationStart(&self, _var1: &str, _var2: &str, _var3: &str, _var4: &str);

    // Java: abstract void onImportRawPart(String var1, String var2, String var3)
    fn onImportRawPart(&self, _var1: &str, _var2: &str, _var3: &str);

    // Java: abstract void onImportPartStart(String var1, String var2)
    fn onImportPartStart(&self, _var1: &str, _var2: &str);

    // Java: abstract void onImportPartEnd()
    fn onImportPartEnd(&self);

    // Java: abstract void onImportBinary(int var1, String var2, String var3, String var4, char[] var5, int var6, int var7)
    fn onImportBinary(&self, _var1: i32, _var2: &str, _var3: &str, _var4: &str, _var5: &[char], _var6: i32, _var7: i32);

    // Java: abstract void onImportSectProp(MutablePropertySet var1, SectHdrFdr07 var2)
    fn onImportSectProp_07(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any);

    // Java: abstract void onImportHyperlink(Attributes var1, SubDocType var2)
    fn onImportHyperlink(&self, _var1: &dyn std::any::Any, _var2: i32);

    // Java: abstract void onImportExtendedProperties(int var1, String var2)
    fn onImportExtendedProperties(&self, _var1: i32, _var2: &str);

    // Java: abstract void onImportCustomPropertyStart(Attributes var1)
    fn onImportCustomPropertyStart(&self, _var1: &dyn std::any::Any);

    // Java: abstract void onImportCustomPropertyEnd(int var1, String var2)
    fn onImportCustomPropertyEnd(&self, _var1: i32, _var2: &str);

    // Java: abstract void onImportCustomPropertiesEnd()
    fn onImportCustomPropertiesEnd(&self);
}
