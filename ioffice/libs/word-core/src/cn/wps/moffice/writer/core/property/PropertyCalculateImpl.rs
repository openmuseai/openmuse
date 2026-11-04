// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
use util::moffice::property::PropertySet::PropertyValue;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.DefaultParagraphProperties
use crate::cn::wps::moffice::writer::data::DefaultParagraphProperties::*;
// Java: import cn.wps.moffice.writer.data.DefaultRunProperties
use crate::cn::wps::moffice::writer::data::DefaultRunProperties::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.TxTabStops
use crate::cn::wps::moffice::writer::data::value::TxTabStops::*;
// Java: import cn.wps.moffice.writer.core.property.IPropertyCalculate
use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::*;
// Java: import cn.wps.moffice.writer.core.property.StyleCache
use crate::cn::wps::moffice::writer::core::property::StyleCache::*;
use crate::cn::wps::moffice::writer::data::ITextStream::ITextStream;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;

// java private static final int[] spaceBeforeTypes = new int[]{217, 218, 219};
static spaceBeforeTypes: [i32; 3] = [217, 218, 219];
// java private static final int[] spaceAfterTypes = new int[]{214, 215, 216};
static spaceAfterTypes: [i32; 3] = [214, 215, 216];

// java  class PropertyCalculateImpl implements IPropertyCalculate
pub struct PropertyCalculateImpl<'a> {
    // java private WeakReference<Document> mDocument;
    // Rust scope guard: no document pointer survives the borrowed document.
    // Factory weak-key caching remains a separate, uncompiled milestone.
    mDocument: &'a dyn Document,
    // java private StyleCache mStyleCache;
    mStyleCache: Rc<RefCell<StyleCache<'a>>>,
}

// java public interface IPropertyCalculate
impl IPropertyCalculate for PropertyCalculateImpl<'_> {
    // java public PropertySet getStyleFormat(int sti, int options)
    fn getStyleFormat__int_int(&self, sti: i32, options: i32) -> PropertySet {
      // java return this.mStyleCache.getStyleFormat(sti);
      return self.mStyleCache.borrow().getStyleFormat(sti);
    }

    // java public PropertySet getRunStyleFormat(int sti, int options)
    fn getRunStyleFormat(&self, sti: i32, options: i32) -> PropertySet {
      // java return this.mStyleCache.getRunStyleFormat(sti);
      return self.mStyleCache.borrow().getRunStyleFormat(sti);
    }

    // java public PropertySet getParagraphStyleFormat(int sti, int options)
    fn getParagraphStyleFormat(&self, sti: i32, options: i32) -> PropertySet {
      // java return this.mStyleCache.getParagraphStyleFormat(sti);
      return self.mStyleCache.borrow().getParagraphStyleFormat(sti);
    }

    // java public Object getStyleFormat(int sti, int prop, int options)
    fn getStyleFormat__int_int_int(&self, sti: i32, prop: i32, options: i32) -> Option<PropertyValue> {
      // java return this.getStyleFormat(sti, options).getValue(prop);
      return self.getStyleFormat__int_int(sti, options).getValue(prop).cloned();
    }

    // java public PropertySet getPap(IPapxTable.Entry entry, int options)
    fn getPap__entry_int(&self, entry: &dyn PapxEntry, options: i32) -> PropertySet {
      // java checkPointer(entry);
      Self::checkPointer(Some(&entry));
      // java MutablePropertySet pap = new MutablePropertySet();
      let mut pap: MutablePropertySet = MutablePropertySet::new();
      // java pap.add(DefaultParagraphProperties.getProperties());
      pap.add(&DefaultParagraphProperties::getProperties());
      // java if (withRevision(options))
      if Self::withRevision(options) {
        // java this.getMergedParaPropertyWithRevision(pap, entry.getProperty(), options);
        self.getMergedParaPropertyWithRevision__mutablepropertyset_propertyset_int(&mut pap, PapxEntry::getProperty(entry).expect("PAPX property"), options);
      }
      // java else
      else {
        // java this.getMergedParaProperty(pap, entry.getProperty(), options);
        self.getMergedParaProperty(&mut pap, PapxEntry::getProperty(entry).expect("PAPX property"), options);
      }
      // java return pap.freeze();
      return pap.freeze();
    }

    // java public Object getPap(IPapxTable.Entry entry, int prop, int options)
    fn getPap__entry_int_int(&self, entry: &dyn PapxEntry, prop: i32, options: i32) -> Option<PropertyValue> {
      // java checkPointer(entry);
      Self::checkPointer(Some(&entry));
      // java return this.getPap(entry, options).getValue(prop);
      return self.getPap__entry_int(entry, options).getValue(prop).cloned();
    }

    // java public PropertySet getPap(IPapxTable.Entry entry, PropertySet incrementalPapx, int options)
    fn getPap__entry_propertyset_int(&self, entry: &dyn PapxEntry, incrementalPapx: &PropertySet, options: i32) -> PropertySet {
      // java checkPointer(entry);
      Self::checkPointer(Some(&entry));
      // java MutablePropertySet m = new MutablePropertySet();
      let mut m: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedParaPropertyWithRevision(m, entry.getProperty(), incrementalPapx, options);
      self.getMergedParaPropertyWithRevision__mutablepropertyset_propertyset_propertyset_int(&mut m, PapxEntry::getProperty(entry).expect("PAPX property"), Some(incrementalPapx), options);
      // java return m.freeze();
      return m.freeze();
    }

    // java public PropertySet getChp(IChpxTable.Entry entry, int options)
    fn getChp__entry_int(&self, entry: &dyn ChpxEntry, options: i32) -> PropertySet {
      // java checkPointer(entry);
      Self::checkPointer(Some(&entry));
      // java KSAssert.assertFalse(entry.isEnd());
      assert!(!entry.isEnd());
      // java IPapxTable.Entry papx = this.getDocument().getTextStream().seekPapxTable(entry.position());
      let stream = self.getDocument().getTextStream().expect("document text stream");
      let papx = unsafe { ITextStream::seekPapxTable(&*stream, entry.position()) }.expect("PAPX entry");
      // java MutablePropertySet m = new MutablePropertySet();
      let mut m: MutablePropertySet = MutablePropertySet::new();
      // java m.add(DefaultRunProperties.getProperties());
      m.add(&DefaultRunProperties::getProperties());
      // java if (withRevision(options))
      if Self::withRevision(options) {
        // java this.getMergedRunPropertyWithRevision(m, entry.getProperty(), papx.getProperty());
        self.getMergedRunPropertyWithRevision__mutablepropertyset_propertyset_propertyset(&mut m, ChpxEntry::getProperty(entry).expect("CHPX property"), PapxEntry::getProperty(&*papx).expect("PAPX property"));
      }
      // java else
      else {
        // java this.getMergedRunProperty(m, entry.getProperty(), papx.getProperty());
        self.getMergedRunProperty(&mut m, ChpxEntry::getProperty(entry).expect("CHPX property"), PapxEntry::getProperty(&*papx).expect("PAPX property"));
      }
      // java return m.freeze();
      return m.freeze();
    }

    // java public PropertySet getMergeChp(PropertySet chpx, PropertySet papx, int options)
    fn getMergeChp(&self, chpx: &PropertySet, papx: &PropertySet, options: i32) -> PropertySet {
      // java MutablePropertySet m = new MutablePropertySet();
      let mut m: MutablePropertySet = MutablePropertySet::new();
      // java m.add(DefaultRunProperties.getProperties());
      m.add(&DefaultRunProperties::getProperties());
      // java if (withRevision(options))
      if Self::withRevision(options) {
        // java this.getMergedRunPropertyWithRevision(m, chpx, papx);
        self.getMergedRunPropertyWithRevision__mutablepropertyset_propertyset_propertyset(&mut m, chpx, papx);
      }
      // java else
      else {
        // java this.getMergedRunProperty(m, chpx, papx);
        self.getMergedRunProperty(&mut m, chpx, papx);
      }
      // java return m.freeze();
      return m.freeze();
    }

    // java public Object getChp(IChpxTable.Entry entry, int prop, int options)
    fn getChp__entry_int_int(&self, entry: &dyn ChpxEntry, prop: i32, options: i32) -> Option<PropertyValue> {
      // java checkPointer(entry);
      Self::checkPointer(Some(&entry));
      // java KSAssert.assertFalse(entry.isEnd());
      assert!(!entry.isEnd());
      // java return this.getChp(entry, options).getValue(prop);
      return self.getChp__entry_int(entry, options).getValue(prop).cloned();
    }

    // java public PropertySet getChp(IChpxTable.Entry entry, PropertySet incrementalChpx)
    fn getChp__entry_propertyset(&self, entry: &dyn ChpxEntry, incrementalChpx: &PropertySet) -> PropertySet {
      // java checkPointer(entry);
      Self::checkPointer(Some(&entry));
      // java KSAssert.assertFalse(entry.isEnd());
      assert!(!entry.isEnd());
      // java IPapxTable.Entry papx = this.getDocument().getTextStream().seekPapxTable(entry.position());
      let stream = self.getDocument().getTextStream().expect("document text stream");
      let papx = unsafe { ITextStream::seekPapxTable(&*stream, entry.position()) }.expect("PAPX entry");
      // java MutablePropertySet m = new MutablePropertySet();
      let mut m: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedRunPropertyWithRevision(m, entry.getProperty(), papx.getProperty(), incrementalChpx);
      self.getMergedRunPropertyWithRevision__mutablepropertyset_propertyset_propertyset_propertyset(&mut m, ChpxEntry::getProperty(entry).expect("CHPX property"), PapxEntry::getProperty(&*papx).expect("PAPX property"), Some(incrementalChpx));
      // java return m.freeze();
      return m.freeze();
    }

    // java public Object getLanguageCode(IChpxTable.Entry entry)
    fn getLanguageCode(&self, entry: &dyn ChpxEntry) -> Option<PropertyValue> {
      // java PropertySet mergedRunProp = this.getChp(entry, 3);
      let mergedRunProp: PropertySet = self.getChp__entry_int(entry, 3);
      // java Object hint = mergedRunProp.getValue(40);
      let hint = mergedRunProp.getValue(40);
      // java int propId;
      let propId: i32;
      // java if (null == hint)
      if hint.is_none() {
        // java boolean isComplexScript = mergedRunProp.getBoolean(72, false);
        let isComplexScript = mergedRunProp.getBoolean__int_boolean(72, false);
        // java if (isComplexScript)
        if isComplexScript {
          // java propId = 43;
          propId = 43;
        }
        // java else
        else {
          // java Integer val = (Integer)mergedRunProp.getObject(42, (Object)null);
          let val = mergedRunProp.getValue(42);
          if let Some(value) = val {
            value.as_ref().as_any().downcast_ref::<i32>().expect("PropertyCalculateImpl: language code must be Integer");
          }
          // java if (null != val)
          if let Some(val) = val {
            // java return val;
            return Some(val.clone());
          }
          // java propId = 41;
          propId = 41;
        }
      }
      // java else if ((Integer)hint == 1)
      else if *hint.unwrap().as_ref().as_any().downcast_ref::<i32>().expect("PropertyCalculateImpl: hint must be Integer") == 1 {
        // java propId = 42;
        propId = 42;
      }
      // java else if ((Integer)hint == 0)
      else if *hint.unwrap().as_ref().as_any().downcast_ref::<i32>().expect("PropertyCalculateImpl: hint must be Integer") == 0 {
        // java propId = 41;
        propId = 41;
      }
      // java else
      else {
        // java propId = 43;
        propId = 43;
      }
      // java return mergedRunProp.getValue(propId);
      return mergedRunProp.getValue(propId).cloned();
    }

}

// java  class PropertyCalculateImpl implements IPropertyCalculate
impl<'a> PropertyCalculateImpl<'a> {
    // java public  PropertyCalculateImpl(Document document, StyleCache styleCache)
    pub fn PropertyCalculateImpl(document: &'a dyn Document, styleCache: Rc<RefCell<StyleCache<'a>>>) -> Self {
      // java this.mDocument = new WeakReference(document);
      // java this.mStyleCache = styleCache;
      Self { mDocument: document, mStyleCache: styleCache }
    }

    // java private static boolean withRevision(int options)
    pub fn withRevision(options: i32) -> bool {
      // java return (options & 1) != 0;
      return options & 1 != 0;
    }

    // java private static boolean withNumbering(int options)
    pub fn withNumbering(options: i32) -> bool {
      // java return (options & 2) != 0;
      return options & 2 != 0;
    }

    // java public PropertySet getDefaultChp()
    pub fn getDefaultChp(&self) -> PropertySet {
      // java return this.mStyleCache.getDefaultChp();
      return self.mStyleCache.borrow().getDefaultChp();
    }

    // java public PropertySet getDefaultPap()
    pub fn getDefaultPap(&self) -> PropertySet {
      // java return this.mStyleCache.getDefaultPap();
      return self.mStyleCache.borrow().getDefaultPap();
    }

    // java static void removeIndirectFormatChp(MutablePropertySet m)
    pub fn removeIndirectFormatChp(m: &MutablePropertySet) {
    }

    // java static void removeIndirectFormatPap(MutablePropertySet m)
    pub fn removeIndirectFormatPap(m: &MutablePropertySet) {
    }

    // java private static void checkPointer(Object entry)
    pub fn checkPointer<T: ?Sized>(entry: Option<&T>) {
      // java if (entry == null)
      if entry.is_none() {
        // java throw new NullPointerException("The entry must not be null");
        panic!("The entry must not be null");
      }
    }

    // java private void getMergedParaProperty(MutablePropertySet result, PropertySet papx, int options)
    pub fn getMergedParaProperty(&self, result: &mut MutablePropertySet, papx: &PropertySet, options: i32) {
      // java checkPointer(papx);
      Self::checkPointer(Some(&papx));
      // java this.mergePapx(result, this.getDefaultPap());
      self.mergePapx(result, Some(&self.getDefaultPap()));
      // java this.mergePapx(result, this.getParagraphStyleFormat(papx.getInt(191, 0), 0));
      self.mergePapx(result, Some(&self.getParagraphStyleFormat(papx.getInt__int_int(191, 0), 0)));
      // java if (withNumbering(options))
      if Self::withNumbering(options) {
        // java int level = this.getListLevel(result, papx);
        let level = self.getListLevel__mutablepropertyset_propertyset(result, papx);
        // java result.setInt(234, level);
        result.setInt(234, level);
        // java this.mergeNumbering(result, this.getLevelFormat(result.get()));
        self.mergeNumbering(result, self.getLevelFormat(result.get()).as_ref());
        // java this.mergeNumbering(result, this.getLevelFormat(papx));
        self.mergeNumbering(result, self.getLevelFormat(papx).as_ref());
      }
      // java this.mergePapx(result, papx);
      self.mergePapx(result, Some(papx));
      // java removeIndirectFormatPap(result);
      Self::removeIndirectFormatPap(&result);
    }

    // java private int getListLevel(MutablePropertySet result, PropertySet papx)
    pub fn getListLevel__mutablepropertyset_propertyset(&self, result: &MutablePropertySet, papx: &PropertySet) -> i32 {
      // java int level = result.getInt(234, 0);
      let level = result.getInt_with_default(234, 0);
      // java return papx.getInt(234, level);
      return papx.getInt__int_int(234, level);
    }

    // java private void getMergedParaPropertyWithRevision(MutablePropertySet result, PropertySet papx, int options)
    pub fn getMergedParaPropertyWithRevision__mutablepropertyset_propertyset_int(&self, result: &mut MutablePropertySet, papx: &PropertySet, options: i32) {
      // java Revision revision = (Revision)papx.getValue(237);
      let revision = papx.getValue(237).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>());
      // java PropertySet revision_papx = revision == null ? null : revision.getProperty();
      let revision_papx = revision.and_then(Revision::getProperty);
      // java this.getMergedParaPropertyWithRevision(result, papx, revision_papx, options);
      self.getMergedParaPropertyWithRevision__mutablepropertyset_propertyset_propertyset_int(result, papx, revision_papx.as_ref(), options);
    }

    // java private void getMergedParaPropertyWithRevision(MutablePropertySet result, PropertySet papx, PropertySet revision_papx, int options)
    pub fn getMergedParaPropertyWithRevision__mutablepropertyset_propertyset_propertyset_int(&self, result: &mut MutablePropertySet, papx: &PropertySet, revision_papx: Option<&PropertySet>, options: i32) {
      // java checkPointer(papx);
      Self::checkPointer(Some(&papx));
      // java this.mergePapx(result, this.getDefaultPap());
      self.mergePapx(result, Some(&self.getDefaultPap()));
      // java int revisionSti = revision_papx == null ? 4095 : revision_papx.getInt(191, 4095);
      let revisionSti = revision_papx.map_or(4095, |p| p.getInt__int_int(191, 4095));
      // java if (revisionSti != 4095)
      if revisionSti != 4095 {
        // java this.mergePapx(result, this.getParagraphStyleFormat(revisionSti, 1));
        self.mergePapx(result, Some(&self.getParagraphStyleFormat(revisionSti, 1)));
      }
      // java else
      else {
        // java this.mergePapx(result, this.getParagraphStyleFormat(papx.getInt(191, 0), 1));
        self.mergePapx(result, Some(&self.getParagraphStyleFormat(papx.getInt__int_int(191, 0), 1)));
      }
      // java if (withNumbering(options))
      if Self::withNumbering(options) {
        // java result.setInt(234, this.getListLevel(result, papx, revision_papx, revisionSti));
        let level = self.getListLevel__mutablepropertyset_propertyset_propertyset_int(result, papx, revision_papx, revisionSti);
        result.setInt(234, level);
        // java this.mergeNumbering(result, this.getLevelFormat(result.get()));
        self.mergeNumbering(result, self.getLevelFormat(result.get()).as_ref());
        // java this.mergeNumbering(result, this.getLevelFormat(papx));
        self.mergeNumbering(result, self.getLevelFormat(papx).as_ref());
      }
      // java if (revisionSti != 4095)
      if revisionSti != 4095 {
        // java this.mergePapx(result, revision_papx);
        self.mergePapx(result, revision_papx);
      }
      // java else
      else {
        // java this.mergePapx(result, papx);
        self.mergePapx(result, Some(papx));
        // java this.mergePapx(result, revision_papx);
        self.mergePapx(result, revision_papx);
      }
      // java removeIndirectFormatPap(result);
      Self::removeIndirectFormatPap(&result);
    }

    // java private int getListLevel(MutablePropertySet result, PropertySet papx, PropertySet revision_papx, int revisionSti)
    pub fn getListLevel__mutablepropertyset_propertyset_propertyset_int(&self, result: &MutablePropertySet, papx: &PropertySet, revision_papx: Option<&PropertySet>, revisionSti: i32) -> i32 {
      // java int level = result.getInt(234, 0);
      let mut level = result.getInt_with_default(234, 0);
      // java if (revisionSti != 4095)
      if revisionSti != 4095 {
        // java level = revision_papx.getInt(234, level);
        level = revision_papx.expect("revision paragraph property").getInt__int_int(234, level);
      }
      // java else
      else {
        // java level = papx.getInt(234, level);
        level = papx.getInt__int_int(234, level);
        // java if (null != revision_papx)
        if let Some(revision_papx) = revision_papx {
          // java level = revision_papx.getInt(234, level);
          level = revision_papx.getInt__int_int(234, level);
        }
      }
      // java return level;
      return level;
    }

    // java private void mergeNumbering(MutablePropertySet result, PropertySet numberingPapx)
    pub fn mergeNumbering(&self, result: &mut MutablePropertySet, numberingPapx: Option<&PropertySet>) {
      // java if (numberingPapx != null)
      if let Some(numberingPapx) = numberingPapx {
        // java int sti = numberingPapx.getInt(191, 4095);
        let mut sti: i32 = numberingPapx.getInt__int_int(191, 4095);
        // java if (sti != 4095)
        if sti != 4095 {
          // java this.mergePapx(result, this.getParagraphStyleFormat(sti, 0));
          self.mergePapx(result, Some(&self.getParagraphStyleFormat(sti, 0)));
        }
        // java this.mergePapx(result, numberingPapx);
        self.mergePapx(result, Some(numberingPapx));
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
        let lfoData = self.getNumberingInstance(lfo);
        // java if (lfoData != null)
        if let Some(lfoData) = lfoData {
          // java LstData lstData = this.getNumberingStyle(lfoData.getLsid());
          let lstData = self.getNumberingStyle(lfoData.getLsid());
          // java if (lstData != null)
          if let Some(lstData) = lstData {
            // java LvlfData lvlfData = lstData.getLvlfDataByIlvl(lvl);
            let lvlfData = lstData.getLvlfDataByIlvl(lvl);
            // java return lvlfData == null ? null : lvlfData.getParaProp();
            return lvlfData.and_then(|level| level.getParaProp());
          }
        }
        // java return null;
        return None;
      }
    }

    // java private LstData getNumberingStyle(int lsid)
    pub fn getNumberingStyle(&self, lsid: i32) -> Option<LstData> {
      // java return this.getNumbering().getLstDataTable().getDataByLsid(lsid);
      return self.getNumbering().mLstTable.as_ref().expect("numbering list table").getDataByLsid(lsid);
    }

    // java private LfoData getNumberingInstance(int lfo)
    pub fn getNumberingInstance(&self, lfo: i32) -> Option<LfoData> {
      // java return this.getNumbering().getLfoDataTable().getDataByIlfo(lfo);
      return self.getNumbering().mLfoTable.as_ref().expect("numbering override table").getDataByIlfo(lfo);
    }

    // java private AutoNumTable getNumbering()
    pub fn getNumbering(&self) -> &AutoNumTable {
      // java Document document = this.getDocument();
      let document = self.getDocument();
      // java return document.getTextDocument().getAutoNumTable();
      return document.getTextDocument().expect("text document").getAutoNumTable().expect("numbering table");
    }

    // java private Document getDocument()
    pub fn getDocument(&self) -> &dyn Document {
      // java Document document = (Document)this.mDocument.get();
      // Rust lifetime ties the calculator to its document; a reclaimed document
      // cannot be observed while this borrowed calculator exists.
      return self.mDocument;
    }

    // java private void mergePapx(MutablePropertySet current, PropertySet nextLevel)
    pub fn mergePapx(&self, current: &mut MutablePropertySet, nextLevel: Option<&PropertySet>) {
      // java if (nextLevel != null)
      if let Some(nextLevel) = nextLevel {
        // java adjustParagraphSpace(spaceBeforeTypes, current, nextLevel);
        Self::adjustParagraphSpace(&spaceBeforeTypes, current, nextLevel);
        // java adjustParagraphSpace(spaceAfterTypes, current, nextLevel);
        Self::adjustParagraphSpace(&spaceAfterTypes, current, nextLevel);
        // java TxTabStops tabStops = getMergedTabStops(current, nextLevel);
        let tabStops = Self::getMergedTabStops(current, nextLevel);
        // java current.addLimit(nextLevel, 191, 376);
        current.addLimit(nextLevel, 191, 376);
        // java if (tabStops != null)
        if let Some(tabStops) = tabStops {
          // java current.setValue(236, tabStops);
          current.setValue(236, Some(Arc::new(tabStops)));
        }
      }
    }

    // java private static void adjustParagraphSpace(int[] types, MutablePropertySet current, PropertySet nextLevel)
    pub fn adjustParagraphSpace(types: &[i32], current: &mut MutablePropertySet, nextLevel: &PropertySet) {
      // java if (nextLevel.contains(types[0]))
      if nextLevel.contains(types[0usize]) {
        // java current.remove(types[1]);
        current.remove(types[1usize]);
        // java current.remove(types[2]);
        current.remove(types[2usize]);
      }
      // java else if (nextLevel.contains(types[1]))
      else if nextLevel.contains(types[1usize]) {
        // java current.remove(types[2]);
        current.remove(types[2usize]);
      }
    }

    // java private static TxTabStops getMergedTabStops(MutablePropertySet current, PropertySet nextLevel)
    pub fn getMergedTabStops(current: &MutablePropertySet, nextLevel: &PropertySet) -> Option<TxTabStops> {
      // java TxTabStops current_tab = (TxTabStops)current.getValue(236);
      let current_tab = current.getValue(236).and_then(|value| value.as_ref().as_any().downcast_ref::<TxTabStops>());
      // java TxTabStops nextLevel_tab = (TxTabStops)nextLevel.getValue(236);
      let nextLevel_tab = nextLevel.getValue(236).and_then(|value| value.as_ref().as_any().downcast_ref::<TxTabStops>());
      // java return null == current_tab && null == nextLevel_tab ? null : TxTabStops.merge(current_tab, nextLevel_tab);
      return if current_tab.is_none() && nextLevel_tab.is_none() { None } else { TxTabStops::merge(current_tab, nextLevel_tab) };
    }

    // java private void getMergedRunProperty(MutablePropertySet result, PropertySet chpx, PropertySet papx)
    pub fn getMergedRunProperty(&self, result: &mut MutablePropertySet, chpx: &PropertySet, papx: &PropertySet) {
      // java checkPointer(chpx);
      Self::checkPointer(Some(&chpx));
      // java mergeChpx(result, this.getDefaultChp());
      Self::mergeChpx(result, &self.getDefaultChp());
      // java int paraStyle = papx.getInt(191, 4095);
      let mut paraStyle: i32 = papx.getInt__int_int(191, 4095);
      // java mergeChpx(result, this.getParagraphStyleFormat(paraStyle, 0));
      Self::mergeChpx(result, &self.getParagraphStyleFormat(paraStyle, 0));
      // java int sti = chpx.getInt(2, 4095);
      let mut sti: i32 = chpx.getInt__int_int(2, 4095);
      // java if (sti != 4095)
      if sti != 4095 {
        // java mergeChpx(result, this.getRunStyleFormat(sti, 0));
        Self::mergeChpx(result, &self.getRunStyleFormat(sti, 0));
      }
      // java mergeChpx(result, chpx);
      Self::mergeChpx(result, chpx);
      // java removeIndirectFormatChp(result);
      Self::removeIndirectFormatChp(&result);
    }

    // java private void getMergedRunPropertyWithRevision(MutablePropertySet result, PropertySet chpx, PropertySet papx)
    pub fn getMergedRunPropertyWithRevision__mutablepropertyset_propertyset_propertyset(&self, result: &mut MutablePropertySet, chpx: &PropertySet, papx: &PropertySet) {
      // java Revision revision = (Revision)chpx.getValue(48);
      let revision = chpx.getValue(48).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>());
      // java PropertySet revision_chpx = revision == null ? null : revision.getProperty();
      let revision_chpx = revision.and_then(Revision::getProperty);
      // java this.getMergedRunPropertyWithRevision(result, chpx, papx, revision_chpx);
      self.getMergedRunPropertyWithRevision__mutablepropertyset_propertyset_propertyset_propertyset(result, chpx, papx, revision_chpx.as_ref());
    }

    // java private void getMergedRunPropertyWithRevision(MutablePropertySet result, PropertySet chpx, PropertySet papx, PropertySet revision_chpx)
    pub fn getMergedRunPropertyWithRevision__mutablepropertyset_propertyset_propertyset_propertyset(&self, result: &mut MutablePropertySet, chpx: &PropertySet, papx: &PropertySet, revision_chpx: Option<&PropertySet>) {
      // java checkPointer(chpx);
      Self::checkPointer(Some(&chpx));
      // java mergeChpx(result, this.getDefaultChp());
      Self::mergeChpx(result, &self.getDefaultChp());
      // java int paraStyle = getParagraphStyleWithRevision(papx);
      let mut paraStyle: i32 = Self::getParagraphStyleWithRevision(&papx);
      // java mergeChpx(result, this.getParagraphStyleFormat(paraStyle, 1));
      Self::mergeChpx(result, &self.getParagraphStyleFormat(paraStyle, 1));
      // java int revisionSti = revision_chpx == null ? 4095 : revision_chpx.getInt(2, 4095);
      let revisionSti = revision_chpx.map_or(4095, |p| p.getInt__int_int(2, 4095));
      // java int sti = revisionSti;
      let mut sti: i32 = revisionSti;
      // java if (revisionSti == 4095)
      if revisionSti == 4095 {
        // java sti = chpx.getInt(2, 4095);
        sti = chpx.getInt__int_int(2, 4095);
      }
      // java if (sti != 4095)
      if sti != 4095 {
        // java mergeChpx(result, this.getRunStyleFormat(sti, 1));
        Self::mergeChpx(result, &self.getRunStyleFormat(sti, 1));
      }
      // java if (revisionSti != 4095)
      if revisionSti != 4095 {
        // java mergeChpx(result, revision_chpx);
        Self::mergeChpx(result, revision_chpx.expect("revision run property"));
      }
      // java else
      else {
        // java mergeChpx(result, chpx);
        Self::mergeChpx(result, chpx);
        // java mergeChpx(result, revision_chpx);
        if let Some(revision_chpx) = revision_chpx { Self::mergeChpx(result, revision_chpx); }
      }
      // java removeIndirectFormatChp(result);
      Self::removeIndirectFormatChp(&result);
    }

    // java private static int getParagraphStyleWithRevision(PropertySet papx)
    pub fn getParagraphStyleWithRevision(papx: &PropertySet) -> i32 {
      // java int revision_sti = 4095;
      let mut revision_sti: i32 = 4095;
      // java Revision revision = (Revision)papx.getValue(237);
      let revision = papx.getValue(237).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>());
      // java if (revision != null)
      if let Some(revision) = revision {
        // java PropertySet revision_papx = revision.getProperty();
        let revision_papx = revision.getProperty();
        // java if (revision_papx != null)
        if let Some(revision_papx) = revision_papx {
          // java revision_sti = revision_papx.getInt(191, 4095);
          revision_sti = revision_papx.getInt__int_int(191, 4095);
        }
      }
      // java return revision_sti != 4095 ? revision_sti : papx.getInt(191, 0);
      return (if revision_sti != 4095 { revision_sti } else { papx.getInt__int_int(191, 0) });
    }

    // java private static void mergeChpx(MutablePropertySet result, PropertySet nextLevel)
    pub fn mergeChpx(result: &mut MutablePropertySet, nextLevel: &PropertySet) {
      // java result.addLimit(nextLevel, 1, 191);
      result.addLimit(nextLevel, 1, 191);
    }

}
