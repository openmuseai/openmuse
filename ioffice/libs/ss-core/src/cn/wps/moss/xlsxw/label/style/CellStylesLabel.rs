use crate::{java_runtime::Rc,cn::wps::moss::{app::xf::{KmoStyle::KmoStyle,KmoXFManager::KmoXFManager},filefmt::biff8::record::StyleRecord::StyleRecord}};
use super::CellStyleXfsLabel::CellStyleXfsLabel;
use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class CellStylesLabel {
pub struct CellStylesLabel;
impl CellStylesLabel{
    // java public static void write(XmlWriter writer, KmoXFManager xfManager, CellStyleXfsLabel styleXfs) {
    pub fn write(writer:&mut dyn XmlWriter,xfManager:&KmoXFManager,styleXfs:&CellStyleXfsLabel){
        // java int size = styleXfs.size(); List<KmoStyle> styles = xfManager.getAllStyles(); int usefulStyleCount = getUsefulStyleCount(styleXfs, styles);
        let size=styleXfs.size();let styles=xfManager.getAllStyles();let usefulStyleCount=Self::getUsefulStyleCount(styleXfs,&styles);
        // java if (usefulStyleCount == 0) { writeDefaultStyle(writer); } else {
        if usefulStyleCount==0{Self::writeDefaultStyle(writer);}else{
            // java writer.startElement("cellStyles"); writer.addAttr("count", usefulStyleCount);
            writer.startElement("cellStyles");writer.addAttr__i32("count",usefulStyleCount);
            // java for(int i = 0; i < size; ++i) { int ixf = styleXfs.getXfIndexAt(i); KmoStyle style = getStyle(styles, ixf); if (style != null) {
            for i in 0..size{let ixf=styleXfs.getXfIndexAt(i);let style=Self::getStyle(&styles,ixf);if let Some(style)=style{
                // java writer.startElement("cellStyle"); if (!style.isBuiltinStyle()) { writer.addAttr("name", style.getName()); }
                writer.startElement("cellStyle");if !style.isBuiltinStyle(){writer.addAttrNullable(Some("name"),style.getName().as_deref());}
                // java if (style.isBuiltinStyle()) { int buildInIndex = style.getBuiltinStyle(); String name = StyleRecord.getName(buildInIndex); if (name != null && name.length() != 0) { writer.addAttr("name", name); } }
                if style.isBuiltinStyle(){let buildInIndex=style.getBuiltinStyle();let name=StyleRecord::getName(buildInIndex);if !name.is_empty(){writer.addAttr("name",name);}}
                // java writer.addAttr("xfId", i); if (style.isBuiltinStyle()) { writer.addAttr("builtinId", style.getBuiltinStyle()); }
                writer.addAttr__i32("xfId",i);if style.isBuiltinStyle(){writer.addAttr__i32("builtinId",style.getBuiltinStyle());}
                // java writer.endElement("cellStyle");
                writer.endElement("cellStyle");
            }}
            // java writer.endElement("cellStyles");
            writer.endElement("cellStyles");
        }
    }
    // java private static int getUsefulStyleCount(CellStyleXfsLabel styleXfs, List<KmoStyle> styles) {
    fn getUsefulStyleCount(styleXfs:&CellStyleXfsLabel,styles:&[Rc<KmoStyle>])->i32{
        // java int count = 0; int i = 0; for(int size = styleXfs.size(); i < size; ++i) { int ixf = styleXfs.getXfIndexAt(i); KmoStyle style = getStyle(styles, ixf); if (style != null) { ++count; } } return count;
        let mut count=0;let mut i=0;let size=styleXfs.size();while i<size{let ixf=styleXfs.getXfIndexAt(i);let style=Self::getStyle(styles,ixf);if style.is_some(){count+=1;}i+=1;}count
    }
    // java private static void writeDefaultStyle(XmlWriter writer) {
    fn writeDefaultStyle(writer:&mut dyn XmlWriter){
        // java writer.startElement("cellStyles"); writer.addAttr("count", "1"); writer.startElement("cellStyle"); writer.addAttr("xfId", "0"); writer.addAttr("builtinId", "0"); writer.addAttr("name", "常规"); writer.endElement("cellStyle"); writer.endElement("cellStyles");
        writer.startElement("cellStyles");writer.addAttr("count","1");writer.startElement("cellStyle");writer.addAttr("xfId","0");writer.addAttr("builtinId","0");writer.addAttr("name","常规");writer.endElement("cellStyle");writer.endElement("cellStyles");
    }
    // java private static KmoStyle getStyle(List<KmoStyle> styles, int ixf) {
    fn getStyle(styles:&[Rc<KmoStyle>],ixf:i32)->Option<Rc<KmoStyle>>{
        // java for(KmoStyle style : styles) { int xf = style.getIXF(); if (xf == ixf) { return style; } } return null;
        for style in styles{let xf=style.getIXF()as i32;if xf==ixf{return Some(style.clone());}}None
    }
}
