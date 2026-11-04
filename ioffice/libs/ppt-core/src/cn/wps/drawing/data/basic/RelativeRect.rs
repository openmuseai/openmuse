#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code)]

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

const FIELD_LEFT: i32 = 1;
const FIELD_TOP: i32 = 2;
const FIELD_RIGHT: i32 = 3;
const FIELD_BOTTOM: i32 = 4;

// Java: public class RelativeRect implements KoAr, KoClosable
pub struct RelativeRect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RelativeRect parent
    pub(crate) parent: Option<Box<RelativeRect>>,
}

// Java: public class RelativeRect implements KoAr, KoClosable
impl RelativeRect {
    // Java: static protected static <Value1 extends RelativeRect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = RelativeRect::RelativeRect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static RelativeRect create()
    pub fn create() -> Self {
      // Java: return initialize(RelativeRect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static RelativeRect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(RelativeRect.class, bin);
      Self::initialize(bin)
    }

    // Java: RelativeRect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      RelativeRect { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: RelativeRect(KoBin bin)
    pub fn RelativeRect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      RelativeRect { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
      let boxed = self.bin_box.get();
      self.map.load(boxed.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin_with_zone(bin, self.zone);
    }

    // Java: void load(RelativeRect value)
    pub fn load_value(&mut self, value: Option<&RelativeRect>) {
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
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(RelativeRect o)
    pub fn equalsLax(&self, o: &RelativeRect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RelativeRect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &RelativeRect, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof RelativeRect)
      if true
      {
         // Java: RelativeRect dst = (RelativeRect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasLeft() || !dst.hasLeft()) && (!this.hasLeft() || dst.hasLeft()))
         else if (self.hasLeft() || !dst.hasLeft()) && (!self.hasLeft() || dst.hasLeft())
         {
            // Java: if (this.hasLeft() && dst.hasLeft() && this.left() != dst.left())
            if self.hasLeft() && dst.hasLeft() && self.left() != dst.left()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasTop() || !dst.hasTop()) && (!this.hasTop() || dst.hasTop()))
            else if (self.hasTop() || !dst.hasTop()) && (!self.hasTop() || dst.hasTop())
            {
               // Java: if (this.hasTop() && dst.hasTop() && this.top() != dst.top())
               if self.hasTop() && dst.hasTop() && self.top() != dst.top()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasRight() || !dst.hasRight()) && (!this.hasRight() || dst.hasRight()))
               else if (self.hasRight() || !dst.hasRight()) && (!self.hasRight() || dst.hasRight())
               {
                  // Java: if (this.hasRight() && dst.hasRight() && this.right() != dst.right())
                  if self.hasRight() && dst.hasRight() && self.right() != dst.right()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasBottom() || !dst.hasBottom()) && (!this.hasBottom() || dst.hasBottom()))
                  else if (self.hasBottom() || !dst.hasBottom()) && (!self.hasBottom() || dst.hasBottom())
                  {
                     // Java: return !this.hasBottom() || !dst.hasBottom() || this.bottom() == dst.bottom();
                     return !self.hasBottom() || !dst.hasBottom() || self.bottom() == dst.bottom();
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
      // Java: if (this.hasLeft())
      if self.hasLeft()
      {
         // Java: ret += Double.valueOf(this.left()).hashCode();
         let bits = self.left().to_bits();
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasTop())
      if self.hasTop()
      {
         // Java: ret += Double.valueOf(this.top()).hashCode();
         let bits = self.top().to_bits();
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasRight())
      if self.hasRight()
      {
         // Java: ret += Double.valueOf(this.right()).hashCode();
         let bits = self.right().to_bits();
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasBottom())
      if self.hasBottom()
      {
         // Java: ret += Double.valueOf(this.bottom()).hashCode();
         let bits = self.bottom().to_bits();
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
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
      self.saveBin(self.zone);
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
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
    }

    // Java: void follow(RelativeRect parent)
    pub fn follow(&mut self, mut parent: Box<RelativeRect>) {
      // Java: if (parent != null)
      {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(parent);
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

    // Java: RelativeRect parent()
    pub fn get_parent(&self) -> Option<&RelativeRect> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: double left()
    pub fn left(&self) -> f64 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_LEFT)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.left();
         return self.parent.as_ref().map_or(0.0f64, |p| p.left());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_LEFT);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasLeft()
    pub fn hasLeft(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasLeft();
      self.map.has(FIELD_LEFT) || self.parent.as_ref().map_or(false, |p| p.hasLeft())
    }

    // Java: void deleteLeft()
    pub fn deleteLeft(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_LEFT);
    }

    // Java: void putLeft(double value)
    pub fn putLeft(&mut self, value: f64) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_LEFT, Encode::from_double(value, self.zone));
    }

    // Java: double top()
    pub fn top(&self) -> f64 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_TOP)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.top();
         return self.parent.as_ref().map_or(0.0f64, |p| p.top());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_TOP);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasTop()
    pub fn hasTop(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasTop();
      self.map.has(FIELD_TOP) || self.parent.as_ref().map_or(false, |p| p.hasTop())
    }

    // Java: void deleteTop()
    pub fn deleteTop(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_TOP);
    }

    // Java: void putTop(double value)
    pub fn putTop(&mut self, value: f64) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_TOP, Encode::from_double(value, self.zone));
    }

    // Java: double right()
    pub fn right(&self) -> f64 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_RIGHT)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.right();
         return self.parent.as_ref().map_or(0.0f64, |p| p.right());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_RIGHT);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasRight()
    pub fn hasRight(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasRight();
      self.map.has(FIELD_RIGHT) || self.parent.as_ref().map_or(false, |p| p.hasRight())
    }

    // Java: void deleteRight()
    pub fn deleteRight(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_RIGHT);
    }

    // Java: void putRight(double value)
    pub fn putRight(&mut self, value: f64) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_RIGHT, Encode::from_double(value, self.zone));
    }

    // Java: double bottom()
    pub fn bottom(&self) -> f64 {
      // Java: if (!this.map.has(4))
      if !self.map.has(FIELD_BOTTOM)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.bottom();
         return self.parent.as_ref().map_or(0.0f64, |p| p.bottom());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(FIELD_BOTTOM);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasBottom()
    pub fn hasBottom(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasBottom();
      self.map.has(FIELD_BOTTOM) || self.parent.as_ref().map_or(false, |p| p.hasBottom())
    }

    // Java: void deleteBottom()
    pub fn deleteBottom(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(FIELD_BOTTOM);
    }

    // Java: void putBottom(double value)
    pub fn putBottom(&mut self, value: f64) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_BOTTOM, Encode::from_double(value, self.zone));
    }

}

impl KoAr for RelativeRect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { RelativeRect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RelativeRect::lastBin(self, var1) }
}

impl KoClosable for RelativeRect {
    fn close(&mut self) { RelativeRect::close(self); }
}
