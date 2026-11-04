// Java: import org.xml.sax.Attributes
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public interface INamesImporter
pub trait INamesImporter {
    // Java: abstract void importNames()
    fn importNames(&self);

    // Java: abstract void importName(Attributes var1)
    fn importName(&self, var1: &dyn XmlRoAttr);

}
