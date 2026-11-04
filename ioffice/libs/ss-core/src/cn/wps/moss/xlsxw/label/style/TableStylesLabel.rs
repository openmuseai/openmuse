use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::{app::{xf::{KmoXFManager::KmoXFManager,KmoTableStyles::{KmoTableStyle,KmoTableStyleElement}},pivot::util::XmlStringUtils::XmlStringUtils},xlsxw::util::WriterUtils::WriterUtils}};
use super::StyleSheetLabel::StyleSheetLabel;
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public class TableStylesLabel {
pub struct TableStylesLabel;
impl TableStylesLabel{
    // java public static void write(XmlWriter writer, KmoXFManager xfManager, StyleSheetLabel styleSheetLabel) {
    pub fn write(writer:&mut dyn XmlWriter,xfManager:&KmoXFManager,styleSheetLabel:&StyleSheetLabel){
        // java KmoTableStyles tableStyles = xfManager.getKmoTableStyles(); if (tableStyles != null) {
        let tableStyles=xfManager.getKmoTableStyles();if let Some(tableStyles)=tableStyles{
            // java writer.startElement("tableStyles"); WriterUtils.addAttr(writer, "defaultTableStyle", tableStyles.getDefaultTableStyle()); WriterUtils.addAttr(writer, "defaultPivotStyle", tableStyles.getDefaultPivotStyle()); WriterUtils.addAttr(writer, "count", (int)tableStyles.getTableStyles().size(), (int)-1);
            writer.startElement("tableStyles");WriterUtils::addAttr(writer,"defaultTableStyle",tableStyles.getDefaultTableStyle().as_deref());WriterUtils::addAttr(writer,"defaultPivotStyle",tableStyles.getDefaultPivotStyle().as_deref());WriterUtils::addAttr__int_default(writer,"count",tableStyles.getTableStyles().expect("NullPointerException").borrow().len()as i32,-1);
            // java for(KmoTableStyles.KmoTableStyle tableStyle : tableStyles.getTableStyles()) { writeTableStyle(writer, tableStyle, styleSheetLabel); }
            for tableStyle in tableStyles.getTableStyles().expect("NullPointerException").borrow().iter(){Self::writeTableStyle(writer,tableStyle,styleSheetLabel);}
            // java writer.endElement("tableStyles");
            writer.endElement("tableStyles");
        }
    }
    // java private static void writeTableStyle(XmlWriter writer, KmoTableStyles.KmoTableStyle tableStyle, StyleSheetLabel styleSheetLabel) {
    fn writeTableStyle(writer:&mut dyn XmlWriter,tableStyle:&KmoTableStyle,styleSheetLabel:&StyleSheetLabel){
        // java writer.startElement("tableStyle"); WriterUtils.addAttr(writer, "name", tableStyle.getName()); WriterUtils.addAttr(writer, "pivot", tableStyle.isPivot(), true); WriterUtils.addAttr(writer, "table", tableStyle.isTable(), true); WriterUtils.addAttr(writer, "count", (int)tableStyle.getTableStyleElements().size(), (int)-1);
        writer.startElement("tableStyle");WriterUtils::addAttr(writer,"name",tableStyle.getName().as_deref());WriterUtils::addAttr__bool_default(writer,"pivot",tableStyle.isPivot(),true);WriterUtils::addAttr__bool_default(writer,"table",tableStyle.isTable(),true);WriterUtils::addAttr__int_default(writer,"count",tableStyle.getTableStyleElements().expect("NullPointerException").borrow().len()as i32,-1);
        // java for(KmoTableStyles.KmoTableStyleElement tableStyleElement : tableStyle.getTableStyleElements()) { writeTableStyleElement(writer, tableStyleElement, styleSheetLabel); }
        for tableStyleElement in tableStyle.getTableStyleElements().expect("NullPointerException").borrow().iter(){Self::writeTableStyleElement(writer,tableStyleElement,styleSheetLabel);}
        // java writer.endElement("tableStyle");
        writer.endElement("tableStyle");
    }
    // java public static void addWriteForDxfs(KmoXFManager xfManager, StyleSheetLabel styleSheetLabel) {
    pub fn addWriteForDxfs(xfManager:&KmoXFManager,styleSheetLabel:&StyleSheetLabel){
        // java KmoTableStyles tableStyles = xfManager.getKmoTableStyles(); if (tableStyles != null) {
        let tableStyles=xfManager.getKmoTableStyles();if let Some(tableStyles)=tableStyles{
            // java for(KmoTableStyles.KmoTableStyle tableStyle : tableStyles.getTableStyles()) { for(KmoTableStyles.KmoTableStyleElement tableStyleElement : tableStyle.getTableStyleElements()) {
            for tableStyle in tableStyles.getTableStyles().expect("NullPointerException").borrow().iter(){for tableStyleElement in tableStyle.getTableStyleElements().expect("NullPointerException").borrow().iter(){
                // java DataFormatting dataFormatting = tableStyleElement.getDataFormatting(); if (tableStyleElement.getDxfId() != -1 && dataFormatting != null) {
                let dataFormatting=tableStyleElement.getDataFormatting();if tableStyleElement.getDxfId()!=-1{
                    // java PatternFormatting pattern = null; BorderFormatting border = null; FontFormatting font = null; NumberFormatFormatting numFmt = null;
                    let mut pattern=None;let mut border=None;let mut font=None;let mut numFmt=None;
                    // java if (dataFormatting.getPatternFormatting() != null) { pattern = (PatternFormatting)dataFormatting.getPatternFormatting().clone(); }
                    if dataFormatting.getPatternFormatting().is_some(){pattern=Some(Rc::new(RefCell::new(dataFormatting.getPatternFormatting().unwrap().borrow().clone())));}
                    // java if (dataFormatting.getBorderFormatting() != null) { border = (BorderFormatting)dataFormatting.getBorderFormatting().clone(); }
                    if dataFormatting.getBorderFormatting().is_some(){border=Some(Rc::new(RefCell::new(dataFormatting.getBorderFormatting().unwrap().borrow().clone())));}
                    // java if (dataFormatting.getFontFormatting() != null) { font = (FontFormatting)dataFormatting.getFontFormatting().clone(); }
                    if dataFormatting.getFontFormatting().is_some(){font=Some(Rc::new(dataFormatting.getFontFormatting().unwrap().as_ref().clone()));}
                    // java if (dataFormatting.getNumberFormatFormatting() != null) { numFmt = dataFormatting.getNumberFormatFormatting().clone(); XmlStringUtils.formatNumFmtCode2Xml(numFmt); }
                    if dataFormatting.getNumberFormatFormatting().is_some(){let mut value=dataFormatting.getNumberFormatFormatting().unwrap().as_ref().clone();XmlStringUtils::formatNumFmtCode2Xml(Some(&mut value));numFmt=Some(Rc::new(value));}
                    // java int id = styleSheetLabel.getDifferentialFormattingLabel().addForWrite(pattern, border, font, numFmt); tableStyleElement.setDxfId(id);
                    let id=styleSheetLabel.getDifferentialFormattingLabel().borrow_mut().addForWrite__4(pattern,border,font,numFmt);tableStyleElement.setDxfId(id);
                }
            }}
        }
    }
    // java private static void writeTableStyleElement(XmlWriter writer, KmoTableStyles.KmoTableStyleElement tableStyleElement, StyleSheetLabel styleSheetLabel) {
    fn writeTableStyleElement(writer:&mut dyn XmlWriter,tableStyleElement:&KmoTableStyleElement,_styleSheetLabel:&StyleSheetLabel){
        // java writer.startElement("tableStyleElement"); WriterUtils.addAttr(writer, "type", tableStyleElement.getType().name()); WriterUtils.addAttr(writer, "size", (int)tableStyleElement.getSize(), (int)1); WriterUtils.addAttr(writer, "dxfId", (int)tableStyleElement.getDxfId(), (int)-1); writer.endElement("tableStyleElement");
        writer.startElement("tableStyleElement");WriterUtils::addAttr(writer,"type",Some(&format!("{:?}",tableStyleElement.getType().expect("NullPointerException"))));WriterUtils::addAttr__int_default(writer,"size",tableStyleElement.getSize(),1);WriterUtils::addAttr__int_default(writer,"dxfId",tableStyleElement.getDxfId(),-1);writer.endElement("tableStyleElement");
    }
}
