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

// Java: public class RecolorEntryColor implements KoAr, KoClosable
pub struct RecolorEntryColor {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color toColor
    pub(crate) toColor: Option<Box<dyn KoAr>>,
    // Java field: Color fromColor
    pub(crate) fromColor: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RecolorEntryColor parent
    pub(crate) parent: Option<Box<RecolorEntryColor>>,
}

// Java: public class RecolorEntryColor implements KoAr, KoClosable
impl RecolorEntryColor {
    // Java: static protected static <Value1 extends RecolorEntryColor> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::RecolorEntryColor_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static RecolorEntryColor create()
    pub fn create() -> Self {
      // Java: return initialize(RecolorEntryColor.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static RecolorEntryColor open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(RecolorEntryColor.class, bin);
      Self::initialize(bin)
    }

    // Java: RecolorEntryColor()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), toColor: None, fromColor: None, parent: None,
      }
    }

    // Java: RecolorEntryColor(KoBin bin)
    pub fn RecolorEntryColor_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), toColor: None, fromColor: None, parent: None,
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

    // Java: void load(RecolorEntryColor value)
    pub fn load_value(&mut self, value: Option<&RecolorEntryColor>) {
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
      // Java: if (this.toColor != null && this.toColor.dirty())
        if self.toColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.toColor.saveBin(zone));
         let saved = self.toColor.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: if (this.fromColor != null && this.fromColor.dirty())
        if self.fromColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(4, this.fromColor.saveBin(zone));
         let saved = self.fromColor.as_mut().unwrap().saveBin(zone);
         self.map.put(4, saved);
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

    // Java: boolean equalsLax(RecolorEntryColor o)
    pub fn equalsLax(&self, o: &RecolorEntryColor) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RecolorEntryColor) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &RecolorEntryColor, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof RecolorEntryColor)
        if true
      {
         let dst = o;
         // Java: RecolorEntryColor dst = (RecolorEntryColor)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFDoRecolor() || !dst.hasFDoRecolor()) && (!this.hasFDoRecolor() || dst.hasFDoRecolor()))
            else if (self.hasFDoRecolor() || !dst.hasFDoRecolor()) && (!self.hasFDoRecolor() || dst.hasFDoRecolor())
         {
            // Java: if (this.hasFDoRecolor() && dst.hasFDoRecolor() && this.isFDoRecolor() != dst.isFDoRecolor())
                if self.hasFDoRecolor() && dst.hasFDoRecolor() && self.isFDoRecolor() != dst.isFDoRecolor()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasToColor() || !dst.hasToColor()) && (!this.hasToColor() || dst.hasToColor()))
                else if (self.hasToColor() || !dst.hasToColor()) && (!self.hasToColor() || dst.hasToColor())
            {
               // Java: if (this.hasToColor() && dst.hasToColor())
                    if self.hasToColor() && dst.hasToColor()
               {
                  // Java: if (isStrict && !this.toColor().equals(dst.toColor()))
                        if isStrict && !self.get_toColor().equals(dst.get_toColor())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.toColor().equalsLax(dst.toColor()))
                        if !isStrict && !self.get_toColor().equalsLax(dst.get_toColor())
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasToIndex() || !dst.hasToIndex()) && (!this.hasToIndex() || dst.hasToIndex()))
                    if (self.hasToIndex() || !dst.hasToIndex()) && (!self.hasToIndex() || dst.hasToIndex())
               {
                  // Java: if (this.hasToIndex() && dst.hasToIndex() && this.toIndex() != dst.toIndex())
                        if self.hasToIndex() && dst.hasToIndex() && self.toIndex() != dst.toIndex()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasFromColor() || !dst.hasFromColor()) && (!this.hasFromColor() || dst.hasFromColor()))
                        else if (self.hasFromColor() || !dst.hasFromColor()) && (!self.hasFromColor() || dst.hasFromColor())
                  {
                     // Java: if (this.hasFromColor() && dst.hasFromColor())
                            if self.hasFromColor() && dst.hasFromColor()
                     {
                        // Java: if (isStrict && !this.fromColor().equals(dst.fromColor()))
                                if isStrict && !self.get_fromColor().equals(dst.get_fromColor())
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.fromColor().equalsLax(dst.fromColor()))
                                if !isStrict && !self.get_fromColor().equalsLax(dst.get_fromColor())
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
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasFDoRecolor())
        if self.hasFDoRecolor()
      {
         // Java: ret += Boolean.valueOf(this.isFDoRecolor()).hashCode();
         ret = ret.wrapping_add(if self.isFDoRecolor() { 1231 } else { 1237 });
      }
      // Java: if (this.hasToColor())
        if self.hasToColor()
      {
         // Java: ret += this.toColor().hashCode();
         ret = ret.wrapping_add(todo!("hashCode"));
      }
      // Java: if (this.hasToIndex())
        if self.hasToIndex()
      {
         // Java: ret += Integer.valueOf(this.toIndex()).hashCode();
         ret = ret.wrapping_add(self.toIndex());
      }
      // Java: if (this.hasFromColor())
        if self.hasFromColor()
      {
         // Java: ret += this.fromColor().hashCode();
         ret = ret.wrapping_add(todo!("hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.toColor != null && this.toColor.dirty())
        if self.toColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.fromColor != null && this.fromColor.dirty() ? true : this.map.dirty();
         return self.fromColor.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
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
      // Java: this.toColor = null;
      self.toColor = None;
      // Java: this.fromColor = null;
      self.fromColor = None;
    }

    // Java: void follow(RecolorEntryColor parent)
    pub fn follow(&mut self, parent: Option<Box<RecolorEntryColor>>) {
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

    // Java: RecolorEntryColor parent()
    pub fn get_parent(&self) -> Option<&RecolorEntryColor> {
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

    // Java: boolean isFDoRecolor()
    pub fn isFDoRecolor(&self) -> bool {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? false : this.parent.isFDoRecolor();
         return self.parent.as_ref().map_or(false, |p| p.isFDoRecolor());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFDoRecolor()
    pub fn hasFDoRecolor(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasFDoRecolor();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasFDoRecolor())
    }

    // Java: void deleteFDoRecolor()
    pub fn deleteFDoRecolor(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putFDoRecolor(boolean value)
    pub fn putFDoRecolor(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_bool(value, self.zone));
    }

    // Java: Color toColor()
    pub fn get_toColor(&self) -> &dyn KoAr {
      // Java: if (this.toColor == null)
        if self.toColor.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.toColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasToColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasToColor())
         {
            // Java: this.toColor = this.parent.toColor();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.toColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.toColor;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasToColor()
    pub fn hasToColor(&self) -> bool {
      // Java: return this.map.has(2) || this.toColor != null && this.toColor.dirty() || this.parent != null && this.parent.hasToColor();
      self.map.has(2) || self.toColor.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasToColor())
    }

    // Java: void deleteToColor()
    pub fn deleteToColor(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.toColor != null)
        if self.toColor.is_some()
      {
         // Java: this.toColor = null;
         self.toColor = None;
      }
    }

    // Java: void writeToColor(Color value)
    pub fn writeToColor(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.toColor != null)
                if self.toColor.is_some()
            {
               // Java: this.toColor = null;
               self.toColor = None;
            }
         }
      }
    }

    // Java: void forceWriteToColor(Color value)
    pub fn forceWriteToColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.toColor != null)
            if self.toColor.is_some()
         {
            // Java: this.toColor = null;
            self.toColor = None;
         }
      }
    }

    // Java: int toIndex()
    pub fn toIndex(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? 0 : this.parent.toIndex();
         return self.parent.as_ref().map_or(0, |p| p.toIndex());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasToIndex()
    pub fn hasToIndex(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasToIndex();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasToIndex())
    }

    // Java: void deleteToIndex()
    pub fn deleteToIndex(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putToIndex(int value)
    pub fn putToIndex(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_int(value, self.zone));
    }

    // Java: Color fromColor()
    pub fn get_fromColor(&self) -> &dyn KoAr {
      // Java: if (this.fromColor == null)
        if self.fromColor.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.fromColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasFromColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasFromColor())
         {
            // Java: this.fromColor = this.parent.fromColor();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.fromColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.fromColor;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasFromColor()
    pub fn hasFromColor(&self) -> bool {
      // Java: return this.map.has(4) || this.fromColor != null && this.fromColor.dirty() || this.parent != null && this.parent.hasFromColor();
      self.map.has(4) || self.fromColor.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFromColor())
    }

    // Java: void deleteFromColor()
    pub fn deleteFromColor(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.fromColor != null)
        if self.fromColor.is_some()
      {
         // Java: this.fromColor = null;
         self.fromColor = None;
      }
    }

    // Java: void writeFromColor(Color value)
    pub fn writeFromColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.fromColor != null)
                if self.fromColor.is_some()
            {
               // Java: this.fromColor = null;
               self.fromColor = None;
            }
         }
      }
    }

    // Java: void forceWriteFromColor(Color value)
    pub fn forceWriteFromColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.fromColor != null)
            if self.fromColor.is_some()
         {
            // Java: this.fromColor = null;
            self.fromColor = None;
         }
      }
    }

}

impl KoAr for RecolorEntryColor {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { RecolorEntryColor::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RecolorEntryColor::lastBin(self, var1) }
}

impl KoClosable for RecolorEntryColor {
    fn close(&mut self) { RecolorEntryColor::close(self); }
}
