// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.crypt.provider.ProviderInfo
use io_poi_compact::cn::wps::crypt::provider::ProviderInfo::*;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
use io_wps_io::io::xwpf::usermodel::exceptions::InvalidFormatException::*;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use util::moffice::crash::NoSpaceLeftException::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.slim.ISlimToolManager
use util_adapter::cn::wps::moffice::slim::ISlimToolManager::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.value.FileFormat
use util::moffice::writer::data::value::FileFormat::*;
// Java: import cn.wps.moffice.writer.io.WriterAutoWriteException
use crate::cn::wps::moffice::writer::io::WriterAutoWriteException::*;
// Java: import cn.wps.moffice.writer.io.WriterIOException
use crate::cn::wps::moffice::writer::io::WriterIOException::*;
// Java: import cn.wps.moffice.writer.io.WriterIOStatus
use crate::cn::wps::moffice::writer::io::WriterIOStatus::*;
// Java: import cn.wps.moffice.writer.io.reader.face.IDocxWriter
use crate::cn::wps::moffice::writer::io::reader::face::IDocxWriter::*;
// Java: import cn.wps.moffice.writer.io.reader.face.IUot3Writer
use crate::cn::wps::moffice::writer::io::reader::face::IUot3Writer::*;
// Java: import cn.wps.moffice.writer.io.reader.face.IXml03Writer
use crate::cn::wps::moffice::writer::io::reader::face::IXml03Writer::*;
// Java: import cn.wps.moffice.writer.io.reader.face.IXml07Writer
use crate::cn::wps::moffice::writer::io::reader::face::IXml07Writer::*;
// Java: import cn.wps.moffice.writer.io.reflection.ReflectionHelper
use crate::cn::wps::moffice::writer::io::reflection::ReflectionHelper::*;
// Java: import cn.wps.moffice.writer.io.writer.doc.DocWriter
// CARGO_BLOCKER: DocWriter screened in doc/mod.rs (detached stub)
// use crate::cn::wps::moffice::writer::io::writer::doc::DocWriter::*;
// Java: import cn.wps.moffice.writer.io.writer.text.TextWriter
use crate::cn::wps::moffice::writer::io::writer::text::TextWriter::*;
// Java: import java.io.FileNotFoundException
use stubs::java::io::FileNotFoundException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.util.Locale
use stubs::java::util::Locale::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java static final String TAG = null;
pub(crate) const TAG: &str = "";
// java private static final String ONLINE_FAKE_PATH = "/storage/emulated/0/" + System.currentTimeMillis() + ".tmp";
const ONLINE_FAKE_PATH: &str = "/storage/emulated/0/" + System::currentTimeMillis() + ".tmp";

// java public class WordWriter
#[derive(Default)]
pub struct WordWriter {
    // java String mSavedFilePath = null;
    pub(crate) mSavedFilePath: Option<String>,
    // java TextDocument mDocument = null;
    pub(crate) mDocument: Option<TextDocument>,
    // java FileFormat mFf;
    pub(crate) mFf: Option<FileFormat>,
    // java ProviderInfo mProviderInfo;
    pub(crate) mProviderInfo: Option<ProviderInfo>,
}

// java public class WordWriter
impl WordWriter {
    // java public  WordWriter(TextDocument coreDocument, String filePath, FileFormat ff)
    pub fn new__textdocument_string_fileformat(coreDocument: &TextDocument, filePath: &str, ff: &FileFormat) -> Self {
      // java this.mFf = FileFormat.FF_TXT;
      let mut this = Self { mSavedFilePath: Default::default(), mDocument: Default::default(), mFf: FileFormat::FF_TXT, mProviderInfo: Default::default() };
      // java this.init(coreDocument, filePath, ff, (ProviderInfo)null);
      this.init(&coreDocument, &filePath, &ff, Default::default());
      this
    }

    // java public  WordWriter(TextDocument coreDocument, String filePath, FileFormat ff, ProviderInfo providerIndo)
    pub fn new__textdocument_string_fileformat_providerinfo(coreDocument: &TextDocument, filePath: &str, ff: &FileFormat, providerIndo: &ProviderInfo) -> Self {
      // java this.mFf = FileFormat.FF_TXT;
      let mut this = Self { mSavedFilePath: Default::default(), mDocument: Default::default(), mFf: FileFormat::FF_TXT, mProviderInfo: Default::default() };
      // java this.init(coreDocument, filePath, ff, providerIndo);
      this.init(&coreDocument, &filePath, &ff, &providerIndo);
      this
    }

    // java public  WordWriter(TextDocument coreDocument, FileFormat ff)
    pub fn new__textdocument_fileformat(coreDocument: &TextDocument, ff: &FileFormat) -> Self {
      // java this.mFf = FileFormat.FF_TXT;
      let mut this = Self { mSavedFilePath: Default::default(), mDocument: Default::default(), mFf: FileFormat::FF_TXT, mProviderInfo: Default::default() };
      // java this.init(coreDocument, ONLINE_FAKE_PATH, ff, (ProviderInfo)null);
      this.init(&coreDocument, &ONLINE_FAKE_PATH, &ff, Default::default());
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
    }

    // java private void init(TextDocument coreDocument, String filePath, FileFormat ff, ProviderInfo providerInfo)
    pub fn init(&mut self, coreDocument: &TextDocument, filePath: &str, ff: &FileFormat, providerInfo: &ProviderInfo) {
      // java IOAssert.assertNotNull("coreDocument should not be null!", coreDocument);
      IOAssert::assertNotNull__string_object(&"coreDocument should not be null!", Some(&coreDocument));
      // java IOAssert.assertNotNull("filePath should not be null!", filePath);
      IOAssert::assertNotNull__string_object(&"filePath should not be null!", Some(&filePath));
      // java IOAssert.assertNotNull("ff should not be null!", ff);
      IOAssert::assertNotNull__string_object(&"ff should not be null!", Some(&ff));
      // java this.mSavedFilePath = filePath;
      self.mSavedFilePath = filePath;
      // java this.mFf = ff;
      self.mFf = ff;
      // java this.mDocument = coreDocument;
      self.mDocument = coreDocument;
      // java this.mProviderInfo = providerInfo;
      self.mProviderInfo = providerInfo;
    }

    // java public final String getTargetPath()
    pub fn getTargetPath(&self) -> String {
      // java return this.mSavedFilePath;
      return self.mSavedFilePath;
    }

    // java public void save()
    pub fn save(&mut self) {
          // java switch (this.mFf)
          match self.mFf {
            // java case FF_DOC:
            FF_DOC => {
              // java this.exportDoc();
              self.exportDoc();
            }
            // java case FF_DOCX:
            FF_DOCX => {
              // java this.exportDocx();
              self.exportDocx();
            }
            // java case FF_DOTX:
            FF_DOTX => {
              // java this.exportDotx(false);
              self.exportDotx(false);
            }
            // java case FF_TXT:
            FF_TXT => {
              // java this.exportTxt();
              self.exportTxt();
            }
            // java case FF_RTF:
            FF_RTF => {
              // java this.exportDoc();
              self.exportDoc();
            }
            // java case FF_XML07:
            FF_XML07 => {
              // java this.exportXml07();
              self.exportXml07();
            }
            // java case FF_XML03:
            FF_XML03 => {
              // java this.exportXml03();
              self.exportXml03();
            }
            // java case FF_EPUB:
            FF_EPUB => {
              // java this.exportEpub();
              self.exportEpub();
            }
            // java case FF_UOT3:
            FF_UOT3 => {
              // java this.exportUot3();
              self.exportUot3();
            }
            // java default:
            _ => {
              // java IOAssert.fail("It should not reach here!");
              IOAssert::fail__string(&"It should not reach here!");
              // java this.exportTxt();
              self.exportTxt();
            }
          }
          // java ISlimToolManager.instance().clearCache();
          ISlimToolManager::instance().clearCache();
          // java IOException e
          todo!();
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java IOLog.e(TAG, "NOSpaceLeftException!!!");
                IOLog::e__string_string(&TAG, &"NOSpaceLeftException!!!");
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
                // java IOLog.e(TAG, "IOException", e);
                IOLog::e__string_string_throwable(&TAG, &"IOException", &e);
                // java throw new WriterIOException(new WriterIOStatus(4, e.getMessage(), e));
                panic!("{}", WriterIOException::new(WriterIOStatus::new(4, e.getMessage(), e)));
          // java RuntimeException e
          todo!();
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java IOLog.e(TAG, "NOSpaceLeftException!!!");
                IOLog::e__string_string(&TAG, &"NOSpaceLeftException!!!");
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
                // java IOLog.e(TAG, "RuntimeException", e);
                IOLog::e__string_string_throwable(&TAG, &"RuntimeException", &e);
                // java throw e;
                panic!("{}", e);
          // java WriterIOException e
          todo!();
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java IOLog.e(TAG, "NOSpaceLeftException!!!");
                IOLog::e__string_string(&TAG, &"NOSpaceLeftException!!!");
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
                // java IOLog.e(TAG, "WriterIOException", e);
                IOLog::e__string_string_throwable(&TAG, &"WriterIOException", &e);
                // java throw e;
                panic!("{}", e);
    }

    // java private void exportEpub()
    pub fn exportEpub(&mut self) {
    }

    // java public void autoSave()
    pub fn autoSave(&mut self) {
          // java switch (this.mFf)
          match self.mFf {
            // java case FF_DOC:
            FF_DOC => {
              // java this.autoExportDoc();
              self.autoExportDoc();
            }
            // java case FF_DOCX:
            FF_DOCX => {
              // java this.autoExportDocx();
              self.autoExportDocx();
            }
            // java case FF_DOTX:
            FF_DOTX => {
              // java this.exportDotx(true);
              self.exportDotx(true);
            }
            // java case FF_TXT:
            FF_TXT => {
              // java this.autoExportTxt();
              self.autoExportTxt();
            }
            // java case FF_RTF: case FF_HTML:
            FF_RTF | FF_HTML => {
              // java this.autoExportDoc();
              self.autoExportDoc();
            }
            // java case FF_XML07:
            FF_XML07 => {
              // java this.autoExportXml07();
              self.autoExportXml07();
            }
            // java case FF_XML03:
            FF_XML03 => {
              // java this.autoExportXml03();
              self.autoExportXml03();
            }
            // java case FF_EPUB:
            FF_EPUB => {
              // java this.exportEpub();
              self.exportEpub();
            }
            // java case FF_UOT3:
            FF_UOT3 => {
              // java this.autoExportUot3();
              self.autoExportUot3();
            }
            // java default:
            _ => {
              // java IOAssert.fail("It should not reach here");
              IOAssert::fail__string(&"It should not reach here");
              // java this.exportTxt();
              self.exportTxt();
            }
          }
          // java ISlimToolManager.instance().clearCache();
          ISlimToolManager::instance().clearCache();
          // java WriterAutoWriteException e
          todo!();
            // java IOLog.e(TAG, "WriterAutoWriteException", e);
            IOLog::e__string_string_throwable(&TAG, &"WriterAutoWriteException", &e);
            // java throw e;
            panic!("{}", e);
          // java IOException e
          todo!();
            // java IOLog.e(TAG, "IOException", e);
            IOLog::e__string_string_throwable(&TAG, &"IOException", &e);
            // java throw new WriterIOException(new WriterIOStatus(4, e.getMessage(), e));
            panic!("{}", WriterIOException::new(WriterIOStatus::new(4, e.getMessage(), e)));
    }

    // java private void exportTxt()
    pub fn exportTxt(&self) {
      // java TextWriter writer = new TextWriter(this.mDocument, this.mSavedFilePath);
      let mut writer: TextWriter = TextWriter::new(self.mDocument, self.mSavedFilePath);
      // java writer.write(false);
      writer.write(false);
    }

    // java private void autoExportTxt()
    pub fn autoExportTxt(&self) {
      // java TextWriter writer = new TextWriter(this.mDocument, this.mSavedFilePath);
      let mut writer: TextWriter = TextWriter::new(self.mDocument, self.mSavedFilePath);
      // java writer.write(true);
      writer.write(true);
    }

    // java private static InputStream getAssetsInputStream(String assetFilePath)
    pub fn getAssetsInputStream(assetFilePath: &str) -> Box<dyn std::io::Read> {
      // java InputStream inputStream = null;
      let mut inputStream: Box<dyn std::io::Read> = Box::new(()) as Box<dyn std::io::Read>;
          // java inputStream = Platform.getAssetManager().open(assetFilePath);
          inputStream = Platform::getAssetManager().open(assetFilePath);
          // java IOException e
          todo!();
            // java IOLog.d(TAG, "IOException", e);
            IOLog::d__string_string_throwable(&TAG, &"IOException", &e);
      // java return inputStream;
      return inputStream;
    }

    // java private static String getCurrentLanguage()
    pub fn getCurrentLanguage() -> String {
      // java Locale locale = Locale.getDefault();
      let mut locale: String = Locale::getDefault();
      // java return locale.getLanguage() + "-" + locale.getCountry();
      return locale.getLanguage() + "-" + locale.getCountry();
    }

    // java public static InputStream getWriterBlankFile(String blankFilePath)
    pub fn getWriterBlankFile(blankFilePath: &str) -> Box<dyn std::io::Read> {
      // java String pBlankFile = StringUtil.stringWithFormat(blankFilePath, new Object[]{getCurrentLanguage()});
      let mut pBlankFile: String = StringUtil::stringWithFormat(&mut blankFilePath, vec![Self::getCurrentLanguage()]);
      // java InputStream inputStream = getAssetsInputStream(pBlankFile);
      let mut inputStream: Box<dyn std::io::Read> = Self::getAssetsInputStream(&pBlankFile);
      // java if (inputStream == null)
      if inputStream.is_none() {
        // java pBlankFile = StringUtil.stringWithFormat(blankFilePath, new Object[]{"default"});
        pBlankFile = StringUtil::stringWithFormat(&mut blankFilePath, vec!["default"]);
        // java inputStream = getAssetsInputStream(pBlankFile);
        inputStream = Self::getAssetsInputStream(&pBlankFile);
      }
      // java if (inputStream == null)
      if inputStream.is_none() {
        // java throw new FileNotFoundException("Can not open blank file of writer");
        panic!("{}", FileNotFoundException::new("Can not open blank file of writer"));
      }
      // java else
      else {
        // java return inputStream;
        return inputStream;
      }
    }

    // java private void exportDoc()
    pub fn exportDoc(&self) {
      // java DocWriter writer = new DocWriter(this.mDocument, this.mSavedFilePath, this.mProviderInfo, false);
      // CARGO_BLOCKER: DocWriter screened in doc/mod.rs (detached stub)
      todo!("DocWriter exportDoc");
    }

    // java private void autoExportDoc()
    pub fn autoExportDoc(&self) {
      // java DocWriter writer = new DocWriter(this.mDocument, this.mSavedFilePath, this.mProviderInfo, true);
      // CARGO_BLOCKER: DocWriter screened in doc/mod.rs (detached stub)
      todo!("DocWriter autoExportDoc");
    }

    // java private void exportDocx()
    pub fn exportDocx(&self) {
      // java IDocxWriter writer = ReflectionHelper.getDocxWriter(this, this.mDocument, this.mSavedFilePath, false, this.mProviderInfo, FileFormat.FF_DOCX);
      let mut writer: Box<dyn IDocxWriter> = ReflectionHelper::getDocxWriter(&self, &self.mDocument, &self.mSavedFilePath, false, &self.mProviderInfo, &FileFormat::FF_DOCX);
      // java writer.write();
      writer.write();
    }

    // java private void autoExportDocx()
    pub fn autoExportDocx(&self) {
      // java IDocxWriter writer = ReflectionHelper.getDocxWriter(this, this.mDocument, this.mSavedFilePath, true, this.mProviderInfo, FileFormat.FF_DOCX);
      let mut writer: Box<dyn IDocxWriter> = ReflectionHelper::getDocxWriter(&self, &self.mDocument, &self.mSavedFilePath, true, &self.mProviderInfo, &FileFormat::FF_DOCX);
      // java writer.write();
      writer.write();
    }

    // java private void exportDotx(boolean isAutoSave)
    pub fn exportDotx(&self, isAutoSave: bool) {
      // java IDocxWriter writer = ReflectionHelper.getDocxWriter(this, this.mDocument, this.mSavedFilePath, true, this.mProviderInfo, FileFormat.FF_DOTX);
      let mut writer: Box<dyn IDocxWriter> = ReflectionHelper::getDocxWriter(&self, &self.mDocument, &self.mSavedFilePath, true, &self.mProviderInfo, &FileFormat::FF_DOTX);
      // java writer.write();
      writer.write();
    }

    // java private void exportXml07()
    pub fn exportXml07(&self) {
      // java IXml07Writer writer = ReflectionHelper.getXml07Writer(this, this.mDocument, this.mSavedFilePath, false, this.mProviderInfo, FileFormat.FF_XML07);
      let mut writer: Box<dyn IXml07Writer> = ReflectionHelper::getXml07Writer(&self, &self.mDocument, &self.mSavedFilePath, false, &self.mProviderInfo, &FileFormat::FF_XML07);
      // java writer.write();
      writer.write();
    }

    // java private void autoExportXml07()
    pub fn autoExportXml07(&self) {
      // java IXml07Writer writer = ReflectionHelper.getXml07Writer(this, this.mDocument, this.mSavedFilePath, true, this.mProviderInfo, FileFormat.FF_XML07);
      let mut writer: Box<dyn IXml07Writer> = ReflectionHelper::getXml07Writer(&self, &self.mDocument, &self.mSavedFilePath, true, &self.mProviderInfo, &FileFormat::FF_XML07);
      // java writer.write();
      writer.write();
    }

    // java private void exportXml03()
    pub fn exportXml03(&self) {
      // java IXml03Writer writer = ReflectionHelper.getXml03Writer(this, this.mDocument, this.mSavedFilePath, false, this.mProviderInfo, FileFormat.FF_XML03);
      let mut writer: Box<dyn IXml03Writer> = ReflectionHelper::getXml03Writer(&self, &self.mDocument, &self.mSavedFilePath, false, &self.mProviderInfo, &FileFormat::FF_XML03);
      // java writer.write();
      writer.write();
    }

    // java private void autoExportXml03()
    pub fn autoExportXml03(&self) {
      // java IXml03Writer writer = ReflectionHelper.getXml03Writer(this, this.mDocument, this.mSavedFilePath, true, this.mProviderInfo, FileFormat.FF_XML03);
      let mut writer: Box<dyn IXml03Writer> = ReflectionHelper::getXml03Writer(&self, &self.mDocument, &self.mSavedFilePath, true, &self.mProviderInfo, &FileFormat::FF_XML03);
      // java writer.write();
      writer.write();
    }

    // java private void exportUot3()
    pub fn exportUot3(&self) {
      // java IUot3Writer writer = ReflectionHelper.getUot3Writer(this, this.mDocument, this.mSavedFilePath, false, this.mProviderInfo);
      let mut writer: Box<dyn IUot3Writer> = ReflectionHelper::getUot3Writer(&self, &self.mDocument, &self.mSavedFilePath, false, &self.mProviderInfo);
      // java writer.write();
      writer.write();
    }

    // java private void autoExportUot3()
    pub fn autoExportUot3(&self) {
      // java IUot3Writer writer = ReflectionHelper.getUot3Writer(this, this.mDocument, this.mSavedFilePath, true, this.mProviderInfo);
      let mut writer: Box<dyn IUot3Writer> = ReflectionHelper::getUot3Writer(&self, &self.mDocument, &self.mSavedFilePath, true, &self.mProviderInfo);
      // java writer.write();
      writer.write();
    }

    // java public static void moveFile(String srcFile, String destFile)
    pub fn moveFile(srcFile: &str, destFile: &str) {
      // java File pDestFile = new File(destFile);
      let mut pDestFile: std::path::PathBuf = File::new(destFile);
      // java pDestFile.getParentFile().mkdirs();
      pDestFile.getParentFile().mkdirs();
      // java boolean bMoveOK = (new File(srcFile)).renameTo(pDestFile);
      let mut bMoveOK: bool = File::new(srcFile).renameTo(pDestFile);
      // java if (!bMoveOK)
      if !bMoveOK {
        // java FileUtil.copyFile(srcFile, destFile);
        FileUtil::copyFile__string_string(Some(&srcFile), Some(&destFile));
        // java File file = new File(srcFile);
        let mut file: std::path::PathBuf = File::new(srcFile);
        // java file.delete();
        file.delete();
      }
    }

    // java public byte[] exportAsArray()
    pub fn exportAsArray(&mut self) -> Vec<u8> {
          // java byte[] bytes;
          let mut bytes: Vec<u8> = vec![];
          // java switch (this.mFf)
          match self.mFf {
            // java case FF_DOC:
            FF_DOC => {
              // java bytes = this.exportDocAsArray();
              bytes = self.exportDocAsArray();
            }
            // java case FF_DOCX:
            FF_DOCX => {
              // java bytes = this.exportDocxAsArray();
              bytes = self.exportDocxAsArray();
            }
            // java case FF_DOTX: default:
            FF_DOTX | _ => {
              // java IOAssert.fail("It should not reach here!");
              IOAssert::fail__string(&"It should not reach here!");
              // java throw new InvalidFormatException("format unsupported: " + this.mFf);
              panic!("{}", InvalidFormatException::new("format unsupported: " + self.mFf));
            }
            // java case FF_TXT:
            FF_TXT => {
              // java bytes = this.exportTxtAsArray();
              bytes = self.exportTxtAsArray();
            }
            // java case FF_RTF:
            FF_RTF => {
              // java bytes = this.exportDocAsArray();
              bytes = self.exportDocAsArray();
            }
          }
          // java ISlimToolManager.instance().clearCache();
          ISlimToolManager::instance().clearCache();
          // java FileUtil.delFile(this.mSavedFilePath);
          FileUtil::delFile(Some(&self.mSavedFilePath));
          // java return bytes;
          return bytes;
          // java IOException e
          todo!();
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java IOLog.e(TAG, "NOSpaceLeftException!!!");
                IOLog::e__string_string(&TAG, &"NOSpaceLeftException!!!");
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
                // java IOLog.e(TAG, "IOException", e);
                IOLog::e__string_string_throwable(&TAG, &"IOException", &e);
                // java throw new WriterIOException(new WriterIOStatus(4, e.getMessage(), e));
                panic!("{}", WriterIOException::new(WriterIOStatus::new(4, e.getMessage(), e)));
          // java RuntimeException e
          todo!();
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java IOLog.e(TAG, "NOSpaceLeftException!!!");
                IOLog::e__string_string(&TAG, &"NOSpaceLeftException!!!");
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
                // java IOLog.e(TAG, "RuntimeException", e);
                IOLog::e__string_string_throwable(&TAG, &"RuntimeException", &e);
                // java throw e;
                panic!("{}", e);
          // java InvalidFormatException e
          todo!();
            // java IOLog.e(TAG, "InvalidFormatException", e);
            IOLog::e__string_string_throwable(&TAG, &"InvalidFormatException", &e);
            // java throw e;
            panic!("{}", e);
          // java WriterIOException e
          todo!();
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java IOLog.e(TAG, "NOSpaceLeftException!!!");
                IOLog::e__string_string(&TAG, &"NOSpaceLeftException!!!");
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
                // java IOLog.e(TAG, "WriterIOException", e);
                IOLog::e__string_string_throwable(&TAG, &"WriterIOException", &e);
                // java throw e;
                panic!("{}", e);
    }

    // java public byte[] exportDocxAsArray()
    pub fn exportDocxAsArray(&self) -> Vec<u8> {
      // java IDocxWriter writer = ReflectionHelper.getDocxWriter(this, this.mDocument, this.mSavedFilePath, false, this.mProviderInfo, FileFormat.FF_DOCX);
      let mut writer: Box<dyn IDocxWriter> = ReflectionHelper::getDocxWriter(&self, &self.mDocument, &self.mSavedFilePath, false, &self.mProviderInfo, &FileFormat::FF_DOCX);
      // java return writer.exportAsArray();
      return writer.exportAsArray();
    }

    // java public byte[] exportDocAsArray()
    pub fn exportDocAsArray(&self) -> Vec<u8> {
      // java DocWriter writer = new DocWriter(this.mDocument, this.mSavedFilePath, this.mProviderInfo, false);
      // CARGO_BLOCKER: DocWriter screened in doc/mod.rs (detached stub)
      todo!("DocWriter exportDocAsArray");
    }

    // java public byte[] exportTxtAsArray()
    pub fn exportTxtAsArray(&mut self) -> Option<Vec<u8>> {
      // java return null;
      return None;
    }

}
