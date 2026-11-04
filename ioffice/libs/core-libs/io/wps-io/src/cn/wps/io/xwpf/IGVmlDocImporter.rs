#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes

use super::IDocumentImporter::IDocumentImporter;

// Java: public interface IGVmlDocImporter extends IDocumentImporter
pub trait IGVmlDocImporter: IDocumentImporter {
    // Java: abstract MutablePropertySet adjustRunProps(MutablePropertySet var1)
    fn adjustRunProps(&self, _var1: &dyn std::any::Any) -> Box<dyn std::any::Any + Send + Sync>;

    // Java: abstract MutablePropertySet adjustParaProps(MutablePropertySet var1)
    fn adjustParaProps(&self, _var1: &dyn std::any::Any) -> Box<dyn std::any::Any + Send + Sync>;

    // Java: abstract void startHighlight(MutablePropertySet var1, int var2, Attributes var3)
    fn startHighlight(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: &dyn std::any::Any);

    // Java: abstract void endHighlight(MutablePropertySet var1, int var2)
    fn endHighlight(&self, _var1: &dyn std::any::Any, _var2: i32);

    // Java: abstract void startSpcElement(MutablePropertySet var1, int var2, Attributes var3)
    fn startSpcElement(&self, _var1: &dyn std::any::Any, _var2: i32, _var3: &dyn std::any::Any);

    // Java: abstract void endLnSpcElement(MutablePropertySet var1, int var2)
    fn endLnSpcElement(&self, _var1: &dyn std::any::Any, _var2: i32);
}
