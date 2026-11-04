use crate::{java_runtime::Rc,cn::wps::moss::{app::xf::KmoFont::KmoFont,filefmt::biff8::util::ColorToolKit::ColorToolKit,xlsxw::{label::Label::Label,util::ValueHelper::ValueHelper}}};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class FontsLabel implements Label {
pub struct FontsLabel{
    // java private Vector<KmoFont> fontList = new Vector();
    fontList:Vec<Option<Rc<KmoFont>>>,
    // java private Vector<Integer> fontIndexList = new Vector();
    fontIndexList:Vec<i32>,
    // java private static HashMap<Byte, String> underlineMap = new HashMap(5);
}
#[cfg(test)]
#[path="FontsLabel.review.rs"]
mod review;
impl FontsLabel{
    // java FontsLabel() { }
    pub(crate) fn FontsLabel()->Self{Self{fontList:vec![],fontIndexList:vec![]}}
    // java public int addForWrite(KmoFont font, int index) {
    pub fn addForWrite(&mut self,font:Option<Rc<KmoFont>>,index:i32)->i32{
        // java for(int i = 0; i < this.fontList.size(); ++i) {
        for i in 0..self.fontList.len(){
            // java if (index == (Integer)this.fontIndexList.get(i)) { return i; }
            if index==self.fontIndexList[i]{return i as i32;}
        }
        // java this.fontList.add(font); this.fontIndexList.add(index);
        self.fontList.push(font);self.fontIndexList.push(index);
        // java return this.fontList.size() - 1;
        self.fontList.len()as i32-1
    }
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java int fontNum = this.fontList.size();
        let fontNum=self.fontList.len()as i32;
        // java if (fontNum != 0) {
        if fontNum!=0{
            // java writer.startElement("fonts"); writer.addAttr("count", fontNum);
            writer.startElement("fonts");writer.addAttr__i32("count",fontNum);
            // java for(int index = 0; index < fontNum; ++index) { this.writeFont(writer, (KmoFont)this.fontList.get(index)); }
            for index in 0..fontNum{self.writeFont(writer,self.fontList[index as usize].as_deref());}
            // java writer.endElement("fonts");
            writer.endElement("fonts");
        }
    }
    // java private void writeFont(XmlWriter writer, KmoFont font) {
    fn writeFont(&self,writer:&mut dyn XmlWriter,font:Option<&KmoFont>){
        // java if (null != font) {
        if let Some(font)=font{
            // java writer.startElement("font"); writer.startElement("name");
            writer.startElement("font");writer.startElement("name");
            // java writer.addAttr("val", font.getFontName()); writer.endElement("name");
            writer.addAttr("val",&font.getFontName());writer.endElement("name");
            // java writeFontImpl(writer, font); writer.endElement("font");
            Self::writeFontImpl(writer,font);writer.endElement("font");
        }
    }
    // java public static void writeFontImpl(XmlWriter writer, KmoFont font) {
    pub fn writeFontImpl(writer:&mut dyn XmlWriter,font:&KmoFont){
        // java if (font.getBoldWeight() == 700) { writer.startElement("b"); writer.endElement("b"); }
        if font.getBoldWeight()==700{writer.startElement("b");writer.endElement("b");}
        // java if (font.isItalic()) { writer.startElement("i"); writer.endElement("i"); }
        if font.isItalic(){writer.startElement("i");writer.endElement("i");}
        // java if (font.isStrikeout()) { writer.startElement("strike"); writer.endElement("strike"); }
        if font.isStrikeout(){writer.startElement("strike");writer.endElement("strike");}
        // java if (getCharSet(font) != 0) { writer.startElement("charset"); writer.addAttr("val", getCharSet(font)); writer.endElement("charset"); }
        if Self::getCharSet(font)!=0{writer.startElement("charset");writer.addAttr__i32("val",Self::getCharSet(font));writer.endElement("charset");}
        // java byte tmp; if ((tmp = font.getUnderline()) != 0) {
        let tmp=font.getUnderline();if tmp!=0{
            // java writer.startElement("u"); String str = getUnderline(tmp);
            writer.startElement("u");let value=Self::getUnderline(tmp);
            // java if (str != null) { writer.addAttr("val", str); } writer.endElement("u");
            if let Some(value)=value{writer.addAttr("val",value);}writer.endElement("u");
        }
        // java short v; if ((v = font.getSuperSubScript()) != 0) {
        let v=font.getSuperSubScript();if v!=0{
            // java String str; if (v == 1) { str = "superscript"; } else { str = "subscript"; }
            let value=if v==1{"superscript"}else{"subscript"};
            // java writer.startElement("vertAlign"); writer.addAttr("val", str); writer.endElement("vertAlign");
            writer.startElement("vertAlign");writer.addAttr("val",value);writer.endElement("vertAlign");
        }
        // java int height = font.getFontHeight(); writer.startElement("sz"); writer.addAttr("val", height / 20); writer.endElement("sz");
        let height=font.getFontHeight()as i32;writer.startElement("sz");writer.addAttr__i32("val",height/20);writer.endElement("sz");
        // java int indexed = font.getColorPaletteIndex();
        let indexed=font.getColorPaletteIndex();
        // java if (ColorToolKit.isValidColor(indexed)) {
        if ColorToolKit::isValidColor(indexed){
            // java if (ColorToolKit.isPaletteIndex(indexed)) {
            if ColorToolKit::isPaletteIndex(indexed){
                // java writer.startElement("color"); writer.addAttr("indexed", indexed); writer.endElement("color");
                writer.startElement("color");writer.addAttr__i32("indexed",indexed);writer.endElement("color");
            // java } else { writer.startElement("color"); writer.addAttr("rgb", ValueHelper.toHexColor(indexed)); writer.endElement("color"); }
            }else{writer.startElement("color");writer.addAttr("rgb",&ValueHelper::toHexColor(indexed));writer.endElement("color");}
        }
    }
    // java private static int getCharSet(KmoFont font) { byte charset = font.getCharset(); return charset >= 0 ? charset : charset + 256; }
    fn getCharSet(font:&KmoFont)->i32{let charset=font.getCharset()as i32;if charset>=0{charset}else{charset+256}}
    // java public static String getUnderline(byte value) { return value == 1 ? null : (String)underlineMap.get(new Byte(value)); }
    pub fn getUnderline(value:i8)->Option<&'static str>{if value==1{None}else{Self::underlineMap.iter().find(|(k,_)|*k==value).map(|(_,v)|*v)}}
    // java static { underlineMap.put(new Byte((byte)2), "double"); underlineMap.put(new Byte((byte)34), "doubleAccounting"); underlineMap.put(new Byte((byte)0), "none"); underlineMap.put(new Byte((byte)1), "single"); underlineMap.put(new Byte((byte)33), "singleAccounting"); }
    const underlineMap:[(i8,&'static str);5]=[(2,"double"),(34,"doubleAccounting"),(0,"none"),(1,"single"),(33,"singleAccounting")];
}
impl Label for FontsLabel{fn write(&self,writer:&mut dyn XmlWriter){FontsLabel::write(self,writer);}}
