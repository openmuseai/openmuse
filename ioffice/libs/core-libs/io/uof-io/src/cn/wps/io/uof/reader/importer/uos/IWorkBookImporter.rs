// Java: import org.xml.sax.Attributes
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public interface IWorkBookImporter
pub trait IWorkBookImporter {
    // Java: abstract int registerSheet(Attributes var1)
    fn registerSheet(&self, var1: &dyn XmlRoAttr) -> i32;

    // Java: abstract void importWorkBookSetting(Attributes var1)
    fn importWorkBookSetting(&self, var1: &dyn XmlRoAttr);

    // Java: abstract void importSharedString()
    fn importSharedString(&self);

    // Java: abstract void importAllSharedStrings()
    fn importAllSharedStrings(&self);

}
