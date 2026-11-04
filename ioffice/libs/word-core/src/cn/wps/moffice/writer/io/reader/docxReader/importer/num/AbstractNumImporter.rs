// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Utils
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use super::AbstractNumIdRedirection::AbstractNumIdRedirection;
use super::IOAbstractNum::IOAbstractNum;
use super::IOLvl::IOLvl;
use super::NumberingLevelImporter::NumberingLevelImporter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// java class AbstractNumImporter extends NumberingLevelImporter
pub struct AbstractNumImporter {
    level: NumberingLevelImporter,
    // java private HashMap<String, LstData> mStyleIdLstDataMap = new HashMap();
    mStyleIdLstDataMap: HashMap<String, LstData>,
    // java private List<IOAbstractNum> mNumStyleLinkList = new ArrayList();
    mNumStyleLinkList: Vec<IOAbstractNum>,
    // java private boolean mIsFinished;
    mIsFinished: bool,
    // java private Integer mAbstractNumId;
    mAbstractNumId: Option<i32>,
    // java private IOAbstractNum mIOAbstractNum;
    mIOAbstractNum: Option<IOAbstractNum>,
}

// java class AbstractNumImporter extends NumberingLevelImporter
impl AbstractNumImporter {
    pub fn AbstractNumImporter(
        document: *mut TextDocument,
        styleId2Sti: Rc<RefCell<StyleId2Sti>>,
        numIdRedirection: Rc<RefCell<NumIdRedirection>>,
        abstractNumIdRedirect: Rc<RefCell<AbstractNumIdRedirection>>,
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
      let level = NumberingLevelImporter::NumberingLevelImporter(
        document, styleId2Sti, numIdRedirection, abstractNumIdRedirect, mode, xml07Context, xml03Context,
      );
      Self {
        level,
        mStyleIdLstDataMap: HashMap::new(),
        mNumStyleLinkList: Vec::new(),
        mIsFinished: false,
        mAbstractNumId: None,
        mIOAbstractNum: None,
      }
    }

    // java boolean isFinished()
    pub fn isFinished(&self) -> bool {
      // java return this.mIsFinished;
      self.mIsFinished
    }

    // java void importAbstractNumsEnd()
    pub fn importAbstractNumsEnd(&mut self) {
      // java this.importNumStyleLinkAbstractNums();
      self.importNumStyleLinkAbstractNums();
      // java this.mIsFinished = true;
      self.mIsFinished = true;
    }

    // java void importAbstractNumStart(Attributes attributes)
    pub fn importAbstractNumStart(&mut self, attributes: &dyn Attributes) {
      let marker = Some(());
      IOAssert::assertNotNull_msg("attributes should not be null", &marker);
      // java this.mAbstractNumId = ShareConvertHelper.getInteger4STDecimalNumber(attributes, "abstractNumId");
      self.mAbstractNumId = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "abstractNumId");
      // java if (this.mAbstractNumId == null)
      if self.mAbstractNumId.is_none() {
        self.mAbstractNumId = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "listDefId");
      }
      // java if (null != this.mAbstractNumId && null == this.mIOAbstractNum)
      if self.mAbstractNumId.is_some() && self.mIOAbstractNum.is_none() {
        let mode = self.level.super_.mode();
        let xml07 = self.level.super_.xml07Context().cloned();
        let xml03 = self.level.super_.xml03Context().cloned();
        self.mIOAbstractNum = Some(IOAbstractNum::IOAbstractNum(mode, xml07, xml03));
      } else if let Some(cur) = self.mIOAbstractNum.as_mut() {
        // java this.mIOAbstractNum.dispose();
        cur.dispose();
      }
      // java this.mIOAbstractNum.mAbstractNumId = this.mAbstractNumId;
      if let Some(cur) = self.mIOAbstractNum.as_mut() {
        cur.mAbstractNumId = self.mAbstractNumId;
      }
    }

    // java void importAbstractNumChild(int elementId, Attributes attributes)
    pub fn importAbstractNumChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      // java if (null != this.mAbstractNumId)
      if self.mAbstractNumId.is_none() {
        return;
      }
      let is_xml03 = self.level.super_.isXml03();
      let Some(cur) = self.mIOAbstractNum.as_mut() else { return };
      match elementId {
        // java case 111096:
        111096 => {
          // java if (this.isXml03())
          if is_xml03 {
            let raw = ConvertHelper::getString(attributes);
            // java this.mIOAbstractNum.mMultiLevelType = Xml03Utils.firstCharToLowerCase(...);
            cur.mMultiLevelType = Xml03Utils::firstCharToLowerCase(raw.as_deref());
          }
        }
        // java case 3373707: fall through 3390880
        3373707 => {
          cur.mName = ConvertHelper::getString(attributes);
        }
        3390880 => {}
        // java case 3564085:
        3564085 => {
          cur.mTemplateCode = ShareConvertHelper::convertSTLongHexNumber(attributes);
        }
        // java case 717051813:
        717051813 => {
          cur.mMultiLevelType = ConvertHelper::getString(attributes);
        }
        // java case 1043350093: case 1093209189:
        1043350093 | 1093209189 => {
          cur.mNumStyleLink = ConvertHelper::getString(attributes);
        }
        // java case 1805058379:
        1805058379 => {
          cur.mStyleLink = ConvertHelper::getString(attributes);
        }
        _ => {
          IOAssert::fail__string("it should not reach here");
        }
      }
    }

    // java void importAbstractNumEnd()
    pub fn importAbstractNumEnd(&mut self) {
      if self.mAbstractNumId.is_none() {
        return;
      }
      let style_link = self.mIOAbstractNum.as_ref().is_some_and(|c| c.isStyleLinkExistence());
      let num_style_link = self.mIOAbstractNum.as_ref().is_some_and(|c| c.isNumStyleLinkExistence());
      if style_link {
        // java this.importStylelinkAbstractNum();
        let lst = self.importNormalAbstractNum();
        self.putCloneLstData2Map(lst);
      } else if num_style_link {
        // java this.mNumStyleLinkList.add(this.mIOAbstractNum);
        if let Some(cur) = self.mIOAbstractNum.take() {
          self.mNumStyleLinkList.push(cur);
        }
      } else {
        // java this.importNormalAbstractNum();
        let _ = self.importNormalAbstractNum();
      }
    }

    // java private LstData importNormalAbstractNum()
    fn importNormalAbstractNum(&mut self) -> LstData {
      let soft = SoftTextDocument::default();
      // java LstData lstData = new LstData(this.mDocument);
      let mut lstData = LstData::LstData(&soft);
      if let Some(cur) = self.mIOAbstractNum.as_mut() {
        let style = self.level.mStyleId2Sti.borrow().clone();
        let redirect = &mut *self.level.mAbstractNumIdRedirect.borrow_mut();
        // java this.mIOAbstractNum.convert2LstData(...);
        cur.convert2LstData(&soft, &style, redirect, &mut lstData);
      }
      // java this.mLstTable.add(lstData);
      let doc = unsafe { &mut *self.level.mDocument };
      doc.getAutoNumTable().getLstDataTable().add(&mut lstData);
      lstData
    }

    // java private void putCloneLstData2Map(LstData lstData)
    fn putCloneLstData2Map(&mut self, lstData: LstData) {
      // java this.mStyleIdLstDataMap.put(this.mIOAbstractNum.mStyleLink, lstData.clone());
      if let Some(link) = self.mIOAbstractNum.as_ref().and_then(|c| c.mStyleLink.clone()) {
        self.mStyleIdLstDataMap.insert(link, lstData.clone());
      }
    }

    // java private void importNumStyleLinkAbstractNums()
    fn importNumStyleLinkAbstractNums(&mut self) {
      let pending = std::mem::take(&mut self.mNumStyleLinkList);
      for mut ioAbstractNum in pending {
        // java LstData lstData = this.getLstDataByNumStyleLink(ioAbstractNum.mNumStyleLink).clone();
        let mut lstData = self.getLstDataByNumStyleLink(ioAbstractNum.mNumStyleLink.as_deref());
        let soft = SoftTextDocument::default();
        let style = self.level.mStyleId2Sti.borrow().clone();
        {
          let redirect = &mut *self.level.mAbstractNumIdRedirect.borrow_mut();
          ioAbstractNum.convert2LstData(&soft, &style, redirect, &mut lstData);
        }
        let doc = unsafe { &mut *self.level.mDocument };
        doc.getAutoNumTable().getLstDataTable().add(&mut lstData);
      }
    }

    // java private LstData getLstDataByNumStyleLink(String numStyleLink)
    fn getLstDataByNumStyleLink(&self, numStyleLink: Option<&str>) -> LstData {
      // java if (null != numStyleLink && numStyleLink.length() > 0)
      if let Some(link) = numStyleLink {
        if !link.is_empty() {
          if let Some(found) = self.mStyleIdLstDataMap.get(link) {
            return found.clone();
          }
        }
      }
      // java return null == lstData ? new LstData(this.mDocument) : lstData;
      LstData::LstData(&SoftTextDocument::default())
    }

    // java void importLevelStart
    pub fn importLevelStart(&mut self, attributes: &dyn Attributes) {
      let lvl = {
        let Some(cur) = self.mIOAbstractNum.as_mut() else { return };
        cur.obtainIOLvl() as *mut IOLvl
      };
      NumberingLevelImporter::importLevelStart(unsafe { &mut *lvl }, attributes);
    }

    pub fn importLevelEnd(&mut self, rPr: &MutablePropertySet, pPr: &MutablePropertySet) {
      let lvl = {
        let Some(cur) = self.mIOAbstractNum.as_mut() else { return };
        if cur.mLvlIndex < 0 { return; }
        let idx = cur.mLvlIndex as usize;
        let Some(slot) = cur.mIOLvlArr.get_mut(idx).and_then(|s| s.as_mut()) else { return };
        slot as *mut IOLvl
      };
      NumberingLevelImporter::importLevelEnd(unsafe { &mut *lvl }, rPr, pPr);
    }

    pub fn importLvlChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
      let lvl = {
        let Some(cur) = self.mIOAbstractNum.as_mut() else { return };
        if cur.mLvlIndex < 0 { return; }
        let idx = cur.mLvlIndex as usize;
        let Some(slot) = cur.mIOLvlArr.get_mut(idx).and_then(|s| s.as_mut()) else { return };
        slot as *mut IOLvl
      };
      self.level.importLvlChild(unsafe { &mut *lvl }, elementId, attributes);
    }
}
