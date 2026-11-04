use crate::{java_runtime::Rc,cn::wps::moss::{app::KmoBook::KmoBook,xlsxw::label::Label::Label}};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class NumFmtsLabel implements Label {
pub struct NumFmtsLabel{
    // java private KmoBook book;
    book:Rc<KmoBook>,
    // java private List<Short> fmtList;
    fmtList:Option<Vec<i16>>,
}
impl NumFmtsLabel{
    // java NumFmtsLabel(KmoBook book) { this.book = book; }
    pub(crate) fn NumFmtsLabel(book:Rc<KmoBook>)->Self{Self{book,fmtList:None}}
    // java public void addForWrite(short numFmtId) {
    pub fn addForWrite(&mut self,numFmtId:i16){
        // java if (this.fmtList == null) { this.fmtList = new ArrayList(); }
        if self.fmtList.is_none(){self.fmtList=Some(Vec::new());}
        // java for(short tmp : this.fmtList) { if (tmp == numFmtId) { return; } }
        for &tmp in self.fmtList.as_ref().unwrap(){if tmp==numFmtId{return;}}
        // java this.fmtList.add(numFmtId);
        self.fmtList.as_mut().unwrap().push(numFmtId);
    }
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java if (this.fmtList != null && this.fmtList.size() != 0) {
        if let Some(fmtList)=&self.fmtList{if !fmtList.is_empty(){
            // java KmoXFManager kmoXFManager = this.book.getXFManager();
            let kmoXFManager=self.book.getXFManager();
            // java writer.startElement("numFmts"); writer.addAttr("count", this.fmtList.size());
            writer.startElement("numFmts");writer.addAttr__i32("count",fmtList.len()as i32);
            // java for(short fmtId : this.fmtList) {
            for &fmtId in fmtList{
                // java KmoFormat format = kmoXFManager.getFormat(fmtId);
                let format=kmoXFManager.borrow().getFormat(fmtId as i32);
                // java if (format != null) {
                if let Some(format)=format{
                    // java writer.startElement("numFmt"); String str = format.getFormatString();
                    writer.startElement("numFmt");let str=format.getFormatString();
                    // java writer.addAttr("numFmtId", fmtId); writer.addAttr("formatCode", str);
                    writer.addAttr__i32("numFmtId",fmtId as i32);writer.addAttrNullable(Some("formatCode"),str.as_deref());
                    // java writer.endElement("numFmt");
                    writer.endElement("numFmt");
                }
            }
            // java writer.endElement("numFmts");
            writer.endElement("numFmts");
        }}
    }
}
impl Label for NumFmtsLabel{fn write(&self,w:&mut dyn XmlWriter){NumFmtsLabel::write(self,w)}}
