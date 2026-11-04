use crate::kfc::xpxml::util::EscapeUtil::EscapeUtil;
use core_libs_kfc_main::java_runtime::FloatingDecimal::FloatingDecimal;

// rust: native virtual dispatch for the two Java protected abstract overloads.
pub trait AnsiOutput {
    fn writeAnsi(&mut self, value:&str);
    fn writeAnsiChar(&mut self, value:char);
    fn writeAnsiNullable(&mut self,value:Option<&str>){self.writeAnsi(value.expect("NullPointerException"));}
    // Native transport of Java String code units, including a substring ending in a surrogate.
    fn writeAnsiUtf16(&mut self,value:&[u16]){self.writeAnsi(&String::from_utf16(value).expect("UTF-16 output adapter unavailable"));}
}
impl AnsiOutput for String {
    fn writeAnsi(&mut self,value:&str){self.push_str(value);}
    fn writeAnsiChar(&mut self,value:char){self.push(value);}
    fn writeAnsiNullable(&mut self,value:Option<&str>){self.push_str(value.unwrap_or("null"));}
}

// Java: public class AbstractXmlWriter implements XmlWriter
pub struct AbstractXmlWriter {
    // Java field: boolean isWritingAttr
    isWritingAttr: bool,
    output: Box<dyn AnsiOutput>,
}

// Java: public class AbstractXmlWriter implements XmlWriter
impl AbstractXmlWriter {
    // Java: AbstractXmlWriter()
    pub fn new() -> Self {
        // Java: super();
        AbstractXmlWriter {
            isWritingAttr: false,
            output: Box::new(String::new()),
        }
    }

    // rust: constructor ownership adapter for Java subclass virtual writeAnsi.
    pub(crate) fn with_output(output:Box<dyn AnsiOutput>)->Self{Self{isWritingAttr:false,output}}

    // Java: void startDocument()
    pub fn start_document(&mut self) {
    }

    // Java: void endDocument()
    pub fn end_document(&mut self) {
    }

    // Java: void addNS(String prefix, String uri)
    pub fn add_ns(&mut self, prefix: Option<&str>, uri: &str) {
        // Java: this.writeAnsi(" xmlns");
        self.write_ansi_str(" xmlns");
        // Java: if (prefix != null)
        if let Some(p) = prefix {
            // Java: this.writeAnsi(':');
            self.write_ansi_char(':');
            // Java: this.writeAnsi(prefix);
            self.write_ansi_str(p);
        }
        // Java: this.writeAnsi('=');
        self.write_ansi_char('=');
        // Java: this.writeAnsi('"');
        self.write_ansi_char('"');
        // Java: this.writeAnsi(uri);
        self.write_ansi_str(uri);
        // Java: this.writeAnsi('"');
        self.write_ansi_char('"');
    }

    // Java: void startElement(String prefix, String name)
    pub fn start_element_ns(&mut self, prefix: Option<&str>, name: &str) {
        // Java: if (this.isWritingAttr)
        if self.isWritingAttr {
            // Java: this.writeAnsi('>');
            self.write_ansi_char('>');
        }
        // Java: this.writeAnsi('<');
        self.write_ansi_char('<');
        // Java: if (prefix != null)
        if let Some(p) = prefix {
            // Java: this.writeAnsi(prefix);
            self.write_ansi_str(p);
            // Java: this.writeAnsi(':');
            self.write_ansi_char(':');
        }
        // Java: this.writeAnsi(name);
        self.write_ansi_str(name);
        // Java: this.isWritingAttr = true;
        self.isWritingAttr = true;
    }

    // Java: void startElement(String name)
    pub fn start_element(&mut self, name: &str) {
        // Java: this.startElement((String)null, name);
        self.start_element_ns(None, name);
    }

    // Java: void endElement(String prefix, String name)
    pub fn end_element_ns(&mut self, prefix: Option<&str>, name: &str) {
        // Java: if (this.isWritingAttr)
        if self.isWritingAttr {
            // Java: this.writeAnsi('/');
            self.write_ansi_char('/');
            // Java: this.writeAnsi('>');
            self.write_ansi_char('>');
            // Java: this.isWritingAttr = false;
            self.isWritingAttr = false;
        }
        // Java: else
        else {
            // Java: this.writeAnsi('<');
            self.write_ansi_char('<');
            // Java: this.writeAnsi('/');
            self.write_ansi_char('/');
            // Java: if (prefix != null)
            if let Some(p) = prefix {
                // Java: this.writeAnsi(prefix);
                self.write_ansi_str(p);
                // Java: this.writeAnsi(':');
                self.write_ansi_char(':');
            }
            // Java: this.writeAnsi(name);
            self.write_ansi_str(name);
            // Java: this.writeAnsi('>');
            self.write_ansi_char('>');
        }
    }

    // Java: void endElement(String name)
    pub fn end_element(&mut self, name: &str) {
        // Java: this.endElement((String)null, name);
        self.end_element_ns(None, name);
    }

    // Java: void addAttr(String prefix, String name, String value)
    pub fn add_attr_ns(&mut self, prefix: Option<&str>, name: &str, value: &str) {
        // Java: this.addAttrImpl(prefix, name, EscapeUtil.escapeCharacter(value));
        let escaped = EscapeUtil::escape_character(value);
        self.add_attr_impl(prefix, Some(name), Some(&escaped));
    }

    // Java: void addAttr(String name, String value)
    pub fn add_attr_str(&mut self, name: &str, value: &str) {
        // Java: this.addAttr((String)null, name, value);
        self.add_attr_ns(None, name, value);
    }

    // java public void addAttr(String prefix, String name, String value) { this.addAttrImpl(prefix, name, EscapeUtil.escapeCharacter(value)); }
    pub fn add_attr_ns_nullable(&mut self,prefix:Option<&str>,name:Option<&str>,value:Option<&str>){
        let escaped=value.map(EscapeUtil::escape_character);self.add_attr_impl(prefix,name,escaped.as_deref());
    }

    // Java: void addAttrImpl(String prefix, String name, String value)
    fn add_attr_impl(&mut self, prefix: Option<&str>, name: Option<&str>, value: Option<&str>) {
        // Java: if (name != null && value != null)
        if let (Some(name),Some(value))=(name,value) {
            // Java: this.writeAnsi(' ');
            self.write_ansi_char(' ');
            // Java: if (prefix != null)
            if let Some(p) = prefix {
                // Java: this.writeAnsi(prefix);
                self.write_ansi_str(p);
                // Java: this.writeAnsi(':');
                self.write_ansi_char(':');
            }
            // Java: this.writeAnsi(name);
            self.write_ansi_str(name);
            // Java: this.writeAnsi('=');
            self.write_ansi_char('=');
            // Java: this.writeAnsi('"');
            self.write_ansi_char('"');
            // Java: this.writeAnsi(value);
            self.write_ansi_str(value);
            // Java: this.writeAnsi('"');
            self.write_ansi_char('"');
        }
    }

    // Java: void addAttr(String name, float value)
    pub fn add_attr_f32(&mut self, name: &str, value: f32) {
        // Java: if (value == Float.POSITIVE_INFINITY)
        if value == f32::INFINITY {
            // Java: this.addAttrImpl((String)null, name, "INF");
            self.add_attr_impl(None, Some(name), Some("INF"));
        }
        // Java: else if (value == Float.NEGATIVE_INFINITY)
        else if value == f32::NEG_INFINITY {
            // Java: this.addAttrImpl((String)null, name, "-INF");
            self.add_attr_impl(None, Some(name), Some("-INF"));
        }
        // Java: else
        else {
            // Java: this.addAttrImpl((String)null, name, Float.toString(value));
            self.add_attr_impl(None, Some(name), Some(&core_libs_kfc_main::java_runtime::FloatToDecimal::FloatToDecimal::toString(value)));
        }
    }

    // Java: void addAttr(String name, double value)
    pub fn add_attr_f64(&mut self, name: &str, value: f64) {
        // Java: if (value == Double.POSITIVE_INFINITY)
        if value == f64::INFINITY {
            // Java: this.addAttrImpl((String)null, name, "INF");
            self.add_attr_impl(None, Some(name), Some("INF"));
        }
        // Java: else if (value == Double.NEGATIVE_INFINITY)
        else if value == f64::NEG_INFINITY {
            // Java: this.addAttrImpl((String)null, name, "-INF");
            self.add_attr_impl(None, Some(name), Some("-INF"));
        }
        // Java: else
        else {
            // Java: this.addAttrImpl((String)null, name, Double.toString(value));
            self.add_attr_impl(None, Some(name), Some(&FloatingDecimal::toJavaFormatString(value)));
        }
    }

    // Java: void addAttr(String name, int value)
    pub fn add_attr_i32(&mut self, name: &str, value: i32) {
        // Java: this.addAttrImpl((String)null, name, Integer.toString(value));
        self.add_attr_impl(None, Some(name), Some(&value.to_string()));
    }

    // Java: void addAttr(String name, long value)
    pub fn add_attr_i64(&mut self, name: &str, value: i64) {
        // Java: this.addAttrImpl((String)null, name, Long.toString(value));
        self.add_attr_impl(None, Some(name), Some(&value.to_string()));
    }

    // Java: void addAttr(String name, short value)
    pub fn add_attr_i16(&mut self, name: &str, value: i16) {
        // Java: this.addAttrImpl((String)null, name, Short.toString(value));
        self.add_attr_impl(None, Some(name), Some(&value.to_string()));
    }

    // Java: void addAttr(String name, boolean value)
    pub fn add_attr_bool(&mut self, name: &str, value: bool) {
        // Java: if (value)
        if value {
            // Java: this.addAttrImpl((String)null, name, "1");
            self.add_attr_impl(None, Some(name), Some("1"));
        }
        // Java: else
        else {
            // Java: this.addAttrImpl((String)null, name, "0");
            self.add_attr_impl(None, Some(name), Some("0"));
        }
    }

    // Java: void addText(String value)
    pub fn add_text(&mut self, value: &str) {
        // Java: this.addText(value, true);
        self.add_text_with_escape(value, true);
    }
    // Native nullable transport of addText(String), retaining virtual writeAnsi(null).
    pub fn add_text_nullable(&mut self,value:Option<&str>){
        if let Some(value)=value{self.add_text(value);}else{
            // java EscapeUtil.escapeCharacter(null) returns null; addTextImpl closes the attribute before writeAnsi(value).
            if self.isWritingAttr{self.write_ansi_char('>');self.isWritingAttr=false;}
            self.output.writeAnsiNullable(None);
        }
    }
    // Same addText(String) / addTextImpl route with lossless Java UTF-16 transport.
    pub fn add_text_utf16(&mut self,value:&[u16]){
        // java this.addTextImpl(EscapeUtil.escapeCharacter(value));
        let value=EscapeUtil::escapeCharacterUtf16(value);
        // java if (this.isWritingAttr) { this.writeAnsi('>'); this.isWritingAttr = false; }
        if self.isWritingAttr{self.write_ansi_char('>');self.isWritingAttr=false;}
        // java this.writeAnsi(value);
        self.output.writeAnsiUtf16(&value);
    }

    // Java: void addText(String value, boolean isEscapeCharacter)
    pub fn add_text_with_escape(&mut self, value: &str, is_escape_character: bool) {
        // Java: if (isEscapeCharacter)
        if is_escape_character {
            // Java: this.addTextImpl(EscapeUtil.escapeCharacter(value));
            let escaped = EscapeUtil::escape_character(value);
            self.add_text_impl(&escaped);
        }
        // Java: else
        else {
            // Java: this.addTextImpl(value);
            self.add_text_impl(value);
        }
    }

    // Java: void addTextImpl(String value)
    fn add_text_impl(&mut self, value: &str) {
        // Java: if (this.isWritingAttr)
        if self.isWritingAttr {
            // Java: this.writeAnsi('>');
            self.write_ansi_char('>');
            // Java: this.isWritingAttr = false;
            self.isWritingAttr = false;
        }
        // Java: this.writeAnsi(value);
        self.write_ansi_str(value);
    }

    // Java: void addText(int value)
    pub fn add_text_i32(&mut self, value: i32) {
        // Java: this.addTextImpl(Integer.toString(value));
        self.add_text_impl(&value.to_string());
    }

    // Java: void addText(short value)
    pub fn add_text_i16(&mut self, value: i16) {
        // Java: this.addTextImpl(Short.toString(value));
        self.add_text_impl(&value.to_string());
    }

    // Java: void addText(boolean value)
    pub fn add_text_bool(&mut self, value: bool) {
        // Java: if (value)
        if value {
            // Java: this.addTextImpl("1");
            self.add_text_impl("1");
        }
        // Java: else
        else {
            // Java: this.addTextImpl("0");
            self.add_text_impl("0");
        }
    }

    // Java: void flush()
    pub fn flush(&mut self) {
    }

    // Java: void pushWriter(XmlWriter labelWriter)
    pub fn push_writer(&mut self, _label_writer: Box<dyn std::any::Any>) {
    }

    // Java: abstract void writeAnsi(String var1)
    pub fn write_ansi_str(&mut self, var1: &str) {
        self.output.writeAnsi(var1);
    }

    // Java: abstract void writeAnsi(char var1)
    pub fn write_ansi_char(&mut self, var1: char) {
        self.output.writeAnsiChar(var1);
    }
}
