use crate::{java_runtime::{Rc,RefCell},cn::wps::et::ss::formula::Formula::Formula};
use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,AtomSnapshot::SharedAtom};
use super::{NameRecord::{NameRecordRef,IntoNameRecordRef},LinkInfoData::LinkInfoData};
pub use super::LinkInfoData::NameList;
// java public class LinkInfo extends KmoAtom<LinkInfoData> {
pub struct LinkInfo{atom:Rc<RefCell<KmoAtom<LinkInfoData>>>,_owner:SharedAtom}
impl LinkInfo{
    // java public LinkInfo(){super(new LinkInfoData());}
    pub fn LinkInfo()->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(LinkInfoData::new());Self{atom,_owner:owner}}
    // java public void removeNameInfo(NameRecord name){int index=this.rawData()._definedNames.indexOf(name);this.removeNameInfo(index);}
    pub fn removeNameInfo__name(&self,name:Option<NameRecordRef>){let index=self.indexOf(name);self.removeNameInfo(index);}
    // java public void removeNameInfo(int index){this.beforeWrite();this.rawData()._definedNames.get(index).setNameDefinition(Formula.EMPTY_PTGS);}
    pub fn removeNameInfo(&self,index:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();let record=atom.rawData()._definedNames.as_ref().expect("NullPointerException").borrow().get(index as usize).unwrap_or_else(||panic!("IndexOutOfBoundsException")).clone().expect("NullPointerException");record.borrow_mut().setNameDefinition(Formula::EMPTY_PTGS());}
    // java public void addNameInfo(NameRecord name){this.beforeWrite();this.rawData()._definedNames.add(name);}
    pub fn addNameInfo(&self,name:impl IntoNameRecordRef){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData()._definedNames.as_ref().expect("NullPointerException").borrow_mut().push(name.intoRecord());}
    // java public int indexOf(NameRecord nr){return this.rawData()._definedNames.indexOf(nr);}
    pub fn indexOf(&self,nr:Option<NameRecordRef>)->i32{let names=self.names();let names=names.expect("NullPointerException");let found=names.borrow().iter().position(|r|match(r,&nr){(None,None)=>true,(Some(a),Some(b))=>Rc::ptr_eq(a,b),_=>false});found.map_or(-1,|i|i as i32)}
    // java public NameRecord get(int index){return this.rawData()._definedNames.get(index);}
    pub fn get(&self,index:i32)->Option<NameRecordRef>{let names=self.names().expect("NullPointerException");let record=names.borrow().get(index as usize).unwrap_or_else(||panic!("IndexOutOfBoundsException")).clone();record}
    // java public int getNamesCount(){return this.rawData()._definedNames.size();}
    pub fn getNamesCount(&self)->i32{self.names().expect("NullPointerException").borrow().len()as i32}
    // java public List<NameRecord> nameInfos(){List<NameRecord> rowList=new ArrayList();rowList.addAll(this.rawData()._definedNames);return rowList;}
    pub fn nameInfos(&self)->NameList{let rowList=Rc::new(RefCell::new(vec![]));rowList.borrow_mut().extend(self.names().expect("NullPointerException").borrow().iter().cloned());rowList}
    // java public void setNameInfos(List<NameRecord> names,boolean isNeedBackup){if(isNeedBackup){this.beforeWrite();}this.rawData()._definedNames=names;}
    pub fn setNameInfos(&self,names:Option<NameList>,isNeedBackup:bool){let mut atom=self.atom.borrow_mut();if isNeedBackup{atom.beforeWrite();}atom.rawData_mut()._definedNames=names;}
    // Native view of the original data field, without copying the Java List.
    fn names(&self)->Option<NameList>{self.atom.borrow().rawData()._definedNames.clone()}
}
