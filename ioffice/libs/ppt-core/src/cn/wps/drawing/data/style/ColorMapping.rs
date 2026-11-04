#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

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

// Java: public class ColorMapping implements KoAr, KoClosable
pub struct ColorMapping {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ColorMapping parent
    pub(crate) parent: Option<Box<ColorMapping>>,
}

impl ColorMapping {
    // Java: static protected static <Value1 extends ColorMapping> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ColorMapping_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ColorMapping create()
    pub fn create() -> Self {
      // Java: return initialize(ColorMapping.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ColorMapping open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ColorMapping.class, bin);
      Self::initialize(bin)
    }

    // Java: ColorMapping()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: ColorMapping(KoBin bin)
    pub fn ColorMapping_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(ColorMapping value)
    pub fn load_value(&mut self, value: Option<&ColorMapping>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(ColorMapping o)
    pub fn equalsLax(&self, o: &ColorMapping) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ColorMapping) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ColorMapping, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ColorMapping)
        if true
      {
         // Java: ColorMapping dst = (ColorMapping)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBg1() || !dst.hasBg1()) && (!this.hasBg1() || dst.hasBg1()))
            else if (self.hasBg1() || !dst.hasBg1()) && (!self.hasBg1() || dst.hasBg1())
         {
            // Java: if (this.hasBg1() && dst.hasBg1() && this.bg1() != dst.bg1())
               if self.hasBg1() && dst.hasBg1() && self.bg1() != dst.bg1()
            {
               // Java: return false;
               return false;
            }
              // Java: else if ((this.hasBg2() || !dst.hasBg2()) && (!this.hasBg2() || dst.hasBg2()))
                 else if (self.hasBg2() || !dst.hasBg2()) && (!self.hasBg2() || dst.hasBg2())
              {
                 // Java: if (this.hasBg2() && dst.hasBg2() && this.bg2() != dst.bg2())
                    if self.hasBg2() && dst.hasBg2() && self.bg2() != dst.bg2()
                 {
                    // Java: return false;
                    return false;
                 }
                 // Java: else if ((this.hasTx1() || !dst.hasTx1()) && (!this.hasTx1() || dst.hasTx1()))
                    else if (self.hasTx1() || !dst.hasTx1()) && (!self.hasTx1() || dst.hasTx1())
                 {
                    // Java: if (this.hasTx1() && dst.hasTx1() && this.tx1() != dst.tx1())
                       if self.hasTx1() && dst.hasTx1() && self.tx1() != dst.tx1()
                    {
                       // Java: return false;
                       return false;
                    }
                    // Java: else if ((this.hasTx2() || !dst.hasTx2()) && (!this.hasTx2() || dst.hasTx2()))
                       else if (self.hasTx2() || !dst.hasTx2()) && (!self.hasTx2() || dst.hasTx2())
                    {
                       // Java: if (this.hasTx2() && dst.hasTx2() && this.tx2() != dst.tx2())
                          if self.hasTx2() && dst.hasTx2() && self.tx2() != dst.tx2()
                       {
                          // Java: return false;
                          return false;
                       }
                       // Java: else if ((this.hasAccent1() || !dst.hasAccent1()) && (!this.hasAccent1() || dst.hasAccent1()))
                          else if (self.hasAccent1() || !dst.hasAccent1()) && (!self.hasAccent1() || dst.hasAccent1())
                       {
                          // Java: if (this.hasAccent1() && dst.hasAccent1() && this.accent1() != dst.accent1())
                             if self.hasAccent1() && dst.hasAccent1() && self.accent1() != dst.accent1()
                          {
                             // Java: return false;
                             return false;
                          }
                          // Java: else if ((this.hasAccent2() || !dst.hasAccent2()) && (!this.hasAccent2() || dst.hasAccent2()))
                             else if (self.hasAccent2() || !dst.hasAccent2()) && (!self.hasAccent2() || dst.hasAccent2())
                          {
                             // Java: if (this.hasAccent2() && dst.hasAccent2() && this.accent2() != dst.accent2())
                                if self.hasAccent2() && dst.hasAccent2() && self.accent2() != dst.accent2()
                             {
                                // Java: return false;
                                return false;
                             }
                             // Java: else if ((this.hasAccent3() || !dst.hasAccent3()) && (!this.hasAccent3() || dst.hasAccent3()))
                                else if (self.hasAccent3() || !dst.hasAccent3()) && (!self.hasAccent3() || dst.hasAccent3())
                             {
                                // Java: if (this.hasAccent3() && dst.hasAccent3() && this.accent3() != dst.accent3())
                                   if self.hasAccent3() && dst.hasAccent3() && self.accent3() != dst.accent3()
                                {
                                   // Java: return false;
                                   return false;
                                }
                                // Java: else if ((this.hasAccent4() || !dst.hasAccent4()) && (!this.hasAccent4() || dst.hasAccent4()))
                                   else if (self.hasAccent4() || !dst.hasAccent4()) && (!self.hasAccent4() || dst.hasAccent4())
                                {
                                   // Java: if (this.hasAccent4() && dst.hasAccent4() && this.accent4() != dst.accent4())
                                      if self.hasAccent4() && dst.hasAccent4() && self.accent4() != dst.accent4()
                                   {
                                      // Java: return false;
                                      return false;
                                   }
                                   // Java: else if ((this.hasAccent5() || !dst.hasAccent5()) && (!this.hasAccent5() || dst.hasAccent5()))
                                      else if (self.hasAccent5() || !dst.hasAccent5()) && (!self.hasAccent5() || dst.hasAccent5())
                                   {
                                      // Java: if (this.hasAccent5() && dst.hasAccent5() && this.accent5() != dst.accent5())
                                         if self.hasAccent5() && dst.hasAccent5() && self.accent5() != dst.accent5()
                                      {
                                         // Java: return false;
                                         return false;
                                      }
                                      // Java: else if ((this.hasAccent6() || !dst.hasAccent6()) && (!this.hasAccent6() || dst.hasAccent6()))
                                         else if (self.hasAccent6() || !dst.hasAccent6()) && (!self.hasAccent6() || dst.hasAccent6())
                                      {
                                         // Java: if (this.hasAccent6() && dst.hasAccent6() && this.accent6() != dst.accent6())
                                            if self.hasAccent6() && dst.hasAccent6() && self.accent6() != dst.accent6()
                                         {
                                            // Java: return false;
                                            return false;
                                         }
                                         // Java: else if ((this.hasHlink() || !dst.hasHlink()) && (!this.hasHlink() || dst.hasHlink()))
                                            else if (self.hasHlink() || !dst.hasHlink()) && (!self.hasHlink() || dst.hasHlink())
                                         {
                                            // Java: if (this.hasHlink() && dst.hasHlink() && this.hlink() != dst.hlink())
                                               if self.hasHlink() && dst.hasHlink() && self.hlink() != dst.hlink()
                                            {
                                               // Java: return false;
                                               return false;
                                            }
                                            // Java: else if ((this.hasFolHlink() || !dst.hasFolHlink()) && (!this.hasFolHlink() || dst.hasFolHlink()))
                                               else if (self.hasFolHlink() || !dst.hasFolHlink()) && (!self.hasFolHlink() || dst.hasFolHlink())
                                            {
                                               // Java: return !this.hasFolHlink() || !dst.hasFolHlink() || this.folHlink() == dst.folHlink();
                                               return !self.hasFolHlink() || !dst.hasFolHlink() || self.folHlink() == dst.folHlink();
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
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBg1())
        if self.hasBg1()
      {
         // Java: ret += Integer.valueOf(this.bg1()).hashCode();
         ret = ret.wrapping_add(self.bg1());
      }
      // Java: if (this.hasBg2())
        if self.hasBg2()
      {
         // Java: ret += Integer.valueOf(this.bg2()).hashCode();
         ret = ret.wrapping_add(self.bg2());
      }
      // Java: if (this.hasTx1())
        if self.hasTx1()
      {
         // Java: ret += Integer.valueOf(this.tx1()).hashCode();
         ret = ret.wrapping_add(self.tx1());
      }
      // Java: if (this.hasTx2())
        if self.hasTx2()
      {
         // Java: ret += Integer.valueOf(this.tx2()).hashCode();
         ret = ret.wrapping_add(self.tx2());
      }
      // Java: if (this.hasAccent1())
        if self.hasAccent1()
      {
         // Java: ret += Integer.valueOf(this.accent1()).hashCode();
         ret = ret.wrapping_add(self.accent1());
      }
      // Java: if (this.hasAccent2())
        if self.hasAccent2()
      {
         // Java: ret += Integer.valueOf(this.accent2()).hashCode();
         ret = ret.wrapping_add(self.accent2());
      }
      // Java: if (this.hasAccent3())
        if self.hasAccent3()
      {
         // Java: ret += Integer.valueOf(this.accent3()).hashCode();
         ret = ret.wrapping_add(self.accent3());
      }
      // Java: if (this.hasAccent4())
        if self.hasAccent4()
      {
         // Java: ret += Integer.valueOf(this.accent4()).hashCode();
         ret = ret.wrapping_add(self.accent4());
      }
      // Java: if (this.hasAccent5())
        if self.hasAccent5()
      {
         // Java: ret += Integer.valueOf(this.accent5()).hashCode();
         ret = ret.wrapping_add(self.accent5());
      }
      // Java: if (this.hasAccent6())
        if self.hasAccent6()
      {
         // Java: ret += Integer.valueOf(this.accent6()).hashCode();
         ret = ret.wrapping_add(self.accent6());
      }
      // Java: if (this.hasHlink())
        if self.hasHlink()
      {
         // Java: ret += Integer.valueOf(this.hlink()).hashCode();
         ret = ret.wrapping_add(self.hlink());
      }
      // Java: if (this.hasFolHlink())
        if self.hasFolHlink()
      {
         // Java: ret += Integer.valueOf(this.folHlink()).hashCode();
         ret = ret.wrapping_add(self.folHlink());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.map.dirty();
      self.map.dirty()
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
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {}

    // Java: void follow(ColorMapping parent)
    pub fn follow(&mut self, parent: Option<&mut ColorMapping>) {
      // Java: if (parent != null)
        if let Some(p) = parent
      {
         // Java: parent.discard();
         let pz = p.zone;
         p.discard_with_zone(pz);
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent = parent;
         todo!()
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { let pz = p.zone; p.discard_with_zone(pz); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: ColorMapping parent()
    pub fn parent(&self) -> Option<&ColorMapping> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let z = self.zone;
      self.discard_with_zone(z);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int bg1()
    pub fn bg1(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.bg1();
         return self.parent.as_ref().map_or(0, |p| p.bg1());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasBg1()
    pub fn hasBg1(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBg1();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBg1())
    }

    // Java: void deleteBg1()
    pub fn deleteBg1(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putBg1(int value)
    pub fn putBg1(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: int bg2()
    pub fn bg2(&self) -> i32 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? 0 : this.parent.bg2();
         return self.parent.as_ref().map_or(0, |p| p.bg2());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasBg2()
    pub fn hasBg2(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasBg2();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasBg2())
    }

    // Java: void deleteBg2()
    pub fn deleteBg2(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putBg2(int value)
    pub fn putBg2(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: int tx1()
    pub fn tx1(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? 0 : this.parent.tx1();
         return self.parent.as_ref().map_or(0, |p| p.tx1());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasTx1()
    pub fn hasTx1(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasTx1();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasTx1())
    }

    // Java: void deleteTx1()
    pub fn deleteTx1(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putTx1(int value)
    pub fn putTx1(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_int(value, self.zone));
    }

    // Java: int tx2()
    pub fn tx2(&self) -> i32 {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? 0 : this.parent.tx2();
         return self.parent.as_ref().map_or(0, |p| p.tx2());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasTx2()
    pub fn hasTx2(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasTx2();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasTx2())
    }

    // Java: void deleteTx2()
    pub fn deleteTx2(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putTx2(int value)
    pub fn putTx2(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_int(value, self.zone));
    }

    // Java: int accent1()
    pub fn accent1(&self) -> i32 {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? 0 : this.parent.accent1();
         return self.parent.as_ref().map_or(0, |p| p.accent1());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasAccent1()
    pub fn hasAccent1(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasAccent1();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasAccent1())
    }

    // Java: void deleteAccent1()
    pub fn deleteAccent1(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putAccent1(int value)
    pub fn putAccent1(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_int(value, self.zone));
    }

    // Java: int accent2()
    pub fn accent2(&self) -> i32 {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? 0 : this.parent.accent2();
         return self.parent.as_ref().map_or(0, |p| p.accent2());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasAccent2()
    pub fn hasAccent2(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasAccent2();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasAccent2())
    }

    // Java: void deleteAccent2()
    pub fn deleteAccent2(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putAccent2(int value)
    pub fn putAccent2(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_int(value, self.zone));
    }

    // Java: int accent3()
    pub fn accent3(&self) -> i32 {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? 0 : this.parent.accent3();
         return self.parent.as_ref().map_or(0, |p| p.accent3());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasAccent3()
    pub fn hasAccent3(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasAccent3();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasAccent3())
    }

    // Java: void deleteAccent3()
    pub fn deleteAccent3(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putAccent3(int value)
    pub fn putAccent3(&mut self, value: i32) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_int(value, self.zone));
    }

    // Java: int accent4()
    pub fn accent4(&self) -> i32 {
      // Java: if (!this.map.has(8))
        if !self.map.has(8)
      {
         // Java: return this.parent == null ? 0 : this.parent.accent4();
         return self.parent.as_ref().map_or(0, |p| p.accent4());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasAccent4()
    pub fn hasAccent4(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasAccent4();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasAccent4())
    }

    // Java: void deleteAccent4()
    pub fn deleteAccent4(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }

    // Java: void putAccent4(int value)
    pub fn putAccent4(&mut self, value: i32) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(8, Encode::from_int(value, self.zone));
    }

    // Java: int accent5()
    pub fn accent5(&self) -> i32 {
      // Java: if (!this.map.has(9))
        if !self.map.has(9)
      {
         // Java: return this.parent == null ? 0 : this.parent.accent5();
         return self.parent.as_ref().map_or(0, |p| p.accent5());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasAccent5()
    pub fn hasAccent5(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasAccent5();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasAccent5())
    }

    // Java: void deleteAccent5()
    pub fn deleteAccent5(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putAccent5(int value)
    pub fn putAccent5(&mut self, value: i32) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(9, Encode::from_int(value, self.zone));
    }

    // Java: int accent6()
    pub fn accent6(&self) -> i32 {
      // Java: if (!this.map.has(10))
        if !self.map.has(10)
      {
         // Java: return this.parent == null ? 0 : this.parent.accent6();
         return self.parent.as_ref().map_or(0, |p| p.accent6());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasAccent6()
    pub fn hasAccent6(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasAccent6();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasAccent6())
    }

    // Java: void deleteAccent6()
    pub fn deleteAccent6(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putAccent6(int value)
    pub fn putAccent6(&mut self, value: i32) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_int(value, self.zone));
    }

    // Java: int hlink()
    pub fn hlink(&self) -> i32 {
      // Java: if (!this.map.has(11))
        if !self.map.has(11)
      {
         // Java: return this.parent == null ? 0 : this.parent.hlink();
         return self.parent.as_ref().map_or(0, |p| p.hlink());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(11);
         let bin = self.map.byKey(11);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasHlink()
    pub fn hasHlink(&self) -> bool {
      // Java: return this.map.has(11) || this.parent != null && this.parent.hasHlink();
      self.map.has(11) || self.parent.as_ref().map_or(false, |p| p.hasHlink())
    }

    // Java: void deleteHlink()
    pub fn deleteHlink(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
    }

    // Java: void putHlink(int value)
    pub fn putHlink(&mut self, value: i32) {
      // Java: this.map.put(11, KoBin.Encode.from(value, this.zone));
      self.map.put(11, Encode::from_int(value, self.zone));
    }

    // Java: int folHlink()
    pub fn folHlink(&self) -> i32 {
      // Java: if (!this.map.has(12))
        if !self.map.has(12)
      {
         // Java: return this.parent == null ? 0 : this.parent.folHlink();
         return self.parent.as_ref().map_or(0, |p| p.folHlink());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(12);
         let bin = self.map.byKey(12);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasFolHlink()
    pub fn hasFolHlink(&self) -> bool {
      // Java: return this.map.has(12) || this.parent != null && this.parent.hasFolHlink();
      self.map.has(12) || self.parent.as_ref().map_or(false, |p| p.hasFolHlink())
    }

    // Java: void deleteFolHlink()
    pub fn deleteFolHlink(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
    }

    // Java: void putFolHlink(int value)
    pub fn putFolHlink(&mut self, value: i32) {
      // Java: this.map.put(12, KoBin.Encode.from(value, this.zone));
      self.map.put(12, Encode::from_int(value, self.zone));
    }

}

impl KoAr for ColorMapping {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for ColorMapping { fn close(&mut self) { self.close(); } }
