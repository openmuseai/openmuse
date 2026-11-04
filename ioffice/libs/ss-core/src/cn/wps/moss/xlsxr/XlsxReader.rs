use crate::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener;
use crate::cn::wps::moss::app::event::KmoBookOpenPasswordProvider::KmoBookOpenPasswordProvider;
// java package cn.wps.moss.xlsxr;
// rust: no-stmt
use crate::java_runtime::Rc;
use std::io::{self, Read};

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::app::KmoBooks::KmoBooks;
use crate::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use crate::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::XlsxrWorkbook;
use crate::cn::wps::moss::xlsxr::docProps::{XlsxrCustom::XlsxrCustom,XlsxrApp::XlsxrApp,XlsxrCore::XlsxrCore};

const OOXML_FILE_HEADER: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];

// java public final class XlsxReader implements BookReader
pub struct XlsxReader {
    // java private KmoBook mBook;
    mBook: Rc<KmoBook>,
    // java private String mFilePath;
    mFilePath: String,
    // java private boolean mBEncrypt;
    mBEncrypt: bool,
    // java private KmoBookOpenEventListener mOpenListener;
    mOpenListener: Option<Rc<dyn KmoBookOpenEventListener>>,
    // java private KmoBookOpenPasswordProvider mPassProvider;
    mPassProvider:Option<Rc<dyn KmoBookOpenPasswordProvider>>,
}

impl XlsxReader {
    // java public XlsxReader(KmoBook book, String path, KmoBookOpenEventListener openListener, boolean bEncrypt, KmoBookOpenPasswordProvider passProvider) {
    pub fn XlsxReader(book: Rc<KmoBook>, path: &str, openListener:Option<Rc<dyn KmoBookOpenEventListener>>,bEncrypt: bool,passProvider:Option<Rc<dyn KmoBookOpenPasswordProvider>>) -> Self {
        // java this.mFilePath = path;
        let mFilePath = path.to_string();
        // java this.init(book, bEncrypt, openListener, passProvider);
        // Native allocation before Java's init assignments.
        let mut reader=Self{mBook:book.clone(),mFilePath,mBEncrypt:false,mOpenListener:None,mPassProvider:None};
        reader.init(book,bEncrypt,openListener,passProvider);reader
    }

    // java public XlsxReader(KmoBook book, RandomAccessFile raf, KmoBookOpenEventListener openListener, boolean bEncrypt, KmoBookOpenPasswordProvider passProvider) {
    pub fn XlsxReader__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mRaf = raf;
        // java this.init(book, bEncrypt, openListener, passProvider);
         */
    }

    // java private void init(KmoBook book, boolean bEncrypt, KmoBookOpenEventListener openListener, KmoBookOpenPasswordProvider passProvider) {
    fn init(&mut self,book: Rc<KmoBook>, bEncrypt: bool,openListener:Option<Rc<dyn KmoBookOpenEventListener>>,passProvider:Option<Rc<dyn KmoBookOpenPasswordProvider>>) {
        // java this.mBook = book;
        self.mBook = book;
        // java this.mBEncrypt = bEncrypt;
        self.mBEncrypt = bEncrypt;
        // java this.mOpenListener = openListener;
        self.mOpenListener=openListener;
        // java this.mPassProvider = passProvider;
        self.mPassProvider=passProvider;
    }

    // java public void open() throws IOException {
    pub fn open(&self) -> io::Result<()> {
        // java synchronized(XlsxReadWriteLock.sLock) {
        // rust: no-stmt — 单线程打开，没有第二把锁。
        // java OxfXlRelations.setXlsRelation();
        kfc_kfc_alpha::kfc::ooxfr::OxfXlRelations::OxfXlRelations::setXlsRelation();
        // java if (null != this.mRaf) { this.mPackage = new OxfrPackage(this.mRaf); this.mBook.getOleMgr().setRaf(this.mRaf); } if (repairLog != null) { this.mBook.getRecoveryLogs().addLog(1, repairLog); } if (!XlsxrPackage.isXlsb(bookPart)) { XlsxrPackage.initLargestFile(this.mFilePath, this.mRaf, this.mPackage); this.mBook.setBookReader(this); } catch (WillOutOfMemoryException var10) { XlsxrPackage.setLargestFile(); return; } finally { if (!XlsxrPackage.isLargestFile()) { this.mBook.setBookReader((BookReader)null); this.close(); } } this.mBook.setFileFormat(10); xbReader.open();
        // rust: no-stmt — 本入口用文件路径，没有 RandomAccessFile。大文件与 OOM 分支不改变 M1 夹具。
        // java if (this.mBEncrypt) {
        if self.mBEncrypt {
            // java this.mFilePath = XlsxrFileDecrypt.decryptFile(this.mFilePath, this.mRaf, this.mBook, this.mPassProvider);
            // rust: no-stmt — 解密类不在本里程碑。
            // java this.mRaf = null;
            return Err(io::Error::other("EncryptFileException"));
        }
        // java this.mPackage = new OxfrPackage(this.mFilePath);
        let mut pack = OxfrPackage::OxfrPackage(&self.mFilePath)
            .map_err(|err| io::Error::other(format!("FileDamagedException: {err}")))?;
        // java this.mBook.getOleMgr().setPath(this.mFilePath);
        // rust: no-stmt — OLE 管理器不参与网格。登记 §1.7.1。
        // java this.mBook.getOleMgr().initXlsxDocument(this.mPackage);
        // rust: no-stmt
        // java String repairLog = this.mPackage.getRepairLog();
        // rust: no-stmt
        // java OpcrPackagePart bookPart = this.mPackage.getOfficeDocument();
        let owned = {
            let bookPart = pack.getOfficeDocument_mut();
            // java if (bookPart == null) {
            let Some(bookPart) = bookPart else {
                // java throw new FileDamagedException("It is not a OOXML file.");
                return Err(io::Error::other(
                    "FileDamagedException: It is not a OOXML file.",
                ));
            };
            // java if (!bookPart.getPartName().contains("workbook")) {
            if !bookPart.getPartName().contains("workbook") {
                // java throw new SuffixErrorException("It is not a Excel file");
                return Err(io::Error::other(
                    "SuffixErrorException: It is not a Excel file",
                ));
            }
            // java if (!XlsxrPackage.isXlsb(bookPart)) {
            if XlsxrPackage::isXlsb(bookPart) {
                // java XlsbrBook xbReader = new XlsbrBook(this.mBook, bookPart, this.mOpenListener, this.mPassProvider);
                let _ = stringify!(xbReader);
                // rust: no-stmt
                // java this.mBook.setFileFormat(10);
                // rust: no-stmt
                // java xbReader.open();
                return Err(io::Error::other("XlsbException: It is a .xlsb file."));
            }
            // java XlsxrPackage.initLargestFile(this.mFilePath, this.mRaf, this.mPackage);
            // rust: no-stmt — M1 夹具远小于大文件门槛，bLargestFile 保持 false。登记 §1.7.1。
            // java this.mBook.setBookReader(this);
            // rust: no-stmt — open 结束前 Java 会在非大文件时清掉 reader。登记 §1.7.1。
            // Native immutable ZIP handle releases the package borrow before the
            // original property readers below. No workbook parsing happens here.
            let zip = bookPart.getZipFile();
            let entry = bookPart.getZipEntry()?;
            OpcrPackagePart::OpcrPackagePart(&zip, &entry)
        };
        // java XlsxrCustom.open(this.mBook, this.mPackage.getCustomPart());
        XlsxrCustom::open(Rc::clone(&self.mBook),pack.getCustomPart().map(|p|&*p))?;
        // java XlsxrApp.open(this.mBook, this.mPackage.getAppPart());
        XlsxrApp::open(&self.mBook,pack.getAppPart())?;
        // java this.mBookReader = new XlsxrWorkbook(this.mBook, bookPart, this.mOpenListener, this.mPassProvider);
        let reader = XlsxrWorkbook::XlsxrWorkbook(Rc::clone(&self.mBook),Some(owned),self.mOpenListener.clone(),self.mPassProvider.clone());
        // java this.mBookReader.open();
        XlsxrWorkbook::open(&reader)?;
        // java XlsxrCore.open(this.mBook, this.mPackage.getCorePart());
        XlsxrCore::open(&self.mBook,pack.getCorePart())?;
        // java XlsxrContentTypes.open(this.mBook, this.mPackage);
        // rust: no-stmt
        // java if (!XlsxrPackage.isLargestFile()) {
        if !XlsxrPackage::isLargestFile() {
            // java this.mOpenListener.bookDidFinishReadingEverything();
            if let Some(listener) = &self.mOpenListener {
                listener.bookDidFinishReadingEverything();
            }
        }
        // java ThemeReader.addDefaultTheme(this.mBook);
        // rust: no-stmt — 主题不参与原始格。登记 §1.7.1。
        // java this.mBook.setBookReader((BookReader)null);
        // rust: no-stmt
        // java this.close();
        // rust: no-stmt — 包在本函数结束时释放。
        let _ = pack;
        Ok(())
    }

    // java public boolean isLargestFile() {
    pub fn isLargestFile(&self) -> bool {
        // java return this.mBookReader == null ? false : XlsxrPackage.isLargestFile();
        XlsxrPackage::isLargestFile()
    }

    // java public void currentGridRange(KmoSheet sheet, int beginRow, int endRow) {
    pub fn currentGridRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.mBookReader != null) {
        // java this.mBookReader.currentGridRange(sheet, beginRow, endRow);
         */
    }

    // java public void close() throws IOException {
    pub fn close(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mBook.getOleMgr().resetXlsxDocument();
        // java if (this.mBookReader != null) {
        // java this.mBookReader = null;
        // java if (this.mPackage != null) {
        // java this.mPackage.close();
        // java this.mPackage = null;
        // java XlsxrFileCache.clearCache();
         */
    }

    // java public static boolean isOOXMLFileFormat(String fileName) throws IOException {
    pub fn isOOXMLFileFormat(fileName: &str) -> io::Result<bool> {
        // java File f = new File(fileName);
        let f = std::path::Path::new(fileName);
        // java if (f.exists() && f.length() >= 4L) {
        if f.exists() && f.metadata().map(|m| m.len()).unwrap_or(0) >= 4 {
            // java FileInputStream stream = null; boolean var4; return var4; if (stream != null) { stream.close(); }
            let _ = stringify!(stream);
            // java stream = new FileInputStream(fileName);
            let mut stream = std::fs::File::open(fileName)?;
            // java byte[] header = new byte[4];
            let mut header = [0u8; 4];
            // java stream.read(header);
            let _ = stream.read(&mut header)?;
            // java var4 = isOOXMLFileFormat(header);
            let var4 = Self::isOOXMLFileFormat__header(&header);
            Ok(var4)
        } else {
            // java return false;
            Ok(false)
        }
    }

    // java public static boolean isOOXMLFileFormat(byte[] header) {
    pub fn isOOXMLFileFormat__header(header: &[u8]) -> bool {
        // java if (header.length < 4) {
        if header.len() < 4 {
            // java return false;
            return false;
        }
        // java byte[] ooxml_header = POIFSConstants.OOXML_FILE_HEADER;
        let ooxml_header = OOXML_FILE_HEADER;
        // java return header[0] == ooxml_header[0] && header[1] == ooxml_header[1] && header[2] == ooxml_header[2] && header[3] == ooxml_header[3];
        header[0] == ooxml_header[0]
            && header[1] == ooxml_header[1]
            && header[2] == ooxml_header[2]
            && header[3] == ooxml_header[3]
    }

    // java public static BookReader openBook(KmoBook book, String path, boolean bOOXMLEncrypt, KmoBookOpenEventListener listener, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook(
        book: &Rc<KmoBook>,
        path: &str,
        bOOXMLEncrypt: bool,
        listener: Option<Rc<dyn KmoBookOpenEventListener>>,
        passProvider:Option<Rc<dyn KmoBookOpenPasswordProvider>>,
    ) -> io::Result<()> {
        // java KmoApp.sharedApp().books().loadTemplates(book);
        KmoBooks::loadTemplates(book);
        // java XlsxReader bookReader = new XlsxReader(book, path, listener, bOOXMLEncrypt, passProvider);
        let bookReader = Self::XlsxReader(Rc::clone(book), path,listener,bOOXMLEncrypt,passProvider);
        // java book.setFileFormat(1);
        book.setFileFormat(1);
        // java bookReader.open();
        bookReader.open()?;
        // java updateFormat(book);
        Self::updateFormat(book);
        // java return bookReader;
        Ok(())
    }

    // java public static BookReader openBook(KmoBook book, RandomAccessFile raf, boolean bOOXMLEncrypt, KmoBookOpenEventListener listener, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoApp.sharedApp().books().loadTemplates(book);
        // java XlsxReader bookReader = new XlsxReader(book, raf, listener, bOOXMLEncrypt, passProvider);
        // java book.setFileFormat(1);
        // java bookReader.open();
        // java updateFormat(book);
        // java return bookReader;
         */
    }

    // java private static void updateFormat(KmoBook book) {
    fn updateFormat(book: &KmoBook) {
        // java if (book.isMacroEnabled()) { if (book.isTemplate()) { book.setFileFormat(9); } else { book.setFileFormat(8); } }
        // rust: no-stmt — isMacroEnabled 仍停车。M1 夹具不是宏工作簿，格式保持 1。登记 §1.7.1。
        let _ = book;
    }
}
