use std::{cell::RefCell,io::{Write,BufWriter},rc::Rc,ops::{Deref,DerefMut},any::Any};
use super::{AbstractXmlWriter::{AbstractXmlWriter,AnsiOutput},MemoryXmlWriter::MemoryXmlWriter};
use crate::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java static final String escape_enter = "&#xA;";
pub(crate) const ESCAPE_ENTER:&str="&#xA;";
// java public final class BufferXmlWriter extends AbstractXmlWriter {
pub struct BufferXmlWriter{
    base:AbstractXmlWriter,
    // java private Writer writer;
    writer:Rc<RefCell<BufferedOutput>>,
    // java private boolean replaceEscape;
    // rust: stored in the virtual output with writer; inherited calls share it.
}
struct BufferedOutput{writer:BufWriter<Box<dyn Write>>,replaceEscape:bool,pendingHigh:Option<u16>}
// Native ownership: Java BufferedWriter is flushed by flush/endDocument only.
// GC after an exception does not flush its pending characters. Rust BufWriter's
// automatic Drop flush would otherwise add partial XML to a ZIP entry in finally.
impl Drop for BufferedOutput{
    fn drop(&mut self){
        let pending=std::mem::replace(&mut self.writer,BufWriter::new(Box::new(std::io::sink())));
        let (_output,_unflushed)=pending.into_parts();
    }
}
impl BufferXmlWriter{
    // java public BufferXmlWriter(OutputStream out) { this(out, false); }
    pub fn new(out:Box<dyn Write>)->Self{Self::new_with_escape(out,false)}
    // java public BufferXmlWriter(OutputStream out, boolean replaceEscape) {
    pub fn new_with_escape(out:Box<dyn Write>,replaceEscape:bool)->Self{
        // java super(); this.writer = null; this.replaceEscape = false;
        // java this.writer = new BufferedWriter(new OutputStreamWriter(out, Charset.forName("UTF-8")));
        // java this.replaceEscape = replaceEscape;
        let writer=Rc::new(RefCell::new(BufferedOutput{writer:BufWriter::new(out),replaceEscape,pendingHigh:None}));
        Self{base:AbstractXmlWriter::with_output(Box::new(writer.clone())),writer}
    }
    // java public void startDocument() {
    pub fn start_document(&mut self){
        // java this.writeAnsiStr("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
        self.writer.borrow_mut().writeAnsiStr("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
    }
    // java public void endDocument() { this.flush(); }
    pub fn end_document(&mut self){self.flush();}
    // java public void flush() { try { this.writer.flush(); } catch (IOException e) { throw new RuntimeException(e); } }
    pub fn flush(&mut self){self.writer.borrow_mut().writer.flush().unwrap_or_else(|e|panic!("RuntimeException: {e}"));}
    // java public void pushWriter(XmlWriter labelWriter) {
    pub fn push_writer(&mut self,labelWriter:Box<dyn XmlWriter>){
        // java if (labelWriter instanceof MemoryXmlWriter) { MemoryXmlWriter tmp = (MemoryXmlWriter)labelWriter; this.writeAnsi(tmp.getString()); }
        if let Some(tmp)=labelWriter.as_any().downcast_ref::<MemoryXmlWriter>(){self.writer.clone().writeAnsiUtf16(&tmp.getCodeUnits());}
    }
    // java protected void writeAnsi(String str) {
    pub fn write_ansi_str(&mut self,value:&str){self.writer.borrow_mut().writeAnsi(value);}
    // java protected void writeAnsi(char ch) {
    pub fn write_ansi_char(&mut self,ch:char){self.writer.borrow_mut().writeAnsiChar(ch);}
}
// rust: subclass inheritance; there is one XML state and one virtual output.
impl Deref for BufferXmlWriter{type Target=AbstractXmlWriter;fn deref(&self)->&Self::Target{&self.base}}
impl DerefMut for BufferXmlWriter{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
impl XmlWriter for BufferXmlWriter{
    fn base(&mut self)->&mut AbstractXmlWriter{&mut self.base}
    fn as_any(&self)->&dyn Any{self}
    fn start_document(&mut self){BufferXmlWriter::start_document(self);}
    fn end_document(&mut self){BufferXmlWriter::end_document(self);}
    fn flush(&mut self){BufferXmlWriter::flush(self);}
    fn push_writer(&mut self,label:Box<dyn XmlWriter>){BufferXmlWriter::push_writer(self,label);}
}
impl BufferedOutput{
    // java protected void writeAnsi(String str) {
    fn writeAnsi(&mut self,value:&str){
        // java if (this.replaceEscape) { this.writeAnsiImpl(str); } else { this.writeAnsiStr(str); }
        if self.replaceEscape{self.writeAnsiImpl(value);}else{self.writeAnsiStr(value);}
    }
    // java private void writeAnsiStr(String str) {
    fn writeAnsiStr(&mut self,value:&str){
        // java try { this.writer.write(str); } catch (IOException e) { throw new RuntimeException(e); }
        self.encodeUtf16(&value.encode_utf16().collect::<Vec<_>>());
    }
    // java private void writeAnsiImpl(String str) {
    fn writeAnsiImpl(&mut self,value:&str){
        // java int len; if (str != null && (len = str.length()) != 0) {
        if !value.is_empty(){
            // java int index = 0; StringBuilder sb = new StringBuilder(len);
            let mut sb=String::with_capacity(value.len());
            // java while(index < len) { char c = str.charAt(index++); this.writeSingleEscapeChar(sb, c); }
            for c in value.chars(){Self::writeSingleEscapeChar(&mut sb,c);}
            // java this.writeAnsiStr(sb.toString());
            self.writeAnsiStr(&sb);
        }
    }
    // java private void writeSingleEscapeChar(StringBuilder sb, char c) {
    fn writeSingleEscapeChar(sb:&mut String,c:char){
        // java switch(c) { case '\n': sb.append("&#xA;"); case '\r': break; default: sb.append(c); }
        match c{'\n'=>sb.push_str(ESCAPE_ENTER),'\r'=>{},_=>sb.push(c)}
    }
    // java protected void writeAnsi(char ch) {
    fn writeAnsiChar(&mut self,ch:char){
        // java try { this.writer.append(ch); } catch (IOException e) { throw new RuntimeException(e); }
        let mut units=[0;2];self.encodeUtf16(ch.encode_utf16(&mut units));
    }
    // Native OutputStreamWriter UTF-8 encoder leaf. Java replaces malformed
    // surrogate sequences with '?', and retains a high surrogate between writes.
    fn encodeUtf16(&mut self,value:&[u16]){for &unit in value{
        if let Some(high)=self.pendingHigh.take(){if (0xdc00..=0xdfff).contains(&unit){let scalar=0x10000+((high as u32-0xd800)<<10)+(unit as u32-0xdc00);let mut bytes=[0;4];self.writer.write_all(char::from_u32(scalar).unwrap().encode_utf8(&mut bytes).as_bytes()).unwrap_or_else(|e|panic!("RuntimeException: {e}"));continue;}self.writer.write_all(b"?").unwrap_or_else(|e|panic!("RuntimeException: {e}"));}
        if (0xd800..=0xdbff).contains(&unit){self.pendingHigh=Some(unit);}else if (0xdc00..=0xdfff).contains(&unit){self.writer.write_all(b"?").unwrap_or_else(|e|panic!("RuntimeException: {e}"));}else{let mut bytes=[0;4];self.writer.write_all(char::from_u32(unit as u32).unwrap().encode_utf8(&mut bytes).as_bytes()).unwrap_or_else(|e|panic!("RuntimeException: {e}"));}
    }}
}
impl AnsiOutput for Rc<RefCell<BufferedOutput>>{
    fn writeAnsi(&mut self,value:&str){self.borrow_mut().writeAnsi(value);}
    // java replaceEscape delegates null to writeAnsiImpl (no output); writer.write(null) otherwise throws.
    fn writeAnsiNullable(&mut self,value:Option<&str>){let mut output=self.borrow_mut();match value{Some(v)=>output.writeAnsi(v),None=>if !output.replaceEscape{panic!("NullPointerException");}}}
    fn writeAnsiChar(&mut self,ch:char){self.borrow_mut().writeAnsiChar(ch);}
    fn writeAnsiUtf16(&mut self,value:&[u16]){let mut output=self.borrow_mut();if output.replaceEscape{let mut units=vec![];for &unit in value{match unit{10=>units.extend(ESCAPE_ENTER.encode_utf16()),13=>{},_=>units.push(unit)}}output.encodeUtf16(&units);}else{output.encodeUtf16(value);}}
}
