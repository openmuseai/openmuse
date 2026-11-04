use std::{rc::Rc,cell::RefCell,io};
use crate::kfc::{ooxfr::OxfRelations,opcw::{OpcwPackage::OpcwPackage,native::{PartRef,RelationRef}}};
use super::{OxfwAppPart::OxfwAppPart,OxfwCorePart::OxfwCorePart,OxfwCustomPart::OxfwCustomPart};
// java public final class OxfwPackage extends OpcwPackage {
pub struct OxfwPackage{
    base:OpcwPackage,
    // java private OxfwAppPart mAppPart = null;
    mAppPart:Option<Rc<RefCell<OxfwAppPart>>>,
    // java private OxfwCorePart mCorePart = null;
    mCorePart:Option<Rc<RefCell<OxfwCorePart>>>,
    // java private OxfwCustomPart mCustomPart = null;
    mCustomPart:Option<Rc<RefCell<OxfwCustomPart>>>,
}
impl OxfwPackage{
    // java public OxfwPackage(String path) throws IOException { super(path); }
    pub fn OxfwPackage(path:&str)->io::Result<Self>{Ok(Self{base:OpcwPackage::OpcwPackage(path)?,mAppPart:None,mCorePart:None,mCustomPart:None})}
    // java public OxfwAppPart getAppPart() {
    pub fn getAppPart(&mut self)->Rc<RefCell<OxfwAppPart>>{
        // java if (this.mAppPart == null) { OpcwRelationship rels = this.mRelses.addRelationship(OxfRelations.APP); this.mAppPart = (OxfwAppPart)rels.createPart(); }
        if self.mAppPart.is_none(){let rels=self.base.mRelses.borrow_mut().addRelationship(RelationRef::Static(&*OxfRelations::APP));let PartRef::App(part)=rels.borrow_mut().createPart()else{panic!("ClassCastException OxfwAppPart");};self.mAppPart=Some(part);}
        // java return this.mAppPart;
        self.mAppPart.as_ref().unwrap().clone()
    }
    // java public OxfwCorePart getCorePart() {
    pub fn getCorePart(&mut self)->Rc<RefCell<OxfwCorePart>>{
        // java if (this.mCorePart == null) { OpcwRelationship rels = this.mRelses.addRelationship(OxfRelations.CORE); this.mCorePart = (OxfwCorePart)rels.createPart(); }
        if self.mCorePart.is_none(){let rels=self.base.mRelses.borrow_mut().addRelationship(RelationRef::Static(&*OxfRelations::CORE));let PartRef::Core(part)=rels.borrow_mut().createPart()else{panic!("ClassCastException OxfwCorePart");};self.mCorePart=Some(part);}
        // java return this.mCorePart;
        self.mCorePart.as_ref().unwrap().clone()
    }
    // java public OxfwCustomPart getCustomPart() {
    pub fn getCustomPart(&mut self)->Rc<RefCell<OxfwCustomPart>>{
        // java if (this.mCustomPart == null) { OpcwRelationship rels = this.mRelses.addRelationship(OxfRelations.CUSTOM); this.mCustomPart = (OxfwCustomPart)rels.createPart(); }
        if self.mCustomPart.is_none(){let rels=self.base.mRelses.borrow_mut().addRelationship(RelationRef::Static(&*OxfRelations::CUSTOM));let PartRef::Custom(part)=rels.borrow_mut().createPart()else{panic!("ClassCastException OxfwCustomPart");};self.mCustomPart=Some(part);}
        // java return this.mCustomPart;
        self.mCustomPart.as_ref().unwrap().clone()
    }
    // java public void close() throws IOException {
    pub fn close(&mut self)->io::Result<()>{
        // java if (this.mAppPart != null) { this.mAppPart.write(); }
        if let Some(part)=&self.mAppPart{part.borrow_mut().write()?;}
        // java if (this.mCorePart != null) { this.mCorePart.write(); }
        if let Some(part)=&self.mCorePart{part.borrow_mut().write()?;}
        // java super.close();
        self.base.close()
    }
}
impl std::ops::Deref for OxfwPackage{type Target=OpcwPackage;fn deref(&self)->&Self::Target{&self.base}}
impl std::ops::DerefMut for OxfwPackage{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
