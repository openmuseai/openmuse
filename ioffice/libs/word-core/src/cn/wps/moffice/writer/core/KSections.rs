// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.table.Tools
use crate::cn::wps::moffice::writer::core::table::Tools::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.KSection
use crate::cn::wps::moffice::writer::core::KSection::*;
// Java: import cn.wps.moffice.writer.core.KFirstSection
use crate::cn::wps::moffice::writer::core::KFirstSection::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.KSectionStart
use crate::cn::wps::moffice::writer::core::KSectionStart::*;

// java public final class KSections
#[derive(Default)]
pub struct KSections {
    // java private TextDocument mParent;
    mParent: Option<TextDocument>,
    // java private SubDocument mMainDocument;
    mMainDocument: Option<SubDocument>,
    // java private PLCSection mPLCSection;
    mPLCSection: Option<PLCSection>,
}

// java public final class KSections
impl KSections {
    // java public static final KSection getSectionByCp(TextDocument document, int cp)
    pub fn getSectionByCp(document: &TextDocument, cp: i32) -> KSection {
      // java KSections kSections = document.getSections();
      let mut kSections: KSections = document.getSections();
      // java int countSections = kSections.count();
      let mut countSections: i32 = kSections.count();
      // java KSection kSection = kSections.item(countSections - 1);
      let mut kSection: KSection = kSections.item(countSections - 1);
      let mut i: i32 = 1;
      // java for(int i = 1; i < countSections; ++i)
      while i < countSections {
        // java if (kSections.item(i).getSectionNode().getCP() >= cp)
        if kSections.item(i).getSectionNode().getCP() >= cp {
          // java kSection = kSections.item(i - 1);
          kSection = kSections.item(i - 1);
          // java break;
          break;
        }
      }
      // java return kSection;
      return kSection;
    }

    // java   KSections(TextDocument parent)
    pub fn KSections(parent: &TextDocument) -> Self {
      // java this.mParent = parent;
      Self { mParent: parent, mMainDocument: (self.mParent.getMainDocument()).downcast_ref::<SubDocument>().unwrap(), mPLCSection: self.mMainDocument.getPLCSection() }
      // java this.mMainDocument = (SubDocument)this.mParent.getMainDocument();
      // java this.mPLCSection = this.mMainDocument.getPLCSection();
    }

    // java public KSection item(int index)
    pub fn item(&self, index: i32) -> Option<KSection> {
      // java if (index < this.mPLCSection.size() && index >= 0)
      if index < self.mPLCSection.size() && index >= 0 {
        // java return (KSection)(0 == index ? new KFirstSection(this, (PLCSection.SectionNode)this.mPLCSection.get(index)) : new KSection(this, (PLCSection.SectionNode)this.mPLCSection.get(index)));
        return ((if 0 == index { Some(KFirstSection::new(self, (self.mPLCSection.get(index)).downcast_ref::<PLCSection::SectionNode>().unwrap())) } else { Some(KSection::new(self, (self.mPLCSection.get(index)).downcast_ref::<PLCSection::SectionNode>().unwrap())) })).downcast_ref::<KSection>().unwrap();
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public int count()
    pub fn count(&self) -> i32 {
      // java Assert.assertNotNull("mPLCSection should not be null!", this.mPLCSection);
      Assert::assertNotNull__string_object(&"mPLCSection should not be null!", Some(&self.mPLCSection));
      // java return this.mPLCSection.size();
      return self.mPLCSection.size();
    }

    // java public TextDocument getParent()
    pub fn getParent(&self) -> TextDocument {
      // java Assert.assertNotNull("mParent should not be null!", this.mParent);
      Assert::assertNotNull__string_object(&"mParent should not be null!", Some(&self.mParent));
      // java return this.mParent;
      return self.mParent;
    }

    // java public KSection add(KRange _kRange, KSectionStart _kSectionStart)
    pub fn add(&mut self, _kRange: Option<&KRange>, _kSectionStart: Option<&KSectionStart>) -> Option<KSection> {
      // java Assert.assertNotNull("mMainDocument should not be null!", this.mMainDocument);
      Assert::assertNotNull__string_object(&"mMainDocument should not be null!", Some(&self.mMainDocument));
      // java if (null == _kRange)
      if _kRange.is_none() {
        // java this.mMainDocument.getRange(this.mMainDocument.getLength());
        self.mMainDocument.getRange__int(self.mMainDocument.getLength());
      }
      // java KSectionStart sectionStart = null == _kSectionStart ? KSectionStart.SectionNewPage : _kSectionStart;
      let mut sectionStart: KSectionStart = (if _kSectionStart.is_none() { KSectionStart::SectionNewPage } else { _kSectionStart });
      // java return new KSection(this, this.insertSection());
      return KSection::new(self, self.insertSection());
    }

    // java public PLCSection getPLCSection()
    pub fn getPLCSection(&self) -> PLCSection {
      // java Assert.assertNotNull("mPLCSection should not be null!", this.mPLCSection);
      Assert::assertNotNull__string_object(&"mPLCSection should not be null!", Some(&self.mPLCSection));
      // java return this.mPLCSection;
      return self.mPLCSection;
    }

    // java private PLCSection.SectionNode insertSection()
    pub fn insertSection(&mut self) -> PLCSection::SectionNode {
      // java CoreHelper.appendParagraph(this.mMainDocument, '\f', PropertySet.empty, PropertySet.empty);
      CoreHelper::appendParagraph__subdocument_char_propertyset_propertyset(&self.mMainDocument, '\u{000c}', &PropertySet::empty(), &PropertySet::empty());
      // java CoreHelper.appendParagraph(this.mMainDocument, '\r', PropertySet.empty, PropertySet.empty);
      CoreHelper::appendParagraph__subdocument_char_propertyset_propertyset(&self.mMainDocument, '\r', &PropertySet::empty(), &PropertySet::empty());
      // java return this.mPLCSection.addSection(this.mMainDocument.getLength() - 2);
      return self.mPLCSection.addSection(self.mMainDocument.getLength() - 2);
    }

    // java public KSection addSection(KRange kRange, int sectionBreak)
    pub fn addSection(&mut self, kRange: &KRange, sectionBreak: i32) -> KSection {
      // java Assert.assertNotNull("mMainDocument should not be null!", this.mMainDocument);
      Assert::assertNotNull__string_object(&"mMainDocument should not be null!", Some(&self.mMainDocument));
      // java Assert.assertNotNull("kRange should not be null!", kRange);
      Assert::assertNotNull__string_object(&"kRange should not be null!", Some(&kRange));
      // java Assert.assertEquals(this.mMainDocument.getType(), kRange.getStoryType());
      Assert::assertEquals__object_object(&self.mMainDocument.getType(), &kRange.getStoryType());
      // java this.getParent().start();
      self.getParent().start();
      // java KSection var3;
      let mut var3: KSection = KSection::default();
          // java var3 = new KSection(this, this.insertSection(kRange, sectionBreak));
          var3 = KSection::new(self, self.insertSection__krange_int(&mut kRange, sectionBreak));
            // java this.getParent().commit("add section");
            self.getParent().commit("add section");
      // java return var3;
      return var3;
    }

    // java private PLCSection.SectionNode insertSection(KRange kRange, int sectionBreak)
    pub fn insertSection__krange_int(&mut self, kRange: &mut KRange, sectionBreak: i32) -> PLCSection::SectionNode {
      // java this.fix_range(kRange);
      self.fix_range(&mut kRange);
      // java Tools.insertParaBefore(kRange.getStory(), kRange.getStart(), '\f', PropertyUtil.getParagraphFormatForInsertBreak(kRange.getPapxEntry().getProperty()));
      Tools::insertParaBefore__document_int_char_propertyset(&kRange.getStory(), kRange.getStart(), '\u{000c}', &PropertyUtil::getParagraphFormatForInsertBreak(&kRange.getPapxEntry().getProperty()));
      // java PLCSection.SectionNode cur_section = getSectionByCp(kRange.getTextDocument(), kRange.getStart()).getSectionNode();
      let mut cur_section: PLCSection::SectionNode = Self::getSectionByCp(&kRange.getTextDocument(), kRange.getStart()).getSectionNode();
      // java PLCSection.SectionNode section = this.mPLCSection.addSection(kRange.getStart());
      let mut section: PLCSection::SectionNode = self.mPLCSection.addSection(kRange.getStart());
      // java MutablePropertySet cur_prop = new MutablePropertySet(cur_section.getPropertySet());
      let mut cur_prop: MutablePropertySet = MutablePropertySet::new(cur_section.getPropertySet());
      // java cur_prop.setValue(681, sectionBreak);
      cur_prop.setValue(681, sectionBreak);
      // java section.setPropertySet(cur_prop.freeze());
      section.setPropertySet(cur_prop.freeze());
      // java kRange.setRange(kRange.getStart() + 1, kRange.getStart() + 1);
      kRange.setRange(kRange.getStart() + 1, kRange.getStart() + 1);
      // java return section;
      return section;
    }

    // java private void fix_range(KRange kRange)
    pub fn fix_range(&mut self, kRange: &mut KRange) {
      // java TableInfo tableInfo = kRange.getSubDocument().getTableInfos().getTableInfoByLevel(kRange.getStart(), 1);
      let mut tableInfo: TableInfo = kRange.getSubDocument().getTableInfos().getTableInfoByLevel(kRange.getStart(), 1);
      // java if (tableInfo != null)
      if tableInfo.is_some() {
        // java RowInfo startRowInfo = tableInfo.getRowByCP(kRange.getStart());
        let mut startRowInfo: RowInfo = tableInfo.getRowByCP(kRange.getStart());
        // java kRange.setRange(startRowInfo.getStartCP(), startRowInfo.getStartCP());
        kRange.setRange(startRowInfo.getStartCP(), startRowInfo.getStartCP());
        // java if (startRowInfo.getIndex() != 0)
        if startRowInfo.getIndex() != 0 {
          // java TableTool.spilitTable(kRange.getSubDocument(), startRowInfo);
          TableTool::spilitTable(&kRange.getSubDocument(), &startRowInfo);
        }
      }
    }

}
