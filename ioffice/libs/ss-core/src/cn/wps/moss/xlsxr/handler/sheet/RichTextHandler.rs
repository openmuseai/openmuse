use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::{app::KmoBook::KmoBook,engine::grid::MemoryMixSQLStringList::OoxmlStr,xlsxr::util::ElementHandlerRef::{share,hold}}};
use super::{RunHandler::RunHandler,RichTextBuilder::RichTextBuilder};
use kfc_kfc_alpha::kfc::xpxml::{reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr},util::EscapeUtil::EscapeUtil};
// java public class RichTextHandler implements ElementHandler {
pub struct RichTextHandler{
    // java private ArrayList<UnicodeString> strs=null;
    strs:RefCell<Option<Rc<RefCell<Vec<OoxmlStr>>>>>,
    // java private RunHandler runHandler;
    runHandler:Rc<RunHandler>,
    // java private RichTextBuilder builder;
    builder:Rc<RefCell<RichTextBuilder>>,
    // java private String mTxtValue="";
    mTxtValue:RefCell<Option<String>>,
    // java private ValueHandler mValueHandler=new ValueHandler();
    mValueHandler:Rc<ValueHandler>,
    selfRef:Weak<Self>,
}
impl RichTextHandler{
    // java public RichTextHandler(ArrayList<UnicodeString> strs,KmoBook book){this.strs=strs;this.runHandler=new RunHandler(book);this.builder=new RichTextBuilder();}
    pub fn RichTextHandler(strs:Option<Rc<RefCell<Vec<OoxmlStr>>>>,book:Rc<KmoBook>)->Rc<Self>{Rc::new_cyclic(|s|Self{strs:RefCell::new(strs),runHandler:RunHandler::RunHandler(book),builder:Rc::new(RefCell::new(RichTextBuilder::default())),mTxtValue:RefCell::new(Some(String::new())),mValueHandler:Rc::new(ValueHandler{outer:s.clone()}),selfRef:s.clone()})}
    // java public void reset(ArrayList<UnicodeString> strs){this.strs=strs;}
    pub fn reset(&self,strs:Option<Rc<RefCell<Vec<OoxmlStr>>>>){*self.strs.borrow_mut()=strs;}
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){this.mTxtValue="";}
    pub fn startElement(&self,_element:i32,_attrsNotCopy:&dyn XmlRoAttr){*self.mTxtValue.borrow_mut()=Some(String::new());}
    // java public void endElement(int element){
    pub fn endElement(&self,_element:i32){
        // java if(!this.builder.empty()){this.strs.add(this.builder.build());this.builder.Clear();}
        if !self.builder.borrow().empty(){let value=self.builder.borrow().build();self.strs.borrow().as_ref().expect("NullPointerException").borrow_mut().push(value);self.builder.borrow_mut().Clear();
        // java else if(null==this.mTxtValue){this.strs.add(new UnicodeString(""));}
        }else if self.mTxtValue.borrow().is_none(){self.strs.borrow().as_ref().expect("NullPointerException").borrow_mut().push(OoxmlStr::new(String::new()));
        // java else{this.strs.add(new UnicodeString(this.mTxtValue));}
        }else{let value=self.mTxtValue.borrow().clone().unwrap();self.strs.borrow().as_ref().expect("NullPointerException").borrow_mut().push(OoxmlStr::new(value));}
    }
    // java public void characters(String chars){}
    pub fn characters(&self,_chars:&str){}
    // java public ElementHandler enterSubElement(int subElement){switch(subElement){case 4151:this.runHandler.init(this.builder);return this.runHandler;case 4155:return this.mValueHandler;default:return null;}}
    pub fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{match subElement{4151=>{self.runHandler.init(Some(self.builder.clone()));Some(share(self.runHandler.clone()))},4155=>Some(hold(self.mValueHandler.clone(),self.selfRef.upgrade().expect("RichTextHandler owner"))),_=>None}}
}
// java private final class ValueHandler implements ElementHandler {
struct ValueHandler{outer:Weak<RichTextHandler>}
impl ElementHandler for ValueHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){}
    fn startElement(&self,_element:i32,_attrsNotCopy:&dyn XmlRoAttr){}
    // java public void endElement(int element){}
    fn endElement(&self,_element:i32){}
    // java public void characters(String chars){RichTextHandler.this.mTxtValue=EscapeUtil.unEscapeControlCharacter(chars);}
    fn characters(&self,chars:&str){*self.outer.upgrade().expect("RichTextHandler owner").mTxtValue.borrow_mut()=Some(EscapeUtil::un_escape_control_character(chars));}
    // java public ElementHandler enterSubElement(int subElement){return null;}
    fn enterSubElement(&self,_subElement:i32)->Option<Box<dyn ElementHandler>>{None}
}
impl ElementHandler for RichTextHandler{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){RichTextHandler::startElement(self,e,a);}
    fn endElement(&self,e:i32){RichTextHandler::endElement(self,e);}
    fn characters(&self,s:&str){RichTextHandler::characters(self,s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{RichTextHandler::enterSubElement(self,e)}
}
