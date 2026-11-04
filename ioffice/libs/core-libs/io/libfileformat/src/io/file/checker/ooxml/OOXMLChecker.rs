// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.core.runtime.IClassLoaderManager
use util::cn::wps::core::runtime::IClassLoaderManager;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform;
// Java: import cn.wps.io.dom.Document
use io_wps_io::cn::wps::io::dom::Document;
// Java: import cn.wps.io.dom.Element
use io_wps_io::io::dom::Element;
// Java: import cn.wps.io.dom.io.check.OOXMLCheckerHelper
use io_wps_io::cn::wps::io::dom::io::check::OOXMLCheckerHelper;
// Java: import cn.wps.io.file.EncryptedType
use io_libfileformat::cn::wps::io::file::EncryptedType;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.io.file.checker.ole.OleFileChecker
use io_libfileformat::io::file::checker::ole::OleFileChecker;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipEntry;
// Java: import cn.wps.moffice.kfs.zip.ZipFile
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipFile;
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import cn.wps.moffice.util.ArtMode
use util::cn::wps::moffice::util::ArtMode;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.InputStream
// TODO_RUST_CODE use ???; // java.io.InputStream
// Java: import java.lang.reflect.InvocationTargetException
// TODO_RUST_CODE use ???; // java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method
// TODO_RUST_CODE use ???; // java.lang.reflect.Method
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentEntry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_ENCRYPTION_INFO
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_ENCRYPTION_INFO: &str = "";
// Java field: String DOCX_CONTENT_TYPE
pub(crate) const DOCX_CONTENT_TYPE: &str = "";
// Java field: String DOTX_CONTENT_TYPE
pub(crate) const DOTX_CONTENT_TYPE: &str = "";
// Java field: String DOCM_CONTENT_TYPE
pub(crate) const DOCM_CONTENT_TYPE: &str = "";
// Java field: String DOTM_CONTENT_TYPE
pub(crate) const DOTM_CONTENT_TYPE: &str = "";
// Java field: String XLSX_CONTENT_TYPE
pub(crate) const XLSX_CONTENT_TYPE: &str = "";
// Java field: String XLTX_CONTENT_TYPE
pub(crate) const XLTX_CONTENT_TYPE: &str = "";
// Java field: String XLSM_CONTENT_TYPE
pub(crate) const XLSM_CONTENT_TYPE: &str = "";
// Java field: String XLTM_CONTENT_TYPE
pub(crate) const XLTM_CONTENT_TYPE: &str = "";
// Java field: String XLAM_CONTENT_TYPE
pub(crate) const XLAM_CONTENT_TYPE: &str = "";
// Java field: String XLSB_CONTENT_TYPE
pub(crate) const XLSB_CONTENT_TYPE: &str = "";
// Java field: String PPTX_CONTENT_TYPE
pub(crate) const PPTX_CONTENT_TYPE: &str = "";
// Java field: String POTX_CONTENT_TYPE
pub(crate) const POTX_CONTENT_TYPE: &str = "";
// Java field: String PPSX_CONTENT_TYPE
pub(crate) const PPSX_CONTENT_TYPE: &str = "";
// Java field: String POTM_CONTENT_TYPE
pub(crate) const POTM_CONTENT_TYPE: &str = "";
// Java field: String UOT3_MIME_TYPE
pub(crate) const UOT3_MIME_TYPE: &str = "";
// Java field: String UOS3_MIME_TYPE
pub(crate) const UOS3_MIME_TYPE: &str = "";
// Java field: String UOP3_MIME_TYPE
pub(crate) const UOP3_MIME_TYPE: &str = "";
// Java field: String OOXMLCHECKERHELPER_CLASS
pub(crate) const OOXMLCHECKERHELPER_CLASS: &str = "";

// Java: public class OOXMLChecker
pub struct OOXMLChecker {
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
}

// Java: public class OOXMLChecker
impl OOXMLChecker {
    // Java: OOXMLChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn OOXMLChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFile = file;
      // TODO_RUST_CODE
      // Java: this.mRaf = raf;
      // TODO_RUST_CODE
      // Java: this.mPoifs = poifs;
      // TODO_RUST_CODE
    }

    // Java: boolean isOOXML()
    pub fn isOOXML(&self) -> bool {
      // Java: if (null != this.mIsOOXML)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsOOXML;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mIsOOXML = this.invokeIsOOXML(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: return this.mIsOOXML;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasContentType(String dstContentType)
    pub fn hasContentType(&self, String dstContentType) -> bool {
      // Java: ZipFile zipFile = null;
      // TODO_RUST_CODE
      // Java: InputStream inputStream = null;
      // TODO_RUST_CODE
      // Java: boolean var11;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: zipFile = this.mRaf != null ? new ZipFile(this.mRaf) : new ZipFile(this.mFile);
         // TODO_RUST_CODE
         // Java: ZipEntry entry = zipFile.getEntry("[Content_Types].xml");
         // TODO_RUST_CODE
         // Java: inputStream = zipFile.getInputStream(entry);
         // TODO_RUST_CODE
         // Java: if (null == inputStream)
            // TODO_RUST_CODE
         {
            // Java: var11 = false;
            // TODO_RUST_CODE
            // Java: return var11;
            // TODO_RUST_CODE
         }
         // Java: var11 = this.hasContentType(inputStream, dstContentType);
         // TODO_RUST_CODE
      }
      // Java: catch (Exception var9)
        // TODO_RUST_CODE
      {
         // Java: var11 = false;
         // TODO_RUST_CODE
         // Java: return var11;
         // TODO_RUST_CODE
      }
      // Java: finally
        // TODO_RUST_CODE
      {
         // Java: FileUtil.closeQuietly(inputStream);
         // TODO_RUST_CODE
         // Java: FileUtil.closeQuietly(zipFile);
         // TODO_RUST_CODE
      }
      // Java: return var11;
      // TODO_RUST_CODE
    }

    // Java: boolean hasContentType(InputStream inputStream, String dstContentType)
    pub fn hasContentType(&self, InputStream inputStream, String dstContentType) -> bool {
      // Java: if (!this.isOOXML())
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Document read = OOXMLCheckerHelper.read(inputStream);
         // TODO_RUST_CODE
         // Java: if (read == null)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: Element rootElement = read.getRootElement();
            // TODO_RUST_CODE
            // Java: boolean override = OOXMLCheckerHelper.hasContentType(rootElement, "Override", dstContentType);
            // TODO_RUST_CODE
            // Java: if (!override)
                // TODO_RUST_CODE
            {
               // Java: override = OOXMLCheckerHelper.hasContentType(rootElement, "Default", dstContentType);
               // TODO_RUST_CODE
            }
            // Java: if (override)
                // TODO_RUST_CODE
            {
               // Java: OOXMLCheckerHelper.recycleNodes4DocxReader(rootElement, dstContentType);
               // TODO_RUST_CODE
            }
            // Java: return override;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: boolean hasContentType(String nameContentType, ZipInputStream is)
    pub fn hasContentType(&self, String nameContentType, ZipInputStream is) -> bool {
      // Java: return !this.isOOXML() ? false : this.invokeHasContentTypeMethod(nameContentType, is);
      // TODO_RUST_CODE
    }

    // Java: boolean invokeHasContentTypeMethod(String nameContentType, ZipInputStream is)
    pub fn invokeHasContentTypeMethod(&self, String nameContentType, ZipInputStream is) -> bool {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: Class<?> c = this.getOOXMLCheckerHelper();
         // TODO_RUST_CODE
         // Java: if (null == c)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: Method m = c.getMethod("hasContentType", String.class, ZipInputStream.class);
         // TODO_RUST_CODE
         // Java: Object result = m.invoke(c, nameContentType, is);
         // TODO_RUST_CODE
         // Java: if (null != result)
            // TODO_RUST_CODE
         {
            // Java: return (Boolean)result;
            // TODO_RUST_CODE
         }
      }
      // Java: catch (SecurityException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "SecurityException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (NoSuchMethodException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "NoSuchMethodException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalArgumentException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "IllegalArgumentException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalAccessException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "IllegalAccessException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (InvocationTargetException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "InvocationTargetException", e);
         // TODO_RUST_CODE
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

    // Java: boolean invokeIsOOXML(File file, RandomAccessFile raf)
    pub fn invokeIsOOXML(&self, File file, RandomAccessFile raf) -> bool {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: Class<?> c = this.getOOXMLCheckerHelper();
         // TODO_RUST_CODE
         // Java: if (null == c)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: Method m = c.getMethod("isOOXML", File.class, RandomAccessFile.class);
         // TODO_RUST_CODE
         // Java: Object result = m.invoke(c, file, raf);
         // TODO_RUST_CODE
         // Java: if (null != result)
            // TODO_RUST_CODE
         {
            // Java: return (Boolean)result;
            // TODO_RUST_CODE
         }
      }
      // Java: catch (SecurityException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "SecurityException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (NoSuchMethodException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "NoSuchMethodException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalArgumentException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "IllegalArgumentException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalAccessException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "IllegalAccessException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (InvocationTargetException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "InvocationTargetException", e);
         // TODO_RUST_CODE
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

    // Java: private Class<?> getOOXMLCheckerHelper()
    pub fn getOOXMLCheckerHelper(&self) -> private Class<?> {
      // Java: ClassLoader cl = null;
      // TODO_RUST_CODE
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
        // TODO_RUST_CODE
      {
         // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: cl = this.getClass().getClassLoader();
         // TODO_RUST_CODE
      }
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.io.dom.io.check.OOXMLCheckerHelper");
         // TODO_RUST_CODE
         // Java: return clazz;
         // TODO_RUST_CODE
      }
      // Java: catch (ClassNotFoundException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "ClassNotFoundException", e);
         // TODO_RUST_CODE
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasFolder(String target)
    pub fn hasFolder(&self, String target) -> bool {
      // Java: Assert.assertNotNull("target should not be null", target);
      // TODO_RUST_CODE
      // Java: ZipInputStream is = null != this.mRaf ? Tools.createZipInputStream(this.mRaf) : Tools.createZipInputStream(this.mFile);
      // TODO_RUST_CODE
      // Java: if (null == is)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: String name;
         // TODO_RUST_CODE
         // Java: do
            // TODO_RUST_CODE
         {
            // Java: ZipEntry entry = Tools.getNextEntry(is);
            // TODO_RUST_CODE
            // Java: if (null == entry)
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: name = entry.getName();
            // TODO_RUST_CODE
            // Java: Tools.checkEntryNameLegal(name);
            // TODO_RUST_CODE
            // Java: Assert.assertNotNull("name should not be null", name);
            // TODO_RUST_CODE
         }
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: EncryptedType isEncrypted()
    pub fn isEncrypted(&self) -> EncryptedType {
      // Java: if (null != this.mEncryptedType)
        // TODO_RUST_CODE
      {
         // Java: return this.mEncryptedType;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: OleFileChecker oleFileChecker = new OleFileChecker(this.mFile, this.mRaf, this.mPoifs);
         // TODO_RUST_CODE
         // Java: if (!oleFileChecker.isOle())
            // TODO_RUST_CODE
         {
            // Java: return this.mEncryptedType = EncryptedType.None;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (null == this.mPoifs)
                // TODO_RUST_CODE
            {
               // Java: long length = 0L;
               // TODO_RUST_CODE
               // Java: try
                    // TODO_RUST_CODE
               {
                  // Java: length = null != this.mRaf ? this.mRaf.length() : this.mFile.length();
                  // TODO_RUST_CODE
                  // Java: if (length <= 524288L)
                        // TODO_RUST_CODE
                  {
                     // Java: if (null != this.mRaf)
                            // TODO_RUST_CODE
                     {
                        // Java: this.mPoifs = Tools.createPOIFSFileSystem(this.mRaf);
                        // TODO_RUST_CODE
                     }
                     // Java: else
                            // TODO_RUST_CODE
                     {
                        // Java: this.mPoifs = Tools.createPOIFSFileSystem(Tools.createFileInputStream(this.mFile));
                        // TODO_RUST_CODE
                     }
                  }
                  // Java: else if (null != this.mRaf)
                        // TODO_RUST_CODE
                  {
                     // Java: this.mPoifs = Tools.createPOIFSFileSystem(this.mRaf);
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: this.mPoifs = Tools.createPOIFSFileSystem(this.mFile);
                     // TODO_RUST_CODE
                  }
               }
               // Java: catch (IOException e)
                    // TODO_RUST_CODE
               {
                  // Java: Log.e(TAG, "IOException", e);
                  // TODO_RUST_CODE
               }
            }
            // Java: if (null == this.mPoifs)
                // TODO_RUST_CODE
            {
               // Java: return this.mEncryptedType = EncryptedType.None;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: DirectoryNode directory = this.mPoifs.getRoot();
               // TODO_RUST_CODE
               // Java: if (directory == null)
                    // TODO_RUST_CODE
               {
                  // Java: return this.mEncryptedType = EncryptedType.None;
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: DocumentEntry documentProps = Tools.createDocumentEntry("EncryptionInfo", directory);
                  // TODO_RUST_CODE
                  // Java: return null == documentProps ? (this.mEncryptedType = EncryptedType.None) : (this.mEncryptedType = EncryptedType.OOXML);
                  // TODO_RUST_CODE
               }
            }
         }
      }
    }

    // Java: boolean hasUof(String nameContentType, ZipInputStream is)
    pub fn hasUof(&self, String nameContentType, ZipInputStream is) -> bool {
      // Java: return !this.isOOXML() ? false : this.invokeHasUofMethod(nameContentType, is);
      // TODO_RUST_CODE
    }

    // Java: boolean invokeHasUofMethod(String nameContentType, ZipInputStream is)
    pub fn invokeHasUofMethod(&self, String nameContentType, ZipInputStream is) -> bool {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: Class<?> c = this.getOOXMLCheckerHelper();
         // TODO_RUST_CODE
         // Java: if (null == c)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: Method m = c.getMethod("hasUof", String.class, ZipInputStream.class);
         // TODO_RUST_CODE
         // Java: Object result = m.invoke(c, nameContentType, is);
         // TODO_RUST_CODE
         // Java: if (null != result)
            // TODO_RUST_CODE
         {
            // Java: return (Boolean)result;
            // TODO_RUST_CODE
         }
      }
      // Java: catch (SecurityException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "SecurityException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (NoSuchMethodException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "NoSuchMethodException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalArgumentException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "IllegalArgumentException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalAccessException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "IllegalAccessException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (InvocationTargetException e)
        // TODO_RUST_CODE
      {
         // Java: KSLog.e(TAG, "InvocationTargetException", e);
         // TODO_RUST_CODE
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

}
