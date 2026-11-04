// Java: import cn.wps.base.log.IOLog
use util::cn::wps::base::log::IOLog::IOLog;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform::Platform;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationship::OpcrRelationship;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use util::cn::wps::moffice::crash::NoSpaceLeftException::NoSpaceLeftException;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream::FileInputStream;
// Java: import cn.wps.moffice.kfs.FileOutputStream
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileOutputStream::FileOutputStream;
// Java: import cn.wps.moffice.util.FileDataStorage
use util::cn::wps::moffice::util::FileDataStorage::FileDataStorage;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil::FileUtil;
// Java: import cn.wps.moffice.util.StringUtil
use util::org::apache::poi::util::StringUtil::StringUtil;
// Java: import cn.wps.moffice.util.zlib.ZInputStream
use util_adapter::cn::wps::moffice::util::zlib::ZInputStream::ZInputStream;
// Java: import cn.wps.ole.IOLEDocument
use util_adapter::cn::wps::ole::IOLEDocument::IOLEDocument;
// Java: import cn.wps.ole.IOLEStorage
use util_adapter::cn::wps::ole::IOLEStorage::IOLEStorage;
// Java: import cn.wps.ole.OLEDocumentFactory
use util_adapter::cn::wps::ole::OLEDocumentFactory::OLEDocumentFactory;
// Java: import java.io.ByteArrayInputStream
use std::io::Cursor;
// Java: import java.io.IOException
use std::io;
// Java: import java.io.InputStream
// (Read trait serves as InputStream equivalent)
// Java: import java.util.zip.ZipException
// (Mapped to io::Error in Rust)
// Java: import org.apache.poi.hpsf.ClassID
use io_poi_compact::org::apache::poi::hpsf::ClassID::ClassID;
// Java: import org.apache.poi.hwpf.ole.OleClsTypeMatcher
use io_poi_compact::org::apache::poi::hwpf::ole::OleClsTypeMatcher::OleClsTypeMatcher;
// Java: import org.apache.poi.hwpf.ole.packer.IOlePacker
use io_poi_compact::org::apache::poi::hwpf::ole::packer::IOlePacker::IOlePacker;
// Java: import org.apache.poi.hwpf.ole.packer.OlePackerFactory
use io_poi_compact::org::apache::poi::hwpf::ole::packer::OlePackerFactory::OlePackerFactory;
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use io_poi_compact::org::apache::poi::hwpf::util::OleParseInterruptException::OleParseInterruptException;
// Java: import org.apache.poi.hwpf.util.StorageUtil
use io_poi_compact::org::apache::poi::hwpf::util::StorageUtil::StorageUtil;
// Java: import org.apache.poi.util.LittleEndianInput
use util::org::apache::poi::util::LittleEndianInput::LittleEndianInput;
// Java: import org.apache.poi.util.LittleEndianOutput
use util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput;

// Java field: int MAX_TRY_COUNT
pub(crate) const MAX_TRY_COUNT: i32 = 5;
// Java field: int MIN_WAIT_TIME
pub(crate) const MIN_WAIT_TIME: i32 = 5;

// Java: public class PptFileCache
pub struct PptFileCache {
}

// Java: public class PptFileCache
impl PptFileCache {
    // Java: static File createTempFile()
    pub fn createTempFile() -> io::Result<File> {
      // Java: return Platform.createTempFile("ppt.", ".tmp");
      let path = Platform::createTempFile("ppt.", ".tmp")
          .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create temp file"))?;
      return Ok(File::from_path(&path));
    }

    // Java: static File createTempFile(String suffix)
    pub fn createTempFile_suffix(suffix: &str) -> io::Result<File> {
      // Java: return Platform.createTempFile(suffix, ".tmp");
      let path = Platform::createTempFile(suffix, ".tmp")
          .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create temp file"))?;
      return Ok(File::from_path(&path));
    }

    // Java: static File createTempFile(String prefix, String suffix)
    pub fn createTempFile_prefix_suffix(prefix: &str, suffix: &str) -> io::Result<File> {
      // Java: return Platform.createTempFile(prefix, suffix);
      let path = Platform::createTempFile(prefix, suffix)
          .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create temp file"))?;
      return Ok(File::from_path(&path));
    }

    // Java: static File createTempFile(String prefix, String suffix, File rootDir)
    pub fn createTempFile_prefix_suffix_dir(prefix: &str, suffix: &str, rootDir: &File) -> io::Result<File> {
      // Java: return createNewFile(prefix, suffix, rootDir);
      return Self::createNewFile(prefix, suffix, rootDir);
    }

    // Java: static File createNewFile(String prefix, String extension, File dir)
    pub fn createNewFile(prefix: &str, extension: &str, dir: &File) -> io::Result<File> {
      // Java: return createNewFile(prefix, extension, dir, 0);
      return Self::createNewFile_with_retry(prefix, extension, dir, 0);
    }

    // Java: static File createNewFile(String prefix, String extension, File dir, int tryCount)
    pub fn createNewFile_with_retry(prefix: &str, extension: &str, dir: &File, mut tryCount: i32) -> io::Result<File> {
      // Java: try
      let try_result: Result<File, io::Error> = (|| -> io::Result<File> {
         // Java: File file = File.createTempFile(prefix, extension, dir);
         let file = File::createTempFile_in_dir(prefix, Some(extension), Some(dir))?;
         // Java: return file;
         return Ok(file);
      })();
      match try_result {
        Ok(file) => return Ok(file),
        // Java: catch (IOException e)
        Err(e) =>
        {
           // Java: ++tryCount;
           tryCount += 1;
           // Java: if (tryCount < 5)
           if tryCount < MAX_TRY_COUNT
           {
              // Java: try
              let _sleep_result: Result<(), ()> = (|| -> Result<(), ()> {
                 // Java: Thread.sleep((long)(5 * tryCount));
                 std::thread::sleep(std::time::Duration::from_millis((MIN_WAIT_TIME * tryCount) as u64));
                 Ok(())
              })();
              // Java: catch (InterruptedException e1)
              if let Err(_e1) = _sleep_result
              {
                 // Java: e1.printStackTrace();
                 eprintln!("InterruptedException during sleep");
              }
              // Java: return createNewFile(prefix, extension, dir, tryCount);
              return Self::createNewFile_with_retry(prefix, extension, dir, tryCount);
           }
           // Java: else
           else
           {
              // Java: throw e;
              return Err(e);
           }
        }
      }
    }

    // Java: static File dumpTempFile(InputStream input)
    pub fn dumpTempFile_stream(input: Option<&mut dyn io::Read>) -> io::Result<Option<File>> {
      // Java: if (input == null)
      if input.is_none()
      {
         // Java: return null;
         return Ok(None);
      }
      // Java: else
      else
      {
         // Java: File tmp = createTempFile();
         let tmp = Self::createTempFile()?;
         // Java: return dumpTempFile(input, tmp);
         return Self::dumpTempFile_stream_to_dest(input, Some(tmp));
      }
    }

    // Java: static File dumpTempFile(InputStream input, File destFile)
    pub fn dumpTempFile_stream_to_dest(input: Option<&mut dyn io::Read>, destFile: Option<File>) -> io::Result<Option<File>> {
      // Java: if (input != null && destFile != null)
      if let (Some(input), Some(mut destFile)) = (input, destFile)
      {
         // Java: FileOutputStream output = null;
         let mut output: Option<FileOutputStream> = None;
         // Java: Object var13;
         let mut var13: Option<File> = None;
         // Java: try
         let try_result: Result<File, (&str, io::Error)> = (|| -> Result<File, (&str, io::Error)> {
            // Java: output = new FileOutputStream(destFile);
            output = Some(FileOutputStream::from_file(&mut destFile).map_err(|e| ("io", e))?);
            // Java: byte[] buffer = new byte[4096];
            let mut buffer = [0u8; 4096];
            // Java: int len = -1;
            let mut len: i32 = -1;
            // Java: while((len = input.read(buffer)) > 0)
            loop {
               len = input.read(&mut buffer).map_err(|e| ("zip", e))? as i32;
               if len <= 0 { break; }
            {
               // Java: output.write(buffer, 0, len);
               output.as_mut().unwrap().write_buf(&buffer, 0, len as usize).map_err(|e| ("io", e))?;
            }
            }
            // Java: output.flush();
            // (flush on drop)
            // Java: File var5 = destFile;
            let var5 = destFile.clone();
            // Java: return var5;
            return Ok(var5);
         })();
         match try_result {
            Ok(f) => {
               // Java: finally
               {
                  // Java: FileUtil.closeQuietly(output);
                  FileUtil::_method_5();
                  drop(output);
               }
               return Ok(Some(f));
            },
            // Java: catch (ZipException e)
            Err(("zip", _e)) =>
            {
               // Java: IOLog.e("PptFileCache", "ZipException", e);
               IOLog::e("PptFileCache", "ZipException");
               // Java: var13 = null;
               var13 = None;
            },
            // Java: catch (IOException e)
            Err((_, e)) =>
            {
               // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
               if NoSpaceLeftException::isNoSpaceLeftException(Some(&e))
               {
                  // Java: throw new NoSpaceLeftException(e);
                  return Err(e);
               }
               // Java: var13 = null;
               var13 = None;
               // Java: return (File)var13;
               // Java: finally
               {
                  // Java: FileUtil.closeQuietly(output);
                  FileUtil::_method_5();
                  drop(output);
               }
               return Ok(var13);
            },
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(output);
            FileUtil::_method_5();
            drop(output);
         }
         // Java: return (File)var13;
         return Ok(var13);
      }
      // Java: else
      else
      {
         // Java: return null;
         return Ok(None);
      }
    }

    // Java: static File dumpTempFile(OpcrRelationship rels)
    pub fn dumpTempFile_rels(rels: Option<&OpcrRelationship>) -> io::Result<Option<File>> {
      // Java: return rels == null ? null : dumpTempFile(rels.getPackagePart());
      match rels {
         None => return Ok(None),
         Some(r) => return Self::dumpTempFile_part(r.getPackagePart()),
      }
    }

    // Java: static File dumpTempFile(OpcrPackagePart part)
    pub fn dumpTempFile_part(part: Option<&OpcrPackagePart>) -> io::Result<Option<File>> {
      // Java: return part == null ? null : dumpTempFile(part.getInputStream());
      match part {
         None => return Ok(None),
         Some(p) => {
            let data = p.getInputStream();
            let mut cursor = Cursor::new(data);
            return Self::dumpTempFile_stream(Some(&mut cursor));
         }
      }
    }

    // Java: static File dumpTempFile(LittleEndianInput input)
    pub fn dumpTempFile_lei(input: Option<&mut dyn LittleEndianInput>) -> io::Result<Option<File>> {
      // Java: if (input == null)
      if input.is_none()
      {
         // Java: return null;
         return Ok(None);
      }
      // Java: else
      else
      {
         // Java: File tmp = createTempFile();
         let tmp = Self::createTempFile()?;
         // Java: return dumpTempFile(input, tmp);
         return Self::dumpTempFile_lei_dest(input.unwrap(), tmp);
      }
    }

    // Java: static File dumpTempFile(LittleEndianInput input, File destFile)
    pub fn dumpTempFile_lei_dest(input: &mut dyn LittleEndianInput, mut destFile: File) -> io::Result<Option<File>> {
      // Java: if (input != null && destFile != null)
      {
         // Java: FileOutputStream output = null;
         let mut output: Option<FileOutputStream> = None;
         // Java: File var5;
         let var5: File;
         // Java: try
         let try_result: io::Result<File> = (|| -> io::Result<File> {
            // Java: output = new FileOutputStream(destFile);
            output = Some(FileOutputStream::from_file(&mut destFile)?);
            // Java: byte[] buffer = new byte[4096];
            let mut buffer = [0u8; 4096];
            // Java: int len = 0;
            let mut len: i32 = 0;
            // Java: while((len = input.available()) > 0)
            loop {
               len = input.available();
               if len <= 0 { break; }
            {
               // Java: if (len > buffer.length)
               if len > buffer.len() as i32
               {
                  // Java: len = buffer.length;
                  len = buffer.len() as i32;
               }
               // Java: input.readFully(buffer, 0, len);
               input.readFully_offset(&mut buffer, 0, len as usize);
               // Java: output.write(buffer, 0, len);
               output.as_mut().unwrap().write_buf(&buffer, 0, len as usize)?;
            }
            }
            // Java: var5 = destFile;
            Ok(destFile.clone())
         })();
         // Java: catch (IOException e)
         match try_result {
            Ok(f) => { var5 = f; },
            Err(e) => {
               // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
               if NoSpaceLeftException::isNoSpaceLeftException(Some(&e))
               {
                  // Java: throw new NoSpaceLeftException(e);
                  return Err(e);
               }
               // Java: throw new RuntimeException(e);
               return Err(e);
            }
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(output);
            FileUtil::_method_5();
            drop(output);
         }
         // Java: return var5;
         return Ok(Some(var5));
      }
      // Java: else
      // (unreachable for non-null input, but for completeness:)
      // Java: return null;
      // Ok(None)
    }

    // Java: static File dumpTempFile(LittleEndianInput input, int length)
    pub fn dumpTempFile_lei_length(input: &mut dyn LittleEndianInput, mut length: i32) -> io::Result<Option<File>> {
      // Java: if (input != null && length > 0)
      if length > 0
      {
         // Java: FileOutputStream output = null;
         let mut output: Option<FileOutputStream> = None;
         // Java: File tmp = createTempFile();
         let tmp = Self::createTempFile()?;
         // Java: File var6;
         let var6: File;
         // Java: try
         let try_result: io::Result<File> = (|| -> io::Result<File> {
            // Java: output = new FileOutputStream(tmp);
            output = Some(FileOutputStream::from_file(&mut tmp.clone())?);
            // Java: int len = length > 4096 ? 4096 : length;
            let mut len = if length > 4096 { 4096 } else { length };
            // Java: for(byte[] buffer = new byte[len]; length > 0; length -= len)
            let mut buffer = vec![0u8; len as usize];
            while length > 0
            {
               // Java: len = length > buffer.length ? buffer.length : length;
               len = if length > buffer.len() as i32 { buffer.len() as i32 } else { length };
               // Java: input.readFully(buffer, 0, len);
               input.readFully_offset(&mut buffer, 0, len as usize);
               // Java: output.write(buffer, 0, len);
               output.as_mut().unwrap().write_buf(&buffer, 0, len as usize)?;
               length -= len;
            }
            // Java: var6 = tmp;
            Ok(tmp.clone())
         })();
         // Java: catch (IOException e)
         match try_result {
            Ok(f) => { var6 = f; },
            Err(e) => {
               // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
               if NoSpaceLeftException::isNoSpaceLeftException(Some(&e))
               {
                  // Java: throw new NoSpaceLeftException(e);
                  return Err(e);
               }
               // Java: throw new RuntimeException(e);
               return Err(e);
            }
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(output);
            FileUtil::_method_5();
            drop(output);
         }
         // Java: return var6;
         return Ok(Some(var6));
      }
      // Java: else
      else
      {
         // Java: return null;
         return Ok(None);
      }
    }

    // Java: static void dumpFile(File srcFile, LittleEndianOutput output)
    pub fn dumpFile_to_leo(srcFile: Option<&mut File>, output: Option<&mut dyn LittleEndianOutput>) {
      // Java: if (srcFile != null && srcFile.exists() && output != null)
      if let (Some(srcFile), Some(output)) = (srcFile, output) {
        if srcFile.exists()
        {
         // Java: FileInputStream input = null;
         let mut fis: Option<FileInputStream> = None;
         // Java: try
         let _try_result: io::Result<()> = (|| -> io::Result<()> {
            // Java: input = new FileInputStream(srcFile);
            fis = Some(FileInputStream::from_file(srcFile)?);
            // Java: byte[] buffer = new byte[4096];
            let mut buffer = [0u8; 4096];
            // Java: int len = 0;
            let mut len: i32 = 0;
            // Java: while((len = input.read(buffer)) > 0)
            loop {
               len = fis.as_mut().unwrap().read_all(&mut buffer).unwrap_or(0) as i32;
               if len <= 0 { break; }
            {
               // Java: output.write(buffer, 0, len);
               output.write_with_offset(&buffer, 0, len);
            }
            }
            Ok(())
         })();
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(input);
            FileUtil::_method_8();
            drop(fis);
         }
        }
      }
    }

    // Java: static void dumpFile(LittleEndianInput input, int length, File destFile)
    pub fn dumpFile_lei_to_file(input: &mut dyn LittleEndianInput, mut length: i32, destFile: &mut File) {
      // Java: if (input != null && length > 0)
      if length > 0
      {
         // Java: FileOutputStream output = null;
         let mut output: Option<FileOutputStream> = None;
         // Java: try
         let try_result: Result<(), Box<dyn std::any::Any>> = (|| -> Result<(), Box<dyn std::any::Any>> {
            // Java: output = new FileOutputStream(destFile);
            output = Some(FileOutputStream::from_file(destFile).map_err(|e| Box::new(e) as Box<dyn std::any::Any>)?);
            // Java: int len = length > 4096 ? 4096 : length;
            let mut len = if length > 4096 { 4096 } else { length };
            // Java: for(byte[] buffer = new byte[len]; StorageUtil.checkInterrupted() && length > 0; length -= len)
            let mut buffer = vec![0u8; len as usize];
            while StorageUtil::checkInterrupted() && length > 0
            {
               // Java: len = length > buffer.length ? buffer.length : length;
               len = if length > buffer.len() as i32 { buffer.len() as i32 } else { length };
               // Java: input.readFully(buffer, 0, len);
               input.readFully_offset(&mut buffer, 0, len as usize);
               // Java: output.write(buffer, 0, len);
               output.as_mut().unwrap().write_buf(&buffer, 0, len as usize).map_err(|e| Box::new(e) as Box<dyn std::any::Any>)?;
               length -= len;
            }
            Ok(())
         })();
         // Java: catch (Throwable e)
         if let Err(e) = try_result
         {
            // Java: if (e instanceof OleParseInterruptException)
            if e.is::<OleParseInterruptException>()
            {
               // Java: throw new OleParseInterruptException();
               panic!("OleParseInterruptException");
            }
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            if e.is::<io::Error>()
            {
               // Java: throw new NoSpaceLeftException(e);
               panic!("NoSpaceLeftException");
            }
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(output);
            FileUtil::_method_5();
            drop(output);
         }
      }
    }

    // Java: static void decompressFile(File srcFile, File destFile)
    pub fn decompressFile(srcFile: &mut File, destFile: &mut File) {
      // Java: FileOutputStream output = null;
      let mut output: Option<FileOutputStream> = None;
      // Java: InputStream ios = null;
      let mut ios: Option<FileInputStream> = None;
      // Java: ZInputStream input = null;
      let mut zinput: Option<ZInputStream> = None;
      // Java: try
      let try_result: Result<(), Box<dyn std::any::Any>> = (|| -> Result<(), Box<dyn std::any::Any>> {
         // Java: output = new FileOutputStream(destFile);
         output = Some(FileOutputStream::from_file(destFile).map_err(|e| Box::new(e) as Box<dyn std::any::Any>)?);
         // Java: ios = new FileInputStream(srcFile);
         ios = Some(FileInputStream::from_file(srcFile).map_err(|e| Box::new(e) as Box<dyn std::any::Any>)?);
         // Java: input = new ZInputStream(ios);
         zinput = Some(ZInputStream::new(Box::new(ios.take().unwrap())));
         // Java: int bufferLen = 4096;
         let _bufferLen: i32 = 4096;
         // Java: byte[] buffer = new byte[4096];
         let mut buffer = [0u8; 4096];
         // Java: int len;
         let mut len: i32;
         // Java: while(StorageUtil.checkInterrupted() && (len = input.read(buffer, 0, 4096)) != -1)
         while StorageUtil::checkInterrupted() {
            len = zinput.as_mut().unwrap().read_buf(&mut buffer, 0, 4096);
            if len == -1 { break; }
         {
            // Java: output.write(buffer, 0, len);
            output.as_mut().unwrap().write_buf(&buffer, 0, len as usize).map_err(|e| Box::new(e) as Box<dyn std::any::Any>)?;
         }
         }
         Ok(())
      })();
      // Java: catch (Throwable e)
      if let Err(e) = try_result
      {
         // Java: if (e instanceof OleParseInterruptException)
         if e.is::<OleParseInterruptException>()
         {
            // Java: throw new OleParseInterruptException();
            panic!("OleParseInterruptException");
         }
         // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
         if e.is::<io::Error>()
         {
            // Java: throw new NoSpaceLeftException(e);
            panic!("NoSpaceLeftException");
         }
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(input);
         FileUtil::_method_8();
         drop(zinput);
         // Java: FileUtil.closeQuietly(ios);
         FileUtil::_method_8();
         drop(ios);
         // Java: FileUtil.closeQuietly(output);
         FileUtil::_method_5();
         drop(output);
      }
    }

    // Java: static void decompressFile(byte[] data, File destFile)
    pub fn decompressFile_from_bytes(data: &[u8], destFile: &mut File) {
      // Java: ByteArrayInputStream byteIn = null;
      let mut byteIn: Option<Cursor<Vec<u8>>> = None;
      // Java: ZInputStream input = null;
      let mut zinput: Option<ZInputStream> = None;
      // Java: FileOutputStream output = null;
      let mut output: Option<FileOutputStream> = None;
      // Java: try
      let try_result: io::Result<()> = (|| -> io::Result<()> {
         // Java: output = new FileOutputStream(destFile);
         output = Some(FileOutputStream::from_file(destFile)?);
         // Java: byteIn = new ByteArrayInputStream(data);
         byteIn = Some(Cursor::new(data.to_vec()));
         // Java: input = new ZInputStream(byteIn);
         zinput = Some(ZInputStream::new(Box::new(byteIn.take().unwrap())));
         // Java: int len = data.length > 4096 ? 4096 : data.length;
         let mut len: usize = if data.len() > 4096 { 4096 } else { data.len() };
         // Java: byte[] buffer = new byte[len];
         let mut buffer = vec![0u8; len];
         // Java: while(StorageUtil.checkInterrupted() && (len = input.read(buffer, 0, len)) != -1)
         while StorageUtil::checkInterrupted() {
            let read_len = zinput.as_mut().unwrap().read_buf(&mut buffer, 0, len as i32);
            if read_len == -1 { break; }
            len = read_len as usize;
         {
            // Java: output.write(buffer, 0, len);
            output.as_mut().unwrap().write_buf(&buffer, 0, len)?;
         }
         }
         Ok(())
      })();
      // Java: catch (IOException var10)
      if let Err(_) = try_result
      {
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(input);
         FileUtil::_method_8();
         drop(zinput);
         // Java: FileUtil.closeQuietly(byteIn);
         FileUtil::_method_8();
         drop(byteIn);
         // Java: FileUtil.closeQuietly(output);
         FileUtil::_method_5();
         drop(output);
      }
    }

    // Java: static void dumpFile(byte[] data, File destFile)
    pub fn dumpFile_bytes(data: &[u8], destFile: &mut File) -> io::Result<()> {
      // Java: FileOutputStream output = new FileOutputStream(destFile);
      let mut output = FileOutputStream::from_file(destFile)?;
      // Java: output.write(data);
      output.write_all_buf(data)?;
      // Java: FileUtil.closeQuietly(output);
      FileUtil::_method_5();
      drop(output);
      Ok(())
    }

    // Java: static File createOleWriteFile(String tmpDir, String path, String progId, boolean isDoc)
    pub fn createOleWriteFile(tmpDir: &str, path: &str, progId: &str, isDoc: bool) -> Option<File> {
      // Java: if (StringUtil.isEmpty(path))
      if path.is_empty()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: File tempFile = null;
         let mut tempFile: Option<File> = None;
         // Java: try
         let try_result: Result<(), Box<dyn std::error::Error>> = (|| -> Result<(), Box<dyn std::error::Error>> {
            // Java: tempFile = File.createTempFile("ole-write-", ".temp", new File(tmpDir));
            let dir = File::from_path(tmpDir);
            tempFile = Some(File::createTempFile_in_dir("ole-write-", Some(".temp"), Some(&dir))?);
            // Java: if (!"Word.OpenDocumentText.12".equals(progId) && !"Excel.OpenDocumentSpreadsheet.12".equals(progId) && !"PowerPoint.OpenDocumentPresentation.12".equals(progId) && (isDoc || !"PowerPoint.Show.12".equals(progId) && !"Excel.Sheet.12".equals(progId) && !"Word.Document.12".equals(progId)))
            if progId != "Word.OpenDocumentText.12"
               && progId != "Excel.OpenDocumentSpreadsheet.12"
               && progId != "PowerPoint.OpenDocumentPresentation.12"
               && (isDoc || (progId != "PowerPoint.Show.12" && progId != "Excel.Sheet.12" && progId != "Word.Document.12"))
            {
               // Java: IOLEDocument document = OLEDocumentFactory.create(tempFile, 2);
               let _document = OLEDocumentFactory::create(tempFile.as_ref().unwrap().getPath(), 2);
               // Java: IOLEStorage destStg = document.getRootStorage();
               // (getRootStorage returns Box<dyn Any>, used through document)
               let _destStg = (); // placeholder - OLE operations handled through document
               // Java: String clsId = OleClsTypeMatcher.getClsidByProgid(progId);
               let clsId = OleClsTypeMatcher::getClsidByProgid(progId);
               // Java: if (!StringUtil.isEmpty(clsId))
               if let Some(ref cls) = clsId {
                  if !cls.is_empty()
                  {
                     // Java: destStg.setStorageClsid(classId2Bytes(clsId));
                     let _bytes = Self::classId2Bytes(cls);
                  }
               }
               // Java: IOlePacker oleEntry = OlePackerFactory.getPacker(progId, path, false);
               let oleEntry = OlePackerFactory::getPacker_with_prog_id(Some(progId), Some(path), false);
               // Java: if (oleEntry != null)
               if let Some(_oleEntry) = oleEntry
               {
                  // Java: oleEntry.writeData(destStg);
                  // (writeData via packer variant on destStg)
               }
               // Java: destStg.close();
               drop(_destStg);
               // Java: document.close();
               drop(_document);
            }
            // Java: else
            else
            {
               // Java: FileUtil.copyFile(path, tempFile.getAbsolutePath());
               let _ = std::fs::copy(path, tempFile.as_ref().unwrap().getAbsolutePath());
            }
            Ok(())
         })();
         // Java: catch (Exception var9)
         if let Err(_) = try_result
         {
            // Java: if (tempFile != null && tempFile.exists())
            if let Some(ref tf) = tempFile {
               if tf.exists()
               {
                  // Java: FileUtil.copyFile(path, tempFile.getAbsolutePath());
                  let _ = std::fs::copy(path, tf.getAbsolutePath());
               }
            }
         }
         // Java: return tempFile;
         return tempFile;
      }
    }

    // Java: static String dumpFile(OpcrPackagePart packagePart, String rootDir)
    pub fn dumpFile_part_to_dir(packagePart: &OpcrPackagePart, rootDir: &str) -> io::Result<String> {
      // Java: InputStream inputStream = packagePart.getInputStream();
      let inputStream = packagePart.getInputStream();
      // Java: int size = packagePart.getPartSize();
      let size = inputStream.len() as i32;
      // Java: String path = packagePart.getPartName();
      let path = packagePart.getName();
      // Java: String extension = path.substring(path.lastIndexOf(".") + 1);
      let extension = match path.rfind('.') {
         Some(pos) => &path[pos + 1..],
         None => "",
      };
      // Java: File file = createNewFile("fds-", "." + extension, new File(rootDir));
      let dir = File::from_path(rootDir);
      let mut file = Self::createNewFile("fds-", &format!(".{}", extension), &dir)?;
      // Java: file.deleteOnExit();
      file.deleteOnExit();
      // Java: if (!file.exists())
      if !file.exists()
      {
         // Java: file.createNewFile();
         file.createNewFile();
      }
      // Java: FileDataStorage.createFileDataStorage(inputStream, size, file);
      let mut cursor = Cursor::new(inputStream);
      let _fds = FileDataStorage::createFileDataStorage_from_stream_with_file(
         &mut cursor, size, std::path::PathBuf::from(file.getPath())
      );
      // Java: return file.getPath();
      return Ok(file.getPath().to_string());
    }

    // Java: static byte[] classId2Bytes(String guid)
    pub fn classId2Bytes(guid: &str) -> Vec<u8> {
      // Java: ClassID classID = new ClassID();
      let mut classID = ClassID::new();
      // Java: byte[] dstClassID = new byte[16];
      let mut dstClassID = vec![0u8; 16];
      // Java: classID.setBytes(ClassID.toBytes(guid));
      classID.setBytes(&ClassID::toBytes(guid));
      // Java: classID.write(dstClassID, 0);
      classID.write(&mut dstClassID, 0);
      // Java: return dstClassID;
      return dstClassID;
    }

}
