use crate::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener;
// java package cn.wps.moss.app;
// rust: no-stmt
use crate::java_runtime::Rc;
use std::io;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::io::BookReaderHelper::BookReaderHelper;

// java public class KmoBooksApi extends KmoBooks
pub struct KmoBooksApi{pub(super) base:super::KmoBooks::KmoBooks}
impl std::ops::Deref for KmoBooksApi{type Target=super::KmoBooks::KmoBooks;fn deref(&self)->&Self::Target{&self.base}}

impl KmoBooksApi {
    pub fn new()->Self{Self{base:super::KmoBooks::KmoBooks::new()}}
    // java public static boolean OPEN_RECALC = true;
    const OPEN_RECALC: bool = true;

    // java public KmoBook appendBook() {
    pub fn appendBook(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoBook book = new KmoBookApi(this, false);
        // java this.books.add(book);
        // java return book;
         */
    }

    // java public void openBook(KmoBook book, String filePath, POIFSFileSystem poifs, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook(book: &Rc<KmoBook>, filePath: &str) -> io::Result<()> {
        Self::openBook__book_listener(book, filePath, None)
    }

    // java this.bookOpenListenerGroup is forwarded into BookReaderHelper before helper_loadFile.
    pub fn openBook__book_listener(
        book: &Rc<KmoBook>,
        filePath: &str,
        listener: Option<Rc<dyn KmoBookOpenEventListener>>,
    ) -> io::Result<()> {
        // java book.setReadOnly(true);
        book.setReadOnly(true);
        // java book.setFilePath(filePath);
        book.setFilePath(filePath);
        // java book.startMonitor();
        // rust: no-stmt — 日志观察器没有类文件。登记 §1.7.1。
        // java book.setIsIOState(true);
        book.setIsIOState(true);
        // java ExpFmlManager.enableCache();
        // rust: no-stmt — 公式缓存管理器不在本里程碑。登记 §1.7.1。
        // java BookReaderHelper.openBook(book, (String)filePath, poifs, this.bookOpenListenerGroup, passProvider);
        let opened = BookReaderHelper::openBook__listener(book, filePath, listener);
        if opened.is_ok() {
            // java this.helper_loadFile(book);
            Self::helper_loadFile(book);
        }
        // java ExpFmlManager.disableCache();
        // rust: no-stmt
        // java book.setIsIOState(false);
        if opened.is_ok() {
            book.setIsIOState(false);
        }
        opened
    }

    // java public void openBook(KmoBook book, String filePath, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook__path(book: &Rc<KmoBook>, filePath: &str) -> io::Result<()> {
        // java this.openBook(book, (String)filePath, (POIFSFileSystem)null, passProvider);
        Self::openBook(book, filePath)
    }

    // java public void openBook(KmoBook book, RandomAccessFile raf, POIFSFileSystem poifs, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn openBook__raf() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java book.setReadOnly(true);
        // java book.setRaf(raf);
        // java book.startMonitor();
        // java book.setIsIOState(true);
        // java ExpFmlManager.enableCache();
        // java BookReaderHelper.openBook(book, (RandomAccessFile)raf, poifs, this.bookOpenListenerGroup, passProvider);
        // java this.helper_loadFile(book);
        // java ExpFmlManager.disableCache();
        // java book.setIsIOState(false);
         */
    }

    // java public void restoreBook(KmoBook book, String realFilePath, String pretendFilePath, KmoBookOpenPasswordProvider passProvider) throws IOException {
    pub fn restoreBook() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java book.setReadOnly(true);
        // java book.setFilePath(pretendFilePath);
        // java BookReaderHelper.openBook(book, (String)realFilePath, (POIFSFileSystem)null, this.bookOpenListenerGroup, passProvider);
        // java this.helper_loadFile(book);
        // java book.setDirty(true);
         */
    }

    // java public KmoBook newBook() throws IOException {
    pub fn newBook() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoBook book = new KmoBookApi(this, true);
        // java book.setIsNewBook(true);
        // java this.books.add(book);
        // java this.loadTemplates(book);
        // java return book;
         */
    }

    // java public KmoBook openBook(String filePath, POIFSFileSystem poifs, KmoBookOpenEventListener listener) throws IOException {
    pub fn openBook__listener() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.bookOpenListenerGroup.add(listener);
        // java KmoBook book = this.appendBook();
        // java this.openBook((KmoBook)book, (String)filePath, (KmoBookOpenPasswordProvider)null);
        // java return book;
         */
    }

    // java public KmoBook openBook(String filePath, KmoBookOpenEventListener listener) throws IOException {
    pub fn openBook__listener_path() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.openBook((String)filePath, (POIFSFileSystem)null, (KmoBookOpenEventListener)listener);
         */
    }

    // java public KmoBook openBook(InputStream in, KmoBookOpenEventListener listener) throws IOException {
    pub fn openBook__stream() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoBook book = new KmoBookApi(this, false);
        // java book.setReadOnly(true);
        // java this.books.add(book);
        // java BookReader reader = new XlsReader(book, in, listener, (KmoBookOpenPasswordProvider)null);
        // java reader.open();
        // java this.helper_loadFile(book);
        // java return book;
         */
    }

    // java private void helper_loadFile(final KmoBook book) {
    fn helper_loadFile(book: &Rc<KmoBook>) {
        // java BookReader reader = book.getBookReader(); boolean bLargestFile = reader != null && reader.isLargestFile();
        // XlsxReader has cleared the bookReader at this point.
        let bLargestFile = false;
        // java book.setCompletelyReaded();
        book.setCompletelyReaded();
        // java if (!bLargestFile && !book.isOOM())
        if !bLargestFile && !book.isOOM() {
            if ![4, 5, 3].contains(&book.getFileFormat()) {
                // java for each sheet: count rawFormulas; matrix.startCmdMode(); sheet.delayedFormulaInit();
                for i in 0..book.sheetCount() {
                    let sheet = book.sheetAt(i);
                    let sheet = sheet.borrow();
                    sheet.getGrid().getMatrix().borrow_mut().startCmdMode();
                    // XLSX rawFormulas is null; CellFmlBackupHandler already installed the real formulas.
                    if book.getFileFormat() == 0 {
                        sheet.delayedFormulaInit();
                    }
                }
                // java CalculationChain cc = book.getCalculationChain(); if (cc != null) { init/fastInit; setCalcChainInitialized(); }
                if let Some(cc) = book.getCalculationChain() {
                    assert_ne!(book.getFileFormat(), 0, "fastInit requires the XLS reader");
                    cc.init();
                    book.setCalcChainInitialized();
                }
                // java cc = book.getCalculationChain(); book.setReadOnly(cc == null || cc.isRepealed());
                book.setReadOnly(book.getCalculationChain().is_none_or(|cc| cc.isRepealed()));
            } else {
                book.setReadOnly(false);
            }
        }
        // java book.setDirty(false);
        book.setDirty(false);
        for i in 0..book.sheetCount() {
            let sheet = book.sheetAt(i);
            let sheet = sheet.borrow();
            // java changeListenerGroup().sheetContentsDidFinishChanging(); matrix.endCmdMode();
            sheet.changeListenerGroup().sheetContentsDidFinishChanging();
            sheet.getGrid().getMatrix().borrow_mut().endCmdMode();
        }
        // java if (OPEN_RECALC && !book.isOOM() && !bLargestFile && book.isAutoCalcOn())
        if Self::OPEN_RECALC && !book.isOOM() && !bLargestFile && book.isAutoCalcOn() {
            let book = book.clone();
            // java new Thread(r); setDaemon(true); setPriority(1); start();
            // Rust detached threads retain the same book until run completes. OS
            // scheduling priority is a host concern, not a timing barrier or sleep.
            std::thread::Builder::new()
                .name("KmoBooksApi.openCalc".into())
                .spawn(move || {
                    if let Some(cc) = book.getCalculationChain() {
                        // java cc.forceMarkRecalculation(-1,-1,-1); try { cc.openCalc(); } catch(CircleReferenceException/RuntimeException) {}
                        cc.forceMarkRecalculation(-1, -1, -1);
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            cc.openCalc()
                        }));
                        for i in 0..book.sheetCount() {
                            book.sheetAt(i)
                                .borrow()
                                .changeListenerGroup()
                                .sheetContentsDidFinishChanging();
                        }
                    }
                })
                .expect("openCalc thread creation failed");
        }
    }

    // java public void run() {
    pub fn run() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java CalculationChain cc = book.getCalculationChain();
        // java if (cc != null) {
        // java cc.forceMarkRecalculation(-1, -1, -1);
        // java cc.openCalc();
        // java book.sheetAt(i).changeListenerGroup().sheetContentsDidFinishChanging();
         */
    }
}
