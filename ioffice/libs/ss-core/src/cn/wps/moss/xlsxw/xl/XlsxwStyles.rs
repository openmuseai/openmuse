use crate::{java_runtime::Rc,cn::wps::moss::{app::KmoBook::KmoBook,xlsxw::{label::style::StyleSheetLabel::StyleSheetLabel,util::XlsxwPackage::XlsxwPackage}}};
use kfc_kfc_alpha::kfc::{opcw::OpcwRelationships::OpcwRelationships,xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
// java public class XlsxwStyles {
pub struct XlsxwStyles{
    // java private OpcwRelationships mRelses;
    mRelses:std::rc::Rc<std::cell::RefCell<OpcwRelationships>>,
    // java private StyleSheetLabel styleLabel;
    styleLabel:Rc<StyleSheetLabel>,
}
impl XlsxwStyles{
    // java XlsxwStyles(KmoBook book, OpcwRelationships relses) { this.mRelses = relses; this.styleLabel = new StyleSheetLabel(book); }
    pub(crate) fn XlsxwStyles(book:Rc<KmoBook>,relses:std::rc::Rc<std::cell::RefCell<OpcwRelationships>>)->Self{Self{mRelses:relses,styleLabel:StyleSheetLabel::StyleSheetLabel(book)}}
    // java public StyleSheetLabel getStyleLabel() { return this.styleLabel; }
    pub fn getStyleLabel(&self)->Rc<StyleSheetLabel>{self.styleLabel.clone()}
    // java public boolean write() {
    pub fn write(&self)->bool{
        // java KmoLog.logPoint(); OpcwPackagePart stylesPart = XlsxwPackage.createStylesPart(this.mRelses);
        let stylesPart=XlsxwPackage::createStylesPart(&mut self.mRelses.borrow_mut());
        // java OutputStream stream = XlsxwPackage.getOutputStream(stylesPart); XmlWriter writer = new BufferXmlWriter(stream);
        let stream=stylesPart.withBaseMut(XlsxwPackage::getOutputStream);let mut writer=BufferXmlWriter::new(Box::new(stream.expect("NullPointerException")));
        // java writer.startDocument(); writer.startElement("styleSheet"); writer.addNS((String)null, "http://schemas.openxmlformats.org/spreadsheetml/2006/main");
        writer.startDocument();writer.startElement("styleSheet");writer.addNS(None,"http://schemas.openxmlformats.org/spreadsheetml/2006/main");
        // java this.styleLabel.write(writer); writer.endElement("styleSheet"); writer.endDocument(); KmoLog.logPoint(); return true;
        self.styleLabel.write(&mut writer);writer.endElement("styleSheet");writer.endDocument();true
    }
}
