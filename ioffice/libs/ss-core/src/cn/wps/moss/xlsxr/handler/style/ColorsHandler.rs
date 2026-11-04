use crate::{java_runtime::Rc,cn::wps::moss::app::KmoBook::KmoBook};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use super::IndexedColorsHandler::IndexedColorsHandler;
// java package cn.wps.moss.xlsxr.handler.style;
// java
// java import cn.wps.kfc.xpxml.reader.DefaultElementHandler;
// java import cn.wps.kfc.xpxml.reader.ElementHandler;
// java import cn.wps.moss.app.KmoBook;
// java
// java public final class ColorsHandler extends DefaultElementHandler {
// java private IndexedColorsHandler idxColorsHandler;
// java
// java public ColorsHandler(KmoBook book) {
// java this.idxColorsHandler = new IndexedColorsHandler(book);
// java }
// java
// java public ElementHandler enterSubElement(int subElement) {
// java return subElement == 5178 ? this.idxColorsHandler : null;
// java }
// java }

pub struct ColorsHandler {idxColorsHandler:Rc<IndexedColorsHandler>}
impl ColorsHandler {pub fn new(book:Rc<KmoBook>)->Self{Self{idxColorsHandler:Rc::new(IndexedColorsHandler::new(book))}}}
struct IndexRef(Rc<IndexedColorsHandler>);
impl ElementHandler for IndexRef {fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.0.startElement(e,a);}fn endElement(&self,e:i32){self.0.endElement(e);}fn characters(&self,s:&str){self.0.characters(s);}fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.0.enterSubElement(e)}}
impl ElementHandler for ColorsHandler {fn startElement(&self,_:i32,_:&dyn XmlRoAttr){}fn endElement(&self,_:i32){}fn characters(&self,_:&str){}fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==5178{Some(Box::new(IndexRef(self.idxColorsHandler.clone())))}else{None}}}
