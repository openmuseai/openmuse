#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::IOListener;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.FootnoteEndnoteDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::FootnoteEndnoteDocumentImporter::FootnoteEndnoteDocumentImporter;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::NumIdRedirection;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;

// java public class EndnoteDocumentImporter extends FootnoteEndnoteDocumentImporter
pub struct EndnoteDocumentImporter {
    // java FootnoteEndnoteDocumentImporter super_
    pub(crate) super_: FootnoteEndnoteDocumentImporter,
}

// java public class EndnoteDocumentImporter extends FootnoteEndnoteDocumentImporter
impl EndnoteDocumentImporter {
    // java protected  EndnoteDocumentImporter(TextDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, IOListener ioListener, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn EndnoteDocumentImporter(document: &mut TextDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, ioListener: Option<Box<dyn IOListener>>, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super((SubDocument)document.createSubDocument(4), styleId2Sti, numIdRedirection, ioListener, initDrawingContainer, mode, xml07Context, xml03Context);
      // rust: M-STAGE2 P1-D1 -- Java 侧是 (SubDocument)document.createSubDocument(4) 强转；
      //   Rust 侧先调用 createSubDocument(4) 触发**真实创建**（Java 语义），
      //   再从 mSubdocuments 取 &mut 视图（与 DocumentImporter::subdoc_mut 同一做法）。
      document.createSubDocument(4);
      let sub_mut: &mut SubDocument = document
        .mSubdocuments
        .as_mut()
        .and_then(|v| v.get_mut(4 as usize))
        .and_then(|s| s.as_mut())
        .map(|b| b.as_mut())
        .expect("subdocument 4");
      Self { super_: FootnoteEndnoteDocumentImporter::FootnoteEndnoteDocumentImporter(sub_mut, styleId2Sti, numIdRedirection, ioListener, initDrawingContainer, mode, xml07Context, xml03Context) }
    }

    // java public void onImportEndnoteStart(Attributes footnoteEndnote)
    pub fn onImportEndnoteStart(&mut self, footnoteEndnote: &dyn Attributes) {
      // java IOAssert.assertNotNull("footnoteEndnote should not be null!", footnoteEndnote);
      // rust: no-stmt
      // java PLCFootnoteEndnoteTxt plcfootnoteTxt = this.mDocument.getEndnoteTxt();
      // java super.onImportFootnoteEndNoteStart(plcfootnoteTxt, footnoteEndnote);
      // rust: ★T7-8 —— 同脚注路径：Java 的 `getEndnoteTxt()` 返回**同一对象引用**，
      //   父类 `onImportFootnoteEndNoteStart` 需要 `&mut` 就地写入。
      //   原模板以「按值返回 + `todo!()`」绕过借用检查（= 假编译面 + 假延迟）。
      //   现取**真实可写别名**，写入直达 `SubDocument.mEndnotTxt`，与 Java 语义一致。
      let doc = self.super_.super_.mDocument.expect("mDocument");
      let plcfootnoteTxt = unsafe { (*doc).getEndnoteTxt_mut() }
        .expect("mDocument.getEndnoteTxt() must not be null");
      self.super_.onImportFootnoteEndNoteStart(plcfootnoteTxt, footnoteEndnote);
    }
}

impl std::ops::Deref for EndnoteDocumentImporter {
    type Target = FootnoteEndnoteDocumentImporter;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EndnoteDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
