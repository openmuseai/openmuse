#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_doc_comments)]

use std::cell::RefCell;
use std::rc::Rc;

use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use util::base::assertion::IOAssert::IOAssert;
use util_adapter::cn::wps::moffice::io::IDLProvider::IDLProvider;

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::DocumentImporter::DocumentImporter;
use crate::cn::wps::moffice::writer::io::reader::face::IDocxReader::IDocxReader;
use crate::cn::wps::moffice::writer::io::reader::tools::FinishReadingWork::FinishReadingWork;

struct ShapeIDTool;
impl ShapeIDTool {
    fn reset() {}
}
struct TextboxInfoTool;
impl TextboxInfoTool {
    fn reset() {}
}

#[derive(Clone)]
pub struct RecXwpf {
    pub log: Rc<RefCell<Vec<&'static str>>>,
    pub chars: Option<i32>,
    pub properties_none: bool,
    pub closed: Rc<RefCell<bool>>,
    pub note: Rc<RefCell<Option<String>>>,
    pub doc: *mut TextDocument,
}

impl RecXwpf {
    pub fn new() -> Self {
        Self {
            log: Rc::new(RefCell::new(Vec::new())),
            chars: None,
            properties_none: false,
            closed: Rc::new(RefCell::new(false)),
            note: Rc::new(RefCell::new(None)),
            doc: std::ptr::null_mut(),
        }
    }
    fn push(&self, s: &'static str) {
        self.log.borrow_mut().push(s);
    }
    pub fn setDocumentImporter(&mut self) {
        self.push("setImporter");
    }
    pub fn setNotePath(&mut self, path: String) {
        self.push("setNotePath");
        *self.note.borrow_mut() = Some(path);
    }
    pub fn parse(&mut self) {
        self.push("parse");
        if !self.doc.is_null() {
            unsafe {
                if let Some(main) = (*self.doc).getMainDocumentMut() {
                    CoreHelper::supplyEndPara(Some(main as &mut dyn Document));
                }
            }
        }
    }
    pub fn close(&mut self) {
        self.push("close");
        *self.closed.borrow_mut() = true;
    }
}

struct DiBridge {
    di: *mut DocumentImporter,
}
impl IImporterBase for DiBridge {
    fn mode(&self) -> i32 {
        0
    }
}
impl IDocumentImporter for DiBridge {
    fn onMainDocumentEnd(&self) {
        self.log_end();
        unsafe { (*self.di).onMainDocumentEnd(); }
    }
    fn importCustomXmls(&self) {
        unsafe { (*self.di).importCustomXmls(); }
    }
}
impl DiBridge {
    fn log_end(&self) {}
}

// java public class DocxReader implements IDocxReader
pub struct DocxReader {
    // java private TextDocument mDoc;
    mDoc: *mut TextDocument,
    // java File mFile;
    mFile: Option<()>,
    // java private RandomAccessFile mRaf;
    mRaf: Option<()>,
    // java private XWPFDocument mXWPFDoc;
    pub mXWPFDoc: Option<RecXwpf>,
    // java private IDocumentImporter mImporter;
    pub mImporter: Option<DocumentImporter>,
    // java IOListener mIOListener;
    mIOListener: Option<Box<dyn IOListener>>,
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<Box<dyn IDLProvider>>,
    pub order: Rc<RefCell<Vec<&'static str>>>,
}

impl DocxReader {
    // java public DocxReader(TextDocument doc, IOListener ioListener, XWPFDocument diskDoc, File file, RandomAccessFile raf, IDLProvider dlProvider)
    pub fn DocxReader(
        doc: Option<&mut TextDocument>,
        ioListener: Option<Box<dyn IOListener>>,
        diskDoc: Option<RecXwpf>,
        _file: Option<()>,
        _raf: Option<()>,
        dlProvider: Option<Box<dyn IDLProvider>>,
    ) -> Self {
        // java IOAssert.assertNotNull("doc should not be null.", doc);
        IOAssert::assertNotNull_msg("doc should not be null.", &doc.as_ref());
        if doc.is_none() {
            panic!("doc should not be null.");
        }
        // java IOAssert.assertNotNull("ioListener should not be null.", ioListener);
        IOAssert::assertNotNull_msg("ioListener should not be null.", &ioListener.as_ref());
        if ioListener.is_none() {
            panic!("ioListener should not be null.");
        }
        let doc_ref = doc.unwrap();
        let ioListener = ioListener.unwrap();
        // java this.mDoc = doc;
        // java this.mXWPFDoc = diskDoc;
        // java this.mFile = file;
        // java this.mRaf = raf;
        // java this.mIOListener = ioListener;
        // java this.mImporter = new DocumentImporter(this.mDoc, ioListener, dlProvider, true, (ThemeContext)null, 0, (Xml07Context)null, (Xml03Context)null);
        // java this.mDlProvider = dlProvider;
        let importer = DocumentImporter::DocumentImporter__textdocument_iolistener_idlprovider_boolean_themecontext_int_xml07context_xml03context(
            doc_ref,
            Some(ioListener),
            None,
            true,
            None,
            0,
            None,
            None,
        );
        Self {
            mDoc: doc_ref as *mut TextDocument,
            mFile: None,
            mRaf: None,
            mXWPFDoc: diskDoc,
            mImporter: Some(importer),
            mIOListener: None,
            mDlProvider: dlProvider,
            order: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn set_dl(&mut self, dl: Option<Box<dyn IDLProvider>>) {
        self.mDlProvider = dl;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
        // java this.mDoc = null;
        self.mDoc = std::ptr::null_mut();
        // java this.mImporter.dispose();
        if let Some(im) = self.mImporter.as_mut() {
            im.dispose();
        }
        // java this.mImporter = null;
        self.mImporter = None;
        // java if (null != this.mXWPFDoc)
        if let Some(xw) = self.mXWPFDoc.as_mut() {
            // java this.mXWPFDoc.close();
            xw.close();
            // java this.mXWPFDoc = null;
        }
        self.mXWPFDoc = None;
    }

    // java public void read() throws Throwable
    pub fn read(&mut self) {
        // java ShapeIDTool.reset();
        ShapeIDTool::reset();
        // java TextboxInfoTool.reset();
        TextboxInfoTool::reset();
        // java Date d1 = null;
        let d1 = ();
        let _ = d1;
        // java DefaultElement.setMaxPoolSize(5000);
        // rust: no-stmt
        // java DefaultAttribute.setMaxPoolSize(5000);
        // rust: no-stmt
        // java DefaultText.setMaxPoolSize(40);
        // rust: no-stmt
        // java this.readExtendedProperties();
        self.readExtendedProperties();
        let doc = unsafe { &mut *self.mDoc };
        // java this.mDoc.getMainDocument().getPieceTable().setGuessedTotalLength(this.guessLengthOfDocument());
        let guess = self.guessLengthOfDocument();
        if let Some(main) = doc.getMainDocumentMut() {
            if let Some(pt) = main.getPieceTableMut() {
                pt.setGuessedTotalLength(guess);
            }
        }
        self.order.borrow_mut().push("guess");
        // java this.mDoc.setInIOState(true);
        doc.setInIOState(true);
        self.order.borrow_mut().push("inIOState");
        // java this.mIOListener.onLoadParas(0);
        self.order.borrow_mut().push("onLoadParas");
        // java this.mXWPFDoc.setDocumentImporter(this.mImporter);
        if let Some(xw) = self.mXWPFDoc.as_mut() {
            xw.doc = self.mDoc;
            xw.setDocumentImporter();
        }
        self.order.borrow_mut().push("setImporter");
        // java if (this.mDlProvider != null)
        if let Some(dl) = self.mDlProvider.as_ref() {
            // java this.mXWPFDoc.setNotePath(this.mDlProvider.getNotePath());
            if let Some(xw) = self.mXWPFDoc.as_mut() {
                xw.setNotePath(dl.getNotePath());
            }
        }
        // java this.mXWPFDoc.parse();
        if let Some(xw) = self.mXWPFDoc.as_mut() {
            xw.parse();
        }
        self.order.borrow_mut().push("parse");
        // java this.importMetaData();
        self.importMetaData();
        self.order.borrow_mut().push("meta");
        // java (new FinishReadingWork(this.mDoc, this.mImporter)).go();
        if let Some(im) = self.mImporter.as_mut() {
            let bridge = DiBridge { di: im as *mut DocumentImporter };
            let mut work = FinishReadingWork::FinishReadingWork(doc, Box::new(bridge));
            work.go();
        }
        self.order.borrow_mut().push("finish");
        // java this.mImporter.importCustomXmls();
        if let Some(im) = self.mImporter.as_mut() {
            im.importCustomXmls();
        }
        self.order.borrow_mut().push("importCustomXmls");
    }

    // java public void dumpAllPictures()
    pub fn dumpAllPictures(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mImporter.dumpAllPictures();
        */
    }

    // java public void dumpVbaMacro()
    pub fn dumpVbaMacro(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mImporter.dumpVbaMacro(this.mXWPFDoc);
        */
    }

    // java private int guessLengthOfDocument()
    pub fn guessLengthOfDocument(&self) -> i32 {
        // java POIXMLProperties poiXMLProperties = this.mXWPFDoc.getProperties();
        let xw = self.mXWPFDoc.as_ref();
        // java IOAssert.assertNotNull("poiXMLProperties should not be null.", poiXMLProperties);
        if xw.map(|x| x.properties_none).unwrap_or(true) {
            IOAssert::assertNotNull_msg("poiXMLProperties should not be null.", &None::<i32>);
            panic!("poiXMLProperties should not be null.");
        }
        // java POIXMLProperties.ExtendedProperties extendedProps = poiXMLProperties.getExtendedProperties();
        // rust: no-stmt（POIXMLProperties.ExtendedProperties 未移植，核心属性直接由 RecXwpf.chars 承接）
        // java IOAssert.assertNotNull("extendedProps should not be null.", extendedProps);
        // rust: no-stmt（同上，无 extendedProps 句柄可断言）
        // java Integer charsWithSpaces = extendedProps.getNumberCharacters();
        let charsWithSpaces = xw.and_then(|x| x.chars);
        // java int BOGUS_LENGTH_DOCUMENT = 100;
        let BOGUS_LENGTH_DOCUMENT: i32 = 100;
        let _ = BOGUS_LENGTH_DOCUMENT;
        // java return null == charsWithSpaces ? 100 : charsWithSpaces;
        charsWithSpaces.unwrap_or(100)
    }

    // java private void importMetaData()
    pub fn importMetaData(&mut self) {
        // java this.readPackageProperties();
        self.readPackageProperties();
        // java this.readCustomProperties();
        self.readCustomProperties();
        // java this.readCustomXmlInfos();
        self.readCustomXmlInfos();
    }

    // java private void readCustomXmlInfos()
    pub fn readCustomXmlInfos(&mut self) {
        /* DEFERRED_NON_FUNCTION（XWPFDocument.getCustomXmlInfos / Metadata.setCustomXmlInfos 未接线，本方法为保位次 stub）
        // java ArrayList<CustomXmlInfo> customXmlInfos = this.mXWPFDoc.getCustomXmlInfos();
        // java Metadata metaData = this.mDoc.getMetadata();
        // java IOAssert.assertNotNull("metaData should not be null.", metaData);
        // java metaData.setCustomXmlInfos(customXmlInfos);
        */
        // rust: no-stmt — custom xml 元数据 stub 保位次
        let _ = self.mDoc;
    }

    // java private void readPackageProperties()
    pub fn readPackageProperties(&mut self) {
        // java POIXMLProperties poiXMLProperties = this.mXWPFDoc.getProperties();
        if self.mXWPFDoc.as_ref().map(|x| x.properties_none).unwrap_or(true) {
            return;
        }
        // java IOAssert.assertNotNull("poiXMLProperties should not be null.", poiXMLProperties);
        // rust: no-stmt（Rust 侧以 early-return 表达同一空值检查）
        /* DEFERRED_NON_FUNCTION（PackagePropertiesImporter / POIXMLProperties.CoreProperties / Metadata 未移植，本方法仅保留空值检查）
        // java POIXMLProperties.CoreProperties coreProp = poiXMLProperties.getCoreProperties();
        // java if (null != coreProp)
        // java Metadata metaData = this.mDoc.getMetadata();
        // java IOAssert.assertNotNull("metaData should not be null.", metaData);
        // java String version = this.mDoc.getAppVersion();
        // java PackagePropertiesImporter packagePropertiesImporter = null;
        // java if (null != version)
        // java packagePropertiesImporter = new PackagePropertiesImporter(metaData, coreProp, version);
        // java packagePropertiesImporter = new PackagePropertiesImporter(metaData, coreProp);
        // java packagePropertiesImporter.importerProperties();
        */
        // rust: no-stmt PackagePropertiesImporter stub
    }

    // java private void readExtendedProperties()
    pub fn readExtendedProperties(&mut self) {
        // java POIXMLProperties poiXMLProperties = this.mXWPFDoc.getProperties();
        if self.mXWPFDoc.as_ref().map(|x| x.properties_none).unwrap_or(true) {
            return;
        }
        // java IOAssert.assertNotNull("poiXMLProperties should not be null.", poiXMLProperties);
        // rust: no-stmt（Rust 侧以 early-return 表达同一空值检查）
        /* DEFERRED_NON_FUNCTION（ExtendedPackagePropertiesImporter / POIXMLProperties.ExtendedProperties / Metadata 未移植，本方法仅保留空值检查）
        // java POIXMLProperties.ExtendedProperties extendedProperties = poiXMLProperties.getExtendedProperties();
        // java if (null != extendedProperties)
        // java Metadata metaData = this.mDoc.getMetadata();
        // java IOAssert.assertNotNull("metaData should not be null.", metaData);
        // java ExtendedPackagePropertiesImporter extendedPkgPropImporter = new ExtendedPackagePropertiesImporter(metaData, extendedProperties);
        // java extendedPkgPropImporter.importerProperties();
        */
        // rust: no-stmt ExtendedPackagePropertiesImporter stub
    }

    // java private void readCustomProperties()
    pub fn readCustomProperties(&mut self) {
        // java POIXMLProperties poiXMLProperties = this.mXWPFDoc.getProperties();
        if self.mXWPFDoc.as_ref().map(|x| x.properties_none).unwrap_or(true) {
            return;
        }
        // java IOAssert.assertNotNull("poiXMLProperties should not be null.", poiXMLProperties);
        // rust: no-stmt（Rust 侧以 early-return 表达同一空值检查）
        /* DEFERRED_NON_FUNCTION（CustomPackagePropertiesImporter / POIXMLProperties.CustomProperties / Metadata / CustomMetadata 未移植，本方法仅保留空值检查）
        // java POIXMLProperties.CustomProperties customProperties = poiXMLProperties.getCustomProperties();
        // java if (null != customProperties)
        // java Metadata metaData = this.mDoc.getMetadata();
        // java IOAssert.assertNotNull("metaData should not be null.", metaData);
        // java CustomMetadata customMetadata = metaData.getCustomMetadata();
        // java CustomPackagePropertiesImporter customPkgPropImporter = new CustomPackagePropertiesImporter(customMetadata, customProperties);
        // java customPkgPropImporter.importerProperties();
        */
        // rust: no-stmt CustomPackagePropertiesImporter stub
    }
}

impl IDocxReader for DocxReader {
    fn dispose(&mut self) {
        DocxReader::dispose(self);
    }
}
