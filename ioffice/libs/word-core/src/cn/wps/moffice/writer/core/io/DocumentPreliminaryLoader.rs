// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.core.runtime.IDisposable
use util_adapter::cn::wps::core::runtime::IDisposable::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.crypt.provider.ProviderInfo
use io_poi_compact::cn::wps::crypt::provider::ProviderInfo::*;
// Java: import cn.wps.io.file.EncryptedType
use core_libs_libfileformat::cn::wps::io::file::EncryptedType::*;
// Java: import cn.wps.io.file.FileFormatEnum
use util::io_file::FileFormatEnum::*;
// Java: import cn.wps.io.file.parser.FileParser
// CARGO_BLOCKER: libfileformat parser module not exported — soft holder
#[derive(Default)]
pub struct FileParser;
// Java: import cn.wps.io.xwpf.usermodel.XWPFDocument
use io_wps_io::io::xwpf::usermodel::XWPFDocument::*;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
use io_wps_io::io::xwpf::usermodel::exceptions::InvalidFormatException::*;
// Java: import cn.wps.moffice.crash.FileDamagedException
use util::moffice::crash::FileDamagedException::*;
// Java: import cn.wps.moffice.io.PasswordErrorException
use util_adapter::cn::wps::moffice::io::PasswordErrorException::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::FileInputStream::*;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile::*;
// Java: import cn.wps.moffice.online.security.OnlineSecurityTool
use io_poi_compact::cn::wps::moffice::online::security::OnlineSecurityTool::*;
// Java: import cn.wps.moffice.online.security.exception.NoNetworkException
use util_adapter::cn::wps::moffice::online::security::exception::NoNetworkException::*;
// Java: import cn.wps.moffice.online.security.exception.OnlineSecurityException
use util_adapter::cn::wps::moffice::online::security::exception::OnlineSecurityException::*;
// Java: import cn.wps.moffice.online.security.exception.PermissionDeniedException
use util_adapter::cn::wps::moffice::online::security::exception::PermissionDeniedException::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.writer.VersionState
use crate::cn::wps::moffice::writer::VersionState::*;
// Java: import java.io.FileNotFoundException
use stubs::java::io::FileNotFoundException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.util.zip.ZipError
use stubs::java::util::zip::ZipError::*;
// Java: import org.apache.poi.hwpf.HWPFDocument
use io_poi_compact::org::apache::poi::hwpf::HWPFDocument::*;
// Java: import org.apache.poi.poifs.filesystem.DocumentNode
// CARGO_BLOCKER: DocumentNode not exported from poi-compact org tree
#[derive(Default, Clone)]
pub struct DocumentNode;
// Java: import org.apache.poi.poifs.filesystem.POIFSDocument
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSDocument::*;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem::*;
// Java: import org.apache.poi.txt.TXTDocument
use io_poi_compact::org::apache::poi::txt::TXTDocument::*;
// Java: import java.lang.Thread
use stubs::java::lang::Thread::*;
// Java: import cn.wps.moffice.writer.core.io.IDocumentCallback
use crate::cn::wps::moffice::writer::core::io::IDocumentCallback::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java private static final String TAG = null;
const TAG: &str = "";

// java public class DocumentPreliminaryLoader implements Runnable, IDisposable
#[derive(Default)]
pub struct DocumentPreliminaryLoader {
    // java int nRet = 2;
    pub(crate) nRet: i32,
    // java private Exception mException;
    mException: Option<String>,
    // java public String mPath;
    pub(crate) mPath: Option<String>,
    // java private RandomAccessFile mRaf;
    mRaf: Option<RandomAccessFile>,
    // java InputStream mStream;
    pub(crate) mStream: Option<Box<dyn std::io::Read>>,
    // java public String mReloadPath;
    pub(crate) mReloadPath: Option<String>,
    // java FileFormatEnum mTemplate = null;
    pub(crate) mTemplate: Option<FileFormatEnum>,
    // java ProviderInfo mProviderInfo = null;
    pub(crate) mProviderInfo: Option<ProviderInfo>,
    // java FileParser fileParser = null;
    pub(crate) fileParser: Option<FileParser>,
    // java FileFormatEnum fileFormat = null;
    pub(crate) fileFormat: Option<FileFormatEnum>,
    // java POIFSFileSystem poifsFileSystem = null;
    pub(crate) poifsFileSystem: Option<POIFSFileSystem>,
    // java EncryptedType encryptedType = null;
    pub(crate) encryptedType: Option<EncryptedType>,
    // java private Thread mPreloadThread;
    mPreloadThread: Option<Box<dyn std::any::Any + Send + Sync>>,
    // java private OnlineSecurityTool mOnlineSecurityTool;
    mOnlineSecurityTool: Option<OnlineSecurityTool>,
    // java private String mOnlineSecurityFilePath;
    mOnlineSecurityFilePath: Option<String>,
    // java private String mXml07Decrypted;
    mXml07Decrypted: Option<String>,
    // java private String mUot3Decrypted;
    mUot3Decrypted: Option<String>,
    // java private boolean mIsLegal = true;
    mIsLegal: bool,
    // java private boolean mSkipOnlineExp = false;
    mSkipOnlineExp: bool,
    // java private IDocumentCallback mDocumentCallback;
    mDocumentCallback: Option<Box<dyn IDocumentCallback>>,
    // java HWPFDocument mDiskDoc;
    pub(crate) mDiskDoc: Option<HWPFDocument>,
    // java XWPFDocument docx_diskDoc;
    pub(crate) docx_diskDoc: Option<XWPFDocument>,
    // java File docx_targetFile;
    pub(crate) docx_targetFile: Option<std::path::PathBuf>,
    // java RandomAccessFile mDocxTargetRaf;
    pub(crate) mDocxTargetRaf: Option<RandomAccessFile>,
    // java TXTDocument disktxt;
    pub(crate) disktxt: Option<TXTDocument>,
}

// java public class DocumentPreliminaryLoader implements Runnable, IDisposable
impl DocumentPreliminaryLoader {
    // java public  DocumentPreliminaryLoader(String path)
    pub fn new__string(path: Option<&str>) -> Self {
      // java throw new IllegalArgumentException("path can't both be null.");
      panic!("{}", IllegalArgumentException::new("path can't both be null."));
      // java this.mPath = path;
      this.mPath = path;
    }

    // java public  DocumentPreliminaryLoader(RandomAccessFile raf)
    pub fn new__randomaccessfile(raf: Option<&RandomAccessFile>) -> Self {
      // java throw new IllegalArgumentException("raf can't both be null.");
      panic!("{}", IllegalArgumentException::new("raf can't both be null."));
      // java this.mRaf = raf;
      this.mRaf = raf;
    }

    // java public  DocumentPreliminaryLoader(String mPath, InputStream stream, FileFormatEnum mTemplate, ProviderInfo mProviderInfo)
    pub fn new__string_inputstream_fileformatenum_providerinfo(mPath: &str, stream: &dyn std::io::Read, mTemplate: &FileFormatEnum, mProviderInfo: &ProviderInfo) -> Self {
      // java this.mPath = mPath;
      let mut this = Self { nRet: Default::default(), mException: Default::default(), mPath: mPath, mRaf: Default::default(), mStream: stream, mReloadPath: Default::default(), mTemplate: mTemplate, mProviderInfo: mProviderInfo, fileParser: Default::default(), fileFormat: Default::default(), poifsFileSystem: Default::default(), encryptedType: Default::default(), mPreloadThread: Default::default(), mOnlineSecurityTool: Default::default(), mOnlineSecurityFilePath: Default::default(), mXml07Decrypted: Default::default(), mUot3Decrypted: Default::default(), mIsLegal: Default::default(), mSkipOnlineExp: Default::default(), mDocumentCallback: Default::default(), mDiskDoc: Default::default(), docx_diskDoc: Default::default(), docx_targetFile: Default::default(), mDocxTargetRaf: Default::default(), disktxt: Default::default() };
      // java this.init(stream, mTemplate, mProviderInfo);
      this.init(&stream, &mTemplate, &mProviderInfo);
      this
    }

    // java public  DocumentPreliminaryLoader(RandomAccessFile raf, FileFormatEnum mTemplate, ProviderInfo mProviderInfo)
    pub fn new__randomaccessfile_fileformatenum_providerinfo(raf: &RandomAccessFile, mTemplate: &FileFormatEnum, mProviderInfo: &ProviderInfo) -> Self {
      // java this.mRaf = raf;
      let mut this = Self { nRet: Default::default(), mException: Default::default(), mPath: Default::default(), mRaf: raf, mStream: Default::default(), mReloadPath: Default::default(), mTemplate: mTemplate, mProviderInfo: mProviderInfo, fileParser: Default::default(), fileFormat: Default::default(), poifsFileSystem: Default::default(), encryptedType: Default::default(), mPreloadThread: Default::default(), mOnlineSecurityTool: Default::default(), mOnlineSecurityFilePath: Default::default(), mXml07Decrypted: Default::default(), mUot3Decrypted: Default::default(), mIsLegal: Default::default(), mSkipOnlineExp: Default::default(), mDocumentCallback: Default::default(), mDiskDoc: Default::default(), docx_diskDoc: Default::default(), docx_targetFile: Default::default(), mDocxTargetRaf: Default::default(), disktxt: Default::default() };
      // java this.init((InputStream)null, mTemplate, mProviderInfo);
      this.init(Default::default(), &mTemplate, &mProviderInfo);
      this
    }

    // java private void init(InputStream stream, FileFormatEnum mTemplate, ProviderInfo mProviderInfo)
    pub fn init(&mut self, stream: &dyn std::io::Read, mTemplate: &FileFormatEnum, mProviderInfo: &ProviderInfo) {
      // java this.mStream = stream;
      self.mStream = stream;
      // java this.mTemplate = mTemplate;
      self.mTemplate = mTemplate;
      // java this.mProviderInfo = mProviderInfo;
      self.mProviderInfo = mProviderInfo;
    }

    // java public void setDocumentCallback(IDocumentCallback documentCallback)
    pub fn setDocumentCallback(&mut self, documentCallback: &dyn IDocumentCallback) {
      // java this.mDocumentCallback = documentCallback;
      self.mDocumentCallback = documentCallback;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (this.poifsFileSystem != null)
      if let Some(poifsFileSystem) = self.poifsFileSystem.as_mut() {
        // java this.poifsFileSystem.dispose();
        poifsFileSystem.dispose();
        // java this.poifsFileSystem = null;
        poifsFileSystem = None;
      }
      // java if (this.mDiskDoc != null)
      if let Some(mDiskDoc) = self.mDiskDoc.as_mut() {
        // java this.mDiskDoc.dispose();
        mDiskDoc.dispose();
        // java this.mDiskDoc = null;
        mDiskDoc = None;
      }
      // java if (this.isEncryption())
      if self.isEncryption() {
        // java this.mOnlineSecurityTool = null;
        self.mOnlineSecurityTool = None;
      }
      // java if (VersionState.isProVersion() && this.isProEncryption())
      if VersionState::isProVersion() && self.isProEncryption() {
        // java this.mDocumentCallback.dispose();
        self.mDocumentCallback.dispose();
      }
      // java this.docx_diskDoc = null;
      self.docx_diskDoc = None;
    }

    // java public int getStatus()
    pub fn getStatus(&self) -> i32 {
      // java return this.nRet;
      return self.nRet;
    }

    // java public boolean isOnlineSecurityDoc()
    pub fn isOnlineSecurityDoc(&self) -> Option<bool> {
      // java return null != this.mOnlineSecurityTool;
      return self.mOnlineSecurityTool.is_some();
    }

    // java public int checkFormat()
    pub fn checkFormat(&mut self) -> i32 {
          // java String decryptPath = this.getPath();
          let mut decryptPath: String = self.getPath();
          // java if (null != decryptPath)
          if decryptPath.is_some() {
            // java this.fileParser = new FileParser(new File(decryptPath), this.poifsFileSystem);
            self.fileParser = FileParser::new(File::new(decryptPath), self.poifsFileSystem);
          }
          // java else if (null != this.mRaf)
          else if let Some(mRaf) = self.mRaf.as_mut() {
            // java this.fileParser = new FileParser(this.mRaf, this.poifsFileSystem);
            self.fileParser = FileParser::new(mRaf, self.poifsFileSystem);
          }
          // java NoNetworkException var3
          todo!();
            // java return this.nRet = 9;
            return { self.nRet = 9; self.nRet };
          // java PermissionDeniedException e
          todo!();
              // java ("MSG_PERMISSION_DENIED_NEED_LOGIN".equals(e.getMessage()))
              "MSG_PERMISSION_DENIED_NEED_LOGIN".equals(e.getMessage());
                // java return this.nRet = 11;
                return { self.nRet = 11; self.nRet };
            // java return this.nRet = 8;
            return { self.nRet = 8; self.nRet };
          // java OnlineSecurityException e
          todo!();
            // java this.mException = e;
            self.mException = e;
            // java return this.nRet = 10;
            return { self.nRet = 10; self.nRet };
          // java IOException var6
          todo!();
            // java return this.nRet = 2;
            return { self.nRet = 2; self.nRet };
      // java this.encryptedType = this.fileParser.getEncryptedType();
      self.encryptedType = self.fileParser.getEncryptedType();
      // java if (null == this.encryptedType || EncryptedType.None == this.encryptedType || this.mProviderInfo != null && this.mProviderInfo.strPassword != null && this.mProviderInfo.strPassword.length() != 0)
      if self.encryptedType.is_none() || EncryptedType::None == self.encryptedType || self.mProviderInfo.is_some() && self.mProviderInfo.strPassword.is_some() && (self.mProviderInfo.strPassword.len() as i32) != 0 {
            // java if (null != this.mProviderInfo)
            if let Some(mProviderInfo) = self.mProviderInfo.as_mut() {
              // java this.fileFormat = this.fileParser.parse(this.mProviderInfo.strPassword);
              self.fileFormat = self.fileParser.parse__string(Some(&mProviderInfo.strPassword));
            }
            // java else
            else {
              // java this.fileFormat = this.fileParser.parse((String)null);
              self.fileFormat = self.fileParser.parse__string(Some(&String::new()));
            }
            // java this.mIsLegal = this.fileParser.mIsLegal;
            self.mIsLegal = self.fileParser.mIsLegal;
            // java PasswordErrorException e
            todo!();
              // java IOLog.e(TAG, "PasswordErrorException", e);
              IOLog::e__string_string_throwable(&TAG, &"PasswordErrorException", &e);
              // java this.nRet = 7;
              self.nRet = 7;
              // java this.mException = e;
              self.mException = e;
              // java return this.nRet;
              return self.nRet;
        // java this.poifsFileSystem = this.fileParser.getPOIFSFileSystem();
        self.poifsFileSystem = self.fileParser.getPOIFSFileSystem();
        // java return this.nRet = 0;
        return { self.nRet = 0; self.nRet };
      }
      // java else
      else {
        // java this.nRet = 6;
        self.nRet = 6;
        // java return this.nRet;
        return self.nRet;
      }
    }

    // java public FileFormatEnum getFileFormat()
    pub fn getFileFormat(&self) -> FileFormatEnum {
      // java if (this.fileFormat == null)
      if self.fileFormat.is_none() {
        // java throw new IllegalStateException("File format has not been checked");
        panic!("{}", IllegalStateException::new("File format has not been checked"));
      }
      // java else
      else {
        // java return this.fileFormat;
        return self.fileFormat;
      }
    }

    // java public boolean isDoc()
    pub fn isDoc(&self) -> bool {
      // java return FileFormatEnum.DOC == this.fileFormat || FileFormatEnum.DOT == this.fileFormat || FileFormatEnum.WPS == this.fileFormat || FileFormatEnum.WPT == this.fileFormat;
      return FileFormatEnum::DOC == self.fileFormat || FileFormatEnum::DOT == self.fileFormat || FileFormatEnum::WPS == self.fileFormat || FileFormatEnum::WPT == self.fileFormat;
    }

    // java public boolean isDocx()
    pub fn isDocx(&self) -> bool {
      // java return FileFormatEnum.DOCX == this.fileFormat || FileFormatEnum.DOTX == this.fileFormat || FileFormatEnum.DOCM == this.fileFormat || FileFormatEnum.DOTM == this.fileFormat;
      return FileFormatEnum::DOCX == self.fileFormat || FileFormatEnum::DOTX == self.fileFormat || FileFormatEnum::DOCM == self.fileFormat || FileFormatEnum::DOTM == self.fileFormat;
    }

    // java public boolean isTxt()
    pub fn isTxt(&self) -> bool {
      // java return FileFormatEnum.TXT == this.fileFormat || FileFormatEnum.HTML == this.fileFormat || FileFormatEnum.HTM == this.fileFormat;
      return FileFormatEnum::TXT == self.fileFormat || FileFormatEnum::HTML == self.fileFormat || FileFormatEnum::HTM == self.fileFormat;
    }

    // java public boolean isXml()
    pub fn isXml(&self) -> bool {
      // java return FileFormatEnum.XML == this.fileFormat;
      return FileFormatEnum::XML == self.fileFormat;
    }

    // java public boolean isXml03()
    pub fn isXml03(&self) -> bool {
      // java return FileFormatEnum.WORD_XML03 == this.fileFormat;
      return FileFormatEnum::WORD_XML03 == self.fileFormat;
    }

    // java public boolean isRtf()
    pub fn isRtf(&self) -> bool {
      // java return FileFormatEnum.RTF == this.fileFormat;
      return FileFormatEnum::RTF == self.fileFormat;
    }

    // java public boolean isHtml()
    pub fn isHtml(&self) -> bool {
      // java return FileFormatEnum.HTML == this.fileFormat || FileFormatEnum.HTM == this.fileFormat;
      return FileFormatEnum::HTML == self.fileFormat || FileFormatEnum::HTM == self.fileFormat;
    }

    // java public boolean isMht()
    pub fn isMht(&self) -> bool {
      // java return FileFormatEnum.MHT == this.fileFormat;
      return FileFormatEnum::MHT == self.fileFormat;
    }

    // java public boolean isXml07()
    pub fn isXml07(&self) -> bool {
      // java return FileFormatEnum.WORD_XML07 == this.fileFormat;
      return FileFormatEnum::WORD_XML07 == self.fileFormat;
    }

    // java public boolean isEpub()
    pub fn isEpub(&self) -> bool {
      // java return FileFormatEnum.EPUB == this.fileFormat;
      return FileFormatEnum::EPUB == self.fileFormat;
    }

    // java public boolean isUot3()
    pub fn isUot3(&self) -> bool {
      // java return FileFormatEnum.UOT3 == this.fileFormat;
      return FileFormatEnum::UOT3 == self.fileFormat;
    }

    // java public POIFSFileSystem getPOIFS()
    pub fn getPOIFS(&self) -> POIFSFileSystem {
      // java return this.poifsFileSystem;
      return self.poifsFileSystem;
    }

    // java public EncryptedType getEncryptedType()
    pub fn getEncryptedType(&self) -> EncryptedType {
      // java return this.encryptedType;
      return self.encryptedType;
    }

    // java public String getXml07Decrypted()
    pub fn getXml07Decrypted(&self) -> String {
      // java return this.mXml07Decrypted;
      return self.mXml07Decrypted;
    }

    // java public String getUot3Decrypted()
    pub fn getUot3Decrypted(&self) -> String {
      // java return this.mUot3Decrypted;
      return self.mUot3Decrypted;
    }

    // java public ProviderInfo getProviderInfo()
    pub fn getProviderInfo(&self) -> ProviderInfo {
      // java return this.mProviderInfo;
      return self.mProviderInfo;
    }

    // java public FileParser getFileParser()
    pub fn getFileParser(&self) -> FileParser {
      // java return this.fileParser;
      return self.fileParser;
    }

    // java public RandomAccessFile getRaf()
    pub fn getRaf(&self) -> RandomAccessFile {
      // java return this.mRaf;
      return self.mRaf;
    }

    // java private void load()
    pub fn load(&mut self) {
      // java if (this.isDoc())
      if self.isDoc() {
        // java if (this.mTemplate != null)
        if let Some(mTemplate) = self.mTemplate.as_mut() {
          // java this.mDiskDoc = new HWPFDocument(this.mStream);
          self.mDiskDoc = HWPFDocument::new(self.mStream);
        }
        // java else
        else {
          // java this.mDiskDoc = new HWPFDocument(this.poifsFileSystem);
          self.mDiskDoc = HWPFDocument::new(self.poifsFileSystem);
        }
        // java if (this.encryptedType != null && this.encryptedType != EncryptedType.None)
        if let Some(encryptedType) = self.encryptedType.as_mut() {
          if self.encryptedType != EncryptedType::None {
            // java if (this.mDiskDoc.initEncryptKey(this.mProviderInfo.strPassword))
            if self.mDiskDoc.initEncryptKey(&self.mProviderInfo.strPassword) {
              // java this.nRet = 0;
              self.nRet = 0;
            }
            // java else
            else {
              // java this.nRet = 7;
              self.nRet = 7;
            }
            // java this.mProviderInfo = this.mDiskDoc.getProviderInfo();
            self.mProviderInfo = self.mDiskDoc.getProviderInfo();
          }
        }
        // java if (this.nRet != 0)
        if self.nRet != 0 {
          // java return;
          return;
        }
        // java if (!this.mDiskDoc.ready(1))
        if !self.mDiskDoc.ready(1) {
          // java this.nRet = 2;
          self.nRet = 2;
        }
      }
      // java else if (this.isDocx())
      else if self.isDocx() {
        // java this.loadXWPFDocument(this.getFileParser(), this.getEncryptedType());
        self.loadXWPFDocument(&mut self.getFileParser(), Some(&self.getEncryptedType()));
      }
      // java else if (this.isTxt())
      else if self.isTxt() {
        // java File txtFile = this.fileParser.getFile();
        let mut txtFile: std::path::PathBuf = self.fileParser.getFile();
        // java this.disktxt = new TXTDocument(null != txtFile ? txtFile.getAbsolutePath() : null, this.fileParser.getRaf(), "UTF-8");
        self.disktxt = TXTDocument::new((if txtFile.is_some() { txtFile.getAbsolutePath() } else { None }), self.fileParser.getRaf(), "UTF-8");
      }
      // java else if (!this.isRtf() && !this.isMht())
      else if !self.isRtf() && !self.isMht() {
        // java if (this.isXml07())
        if self.isXml07() {
          // java if (null != this.encryptedType && EncryptedType.None != this.encryptedType)
          if let Some(encryptedType) = self.encryptedType.as_mut() {
            if EncryptedType::None != self.encryptedType {
              // java File ooxmlDecrypted = this.fileParser.get_OOXMLDecrypted();
              let mut ooxmlDecrypted: std::path::PathBuf = self.fileParser.get_OOXMLDecrypted();
              // java if (null == ooxmlDecrypted)
              if ooxmlDecrypted.is_none() {
                // java throw new IOException("Failed to decrypt file");
                panic!("{}", IOException::new("Failed to decrypt file"));
              }
              // java this.mXml07Decrypted = ooxmlDecrypted.getAbsolutePath();
              self.mXml07Decrypted = ooxmlDecrypted.getAbsolutePath();
            }
          }
        }
        // java else if (!this.isXml03() && !this.isEpub())
        else if !self.isXml03() && !self.isEpub() {
          // java if (this.isUot3())
          if self.isUot3() {
            // java if (null != this.encryptedType && EncryptedType.None != this.encryptedType)
            if let Some(encryptedType) = self.encryptedType.as_mut() {
              if EncryptedType::None != self.encryptedType {
                // java this.mUot3Decrypted = this.fileParser.get_UofDecrypted();
                self.mUot3Decrypted = self.fileParser.get_UofDecrypted();
              }
            }
          }
          // java else
          else {
            // java Assert.fail("it should not reach here");
            Assert::fail__string(&"it should not reach here");
            // java File txtFile = this.fileParser.getFile();
            let mut txtFile: std::path::PathBuf = self.fileParser.getFile();
            // java this.disktxt = new TXTDocument(null != txtFile ? txtFile.getAbsolutePath() : null, this.fileParser.getRaf(), "UTF-8");
            self.disktxt = TXTDocument::new((if txtFile.is_some() { txtFile.getAbsolutePath() } else { None }), self.fileParser.getRaf(), "UTF-8");
          }
        }
      }
    }

    // java private String getPath()
    pub fn getPath(&mut self) -> Option<String> {
      // java if (this.mTemplate != null)
      if let Some(mTemplate) = self.mTemplate.as_mut() {
        // java return this.mPath;
        return self.mPath;
      }
      // java else
      else {
        // java String onlineDecryptSrcPath;
        let mut onlineDecryptSrcPath: String = String::new();
        // java if (VersionState.isProVersion() && this.mDocumentCallback != null)
        if let Some(mDocumentCallback) = self.mDocumentCallback.as_mut() {
          if VersionState::isProVersion() {
            // java File tempFile = Platform.createTempFile("decr", (String)null);
            let mut tempFile: std::path::PathBuf = Platform::createTempFile__string_string(&"decr", &String::new());
            // java if (tempFile != null && tempFile.exists())
            if tempFile.is_some() && tempFile.exists() {
              // java String encryptedDestPath = tempFile.getAbsolutePath();
              let mut encryptedDestPath: String = tempFile.getAbsolutePath();
              // java onlineDecryptSrcPath = this.mDocumentCallback.decryptFile(this.mPath, encryptedDestPath);
              onlineDecryptSrcPath = mDocumentCallback.decryptFile(&self.mPath, &encryptedDestPath);
            }
            // java else
            else {
              // java onlineDecryptSrcPath = this.mPath;
              onlineDecryptSrcPath = self.mPath;
            }
          }
        }
        // java else
        else {
          // java onlineDecryptSrcPath = this.mPath;
          onlineDecryptSrcPath = self.mPath;
        }
        // java String onlineDecryptDestPath = this.onlineDecrypt(onlineDecryptSrcPath, this.mPath, this.mRaf);
        let mut onlineDecryptDestPath: String = self.onlineDecrypt(&onlineDecryptSrcPath, Some(&self.mPath), &self.mRaf);
        // java return onlineDecryptDestPath != null ? onlineDecryptDestPath : onlineDecryptSrcPath;
        return (if onlineDecryptDestPath.is_some() { Some(onlineDecryptDestPath) } else { Some(onlineDecryptSrcPath) });
      }
    }

    // java private String onlineDecrypt(String srcPath, String realPath, RandomAccessFile raf)
    pub fn onlineDecrypt(&mut self, srcPath: &str, realPath: Option<&str>, raf: &RandomAccessFile) -> Option<String> {
      // java String decryptedDestPath;
      let mut decryptedDestPath: String = String::new();
          // java File tempFile = Platform.createTempFile("w_o_decr", (String)null);
          let mut tempFile: std::path::PathBuf = Platform::createTempFile__string_string(&"w_o_decr", &String::new());
          // java if (tempFile != null && tempFile.exists())
          if tempFile.is_some() && tempFile.exists() {
            // java decryptedDestPath = tempFile.getAbsolutePath();
            decryptedDestPath = tempFile.getAbsolutePath();
            // java OnlineSecurityTool onlineSecurityTool = new OnlineSecurityTool(null != realPath ? (new File(realPath)).getName() : null);
            let mut onlineSecurityTool: OnlineSecurityTool = OnlineSecurityTool::new((if realPath.is_some() { Some(File::new(realPath).getName()) } else { None }));
            // java if (!onlineSecurityTool.decrypt(srcPath, decryptedDestPath, this.mSkipOnlineExp))
            if !onlineSecurityTool.decrypt(srcPath, decryptedDestPath, self.mSkipOnlineExp) {
              // java deleteEncryptedDestFile(decryptedDestPath);
              Self::deleteEncryptedDestFile(&decryptedDestPath);
              // java this.poifsFileSystem = onlineSecurityTool.getPOIFS();
              self.poifsFileSystem = onlineSecurityTool.getPOIFS();
              // java Object var12 = null;
              let mut var12: Box<dyn Object> = Box::new(()) as Box<dyn Object>;
              // java return (String)var12;
              return var12.to_string();
            }
            // java this.mOnlineSecurityTool = onlineSecurityTool;
            self.mOnlineSecurityTool = onlineSecurityTool;
            // java this.mOnlineSecurityFilePath = decryptedDestPath;
            self.mOnlineSecurityFilePath = decryptedDestPath;
            // java String var7 = decryptedDestPath;
            let mut var7: String = decryptedDestPath;
            // java return var7;
            return var7;
          }
          // java decryptedDestPath = null;
          decryptedDestPath = None;
            // java this.mSkipOnlineExp = false;
            self.mSkipOnlineExp = false;
      // java return decryptedDestPath;
      return decryptedDestPath;
    }

    // java private static void deleteEncryptedDestFile(String path)
    pub fn deleteEncryptedDestFile(path: &str) {
      // java FileUtil.delFile(path);
      FileUtil::delFile(Some(&path));
    }

    // java public HWPFDocument getHWPFDocument()
    pub fn getHWPFDocument(&self) -> HWPFDocument {
      // java return this.mDiskDoc;
      return self.mDiskDoc;
    }

    // java public XWPFDocument getXWPFDocument()
    pub fn getXWPFDocument(&self) -> XWPFDocument {
      // java return this.docx_diskDoc;
      return self.docx_diskDoc;
    }

    // java public File getXWPFTargetFile()
    pub fn getXWPFTargetFile(&self) -> std::path::PathBuf {
      // java return this.docx_targetFile;
      return self.docx_targetFile;
    }

    // java public RandomAccessFile getXWPFTargetRaf()
    pub fn getXWPFTargetRaf(&self) -> RandomAccessFile {
      // java return this.mDocxTargetRaf;
      return self.mDocxTargetRaf;
    }

    // java public String getSecurityFile()
    pub fn getSecurityFile(&self) -> String {
      // java return this.mOnlineSecurityFilePath;
      return self.mOnlineSecurityFilePath;
    }

    // java private void loadXWPFDocument(FileParser fileParser, EncryptedType encryptedType)
    pub fn loadXWPFDocument(&mut self, fileParser: &mut FileParser, encryptedType: Option<&EncryptedType>) {
      // java if (null != encryptedType && EncryptedType.None != encryptedType)
      if let Some(encryptedType) = encryptedType {
        if EncryptedType::None != encryptedType {
          // java Assert.assertNotNull("fileParser should not be null.", fileParser);
          Assert::assertNotNull__string_object(&"fileParser should not be null.", Some(&fileParser));
          // java File ooxmlDecrypted = fileParser.get_OOXMLDecrypted();
          let mut ooxmlDecrypted: std::path::PathBuf = fileParser.get_OOXMLDecrypted();
          // java if (null == ooxmlDecrypted)
          if ooxmlDecrypted.is_none() {
            // java throw new IOException("Failed to decrypt file");
            panic!("{}", IOException::new("Failed to decrypt file"));
          }
          // java Assert.assertNotNull("ooxmlDecrypted should not be null.", ooxmlDecrypted);
          Assert::assertNotNull__string_object(&"ooxmlDecrypted should not be null.", Some(&ooxmlDecrypted));
              // java this.buildXWPFDocument(ooxmlDecrypted, (RandomAccessFile)null);
              self.buildXWPFDocument(&ooxmlDecrypted, None);
                // java fileParser.dispose();
                fileParser.dispose();
        }
      }
      // java else
      else {
        // java this.buildXWPFDocument(fileParser.getFile(), fileParser.getRaf());
        self.buildXWPFDocument(&fileParser.getFile(), Some(&fileParser.getRaf()));
      }
    }

    // java private XWPFDocument buildXWPFDocument(File file, RandomAccessFile raf)
    pub fn buildXWPFDocument(&mut self, file: &std::path::PathBuf, raf: Option<&RandomAccessFile>) -> Option<XWPFDocument> {
      // java this.docx_targetFile = file;
      self.docx_targetFile = file;
      // java this.mDocxTargetRaf = raf;
      self.mDocxTargetRaf = raf;
          // java return null != raf ? (this.docx_diskDoc = new XWPFDocument(raf)) : (this.docx_diskDoc = new XWPFDocument(file, false));
          return (if raf.is_some() { Some({ self.docx_diskDoc = XWPFDocument::new(raf); self.docx_diskDoc }) } else { Some({ self.docx_diskDoc = XWPFDocument::new(file, false); self.docx_diskDoc }) });
          // java InvalidFormatException e
          todo!();
            // java throw new IOException("Invalid format", e);
            panic!("{}", IOException::new("Invalid format", e));
          // java ZipError var7
          todo!();
                // java return null != raf ? (this.docx_diskDoc = new XWPFDocument(raf)) : (this.docx_diskDoc = new XWPFDocument(new FileInputStream(file)));
                return (if raf.is_some() { Some({ self.docx_diskDoc = XWPFDocument::new(raf); self.docx_diskDoc }) } else { Some({ self.docx_diskDoc = XWPFDocument::new(FileInputStream::new(file)); self.docx_diskDoc }) });
              // java catch (InvalidFormatException e2) { throw new IOException("Invalid format", e2);
              panic!("{}", IOException::new("Invalid format", e2));
    }

    // java public TXTDocument getTXTDocument()
    pub fn getTXTDocument(&self) -> TXTDocument {
      // java return this.disktxt;
      return self.disktxt;
    }

    // java public void startPreload(boolean newThread)
    pub fn startPreload(&mut self, newThread: bool) {
      // java if (newThread)
      if newThread {
        // java this.mPreloadThread = new Thread(this, "preload");
        self.mPreloadThread = Thread::new(self, "preload");
        // java this.mPreloadThread.start();
        self.mPreloadThread.start();
      }
      // java else
      else {
        // java this.run();
        self.run();
      }
    }

    // java public void joinLoadFinish()
    pub fn joinLoadFinish(&mut self) {
      // java if (null != this.mPreloadThread)
      if let Some(mPreloadThread) = self.mPreloadThread.as_mut() {
            // java this.mPreloadThread.join();
            mPreloadThread.join();
            // java InterruptedException e
            todo!();
              // java KSLog.e(TAG, (String)null, e);
              KSLog::e__string_string_throwable(&TAG, &String::new(), &e);
      }
    }

    // java public void run()
    pub fn run(&mut self) {
      // java this.checkFormat();
      self.checkFormat();
      // java if (this.nRet == 0)
      if self.nRet == 0 {
            // java this.load();
            self.load();
            // java Throwable var2
            todo!();
              // java this.nRet = this.reload();
              self.nRet = self.reload();
        // java if (this.nRet == 2)
        if self.nRet == 2 {
          // java this.loadAsTxt();
          self.loadAsTxt();
        }
      }
    }

    // java private void loadAsTxt()
    pub fn loadAsTxt(&mut self) {
          // java File txtFile = this.getFileParser().getFile();
          let mut txtFile: std::path::PathBuf = self.getFileParser().getFile();
          // java this.disktxt = new TXTDocument(null != txtFile ? txtFile.getAbsolutePath() : null, this.getFileParser().getRaf(), "UTF-8");
          self.disktxt = TXTDocument::new((if txtFile.is_some() { txtFile.getAbsolutePath() } else { None }), self.getFileParser().getRaf(), "UTF-8");
          // java this.fileFormat = FileFormatEnum.TXT;
          self.fileFormat = FileFormatEnum::TXT;
          // java this.nRet = 0;
          self.nRet = 0;
          // java FileNotFoundException e
          todo!();
            // java IOLog.e(TAG, "FileNotFoundException", e);
            IOLog::e__string_string_throwable(&TAG, &"FileNotFoundException", &e);
            // java this.nRet = 2;
            self.nRet = 2;
            // java this.mException = new FileDamagedException(e);
            self.mException = FileDamagedException::new(e);
          // java IOException e
          todo!();
            // java IOLog.e(TAG, "IOException", e);
            IOLog::e__string_string_throwable(&TAG, &"IOException", &e);
            // java this.nRet = 2;
            self.nRet = 2;
            // java this.mException = new FileDamagedException(e);
            self.mException = FileDamagedException::new(e);
    }

    // java private int reload()
    pub fn reload(&mut self) -> i32 {
      // java int ret = 2;
      let mut ret: i32 = 2;
      // java if (FileFormatEnum.DOC != this.fileFormat)
      if FileFormatEnum::DOC != self.fileFormat {
        // java return ret;
        return ret;
      }
      // java else
      else {
        // java Entry entry = this.poifsFileSystem.getRoot().getEntry2("WordDocument");
        let mut entry: Box<dyn Entry> = self.poifsFileSystem.getRoot().getEntry2("WordDocument");
        // java if (null == entry)
        if entry.is_none() {
          // java return ret;
          return ret;
        }
        // java else
        else {
          // java DocumentNode node = (DocumentNode)entry;
          let mut node: DocumentNode = (entry).downcast_ref::<DocumentNode>().unwrap();
          // java POIFSDocument document = node.getDocument();
          let mut document: POIFSDocument = node.getDocument();
          // java this.mReloadPath = document.serialize();
          self.mReloadPath = document.serialize();
          // java if (null == this.mReloadPath)
          if self.mReloadPath.is_none() {
            // java return ret;
            return ret;
          }
          // java else
          else {
            // java this.fileFormat = FileFormatEnum.RTF;
            self.fileFormat = FileFormatEnum::RTF;
            // java ret = 0;
            ret = 0;
            // java return ret;
            return ret;
          }
        }
      }
    }

    // java public Exception getException()
    pub fn getException(&self) -> String {
      // java return this.mException;
      return self.mException;
    }

    // java private boolean isEncryption()
    pub fn isEncryption(&self) -> Option<bool> {
      // java return this.mOnlineSecurityTool != null;
      return self.mOnlineSecurityTool.is_some();
    }

    // java public OnlineSecurityTool getOnlineSecurityTool()
    pub fn getOnlineSecurityTool(&self) -> OnlineSecurityTool {
      // java return this.mOnlineSecurityTool;
      return self.mOnlineSecurityTool;
    }

    // java private boolean isProEncryption()
    pub fn isProEncryption(&self) -> Option<bool> {
      // java return this.mDocumentCallback != null && this.mDocumentCallback.isProEncryption();
      return self.mDocumentCallback.is_some() && self.mDocumentCallback.isProEncryption();
    }

    // java public boolean isLegal()
    pub fn isLegal(&self) -> bool {
      // java return this.mIsLegal;
      return self.mIsLegal;
    }

    // java public void setSkipOnlineExp(boolean skipOnlineExp)
    pub fn setSkipOnlineExp(&mut self, skipOnlineExp: bool) {
      // java this.mSkipOnlineExp = skipOnlineExp;
      self.mSkipOnlineExp = skipOnlineExp;
    }

}
