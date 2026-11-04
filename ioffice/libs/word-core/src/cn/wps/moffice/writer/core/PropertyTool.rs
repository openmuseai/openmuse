// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.DefaultParagraphProperties
use crate::cn::wps::moffice::writer::data::DefaultParagraphProperties::*;
// Java: import cn.wps.moffice.writer.data.DefaultRunProperties
use crate::cn::wps::moffice::writer::data::DefaultRunProperties::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Disambiguate nested Entry / Toggle / DefaultValue globs
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
use crate::cn::wps::moffice::writer::data::DefaultValue; // module path for DefaultValue::run_*
use crate::cn::wps::moffice::writer::data::value::Toggle::Toggle;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoDataTable
use crate::cn::wps::moffice::writer::data::autonum::LfoDataTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoLvlData
use crate::cn::wps::moffice::writer::data::autonum::LfoLvlData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstDataTable
use crate::cn::wps::moffice::writer::data::autonum::LstDataTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.Toggle
use crate::cn::wps::moffice::writer::data::value::Toggle::*;
// Java: import cn.wps.moffice.writer.data.value.TxTabStops
use crate::cn::wps::moffice::writer::data::value::TxTabStops::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java static MutablePropertySet paraPropCache = new MutablePropertySet();
pub(crate) static paraPropCache: std::sync::LazyLock<std::sync::Arc<Option<MutablePropertySet>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(MutablePropertySet::new()));
// java static MutablePropertySet runPropCache = new MutablePropertySet();
pub(crate) static runPropCache: std::sync::LazyLock<std::sync::Arc<Option<MutablePropertySet>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(MutablePropertySet::new()));
// java static final int[] spaceBeforeTypes = new int[]{217, 218, 219};
pub(crate) static spaceBeforeTypes: std::sync::LazyLock<Option<Vec<i32>>> =
                    std::sync::LazyLock::new(|| &[217, 218, 219]);
// java static final int[] spaceAfterTypes = new int[]{214, 215, 216};
pub(crate) static spaceAfterTypes: std::sync::LazyLock<Option<Vec<i32>>> =
                    std::sync::LazyLock::new(|| &[214, 215, 216]);

// java public class PropertyTool
#[derive(Default)]
pub struct PropertyTool {
    // java Document document;
    // rust: ★Java `Document` 是**接口** ⇒ Rust trait；字段须为 trait 对象（E0782）。
    pub(crate) document: Option<Box<dyn Document>>,
    // java KStyles styleSheet = null;
    pub(crate) styleSheet: Option<KStyles>,
    // java PropertySet defaultPapProperty = null;
    pub(crate) defaultPapProperty: Option<PropertySet>,
    // java PropertySet defaultRunProperty = null;
    pub(crate) defaultRunProperty: Option<PropertySet>,
}

// java public class PropertyTool
impl PropertyTool {
    // java public  PropertyTool(Document doc)
    pub fn PropertyTool(doc: &dyn Document) -> Self {
      // java this.document = doc;
      Self { document: doc, styleSheet: self.document.getStyles(), defaultPapProperty: PropertySet::compose(&DefaultParagraphProperties::getProperties(), &self.styleSheet.getPPrDefault(), 2), defaultRunProperty: PropertySet::compose(&DefaultRunProperties::getProperties(), &self.styleSheet.getRPrDefault(), 2) }
      // java this.styleSheet = this.document.getStyles();
      // java this.defaultPapProperty = PropertySet.compose(DefaultParagraphProperties.getProperties(), this.styleSheet.getPPrDefault(), 2);
      // java this.defaultRunProperty = PropertySet.compose(DefaultRunProperties.getProperties(), this.styleSheet.getRPrDefault(), 2);
    }

    // java public static void clearCache()
    pub fn clearCache() {
        // java (paraPropCache)
        paraPropCache;
          // java paraPropCache.clear();
          paraPropCache.clear();
        // java (runPropCache)
        runPropCache;
          // java runPropCache.clear();
          runPropCache.clear();
    }

    // java private PropertySet getParaPropertySet(int id)
    pub fn getParaPropertySet(&self, id: i32) -> Option<PropertySet> {
      // java PropertySet para = null;
      let mut para: PropertySet = PropertySet::default();
      // java if (id >= 0 && id <= 267)
      if id >= 0 && id <= 267 {
          // java (paraPropCache)
          paraPropCache;
            // java para = (PropertySet)paraPropCache.getValue(id);
            para = (paraPropCache.getValue(id)).downcast_ref::<PropertySet>().unwrap();
            // java if (para == null)
            if para.is_none() {
              // java KStyle baseStyle = this.styleSheet.getStyleById(id);
              let mut baseStyle: KStyle = self.styleSheet.getStyleById__int(id);
              // java if (baseStyle != null && baseStyle.getType() == 1)
              if baseStyle.is_some() && baseStyle.getType() == 1 {
                // java para = baseStyle.getFinalProperty();
                para = baseStyle.getFinalProperty();
                // java paraPropCache.setValue(id, para);
                paraPropCache.setValue(id, &para);
              }
            }
      }
      // java else
      else {
        // java KStyle baseStyle = this.styleSheet.getStyleById(id);
        let mut baseStyle: KStyle = self.styleSheet.getStyleById__int(id);
        // java if (baseStyle != null && baseStyle.getType() == 1)
        if baseStyle.is_some() && baseStyle.getType() == 1 {
          // java para = baseStyle.getFinalProperty();
          para = baseStyle.getFinalProperty();
        }
      }
      // java return para == null ? PropertySet.empty : para;
      return (if para.is_none() { Some(PropertySet::empty()) } else { Some(para) });
    }

    // java public PropertySet getRunPropertySet(int id)
    pub fn getRunPropertySet(&self, id: i32) -> PropertySet {
      // java PropertySet run = null;
      let mut run: PropertySet = PropertySet::default();
      // java if (id >= 0 && id <= 267)
      if id >= 0 && id <= 267 {
          // java (runPropCache)
          runPropCache;
            // java run = (PropertySet)runPropCache.getValue(id);
            run = (runPropCache.getValue(id)).downcast_ref::<PropertySet>().unwrap();
            // java if (run == null || run.isEmpty())
            if run.is_none() || run.is_empty() {
              // java KStyle baseStyle = this.styleSheet.getStyleById(id);
              let mut baseStyle: KStyle = self.styleSheet.getStyleById__int(id);
              // java if (baseStyle != null && baseStyle.getType() == 2)
              if baseStyle.is_some() && baseStyle.getType() == 2 {
                // java run = baseStyle.getFinalProperty();
                run = baseStyle.getFinalProperty();
                // java runPropCache.setValue(id, run);
                runPropCache.setValue(id, &run);
              }
            }
      }
      // java else
      else {
        // java KStyle baseStyle = this.styleSheet.getStyleById(id);
        let mut baseStyle: KStyle = self.styleSheet.getStyleById__int(id);
        // java if (baseStyle != null && baseStyle.getType() == 2)
        if baseStyle.is_some() && baseStyle.getType() == 2 {
          // java run = baseStyle.getFinalProperty();
          run = baseStyle.getFinalProperty();
        }
      }
      // java return run;
      return run;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.document = null;
      self.document = None;
      // java this.styleSheet = null;
      self.styleSheet = None;
      // java this.defaultPapProperty = null;
      self.defaultPapProperty = None;
      // java this.defaultRunProperty = null;
      self.defaultRunProperty = None;
        // java (paraPropCache)
        paraPropCache;
          // java paraPropCache.clear();
          paraPropCache.clear();
        // java (runPropCache)
        runPropCache;
          // java runPropCache.clear();
          runPropCache.clear();
    }

    // java public void setDocument(Document doc)
    pub fn setDocument(&mut self, doc: Option<&dyn Document>) {
      // java if (null != doc)
      if let Some(doc) = doc {
        // java this.document = doc;
        self.document = doc;
        // java this.styleSheet = this.document.getStyles();
        self.styleSheet = self.document.getStyles();
          // java (paraPropCache)
          paraPropCache;
            // java paraPropCache.clear();
            paraPropCache.clear();
          // java (runPropCache)
          runPropCache;
            // java runPropCache.clear();
            runPropCache.clear();
      }
    }

    // java public PropertySet getMergedParaProperty(int cp)
    pub fn getMergedParaProperty__int(&self, cp: i32) -> PropertySet {
      // java return this.getMergedParaProperty(cp, false);
      return self.getMergedParaProperty__int_boolean(cp, false);
    }

    // java public PropertySet getMergedParaProperty(int cp, boolean bRevision)
    pub fn getMergedParaProperty__int_boolean(&self, cp: i32, bRevision: bool) -> PropertySet {
      // java return this.getMergedParaProperty(cp, bRevision, false);
      return self.getMergedParaProperty__int_boolean_boolean(cp, bRevision, false);
    }

    // java public PropertySet getMergedParaProperty(int cp, boolean bRevision, boolean mergedNumbering)
    pub fn getMergedParaProperty__int_boolean_boolean(&self, cp: i32, bRevision: bool, mergedNumbering: bool) -> PropertySet {
      // java MutablePropertySet propertySet = new MutablePropertySet();
      let mut propertySet: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedParaProperty(propertySet, cp, bRevision, mergedNumbering);
      self.getMergedParaProperty__mutablepropertyset_int_boolean_boolean(&propertySet, cp, bRevision, mergedNumbering);
      // java return propertySet.freeze();
      return propertySet.freeze();
    }

    // java public void getMergedParaProperty(MutablePropertySet propertySet, int cp, boolean bRevision, boolean mergedNumbering)
    pub fn getMergedParaProperty__mutablepropertyset_int_boolean_boolean(&self, propertySet: &MutablePropertySet, cp: i32, bRevision: bool, mergedNumbering: bool) {
      // java IPapxTable.Entry paragraph = this.document.getPapxTable().seek(cp);
      let mut paragraph: Box<dyn PapxEntry> = self.document.getPapxTable().seek(cp);
      // java this.getMergedParaProperty(propertySet, paragraph, bRevision, mergedNumbering);
      self.getMergedParaProperty__mutablepropertyset_int_boolean_boolean(&propertySet, paragraph, bRevision, mergedNumbering);
    }

    // java public PropertySet getMergedParaProperty(IPapxTable.Entry paraBreak)
    pub fn getMergedParaProperty__entry(&self, paraBreak: &dyn PapxEntry) -> PropertySet {
      // java return this.getMergedParaProperty(paraBreak, false);
      return self.getMergedParaProperty__int_boolean(paraBreak, false);
    }

    // java public PropertySet getMergedParaProperty(IPapxTable.Entry paraBreak, boolean bMergedNumbering)
    pub fn getMergedParaProperty__entry_boolean(&self, paraBreak: &dyn PapxEntry, bMergedNumbering: bool) -> PropertySet {
      // java return this.getMergedParaProperty(paraBreak, bMergedNumbering, false);
      return self.getMergedParaProperty__int_boolean_boolean(paraBreak, bMergedNumbering, false);
    }

    // java public PropertySet getMergedParaProperty(IPapxTable.Entry paraBreak, boolean bMergedNumbering, boolean bRevision)
    pub fn getMergedParaProperty__entry_boolean_boolean(&self, paraBreak: &dyn PapxEntry, bMergedNumbering: bool, bRevision: bool) -> PropertySet {
      // java MutablePropertySet paraProperty = new MutablePropertySet();
      let mut paraProperty: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedParaProperty(paraProperty, paraBreak, bMergedNumbering, bRevision);
      self.getMergedParaProperty__mutablepropertyset_int_boolean_boolean(&paraProperty, paraBreak, bMergedNumbering, bRevision);
      // java return paraProperty.freeze();
      return paraProperty.freeze();
    }

    // java public void getMergedParaProperty(MutablePropertySet propertySet, IPapxTable.Entry paragraph)
    pub fn getMergedParaProperty__mutablepropertyset_entry(&self, propertySet: &MutablePropertySet, paragraph: &dyn PapxEntry) {
      // java this.getMergedParaProperty(propertySet, paragraph, false);
      self.getMergedParaProperty__int_boolean_boolean(propertySet, paragraph, false);
    }

    // java public void getMergedParaProperty(MutablePropertySet propertySet, IPapxTable.Entry paragraph, boolean bRevision)
    pub fn getMergedParaProperty__mutablepropertyset_entry_boolean(&self, propertySet: &MutablePropertySet, paragraph: &dyn PapxEntry, bRevision: bool) {
      // java this.getMergedParaProperty(propertySet, paragraph, false, bRevision);
      self.getMergedParaProperty__mutablepropertyset_int_boolean_boolean(&propertySet, paragraph, false, bRevision);
    }

    // java public void getMergedParaProperty(MutablePropertySet propertySet, IPapxTable.Entry paragraph, boolean bMergedNumbering, boolean bRevision)
    pub fn getMergedParaProperty__mutablepropertyset_entry_boolean_boolean(&self, propertySet: &MutablePropertySet, paragraph: Option<&dyn PapxEntry>, bMergedNumbering: bool, bRevision: bool) {
      // java if (paragraph == null)
      if paragraph.is_none() {
        // java this.getDefaultParaProperty(propertySet);
        self.getDefaultParaProperty__mutablepropertyset(&mut propertySet);
      }
      // java else
      else {
        // java this.getMergedParaProperty(propertySet, paragraph.getProperty(), bMergedNumbering, bRevision);
        self.getMergedParaProperty__mutablepropertyset_int_boolean_boolean(&propertySet, paragraph.getProperty(), bMergedNumbering, bRevision);
      }
    }

    // java public PropertySet getMergedRunProperty(int cp)
    pub fn getMergedRunProperty__int(&self, cp: i32) -> PropertySet {
      // java MutablePropertySet runProperty = new MutablePropertySet();
      let mut runProperty: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedRunProperty(runProperty, cp);
      self.getMergedRunProperty__mutablepropertyset_int(&runProperty, cp);
      // java return runProperty.freeze();
      return runProperty.freeze();
    }

    // java public void getMergedRunProperty(MutablePropertySet propertySet, int cp)
    pub fn getMergedRunProperty__mutablepropertyset_int(&self, propertySet: &MutablePropertySet, cp: i32) {
      // java this.getMergedRunProperty(propertySet, this.document.getChpxTable().seek(cp));
      self.getMergedRunProperty__mutablepropertyset_int(&propertySet, self.document.getChpxTable().seek(cp));
    }

    // java public PropertySet getMergedRunPropertyNoDefault(IChpxTable.Entry pFrag)
    pub fn getMergedRunPropertyNoDefault(&mut self, pFrag: Option<&mut dyn ChpxEntry>) -> PropertySet {
      // java MutablePropertySet desPropertySet = new MutablePropertySet();
      let mut desPropertySet: MutablePropertySet = MutablePropertySet::new();
      // java if (pFrag != null && !pFrag.isEnd())
      if let Some(pFrag) = pFrag {
        if !pFrag.isEnd() {
          // java IPapxTable.Entry paragraph = this.document.getPapxTable().seek(pFrag.position());
          let mut paragraph: Box<dyn PapxEntry> = self.document.getPapxTable().seek(pFrag.position());
          // java PropertySet runProperty = pFrag.getProperty();
          let mut runProperty: PropertySet = pFrag.getProperty();
          // java PropertySet paraRunProperty = this.getParaPropertySet(PropertyUtil.getFinalNormalParageraphFormat(paragraph.getProperty()).getInt(191, 4095));
          let mut paraRunProperty: PropertySet = self.getParaPropertySet(PropertyUtil::getFinalNormalParageraphFormat(&paragraph.getProperty()).getInt(191, 4095));
          // java if (runProperty != null)
          if runProperty.is_some() {
            // java int runStyleName = runProperty.getInt(2, 4095);
            let mut runStyleName: i32 = runProperty.getInt(2, 4095);
            // java if (runStyleName != 4095)
            if runStyleName != 4095 {
              // java PropertySet baseRunProperty = this.getRunPropertySet(runStyleName);
              let mut baseRunProperty: PropertySet = self.getRunPropertySet(runStyleName);
              // java Toggle orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
              let mut orgToggle: Toggle = (desPropertySet.getObject(57, DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
              // java desPropertySet.addLimit(paraRunProperty, 1, 191);
              desPropertySet.addLimit(paraRunProperty, 1, 191);
              // java this.justifyToggle(desPropertySet, paraRunProperty, 57, orgToggle);
              self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &orgToggle);
              // java orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
              orgToggle = (desPropertySet.getObject(57, DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
              // java desPropertySet.addLimit(baseRunProperty, 1, 191);
              desPropertySet.addLimit(baseRunProperty, 1, 191);
              // java this.justifyToggle(desPropertySet, baseRunProperty, 57, orgToggle);
              self.justifyToggle(&mut desPropertySet, Some(&baseRunProperty), 57, &orgToggle);
              // java orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
              orgToggle = (desPropertySet.getObject(57, DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
              // java desPropertySet.addLimit(runProperty, 1, 191);
              desPropertySet.addLimit(runProperty, 1, 191);
              // java this.justifyToggle(desPropertySet, runProperty, 57, orgToggle);
              self.justifyToggle(&mut desPropertySet, Some(&runProperty), 57, &orgToggle);
            }
            // java else
            else {
              // java Toggle orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
              let mut orgToggle: Toggle = (desPropertySet.getObject(57, DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
              // java desPropertySet.addLimit(paraRunProperty, 1, 191);
              desPropertySet.addLimit(paraRunProperty, 1, 191);
              // java this.justifyToggle(desPropertySet, paraRunProperty, 57, orgToggle);
              self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &orgToggle);
              // java orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
              orgToggle = (desPropertySet.getObject(57, DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
              // java desPropertySet.addLimit(runProperty, 1, 191);
              desPropertySet.addLimit(runProperty, 1, 191);
              // java this.justifyToggle(desPropertySet, runProperty, 57, orgToggle);
              self.justifyToggle(&mut desPropertySet, Some(&runProperty), 57, &orgToggle);
            }
          }
          // java else
          else {
            // java Toggle orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
            let mut orgToggle: Toggle = (desPropertySet.getObject(57, DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
            // java desPropertySet.addLimit(paraRunProperty, 1, 191);
            desPropertySet.addLimit(paraRunProperty, 1, 191);
            // java this.justifyToggle(desPropertySet, paraRunProperty, 57, orgToggle);
            self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &orgToggle);
          }
        }
      }
      // java else
      else {
        // java this.getDefaultRunProperty(desPropertySet);
        self.getDefaultRunProperty__mutablepropertyset(&mut desPropertySet);
      }
      // java return desPropertySet.freeze();
      return desPropertySet.freeze();
    }

    // java public PropertySet getMergedRunProperty(IChpxTable.Entry pFrag)
    pub fn getMergedRunProperty__entry(&self, pFrag: &dyn ChpxEntry) -> PropertySet {
      // java MutablePropertySet runProperty = new MutablePropertySet();
      let mut runProperty: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedRunProperty(runProperty, pFrag);
      self.getMergedRunProperty__mutablepropertyset_int(&runProperty, pFrag);
      // java return runProperty.freeze();
      return runProperty.freeze();
    }

    // java public PropertySet getMergedRunProperty(IChpxTable.Entry pFrag, boolean bRevision)
    pub fn getMergedRunProperty__entry_boolean(&self, pFrag: &dyn ChpxEntry, bRevision: bool) -> PropertySet {
      // java MutablePropertySet runProperty = new MutablePropertySet();
      let mut runProperty: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedRunProperty(runProperty, pFrag, bRevision);
      self.getMergedRunProperty__mutablepropertyset_entry_boolean(&runProperty, Some(&mut pFrag), bRevision);
      // java return runProperty.freeze();
      return runProperty.freeze();
    }

    // java public void getMergedRunProperty(MutablePropertySet propertySet, IChpxTable.Entry pFrag)
    pub fn getMergedRunProperty__mutablepropertyset_entry(&self, propertySet: &MutablePropertySet, pFrag: Option<&mut dyn ChpxEntry>) {
      // java if (pFrag != null && !pFrag.isEnd())
      if let Some(pFrag) = pFrag {
        if !pFrag.isEnd() {
          // java IPapxTable.Entry paragraph = this.document.getPapxTable().seek(pFrag.position());
          let mut paragraph: Box<dyn PapxEntry> = self.document.getPapxTable().seek(pFrag.position());
          // java this.getMergedRunProperty(propertySet, pFrag.getProperty(), paragraph.getProperty().getInt(191, 4095));
          self.getMergedRunProperty__mutablepropertyset_entry_boolean(&propertySet, Some(&mut pFrag.getProperty()), paragraph.getProperty().getInt(191, 4095));
        }
      }
      // java else
      else {
        // java this.getDefaultRunProperty(propertySet);
        self.getDefaultRunProperty__mutablepropertyset(&mut propertySet);
      }
    }

    // java public void getMergedRunProperty(MutablePropertySet propertySet, IChpxTable.Entry pFrag, boolean bRevision)
    pub fn getMergedRunProperty__mutablepropertyset_entry_boolean(&self, propertySet: &MutablePropertySet, pFrag: Option<&mut dyn ChpxEntry>, bRevision: bool) {
      // java if (pFrag != null && !pFrag.isEnd())
      if let Some(pFrag) = pFrag {
        if !pFrag.isEnd() {
          // java int paraStyleName = 4095;
          let mut paraStyleName: i32 = 4095;
          // java IPapxTable.Entry paragraph = this.document.getPapxTable().seek(pFrag.position());
          let mut paragraph: Box<dyn PapxEntry> = self.document.getPapxTable().seek(pFrag.position());
          // java paraStyleName = paragraph.getProperty().getInt(191, 4095);
          paraStyleName = paragraph.getProperty().getInt(191, 4095);
          // java if (paragraph.getProperty().contains(237) && bRevision)
          if paragraph.getProperty().contains(237) && bRevision {
            // java Revision revision = (Revision)paragraph.getProperty().getValue(237);
            let mut revision: Revision = (paragraph.getProperty().getValue(237)).downcast_ref::<Revision>().unwrap();
            // java PropertySet prop = revision.getProperty();
            let mut prop: PropertySet = revision.getProperty();
            // java if (prop != null && prop.contains(191))
            if prop.is_some() && prop.contains(191) {
              // java paraStyleName = prop.getInt(191);
              paraStyleName = prop.getInt(191);
            }
          }
          // java this.getMergedRunProperty(propertySet, pFrag.getProperty(), paraStyleName, bRevision);
          self.getMergedRunProperty__mutablepropertyset_propertyset_int_boolean(&mut propertySet, Some(&pFrag.getProperty()), paraStyleName, bRevision);
        }
      }
      // java else
      else {
        // java this.getDefaultRunProperty(propertySet);
        self.getDefaultRunProperty__mutablepropertyset(&mut propertySet);
      }
    }

    // java public PropertySet getDefaultParaProperty()
    pub fn getDefaultParaProperty(&self) -> PropertySet {
      // java return this.defaultPapProperty;
      return self.defaultPapProperty;
    }

    // java public void getDefaultParaProperty(MutablePropertySet propertySet)
    pub fn getDefaultParaProperty__mutablepropertyset(&self, propertySet: &mut MutablePropertySet) {
      // java propertySet.add(this.defaultPapProperty);
      propertySet.add__mutablepropertyset(Some(&self.defaultPapProperty));
    }

    // java public PropertySet getDefaultRunProperty()
    pub fn getDefaultRunProperty(&self) -> PropertySet {
      // java return this.defaultRunProperty;
      return self.defaultRunProperty;
    }

    // java public void getDefaultRunProperty(MutablePropertySet propertySet)
    pub fn getDefaultRunProperty__mutablepropertyset(&self, propertySet: &mut MutablePropertySet) {
      // java propertySet.add(this.defaultRunProperty);
      propertySet.add__mutablepropertyset(Some(&self.defaultRunProperty));
    }

    // java public PropertySet getMergedStyleProperty(KStyle style)
    pub fn getMergedStyleProperty__kstyle(&self, style: &KStyle) -> PropertySet {
      // java MutablePropertySet finalProp = new MutablePropertySet();
      let mut finalProp: MutablePropertySet = MutablePropertySet::new();
      // java this.getMergedStyleProperty(finalProp, style);
      self.getMergedStyleProperty__mutablepropertyset_kstyle(&mut finalProp, &style);
      // java return finalProp.freeze();
      return finalProp.freeze();
    }

    // java public void getMergedStyleProperty(MutablePropertySet propertySet, KStyle style)
    pub fn getMergedStyleProperty__mutablepropertyset_kstyle(&mut self, propertySet: &mut MutablePropertySet, style: &KStyle) {
      // java if (style.getType() == 2)
      if style.getType() == 2 {
        // java propertySet.add(this.getDefaultRunProperty());
        propertySet.add__mutablepropertyset(Some(&self.getDefaultRunProperty()));
      }
      // java else if (style.getType() == 1)
      else if style.getType() == 1 {
        // java propertySet.add(this.getDefaultRunProperty());
        propertySet.add__mutablepropertyset(Some(&self.getDefaultRunProperty()));
        // java propertySet.add(this.getDefaultParaProperty());
        propertySet.add__mutablepropertyset(Some(&self.getDefaultParaProperty()));
      }
      // java this.justifyParagraphMargin(spaceBeforeTypes, propertySet, style.getFinalProperty());
      self.justifyParagraphMargin(&spaceBeforeTypes, &mut propertySet, &style.getFinalProperty());
      // java this.justifyParagraphMargin(spaceAfterTypes, propertySet, style.getFinalProperty());
      self.justifyParagraphMargin(&spaceAfterTypes, &mut propertySet, &style.getFinalProperty());
      // java propertySet.add(style.getFinalProperty());
      propertySet.add__mutablepropertyset(Some(&style.getFinalProperty()));
    }

    // java public void getMergedParaProperty(MutablePropertySet desPropertySet, PropertySet paraProperty)
    pub fn getMergedParaProperty__mutablepropertyset_propertyset(&mut self, desPropertySet: &mut MutablePropertySet, paraProperty: Option<&PropertySet>) {
      // java if (paraProperty != null)
      if let Some(paraProperty) = paraProperty {
        // java int styleName = paraProperty.getInt(191, 0);
        let mut styleName: i32 = paraProperty.getInt__int_int(191, 0);
        // java PropertySet baseParaProp = this.getParaPropertySet(styleName);
        let mut baseParaProp: PropertySet = self.getParaPropertySet(styleName);
        // java desPropertySet.addLimit(this.getDefaultParaProperty(), 191, 376);
        desPropertySet.addLimit(Some(&self.getDefaultParaProperty()), 191, 376);
        // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, baseParaProp);
        self.justifyParagraphMargin(&spaceBeforeTypes, &mut desPropertySet, &baseParaProp);
        // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, baseParaProp);
        self.justifyParagraphMargin(&spaceAfterTypes, &mut desPropertySet, &baseParaProp);
        // java desPropertySet.addLimit(baseParaProp, 191, 376);
        desPropertySet.addLimit(Some(&baseParaProp), 191, 376);
        // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, paraProperty);
        self.justifyParagraphMargin(&spaceBeforeTypes, &mut desPropertySet, &paraProperty);
        // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, paraProperty);
        self.justifyParagraphMargin(&spaceAfterTypes, &mut desPropertySet, &paraProperty);
        // java TxTabStops tabStops = this.getMarginTabs(desPropertySet, paraProperty);
        let mut tabStops: TxTabStops = self.getMarginTabs(&desPropertySet, &paraProperty);
        // java desPropertySet.addLimit(paraProperty, 191, 376);
        desPropertySet.addLimit(Some(&paraProperty), 191, 376);
        // java desPropertySet.setValue(236, tabStops);
        desPropertySet.setValue(236, &tabStops);
        // java Revision revision = (Revision)paraProperty.getValue(237);
        let mut revision: Revision = (paraProperty.getValue(237)).downcast_ref::<Revision>().unwrap();
        // java PropertySet revisionProp = null == revision ? null : revision.getProperty();
        let mut revisionProp: PropertySet = (if revision.is_none() { PropertySet::default() } else { revision.getProperty() });
        // java if (null != revisionProp)
        if revisionProp.is_some() {
          // java desPropertySet.add(revisionProp);
          desPropertySet.add__mutablepropertyset(Some(&revisionProp));
        }
      }
    }

    // java public void getMergedParaProperty(MutablePropertySet desPropertySet, PropertySet paraProperty, boolean bMergedNumbering, boolean bRevision)
    pub fn getMergedParaProperty__mutablepropertyset_propertyset_boolean_boolean(&self, desPropertySet: &MutablePropertySet, paraProperty: &PropertySet, bMergedNumbering: bool, bRevision: bool) {
      // java this.getMergedParaProperty(desPropertySet, paraProperty, bMergedNumbering, bRevision, true, 4095);
      self.getMergedParaProperty__mutablepropertyset_propertyset_boolean_boolean_boolean_int(&mut desPropertySet, Some(&paraProperty), bMergedNumbering, bRevision, true, 4095);
    }

    // java public void getMergedParaProperty(MutablePropertySet desPropertySet, PropertySet paraProperty, boolean bMergedNumbering, boolean bRevision, boolean bMergeDefaultBase, int tableStyleIdUseToPara)
    pub fn getMergedParaProperty__mutablepropertyset_propertyset_boolean_boolean_boolean_int(&mut self, desPropertySet: &mut MutablePropertySet, paraProperty: Option<&PropertySet>, bMergedNumbering: bool, bRevision: bool, bMergeDefaultBase: bool, tableStyleIdUseToPara: i32) {
      // java if (paraProperty != null)
      if let Some(paraProperty) = paraProperty {
        // java boolean styleInRevision = false;
        let mut styleInRevision: bool = false;
        // java int styleName = paraProperty.getInt(191, 0);
        let mut styleName: i32 = paraProperty.getInt__int_int(191, 0);
        // java Revision revision = null;
        let mut revision: Revision = Revision::default();
        // java PropertySet revisionProp = null;
        let mut revisionProp: PropertySet = PropertySet::default();
        // java if (paraProperty.contains(237) && bRevision)
        if paraProperty.contains(237) && bRevision {
          // java revision = (Revision)paraProperty.getValue(237);
          revision = (paraProperty.getValue(237)).downcast_ref::<Revision>().unwrap();
          // java revisionProp = revision.getProperty();
          revisionProp = revision.getProperty();
          // java if (revisionProp != null && revisionProp.contains(191))
          if revisionProp.is_some() && revisionProp.contains(191) {
            // java styleName = revisionProp.getInt(191);
            styleName = revisionProp.getInt(191);
            // java styleInRevision = true;
            styleInRevision = true;
          }
        }
        // java PropertySet baseParaProp = this.getParaPropertySet(styleName);
        let mut baseParaProp: PropertySet = self.getParaPropertySet(styleName);
        // java if (bMergeDefaultBase)
        if bMergeDefaultBase {
          // java desPropertySet.addLimit(this.getDefaultParaProperty(), 191, 376);
          desPropertySet.addLimit(Some(&self.getDefaultParaProperty()), 191, 376);
          // java if (tableStyleIdUseToPara != 4095)
          if tableStyleIdUseToPara != 4095 {
            // java this.useParaFromTableStyle(tableStyleIdUseToPara, desPropertySet);
            self.useParaFromTableStyle(tableStyleIdUseToPara, &mut desPropertySet);
          }
          // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, baseParaProp);
          self.justifyParagraphMargin(&spaceBeforeTypes, &mut desPropertySet, &baseParaProp);
          // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, baseParaProp);
          self.justifyParagraphMargin(&spaceAfterTypes, &mut desPropertySet, &baseParaProp);
        }
        // java if (bMergeDefaultBase || styleName != 0)
        if bMergeDefaultBase || styleName != 0 {
          // java desPropertySet.addLimit(baseParaProp, 191, 376);
          desPropertySet.addLimit(Some(&baseParaProp), 191, 376);
          // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, paraProperty);
          self.justifyParagraphMargin(&spaceBeforeTypes, &mut desPropertySet, &paraProperty);
          // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, paraProperty);
          self.justifyParagraphMargin(&spaceAfterTypes, &mut desPropertySet, &paraProperty);
        }
        // java if (bMergedNumbering)
        if bMergedNumbering {
          // java boolean hasNumber = false;
          let mut hasNumber: bool = false;
          // java int lfo = -1;
          let mut lfo: i32 = -1;
          // java int lvl = 0;
          let mut lvl: i32 = 0;
          // java if (styleInRevision)
          if styleInRevision {
            // java hasNumber = baseParaProp.contains(235);
            hasNumber = baseParaProp.contains(235);
            // java if (hasNumber)
            if hasNumber {
              // java lfo = baseParaProp.getInt(235, -1);
              lfo = baseParaProp.getInt(235, -1);
              // java lvl = baseParaProp.getInt(234, 0);
              lvl = baseParaProp.getInt(234, 0);
            }
          }
          // java else
          else {
            // java hasNumber = baseParaProp.contains(235) || paraProperty.contains(235);
            hasNumber = baseParaProp.contains(235) || paraProperty.contains(235);
            // java if (hasNumber)
            if hasNumber {
              // java lfo = paraProperty.contains(235) ? paraProperty.getInt(235) : baseParaProp.getInt(235, -1);
              lfo = (if paraProperty.contains(235) { paraProperty.getInt__int(235) } else { baseParaProp.getInt(235, -1) });
              // java lvl = paraProperty.contains(234) ? paraProperty.getInt(234) : baseParaProp.getInt(234, 0);
              lvl = (if paraProperty.contains(234) { paraProperty.getInt__int(234) } else { baseParaProp.getInt(234, 0) });
            }
          }
          // java if (hasNumber)
          if hasNumber {
            // java LfoDataTable lfoDataTable = this.document.getTextDocument().getAutoNumTable().getLfoDataTable();
            let mut lfoDataTable: LfoDataTable = self.document.getTextDocument().getAutoNumTable().getLfoDataTable();
            // java LfoData lfoData = lfoDataTable.getDataByIlfo(lfo);
            let mut lfoData: LfoData = lfoDataTable.getDataByIlfo(lfo);
            // java LvlfData lvlfData = null;
            let mut lvlfData: LvlfData = LvlfData::default();
            // java if (lfoData == null)
            if lfoData.is_none() {
              // java desPropertySet.remove(208);
              desPropertySet.remove(208);
              // java desPropertySet.remove(209);
              desPropertySet.remove(209);
              // java desPropertySet.remove(278);
              desPropertySet.remove(278);
              // java desPropertySet.remove(211);
              desPropertySet.remove(211);
              // java desPropertySet.remove(210);
              desPropertySet.remove(210);
            }
            // java else
            else {
              // java LfoLvlData lfoLvlData = lfoData.getLfoLvlByIlvl(lvl);
              let mut lfoLvlData: LfoLvlData = lfoData.getLfoLvlByIlvl(lvl);
              // java if (lfoLvlData != null && null != lfoLvlData.getLvlfData())
              if lfoLvlData.is_some() && lfoLvlData.getLvlfData().is_some() {
                // java lvlfData = lfoLvlData.getLvlfData();
                lvlfData = lfoLvlData.getLvlfData();
              }
              // java else
              else {
                // java LstDataTable lstDataTable = this.document.getTextDocument().getAutoNumTable().getLstDataTable();
                let mut lstDataTable: LstDataTable = self.document.getTextDocument().getAutoNumTable().getLstDataTable();
                // java LstData lstData = lstDataTable.getDataByLsid(lfoData.getLsid());
                let mut lstData: LstData = lstDataTable.getDataByLsid(lfoData.getLsid());
                // java if (lstData != null)
                if lstData.is_some() {
                  // java lvlfData = lstData.getLvlfDataByIlvl(lvl);
                  lvlfData = lstData.getLvlfDataByIlvl(lvl);
                }
              }
              // java if (lvlfData != null && lvlfData.getParaProp() != null)
              if lvlfData.is_some() && lvlfData.getParaProp().is_some() {
                // java MutablePropertySet lvfParaProp = new MutablePropertySet();
                let mut lvfParaProp: MutablePropertySet = MutablePropertySet::new();
                // java this.getMergedParaProperty(lvfParaProp, lvlfData.getParaProp(), false, bRevision, false, 4095);
                self.getMergedParaProperty__mutablepropertyset_propertyset_boolean_boolean_boolean_int(&mut lvfParaProp, Some(&lvlfData.getParaProp()), false, bRevision, false, 4095);
                // java desPropertySet.addLimit(lvfParaProp.freeze(), 191, 376);
                desPropertySet.addLimit(Some(&lvfParaProp.freeze()), 191, 376);
                // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, paraProperty);
                self.justifyParagraphMargin(&spaceBeforeTypes, &mut desPropertySet, &paraProperty);
                // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, paraProperty);
                self.justifyParagraphMargin(&spaceAfterTypes, &mut desPropertySet, &paraProperty);
              }
            }
          }
        }
        // java TxTabStops tabStops = this.getMarginTabs(desPropertySet, paraProperty);
        let mut tabStops: TxTabStops = self.getMarginTabs(&desPropertySet, &paraProperty);
        // java desPropertySet.addLimit(paraProperty, 191, 376);
        desPropertySet.addLimit(Some(&paraProperty), 191, 376);
        // java desPropertySet.setValue(236, tabStops);
        desPropertySet.setValue(236, &tabStops);
        // java if (styleInRevision)
        if styleInRevision {
          // java desPropertySet.setValue(235, baseParaProp.getValue(235));
          desPropertySet.setValue(235, &baseParaProp.getValue(235));
          // java desPropertySet.setValue(234, baseParaProp.getValue(234));
          desPropertySet.setValue(234, &baseParaProp.getValue(234));
        }
        // java if (null != revision && null != revisionProp)
        if revision.is_some() && revisionProp.is_some() {
          // java this.getMergedParaProperty(desPropertySet, revisionProp, false, bMergedNumbering, false, 4095);
          self.getMergedParaProperty__mutablepropertyset_propertyset_boolean_boolean_boolean_int(&mut desPropertySet, Some(&revisionProp), false, bMergedNumbering, false, 4095);
          // java this.justifyParagraphMargin(spaceBeforeTypes, desPropertySet, revisionProp);
          self.justifyParagraphMargin(&spaceBeforeTypes, &mut desPropertySet, &revisionProp);
          // java this.justifyParagraphMargin(spaceAfterTypes, desPropertySet, revisionProp);
          self.justifyParagraphMargin(&spaceAfterTypes, &mut desPropertySet, &revisionProp);
        }
      }
    }

    // java private TxTabStops getMarginTabs(MutablePropertySet newProperty, PropertySet baseProperty)
    pub fn getMarginTabs(&self, newProperty: &MutablePropertySet, baseProperty: &PropertySet) -> Option<TxTabStops> {
      // java TxTabStops newStops = (TxTabStops)newProperty.getValue(236);
      let mut newStops: TxTabStops = (newProperty.getValue(236)).downcast_ref::<TxTabStops>().unwrap();
      // java TxTabStops baseStops = (TxTabStops)baseProperty.getValue(236);
      let mut baseStops: TxTabStops = (baseProperty.getValue(236)).downcast_ref::<TxTabStops>().unwrap();
      // java return null == newStops && null == baseStops ? null : TxTabStops.merge(newStops, baseStops);
      return (if newStops.is_none() && baseStops.is_none() { None } else { Some(TxTabStops::merge(Some(&newStops), Some(&baseStops))) });
    }

    // java private void justifyParagraphMargin(int[] types, MutablePropertySet newProperty, PropertySet baseProperty)
    pub fn justifyParagraphMargin(&mut self, types: &Vec<i32>, newProperty: &mut MutablePropertySet, baseProperty: &PropertySet) {
      // java if (baseProperty.contains(types[0]))
      if baseProperty.contains(types[0usize]) {
        // java newProperty.remove(types[1]);
        newProperty.remove(types[1usize]);
        // java newProperty.remove(types[2]);
        newProperty.remove(types[2usize]);
      }
      // java else if (baseProperty.contains(types[1]))
      else if baseProperty.contains(types[1usize]) {
        // java newProperty.remove(types[2]);
        newProperty.remove(types[2usize]);
      }
    }

    // java private void useParaFromTableStyle(int tableStyleIdUseToPara, MutablePropertySet newProperty)
    pub fn useParaFromTableStyle(&self, tableStyleIdUseToPara: i32, newProperty: &mut MutablePropertySet) {
      // java if (tableStyleIdUseToPara != 4095)
      if tableStyleIdUseToPara != 4095 {
        // java KStyle baseStyle = this.styleSheet.getStyleById(tableStyleIdUseToPara);
        let mut baseStyle: KStyle = self.styleSheet.getStyleById__int(tableStyleIdUseToPara);
        // java if (baseStyle != null && baseStyle.getType() == 3)
        if baseStyle.is_some() && baseStyle.getType() == 3 {
          // java PropertySet paraProperty = baseStyle.getFinalProperty();
          let mut paraProperty: PropertySet = baseStyle.getFinalProperty();
          // java if (paraProperty != null)
          if paraProperty.is_some() {
            // java newProperty.addLimit(paraProperty, 191, 291);
            newProperty.addLimit(Some(&paraProperty), 191, 291);
          }
        }
      }
    }

    // java private void justifyToggle(MutablePropertySet pFinal, PropertySet property, int id, Toggle orgToggle)
    pub fn justifyToggle(&mut self, pFinal: &mut MutablePropertySet, property: Option<&PropertySet>, id: i32, orgToggle: &Toggle) {
      // java if (null != property && property.contains(id))
      if let Some(property) = property {
        if property.contains(id) {
          // java pFinal.setValue(id, this.mergeToggle(orgToggle, (Toggle)property.getValue(id)));
          pFinal.setValue(id, &self.mergeToggle(&orgToggle, &(property.getValue(id)).downcast_ref::<Toggle>().unwrap()));
        }
      }
    }

    // java private Toggle mergeToggle(Toggle orgToggle, Toggle toggle)
    pub fn mergeToggle(&mut self, orgToggle: &Toggle, toggle: &Toggle) -> Toggle {
      // java return toggle;
      return toggle;
    }

    // java public void getMergedRunProperty(MutablePropertySet desPropertySet, PropertySet runProperty, int paraStyleName)
    pub fn getMergedRunProperty__mutablepropertyset_propertyset_int(&mut self, desPropertySet: &mut MutablePropertySet, runProperty: Option<&PropertySet>, paraStyleName: i32) {
      // java PropertySet paraRunProperty = this.getParaPropertySet(paraStyleName);
      let mut paraRunProperty: PropertySet = self.getParaPropertySet(paraStyleName);
      // java if (runProperty != null)
      if let Some(runProperty) = runProperty {
        // java int runStyleName = runProperty.getInt(2, 4095);
        let mut runStyleName: i32 = runProperty.getInt__int_int(2, 4095);
        // java if (runStyleName != 4095)
        if runStyleName != 4095 {
          // java PropertySet baseRunProperty = this.getRunPropertySet(runStyleName);
          let mut baseRunProperty: PropertySet = self.getRunPropertySet(runStyleName);
          // java Toggle orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
          let mut orgToggle: Toggle = (desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
          // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
          desPropertySet.addLimit(Some(&self.getDefaultRunProperty()), 1, 191);
          // java desPropertySet.addLimit(paraRunProperty, 1, 191);
          desPropertySet.addLimit(Some(&paraRunProperty), 1, 191);
          // java this.justifyToggle(desPropertySet, paraRunProperty, 57, orgToggle);
          self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &orgToggle);
          // java orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
          orgToggle = (desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
          // java desPropertySet.addLimit(baseRunProperty, 1, 191);
          desPropertySet.addLimit(Some(&baseRunProperty), 1, 191);
          // java this.justifyToggle(desPropertySet, baseRunProperty, 57, orgToggle);
          self.justifyToggle(&mut desPropertySet, Some(&baseRunProperty), 57, &orgToggle);
          // java orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
          orgToggle = (desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
          // java desPropertySet.addLimit(runProperty, 1, 191);
          desPropertySet.addLimit(Some(&runProperty), 1, 191);
          // java this.justifyToggle(desPropertySet, runProperty, 57, orgToggle);
          self.justifyToggle(&mut desPropertySet, Some(&runProperty), 57, &orgToggle);
        }
        // java else
        else {
          // java Toggle orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
          let mut orgToggle: Toggle = (desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
          // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
          desPropertySet.addLimit(Some(&self.getDefaultRunProperty()), 1, 191);
          // java desPropertySet.addLimit(paraRunProperty, 1, 191);
          desPropertySet.addLimit(Some(&paraRunProperty), 1, 191);
          // java this.justifyToggle(desPropertySet, paraRunProperty, 57, orgToggle);
          self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &orgToggle);
          // java orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
          orgToggle = (desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
          // java desPropertySet.addLimit(runProperty, 1, 191);
          desPropertySet.addLimit(Some(&runProperty), 1, 191);
          // java this.justifyToggle(desPropertySet, runProperty, 57, orgToggle);
          self.justifyToggle(&mut desPropertySet, Some(&runProperty), 57, &orgToggle);
        }
        // java PropertySet var8 = null;
        let mut var8: PropertySet = PropertySet::default();
      }
      // java else
      else {
        // java Toggle orgToggle = (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi);
        let mut orgToggle: Toggle = (desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap();
        // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
        desPropertySet.addLimit(Some(&self.getDefaultRunProperty()), 1, 191);
        // java desPropertySet.addLimit(paraRunProperty, 1, 191);
        desPropertySet.addLimit(Some(&paraRunProperty), 1, 191);
        // java this.justifyToggle(desPropertySet, paraRunProperty, 57, orgToggle);
        self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &orgToggle);
      }
    }

    // java public void getMergedRunProperty(MutablePropertySet desPropertySet, PropertySet runProperty, int paraStyleName, boolean bEnableRevision)
    pub fn getMergedRunProperty__mutablepropertyset_propertyset_int_boolean(&mut self, desPropertySet: &mut MutablePropertySet, runProperty: Option<&PropertySet>, paraStyleName: i32, bEnableRevision: bool) {
      // java PropertySet paraRunProperty = this.getParaPropertySet(paraStyleName);
      let mut paraRunProperty: PropertySet = self.getParaPropertySet(paraStyleName);
      // java if (runProperty != null)
      if let Some(runProperty) = runProperty {
        // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
        desPropertySet.addLimit(Some(&self.getDefaultRunProperty()), 1, 191);
        // java desPropertySet.remove(43);
        desPropertySet.remove(43);
        // java desPropertySet.addLimit(paraRunProperty, 1, 191);
        desPropertySet.addLimit(Some(&paraRunProperty), 1, 191);
        // java this.justifyToggle(desPropertySet, paraRunProperty, 57, (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi));
        self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &(desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap());
        // java int runStyleName = runProperty.getInt(2, 4095);
        let mut runStyleName: i32 = runProperty.getInt__int_int(2, 4095);
        // java if (runStyleName != 4095)
        if runStyleName != 4095 {
          // java PropertySet baseRunProperty = this.getRunPropertySet(runStyleName);
          let mut baseRunProperty: PropertySet = self.getRunPropertySet(runStyleName);
          // java desPropertySet.addLimit(baseRunProperty, 1, 191);
          desPropertySet.addLimit(Some(&baseRunProperty), 1, 191);
          // java this.justifyToggle(desPropertySet, baseRunProperty, 57, (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi));
          self.justifyToggle(&mut desPropertySet, Some(&baseRunProperty), 57, &(desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap());
        }
        // java desPropertySet.addLimit(runProperty, 1, 191);
        desPropertySet.addLimit(Some(&runProperty), 1, 191);
        // java this.justifyToggle(desPropertySet, runProperty, 57, (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi));
        self.justifyToggle(&mut desPropertySet, Some(&runProperty), 57, &(desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap());
        // java Revision revision = null;
        let mut revision: Revision = Revision::default();
        // java if (runProperty.contains(48) && bEnableRevision)
        if runProperty.contains(48) && bEnableRevision {
          // java revision = (Revision)runProperty.getValue(48);
          revision = (runProperty.getValue(48)).downcast_ref::<Revision>().unwrap();
          // java PropertySet prop = revision.getProperty();
          let mut prop: PropertySet = revision.getProperty();
          // java if (prop != null && prop.contains(2))
          if prop.is_some() && prop.contains(2) {
            // java runStyleName = prop.getInt(2);
            runStyleName = prop.getInt(2);
            // java PropertySet baseRunProperty = this.getRunPropertySet(runStyleName);
            let mut baseRunProperty: PropertySet = self.getRunPropertySet(runStyleName);
            // java MutablePropertySet styleRunProperty = new MutablePropertySet(baseRunProperty);
            let mut styleRunProperty: MutablePropertySet = MutablePropertySet::new(baseRunProperty);
            // java styleRunProperty.addLimit(this.getDefaultRunProperty(), 1, 191);
            styleRunProperty.addLimit(self.getDefaultRunProperty(), 1, 191);
            // java styleRunProperty.addLimit(baseRunProperty, 1, 191);
            styleRunProperty.addLimit(baseRunProperty, 1, 191);
            // java desPropertySet.addLimit(styleRunProperty.freeze(), 1, 191);
            desPropertySet.addLimit(Some(&styleRunProperty.freeze()), 1, 191);
            // java this.justifyToggle(desPropertySet, styleRunProperty.freeze(), 57, (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi));
            self.justifyToggle(&mut desPropertySet, Some(&styleRunProperty.freeze()), 57, &(desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap());
          }
        }
        // java if (null != revision)
        if revision.is_some() {
          // java PropertySet prop = revision.getProperty();
          let mut prop: PropertySet = revision.getProperty();
          // java if (prop != null)
          if prop.is_some() {
            // java desPropertySet.addLimit(prop, 1, 191);
            desPropertySet.addLimit(Some(&prop), 1, 191);
          }
        }
        // java PropertySet var11 = null;
        let mut var11: PropertySet = PropertySet::default();
      }
      // java else
      else {
        // java desPropertySet.addLimit(this.getDefaultRunProperty(), 1, 191);
        desPropertySet.addLimit(Some(&self.getDefaultRunProperty()), 1, 191);
        // java desPropertySet.addLimit(paraRunProperty, 1, 191);
        desPropertySet.addLimit(Some(&paraRunProperty), 1, 191);
        // java this.justifyToggle(desPropertySet, paraRunProperty, 57, (Toggle)desPropertySet.getObject(57, DefaultValue.run_bidi));
        self.justifyToggle(&mut desPropertySet, Some(&paraRunProperty), 57, &(desPropertySet.getObject__int_object(57, &DefaultValue::run_bidi)).downcast_ref::<Toggle>().unwrap());
      }
    }

    // java private Object mergeValue(Object orgValue, Object value)
    pub fn mergeValue(&mut self, orgValue: &dyn Object, value: &dyn Object) -> Box<dyn Object> {
      // java return value instanceof Toggle ? this.mergeToggle((Toggle)orgValue, (Toggle)value) : value;
      return (if value.downcast_ref::<Toggle>().is_some() { self.mergeToggle(&(orgValue).downcast_ref::<Toggle>().unwrap(), &(value).downcast_ref::<Toggle>().unwrap()) } else { value });
    }

    // java public Object getMergedParaProperty(PropertySet paraProp, int nProp)
    pub fn getMergedParaProperty__propertyset_int(&mut self, paraProp: Option<&PropertySet>, nProp: i32) -> Box<dyn Object> {
      // java Object object = this.getDefaultParaProperty().getValue(nProp);
      let mut object: Box<dyn Object> = self.getDefaultParaProperty().getValue(nProp);
      // java if (paraProp != null)
      if let Some(paraProp) = paraProp {
        // java Object value = null;
        let mut value: Box<dyn Object> = Box::new(()) as Box<dyn Object>;
        // java int styleName = paraProp.getInt(191, 0);
        let mut styleName: i32 = paraProp.getInt__int_int(191, 0);
        // java PropertySet baseParaProperty = this.getParaPropertySet(styleName);
        let mut baseParaProperty: PropertySet = self.getParaPropertySet(styleName);
        // java if (baseParaProperty != null)
        if baseParaProperty.is_some() {
          // java value = baseParaProperty.getValue(nProp);
          value = baseParaProperty.getValue(nProp);
        }
        // java if (value != null)
        if value.is_some() {
          // java object = this.mergeValue(object, value);
          object = self.mergeValue(&object, &value);
        }
        // java value = paraProp.getValue(nProp);
        value = paraProp.getValue(nProp);
        // java if (value != null)
        if value.is_some() {
          // java object = this.mergeValue(object, value);
          object = self.mergeValue(&object, &value);
        }
      }
      // java return object;
      return object;
    }

    // java public Object getMergedParaRevisionRunProperty(IChpxTable.Entry pFrag, int nProp)
    pub fn getMergedParaRevisionRunProperty(&self, pFrag: Option<&mut dyn ChpxEntry>, nProp: i32) -> Option<Box<dyn Object>> {
      // java if (pFrag != null && !pFrag.isEnd())
      if let Some(pFrag) = pFrag {
        if !pFrag.isEnd() {
          // java IPapxTable.Entry paragraph = this.document.getPapxTable().seek(pFrag.position());
          let mut paragraph: Box<dyn PapxEntry> = self.document.getPapxTable().seek(pFrag.position());
          // java PropertySet paraProp = paragraph.getProperty();
          let mut paraProp: PropertySet = paragraph.getProperty();
          // java int paraStyleId = paraProp.getInt(191, 4095);
          let mut paraStyleId: i32 = paraProp.getInt(191, 4095);
          // java Revision paraRevision = (Revision)paraProp.getValue(237);
          let mut paraRevision: Revision = (paraProp.getValue(237)).downcast_ref::<Revision>().unwrap();
          // java if (null != paraRevision)
          if paraRevision.is_some() {
            // java Integer changeParaStyleId = (Integer)paraRevision.getProperty().getValue(191);
            let mut changeParaStyleId: i32 = ((paraRevision.getProperty().getValue(191)) as i32);
            // java paraStyleId = null == changeParaStyleId ? paraStyleId : changeParaStyleId;
            paraStyleId = (if changeParaStyleId.is_none() { Some(paraStyleId) } else { Some(changeParaStyleId) });
          }
          // java return this.getMergedRunProperty(pFrag.getProperty(), paraStyleId, nProp);
          return self.getMergedRunProperty__mutablepropertyset_propertyset_int(&mut pFrag.getProperty(), Some(&paraStyleId), nProp);
        }
      }
      // java else
      else {
        // java return this.getDefaultRunProperty().getValue(nProp);
        return self.getDefaultRunProperty().getValue(nProp);
      }
    }

    // java public Object getMergedRunProperty(IChpxTable.Entry pFrag, int nProp)
    pub fn getMergedRunProperty__entry_int(&self, pFrag: Option<&mut dyn ChpxEntry>, nProp: i32) -> Box<dyn Object> {
      // java if (pFrag != null && !pFrag.isEnd())
      if let Some(pFrag) = pFrag {
        if !pFrag.isEnd() {
          // java IPapxTable.Entry paragraph = this.document.getPapxTable().seek(pFrag.position());
          let mut paragraph: Box<dyn PapxEntry> = self.document.getPapxTable().seek(pFrag.position());
          // java return this.getMergedRunProperty(pFrag.getProperty(), paragraph.getProperty().getInt(191, 4095), nProp);
          return self.getMergedRunProperty__mutablepropertyset_propertyset_int(&mut pFrag.getProperty(), Some(&paragraph.getProperty().getInt(191, 4095)), nProp);
        }
      }
      // java else
      else {
        // java return this.getDefaultRunProperty().getValue(nProp);
        return self.getDefaultRunProperty().getValue(nProp);
      }
    }

    // java public Object getMergedRunProperty(PropertySet runProperty, int paraStyleName, int nProp)
    pub fn getMergedRunProperty__propertyset_int_int(&mut self, runProperty: Option<&PropertySet>, paraStyleName: i32, nProp: i32) -> Option<Box<dyn Object>> {
      // java Object object = this.getDefaultRunProperty().getValue(nProp);
      let mut object: Box<dyn Object> = self.getDefaultRunProperty().getValue(nProp);
      // java PropertySet paraRunProperty = this.styleSheet.getStyleById(paraStyleName).getFinalProperty();
      let mut paraRunProperty: PropertySet = self.styleSheet.getStyleById__int(paraStyleName).getFinalProperty();
      // java if (paraRunProperty != null)
      if paraRunProperty.is_some() {
        // java Object value = paraRunProperty.getValue(nProp);
        let mut value: Box<dyn Object> = paraRunProperty.getValue(nProp);
        // java if (value != null)
        if value.is_some() {
          // java this.mergeValue(object, value);
          self.mergeValue(&object, &value);
        }
      }
      // java if (runProperty != null)
      if let Some(runProperty) = runProperty {
        // java Revision revision = (Revision)runProperty.getValue(48);
        let mut revision: Revision = (runProperty.getValue(48)).downcast_ref::<Revision>().unwrap();
        // java PropertySet revisionProp = null == revision ? null : revision.getProperty();
        let mut revisionProp: PropertySet = (if revision.is_none() { PropertySet::default() } else { revision.getProperty() });
        // java if (null != revisionProp && !revisionProp.isEmpty())
        if revisionProp.is_some() && !revisionProp.is_empty() {
          // java object = revisionProp.getValue(nProp);
          object = revisionProp.getValue(nProp);
          // java if (null != object)
          if object.is_some() {
            // java return object;
            return object;
          }
        }
        // java object = runProperty.getValue(nProp);
        object = runProperty.getValue(nProp);
        // java if (object != null)
        if object.is_some() {
          // java return object;
          return object;
        }
        // java Integer styleName = (Integer)runProperty.getValue(2);
        let mut styleName: i32 = ((runProperty.getValue(2)) as i32);
        // java if (styleName != null)
        if styleName.is_some() {
          // java KStyle baseStyle = this.styleSheet.getStyleById(styleName);
          let mut baseStyle: KStyle = self.styleSheet.getStyleById__int(styleName);
          // java if (baseStyle != null)
          if baseStyle.is_some() {
            // java PropertySet baseRunProperty = baseStyle.getFinalProperty();
            let mut baseRunProperty: PropertySet = baseStyle.getFinalProperty();
            // java Object value = baseRunProperty.getValue(nProp);
            let mut value: Box<dyn Object> = baseRunProperty.getValue(nProp);
            // java if (value != null)
            if value.is_some() {
              // java object = this.mergeValue(object, value);
              object = self.mergeValue(&object, &value);
            }
          }
        }
        // java Object value = runProperty.getValue(nProp);
        let mut value: Box<dyn Object> = runProperty.getValue(nProp);
        // java if (value != null)
        if value.is_some() {
          // java this.mergeValue(object, value);
          self.mergeValue(&object, &value);
        }
      }
      // java if (paraRunProperty != null)
      if paraRunProperty.is_some() {
        // java object = paraRunProperty.getValue(nProp);
        object = paraRunProperty.getValue(nProp);
        // java if (object != null)
        if object.is_some() {
          // java return object;
          return object;
        }
      }
      // java return this.getDefaultRunProperty().getValue(nProp);
      return self.getDefaultRunProperty().getValue(nProp);
    }

    // java public PropertySet getMergedTableProperty(int styleId)
    pub fn getMergedTableProperty__int(&self, styleId: i32) -> PropertySet {
      // java KStyle style = this.styleSheet.getStyleById(styleId);
      let mut style: KStyle = self.styleSheet.getStyleById__int(styleId);
      // java return style.getFinalProperty();
      return style.getFinalProperty();
    }

    // java public void getMergedTableProperty(MutablePropertySet desPropertySet, PropertySet paraProperty, boolean bEnableRevision)
    pub fn getMergedTableProperty__mutablepropertyset_propertyset_boolean(&self, desPropertySet: &mut MutablePropertySet, paraProperty: &PropertySet, bEnableRevision: bool) {
      // java this.getMergedParaProperty(desPropertySet, paraProperty);
      self.getMergedParaProperty__int_boolean(desPropertySet, paraProperty);
      // java if (bEnableRevision)
      if bEnableRevision {
        // java if (!desPropertySet.contains(363))
        if !desPropertySet.contains(363) {
          // java return;
          return;
        }
        // java PropertySet revisionProp = ((Revision)desPropertySet.getValue(363)).getProperty();
        let mut revisionProp: PropertySet = (desPropertySet.getValue(363)).downcast_ref::<Revision>().unwrap().getProperty();
        // java desPropertySet.addLimit(revisionProp, 291, 376);
        desPropertySet.addLimit(Some(&revisionProp), 291, 376);
      }
    }

}
