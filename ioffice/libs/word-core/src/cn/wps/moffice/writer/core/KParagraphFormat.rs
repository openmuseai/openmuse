// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.property.SetPropertyMode
use util::moffice::property::SetPropertyMode::*;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::*;
// Java: import cn.wps.moffice.writer.core.list.impl.IndentsUtil
// rust: DEFERRED_NON_FUNCTION —— `core::list::impl::IndentsUtil` → `core::PropertyTool`
//   构成一条较大的未接线闭包（实测 +63 错误）。本类对该工具的引用不在 docx
//   渲染管线上 ⇒ 暂缓登记，保留 Java 注释对齐。
// use crate::cn::wps::moffice::writer::core::list::r#impl::IndentsUtil::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.value.LineSpacing
use crate::cn::wps::moffice::writer::data::value::LineSpacing::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import cn.wps.moffice.writer.core.ParagraphFormat
use crate::cn::wps::moffice::writer::core::ParagraphFormat::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import IPapxTable.Entry  (via IPapxTable::* above — Entry is nested trait)
// Java: import cn.wps.moffice.writer.core.WtReadingOrder
use crate::cn::wps::moffice::writer::core::WtReadingOrder::*;

// java public class KParagraphFormat implements PropertyId, SetPropertyMode, ParagraphFormat
#[derive(Default)]
pub struct KParagraphFormat {
    // java protected KRange mRange;
    pub(crate) mRange: Option<KRange>,
}

// java public class KParagraphFormat implements PropertyId, SetPropertyMode, ParagraphFormat
impl KParagraphFormat {
    // java protected  KParagraphFormat(KRange range)
    pub fn KParagraphFormat(range: &KRange) -> Self {
      // java this.mRange = range;
      Self { mRange: range }
    }

    // java private void setProperty(int id, Object value, String description, int mode)
    pub fn setProperty__int_object_string_int(&mut self, id: i32, value: &dyn Object, description: &str, mode: i32) {
      // java PropertySet p = PropertySet.valueOf(id, value);
      let mut p: PropertySet = PropertySet::valueOf__int_object(id, &value);
      // java this.setProperty(p, description, mode);
      self.setProperty__propertyset_string_int(&p, &description, mode);
    }

    // java private void setProperty(int[] id, Object[] value, String description, int mode)
    pub fn setProperty__intarr_objectarr_string_int(&mut self, id: &Vec<i32>, value: &dyn Object, description: &str, mode: i32) {
      // java PropertySet p = PropertySet.valueOf(id, value);
      let mut p: PropertySet = PropertySet::valueOf__int_object(id, &value);
      // java this.setProperty(p, description, mode);
      self.setProperty__propertyset_string_int(&p, &description, mode);
    }

    // java private void setProperty(int id, Object value, String description)
    pub fn setProperty__int_object_string(&mut self, id: i32, value: &dyn Object, description: &str) {
      // java PropertySet p = PropertySet.valueOf(id, value);
      let mut p: PropertySet = PropertySet::valueOf__int_object(id, &value);
      // java this.setProperty(p, description, 2);
      self.setProperty__propertyset_string_int(&p, &description, 2);
    }

    // java private void setProperty(PropertySet prop, String description, int mode)
    pub fn setProperty__propertyset_string_int(&mut self, prop: &PropertySet, description: &str, mode: i32) {
      // java this._setProperty(prop, description, mode);
      self._setProperty(&prop, &description, mode);
    }

    // java protected void _setProperty(PropertySet prop, String description, int mode)
    pub fn _setProperty(&mut self, prop: &PropertySet, description: &str, mode: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.mRange.setParagraphProperty(prop, mode);
          self.mRange.setParagraphProperty(&prop, mode);
            // java this.mRange._commit(description);
            self.mRange._commit(&description);
    }

    // java protected Object _getProperty(int id)
    pub fn _getProperty(&self, id: i32) -> Box<dyn Object> {
      // java return this.mRange.getParagraphProperty(id);
      return self.mRange.getParagraphProperty(id);
    }

    // java public void setAlignment(int nAlignment)
    pub fn setAlignment(&mut self, nAlignment: i32) {
      // java if (WtReadingOrder.wtReadingOrderRtl == this.getReadingOrder())
      if WtReadingOrder::wtReadingOrderRtl == self.getReadingOrder() {
        // java switch (nAlignment)
        match nAlignment {
          // java case 0:
          0 => {
            // java nAlignment = 2;
            nAlignment = 2;
          }
          // java case 2:
          2 => {
            // java nAlignment = 0;
            nAlignment = 0;
          }
          // java default:
          _ => {
          }
        }
      }
      // java this.setProperty(193, nAlignment, "Set Alignment");
      self.setProperty__int_object_string(193, &nAlignment, &"Set Alignment");
    }

    // java public Integer getAlignment()
    pub fn getAlignment(&self) -> i32 {
      // java Integer alignment = (Integer)this._getProperty(193);
      let mut alignment: i32 = ((self._getProperty(193)) as i32);
      // java if (null == alignment)
      if alignment.is_none() {
        // java return alignment;
        return alignment;
      }
      // java else
      else {
        // java if (WtReadingOrder.wtReadingOrderRtl == this.getReadingOrder())
        if WtReadingOrder::wtReadingOrderRtl == self.getReadingOrder() {
          // java switch (alignment)
          match alignment {
            // java case 0:
            0 => {
              // java alignment = 2;
              alignment = 2;
            }
            // java case 2:
            2 => {
              // java alignment = 0;
              alignment = 0;
            }
            // java default:
            _ => {
            }
          }
        }
        // java return alignment;
        return alignment;
      }
    }

    // java  Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.mRange.getMainDocument();
      return self.mRange.getMainDocument();
    }

    // java public void setStyle(int strStyleId)
    pub fn setStyle(&mut self, strStyleId: i32) {
      // java if (this.mRange.hasEditPermission())
      if self.mRange.hasEditPermission() {
        // java this.mRange._start();
        self.mRange._start();
            // java this.mRange.document.getTextStream().disableTrackChange();
            self.mRange.document.getTextStream().disableTrackChange();
            // java this.mRange.clearRunFormatWhenSetParagraphStyle();
            self.mRange.clearRunFormatWhenSetParagraphStyle();
            // java this.mRange.document.getTextStream().enableTrackChange();
            self.mRange.document.getTextStream().enableTrackChange();
            // java this.mRange.invalidateCache();
            self.mRange.invalidateCache();
            // java this.getMainDocument().getTextDocument().addBuiltinStyles(strStyleId);
            self.getMainDocument().getTextDocument().addBuiltinStyles(strStyleId);
            // java this.mRange.setParagraphStyle(strStyleId, 1);
            self.mRange.setParagraphStyle(strStyleId, 1);
              // java this.mRange._commit("");
              self.mRange._commit(&"");
      }
    }

    // java public int getStyle()
    pub fn getStyle(&self) -> Option<i32> {
      // java Integer styleId = (Integer)this._getProperty(191);
      let mut styleId: i32 = ((self._getProperty(191)) as i32);
      // java return styleId != null && this.isParagraphStyleType(styleId) ? styleId : 4095;
      return (if styleId.is_some() && self.isParagraphStyleType(styleId) { Some(styleId) } else { Some(4095) });
    }

    // java private boolean isParagraphStyleType(int styleId)
    pub fn isParagraphStyleType(&self, styleId: i32) -> bool {
      // java KStyle pStyle = this.getMainDocument().getStyles().getStyleById(styleId);
      let mut pStyle: KStyle = self.getMainDocument().getStyles().getStyleById(styleId);
      // java if (pStyle != null)
      if pStyle.is_some() {
        // java return pStyle.getType() == 1;
        return pStyle.getType() == 1;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public void setFirstLineIndent(int nIndent)
    pub fn setFirstLineIndent(&mut self, nIndent: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(209, 0, "Set First Line Indent", 2);
          self.setProperty__int_object_string_int(209, &0, &"Set First Line Indent", 2);
          // java this.setProperty(208, nIndent, "Set First Line Indent");
          self.setProperty__int_object_string(208, &nIndent, &"Set First Line Indent");
          // java this.setProperty(211, 0, "Set left indent chars");
          self.setProperty__int_object_string(211, &0, &"Set left indent chars");
          // java IndentsUtil mIndentsUtil = new IndentsUtil(this.mRange.getSubDocument());
          let mut mIndentsUtil: IndentsUtil = IndentsUtil::new(self.mRange.getSubDocument());
          // java IPapxTable.Entry[] paras = this.mRange.getPapxEntries();
          let mut paras: Box<dyn Entry> = self.mRange.getPapxEntries();
          // java int i = 0;
          let mut i: i32 = 0;
          let mut len: i32 = (paras.len() as i32);
          // java for(int len = paras.length; i < len; ++i)
          while i < len {
            // java IPapxTable.Entry para = paras[i];
            let mut para: Box<dyn Entry> = paras[(i) as usize];
            // java Assert.assertNotNull("para should not be null.", para);
            Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
            // java IndentsUtil.Indents mergedIndents = mIndentsUtil.getIndentEffect(para);
            let mut mergedIndents: IndentsUtil::Indents = mIndentsUtil.getIndentEffect(para);
            // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
            Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
            // java IndentsUtil.Indents desiredIndents = mIndentsUtil.getIndents(para.getProperty());
            let mut desiredIndents: IndentsUtil::Indents = mIndentsUtil.getIndents(para.getProperty());
            // java Assert.assertNotNull("desiredIndents should not be null.", desiredIndents);
            Assert::assertNotNull__string_object(&"desiredIndents should not be null.", Some(&desiredIndents));
            // java IndentsUtil.Indents preventIndents = mIndentsUtil.getHelpPreventIndents(desiredIndents, mergedIndents);
            let mut preventIndents: IndentsUtil::Indents = mIndentsUtil.getHelpPreventIndents(desiredIndents, mergedIndents);
            // java if (null != preventIndents && null != preventIndents.firstLineChars)
            if preventIndents.is_some() && preventIndents.firstLineChars.is_some() {
              // java MutablePropertySet mutableProps = new MutablePropertySet();
              let mut mutableProps: MutablePropertySet = MutablePropertySet::new();
              // java mutableProps.add(para.getProperty());
              mutableProps.add(para.getProperty());
              // java mutableProps.setInt(209, preventIndents.firstLineChars);
              mutableProps.setInt(209, preventIndents.firstLineChars);
              // java para.setProperty(mutableProps.freeze());
              para.setProperty(mutableProps.freeze());
            }
          }
            // java this.mRange._commit("Set First Line Indent");
            self.mRange._commit(&"Set First Line Indent");
    }

    // java public Integer getFirstLineIndent()
    pub fn getFirstLineIndent(&self) -> i32 {
      // java return (Integer)this._getProperty(208);
      return ((self._getProperty(208)) as i32);
    }

    // java public void indentFirstLineCharWidth(int nChar)
    pub fn indentFirstLineCharWidth(&mut self, nChar: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(208, 0, "Set First Line Indent", 3);
          self.setProperty__int_object_string_int(208, &0, &"Set First Line Indent", 3);
          // java this.setProperty(209, nChar, "Set First Line Indent");
          self.setProperty__int_object_string(209, &nChar, &"Set First Line Indent");
            // java this.mRange._commit("Set First Line Indent");
            self.mRange._commit(&"Set First Line Indent");
    }

    // java public void increaseLeftIndentChars(int nChars)
    pub fn increaseLeftIndentChars(&mut self, nChars: i32) {
      // java IPapxTable.Entry[] entrys = this.mRange.getPapxEntries();
      let mut entrys: Box<dyn Entry> = self.mRange.getPapxEntries();
      // java Assert.assertNotNull("entrys should not be null.", entrys);
      Assert::assertNotNull__string_object(&"entrys should not be null.", Some(&entrys));
      // java if (entrys.length > 1)
      if (entrys.len() as i32) > 1 {
        // java this.mRange._start();
        self.mRange._start();
            // java KRange paraRange = KRange.newInstance(this.mRange.getSubDocument());
            let mut paraRange: KRange = KRange::newInstance__document(&self.mRange.getSubDocument());
            let mut i: i32 = 0;
            // java for(int i = 0; i < entrys.length; ++i)
            while i < (entrys.len() as i32) {
              // java IPapxTable.Entry entry = entrys[i];
              let mut entry: Box<dyn Entry> = entrys[(i) as usize];
              // java paraRange.setRange(entry.position(), entry.endCp());
              paraRange.setRange(entry.position(), entry.endCp());
              // java paraRange.getParagraphFormat().paraRngIncreaseLeftIndent(nChars);
              paraRange.getParagraphFormat().paraRngIncreaseLeftIndent(nChars);
            }
              // java this.mRange._commit("Increase LeftIndentChars");
              self.mRange._commit(&"Increase LeftIndentChars");
      }
      // java else
      else {
        // java this.paraRngIncreaseLeftIndent(nChars);
        self.paraRngIncreaseLeftIndent(nChars);
      }
    }

    // java private void paraRngIncreaseLeftIndent(int nChars)
    pub fn paraRngIncreaseLeftIndent(&mut self, nChars: i32) {
      // java int leftIndentEx = (int)((double)(UnitUtil.EMU2TWIPS(this.getGridWidth()) * (float)nChars) + (double)0.5F);
      let mut leftIndentEx: i32 = ((((UnitUtil::EMU2TWIPS(self.getGridWidth()) * ((nChars) as f32)) as f64) + ((0.5) as f64)) as i32);
      // java Integer lastIncreaseLeftIndent = this.getLeftIndent();
      let mut lastIncreaseLeftIndent: i32 = self.getLeftIndent();
      // java if (lastIncreaseLeftIndent != null)
      if lastIncreaseLeftIndent.is_some() {
        // java leftIndentEx += lastIncreaseLeftIndent;
        leftIndentEx = leftIndentEx + lastIncreaseLeftIndent;
      }
      // java this.setLeftIndent(leftIndentEx);
      self.setLeftIndent(leftIndentEx);
    }

    // java public void decreaseLeftIndentChars(int nChars)
    pub fn decreaseLeftIndentChars(&mut self, nChars: i32) {
      // java IPapxTable.Entry[] entrys = this.mRange.getPapxEntries();
      let mut entrys: Box<dyn Entry> = self.mRange.getPapxEntries();
      // java Assert.assertNotNull("entrys should not be null.", entrys);
      Assert::assertNotNull__string_object(&"entrys should not be null.", Some(&entrys));
      // java if (this.canDecreaseLeftIndent())
      if self.canDecreaseLeftIndent() {
        // java if (entrys.length > 1)
        if (entrys.len() as i32) > 1 {
          // java this.mRange._start();
          self.mRange._start();
              // java KRange paraRange = KRange.newInstance(this.mRange.getSubDocument());
              let mut paraRange: KRange = KRange::newInstance__document(&self.mRange.getSubDocument());
              let mut i: i32 = 0;
              // java for(int i = 0; i < entrys.length; ++i)
              while i < (entrys.len() as i32) {
                // java IPapxTable.Entry entry = entrys[i];
                let mut entry: Box<dyn Entry> = entrys[(i) as usize];
                // java paraRange.setRange(entry.position(), entry.endCp());
                paraRange.setRange(entry.position(), entry.endCp());
                // java paraRange.getParagraphFormat().paraRngDecreaseLeftIndent(nChars);
                paraRange.getParagraphFormat().paraRngDecreaseLeftIndent(nChars);
              }
                // java this.mRange._commit("Decrease LeftIndentChars");
                self.mRange._commit(&"Decrease LeftIndentChars");
        }
        // java else
        else {
          // java this.paraRngDecreaseLeftIndent(nChars);
          self.paraRngDecreaseLeftIndent(nChars);
        }
      }
    }

    // java private boolean canDecreaseLeftIndent()
    pub fn canDecreaseLeftIndent(&self) -> bool {
      // java boolean canDecrease = true;
      let mut canDecrease: bool = true;
      // java IPapxTable.Entry[] entrys = this.mRange.getPapxEntries();
      let mut entrys: Box<dyn Entry> = self.mRange.getPapxEntries();
      // java Assert.assertNotNull("entrys should not be null.", entrys);
      Assert::assertNotNull__string_object(&"entrys should not be null.", Some(&entrys));
      // java IPapxTable.Entry firstPara = entrys[0];
      let mut firstPara: Box<dyn Entry> = entrys[0usize];
      // java Assert.assertNotNull("firstPara should not be null.", firstPara);
      Assert::assertNotNull__string_object(&"firstPara should not be null.", Some(&firstPara));
      // java Integer firstParaLastLeftIndent = null;
      let mut firstParaLastLeftIndent: i32 = 0;
      // java if (this.mRange.getListFormat().getListUtil().isSingleTypeItem())
      if self.mRange.getListFormat().getListUtil().isSingleTypeItem() {
        // java firstParaLastLeftIndent = this.getLeftIndent();
        firstParaLastLeftIndent = self.getLeftIndent();
      }
      // java else
      else {
        // java PropertySet finalProp = PropertyUtil.getFinalParagraphFormat(firstPara.getProperty());
        let mut finalProp: PropertySet = PropertyUtil::getFinalParagraphFormat(&firstPara.getProperty());
        // java firstParaLastLeftIndent = (Integer)finalProp.getValue(210);
        firstParaLastLeftIndent = ((finalProp.getValue(210)) as i32);
      }
      // java if (firstParaLastLeftIndent == null || firstParaLastLeftIndent == 0)
      if firstParaLastLeftIndent.is_none() || firstParaLastLeftIndent == 0 {
        // java canDecrease = false;
        canDecrease = false;
      }
      // java return canDecrease;
      return canDecrease;
    }

    // java private void paraRngDecreaseLeftIndent(int nChars)
    pub fn paraRngDecreaseLeftIndent(&mut self, nChars: i32) {
      // java int leftIndentEx = -((int)((double)(UnitUtil.EMU2TWIPS(this.getGridWidth()) * (float)nChars) + (double)0.5F));
      let mut leftIndentEx: i32 = -((((UnitUtil::EMU2TWIPS(self.getGridWidth()) * ((nChars) as f32)) as f64) + ((0.5) as f64)) as i32);
      // java Integer lastDecreaseLeftIndent = this.getLeftIndent();
      let mut lastDecreaseLeftIndent: i32 = self.getLeftIndent();
      // java if (lastDecreaseLeftIndent != null && lastDecreaseLeftIndent.floatValue() > 0.0F)
      if lastDecreaseLeftIndent.is_some() && lastDecreaseLeftIndent.floatValue() > 0 {
        // java leftIndentEx += lastDecreaseLeftIndent;
        leftIndentEx = leftIndentEx + lastDecreaseLeftIndent;
      }
      // java else
      else {
        // java leftIndentEx = 0;
        leftIndentEx = 0;
      }
      // java this.setLeftIndent(Math.max(0, leftIndentEx));
      self.setLeftIndent(Math::max(0, leftIndentEx));
    }

    // java private int getGridWidth()
    pub fn getGridWidth(&self) -> i32 {
      // java KStyle normalStyle = this.mRange.getTextDocument().getStyles().getStyleById(0);
      let mut normalStyle: KStyle = self.mRange.getTextDocument().getStyles().getStyleById(0);
      // java PropertySet normalProperty = normalStyle.getFinalProperty();
      let mut normalProperty: PropertySet = normalStyle.getFinalProperty();
      // java return UnitUtil.PT2EMU(normalProperty.getFloat(10, 10.0F));
      return UnitUtil::PT2EMU(normalProperty.getFloat(10, 10));
    }

    // java public void setLeftIndent(int nIndent)
    pub fn setLeftIndent(&mut self, nIndent: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(211, 0, "Set Left Indent", 2);
          self.setProperty__int_object_string_int(211, &0, &"Set Left Indent", 2);
          // java this.setProperty(210, nIndent, "Set Left Indent");
          self.setProperty__int_object_string(210, &nIndent, &"Set Left Indent");
          // java KRange firstCharRange = this.mRange.getSubDocument().getRange(this.mRange.getStart(), this.mRange.getStart());
          let mut firstCharRange: KRange = self.mRange.getSubDocument().getRange(self.mRange.getStart(), self.mRange.getStart());
              // java Integer firstLineChars = (Integer)firstCharRange.getParagraphProperty(209);
              let mut firstLineChars: i32 = ((firstCharRange.getParagraphProperty(209)) as i32);
              // java if (firstLineChars != null && firstLineChars < 0)
              if firstLineChars.is_some() && firstLineChars < 0 {
                // java this.setProperty(209, 0, "Set first line indent chars");
                self.setProperty__int_object_string(209, &0, &"Set first line indent chars");
              }
                // java firstCharRange.dispose();
                firstCharRange.dispose();
          // java IndentsUtil var18 = new IndentsUtil(this.mRange.getSubDocument());
          let mut var18: IndentsUtil = IndentsUtil::new(self.mRange.getSubDocument());
          // java IPapxTable.Entry[] var19 = this.mRange.getPapxEntries();
          let mut var19: Box<dyn Entry> = self.mRange.getPapxEntries();
          // java int i = 0;
          let mut i: i32 = 0;
          let mut len: i32 = (var19.len() as i32);
          // java for(int len = var19.length; i < len; ++i)
          while i < len {
            // java IPapxTable.Entry para = var19[i];
            let mut para: Box<dyn Entry> = var19[(i) as usize];
            // java Assert.assertNotNull("para should not be null.", para);
            Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
            // java IndentsUtil.Indents mergedIndents = var18.getIndentEffect(para);
            let mut mergedIndents: IndentsUtil::Indents = var18.getIndentEffect(para);
            // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
            Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
            // java IndentsUtil.Indents desiredIndents = var18.getIndents(para.getProperty());
            let mut desiredIndents: IndentsUtil::Indents = var18.getIndents(para.getProperty());
            // java Assert.assertNotNull("desiredIndents should not be null.", desiredIndents);
            Assert::assertNotNull__string_object(&"desiredIndents should not be null.", Some(&desiredIndents));
            // java IndentsUtil.Indents preventIndents = var18.getHelpPreventIndents(desiredIndents, mergedIndents);
            let mut preventIndents: IndentsUtil::Indents = var18.getHelpPreventIndents(desiredIndents, mergedIndents);
            // java if (null != preventIndents && null != preventIndents.leftChars)
            if preventIndents.is_some() && preventIndents.leftChars.is_some() {
              // java MutablePropertySet mutableProps = new MutablePropertySet(para.getProperty());
              let mut mutableProps: MutablePropertySet = MutablePropertySet::new(para.getProperty());
              // java mutableProps.add(para.getProperty());
              mutableProps.add(para.getProperty());
              // java mutableProps.setInt(211, preventIndents.leftChars);
              mutableProps.setInt(211, preventIndents.leftChars);
              // java para.setProperty(mutableProps.freeze());
              para.setProperty(mutableProps.freeze());
            }
          }
            // java this.mRange._commit("Set Left Indent");
            self.mRange._commit(&"Set Left Indent");
    }

    // java public Integer getLeftIndent()
    pub fn getLeftIndent(&self) -> i32 {
      // java return (Integer)this._getProperty(210);
      return ((self._getProperty(210)) as i32);
    }

    // java public void setRightIndent(int nIndent)
    pub fn setRightIndent(&mut self, nIndent: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(213, 0, "Set Right Indent", 3);
          self.setProperty__int_object_string_int(213, &0, &"Set Right Indent", 3);
          // java this.setProperty(212, nIndent, "Set Right Indent");
          self.setProperty__int_object_string(212, &nIndent, &"Set Right Indent");
            // java this.mRange._commit("Set Right Indent");
            self.mRange._commit(&"Set Right Indent");
    }

    // java public Integer getRightIndent()
    pub fn getRightIndent(&self) -> i32 {
      // java return (Integer)this._getProperty(212);
      return ((self._getProperty(212)) as i32);
    }

    // java public void setLineSpacing(float nSpacing, boolean bExact)
    pub fn setLineSpacing(&mut self, nSpacing: f32, bExact: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java int rule = bExact ? 1 : 0;
          let mut rule: i32 = (if bExact { 1 } else { 0 });
          // java LineSpacing lineSpacing = new LineSpacing(rule, bExact ? (float)UnitUtil.PT2TWIPS(nSpacing) : nSpacing);
          let mut lineSpacing: LineSpacing = LineSpacing::new(rule, (if bExact { ((UnitUtil::PT2TWIPS(nSpacing)) as f32) } else { nSpacing }));
          // java this.setProperty(220, lineSpacing, "Set Line Spacing Rule");
          self.setProperty__int_object_string(220, &lineSpacing, &"Set Line Spacing Rule");
            // java this.mRange._commit("set line spacing");
            self.mRange._commit(&"set line spacing");
    }

    // java public Float getLineSpacing()
    pub fn getLineSpacing(&self) -> Option<f32> {
      // java LineSpacing lineSpacing = (LineSpacing)this._getProperty(220);
      let mut lineSpacing: LineSpacing = (self._getProperty(220)).downcast_ref::<LineSpacing>().unwrap();
      // java return null == lineSpacing ? null : lineSpacing.getSpacing();
      return (if lineSpacing.is_none() { None } else { Some(lineSpacing.getSpacing()) });
    }

    // java public Integer getLineSpacingRule()
    pub fn getLineSpacingRule(&self) -> Option<i32> {
      // java LineSpacing lineSpacing = (LineSpacing)this._getProperty(220);
      let mut lineSpacing: LineSpacing = (self._getProperty(220)).downcast_ref::<LineSpacing>().unwrap();
      // java return null == lineSpacing ? null : lineSpacing.getRule();
      return (if lineSpacing.is_none() { None } else { Some(lineSpacing.getRule()) });
    }

    // java public void setSpaceBefore(int nSpace)
    pub fn setSpaceBefore(&mut self, nSpace: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(219, false, "Set Space Before Auto", 3);
          self.setProperty__int_object_string_int(219, &false, &"Set Space Before Auto", 3);
          // java this.setProperty(218, 0, "Set Space Before lines", 3);
          self.setProperty__int_object_string_int(218, &0, &"Set Space Before lines", 3);
          // java this.setProperty(217, nSpace, "Set Space Before");
          self.setProperty__int_object_string(217, &nSpace, &"Set Space Before");
            // java this.mRange._commit("Set Space Before");
            self.mRange._commit(&"Set Space Before");
    }

    // java public Integer getSpaceBefore()
    pub fn getSpaceBefore(&self) -> i32 {
      // java return (Integer)this._getProperty(217);
      return ((self._getProperty(217)) as i32);
    }

    // java public void setSpaceBeforeAuto(boolean bAuto)
    pub fn setSpaceBeforeAuto(&mut self, bAuto: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(217, 0, "Set Space Before", 3);
          self.setProperty__int_object_string_int(217, &0, &"Set Space Before", 3);
          // java this.setProperty(218, 0, "Set Space Before lines", 3);
          self.setProperty__int_object_string_int(218, &0, &"Set Space Before lines", 3);
          // java this.setProperty(219, bAuto, "Set Space Before Auto");
          self.setProperty__int_object_string(219, &bAuto, &"Set Space Before Auto");
            // java this.mRange._commit("Set Space Before Auto");
            self.mRange._commit(&"Set Space Before Auto");
    }

    // java public Boolean getSpaceBeforeAuto()
    pub fn getSpaceBeforeAuto(&self) -> bool {
      // java return (Boolean)this._getProperty(219);
      return todo!("cast → bool");
    }

    // java public void setLineUnitBefore(int nLine)
    pub fn setLineUnitBefore(&mut self, nLine: i32) {
      // java this.setProperty(218, nLine, "Set Space Before");
      self.setProperty__int_object_string(218, &nLine, &"Set Space Before");
    }

    // java public Integer getLineUnitBefore()
    pub fn getLineUnitBefore(&self) -> i32 {
      // java return (Integer)this._getProperty(218);
      return ((self._getProperty(218)) as i32);
    }

    // java public void setSpaceAfter(int nSpace)
    pub fn setSpaceAfter(&mut self, nSpace: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(216, false, "Set Space After Auto", 3);
          self.setProperty__int_object_string_int(216, &false, &"Set Space After Auto", 3);
          // java this.setProperty(215, 0, "Set Space After lines", 3);
          self.setProperty__int_object_string_int(215, &0, &"Set Space After lines", 3);
          // java this.setProperty(214, nSpace, "Set Space After");
          self.setProperty__int_object_string(214, &nSpace, &"Set Space After");
            // java this.mRange._commit("Set Space After");
            self.mRange._commit(&"Set Space After");
    }

    // java public Integer getSpaceAfter()
    pub fn getSpaceAfter(&self) -> i32 {
      // java return (Integer)this._getProperty(214);
      return ((self._getProperty(214)) as i32);
    }

    // java public void setSpaceAfterAuto(boolean bAuto)
    pub fn setSpaceAfterAuto(&mut self, bAuto: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(214, 0, "Set Space After", 3);
          self.setProperty__int_object_string_int(214, &0, &"Set Space After", 3);
          // java this.setProperty(215, 0, "Set Space After lines", 3);
          self.setProperty__int_object_string_int(215, &0, &"Set Space After lines", 3);
          // java this.setProperty(216, bAuto, "Set Space After Auto");
          self.setProperty__int_object_string(216, &bAuto, &"Set Space After Auto");
            // java this.mRange._commit("Set Space After Auto");
            self.mRange._commit(&"Set Space After Auto");
    }

    // java public Boolean getSpaceAfterAuto()
    pub fn getSpaceAfterAuto(&self) -> bool {
      // java return (Boolean)this._getProperty(216);
      return todo!("cast → bool");
    }

    // java public void setLineUnitAfter(int nLine)
    pub fn setLineUnitAfter(&mut self, nLine: i32) {
      // java this.setProperty(215, nLine, "Set Space After");
      self.setProperty__int_object_string(215, &nLine, &"Set Space After");
    }

    // java public Integer getLineUnitAfter()
    pub fn getLineUnitAfter(&self) -> i32 {
      // java return (Integer)this._getProperty(215);
      return ((self._getProperty(215)) as i32);
    }

    // java public void setFirstLineIndent_LeftIndent(int firstLineIndent, int leftIndent)
    pub fn setFirstLineIndent_LeftIndent(&mut self, firstLineIndent: i32, leftIndent: i32) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(new int[]{209, 211}, new Object[]{0, 0}, "Set FirstLineIndent and LeftIndent", 3);
          self.setProperty__int_object_string_int(vec![209, 211], &vec![0, 0], &"Set FirstLineIndent and LeftIndent", 3);
          // java this.setProperty(new int[]{208, 210}, new Object[]{firstLineIndent, leftIndent}, "Set FirstLineIndent and LeftIndent", 2);
          self.setProperty__int_object_string_int(vec![208, 210], &vec![firstLineIndent, leftIndent], &"Set FirstLineIndent and LeftIndent", 2);
          // java IndentsUtil mIndentsUtil = new IndentsUtil(this.mRange.getSubDocument());
          let mut mIndentsUtil: IndentsUtil = IndentsUtil::new(self.mRange.getSubDocument());
          // java for(IPapxTable.Entry para : this.mRange.getPapxEntries())
          for para in self.mRange.getPapxEntries().iter() {
            // java Assert.assertNotNull("para should not be null.", para);
            Assert::assertNotNull__string_object(&"para should not be null.", Some(&para));
            // java IndentsUtil.Indents mergedIndents = mIndentsUtil.getIndentEffect(para);
            let mut mergedIndents: IndentsUtil::Indents = mIndentsUtil.getIndentEffect(para);
            // java Assert.assertNotNull("mergedIndents should not be null.", mergedIndents);
            Assert::assertNotNull__string_object(&"mergedIndents should not be null.", Some(&mergedIndents));
            // java IndentsUtil.Indents desiredIndents = mIndentsUtil.getIndents(para.getProperty());
            let mut desiredIndents: IndentsUtil::Indents = mIndentsUtil.getIndents(para.getProperty());
            // java Assert.assertNotNull("desiredIndents should not be null.", desiredIndents);
            Assert::assertNotNull__string_object(&"desiredIndents should not be null.", Some(&desiredIndents));
            // java IndentsUtil.Indents preventIndents = mIndentsUtil.getHelpPreventIndents(desiredIndents, mergedIndents);
            let mut preventIndents: IndentsUtil::Indents = mIndentsUtil.getHelpPreventIndents(desiredIndents, mergedIndents);
            // java if (null != preventIndents && (null != preventIndents.leftChars || null != preventIndents.firstLineChars))
            if preventIndents.is_some() && preventIndents.leftChars.is_some() || preventIndents.firstLineChars.is_some() {
              // java MutablePropertySet mutableProps = new MutablePropertySet(para.getProperty());
              let mut mutableProps: MutablePropertySet = MutablePropertySet::new(para.getProperty());
              // java if (null != preventIndents.leftChars)
              if preventIndents.leftChars.is_some() {
                // java mutableProps.setInt(211, preventIndents.leftChars);
                mutableProps.setInt(211, preventIndents.leftChars);
              }
              // java if (null != preventIndents.firstLineChars)
              if preventIndents.firstLineChars.is_some() {
                // java mutableProps.setInt(209, preventIndents.firstLineChars);
                mutableProps.setInt(209, preventIndents.firstLineChars);
              }
              // java para.setProperty(mutableProps.freeze());
              para.setProperty(mutableProps.freeze());
            }
          }
            // java this.mRange._commit("Set FirstLineIndent and LeftIndent");
            self.mRange._commit(&"Set FirstLineIndent and LeftIndent");
    }

    // java public void setReadingOrder(WtReadingOrder order)
    pub fn setReadingOrder(&mut self, order: &WtReadingOrder) {
      // java this.mRange.setParagraphReadingOrder(order);
      self.mRange.setParagraphReadingOrder(&order);
    }

    // java public WtReadingOrder getReadingOrder()
    pub fn getReadingOrder(&self) -> WtReadingOrder {
      // java return this.mRange.getParagraphReadingOrder();
      return self.mRange.getParagraphReadingOrder();
    }

    // java public RangeReadLock readLock()
    pub fn readLock(&self) -> RangeReadLock {
      // java return this.getMainDocument().readLock();
      return self.getMainDocument().readLock();
    }

}
