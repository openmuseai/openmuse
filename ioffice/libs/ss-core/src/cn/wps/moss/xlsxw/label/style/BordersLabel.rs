use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::{filefmt::biff8::{record::cf::BorderFormatting::BorderFormatting,util::ColorToolKit::ColorToolKit},xlsxw::{label::Label::Label,util::ValueHelper::ValueHelper}}};
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class BordersLabel implements Label {
pub struct BordersLabel{
    // java private Vector<BorderFormatting> borderList = new Vector();
    borderList:Vec<Option<Rc<RefCell<BorderFormatting>>>>,
}
impl BordersLabel{
    // java private static String[] borderStyleList = new String[14];
    const borderStyleList:[&'static str;14]=["none","thin","medium","dashed","dotted","thick","double","hair","mediumDashed","dashDot","mediumDashDot","dashDotDot","mediumDashDotDot","slantDashDot"];
    // java BordersLabel() { }
    pub(crate) fn BordersLabel()->Self{Self{borderList:Vec::new()}}
    // java public int addForWriter(BorderFormatting border) {
    pub fn addForWriter(&mut self,border:Option<Rc<RefCell<BorderFormatting>>>)->i32{
        // java for(int index = 0; index < this.borderList.size(); ++index) {
        for index in 0..self.borderList.len(){
            // java if (border.equals(this.borderList.get(index))) { return index; }
            let border=border.as_ref().expect("NullPointerException");
            if self.borderList[index].as_ref().is_some_and(|b|border.borrow().equals(&b.borrow())){return index as i32;}
        }
        // java this.borderList.add(border); return this.borderList.size() - 1;
        self.borderList.push(border);self.borderList.len()as i32-1
    }
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java if (this.borderList.size() != 0) {
        if !self.borderList.is_empty(){
            // java writer.startElement("borders"); writer.addAttr("count", this.borderList.size());
            writer.startElement("borders");writer.addAttr__i32("count",self.borderList.len()as i32);
            // java for(int index = 0; index < this.borderList.size(); ++index) { writeBorder(writer, (BorderFormatting)this.borderList.get(index)); }
            for index in 0..self.borderList.len(){Self::writeBorder(writer,self.borderList[index].as_ref().map(|b|b.borrow()).as_deref());}
            // java writer.endElement("borders");
            writer.endElement("borders");
        }
    }
    // java public static void writeBorder(XmlWriter writer, BorderFormatting border) {
    pub fn writeBorder(writer:&mut dyn XmlWriter,border:Option<&BorderFormatting>){
        // java writer.startElement("border");
        writer.startElement("border");let border=border.expect("NullPointerException");
        // java if (border.isForwardDiagonalOn()) { writer.addAttr("diagonalUp", true); }
        if border.isForwardDiagonalOn(){writer.addAttr__bool("diagonalUp",true);}
        // java if (border.isBackwardDiagonalOn()) { writer.addAttr("diagonalDown", true); }
        if border.isBackwardDiagonalOn(){writer.addAttr__bool("diagonalDown",true);}
        // java writer.startElement("left"); String str = getStyle((short)border.getBorderLeft());
        writer.startElement("left");let mut str=Self::getStyle(Some(border.getBorderLeft()as i16));
        // java if (!str.equals("none")) { writer.addAttr("style", getStyle((short)border.getBorderLeft())); writeBorderColor(writer, border.getLeftBorderColor()); }
        if str!="none"{writer.addAttr("style",Self::getStyle(Some(border.getBorderLeft()as i16)));Self::writeBorderColor(writer,border.getLeftBorderColor());}
        // java writer.endElement("left"); writer.startElement("right"); str = getStyle((short)border.getBorderRight());
        writer.endElement("left");writer.startElement("right");str=Self::getStyle(Some(border.getBorderRight()as i16));
        // java if (!str.equals("none")) { writer.addAttr("style", str); writeBorderColor(writer, border.getRightBorderColor()); }
        if str!="none"{writer.addAttr("style",str);Self::writeBorderColor(writer,border.getRightBorderColor());}
        // java writer.endElement("right"); writer.startElement("top"); str = getStyle((short)border.getBorderTop());
        writer.endElement("right");writer.startElement("top");str=Self::getStyle(Some(border.getBorderTop()as i16));
        // java if (!str.equals("none")) { writer.addAttr("style", str); writeBorderColor(writer, border.getTopBorderColor()); }
        if str!="none"{writer.addAttr("style",str);Self::writeBorderColor(writer,border.getTopBorderColor());}
        // java writer.endElement("top"); writer.startElement("bottom"); str = getStyle((short)border.getBorderBottom());
        writer.endElement("top");writer.startElement("bottom");str=Self::getStyle(Some(border.getBorderBottom()as i16));
        // java if (!str.equals("none")) { writer.addAttr("style", str); writeBorderColor(writer, border.getBottomBorderColor()); }
        if str!="none"{writer.addAttr("style",str);Self::writeBorderColor(writer,border.getBottomBorderColor());}
        // java writer.endElement("bottom"); writer.startElement("diagonal"); str = getStyle((short)border.getBorderDiagonal());
        writer.endElement("bottom");writer.startElement("diagonal");str=Self::getStyle(Some(border.getBorderDiagonal()as i16));
        // java if (!str.equals("none")) { writer.addAttr("style", str); writeBorderColor(writer, border.getDiagonalBorderColor()); }
        if str!="none"{writer.addAttr("style",str);Self::writeBorderColor(writer,border.getDiagonalBorderColor());}
        // java writer.endElement("diagonal"); str = getStyle((short)border.getBorderVertical());
        writer.endElement("diagonal");str=Self::getStyle(Some(border.getBorderVertical()as i16));
        // java if (!str.equals("none")) { writer.startElement("vertical"); writer.addAttr("style", str); writeBorderColor(writer, border.getVerticalBorderColor()); writer.endElement("vertical"); }
        if str!="none"{writer.startElement("vertical");writer.addAttr("style",str);Self::writeBorderColor(writer,border.getVerticalBorderColor());writer.endElement("vertical");}
        // java else if (border.hasVertical()) { writer.startElement("vertical"); writer.endElement("vertical"); }
        else if border.hasVertical(){writer.startElement("vertical");writer.endElement("vertical");}
        // java str = getStyle((short)border.getBorderHorizontal());
        str=Self::getStyle(Some(border.getBorderHorizontal()as i16));
        // java if (!str.equals("none")) { writer.startElement("horizontal"); writer.addAttr("style", str); writeBorderColor(writer, border.getHorizontalBorderColor()); writer.endElement("horizontal"); }
        if str!="none"{writer.startElement("horizontal");writer.addAttr("style",str);Self::writeBorderColor(writer,border.getHorizontalBorderColor());writer.endElement("horizontal");}
        // java else if (border.hasHorizontal()) { writer.startElement("horizontal"); writer.endElement("horizontal"); }
        else if border.hasHorizontal(){writer.startElement("horizontal");writer.endElement("horizontal");}
        // java writer.endElement("border");
        writer.endElement("border");
    }
    // java public static void writeBorderColor(XmlWriter writer, int color) {
    pub fn writeBorderColor(writer:&mut dyn XmlWriter,color:i32){
        // java if (ColorToolKit.isValidColor(color)) {
        if ColorToolKit::isValidColor(color){
            // java writer.startElement("color");
            writer.startElement("color");
            // java if (ColorToolKit.isPaletteIndex(color)) { writer.addAttr("indexed", color); }
            if ColorToolKit::isPaletteIndex(color){writer.addAttr__i32("indexed",color);}
            // java else { writer.addAttr("rgb", ValueHelper.toHexColor(color)); }
            else{writer.addAttr("rgb",&ValueHelper::toHexColor(color));}
            // java writer.endElement("color");
            writer.endElement("color");
        }
    }
    // java public static String getStyle(Short value) {
    pub fn getStyle(value:Option<i16>)->&'static str{
        let mut value=value.expect("NullPointerException");
        // java if (value < 0 || value >= borderStyleList.length) { value = Short.valueOf((short)1); }
        if value<0||value as usize>=Self::borderStyleList.len(){value=1;}
        // java return borderStyleList[value];
        Self::borderStyleList[value as usize]
    }
    // java static { borderStyleList[0] = "none"; borderStyleList[1] = "thin"; borderStyleList[2] = "medium";
    // java borderStyleList[3] = "dashed"; borderStyleList[4] = "dotted"; borderStyleList[5] = "thick";
    // java borderStyleList[6] = "double"; borderStyleList[7] = "hair"; borderStyleList[8] = "mediumDashed";
    // java borderStyleList[9] = "dashDot"; borderStyleList[10] = "mediumDashDot"; borderStyleList[11] = "dashDotDot";
    // java borderStyleList[12] = "mediumDashDotDot"; borderStyleList[13] = "slantDashDot"; }
}
impl Label for BordersLabel{fn write(&self,w:&mut dyn XmlWriter){BordersLabel::write(self,w)}}
