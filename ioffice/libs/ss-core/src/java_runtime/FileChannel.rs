//! Native FileChannel/MapMode leaf. ET retains block layout, byte order and control flow.
use std::{fs::{File,OpenOptions},io::{self,Read,Seek,SeekFrom},ops::{Deref,DerefMut}};
pub struct FileChannel{file:Option<File>}
pub enum MappedBytes{
    #[cfg(not(target_family="wasm"))] Read(memmap2::Mmap),
    #[cfg(not(target_family="wasm"))] Write(memmap2::MmapMut),
    Empty,
}
impl Deref for MappedBytes{type Target=[u8];fn deref(&self)->&[u8]{match self{
    #[cfg(not(target_family="wasm"))]Self::Read(v)=>v,
    #[cfg(not(target_family="wasm"))]Self::Write(v)=>v,
    Self::Empty=>&[],
}}}
impl DerefMut for MappedBytes{fn deref_mut(&mut self)->&mut[u8]{match self{
    #[cfg(not(target_family="wasm"))]Self::Write(v)=>v,
    Self::Empty=>&mut[],
    #[cfg(not(target_family="wasm"))]Self::Read(_)=>panic!("ReadOnlyBufferException"),
}}}
impl FileChannel{
    pub fn open(path:&str)->io::Result<Self>{Ok(Self{file:Some(OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path)?)})}
    pub fn close(&mut self)->io::Result<()>{self.file.take();Ok(())}
    pub(crate) fn map(&self,writable:bool,position:i64,size:i64)->io::Result<MappedBytes>{
        let file=self.file.as_ref().ok_or_else(||io::Error::other("ClosedChannelException"))?;
        assert!(position>=0&&size>=0&&size<=i32::MAX as i64,"IllegalArgumentException");
        let end=position.checked_add(size).expect("IllegalArgumentException");
        if file.metadata()?.len()<end as u64{file.set_len(end as u64)?;}
        if size==0{return Ok(MappedBytes::Empty);}
        #[cfg(not(target_family="wasm"))]{
            let mut options=memmap2::MmapOptions::new();options.offset(position as u64).len(size as usize);
            // SAFETY: the file remains open, the region is extended above, and ET
            // owns these temporary files. No borrowed Rust slice aliases a map.
            unsafe{if writable{options.map_mut(file).map(MappedBytes::Write)}else{options.map(file).map(MappedBytes::Read)}}
        }
        #[cfg(target_family="wasm")]{let _=writable;Err(io::Error::new(io::ErrorKind::Unsupported,"FileChannel.map host adapter unavailable on wasm"))}
    }
    pub fn read(&mut self,bytes:&mut[u8],position:i64)->io::Result<usize>{
        let file=self.file.as_mut().ok_or_else(||io::Error::other("ClosedChannelException"))?;
        assert!(position>=0,"IllegalArgumentException");file.seek(SeekFrom::Start(position as u64))?;file.read(bytes)
    }
    pub(crate) fn size(&self)->io::Result<u64>{self.file.as_ref().ok_or_else(||io::Error::other("ClosedChannelException"))?.metadata().map(|m|m.len())}
}
