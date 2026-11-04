// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.document.TextboxDocumentImporter
//
// ★M-STAGE1 P2-7 —— 由 auto-generated 桩（14 行 Java / 11 行桩）**做实**。
//   Java 原文只有 1 个构造器：转发给 `SubDocumentImporter`，并把 `IOListener` 传 `null`。
//   它是 `MainTextboxDocumentImporter` / `HeaderTextboxDocumentImporter` 的**公共父类**。

// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.core.io.IOListener
// rust: `(IOListener)null` —— `IOListener` 经 `SubDocumentImporter` 再导出，此处只需 `None`
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::NumIdRedirection;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.SubDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::SubDocumentImporter;

// java public abstract class TextboxDocumentImporter extends SubDocumentImporter
#[derive(Default)]
pub struct TextboxDocumentImporter {
    // java SubDocumentImporter super_
    pub(crate) super_: SubDocumentImporter,
}

// java public abstract class TextboxDocumentImporter extends SubDocumentImporter
impl TextboxDocumentImporter {
    // java protected TextboxDocumentImporter(SubDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn TextboxDocumentImporter(document: &mut SubDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(document, styleId2Sti, numIdRedirection, (IOListener)null, initDrawingContainer, mode, xml07Context, xml03Context);
      Self {
        super_: SubDocumentImporter::SubDocumentImporter(
          document,
          styleId2Sti,
          numIdRedirection,
          None,
          initDrawingContainer,
          mode,
          xml07Context,
          xml03Context,
        ),
      }
    }
}

impl std::ops::Deref for TextboxDocumentImporter {
    type Target = SubDocumentImporter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TextboxDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
