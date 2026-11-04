use crate::{java_runtime::{Rc,Weak,RefCell,Cell},cn::wps::moss::{app::KmoBook::KmoBook,filefmt::biff8::record::FontRecord::FontRecord,xlsxr::util::ElementHandlerRef::hold}};
use super::{RichTextBuilder::RichTextBuilder,super::style::FontHandler::FontHandler};
use kfc_kfc_alpha::kfc::xpxml::{reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr,ElementAttr::ElementAttr},util::EscapeUtil::EscapeUtil};
// java class RunHandler extends DefaultElementHandler {
pub struct RunHandler{
    // java private valueHandler tHandler;
    tHandler:Rc<valueHandler>,
    // java private FontHandler fontHandler;
    fontHandler:RefCell<Option<std::rc::Rc<FontHandler>>>,
    // java private KmoBook book;
    book:Rc<KmoBook>,
    // java private FontRecord font;
    font:RefCell<Option<Rc<RefCell<FontRecord>>>>,
    // java private String value;
    value:RefCell<Option<String>>,
    // java private RichTextBuilder builder;
    builder:RefCell<Option<Rc<RefCell<RichTextBuilder>>>>,
    // java private int fontId;
    fontId:Cell<i32>,
    selfRef:Weak<Self>,
}
impl RunHandler{
    // java public RunHandler(KmoBook book){this.book=book;this.tHandler=new valueHandler();}
    pub fn RunHandler(book:Rc<KmoBook>)->Rc<Self>{Rc::new_cyclic(|s|Self{tHandler:Rc::new(valueHandler{outer:s.clone()}),fontHandler:RefCell::new(None),book,font:RefCell::new(None),value:RefCell::new(None),builder:RefCell::new(None),fontId:Cell::new(0),selfRef:s.clone()})}
    // java public void init(RichTextBuilder builder){this.builder=builder;}
    pub fn init(&self,builder:Option<Rc<RefCell<RichTextBuilder>>>){*self.builder.borrow_mut()=builder;}
}
impl ElementHandler for RunHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){this.font=null;}
    fn startElement(&self,_element:i32,_attrsNotCopy:&dyn XmlRoAttr){*self.font.borrow_mut()=None;}
    // java public ElementHandler enterSubElement(int subElement){switch(subElement){case 4155:return this.tHandler;case 4653:this.fontHandler=new FontHandler(this.book,false);this.font=new FontRecord();this.fontHandler.init(this.font);return new ElementAttr(this.fontHandler);default:return null;}}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{match subElement{4155=>Some(hold(self.tHandler.clone(),self.selfRef.upgrade().expect("RunHandler owner"))),4653=>{let fontHandler=std::rc::Rc::new(FontHandler::fromBook(self.book.clone(),false));*self.fontHandler.borrow_mut()=Some(fontHandler.clone());let font=Rc::new(RefCell::new(FontRecord::FontRecord()));*self.font.borrow_mut()=Some(font.clone());fontHandler.init(font);Some(Box::new(ElementAttr::new(fontHandler)))},_=>None}}
    // java public void endElement(int element){if(this.font!=null){this.fontId=this.book.getXFManager().createFont(this.font);}else{this.fontId=-1;}this.builder.add(this.value,this.fontId);this.font=null;this.value=null;}
    fn endElement(&self,_element:i32){let font=self.font.borrow().clone();if let Some(font)=font{self.fontId.set(self.book.getXFManager().borrow().createFont(&font.borrow()));}else{self.fontId.set(-1);}self.builder.borrow().as_ref().expect("NullPointerException").borrow_mut().add(self.value.borrow().clone(),self.fontId.get());*self.font.borrow_mut()=None;*self.value.borrow_mut()=None;}
    // Native inherited DefaultElementHandler callback, whose original body is empty.
    fn characters(&self,s:&str){kfc_kfc_alpha::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler.characters(s);}
}
// java private final class valueHandler implements ElementHandler {
struct valueHandler{outer:Weak<RunHandler>}
impl ElementHandler for valueHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){RunHandler.this.value="";RunHandler.this.fontId=-1;}
    fn startElement(&self,_element:i32,_attrsNotCopy:&dyn XmlRoAttr){let outer=self.outer.upgrade().expect("RunHandler owner");*outer.value.borrow_mut()=Some(String::new());outer.fontId.set(-1);}
    // java public void endElement(int element){}
    fn endElement(&self,_element:i32){}
    // java public void characters(String chars){RunHandler.this.value=EscapeUtil.unEscapeControlCharacter(chars);}
    fn characters(&self,chars:&str){*self.outer.upgrade().expect("RunHandler owner").value.borrow_mut()=Some(EscapeUtil::un_escape_control_character(chars));}
    // java public ElementHandler enterSubElement(int subElement){return null;}
    fn enterSubElement(&self,_subElement:i32)->Option<Box<dyn ElementHandler>>{None}
}
