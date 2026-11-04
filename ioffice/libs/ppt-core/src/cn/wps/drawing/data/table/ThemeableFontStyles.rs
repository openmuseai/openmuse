#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.style.FontCollection
use crate::cn::wps::drawing::data::style::FontCollection::FontCollection;
// Java: import cn.wps.drawing.data.style.FontRef
use crate::cn::wps::drawing::data::style::FontRef::FontRef;
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

// Java: public class ThemeableFontStyles implements KoAr, KoClosable
pub struct ThemeableFontStyles {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: FontCollection font
    pub(crate) font: Option<Box<FontCollection>>,
    // Java field: FontRef fontRef
    pub(crate) fontRef: Option<Box<FontRef>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ThemeableFontStyles parent
    pub(crate) parent: Option<Box<ThemeableFontStyles>>,
}

// Java: public class ThemeableFontStyles implements KoAr, KoClosable
impl ThemeableFontStyles {
    // Java: static protected static <Value1 extends ThemeableFontStyles> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ThemeableFontStyles_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ThemeableFontStyles create()
    pub fn create() -> Self {
      // Java: return initialize(ThemeableFontStyles.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ThemeableFontStyles open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ThemeableFontStyles.class, bin);
      Self::initialize(bin)
    }

    // Java: ThemeableFontStyles()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), font: None, fontRef: None, parent: None,
      }
    }

    // Java: ThemeableFontStyles(KoBin bin)
    pub fn ThemeableFontStyles_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), font: None, fontRef: None, parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(ThemeableFontStyles value)
    pub fn load_value(&mut self, value: Option<&ThemeableFontStyles>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin_with_zone(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.font != null && this.font.dirty())
        if self.font.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(1, this.font.saveBin(zone));
         let bin = self.font.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.fontRef != null && this.fontRef.dirty())
        if self.fontRef.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(2, this.fontRef.saveBin(zone));
         let bin = self.fontRef.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(ThemeableFontStyles o)
    pub fn equalsLax(&mut self, o: &mut ThemeableFontStyles) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut ThemeableFontStyles) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut ThemeableFontStyles, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ThemeableFontStyles)
        if true
      {
         // Java: ThemeableFontStyles dst = (ThemeableFontStyles)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFont() || !dst.hasFont()) && (!this.hasFont() || dst.hasFont()))
            else if (self.hasFont() || !dst.hasFont()) && (!self.hasFont() || dst.hasFont())
         {
            // Java: if (this.hasFont() && dst.hasFont())
                if self.hasFont() && dst.hasFont()
            {
               // Java: if (isStrict && !this.font().equals(dst.font()))
                    if isStrict && !self.get_font_mut().equals(dst.get_font_mut())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.font().equalsLax(dst.font()))
                    if !isStrict && !self.get_font_mut().equalsLax(dst.get_font_mut())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasFontRef() || !dst.hasFontRef()) && (!this.hasFontRef() || dst.hasFontRef()))
                if (self.hasFontRef() || !dst.hasFontRef()) && (!self.hasFontRef() || dst.hasFontRef())
            {
               // Java: if (this.hasFontRef() && dst.hasFontRef())
                    if self.hasFontRef() && dst.hasFontRef()
               {
                  // Java: if (isStrict && !this.fontRef().equals(dst.fontRef()))
                        if isStrict && !self.get_fontRef_mut().equals(dst.get_fontRef_mut())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.fontRef().equalsLax(dst.fontRef()))
                        if !isStrict && !self.get_fontRef_mut().equalsLax(dst.get_fontRef_mut())
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: return true;
               return true;
            }
            // Java: else
                else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
            else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
        else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasFont())
        if self.hasFont()
      {
         // Java: ret += this.font().hashCode();
         ret = ret.wrapping_add(self.get_font_mut().hashCode());
      }
      // Java: if (this.hasFontRef())
        if self.hasFontRef()
      {
         // Java: ret += this.fontRef().hashCode();
         ret = ret.wrapping_add(self.get_fontRef_mut().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.font != null && this.font.dirty())
        if self.font.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.fontRef != null && this.fontRef.dirty() ? true : this.map.dirty();
         if self.fontRef.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanParent();
      self.cleanParent();
      // Java: this.map.load(KoBin.Null.value1());
      let null_bin = Null::value1();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.font = null;
      self.font = None;
      // Java: this.fontRef = null;
      self.fontRef = None;
    }

    // Java: void follow(ThemeableFontStyles parent)
    pub fn follow(&mut self, parent: Option<Box<ThemeableFontStyles>>) {
      // Java: if (parent != null)
        if let Some(mut p) = parent
      {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(p);
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: ThemeableFontStyles parent()
    pub fn get_parent(&self) -> Option<&ThemeableFontStyles> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    pub fn get_font_mut(&mut self) -> &mut FontCollection {
      if self.font.is_none() {
        if self.map.has(1) {
          todo!("lazy init font from map");
        } else if self.parent.as_ref().map_or(false, |p| p.hasFont()) {
          todo!("copy font from parent");
        } else {
          todo!("default init font");
        }
      }
      self.font.as_deref_mut().unwrap()
    }

    pub fn get_fontRef_mut(&mut self) -> &mut FontRef {
      if self.fontRef.is_none() {
        if self.map.has(2) {
          todo!("lazy init fontRef from map");
        } else if self.parent.as_ref().map_or(false, |p| p.hasFontRef()) {
          todo!("copy fontRef from parent");
        } else {
          todo!("default init fontRef");
        }
      }
      self.fontRef.as_deref_mut().unwrap()
    }

    // Java: FontCollection font()
    pub fn get_font(&self) -> &FontCollection {
      // Java: if (this.font == null)
        if self.font.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.font = (FontCollection)KoBin.Decode.toAny(FontCollection.class, bin, this.zone);
            todo!("lazy init font from map");
         }
         // Java: else if (this.parent != null && this.parent.hasFont())
            else if self.parent.as_ref().map_or(false, |p| p.hasFont())
         {
            // Java: this.font = this.parent.font();
            todo!("copy font from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.font = (FontCollection)KoBin.Decode.toAny(FontCollection.class, bin, this.zone);
            todo!("default init font");
         }
      }
      // Java: return this.font;
      self.font.as_deref().unwrap()
    }

    // Java: boolean hasFont()
    pub fn hasFont(&self) -> bool {
      // Java: return this.map.has(1) || this.font != null && this.font.dirty() || this.parent != null && this.parent.hasFont();
      self.map.has(1) || self.font.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFont())
    }

    // Java: void deleteFont()
    pub fn deleteFont(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.font != null)
        if self.font.is_some()
      {
         // Java: this.font = null;
         self.font = None;
      }
    }

    // Java: void writeFont(FontCollection value)
    pub fn writeFont(&mut self, value: Option<&FontCollection>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.font != null)
                if self.font.is_some()
            {
               // Java: this.font = null;
               self.font = None;
            }
         }
      }
    }

    // Java: void forceWriteFont(FontCollection value)
    pub fn forceWriteFont(&mut self, value: Option<&FontCollection>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.font != null)
            if self.font.is_some()
         {
            // Java: this.font = null;
            self.font = None;
         }
      }
    }

    // Java: FontRef fontRef()
    pub fn get_fontRef(&self) -> &FontRef {
      // Java: if (this.fontRef == null)
        if self.fontRef.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.fontRef = (FontRef)KoBin.Decode.toAny(FontRef.class, bin, this.zone);
            todo!("lazy init fontRef from map");
         }
         // Java: else if (this.parent != null && this.parent.hasFontRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasFontRef())
         {
            // Java: this.fontRef = this.parent.fontRef();
            todo!("copy fontRef from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.fontRef = (FontRef)KoBin.Decode.toAny(FontRef.class, bin, this.zone);
            todo!("default init fontRef");
         }
      }
      // Java: return this.fontRef;
      self.fontRef.as_deref().unwrap()
    }

    // Java: boolean hasFontRef()
    pub fn hasFontRef(&self) -> bool {
      // Java: return this.map.has(2) || this.fontRef != null && this.fontRef.dirty() || this.parent != null && this.parent.hasFontRef();
      self.map.has(2) || self.fontRef.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFontRef())
    }

    // Java: void deleteFontRef()
    pub fn deleteFontRef(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.fontRef != null)
        if self.fontRef.is_some()
      {
         // Java: this.fontRef = null;
         self.fontRef = None;
      }
    }

    // Java: void writeFontRef(FontRef value)
    pub fn writeFontRef(&mut self, value: Option<&FontRef>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(2, bin);
            // Java: if (this.fontRef != null)
                if self.fontRef.is_some()
            {
               // Java: this.fontRef = null;
               self.fontRef = None;
            }
         }
      }
    }

    // Java: void forceWriteFontRef(FontRef value)
    pub fn forceWriteFontRef(&mut self, value: Option<&FontRef>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.fontRef != null)
            if self.fontRef.is_some()
         {
            // Java: this.fontRef = null;
            self.fontRef = None;
         }
      }
    }

}

impl KoAr for ThemeableFontStyles {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ThemeableFontStyles::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ThemeableFontStyles::lastBin(self, var1) }
}

impl KoClosable for ThemeableFontStyles {
    fn close(&mut self) { ThemeableFontStyles::close(self); }
}
