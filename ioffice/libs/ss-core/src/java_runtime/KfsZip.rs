//! KFC OutputStream leaf binding to the already ported original KFS ZIP classes.
//! No ET/OPC decisions live in this adapter; one KFS stream serves every part.
use std::{rc::Rc,cell::RefCell,io::{self,Write}};
use kfc_kfc_alpha::kfc::opcw::native as native;
use kfs_kfs_app::kfs::zip::{ZipOutputStream::{ZipOutputStream,IListener},ZipEntry::ZipEntry,Deflater::DeflateDataCallBack};
pub struct Entry(pub ZipEntry);
impl native::ZipEntry for Entry{
    fn cloneEntry(&self)->Box<dyn native::ZipEntry>{Box::new(Entry(self.0.clone()))}
    fn setName(&mut self,name:Option<String>){self.0.setName(name);}
    fn as_any(&self)->&dyn std::any::Any{self}
}
struct Listener(Box<dyn native::IListener>);
impl IListener for Listener{fn onUpdateEntry(&mut self,entry:&ZipEntry){self.0.onUpdateEntry(&Entry(entry.clone()));}}
struct Callback(Box<dyn native::DeflateDataCallBack>);
impl DeflateDataCallBack for Callback{fn callBack(&self,b:&[u8],off:i32,len:i32){self.0.callBack(b,off,len);}}
pub struct Stream(ZipOutputStream);
impl Write for Stream{
    fn write(&mut self,b:&[u8])->io::Result<usize>{self.0.try_write_buf(b,0,b.len())?;Ok(b.len())}
    fn flush(&mut self)->io::Result<()>{self.0.try_flush()}
}
impl native::ZipOutputStream for Stream{
    fn newEntry(&self,name:Option<String>)->Box<dyn native::ZipEntry>{Box::new(Entry(match name{Some(name)=>ZipEntry::new__string(name),None=>ZipEntry::new()}))}
    fn putNextEntry(&mut self,entry:&dyn native::ZipEntry,listener:Option<Box<dyn native::IListener>>,callback:Option<Box<dyn native::DeflateDataCallBack>>)->io::Result<()>{
        let entry=entry.as_any().downcast_ref::<Entry>().expect("KFS ZipEntry");
        self.0.try_putNextEntry__zipentry_listener_callback(entry.0.clone(),listener.map(|v|Box::new(Listener(v))as Box<dyn IListener>),callback.map(|v|Box::new(Callback(v))as Box<dyn DeflateDataCallBack>))
    }
    fn close(&mut self)->io::Result<()>{self.0.try_close()}
}
pub fn new(output:Box<dyn Write>)->native::SharedZip{Rc::new(RefCell::new(Box::new(Stream(ZipOutputStream::new(output)))))}
fn file(path:&str)->io::Result<native::SharedZip>{Ok(new(Box::new(std::io::BufWriter::new(std::fs::File::create(path)?))))}
pub fn install(){native::installZipFactory(file);}
