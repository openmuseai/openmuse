use kfc_kfc_alpha::kfc::xpxml::{writer::{XmlWriter::XmlWriter,r#impl::{BufferXmlWriter::BufferXmlWriter,MemoryXmlWriter::MemoryXmlWriter}},util::EscapeUtil::{EscapeUtil,CONTROL_CHARS,CONTROL_CHARS_V2}};
use std::{cell::RefCell,rc::Rc,io::{self,Write}};
#[derive(Clone)]struct Output(Rc<RefCell<Vec<u8>>>);
impl Write for Output{fn write(&mut self,b:&[u8])->io::Result<usize>{self.0.borrow_mut().extend_from_slice(b);Ok(b.len())}fn flush(&mut self)->io::Result<()>{Ok(())}}
fn decode(hex:&str)->Vec<u16>{(0..hex.len()).step_by(4).map(|i|u16::from_str_radix(&hex[i..i+4],16).unwrap()).collect()}
fn hex(value:&[u16])->String{value.iter().map(|v|format!("{v:04x}")).collect()}
fn text_hex(value:Option<String>)->String{value.map_or("NULL".into(),|v|hex(&v.encode_utf16().collect::<Vec<_>>()))}
#[test]
fn original_java_xml_writer_and_escape(){
    let path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/xml-writer-r0b26");
    let source=std::fs::read_to_string(path.join("expected-android.tsv")).unwrap();let mut rust=vec![];
    for (i,line) in source.lines().enumerate(){
        let a:Vec<_>=line.split('\t').collect();let s=if a[2]=="NULL"{None}else{Some(String::from_utf16(&decode(a[2])).unwrap())};
        let result=if a[0]=="E"{
            let v=s.as_deref();format!("{}\t{}\t{}\t{}\t{}",EscapeUtil::needEscapeCharacter(v),text_hex(EscapeUtil::escapeCharacter(v)),
                v.map_or("NULL".into(),|v|hex(&EscapeUtil::unEscapeControlCharacter(&v.encode_utf16().collect::<Vec<_>>()))),
                text_hex(v.map(|v|EscapeUtil::escapeControlCharacter(v,CONTROL_CHARS))),text_hex(v.map(|v|EscapeUtil::escapeControlCharacter(v,CONTROL_CHARS_V2))))
        }else{
            let s=s.unwrap();let output=Output(Rc::new(RefCell::new(vec![])));let mut w:Box<dyn XmlWriter>=Box::new(BufferXmlWriter::new_with_escape(Box::new(output.clone()),a[1]=="1"));
            w.startDocument();w.startElement__ns(Some("p"),"root");w.addNS(None,"urn:main");w.addNS(Some("p"),&s);
            w.addAttr("name",&s);w.addAttr("",&s);w.addAttr__ns(Some("p"),"value",&s);
            w.addAttr__f64("double",f64::from_bits(u64::from_str_radix(a[3],16).unwrap()));w.addAttr__f32("float",f32::from_bits(u32::from_str_radix(a[4],16).unwrap()));
            w.addAttr__i32("int",i32::MIN);w.addAttr__i64("long",i64::MIN);w.addAttr__i16("short",-2);w.addAttr__bool("true",true);w.addAttr__bool("false",false);
            w.startElement("empty");w.endElement("empty");w.startElement("text");w.addText(&s);w.addText__escape(&s,false);w.addText__i32(-3);w.addText__i16(4);w.addText__bool(true);w.addText__bool(false);w.endElement("text");
            let mut memory=MemoryXmlWriter::new();memory.startElement("m");memory.addText(&s);memory.endElement("m");w.pushWriter(Box::new(memory));
            w.pushWriter(Box::new(BufferXmlWriter::new(Box::new(Vec::<u8>::new()))));w.endElement__ns(Some("p"),"root");w.endDocument();
            let bytes=output.0.borrow();let text=std::str::from_utf8(&bytes).unwrap();hex(&text.encode_utf16().collect::<Vec<_>>())
        };
        let split=if a[0]=="E"{3}else{5};assert_eq!(result,a[split..].join("\t"),"vector {i}: {:?}",&a[..split]);
        rust.push(format!("{}\t{}",a[..split].join("\t"),result));
    }
    std::fs::write(path.join("rust.tsv"),rust.join("\n")+"\n").unwrap();
}
struct FailedOutput(bool);
impl Write for FailedOutput{
    fn write(&mut self,_:&[u8])->io::Result<usize>{if self.0{Err(io::Error::other("write failed"))}else{Ok(1)}}
    fn flush(&mut self)->io::Result<()>{Err(io::Error::other("flush failed"))}
}
#[test]
fn original_writer_io_exception_is_not_swallowed(){
    for write in [false,true]{
        let mut w=BufferXmlWriter::new(Box::new(FailedOutput(write)));
        w.start_document();
        let failure=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||w.end_document())).unwrap_err();
        assert!(failure.downcast_ref::<String>().unwrap().starts_with("RuntimeException:"));
    }
}
