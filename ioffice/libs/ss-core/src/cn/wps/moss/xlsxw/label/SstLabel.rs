use crate::{java_runtime::Rc,cn::wps::moss::{app::{KmoBook::KmoBook,xf::KmoFont::KmoFont},filefmt::biff8::record::common::UnicodeString::UnicodeString,xlsxw::label::{Label::Label,style::FontsLabel::FontsLabel}}};
use kfc_kfc_alpha::kfc::xpxml::{writer::XmlWriter::XmlWriter,util::EscapeUtil::{EscapeUtil,CONTROL_CHARS}};
// java public final class SstLabel implements Label {
pub struct SstLabel{
    // java private KmoBook book;
    book:Rc<KmoBook>,
}
#[cfg(test)]
#[path="SstLabel.review.rs"]
mod review;
impl SstLabel{
    // java public static final short NO_FONT = 0;
    pub const NO_FONT:i16=0;
    // java public SstLabel(KmoBook book) { this.book = book; }
    pub fn SstLabel(book:Rc<KmoBook>)->Self{Self{book}}
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java int count = this.getUniqueCount(); KmoLog.logPoint("count " + count);
        let count=self.getUniqueCount();
        // java writer.addAttr("uniqueCount", count); writer.addAttr("count", count);
        writer.addAttr__i32("uniqueCount",count);writer.addAttr__i32("count",count);
        // java writer.addNS((String)null, "http://schemas.openxmlformats.org/spreadsheetml/2006/main");
        writer.addNS(None,"http://schemas.openxmlformats.org/spreadsheetml/2006/main");
        // java Iterator<UnicodeString> iter = this.book.getCellValueStore().getStrStoreIterator();
        let store=self.book.getCellValueStore();let mut store=store.borrow_mut();let mut iter=store.getStrStoreIterator();
        // java while(iter.hasNext()) { UnicodeString string = (UnicodeString)iter.next(); this.writeSharedItem(writer, string); }
        while iter.hasNext(){let string=iter.next();self.writeSharedItem(writer,&string);}
    }
    // java private void writeSharedItem(XmlWriter writer, UnicodeString richtext) {
    fn writeSharedItem(&self,writer:&mut dyn XmlWriter,richtext:&UnicodeString){
        // java writer.startElement("si"); String str = richtext.getString();
        writer.startElement("si");let text:Vec<_>=richtext.getString().encode_utf16().collect();
        // java int no = richtext.getFormatRunCount(); int left = 0; int right = str.length() - 1;
        let no=richtext.getFormatRunCount();let mut left=0i32;let mut right=text.len()as i32-1;
        // java if (no != 0) {
        if no!=0{
            // java FormatRun r = richtext.getFormatRun(0); int id = r.getCharacterPos();
            let mut r=richtext.getFormatRun(0).expect("NullPointerException");let mut id=r.getCharacterPos()as i32;
            // java if (id != 0 && id <= str.length() && id >= left) {
            if id!=0&&id<=text.len()as i32&&id>=left{
                // java writer.startElement("r"); this.writerText(writer, str.substring(left, id)); writer.endElement("r"); left = id;
                writer.startElement("r");self.writerText(writer,&text[left as usize..id as usize]);writer.endElement("r");left=id;
            }
            // java for(int i = 0; i < no; ++i) {
            for i in 0..no{
                // java r = richtext.getFormatRun(i); id = r.getCharacterPos();
                r=richtext.getFormatRun(i).expect("NullPointerException");id=r.getCharacterPos()as i32;
                // java if (i == no - 1) { right = str.length(); }
                if i==no-1{right=text.len()as i32;}
                // java else { r = richtext.getFormatRun(i + 1); right = r.getCharacterPos(); if (right > str.length() || right < left) { break; } }
                else{r=richtext.getFormatRun(i+1).expect("NullPointerException");right=r.getCharacterPos()as i32;if right>text.len()as i32||right<left{break;}}
                // java if (left < right) {
                if left<right{
                    // java KmoFont font = this.book.getXFManager().getFont(this.getFontAtIndex(id, richtext));
                    let font=self.book.getXFManager().borrow().getFont(self.getFontAtIndex(id,richtext));
                    // java writer.startElement("r"); if (null != font) { this.writerFont(writer, font); }
                    // getFont's existing native Rc facade guarantees its fallback font.
                    writer.startElement("r");self.writerFont(writer,&font);
                    // java this.writerText(writer, str.substring(left, right)); writer.endElement("r");
                    self.writerText(writer,&text[left as usize..right as usize]);writer.endElement("r");
                }
                // java left = right;
                left=right;
            }
        // java } else { this.writerText(writer, str); }
        }else{self.writerText(writer,&text);}
        // java writer.endElement("si");
        writer.endElement("si");
    }
    // java public int getFontAtIndex(int index, UnicodeString richtext) {
    pub fn getFontAtIndex(&self,index:i32,richtext:&UnicodeString)->i32{
        // java int size = richtext.getFormatRunCount(); FormatRun currentRun = null;
        let size=richtext.getFormatRunCount();let mut currentRun=None;
        // java for(int i = 0; i < size; ++i) { FormatRun r = richtext.getFormatRun(i); if (r.getCharacterPos() > index) { break; } currentRun = r; }
        for i in 0..size{let r=richtext.getFormatRun(i).expect("NullPointerException");if r.getCharacterPos()as i32>index{break;}currentRun=Some(r);}
        // java return currentRun == null ? 0 : currentRun.getFontIndex();
        currentRun.map_or(0,|r|r.getFontIndex()as i32)
    }
    // java private void writerFont(XmlWriter writer, KmoFont font) {
    fn writerFont(&self,writer:&mut dyn XmlWriter,font:&KmoFont){
        // java writer.startElement("rPr"); FontsLabel.writeFontImpl(writer, font);
        writer.startElement("rPr");FontsLabel::writeFontImpl(writer,font);
        // java writer.startElement("rFont"); writer.addAttr("val", font.getFontName()); writer.endElement("rFont"); writer.endElement("rPr");
        writer.startElement("rFont");writer.addAttr("val",&font.getFontName());writer.endElement("rFont");writer.endElement("rPr");
    }
    // java private void writerText(XmlWriter writer, String text) {
    fn writerText(&self,writer:&mut dyn XmlWriter,text:&[u16]){
        // java writer.startElement("t");
        writer.startElement("t");let mut text=text.to_vec();
        // java if (text.length() > 0 && text.charAt(text.length() - 1) == 0) { text = text.substring(0, text.length() - 1) + "_x0000_"; }
        if !text.is_empty()&&text[text.len()-1]==0{text.pop();text.extend("_x0000_".encode_utf16());}
        // java if (this.hasWhiteSpace(text)) { writer.addAttr("xml:space", "preserve"); }
        if self.hasWhiteSpace(&text){writer.addAttr("xml:space","preserve");}
        // java text = EscapeUtil.escapeControlCharacter(text, EscapeUtil.CONTROL_CHARS);
        text=EscapeUtil::escapeControlCharacterUtf16(&text,CONTROL_CHARS);
        // java writer.addText(text); writer.endElement("t");
        writer.addTextUtf16(&text);writer.endElement("t");
    }
    // java private boolean hasWhiteSpace(String text) {
    fn hasWhiteSpace(&self,text:&[u16])->bool{
        // java int size = text.length(); if (size == 0) { return false; } else {
        let size=text.len();if size==0{return false;}
        // java char tmp = text.charAt(0); if (tmp != '\n' && tmp != '\r' && tmp != ' ' && tmp != '\t') {
        let mut tmp=text[0];if tmp!=10&&tmp!=13&&tmp!=32&&tmp!=9{
            // java tmp = text.charAt(size - 1); return tmp == '\n' || tmp == '\r' || tmp == ' ' || tmp == '\t';
            tmp=text[size-1];tmp==10||tmp==13||tmp==32||tmp==9
        // java } else { return true; }
        }else{true}
    }
    // java private int getUniqueCount() { return this.book.getCellValueStore().getUniqueCount(); }
    fn getUniqueCount(&self)->i32{self.book.getCellValueStore().borrow().getUniqueCount()}
}
impl Label for SstLabel{fn write(&self,writer:&mut dyn XmlWriter){SstLabel::write(self,writer);}}
