// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.util.KSFileLog
// Java: import cn.wps.ole.IOLEDocument
// Java: import cn.wps.ole.IOLEStorage
// Java: import cn.wps.ole.IOLEStream
// Java: import cn.wps.ole.OLEDocumentFactory
// Java: import cn.wps.ole.OLEEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.util.Iterator
// Java: import org.apache.poi.hpsf.ClassID
use crate::org::apache::poi::hpsf::ClassID::ClassID;
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::Entry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

use std::io::{Read, Write};

pub trait IOLEStorage {
    fn openStream(&self, name: &str) -> Option<Box<dyn IOLEStream>>;
    fn openStorage(&self, name: &str) -> Option<Box<dyn IOLEStorage>>;
    fn createStream(&mut self, name: &str) -> Option<Box<dyn IOLEStream>>;
    fn createStorage(&mut self, name: &str) -> Option<Box<dyn IOLEStorage>>;
    fn getElementCount(&self) -> i32;
    fn getStorageClsid(&self) -> Option<Vec<u8>>;
    fn setStorageClsid(&mut self, clsid: &[u8]);
    fn close(&mut self);
}

pub trait IOLEDocument {
    fn getRootStorage(&mut self) -> Option<Box<dyn IOLEStorage>>;
    fn close(&mut self);
}

pub trait IOLEStream {
    fn read(&mut self, buf: &mut [u8], offset: i32, len: i32) -> i32;
    fn write(&mut self, buf: &[u8], offset: i32, len: i32);
    fn close(&mut self);
}

pub struct OLEEntry {
    name: String,
    entry_type: i32,
}

impl OLEEntry {
    pub fn new() -> Self {
        OLEEntry { name: String::new(), entry_type: 0 }
    }
    pub fn getName(&self) -> &str { &self.name }
    pub fn getType(&self) -> i32 { self.entry_type }
}

// Java field: int BUFFER_SIZE
pub(crate) const BUFFER_SIZE: i32 = 1444;
// Java field: int IO_BUFFER_SIZE
pub(crate) const IO_BUFFER_SIZE: i32 = 8192;
// Java field: String DEFAULT_CLSID
pub(crate) const DEFAULT_CLSID: &str = "{00000000-0000-0000-0000-000000000000}";
// Java field: String[] BIN_ENTRIES
pub(crate) static BIN_ENTRIES: &[&str] = &["Package", "package", "Contents", "CorelDRAW", "XaraDrawing 1.5 (Native)", "\u{0001}Ole10Native"];

// Java: public class StorageUtil
pub struct StorageUtil {
}

// Java: public class StorageUtil
impl StorageUtil {
    // Java: StorageUtil()
    pub fn new() -> Self {
      // Java: super();
      StorageUtil {}
    }

    // Java: static void copyStorage(IOLEStorage srcStg, IOLEStorage destStg)
    pub fn copyStorage(src_stg: &dyn IOLEStorage, dest_stg: &mut dyn IOLEStorage) {
      // Java: if (!hasSameCount(srcStg, destStg))
      if !Self::hasSameCount_internal(src_stg, dest_stg) {
         // Java: int count = srcStg.getElementCount();
         let count = src_stg.getElementCount();
         // Java: OLEEntry[] entrys = new OLEEntry[count];
         let mut entrys: Vec<OLEEntry> = Vec::with_capacity(count as usize);
         // Java: for(int i = 0; i < count; ++i)
         for _i in 0..count {
            // Java: entrys[i] = new OLEEntry();
            entrys.push(OLEEntry::new());
         }
         // Java: srcStg.getElements(0, count, entrys);
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count as usize {
            // Java: switch (entrys[i].getType())
            match entrys[i].getType() {
               // Java: case 1:
               1 => {
                  // Java: IOLEStorage srcStgNew = srcStg.openStorage(entrys[i].getName());
                  // Java: IOLEStorage destStgNew = destStg.createStorage(entrys[i].getName());
                  // Java: copyStorage(srcStgNew, destStgNew);
                  // Java: destStgNew.close();
                  // Java: srcStgNew.close();
                  // Java: break;
               }
               // Java: case 2:
               2 => {
                  // Java: IOLEStream srcStmNew = srcStg.openStream(entrys[i].getName());
                  // Java: IOLEStream destStmNew = destStg.createStream(entrys[i].getName());
                  // Java: byte[] buffer = new byte[1444];
                  let mut buffer = [0u8; 1444];
                  // Java: int len = 0;
                  // Java: while((len = srcStmNew.read(buffer, 0, 1444)) > 0)
                  if let Some(mut src_stm) = src_stg.openStream(entrys[i].getName()) {
                      if let Some(mut dest_stm) = dest_stg.createStream(entrys[i].getName()) {
                          loop {
                              let len = src_stm.read(&mut buffer, 0, 1444);
                              if len <= 0 { break; }
                              // Java: destStmNew.write(buffer, 0, len);
                              dest_stm.write(&buffer, 0, len);
                          }
                          // Java: srcStmNew.close();
                          src_stm.close();
                          // Java: destStmNew.close();
                          dest_stm.close();
                      }
                  }
               }
               _ => {}
            }
         }
      }
    }

    // Java: static void writeEntry2File(String desPath, DirectoryNode directoryNode)
    pub fn writeEntry2File(_des_path: &str, _directory_node: &mut DirectoryNode) {
      // Java: IOLEDocument document = null;
      // Java: IOLEStorage storage = null;
      // Java: try
      {
         // Java: document = OLEDocumentFactory.create(desPath, 2);
         // Java: storage = document.getRootStorage();
         // Java: ClassID classId = directoryNode.getStorageClsid();
         // Java: storage.setStorageClsid(classId.getBytesLittleEndian());
         // Java: writeStorage(storage, directoryNode);
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.d("StorageUtil", "writeEntry2File", e);
      }
      // Java: finally
      {
         // Java: closeDocument(document);
         // Java: closeStorage(storage);
      }
    }

    // Java: static void closeDocument(IOLEDocument document)
    pub fn closeDocument(document: Option<&mut dyn IOLEDocument>) {
      // Java: try
      {
         // Java: if (null != document)
         if let Some(doc) = document {
            // Java: document.close();
            doc.close();
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.d("StorageUtil", "closeDocument", e);
      }
    }

    // Java: static void closeStorage(IOLEStorage storage)
    pub fn closeStorage(storage: Option<&mut dyn IOLEStorage>) {
      // Java: try
      {
         // Java: if (null != storage)
         if let Some(stg) = storage {
            // Java: storage.close();
            stg.close();
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.d("StorageUtil", "closeStorage", e);
      }
    }

    // Java: static void closeStream(IOLEStream stream)
    pub fn closeStream(stream: Option<&mut dyn IOLEStream>) {
      // Java: try
      {
         // Java: if (null != stream)
         if let Some(stm) = stream {
            // Java: stream.close();
            stm.close();
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.d("StorageUtil", "closeStream", e);
      }
    }

    // Java: static boolean hasSameCount(IOLEStorage srcStg, IOLEStorage destStg)
    pub fn hasSameCount(src_stg: Option<&dyn IOLEStorage>, dest_stg: Option<&dyn IOLEStorage>) -> bool {
      // Java: if (null != srcStg && null != destStg)
      if let (Some(src), Some(dest)) = (src_stg, dest_stg) {
         // Java: try
         {
            // Java: if (srcStg.getElementCount() == destStg.getElementCount())
            if src.getElementCount() == dest.getElementCount() {
               // Java: return true;
               return true;
            }
         }
         // Java: catch (IOException e)
         {
            // Java: IOLog.d("StorageUtil", "hasSameEntries", e);
         }
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    fn hasSameCount_internal(src_stg: &dyn IOLEStorage, dest_stg: &dyn IOLEStorage) -> bool {
        src_stg.getElementCount() == dest_stg.getElementCount()
    }

    // Java: static void writeStorage(IOLEStorage storage, Entry entry)
    pub fn writeStorage(_storage: &mut dyn IOLEStorage, entry: &mut DirectoryNode) {
      // Java: IOAssert.assertNotNull("storage should not be null!", storage);
      // Java: IOAssert.assertNotNull("entry should not be null!", entry);
      // Java: Iterator<Entry> iter = ((DirectoryNode)entry).getEntries();
      let entries = entry.getEntries();
      // Java: try
      {
         // Java: while(iter.hasNext())
         for _entry_node in entries.iter() {
            // Java: Entry entry1 = (Entry)iter.next();
            // Java: if (entry1.isDirectoryEntry() && checkInterrupted())
            if _entry_node.isDirectoryEntry() && Self::checkInterrupted() {
               // Java: IOLEStorage stgNew = storage.createStorage(entry1.getName());
               // Java: stgNew.setStorageClsid(((DirectoryNode)entry1).getStorageClsid().getBytesLittleEndian());
               // Java: writeStorage(stgNew, entry1);
               // Java: stgNew.close();
            }
            // Java: else
            else {
               // Java: IOLEStream destStm = storage.createStream(entry1.getName());
               // Java: DocumentInputStream srcStm = ((DirectoryNode)entry).createDocumentInputStream(entry1);
               // Java: writeStream(destStm, srcStm);
               // Java: srcStm.close();
               // Java: destStm.close();
            }
         }
      }
      // Java: catch (FileDamagedException e)
      {
         // Java: throw new IOException(e);
      }
    }

    // Java: static boolean checkInterrupted()
    pub fn checkInterrupted() -> bool {
      // Java: if (Thread.currentThread().isInterrupted())
      // Java: throw new OleParseInterruptException();
      // Java: else
      {
         // Java: return true;
         return true;
      }
    }

    // Java: static void writeStream(IOLEStream stream, DocumentInputStream in)
    pub fn writeStream_doc(stream: &mut dyn IOLEStream, input: &mut DocumentInputStream) {
      // Java: IOAssert.assertNotNull("stream should not be null!", stream);
      // Java: IOAssert.assertNotNull("in should not be null!", in);
      // Java: byte[] buffer = new byte[4096];
      let mut buffer = [0u8; 4096];
      // Java: int len = 0;
      let mut len: i32;
      // Java: while(checkInterrupted() && (len = in.read(buffer)) > 0)
      loop {
         if !Self::checkInterrupted() { break; }
         len = input.read(&mut buffer);
         if len <= 0 { break; }
         // Java: stream.write(buffer, 0, len);
         stream.write(&buffer, 0, len);
      }
    }

    // Java: static void copyOleFile2Storage(String oleFilepath, IOLEStorage desStorage)
    pub fn copyOleFile2Storage(ole_filepath: Option<&str>, des_storage: Option<&mut dyn IOLEStorage>) {
      // Java: if (null != oleFilepath && null != desStorage)
      if let (Some(_filepath), Some(_des_stg)) = (ole_filepath, des_storage) {
         // Java: IOLEDocument doc = null;
         // Java: try
         {
            // Java: doc = OLEDocumentFactory.open(oleFilepath, 2);
            // Java: IOLEStorage srcStg = doc.getRootStorage();
            // Java: copyStorage(srcStg, desStorage);
            // Java: if (null != srcStg.getStorageClsid())
            {
               // Java: desStorage.setStorageClsid(srcStg.getStorageClsid());
            }
            // Java: doc.close();
            // Java: srcStg.close();
         }
         // Java: catch (IOException e)
         {
            // Java: IOLog.d("StorageUtil", "copyOleFile2Storage failed", e);
         }
      }
      // Java: else
      else {
         // Java: IOLog.e("StorageUtil", "copyOleFile2Storage illegal param");
      }
    }

    // Java: static void copyStorage2OleFile(IOLEStorage srcStorage, String fiePath)
    pub fn copyStorage2OleFile(src_storage: Option<&dyn IOLEStorage>, fie_path: Option<&str>) {
      // Java: if (null != fiePath && null != srcStorage)
      if let (Some(_path), Some(_src)) = (fie_path, src_storage) {
         // Java: IOLEDocument doc = null;
         // Java: try
         {
            // Java: doc = OLEDocumentFactory.create(fiePath, 2);
            // Java: IOLEStorage desStorage = doc.getRootStorage();
            // Java: copyStorage(srcStorage, desStorage);
            // Java: if (null != srcStorage.getStorageClsid())
            {
               // Java: desStorage.setStorageClsid(srcStorage.getStorageClsid());
            }
            // Java: desStorage.close();
            // Java: doc.close();
         }
         // Java: catch (IOException e)
         {
            // Java: IOLog.d("StorageUtil", "copyStorage2OleFile failed", e);
         }
      }
      // Java: else
      else {
         // Java: IOLog.e("StorageUtil", "copyStorage2OleFile illegal param");
      }
    }

    // Java: static boolean isOleFile(String filePath)
    pub fn isOleFile(file_path: &str) -> bool {
      // Java: return containEntry(filePath, "\u0003ObjInfo");
      Self::containEntry(file_path, Some("\u{0003}ObjInfo"))
    }

    // Java: static boolean isPackedFile(String filePath)
    pub fn isPackedFile(file_path: &str) -> bool {
      // Java: return containEntry(filePath, "Package");
      Self::containEntry(file_path, Some("Package"))
    }

    // Java: static boolean isBinFile(String filePath)
    pub fn isBinFile(file_path: &str) -> bool {
      // Java: RandomAccessFile file = null;
      // Java: POIFSFileSystem poifsFileSystem = null;
      // Java: boolean var4;
      let var4: bool;
      // Java: try
      {
         // Java: file = new RandomAccessFile(filePath, "r");
         let file = std::fs::File::open(file_path);
         if file.is_err() { return false; }
         let mut file = file.unwrap();
         // Java: if (!POIFSFileSystem.hasPOIFSHeader(file))
         {
            // Java: return false;
         }
         // Java: poifsFileSystem = new POIFSFileSystem(file);
         let mut poifs_file_system = match POIFSFileSystem::from_stream(&mut file) {
             Ok(fs) => fs,
             Err(_) => return false,
         };
         // Java: DirectoryNode root = poifsFileSystem.getRoot();
         let root = poifs_file_system.getRoot();
         // Java: if (null != root)
         {
            // Java: for(String entry : BIN_ENTRIES)
            for entry in BIN_ENTRIES {
               // Java: if (root.hasEntry(entry))
               if root.hasEntry(entry) {
                  // Java: boolean var8 = true;
                  // Java: return var8;
                  return true;
               }
            }
            // Java: return false;
            return false;
         }
         // Java: var4 = false;
         var4 = false;
      }
      // Java: catch (Exception e)
      {
         // Java: IOLog.d("StorageUtil", "isBinFile failed", e);
         // Java: return false;
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(file);
         // Java: if (null != poifsFileSystem)
         {
            // Java: poifsFileSystem.dispose();
         }
      }
      // Java: return var4;
      var4
    }

    // Java: static boolean containEntry(String filePath, String entry)
    pub fn containEntry(file_path: &str, entry: Option<&str>) -> bool {
      // Java: RandomAccessFile file = null;
      // Java: POIFSFileSystem poifsFileSystem = null;
      // Java: boolean var5;
      let var5: bool;
      // Java: try
      {
         // Java: file = new RandomAccessFile(filePath, "r");
         let file = std::fs::File::open(file_path);
         if file.is_err() { return false; }
         let mut file = file.unwrap();
         // Java: if (!POIFSFileSystem.hasPOIFSHeader(file))
         {
            // Java: return false;
         }
         // Java: poifsFileSystem = new POIFSFileSystem(file);
         let mut poifs_file_system = match POIFSFileSystem::from_stream(&mut file) {
             Ok(fs) => fs,
             Err(_) => return false,
         };
         // Java: DirectoryNode root = poifsFileSystem.getRoot();
         let root = poifs_file_system.getRoot();
         // Java: var5 = null != root && (entry == null || root.hasEntry(entry));
         var5 = match entry {
             None => true,
             Some(e) => root.hasEntry(e),
         };
      }
      // Java: catch (Exception e)
      {
         // Java: IOLog.d("StorageUtil", "isOleFile failed", e);
         // Java: return false;
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(file);
         // Java: if (null != poifsFileSystem)
         {
            // Java: poifsFileSystem.dispose();
         }
      }
      // Java: return var5;
      var5
    }

    // Java: static boolean isPackedOleFile(String filePath)
    pub fn isPackedOleFile(file_path: &str) -> bool {
      // Java: String[] entrys = new String[]{"Package", "package"};
      let entrys = ["Package", "package"];
      // Java: return containDiffEntry(filePath, entrys);
      Self::containDiffEntry(file_path, Some(&entrys))
    }

    // Java: static boolean containDiffEntry(String filePath, String[] entrys)
    pub fn containDiffEntry(file_path: &str, entrys: Option<&[&str]>) -> bool {
      // Java: RandomAccessFile file = null;
      // Java: POIFSFileSystem poifsFileSystem = null;
      // Java: boolean var5;
      let var5: bool;
      // Java: try
      {
         // Java: file = new RandomAccessFile(filePath, "r");
         let file = std::fs::File::open(file_path);
         if file.is_err() { return false; }
         let mut file = file.unwrap();
         // Java: if (!POIFSFileSystem.hasPOIFSHeader(file))
         {
            // Java: return false;
         }
         // Java: poifsFileSystem = new POIFSFileSystem(file);
         let mut poifs_file_system = match POIFSFileSystem::from_stream(&mut file) {
             Ok(fs) => fs,
             Err(_) => return false,
         };
         // Java: DirectoryNode root = poifsFileSystem.getRoot();
         let root = poifs_file_system.getRoot();
         // Java: if (root != null)
         {
            // Java: if (entrys == null)
            if entrys.is_none() {
               // Java: var5 = true;
               // Java: return var5;
               return true;
            }
            // Java: for(String entry : entrys)
            if let Some(entries) = entrys {
                for entry in entries {
                   // Java: if (root.hasEntry(entry))
                   if root.hasEntry(entry) {
                      // Java: boolean var9 = true;
                      // Java: return var9;
                      return true;
                   }
                }
            }
            // Java: var5 = false;
            // Java: return var5;
            return false;
         }
         // Java: var5 = false;
         var5 = false;
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.d("StorageUtil", "isOleFile failed", e);
         // Java: return false;
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(file);
         // Java: if (null != poifsFileSystem)
         {
            // Java: poifsFileSystem.dispose();
         }
      }
      // Java: return var5;
      var5
    }

    // Java: static boolean isCompoundFile(String filePath)
    pub fn isCompoundFile(file_path: &str) -> bool {
      // Java: return containEntry(filePath, (String)null);
      Self::containEntry(file_path, None)
    }

    // Java: static void writeStorageClsid(String oleFilepath, String clsId)
    pub fn writeStorageClsid(ole_filepath: &str, cls_id: Option<&str>) {
      // Java: if (null != clsId)
      if let Some(_cls) = cls_id {
         // Java: IOLEDocument doc = null;
         // Java: try
         {
            // Java: doc = OLEDocumentFactory.open(oleFilepath, 2);
            // Java: IOLEStorage srcStg = doc.getRootStorage();
            // Java: srcStg.setStorageClsid(ClassID.toBytes2(clsId));
            // Java: srcStg.close();
            // Java: doc.close();
         }
         // Java: catch (IOException e)
         {
            // Java: IOLog.d("StorageUtil", "writeStorageClsid failed", e);
         }
      }
    }

    // Java: static String readStorageClsid(String oleFilepath)
    pub fn readStorageClsid(ole_filepath: Option<&str>) -> Option<String> {
      // Java: if (null == oleFilepath)
      if ole_filepath.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: IOLEDocument doc = null;
         // Java: IOLEStorage srcStg = null;
         // Java: String result;
         let result: Option<String>;
         // Java: try
         {
            // Java: doc = OLEDocumentFactory.open(oleFilepath, 2);
            // Java: if (null == doc)
            {
               // Java: return null;
            }
            // Java: srcStg = doc.getRootStorage();
            // Java: if (null == srcStg || null == srcStg.getStorageClsid())
            {
               // Java: return null;
            }
            // Java: result = (new ClassID(srcStg.getStorageClsid(), 0)).toString();
            result = None;
         }
         // Java: catch (IOException e)
         {
            // Java: IOLog.d("StorageUtil", "readStorageClsid", e);
            // Java: return null;
         }
         // Java: finally
         {
            // Java: closeDocument(doc);
            // Java: closeStorage(srcStg);
         }
         // Java: return result;
         result
      }
    }

    // Java: static boolean writeToFile(String strFileName, InputStream ins)
    pub fn writeToFile(str_file_name: Option<&str>, ins: Option<&mut dyn Read>) -> bool {
      // Java: return writeToFile(strFileName, ins, false);
      Self::writeToFile_append(str_file_name, ins, false)
    }

    // Java: static boolean writeToFile(String strFileName, InputStream ins, boolean append)
    pub fn writeToFile_append(str_file_name: Option<&str>, ins: Option<&mut dyn Read>, append: bool) -> bool {
      // Java: return writeToFile(strFileName, ins, append, true);
      Self::writeToFile_full(str_file_name, ins, append, true)
    }

    // Java: static boolean writeToFile(String strFileName, InputStream ins, boolean append, boolean closeInput)
    pub fn writeToFile_full(str_file_name: Option<&str>, ins: Option<&mut dyn Read>, _append: bool, _close_input: bool) -> bool {
      // Java: if (null != strFileName && null != ins)
      if let (Some(file_name), Some(input)) = (str_file_name, ins) {
         // Java: FileOutputStream fouts = null;
         // Java: try
         {
            // Java: fouts = new FileOutputStream(strFileName, append);
            let fouts = std::fs::File::create(file_name);
            if let Ok(mut fouts) = fouts {
                // Java: copy(ins, fouts, (byte[])null);
                let mut buf = [0u8; IO_BUFFER_SIZE as usize];
                loop {
                    let read = match input.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    if fouts.write_all(&buf[..read]).is_err() { break; }
                }
                // Java: boolean e = true;
                // Java: return e;
                return true;
            }
         }
         // Java: catch (IOException e)
         {
            // Java: e.printStackTrace();
            // Java: KSFileLog.eForSave("oleFileUtil", "writeToFile", e);
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(fouts);
            // Java: if (closeInput)
            {
               // Java: FileUtil.closeQuietly(ins);
            }
         }
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: static void copy(InputStream in, OutputStream out, byte[] buffer)
    pub fn copy(input: &mut dyn Read, output: &mut dyn Write, buffer: Option<&mut Vec<u8>>) {
      // Java: if (null == buffer)
      let mut default_buf = vec![0u8; IO_BUFFER_SIZE as usize];
      let buf = match buffer {
         None => {
            // Java: buffer = new byte[8192];
            default_buf.as_mut_slice()
         }
         Some(b) => b.as_mut_slice(),
      };
      // Java: int read;
      // Java: while(checkInterrupted() && (read = in.read(buffer)) != -1)
      loop {
          if !Self::checkInterrupted() { break; }
          let read = match input.read(buf) {
              Ok(0) => break,
              Ok(n) => n,
              Err(_) => break,
          };
          // Java: out.write(buffer, 0, read);
          output.write_all(&buf[..read]).ok();
      }
    }

    // Java: static void copyEntry(IOLEStorage srcStg, DirectoryEntry desEntry)
    pub fn copyEntry(src_stg: &dyn IOLEStorage, _des_entry: &mut DirectoryNode) {
      // Java: int count = srcStg.getElementCount();
      let count = src_stg.getElementCount();
      // Java: OLEEntry[] entrys = new OLEEntry[count];
      let mut entrys: Vec<OLEEntry> = Vec::with_capacity(count as usize);
      // Java: for(int i = 0; i < count; ++i)
      for _i in 0..count {
         // Java: entrys[i] = new OLEEntry();
         entrys.push(OLEEntry::new());
      }
      // Java: srcStg.getElements(0, count, entrys);
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count as usize {
         // Java: switch (entrys[i].getType())
         match entrys[i].getType() {
            // Java: case 1:
            1 => {
               // Java: IOLEStorage srcStgNew = srcStg.openStorage(entrys[i].getName());
               // Java: DirectoryEntry newNode = desEntry.createDirectory(entrys[i].getName());
               // Java: copyEntry(srcStgNew, newNode);
               // Java: srcStgNew.close();
               // Java: break;
            }
            // Java: case 2:
            2 => {
               // Java: writeStream(srcStg, desEntry, entrys[i].getName());
            }
            _ => {}
         }
      }
    }

    // Java: static void writeStream(IOLEStorage srcStg, DirectoryEntry desEntry, String entryName)
    pub fn writeStream_entry(_src_stg: &dyn IOLEStorage, _des_entry: &mut DirectoryNode, _entry_name: &str) {
      // Java: FileOutputStream outputStream = null;
      // Java: IOLEStream srcStream = null;
      // Java: try
      {
         // Java: File tmp = Platform.createTempFile("entry", "bin");
         // Java: srcStream = srcStg.openStream(entryName);
         // Java: outputStream = new FileOutputStream(tmp);
         // Java: byte[] buffer = new byte[1444];
         // Java: int len = 0;
         // Java: while((len = srcStream.read(buffer, 0, 1444)) > 0)
         {
            // Java: outputStream.write(buffer, 0, len);
         }
         // Java: outputStream.flush();
         // Java: desEntry.createDocument(entryName, new FileInputStream(tmp));
      }
      // Java: finally
      {
         // Java: if (outputStream != null)
         {
            // Java: outputStream.close();
         }
         // Java: srcStream.close();
      }
    }

    // Java: static void copyOleFile2Entry(String oleFilepath, DirectoryEntry desEntry)
    pub fn copyOleFile2Entry(_ole_filepath: &str, _des_entry: &mut DirectoryNode) {
      // Java: IOLEDocument doc = null;
      // Java: try
      {
         // Java: doc = OLEDocumentFactory.open(oleFilepath, 2);
         // Java: IOLEStorage srcStg = doc.getRootStorage();
         // Java: copyEntry(srcStg, desEntry);
         // Java: if (null != srcStg.getStorageClsid())
         {
            // Java: desEntry.setStorageClsid(new ClassID(srcStg.getStorageClsid(), 0));
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.e("StorageUtil", "copyOleFile2Entry failed", e);
      }
    }

    // Java: static boolean isCoumpondFile(String filePath)
    pub fn isCoumpondFile(file_path: &str) -> bool {
      // Java: RandomAccessFile file = null;
      // Java: boolean isOle = false;
      let mut is_ole = false;
      // Java: try
      {
         // Java: file = new RandomAccessFile(filePath, "r");
         let file = std::fs::File::open(file_path);
         if let Ok(mut f) = file {
            // Java: isOle = POIFSFileSystem.hasPOIFSHeader(file);
            is_ole = POIFSFileSystem::from_stream(&mut f).is_ok();
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.e("StorageUtil", "IOException", e);
      }
      // Java: if (null != file)
      {
         // Java: try
         {
            // Java: file.close();
         }
         // Java: catch (IOException e)
         {
            // Java: IOLog.e("StorageUtil", "IOException", e);
         }
      }
      // Java: return isOle;
      is_ole
    }

    // Java: static boolean hasClsid(DirectoryEntry entry)
    pub fn hasClsid(entry: Option<&DirectoryNode>) -> bool {
      // Java: if (null != entry && entry.getStorageClsid() != null)
      if let Some(e) = entry {
         // Java: String clsId = entry.getStorageClsid().toString();
         let cls_id = e.getStorageClsid().toString();
         // Java: return !"{00000000-0000-0000-0000-000000000000}".equals(clsId);
         cls_id != "{00000000-0000-0000-0000-000000000000}"
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

}
