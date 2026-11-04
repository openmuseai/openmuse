// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform;
// Java: import cn.wps.crypt.CryptDocxFile
use io_poi_compact::cn::wps::crypt::CryptDocxFile;
// Java: import cn.wps.io.file.EncryptedType
use io_libfileformat::cn::wps::io::file::EncryptedType;
// Java: import cn.wps.io.file.FileFormatEnum
use util::cn::wps::io::file::FileFormatEnum;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.io.file.checker.html.HtmlChecker
use io_libfileformat::io::file::checker::html::HtmlChecker;
// Java: import cn.wps.io.file.checker.html.MHTChecker
use io_libfileformat::cn::wps::io::file::checker::html::MHTChecker;
// Java: import cn.wps.io.file.checker.ole.DocChecker
use io_libfileformat::io::file::checker::ole::DocChecker;
// Java: import cn.wps.io.file.checker.ole.PPTChecker
use io_libfileformat::io::file::checker::ole::PPTChecker;
// Java: import cn.wps.io.file.checker.ole.XlsChecker
use io_libfileformat::io::file::checker::ole::XlsChecker;
// Java: import cn.wps.io.file.checker.ooxml.DocmChecker
use io_libfileformat::io::file::checker::ooxml::DocmChecker;
// Java: import cn.wps.io.file.checker.ooxml.DocxChecker
use io_libfileformat::io::file::checker::ooxml::DocxChecker;
// Java: import cn.wps.io.file.checker.ooxml.DotmChecker
use io_libfileformat::io::file::checker::ooxml::DotmChecker;
// Java: import cn.wps.io.file.checker.ooxml.DotxChecker
use io_libfileformat::io::file::checker::ooxml::DotxChecker;
// Java: import cn.wps.io.file.checker.ooxml.OOXMLChecker
use io_libfileformat::io::file::checker::ooxml::OOXMLChecker;
// Java: import cn.wps.io.file.checker.ooxml.PotmChecker
use io_libfileformat::io::file::checker::ooxml::PotmChecker;
// Java: import cn.wps.io.file.checker.ooxml.PotxChecker
use io_libfileformat::io::file::checker::ooxml::PotxChecker;
// Java: import cn.wps.io.file.checker.ooxml.PpsxChecker
use io_libfileformat::io::file::checker::ooxml::PpsxChecker;
// Java: import cn.wps.io.file.checker.ooxml.PptxChecker
use io_libfileformat::io::file::checker::ooxml::PptxChecker;
// Java: import cn.wps.io.file.checker.ooxml.Uof3Checker
use io_libfileformat::io::file::checker::ooxml::Uof3Checker;
// Java: import cn.wps.io.file.checker.ooxml.Uop3Checker
use io_libfileformat::io::file::checker::ooxml::Uop3Checker;
// Java: import cn.wps.io.file.checker.ooxml.Uos3Checker
use io_libfileformat::io::file::checker::ooxml::Uos3Checker;
// Java: import cn.wps.io.file.checker.ooxml.Uot3Checker
use io_libfileformat::io::file::checker::ooxml::Uot3Checker;
// Java: import cn.wps.io.file.checker.ooxml.XlamChecker
use io_libfileformat::io::file::checker::ooxml::XlamChecker;
// Java: import cn.wps.io.file.checker.ooxml.XlsbChecker
use io_libfileformat::io::file::checker::ooxml::XlsbChecker;
// Java: import cn.wps.io.file.checker.ooxml.XlsmChecker
use io_libfileformat::io::file::checker::ooxml::XlsmChecker;
// Java: import cn.wps.io.file.checker.ooxml.XlsxChecker
use io_libfileformat::io::file::checker::ooxml::XlsxChecker;
// Java: import cn.wps.io.file.checker.ooxml.XltmChecker
use io_libfileformat::io::file::checker::ooxml::XltmChecker;
// Java: import cn.wps.io.file.checker.ooxml.XltxChecker
use io_libfileformat::io::file::checker::ooxml::XltxChecker;
// Java: import cn.wps.io.file.checker.pdf.PdfChecker
use io_libfileformat::cn::wps::io::file::checker::pdf::PdfChecker;
// Java: import cn.wps.io.file.checker.rtf.RtfChecker
use io_libfileformat::io::file::checker::rtf::RtfChecker;
// Java: import cn.wps.io.file.checker.xml.WordXml07Checker
use io_libfileformat::cn::wps::io::file::checker::xml::WordXml07Checker;
// Java: import cn.wps.io.file.checker.xml.XmlChecker
use io_libfileformat::cn::wps::io::file::checker::xml::XmlChecker;
// Java: import cn.wps.io.file.csv.CsvChecker
use io_libfileformat::io::file::csv::CsvChecker;
// Java: import cn.wps.io.uof.crypt.util.EncryptPkgUtils
use io_uof_io::cn::wps::io::uof::crypt::util::EncryptPkgUtils;
// Java: import cn.wps.moffice.io.IFileParser
use util_adapter::cn::wps::moffice::io::IFileParser;
// Java: import cn.wps.moffice.io.PasswordErrorException
use util_adapter::cn::wps::moffice::io::PasswordErrorException;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.FileOutputStream
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileOutputStream;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import java.io.FileNotFoundException
// TODO_RUST_CODE use ???; // java.io.FileNotFoundException
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.InputStream
// TODO_RUST_CODE use ???; // java.io.InputStream
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "";

// Java: public class FileParser implements IFileParser
pub struct FileParser {
    // Java field: RandomAccessFile mRaf
    pub(crate) mRaf: Arc<Mutex<RandomAccessFile>>,
    // Java field: File mFile
    mFile: Option<File>,
    // Java field: FileFormatEnum mFileFormat
    pub(crate) mFileFormat: Arc<Mutex<FileFormatEnum>>,
    // Java field: EncryptedType mEncryptedType
    pub(crate) mEncryptedType: Arc<Mutex<EncryptedType>>,
    // Java field: Boolean mIsDoc
    pub(crate) mIsDoc: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsDocx
    pub(crate) mIsDocx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsDotx
    pub(crate) mIsDotx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsDocm
    pub(crate) mIsDocm: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsDotm
    pub(crate) mIsDotm: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsUot3
    pub(crate) mIsUot3: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsRtf
    pub(crate) mIsRtf: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXls
    pub(crate) mIsXls: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXlsx
    pub(crate) mIsXlsx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXltx
    pub(crate) mIsXltx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXlsm
    pub(crate) mIsXlsm: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXltm
    pub(crate) mIsXltm: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXlsb
    pub(crate) mIsXlsb: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsUos3
    pub(crate) mIsUos3: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXlam
    pub(crate) mIsXlam: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsPpt
    pub(crate) mIsPpt: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsPptx
    pub(crate) mIsPptx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsPotx
    pub(crate) mIsPotx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsPpsx
    pub(crate) mIsPpsx: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsUop3
    pub(crate) mIsUop3: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsPotm
    pub(crate) mIsPotm: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsMht
    pub(crate) mIsMht: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsHtml
    pub(crate) mIsHtml: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsXml
    pub(crate) mIsXml: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsWordXml03
    mIsWordXml03: Option<Boolean>,
    // Java field: Boolean mIsWordXml07
    pub(crate) mIsWordXml07: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsPdf
    pub(crate) mIsPdf: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsCsv
    pub(crate) mIsCsv: Arc<Mutex<Boolean>>,
    // Java field: Boolean mIsOfd
    pub(crate) mIsOfd: Arc<Mutex<Boolean>>,
    // Java field: DocChecker mDocChecker
    pub(crate) mDocChecker: Arc<Mutex<DocChecker>>,
    // Java field: DocxChecker mDocxChecker
    pub(crate) mDocxChecker: Arc<Mutex<DocxChecker>>,
    // Java field: DotxChecker mDotxChecker
    pub(crate) mDotxChecker: Arc<Mutex<DotxChecker>>,
    // Java field: DocmChecker mDocmChecker
    pub(crate) mDocmChecker: Arc<Mutex<DocmChecker>>,
    // Java field: DotmChecker mDotmChecker
    pub(crate) mDotmChecker: Arc<Mutex<DotmChecker>>,
    // Java field: RtfChecker mRtfChecker
    pub(crate) mRtfChecker: Arc<Mutex<RtfChecker>>,
    // Java field: XlsChecker mXlsChecker
    pub(crate) mXlsChecker: Arc<Mutex<XlsChecker>>,
    // Java field: XlsxChecker mXlsxChecker
    pub(crate) mXlsxChecker: Arc<Mutex<XlsxChecker>>,
    // Java field: XltxChecker mXltxChecker
    pub(crate) mXltxChecker: Arc<Mutex<XltxChecker>>,
    // Java field: XlsmChecker mXlsmChecker
    pub(crate) mXlsmChecker: Arc<Mutex<XlsmChecker>>,
    // Java field: XltmChecker mXltmChecker
    pub(crate) mXltmChecker: Arc<Mutex<XltmChecker>>,
    // Java field: XlsbChecker mXlsbChecker
    pub(crate) mXlsbChecker: Arc<Mutex<XlsbChecker>>,
    // Java field: XlamChecker mXlamChecker
    pub(crate) mXlamChecker: Arc<Mutex<XlamChecker>>,
    // Java field: PPTChecker mPptChecker
    pub(crate) mPptChecker: Arc<Mutex<PPTChecker>>,
    // Java field: PptxChecker mPptxChecker
    pub(crate) mPptxChecker: Arc<Mutex<PptxChecker>>,
    // Java field: PotxChecker mPotxChecker
    pub(crate) mPotxChecker: Arc<Mutex<PotxChecker>>,
    // Java field: PpsxChecker mPpsxChecker
    pub(crate) mPpsxChecker: Arc<Mutex<PpsxChecker>>,
    // Java field: PotmChecker mPotmChecker
    pub(crate) mPotmChecker: Arc<Mutex<PotmChecker>>,
    // Java field: OOXMLChecker mOOXMLChecker
    pub(crate) mOOXMLChecker: Arc<Mutex<OOXMLChecker>>,
    // Java field: PdfChecker mPdfChecker
    pub(crate) mPdfChecker: Arc<Mutex<PdfChecker>>,
    // Java field: MHTChecker mMhtChecker
    pub(crate) mMhtChecker: Arc<Mutex<MHTChecker>>,
    // Java field: HtmlChecker mHtmlChecker
    pub(crate) mHtmlChecker: Arc<Mutex<HtmlChecker>>,
    // Java field: XmlChecker mXmlChecker
    pub(crate) mXmlChecker: Arc<Mutex<XmlChecker>>,
    // Java field: WordXml07Checker mWordXml07Checker
    pub(crate) mWordXml07Checker: Arc<Mutex<WordXml07Checker>>,
    // Java field: CsvChecker mCsvChecker
    pub(crate) mCsvChecker: Arc<Mutex<CsvChecker>>,
    // Java field: Uof3Checker mUof3Checker
    pub(crate) mUof3Checker: Arc<Mutex<Uof3Checker>>,
    // Java field: Uot3Checker mUotChecker
    pub(crate) mUotChecker: Arc<Mutex<Uot3Checker>>,
    // Java field: Uos3Checker mUosChecker
    pub(crate) mUosChecker: Arc<Mutex<Uos3Checker>>,
    // Java field: Uop3Checker mUopChecker
    pub(crate) mUopChecker: Arc<Mutex<Uop3Checker>>,
    // Java field: POIFSFileSystem _poifsFileSystem
    pub(crate) _poifsFileSystem: Arc<Mutex<POIFSFileSystem>>,
    // Java field: File mOOXMLDecrypted
    pub(crate) mOOXMLDecrypted: Arc<Mutex<File>>,
    // Java field: String mUofDecrypted
    pub(crate) mUofDecrypted: Arc<Mutex<String>>,
    // Java field: boolean mIsLegal
    pub(crate) mIsLegal: bool,
}

// Java: public class FileParser implements IFileParser
impl FileParser {
    // Java: FileParser(File file, POIFSFileSystem poifs)
    pub fn FileParser(&self, File file, POIFSFileSystem poifs) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mRaf = null;
      // TODO_RUST_CODE
      // Java: this.mFile = null;
      // TODO_RUST_CODE
      // Java: this.mFileFormat = null;
      // TODO_RUST_CODE
      // Java: this.mEncryptedType = null;
      // TODO_RUST_CODE
      // Java: this.mIsDoc = null;
      // TODO_RUST_CODE
      // Java: this.mIsDocx = null;
      // TODO_RUST_CODE
      // Java: this.mIsDotx = null;
      // TODO_RUST_CODE
      // Java: this.mIsDocm = null;
      // TODO_RUST_CODE
      // Java: this.mIsDotm = null;
      // TODO_RUST_CODE
      // Java: this.mIsUot3 = null;
      // TODO_RUST_CODE
      // Java: this.mIsRtf = null;
      // TODO_RUST_CODE
      // Java: this.mIsXls = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlsx = null;
      // TODO_RUST_CODE
      // Java: this.mIsXltx = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlsm = null;
      // TODO_RUST_CODE
      // Java: this.mIsXltm = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlsb = null;
      // TODO_RUST_CODE
      // Java: this.mIsUos3 = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlam = null;
      // TODO_RUST_CODE
      // Java: this.mIsPpt = null;
      // TODO_RUST_CODE
      // Java: this.mIsPptx = null;
      // TODO_RUST_CODE
      // Java: this.mIsPotx = null;
      // TODO_RUST_CODE
      // Java: this.mIsPpsx = null;
      // TODO_RUST_CODE
      // Java: this.mIsUop3 = null;
      // TODO_RUST_CODE
      // Java: this.mIsPotm = null;
      // TODO_RUST_CODE
      // Java: this.mIsMht = null;
      // TODO_RUST_CODE
      // Java: this.mIsHtml = null;
      // TODO_RUST_CODE
      // Java: this.mIsXml = null;
      // TODO_RUST_CODE
      // Java: this.mIsWordXml03 = null;
      // TODO_RUST_CODE
      // Java: this.mIsWordXml07 = null;
      // TODO_RUST_CODE
      // Java: this.mIsPdf = null;
      // TODO_RUST_CODE
      // Java: this.mIsCsv = null;
      // TODO_RUST_CODE
      // Java: this.mIsOfd = null;
      // TODO_RUST_CODE
      // Java: this.mDocChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDocxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDotxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDocmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDotmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mRtfChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXltxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXltmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsbChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlamChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPptChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPptxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPotxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPpsxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPotmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mOOXMLChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPdfChecker = null;
      // TODO_RUST_CODE
      // Java: this.mMhtChecker = null;
      // TODO_RUST_CODE
      // Java: this.mHtmlChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXmlChecker = null;
      // TODO_RUST_CODE
      // Java: this.mWordXml07Checker = null;
      // TODO_RUST_CODE
      // Java: this.mCsvChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUof3Checker = null;
      // TODO_RUST_CODE
      // Java: this.mUotChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUosChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUopChecker = null;
      // TODO_RUST_CODE
      // Java: this._poifsFileSystem = null;
      // TODO_RUST_CODE
      // Java: this.mOOXMLDecrypted = null;
      // TODO_RUST_CODE
      // Java: this.mUofDecrypted = null;
      // TODO_RUST_CODE
      // Java: this.mIsLegal = true;
      // TODO_RUST_CODE
      // Java: this.mFile = file;
      // TODO_RUST_CODE
      // Java: this._poifsFileSystem = poifs;
      // TODO_RUST_CODE
    }

    // Java: FileParser(File file)
    pub fn FileParser(&self, File file) {
      // Java: this((File)file, (POIFSFileSystem)null);
      // TODO_RUST_CODE
    }

    // Java: FileParser(RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn FileParser(&self, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mRaf = null;
      // TODO_RUST_CODE
      // Java: this.mFile = null;
      // TODO_RUST_CODE
      // Java: this.mFileFormat = null;
      // TODO_RUST_CODE
      // Java: this.mEncryptedType = null;
      // TODO_RUST_CODE
      // Java: this.mIsDoc = null;
      // TODO_RUST_CODE
      // Java: this.mIsDocx = null;
      // TODO_RUST_CODE
      // Java: this.mIsDotx = null;
      // TODO_RUST_CODE
      // Java: this.mIsDocm = null;
      // TODO_RUST_CODE
      // Java: this.mIsDotm = null;
      // TODO_RUST_CODE
      // Java: this.mIsUot3 = null;
      // TODO_RUST_CODE
      // Java: this.mIsRtf = null;
      // TODO_RUST_CODE
      // Java: this.mIsXls = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlsx = null;
      // TODO_RUST_CODE
      // Java: this.mIsXltx = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlsm = null;
      // TODO_RUST_CODE
      // Java: this.mIsXltm = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlsb = null;
      // TODO_RUST_CODE
      // Java: this.mIsUos3 = null;
      // TODO_RUST_CODE
      // Java: this.mIsXlam = null;
      // TODO_RUST_CODE
      // Java: this.mIsPpt = null;
      // TODO_RUST_CODE
      // Java: this.mIsPptx = null;
      // TODO_RUST_CODE
      // Java: this.mIsPotx = null;
      // TODO_RUST_CODE
      // Java: this.mIsPpsx = null;
      // TODO_RUST_CODE
      // Java: this.mIsUop3 = null;
      // TODO_RUST_CODE
      // Java: this.mIsPotm = null;
      // TODO_RUST_CODE
      // Java: this.mIsMht = null;
      // TODO_RUST_CODE
      // Java: this.mIsHtml = null;
      // TODO_RUST_CODE
      // Java: this.mIsXml = null;
      // TODO_RUST_CODE
      // Java: this.mIsWordXml03 = null;
      // TODO_RUST_CODE
      // Java: this.mIsWordXml07 = null;
      // TODO_RUST_CODE
      // Java: this.mIsPdf = null;
      // TODO_RUST_CODE
      // Java: this.mIsCsv = null;
      // TODO_RUST_CODE
      // Java: this.mIsOfd = null;
      // TODO_RUST_CODE
      // Java: this.mDocChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDocxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDotxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDocmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDotmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mRtfChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXltxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXltmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsbChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlamChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPptChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPptxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPotxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPpsxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPotmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mOOXMLChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPdfChecker = null;
      // TODO_RUST_CODE
      // Java: this.mMhtChecker = null;
      // TODO_RUST_CODE
      // Java: this.mHtmlChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXmlChecker = null;
      // TODO_RUST_CODE
      // Java: this.mWordXml07Checker = null;
      // TODO_RUST_CODE
      // Java: this.mCsvChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUof3Checker = null;
      // TODO_RUST_CODE
      // Java: this.mUotChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUosChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUopChecker = null;
      // TODO_RUST_CODE
      // Java: this._poifsFileSystem = null;
      // TODO_RUST_CODE
      // Java: this.mOOXMLDecrypted = null;
      // TODO_RUST_CODE
      // Java: this.mUofDecrypted = null;
      // TODO_RUST_CODE
      // Java: this.mIsLegal = true;
      // TODO_RUST_CODE
      // Java: this.mRaf = raf;
      // TODO_RUST_CODE
      // Java: this._poifsFileSystem = poifs;
      // TODO_RUST_CODE
    }

    // Java: FileParser(RandomAccessFile raf)
    pub fn FileParser(&self, RandomAccessFile raf) {
      // Java: this((RandomAccessFile)raf, (POIFSFileSystem)null);
      // TODO_RUST_CODE
    }

    // Java: void dispose()
    pub fn dispose(&self) {
      // Java: this.mFile = null;
      // TODO_RUST_CODE
      // Java: this.mRaf = null;
      // TODO_RUST_CODE
      // Java: this.mDocChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDocxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mDotxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXltxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXltmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlsbChecker = null;
      // TODO_RUST_CODE
      // Java: this.mXlamChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPptChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPptxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPotxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPpsxChecker = null;
      // TODO_RUST_CODE
      // Java: this.mPotmChecker = null;
      // TODO_RUST_CODE
      // Java: this.mMhtChecker = null;
      // TODO_RUST_CODE
      // Java: this.mHtmlChecker = null;
      // TODO_RUST_CODE
      // Java: this.mCsvChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUotChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUosChecker = null;
      // TODO_RUST_CODE
      // Java: this.mUopChecker = null;
      // TODO_RUST_CODE
    }

    // Java: FileFormatEnum parse()
    pub fn parse(&self) -> FileFormatEnum {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: return this.parse((String)null);
         // TODO_RUST_CODE
      }
      // Java: catch (PasswordErrorException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "PasswordErrorException", e);
         // TODO_RUST_CODE
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: FileFormatEnum parse(String password)
    pub fn parse(&self, String password) -> FileFormatEnum {
      // Java: if (null != this.mFileFormat)
        // TODO_RUST_CODE
      {
         // Java: return this.mFileFormat;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: EncryptedType encryptedType = this.getEncryptedType();
         // TODO_RUST_CODE
         // Java: Assert.assertNotNull("encryptedType should not be null", encryptedType);
         // TODO_RUST_CODE
         // Java: if (EncryptedType.OOXML == encryptedType)
            // TODO_RUST_CODE
         {
            // Java: if (null == password)
                // TODO_RUST_CODE
            {
               // Java: this.mFileFormat = FileFormatEnum.getFileFormatEnum(null != this.mRaf ? Tools.getLowerCaseExtension(this.mRaf.getFileName()) : Tools.getLowerCaseExtension(this.mFile.getAbsolutePath()));
               // TODO_RUST_CODE
               // Java: this.mFileFormat = null == this.mFileFormat ? FileFormatEnum.TXT : this.mFileFormat;
               // TODO_RUST_CODE
               // Java: if (this.mFileFormat == FileFormatEnum.DOC)
                    // TODO_RUST_CODE
               {
                  // Java: this.mFileFormat = FileFormatEnum.DOCX;
                  // TODO_RUST_CODE
               }
               // Java: if (this.mFileFormat == FileFormatEnum.XLS)
                    // TODO_RUST_CODE
               {
                  // Java: this.mFileFormat = FileFormatEnum.XLSX;
                  // TODO_RUST_CODE
               }
               // Java: if (this.mFileFormat == FileFormatEnum.PPT)
                    // TODO_RUST_CODE
               {
                  // Java: this.mFileFormat = FileFormatEnum.PPTX;
                  // TODO_RUST_CODE
               }
               // Java: return this.mFileFormat;
               // TODO_RUST_CODE
            }
            // Java: this.mOOXMLDecrypted = this.decryptOOXML(password);
            // TODO_RUST_CODE
            // Java: if (null != this.mOOXMLDecrypted)
                // TODO_RUST_CODE
            {
               // Java: this.mRaf = null;
               // TODO_RUST_CODE
            }
         }
         // Java: if (EncryptedType.UOF == encryptedType && null != password)
            // TODO_RUST_CODE
         {
            // Java: this.mUofDecrypted = this.decryptUof(password);
            // TODO_RUST_CODE
            // Java: if (null != this.mUofDecrypted)
                // TODO_RUST_CODE
            {
               // Java: this.mRaf = null;
               // TODO_RUST_CODE
            }
         }
         // Java: this._poifsFileSystem = GuessWordMethod.guessWord(this);
         // TODO_RUST_CODE
         // Java: if (null != this.mFileFormat)
            // TODO_RUST_CODE
         {
            // Java: return this.mFileFormat;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: GuessExcelMethod.guessExcel(this);
            // TODO_RUST_CODE
            // Java: if (null != this.mFileFormat)
                // TODO_RUST_CODE
            {
               // Java: return this.mFileFormat;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: GuessPowerPointMethod.guessPowerPoint(this);
               // TODO_RUST_CODE
               // Java: if (null != this.mFileFormat)
                    // TODO_RUST_CODE
               {
                  // Java: return this.mFileFormat;
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: GuessPdfMethod.guessPdf(this);
                  // TODO_RUST_CODE
                  // Java: if (null != this.mFileFormat)
                        // TODO_RUST_CODE
                  {
                     // Java: return this.mFileFormat;
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: GuessMHTMethod.guessMHT(this);
                     // TODO_RUST_CODE
                     // Java: if (null != this.mFileFormat)
                            // TODO_RUST_CODE
                     {
                        // Java: return this.mFileFormat;
                        // TODO_RUST_CODE
                     }
                     // Java: else
                            // TODO_RUST_CODE
                     {
                        // Java: GuessHtmlMethod.guessHtml(this);
                        // TODO_RUST_CODE
                        // Java: if (null != this.mFileFormat)
                                // TODO_RUST_CODE
                        {
                           // Java: return this.mFileFormat;
                           // TODO_RUST_CODE
                        }
                        // Java: else
                                // TODO_RUST_CODE
                        {
                           // Java: GuessWordXml07Method.guessWordXml07(this);
                           // TODO_RUST_CODE
                           // Java: if (null != this.mFileFormat)
                                    // TODO_RUST_CODE
                           {
                              // Java: return this.mFileFormat;
                              // TODO_RUST_CODE
                           }
                           // Java: else
                                    // TODO_RUST_CODE
                           {
                              // Java: GuessXmlMethod.guessXml(this);
                              // TODO_RUST_CODE
                              // Java: if (null != this.mFileFormat)
                                        // TODO_RUST_CODE
                              {
                                 // Java: return this.mFileFormat;
                                 // TODO_RUST_CODE
                              }
                              // Java: else
                                        // TODO_RUST_CODE
                              {
                                 // Java: GuessCsvMethod.guessCsv(this);
                                 // TODO_RUST_CODE
                                 // Java: if (null != this.mFileFormat)
                                            // TODO_RUST_CODE
                                 {
                                    // Java: return this.mFileFormat;
                                    // TODO_RUST_CODE
                                 }
                                 // Java: else
                                            // TODO_RUST_CODE
                                 {
                                    // Java: GuessRtfMethod.guessRtf(this);
                                    // TODO_RUST_CODE
                                    // Java: if (null != this.mFileFormat)
                                                // TODO_RUST_CODE
                                    {
                                       // Java: return this.mFileFormat;
                                       // TODO_RUST_CODE
                                    }
                                    // Java: else
                                                // TODO_RUST_CODE
                                    {
                                       // Java: GuessOfdMethod.guessOFD(this);
                                       // TODO_RUST_CODE
                                       // Java: if (null != this.mFileFormat)
                                                    // TODO_RUST_CODE
                                       {
                                          // Java: return this.mFileFormat;
                                          // TODO_RUST_CODE
                                       }
                                       // Java: else if (this.isEbook())
                                                    // TODO_RUST_CODE
                                       {
                                          // Java: this.mFileFormat = FileFormatEnum.EPUB;
                                          // TODO_RUST_CODE
                                          // Java: return this.mFileFormat;
                                          // TODO_RUST_CODE
                                       }
                                       // Java: else
                                                    // TODO_RUST_CODE
                                       {
                                          // Java: this._poifsFileSystem = GuessAllMethod.guessAll(this);
                                          // TODO_RUST_CODE
                                          // Java: Assert.assertNotNull("mFileFormat should not be null", this.mFileFormat);
                                          // TODO_RUST_CODE
                                          // Java: return this.mFileFormat;
                                          // TODO_RUST_CODE
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
            }
         }
      }
    }

    // Java: EncryptedType getEncryptedType()
    pub fn getEncryptedType(&self) -> EncryptedType {
      // Java: if (null != this.mEncryptedType)
        // TODO_RUST_CODE
      {
         // Java: return this.mEncryptedType;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Tools.newDocChecker(this);
         // TODO_RUST_CODE
         // Java: this.mEncryptedType = this.mDocChecker.isEncrypted();
         // TODO_RUST_CODE
         // Java: if (null != this.mEncryptedType)
            // TODO_RUST_CODE
         {
            // Java: return this.mEncryptedType;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: Tools.newOOXMLChecker(this);
            // TODO_RUST_CODE
            // Java: this.mEncryptedType = this.mOOXMLChecker.isEncrypted();
            // TODO_RUST_CODE
            // Java: if (null != this.mEncryptedType && EncryptedType.None != this.mEncryptedType)
                // TODO_RUST_CODE
            {
               // Java: return this.mEncryptedType;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: Tools.newUof3Checker(this);
               // TODO_RUST_CODE
               // Java: this.mEncryptedType = this.mUof3Checker.isEncrypted();
               // TODO_RUST_CODE
               // Java: return null != this.mEncryptedType ? this.mEncryptedType : (this.mEncryptedType = EncryptedType.None);
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: boolean isEbook()
    pub fn isEbook(&self) -> bool {
      // Java: if (this.mFile != null && this.mFile.getAbsolutePath() != null)
        // TODO_RUST_CODE
      {
         // Java: String path = this.mFile.getAbsolutePath().toLowerCase();
         // TODO_RUST_CODE
         // Java: return path.endsWith("epub");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isWriteReservation()
    pub fn isWriteReservation(&self) -> bool {
      // Java: return this.mDocChecker.isWriteReservation();
      // TODO_RUST_CODE
    }

    // Java: POIFSFileSystem getPOIFSFileSystem()
    pub fn getPOIFSFileSystem(&self) -> POIFSFileSystem {
      // Java: return this._poifsFileSystem;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isDoc()
    pub fn get_isDoc(&self) -> Boolean {
      // Java: return this.mIsDoc;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isDocx()
    pub fn get_isDocx(&self) -> Boolean {
      // Java: return this.mIsDocx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isDotx()
    pub fn get_isDotx(&self) -> Boolean {
      // Java: return this.mIsDotx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isDocm()
    pub fn get_isDocm(&self) -> Boolean {
      // Java: return this.mIsDocm;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isDotm()
    pub fn get_isDotm(&self) -> Boolean {
      // Java: return this.mIsDotm;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXls()
    pub fn get_isXls(&self) -> Boolean {
      // Java: return this.mIsXls;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXlsx()
    pub fn get_isXlsx(&self) -> Boolean {
      // Java: return this.mIsXlsx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXltx()
    pub fn get_isXltx(&self) -> Boolean {
      // Java: return this.mIsXltx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXlsm()
    pub fn get_isXlsm(&self) -> Boolean {
      // Java: return this.mIsXlsm;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXltm()
    pub fn get_isXltm(&self) -> Boolean {
      // Java: return this.mIsXltm;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXlsb()
    pub fn get_isXlsb(&self) -> Boolean {
      // Java: return this.mIsXlsb;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXlam()
    pub fn get_isXlam(&self) -> Boolean {
      // Java: return this.mIsXlam;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isPPT()
    pub fn get_isPPT(&self) -> Boolean {
      // Java: return this.mIsPpt;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isPptx()
    pub fn get_isPptx(&self) -> Boolean {
      // Java: return this.mIsPptx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isPpsx()
    pub fn get_isPpsx(&self) -> Boolean {
      // Java: return this.mIsPpsx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isPotx()
    pub fn get_isPotx(&self) -> Boolean {
      // Java: return this.mIsPotx;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isPotm()
    pub fn get_isPotm(&self) -> Boolean {
      // Java: return this.mIsPotm;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isMHT()
    pub fn get_isMHT(&self) -> Boolean {
      // Java: return this.mIsMht;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isHtml()
    pub fn get_isHtml(&self) -> Boolean {
      // Java: return this.mIsHtml;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isXml()
    pub fn get_isXml(&self) -> Boolean {
      // Java: return this.mIsXml;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isWordXml07()
    pub fn get_isWordXml07(&self) -> Boolean {
      // Java: return this.mIsWordXml07;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isPdf()
    pub fn get_isPdf(&self) -> Boolean {
      // Java: return this.mIsPdf;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isCsv()
    pub fn get_isCsv(&self) -> Boolean {
      // Java: return this.mIsCsv;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isRtf()
    pub fn get_isRtf(&self) -> Boolean {
      // Java: return this.mIsRtf;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isOdf()
    pub fn get_isOdf(&self) -> Boolean {
      // Java: return this.mIsOfd;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isUot3()
    pub fn get_isUot3(&self) -> Boolean {
      // Java: return this.mIsUot3;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isUos3()
    pub fn get_isUos3(&self) -> Boolean {
      // Java: return this.mIsUos3;
      // TODO_RUST_CODE
    }

    // Java: Boolean get_isUop3()
    pub fn get_isUop3(&self) -> Boolean {
      // Java: return this.mIsUop3;
      // TODO_RUST_CODE
    }

    // Java: FileFormatEnum get_fileFormat()
    pub fn get_fileFormat(&self) -> FileFormatEnum {
      // Java: return this.mFileFormat;
      // TODO_RUST_CODE
    }

    // Java: XlsChecker get_xlsChecker()
    pub fn get_xlsChecker(&self) -> XlsChecker {
      // Java: return this.mXlsChecker;
      // TODO_RUST_CODE
    }

    // Java: XlsxChecker get_xlsxChecker()
    pub fn get_xlsxChecker(&self) -> XlsxChecker {
      // Java: return this.mXlsxChecker;
      // TODO_RUST_CODE
    }

    // Java: XltxChecker get_xltxChecker()
    pub fn get_xltxChecker(&self) -> XltxChecker {
      // Java: return this.mXltxChecker;
      // TODO_RUST_CODE
    }

    // Java: XlsmChecker get_xlsmChecker()
    pub fn get_xlsmChecker(&self) -> XlsmChecker {
      // Java: return this.mXlsmChecker;
      // TODO_RUST_CODE
    }

    // Java: XltmChecker get_xltmChecker()
    pub fn get_xltmChecker(&self) -> XltmChecker {
      // Java: return this.mXltmChecker;
      // TODO_RUST_CODE
    }

    // Java: XlsbChecker get_xlsbChecker()
    pub fn get_xlsbChecker(&self) -> XlsbChecker {
      // Java: return this.mXlsbChecker;
      // TODO_RUST_CODE
    }

    // Java: XlamChecker get_xlamChecker()
    pub fn get_xlamChecker(&self) -> XlamChecker {
      // Java: return this.mXlamChecker;
      // TODO_RUST_CODE
    }

    // Java: DocChecker get_docChecker()
    pub fn get_docChecker(&self) -> DocChecker {
      // Java: return this.mDocChecker;
      // TODO_RUST_CODE
    }

    // Java: DocxChecker get_docxChecker()
    pub fn get_docxChecker(&self) -> DocxChecker {
      // Java: return this.mDocxChecker;
      // TODO_RUST_CODE
    }

    // Java: DotxChecker get_dotxChecker()
    pub fn get_dotxChecker(&self) -> DotxChecker {
      // Java: return this.mDotxChecker;
      // TODO_RUST_CODE
    }

    // Java: DocmChecker get_docmChecker()
    pub fn get_docmChecker(&self) -> DocmChecker {
      // Java: return this.mDocmChecker;
      // TODO_RUST_CODE
    }

    // Java: DotmChecker get_dotmChecker()
    pub fn get_dotmChecker(&self) -> DotmChecker {
      // Java: return this.mDotmChecker;
      // TODO_RUST_CODE
    }

    // Java: PPTChecker get_pptChecker()
    pub fn get_pptChecker(&self) -> PPTChecker {
      // Java: return this.mPptChecker;
      // TODO_RUST_CODE
    }

    // Java: PptxChecker get_pptxChecker()
    pub fn get_pptxChecker(&self) -> PptxChecker {
      // Java: return this.mPptxChecker;
      // TODO_RUST_CODE
    }

    // Java: PotxChecker get_potxChecker()
    pub fn get_potxChecker(&self) -> PotxChecker {
      // Java: return this.mPotxChecker;
      // TODO_RUST_CODE
    }

    // Java: PpsxChecker get_ppsxChecker()
    pub fn get_ppsxChecker(&self) -> PpsxChecker {
      // Java: return this.mPpsxChecker;
      // TODO_RUST_CODE
    }

    // Java: PotmChecker get_potmChecker()
    pub fn get_potmChecker(&self) -> PotmChecker {
      // Java: return this.mPotmChecker;
      // TODO_RUST_CODE
    }

    // Java: OOXMLChecker get_OOXMLChecker()
    pub fn get_OOXMLChecker(&self) -> OOXMLChecker {
      // Java: return this.mOOXMLChecker;
      // TODO_RUST_CODE
    }

    // Java: MHTChecker get_mhtChecker()
    pub fn get_mhtChecker(&self) -> MHTChecker {
      // Java: return this.mMhtChecker;
      // TODO_RUST_CODE
    }

    // Java: HtmlChecker get_htmlChecker()
    pub fn get_htmlChecker(&self) -> HtmlChecker {
      // Java: return this.mHtmlChecker;
      // TODO_RUST_CODE
    }

    // Java: XmlChecker get_xmlChecker()
    pub fn get_xmlChecker(&self) -> XmlChecker {
      // Java: return this.mXmlChecker;
      // TODO_RUST_CODE
    }

    // Java: WordXml07Checker get_wordXml07Checker()
    pub fn get_wordXml07Checker(&self) -> WordXml07Checker {
      // Java: return this.mWordXml07Checker;
      // TODO_RUST_CODE
    }

    // Java: PdfChecker get_pdfChecker()
    pub fn get_pdfChecker(&self) -> PdfChecker {
      // Java: return this.mPdfChecker;
      // TODO_RUST_CODE
    }

    // Java: CsvChecker get_csvChecker()
    pub fn get_csvChecker(&self) -> CsvChecker {
      // Java: return this.mCsvChecker;
      // TODO_RUST_CODE
    }

    // Java: RtfChecker get_rtfChecker()
    pub fn get_rtfChecker(&self) -> RtfChecker {
      // Java: return this.mRtfChecker;
      // TODO_RUST_CODE
    }

    // Java: Uof3Checker get_uof3Checker()
    pub fn get_uof3Checker(&self) -> Uof3Checker {
      // Java: return this.mUof3Checker;
      // TODO_RUST_CODE
    }

    // Java: Uot3Checker get_uot3Checker()
    pub fn get_uot3Checker(&self) -> Uot3Checker {
      // Java: return this.mUotChecker;
      // TODO_RUST_CODE
    }

    // Java: Uos3Checker get_uos3Checker()
    pub fn get_uos3Checker(&self) -> Uos3Checker {
      // Java: return this.mUosChecker;
      // TODO_RUST_CODE
    }

    // Java: Uop3Checker get_uop3Checker()
    pub fn get_uop3Checker(&self) -> Uop3Checker {
      // Java: return this.mUopChecker;
      // TODO_RUST_CODE
    }

    // Java: File getFile()
    pub fn getFile(&self) -> File {
      // Java: return null != this.mOOXMLDecrypted ? this.mOOXMLDecrypted : this.mFile;
      // TODO_RUST_CODE
    }

    // Java: RandomAccessFile getRaf()
    pub fn getRaf(&self) -> RandomAccessFile {
      // Java: return this.mRaf;
      // TODO_RUST_CODE
    }

    // Java: File get_OOXMLDecrypted()
    pub fn get_OOXMLDecrypted(&self) -> File {
      // Java: return this.mOOXMLDecrypted;
      // TODO_RUST_CODE
    }

    // Java: String get_UofDecrypted()
    pub fn get_UofDecrypted(&self) -> String {
      // Java: return this.mUofDecrypted;
      // TODO_RUST_CODE
    }

    // Java: void set_isDoc(boolean value)
    pub fn set_isDoc(&self, boolean value) {
      // Java: this.mIsDoc = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isDocx(boolean value)
    pub fn set_isDocx(&self, boolean value) {
      // Java: this.mIsDocx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isDotx(boolean value)
    pub fn set_isDotx(&self, boolean value) {
      // Java: this.mIsDotx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isDocm(boolean value)
    pub fn set_isDocm(&self, boolean value) {
      // Java: this.mIsDocm = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isDotm(boolean value)
    pub fn set_isDotm(&self, boolean value) {
      // Java: this.mIsDotm = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXls(boolean value)
    pub fn set_isXls(&self, boolean value) {
      // Java: this.mIsXls = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXlsx(boolean value)
    pub fn set_isXlsx(&self, boolean value) {
      // Java: this.mIsXlsx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXltx(boolean value)
    pub fn set_isXltx(&self, boolean value) {
      // Java: this.mIsXltx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXlsm(boolean value)
    pub fn set_isXlsm(&self, boolean value) {
      // Java: this.mIsXlsm = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXltm(boolean value)
    pub fn set_isXltm(&self, boolean value) {
      // Java: this.mIsXltm = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXlsb(boolean value)
    pub fn set_isXlsb(&self, boolean value) {
      // Java: this.mIsXlsb = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXlam(boolean value)
    pub fn set_isXlam(&self, boolean value) {
      // Java: this.mIsXlam = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isPPT(boolean value)
    pub fn set_isPPT(&self, boolean value) {
      // Java: this.mIsPpt = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isPptx(boolean value)
    pub fn set_isPptx(&self, boolean value) {
      // Java: this.mIsPptx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isPotx(boolean value)
    pub fn set_isPotx(&self, boolean value) {
      // Java: this.mIsPotx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isPpsx(boolean value)
    pub fn set_isPpsx(&self, boolean value) {
      // Java: this.mIsPpsx = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isPotm(boolean value)
    pub fn set_isPotm(&self, boolean value) {
      // Java: this.mIsPotm = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isMHT(boolean value)
    pub fn set_isMHT(&self, boolean value) {
      // Java: this.mIsMht = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isHtml(boolean value)
    pub fn set_isHtml(&self, boolean value) {
      // Java: this.mIsHtml = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isXml(boolean value)
    pub fn set_isXml(&self, boolean value) {
      // Java: this.mIsXml = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isWordXml03(boolean value)
    pub fn set_isWordXml03(&self, boolean value) {
      // Java: this.mIsWordXml03 = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isWordXml07(boolean value)
    pub fn set_isWordXml07(&self, boolean value) {
      // Java: this.mIsWordXml07 = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isPdf(boolean value)
    pub fn set_isPdf(&self, boolean value) {
      // Java: this.mIsPdf = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isCsv(boolean value)
    pub fn set_isCsv(&self, boolean value) {
      // Java: this.mIsCsv = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isRtf(boolean value)
    pub fn set_isRtf(&self, boolean value) {
      // Java: this.mIsRtf = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isOfd(boolean value)
    pub fn set_isOfd(&self, boolean value) {
      // Java: this.mIsOfd = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isUot3(boolean value)
    pub fn set_isUot3(&self, boolean value) {
      // Java: this.mIsUot3 = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isUos3(boolean value)
    pub fn set_isUos3(&self, boolean value) {
      // Java: this.mIsUos3 = value;
      // TODO_RUST_CODE
    }

    // Java: void set_isUop3(boolean value)
    pub fn set_isUop3(&self, boolean value) {
      // Java: this.mIsUop3 = value;
      // TODO_RUST_CODE
    }

    // Java: void set_fileFormat(FileFormatEnum value)
    pub fn set_fileFormat(&self, FileFormatEnum value) {
      // Java: this.mFileFormat = value;
      // TODO_RUST_CODE
    }

    // Java: void set_docChecker(DocChecker value)
    pub fn set_docChecker(&self, DocChecker value) {
      // Java: this.mDocChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_docxChecker(DocxChecker value)
    pub fn set_docxChecker(&self, DocxChecker value) {
      // Java: this.mDocxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_dotxChecker(DotxChecker value)
    pub fn set_dotxChecker(&self, DotxChecker value) {
      // Java: this.mDotxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_docmChecker(DocmChecker value)
    pub fn set_docmChecker(&self, DocmChecker value) {
      // Java: this.mDocmChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_dotmChecker(DotmChecker value)
    pub fn set_dotmChecker(&self, DotmChecker value) {
      // Java: this.mDotmChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xlsChecker(XlsChecker value)
    pub fn set_xlsChecker(&self, XlsChecker value) {
      // Java: this.mXlsChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xlsxChecker(XlsxChecker value)
    pub fn set_xlsxChecker(&self, XlsxChecker value) {
      // Java: this.mXlsxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xltxChecker(XltxChecker value)
    pub fn set_xltxChecker(&self, XltxChecker value) {
      // Java: this.mXltxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xlsmChecker(XlsmChecker value)
    pub fn set_xlsmChecker(&self, XlsmChecker value) {
      // Java: this.mXlsmChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xltmChecker(XltmChecker value)
    pub fn set_xltmChecker(&self, XltmChecker value) {
      // Java: this.mXltmChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xlsbChecker(XlsbChecker value)
    pub fn set_xlsbChecker(&self, XlsbChecker value) {
      // Java: this.mXlsbChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xlamChecker(XlamChecker value)
    pub fn set_xlamChecker(&self, XlamChecker value) {
      // Java: this.mXlamChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_pptChecker(PPTChecker value)
    pub fn set_pptChecker(&self, PPTChecker value) {
      // Java: this.mPptChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_pptxCheker(PptxChecker value)
    pub fn set_pptxCheker(&self, PptxChecker value) {
      // Java: this.mPptxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_potxCheker(PotxChecker value)
    pub fn set_potxCheker(&self, PotxChecker value) {
      // Java: this.mPotxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_ppsxCheker(PpsxChecker value)
    pub fn set_ppsxCheker(&self, PpsxChecker value) {
      // Java: this.mPpsxChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_potmCheker(PotmChecker value)
    pub fn set_potmCheker(&self, PotmChecker value) {
      // Java: this.mPotmChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_OOXMLChecker(OOXMLChecker value)
    pub fn set_OOXMLChecker(&self, OOXMLChecker value) {
      // Java: this.mOOXMLChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_mhtChecker(MHTChecker value)
    pub fn set_mhtChecker(&self, MHTChecker value) {
      // Java: this.mMhtChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_htmlChecker(HtmlChecker value)
    pub fn set_htmlChecker(&self, HtmlChecker value) {
      // Java: this.mHtmlChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_xmlChecker(XmlChecker value)
    pub fn set_xmlChecker(&self, XmlChecker value) {
      // Java: this.mXmlChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_WordXml07Checker(WordXml07Checker value)
    pub fn set_WordXml07Checker(&self, WordXml07Checker value) {
      // Java: this.mWordXml07Checker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_pdfChecker(PdfChecker value)
    pub fn set_pdfChecker(&self, PdfChecker value) {
      // Java: this.mPdfChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_csvChecker(CsvChecker value)
    pub fn set_csvChecker(&self, CsvChecker value) {
      // Java: this.mCsvChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_rtfChecker(RtfChecker value)
    pub fn set_rtfChecker(&self, RtfChecker value) {
      // Java: this.mRtfChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_uof3Checker(Uof3Checker value)
    pub fn set_uof3Checker(&self, Uof3Checker value) {
      // Java: this.mUof3Checker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_uot3Checker(Uot3Checker value)
    pub fn set_uot3Checker(&self, Uot3Checker value) {
      // Java: this.mUotChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_uos3Checker(Uos3Checker value)
    pub fn set_uos3Checker(&self, Uos3Checker value) {
      // Java: this.mUosChecker = value;
      // TODO_RUST_CODE
    }

    // Java: void set_uop3Checker(Uop3Checker value)
    pub fn set_uop3Checker(&self, Uop3Checker value) {
      // Java: this.mUopChecker = value;
      // TODO_RUST_CODE
    }

    // Java: File decryptOOXML(String password)
    pub fn decryptOOXML(&self, String password) -> File {
      // Java: Assert.assertNotNull("password should not be null!", password);
      // TODO_RUST_CODE
      // Java: CryptDocxFile cryptDocxFile = null != this.mRaf ? new CryptDocxFile(Tools.createRafStream(this.mRaf), password) : new CryptDocxFile(Tools.createFileInputStream(this.mFile), password);
      // TODO_RUST_CODE
      // Java: boolean isValid = cryptDocxFile.verifyPassword();
      // TODO_RUST_CODE
      // Java: if (!isValid)
        // TODO_RUST_CODE
      {
         // Java: throw new PasswordErrorException();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: InputStream stream = null;
         // TODO_RUST_CODE
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: stream = cryptDocxFile.read();
            // TODO_RUST_CODE
         }
         // Java: catch (IOException e)
            // TODO_RUST_CODE
         {
            // Java: Log.e(TAG, "IOException", e);
            // TODO_RUST_CODE
         }
         // Java: return null == stream ? null : this.serializeOOXML(stream);
         // TODO_RUST_CODE
      }
    }

    // Java: File serializeOOXML(InputStream stream)
    pub fn serializeOOXML(&self, InputStream stream) -> File {
      // Java: Assert.assertNotNull("stream should not be null!", stream);
      // TODO_RUST_CODE
      // Java: String tempFilePath = Platform.getTempDirectory() + ".decrypted.ooxml";
      // TODO_RUST_CODE
      // Java: FileUtil.newFolder(Platform.getTempDirectory());
      // TODO_RUST_CODE
      // Java: FileUtil.delFile(tempFilePath);
      // TODO_RUST_CODE
      // Java: FileOutputStream out = this.createOutputStream(tempFilePath);
      // TODO_RUST_CODE
      // Java: Assert.assertNotNull("out should not be null!", out);
      // TODO_RUST_CODE
      // Java: byte[] b = new byte[10240];
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: int len;
         // TODO_RUST_CODE
         // Java: while((len = stream.read(b)) > 0)
            // TODO_RUST_CODE
         {
            // Java: out.write(b, 0, len);
            // TODO_RUST_CODE
         }
         // Java: out.close();
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "IOException", e);
         // TODO_RUST_CODE
         // Java: FileUtil.delFile(tempFilePath);
         // TODO_RUST_CODE
      }
      // Java: return new File(tempFilePath);
      // TODO_RUST_CODE
    }

    // Java: FileOutputStream createOutputStream(String path)
    pub fn createOutputStream(&self, String path) -> FileOutputStream {
      // Java: Assert.assertNotNull("path should not be null!", path);
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: return new FileOutputStream(path);
         // TODO_RUST_CODE
      }
      // Java: catch (FileNotFoundException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "FileNotFoundException", e);
         // TODO_RUST_CODE
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: String decryptUof(String password)
    pub fn decryptUof(&self, String password) -> String {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: String decryptPath = null != this.mRaf ? EncryptPkgUtils.decrypt(this.mRaf, password) : EncryptPkgUtils.decrypt(this.mFile, password);
         // TODO_RUST_CODE
         // Java: return decryptPath;
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: e.printStackTrace();
         // TODO_RUST_CODE
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

}

impl IFileParser for FileParser {
}
