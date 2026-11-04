#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.drawing.data.style.StyleMatrixReference
use crate::cn::wps::drawing::data::style::StyleMatrixReference::StyleMatrixReference;
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

// Java: public class ThemeableLineStyle implements KoAr, KoClosable
pub struct ThemeableLineStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Outline outline
    pub(crate) outline: Option<Box<Outline>>,
    // Java field: StyleMatrixReference styleMatrixRef
    pub(crate) styleMatrixRef: Option<Box<StyleMatrixReference>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ThemeableLineStyle parent
    pub(crate) parent: Option<Box<ThemeableLineStyle>>,
}

// Java: public class ThemeableLineStyle implements KoAr, KoClosable
impl ThemeableLineStyle {
    // Java: static protected static <Value1 extends ThemeableLineStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ThemeableLineStyle_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ThemeableLineStyle create()
    pub fn create() -> Self {
      // Java: return initialize(ThemeableLineStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ThemeableLineStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ThemeableLineStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: ThemeableLineStyle()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), outline: None, styleMatrixRef: None, parent: None,
      }
    }

    // Java: ThemeableLineStyle(KoBin bin)
    pub fn ThemeableLineStyle_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), outline: None, styleMatrixRef: None, parent: None,
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

    // Java: void load(ThemeableLineStyle value)
    pub fn load_value(&mut self, value: Option<&ThemeableLineStyle>) {
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
      // Java: if (this.outline != null && this.outline.dirty())
        if self.outline.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(1, this.outline.saveBin(zone));
         let bin = self.outline.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.styleMatrixRef != null && this.styleMatrixRef.dirty())
        if self.styleMatrixRef.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(2, this.styleMatrixRef.saveBin(zone));
         let bin = self.styleMatrixRef.as_mut().unwrap().saveBin(zone);
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

    // Java: boolean equalsLax(ThemeableLineStyle o)
    pub fn equalsLax(&self, o: &ThemeableLineStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ThemeableLineStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ThemeableLineStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ThemeableLineStyle)
        if true
      {
         // Java: ThemeableLineStyle dst = (ThemeableLineStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasOutline() || !dst.hasOutline()) && (!this.hasOutline() || dst.hasOutline()))
            else if (self.hasOutline() || !dst.hasOutline()) && (!self.hasOutline() || dst.hasOutline())
         {
            // Java: if (this.hasOutline() && dst.hasOutline())
                if self.hasOutline() && dst.hasOutline()
            {
               // Java: if (isStrict && !this.outline().equals(dst.outline()))
                    if isStrict && !self.get_outline().equals(dst.get_outline())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.outline().equalsLax(dst.outline()))
                    if !isStrict && !self.get_outline().equalsLax(dst.get_outline())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasStyleMatrixRef() || !dst.hasStyleMatrixRef()) && (!this.hasStyleMatrixRef() || dst.hasStyleMatrixRef()))
                if (self.hasStyleMatrixRef() || !dst.hasStyleMatrixRef()) && (!self.hasStyleMatrixRef() || dst.hasStyleMatrixRef())
            {
               // Java: if (this.hasStyleMatrixRef() && dst.hasStyleMatrixRef())
                    if self.hasStyleMatrixRef() && dst.hasStyleMatrixRef()
               {
                  // Java: if (isStrict && !this.styleMatrixRef().equals(dst.styleMatrixRef()))
                        if isStrict && !self.get_styleMatrixRef().equals(dst.get_styleMatrixRef())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.styleMatrixRef().equalsLax(dst.styleMatrixRef()))
                        if !isStrict && !self.get_styleMatrixRef().equalsLax(dst.get_styleMatrixRef())
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
      // Java: if (this.hasOutline())
        if self.hasOutline()
      {
         // Java: ret += this.outline().hashCode();
         ret = ret.wrapping_add(self.get_outline().hashCode());
      }
      // Java: if (this.hasStyleMatrixRef())
        if self.hasStyleMatrixRef()
      {
         // Java: ret += this.styleMatrixRef().hashCode();
         ret = ret.wrapping_add(self.get_styleMatrixRef_mut().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.outline != null && this.outline.dirty())
        if self.outline.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.styleMatrixRef != null && this.styleMatrixRef.dirty() ? true : this.map.dirty();
         if self.styleMatrixRef.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.outline = null;
      self.outline = None;
      // Java: this.styleMatrixRef = null;
      self.styleMatrixRef = None;
    }

    // Java: void follow(ThemeableLineStyle parent)
    pub fn follow(&mut self, parent: Option<Box<ThemeableLineStyle>>) {
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

    // Java: ThemeableLineStyle parent()
    pub fn get_parent(&self) -> Option<&ThemeableLineStyle> {
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

    pub fn get_styleMatrixRef_mut(&mut self) -> &mut StyleMatrixReference {
      if self.styleMatrixRef.is_none() {
        if self.map.has(2) {
          todo!("lazy init styleMatrixRef from map");
        } else if self.parent.as_ref().map_or(false, |p| p.hasStyleMatrixRef()) {
          todo!("copy styleMatrixRef from parent");
        } else {
          todo!("default init styleMatrixRef");
        }
      }
      self.styleMatrixRef.as_deref_mut().unwrap()
    }

    // Java: Outline outline()
    pub fn get_outline(&self) -> &Outline {
      // Java: if (this.outline == null)
        if self.outline.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.outline = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init outline from map");
         }
         // Java: else if (this.parent != null && this.parent.hasOutline())
            else if self.parent.as_ref().map_or(false, |p| p.hasOutline())
         {
            // Java: this.outline = this.parent.outline();
            todo!("copy outline from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.outline = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init outline");
         }
      }
      // Java: return this.outline;
      self.outline.as_deref().unwrap()
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
      // Java: return this.map.has(1) || this.outline != null && this.outline.dirty() || this.parent != null && this.parent.hasOutline();
      self.map.has(1) || self.outline.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasOutline())
    }

    // Java: void deleteOutline()
    pub fn deleteOutline(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.outline != null)
        if self.outline.is_some()
      {
         // Java: this.outline = null;
         self.outline = None;
      }
    }

    // Java: void writeOutline(Outline value)
    pub fn writeOutline(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.outline != null)
                if self.outline.is_some()
            {
               // Java: this.outline = null;
               self.outline = None;
            }
         }
      }
    }

    // Java: void forceWriteOutline(Outline value)
    pub fn forceWriteOutline(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.outline != null)
            if self.outline.is_some()
         {
            // Java: this.outline = null;
            self.outline = None;
         }
      }
    }

    // Java: StyleMatrixReference styleMatrixRef()
    pub fn get_styleMatrixRef(&self) -> &StyleMatrixReference {
      // Java: if (this.styleMatrixRef == null)
        if self.styleMatrixRef.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.styleMatrixRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            todo!("lazy init styleMatrixRef from map");
         }
         // Java: else if (this.parent != null && this.parent.hasStyleMatrixRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasStyleMatrixRef())
         {
            // Java: this.styleMatrixRef = this.parent.styleMatrixRef();
            todo!("copy styleMatrixRef from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.styleMatrixRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            todo!("default init styleMatrixRef");
         }
      }
      // Java: return this.styleMatrixRef;
      self.styleMatrixRef.as_deref().unwrap()
    }

    // Java: boolean hasStyleMatrixRef()
    pub fn hasStyleMatrixRef(&self) -> bool {
      // Java: return this.map.has(2) || this.styleMatrixRef != null && this.styleMatrixRef.dirty() || this.parent != null && this.parent.hasStyleMatrixRef();
      self.map.has(2) || self.styleMatrixRef.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasStyleMatrixRef())
    }

    // Java: void deleteStyleMatrixRef()
    pub fn deleteStyleMatrixRef(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.styleMatrixRef != null)
        if self.styleMatrixRef.is_some()
      {
         // Java: this.styleMatrixRef = null;
         self.styleMatrixRef = None;
      }
    }

    // Java: void writeStyleMatrixRef(StyleMatrixReference value)
    pub fn writeStyleMatrixRef(&mut self, value: Option<&StyleMatrixReference>) {
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
            // Java: if (this.styleMatrixRef != null)
                if self.styleMatrixRef.is_some()
            {
               // Java: this.styleMatrixRef = null;
               self.styleMatrixRef = None;
            }
         }
      }
    }

    // Java: void forceWriteStyleMatrixRef(StyleMatrixReference value)
    pub fn forceWriteStyleMatrixRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.styleMatrixRef != null)
            if self.styleMatrixRef.is_some()
         {
            // Java: this.styleMatrixRef = null;
            self.styleMatrixRef = None;
         }
      }
    }

}

impl KoAr for ThemeableLineStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ThemeableLineStyle::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ThemeableLineStyle::lastBin(self, var1) }
}

impl KoClosable for ThemeableLineStyle {
    fn close(&mut self) { ThemeableLineStyle::close(self); }
}
