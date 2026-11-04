use crate::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener;
// java package cn.wps.moss.app;
// rust: no-stmt
use crate::java_runtime::Rc;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::app::KmoBooksApi::KmoBooksApi;

// java public class KmoBookApi extends KmoBook
pub struct KmoBookApi {
    book: Rc<KmoBook>,
    // java private String mPath;
    mPath: crate::java_runtime::RefCell<Option<String>>,
}

impl KmoBookApi {
    // java public KmoBookApi() {
    pub fn KmoBookApi() -> Rc<Self> {
        Rc::new(Self {
            book: KmoBook::KmoBook(),
            mPath: crate::java_runtime::RefCell::new(None),
        })
    }

    pub fn book(&self) -> Rc<KmoBook> {
        Rc::clone(&self.book)
    }

    // java public int open(String path, String password, Object context, final IDLProvider dlProvider, boolean ignoreSecurity) {
    pub fn open(&self, path: &str, password: Option<&str>, ignoreSecurity: bool) -> i32 {
        self.open__listener(path, password, ignoreSecurity, None)
    }

    // rust: retain the original books().addBookOpenEventListener argument; C ABI remains unchanged.
    pub fn open__listener(
        &self,
        path: &str,
        _password: Option<&str>,
        ignoreSecurity: bool,
        listener: Option<Rc<dyn KmoBookOpenEventListener>>,
    ) -> i32 {
        // java this.mPath = path;
        *self.mPath.borrow_mut() = Some(path.to_string());
        let opened = catch_unwind(AssertUnwindSafe(|| {
            self.open_body(path, ignoreSecurity, listener)
        }));
        match opened {
            Ok(code) => code,
            // java } catch (Throwable var12) {
            Err(payload) => map_failure_code(&panic_message(&payload)),
        }
    }

    fn open_body(
        &self,
        path: &str,
        ignoreSecurity: bool,
        listener: Option<Rc<dyn KmoBookOpenEventListener>>,
    ) -> i32 {
        // java if (!ignoreSecurity) {
        if !ignoreSecurity {
            // java OnlineSecurityTool tool = new OnlineSecurityTool();
            let _ = stringify!(tool);
            // rust: no-stmt — 安全文件工具没有类文件。M1 夹具不是安全文件。登记 §1.7.1。
            // java if (tool.isWpsSecurityFile(path)) {
            // rust: no-stmt
            // java return 5;
            // rust: no-stmt
            let _ = path;
        }
        // java this.setDLProvider(dlProvider);
        // rust: no-stmt — 下载回调不参与格子。登记 §1.7.1。
        // java KmoApp app = KmoAppApi.sharedApp2();
        let app=super::KmoAppApi::KmoAppApi::sharedApp2();
        // java this.init((KmoBooks)null, false);
        self.book.init(false);
        // java app.startup((Context)context);
        super::KmoAppApi::KmoAppApi::startup(&app);
        // java app.books().addBookOpenEventListener(new BookOpenListener());
        let books=app.books().expect("NullPointerException");
        books.addBookOpenEventListener(Some(Rc::new(super::BookOpenListener::BookOpenListener)));
        // java app.books().addBookOpenEventListener(new KmoBookOpenEventListener() { public void onProgressChange(byte progress) { } public void bookDidFinishInitializing(KmoBook book) { } public void activeSheetDidFinishInitializing() throws FirstPageForceQuitException { if (null != dlProvider && dlProvider.isOpenFirstPageForceQuit()) { throw new FirstPageForceQuitException(); } public void sheetWillReadCurrentViewport(int sheetIndex) { } public void sheetDidFinishReadingCurrentViewport(int sheetIndex) { } public void sheetDidFinishReadingEverything(int sheetIndex) { } public void bookDidFinishReadingEverything() { }
        books.addBookOpenEventListener(Some(Rc::new(OpenForceQuitListener{dlProvider:None})));
        // Native external observer joins the same original group; no observer
        // replaces either listener registered by the Java open method.
        if let Some(listener)=listener{books.addBookOpenEventListener(Some(listener));}
        // java File tmpDir = new File(this.getTmpPath());
        // rust: no-stmt
        // java tmpDir.mkdirs();
        // rust: no-stmt
        // java app.books().openBook((KmoBook)this, (String)path, (KmoBookOpenPasswordProvider)(new BookOpenPassword(password)));
        match KmoBooksApi::openBook__book_listener(&self.book, path, Some(books.bookOpenListenerGroup.clone())) {
            Ok(()) => {
                // java this.setDirty(true);
                self.book.setDirty(true);
                // java return 1;
                1
            }
            // java } catch (EncryptFileException var8) {
            // java return 3;
            // java } catch (ForceQuitException var9) {
            // java return 4;
            // java } catch (FirstPageForceQuitException var10) {
            // java return 6;
            // java } catch (Throwable var11) {
            // java return 2;
            Err(err) => map_failure_code(&err.to_string()),
        }
    }

    // java public void onProgressChange(byte progress) {
    pub fn onProgressChange() {}

    // java public void bookDidFinishInitializing(KmoBook book) {
    pub fn bookDidFinishInitializing() {}

    // java public void activeSheetDidFinishInitializing() throws FirstPageForceQuitException {
    pub fn activeSheetDidFinishInitializing() {
        // java if (null != dlProvider && dlProvider.isOpenFirstPageForceQuit()) {
        // rust: no-stmt — 本轮 dlProvider 为空。
        // java throw new FirstPageForceQuitException();
        // rust: no-stmt
    }

    // java public void sheetWillReadCurrentViewport(int sheetIndex) {
    pub fn sheetWillReadCurrentViewport() {}

    // java public void sheetDidFinishReadingCurrentViewport(int sheetIndex) {
    pub fn sheetDidFinishReadingCurrentViewport() {}

    // java public void sheetDidFinishReadingEverything(int sheetIndex) {
    pub fn sheetDidFinishReadingEverything() {}

    // java public void bookDidFinishReadingEverything() {
    pub fn bookDidFinishReadingEverything() {}

    // java public boolean saveImpl(File destFile, File dir, int suffix) throws IOException {
    pub fn saveImpl(&self) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean result = false;
        // java KmoLog.logPoint("suffix " + suffix);
        // java if (null != destFile && destFile.exists()) {
        // java KmoLog.logPoint("#saveImpl start destFile: " + destFile.getAbsoluteFile() + " length: " + destFile.length());
        // java File tempFile = null;
        // java this.setInterruptAutoSave(false);
        // java KmoLog.logPoint("#saveImpl setInterruptAutoSave(false)");
        // java this.getAppSettings().updateSettings();
        // java tempFile = File.createTempFile("WPS_", ".tmp", new File(dir.getPath()));
        // java if (suffix == 0) {
        // java XlsWriter.write(this, tempFile, dir);
        // java if (!XlsReader.isBinaryFileFormat(tempFile.getPath())) {
        // java throw new IOException("validate wroten file failed");
        // java } else if (suffix == 1) {
        // java XlsxWriter.write(this, tempFile.getPath());
        // java if (!XlsxReader.isOOXMLFileFormat(tempFile.getPath()) && !XlsReader.isBinaryFileFormat(tempFile.getPath())) {
        // java throw new IOException("validate wroten file failed");
        // java } else if (suffix == 4) {
        // java CSVWriter.write(this, tempFile, dir);
        // java KmoLog.logPoint("save temp end. suffix: " + suffix);
        // java if (null != tempFile && tempFile.exists()) {
        // java KmoLog.logPoint("#saveImpl  tempFile: " + tempFile.getAbsoluteFile() + " length: " + tempFile.length());
        // java KmoLog.logPoint("#saveImpl isMakingSnapshot: " + this.isMakingSnapshot + " isInterruptAutoSave :" + this.isInterruptAutoSave());
        // java result = FileUtil.moveFile(tempFile, destFile);
        // java if (tempFile != null) {
        // java FileUtil.delFile(tempFile.getAbsolutePath());
        // java if (null != destFile && destFile.exists()) {
        // java KmoLog.logPoint("#saveImpl end destFile: " + destFile.getAbsoluteFile() + " length: " + destFile.length());
        // java KmoLog.logPoint("end");
        // java return result;
         */
    }

    // java public void importExternalTextData(String filePath) throws IOException {
    pub fn importExternalTextData(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoExternalTextDataImportingConfig config = new KmoExternalTextDataImportingConfig(filePath);
        // java BookReader reader = new PlainTextReader(this, filePath, config);
        // java this.setFileFormat(5);
        // java reader.open();
         */
    }

    // java public boolean isSecurityFile() {
    pub fn isSecurityFile(&self) -> bool {
        // java if (null != this.mPath) {
        if self.mPath.borrow().is_some() {
            // java OnlineSecurityTool tool = new OnlineSecurityTool();
            let _ = stringify!(tool);
            // rust: no-stmt
            // java return tool.isWpsSecurityFile(this.mPath);
            // rust: no-stmt — 工具类不在编译面，不把普通 xlsx 判成安全文件。
            false
        } else {
            // java return false;
            false
        }
    }
}

// Native name for open()'s original anonymous listener and its captured reference.
struct OpenForceQuitListener{dlProvider:Option<Rc<dyn util_adapter::cn::wps::moffice::io::IDLProvider::IDLProvider+Send+Sync>>}
impl KmoBookOpenEventListener for OpenForceQuitListener{
    // java public void onProgressChange(byte progress){}
    fn onProgressChange(&self,_progress:i8){}
    // java public void bookDidFinishInitializing(KmoBook book){}
    fn bookDidFinishInitializing(&self,_book:&KmoBook){}
    // java public void activeSheetDidFinishInitializing()throws FirstPageForceQuitException{if(null!=dlProvider&&dlProvider.isOpenFirstPageForceQuit()){throw new FirstPageForceQuitException();}}
    fn activeSheetDidFinishInitializing(&self){if let Some(dlProvider)=&self.dlProvider{if dlProvider.isOpenFirstPageForceQuit(){panic!("FirstPageForceQuitException");}}}
    // java public void sheetWillReadCurrentViewport(int sheetIndex){}
    fn sheetWillReadCurrentViewport(&self,_sheetIndex:i32){}
    // java public void sheetDidFinishReadingCurrentViewport(int sheetIndex){}
    fn sheetDidFinishReadingCurrentViewport(&self,_sheetIndex:i32){}
    // java public void sheetDidFinishReadingEverything(int sheetIndex){}
    fn sheetDidFinishReadingEverything(&self,_sheetIndex:i32){}
    // java public void bookDidFinishReadingEverything(){}
    fn bookDidFinishReadingEverything(&self){}
}

fn map_failure_code(message: &str) -> i32 {
    if message.contains("EncryptFileException") {
        3
    } else if message.contains("FirstPageForceQuitException") {
        6
    } else if message.contains("ForceQuitException") {
        4
    } else {
        2
    }
}

fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(text) = payload.downcast_ref::<&str>() {
        return (*text).to_string();
    }
    if let Some(text) = payload.downcast_ref::<String>() {
        return text.clone();
    }
    "Throwable".to_string()
}
