use crate::java_runtime::{Rc,RefCell};
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData,KmoAtomDataSub};
use super::NameRecord::NameRecordRef;
pub type NameList=Rc<RefCell<Vec<Option<NameRecordRef>>>>;
// java class LinkInfoData extends KmoAtomData {
pub(super) struct LinkInfoData{
    base:KmoAtomData,
    // java public List<NameRecord> _definedNames=new ArrayList();
    pub _definedNames:Option<NameList>,
}
impl LinkInfoData{
    pub fn new()->Self{Self{base:KmoAtomData::new(),_definedNames:Some(Rc::new(RefCell::new(vec![])))}}
    // java public KmoAtomData copy(){LinkInfoData infoDatat=new LinkInfoData();int i=0;for(int size=this._definedNames.size();i<size;++i){infoDatat._definedNames.add(this._definedNames.get(i).copy());}return infoDatat;}
    pub fn copy(&self)->Self{let infoDatat=Self::new();let mut i=0;let names=self._definedNames.as_ref().expect("NullPointerException");let size=names.borrow().len();while i<size{let record=names.borrow()[i].clone().expect("NullPointerException");let copy=record.borrow().copy();infoDatat._definedNames.as_ref().unwrap().borrow_mut().push(Some(Rc::new(RefCell::new(copy))));i+=1;}infoDatat}
}
impl KmoAtomDataSub for LinkInfoData{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{LinkInfoData::copy(self)}}
