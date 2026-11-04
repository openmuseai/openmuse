use kfc_kfc_alpha::kfc::{ooxfw::OxfwCustomPart::OxfwCustomPart,opcr::OpcUtil::OpcUtil,xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
use io_poi_compact::org::apache::poi::hpsf::Property::{Property,PropertyValue};
use crate::cn::wps::moss::{core::book::KmoAppSettings::KmoAppSettings,xlsxw::util::XlsxwPackage::XlsxwPackage};
// java public class XlsxwCustom {
pub struct XlsxwCustom;
#[cfg(test)]
#[path="XlsxwCustom.review.rs"]
mod review;
impl XlsxwCustom {
    // java public static void write(OxfwCustomPart part, KmoAppSettings settings) {
    pub fn write(part:&mut OxfwCustomPart,settings:&mut KmoAppSettings){
        // java if (!settings.hasIcv()) { settings.addIcv(); }
        if !settings.hasIcv(){settings.addIcv();}
        // java Map<String, Property> props = settings.getCustomProps();
        let props=settings.getCustomProps();let props=props.borrow();
        // java if (!props.isEmpty()) {
        if !props.isEmpty(){
            // java KmoLog.logPoint(); OutputStream stream = XlsxwPackage.getOutputStream(part);
            let stream=XlsxwPackage::getOutputStream(part);
            // java XmlWriter writer = new BufferXmlWriter(stream);
            let mut writer=BufferXmlWriter::new(Box::new(stream.expect("NullPointerException")));
            // java writer.startDocument(); writer.startElement("Properties");
            writer.startDocument();writer.startElement("Properties");
            // java writer.addNS((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties");
            writer.addNS(None,"http://schemas.openxmlformats.org/officeDocument/2006/custom-properties");
            // java writer.addNS("vt", "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes");
            writer.addNS(Some("vt"),"http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes");
            // java int pid = 2;
            let mut pid=2i32;
            // java for(String key : props.keySet()) { Property prop = (Property)props.get(key); writeProperty(writer, key, prop, pid++); }
            for (key,prop) in props.entrySet(){Self::writeProperty(&mut writer,Some(key),prop.as_deref(),pid);pid=pid.wrapping_add(1);}
            // java writer.endElement("Properties"); writer.endDocument(); KmoLog.logPoint();
            writer.endElement("Properties");writer.endDocument();
        }
    }
    // java private static void writeProperty(XmlWriter writer, String name, Property prop, int pid) {
    fn writeProperty(writer:&mut dyn XmlWriter,name:Option<&str>,prop:Option<&Property>,pid:i32){
        // java if (isValidProperty(prop.getType())) {
        let prop=prop.expect("NullPointerException");
        if Self::isValidProperty(prop.getType()){
            // java writer.startElement("property");
            writer.startElement("property");
            // java writer.addAttr("fmtid", "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}");
            writer.addAttr("fmtid","{D5CDD505-2E9C-101B-9397-08002B2CF9AE}");
            // java writer.addAttr("pid", pid);
            writer.addAttr__i32("pid",pid);
            // java writer.addAttr("name", name);
            writer.addAttrNullable(Some("name"),name);
            // java writePropertyValue(writer, prop);
            Self::writePropertyValue(writer,prop);
            // java writer.endElement("property");
            writer.endElement("property");
        }
    }
    // java private static boolean isValidProperty(long type) { return type == 11L || type == 31L || type == 30L || type == 3L || type == 64L; }
    fn isValidProperty(r#type:i64)->bool{r#type==11||r#type==31||r#type==30||r#type==3||r#type==64}
    // java private static void writePropertyValue(XmlWriter writer, Property prop) {
    fn writePropertyValue(writer:&mut dyn XmlWriter,prop:&Property){
        // java long type = prop.getType();
        let r#type=prop.getType();
        // java if (type == 11L) {
        if r#type==11{
            // java writer.startElement("vt:bool"); boolean b = (Boolean)prop.getValue();
            writer.startElement("vt:bool");let b=match prop.getValue(){PropertyValue::Bool(b)=>*b,PropertyValue::None=>panic!("NullPointerException"),_=>panic!("ClassCastException Boolean")};
            // java if (b) { writer.addText("true"); } else { writer.addText("false"); }
            if b{writer.addText("true");}else{writer.addText("false");}
            // java writer.endElement("vt:bool");
            writer.endElement("vt:bool");
        // java } else if (type != 31L && type != 30L) {
        }else if r#type!=31&&r#type!=30{
            // java if (type == 3L) {
            if r#type==3{
                // java writer.startElement("vt:i4"); writer.addText((Integer)prop.getValue()); writer.endElement("vt:i4");
                writer.startElement("vt:i4");let v=match prop.getValue(){PropertyValue::Int(v)=>*v,PropertyValue::None=>panic!("NullPointerException"),_=>panic!("ClassCastException Integer")};writer.addText__i32(v);writer.endElement("vt:i4");
            // java } else if (type == 64L) {
            }else if r#type==64{
                // java writer.startElement("vt:filetime"); String tmp = OpcUtil.getDateString((Date)prop.getValue()); writer.addText(tmp); writer.endElement("vt:filetime");
                writer.startElement("vt:filetime");let v=match prop.getValue(){PropertyValue::Date(v)=>v,PropertyValue::None=>panic!("NullPointerException"),_=>panic!("ClassCastException Date")};let tmp=OpcUtil::getDateString(v.getTime());writer.addText(&tmp);writer.endElement("vt:filetime");
            }
        // java } else {
        }else{
            // java writer.startElement("vt:lpwstr"); writer.addText((String)prop.getValue()); writer.endElement("vt:lpwstr");
            writer.startElement("vt:lpwstr");let v=match prop.getValue(){PropertyValue::Str(v)=>Some(v.as_str()),PropertyValue::None=>None,_=>panic!("ClassCastException String")};writer.addTextNullable(v);writer.endElement("vt:lpwstr");
        }
    }
}
