use std::{cell::RefCell,rc::Rc,ops::{Deref,DerefMut},any::Any};
use super::AbstractXmlWriter::{AbstractXmlWriter,AnsiOutput};
use crate::kfc::xpxml::writer::XmlWriter::XmlWriter;
// java public final class MemoryXmlWriter extends AbstractXmlWriter {
pub struct MemoryXmlWriter{
    base:AbstractXmlWriter,
    // java private StringBuilder builder = new StringBuilder();
    builder:Rc<RefCell<Vec<u16>>>,
}
impl MemoryXmlWriter{
    // java public MemoryXmlWriter() { super(); }
    pub fn new()->Self{let builder=Rc::new(RefCell::new(Vec::new()));Self{base:AbstractXmlWriter::with_output(Box::new(builder.clone())),builder}}
    // java protected void writeAnsi(String str) { this.builder.append(str); }
    pub fn write_ansi_str(&mut self,value:&str){self.builder.borrow_mut().extend(value.encode_utf16());}
    // java protected void writeAnsi(char ch) { this.builder.append(ch); }
    pub fn write_ansi_char(&mut self,ch:char){let mut units=[0;2];self.builder.borrow_mut().extend_from_slice(ch.encode_utf16(&mut units));}
    // java String getString() { return this.builder.toString(); }
    pub fn getString(&self)->String{String::from_utf16(&self.builder.borrow()).expect("Java UTF-16 string requires getCodeUnits transport")}
    pub fn getCodeUnits(&self)->Vec<u16>{self.builder.borrow().clone()}
}
impl Deref for MemoryXmlWriter{type Target=AbstractXmlWriter;fn deref(&self)->&Self::Target{&self.base}}
impl DerefMut for MemoryXmlWriter{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
impl XmlWriter for MemoryXmlWriter{
    fn base(&mut self)->&mut AbstractXmlWriter{&mut self.base}
    fn as_any(&self)->&dyn Any{self}
}
impl AnsiOutput for Rc<RefCell<Vec<u16>>>{
    fn writeAnsi(&mut self,value:&str){self.borrow_mut().extend(value.encode_utf16());}
    fn writeAnsiChar(&mut self,ch:char){let mut units=[0;2];self.borrow_mut().extend_from_slice(ch.encode_utf16(&mut units));}
    // java builder.append((String)null) appends the literal null.
    fn writeAnsiNullable(&mut self,value:Option<&str>){self.borrow_mut().extend(value.unwrap_or("null").encode_utf16());}
    fn writeAnsiUtf16(&mut self,value:&[u16]){self.borrow_mut().extend_from_slice(value);}
}
