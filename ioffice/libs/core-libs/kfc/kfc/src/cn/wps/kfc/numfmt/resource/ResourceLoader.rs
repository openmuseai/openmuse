use std::{collections::HashMap,sync::Mutex};
use super::{IResourceParser::IResourceParser,ResourceSaxParser::ResourceSaxParser};
// java public final class ResourceLoader
pub struct ResourceLoader;
// java public interface IResOpenHandler
pub trait IResOpenHandlerTrait {fn open(&self,name:&str)->Option<Vec<u8>>;}
static OPEN_HANDLER:Mutex<Option<Box<dyn IResOpenHandlerTrait+Send>>>=Mutex::new(None);
impl ResourceLoader {
    // java mResOpenHandler=openHandler;
    pub fn setOpenHandler(openHandler:Box<dyn IResOpenHandlerTrait+Send>){*OPEN_HANDLER.lock().unwrap_or_else(std::sync::PoisonError::into_inner)=Some(openHandler);}
    // java ResourceSaxParser parser=Factory.newInstance();return parser!=null&&parser.parse(getInputStream(resName))?parser:null;
    pub fn getSaxParser(resName:&str)->Option<ResourceSaxParser>{let bytes=Self::getInputStream(resName)?;let mut parser=ResourceSaxParser::new();if parser.parse(&mut bytes.as_slice()){Some(parser)}else{None}}
    // java public static IResourceParser getDomParser(String resName)
    // rust: Retained DOM facade; the Excel read closure uses the original SAX branch.
    pub fn getDomParser(_resName:&str)->Option<Box<dyn IResourceParser>>{todo!("ResourceDomParser not ported")}
    pub fn getSaxParserCache(resName:&str)->Option<HashMap<String,Vec<String>>>{Self::getSaxParser(resName).map(|p|p.getCache())}
    // java handler.open(resName+".xml") or class.getResource(resName+".xml").openStream().
    pub fn getInputStream(resName:&str)->Option<Vec<u8>>{
        if let Some(h)=OPEN_HANDLER.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref(){return h.open(&(resName.to_string()+".xml"));}
        // Rust classpath adapter: package resources are embedded unchanged.
        let data:&str=match resName {
"builtinfmt_0409"=>include_str!("data/builtinfmt_0409.xml"),
"builtinfmt_0411"=>include_str!("data/builtinfmt_0411.xml"),
"builtinfmt_0804"=>include_str!("data/builtinfmt_0804.xml"),
"locale_0409"=>include_str!("data/locale_0409.xml"),
"locale_0411"=>include_str!("data/locale_0411.xml"),
"locale_0804"=>include_str!("data/locale_0804.xml"),
"numeral_shape"=>include_str!("data/numeral_shape.xml"),
"token_0409"=>include_str!("data/token_0409.xml"),
"token_0411"=>include_str!("data/token_0411.xml"),
"token_0804"=>include_str!("data/token_0804.xml"),
_=>return None};Some(data.as_bytes().to_vec())
    }
    // java return parser.getArray(name);
    pub fn getArray_from_parser(parser:&dyn IResourceParser,name:&str)->Option<Vec<String>>{Some(parser.getArray(name))}
    pub fn getArray_from_properties(p:&HashMap<String,Vec<String>>,name:&str)->Option<Vec<String>>{p.get(name).cloned()}
    // java return parser.getItem(name);
    pub fn getItem_from_parser(parser:&dyn IResourceParser,name:&str)->Option<String>{Some(parser.getItem(name))}
    pub fn getItem_from_properties(p:&HashMap<String,String>,name:&str)->Option<String>{p.get(name).cloned()}
    // java new String(str.getBytes("iso8859-1"),"utf-8")
    pub fn changeCharset(s:Option<&str>)->String{let Some(s)=s else{return String::new()};let bytes=s.chars().map(|c|if c as u32<=255{c as u8}else{b'?'}).collect::<Vec<_>>();String::from_utf8_lossy(&bytes).into_owned()}
}
