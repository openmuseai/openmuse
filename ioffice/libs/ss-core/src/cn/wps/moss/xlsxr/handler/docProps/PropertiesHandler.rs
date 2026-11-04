use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::{app::KmoBook::KmoBook,xlsxr::util::ValueHelper::ValueHelper}};
use kfc_kfc_alpha::{kfc::{opcr::OpcUtil::OpcUtil,xpxml::reader::{ElementStrHandler::ElementStrHandler,DefaultElementStrHandler::Subclass}},native_runtime::Date};
use io_poi_compact::org::apache::poi::hpsf::Property::{Property,PropertyValue};
// java public class PropertiesHandler extends DefaultElementStrHandler {
pub struct PropertiesHandler {
    // java private PropertyHandler mHandler;
    mHandler:Rc<RefCell<PropertyHandler>>,
    // java private KmoBook mBook;
    mBook:Rc<KmoBook>,
}
impl PropertiesHandler {
    // java public PropertiesHandler(KmoBook book) {
    pub fn PropertiesHandler(book:Rc<KmoBook>)->Self {
        // java this.mBook = book;
        let mBook=book;
        // java this.mHandler = new PropertyHandler();
        let mHandler=PropertyHandler::PropertyHandler(Rc::clone(&mBook));
        Self{mHandler,mBook}
    }
}
impl Subclass for PropertiesHandler {
    // java public ElementStrHandler enterSubElement(String subLabel) { return subLabel.equals("property") ? this.mHandler : null; }
    fn enter_sub_element(&mut self,subLabel:&str)->Option<Box<dyn ElementStrHandler>>{
        if subLabel=="property"{Some(Box::new(SharedHandlerRef{handler:Rc::clone(&self.mHandler),outer:None}))}else{None}
    }
}
// java private class PropertyHandler extends DefaultElementStrHandler {
struct PropertyHandler {
    // java private String mName;
    mName:Option<String>,
    // java private long mType;
    mType:i64,
    // java private Object mValue;
    mValue:Option<PropertyValue>,
    // java private PropertyValueHandler mValueHandler;
    mValueHandler:Rc<RefCell<PropertyValueHandler>>,
    // Native capture of PropertiesHandler.this.mBook (same object).
    outerBook:Rc<KmoBook>,
    selfRef:Weak<RefCell<PropertyHandler>>,
}
impl PropertyHandler {
    // java private PropertyHandler() {
    fn PropertyHandler(outerBook:Rc<KmoBook>)->Rc<RefCell<Self>>{
        // java this.mName = null; this.mType = -1L; this.mValue = null;
        // java this.mValueHandler = new PropertyValueHandler();
        Rc::new_cyclic(|outer|RefCell::new(Self{mName:None,mType:-1,mValue:None,mValueHandler:Rc::new(RefCell::new(PropertyValueHandler{outer:outer.clone()})),outerBook,selfRef:outer.clone()}))
    }
    // Native reference to the same Java inner instance, retaining its outer object.
    fn valueHandlerRef(&self)->Box<dyn ElementStrHandler>{Box::new(SharedHandlerRef{handler:Rc::clone(&self.mValueHandler),outer:Some(self.selfRef.upgrade().expect("PropertyHandler outer"))})}
}
impl Subclass for PropertyHandler {
    // java public void startElement(String label) {
    fn start_element(&mut self,_label:&str){
        // java this.mName = null;
        self.mName=None;
        // java this.mType = -1L;
        self.mType=-1;
        // java this.mValue = null;
        self.mValue=None;
    }
    // java public void addAttr(String name, String value) {
    fn add_attr(&mut self,name:&str,value:&str){
        // java if (name.equals("name")) { this.mName = value; }
        if name=="name"{self.mName=Some(value.into());}
    }
    // java public void endElement(String label) {
    fn end_element(&mut self,_label:&str){
        // java KmoAppSettings settings = PropertiesHandler.this.mBook.getAppSettings();
        let settings=self.outerBook.getAppSettings();
        // java if (this.mName != null && this.mType != -1L && this.mValue != null) {
        if let (Some(name),Some(value))=(&self.mName,&self.mValue){if self.mType != -1 {
            // java settings.getCustomProps().put(this.mName, new Property(-1L, this.mType, this.mValue));
            settings.borrow().getCustomProps().borrow_mut().put(name.clone(),Some(Rc::new(Property::new(-1,self.mType,value.clone()))));
        }}
    }
    // java public ElementStrHandler enterSubElement(String subLabel) {
    fn enter_sub_element(&mut self,subLabel:&str)->Option<Box<dyn ElementStrHandler>>{
        // java if (subLabel.equals("vt:bool")) { this.mType = 11L; return this.mValueHandler; }
        if subLabel=="vt:bool"{self.mType=11;Some(self.valueHandlerRef())}
        // java else if (subLabel.equals("vt:lpwstr")) { this.mType = 31L; return this.mValueHandler; }
        else if subLabel=="vt:lpwstr"{self.mType=31;Some(self.valueHandlerRef())}
        // java else if (subLabel.equals("vt:lpstr")) { this.mType = 30L; return this.mValueHandler; }
        else if subLabel=="vt:lpstr"{self.mType=30;Some(self.valueHandlerRef())}
        // java else if (subLabel.equals("vt:i4")) { this.mType = 3L; return this.mValueHandler; }
        else if subLabel=="vt:i4"{self.mType=3;Some(self.valueHandlerRef())}
        // java else if (subLabel.equals("vt:filetime")) { this.mType = 64L; return this.mValueHandler; }
        else if subLabel=="vt:filetime"{self.mType=64;Some(self.valueHandlerRef())}
        // java else { return null; }
        else{None}
    }
}
// java private class PropertyValueHandler extends DefaultElementStrHandler {
struct PropertyValueHandler{outer:Weak<RefCell<PropertyHandler>>}
impl Subclass for PropertyValueHandler {
    // java public void characters(String chars) {
    fn characters(&mut self,chars:&str){
        let outer=self.outer.upgrade().expect("PropertyHandler outer");let mut outer=outer.borrow_mut();
        // java if (PropertyHandler.this.mType == 11L) {
        if outer.mType==11 {
            // java PropertyHandler.this.mValue = ValueHelper.toBoolean(chars);
            outer.mValue=Some(PropertyValue::Bool(ValueHelper::toBoolean(Some(chars))));
        // java } else if (PropertyHandler.this.mType != 31L && PropertyHandler.this.mType != 30L) {
        }else if outer.mType!=31&&outer.mType!=30 {
            // java if (PropertyHandler.this.mType == 3L) {
            if outer.mType==3 {
                // java PropertyHandler.this.mValue = ValueHelper.toInt(chars);
                outer.mValue=Some(PropertyValue::Int(ValueHelper::toInt(Some(chars))));
            // java } else if (PropertyHandler.this.mType == 64L) {
            }else if outer.mType==64 {
                // java PropertyHandler.this.mValue = OpcUtil.getDate(chars);
                outer.mValue=OpcUtil::getDate(chars).map(|t|PropertyValue::Date(Rc::new(Date::fromTime(t))));
            }
        // java } else { PropertyHandler.this.mValue = chars; }
        }else{outer.mValue=Some(PropertyValue::Str(chars.into()));}
    }
}
// Native Java reference transport. Repeated enterSubElement returns references to
// the same handler state, and inner references keep their outer alive without a cycle.
struct SharedHandlerRef<T:ElementStrHandler>{handler:Rc<RefCell<T>>,outer:Option<Rc<RefCell<PropertyHandler>>>}
impl<T:ElementStrHandler+'static> Subclass for SharedHandlerRef<T>{
    fn start_element(&mut self,s:&str){self.handler.borrow_mut().start_element(s)}
    fn end_element(&mut self,s:&str){self.handler.borrow_mut().end_element(s)}
    fn add_attr(&mut self,n:&str,v:&str){self.handler.borrow_mut().add_attr(n,v)}
    fn characters(&mut self,s:&str){self.handler.borrow_mut().characters(s)}
    fn enter_sub_element(&mut self,s:&str)->Option<Box<dyn ElementStrHandler>>{self.handler.borrow_mut().enter_sub_element(s)}
}
