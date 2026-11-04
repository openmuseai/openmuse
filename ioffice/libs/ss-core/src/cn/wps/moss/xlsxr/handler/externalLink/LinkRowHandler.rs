use crate::{java_runtime::{Rc,Weak,RefCell,Cell,JavaObject::NullableObject},cn::wps::{moss::{engine::linktb::CRNRecord::CRNRecord,xlsxr::{handler::sheet::CellHandler::CellHandler,util::ValueHelper::ValueHelper}},et::ss::formula::constant::ErrorConstant::ErrorConstant}};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use super::{CellValue::CellValue,native::{Subclass,hold}};
pub type CRNRecords=Rc<RefCell<Vec<Option<Rc<CRNRecord>>>>>;
// java public final class LinkRowHandler extends DefaultElementHandler {
pub struct LinkRowHandler{
    // java private List<CRNRecord> crnRecs;
    crnRecs:Option<CRNRecords>,
    // java private int row;
    row:Cell<i32>,
    // java private LinkCellHandler cellHandler=new LinkCellHandler();
    cellHandler:Rc<LinkCellHandler>,
    // java private List<CellValue> cellValues=new ArrayList();
    cellValues:RefCell<Vec<CellValue>>,
    selfRef:Weak<Self>,
}
impl LinkRowHandler{
    // java public LinkRowHandler(List<CRNRecord> crnRecs){this.crnRecs=crnRecs;}
    pub fn LinkRowHandler(crnRecs:Option<CRNRecords>)->Rc<Self>{Rc::new_cyclic(|s|Self{crnRecs,row:Cell::new(0),cellHandler:LinkCellHandler::LinkCellHandler(s.clone()),cellValues:RefCell::new(vec![]),selfRef:s.clone()})}
}
impl Subclass for LinkRowHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){this.row=attrsNotCopy.getByName(4151).toInt()-1;}
    fn startElement(&self,_element:i32,attrsNotCopy:&dyn XmlRoAttr){self.row.set(attrsNotCopy.get_by_name(4151).expect("NullPointerException").to_int().wrapping_sub(1));}
    // java public ElementHandler enterSubElement(int subElement){return subElement==4267?this.cellHandler:null;}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==4267{Some(hold(self.cellHandler.clone(),self.selfRef.upgrade().expect("LinkRowHandler owner")))}else{None}}
    // java public void endElement(int element){
    fn endElement(&self,_element:i32){
        // java if(this.cellValues.size()!=0){int begin=0;int count=this.cellValues.size();for(int i=1;i<count;++i){
        if !self.cellValues.borrow().is_empty(){let mut begin=0;let count=self.cellValues.borrow().len();let mut i=1;while i<count{
            // java if(this.cellValues.get(i).column!=this.cellValues.get(i-1).column+1){this.createCRNRecord(begin,i-1);begin=i;}
            if self.cellValues.borrow()[i].column!=self.cellValues.borrow()[i-1].column.wrapping_add(1){self.createCRNRecord(begin,i-1);begin=i;}i+=1;
        }
        // java if(begin<count){this.createCRNRecord(begin,count-1);}this.cellValues.clear();
        if begin<count{self.createCRNRecord(begin,count-1);}self.cellValues.borrow_mut().clear();}
    }
}
impl LinkRowHandler{
    // java private void createCRNRecord(int begin,int end){
    fn createCRNRecord(&self,begin:usize,end:usize){
        // java Object[] objs=new Object[end-begin+1];for(int i=begin;i<=end;++i){objs[i-begin]=this.cellValues.get(i).value;}
        let mut objs:Vec<NullableObject>=vec![None;end-begin+1];let mut i=begin;while i<=end{objs[i-begin]=self.cellValues.borrow()[i].value.clone();i+=1;}
        // java int beginColumn=this.cellValues.get(begin).column;int endColumn=this.cellValues.get(end).column;CRNRecord crnRec=CRNRecord.createCRNRecord(beginColumn,endColumn,this.row,objs);this.crnRecs.add(crnRec);
        let beginColumn=self.cellValues.borrow()[begin].column;let endColumn=self.cellValues.borrow()[end].column;let crnRec=CRNRecord::createCRNRecord(beginColumn,endColumn,self.row.get(),Some(Rc::new(RefCell::new(objs))));self.crnRecs.as_ref().expect("NullPointerException").borrow_mut().push(Some(Rc::new(crnRec)));
    }
}
// java private final class LinkCellHandler extends DefaultElementHandler {
struct LinkCellHandler{
    // java private LinkValueHandler valueHandler;
    valueHandler:Rc<LinkValueHandler>,
    // java private int column;
    column:Cell<i32>,
    // java private String type;
    r#type:RefCell<Option<String>>,
    outer:Weak<LinkRowHandler>,selfRef:Weak<Self>,
}
impl LinkCellHandler{
    // java private LinkCellHandler(){this.valueHandler=new LinkValueHandler();}
    fn LinkCellHandler(outer:Weak<LinkRowHandler>)->Rc<Self>{Rc::new_cyclic(|s|Self{valueHandler:Rc::new(LinkValueHandler{outer:s.clone()}),column:Cell::new(0),r#type:RefCell::new(None),outer,selfRef:s.clone()})}
}
impl Subclass for LinkCellHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){
    fn startElement(&self,_element:i32,attrsNotCopy:&dyn XmlRoAttr){
        // java this.column=CellHandler.getCellIndex(attrsNotCopy.getByName(4151).toString());XmlRoAttr tmp;if((tmp=attrsNotCopy.getByName(4155))!=null){this.type=tmp.toString();}else{this.type="";}
        self.column.set(CellHandler::getCellIndex(&attrsNotCopy.get_by_name(4151).expect("NullPointerException").to_string()));let tmp=attrsNotCopy.get_by_name(4155);*self.r#type.borrow_mut()=Some(if let Some(tmp)=tmp{tmp.to_string()}else{String::new()});
    }
    // java public ElementHandler enterSubElement(int subElement){return subElement==4143?this.valueHandler:null;}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==4143{Some(hold(self.valueHandler.clone(),self.selfRef.upgrade().expect("LinkCellHandler owner")))}else{None}}
}
// java private final class LinkValueHandler extends DefaultElementHandler {
struct LinkValueHandler{outer:Weak<LinkCellHandler>}
impl Subclass for LinkValueHandler{
    // java public void characters(String chars){
    fn characters(&self,chars:&str){let cell=self.outer.upgrade().expect("LinkCellHandler owner");let row=cell.outer.upgrade().expect("LinkRowHandler owner");
        // java if(LinkCellHandler.this.type.length()==0){try{Double d=Double.valueOf(chars);LinkRowHandler.this.cellValues.add(new CellValue(LinkCellHandler.this.column,d));}catch(NumberFormatException var3){LinkRowHandler.this.cellValues.add(new CellValue(LinkCellHandler.this.column,chars));}}
        if cell.r#type.borrow().as_ref().expect("NullPointerException").is_empty(){if let Some(d)=crate::java_runtime::parseDouble(chars){row.cellValues.borrow_mut().push(CellValue::CellValue(cell.column.get(),Some(Rc::new(d))));}else{row.cellValues.borrow_mut().push(CellValue::CellValue(cell.column.get(),Some(Rc::new(chars.to_string()))));}
        // java else if(LinkCellHandler.this.type.equals("e")){byte errorCode=CellHandler.ErrorCode.parseCode(chars);LinkRowHandler.this.cellValues.add(new CellValue(LinkCellHandler.this.column,ErrorConstant.valueOf(errorCode)));}
        }else if cell.r#type.borrow().as_deref()==Some("e"){let errorCode=CellHandler::parseCode(chars).expect("NullPointerException");row.cellValues.borrow_mut().push(CellValue::CellValue(cell.column.get(),Some(ErrorConstant::valueOf(errorCode as i32))));
        // java else if(LinkCellHandler.this.type.equals("b")){boolean b=ValueHelper.toBoolean(chars);LinkRowHandler.this.cellValues.add(new CellValue(LinkCellHandler.this.column,b));}
        }else if cell.r#type.borrow().as_deref()==Some("b"){let b=ValueHelper::toBoolean(Some(chars));row.cellValues.borrow_mut().push(CellValue::CellValue(cell.column.get(),Some(Rc::new(b))));
        // java else{LinkRowHandler.this.cellValues.add(new CellValue(LinkCellHandler.this.column,chars));}
        }else{row.cellValues.borrow_mut().push(CellValue::CellValue(cell.column.get(),Some(Rc::new(chars.to_string()))));}
    }
}
