// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.Log
// use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.core.runtime.IClassLoaderManager
// use util::cn::wps::core::runtime::IClassLoaderManager;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform;
// Java: import cn.wps.io.dom.Document
use io_wps_io::cn::wps::io::dom::Document;
// Java: import cn.wps.io.dom.Element
use io_wps_io::io::dom::Element;
// Java: import cn.wps.io.dom.io.check.OOXMLCheckerHelper
use io_wps_io::cn::wps::io::dom::io::check::OOXMLCheckerHelper;
// Java: import cn.wps.io.file.EncryptedType
// use io_libfileformat::cn::wps::io::file::EncryptedType;
// Java: import cn.wps.io.file.Tools
// use io_libfileformat::cn::wps::io::file::Tools;
// Java: import cn.wps.io.file.checker.ole.OleFileChecker
// use io_libfileformat::io::file::checker::ole::OleFileChecker;
// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipEntry;
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipFile;
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import cn.wps.moffice.util.ArtMode
// use util::cn::wps::moffice::util::ArtMode;
// Java: import cn.wps.moffice.util.FileUtil
// use util::cn::wps::moffice::util::FileUtil;
// Java: import cn.wps.moffice.util.KSLog
// use util::cn::wps::moffice::util::KSLog;
// Java: import java.io.IOException
// LLM No Rust equivalent: java.io.IOException mapped to Rust equivalent
// Java: import java.io.InputStream
// LLM No Rust equivalent: java.io.InputStream mapped to Rust equivalent
// Java: import java.lang.reflect.InvocationTargetException
// LLM No Rust equivalent: java.lang.reflect.InvocationTargetException mapped to Rust equivalent
// Java: import java.lang.reflect.Method
// LLM No Rust equivalent: java.lang.reflect.Method mapped to Rust equivalent
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentEntry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

pub(crate) const TAG: &str = "";
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_ENCRYPTION_INFO: &str = "EncryptionInfo";
pub(crate) const DOCX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
pub(crate) const DOTX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml";
pub(crate) const DOCM_CONTENT_TYPE: &str = "application/vnd.ms-word.document.macroEnabled.main+xml";
pub(crate) const DOTM_CONTENT_TYPE: &str = "application/vnd.ms-word.template.macroEnabledTemplate.main+xml";
pub(crate) const XLSX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
pub(crate) const XLTX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml";
pub(crate) const XLSM_CONTENT_TYPE: &str = "application/vnd.ms-excel.sheet.macroEnabled.main+xml";
pub(crate) const XLTM_CONTENT_TYPE: &str = "application/vnd.ms-excel.template.macroEnabled.main+xml";
pub(crate) const XLAM_CONTENT_TYPE: &str = "application/vnd.ms-excel.addin.macroEnabled.main+xml";
pub(crate) const XLSB_CONTENT_TYPE: &str = "application/vnd.ms-excel.sheet.binary.macroEnabled.main";
pub(crate) const PPTX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
pub(crate) const POTX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml";
pub(crate) const PPSX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml";
pub(crate) const POTM_CONTENT_TYPE: &str = "application/vnd.ms-powerpoint.template.macroEnabled.main+xml";
pub(crate) const UOT3_MIME_TYPE: &str = "vnd.uof.text";
pub(crate) const UOS3_MIME_TYPE: &str = "vnd.uof.spreadsheet";
pub(crate) const UOP3_MIME_TYPE: &str = "vnd.uof.presentation";
pub(crate) const OOXMLCHECKERHELPER_CLASS: &str = "cn.wps.io.dom.io.check.OOXMLCheckerHelper";

// Java: public class OOXMLChecker
pub struct OOXMLChecker {
    // Java field: String TAG
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String OLE_DIRECTORY_ENTRY_ELENAME_ENCRYPTION_INFO
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String DOCX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String DOTX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String DOCM_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String DOTM_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String XLSX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String XLTX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String XLSM_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String XLTM_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String XLAM_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String XLSB_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String PPTX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String POTX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String PPSX_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String POTM_CONTENT_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String UOT3_MIME_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String UOS3_MIME_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: String UOP3_MIME_TYPE
    // LLM No Rust equivalent: static final constant defined at module level
    // Java field: File mFile
    pub(crate) mFile: Arc<Mutex<File>>,
    // Java field: RandomAccessFile mRaf
    pub(crate) mRaf: Arc<Mutex<RandomAccessFile>>,
    // Java field: Boolean mIsOOXML
    mIsOOXML: Option<Boolean>,
    // Java field: EncryptedType mEncryptedType
    pub(crate) mEncryptedType: Arc<Mutex<EncryptedType>>,
    // Java field: POIFSFileSystem mPoifs
    mPoifs: Option<POIFSFileSystem>,
    // Java field: String OOXMLCHECKERHELPER_CLASS
    // LLM No Rust equivalent: static final constant defined at module level
}

// Java: public class OOXMLChecker
impl OOXMLChecker {
    // Java: OOXMLChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn OOXMLChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super();
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: this.mFile = file;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: this.mRaf = raf;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: this.mPoifs = poifs;
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: boolean isOOXML()
    pub fn isOOXML(&self) -> bool {
      // Java: if (null != this.mIsOOXML)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return this.mIsOOXML;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: this.mIsOOXML = this.invokeIsOOXML(this.mFile, this.mRaf);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: return this.mIsOOXML;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
    }

    // Java: boolean hasContentType(String dstContentType)
    pub fn hasContentType(&self, dstContentType: String) -> bool {
      // Java: ZipFile zipFile = null;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: InputStream inputStream = null;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: boolean var11;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: try
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: zipFile = this.mRaf != null ? new ZipFile(this.mRaf) : new ZipFile(this.mFile);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: ZipEntry entry = zipFile.getEntry("[Content_Types].xml");
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: inputStream = zipFile.getInputStream(entry);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null == inputStream)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: var11 = false;
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: return var11;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: var11 = this.hasContentType(inputStream, dstContentType);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (Exception var9)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: var11 = false;
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: return var11;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: finally
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: FileUtil.closeQuietly(inputStream);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: FileUtil.closeQuietly(zipFile);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: return var11;
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: boolean hasContentType(InputStream inputStream, String dstContentType)
    pub fn hasContentType(&self, inputStream: InputStream, dstContentType: String) -> bool {
      // Java: if (!this.isOOXML())
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return false;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: Document read = OOXMLCheckerHelper.read(inputStream);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (read == null)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return false;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: else
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: Element rootElement = read.getRootElement();
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: boolean override = OOXMLCheckerHelper.hasContentType(rootElement, "Override", dstContentType);
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: if (!override)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: override = OOXMLCheckerHelper.hasContentType(rootElement, "Default", dstContentType);
               // LLM No Rust equivalent: module disabled, type not accessible
            }
            // Java: if (override)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: OOXMLCheckerHelper.recycleNodes4DocxReader(rootElement, dstContentType);
               // LLM No Rust equivalent: module disabled, type not accessible
            }
            // Java: return override;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
      }
    }

    // Java: boolean hasContentType(String nameContentType, ZipInputStream is)
    pub fn hasContentType(&self, nameContentType: String, is: ZipInputStream) -> bool {
      // Java: return !this.isOOXML() ? false : this.invokeHasContentTypeMethod(nameContentType, is);
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: boolean invokeHasContentTypeMethod(String nameContentType, ZipInputStream is)
    pub fn invokeHasContentTypeMethod(&self, nameContentType: String, is: ZipInputStream) -> bool {
      // Java: try
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: Class<?> c = this.getOOXMLCheckerHelper();
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null == c)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return false;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: Method m = c.getMethod("hasContentType", String.class, ZipInputStream.class);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: Object result = m.invoke(c, nameContentType, is);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null != result)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return (Boolean)result;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
      }
      // Java: catch (SecurityException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "SecurityException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (NoSuchMethodException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "NoSuchMethodException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (IllegalArgumentException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "IllegalArgumentException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (IllegalAccessException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "IllegalAccessException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (InvocationTargetException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "InvocationTargetException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: return false;
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: boolean invokeIsOOXML(File file, RandomAccessFile raf)
    pub fn invokeIsOOXML(&self, file: File, raf: RandomAccessFile) -> bool {
      // Java: try
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: Class<?> c = this.getOOXMLCheckerHelper();
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null == c)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return false;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: Method m = c.getMethod("isOOXML", File.class, RandomAccessFile.class);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: Object result = m.invoke(c, file, raf);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null != result)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return (Boolean)result;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
      }
      // Java: catch (SecurityException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "SecurityException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (NoSuchMethodException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "NoSuchMethodException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (IllegalArgumentException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "IllegalArgumentException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (IllegalAccessException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "IllegalAccessException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (InvocationTargetException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "InvocationTargetException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: return false;
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: private Class<?> getOOXMLCheckerHelper()
    pub fn getOOXMLCheckerHelper(&self) -> private Class<?> {
      // Java: ClassLoader cl = null;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: cl = this.getClass().getClassLoader();
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: try
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.io.dom.io.check.OOXMLCheckerHelper");
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: return clazz;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (ClassNotFoundException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "ClassNotFoundException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: return null;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
    }

    // Java: boolean hasFolder(String target)
    pub fn hasFolder(&self, target: String) -> bool {
      // Java: Assert.assertNotNull("target should not be null", target);
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: ZipInputStream is = null != this.mRaf ? Tools.createZipInputStream(this.mRaf) : Tools.createZipInputStream(this.mFile);
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: if (null == is)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return false;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: String name;
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: do
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: ZipEntry entry = Tools.getNextEntry(is);
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: if (null == entry)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: return false;
               // LLM No Rust equivalent: module disabled, type not accessible
            }
            // Java: name = entry.getName();
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: Tools.checkEntryNameLegal(name);
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: Assert.assertNotNull("name should not be null", name);
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: return true;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
    }

    // Java: EncryptedType isEncrypted()
    pub fn isEncrypted(&self) -> EncryptedType {
      // Java: if (null != this.mEncryptedType)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return this.mEncryptedType;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: OleFileChecker oleFileChecker = new OleFileChecker(this.mFile, this.mRaf, this.mPoifs);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (!oleFileChecker.isOle())
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return this.mEncryptedType = EncryptedType.None;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: else
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: if (null == this.mPoifs)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: long length = 0L;
               // LLM No Rust equivalent: module disabled, type not accessible
               // Java: try
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: length = null != this.mRaf ? this.mRaf.length() : this.mFile.length();
                  // LLM No Rust equivalent: module disabled, type not accessible
                  // Java: if (length <= 524288L)
                        // LLM No Rust equivalent: module disabled, type not accessible
                  {
                     // Java: if (null != this.mRaf)
                            // LLM No Rust equivalent: module disabled, type not accessible
                     {
                        // Java: this.mPoifs = Tools.createPOIFSFileSystem(this.mRaf);
                        // LLM No Rust equivalent: module disabled, type not accessible
                     }
                     // Java: else
                            // LLM No Rust equivalent: module disabled, type not accessible
                     {
                        // Java: this.mPoifs = Tools.createPOIFSFileSystem(Tools.createFileInputStream(this.mFile));
                        // LLM No Rust equivalent: module disabled, type not accessible
                     }
                  }
                  // Java: else if (null != this.mRaf)
                        // LLM No Rust equivalent: module disabled, type not accessible
                  {
                     // Java: this.mPoifs = Tools.createPOIFSFileSystem(this.mRaf);
                     // LLM No Rust equivalent: module disabled, type not accessible
                  }
                  // Java: else
                        // LLM No Rust equivalent: module disabled, type not accessible
                  {
                     // Java: this.mPoifs = Tools.createPOIFSFileSystem(this.mFile);
                     // LLM No Rust equivalent: module disabled, type not accessible
                  }
               }
               // Java: catch (IOException e)
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: Log.e(TAG, "IOException", e);
                  // LLM No Rust equivalent: module disabled, type not accessible
               }
            }
            // Java: if (null == this.mPoifs)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: return this.mEncryptedType = EncryptedType.None;
               // LLM No Rust equivalent: module disabled, type not accessible
            }
            // Java: else
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: DirectoryNode directory = this.mPoifs.getRoot();
               // LLM No Rust equivalent: module disabled, type not accessible
               // Java: if (directory == null)
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: return this.mEncryptedType = EncryptedType.None;
                  // LLM No Rust equivalent: module disabled, type not accessible
               }
               // Java: else
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: DocumentEntry documentProps = Tools.createDocumentEntry("EncryptionInfo", directory);
                  // LLM No Rust equivalent: module disabled, type not accessible
                  // Java: return null == documentProps ? (this.mEncryptedType = EncryptedType.None) : (this.mEncryptedType = EncryptedType.OOXML);
                  // LLM No Rust equivalent: module disabled, type not accessible
               }
            }
         }
      }
    }

    // Java: boolean hasUof(String nameContentType, ZipInputStream is)
    pub fn hasUof(&self, nameContentType: String, is: ZipInputStream) -> bool {
      // Java: return !this.isOOXML() ? false : this.invokeHasUofMethod(nameContentType, is);
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: boolean invokeHasUofMethod(String nameContentType, ZipInputStream is)
    pub fn invokeHasUofMethod(&self, nameContentType: String, is: ZipInputStream) -> bool {
      // Java: try
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: Class<?> c = this.getOOXMLCheckerHelper();
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null == c)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return false;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: Method m = c.getMethod("hasUof", String.class, ZipInputStream.class);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: Object result = m.invoke(c, nameContentType, is);
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (null != result)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return (Boolean)result;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
      }
      // Java: catch (SecurityException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "SecurityException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (NoSuchMethodException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "NoSuchMethodException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (IllegalArgumentException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "IllegalArgumentException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (IllegalAccessException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "IllegalAccessException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (InvocationTargetException e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.e(TAG, "InvocationTargetException", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: return false;
      // LLM No Rust equivalent: module disabled, type not accessible
    }

}
