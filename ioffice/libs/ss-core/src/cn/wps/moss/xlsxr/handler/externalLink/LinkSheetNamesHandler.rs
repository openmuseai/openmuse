use crate::java_runtime::{Rc,Weak,RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use super::native::{Subclass,hold};
// java public final class LinkSheetNamesHandler extends DefaultElementHandler {
pub struct LinkSheetNamesHandler{
    // java private SheetNameHandler stNameHandler=new SheetNameHandler();
    stNameHandler:Rc<SheetNameHandler>,
    // java private List<String> sheetNames=new ArrayList();
    sheetNames:Rc<RefCell<Vec<Option<String>>>>,
    selfRef:Weak<Self>,
}
impl LinkSheetNamesHandler{
    // Native allocation of Java implicit constructor and inner this reference.
    pub fn new()->Rc<Self>{Rc::new_cyclic(|s|Self{stNameHandler:Rc::new(SheetNameHandler{outer:s.clone()}),sheetNames:Rc::new(RefCell::new(vec![])),selfRef:s.clone()})}
}
impl Subclass for LinkSheetNamesHandler{
    // java public ElementHandler enterSubElement(int subElement){return subElement==5201?this.stNameHandler:null;}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==5201{Some(hold(self.stNameHandler.clone(),self.selfRef.upgrade().expect("LinkSheetNamesHandler owner")))}else{None}}
}
impl LinkSheetNamesHandler{
    // java List<String> getSheetNames(){return this.sheetNames;}
    pub(super) fn getSheetNames(&self)->Rc<RefCell<Vec<Option<String>>>>{self.sheetNames.clone()}
}
// java private final class SheetNameHandler extends DefaultElementHandler {
struct SheetNameHandler{outer:Weak<LinkSheetNamesHandler>}
impl Subclass for SheetNameHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){String sheetName=attrsNotCopy.getByName(4114).toString();LinkSheetNamesHandler.this.sheetNames.add(sheetName);}
    fn startElement(&self,_element:i32,attrsNotCopy:&dyn XmlRoAttr){let sheetName=attrsNotCopy.get_by_name(4114).expect("NullPointerException").to_string();self.outer.upgrade().expect("LinkSheetNamesHandler owner").sheetNames.borrow_mut().push(Some(sheetName));}
}
