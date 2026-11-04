use std::{rc::Rc,cell::RefCell};
use super::{ElementHandler::ElementHandler,ElementAttrHandler::ElementAttrHandler,XmlRoAttr::XmlRoAttr,XmlRoAttrNest::XmlRoAttrNest,XmlRoAttrLeaf::XmlRoAttrLeaf};
// java public final class ElementAttr implements ElementHandler
#[derive(Clone)]
pub struct ElementAttr {mHandler:Rc<dyn ElementAttrHandler>,mRoAttrStack:Rc<RefCell<Vec<XmlRoAttrNest>>>}
impl ElementAttr {
    // java this.mHandler=handler; private Stack<XmlRoAttr> mRoAttrStack=new Stack();
    pub fn new(handler:Rc<dyn ElementAttrHandler>)->Self {Self{mHandler:handler,mRoAttrStack:Rc::new(RefCell::new(Vec::new()))}}
}
impl ElementHandler for ElementAttr {
    fn startElement(&self,element:i32,attrs:&dyn XmlRoAttr){
        // java XmlRoAttr roAttr=this.copyRoAttr(element,attrsNotCopy);
        let mut roAttr=XmlRoAttrNest::new(element);
        // java for(...) {int name=attrsNotCopy.getNameAt(i); String value=attrsNotCopy.getAttrAt(i).toString(); roAttr.addAttr(new XmlRoAttrLeaf(name,value));}
        for i in 0..attrs.count(){let mut leaf=XmlRoAttrLeaf::new();leaf.reset(attrs.get_name_at(i),&attrs.get_attr_at(i).unwrap().to_string());roAttr.addAttr(Box::new(leaf));}
        // java this.mRoAttrStack.push(roAttr);
        // Rust: own a child in its stack frame until endElement; it is moved
        // into its parent before the sole root callback, preserving insertion order.
        self.mRoAttrStack.borrow_mut().push(roAttr);
    }
    fn endElement(&self,element:i32){
        // java XmlRoAttr roAttr=this.mRoAttrStack.pop(); if(this.mRoAttrStack.empty()) this.mHandler.addElementAttr(element,roAttr);
        let mut stack=self.mRoAttrStack.borrow_mut();let roAttr=stack.pop().expect("ElementAttr stack");
        if let Some(parent)=stack.last_mut(){parent.addAttr(Box::new(roAttr));}else{drop(stack);self.mHandler.addElementAttr(element,&roAttr);}
    }
    // java lastRoAttr.characters(chars);
    fn characters(&self,chars:&str){self.mRoAttrStack.borrow_mut().last_mut().expect("ElementAttr stack").characters(chars);}
    // java return this;
    fn enterSubElement(&self,_:i32)->Option<Box<dyn ElementHandler>>{Some(Box::new(self.clone()))}
}
