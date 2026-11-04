// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.PLCDocumentFields
use crate::cn::wps::moffice::writer::data::PLCDocumentFields::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import org.apache.poi.hwpf.model.CPSplitCalculator
use io_poi_compact::org::apache::poi::hwpf::model::CPSplitCalculator::*;
// Java: import org.apache.poi.hwpf.model.DocField
use io_poi_compact::org::apache::poi::hwpf::model::DocField::*;
// Java: import org.apache.poi.hwpf.model.DocFields
use io_poi_compact::org::apache::poi::hwpf::model::DocFields::*;
// Java: import org.apache.poi.hwpf.model.WpsCustomData
use io_poi_compact::org::apache::poi::hwpf::model::WpsCustomData::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.DocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::DocumentImporter::*;

// java public class DocFieldImporter
#[derive(Default)]
pub struct DocFieldImporter {
    // java private DocFields mDocFieldList;
    mDocFieldList: Option<DocFields>,
    // java private DocumentImporter documentImporter;
    documentImporter: Option<DocumentImporter>,
    // java private CPSplitCalculator mCPCalculator;
    mCPCalculator: Option<CPSplitCalculator>,
}

// java public class DocFieldImporter
impl DocFieldImporter {
    // java public  DocFieldImporter(WpsCustomData wpsCustomData, CPSplitCalculator cpCalculator, DocumentImporter documentImporter)
    pub fn DocFieldImporter(wpsCustomData: &WpsCustomData, cpCalculator: &CPSplitCalculator, documentImporter: &DocumentImporter) -> Self {
      // java this.mDocFieldList = wpsCustomData.getDocFields();
      Self { mDocFieldList: wpsCustomData.getDocFields(), documentImporter: documentImporter, mCPCalculator: cpCalculator }
      // java this.documentImporter = documentImporter;
      // java this.mCPCalculator = cpCalculator;
    }

    // java public void onImportDocField(TextDocument document, int headerOffsetCP)
    pub fn onImportDocField__textdocument_int(&mut self, document: &TextDocument, headerOffsetCP: i32) {
      // java IOAssert.assertNotNull("mDocument should not be null!", document);
      IOAssert::assertNotNull__string_object(&"mDocument should not be null!", Some(&document));
      // java IOAssert.assertNotNull("mDocFieldList should not be null!", this.mDocFieldList);
      IOAssert::assertNotNull__string_object(&"mDocFieldList should not be null!", Some(&self.mDocFieldList));
      // java int nDocFileCount = this.mDocFieldList.size();
      let mut nDocFileCount: i32 = self.mDocFieldList.size();
      // java if (nDocFileCount != 0)
      if nDocFileCount != 0 {
        // java int pos = 0;
        let mut pos: i32 = 0;
        // java Document subDocument = document.getMainDocument();
        let mut subDocument: Box<dyn Document> = document.getMainDocument();
        // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getMainDocumentStart(), this.mCPCalculator.getMainDocumentEnd(), pos, nDocFileCount, this.documentImporter.getMainDocumentLoadCp());
        pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getMainDocumentStart(), self.mCPCalculator.getMainDocumentEnd(), pos, nDocFileCount, self.documentImporter.getMainDocumentLoadCp());
        // java if (pos < nDocFileCount)
        if pos < nDocFileCount {
          // java subDocument = document.getSubdocument(1);
          subDocument = document.getSubdocument(1);
          // java if (null != subDocument && subDocument.getLength() > 0 && null != this.documentImporter.getFootnoteDocumentImporter())
          if subDocument.is_some() && subDocument.getLength() > 0 && self.documentImporter.getFootnoteDocumentImporter().is_some() {
            // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getFootnoteStart(), this.mCPCalculator.getFootnoteEnd(), pos, nDocFileCount, this.documentImporter.getFootnoteDocumentLoadCp());
            pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getFootnoteStart(), self.mCPCalculator.getFootnoteEnd(), pos, nDocFileCount, self.documentImporter.getFootnoteDocumentLoadCp());
            // java if (pos >= nDocFileCount)
            if pos >= nDocFileCount {
              // java return;
              return;
            }
          }
          // java subDocument = document.getSubdocument(2);
          subDocument = document.getSubdocument(2);
          // java if (null != subDocument && subDocument.getLength() > 0 && null != this.documentImporter.getHeaderDocumentImporter())
          if subDocument.is_some() && subDocument.getLength() > 0 && self.documentImporter.getHeaderDocumentImporter().is_some() {
            // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getHeaderStoryStart() + headerOffsetCP, this.mCPCalculator.getHeaderStoryEnd(), pos, nDocFileCount, this.documentImporter.getHeaderDocumentLoadCp());
            pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getHeaderStoryStart() + headerOffsetCP, self.mCPCalculator.getHeaderStoryEnd(), pos, nDocFileCount, self.documentImporter.getHeaderDocumentLoadCp());
            // java if (pos >= nDocFileCount)
            if pos >= nDocFileCount {
              // java return;
              return;
            }
          }
          // java subDocument = document.getSubdocument(3);
          subDocument = document.getSubdocument(3);
          // java if (null != subDocument && subDocument.getLength() > 0 && null != this.documentImporter.getCommentDocumentImporter())
          if subDocument.is_some() && subDocument.getLength() > 0 && self.documentImporter.getCommentDocumentImporter().is_some() {
            // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getCommentsStart(), this.mCPCalculator.getCommentsEnd(), pos, nDocFileCount, this.documentImporter.getCommentDocumentLoadCp());
            pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getCommentsStart(), self.mCPCalculator.getCommentsEnd(), pos, nDocFileCount, self.documentImporter.getCommentDocumentLoadCp());
            // java if (pos >= nDocFileCount)
            if pos >= nDocFileCount {
              // java return;
              return;
            }
          }
          // java subDocument = document.getSubdocument(4);
          subDocument = document.getSubdocument(4);
          // java if (null != subDocument && subDocument.getLength() > 0 && null != this.documentImporter.getEndnoteDocumentImporter())
          if subDocument.is_some() && subDocument.getLength() > 0 && self.documentImporter.getEndnoteDocumentImporter().is_some() {
            // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getEndNoteStart(), this.mCPCalculator.getEndNoteEnd(), pos, nDocFileCount, this.documentImporter.getEndnotDocumentLoadCp());
            pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getEndNoteStart(), self.mCPCalculator.getEndNoteEnd(), pos, nDocFileCount, self.documentImporter.getEndnotDocumentLoadCp());
            // java if (pos >= nDocFileCount)
            if pos >= nDocFileCount {
              // java return;
              return;
            }
          }
          // java subDocument = document.getSubdocument(5);
          subDocument = document.getSubdocument(5);
          // java if (null != subDocument && subDocument.getLength() > 0 && null != this.documentImporter.getTextboxDocumentImporter())
          if subDocument.is_some() && subDocument.getLength() > 0 && self.documentImporter.getTextboxDocumentImporter().is_some() {
            // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getMainTextboxStart(), this.mCPCalculator.getMainTextboxEnd(), pos, nDocFileCount, this.documentImporter.getTextBoxDocumentLoadCp());
            pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getMainTextboxStart(), self.mCPCalculator.getMainTextboxEnd(), pos, nDocFileCount, self.documentImporter.getTextBoxDocumentLoadCp());
            // java if (pos >= nDocFileCount)
            if pos >= nDocFileCount {
              // java return;
              return;
            }
          }
          // java subDocument = document.getSubdocument(6);
          subDocument = document.getSubdocument(6);
          // java if (null != subDocument && subDocument.getLength() > 0 && null != this.documentImporter.getHeaderTextboxDocumentImporter())
          if subDocument.is_some() && subDocument.getLength() > 0 && self.documentImporter.getHeaderTextboxDocumentImporter().is_some() {
            // java pos = this.onImportSubDocumentDocField(subDocument, this.mCPCalculator.getHeaderTextboxStart(), this.mCPCalculator.getHeaderTextboxEnd(), pos, nDocFileCount, this.documentImporter.getHeaderTextBoxDocumentLoadCp());
            pos = self.onImportSubDocumentDocField(&subDocument, self.mCPCalculator.getHeaderTextboxStart(), self.mCPCalculator.getHeaderTextboxEnd(), pos, nDocFileCount, self.documentImporter.getHeaderTextBoxDocumentLoadCp());
            // java if (pos >= nDocFileCount)
            if pos >= nDocFileCount {
              // java return;
              return;
            }
          }
        }
      }
    }

    // java private int onImportSubDocumentDocField(Document subDocument, int startGCP, int endGCP, int pos, int count, int loadCp)
    pub fn onImportSubDocumentDocField(&mut self, subDocument: &dyn Document, startGCP: i32, endGCP: i32, pos: i32, count: i32, loadCp: i32) -> i32 {
      // java IOAssert.assertNotNull("subDocument should not be null!", subDocument);
      IOAssert::assertNotNull__string_object(&"subDocument should not be null!", Some(&subDocument));
      // java IOAssert.assertTrue("subDocument.getLength() > 0 should be true!", subDocument.getLength() > 0);
      IOAssert::assertTrue__string_boolean(&"subDocument.getLength() > 0 should be true!", subDocument.getLength() > 0);
      // java int currentPos = pos;
      let mut currentPos: i32 = pos;
      // java IOAssert.assertNotNull("mDocFieldList should not be null!", this.mDocFieldList);
      IOAssert::assertNotNull__string_object(&"mDocFieldList should not be null!", Some(&self.mDocFieldList));
      // java if (null != this.mDocFieldList && this.mDocFieldList.size() > pos)
      if let Some(mDocFieldList) = self.mDocFieldList.as_mut() {
        if self.mDocFieldList.size() > pos {
          // java currentPos = this.onImportDocField(this.mDocFieldList, subDocument, startGCP, pos, endGCP, count, loadCp);
          currentPos = self.onImportDocField__docfields_document_int_int_int_int_int(&mDocFieldList, &subDocument, startGCP, pos, endGCP, count, loadCp);
        }
      }
      // java return currentPos;
      return currentPos;
    }

    // java private int onImportDocField(DocFields docFields, Document subDocument, int startGCP, int pos, int endGCP, int count, int loadCp)
    pub fn onImportDocField__docfields_document_int_int_int_int_int(&mut self, docFields: &DocFields, subDocument: &dyn Document, startGCP: i32, pos: i32, endGCP: i32, count: i32, loadCp: i32) -> i32 {
      // java PLCDocumentFields plcDocumentFields = subDocument.getDocumentFileds();
      let mut plcDocumentFields: PLCDocumentFields = subDocument.getDocumentFileds();
      // java int startPos;
      let mut startPos: i32;
      startPos = pos;
      // java for(startPos = pos startPos < count ++startPos)
      while startPos < count {
        // java DocField docField = docFields.get(startPos);
        let mut docField: DocField = docFields.get(startPos);
        // java if (null == docField)
        if docField.is_none() {
          // java break;
          break;
        }
        // java int nStartGCP = docField.getGcpStart();
        let mut nStartGCP: i32 = docField.getGcpStart();
        // java int nEndGCP = docField.getGcpEnd();
        let mut nEndGCP: i32 = docField.getGcpEnd();
        // java if (nStartGCP >= endGCP || nStartGCP > nEndGCP || nEndGCP > endGCP)
        if nStartGCP >= endGCP || nStartGCP > nEndGCP || nEndGCP > endGCP {
          // java break;
          break;
        }
        // java int start = nStartGCP - startGCP + loadCp;
        let mut start: i32 = nStartGCP - startGCP + loadCp;
        // java int end = nEndGCP - startGCP + loadCp;
        let mut end: i32 = nEndGCP - startGCP + loadCp;
        // java PLCDocumentFields.DocuemntFieldNode node = plcDocumentFields.addDocumentField(start, end);
        let mut node: PLCDocumentFields::DocuemntFieldNode = plcDocumentFields.addDocumentField(start, end);
        // java if (null == node)
        if node.is_none() {
          // java break;
          break;
        }
        // java node.setName(docField.getName());
        node.setName(docField.getName());
        // java this.importBooleanAttr(node, docField.getBooleanAttr());
        self.importBooleanAttr(&mut node, docField.getBooleanAttr());
      }
      // java return startPos;
      return startPos;
    }

    // java private void importBooleanAttr(PLCDocumentFields.DocuemntFieldNode node, byte booleanAttr)
    pub fn importBooleanAttr(&mut self, node: &mut PLCDocumentFields::DocuemntFieldNode, booleanAttr: u8) {
      // java byte P = 4;
      let mut P: u8 = 4;
      // java byte R = 2;
      let mut R: u8 = 2;
      // java byte H = 1;
      let mut H: u8 = 1;
      // java node.setPrintOut((booleanAttr & 4) != 0);
      node.setPrintOut(booleanAttr & 4 != 0);
      // java node.setReadOnly((booleanAttr & 2) != 0);
      node.setReadOnly(booleanAttr & 2 != 0);
      // java node.setHidden((booleanAttr & 1) != 0);
      node.setHidden(booleanAttr & 1 != 0);
    }

}
