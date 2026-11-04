use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::{filefmt::biff8::{record::cf::PatternFormatting::PatternFormatting,util::ColorToolKit::ColorToolKit},xlsxw::{label::Label::Label,util::ValueHelper::ValueHelper}}};
use super::PatternTypeMap::PatternTypeMap;
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class FillsLabel implements Label {
pub struct FillsLabel{
    // java private Vector<PatternFormatting> patternFillList = new Vector();
    patternFillList:Vec<Rc<RefCell<PatternFormatting>>>,
}
impl FillsLabel{
    // java FillsLabel() {
    pub(crate) fn FillsLabel()->Self{
        let mut this=Self{patternFillList:Vec::new()};
        // java PatternFormatting pa = new PatternFormatting();
        let mut pa=PatternFormatting::PatternFormatting();
        // java pa.setFillForegroundColor(64); pa.setFillBackgroundColor(65); pa.setFillPattern(0);
        pa.setFillForegroundColor(64);pa.setFillBackgroundColor(65);pa.setFillPattern(0);
        // java this.addForWrite(pa);
        this.addForWrite(Some(Rc::new(RefCell::new(pa))));
        // java pa = new PatternFormatting(); pa.setFillPattern(17); this.addForWrite(pa);
        let mut pa=PatternFormatting::PatternFormatting();pa.setFillPattern(17);this.addForWrite(Some(Rc::new(RefCell::new(pa))));this
    }
    // java public int addForWrite(PatternFormatting pattern) {
    pub fn addForWrite(&mut self,pattern:Option<Rc<RefCell<PatternFormatting>>>)->i32{
        // java for(int index = 0; index < this.patternFillList.size(); ++index) {
        for index in 0..self.patternFillList.len(){
            // java if (pattern.equals(this.patternFillList.get(index))) { return index; }
            if pattern.as_ref().expect("NullPointerException").borrow().equals(&self.patternFillList[index].borrow()){return index as i32;}
        }
        // java this.patternFillList.add(pattern); return this.patternFillList.size() - 1;
        self.patternFillList.push(pattern.expect("NullPointerException"));self.patternFillList.len()as i32-1
    }
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java if (this.patternFillList.size() != 0) {
        if !self.patternFillList.is_empty(){
            // java writer.startElement("fills"); writer.addAttr("count", this.patternFillList.size());
            writer.startElement("fills");writer.addAttr__i32("count",self.patternFillList.len()as i32);
            // java for(int index = 0; index < this.patternFillList.size(); ++index) { writeFill(writer, (PatternFormatting)this.patternFillList.get(index), false); }
            for index in 0..self.patternFillList.len(){Self::writeFill(writer,Some(&self.patternFillList[index].borrow()),false);}
            // java writer.endElement("fills");
            writer.endElement("fills");
        }
    }
    // java public static void writeFill(XmlWriter writer, PatternFormatting pattern, boolean contition) {
    pub fn writeFill(writer:&mut dyn XmlWriter,pattern:Option<&PatternFormatting>,contition:bool){
        // java writer.startElement("fill"); XFGradient gradient = pattern.getXFGradient();
        writer.startElement("fill");let pattern=pattern.expect("NullPointerException");let gradient=pattern.getXFGradient();
        // java if (gradient != null && pattern.getFillPattern() == 32) {
        if gradient.is_some()&&pattern.getFillPattern()==32{
            let gradient=gradient.as_ref().unwrap().borrow();
            // java writer.startElement("gradientFill");
            writer.startElement("gradientFill");
            // java if (!gradient.isLinear) {
            if !gradient.isLinear{
                // java writer.addAttr("type", "path"); writer.addAttr("left", gradient.left); writer.addAttr("right", gradient.right); writer.addAttr("top", gradient.top); writer.addAttr("bottom", gradient.bottom);
                writer.addAttr("type","path");writer.addAttr__f32("left",gradient.left);writer.addAttr__f32("right",gradient.right);writer.addAttr__f32("top",gradient.top);writer.addAttr__f32("bottom",gradient.bottom);
            // java } else { writer.addAttr("degree", gradient.degree); }
            }else{writer.addAttr__i32("degree",gradient.degree);}
            // java for(int i = 0; i < gradient.colorStops.size(); ++i) {
            for i in 0..gradient.colorStops.len(){
                // java XFGradient.ColorStop colorStop = (XFGradient.ColorStop)gradient.colorStops.get(i);
                let colorStop=gradient.colorStops[i].borrow();
                // java writer.startElement("stop"); writer.addAttr("position", colorStop.position);
                writer.startElement("stop");writer.addAttr__f32("position",colorStop.position);
                // java int pindex = colorStop.color; writeColor(writer, "color", pindex); writer.endElement("stop");
                let pindex=colorStop.color;Self::writeColor(writer,"color",pindex);writer.endElement("stop");
            }
            // java writer.endElement("gradientFill");
            writer.endElement("gradientFill");
        // java } else {
        }else{
            // java writer.startElement("patternFill"); String str = PatternTypeMap.getType((short)pattern.getFillPattern());
            writer.startElement("patternFill");let str=PatternTypeMap::getType(Some(pattern.getFillPattern()as i16));
            // java if (!contition || !str.equals("none")) { writer.addAttr("patternType", str); }
            if !contition||str.expect("NullPointerException")!="none"{writer.addAttrNullable(Some("patternType"),str);}
            // java int pindex = pattern.getFillForegroundColor();
            let mut pindex=pattern.getFillForegroundColor();
            // java if (pindex != 64 && ColorToolKit.isValidColor(pindex)) { writeColor(writer, "fgColor", pindex); }
            if pindex!=64&&ColorToolKit::isValidColor(pindex){Self::writeColor(writer,"fgColor",pindex);}
            // java pindex = pattern.getFillBackgroundColor();
            pindex=pattern.getFillBackgroundColor();
            // java if (pindex != 65 && ColorToolKit.isValidColor(pindex)) { writeColor(writer, "bgColor", pindex); }
            if pindex!=65&&ColorToolKit::isValidColor(pindex){Self::writeColor(writer,"bgColor",pindex);}
            // java writer.endElement("patternFill");
            writer.endElement("patternFill");
        }
        // java writer.endElement("fill");
        writer.endElement("fill");
    }
    // java private static void writeColor(XmlWriter writer, String elementName, int pindex) {
    fn writeColor(writer:&mut dyn XmlWriter,elementName:&str,pindex:i32){
        // java writer.startElement(elementName);
        writer.startElement(elementName);
        // java if (ColorToolKit.isPaletteIndex(pindex)) { writer.addAttr("indexed", pindex); }
        if ColorToolKit::isPaletteIndex(pindex){writer.addAttr__i32("indexed",pindex);}
        // java else { writer.addAttr("rgb", ValueHelper.toHexColor(pindex)); }
        else{writer.addAttr("rgb",&ValueHelper::toHexColor(pindex));}
        // java writer.endElement(elementName);
        writer.endElement(elementName);
    }
}
impl Label for FillsLabel{fn write(&self,w:&mut dyn XmlWriter){FillsLabel::write(self,w)}}
