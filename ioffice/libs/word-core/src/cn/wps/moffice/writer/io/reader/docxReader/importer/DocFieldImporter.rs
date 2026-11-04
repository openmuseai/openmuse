// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.PLCDocumentFields
use crate::cn::wps::moffice::writer::data::PLCDocumentFields::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import java.util.HashMap
// Java: import org.xml.sax.Attributes
use kfc_kfc_alpha::kfc::html::reader::lexical::token::Attributes::*;
use std::collections::{HashMap};

// java public class DocFieldImporter
#[derive(Default)]
pub struct DocFieldImporter {
    // java private Document mDocument;
    mDocument: Option<Document>,
    // java private HashMap<Integer, PLCDocumentFields.FieldProps> mId2FieldProps;
    mId2FieldProps: Option<HashMap<i32, PLCDocumentFields::FieldProps>>,
    // java private HashMap<Integer, Integer> mId2FieldStartCP;
    mId2FieldStartCP: Option<HashMap<i32, i32>>,
}

// java public class DocFieldImporter
impl DocFieldImporter {
    // java public  DocFieldImporter(Document document)
    pub fn DocFieldImporter(document: &dyn Document) -> Self {
      // java this.mDocument = document;
      Self { mDocument: document, mId2FieldProps: HashMap::new(), mId2FieldStartCP: HashMap::new() }
      // java this.mId2FieldProps = new HashMap();
      // java this.mId2FieldStartCP = new HashMap();
    }

    // java public void importDocFieldStart(Attributes attrs)
    pub fn importDocFieldStart(&mut self, attrs: &Attributes) {
      // java Integer id = ConvertHelper.getInt(attrs, "id");
      let mut id: i32 = ConvertHelper::getInt(&attrs, &"id");
      // java IOAssert.assertNotNull("id should not be null", id);
      IOAssert::assertNotNull__string_object(&"id should not be null", Some(&id));
      // java if (null != id)
      if id.is_some() {
        // java PLCDocumentFields.FieldProps fieldProps = this.createDocumentFieldProps(attrs);
        let mut fieldProps: PLCDocumentFields::FieldProps = self.createDocumentFieldProps(&attrs);
        // java if (fieldProps != null)
        if fieldProps.is_some() {
          // java this.mId2FieldProps.put(id, fieldProps);
          self.mId2FieldProps.put(id, fieldProps);
          // java this.mId2FieldStartCP.put(id, this.mDocument.getLength());
          self.mId2FieldStartCP.put(id, self.mDocument.getLength());
        }
      }
    }

    // java public void importDocFieldEnd(Attributes attrs)
    pub fn importDocFieldEnd(&mut self, attrs: &Attributes) {
      // java Integer id = ConvertHelper.getInt(attrs, "id");
      let mut id: i32 = ConvertHelper::getInt(&attrs, &"id");
      // java IOAssert.assertNotNull("id should not be null", id);
      IOAssert::assertNotNull__string_object(&"id should not be null", Some(&id));
      // java if (null != id)
      if id.is_some() {
        // java PLCDocumentFields.FieldProps fieldProps = (PLCDocumentFields.FieldProps)this.mId2FieldProps.get(id);
        let mut fieldProps: PLCDocumentFields::FieldProps = (self.mId2FieldProps.get(id)).downcast_ref::<PLCDocumentFields::FieldProps>().unwrap();
        // java Integer startCP = (Integer)this.mId2FieldStartCP.get(id);
        let mut startCP: i32 = ((self.mId2FieldStartCP.get(id)) as i32);
        // java if (fieldProps != null && startCP != null)
        if fieldProps.is_some() && startCP.is_some() {
          // java this.importDocumentField(startCP, fieldProps);
          self.importDocumentField(startCP, &fieldProps);
        }
      }
    }

    // java private void importDocumentField(int start, PLCDocumentFields.FieldProps fieldProps)
    pub fn importDocumentField(&self, start: i32, fieldProps: &PLCDocumentFields::FieldProps) {
      // java int end = this.mDocument.getLength();
      let mut end: i32 = self.mDocument.getLength();
      // java PLCDocumentFields.DocuemntFieldNode fieldNode = this.mDocument.getDocumentFileds().addDocumentField(start, end);
      let mut fieldNode: PLCDocumentFields::DocuemntFieldNode = self.mDocument.getDocumentFileds().addDocumentField(start, end);
      // java IOAssert.assertNotNull("fieldNode should not be null!", fieldNode);
      IOAssert::assertNotNull__string_object(&"fieldNode should not be null!", Some(&fieldNode));
      // java if (fieldNode != null)
      if fieldNode.is_some() {
        // java fieldNode.setFieldProp(fieldProps);
        fieldNode.setFieldProp(fieldProps);
        // java TextDocument textDocument = this.mDocument.getTextDocument();
        let mut textDocument: TextDocument = self.mDocument.getTextDocument();
        // java if (textDocument != null && textDocument.isDebugging())
        if textDocument.is_some() && textDocument.isDebugging() {
          // java textDocument.onDebug("read.dfd", this.mDocument, fieldNode);
          textDocument.onDebug("read.dfd", self.mDocument, fieldNode);
        }
      }
    }

    // java private PLCDocumentFields.FieldProps createDocumentFieldProps(Attributes attrs)
    pub fn createDocumentFieldProps(&mut self, attrs: &Attributes) -> Option<PLCDocumentFields::FieldProps> {
      // java String docfieldname = ConvertHelper.getString(attrs, "docfieldname");
      let mut docfieldname: String = ConvertHelper::getString__attributes_string(&attrs, &"docfieldname");
      // java if (docfieldname == null)
      if docfieldname.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Boolean hidden = ConvertHelper.getBoolean(attrs, "hidden");
        let mut hidden: bool = ConvertHelper::getBoolean(&attrs, &"hidden");
        // java Boolean print = ConvertHelper.getBoolean(attrs, "print");
        let mut print: bool = ConvertHelper::getBoolean(&attrs, &"print");
        // java Boolean readonly = ConvertHelper.getBoolean(attrs, "readonly");
        let mut readonly: bool = ConvertHelper::getBoolean(&attrs, &"readonly");
        // java return new PLCDocumentFields.FieldProps(docfieldname, readonly != null && readonly, hidden != null && hidden, print != null ? print : true);
        return FieldProps::new(docfieldname, readonly.is_some() && readonly, hidden.is_some() && hidden, (if print.is_some() { Some(print) } else { Some(true) }));
      }
    }

}
