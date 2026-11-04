#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
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

// Java: public class StyleMatrixReference implements KoAr, KoClosable
pub struct StyleMatrixReference {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color color
    pub(crate) color: Option<Box<Color>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: StyleMatrixReference parent
    pub(crate) parent: Option<Box<StyleMatrixReference>>,
}

// Java: public class StyleMatrixReference implements KoAr, KoClosable
impl StyleMatrixReference {
    // Java: private static final int $idx = 1
    const FIELD_IDX: i32 = 1;
    // Java: private static final int $color = 2
    const FIELD_COLOR: i32 = 2;

    // Java: static protected static <Value1 extends StyleMatrixReference> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      // Java: inst.zone = zone;
      // Java: inst.load(bin);
      let mut inst = Self::new_from(bin);
      inst.zone = zone;
      // Java: return inst;
      inst
    }

    // Java: static StyleMatrixReference create()
    pub fn create() -> Self {
      // Java: return initialize(StyleMatrixReference.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static StyleMatrixReference open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(StyleMatrixReference.class, bin);
      Self::initialize(bin)
    }

    // Java: StyleMatrixReference()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        color: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: StyleMatrixReference(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        color: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(StyleMatrixReference value)
    pub fn load_from(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.color != null && this.color.dirty())
        if self.color.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.color.saveBin(zone));
         let bin = self.color.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(StyleMatrixReference o)
    pub fn equalsLax(&self, o: &StyleMatrixReference) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &StyleMatrixReference) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &StyleMatrixReference, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof StyleMatrixReference)
      {
         // Java: StyleMatrixReference dst = (StyleMatrixReference)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasIdx() || !dst.hasIdx()) && (!this.hasIdx() || dst.hasIdx()))
            else if (self.hasIdx() || !dst.hasIdx()) && (!self.hasIdx() || dst.hasIdx())
         {
            // Java: if (this.hasIdx() && dst.hasIdx() && this.idx() != dst.idx())
               if self.hasIdx() && dst.hasIdx() && self.idx() != dst.idx()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasColor() || !dst.hasColor()) && (!this.hasColor() || dst.hasColor()))
               else if (self.hasColor() || !dst.hasColor()) && (!self.hasColor() || dst.hasColor())
            {
               // Java: if (this.hasColor() && dst.hasColor())
                  if self.hasColor() && dst.hasColor()
               {
                  // Java: if (isStrict && !this.color().equals(dst.color()))
                     if isStrict
                  {
                     // Java: return false;
                     todo!("color equals");
                  }
                  // Java: if (!isStrict && !this.color().equalsLax(dst.color()))
                     if !isStrict
                  {
                     // Java: return false;
                     todo!("color equalsLax");
                  }
               }
               // Java: return true;
               return true;
            }
            // Java: else
            else {
               // Java: return false;
               return false;
            }
         }
         // Java: else
         else {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasIdx())
      if self.hasIdx() {
         // Java: ret += Integer.valueOf(this.idx()).hashCode();
         ret = ret.wrapping_add(self.idx());
      }
      // Java: if (this.hasColor())
      if self.hasColor() {
         // Java: ret += this.color().hashCode();
         ret = ret.wrapping_add(self.color().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.color != null && this.color.dirty() ? true : this.map.dirty();
      self.color.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
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
      self.loadBin(bin.as_ref(), zone);
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.color = null;
      self.color = None;
    }

    // Java: void follow(StyleMatrixReference parent)
    pub fn follow(&mut self, parent: Option<Box<StyleMatrixReference>>) {
      // Java: if (parent != null)
      if let Some(mut p) = parent {
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
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent {
           p.discard();
         }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: StyleMatrixReference parent()
    pub fn parent(&self) -> &StyleMatrixReference {
      // Java: return this.parent;
      self.parent.as_deref().unwrap()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int idx()
    pub fn idx(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.idx();
         return self.parent.as_ref().map_or(0, |p| p.idx());
      }
      // Java: else
         else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasIdx()
    pub fn hasIdx(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasIdx();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasIdx())
    }

    // Java: void deleteIdx()
    pub fn deleteIdx(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putIdx(int value)
    pub fn putIdx(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: Color color()
    pub fn color(&mut self) -> &Color {
      // Java: if (this.color == null)
        if self.color.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.color = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasColor())
         {
            // Java: this.color = this.parent.color();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.color = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.color;
      self.color.as_deref().unwrap()
    }

    // Java: boolean hasColor()
    pub fn hasColor(&self) -> bool {
      // Java: return this.map.has(2) || this.color != null && this.color.dirty() || this.parent != null && this.parent.hasColor();
      self.map.has(2)
        || self.color.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasColor())
    }

    // Java: void deleteColor()
    pub fn deleteColor(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.color != null)
        if self.color.is_some()
      {
         // Java: this.color = null;
         self.color = None;
      }
    }

    // Java: void writeColor(Color value)
    pub fn writeColor(&mut self, value: Option<&Color>) {
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
            // Java: if (this.color != null)
               if self.color.is_some()
            {
               // Java: this.color = null;
               self.color = None;
            }
         }
      }
    }

    // Java: void forceWriteColor(Color value)
    pub fn forceWriteColor(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.color != null)
            if self.color.is_some()
         {
            // Java: this.color = null;
            self.color = None;
         }
      }
    }

}

impl KoAr for StyleMatrixReference {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for StyleMatrixReference { fn close(&mut self) { self.close(); } }

