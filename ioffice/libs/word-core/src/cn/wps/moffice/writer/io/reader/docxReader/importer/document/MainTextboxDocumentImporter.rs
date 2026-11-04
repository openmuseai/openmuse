// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.document.MainTextboxDocumentImporter
//
// ★M-STAGE1 P2-7 —— 由 auto-generated 桩（13 行 Java / 12 行桩）**做实**。
//   Java 原文只有 1 个构造器：转发给父类 `TextboxDocumentImporter`。

// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::NumIdRedirection;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.TextboxDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::TextboxDocumentImporter::TextboxDocumentImporter;

// java public class MainTextboxDocumentImporter extends TextboxDocumentImporter
#[derive(Default)]
pub struct MainTextboxDocumentImporter {
    // java TextboxDocumentImporter super_
    pub(crate) super_: TextboxDocumentImporter,
}

// java public class MainTextboxDocumentImporter extends TextboxDocumentImporter
impl MainTextboxDocumentImporter {
    // java public MainTextboxDocumentImporter(SubDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn MainTextboxDocumentImporter(document: &mut SubDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(document, styleId2Sti, numIdRedirection, initDrawingContainer, mode, xml07Context, xml03Context);
      Self {
        super_: TextboxDocumentImporter::TextboxDocumentImporter(
          document,
          styleId2Sti,
          numIdRedirection,
          initDrawingContainer,
          mode,
          xml07Context,
          xml03Context,
        ),
      }
    }
}

impl std::ops::Deref for MainTextboxDocumentImporter {
    type Target = TextboxDocumentImporter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for MainTextboxDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
