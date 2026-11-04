use crate::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener;
// java package cn.wps.moss.io;
// rust: no-stmt
use crate::java_runtime::Rc;
use std::io::{self, Read};

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::xlsxr::XlsxReader::XlsxReader;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java public final class BookReaderHelper
pub struct BookReaderHelper;

impl BookReaderHelper {
    // java public static void openBook(KmoBook book, String filePath, POIFSFileSystem poifs, KmoBookOpenEventListener listener, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook(book: &Rc<KmoBook>, filePath: &str) -> io::Result<()> {
        Self::openBook__listener(book, filePath, None)
    }

    // java retain KmoBookOpenEventListener listener through openBookImpl.
    pub fn openBook__listener(
        book: &Rc<KmoBook>,
        filePath: &str,
        listener: Option<Rc<dyn KmoBookOpenEventListener>>,
    ) -> io::Result<()> {
        // java File f = new File(filePath);
        let f = std::path::Path::new(filePath);
        // java if (!f.exists()) {
        if !f.exists() {
            // java throw new FileNotExistException(filePath + " is not exist.");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("FileNotExistException: {filePath} is not exist."),
            ));
        }
        // java int fileFormat = getFileFormatImpl(filePath);
        let fileFormat = Self::getFileFormatImpl(filePath)?;
        // java openBookImpl(book, filePath, poifs, listener, passProvider, fileFormat);
        // java catch (IOException e) { if (NoSpaceLeftException.isNoSpaceLeftException(e)) { throw new NoSpaceLeftException(e); } else { throw e; }
        Self::openBookImpl(book, filePath, fileFormat, listener)
    }

    // java public static void openBook(KmoBook book, RandomAccessFile raf, POIFSFileSystem poifs, KmoBookOpenEventListener listener, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == raf) {
        // java throw new FileNotExistException("RandomAccessFile is invalid.");
        // java int fileFormat = getFileFormatImpl(raf);
        // java openBookImpl(book, raf, poifs, listener, passProvider, fileFormat);
        // java catch (IOException e) { if (NoSpaceLeftException.isNoSpaceLeftException(e)) { throw new NoSpaceLeftException(e); } else { throw e; }
         */
    }

    // java private static BookReader openBookImpl(KmoBook book, RandomAccessFile raf, POIFSFileSystem poifs, KmoBookOpenEventListener listener, KmoBookOpenPasswordProvider passProvider, int fileFormat) throws IOException {
    fn openBookImpl__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean bOOXMLEncrypt = false;
        // java initBookVersion(book, fileFormat);
        // java ThemeReader.addDefaultTheme(book);
        // java if (fileFormat == 0) {
        // java return XlsReader.openBook(book, raf, poifs, listener, passProvider);
        // java catch (OOXMLDecryptNotSupportException var10) {
        // java fileFormat = 1;
        // java initBookVersion(book, fileFormat);
        // java bOOXMLEncrypt = true;
        // java catch (RecordInputStream.LeftoverDataException e) {
        // java throw new FileDamagedException(e);
        // java catch (RecordFormatException e) {
        // java throw new FileDamagedException(e);
        // java if (fileFormat == 1) {
        // java return XlsxReader.openBook(book, raf, bOOXMLEncrypt, listener, passProvider);
        // java catch (XlsbException var9) {
        // java fileFormat = 10;
        // java switch (fileFormat) {
        // java case 2:
        // java return SXmlReader.openBook(book, raf, listener);
        // java case 3:
        // java case 5:
        // java case 6:
        // java case 7:
        // java case 8:
        // java case 9:
        // java case 11:
        // java default:
        // java return HtmlReader.openBook(book, raf, listener);
        // java catch (NotExcelHtmlException var8) {
        // java return PlainTextReader.openBook(book, raf, listener);
        // java case 4:
        // java return CSVReader.openBook(book, raf, listener);
        // java case 10:
        // java return XlsbReader.openBook(book, raf, bOOXMLEncrypt, listener, passProvider);
        // java case 12:
        // java return XlsReaderBiff4.openBook(book, raf, listener, passProvider);
        // java case 13:
        // java return XlsReaderBiff3.openBook(book, raf, listener, passProvider);
        // java case 14:
        // java return XlsReaderBiff2.openBook(book, raf, listener, passProvider);
         */
    }

    // java private static BookReader openBookImpl(KmoBook book, String filePath, POIFSFileSystem poifs, KmoBookOpenEventListener listener, KmoBookOpenPasswordProvider passProvider, int fileFormat) throws IOException {
    fn openBookImpl(
        book: &Rc<KmoBook>,
        filePath: &str,
        fileFormat: i32,
        listener: Option<Rc<dyn KmoBookOpenEventListener>>,
    ) -> io::Result<()> {
        // java boolean bOOXMLEncrypt = false;
        let bOOXMLEncrypt = false;
        // java initBookVersion(book, fileFormat);
        Self::initBookVersion(book, fileFormat);
        // java ThemeReader.addDefaultTheme(book);
        // rust: no-stmt — 主题不参与原始格。登记 §1.7.1。
        // java if (fileFormat == 0) {
        if fileFormat == 0 {
            // java return XlsReader.openBook(book, filePath, poifs, listener, passProvider);
            // rust: no-stmt — xls 阅读器不在本里程碑。
            return Err(io::Error::other(
                "FileDamagedException: xls reader is not in this milestone",
            ));
        }
        // java catch (OOXMLDecryptNotSupportException var10) { fileFormat = 1; initBookVersion(book, fileFormat); bOOXMLEncrypt = true; } catch (RecordInputStream.LeftoverDataException e) { throw new FileDamagedException(e); } catch (RecordFormatException e) { throw new FileDamagedException(e); }
        // rust: no-stmt — 路径入口先按文件头分流，xls 头不会进到解密失败后再改成 xlsx。
        // java if (fileFormat == 1) {
        if fileFormat == 1 {
            // java return XlsxReader.openBook(book, filePath, bOOXMLEncrypt, listener, passProvider);
            return XlsxReader::openBook(book, filePath, bOOXMLEncrypt, listener,None);
        }
        // java catch (XlsbException var9) { fileFormat = 10; }
        // rust: no-stmt — XlsxReader.openBook 自己把 xlsb 收成错误。
        // java switch (fileFormat) { case 2: return SXmlReader.openBook(book, filePath, listener, passProvider); case 3: case 5: case 6: case 7: case 8: case 9: case 11: default: return HtmlReader.openBook(book, filePath, listener, passProvider); catch (NotExcelHtmlException var8) { return PlainTextReader.openBook(book, filePath, listener); } case 4: return CSVReader.openBook(book, filePath, listener); case 10: return XlsbReader.openBook(book, filePath, bOOXMLEncrypt, listener, passProvider); case 12: return XlsReaderBiff4.openBook(book, filePath, listener, passProvider); case 13: return XlsReaderBiff3.openBook(book, filePath, listener, passProvider); case 14: return XlsReaderBiff2.openBook(book, filePath, listener, passProvider); }
        // rust: no-stmt — html / 纯文本阅读器不在本里程碑。KmoBookApi 收到异常后返回 2。
        Err(io::Error::other(
            "FileDamagedException: not an xlsx workbook",
        ))
    }

    // java public static int getFileFormat(String filePath) throws IOException {
    pub fn getFileFormat(filePath: &str) -> io::Result<i32> {
        // java File f = new File(filePath);
        let f = std::path::Path::new(filePath);
        // java if (!f.exists()) {
        if !f.exists() {
            // java throw new FileNotExistException(filePath + " is not exist.");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("FileNotExistException: {filePath} is not exist."),
            ));
        }
        // java return getFileFormatImpl(filePath);
        Self::getFileFormatImpl(filePath)
    }

    // java private static int getFileFormatImpl(String filePath) throws IOException {
    fn getFileFormatImpl(filePath: &str) -> io::Result<i32> {
        // java byte[] header = new byte[16];
        let mut header = [0u8; 16];
        // java readFileHeader(filePath, header);
        Self::readFileHeader(filePath, &mut header)?;
        // java return getFileFormatByHead(header, filePath, (RandomAccessFile)null);
        Ok(Self::getFileFormatByHead(&header, filePath))
    }

    // java private static int getFileFormatImpl(RandomAccessFile raf) throws IOException {
    fn getFileFormatImpl__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java byte[] header = new byte[16];
        // java readFileHeader(raf, header);
        // java return getFileFormatByHead(header, (String)null, raf);
         */
    }

    // java private static int getFileFormatByHead(byte[] header, String filePath, RandomAccessFile raf) {
    fn getFileFormatByHead(header: &[u8], filePath: &str) -> i32 {
        // java int format;
        let format;
        // java if (XlsReader.isBinaryFileFormat(header)) { format = 0; } else if (XlsReaderBiff4.isBiff4(header)) { format = 12; } else if (XlsReaderBiff3.isBiff3(header)) { format = 13; } else if (XlsReaderBiff2.isBiff2(header)) { format = 14; } else if (CSVReader.isCSVFileFormat(null != raf ? raf.getFileName() : filePath)) { format = 4; } else if (SXmlReader.isSXMLFileFormat(filePath, raf)) { format = 2; }
        // rust: no-stmt — xls / biff / csv / sxml 头判别不在本里程碑。zip 头走下面的 OOXML 分支。
        // java } else if (XlsxReader.isOOXMLFileFormat(header)) {
        if XlsxReader::isOOXMLFileFormat__header(header) {
            // java format = 1;
            format = 1;
        } else {
            let _ = filePath;
            // java format = 5;
            format = 5;
        }
        // java return format;
        format
    }

    // java private static void initBookVersion(KmoBook book, int fileFormat) {
    fn initBookVersion(book: &KmoBook, fileFormat: i32) {
        // java SpreadsheetVersion version;
        let version;
        // java switch (fileFormat) {
        match fileFormat {
            // java case 1:
            // java case 2:
            // java case 3:
            // java case 4:
            1 | 2 | 3 | 4 => {
                // java version = SpreadsheetVersion.EXCEL2007;
                version = SpreadsheetVersion::EXCEL2007;
                // java break;
            }
            // java default:
            _ => {
                // java version = SpreadsheetVersion.EXCEL97;
                version = SpreadsheetVersion::EXCEL97;
                // java break;
            }
        }
        // java book.setSpreadsheetVersion(version);
        book.setSpreadsheetVersion(version);
        // java SpreadsheetVersion.CUR_VERSION = version;
        if let Ok(mut cur) = crate::org::apache::poi::ss::SpreadsheetVersion::CUR_VERSION.lock() {
            *cur = version;
        }
    }

    // java private static void readFileHeader(String filepath, byte[] header) throws IOException {
    fn readFileHeader(filepath: &str, header: &mut [u8]) -> io::Result<()> {
        // java FileInputStream stream = null; if (stream != null) { stream.close(); }
        let _ = stringify!(stream);
        // java stream = new FileInputStream(filepath);
        let mut stream = std::fs::File::open(filepath)?;
        // java stream.read(header);
        let _ = stream.read(header)?;
        // java stream.close();
        Ok(())
    }

    // java private static void readFileHeader(RandomAccessFile raf, byte[] header) throws IOException {
    fn readFileHeader__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java raf.seek(0L);
        // java raf.read(header);
        // java raf.seek(0L);
         */
    }
}
