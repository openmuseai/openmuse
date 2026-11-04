use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::{app::{KmoBook::KmoBook,xf::KmoXF::KmoXF},xlsxw::label::Label::Label}};
use super::{StyleSheetLabel::StyleSheetLabel,CellXfsLabel::CellXfsLabel};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class CellStyleXfsLabel implements Label {
pub struct CellStyleXfsLabel{
    // java private KmoBook book;
    book:Rc<KmoBook>,
    // java private StyleSheetLabel ssLabel;
    ssLabel:Weak<StyleSheetLabel>,
    // java private Vector<KmoXF> styleXfList;
    styleXfList:RefCell<Vec<Option<Rc<KmoXF>>>>,
    // java private Vector<Integer> styleXfIndexList;
    styleXfIndexList:RefCell<Vec<i32>>,
}
impl CellStyleXfsLabel{
    // java CellStyleXfsLabel(StyleSheetLabel ssLabel, KmoBook book) { this.book = book; this.ssLabel = ssLabel; this.styleXfList = new Vector(); this.styleXfIndexList = new Vector(); }
    pub(crate) fn CellStyleXfsLabel(ssLabel:Weak<StyleSheetLabel>,book:Rc<KmoBook>)->Self{Self{book,ssLabel,styleXfList:RefCell::new(Vec::new()),styleXfIndexList:RefCell::new(Vec::new())}}
    // java public int addForWrite(KmoXF style, int index) {
    pub fn addForWrite(&self,style:Option<Rc<KmoXF>>,index:i32)->i32{
        // java for(int i = 0; i < this.styleXfList.size(); ++i) { if (index == (Integer)this.styleXfIndexList.get(i)) { return i; } }
        for i in 0..self.styleXfList.borrow().len(){if index==self.styleXfIndexList.borrow()[i]{return i as i32;}}
        // java this.styleXfList.add(style); this.styleXfIndexList.add(index); return this.styleXfList.size() - 1;
        self.styleXfList.borrow_mut().push(style);self.styleXfIndexList.borrow_mut().push(index);self.styleXfList.borrow().len()as i32-1
    }
    // java int size() { return this.styleXfList.size(); }
    pub(crate) fn size(&self)->i32{self.styleXfList.borrow().len()as i32}
    // java KmoXF getXfAt(int index) { return index >= 0 && index < this.styleXfList.size() ? (KmoXF)this.styleXfList.get(index) : null; }
    pub(crate) fn getXfAt(&self,index:i32)->Option<Rc<KmoXF>>{if index>=0&&index<self.size(){self.styleXfList.borrow()[index as usize].clone()}else{None}}
    // java int getXfIndexAt(int index) { return index >= 0 && index < this.styleXfList.size() ? (Integer)this.styleXfIndexList.get(index) : -1; }
    pub(crate) fn getXfIndexAt(&self,index:i32)->i32{if index>=0&&index<self.size(){self.styleXfIndexList.borrow()[index as usize]}else{-1}}
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java writer.startElement("cellStyleXfs"); int size = this.styleXfList.size(); writer.addAttr("count", size);
        writer.startElement("cellStyleXfs");let size=self.styleXfList.borrow().len()as i32;writer.addAttr__i32("count",size);
        // java for(int index = 0; index < size; ++index) { this.writeCellXf(writer, index); } writer.endElement("cellStyleXfs");
        for index in 0..size{self.writeCellXf(writer,index);}writer.endElement("cellStyleXfs");
    }
    // java private void writeCellXf(XmlWriter writer, int index) {
    fn writeCellXf(&self,writer:&mut dyn XmlWriter,index:i32){
        // java KmoXF format = (KmoXF)this.styleXfList.get(index); short numFmtId = format.getFormatIndex();
        let format=self.styleXfList.borrow()[index as usize].clone().expect("NullPointerException");let numFmtId=format.getFormatIndex();let ssLabel=self.ssLabel.upgrade().expect("StyleSheetLabel owner");
        // java this.ssLabel.getNumFmtsLabel().addForWrite(numFmtId);
        ssLabel.getNumFmtsLabel().borrow_mut().addForWrite(numFmtId);
        // java int borderId = this.ssLabel.getBordersLabel().addForWriter(format.getBorderFormatting());
        let borderId=ssLabel.getBordersLabel().borrow_mut().addForWriter(Some(format.getBorderFormatting()));
        // java int fillId = this.ssLabel.getFillsLabel().addForWrite(format.getPatternFormating());
        let fillId=ssLabel.getFillsLabel().borrow_mut().addForWrite(Some(format.getPatternFormating()));
        // java int id = format.getFontIndex(); KmoFont kmoFont = this.book.getXFManager().getFont(id);
        let id=format.getFontIndex()as i32;let kmoFont=self.book.getXFManager().borrow().getFont(id);
        // java int fontId = this.ssLabel.getFontsLabel().addForWrite(kmoFont, id);
        let fontId=ssLabel.getFontsLabel().borrow_mut().addForWrite(Some(kmoFont),id);
        // java writer.startElement("xf"); writer.addAttr("numFmtId", numFmtId); writer.addAttr("fontId", fontId); writer.addAttr("fillId", fillId); writer.addAttr("borderId", borderId);
        writer.startElement("xf");writer.addAttr__i32("numFmtId",numFmtId as i32);writer.addAttr__i32("fontId",fontId);writer.addAttr__i32("fillId",fillId);writer.addAttr__i32("borderId",borderId);
        // java if (format.get123Prefix()) { writer.addAttr("quotePrefix", true); }
        if format.get123Prefix(){writer.addAttr__bool("quotePrefix",true);}
        // java CellXfsLabel.writeAlignment(writer, format); CellXfsLabel.writeProtection(writer, format); writer.endElement("xf");
        CellXfsLabel::writeAlignment(writer,Some(&format));CellXfsLabel::writeProtection(writer,Some(&format));writer.endElement("xf");
    }
    // java public String toString() {
    pub fn toString(&self)->String{
        // java StringBuffer buffer = new StringBuffer(); buffer.append("<cellStyleXfs count="); buffer.append(this.styleXfList.size()); buffer.append("/>\n");
        let mut buffer=format!("<cellStyleXfs count={}/>\n",self.styleXfList.borrow().len());
        // java for(int index = 0; index < this.styleXfList.size(); ++index) { buffer.append(((KmoXF)this.styleXfList.get(index)).toString()); }
        for index in 0..self.styleXfList.borrow().len(){buffer.push_str(&self.styleXfList.borrow()[index].as_ref().expect("NullPointerException").toString());}
        // java return buffer.toString();
        buffer
    }
}
impl Label for CellStyleXfsLabel{fn write(&self,w:&mut dyn XmlWriter){CellStyleXfsLabel::write(self,w)}}
