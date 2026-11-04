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

const FIELD_KX: i32 = 1;
const FIELD_KY: i32 = 2;
const FIELD_SX: i32 = 3;
const FIELD_SY: i32 = 4;
const FIELD_TX: i32 = 5;
const FIELD_TY: i32 = 6;

// Java: public class XfrmEffect implements KoAr, KoClosable
pub struct XfrmEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: XfrmEffect parent
    pub(crate) parent: Option<Box<XfrmEffect>>,
}

// Java: public class XfrmEffect implements KoAr, KoClosable
impl XfrmEffect {
    // Java: static protected static <Value1 extends XfrmEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::XfrmEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      return inst;
    }

    // Java: static XfrmEffect create()
    pub fn create() -> Self {
      // Java: return initialize(XfrmEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static XfrmEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(XfrmEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: XfrmEffect()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(),
         parent: None,
      }
    }

    // Java: XfrmEffect(KoBin bin)
    pub fn XfrmEffect_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.map = new KpMap(bin);
         map: KpMap::KpMap_from(bin),
         zone: KoZone::defaultZone(),
         parent: None,
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

    // Java: void load(XfrmEffect value)
    pub fn load_value(&mut self, value: Option<&XfrmEffect>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin.as_ref(), zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(XfrmEffect o)
    pub fn equalsLax(&self, o: &XfrmEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &XfrmEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &XfrmEffect, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof XfrmEffect)
      let dst = o;
      {
         // Java: XfrmEffect dst = (XfrmEffect)o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasKx() || !dst.hasKx()) && (!this.hasKx() || dst.hasKx()))
         else if (self.hasKx() || !dst.hasKx()) && (!self.hasKx() || dst.hasKx())
         {
            // Java: if (this.hasKx() && dst.hasKx() && this.kx() != dst.kx())
            if self.hasKx() && dst.hasKx() && self.kx() != dst.kx()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasKy() || !dst.hasKy()) && (!this.hasKy() || dst.hasKy()))
            else if (self.hasKy() || !dst.hasKy()) && (!self.hasKy() || dst.hasKy())
            {
               // Java: if (this.hasKy() && dst.hasKy() && this.ky() != dst.ky())
               if self.hasKy() && dst.hasKy() && self.ky() != dst.ky()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasSx() || !dst.hasSx()) && (!this.hasSx() || dst.hasSx()))
               else if (self.hasSx() || !dst.hasSx()) && (!self.hasSx() || dst.hasSx())
               {
                  // Java: if (this.hasSx() && dst.hasSx() && this.sx() != dst.sx())
                  if self.hasSx() && dst.hasSx() && self.sx() != dst.sx()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasSy() || !dst.hasSy()) && (!this.hasSy() || dst.hasSy()))
                  else if (self.hasSy() || !dst.hasSy()) && (!self.hasSy() || dst.hasSy())
                  {
                     // Java: if (this.hasSy() && dst.hasSy() && this.sy() != dst.sy())
                     if self.hasSy() && dst.hasSy() && self.sy() != dst.sy()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasTx() || !dst.hasTx()) && (!this.hasTx() || dst.hasTx()))
                     else if (self.hasTx() || !dst.hasTx()) && (!self.hasTx() || dst.hasTx())
                     {
                        // Java: if (this.hasTx() && dst.hasTx() && this.tx() != dst.tx())
                        if self.hasTx() && dst.hasTx() && self.tx() != dst.tx()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasTy() || !dst.hasTy()) && (!this.hasTy() || dst.hasTy()))
                        else if (self.hasTy() || !dst.hasTy()) && (!self.hasTy() || dst.hasTy())
                        {
                           // Java: return !this.hasTy() || !dst.hasTy() || this.ty() == dst.ty();
                           return !self.hasTy() || !dst.hasTy() || self.ty() == dst.ty();
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
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasKx())
      if self.hasKx()
      {
         // Java: ret += Integer.valueOf(this.kx()).hashCode();
         ret = ret.wrapping_add(self.kx());
      }
      // Java: if (this.hasKy())
      if self.hasKy()
      {
         // Java: ret += Integer.valueOf(this.ky()).hashCode();
         ret = ret.wrapping_add(self.ky());
      }
      // Java: if (this.hasSx())
      if self.hasSx()
      {
         // Java: ret += Double.valueOf(this.sx()).hashCode();
         let bits = self.sx().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasSy())
      if self.hasSy()
      {
         // Java: ret += Double.valueOf(this.sy()).hashCode();
         let bits = self.sy().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasTx())
      if self.hasTx()
      {
         // Java: ret += Integer.valueOf(this.tx()).hashCode();
         ret = ret.wrapping_add(self.tx());
      }
      // Java: if (this.hasTy())
      if self.hasTy()
      {
         // Java: ret += Integer.valueOf(this.ty()).hashCode();
         ret = ret.wrapping_add(self.ty());
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
    }

    // Java: void follow(XfrmEffect parent)
    pub fn follow(&mut self, parent: Option<Box<XfrmEffect>>) {
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

    // Java: XfrmEffect parent()
    pub fn get_parent(&self) -> Option<&XfrmEffect> {
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

    // Java: int kx()
    pub fn kx(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_KX)
      {
         // Java: return this.parent == null ? 0 : this.parent.kx();
         return self.parent.as_ref().map_or(0, |p| p.kx());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_KX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasKx()
    pub fn hasKx(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasKx();
      self.map.has(FIELD_KX) || self.parent.as_ref().map_or(false, |p| p.hasKx())
    }

    // Java: void deleteKx()
    pub fn deleteKx(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_KX);
    }

    // Java: void putKx(int value)
    pub fn putKx(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_KX, Encode::from_int(value, self.zone));
    }

    // Java: int ky()
    pub fn ky(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_KY)
      {
         // Java: return this.parent == null ? 0 : this.parent.ky();
         return self.parent.as_ref().map_or(0, |p| p.ky());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_KY);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasKy()
    pub fn hasKy(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasKy();
      self.map.has(FIELD_KY) || self.parent.as_ref().map_or(false, |p| p.hasKy())
    }

    // Java: void deleteKy()
    pub fn deleteKy(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_KY);
    }

    // Java: void putKy(int value)
    pub fn putKy(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_KY, Encode::from_int(value, self.zone));
    }

    // Java: double sx()
    pub fn sx(&self) -> f64 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_SX)
      {
         // Java: return this.parent == null ? (double)1.0F : this.parent.sx();
         return self.parent.as_ref().map_or(1.0_f64, |p| p.sx());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_SX);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSx()
    pub fn hasSx(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasSx();
      self.map.has(FIELD_SX) || self.parent.as_ref().map_or(false, |p| p.hasSx())
    }

    // Java: void deleteSx()
    pub fn deleteSx(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_SX);
    }

    // Java: void putSx(double value)
    pub fn putSx(&mut self, value: f64) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SX, Encode::from_double(value, self.zone));
    }

    // Java: double sy()
    pub fn sy(&self) -> f64 {
      // Java: if (!this.map.has(4))
      if !self.map.has(FIELD_SY)
      {
         // Java: return this.parent == null ? (double)1.0F : this.parent.sy();
         return self.parent.as_ref().map_or(1.0_f64, |p| p.sy());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(FIELD_SY);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSy()
    pub fn hasSy(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasSy();
      self.map.has(FIELD_SY) || self.parent.as_ref().map_or(false, |p| p.hasSy())
    }

    // Java: void deleteSy()
    pub fn deleteSy(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(FIELD_SY);
    }

    // Java: void putSy(double value)
    pub fn putSy(&mut self, value: f64) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SY, Encode::from_double(value, self.zone));
    }

    // Java: int tx()
    pub fn tx(&self) -> i32 {
      // Java: if (!this.map.has(5))
      if !self.map.has(FIELD_TX)
      {
         // Java: return this.parent == null ? 0 : this.parent.tx();
         return self.parent.as_ref().map_or(0, |p| p.tx());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(FIELD_TX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTx()
    pub fn hasTx(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasTx();
      self.map.has(FIELD_TX) || self.parent.as_ref().map_or(false, |p| p.hasTx())
    }

    // Java: void deleteTx()
    pub fn deleteTx(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(FIELD_TX);
    }

    // Java: void putTx(int value)
    pub fn putTx(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_TX, Encode::from_int(value, self.zone));
    }

    // Java: int ty()
    pub fn ty(&self) -> i32 {
      // Java: if (!this.map.has(6))
      if !self.map.has(FIELD_TY)
      {
         // Java: return this.parent == null ? 0 : this.parent.ty();
         return self.parent.as_ref().map_or(0, |p| p.ty());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(FIELD_TY);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTy()
    pub fn hasTy(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasTy();
      self.map.has(FIELD_TY) || self.parent.as_ref().map_or(false, |p| p.hasTy())
    }

    // Java: void deleteTy()
    pub fn deleteTy(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(FIELD_TY);
    }

    // Java: void putTy(int value)
    pub fn putTy(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_TY, Encode::from_int(value, self.zone));
    }

}

impl KoAr for XfrmEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { XfrmEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { XfrmEffect::lastBin(self, var1) }
}

impl KoClosable for XfrmEffect {
    fn close(&mut self) { XfrmEffect::close(self); }
}
