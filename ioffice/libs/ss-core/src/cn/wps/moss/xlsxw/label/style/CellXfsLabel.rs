use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::{app::{KmoBook::KmoBook,xf::KmoXF::KmoXF},xlsxw::label::Label::Label}};
use super::StyleSheetLabel::StyleSheetLabel;
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public class CellXfsLabel implements Label {
pub struct CellXfsLabel{
    // java private Vector<KmoXF> cellXfList;
    cellXfList:RefCell<Vec<Option<Rc<KmoXF>>>>,
    // java private Map<Integer, Integer> cellXfMap;
    cellXfMap:RefCell<std::collections::HashMap<i32,i32>>,
    // java private StyleSheetLabel ssLabel;
    ssLabel:Weak<StyleSheetLabel>,
    // java private KmoBook book;
    book:Rc<KmoBook>,
    // java private KmoXF format;
    format:RefCell<Option<Rc<KmoXF>>>,
}
impl CellXfsLabel{
    // java private static String[] typeListHorizontal = new String[8]; private static String[] typeListVertical;
    const typeListHorizontal:[&'static str;8]=["general","left","center","right","fill","justify","centerContinuous","distributed"];
    const typeListVertical:[&'static str;5]=["top","center","bottom","justify","distributed"];
    // java CellXfsLabel(StyleSheetLabel ssLabel, KmoBook book) {
    pub(crate) fn CellXfsLabel(ssLabel:Weak<StyleSheetLabel>,book:Rc<KmoBook>)->Self{
        // java this.ssLabel = ssLabel; this.book = book; this.cellXfList = new Vector(); this.cellXfMap = new HashMap();
        let cellXfList=RefCell::new(Vec::new());let cellXfMap=RefCell::new(std::collections::HashMap::new());
        // java this.cellXfMap.put(15, 0); this.cellXfList.add(book.getXFManager().getXF(15));
        cellXfMap.borrow_mut().insert(15,0);cellXfList.borrow_mut().push(book.getXFManager().borrow().getXF(15));
        Self{cellXfList,cellXfMap,ssLabel,book,format:RefCell::new(None)}
    }
    // java public int addForWrite(KmoXF style, int index) {
    pub fn addForWrite(&self,style:Option<Rc<KmoXF>>,index:i32)->i32{
        // java if (this.cellXfMap.containsKey(index)) { return (Integer)this.cellXfMap.get(index); }
        if self.cellXfMap.borrow().contains_key(&index){return self.cellXfMap.borrow()[&index];}
        // java else { this.cellXfList.add(style); this.cellXfMap.put(index, this.cellXfList.size() - 1); return this.cellXfList.size() - 1; }
        self.cellXfList.borrow_mut().push(style);self.cellXfMap.borrow_mut().insert(index,self.cellXfList.borrow().len()as i32-1);self.cellXfList.borrow().len()as i32-1
    }
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java writer.startElement("cellXfs"); int size = this.cellXfList.size(); writer.addAttr("count", size);
        writer.startElement("cellXfs");let size=self.cellXfList.borrow().len()as i32;writer.addAttr__i32("count",size);
        // java for(int index = 0; index < size; ++index) { this.writeCellXf(writer, index); }
        for index in 0..size{self.writeCellXf(writer,index);}
        // java writer.endElement("cellXfs");
        writer.endElement("cellXfs");
    }
    // java private void writeCellXf(XmlWriter writer, int index) {
    fn writeCellXf(&self,writer:&mut dyn XmlWriter,index:i32){
        // java this.format = (KmoXF)this.cellXfList.get(index); if (null != this.format) {
        *self.format.borrow_mut()=self.cellXfList.borrow()[index as usize].clone();let format=self.format.borrow().clone();if let Some(format)=format{
            let ssLabel=self.ssLabel.upgrade().expect("StyleSheetLabel owner");
            // java short numFmtId = this.format.getFormatIndex(); this.ssLabel.getNumFmtsLabel().addForWrite(numFmtId);
            let numFmtId=format.getFormatIndex();ssLabel.getNumFmtsLabel().borrow_mut().addForWrite(numFmtId);
            // java boolean applyNumFmt = this.format.getAttrFormat(); int id = this.format.getFontIndex();
            let applyNumFmt=format.getAttrFormat();let mut id=format.getFontIndex()as i32;
            // java KmoFont kmoFont = this.book.getXFManager().getFont(id); int fontId = this.ssLabel.getFontsLabel().addForWrite(kmoFont, id);
            let kmoFont=self.book.getXFManager().borrow().getFont(id);let fontId=ssLabel.getFontsLabel().borrow_mut().addForWrite(Some(kmoFont),id);
            // java boolean applyFont = this.format.getAttrFont(); int fillId = this.ssLabel.getFillsLabel().addForWrite(this.format.getPatternFormating());
            let applyFont=format.getAttrFont();let fillId=ssLabel.getFillsLabel().borrow_mut().addForWrite(Some(format.getPatternFormating()));
            // java boolean applyFill = this.format.getAttrPattern(); int borderId = this.ssLabel.getBordersLabel().addForWriter(this.format.getBorderFormatting());
            let applyFill=format.getAttrPattern();let borderId=ssLabel.getBordersLabel().borrow_mut().addForWriter(Some(format.getBorderFormatting()));
            // java boolean applyBorder = this.format.getAttrBorder(); id = this.format.getParentIndex();
            let applyBorder=format.getAttrBorder();id=format.getParentIndex()as i32;
            // java KmoXF kmoXF = this.book.getXFManager().getXF(id); int xfId;
            let kmoXF=self.book.getXFManager().borrow().getXF(id);
            // java if (kmoXF == null) { xfId = 0; } else { xfId = this.ssLabel.getCellStyleXfsLabel().addForWrite(kmoXF, id); }
            let xfId=if let Some(kmoXF)=kmoXF{ssLabel.getCellStyleXfsLabel().borrow().addForWrite(Some(kmoXF),id)}else{0};
            // java writer.startElement("xf"); writer.addAttr("numFmtId", numFmtId); writer.addAttr("fontId", fontId); writer.addAttr("fillId", fillId); writer.addAttr("borderId", borderId); writer.addAttr("xfId", xfId);
            writer.startElement("xf");writer.addAttr__i32("numFmtId",numFmtId as i32);writer.addAttr__i32("fontId",fontId);writer.addAttr__i32("fillId",fillId);writer.addAttr__i32("borderId",borderId);writer.addAttr__i32("xfId",xfId);
            // java if (applyNumFmt) { writer.addAttr("applyNumberFormat", true); }
            if applyNumFmt{writer.addAttr__bool("applyNumberFormat",true);}
            // java if (applyFont) { writer.addAttr("applyFont", true); }
            if applyFont{writer.addAttr__bool("applyFont",true);}
            // java if (applyFill) { writer.addAttr("applyFill", true); }
            if applyFill{writer.addAttr__bool("applyFill",true);}
            // java if (applyBorder) { writer.addAttr("applyBorder", true); }
            if applyBorder{writer.addAttr__bool("applyBorder",true);}
            // java if (this.applyAlignment(this.format)) { writer.addAttr("applyAlignment", true); }
            if self.applyAlignment(&format){writer.addAttr__bool("applyAlignment",true);}
            // java if (this.format.get123Prefix()) { writer.addAttr("quotePrefix", true); }
            if format.get123Prefix(){writer.addAttr__bool("quotePrefix",true);}
            // java writeAlignment(writer, this.format); writeProtection(writer, this.format); writer.endElement("xf");
            Self::writeAlignment(writer,Some(&format));Self::writeProtection(writer,Some(&format));writer.endElement("xf");
        }
    }
    // java private boolean applyAlignment(KmoXF format) {
    fn applyAlignment(&self,format:&KmoXF)->bool{
        // java return !getTypeHorizontal(format.getAlignment()).equals("general") || !getTypeVertical(format.getVerticalAlignment()).equals("center") || format.getWrapText() || format.getIndent() != 0 || format.getShrinkToFit() || format.getRotation() != 0;
        Self::getTypeHorizontal(Some(format.getAlignment())).expect("NullPointerException")!="general"||Self::getTypeVertical(Some(format.getVerticalAlignment())).expect("NullPointerException")!="center"||format.getWrapText()||format.getIndent()!=0||format.getShrinkToFit()||format.getRotation()!=0
    }
    // java public static void writeProtection(XmlWriter writer, KmoXF format) {
    pub fn writeProtection(writer:&mut dyn XmlWriter,format:Option<&KmoXF>){
        let format=format.expect("NullPointerException");
        // java if (!format.isLocked() || format.isHidden()) { writer.startElement("protection"); writer.addAttr("locked", format.isLocked()); writer.addAttr("hidden", format.isHidden()); writer.endElement("protection"); }
        if !format.isLocked()||format.isHidden(){writer.startElement("protection");writer.addAttr__bool("locked",format.isLocked());writer.addAttr__bool("hidden",format.isHidden());writer.endElement("protection");}
    }
    // java public static void writeAlignment(XmlWriter writer, KmoXF format) {
    pub fn writeAlignment(writer:&mut dyn XmlWriter,format:Option<&KmoXF>){
        // java writer.startElement("alignment"); String str = getTypeHorizontal(format.getAlignment());
        writer.startElement("alignment");let format=format.expect("NullPointerException");let mut str=Self::getTypeHorizontal(Some(format.getAlignment()));
        // java if (format.getAlignment() != 0) { writer.addAttr("horizontal", str); }
        if format.getAlignment()!=0{writer.addAttrNullable(Some("horizontal"),str);}
        // java str = getTypeVertical(format.getVerticalAlignment()); if (str != null) { writer.addAttr("vertical", str); }
        str=Self::getTypeVertical(Some(format.getVerticalAlignment()));if let Some(str)=str{writer.addAttr("vertical",str);}
        // java if (format.getWrapText()) { writer.addAttr("wrapText", true); }
        if format.getWrapText(){writer.addAttr__bool("wrapText",true);}
        // java int v; if ((v = format.getIndent()) != 0) { writer.addAttr("indent", v); }
        let mut v=format.getIndent()as i32;if v!=0{writer.addAttr__i32("indent",v);}
        // java if (format.getShrinkToFit()) { writer.addAttr("shrinkToFit", true); }
        if format.getShrinkToFit(){writer.addAttr__bool("shrinkToFit",true);}
        // java v = format.getRotation(); if (v < 0) { v = 90 - v; } if (v != 0) { writer.addAttr("textRotation", v); }
        v=format.getRotation()as i32;if v<0{v=90-v;}if v!=0{writer.addAttr__i32("textRotation",v);}
        // java v = format.getReadingOrder(); if (v != 0) { writer.addAttr("readingOrder", v); }
        v=format.getReadingOrder()as i32;if v!=0{writer.addAttr__i32("readingOrder",v);}
        // java writer.endElement("alignment");
        writer.endElement("alignment");
    }
    // java public static String getTypeHorizontal(Short value) { return value >= 0 && value < 8 ? typeListHorizontal[value] : null; }
    pub fn getTypeHorizontal(value:Option<i16>)->Option<&'static str>{let value=value.expect("NullPointerException");if value>=0&&value<8{Some(Self::typeListHorizontal[value as usize])}else{None}}
    // java public static String getTypeVertical(Short value) { return value >= 0 && value < typeListVertical.length ? typeListVertical[value] : null; }
    pub fn getTypeVertical(value:Option<i16>)->Option<&'static str>{let value=value.expect("NullPointerException");if value>=0&&(value as usize)<Self::typeListVertical.len(){Some(Self::typeListVertical[value as usize])}else{None}}
    // java static { typeListHorizontal[0..7] = general,left,center,right,fill,justify,centerContinuous,distributed;
    // java typeListVertical[0..4] = top,center,bottom,justify,distributed; }
}
impl Label for CellXfsLabel{fn write(&self,w:&mut dyn XmlWriter){CellXfsLabel::write(self,w)}}
