// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.font.FontHost
use thirdparty_font::cn::wps::font::FontHost::*;
// Java: import cn.wps.font.IFontFamily
use thirdparty_font::cn::wps::font::IFontFamily::*;
// Java: import cn.wps.font.IFontTable
use thirdparty_font::cn::wps::font::IFontTable::*;
// Java: import cn.wps.font.ITypeface
use thirdparty_font::cn::wps::font::ITypeface::*;
// Java: import cn.wps.kfc.api.list.ILongList
use util::kfc::api::list::ILongList::*;
// Java: import cn.wps.kfc.impl.arrayList.WLongArrayList
use util::kfc::r#impl::arrayList::WLongArrayList::*;
// Java: import cn.wps.moffice.define.FontName
use util::moffice::define::FontName::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.property.SetPropertyMode
use util::moffice::property::SetPropertyMode::*;
// Java: import cn.wps.moffice.writer.data.DefaultRunProperties
use crate::cn::wps::moffice::writer::data::DefaultRunProperties::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::*;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
// Java: import cn.wps.moffice.writer.data.ITextStream
use crate::cn::wps::moffice::writer::data::ITextStream::*;
// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;
// Java: import cn.wps.moffice.writer.data.PropertyOperations
use crate::cn::wps::moffice::writer::data::PropertyOperations::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import cn.wps.moffice.writer.core.Font
use crate::cn::wps::moffice::writer::core::Font::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument as WriterTextDocument;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.IInterruptState
use crate::cn::wps::moffice::writer::core::IInterruptState::*;
// Java: import cn.wps.moffice.writer.core.PropertyTool
use crate::cn::wps::moffice::writer::core::PropertyTool::*;
use crate::cn::wps::moffice::writer::core::PropertyTool::PropertyTool;

// java public static final int FONT_ERR_BASE = 200;
pub(crate) const FONT_ERR_BASE: i32 = 200;
// java public static final int FONT_SUCCESS = 100;
pub(crate) const FONT_SUCCESS: i32 = 100;
// java public static final int FONT_RANGE_SUCCESS = 101;
pub(crate) const FONT_RANGE_SUCCESS: i32 = 101;
// java public static final int FONT_IS_CARET = 102;
pub(crate) const FONT_IS_CARET: i32 = 102;
// java public static final int FONT_FILE_LOST = 201;
pub(crate) const FONT_FILE_LOST: i32 = 201;
// java public static final int FONT_FILE_ERR = 202;
pub(crate) const FONT_FILE_ERR: i32 = 202;

// java public class KFont implements PropertyId, SetPropertyMode, Font
#[derive(Default)]
pub struct KFont {
    // java protected KRange mRange;
    pub(crate) mRange: Option<KRange>,
}

// java  class GrowShrink implements ITextStream.IPropertyModify
#[derive(Default)]
pub struct GrowShrink {
    // java boolean mGrow;
    pub(crate) mGrow: bool,
}

// java public class KFont implements PropertyId, SetPropertyMode, Font
impl KFont {
    // java protected  KFont(KRange range)
    pub fn KFont(range: &KRange) -> Self {
      // java this.mRange = range;
      Self { mRange: range }
    }

    // java public Document getSubDocument()
    pub fn getSubDocument(&self) -> Box<dyn Document> {
      // java return this.mRange.getSubDocument();
      return self.mRange.getSubDocument();
    }

    // java public Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.mRange.getMainDocument();
      return self.mRange.getMainDocument();
    }

    // java protected RangeReadLock readLock()
    pub fn readLock(&self) -> RangeReadLock {
      // java return this.getMainDocument().readLock();
      return self.getMainDocument().readLock();
    }

    // java protected RangeWriteLock writeLock()
    pub fn writeLock(&self) -> RangeWriteLock {
      // java return this.getMainDocument().writeLock();
      return self.getMainDocument().writeLock();
    }

    // java protected void _setProperty(int id, Object value, String description, int mode)
    pub fn _setProperty(&mut self, id: i32, value: &dyn Object, description: &str, mode: i32) {
      // java RangeWriteLock lock = this.writeLock();
      let mut lock: RangeWriteLock = self.writeLock();
      // java this.mRange._start();
      self.mRange._start();
          // java PropertySet p = PropertySet.valueOf(id, value);
          let mut p: PropertySet = PropertySet::valueOf__int_object(id, &value);
          // java this.mRange.setRunProperty(p, mode);
          self.mRange.setRunProperty(&p, mode);
            // java this.mRange._commit(description);
            self.mRange._commit(&description);
            // java lock.unlock();
            lock.unlock();
    }

    // java private void setProperty(int id, Object value, String description)
    pub fn setProperty__int_object_string(&mut self, id: i32, value: &dyn Object, description: &str) {
      // java this.setProperty(id, value, description, 2);
      self.setProperty__int_object_string_int(id, &value, &description, 2);
    }

    // java private void setProperty(int id, Object value, String description, int mode)
    pub fn setProperty__int_object_string_int(&mut self, id: i32, value: &dyn Object, description: &str, mode: i32) {
      // java this._setProperty(id, value, description, mode);
      self._setProperty(id, &value, &description, mode);
    }

    // java public ITypeface createTypeface(String fontName)
    pub fn createTypeface(&mut self, fontName: Option<&str>) -> Option<Box<dyn ITypeface>> {
      // java if (null == fontName)
      if fontName.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int fontStyle = 0;
        let mut fontStyle: i32 = 0;
        // java IFontTable fontHostFontTable = FontHost.getFontTable();
        let mut fontHostFontTable: Box<dyn IFontTable> = FontHost::getFontTable();
        // java IFontFamily fontFamily = fontHostFontTable.get(fontName, false);
        let mut fontFamily: Box<dyn IFontFamily> = fontHostFontTable.get(fontName, false);
        // java return null == fontFamily ? null : fontFamily.getTypeface(fontStyle);
        return (if fontFamily.is_none() { None } else { Some(fontFamily.getTypeface(fontStyle)) });
      }
    }

    // java protected Object _getProperty(int id)
    pub fn _getProperty(&mut self, id: i32) -> Box<dyn Object> {
      // java RangeReadLock lock = this.readLock();
      let mut lock: RangeReadLock = self.readLock();
      // java Object var3;
      let mut var3: Box<dyn Object> = Box::new(()) as Box<dyn Object>;
          // java var3 = this.mRange.getRunProperty(id);
          var3 = self.mRange.getRunProperty(id);
            // java lock.unlock();
            lock.unlock();
      // java return var3;
      return var3;
    }

    // java public KFont getStartFont()
    pub fn getStartFont(&self) -> KFont {
      // java int start = this.mRange.getStart();
      let mut start: i32 = self.mRange.getStart();
      // java KRange newRange = new KRange(this.mRange.getSubDocument(), start, start + 1);
      let mut newRange: KRange = KRange::new(self.mRange.getSubDocument(), start, start + 1);
      // java return new KFont(newRange);
      return KFont::new(newRange);
    }

    // java  TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> WriterTextDocument {
      // java return this.mRange.getTextDocument();
      return self.mRange.getTextDocument();
    }

    // java public void setStyle(int strStyleId)
    pub fn setStyle(&mut self, strStyleId: i32) {
      // java this.mRange._start();
      self.mRange._start();
      // java this.mRange.setStyle(strStyleId);
      self.mRange.setStyle(strStyleId);
      // java this.mRange._commit("");
      self.mRange._commit(&"");
    }

    // java public int getStyle()
    pub fn getStyle(&self) -> Option<i32> {
      // java Integer styleId = null;
      let mut styleId: i32 = 0;
      // java if (null != this.mRange.mCache && this.mRange.getStart() == this.mRange.getEnd())
      if self.mRange.mCache.is_some() && self.mRange.getStart() == self.mRange.getEnd() {
        // java styleId = (Integer)this.mRange.mCache.getObject(2, (Object)null);
        styleId = ((self.mRange.mCache.getObject(2, None)) as i32);
        // java if (null != styleId && this.isRunStyleType(styleId))
        if styleId.is_some() && self.isRunStyleType(styleId) {
          // java return styleId;
          return styleId;
        }
      }
      // java styleId = (Integer)this._getProperty(2);
      styleId = ((self._getProperty(2)) as i32);
      // java return null != styleId && this.isRunStyleType(styleId) ? styleId : 4095;
      return (if styleId.is_some() && self.isRunStyleType(styleId) { Some(styleId) } else { Some(4095) });
    }

    // java private boolean isRunStyleType(int styleId)
    pub fn isRunStyleType(&self, styleId: i32) -> bool {
      // java KStyle pStyle = this.getMainDocument().getStyles().getStyleById(styleId);
      let mut pStyle: KStyle = self.getMainDocument().getStyles().getStyleById(styleId);
      // java if (pStyle != null)
      if pStyle.is_some() {
        // java return pStyle.getType() == 2;
        return pStyle.getType() == 2;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public void setBold(boolean bBold)
    pub fn setBold(&mut self, bBold: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setRunBold(bBold);
          self.setRunBold(bBold);
          // java this.setRunBoldBi(bBold);
          self.setRunBoldBi(bBold);
            // java this.mRange._commit("set Bold and BoldBi");
            self.mRange._commit(&"set Bold and BoldBi");
    }

    // java public void setRunBoldBi(boolean bBold)
    pub fn setRunBoldBi(&mut self, bBold: bool) {
      // java this.setProperty(38, bBold, "set Bold");
      self.setProperty__int_object_string(38, &bBold, &"set Bold");
    }

    // java public void setRunBold(boolean bBold)
    pub fn setRunBold(&mut self, bBold: bool) {
      // java this.setProperty(5, bBold, "set BoldBi");
      self.setProperty__int_object_string(5, &bBold, &"set BoldBi");
    }

    // java public Boolean getBold()
    pub fn getBold(&self) -> bool {
      // java Boolean bold = this.getRunBold();
      let mut bold: bool = self.getRunBold();
      // java if (null == bold)
      if bold.is_none() {
        // java bold = this.getRunBoldBi();
        bold = self.getRunBoldBi();
      }
      // java return bold;
      return bold;
    }

    // java public Boolean getRunBoldBi()
    pub fn getRunBoldBi(&self) -> bool {
      // java return (Boolean)this._getProperty(38);
      return todo!("cast → bool");
    }

    // java public Boolean getRunBold()
    pub fn getRunBold(&self) -> bool {
      // java return (Boolean)this._getProperty(5);
      return todo!("cast → bool");
    }

    // java public void setItalic(boolean bItalic)
    pub fn setItalic(&mut self, bItalic: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setRunItalic(bItalic);
          self.setRunItalic(bItalic);
          // java this.setRunItalicBi(bItalic);
          self.setRunItalicBi(bItalic);
            // java this.mRange._commit("set Italic and ItalicBi");
            self.mRange._commit(&"set Italic and ItalicBi");
    }

    // java public void setRunItalicBi(boolean bItalic)
    pub fn setRunItalicBi(&mut self, bItalic: bool) {
      // java this.setProperty(39, bItalic, "set ItalicBi");
      self.setProperty__int_object_string(39, &bItalic, &"set ItalicBi");
    }

    // java public void setRunItalic(boolean bItalic)
    pub fn setRunItalic(&mut self, bItalic: bool) {
      // java this.setProperty(6, bItalic, "set Italic");
      self.setProperty__int_object_string(6, &bItalic, &"set Italic");
    }

    // java public Boolean getItalic()
    pub fn getItalic(&self) -> bool {
      // java Boolean italic = this.getRunItalic();
      let mut italic: bool = self.getRunItalic();
      // java if (null == italic)
      if italic.is_none() {
        // java italic = this.getRunItalicBi();
        italic = self.getRunItalicBi();
      }
      // java return italic;
      return italic;
    }

    // java public Boolean getRunItalicBi()
    pub fn getRunItalicBi(&self) -> bool {
      // java return (Boolean)this._getProperty(39);
      return todo!("cast → bool");
    }

    // java public Boolean getRunItalic()
    pub fn getRunItalic(&self) -> bool {
      // java return (Boolean)this._getProperty(6);
      return todo!("cast → bool");
    }

    // java public void setUnderline(int nUnderline)
    pub fn setUnderline(&mut self, nUnderline: i32) {
      // java this.setProperty(8, nUnderline, "Underline");
      self.setProperty__int_object_string(8, &nUnderline, &"Underline");
    }

    // java public Integer getUnderline()
    pub fn getUnderline(&self) -> i32 {
      // java return (Integer)this._getProperty(8);
      return ((self._getProperty(8)) as i32);
    }

    // java public void setUnderlineColor(int nColor)
    pub fn setUnderlineColor(&mut self, nColor: i32) {
      // java this.setProperty(9, nColor, "Underline Color");
      self.setProperty__int_object_string(9, &nColor, &"Underline Color");
    }

    // java public Integer getUnderlineColor()
    pub fn getUnderlineColor(&self) -> i32 {
      // java return (Integer)this._getProperty(9);
      return ((self._getProperty(9)) as i32);
    }

    // java public void setTextColor(int nColor)
    pub fn setTextColor(&mut self, nColor: i32) {
      // java this.setProperty(11, nColor, "Text Color");
      self.setProperty__int_object_string(11, &nColor, &"Text Color");
    }

    // java public Integer getTextColor()
    pub fn getTextColor(&self) -> i32 {
      // java return (Integer)this._getProperty(11);
      return ((self._getProperty(11)) as i32);
    }

    // java public void setHighlightColorIndex(int index)
    pub fn setHighlightColorIndex(&mut self, index: i32) {
      // java if (index < 0 || index >= 17)
      if index < 0 || index >= 17 {
        // java index = 0;
        index = 0;
      }
      // java this.setProperty(15, index, "Highlight Color");
      self.setProperty__int_object_string(15, &index, &"Highlight Color");
    }

    // java public Integer getHighlightColorIndex()
    pub fn getHighlightColorIndex(&self) -> i32 {
      // java return (Integer)this._getProperty(15);
      return ((self._getProperty(15)) as i32);
    }

    // java public void setStrikeThrough()
    pub fn setStrikeThrough(&mut self) {
      // java this.setProperty(7, 1, "Strike Through");
      self.setProperty__int_object_string(7, &1, &"Strike Through");
    }

    // java public void setNoneStrikeThrough()
    pub fn setNoneStrikeThrough(&mut self) {
      // java this.setProperty(7, 0, "None Strike Through");
      self.setProperty__int_object_string(7, &0, &"None Strike Through");
    }

    // java public Boolean getStrikeThrough()
    pub fn getStrikeThrough(&self) -> Option<bool> {
      // java Integer v = (Integer)this._getProperty(7);
      let mut v: i32 = ((self._getProperty(7)) as i32);
      // java if (v == null)
      if v.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java return v == 1 ? true : false;
        return (if v == 1 { Some(true) } else { Some(false) });
      }
    }

    // java public void setDoubleStrikeThrough()
    pub fn setDoubleStrikeThrough(&mut self) {
      // java this.setProperty(7, 2, "Double Strike Through");
      self.setProperty__int_object_string(7, &2, &"Double Strike Through");
    }

    // java public Boolean getDoubleStrikeThrough()
    pub fn getDoubleStrikeThrough(&self) -> Option<bool> {
      // java Integer v = (Integer)this._getProperty(7);
      let mut v: i32 = ((self._getProperty(7)) as i32);
      // java if (v == null)
      if v.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java return v == 2 ? true : false;
        return (if v == 2 { Some(true) } else { Some(false) });
      }
    }

    // java public void setEmphasisMark(int nEmphasisMark)
    pub fn setEmphasisMark(&mut self, nEmphasisMark: i32) {
      // java this.setProperty(12, nEmphasisMark, "Emphasis Mark");
      self.setProperty__int_object_string(12, &nEmphasisMark, &"Emphasis Mark");
    }

    // java public Integer getEmphasisMark()
    pub fn getEmphasisMark(&self) -> i32 {
      // java return (Integer)this._getProperty(12);
      return ((self._getProperty(12)) as i32);
    }

    // java public void modifyF0(String name)
    pub fn modifyF0(&mut self, name: &str) {
      // java boolean specialFontType = FontName.isSpecialFontName(name);
      let mut specialFontType: bool = FontName::isSpecialFontName(&name);
      // java ILongList ranges = new WLongArrayList();
      let mut ranges: Box<dyn ILongList> = WLongArrayList::new();
      // java this.collectRanges(specialFontType, ranges);
      self.collectRanges(specialFontType, &mut ranges);
      // java if (specialFontType)
      if specialFontType {
        // java this.getSubDocument().getTextRope().addF0(ranges.toArray());
        self.getSubDocument().getTextRope().addF0(ranges.toArray());
      }
      // java else
      else {
        // java this.getSubDocument().getTextRope().removeF0(ranges.toArray());
        self.getSubDocument().getTextRope().removeF0(ranges.toArray());
      }
    }

    // java public int setName(String name)
    pub fn setName__string(&mut self, name: &str) -> i32 {
      // java return this.setName(name, DefaultInterrupState.INSTANCE);
      return self.setName__string_iinterruptstate(&name, Some(&mut DefaultInterrupState::INSTANCE));
    }

    // java public int setName(String name, IInterruptState interrupt)
    pub fn setName__string_iinterruptstate(&mut self, name: &str, interrupt: Option<&mut dyn IInterruptState>) -> i32 {
      // java if (interrupt == null)
      if interrupt.is_none() {
        // java return 200;
        return 200;
      }
      // java else
      else {
        // java Document document = this.mRange.getStory();
        let mut document: Box<dyn Document> = self.mRange.getStory();
        // java ITypeface typeface = this.createTypeface(name);
        let mut typeface: Box<dyn ITypeface> = self.createTypeface(Some(&name));
        // java ITextRope textRope = document.getTextRope();
        let mut textRope: Box<dyn ITextRope> = document.getTextRope();
        // java KRange range = KRange.newInstance(document, this.mRange.getStart(), this.mRange.getEnd());
        let mut range: KRange = KRange::newInstance__document_int_int(&document, self.mRange.getStart(), self.mRange.getEnd());
        // java int invalid = -1;
        let mut invalid: i32 = -1;
        // java int start = -1;
        let mut start: i32 = -1;
        // java int end = -1;
        let mut end: i32 = -1;
        // java int retCode = -1;
        let mut retCode: i32 = -1;
        // java if (typeface == null)
        if typeface.is_none() {
          // java retCode = 201;
          retCode = 201;
          // java this.setNameWithoutCheck(name, (ITypeface)null);
          self.setNameWithoutCheck(&name, None);
        }
        // java else if (this.mRange.isEmpty())
        else if self.mRange.isEmpty() {
          // java retCode = 102;
          retCode = 102;
          // java this.setNameWithoutCheck(name, typeface);
          self.setNameWithoutCheck(&name, Some(&typeface));
        }
        // java else
        else {
          let mut i: i32 = self.mRange.getStart();
          // java for(int i = this.mRange.getStart(); i < this.mRange.getEnd(); ++i)
          while i < self.mRange.getEnd() {
            // java if (i % 100 == 0 && interrupt.checkInterruptedToExit())
            if i % 100 == 0 && interrupt.checkInterruptedToExit() {
              // java retCode = retCode == -1 ? 200 : retCode;
              retCode = (if retCode == -1 { 200 } else { retCode });
              // java break;
              break;
            }
            // java if (typeface.hasUnicodeBlock(textRope.charAt(i)))
            if typeface.hasUnicodeBlock(textRope.charAt(i)) {
              // java start = start == -1 ? i : start;
              start = (if start == -1 { i } else { start });
            }
            // java else
            else {
              // java end = i;
              end = i;
            }
            // java if (-1 != start && end > start)
            if -1 != start && end > start {
              // java retCode = 101;
              retCode = 101;
              // java range.setRange(start, end);
              range.setRange(start, end);
              // java range.getFont().setNameWithoutCheck(name, typeface);
              range.getFont().setNameWithoutCheck(name, typeface);
              // java end = -1;
              end = -1;
              // java start = -1;
              start = -1;
            }
          }
          // java if (!interrupt.isInterrupted())
          if !interrupt.isInterrupted() {
            // java if (-1 != start)
            if -1 != start {
              // java retCode = retCode == -1 ? 100 : retCode;
              retCode = (if retCode == -1 { 100 } else { retCode });
              // java range.setRange(start, this.mRange.getEnd());
              range.setRange(start, self.mRange.getEnd());
              // java range.getFont().setNameWithoutCheck(name, typeface);
              range.getFont().setNameWithoutCheck(name, typeface);
            }
            // java else if (end != -1 && retCode == -1)
            else if end != -1 && retCode == -1 {
              // java retCode = 202;
              retCode = 202;
            }
          }
        }
        // java return retCode;
        return retCode;
      }
    }

    // java private void setNameWithoutCheck(String name, ITypeface typeface)
    pub fn setNameWithoutCheck(&mut self, name: &str, typeface: Option<&dyn ITypeface>) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.modifyF0(name);
          self.modifyF0(&name);
          // java if (null != typeface && !this.mRange.isEmpty())
          if let Some(typeface) = typeface {
            if !self.mRange.isEmpty() {
              // java if (typeface.isComplexFont())
              if typeface.isComplexFont() {
                // java this.setNameComplexScript(name);
                self.setNameComplexScript(&name);
              }
              // java if (typeface.isCJKFont())
              if typeface.isCJKFont() {
                // java this.setNameFarEast(name);
                self.setNameFarEast(&name);
              }
              // java if (typeface.isLatinFont())
              if typeface.isLatinFont() {
                // java this.setNameAscii(name);
                self.setNameAscii(&name);
              }
            }
          }
          // java else
          else {
            // java this.setNameAscii(name);
            self.setNameAscii(&name);
            // java this.setNameFarEast(name);
            self.setNameFarEast(&name);
            // java this.setNameComplexScript(name);
            self.setNameComplexScript(&name);
          }
          // java if (null != this.getNameOther() || FontName.isSpecialFontName(name))
          if self.getNameOther().is_some() || FontName::isSpecialFontName(&name) {
            // java this.setNameOther(name);
            self.setNameOther(&name);
          }
          // java this.mRange.getSubDocument().getTextDocument().getFontTable().add(name);
          self.mRange.getSubDocument().getTextDocument().getFontTable().add(name);
            // java this.mRange._commit("Font Name");
            self.mRange._commit(&"Font Name");
    }

    // java private void collectRanges(boolean specialFontType, ILongList ranges)
    pub fn collectRanges(&self, specialFontType: bool, ranges: &mut dyn ILongList) {
      // java String name = this.getName();
      let mut name: String = self.getName();
      // java if (!specialFontType || !FontName.isSpecialFontName(name))
      if !specialFontType || !FontName::isSpecialFontName(&name) {
        // java if (name != null)
        if name.is_some() {
          // java ranges.add(this.mRange.getRange());
          ranges.add__long(self.mRange.getRange());
        }
        // java else
        else {
          // java IChpxTable runs = this.getSubDocument().getChpxTable();
          let mut runs: Box<dyn IChpxTable> = self.getSubDocument().getChpxTable();
          // java IChpxTable.Entry s = runs.seek(this.mRange.getStart());
          let mut s: Box<dyn ChpxEntry> = runs.seek(self.mRange.getStart());
          // java KRange rg = KRange.newInstance(this.getSubDocument(), this.mRange.getStart(), this.mRange.getEnd());
          let mut rg: KRange = KRange::newInstance__document_int_int(&self.getSubDocument(), self.mRange.getStart(), self.mRange.getEnd());
          // java KFont font = null;
          let mut font: KFont = KFont::default();
          // java int cp = this.mRange.getStart();
          let mut cp: i32 = self.mRange.getStart();
          let mut end: i32 = self.mRange.getEnd();
          // java for(int end = this.mRange.getEnd(); cp < end cp = s.position())
          while cp < end {
            // java rg.setRange(cp, Math.min(s.endCp(), end));
            rg.setRange(cp, Math::min(s.endCp(), end));
            // java font = new KFont(rg);
            font = KFont::new(rg);
            // java name = font.getName();
            name = font.getName();
            // java if (specialFontType)
            if specialFontType {
              // java if (!FontName.isSpecialFontName(name))
              if !FontName::isSpecialFontName(&name) {
                // java ranges.add(rg.getRange());
                ranges.add__long(rg.getRange());
              }
            }
            // java else if (FontName.isSpecialFontName(name))
            else if FontName::isSpecialFontName(&name) {
              // java ranges.add(rg.getRange());
              ranges.add__long(rg.getRange());
            }
            // java s = s.getNext();
            s = s.getNext();
          }
          // java rg.dispose();
          rg.dispose();
        }
      }
    }

    // java public String getName()
    pub fn getName(&self) -> String {
      // java return (String)this._getMultiProperty(4);
      return self._getMultiProperty(4).to_string();
    }

    // java protected Object _getMultiProperty(int PropertyId)
    pub fn _getMultiProperty(&mut self, PropertyId: i32) -> Box<dyn Object> {
      // java RangeReadLock lock = this.readLock();
      let mut lock: RangeReadLock = self.readLock();
      // java Object var3;
      let mut var3: Box<dyn Object> = Box::new(()) as Box<dyn Object>;
          // java var3 = this.mRange.getMultiProperty(PropertyId);
          var3 = self.mRange.getMultiProperty(PropertyId);
            // java lock.unlock();
            lock.unlock();
      // java return var3;
      return var3;
    }

    // java public void setNameAscii(String name)
    pub fn setNameAscii(&mut self, name: &str) {
      // java this.setProperty(3, name, "Font Name");
      self.setProperty__int_object_string(3, &name, &"Font Name");
    }

    // java public String getNameAscii()
    pub fn getNameAscii(&self) -> String {
      // java return (String)this._getProperty(3);
      return self._getProperty(3).to_string();
    }

    // java public void setNameComplexScript(String name)
    pub fn setNameComplexScript(&mut self, name: &str) {
      // java this.setProperty(36, name, "Font Name");
      self.setProperty__int_object_string(36, &name, &"Font Name");
    }

    // java public String getNameComplexScript()
    pub fn getNameComplexScript(&self) -> String {
      // java return (String)this._getProperty(36);
      return self._getProperty(36).to_string();
    }

    // java public void setNameFarEast(String name)
    pub fn setNameFarEast(&mut self, name: &str) {
      // java this.setProperty(4, name, "Font Name");
      self.setProperty__int_object_string(4, &name, &"Font Name");
    }

    // java public String getNameFarEast()
    pub fn getNameFarEast(&self) -> String {
      // java return (String)this._getProperty(4);
      return self._getProperty(4).to_string();
    }

    // java public void setNameOther(String name)
    pub fn setNameOther(&mut self, name: &str) {
      // java this.setProperty(35, name, "Font Name");
      self.setProperty__int_object_string(35, &name, &"Font Name");
    }

    // java public String getNameOther()
    pub fn getNameOther(&self) -> String {
      // java return (String)this._getProperty(35);
      return self._getProperty(35).to_string();
    }

    // java public void setSize(float nSize)
    pub fn setSize(&mut self, nSize: f32) {
      // java this.mRange._start();
      self.mRange._start();
          // java MutablePropertySet prop = new MutablePropertySet();
          let mut prop: MutablePropertySet = MutablePropertySet::new();
          // java prop.setFloat(10, nSize);
          prop.setFloat(10, nSize);
          // java prop.setFloat(37, nSize);
          prop.setFloat(37, nSize);
          // java this.setRunProperty(prop.freeze(), 2);
          self.setRunProperty(&prop.freeze(), 2);
            // java this.mRange._commit("set font size and sizeBi");
            self.mRange._commit(&"set font size and sizeBi");
    }

    // java private void setFontSizeBi(float nSize)
    pub fn setFontSizeBi(&mut self, nSize: f32) {
      // java this.setProperty(37, nSize, "Font SizeBi");
      self.setProperty__int_object_string(37, &nSize, &"Font SizeBi");
    }

    // java public void setFontSize(float nSize)
    pub fn setFontSize(&mut self, nSize: f32) {
      // java this.setProperty(10, nSize, "Font Size");
      self.setProperty__int_object_string(10, &nSize, &"Font Size");
    }

    // java public Float getSize()
    pub fn getSize(&self) -> f32 {
      // java return (Float)this._getMultiProperty(10);
      return ((self._getMultiProperty(10)) as f32);
    }

    // java public Float getFontSize()
    pub fn getFontSize(&self) -> f32 {
      // java return this.getSize();
      return self.getSize();
    }

    // java public float getMinimumSize()
    pub fn getMinimumSize(&self) -> f32 {
      // java return this.mRange.getMinimumFontSize();
      return self.mRange.getMinimumFontSize();
    }

    // java public void grow()
    pub fn grow(&mut self) {
      // java Float size = this.getSize();
      let mut size: f32 = self.getSize();
      // java this.mRange._start();
      self.mRange._start();
          // java if (size != null)
          if size.is_some() {
            // java this.setSize(DefaultRunProperties.growSize(size));
            self.setSize(DefaultRunProperties::growSize(size));
          }
          // java else if (this.mRange.hasSelection())
          else if self.mRange.hasSelection() {
            // java this.changeRunProperty(new GrowShrink(true));
            self.changeRunProperty(&GrowShrink::new(true));
          }
            // java this.mRange._commit("Grow Font");
            self.mRange._commit(&"Grow Font");
    }

    // java public void shrink()
    pub fn shrink(&mut self) {
      // java Float size = this.getSize();
      let mut size: f32 = self.getSize();
      // java this.mRange._start();
      self.mRange._start();
          // java if (size != null)
          if size.is_some() {
            // java this.setSize(DefaultRunProperties.shrinkSize(size));
            self.setSize(DefaultRunProperties::shrinkSize(size));
          }
          // java else if (this.mRange.hasSelection())
          else if self.mRange.hasSelection() {
            // java this.changeRunProperty(new GrowShrink(false));
            self.changeRunProperty(&GrowShrink::new(false));
          }
            // java this.mRange._commit("Shrink Font");
            self.mRange._commit(&"Shrink Font");
    }

    // java public void setSubscript(boolean bSubscript)
    pub fn setSubscript(&mut self, bSubscript: bool) {
      // java int nVAlign = bSubscript ? 2 : 0;
      let mut nVAlign: i32 = (if bSubscript { 2 } else { 0 });
      // java this.setProperty(13, nVAlign, "Subscript");
      self.setProperty__int_object_string(13, &nVAlign, &"Subscript");
    }

    // java public Boolean getSubscript()
    pub fn getSubscript(&self) -> Option<bool> {
      // java Integer v = (Integer)this._getProperty(13);
      let mut v: i32 = ((self._getProperty(13)) as i32);
      // java return v == null ? null : v == 2;
      return (if v.is_none() { None } else { Some(v == 2) });
    }

    // java public void setSuperscript(boolean bSuperscript)
    pub fn setSuperscript(&mut self, bSuperscript: bool) {
      // java int nVAlign = bSuperscript ? 1 : 0;
      let mut nVAlign: i32 = (if bSuperscript { 1 } else { 0 });
      // java this.setProperty(13, nVAlign, "Superscript");
      self.setProperty__int_object_string(13, &nVAlign, &"Superscript");
    }

    // java public Boolean getSuperscript()
    pub fn getSuperscript(&self) -> Option<bool> {
      // java Integer v = (Integer)this._getProperty(13);
      let mut v: i32 = ((self._getProperty(13)) as i32);
      // java return v == null ? null : v == 1;
      return (if v.is_none() { None } else { Some(v == 1) });
    }

    // java public Boolean getSmallCapital()
    pub fn getSmallCapital(&self) -> bool {
      // java return (Boolean)this._getProperty(16);
      return todo!("cast → bool");
    }

    // java public Boolean getAllCapital()
    pub fn getAllCapital(&self) -> bool {
      // java return (Boolean)this._getProperty(17);
      return todo!("cast → bool");
    }

    // java public void setSmallCapital(boolean b)
    pub fn setSmallCapital(&mut self, b: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(17, false, "", 3);
          self.setProperty__int_object_string_int(17, &false, &"", 3);
          // java this.setProperty(16, b, "");
          self.setProperty__int_object_string(16, &b, &"");
            // java this.mRange._commit("Small Capital");
            self.mRange._commit(&"Small Capital");
    }

    // java public void setAllCapital(boolean b)
    pub fn setAllCapital(&mut self, b: bool) {
      // java this.mRange._start();
      self.mRange._start();
          // java this.setProperty(16, false, "", 3);
          self.setProperty__int_object_string_int(16, &false, &"", 3);
          // java this.setProperty(17, b, "");
          self.setProperty__int_object_string(17, &b, &"");
            // java this.mRange._commit("All Capital");
            self.mRange._commit(&"All Capital");
    }

    // java public void setRunProperty(PropertySet prop, int mode)
    pub fn setRunProperty(&mut self, prop: &PropertySet, mode: i32) {
      // java RangeWriteLock lock = this.writeLock();
      let mut lock: RangeWriteLock = self.writeLock();
      // java this.mRange._start();
      self.mRange._start();
          // java this.mRange.setRunProperty(prop, mode);
          self.mRange.setRunProperty(&prop, mode);
            // java this.mRange._commit("set run property");
            self.mRange._commit(&"set run property");
            // java lock.unlock();
            lock.unlock();
    }

    // java public void changeRunProperty(ITextStream.IPropertyModify op)
    pub fn changeRunProperty(&mut self, op: &ITextStream::IPropertyModify) {
      // java this.mRange.changeRunProperty(op);
      self.mRange.changeRunProperty(&op);
    }

}

// java  class GrowShrink implements ITextStream.IPropertyModify
impl GrowShrink {
    // java   GrowShrink(boolean grow)
    pub fn GrowShrink(grow: bool) -> Self {
      // java this.mGrow = grow;
      Self { mGrow: grow }
    }

    // java public void compose(ITable.Entry entry, PropertyAction action)
    pub fn compose(&self, entry: &dyn ITableEntry, action: &mut dyn PropertyAction) {
      // java IChpxTable.Entry frag = (IChpxTable.Entry)entry;
      let mut frag: Box<dyn ChpxEntry> = todo!("cast → Box<dyn ChpxEntry>");
      // java PropertyTool tool = new PropertyTool(KFont.this.mRange.getStory());
      let mut tool: PropertyTool = PropertyTool::new(self.mRange.getStory());
      // java PropertySet p = tool.getMergedRunProperty(frag, true);
      let mut p: PropertySet = tool.getMergedRunProperty(frag, true);
      // java float fontSize = p.getFloat(10);
      let mut fontSize: f32 = p.getFloat(10);
      // java float fontSizeCs = p.getFloat(37, fontSize);
      let mut fontSizeCs: f32 = p.getFloat(37, fontSize);
      // java fontSize = this.mGrow ? DefaultRunProperties.growSize(fontSize) : DefaultRunProperties.shrinkSize(fontSize);
      fontSize = (if self.mGrow { DefaultRunProperties::growSize(fontSize) } else { DefaultRunProperties::shrinkSize(fontSize) });
      // java action.addOperation(PropertyOperations.merge(10, fontSize));
      action.addOperation(&PropertyOperations::merge__int_object(10, &fontSize));
      // java fontSizeCs = this.mGrow ? DefaultRunProperties.growSize(fontSizeCs) : DefaultRunProperties.shrinkSize(fontSizeCs);
      fontSizeCs = (if self.mGrow { DefaultRunProperties::growSize(fontSizeCs) } else { DefaultRunProperties::shrinkSize(fontSizeCs) });
      // java action.addOperation(PropertyOperations.merge(37, fontSizeCs));
      action.addOperation(&PropertyOperations::merge__int_object(37, &fontSizeCs));
    }

}
