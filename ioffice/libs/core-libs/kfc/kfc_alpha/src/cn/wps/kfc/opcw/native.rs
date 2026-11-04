//! Ownership and OutputStream adapters, with the same original KFS ZIP instance.
use std::{cell::{Ref,RefCell},rc::Rc,io::{self,Write}};
use crate::kfc::{opcr::OpcRelation::OpcRelation,ooxfw::{OxfwAppPart::OxfwAppPart,OxfwCorePart::OxfwCorePart,OxfwCustomPart::OxfwCustomPart}};
use super::{OpcwPackagePart::OpcwPackagePart,OpcwRelationships::OpcwRelationships};
// Native dependency inversion: KFS depends on util, which already depends on KFC.
// The owner supplies the original KFS object through this leaf API, avoiding a Cargo cycle.
pub trait ZipEntry:std::any::Any{
    fn cloneEntry(&self)->Box<dyn ZipEntry>;
    fn setName(&mut self,name:Option<String>);
    fn as_any(&self)->&dyn std::any::Any;
}
pub trait IListener{fn onUpdateEntry(&mut self,entry:&dyn ZipEntry);}
pub trait DeflateDataCallBack:Send{fn callBack(&self,b:&[u8],off:i32,len:i32);}
pub trait ZipOutputStream:Write{
    fn newEntry(&self,name:Option<String>)->Box<dyn ZipEntry>;
    fn putNextEntry(&mut self,entry:&dyn ZipEntry,listener:Option<Box<dyn IListener>>,callback:Option<Box<dyn DeflateDataCallBack>>)->io::Result<()>;
    fn close(&mut self)->io::Result<()>;
}
pub type SharedZip=Rc<RefCell<Box<dyn ZipOutputStream>>>;
pub type ZipFactory=fn(&str)->io::Result<SharedZip>;
static ZIP_FACTORY:std::sync::OnceLock<ZipFactory>=std::sync::OnceLock::new();
// Native platform initialization, analogous to the existing resource-loader binding.
pub fn installZipFactory(factory:ZipFactory){let _=ZIP_FACTORY.set(factory);}
pub fn openZip(path:&str)->io::Result<SharedZip>{ZIP_FACTORY.get().ok_or_else(||io::Error::new(io::ErrorKind::Unsupported,"KFS ZIP platform binding is not installed"))?(path)}
#[derive(Clone)]pub enum RelationRef{Static(&'static OpcRelation),Local(Rc<OpcRelation>),Shared(std::sync::Arc<OpcRelation>)}
impl std::ops::Deref for RelationRef{type Target=OpcRelation;fn deref(&self)->&Self::Target{match self{Self::Static(v)=>v,Self::Local(v)=>v,Self::Shared(v)=>v}}}
impl From<&'static OpcRelation>for RelationRef{fn from(v:&'static OpcRelation)->Self{Self::Static(v)}}
impl From<Rc<OpcRelation>>for RelationRef{fn from(v:Rc<OpcRelation>)->Self{Self::Local(v)}}
#[derive(Clone)]pub struct ZipSink(pub SharedZip);
impl Write for ZipSink{
    fn write(&mut self,bytes:&[u8])->io::Result<usize>{self.0.borrow_mut().write(bytes)}
    fn flush(&mut self)->io::Result<()>{self.0.borrow_mut().flush()}
}
// Java subclass references retain identity; the base is never copied or flattened.
#[derive(Clone)]pub enum PartRef{Base(Rc<RefCell<OpcwPackagePart>>),App(Rc<RefCell<OxfwAppPart>>),Core(Rc<RefCell<OxfwCorePart>>),Custom(Rc<RefCell<OxfwCustomPart>>)}
impl PartRef{
    pub fn base(&self)->Ref<'_,OpcwPackagePart>{match self{Self::Base(v)=>v.borrow(),Self::App(v)=>Ref::map(v.borrow(),|v|&v.base),Self::Core(v)=>Ref::map(v.borrow(),|v|&v.base),Self::Custom(v)=>Ref::map(v.borrow(),|v|&v.base)}}
    pub fn withBaseMut<T>(&self,f:impl FnOnce(&mut OpcwPackagePart)->T)->T{match self{Self::Base(v)=>f(&mut v.borrow_mut()),Self::App(v)=>f(&mut v.borrow_mut().base),Self::Core(v)=>f(&mut v.borrow_mut().base),Self::Custom(v)=>f(&mut v.borrow_mut().base)}}
    pub fn getRelationships(&self)->Rc<RefCell<OpcwRelationships>>{self.withBaseMut(|p|p.getRelationships())}
    pub fn getOutputStream(&self)->io::Result<Option<ZipSink>>{self.withBaseMut(|p|p.getOutputStream())}
    pub fn getPartName(&self)->String{self.base().getPartName()}
}
