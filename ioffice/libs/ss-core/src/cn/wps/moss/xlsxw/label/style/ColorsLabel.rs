use crate::{java_runtime::Rc,cn::wps::moss::{app::KmoBook::KmoBook,core::book::KmoPalette::KmoPalette,xlsxw::label::Label::Label}};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public class ColorsLabel implements Label {
pub struct ColorsLabel{
    // java private KmoPalette kmoPalette;
    kmoPalette:Rc<KmoPalette>,
}
impl ColorsLabel{
    // java public ColorsLabel(KmoBook book) { this.kmoPalette = book.getPalette(); }
    pub fn ColorsLabel(book:&KmoBook)->Self{Self{kmoPalette:book.getPalette()}}
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java if (this.kmoPalette.isPaletteMod()) {
        if self.kmoPalette.isPaletteMod(){
            // java writer.startElement("colors"); writer.startElement("indexedColors"); this.writerColor(writer);
            writer.startElement("colors");writer.startElement("indexedColors");self.writerColor(writer);
            // java writer.endElement("indexedColors"); writer.endElement("colors");
            writer.endElement("indexedColors");writer.endElement("colors");
        }
    }
    // java private boolean writerColor(XmlWriter writer) {
    fn writerColor(&self,writer:&mut dyn XmlWriter)->bool{
        // java for(int index = 0; index < 64; ++index) {
        for index in 0..64{
            // java writer.startElement("rgbColor"); int color = this.kmoPalette.getRGBColor((short)index);
            writer.startElement("rgbColor");let color=self.kmoPalette.getRGBColor(index as i16);
            // java String value = Integer.toHexString(color).toUpperCase();
            let mut value=format!("{:X}",color as u32);
            // java if (value.charAt(0) == 'F') { value = value.substring(2, 8); }
            if value.as_bytes()[0]==b'F'{value=value.get(2..8).expect("StringIndexOutOfBoundsException").into();}
            // java value = "00" + value; writer.addAttr("rgb", value); writer.endElement("rgbColor");
            value="00".to_string()+&value;writer.addAttr("rgb",&value);writer.endElement("rgbColor");
        }
        // java return true;
        true
    }
}
impl Label for ColorsLabel{fn write(&self,w:&mut dyn XmlWriter){ColorsLabel::write(self,w)}}
