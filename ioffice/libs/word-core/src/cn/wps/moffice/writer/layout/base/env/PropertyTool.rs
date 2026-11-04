#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unused_assignments, unreachable_code)]
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{PropertySet, PropertyValue};
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
// Java: import cn.wps.moffice.writer.data.DefaultParagraphProperties
use crate::cn::wps::moffice::writer::data::DefaultParagraphProperties::*;
// Java: import cn.wps.moffice.writer.data.DefaultRunProperties
use crate::cn::wps::moffice::writer::data::DefaultRunProperties::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::KStyle;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::AutoNumTable;
// Java: import cn.wps.moffice.writer.data.value.TxTabStops;
use crate::cn::wps::moffice::writer::data::value::TxTabStops::TxTabStops;
// Java: import cn.wps.moffice.writer.layout.base.env.CachedPropertySet
use crate::cn::wps::moffice::writer::layout::base::env::CachedPropertySet::{CachedPropertySet, getFlagBit, LONG_BIT_REVISION_FINAL};
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
use std::collections::HashMap;
use std::sync::Arc;

// java static final int[] spaceBeforeTypes = new int[]{217, 218, 219};
const SPACE_BEFORE_TYPES: [i32; 3] = [217, 218, 219];
// java static final int[] spaceAfterTypes = new int[]{214, 215, 216};
const SPACE_AFTER_TYPES: [i32; 3] = [214, 215, 216];

// rust: wr-29 —— env PropertyTool（Java PropertyTool.java:1-331 忠实移植）。
//       ★合并引擎内联实现（Java 委托 cn.wps.moffice.writer.core.PropertyTool.getMergedParaProperty/
//       getMergedRunProperty 的纯文本级联：defaults → paraStyle 链 → para 直接 → runStyle → run 直接）。
//       编号（bMergedNumbering 的 Lfo/Lvf 展开）与修订（bRevision 的 styleInRevision）按
//       milestone GR-D3 DEFERRED —— hello/sample2/multipage 不触发。
//       缓存层：Java LinkedHashMap 访问序 LRU → HashMap + 满则清（顺序不可观察，L4-5 纪律）。
pub struct PropertyTool {
    // java Document mDocument = null;
    // rust: ★合并引擎内联合并（Java 委托 core.PropertyTool）；mDocument 仅用于转交 styleSheet，
    //       本实现不保留引用（工厂/调用方持有 doc 生命周期）
    // java cn.wps.moffice.writer.core.PropertyTool mImpl —— rust: 内联合并 → PropMergeCore
    // java cn.wps.moffice.writer.core.PropertyTool mImpl（合并引擎）
    mMerge: PropMergeCore,
    // java MutablePropertySet mParaProp = new MutablePropertySet();
    mParaProp: MutablePropertySet,
    // java MutablePropertySet mRunProp = new MutablePropertySet();
    mRunProp: MutablePropertySet,
    // java Object[] mPropValues = null; int[] mPropIds = null;
    mPropValues: Vec<Option<PropertyValue>>,
    mPropIds: Vec<i32>,
    // java ParaPropLRUCache mParaPropCache = null;
    mParaPropCache: HashMap<i64, Box<CachedPropertySet>>,
    // java RunPropLRUCache mRunPropCache = null;
    mRunPropCache: HashMap<i64, Box<CachedPropertySet>>,
    // rust: 非缓存路径 return 的冻结 PropertySet 存活区（Java 直接 clone().freeze() 返回新对象）
    mPendingProps: Vec<Box<PropertySet>>,
    // rust: Java `finalParaProp instanceof CachedPropertySet` 等价 —— 最近一次 getFinalParaProp 返回的 property
    mLastParaPropAddr: usize,
    mLastParaMark: i64,
}

// rust: ★合并引擎（Java 委托 cn.wps.moffice.writer.core.PropertyTool 的纯文本级联部分）
//       供 env PropertyTool 持有，避免 &mut self 与 mParaProp/mRunProp 的借用冲突。
pub(crate) struct PropMergeCore {
    // java KStyles styleSheet（core.PropertyTool 字段）
    mStyles: Option<*mut KStyles>,
    // java this.document.getTextDocument().getAutoNumTable() in numbering merge.
    // The imported TextDocument owns this table throughout layout, like mStyles above.
    mAutoNumTable: Option<*const AutoNumTable>,
    // java PropertySet defaultPapProperty / defaultRunProperty（core.PropertyTool 字段）
    mDefaultPapProperty: Option<PropertySet>,
    mDefaultRunProperty: Option<PropertySet>,
}

impl PropMergeCore {
    pub(crate) fn new(document: &dyn Document) -> Self {
        // java this.mImpl.styleSheet = this.mDocument.getStyles();
        let mStyles: Option<*mut KStyles> = document.getStyles();
        let mAutoNumTable = document.getTextDocument().and_then(|doc| doc.getAutoNumTable()).map(|table| table as *const AutoNumTable);
        // java this.mImpl.defaultPapProperty = PropertySet.compose(DefaultParagraphProperties.getProperties(), this.mImpl.styleSheet.getPPrDefault(), 2);
        let mDefaultPapProperty: Option<PropertySet> = mStyles.map(|ss| PropertySet::compose(&DefaultParagraphProperties::getProperties(), &unsafe { (*ss).getPPrDefault() }, 2));
        // java this.mImpl.defaultRunProperty = PropertySet.compose(DefaultRunProperties.getProperties(), this.mImpl.styleSheet.getRPrDefault(), 2);
        let mDefaultRunProperty: Option<PropertySet> = mStyles.map(|ss| PropertySet::compose(&DefaultRunProperties::getProperties(), &unsafe { (*ss).getRPrDefault() }, 2));
        Self { mStyles, mAutoNumTable, mDefaultPapProperty, mDefaultRunProperty }
    }

    pub(crate) fn dispose(&mut self) {
        self.mStyles = None;
        self.mAutoNumTable = None;
        self.mDefaultPapProperty = None;
        self.mDefaultRunProperty = None;
    }

    // java private PropertySet getParaPropertySet(int id)（core PropertyTool.java:48-72）
    fn getParaPropertySet(&mut self, id: i32) -> PropertySet {
      // java PropertySet para = null;
      // java KStyle baseStyle = this.styleSheet.getStyleById(id);
      let baseStyle: Option<KStyle> = self.mStyles.and_then(|ss| unsafe { (*ss).getStyleById__int(id) });
      // java if (baseStyle != null && baseStyle.getType() == 1)
      if let Some(mut baseStyle) = baseStyle {
        if baseStyle.getType() == 1 {
          // java para = baseStyle.getFinalProperty();
          return baseStyle.getFinalProperty().unwrap_or_else(|| PropertySet::empty());
        }
      }
      // java return para == null ? PropertySet.empty : para;
      return PropertySet::empty();
    }

    // java public PropertySet getRunPropertySet(int id)（core PropertyTool.java:74-100）
    pub fn getRunPropertySet(&mut self, id: i32) -> PropertySet {
      // java PropertySet run = null;
      // java KStyle baseStyle = this.styleSheet.getStyleById(id);
      let baseStyle: Option<KStyle> = self.mStyles.and_then(|ss| unsafe { (*ss).getStyleById__int(id) });
      // java if (baseStyle != null && baseStyle.getType() == 2)
      if let Some(mut baseStyle) = baseStyle {
        if baseStyle.getType() == 2 {
          // java run = baseStyle.getFinalProperty();
          return baseStyle.getFinalProperty().unwrap_or_else(|| PropertySet::empty());
        }
      }
      // java return run;
      return PropertySet::empty();
    }

    // java public void getMergedParaProperty(MutablePropertySet desPropertySet, PropertySet paraProperty, boolean bMergedNumbering, boolean bRevision, boolean bMergeDefaultBase, int tableStyleIdUseToPara)（core PropertyTool.java:321-439）
    pub fn getMergedParaProperty(&mut self, desPropertySet: &mut MutablePropertySet, paraProperty: Option<&PropertySet>, bMergedNumbering: bool, bRevision: bool, bMergeDefaultBase: bool, tableStyleIdUseToPara: i32) {
      // java if (paraProperty != null)
      if let Some(paraProperty) = paraProperty {
        // java boolean styleInRevision = false;
        let mut styleInRevision: bool = false;
        // java int styleName = paraProperty.getInt(191, 0);
        let styleName: i32 = paraProperty.getInt__int_int(191, 0);
        // java Revision revision = null; PropertySet revisionProp = null;
        // java if (paraProperty.contains(237) && bRevision) { ... styleName = revisionProp.getInt(191); styleInRevision = true; }
        // rust: ★wr-29 GR-D3 —— 段修订（237）非 hello；DEFERRED
        let _ = (styleInRevision, bRevision);
        // java PropertySet baseParaProp = this.getParaPropertySet(styleName);
        let baseParaProp: PropertySet = self.getParaPropertySet(styleName);
        // java if (bMergeDefaultBase)
        if bMergeDefaultBase {
          // java desPropertySet.addLimit(this.getDefaultParaProperty(), 191, 376);
          if let Some(dp) = self.mDefaultPapProperty.as_ref() {
            desPropertySet.addLimit(dp, 191, 376);
          }
          // java if (tableStyleIdUseToPara != 4095) { this.useParaFromTableStyle(tableStyleIdUseToPara, desPropertySet); }
          if tableStyleIdUseToPara != 4095 {
            /* DEFERRED_NON_FUNCTION useParaFromTableStyle（Java core PropertyTool.java:445-458） */
          }
          // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, baseParaProp);
          self.justifyParagraphMargin(&SPACE_BEFORE_TYPES, desPropertySet, &baseParaProp);
          // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, baseParaProp);
          self.justifyParagraphMargin(&SPACE_AFTER_TYPES, desPropertySet, &baseParaProp);
        }
        // java if (bMergeDefaultBase || styleName != 0)
        if bMergeDefaultBase || styleName != 0 {
          // java desPropertySet.addLimit(baseParaProp, 191, 376);
          desPropertySet.addLimit(&baseParaProp, 191, 376);
          // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, paraProperty);
          self.justifyParagraphMargin(&SPACE_BEFORE_TYPES, desPropertySet, paraProperty);
          // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, paraProperty);
          self.justifyParagraphMargin(&SPACE_AFTER_TYPES, desPropertySet, paraProperty);
        }
        // java if (bMergedNumbering)
        if bMergedNumbering {
          // java boolean hasNumber = false; int lfo = -1; int lvl = 0;
          let mut hasNumber = false;
          let mut lfo = -1;
          let mut lvl = 0;
          // java if (styleInRevision) { hasNumber = baseParaProp.contains(235); ... }
          if styleInRevision {
            hasNumber = baseParaProp.contains(235);
            if hasNumber {
              lfo = baseParaProp.getInt__int_int(235, -1);
              lvl = baseParaProp.getInt__int_int(234, 0);
            }
          // java else { hasNumber = baseParaProp.contains(235) || paraProperty.contains(235); ... }
          } else {
            hasNumber = baseParaProp.contains(235) || paraProperty.contains(235);
            if hasNumber {
              lfo = if paraProperty.contains(235) { paraProperty.getInt__int_int(235, 0) } else { baseParaProp.getInt__int_int(235, -1) };
              lvl = if paraProperty.contains(234) { paraProperty.getInt__int_int(234, 0) } else { baseParaProp.getInt__int_int(234, 0) };
            }
          }
          // java if (hasNumber)
          if hasNumber {
            // java LfoData lfoData = document.getTextDocument().getAutoNumTable().getLfoDataTable().getDataByIlfo(lfo);
            let table = self.mAutoNumTable.map(|ptr| unsafe { &*ptr });
            let lfoData = table.and_then(|table| table.mLfoTable.as_ref()).and_then(|table| table.getDataByIlfo(lfo));
            // java if (lfoData == null) { remove(208, 209, 278, 211, 210); }
            if let Some(lfoData) = lfoData {
              // java LfoLvlData lfoLvlData = lfoData.getLfoLvlByIlvl(lvl);
              let lfoLvlData = lfoData.getLfoLvlByIlvl(lvl);
              // java if (lfoLvlData != null && lfoLvlData.getLvlfData() != null) ... else LstDataTable lookup.
              let lvlfData = lfoLvlData.and_then(|data| data.getLvlfData()).or_else(|| {
                table.and_then(|table| table.mLstTable.as_ref())
                  .and_then(|table| table.getDataByLsid(lfoData.getLsid()))
                  .and_then(|data| data.getLvlfDataByIlvl(lvl))
              });
              // java if (lvlfData != null && lvlfData.getParaProp() != null)
              if let Some(lvfParaProp) = lvlfData.and_then(|data| data.getParaProp()) {
                // java MutablePropertySet lvfParaProp = new MutablePropertySet();
                let mut mergedLevel = MutablePropertySet::new();
                // java this.getMergedParaProperty(lvfParaProp, lvlfData.getParaProp(), false, bRevision, false, 4095);
                self.getMergedParaProperty(&mut mergedLevel, Some(&lvfParaProp), false, bRevision, false, 4095);
                // java desPropertySet.addLimit(lvfParaProp.freeze(), 191, 376);
                desPropertySet.addLimit(&mergedLevel.freeze(), 191, 376);
                // java this.justifyParagraphMargin(spaceBeforeTypes/spaceAfterTypes, desPropertySet, paraProperty);
                self.justifyParagraphMargin(&SPACE_BEFORE_TYPES, desPropertySet, paraProperty);
                self.justifyParagraphMargin(&SPACE_AFTER_TYPES, desPropertySet, paraProperty);
              }
            } else {
              desPropertySet.remove(208);
              desPropertySet.remove(209);
              desPropertySet.remove(278);
              desPropertySet.remove(211);
              desPropertySet.remove(210);
            }
          }
        }
        // java TxTabStops tabStops = this.getMarginTabs(desPropertySet, paraProperty);
        let tabStops: Option<PropertyValue> = self.getMarginTabs(&desPropertySet, paraProperty);
        // java desPropertySet.addLimit(paraProperty, 191, 376);
        desPropertySet.addLimit(paraProperty, 191, 376);
        // java desPropertySet.setValue(236, tabStops);
        desPropertySet.setValue(236, tabStops);
        // java if (styleInRevision) { ... }
        // rust: DEFERRED —— 修订样式（同 GR-D3）
        // java if (null != revision && null != revisionProp) { ... }
        // rust: DEFERRED —— 修订段属性（同 GR-D3）
      }
    }

    // java private TxTabStops getMarginTabs(MutablePropertySet newProperty, PropertySet baseProperty)
    fn getMarginTabs(&mut self, newProperty: &MutablePropertySet, baseProperty: &PropertySet) -> Option<PropertyValue> {
      // java TxTabStops newStops = (TxTabStops)newProperty.getValue(236);
      let newStops: Option<&PropertyValue> = newProperty.getValue(236);
      // java TxTabStops baseStops = (TxTabStops)baseProperty.getValue(236);
      let baseStops: Option<&PropertyValue> = baseProperty.getValue(236);
      // java return null == newStops && null == baseStops ? null : TxTabStops.merge(newStops, baseStops);
      let newTab = newStops.map(|v| v.as_ref().as_any().downcast_ref::<TxTabStops>().expect("property 236 must be TxTabStops"));
      let baseTab = baseStops.map(|v| v.as_ref().as_any().downcast_ref::<TxTabStops>().expect("property 236 must be TxTabStops"));
      // Java: null on both sides; preserve the original object when only one side exists.
      return match (newTab, baseTab) {
        (None, None) => None,
        (Some(_), None) => newStops.cloned(),
        (None, Some(_)) => baseStops.cloned(),
        (Some(a), Some(b)) => TxTabStops::merge(Some(a), Some(b)).map(|tabs| Arc::new(tabs) as PropertyValue),
      };
    }

    // java private void justifyParagraphMargin(int[] types, MutablePropertySet newProperty, PropertySet baseProperty)
    fn justifyParagraphMargin(&mut self, types: &[i32], newProperty: &mut MutablePropertySet, baseProperty: &PropertySet) {
      // java if (baseProperty.contains(types[0]))
      if baseProperty.contains(types[0]) {
        // java newProperty.remove(types[1]);
        newProperty.remove(types[1]);
        // java newProperty.remove(types[2]);
        newProperty.remove(types[2]);
      }
      // java else if (baseProperty.contains(types[1]))
      else if baseProperty.contains(types[1]) {
        // java newProperty.remove(types[2]);
        newProperty.remove(types[2]);
      }
    }

    // java private void justifyToggle(MutablePropertySet pFinal, PropertySet property, int id, Toggle orgToggle)
    fn justifyToggle(&mut self, pFinal: &mut MutablePropertySet, property: Option<&PropertySet>, id: i32, orgToggle: Option<&PropertyValue>) {
      // java if (null != property && property.contains(id))
      if let Some(property) = property {
        if property.contains(id) {
          // java pFinal.setValue(id, this.mergeToggle(orgToggle, (Toggle)property.getValue(id)));
          // rust: mergeToggle(org, toggle) = toggle（Java 恒等）
          if let Some(_org) = orgToggle {
            if let Some(newVal) = property.getValue(id) {
              pFinal.setValue(id, Some(newVal.clone()));
            }
          }
        }
      }
    }

    // java public void getMergedRunProperty(MutablePropertySet desPropertySet, PropertySet runProperty, int paraStyleName, boolean bEnableRevision)（core PropertyTool.java:500-562）
    pub fn getMergedRunProperty(&mut self, desPropertySet: &mut MutablePropertySet, runProperty: Option<&PropertySet>, paraStyleName: i32, bEnableRevision: bool) {
      // java PropertySet paraRunProperty = this.getParaPropertySet(paraStyleName);
      let paraRunProperty: PropertySet = self.getParaPropertySet(paraStyleName);
      // java if (runProperty != null)
      if let Some(runProperty) = runProperty {
        // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
        if let Some(dr) = self.mDefaultRunProperty.as_ref() {
          desPropertySet.addLimit(dr, 1, 191);
        }
        // java desPropertySet.remove(43);
        desPropertySet.remove(43);
        // java desPropertySet.addLimit(paraRunProperty, 1, 191);
        desPropertySet.addLimit(&paraRunProperty, 1, 191);
        // java this.justifyToggle(desPropertySet, paraRunProperty, 57, desPropertySet.getObject(57, DefaultValue.run_bidi));
        let orgToggle: Option<PropertyValue> = desPropertySet.getValue(57).cloned();
        self.justifyToggle(desPropertySet, Some(&paraRunProperty), 57, orgToggle.as_ref());
        // java int runStyleName = runProperty.getInt(2, 4095);
        let runStyleName: i32 = runProperty.getInt__int_int(2, 4095);
        // java if (runStyleName != 4095)
        if runStyleName != 4095 {
          // java PropertySet baseRunProperty = this.getRunPropertySet(runStyleName);
          let baseRunProperty: PropertySet = self.getRunPropertySet(runStyleName);
          // java desPropertySet.addLimit(baseRunProperty, 1, 191);
          desPropertySet.addLimit(&baseRunProperty, 1, 191);
          // java this.justifyToggle(desPropertySet, baseRunProperty, 57, desPropertySet.getObject(57, DefaultValue.run_bidi));
          let orgToggle: Option<PropertyValue> = desPropertySet.getValue(57).cloned();
          self.justifyToggle(desPropertySet, Some(&baseRunProperty), 57, orgToggle.as_ref());
        }
        // java desPropertySet.addLimit(runProperty, 1, 191);
        desPropertySet.addLimit(runProperty, 1, 191);
        // java this.justifyToggle(desPropertySet, runProperty, 57, desPropertySet.getObject(57, DefaultValue.run_bidi));
        let orgToggle: Option<PropertyValue> = desPropertySet.getValue(57).cloned();
        self.justifyToggle(desPropertySet, Some(runProperty), 57, orgToggle.as_ref());
        // java Revision revision = null;
        // java if (runProperty.contains(48) && bEnableRevision) { ... }
        // rust: ★wr-29 GR-D3 —— run 修订（48）非 hello；DEFERRED
        let _ = bEnableRevision;
        /* DEFERRED_NON_FUNCTION run 修订 48（Java core PropertyTool.java:529-549） */
        // java PropertySet var11 = null;
      }
      // java else
      else {
        // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
        if let Some(dr) = self.mDefaultRunProperty.as_ref() {
          desPropertySet.addLimit(dr, 1, 191);
        }
        // java desPropertySet.addLimit(paraRunProperty, 1, 191);
        desPropertySet.addLimit(&paraRunProperty, 1, 191);
        // java this.justifyToggle(desPropertySet, paraRunProperty, 57, desPropertySet.getObject(57, DefaultValue.run_bidi));
        let orgToggle: Option<PropertyValue> = desPropertySet.getValue(57).cloned();
        self.justifyToggle(desPropertySet, Some(&paraRunProperty), 57, orgToggle.as_ref());
      }
    }
}

// java final class PropertyTool implements IPropertyTool
impl PropertyTool {
    // java public  PropertyTool(Document document, int paraCacheSize, int runCacheSize)
    pub fn PropertyTool(document: &dyn Document, paraCacheSize: i32, runCacheSize: i32) -> Self {
      // java this.mDocument = document;
      // java this.mImpl = new cn.wps.moffice.writer.core.PropertyTool(this.mDocument);
      let _ = (paraCacheSize, runCacheSize);
      // java this.mImpl = new cn.wps.moffice.writer.core.PropertyTool(this.mDocument);
      let mMerge: PropMergeCore = PropMergeCore::new(document);
      // java this.mParaProp = new MutablePropertySet();
      // java this.mRunProp = new MutablePropertySet();
      // java this.mParaPropCache = new ParaPropLRUCache(paraCacheSize);
      // java this.mRunPropCache = new RunPropLRUCache(runCacheSize);
      Self {
        mMerge,
        mParaProp: MutablePropertySet::new(),
        mRunProp: MutablePropertySet::new(),
        mPropValues: Vec::new(),
        mPropIds: Vec::new(),
        mParaPropCache: HashMap::new(),
        mRunPropCache: HashMap::new(),
        mPendingProps: Vec::new(),
        mLastParaPropAddr: 0,
        mLastParaMark: -1,
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mPropIds = null;
      self.mPropIds.clear();
      // java this.mImpl.dispose(): document/styleSheet/defaults = null;
      self.mMerge.dispose();
      // java this.mImpl.dispose(): paraPropCache.clear(); runPropCache.clear();
      // java this.mParaProp.clear(); this.mParaProp = null;
      self.mParaProp.clear();
      // java this.mRunProp.clear(); this.mRunProp = null;
      self.mRunProp.clear();
      // java Arrays.fill(this.mPropValues, null); this.mPropValues = null;
      self.mPropValues.clear();
      // java this.mParaPropCache.dispose();
      self.mParaPropCache.clear();
      // java this.mRunPropCache.dispose();
      self.mRunPropCache.clear();
    }

    // java private CachedPropertySet createInternalPropertySet(MutablePropertySet prop)
    fn createInternalPropertySet(mPropIds: &mut Vec<i32>, mPropValues: &mut Vec<Option<PropertyValue>>, prop: &mut MutablePropertySet) -> PropertySet {
      // java int propSize = prop.size();
      let propSize: i32 = prop.size();
      // java int[] propIds = this.mPropIds;
      // java Object[] propValues = this.mPropValues;
      // java if (propIds == null || propIds.length < propSize)
      if (mPropIds.len() as i32) < propSize {
        // java propValues 清空；propIds/propValues = new int/Object[propSize]
        *mPropIds = vec![0; propSize as usize];
        *mPropValues = vec![None; propSize as usize];
      }
      // java prop.getAll(propIds, propValues);
      let mut ids: Vec<i32> = vec![0; propSize as usize];
      let mut vals: Vec<Option<PropertyValue>> = vec![None; propSize as usize];
      prop.getAll(&mut ids, Some(&mut vals));
      let n = propSize as usize;
      mPropIds[..n].copy_from_slice(&ids[..n]);
      mPropValues[..n].clone_from_slice(&vals[..n]);
      // java return new CachedPropertySet(propIds, propValues, propSize);
      // Java: new CachedPropertySet copies the values; mParaProp/mRunProp stay
      // mutable for the next paragraph. Freeze only the returned copy.
      let mut frozen = prop.get().clone();
      frozen._freeze();
      return frozen;
    }

    // java public PropertySet getFinalParaProp(IPapxTable.Entry paraEntry, boolean revisionFinal)
    pub fn getFinalParaProp(&mut self, paraEntry: &dyn PapxEntry, revisionFinal: bool) -> &PropertySet {
      // java return this.getFinalParaProp(paraEntry, revisionFinal, 4095);
      return self.getFinalParaProp__entry_boolean_int(paraEntry, revisionFinal, 4095);
    }

    // java public PropertySet getFinalParaProp(IPapxTable.Entry paraEntry, boolean revisionFinal, int tableStyleIdUseToPara)
    pub fn getFinalParaProp__entry_boolean_int(&mut self, paraEntry: &dyn PapxEntry, revisionFinal: bool, tableStyleIdUseToPara: i32) -> &PropertySet {
      // java long entry = CachedPropertySet.getParaPropCacheID(paraEntry, tableStyleIdUseToPara, revisionFinal);
      let entry: i64 = CachedPropertySet::getParaPropCacheID(paraEntry, tableStyleIdUseToPara, revisionFinal);
      // java PropertySet res = this.mParaPropCache.get(entry);
      // java if (res != null)
      if self.mParaPropCache.contains_key(&entry) {
        // java return res;
        return &self.mParaPropCache[&entry].property;
      }
      // java else
      else {
        // java this.mParaProp.clear();
        self.mParaProp.clear();
        // java this.mImpl.getMergedParaProperty(this.mParaProp, paraEntry.getProperty(), true, revisionFinal, true, tableStyleIdUseToPara);
        let paraProp: Option<&PropertySet> = PapxEntry::getProperty(paraEntry);
        self.mMerge.getMergedParaProperty(&mut self.mParaProp, paraProp, true, revisionFinal, true, tableStyleIdUseToPara);
        // java CachedPropertySet paraProp = this.createInternalPropertySet(this.mParaProp);
        let property: PropertySet = Self::createInternalPropertySet(&mut self.mPropIds, &mut self.mPropValues, &mut self.mParaProp);
        // java paraProp.setParaMark(paraEntry, tableStyleIdUseToPara, revisionFinal);
        let mut paraCached: CachedPropertySet = CachedPropertySet::CachedPropertySet(property);
        paraCached.setParaMark(paraEntry, tableStyleIdUseToPara, revisionFinal);
        // rust: 记录 last-seen（Java instanceof CachedPropertySet 等价）
        let addr: usize = &paraCached.property as *const PropertySet as usize;
        self.mLastParaPropAddr = addr;
        self.mLastParaMark = paraCached.get_para_mark();
        // java if (enableCache) { this.mParaPropCache.put(entry, paraProp); }
        self.mParaPropCache.insert(entry, Box::new(paraCached));
        // java return paraProp;
        return &self.mParaPropCache[&entry].property;
      }
    }

    // java public PropertySet getFinalRunPropert(IChpxTable.Entry runEntry, PropertySet finalParaProp, boolean revisionFinal)
    pub fn getFinalRunPropert(&mut self, runEntry: &dyn ChpxEntry, finalParaProp: &PropertySet, revisionFinal: bool) -> &PropertySet {
      // java PropertySet res = null;
      // java long entry = CachedPropertySet.getRunPropCacheID(finalParaProp, runEntry, revisionFinal);
      let entry: i64 = if finalParaProp as *const PropertySet as usize == self.mLastParaPropAddr {
        CachedPropertySet::getRunPropCacheID__property_entry_boolean(self.mLastParaMark, runEntry, revisionFinal)
      } else {
        // java if (!(finalParaProp instanceof CachedPropertySet)) { return -1L; }
        -1
      };
      // java if (-1L != entry)
      if -1 != entry {
        // java res = this.mRunPropCache.get(entry);
        // java if (null != res) { return res; }
        if self.mRunPropCache.contains_key(&entry) {
          return &self.mRunPropCache[&entry].property;
        }
      }
      // java this.mRunProp.clear();
      self.mRunProp.clear();
      // java this.mImpl.getMergedRunProperty(this.mRunProp, runEntry.getProperty(), finalParaProp.getInt(191, 4095), revisionFinal);
      let runProp: Option<&PropertySet> = ChpxEntry::getProperty(runEntry);
      let paraStyleName: i32 = finalParaProp.getInt__int_int(191, 4095);
      self.mMerge.getMergedRunProperty(&mut self.mRunProp, runProp, paraStyleName, revisionFinal);
      // java if (finalParaProp instanceof CachedPropertySet)
      if finalParaProp as *const PropertySet as usize == self.mLastParaPropAddr {
        // java CachedPropertySet runProp = this.createInternalPropertySet(this.mRunProp);
        let property: PropertySet = Self::createInternalPropertySet(&mut self.mPropIds, &mut self.mPropValues, &mut self.mRunProp);
        // java runProp.setRunMark((CachedPropertySet)finalParaProp, runEntry, revisionFinal);
        let mut runCached: CachedPropertySet = CachedPropertySet::CachedPropertySet(property);
        runCached.setRunMark_with_para_mark(self.mLastParaMark, runEntry, revisionFinal);
        // java if (enableCache && -1L != entry) { this.mRunPropCache.put(entry, runProp); }
        if -1 != entry {
          self.mRunPropCache.insert(entry, Box::new(runCached));
          // java res = runProp;
          return &self.mRunPropCache[&entry].property;
        }
        // java res = runProp; （entry == -1 不缓存，直接返回新建对象）
        self.mPendingProps.push(Box::new(runCached.property));
        return &**self.mPendingProps.last().unwrap();
      }
      // java else
      else {
        // java try { res = this.mRunProp.clone().freeze(); } catch ...
        let mut cloned: MutablePropertySet = self.mRunProp.clone_mps();
        let frozen: PropertySet = cloned.freeze();
        self.mPendingProps.push(Box::new(frozen));
        // java return res;
        return &**self.mPendingProps.last().unwrap();
      }
    }

    // java public PropertySet getTocFinalRunPropert(IChpxTable.Entry runEntry, PropertySet finalParaProp, boolean revisionFinal)
    pub fn getTocFinalRunPropert(&mut self, runEntry: &dyn ChpxEntry, finalParaProp: &PropertySet, revisionFinal: bool) -> &PropertySet {
      // java PropertySet res = null;
      // java this.mRunProp.clear();
      self.mRunProp.clear();
      // java this.mRunProp.add(runEntry.getProperty());
      if let Some(runProp) = ChpxEntry::getProperty(runEntry) {
        self.mRunProp.add(runProp);
      }
      // java this.mRunProp.remove(2);
      self.mRunProp.remove(2);
      // java this.mImpl.getMergedRunProperty(this.mRunProp, this.mRunProp.clone().freeze(), finalParaProp.getInt(191, 4095), revisionFinal);
      let mut clone: MutablePropertySet = self.mRunProp.clone_mps();
      let frozen: PropertySet = clone.freeze();
      let paraStyleName: i32 = finalParaProp.getInt__int_int(191, 4095);
      self.mMerge.getMergedRunProperty(&mut self.mRunProp, Some(&frozen), paraStyleName, revisionFinal);
      // java res = this.mRunProp.clone().freeze();
      let mut cloned: MutablePropertySet = self.mRunProp.clone_mps();
      let frozen: PropertySet = cloned.freeze();
      self.mPendingProps.push(Box::new(frozen));
      // java return res;
      return &**self.mPendingProps.last().unwrap();
    }

    // java public void getFinalRunPropert(MutablePropertySet finalRunProp, PropertySet property, PropertySet finalParaProp, boolean revisionFinal)
    pub fn getFinalRunPropert_mps(&mut self, finalRunProp: &mut MutablePropertySet, property: &PropertySet, finalParaProp: &PropertySet, revisionFinal: bool) {
      // java this.mRunProp.clear();
      self.mRunProp.clear();
      // java this.mImpl.getMergedRunProperty(this.mRunProp, property, finalParaProp.getInt(191, 4095), revisionFinal);
      let paraStyleName: i32 = finalParaProp.getInt__int_int(191, 4095);
      self.mMerge.getMergedRunProperty(&mut self.mRunProp, Some(property), paraStyleName, revisionFinal);
      // java finalRunProp.add(this.mRunProp);
      finalRunProp.add_mutable(&self.mRunProp);
    }

    // java public void clearMergedStylePropCache(FontStyleCache fontStyleCache)
    pub fn clearMergedStylePropCache(&mut self, fontStyleCache: Option<()>) {
      // java RunPropLRUCache runCache = this.mRunPropCache; ... if (prop.isMergedFromStyle()) { it1.remove(); ... }
      self.mRunPropCache.retain(|_k, v| !v.isMergedFromStyle());
      // java ParaPropLRUCache paraCache = this.mParaPropCache; ... if (prop.isMergedFromStyle()) { it2.remove(); ... }
      self.mParaPropCache.retain(|_k, v| !v.isMergedFromStyle());
      let _ = fontStyleCache;
    }
}

// rust: CachedPropertySet mark 访问辅助（env PropertyTool 内部）
impl CachedPropertySet {
    pub(crate) fn get_para_mark(&self) -> i64 { self.mParaMark }
    pub(crate) fn get_run_mark(&self) -> i64 { self.mRunMark }
}

// rust: setRunMark 的 paraMark 注入版本（env PropertyTool 保留 last paraMark，见 getFinalRunPropert）
impl CachedPropertySet {
    pub(crate) fn setRunMark_with_para_mark(&mut self, paraMark: i64, run: &dyn ChpxEntry, revFinal: bool) {
      // java public void setRunMark(CachedPropertySet paraProp, IChpxTable.Entry run, boolean revFinal)
      // java long paraMark = paraProp.mParaMark;
      // java if (revFinal != getFlagBit(paraMark, 4611686018427387904L))
      if revFinal != getFlagBit(paraMark, LONG_BIT_REVISION_FINAL) {
        // java this.mParaMark = -1L;
        self.mParaMark = -1;
        // java this.mRunMark = -1L;
        self.mRunMark = -1;
      }
      // java else
      else {
        // java this.mRunMark = getMark(run.getProperty(), run.getPropertySetId(), 4095, revFinal);
        let runProp: Option<&PropertySet> = ChpxEntry::getProperty(run);
        self.mRunMark = CachedPropertySet::getMark(runProp, run.getPropertySetId(), 4095, revFinal);
        // java this.mParaMark = paraMark;
        self.mParaMark = paraMark;
      }
    }
}

// java final class PropertyTool implements IPropertyTool
impl IPropertyTool for PropertyTool {
    // java  PropertySet getFinalParaProp(IPapxTable.Entry var1, boolean var2)
    fn getFinalParaProp__entry_boolean(&mut self, var1: &dyn PapxEntry, var2: bool) -> &PropertySet {
        self.getFinalParaProp(var1, var2)
    }

    // java  PropertySet getFinalParaProp(IPapxTable.Entry var1, boolean var2, int var3)
    fn getFinalParaProp__entry_boolean_int(&mut self, var1: &dyn PapxEntry, var2: bool, var3: i32) -> &PropertySet {
        self.getFinalParaProp__entry_boolean_int(var1, var2, var3)
    }

    // java  PropertySet getFinalRunPropert(IChpxTable.Entry var1, PropertySet var2, boolean var3)
    fn getFinalRunPropert__entry_propertyset_boolean(&mut self, var1: &dyn ChpxEntry, var2: &PropertySet, var3: bool) -> &PropertySet {
        self.getFinalRunPropert(var1, var2, var3)
    }

    // java  void getFinalRunPropert(MutablePropertySet var1, PropertySet var2, PropertySet var3, boolean var4)
    fn getFinalRunPropert__mutablepropertyset_propertyset_propertyset_boolean(&mut self, var1: &mut MutablePropertySet, var2: &PropertySet, var3: &PropertySet, var4: bool) {
        self.getFinalRunPropert_mps(var1, var2, var3, var4)
    }

    // java  PropertySet getTocFinalRunPropert(IChpxTable.Entry var1, PropertySet var2, boolean var3)
    fn getTocFinalRunPropert(&mut self, var1: &dyn ChpxEntry, var2: &PropertySet, var3: bool) -> &PropertySet {
        self.getTocFinalRunPropert(var1, var2, var3)
    }
}
