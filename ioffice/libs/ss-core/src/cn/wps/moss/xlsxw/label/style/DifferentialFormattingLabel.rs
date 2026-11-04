use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::{app::{KmoBook::KmoBook,tables::DataFormatting::DataFormatting},filefmt::biff8::{record::cf::{FontFormatting::FontFormatting,BorderFormatting::BorderFormatting,PatternFormatting::PatternFormatting,NumberFormatFormatting::NumberFormatFormatting},util::ColorToolKit::ColorToolKit},xlsxr::model::DifferentialFormatting::DifferentialFormatting,xlsxw::{label::Label::Label,util::ValueHelper::ValueHelper}}};
use super::{FillsLabel::FillsLabel,BordersLabel::BordersLabel,FontsLabel::FontsLabel};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class DifferentialFormattingLabel implements Label {
// Original method slice: ColorFilter and legacy CFRuleRecord overloads await
// their original class dependencies; no guessed registry/formatting objects.
pub struct DifferentialFormattingLabel{
    // java private Vector<DifferentialFormatting> cfList = new Vector();
    cfList:Vec<Rc<RefCell<DifferentialFormatting>>>,
    // java private KmoBook book;
    book:Rc<KmoBook>,
}
#[cfg(test)]
#[path="DifferentialFormattingLabel.review.rs"]
mod review;
impl DifferentialFormattingLabel{
    // java public DifferentialFormattingLabel(KmoBook book) { this.book = book; }
    pub fn DifferentialFormattingLabel(book:Rc<KmoBook>)->Self{Self{cfList:Vec::new(),book}}
    // java public int addForWrite(DataFormatting dataFormatting) { return dataFormatting == null ? -1 : this.addForWrite(dataFormatting.getPatternFormatting(), dataFormatting.getBorderFormatting(), dataFormatting.getFontFormatting(), dataFormatting.getNumberFormatFormatting()); }
    pub fn addForWrite(&mut self,dataFormatting:Option<&DataFormatting>)->i32{match dataFormatting{None=>-1,Some(d)=>self.addForWrite__4(d.getPatternFormatting(),d.getBorderFormatting(),d.getFontFormatting(),d.getNumberFormatFormatting())}}
    // java public int addForWrite(DifferentialFormatting df) {
    pub fn addForWrite__df(&mut self,df:Option<Rc<RefCell<DifferentialFormatting>>>)->i32{
        // java if (null == df) { return -1; } else { this.cfList.add(df); return this.cfList.size() - 1; }
        if let Some(df)=df{self.cfList.push(df);self.cfList.len()as i32-1}else{-1}
    }
    // java public int addForWrite(PatternFormatting patternFormatting) {
    pub fn addForWrite__pattern(&mut self,patternFormatting:Option<Rc<RefCell<PatternFormatting>>>)->i32{
        // java DifferentialFormatting df = new DifferentialFormatting(); df.patternFormatting = patternFormatting;
        let mut df=DifferentialFormatting::DifferentialFormatting();df.patternFormatting=patternFormatting;
        // java this.cfList.add(df); return this.cfList.size() - 1;
        self.cfList.push(Rc::new(RefCell::new(df)));self.cfList.len()as i32-1
    }
    // java public int addForWrite(PatternFormatting pattern, BorderFormatting border, FontFormatting font, NumberFormatFormatting numFmt) {
    pub fn addForWrite__4(&mut self,pattern:Option<Rc<RefCell<PatternFormatting>>>,border:Option<Rc<RefCell<BorderFormatting>>>,font:Option<Rc<FontFormatting>>,numFmt:Option<Rc<NumberFormatFormatting>>)->i32{
        // java DifferentialFormatting df = new DifferentialFormatting();
        let mut df=DifferentialFormatting::DifferentialFormatting();
        // java if (pattern != null) { df.patternFormatting = pattern; }
        if pattern.is_some(){df.patternFormatting=pattern;}
        // java if (border != null) { df.borderFormatting = border; }
        if border.is_some(){df.borderFormatting=border;}
        // java if (font != null) { df.fontFormatting = font; }
        if font.is_some(){df.fontFormatting=font;}
        // java if (numFmt != null) { df.setNumberFormatFormatting(numFmt); }
        if numFmt.is_some(){df.setNumberFormatFormatting(numFmt);}
        // java this.cfList.add(df); return this.cfList.size() - 1;
        self.cfList.push(Rc::new(RefCell::new(df)));self.cfList.len()as i32-1
    }
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java writer.startElement("dxfs"); writer.addAttr("count", this.cfList.size());
        writer.startElement("dxfs");writer.addAttr__i32("count",self.cfList.len()as i32);
        // java for(int index = 0; index < this.cfList.size(); ++index) {
        for index in 0..self.cfList.len(){
            // java writer.startElement("dxf"); this.writeDxf(writer, (DifferentialFormatting)this.cfList.get(index)); writer.endElement("dxf");
            writer.startElement("dxf");self.writeDxf(writer,&self.cfList[index].borrow());writer.endElement("dxf");
        }
        // java writer.endElement("dxfs");
        writer.endElement("dxfs");
    }
    // java private void writeDxf(XmlWriter writer, DifferentialFormatting df) {
    fn writeDxf(&self,writer:&mut dyn XmlWriter,df:&DifferentialFormatting){
        // java FontFormatting font = df.fontFormatting; if (font != null) { this.writeFont(writer, font); }
        let font=df.fontFormatting.clone();if let Some(font)=font{self.writeFont(writer,&font);}
        // java NumberFormatFormatting numFmt = df.numberFormatFormatting(); if (numFmt != null) { this.writeNumFmt(writer, numFmt); }
        let numFmt=df.numberFormatFormatting();if let Some(numFmt)=numFmt{self.writeNumFmt(writer,&numFmt);}
        // java PatternFormatting pattern = df.patternFormatting; if (pattern != null) { FillsLabel.writeFill(writer, pattern, true); }
        let pattern=df.patternFormatting.clone();if let Some(pattern)=pattern{FillsLabel::writeFill(writer,Some(&pattern.borrow()),true);}
        // java BorderFormatting border = df.borderFormatting; if (border != null) { BordersLabel.writeBorder(writer, border); }
        let border=df.borderFormatting.clone();if let Some(border)=border{BordersLabel::writeBorder(writer,Some(&border.borrow()));}
    }
    // java private void writeNumFmt(XmlWriter writer, NumberFormatFormatting numFmt) {
    fn writeNumFmt(&self,writer:&mut dyn XmlWriter,numFmt:&NumberFormatFormatting){
        // java String fmtCode = numFmt.fmtCode();
        let mut fmtCode=numFmt.fmtCode();
        // java if (fmtCode == null) {
        if fmtCode.is_none(){
            // java KmoXFManager xfManager = this.book.getXFManager(); KmoFormat xf = xfManager.getFormat(numFmt.numFmtId());
            let xfManager=self.book.getXFManager();let xf=xfManager.borrow().getFormat(numFmt.numFmtId());
            // java if (xf == null) { return; }
            let Some(xf)=xf else{return;};
            // java fmtCode = xf.getFormatString();
            fmtCode=xf.getFormatString();
        }
        // java if (fmtCode != null) {
        if let Some(fmtCode)=fmtCode{
            // java writer.startElement("numFmt"); int numFmtId = numFmt.numFmtId();
            writer.startElement("numFmt");let numFmtId=numFmt.numFmtId();
            // java writer.addAttr("numFmtId", numFmtId); writer.addAttr("formatCode", fmtCode); writer.endElement("numFmt");
            writer.addAttr__i32("numFmtId",numFmtId);writer.addAttr("formatCode",&fmtCode);writer.endElement("numFmt");
        }
    }
    // java private void writeFont(XmlWriter writer, FontFormatting font) {
    fn writeFont(&self,writer:&mut dyn XmlWriter,font:&FontFormatting){
        // java writer.startElement("font");
        writer.startElement("font");
        // java if (font.hasItalic()) {
        if font.hasItalic(){
            // java if (font.isItalic()) { writer.startElement("i"); writer.endElement("i"); }
            if font.isItalic(){writer.startElement("i");writer.endElement("i");}
            // java else { writer.startElement("i"); writer.addAttr("val", 0); writer.endElement("i"); }
            else{writer.startElement("i");writer.addAttr__i32("val",0);writer.endElement("i");}
        }
        // java if (font.hasBold()) {
        if font.hasBold(){
            // java if (font.isBold()) { writer.startElement("b"); writer.endElement("b"); }
            if font.isBold(){writer.startElement("b");writer.endElement("b");}
            // java else { writer.startElement("b"); writer.addAttr("val", 0); writer.endElement("b"); }
            else{writer.startElement("b");writer.addAttr__i32("val",0);writer.endElement("b");}
        }
        // java if (font.getFontHeight() / 20 != 0) { writer.startElement("sz"); writer.addAttr("val", font.getFontHeight() / 20); writer.endElement("sz"); }
        if font.getFontHeight()/20!=0{writer.startElement("sz");writer.addAttr__i32("val",font.getFontHeight()/20);writer.endElement("sz");}
        // java short v; if ((v = font.getEscapementType()) != 0) {
        let v=font.getEscapementType();if v!=0{
            // java if (v == 1) { writer.startElement("vertAlign"); writer.addAttr("val", "superscript"); writer.endElement("vertAlign"); }
            if v==1{writer.startElement("vertAlign");writer.addAttr("val","superscript");writer.endElement("vertAlign");}
            // java else if (v == 2) { writer.startElement("vertAlign"); writer.addAttr("val", "subscript"); writer.endElement("vertAlign"); }
            else if v==2{writer.startElement("vertAlign");writer.addAttr("val","subscript");writer.endElement("vertAlign");}
        }
        // java if (font.hasColor()) {
        if font.hasColor(){
            // java int indexed = font.getFontColorIndex();
            let indexed=font.getFontColorIndex();
            // java if (ColorToolKit.isValidColor(indexed)) {
            if ColorToolKit::isValidColor(indexed){
                // java writer.startElement("color");
                writer.startElement("color");
                // java if (ColorToolKit.isPaletteIndex(indexed)) { writer.addAttr("indexed", indexed); }
                if ColorToolKit::isPaletteIndex(indexed){writer.addAttr__i32("indexed",indexed);}
                // java else { writer.addAttr("rgb", ValueHelper.toHexColor(indexed)); }
                else{writer.addAttr("rgb",&ValueHelper::toHexColor(indexed));}
                // java writer.endElement("color");
                writer.endElement("color");
            // java } else { writer.startElement("color"); writer.addAttr("auto", 1); writer.endElement("color"); }
            }else{writer.startElement("color");writer.addAttr__i32("auto",1);writer.endElement("color");}
        }
        // java if (font.hasStrike()) {
        if font.hasStrike(){
            // java if (font.isStruckout()) { writer.startElement("strike"); writer.endElement("strike"); }
            if font.isStruckout(){writer.startElement("strike");writer.endElement("strike");}
            // java else { writer.startElement("strike"); writer.addAttr("val", 0); writer.endElement("strike"); }
            else{writer.startElement("strike");writer.addAttr__i32("val",0);writer.endElement("strike");}
        }
        // java if (font.hasUnderline()) {
        if font.hasUnderline(){
            // java if (font.isUnderlineTypeModified()) {
            if font.isUnderlineTypeModified(){
                // java writer.startElement("u"); String str = FontsLabel.getUnderline((byte)font.getUnderlineType());
                writer.startElement("u");let str=FontsLabel::getUnderline(font.getUnderlineType()as i8);
                // java if (str != null) { writer.addAttr("val", str); }
                if let Some(str)=str{writer.addAttr("val",str);}
                // java writer.endElement("u");
                writer.endElement("u");
            // java } else { writer.startElement("u"); writer.addAttr("val", "none"); writer.endElement("u"); }
            }else{writer.startElement("u");writer.addAttr("val","none");writer.endElement("u");}
        }
        // java writer.endElement("font");
        writer.endElement("font");
    }
}
impl Label for DifferentialFormattingLabel{fn write(&self,w:&mut dyn XmlWriter){DifferentialFormattingLabel::write(self,w)}}
