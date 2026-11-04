// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes

// Java: public interface IVmlImporter extends IImporterBase
pub type VmlImporterRef = std::rc::Rc<std::cell::RefCell<dyn IVmlImporter>>;

pub trait IVmlImporter: crate::io::xwpf::IImporterBase::IImporterBase {
    // Java: abstract IDocumentImporter getDocumentImporter()
    fn getDocumentImporter(&self) -> &dyn std::any::Any;

    // Java: abstract void onImportVmlStart(POIXMLDocumentPart var1, MutablePropertySet var2)
    fn onImportVmlStart(&mut self, var1: &dyn std::any::Any, var2: &dyn std::any::Any);

    // Java: abstract void onImportVmlEnd()
    fn onImportVmlEnd(&mut self);

    // Java: abstract void onImportPictChildStart(int var1, Attributes var2)
    fn onImportPictChildStart(&mut self, var1: i32, var2: &dyn std::any::Any);

    // Java: abstract void onImportPictChildEnd(int var1)
    fn onImportPictChildEnd(&mut self, var1: i32);

    // Java: abstract void onImportEGShapeElementsStart(int var1, String var2, Attributes var3)
    fn onImportEGShapeElementsStart(&mut self, var1: i32, var2: &str, var3: &dyn std::any::Any);

    // Java: abstract void onImportTextboxEnd(boolean var1)
    fn onImportTextboxEnd(&mut self, var1: bool);

    // Java: abstract void onImportWBackgroundStart(POIXMLDocumentPart var1, int var2, Attributes var3)
    fn onImportWBackgroundStart(&mut self, var1: &dyn std::any::Any, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void onImportVBackgroundStart(int var1, String var2, Attributes var3)
    fn onImportVBackgroundStart(&mut self, var1: i32, var2: &str, var3: &dyn std::any::Any);

    // Java: abstract void onImportBackgroundEnd(int var1, String var2)
    fn onImportBackgroundEnd(&mut self, var1: i32, var2: &str);

    // Java: abstract void onImportInk(Attributes var1)
    fn onImportInk(&mut self, var1: &dyn std::any::Any);

    // Java: abstract void onImportOLEObject(Attributes var1)
    fn onImportOLEObject(&mut self, var1: &dyn std::any::Any);

    // Java: abstract void onImportControlObject(Attributes var1)
    fn onImportControlObject(&mut self, var1: &dyn std::any::Any);

}
