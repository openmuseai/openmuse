#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
// TODO: Color not yet available
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

const FIELD_PRST: i32 = 1;
const FIELD_FG_CLR: i32 = 2;
const FIELD_BG_CLR: i32 = 3;

// Java: public class PatternFill implements KoAr, KoClosable
pub struct PatternFill {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color fgClr
    pub(crate) fgClr: Option<Box<dyn KoAr>>,
    // Java field: Color bgClr
    pub(crate) bgClr: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: PatternFill parent
    pub(crate) parent: Option<Box<PatternFill>>,
}

// Java: public class PatternFill implements KoAr, KoClosable
impl PatternFill {
    // Java: static protected static <Value1 extends PatternFill> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = PatternFill::PatternFill_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static PatternFill create()
    pub fn create() -> Self {
      // Java: return initialize(PatternFill.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static PatternFill open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(PatternFill.class, bin);
      Self::initialize(bin)
    }

    // Java: PatternFill()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      PatternFill { zone: KoZone::defaultZone(), fgClr: None, bgClr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: PatternFill(KoBin bin)
    pub fn PatternFill_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      PatternFill { zone: KoZone::defaultZone(), fgClr: None, bgClr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(PatternFill value)
    pub fn load_value(&mut self, value: Option<&PatternFill>) {
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
      // Java: if (this.fgClr != null && this.fgClr.dirty())
      if self.fgClr.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(2, this.fgClr.saveBin(zone));
      }
      // Java: if (this.bgClr != null && this.bgClr.dirty())
      if self.bgClr.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(3, this.bgClr.saveBin(zone));
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(PatternFill o)
    pub fn equalsLax(&self, o: &PatternFill) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PatternFill) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PatternFill, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof PatternFill)
      if true
      {
         // Java: PatternFill dst = (PatternFill)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasPrst() || !dst.hasPrst()) && (!this.hasPrst() || dst.hasPrst()))
         else if (self.hasPrst() || !dst.hasPrst()) && (!self.hasPrst() || dst.hasPrst())
         {
            // Java: if (this.hasPrst() && dst.hasPrst() && this.prst() != dst.prst())
            if self.hasPrst() && dst.hasPrst() && self.prst() != dst.prst()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasFgClr() || !dst.hasFgClr()) && (!this.hasFgClr() || dst.hasFgClr()))
            else if (self.hasFgClr() || !dst.hasFgClr()) && (!self.hasFgClr() || dst.hasFgClr())
            {
               // Java: if (this.hasFgClr() && dst.hasFgClr())
               if self.hasFgClr() && dst.hasFgClr()
               {
                  // Java: if (isStrict && !this.fgClr().equals(dst.fgClr()))
                  if isStrict
                  {
                     // Java: return false;
                  }
                  // Java: if (!isStrict && !this.fgClr().equalsLax(dst.fgClr()))
                  if !isStrict
                  {
                     // Java: return false;
                  }
               }
               // Java: if ((this.hasBgClr() || !dst.hasBgClr()) && (!this.hasBgClr() || dst.hasBgClr()))
               if (self.hasBgClr() || !dst.hasBgClr()) && (!self.hasBgClr() || dst.hasBgClr())
               {
                  // Java: if (this.hasBgClr() && dst.hasBgClr())
                  if self.hasBgClr() && dst.hasBgClr()
                  {
                     // Java: if (isStrict && !this.bgClr().equals(dst.bgClr()))
                     if isStrict
                     {
                        // Java: return false;
                     }
                     // Java: if (!isStrict && !this.bgClr().equalsLax(dst.bgClr()))
                     if !isStrict
                     {
                        // Java: return false;
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasPrst())
      if self.hasPrst()
      {
         // Java: ret += Integer.valueOf(this.prst()).hashCode();
         ret = ret.wrapping_add(self.prst());
      }
      // Java: if (this.hasFgClr())
      if self.hasFgClr()
      {
         // Java: ret += this.fgClr().hashCode();
      }
      // Java: if (this.hasBgClr())
      if self.hasBgClr()
      {
         // Java: ret += this.bgClr().hashCode();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.fgClr != null && this.fgClr.dirty())
      if self.fgClr.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.bgClr != null && this.bgClr.dirty() ? true : this.map.dirty();
         if self.bgClr.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
      }
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
      // Java: this.fgClr = null;
      self.fgClr = None;
      // Java: this.bgClr = null;
      self.bgClr = None;
    }

    // Java: void follow(PatternFill parent)
    pub fn follow(&mut self, mut parent: Box<PatternFill>) {
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

    // Java: PatternFill parent()
    pub fn get_parent(&self) -> Option<&PatternFill> {
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

    // Java: int prst()
    pub fn prst(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_PRST)
      {
         // Java: return this.parent == null ? 22 : this.parent.prst();
         return self.parent.as_ref().map_or(22, |p| p.prst());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_PRST);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPrst()
    pub fn hasPrst(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasPrst();
      self.map.has(FIELD_PRST) || self.parent.as_ref().map_or(false, |p| p.hasPrst())
    }

    // Java: void deletePrst()
    pub fn deletePrst(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_PRST);
    }

    // Java: void putPrst(int value)
    pub fn putPrst(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_PRST, Encode::from_int(value, self.zone));
    }

    // Java: Color fgClr()
    pub fn fgClr(&self) -> &dyn KoAr {
      // Java: if (this.fgClr == null)
      // Java: if (this.map.has(2))
      // Java: KoBin bin = this.map.byKey(2);
      // Java: this.fgClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasFgClr())
      // Java: this.fgClr = this.parent.fgClr();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.fgClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.fgClr;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasFgClr()
    pub fn hasFgClr(&self) -> bool {
      // Java: return this.map.has(2) || this.fgClr != null && this.fgClr.dirty() || this.parent != null && this.parent.hasFgClr();
      self.map.has(FIELD_FG_CLR) || self.fgClr.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFgClr())
    }

    // Java: void deleteFgClr()
    pub fn deleteFgClr(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_FG_CLR);
      // Java: if (this.fgClr != null)
      if self.fgClr.is_some()
      {
         // Java: this.fgClr = null;
         self.fgClr = None;
      }
    }

    // Java: void writeFgClr(Color value)
    pub fn writeFgClr(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(FIELD_FG_CLR, bin);
            // Java: if (this.fgClr != null)
            if self.fgClr.is_some()
            {
               // Java: this.fgClr = null;
               self.fgClr = None;
            }
         }
      }
    }

    // Java: void forceWriteFgClr(Color value)
    pub fn forceWriteFgClr(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(FIELD_FG_CLR, value.lastBin(self.zone));
         // Java: if (this.fgClr != null)
         if self.fgClr.is_some()
         {
            // Java: this.fgClr = null;
            self.fgClr = None;
         }
      }
    }

    // Java: Color bgClr()
    pub fn bgClr(&self) -> &dyn KoAr {
      // Java: if (this.bgClr == null)
      // Java: if (this.map.has(3))
      // Java: KoBin bin = this.map.byKey(3);
      // Java: this.bgClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasBgClr())
      // Java: this.bgClr = this.parent.bgClr();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.bgClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.bgClr;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasBgClr()
    pub fn hasBgClr(&self) -> bool {
      // Java: return this.map.has(3) || this.bgClr != null && this.bgClr.dirty() || this.parent != null && this.parent.hasBgClr();
      self.map.has(FIELD_BG_CLR) || self.bgClr.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBgClr())
    }

    // Java: void deleteBgClr()
    pub fn deleteBgClr(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_BG_CLR);
      // Java: if (this.bgClr != null)
      if self.bgClr.is_some()
      {
         // Java: this.bgClr = null;
         self.bgClr = None;
      }
    }

    // Java: void writeBgClr(Color value)
    pub fn writeBgClr(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(FIELD_BG_CLR, bin);
            // Java: if (this.bgClr != null)
            if self.bgClr.is_some()
            {
               // Java: this.bgClr = null;
               self.bgClr = None;
            }
         }
      }
    }

    // Java: void forceWriteBgClr(Color value)
    pub fn forceWriteBgClr(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(FIELD_BG_CLR, value.lastBin(self.zone));
         // Java: if (this.bgClr != null)
         if self.bgClr.is_some()
         {
            // Java: this.bgClr = null;
            self.bgClr = None;
         }
      }
    }

}

impl KoAr for PatternFill {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { PatternFill::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { PatternFill::lastBin(self, var1) }
}

impl KoClosable for PatternFill {
    fn close(&mut self) { PatternFill::close(self); }
}
