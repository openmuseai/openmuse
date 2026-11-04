// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.kfc.ooxfr.OxfrPackage
use super::ZipFile::{ZipEntry, ZipFile};
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use super::OpcrRelationships::OpcrRelationships;
use super::OpcrContentTypes::OpcrContentTypes;
// Java: import cn.wps.kfc.opcr.OpcrContentTypes
// rust: ContentTypes deferred; field stays Option until that leaf.
// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.util.KSLog
// Java: import java.io.EOFException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Enumeration
// Java: import java.util.zip.ZipException
// Java: import java.util.zip.ZipInputStream
// Java: import java.util.zip.ZipOutputStream
// Java: import cn.wps.kfc.opcr.OpcrRelationships
// rust: no-stmt
// Java: import cn.wps.kfc.opcr.OpcrContentTypes
// rust: ContentTypes deferred; field stays Option until that leaf.
// Java: import java.util.zip
// Java: import java.util

// java private static final String TAG = null;
const TAG: &str = "";
// java private static final int MAX_SIZE_ENTRY = 1073741824;
const MAX_SIZE_ENTRY: i64 = 1073741824;
// java private static final String INAVALID_NAME = "../";
const INAVALID_NAME: &str = "../";
// java public static final int TYPE_GVML = 0;
pub const TYPE_GVML: i32 = 0;
// java public static final int TYPE_OOXML = 1;
pub const TYPE_OOXML: i32 = 1;
// java public static final int TYPE_ARTTABLE = 2;
pub const TYPE_ARTTABLE: i32 = 2;

// java public abstract class OpcrPackage
pub struct OpcrPackage {
    // java protected ZipFile mZipFile = null;
    pub mZipFile: Option<ZipFile>,
    // java protected OpcrRelationships mRelses;
    pub mRelses: OpcrRelationships,
    // java private OpcrContentTypes mContentTypes = null;
    mContentTypes: Option<OpcrContentTypes>,
    // java private String mRepairFile = null;
    mRepairFile: Option<String>,
    // java private String mRepairLog = null;
    mRepairLog: Option<String>,
}

// java public abstract class OpcrPackage
impl OpcrPackage {
    // java public  OpcrPackage(String filePath)
    pub fn OpcrPackage(filePath: &str) -> Result<Self, std::io::Error> {
      // java super();
      // rust: no-stmt
      // java this.mZipFile = new ZipFile(filePath);
      let mZipFile = ZipFile::from_path(filePath)?;
      // java this.mRelses = new OpcrRelationships(this.mZipFile, "");
      let mRelses = OpcrRelationships::OpcrRelationships(&mZipFile, "")?;
      // java } catch (IOException e) {
      // java File tempDir = new File(Platform.getTempDirectory());
      // java File file = File.createTempFile("FileRepair.", ".ooxml.tmp", tempDir);
      // java this.mRepairFile = file.getAbsolutePath();
      // java zipFileRepair(filePath, this.mRepairFile);
      // java this.mRepairLog = e.toString();
      // java this.mZipFile = new ZipFile(this.mRepairFile);
      // java this.mRelses = new OpcrRelationships(this.mZipFile, "");
      // java } catch (ArrayIndexOutOfBoundsException e1) {
      // java throw new FileDamagedException(e1);
      // ★S0 轨等价物：Java 的两段 catch 恢复链在 Rust 侧以 `?` 直接上抛（登记 S0M01/S0M02）。
      Ok(Self {
        mZipFile: Some(mZipFile),
        mRelses,
        mContentTypes: None,
        mRepairFile: None,
        mRepairLog: None,
      })
    }

    // java public  OpcrPackage(RandomAccessFile raf)
    pub fn OpcrPackage__raf(raf: &mut dyn std::io::Read) -> Result<Self, std::io::Error> {
      // java super();
      // rust: no-stmt
      // java this.mZipFile = new ZipFile(raf);
      let mZipFile = ZipFile::from_reader(raf)?;
      // java this.mRelses = new OpcrRelationships(this.mZipFile, "");
      let mRelses = OpcrRelationships::OpcrRelationships(&mZipFile, "")?;
      // java } catch (IOException e) {
      // java this.mRepairLog = e.toString();
      // ★S0 轨等价物：Java 的 catch 吸收在 Rust 侧以 `?` 上抛（登记 S0M01/S0M02）。
      Ok(Self {
        mZipFile: Some(mZipFile),
        mRelses,
        mContentTypes: None,
        mRepairFile: None,
        mRepairLog: None,
      })
    }

    // java public abstract int type()
    pub fn type_(&self) -> i32 {
      panic!("OpcrPackage::type not implemented");
    }

    // java public String getDefaultContentType(String extension)
    pub fn getDefaultContentType(&mut self, extension: &str) -> Option<String> {
      // java if (this.mContentTypes == null)
      if self.mContentTypes.is_none() {
        // java this.mContentTypes = new OpcrContentTypes(this.mZipFile);
        // ★门面缺口（登记 S0M71）：Java 构造器吃 ZipFile 且可抛 IOException；Rust 侧 OpcrContentTypes::new() 现为无参、不返回 Result ⇒ 按 stub 现状调用
        self.mContentTypes = Some(OpcrContentTypes::new());
        // java } catch (IOException e) {
        // java e.printStackTrace();
        // java return null;
        // rust: no-stmt（Rust `new()` 非 Result ⇒ 失败分支不可达）
      }
      // java return this.mContentTypes.getDefaultContentType(extension);
      self.mContentTypes.as_ref().map(|ct| ct.getDefaultContentType(extension.to_string()))
    }

    // java public String getContentType(String entryName)
    pub fn getContentType(&mut self, entryName: &str) -> Option<String> {
      // java if (this.mContentTypes == null)
      if self.mContentTypes.is_none() {
        // java this.mContentTypes = new OpcrContentTypes(this.mZipFile);
        // ★门面缺口（登记 S0M71）：Java 构造器吃 ZipFile 且可抛 IOException；Rust 侧 OpcrContentTypes::new() 现为无参、不返回 Result ⇒ 按 stub 现状调用
        self.mContentTypes = Some(OpcrContentTypes::new());
        // java } catch (IOException e) {
        // java e.printStackTrace();
        // java return null;
        // rust: no-stmt（Rust `new()` 非 Result ⇒ 失败分支不可达）
      }
      // java return this.mContentTypes.getContentType(entryName);
      self.mContentTypes.as_ref().map(|ct| ct.getContentType(entryName.to_string()))
    }

    // java public static void zipFileRepair(String src, String dest)
    pub fn zipFileRepair(src: &str, dest: &str) -> Result<(), std::io::Error> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ZipInputStream input = new ZipInputStream(new FileInputStream(src));
      let mut input: ZipInputStream = ZipInputStream::new(FileInputStream::new(src));
      // java ZipOutputStream output = new ZipOutputStream(new FileOutputStream(dest));
      let mut output: ZipOutputStream = ZipOutputStream::new(FileOutputStream::new(dest));
      // java java.util.zip.ZipEntry srcEntry = null;
      let mut srcEntry: ZipEntry = Default::default();
      // java java.util.zip.ZipEntry destEntry = null;
      let mut destEntry: ZipEntry = Default::default();
      // java byte[] buffer = new byte[4096];
      let mut buffer: Vec<u8> = vec![Default::default(); 4096usize];
      // java int length = 0;
      let mut length: i32 = 0;
      // java boolean bException = false;
      let mut bException: bool = false;
      // java File tempDir = new File(Platform.getTempDirectory());
      let mut tempDir: std::path::PathBuf = { let __x: std::path::PathBuf = todo!("no align strategy matched"); __x };
      // java File tempFile = File.createTempFile("FileRepair.", ".pptx.tmp", tempDir);
      let mut tempFile: std::path::PathBuf = File::createTempFile__string_string_file("FileRepair.", ".pptx.tmp", tempDir);
          // java while((srcEntry = input.getNextEntry()) != null)
          while { srcEntry = input.getNextEntry(); srcEntry }.is_some() {
            // java FileOutputStream tempOutput = new FileOutputStream(tempFile);
            let mut tempOutput: Box<dyn std::any::Any + Send + Sync> = FileOutputStream::new(tempFile);
                // java while((length = input.read(buffer)) > 0)
                while { length = input.read(buffer); length } > 0 {
                  // java tempOutput.write(buffer, 0, length);
                  tempOutput.write(buffer, 0, length);
                }
                // java } catch (Exception e) {
                todo!();
                  // java bException = true;
                  bException = true;
                  // java e.printStackTrace();
                  e.printStackTrace();
                  // java continue;
                  continue;
                  // java FileUtil.closeQuietly(tempOutput);
                  FileUtil::closeQuietly__fileoutputstream(tempOutput);
            // java String entryName = srcEntry.getName();
            let mut entryName: String = srcEntry.getName();
            // java OxfrPackage.checkEntryNameLegal(entryName);
            OxfrPackage::checkEntryNameLegal(entryName);
            // java if (srcEntry.getSize() > 0L && srcEntry.getSize() > tempFile.length())
            if srcEntry.getSize() > 0 && srcEntry.getSize() > (tempFile.len() as i32) {
              // java throw new EOFException(entryName + " " + Long.toString(srcEntry.getSize()) + " " + Long.toString(destEntry.getSize()));
              panic!("{}", EOFException::new(entryName + " " + Long::toString(srcEntry.getSize()) + " " + Long::toString(destEntry.getSize())));
            }
            // java destEntry = new java.util.zip.ZipEntry(entryName);
            destEntry = ZipEntry::new(entryName);
            // java output.putNextEntry(destEntry);
            output.putNextEntry(destEntry);
            // java FileInputStream tempInput = new FileInputStream(tempFile);
            let mut tempInput: Box<dyn std::any::Any + Send + Sync> = FileInputStream::new(tempFile);
            // java while((length = tempInput.read(buffer)) > 0)
            while { length = tempInput.read(buffer); length } > 0 {
              // java output.write(buffer, 0, length);
              output.write(buffer, 0, length);
            }
                // java output.closeEntry();
                output.closeEntry();
                // java input.closeEntry();
                input.closeEntry();
                // java tempInput.close();
                tempInput.close();
                // java } catch (ZipException var26) {
                todo!();
                  // java IOLog.e(TAG, "closeEntry: failed");
                  IOLog::e__string_string(TAG, "closeEntry: failed");
          }
          // java } catch (Exception e) {
          todo!();
            // java bException = true;
            bException = true;
            // java e.printStackTrace();
            e.printStackTrace();
            // java throw new FileDamagedException(e);
            panic!("{}", FileDamagedException::new(e));
            // java output.flush();
            output.flush();
            // java FileUtil.closeQuietly(output);
            FileUtil::closeQuietly__fileoutputstream(output);
              // java if (!bException)
              !bException;
                // java input.close();
                input.close();
              // java (tempFile.exists())
              tempFile.exists();
                // java tempFile.delete();
                tempFile.delete();
      */
    }

    // java public void close()
    pub fn close(&mut self) -> Result<(), std::io::Error> {
      // java if (this.mZipFile != null)
      if self.mZipFile.is_some() {
        // java this.mZipFile.close();
        if let Some(z) = self.mZipFile.as_mut() {
            z.close();
        }
        // java this.mZipFile = null;
        self.mZipFile = None;
        // java if (this.mRepairFile != null)
        if let Some(repair) = self.mRepairFile.as_ref() {
          // java File f = new File(this.mRepairFile);
          let f = std::path::Path::new(repair);
          // java if (f.exists())
          if f.exists() {
            // java f.delete();
            let _ = std::fs::remove_file(f);
          }
        }
      }
      Ok(())
    }

    // java public OpcrRelationships getRelationships()
    pub fn getRelationships(&mut self) -> &OpcrRelationships {
      // java return this.mRelses;
      return &self.mRelses;
    }

    // java public ZipFile getZipFile()
    pub fn getZipFile(&self) -> Option<ZipFile> {
      // java return this.mZipFile;
      return self.mZipFile.clone();
    }

    // java public static InputStream getInputStream(ZipFile zipFile, String entryName, boolean isOriginInput)
    pub fn getInputStream__name_flag(zipFile: &ZipFile, entryName: &str, isOriginInput: bool) -> Result<Option<Vec<u8>>, std::io::Error> {
      // java ZipEntry entry = getEntry(zipFile, entryName);
      let entry = Self::getEntry(zipFile, Some(entryName));
      // java if (entry == null)
      if entry.is_none() {
        // java return null;
        return Ok(None);
      }
      // java else
      else {
        // java InputStream inputStream = null;
        let mut inputStream = None;
        // java try {
        // java inputStream = zipFile.getInputStream(entry, isOriginInput);
        inputStream = zipFile.getInputStream(&entry.unwrap(), isOriginInput);
        // java } catch (ZipException var6) {
        // java KSLog.e(TAG, "getInputStream by entryName: failed");
        // java return inputStream;
        return Ok(inputStream);
      }
    }

    // java public static InputStream getInputStream(ZipFile zipFile, String entryName)
    pub fn getInputStream__name(zipFile: &ZipFile, entryName: &str) -> Result<Option<Vec<u8>>, std::io::Error> {
      // java return getInputStream(zipFile, entryName, false);
      return Self::getInputStream__name_flag(zipFile, entryName, false);
    }

    // java public static InputStream getInputStream(ZipFile zipFile, ZipEntry entry)
    pub fn getInputStream__entry(zipFile: &ZipFile, entry: &ZipEntry) -> Result<Option<Vec<u8>>, std::io::Error> {
      // java return getInputStream(zipFile, entry, false);
      return Self::getInputStream__entry_flag(zipFile, entry, false);
    }

    // java public static InputStream getInputStream(ZipFile zipFile, ZipEntry entry, boolean isOriginInput)
    pub fn getInputStream__entry_flag(zipFile: &ZipFile, entry: &ZipEntry, isOriginInput: bool) -> Result<Option<Vec<u8>>, std::io::Error> {
          // java try {
          // java return zipFile.getInputStream(entry, isOriginInput);
          // java } catch (Throwable e) {
          // java IOLog.e(TAG, "Throwable", e);
          // java return null;
          Ok(zipFile.getInputStream(entry, isOriginInput))
    }

    // java public static ZipEntry getEntry(ZipFile zipFile, String entryName)
    pub fn getEntry(zipFile: &ZipFile, entryName: Option<&str>) -> Option<ZipEntry> {
      // java if (entryName == null)
      if entryName.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        let entryName = entryName.unwrap();
        // java ZipEntry entry = zipFile.getEntry(entryName);
        let mut entry = zipFile.getEntry(entryName);
        // java if (entry == null)
        if entry.is_none() {
          // java String newEntryName = entryName.replace('/', '\\');
          let newEntryName = entryName.replace('/', "\\");
          // java entry = zipFile.getEntry(newEntryName);
          entry = zipFile.getEntry(&newEntryName);
        }
        // java if (entry == null)
        if entry.is_none() {
          // java String newEntryName = entryName.toLowerCase();
          let newEntryName = entryName.to_lowercase();
          // java entry = zipFile.getEntry(newEntryName);
          entry = zipFile.getEntry(&newEntryName);
        }
        // java if (null != entry && entry.getSize() > 1073741824L)
        if let Some(ref e) = entry {
          if e.getSize() > MAX_SIZE_ENTRY {
            // java throw new FileDamagedException();
            panic!("FileDamagedException");
          }
        }
        // java else
        // java return entry;
        return entry;
      }
    }

    // java public static void checkEntryNameLegal(String entryName)
    pub fn checkEntryNameLegal(entryName: &str) {
      // java if (entryName.contains("../"))
      if entryName.contains("../") {
        // java throw new FileDamagedException();
        panic!("FileDamagedException");
      }
    }

    // java public String getRepairLog()
    pub fn getRepairLog(&self) -> Option<String> {
      // java return this.mRepairLog;
      return self.mRepairLog.clone();
    }

    // java public int getMaxMemory()
    pub fn getMaxMemory(&self) -> i32 {
      // java int ret = 0;
      let mut ret: i32 = 0;
      // java Enumeration<? extends ZipEntry> entrys = this.mZipFile.entries();
      let entrys: Vec<ZipEntry> = match self.mZipFile.as_ref() {
        Some(zf) => zf.entries(),
        None => Vec::new(),
      };
      // java while(entrys.hasMoreElements())
      for entry in entrys.iter() {
        // java ZipEntry entry = (ZipEntry)entrys.nextElement();
        // rust: no-stmt（Enumeration 游标由 `for` 承担）
        // java String entryName = entry.getName();
        let entryName: String = entry.getName();
        // java OxfrPackage.checkEntryNameLegal(entryName);
        OpcrPackage::checkEntryNameLegal(&entryName);
        // java if (entryName.endsWith(".xml"))
        if entryName.ends_with(".xml") {
          // java ret = (int)((long)ret + entry.getSize() * 2L);
          ret = ((ret as i64) + entry.getSize() * 2) as i32;
        }
        // java else if (entryName.contains("/media/image"))
        else if entryName.contains("/media/image") {
          // java long cs = entry.getCompressedSize();
          let cs: i64 = entry.getCompressedSize();
          // java if (cs < 2097152L)
          if cs < 2097152 {
            // java ret = (int)((long)ret + cs * 4L);
            ret = ((ret as i64) + cs * 4) as i32;
          }
          // java else
          else {
            // java ret = (int)((long)ret + cs * 2L);
            ret = ((ret as i64) + cs * 2) as i32;
          }
        }
      }
      // java return ret;
      ret
    }

}
