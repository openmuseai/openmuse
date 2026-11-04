// java package cn.wps.moss.io;
// java
// java import cn.wps.moss.app.KmoSheet;
// java import java.io.Closeable;
// java import java.io.IOException;
// java
// java public interface BookReader extends Closeable {
// java void open() throws IOException;
// java
// java boolean isLargestFile();
// java
// java void currentGridRange(KmoSheet var1, int var2, int var3);
// java }
use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::app::KmoSheet::KmoSheet};pub trait BookReader:Send+Sync{fn open(&self)->std::io::Result<()>;fn isLargestFile(&self)->bool;fn currentGridRange(&self,s:Rc<RefCell<KmoSheet>>,first:i32,last:i32);fn close(&self)->std::io::Result<()>;}
