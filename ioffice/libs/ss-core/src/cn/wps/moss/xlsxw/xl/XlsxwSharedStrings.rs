use crate::{java_runtime::Rc,cn::wps::moss::{app::KmoBook::KmoBook,xlsxw::{label::SstLabel::SstLabel,util::XlsxwPackage::XlsxwPackage}}};
use kfc_kfc_alpha::kfc::{opcw::OpcwRelationships::OpcwRelationships,xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
// java public class XlsxwSharedStrings {
pub struct XlsxwSharedStrings{
    // java OpcwRelationships mRelses;
    mRelses:std::rc::Rc<std::cell::RefCell<OpcwRelationships>>,
    // java private SstLabel sstLabel;
    sstLabel:SstLabel,
}
impl XlsxwSharedStrings {
    // java XlsxwSharedStrings(KmoBook book, OpcwRelationships relses) {
    pub(crate) fn XlsxwSharedStrings(book:Rc<KmoBook>,relses:std::rc::Rc<std::cell::RefCell<OpcwRelationships>>)->Self{
        // java this.sstLabel = new SstLabel(book);
        let sstLabel=SstLabel::SstLabel(book);
        // java this.mRelses = relses;
        Self{mRelses:relses,sstLabel}
    }
    // java public boolean write() throws IOException {
    pub fn write(&self)->std::io::Result<bool>{
        // java KmoLog.logPoint(); OpcwPackagePart stringsPart = XlsxwPackage.createStringsPart(this.mRelses);
        let stringsPart=XlsxwPackage::createStringsPart(&mut self.mRelses.borrow_mut());
        // java OutputStream stream = XlsxwPackage.getOutputStream(stringsPart);
        let stream=stringsPart.withBaseMut(XlsxwPackage::getOutputStream);
        // java XmlWriter writer = new BufferXmlWriter(stream, true);
        let mut writer=BufferXmlWriter::new_with_escape(Box::new(stream.expect("NullPointerException")),true);
        // java writer.startDocument();
        writer.startDocument();
        // java writer.startElement("sst");
        writer.startElement("sst");
        // java this.sstLabel.write(writer);
        self.sstLabel.write(&mut writer);
        // java writer.endElement("sst");
        writer.endElement("sst");
        // java writer.endDocument();
        writer.endDocument();
        // java KmoLog.logPoint(); return true;
        Ok(true)
    }
}
