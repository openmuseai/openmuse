use super::r#impl::AbstractXmlWriter::AbstractXmlWriter;
use std::any::Any;
// java public interface XmlWriter {
// rust: default forwarding is Java abstract-class inheritance; subclasses override
// start/end/flush/push as in the original. camelCase names preserve the Java facade;
// snake_case names remain compatible with the pre-existing shared-crate callers.
pub trait XmlWriter:Any {
    fn base(&mut self)->&mut AbstractXmlWriter;
    fn as_any(&self)->&dyn Any;
    // java void startDocument();
    fn startDocument(&mut self){self.start_document();}
    fn start_document(&mut self){self.base().start_document();}
    // java void endDocument();
    fn endDocument(&mut self){self.end_document();}
    fn end_document(&mut self){self.base().end_document();}
    // java void addNS(String prefix, String uri);
    fn addNS(&mut self,prefix:Option<&str>,uri:&str){self.add_ns(prefix,uri);}
    fn add_ns(&mut self,prefix:Option<&str>,uri:&str){self.base().add_ns(prefix,uri);}
    // java void startElement(String prefix, String name);
    fn startElement__ns(&mut self,prefix:Option<&str>,name:&str){self.start_element_ns(prefix,name);}
    fn start_element_ns(&mut self,prefix:Option<&str>,name:&str){self.base().start_element_ns(prefix,name);}
    // java void startElement(String name);
    fn startElement(&mut self,name:&str){self.start_element(name);}
    fn start_element(&mut self,name:&str){self.base().start_element(name);}
    // java void endElement(String prefix, String name);
    fn endElement__ns(&mut self,prefix:Option<&str>,name:&str){self.end_element_ns(prefix,name);}
    fn end_element_ns(&mut self,prefix:Option<&str>,name:&str){self.base().end_element_ns(prefix,name);}
    // java void endElement(String name);
    fn endElement(&mut self,name:&str){self.end_element(name);}
    fn end_element(&mut self,name:&str){self.base().end_element(name);}
    // java void addAttr(String prefix, String name, String value);
    fn addAttr__ns(&mut self,prefix:Option<&str>,name:&str,value:&str){self.add_attr_ns(prefix,name,value);}
    fn add_attr_ns(&mut self,prefix:Option<&str>,name:&str,value:&str){self.base().add_attr_ns(prefix,name,value);}
    // java void addAttr(String name, String value);
    fn addAttr(&mut self,name:&str,value:&str){self.add_attr_str(name,value);}
    // Native nullable String facade of the same Java addAttr overload.
    fn addAttrNullable(&mut self,name:Option<&str>,value:Option<&str>){self.base().add_attr_ns_nullable(None,name,value);}
    fn addAttr__nsNullable(&mut self,prefix:Option<&str>,name:Option<&str>,value:Option<&str>){self.base().add_attr_ns_nullable(prefix,name,value);}
    fn add_attr_str(&mut self,name:&str,value:&str){self.base().add_attr_str(name,value);}
    // java void addAttr(String name, int value);
    fn addAttr__i32(&mut self,name:&str,value:i32){self.add_attr_i32(name,value);}
    fn add_attr_i32(&mut self,name:&str,value:i32){self.base().add_attr_i32(name,value);}
    // java void addAttr(String name, float value);
    fn addAttr__f32(&mut self,name:&str,value:f32){self.add_attr_f32(name,value);}
    fn add_attr_f32(&mut self,name:&str,value:f32){self.base().add_attr_f32(name,value);}
    // java void addAttr(String name, double value);
    fn addAttr__f64(&mut self,name:&str,value:f64){self.add_attr_f64(name,value);}
    fn add_attr_f64(&mut self,name:&str,value:f64){self.base().add_attr_f64(name,value);}
    // java void addAttr(String name, short value);
    fn addAttr__i16(&mut self,name:&str,value:i16){self.add_attr_i16(name,value);}
    fn add_attr_i16(&mut self,name:&str,value:i16){self.base().add_attr_i16(name,value);}
    // java void addAttr(String name, boolean value);
    fn addAttr__bool(&mut self,name:&str,value:bool){self.add_attr_bool(name,value);}
    fn add_attr_bool(&mut self,name:&str,value:bool){self.base().add_attr_bool(name,value);}
    // java void addAttr(String name, long value);
    fn addAttr__i64(&mut self,name:&str,value:i64){self.add_attr_i64(name,value);}
    fn add_attr_i64(&mut self,name:&str,value:i64){self.base().add_attr_i64(name,value);}
    // java void addText(String value);
    fn addText(&mut self,value:&str){self.add_text(value);}
    fn addTextNullable(&mut self,value:Option<&str>){self.base().add_text_nullable(value);}
    fn addTextUtf16(&mut self,value:&[u16]){self.base().add_text_utf16(value);}
    fn add_text(&mut self,value:&str){self.base().add_text(value);}
    // java void addText(String value, boolean isEscapeCharacter);
    fn addText__escape(&mut self,value:&str,escape:bool){self.add_text_with_escape(value,escape);}
    fn add_text_with_escape(&mut self,value:&str,escape:bool){self.base().add_text_with_escape(value,escape);}
    // java void addText(int value);
    fn addText__i32(&mut self,value:i32){self.add_text_i32(value);}
    fn add_text_i32(&mut self,value:i32){self.base().add_text_i32(value);}
    // java void addText(short value);
    fn addText__i16(&mut self,value:i16){self.add_text_i16(value);}
    fn add_text_i16(&mut self,value:i16){self.base().add_text_i16(value);}
    // java void addText(boolean value);
    fn addText__bool(&mut self,value:bool){self.add_text_bool(value);}
    fn add_text_bool(&mut self,value:bool){self.base().add_text_bool(value);}
    // java void flush();
    fn flush(&mut self){self.base().flush();}
    // java void pushWriter(XmlWriter labelWriter);
    fn pushWriter(&mut self,labelWriter:Box<dyn XmlWriter>){self.push_writer(labelWriter);}
    fn push_writer(&mut self,labelWriter:Box<dyn XmlWriter>){self.base().push_writer(Box::new(labelWriter));}
}
