use std::{collections::HashMap,io::Read};
use super::IResourceParser::IResourceParser;
use quick_xml::{events::Event,Reader};
// java public class ResourceSaxParser extends DefaultHandler implements IResourceParser
#[derive(Clone,Default)]
pub struct ResourceSaxParser {pub(crate) mProperties:HashMap<String,Vec<String>>,mTypes:Vec<String>,mKeys:Vec<String>,mValueBuffer:String}
impl ResourceSaxParser {
    pub fn new()->Self {Self::default()}
    // java private void clear(): reset parser and all property/type/key/value state.
    fn clear(&mut self){self.mProperties.clear();self.mTypes.clear();self.mKeys.clear();self.mValueBuffer.clear();}
    // java startElement: clear value; push type and localName; allocate array if absent.
    fn startElement(&mut self,name:String,ty:Option<String>){self.mValueBuffer.clear();if let Some(ty)=ty{if ty=="string-array"{self.mProperties.entry(name.clone()).or_default();}self.mTypes.push(ty);self.mKeys.push(name);}}
    // java endElement: item appends value under first key; other elements pop type and key, assigning strings.
    fn endElement(&mut self,name:&str){if name=="item"{let key=self.mKeys.first().expect("resource item key");self.mProperties.get_mut(key).expect("resource item array").push(self.mValueBuffer.clone());}else if let Some(ty)=self.mTypes.pop(){if ty=="string"{self.mProperties.insert(name.into(),vec![self.mValueBuffer.clone()]);}self.mKeys.pop();}}
    pub fn getCache(&self)->HashMap<String,Vec<String>>{self.mProperties.clone()}
}
impl IResourceParser for ResourceSaxParser {
    // java this.clear();try{this.mParser.parse(input,this);return true;}catch(Exception e){return false;}
    fn parse(&mut self,input:&mut dyn Read)->bool{
        self.clear();let mut xml=String::new();if input.read_to_string(&mut xml).is_err(){return false;}let mut reader=Reader::from_str(&xml);
        loop{match reader.read_event(){
            Ok(Event::Start(e))=>{let name=String::from_utf8_lossy(e.local_name().as_ref()).into_owned();let ty=e.attributes().filter_map(Result::ok).find(|a|a.key.as_ref()==b"type").and_then(|a|a.unescape_value().ok().map(|s|s.into_owned()));self.startElement(name,ty);},
            // java SAX emits startElement and endElement for an empty XML tag too.
            Ok(Event::Empty(e))=>{let name=String::from_utf8_lossy(e.local_name().as_ref()).into_owned();let ty=e.attributes().filter_map(Result::ok).find(|a|a.key.as_ref()==b"type").and_then(|a|a.unescape_value().ok().map(|s|s.into_owned()));self.startElement(name.clone(),ty);self.endElement(&name);},
            Ok(Event::End(e))=>self.endElement(&String::from_utf8_lossy(e.local_name().as_ref())),
            // java this.mValueBuffer.append(ch,start,length);
            Ok(Event::Text(e))=>match e.unescape(){Ok(s)=>self.mValueBuffer.push_str(&s),Err(_)=>return false},
            Ok(Event::CData(e))=>self.mValueBuffer.push_str(&String::from_utf8_lossy(e.as_ref())),
            Ok(Event::Eof)=>return true,Err(_)=>return false,_=>{}
        }}
    }
    // java return (String)this.mProperties.get(nodeName);
    fn getItem(&self,key:&str)->String{self.mProperties.get(key).and_then(|v|v.first()).cloned().unwrap_or_default()}
    // java return (List)this.mProperties.get(nodeName);
    fn getArray(&self,key:&str)->Vec<String>{self.mProperties.get(key).cloned().unwrap_or_default()}
    // java public void destroy(): clear and release parser state.
    fn destroy(&mut self){self.clear();}
}
// java public static class Factory
pub struct Factory;
impl Factory {pub fn newInstance()->Option<ResourceSaxParser>{Some(ResourceSaxParser::new())}}
