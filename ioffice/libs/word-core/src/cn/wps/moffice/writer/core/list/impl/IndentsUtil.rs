// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.PropertyTool
use crate::cn::wps::moffice::writer::core::PropertyTool::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.property.IPropertyCalculate
/* DEFERRED_NON_FUNCTION —— `core::property` 闭包（`PropertyCalculateImpl` 31.5k 行
 *   + `StyleCache`）登记实测 99 错误 / 零管线收益；`core::PropertyTool` 单登记 48 错误。
 *   本文件仅经 `list/impl` 可达，不在 docx 解析→排版→渲染必径上 ⇒ 整文件延后。 */
use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::*;
// Java: import cn.wps.moffice.writer.core.property.PropertyCalculateFactory
use crate::cn::wps::moffice::writer::core::property::PropertyCalculateFactory::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.util.Pair
use util::util::Pair::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class IndentsUtil
#[derive(Default)]
pub struct IndentsUtil {
    // java private Document mDocument;
    mDocument: Option<Document>,
    // java private TextDocument mTextDoc;
    mTextDoc: Option<TextDocument>,
    // java private IPropertyCalculate mCal;
    mCal: Option<Box<dyn IPropertyCalculate>>,
}

// java public static class Indents
#[derive(Default)]
pub struct Indents {
    // java public Integer leftChars;
    pub(crate) leftChars: Option<i32>,
    // java public Integer left;
    pub(crate) left: Option<i32>,
    // java public Integer firstLineChars;
    pub(crate) firstLineChars: Option<i32>,
    // java public Integer firstLine;
    pub(crate) firstLine: Option<i32>,
}

// java public class IndentsUtil
impl IndentsUtil {
    // java public  IndentsUtil(Document document)
    pub fn IndentsUtil(document: &dyn Document) -> Self {
      // java Assert.assertNotNull("document should not be null.", document);
      Assert::assertNotNull__string_object(&"document should not be null.", Some(&document));
      // java this.mDocument = document;
      let mut this = Self { mDocument: document, mTextDoc: document.getTextDocument(), mCal: PropertyCalculateFactory::create(Some(&document)) };
      // java this.mTextDoc = document.getTextDocument();
      // java Assert.assertNotNull("mTextDoc should not be null.", this.mTextDoc);
      Assert::assertNotNull__string_object(&"mTextDoc should not be null.", Some(&this.mTextDoc));
      // java this.mCal = PropertyCalculateFactory.create(document);
      // java Assert.assertNotNull("mCal should not be null.", this.mCal);
      Assert::assertNotNull__string_object(&"mCal should not be null.", Some(&this.mCal));
      this
    }

    // java public static int convertLeftCharsToTwips(int leftChars, int startCP, Document doc)
    pub fn convertLeftCharsToTwips(leftChars: i32, startCP: i32, doc: &dyn Document) -> i32 {
      // java PLCSection.SectionNode section = doc.getTextDocument().getSections().getPLCSection().getSection(startCP);
      let mut section: PLCSection::SectionNode = doc.getTextDocument().getSections().getPLCSection().getSection(startCP);
      // java PropertySet props = section.getPropertySet();
      let mut props: PropertySet = section.getPropertySet();
      // java float gridCharWidth = props.getFloat(679, 10.5F);
      let mut gridCharWidth: f32 = props.getFloat(679, 10.5);
      // java return (int)((double)(gridCharWidth * (float)leftChars) * 0.01 * (double)20.0F);
      return ((((gridCharWidth * ((leftChars) as f32)) as f64) * 0.01 * ((20) as f64)) as i32);
    }

    // java public static int convertFirstCharsToTwips(int firstChars, int startCP, Document doc)
    pub fn convertFirstCharsToTwips(firstChars: i32, startCP: i32, doc: &dyn Document) -> i32 {
      // java PLCSection.SectionNode section = doc.getTextDocument().getSections().getPLCSection().getSection(startCP);
      let mut section: PLCSection::SectionNode = doc.getTextDocument().getSections().getPLCSection().getSection(startCP);
      // java PropertySet props = section.getPropertySet();
      let mut props: PropertySet = section.getPropertySet();
      // java float gridCharWidth = props.getFloat(679, 10.5F);
      let mut gridCharWidth: f32 = props.getFloat(679, 10.5);
      // java IChpxTable.Entry fragment = doc.getChpxTable().seek(startCP);
      let mut fragment: Box<dyn ChpxEntry> = doc.getChpxTable().seek(startCP);
      // java PropertyTool tool = new PropertyTool(doc);
      let mut tool: PropertyTool = PropertyTool::new(doc);
      // java PropertySet mergedChpx = tool.getMergedRunProperty(fragment);
      let mut mergedChpx: PropertySet = tool.getMergedRunProperty(fragment);
      // java float fontSize = mergedChpx.getFloat(10, gridCharWidth);
      let mut fontSize: f32 = mergedChpx.getFloat(10, gridCharWidth);
      // java return (int)((double)(fontSize * (float)firstChars) * 0.01 * (double)20.0F);
      return ((((fontSize * ((firstChars) as f32)) as f64) * 0.01 * ((20) as f64)) as i32);
    }

    // java public static boolean isExistCharIndent(Integer chars)
    pub fn isExistCharIndent(chars: i32) -> Option<bool> {
      // java return null != chars && 0 != chars;
      return true && 0 != chars;
    }

    // java public static boolean isExistTwipsIndent(Integer twips)
    pub fn isExistTwipsIndent(twips: i32) -> Option<bool> {
      // java return null != twips;
      return true;
    }

    // java public Indents getIndentEffect(IPapxTable.Entry para)
    pub fn getIndentEffect(&self, para: &dyn PapxEntry) -> Indents {
      // java Assert.assertNotNull("para should not be null.", para);
      Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
      // java PropertySet mergedPapx = this.mCal.getPap(para, 3);
      let mut mergedPapx: PropertySet = self.mCal.getPap__entry_int(&para, 3);
      // java Assert.assertNotNull("mergedPapx should not be null.", mergedPapx);
      Assert::assertNotNull__string_object(&"mergedPapx should not be null.", Some(&mergedPapx));
      // java return this.getIndents(mergedPapx);
      return self.getIndents(&mergedPapx);
    }

    // java public Pair<Integer, Integer> getIndentEffectNormalized(IPapxTable.Entry para)
    pub fn getIndentEffectNormalized__entry(&self, para: &mut dyn PapxEntry) -> Pair {
      // java Assert.assertNotNull("para should not be null.", para);
      Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
      // java Indents mergedIndents = this.getIndentEffect(para);
      let mut mergedIndents: Indents = self.getIndentEffect(&para);
      // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
      Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
      // java int startCP = para.position();
      let mut startCP: i32 = para.position();
      // java return this.getIndentEffectNormalized(startCP, mergedIndents);
      return self.getIndentEffectNormalized__int_indents(startCP, &mergedIndents);
    }

    // java public Pair<Integer, Integer> getIndentEffectNormalized(int startCP, Indents mergedIndents)
    pub fn getIndentEffectNormalized__int_indents(&self, startCP: i32, mergedIndents: &Indents) -> Pair {
      // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
      Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
      // java int firstLine = 0;
      let mut firstLine: i32 = 0;
      // java if (isExistCharIndent(mergedIndents.firstLineChars))
      if Self::isExistCharIndent(mergedIndents.firstLineChars) {
        // java firstLine = convertFirstCharsToTwips(mergedIndents.firstLineChars, startCP, this.mDocument);
        firstLine = Self::convertFirstCharsToTwips(mergedIndents.firstLineChars, startCP, &self.mDocument);
      }
      // java else if (isExistTwipsIndent(mergedIndents.firstLine))
      else if Self::isExistTwipsIndent(mergedIndents.firstLine) {
        // java firstLine = mergedIndents.firstLine;
        firstLine = mergedIndents.firstLine;
      }
      // java int left = 0;
      let mut left: i32 = 0;
      // java if (isExistCharIndent(mergedIndents.leftChars))
      if Self::isExistCharIndent(mergedIndents.leftChars) {
        // java left = convertLeftCharsToTwips(mergedIndents.leftChars, startCP, this.mDocument);
        left = Self::convertLeftCharsToTwips(mergedIndents.leftChars, startCP, &self.mDocument);
        // java if (isExistCharIndent(mergedIndents.firstLineChars) && isHangingChars(mergedIndents.firstLineChars))
        if Self::isExistCharIndent(mergedIndents.firstLineChars) && Self::isHangingChars(mergedIndents.firstLineChars) {
          // java left += firstLine;
          left = left + firstLine;
        }
      }
      // java else if (!isExistTwipsIndent(mergedIndents.left) || isExistCharIndent(mergedIndents.firstLineChars) && isHangingChars(mergedIndents.firstLineChars))
      else if !Self::isExistTwipsIndent(mergedIndents.left) || Self::isExistCharIndent(mergedIndents.firstLineChars) && Self::isHangingChars(mergedIndents.firstLineChars) {
        // java if (isExistCharIndent(mergedIndents.firstLineChars) && isHangingChars(mergedIndents.firstLineChars))
        if Self::isExistCharIndent(mergedIndents.firstLineChars) && Self::isHangingChars(mergedIndents.firstLineChars) {
          // java left += Math.abs(firstLine);
          left = left + Math::abs(firstLine);
        }
      }
      // java else
      else {
        // java left = mergedIndents.left;
        left = mergedIndents.left;
      }
      // java return new Pair(left, firstLine);
      return Pair::new(left, firstLine);
    }

    // java public Indents getIndents(PropertySet papx)
    pub fn getIndents(&self, papx: &PropertySet) -> Indents {
      // java Assert.assertNotNull("papx should not be null.", papx);
      Assert::assertNotNull__string_object(&"papx should not be null.", Some(&papx));
      // java Indents indents = new Indents();
      let mut indents: Indents = Indents::new();
      // java if (papx.contains(211))
      if papx.contains(211) {
        // java indents.leftChars = (Integer)papx.getValue(211);
        indents.leftChars = ((papx.getValue(211)) as i32);
      }
      // java if (papx.contains(210))
      if papx.contains(210) {
        // java indents.left = (Integer)papx.getValue(210);
        indents.left = ((papx.getValue(210)) as i32);
      }
      // java if (papx.contains(209))
      if papx.contains(209) {
        // java indents.firstLineChars = (Integer)papx.getValue(209);
        indents.firstLineChars = ((papx.getValue(209)) as i32);
      }
      // java if (papx.contains(208))
      if papx.contains(208) {
        // java indents.firstLine = (Integer)papx.getValue(208);
        indents.firstLine = ((papx.getValue(208)) as i32);
      }
      // java return indents;
      return indents;
    }

    // java public void removeIndents(IPapxTable.Entry para)
    pub fn removeIndents(&self, para: &mut dyn PapxEntry) {
      // java Assert.assertNotNull("para should not be null.", para);
      Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
      // java MutablePropertySet mutableProps = new MutablePropertySet();
      let mut mutableProps: MutablePropertySet = MutablePropertySet::new();
      // java PropertySet finalParaProp = PropertyUtil.getFinalNormalParageraphFormat(para.getProperty());
      let mut finalParaProp: PropertySet = PropertyUtil::getFinalNormalParageraphFormat(&para.getProperty());
      // java if (finalParaProp.contains(211))
      if finalParaProp.contains(211) {
        // java mutableProps.setValue(211, 0);
        mutableProps.setValue(211, 0);
      }
      // java if (finalParaProp.contains(209))
      if finalParaProp.contains(209) {
        // java mutableProps.setValue(209, 0);
        mutableProps.setValue(209, 0);
      }
      // java if (finalParaProp.contains(210))
      if finalParaProp.contains(210) {
        // java mutableProps.setValue(210, 0);
        mutableProps.setValue(210, 0);
      }
      // java if (finalParaProp.contains(208))
      if finalParaProp.contains(208) {
        // java mutableProps.setValue(208, 0);
        mutableProps.setValue(208, 0);
      }
      // java KRange paraRange = new KRange(this.mDocument, para.position(), para.endCp());
      let mut paraRange: KRange = KRange::new(self.mDocument, para.position(), para.endCp());
      // java paraRange.setParagraphProperty(mutableProps.freeze(), 3);
      paraRange.setParagraphProperty(mutableProps.freeze(), 3);
      // java paraRange.dispose();
      paraRange.dispose();
    }

    // java public void setIndents(IPapxTable.Entry para, Indents indents)
    pub fn setIndents(&self, para: &mut dyn PapxEntry, indents: &Indents) {
      // java Assert.assertNotNull("para should not be null.", para);
      Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
      // java PropertySet paraProps = para.getProperty();
      let mut paraProps: PropertySet = para.getProperty();
      // java Assert.assertNotNull("paraProps should not be null.", paraProps);
      Assert::assertNotNull__string_object(&"paraProps should not be null.", Some(&paraProps));
      // java MutablePropertySet mutableParaProps = new MutablePropertySet();
      let mut mutableParaProps: MutablePropertySet = MutablePropertySet::new();
      // java Integer leftChars = indents.leftChars;
      let mut leftChars: i32 = indents.leftChars;
      // java if (null != leftChars)
      if leftChars.is_some() {
        // java mutableParaProps.setValue(211, leftChars);
        mutableParaProps.setValue(211, leftChars);
      }
      // java Integer left = indents.left;
      let mut left: i32 = indents.left;
      // java if (null != left)
      if left.is_some() {
        // java mutableParaProps.setValue(210, left);
        mutableParaProps.setValue(210, left);
      }
      // java Integer firstLineChars = indents.firstLineChars;
      let mut firstLineChars: i32 = indents.firstLineChars;
      // java if (null != firstLineChars)
      if firstLineChars.is_some() {
        // java mutableParaProps.setValue(209, firstLineChars);
        mutableParaProps.setValue(209, firstLineChars);
      }
      // java Integer firstLine = indents.firstLine;
      let mut firstLine: i32 = indents.firstLine;
      // java if (null != firstLine)
      if firstLine.is_some() {
        // java mutableParaProps.setValue(208, firstLine);
        mutableParaProps.setValue(208, firstLine);
      }
      // java MutablePropertySet mutaChange = new MutablePropertySet(PropertyUtil.getFinalParagraphFormat(para.getProperty()));
      let mut mutaChange: MutablePropertySet = MutablePropertySet::new(PropertyUtil::getFinalParagraphFormat(&para.getProperty()));
      // java mutableParaProps.differenceSet(mutaChange);
      mutableParaProps.differenceSet(mutaChange);
      // java if (!mutableParaProps.isEmpty())
      if !mutableParaProps.is_empty() {
        // java KRange paraRange = new KRange(this.mDocument, para.position(), para.endCp());
        let mut paraRange: KRange = KRange::new(self.mDocument, para.position(), para.endCp());
        // java paraRange.setParagraphProperty(mutableParaProps.freeze(), 2);
        paraRange.setParagraphProperty(mutableParaProps.freeze(), 2);
        // java paraRange.dispose();
        paraRange.dispose();
      }
    }

    // java public boolean deleteIndent(IPapxTable.Entry para)
    pub fn deleteIndent(&mut self, para: &mut dyn PapxEntry) -> bool {
      // java Assert.assertNotNull("para should not be null.", para);
      Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
      // java PropertySet mergedProps = this.mCal.getPap(para, 1);
      let mut mergedProps: PropertySet = self.mCal.getPap__entry_int(&para, 1);
      // java Assert.assertNotNull("mergedProps should not be null.", mergedProps);
      Assert::assertNotNull__string_object(&"mergedProps should not be null.", Some(&mergedProps));
      // java Pair<Integer, Integer> normalizedIndent = this.getIndentEffectNormalized(para);
      let mut normalizedIndent: Pair = self.getIndentEffectNormalized__entry(&mut para);
      // java int leftAbsPos = (Integer)normalizedIndent.first;
      let mut leftAbsPos: i32 = ((normalizedIndent.first) as i32);
      // java int firstLineRel = (Integer)normalizedIndent.second;
      let mut firstLineRel: i32 = ((normalizedIndent.second) as i32);
      // java if (0 == leftAbsPos && 0 == firstLineRel)
      if 0 == leftAbsPos && 0 == firstLineRel {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.start();
        self.start();
            // java int newLeftAbsPos = 0;
            let mut newLeftAbsPos: i32 = 0;
            // java int newFirstLineRel = 0;
            let mut newFirstLineRel: i32 = 0;
            // java if (firstLineRel > 0)
            if firstLineRel > 0 {
              // java newLeftAbsPos = leftAbsPos;
              newLeftAbsPos = leftAbsPos;
              // java newFirstLineRel = 0;
              newFirstLineRel = 0;
            }
            // java else if (firstLineRel < 0)
            else if firstLineRel < 0 {
              // java newLeftAbsPos = leftAbsPos + firstLineRel;
              newLeftAbsPos = leftAbsPos + firstLineRel;
              // java newFirstLineRel = 0;
              newFirstLineRel = 0;
            }
            // java else
            else {
              // java newLeftAbsPos = 0;
              newLeftAbsPos = 0;
              // java newFirstLineRel = 0;
              newFirstLineRel = 0;
              // java Integer paraStyle = (Integer)mergedProps.getValue(191);
              let mut paraStyle: i32 = ((mergedProps.getValue(191)) as i32);
              // java if (null != paraStyle && 179 == paraStyle)
              if paraStyle.is_some() && 179 == paraStyle {
                // java KRange paraRange = new KRange(this.mDocument, para.position(), para.endCp());
                let mut paraRange: KRange = KRange::new(self.mDocument, para.position(), para.endCp());
                // java paraRange.setParagraphProperty(PropertySet.valueOf(191, 0), 3);
                paraRange.setParagraphProperty(PropertySet::valueOf__int_object(191, &0), 3);
                // java paraRange.dispose();
                paraRange.dispose();
              }
            }
            // java this.removeIndents(para);
            self.removeIndents(&mut para);
            // java Indents indentsToBeInEffect = new Indents();
            let mut indentsToBeInEffect: Indents = Indents::new();
            // java indentsToBeInEffect.left = newLeftAbsPos;
            indentsToBeInEffect.left = newLeftAbsPos;
            // java indentsToBeInEffect.firstLine = newFirstLineRel;
            indentsToBeInEffect.firstLine = newFirstLineRel;
            // java Indents mergedIndents = this.getIndentEffect(para);
            let mut mergedIndents: Indents = self.getIndentEffect(&para);
            // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
            Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
            // java if (null != mergedIndents.left || newLeftAbsPos != 0)
            if mergedIndents.left.is_some() || newLeftAbsPos != 0 {
              // java mergedIndents.left = newLeftAbsPos;
              mergedIndents.left = newLeftAbsPos;
            }
            // java if (null != mergedIndents.firstLine || newFirstLineRel != 0)
            if mergedIndents.firstLine.is_some() || newFirstLineRel != 0 {
              // java mergedIndents.firstLine = newFirstLineRel;
              mergedIndents.firstLine = newFirstLineRel;
            }
            // java Indents preventIndents = this.getHelpPreventIndents(indentsToBeInEffect, mergedIndents);
            let mut preventIndents: Indents = self.getHelpPreventIndents(&indentsToBeInEffect, &mergedIndents);
            // java if (null != preventIndents)
            if preventIndents.is_some() {
              // java if (null != preventIndents.leftChars)
              if preventIndents.leftChars.is_some() {
                // java mergedIndents.leftChars = preventIndents.leftChars;
                mergedIndents.leftChars = preventIndents.leftChars;
              }
              // java if (null != preventIndents.firstLineChars)
              if preventIndents.firstLineChars.is_some() {
                // java mergedIndents.firstLineChars = preventIndents.firstLineChars;
                mergedIndents.firstLineChars = preventIndents.firstLineChars;
              }
            }
            // java this.setIndents(para, mergedIndents);
            self.setIndents(&mut para, &mergedIndents);
              // java this.commit("delete range");
              self.commit(&"delete range");
        // java return true;
        return true;
      }
    }

    // java public void ensureNumberingIndentEffect(IPapxTable.Entry para)
    pub fn ensureNumberingIndentEffect(&mut self, para: &dyn PapxEntry) {
      // java Assert.assertNotNull("para should not be null.", para);
      Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
      // java PropertySet mergedPapx = this.mCal.getPap(para, 3);
      let mut mergedPapx: PropertySet = self.mCal.getPap__entry_int(&para, 3);
      // java Assert.assertNotNull("mergedPapx should not be null.", mergedPapx);
      Assert::assertNotNull__string_object(&"mergedPapx should not be null.", Some(&mergedPapx));
      // java PropertySet numPapx = this.getLevelFormat(mergedPapx);
      let mut numPapx: PropertySet = self.getLevelFormat(&mergedPapx);
      // java if (null == numPapx)
      if numPapx.is_none() {
        // java mergedPapx = this.mCal.getPap(para, 2);
        mergedPapx = self.mCal.getPap__entry_int(&para, 2);
        // java numPapx = this.getLevelFormat(mergedPapx);
        numPapx = self.getLevelFormat(&mergedPapx);
      }
      // java Assert.assertNotNull("numPapx should not be null.", numPapx);
      Assert::assertNotNull__string_object(&"numPapx should not be null.", Some(&numPapx));
      // java Indents numIndents = this.getIndents(numPapx);
      let mut numIndents: Indents = self.getIndents(&numPapx);
      // java Assert.assertNotNull("numIndents should not be null.", numIndents);
      Assert::assertNotNull__string_object(&"numIndents should not be null.", Some(&numIndents));
      // java Indents mergedIndents = this.getIndents(mergedPapx);
      let mut mergedIndents: Indents = self.getIndents(&mergedPapx);
      // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
      Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
      // java Indents helpPreventIndents = this.getHelpPreventIndents(numIndents, mergedIndents);
      let mut helpPreventIndents: Indents = self.getHelpPreventIndents(&numIndents, &mergedIndents);
      // java if (null != helpPreventIndents)
      if helpPreventIndents.is_some() {
        // java this.setIndents(para, helpPreventIndents);
        self.setIndents(&mut para, &helpPreventIndents);
      }
    }

    // java public Pair<Integer, Integer> checkIfIndentsDiff(IPapxTable.Entry[] paras)
    pub fn checkIfIndentsDiff(&self, paras: &dyn PapxEntry) -> Option<Pair> {
      // java Assert.assertNotNull("paras should not be null.", paras);
      Assert::assertNotNull__string_object(&"paras should not be null.", Some(&paras));
      // java Pair<Integer, Integer> sameIndent = null;
      let mut sameIndent: Pair = Pair::default();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut len: i32 = (paras.len() as i32);
      // java for(int len = paras.length; i < len; ++i)
      while i < len {
        // java IPapxTable.Entry para = paras[i];
        let mut para: Box<dyn PapxEntry> = paras[(i) as usize];
        // java Pair<Integer, Integer> indent = this.getIndentEffectNormalized(para);
        let mut indent: Pair = self.getIndentEffectNormalized__entry(&mut para);
        // java Assert.assertNotNull("indent should not be null.", indent);
        Assert::assertNotNull__string_object(&"indent should not be null.", Some(&indent));
        // java Assert.assertNotNull("indent.first should not be null.", indent.first);
        Assert::assertNotNull__string_object(&"indent.first should not be null.", Some(&indent.first));
        // java Assert.assertNotNull("indent.second should not be null.", indent.second);
        Assert::assertNotNull__string_object(&"indent.second should not be null.", Some(&indent.second));
        // java if (0 == i)
        if 0 == i {
          // java sameIndent = indent;
          sameIndent = indent;
        }
        // java else
        else {
          // java int sameLeft = (Integer)sameIndent.first;
          let mut sameLeft: i32 = ((sameIndent.first) as i32);
          // java int sameFirst = (Integer)sameIndent.second;
          let mut sameFirst: i32 = ((sameIndent.second) as i32);
          // java int left = (Integer)indent.first;
          let mut left: i32 = ((indent.first) as i32);
          // java int first = (Integer)indent.second;
          let mut first: i32 = ((indent.second) as i32);
          // java if (sameLeft != left || sameFirst != first)
          if sameLeft != left || sameFirst != first {
            // java return null;
            return None;
          }
        }
      }
      // java return sameIndent;
      return sameIndent;
    }

    // java public int getFirstLineAbsPos(IPapxTable.Entry para)
    pub fn getFirstLineAbsPos__entry(&self, para: &dyn PapxEntry) -> i32 {
      // java Assert.assertNotNull("paras should not be null.", para);
      Assert::assertNotNull__string_object(&"paras should not be null.", Some(&para));
      // java Pair<Integer, Integer> indent = this.getIndentEffectNormalized(para);
      let mut indent: Pair = self.getIndentEffectNormalized__entry(&mut para);
      // java Assert.assertNotNull("indent should not be null.", indent);
      Assert::assertNotNull__string_object(&"indent should not be null.", Some(&indent));
      // java Assert.assertNotNull("indent.first should not be null.", indent.first);
      Assert::assertNotNull__string_object(&"indent.first should not be null.", Some(&indent.first));
      // java Assert.assertNotNull("indent.second should not be null.", indent.second);
      Assert::assertNotNull__string_object(&"indent.second should not be null.", Some(&indent.second));
      // java return getFirstLineAbsPos(indent);
      return self.getFirstLineAbsPos__entry(&indent);
    }

    // java public static int getFirstLineAbsPos(Pair<Integer, Integer> normalized)
    pub fn getFirstLineAbsPos__pairinteger(normalized: &Pair) -> i32 {
      // java Assert.assertNotNull("normalized should not be null.", normalized);
      Assert::assertNotNull__string_object(&"normalized should not be null.", Some(&normalized));
      // java int leftAbsPos = (Integer)normalized.first;
      let mut leftAbsPos: i32 = ((normalized.first) as i32);
      // java int firstLineRel = (Integer)normalized.second;
      let mut firstLineRel: i32 = ((normalized.second) as i32);
      // java int firstLineAbsPos = leftAbsPos + firstLineRel;
      let mut firstLineAbsPos: i32 = leftAbsPos + firstLineRel;
      // java return firstLineAbsPos;
      return firstLineAbsPos;
    }

    // java public Indents getHelpPreventIndents(Indents desiredIndents, Indents mergedIndents)
    pub fn getHelpPreventIndents(&self, desiredIndents: &Indents, mergedIndents: &Indents) -> Option<Indents> {
      // java Integer leftChars = null;
      let mut leftChars: i32 = 0;
      // java if (null != mergedIndents.leftChars && (null == desiredIndents.leftChars || mergedIndents.leftChars != desiredIndents.leftChars))
      if mergedIndents.leftChars.is_some() && desiredIndents.leftChars.is_none() || mergedIndents.leftChars != desiredIndents.leftChars {
        // java leftChars = 0;
        leftChars = 0;
      }
      // java Integer firstLineChars = null;
      let mut firstLineChars: i32 = 0;
      // java if (null != mergedIndents.firstLineChars && (null == desiredIndents.firstLineChars || mergedIndents.firstLineChars != desiredIndents.firstLineChars))
      if mergedIndents.firstLineChars.is_some() && desiredIndents.firstLineChars.is_none() || mergedIndents.firstLineChars != desiredIndents.firstLineChars {
        // java firstLineChars = 0;
        firstLineChars = 0;
      }
      // java if (null == leftChars && null == firstLineChars)
      if leftChars.is_none() && firstLineChars.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Indents directIndents = new Indents();
        let mut directIndents: Indents = Indents::new();
        // java directIndents.leftChars = leftChars;
        directIndents.leftChars = leftChars;
        // java directIndents.firstLineChars = firstLineChars;
        directIndents.firstLineChars = firstLineChars;
        // java directIndents.left = null;
        directIndents.left = None;
        // java directIndents.firstLine = null;
        directIndents.firstLine = None;
        // java return directIndents;
        return directIndents;
      }
    }

    // java private PropertySet getLevelFormat(PropertySet papx)
    pub fn getLevelFormat(&self, papx: &PropertySet) -> Option<PropertySet> {
      // java if (!papx.contains(235))
      if !papx.contains(235) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int lfo = papx.getInt(235, -1);
        let mut lfo: i32 = papx.getInt__int_int(235, -1);
        // java int lvl = papx.getInt(234, 0);
        let mut lvl: i32 = papx.getInt__int_int(234, 0);
        // java LfoData lfoData = this.getNumberingInstance(lfo);
        let mut lfoData: LfoData = self.getNumberingInstance(lfo);
        // java if (lfoData != null)
        if lfoData.is_some() {
          // java LstData lstData = this.getNumberingStyle(lfoData.getLsid());
          let mut lstData: LstData = self.getNumberingStyle(lfoData.getLsid());
          // java if (lstData != null)
          if lstData.is_some() {
            // java LvlfData lvlfData = lstData.getLvlfDataByIlvl(lvl);
            let mut lvlfData: LvlfData = lstData.getLvlfDataByIlvl(lvl);
            // java return lvlfData == null ? null : lvlfData.getParaProp();
            return (if lvlfData.is_none() { None } else { Some(lvlfData.getParaProp()) });
          }
        }
        // java return null;
        return None;
      }
    }

    // java private LstData getNumberingStyle(int lsid)
    pub fn getNumberingStyle(&self, lsid: i32) -> LstData {
      // java return this.getNumbering().getLstDataTable().getDataByLsid(lsid);
      return self.getNumbering().getLstDataTable().getDataByLsid(lsid);
    }

    // java private LfoData getNumberingInstance(int lfo)
    pub fn getNumberingInstance(&self, lfo: i32) -> LfoData {
      // java return this.getNumbering().getLfoDataTable().getDataByIlfo(lfo);
      return self.getNumbering().getLfoDataTable().getDataByIlfo(lfo);
    }

    // java private AutoNumTable getNumbering()
    pub fn getNumbering(&self) -> AutoNumTable {
      // java return this.mDocument.getTextDocument().getAutoNumTable();
      return self.mDocument.getTextDocument().getAutoNumTable();
    }

    // java private static boolean isHangingChars(int firstLineChars)
    pub fn isHangingChars(firstLineChars: i32) -> bool {
      // java return firstLineChars < 0;
      return firstLineChars < 0;
    }

    // java private void start()
    pub fn start(&mut self) {
      // java this.mTextDoc.start();
      self.mTextDoc.start();
    }

    // java private void commit(String description)
    pub fn commit(&mut self, description: &str) {
      // java this.mTextDoc.commit(description);
      self.mTextDoc.commit__string(&description);
    }

    // java private void rollback()
    pub fn rollback(&mut self) {
      // java this.mTextDoc.rollback();
      self.mTextDoc.rollback();
    }

    // java public static boolean hasFirstLineIndent(Document document, IPapxTable.Entry para)
    pub fn hasFirstLineIndent(document: &dyn Document, para: &dyn PapxEntry) -> bool {
      // java IPropertyCalculate mCal = PropertyCalculateFactory.create(document);
      let mut mCal: Box<dyn IPropertyCalculate> = PropertyCalculateFactory::create(Some(&document));
      // java PropertySet mergedPapx = mCal.getPap(para, 3);
      let mut mergedPapx: PropertySet = mCal.getPap__entry_int(&para, 3);
      // java int firstLineIndentChar = mergedPapx.getInt(209, 0);
      let mut firstLineIndentChar: i32 = mergedPapx.getInt(209, 0);
      // java if (firstLineIndentChar > 0)
      if firstLineIndentChar > 0 {
        // java return true;
        return true;
      }
      // java else
      else {
        // java Object firstLineIndent = mergedPapx.getObject(208, (Object)null);
        let mut firstLineIndent: Box<dyn Object> = mergedPapx.getObject(208, None);
        // java if (null == firstLineIndent)
        if firstLineIndent.is_none() {
          // java return false;
          return false;
        }
        // java else
        else {
          // java int bitValue = (Integer)firstLineIndent;
          let mut bitValue: i32 = ((firstLineIndent) as i32);
          // java return bitValue != 0 && (bitValue & Integer.MIN_VALUE) == 0;
          return bitValue != 0 && bitValue & Integer::MIN_VALUE == 0;
        }
      }
    }

}

// java public static class Indents
impl Indents {
}
