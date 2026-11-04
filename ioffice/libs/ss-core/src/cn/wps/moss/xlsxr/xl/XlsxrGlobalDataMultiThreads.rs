use super::{
    XlsxrSharedStrings::XlsxrSharedStrings, XlsxrStyles::XlsxrStyles, XlsxrWorkbook::XlsxrWorkbook,
};
use crate::java_runtime::Rc;
use std::{
    io,
    panic::{catch_unwind, AssertUnwindSafe},
};
// java public final class XlsxrGlobalDataMultiThreads
pub struct XlsxrGlobalDataMultiThreads;
impl XlsxrGlobalDataMultiThreads {
    // java public void open() throws IOException {
    // java OpcrRelationships relses = this.mBookPart.getRelationships();
    // java if (relses != null && relses.size() != 0) {
    // java OpcrRelationship themeRels = relses.getRelationshipByType(OxfXlRelations.theme().getRelsType());
    // java if (themeRels != null) {
    // java XlsxrTheme themeReader = new XlsxrTheme(this.mBook.theme(), this.mBook, themeRels);
    // java themeReader.open();
    // java }
    // java
    // java ThreadWaiter waiter = new ThreadWaiter();
    // java waiter.addWaiter(3);
    // java WorkbookThread wbThread = new WorkbookThread(waiter);
    // java wbThread.start();
    // java StringsThread ssThread = new StringsThread(waiter);
    // java ssThread.start();
    // java StylesThread stThread = new StylesThread(waiter);
    // java stThread.start();
    // java synchronized(waiter) {
    // java try {
    // java if (waiter.needWait()) {
    // java waiter.wait();
    // java }
    // java } catch (InterruptedException var10) {
    // java }
    // java }
    // java
    // java if (wbThread.getException() != null) {
    // java throw wbThread.getException();
    // java } else if (wbThread.getIOException() != null) {
    // java throw wbThread.getIOException();
    // java } else if (ssThread.getIOException() != null) {
    // java throw ssThread.getIOException();
    // java } else if (stThread.getIOException() != null) {
    // java throw stThread.getIOException();
    // java }
    // java } else {
    // java throw new FileDamagedException();
    // java }
    // java }
    pub fn open(reader: &Rc<XlsxrWorkbook>) -> io::Result<()> {
        let (mut strings, mut styles, table, book) = reader.globalReaderInputs()?;
        XlsxrWorkbook::open_theme(reader)?;
        let wb = reader.clone();
        let wbThread = std::thread::spawn(move || XlsxrWorkbook::read_workbook(&wb));
        let ssBook = book.clone();
        let ssThread = std::thread::spawn(move || {
            catch_unwind(AssertUnwindSafe(|| {
                XlsxrSharedStrings::open(&ssBook, strings.as_mut())
            }))
        });
        let stThread =
            std::thread::spawn(move || XlsxrStyles::open(&book, styles.as_mut(), &table));
        // rust: Joining all three workers is the native ThreadWaiter completion barrier. Exception inspection retains original workbook, strings, styles order.
        let wb = wbThread.join();
        let ss = ssThread.join();
        let st = stThread.join();
        match wb {
            Ok(r) => r?,
            Err(p) => std::panic::resume_unwind(p),
        };
        // java StringsThread.run catches Throwable after IOException; only its IOException is rethrown by open().
        if let Ok(Ok(r)) = ss {
            r?;
        }
        match st {
            Ok(r) => r?,
            Err(p) => std::panic::resume_unwind(p),
        };
        Ok(())
    }
}
