// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.core.runtime.IAssetManager
use util_adapter::cn::wps::core::runtime::IAssetManager::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.io.xwpf.IDocumentImporter
// soft: IDocumentImporter / StylesHandler / XWPFStyles not wired in this crate graph
// Java: import cn.wps.io.xwpf.element_handler.styles.StylesHandler
// Java: import cn.wps.io.xwpf.usermodel.XWPFStyles
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use util::moffice::crash::NoSpaceLeftException::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::Atom;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.util.StreamUtil
// soft: StreamUtil
// Java: import cn.wps.moffice.writer.core.property.PropertyCalculateFactory
// soft: property module not exported
// Java: import cn.wps.moffice.writer.io.reflection.ReflectionHelper
// soft: ReflectionHelper.getDocumentImporter path incomplete
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Set
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument;
use crate::cn::wps::moffice::writer::core::KStyle::KStyle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct StreamUtil;
impl StreamUtil {
    fn close__closeable<T>(_c: Option<&mut T>) {}
}
struct PropertyCalculateFactory;
impl PropertyCalculateFactory {
    fn updateStyleCache(_doc: &TextDocument) {}
}
struct XWPFStyles;
impl XWPFStyles {
    fn parse4BuiltinTblStyles<R, H>(_is: &R, _handler: &H) {}
}
struct BuiltinStyles;
impl BuiltinStyles {
    fn getBuiltinStyles() -> HashMap<i32, KStyle> {
        HashMap::new()
    }
}

/// java public interface IBuiltinStyleInitialize
pub trait IBuiltinStyleInitialize {
    fn initialize(&self, styles: &HashMap<i32, KStyle>);
}

fn property_set_empty() -> PropertySet {
    (*empty).clone()
}

fn clone_style_map(m: &HashMap<i32, KStyle>) -> HashMap<i32, KStyle> {
    m.iter().map(|(k, v)| (*k, v.clone_style())).collect()
}

// java private static final String TAG = null;
const TAG: &str = "";
// java static HashMap<Integer, KStyle> mBuiltinStyles;
// soft: thread_local - KStyle holds Rc (Atom.mTransaction) so not Sync for Mutex/OnceLock
thread_local! {
    static mBuiltinStyles: RefCell<Option<HashMap<i32, KStyle>>> = RefCell::new(None);
    // java static HashMap<Integer, KStyle> mBuiltinTableStyles;
    static mBuiltinTableStyles: RefCell<Option<HashMap<i32, KStyle>>> = RefCell::new(None);
    // java private static IBuiltinStyleInitialize sInitializer;
    static sInitializer: RefCell<Option<Rc<dyn IBuiltinStyleInitialize>>> = RefCell::new(None);
}

// java public class KStyles extends Atom
pub struct KStyles {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private final TextDocument mTextDocument;
    mTextDocument: TextDocument,
    // java @AtomMember PropertySet rPrDefault;
    pub(crate) rPrDefault: PropertySet,
    // java @AtomMember PropertySet pPrDefault;
    pub(crate) pPrDefault: PropertySet,
    // java @AtomMember(1) ArrayList<KStyle> styles;
    pub(crate) styles: Option<Vec<KStyle>>,
    // java HashMap<Integer, Integer> stylesInUse;
    pub(crate) stylesInUse: HashMap<i32, i32>,
}

// java public class KStyles extends Atom
impl KStyles {
    // java public  KStyles(TextDocument textDocument)
    pub fn KStyles(textDocument: &TextDocument) -> Self {
      // java this.rPrDefault = PropertySet.empty;
      // java this.pPrDefault = PropertySet.empty;
      // java this.styles = new ArrayList(10);
      // java this.stylesInUse = new HashMap();
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      let td_opt = Some(textDocument);
      Assert::assertNotNull_msg("textDocument should not be null.", &td_opt);
      // java this.mTextDocument = textDocument;
      let mut this = Self {
        super_: Atom::new(),
        mTextDocument: textDocument.clone(),
        rPrDefault: property_set_empty(),
        pPrDefault: property_set_empty(),
        styles: Some(Vec::with_capacity(10)),
        stylesInUse: HashMap::new(),
      };
      // java this.setTransaction(textDocument.getTransaction(), false);
      this
        .super_
        .setTransaction_recursive(textDocument.getTransaction(), false);
      this
    }

    // java public void deleteUnusedStyles()
    pub fn deleteUnusedStyles(&mut self) {
      // java HashMap<Integer, KStyle> styleInUse = this.getStylesPedigreeInUse();
      let styleInUse: HashMap<i32, KStyle> = self.getStylesPedigreeInUse();
      // java Set<Integer> keys = styleInUse.keySet();
      let keys: std::collections::HashSet<i32> = styleInUse.keys().copied().collect();
      // java ArrayList<KStyle> unUsedStyles = new ArrayList();
      let mut unUsedStyles: Vec<KStyle> = Vec::new();
      // java for(KStyle style : this.styles)
      if let Some(styles) = self.styles.as_ref() {
        for style in styles.iter() {
          // java int styleId = style.getId();
          let styleId: i32 = style.getId();
          // java if (0 != styleId && !keys.contains(styleId))
          if 0 != styleId && !keys.contains(&styleId) {
            // java unUsedStyles.add(style);
            unUsedStyles.push(style.clone_style());
          }
        }
      }
      // java this.styles.removeAll(unUsedStyles);
      if let Some(styles) = self.styles.as_mut() {
        let unused_ids: std::collections::HashSet<i32> =
          unUsedStyles.iter().map(|s| s.getId()).collect();
        styles.retain(|s| !unused_ids.contains(&s.getId()));
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (this.styles != null)
      if let Some(styles) = self.styles.as_mut() {
        // java int size = this.styles.size();
        let size: i32 = styles.len() as i32;
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java KStyle style = (KStyle)this.styles.get(i);
          if let Some(style) = styles.get_mut(i as usize) {
            // java if (style != null)
            // java style.dispose();
            style.dispose();
          }
          i += 1;
        }
        // java this.styles.clear();
        styles.clear();
        // java this.styles = null;
      }
      self.styles = None;
    }

    // java public TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> TextDocument {
      // java return this.mTextDocument;
      return self.mTextDocument.clone();
    }

    // java public PropertySet getRPrDefault()
    pub fn getRPrDefault(&self) -> PropertySet {
      // java return this.rPrDefault;
      return self.rPrDefault.clone();
    }

    // java public void setRPrDefault(PropertySet prop)
    pub fn setRPrDefault(&mut self, prop: &PropertySet) {
      // java if (this.rPrDefault.isEmpty())
      if self.rPrDefault.isEmpty() {
        // java this.writeLockAtom();
        self.super_.writeLockAtom();
        // java this.rPrDefault = prop;
        self.rPrDefault = prop.clone();
        // java if (this.mTextDocument != null)
        // java PropertyCalculateFactory.updateStyleCache(this.mTextDocument);
        PropertyCalculateFactory::updateStyleCache(&self.mTextDocument);
      }
    }

    // java public PropertySet getPPrDefault()
    pub fn getPPrDefault(&self) -> PropertySet {
      // java return this.pPrDefault;
      return self.pPrDefault.clone();
    }

    // java public void setPPrDefault(PropertySet prop)
    pub fn setPPrDefault(&mut self, prop: &PropertySet) {
      // java if (this.pPrDefault.isEmpty())
      if self.pPrDefault.isEmpty() {
        // java this.writeLockAtom();
        self.super_.writeLockAtom();
        // java this.pPrDefault = prop;
        self.pPrDefault = prop.clone();
        // java if (this.mTextDocument != null)
        // java PropertyCalculateFactory.updateStyleCache(this.mTextDocument);
        PropertyCalculateFactory::updateStyleCache(&self.mTextDocument);
      }
    }

    // java public String getFontLatin()
    pub fn getFontLatin(&self) -> String {
      // java return this.rPrDefault.getString(3, "Times New Roman");
      return self.rPrDefault.getString__int_string(3, "Times New Roman");
    }

    // java public String getFontAsian()
    pub fn getFontAsian(&self) -> String {
      // java return this.rPrDefault.getString(4, "??");
      return self.rPrDefault.getString__int_string(4, "??");
    }

    // java public String getFontOther()
    pub fn getFontOther(&self) -> String {
      // java return this.rPrDefault.getString(35, "Times New Roman");
      return self.rPrDefault.getString__int_string(35, "Times New Roman");
    }

    // java public KStyle getStyleById(int id, boolean bForceNormal)
    pub fn getStyleById__int_boolean(&self, id: i32, bForceNormal: bool) -> Option<KStyle> {
      // java int i = 0;
      let mut i: i32 = 0;
      let size: i32 = self.styles.as_ref().map(|a| a.len() as i32).unwrap_or(0);
      // java for(int size = this.styles.size(); i < size; ++i)
      while i < size {
        // java KStyle style = (KStyle)this.styles.get(i);
        if let Some(styles) = self.styles.as_ref() {
          if let Some(style) = styles.get(i as usize) {
            // java if (style.id == id)
            if style.id == id {
              // java return style;
              return Some(self.resolve_style_for_lookup(style));
            }
          }
        }
        i += 1;
      }
      // java if (bForceNormal && id != 0)
      if bForceNormal && id != 0 {
        // java return this.getStyleById(0);
        return self.getStyleById__int(0);
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // Rust ownership glue for Java's shared KStyle reference. The pointer is used
    // only while this borrowed style sheet is alive, then removed from the
    // returned value. KStyle.clone() itself must continue to detach the sheet.
    fn resolve_style_for_lookup(&self, style: &KStyle) -> KStyle {
      let mut resolved = self.clone_style_for_inheritance(style);
      resolved.getFinalProperty();
      resolved.styleSheet = None;
      resolved
    }

    fn clone_style_for_inheritance(&self, style: &KStyle) -> KStyle {
      let mut copy = style.clone_style();
      copy.styleSheet = Some(self as *const KStyles as *mut KStyles);
      copy
    }

    // Called only by KStyle.getFinalProperty(path), while the sheet is borrowed.
    pub(crate) fn getStyleByIdForInheritance(&self, id: i32) -> Option<KStyle> {
      let styles = self.styles.as_ref()?;
      let style = styles.iter().find(|style| style.id == id)
        .or_else(|| if id == 0 { None } else { styles.iter().find(|style| style.id == 0) })?;
      Some(self.clone_style_for_inheritance(style))
    }

    // java public KStyle getStyleById(int id)
    pub fn getStyleById__int(&self, id: i32) -> Option<KStyle> {
      // java return this.getStyleById(id, true);
      return self.getStyleById__int_boolean(id, true);
    }

    // java public KStyle getStyleByName(String name)
    pub fn getStyleByName(&self, name: Option<&str>) -> Option<KStyle> {
      // java if (name == null)
      if name.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        let name = name.unwrap();
        // java int i = 0;
        let mut i: i32 = 0;
        let size: i32 = self.styles.as_ref().map(|a| a.len() as i32).unwrap_or(0);
        // java for(int size = this.styles.size(); i < size; ++i)
        while i < size {
          // java if (name.equals(((KStyle)this.styles.get(i)).name))
          if let Some(styles) = self.styles.as_ref() {
            if let Some(style) = styles.get(i as usize) {
              if style.name.as_deref() == Some(name) {
                // java return (KStyle)this.styles.get(i);
                return Some(self.resolve_style_for_lookup(style));
              }
            }
          }
          i += 1;
        }
        // java KStyle pStyle = null;
        // java pStyle = this.getStyleById(0);
        let pStyle = self.getStyleById__int(0);
        // java return pStyle;
        return pStyle;
      }
    }

    // java public KStyle getStyleByIndex(int index)
    pub fn getStyleByIndex(&self, index: i32) -> Option<KStyle> {
      // java return (KStyle)this.styles.get(index);
      return self
        .styles
        .as_ref()
        .and_then(|s| s.get(index as usize).map(|st| self.resolve_style_for_lookup(st)));
    }

    // java public void add(KStyle style)
    pub fn add(&mut self, style: &mut KStyle) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java style.styleSheet = this;
      style.styleSheet = Some(self as *mut KStyles);
      let mut iStyle: i32 = 0;
      // java for(int iStyle = 0; iStyle < this.styles.size(); ++iStyle)
      while iStyle < self.styles.as_ref().map(|a| a.len() as i32).unwrap_or(0) {
        // java KStyle pStyle = (KStyle)this.styles.get(iStyle);
        if let Some(styles) = self.styles.as_ref() {
          if let Some(pStyle) = styles.get(iStyle as usize) {
            // java if (pStyle.id == style.id)
            if pStyle.id == style.id {
              // java this.styles.set(iStyle, style);
              if let Some(styles) = self.styles.as_mut() {
                styles[iStyle as usize] = style.clone_style();
              }
              // java return;
              return;
            }
          }
        }
        iStyle += 1;
      }
      // java this.styles.add(style);
      if let Some(styles) = self.styles.as_mut() {
        styles.push(style.clone_style());
      }
    }

    // java public ArrayList<KStyle> getStyles()
    pub fn getStyles(&self) -> Vec<KStyle> {
      // java return this.styles;
      return self
        .styles
        .as_ref()
        .map(|s| s.iter().map(|st| self.resolve_style_for_lookup(st)).collect())
        .unwrap_or_default();
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.styles.size();
      return self.styles.as_ref().map(|a| a.len() as i32).unwrap_or(0);
    }

    // java public static synchronized HashMap<Integer, KStyle> getBuiltinStyles()
    pub fn getBuiltinStyles() -> HashMap<i32, KStyle> {
      // java if (mBuiltinStyles == null)
      mBuiltinStyles.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
          // java mBuiltinStyles = BuiltinStyles.getBuiltinStyles();
          let map = BuiltinStyles::getBuiltinStyles();
          // java if (sInitializer != null)
          sInitializer.with(|init_cell| {
            if let Some(init) = init_cell.borrow().as_ref() {
              // java sInitializer.initialize(mBuiltinStyles);
              init.initialize(&map);
            }
          });
          *slot = Some(map);
        }
        // java return mBuiltinStyles;
        clone_style_map(slot.as_ref().unwrap())
      })
    }

    // java public static void resetBuiltinStyles()
    pub fn resetBuiltinStyles() {
      // java mBuiltinStyles = null;
      mBuiltinStyles.with(|cell| {
        *cell.borrow_mut() = None;
      });
    }

    // java public static void setBuiltinStyleInitializer(IBuiltinStyleInitialize p)
    pub fn setBuiltinStyleInitializer(p: Option<Rc<dyn IBuiltinStyleInitialize>>) {
      // java sInitializer = p;
      sInitializer.with(|cell| {
        *cell.borrow_mut() = p;
      });
    }

    // java public static synchronized HashMap<Integer, KStyle> getBuiltinTableStyle(TextDocument textDocument)
    pub fn getBuiltinTableStyle__textdocument(textDocument: &TextDocument) -> HashMap<i32, KStyle> {
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      let td_opt = Some(textDocument);
      Assert::assertNotNull_msg("textDocument should not be null.", &td_opt);
      // java if (null != mBuiltinTableStyles)
      let existing = mBuiltinTableStyles.with(|cell| cell.borrow().as_ref().map(clone_style_map));
      if let Some(map) = existing {
        // java return mBuiltinTableStyles;
        return map;
      }
      // java else
      else {
        // java mBuiltinTableStyles = new HashMap();
        mBuiltinTableStyles.with(|cell| {
          *cell.borrow_mut() = Some(HashMap::new());
        });
        // java InputStream is = getStyleInputStream();
        let is = Self::getStyleInputStream();
        // java if (null == is)
        if is.is_none() {
          // java return mBuiltinTableStyles;
          return mBuiltinTableStyles
            .with(|cell| cell.borrow().as_ref().map(clone_style_map).unwrap_or_default());
        }
        // java else
        else {
          let mut is = is.unwrap();
          // java KStyles kStyles = new KStyles(textDocument);
          let kStyles: KStyles = KStyles::KStyles(textDocument);
          // java parseStyleXml(is, kStyles);
          Self::parseStyleXml(&mut is, &kStyles);
          // java putBuiltinTblStyles2Map(kStyles);
          Self::putBuiltinTblStyles2Map(&kStyles);
          // java StreamUtil.close(is);
          StreamUtil::close__closeable(Some(&mut is));
          // java return mBuiltinTableStyles;
          return mBuiltinTableStyles
            .with(|cell| cell.borrow().as_ref().map(clone_style_map).unwrap_or_default());
        }
      }
    }

    // java private static void parseStyleXml(InputStream is, KStyles kStyles)
    pub fn parseStyleXml(_is: &mut SoftInputStream, kStyles: &KStyles) {
      // java IDocumentImporter importer = ReflectionHelper.getDocumentImporter(kStyles.getTextDocument(), kStyles, 0);
      let _ = kStyles.getTextDocument();
      // soft: ReflectionHelper / StylesHandler / XWPFStyles not available - no-op parse
      // java StylesHandler handler = new StylesHandler(importer);
      // java XWPFStyles.parse4BuiltinTblStyles(is, handler);
      XWPFStyles::parse4BuiltinTblStyles(&_is, &());
      // java importer.dispose4StyleXml();
    }

    // java private static InputStream getStyleInputStream()
    pub fn getStyleInputStream() -> Option<SoftInputStream> {
      // java String path = "styles.xml";
      let _path: String = "styles.xml".to_string();
      // java InputStream is = null;
      let mut is: Option<SoftInputStream> = None;
      // java try
      {
        // java IAssetManager assetManager = Platform.getAssetManager();
        let _assetManager = Platform::getAssetManager();
        // java is = assetManager.open("styles.xml");
        // soft: asset open returns opaque Any - leave None if unavailable
        is = None;
      }
      // java catch (IOException e)
      // soft: no fallible open wired; catch path unused
      let _ = (
        TAG,
        // java KSLog.e(TAG, "IOException", e);
        // java Assert.fail("it should not reach here");
        // java if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // java throw new RuntimeException(e);
        NoSpaceLeftException::isNoSpaceLeftException(None::<&dyn std::error::Error>),
      );
      // java return is;
      return is;
    }

    // java private static void putBuiltinTblStyles2Map(KStyles kStyles)
    pub fn putBuiltinTblStyles2Map(kStyles: &KStyles) {
      // java ArrayList<KStyle> list = kStyles.getStyles();
      let list: Vec<KStyle> = kStyles.getStyles();
      // java int size = list.size();
      let size: i32 = list.len() as i32;
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java KStyle style = (KStyle)list.get(i);
        if let Some(style) = list.get(i as usize) {
          // java if (null != style)
          // java mBuiltinTableStyles.put(style.getId(), style);
          mBuiltinTableStyles.with(|cell| {
            if let Some(map) = cell.borrow_mut().as_mut() {
              map.insert(style.getId(), style.clone_style());
            }
          });
        }
        i += 1;
      }
    }

    // java public static synchronized KStyle getBuiltinTableStyle(TextDocument textDocument, int styleID)
    pub fn getBuiltinTableStyle__textdocument_int(
      textDocument: &TextDocument,
      styleID: i32,
    ) -> Option<KStyle> {
      // java HashMap<Integer, KStyle> styles = getBuiltinTableStyle(textDocument);
      let styles: HashMap<i32, KStyle> = Self::getBuiltinTableStyle__textdocument(textDocument);
      // java return styles == null ? null : (KStyle)styles.get(styleID);
      return styles.get(&styleID).map(|s| s.clone_style());
    }

    // java public HashMap<Integer, KStyle> getStylesPedigreeInUse()
    pub fn getStylesPedigreeInUse(&mut self) -> HashMap<i32, KStyle> {
      // java HashMap<Integer, KStyle> stylesMap = this.initStylesMap();
      let stylesMap: HashMap<i32, KStyle> = self.initStylesMap();
      // java HashMap<Integer, KStyle> ret = new HashMap();
      let mut ret: HashMap<i32, KStyle> = HashMap::new();
      // java for(Integer k : this.stylesInUse.keySet())
      let keys: Vec<i32> = self.stylesInUse.keys().copied().collect();
      for k in keys {
        // java Integer value = (Integer)this.stylesInUse.get(k);
        let value = self.stylesInUse.get(&k).copied();
        // java if (null != value && 0 != value)
        if let Some(value) = value {
          if 0 != value {
            // java KStyle style = (KStyle)stylesMap.get(k);
            if let Some(style) = stylesMap.get(&k) {
              // java if (null != style)
              // java this.recordStyle(stylesMap, ret, style);
              self.recordStyle(&stylesMap, &mut ret, style);
            }
          }
        }
      }
      // java return ret;
      return ret;
    }

    // java private void recordStyle(HashMap<Integer, KStyle> stylesMap, HashMap<Integer, KStyle> target, KStyle kStyle)
    pub fn recordStyle(
      &mut self,
      stylesMap: &HashMap<i32, KStyle>,
      target: &mut HashMap<i32, KStyle>,
      kStyle: &KStyle,
    ) {
      // java target.put(kStyle.id, kStyle);
      target.insert(kStyle.id, kStyle.clone_style());
      // java if (!target.containsKey(kStyle.base))
      if !target.contains_key(&kStyle.base) {
        // java KStyle base = (KStyle)stylesMap.get(kStyle.base);
        if let Some(base) = stylesMap.get(&kStyle.base) {
          // java if (null != base)
          // java this.recordStyle(stylesMap, target, base);
          self.recordStyle(stylesMap, target, base);
        }
      }
    }

    // java private HashMap<Integer, KStyle> initStylesMap()
    pub fn initStylesMap(&self) -> HashMap<i32, KStyle> {
      // java HashMap<Integer, KStyle> ret = new HashMap();
      let mut ret: HashMap<i32, KStyle> = HashMap::new();
      // java int size = this.styles.size();
      let size: i32 = self.styles.as_ref().map(|a| a.len() as i32).unwrap_or(0);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java KStyle style = (KStyle)this.styles.get(i);
        if let Some(styles) = self.styles.as_ref() {
          if let Some(style) = styles.get(i as usize) {
            // java ret.put(style.id, style);
            ret.insert(style.id, style.clone_style());
          }
        }
        i += 1;
      }
      // java return ret;
      return ret;
    }
}

/// Soft stand-in for java.io.InputStream used by builtin table-style load path.
pub struct SoftInputStream;

impl std::ops::Deref for KStyles {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KStyles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
