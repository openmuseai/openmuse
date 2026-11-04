// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use io_wps_io::cn::wps::io::xwpf::element_handler::numbering::NumLvlType::NumLvlType;
use super::AbstractNumIdRedirection::AbstractNumIdRedirection;
use super::AbstractNumImporter::AbstractNumImporter;
use super::NumImporter::NumImporter;
use std::cell::RefCell;
use std::rc::Rc;

// java public final class NumberingImporter
pub struct NumberingImporter {
    // java private NumImporter mNumImporter;
    mNumImporter: NumImporter,
    // java private AbstractNumImporter mAbstractNumImporter;
    mAbstractNumImporter: AbstractNumImporter,
}

// java public final class NumberingImporter
impl NumberingImporter {
    // java public NumberingImporter(TextDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn NumberingImporter(
        document: *mut TextDocument,
        styleId2Sti: Rc<RefCell<StyleId2Sti>>,
        numIdRedirection: Rc<RefCell<NumIdRedirection>>,
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
      let doc_opt = if document.is_null() { None } else { Some(()) };
      // java IOAssert.assertNotNull("document should not be null", document);
      IOAssert::assertNotNull_msg("document should not be null", &doc_opt);
      // java AbstractNumIdRedirection abstractNumIdRedirection = new AbstractNumIdRedirection(document.getUULsid());
      let uu = unsafe { (*document).getUULsid() as *mut _ };
      let redirect = Rc::new(RefCell::new(AbstractNumIdRedirection::AbstractNumIdRedirection(uu)));
      // java this.mNumImporter = new NumImporter(...);
      let mNumImporter = NumImporter::NumImporter(
        document, styleId2Sti.clone(), numIdRedirection.clone(), redirect.clone(),
        mode, xml07Context.clone(), xml03Context.clone(),
      );
      // java this.mAbstractNumImporter = new AbstractNumImporter(...);
      let mAbstractNumImporter = AbstractNumImporter::AbstractNumImporter(
        document, styleId2Sti, numIdRedirection, redirect, mode, xml07Context, xml03Context,
      );
      Self { mNumImporter, mAbstractNumImporter }
    }

    // java public void importNumberingNumStart(int elementId, Attributes attributes)
    pub fn importNumberingNumStart(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java if (!this.mAbstractNumImporter.isFinished())
      if !self.mAbstractNumImporter.isFinished() {
        // java this.mAbstractNumImporter.importAbstractNumsEnd();
        self.mAbstractNumImporter.importAbstractNumsEnd();
      }
      // java this.mNumImporter.importNumStart(elementId, attributes);
      self.mNumImporter.importNumStart(elementId, attributes);
    }

    // java public void importNumberingNumChild(int elementId, Attributes attributes)
    pub fn importNumberingNumChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      self.mNumImporter.importNumChild(elementId, attributes);
    }

    // java public void importNumberingNumEnd()
    pub fn importNumberingNumEnd(&mut self) {
      self.mNumImporter.importNumEnd();
    }

    // java public void importNumberingLevelStart(NumLvlType lvlType, Attributes attributes)
    pub fn importNumberingLevelStart(&mut self, lvlType: &NumLvlType, attributes: &dyn Attributes) {
      match lvlType {
        // java case Num:
        NumLvlType::Num => self.mNumImporter.importLevelStart(attributes),
        // java case AbstractNum:
        NumLvlType::AbstractNum => self.mAbstractNumImporter.importLevelStart(attributes),
      }
    }

    // java public void importNumberingLevelEnd(...)
    pub fn importNumberingLevelEnd(&mut self, lvlType: &NumLvlType, rPr: &MutablePropertySet, pPr: &MutablePropertySet) {
      match lvlType {
        NumLvlType::Num => self.mNumImporter.importLevelEnd(rPr, pPr),
        NumLvlType::AbstractNum => self.mAbstractNumImporter.importLevelEnd(rPr, pPr),
      }
    }

    // java public void importNumberingLevelChild(...)
    pub fn importNumberingLevelChild(&mut self, lvlType: &NumLvlType, elementId: i32, attributes: &dyn Attributes) {
      match lvlType {
        NumLvlType::Num => self.mNumImporter.importLvlChild(elementId, attributes),
        NumLvlType::AbstractNum => self.mAbstractNumImporter.importLvlChild(elementId, attributes),
      }
    }

    // java public void importNumberingAbstractNumStart(Attributes attributes)
    pub fn importNumberingAbstractNumStart(&mut self, attributes: &dyn Attributes) {
      self.mAbstractNumImporter.importAbstractNumStart(attributes);
    }

    // java public void importNumberingAbstractNumChild(int elementId, Attributes attributes)
    pub fn importNumberingAbstractNumChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      self.mAbstractNumImporter.importAbstractNumChild(elementId, attributes);
    }

    // java public void importNumberingAbstractNumEnd()
    pub fn importNumberingAbstractNumEnd(&mut self) {
      self.mAbstractNumImporter.importAbstractNumEnd();
    }
}
