#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_doc_comments)]

use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use util::base::assertion::IOAssert::IOAssert;
use util::base::io::SubDocType::SubDocType;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;

struct ShapeIDTool;
impl ShapeIDTool {
    pub fn reset() {}
}

struct TextboxInfoTool;
impl TextboxInfoTool {
    pub fn run() {}
}

// java public class FinishReadingWork
pub struct FinishReadingWork {
    // java private TextDocument mDoc;
    mDoc: *mut TextDocument,
    // java private IDocumentImporter mImporter;
    mImporter: Box<dyn IDocumentImporter>,
}

impl FinishReadingWork {
    // java public FinishReadingWork(TextDocument doc, IDocumentImporter importer)
    pub fn FinishReadingWork(doc: &mut TextDocument, importer: Box<dyn IDocumentImporter>) -> Self {
        // java this.mDoc = doc;
        // java this.mImporter = importer;
        Self { mDoc: doc as *mut TextDocument, mImporter: importer }
    }

    // java public void go()
    pub fn go(&mut self) {
        let doc = unsafe { &mut *self.mDoc };
        // java IOHelper.endAllSubdocumentsDrawingContainer(this.mDoc);
        IOHelper::endAllSubdocumentsDrawingContainer(doc);
        // java ShapeIDTool.reset();
        ShapeIDTool::reset();
        // java this.correctCoreData();
        self.correctCoreData();
        // java this.processDmlLinkedTxbx();
        self.processDmlLinkedTxbx();
        // java TextboxInfoTool.run();
        TextboxInfoTool::run();
        // java IOHelper.addNormalStyle(this.mDoc.getStyles());
        if let Some(styles) = doc.getStylesMut() {
            IOHelper::addNormalStyle(styles);
        }
        // java this.mImporter.onMainDocumentEnd();
        self.mImporter.onMainDocumentEnd();
    }

    // java private void processDmlLinkedTxbx()
    pub fn processDmlLinkedTxbx(&mut self) {
        // java IOAssert.assertNotNull("mImporter should not null!", this.mImporter);
        IOAssert::assertNotNull_msg("mImporter should not null!", &Some(&self.mImporter));
        // java SubDocType[] values = SubDocType.values();
        let values = SubDocType::values();
        // java int size = values.length;
        let size = values.len();
        let mut i: usize = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
            // java IDmlImporter dmlImporter = this.mImporter.getDmlImporter(values[i]);
            let dmlImporter = self.mImporter.getDmlImporter(values[i] as i32);
            // java if (null != dmlImporter)
            if let Some(dmlImporter) = dmlImporter {
                // java dmlImporter.buildTxbxChain();
                dmlImporter.borrow_mut().buildTxbxChain();
            }
            i += 1;
        }
    }

    // java private void correctCoreData()
    pub fn correctCoreData(&mut self) {
        let doc = unsafe { &mut *self.mDoc };
        // java CoreHelper.supplyEndPara((SubDocument)this.mDoc.getMainDocument());
        if let Some(main) = doc.getMainDocumentMut() {
            CoreHelper::supplyEndPara(Some(main as &mut dyn Document));
        }
        // java this.correctShapePropertyId_textNext();
        self.correctShapePropertyId_textNext();
        // java this.setTextIdentifiers();
        self.setTextIdentifiers();
        // java this.correctSectionData();
        self.correctSectionData();
        // java this.correctFootnoteDocument();
        self.correctFootnoteDocument();
        // java this.correctEndnoteDocument();
        self.correctEndnoteDocument();
    }

    // java private void correctFootnoteDocument()
    pub fn correctFootnoteDocument(&mut self) {
        let doc = unsafe { &mut *self.mDoc };
        // java SubDocument footnote = (SubDocument)this.mDoc.getSubdocument(1);
        let Some(footnote) = doc.getSubdocumentMut(1) else { return; };
        // java if (null != footnote && footnote.getLength() != 0)
        if footnote.getLength() != 0 {
            // java PLCFootnoteEndnoteTxt footnoteEndnoteTxt = footnote.getFootnoteTxt();
            if footnote.mFootnoteTxt.is_none() {
                footnote.mFootnoteTxt = Some(Default::default());
            }
            // java removeInvalidFootnoteEndnoteTxt(footnoteEndnoteTxt);
            if let Some(txt) = footnote.mFootnoteTxt.as_mut() {
                Self::removeInvalidFootnoteEndnoteTxt(txt);
            }
        }
    }

    // java private void correctEndnoteDocument()
    pub fn correctEndnoteDocument(&mut self) {
        let doc = unsafe { &mut *self.mDoc };
        // java SubDocument endnote = (SubDocument)this.mDoc.getSubdocument(4);
        let Some(endnote) = doc.getSubdocumentMut(4) else { return; };
        // java if (null != endnote && endnote.getLength() != 0)
        if endnote.getLength() != 0 {
            // java PLCFootnoteEndnoteTxt footnoteEndnoteTxt = endnote.getEndnoteTxt();
            // java removeInvalidFootnoteEndnoteTxt(footnoteEndnoteTxt);
            // rust: no-stmt
            // java CoreHelper.supplyEndPara(endnote);
            CoreHelper::supplyEndPara(Some(endnote as &mut dyn Document));
        }
    }

    // java private static void removeInvalidFootnoteEndnoteTxt(PLCFootnoteEndnoteTxt footnoteEndnoteTxt)
    pub fn removeInvalidFootnoteEndnoteTxt(footnoteEndnoteTxt: &mut crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt) {
        // java IOAssert.assertNotNull("footnoteEndnoteTxt should not be null.", footnoteEndnoteTxt);
        IOAssert::assertNotNull_msg("footnoteEndnoteTxt should not be null.", &Some(&*footnoteEndnoteTxt));
        // java PLC.Iterator iter = footnoteEndnoteTxt.createIterator();
        // rust: no-stmt
        // java while (iter.hasNext())
        // java if (txtNode.getFootnoteEndnoteRef() == null)
        // java footnoteEndnoteTxt.removeFootnoteEndnoteTxt(txtNode);
        footnoteEndnoteTxt.removeInvalids();
    }

    // java private void correctSectionData()
    pub fn correctSectionData(&mut self) {
        let doc = unsafe { &mut *self.mDoc };
        // java IOAssert.assertNotNull("mDoc should not be null.", this.mDoc);
        IOAssert::assertNotNull_msg("mDoc should not be null.", &Some(&*doc));
        // java Document mainDocument = this.mDoc.getSubdocument(0);
        let Some(mainDocument) = doc.getSubdocumentMut(0) else {
            panic!("mainDocument should not be null.");
        };
        // java IOAssert.assertNotNull("mainDocument should not be null.", mainDocument);
        // java PLCSection plcSection = mainDocument.getPLCSection();
        let Some(plcSection) = mainDocument.getPLCSectionMut() else {
            panic!("plcSection should not be null.");
        };
        // java IOAssert.assertNotNull("plcSection should not be null.", plcSection);
        // java PLC.Node node = plcSection.get(0);
        // java if (!(node instanceof PLCSection.SectionNode))
        if plcSection.getFirstSection().is_none() {
            // java PLCSection.SectionNode firstSectionNode = plcSection.addSection(0);
            let mut firstSectionNode = plcSection.addSection(0);
            // java firstSectionNode.setPropertySet(createDefaultSectionPropertySet());
            firstSectionNode.setPropertySet(&Self::createDefaultSectionPropertySet());
        }
    }

    // java private void correctShapePropertyId_textNext()
    pub fn correctShapePropertyId_textNext(&mut self) {
        // java IOAssert.assertNotNull("mDoc should not be null.", this.mDoc);
        // rust: no-stmt — textbox 链 P1 延后
        let _ = self.mDoc;
    }

    // java private void correctShapePropertyId_textNext(PLCTextboxText plcTextboxText)
    pub fn correctShapePropertyId_textNext__plctextboxtext(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("plcTextboxText should not be null.", plcTextboxText);
        */
    }

    // java private void correctShapePropertyId_textNext(Shape shape)
    pub fn correctShapePropertyId_textNext__shape(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("shape should not be null.", shape);
        */
    }

    // java public boolean execute(Shape shape)
    pub fn execute() {
        todo!("DEFERRED_NON_FUNCTION");
    }

    // java private static void correctShapePropertyId_textNext(final IDrawingContainer container)
    pub fn correctShapePropertyId_textNext__idrawingcontainer() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("container should not be null.", container);
        */
    }

    // java private void setTextIdentifiers()
    pub fn setTextIdentifiers(&mut self) {
        // java IOAssert.assertNotNull("mDoc should not be null.", this.mDoc);
        // rust: no-stmt — textbox identifier P1 延后
        let _ = self.mDoc;
    }

    // java private static void setTextIdentifiers(PLCTextboxText plcTextboxText)
    pub fn setTextIdentifiers__plctextboxtext() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("plcTextboxText should not be null.", plcTextboxText);
        */
    }

    // java private static void setTextIdentifier(Shape shape, int whereText, int indexTxbxChain)
    pub fn setTextIdentifier() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("shape should not be null.", shape);
        */
    }

    // java private static Shape findShapeLikingTextbox(int id, Document document)
    pub fn findShapeLikingTextbox() {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IOAssert.assertNotNull("document should not be null.", document);
        */
    }

    // java private static int toLTextId(int whereText, int indexTxbxChain)
    pub fn toLTextId(whereText: i32, indexTxbxChain: i32) -> i32 {
        // java int ret = 0;
        let mut ret: i32 = 0;
        // java ret = setHigh2Bytes(ret, whereText + 1);
        ret = Self::setHigh2Bytes(ret, whereText + 1);
        // java ret = setLow2Bytes(ret, indexTxbxChain);
        ret = Self::setLow2Bytes(ret, indexTxbxChain);
        // java return ret;
        ret
    }

    // java private static int setHigh2Bytes(int holder, int value)
    pub fn setHigh2Bytes(holder: i32, value: i32) -> i32 {
        // java return holder | value << 16;
        holder | (value << 16)
    }

    // java private static int setLow2Bytes(int holder, int value)
    pub fn setLow2Bytes(holder: i32, value: i32) -> i32 {
        // java return holder | value & '\uffff';
        holder | (value & 0xffff)
    }

    // java private static PropertySet createDefaultSectionPropertySet()
    pub fn createDefaultSectionPropertySet() -> PropertySet {
        // java MutablePropertySet props = new MutablePropertySet();
        let mut props = MutablePropertySet::new();
        // java props.setInt(678, 312);
        props.setInt(678, 312);
        // java props.setInt(677, 0);
        props.setInt(677, 0);
        // java props.setInt(682, 720);
        props.setInt(682, 720);
        // java props.setInt(683, 720);
        props.setInt(683, 720);
        // java props.setValue(676, new PageSetup(11906,16838,1800,1800,1440,1440));
        props.setValue(
            676,
            Some(std::sync::Arc::new(PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440))),
        );
        // java return props.freeze();
        props.freeze()
    }
}
