use kfc_kfc_alpha::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public class WriterUtils {
// Original overload slice used by TableStylesLabel; date/BigDecimal leaves remain separate dependencies.
pub struct WriterUtils;
impl WriterUtils{
    // java public static void addAttr(XmlWriter writer, String name, String value) { addAttr(writer, name, value, (String)null); }
    pub fn addAttr(writer:&mut dyn XmlWriter,name:&str,value:Option<&str>){Self::addAttr__string_default(writer,name,value,None)}
    // java public static void addAttr(XmlWriter writer, String name, String value, String defaultValue) {
    pub fn addAttr__string_default(writer:&mut dyn XmlWriter,name:&str,value:Option<&str>,defaultValue:Option<&str>){
        // java if (defaultValue == null || !defaultValue.equalsIgnoreCase(value)) { writer.addAttr(name, value); }
        if defaultValue.is_none()||!match (defaultValue,value){(Some(a),Some(b))=>crate::java_runtime::Character::equalsIgnoreCase(&a.encode_utf16().collect::<Vec<_>>(),&b.encode_utf16().collect::<Vec<_>>()),_=>false}{writer.addAttrNullable(Some(name),value);}
    }
    // java public static void addAttr(XmlWriter writer, String name, int value) { addAttr(writer, name, (int)value, (int)0); }
    pub fn addAttr__int(writer:&mut dyn XmlWriter,name:&str,value:i32){Self::addAttr__int_default(writer,name,value,0)}
    // java public static void addAttr(XmlWriter writer, String name, int value, int defaultValue) { if (value != defaultValue) { writer.addAttr(name, value); } }
    pub fn addAttr__int_default(writer:&mut dyn XmlWriter,name:&str,value:i32,defaultValue:i32){if value!=defaultValue{writer.addAttr__i32(name,value);}}
    // java public static void addAttrDirectly(XmlWriter writer, String name, int value) { writer.addAttr(name, value); }
    pub fn addAttrDirectly(writer:&mut dyn XmlWriter,name:&str,value:i32){writer.addAttr__i32(name,value);}
    // java public static void addAttr(XmlWriter writer, String name, boolean value) { addAttr(writer, name, value, false); }
    pub fn addAttr__bool(writer:&mut dyn XmlWriter,name:&str,value:bool){Self::addAttr__bool_default(writer,name,value,false)}
    // java public static void addAttr(XmlWriter writer, String name, boolean value, boolean defaultValue) { if (value != defaultValue) { writer.addAttr(name, value); } }
    pub fn addAttr__bool_default(writer:&mut dyn XmlWriter,name:&str,value:bool,defaultValue:bool){if value!=defaultValue{writer.addAttr__bool(name,value);}}
}
