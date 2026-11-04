// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use super::AbstractNumIdRedirection::AbstractNumIdRedirection;
use super::IONum::IONum;
use super::NumberingLevelImporter::NumberingLevelImporter;
use std::cell::RefCell;
use std::rc::Rc;

// java class NumImporter extends NumberingLevelImporter
pub struct NumImporter {
    // java NumberingLevelImporter super
    level: NumberingLevelImporter,
    // java private IONum mIONum = null;
    mIONum: IONum,
}

// java class NumImporter extends NumberingLevelImporter
impl NumImporter {
    // java NumImporter(...)
    pub fn NumImporter(
        document: *mut TextDocument,
        styleId2Sti: Rc<RefCell<StyleId2Sti>>,
        numIdRedirection: Rc<RefCell<NumIdRedirection>>,
        abstractNumIdRedirect: Rc<RefCell<AbstractNumIdRedirection>>,
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
      // java super(...);
      let level = NumberingLevelImporter::NumberingLevelImporter(
        document, styleId2Sti, numIdRedirection, abstractNumIdRedirect, mode,
        xml07Context.clone(), xml03Context.clone(),
      );
      // java this.mIONum = new IONum(this.mode(), this.xml07Context(), this.xml03Context());
      let mIONum = IONum::IONum(mode, xml07Context, xml03Context);
      Self { level, mIONum }
    }

    // java void importNumStart(int elementId, Attributes attributes)
    pub fn importNumStart(&mut self, elementId: i32, attributes: &dyn Attributes) {
      let marker = Some(());
      IOAssert::assertNotNull_msg("attributes should not be null", &marker);
      match elementId {
        // java case 109446:
        109446 => {
          // java this.mIONum.mNumId = ShareConvertHelper.getInteger4STDecimalNumber(attributes, ATTR_NUMID);
          self.mIONum.mNumId = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "numId");
        }
        // java case 3322014:
        3322014 => {
          self.mIONum.mNumId = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "ilfo");
        }
        // java case 447078926:
        447078926 => {
          // java this.mIOLvlOverride = this.mIONum.obtainIOLvlOverride();
          let ov = self.mIONum.obtainIOLvlOverride();
          // java this.mIOLvlOverride.mIlvl = ShareConvertHelper.getInteger4STDecimalNumber(attributes, ATTR_ILVL);
          ov.mIlvl = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "ilvl");
        }
        _ => {
          IOAssert::fail__string("it should not reach here");
        }
      }
    }

    // java void importNumChild(int elementId, Attributes attributes)
    pub fn importNumChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      let marker = Some(());
      IOAssert::assertNotNull_msg("attributes should not be null", &marker);
      match elementId {
        // java case -2014130113: case 3235524:
        -2014130113 | 3235524 => {
          // java this.mIONum.mAbstractNumId = ShareConvertHelper.getInteger4STDecimalNumber(attributes);
          self.mIONum.mAbstractNumId = ShareConvertHelper::getInteger4STDecimalNumber(attributes);
        }
        // java case 657113678:
        657113678 => {
          // java this.mIOLvlOverride.mStartOverride = ...
          if let Some(ov) = self.mIONum.currentOverride() {
            ov.mStartOverride = ShareConvertHelper::getInteger4STDecimalNumber(attributes);
          }
        }
        _ => {
          IOAssert::fail__string("it should not reach here");
        }
      }
    }

    // java void importNumEnd()
    pub fn importNumEnd(&mut self) {
      // java if (this.mIONum.isValid())
      if self.mIONum.isValid() {
        let soft = SoftTextDocument::default();
        // java LfoData data = new LfoData(this.mDocument);
        let mut data = LfoData::LfoData(&soft);
        // java this.mIONum.convert2LfoData(this.mDocument, this.mAbstractNumIdRedirect, data);
        {
          let redirect = &mut *self.level.mAbstractNumIdRedirect.borrow_mut();
          self.mIONum.convert2LfoData(&soft, redirect, &mut data);
        }
        // java Integer redirectedNumId = this.mNumIdRedirection.getId(this.mIONum.mNumId);
        let num_id = self.mIONum.mNumId.unwrap_or(0);
        let mut redirected = self.level.mNumIdRedirection.borrow().getId(num_id);
        // java if (null == redirectedNumId)
        if redirected.is_none() {
          // java redirectedNumId = this.mNumIdRedirection.put(this.mIONum.mNumId);
          redirected = Some(self.level.mNumIdRedirection.borrow_mut().put(num_id));
        }
        // java this.mLfoDataTable.add(redirectedNumId, data);
        let doc = unsafe { &mut *self.level.mDocument };
        doc.getAutoNumTable().getLfoDataTable().add(redirected.unwrap_or(0), &mut data);
      }
      // java this.mIONum.reset();
      self.mIONum.reset();
    }

    // java void importLevelStart(Attributes attributes)
    pub fn importLevelStart(&mut self, attributes: &dyn Attributes) {
      let lvl = {
        let ov = self.mIONum.currentOverride();
        let Some(ov) = ov else { return };
        ov.getIOLvl() as *mut super::IOLvl::IOLvl
      };
      NumberingLevelImporter::importLevelStart(unsafe { &mut *lvl }, attributes);
    }

    // java void importLevelEnd(...)
    pub fn importLevelEnd(&mut self, rPr: &MutablePropertySet, pPr: &MutablePropertySet) {
      let lvl = {
        let ov = self.mIONum.currentOverride();
        let Some(ov) = ov else { return };
        ov.getIOLvl() as *mut super::IOLvl::IOLvl
      };
      NumberingLevelImporter::importLevelEnd(unsafe { &mut *lvl }, rPr, pPr);
    }

    // java void importLvlChild(...)
    pub fn importLvlChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      let lvl = {
        let ov = self.mIONum.currentOverride();
        let Some(ov) = ov else { return };
        ov.getIOLvl() as *mut super::IOLvl::IOLvl
      };
      self.level.importLvlChild(unsafe { &mut *lvl }, elementId, attributes);
    }
}
