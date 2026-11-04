use std::{io::{self,Read,Write},fs::{self,OpenOptions}};
// java public final class XlsxrFileCache
pub struct XlsxrFileCache;
impl XlsxrFileCache {
// java File dest = File.createTempFile("xlsx-", ".tmp", rootDir); String path = dest.getAbsolutePath(); dumpTempFile(stream, path); return path;
pub fn dumpTempFile(stream:&mut dyn Read,rootDir:&str)->io::Result<String>{
// rust: The native host creates the document cache directory formerly owned by FileManager.
fs::create_dir_all(rootDir)?;loop{let path=std::path::Path::new(rootDir).join(format!("xlsx-{}.tmp",uuid::Uuid::new_v4()));match OpenOptions::new().write(true).create_new(true).open(&path){Ok(mut output)=>{let mut buffer=[0u8;8192];loop{let n=stream.read(&mut buffer)?;if n==0{break;}output.write_all(&buffer[..n])?;}output.flush()?;return Ok(path.to_string_lossy().into_owned());},Err(e) if e.kind()==io::ErrorKind::AlreadyExists=>continue,Err(e)=>return Err(e)}}}
}
