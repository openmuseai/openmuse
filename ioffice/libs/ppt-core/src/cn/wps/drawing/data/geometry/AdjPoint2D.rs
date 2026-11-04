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

// Java: public class AdjPoint2D implements KoAr, KoClosable
pub struct AdjPoint2D {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjCoordinate x
    pub(crate) x: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate y
    pub(crate) y: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AdjPoint2D parent
    pub(crate) parent: Option<Box<AdjPoint2D>>,
}

// Java: public class AdjPoint2D implements KoAr, KoClosable
impl AdjPoint2D {
    // Java: static protected static <Value1 extends AdjPoint2D> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::AdjPoint2D_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static AdjPoint2D create()
    pub fn create() -> Self {
      // Java: return initialize(AdjPoint2D.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static AdjPoint2D open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(AdjPoint2D.class, bin);
      Self::initialize(bin)
    }

    // Java: AdjPoint2D()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), x: None, y: None, parent: None,
      }
    }

    // Java: AdjPoint2D(KoBin bin)
    pub fn AdjPoint2D_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), x: None, y: None, parent: None,
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

    // Java: void load(AdjPoint2D value)
    pub fn load_value(&mut self, value: Option<&AdjPoint2D>) {
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
      // Java: if (this.x != null && this.x.dirty())
        if self.x.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(1, this.x.saveBin(zone));
         let saved = self.x.as_mut().unwrap().saveBin(zone);
         self.map.put(1, saved);
      }
      // Java: if (this.y != null && this.y.dirty())
        if self.y.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.y.saveBin(zone));
         let saved = self.y.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
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

    // Java: boolean equalsLax(AdjPoint2D o)
    pub fn equalsLax(&self, o: &AdjPoint2D) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AdjPoint2D) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AdjPoint2D, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof AdjPoint2D)
        if true
      {
          let dst = o;
         // Java: AdjPoint2D dst = (AdjPoint2D)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasX() || !dst.hasX()) && (!this.hasX() || dst.hasX()))
            else if (self.hasX() || !dst.hasX()) && (!self.hasX() || dst.hasX())
         {
            // Java: if (this.hasX() && dst.hasX())
                if self.hasX() && dst.hasX()
            {
               // Java: if (isStrict && !this.x().equals(dst.x()))
                    if todo!("KoAr equals/equalsLax")

               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.x().equalsLax(dst.x()))
                    if todo!("KoAr equals/equalsLax")

               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasY() || !dst.hasY()) && (!this.hasY() || dst.hasY()))
                if (self.hasY() || !dst.hasY()) && (!self.hasY() || dst.hasY())
            {
               // Java: if (this.hasY() && dst.hasY())
                    if self.hasY() && dst.hasY()
               {
                  // Java: if (isStrict && !this.y().equals(dst.y()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.y().equalsLax(dst.y()))
                        if todo!("KoAr equals/equalsLax")

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
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasX())
        if self.hasX()
      {
         // Java: ret += this.x().hashCode();
         ret = ret.wrapping_add(todo!("x hashCode"));
      }
      // Java: if (this.hasY())
        if self.hasY()
      {
         // Java: ret += this.y().hashCode();
         ret = ret.wrapping_add(todo!("y hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.x != null && this.x.dirty())
        if self.x.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.y != null && this.y.dirty() ? true : this.map.dirty();
         return self.y.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
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
      // Java: this.x = null;
      self.x = None;
      // Java: this.y = null;
      self.y = None;
    }

    // Java: void follow(AdjPoint2D parent)
    pub fn follow(&mut self, parent: Option<Box<AdjPoint2D>>) {
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

    // Java: AdjPoint2D parent()
    pub fn get_parent(&self) -> Option<&AdjPoint2D> {
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

    // Java: AdjCoordinate x()
    pub fn get_x(&self) -> &dyn KoAr {
      // Java: if (this.x == null)
        if self.x.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.x = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasX())
            else if self.parent.as_ref().map_or(false, |p| p.hasX())
         {
            // Java: this.x = this.parent.x();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.x = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.x;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasX()
    pub fn hasX(&self) -> bool {
      // Java: return this.map.has(1) || this.x != null && this.x.dirty() || this.parent != null && this.parent.hasX();
      self.map.has(1) || self.x.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasX())
    }

    // Java: void deleteX()
    pub fn deleteX(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.x != null)
        if self.x.is_some()
      {
         // Java: this.x = null;
         self.x = None;
      }
    }

    // Java: void writeX(AdjCoordinate value)
    pub fn writeX(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.x != null)
                if self.x.is_some()
            {
               // Java: this.x = null;
               self.x = None;
            }
         }
      }
    }

    // Java: void forceWriteX(AdjCoordinate value)
    pub fn forceWriteX(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.x != null)
            if self.x.is_some()
         {
            // Java: this.x = null;
            self.x = None;
         }
      }
    }

    // Java: AdjCoordinate y()
    pub fn get_y(&self) -> &dyn KoAr {
      // Java: if (this.y == null)
        if self.y.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.y = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasY())
            else if self.parent.as_ref().map_or(false, |p| p.hasY())
         {
            // Java: this.y = this.parent.y();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.y = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.y;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasY()
    pub fn hasY(&self) -> bool {
      // Java: return this.map.has(2) || this.y != null && this.y.dirty() || this.parent != null && this.parent.hasY();
      self.map.has(2) || self.y.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasY())
    }

    // Java: void deleteY()
    pub fn deleteY(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.y != null)
        if self.y.is_some()
      {
         // Java: this.y = null;
         self.y = None;
      }
    }

    // Java: void writeY(AdjCoordinate value)
    pub fn writeY(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.y != null)
                if self.y.is_some()
            {
               // Java: this.y = null;
               self.y = None;
            }
         }
      }
    }

    // Java: void forceWriteY(AdjCoordinate value)
    pub fn forceWriteY(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.y != null)
            if self.y.is_some()
         {
            // Java: this.y = null;
            self.y = None;
         }
      }
    }

}

impl KoAr for AdjPoint2D {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { AdjPoint2D::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { AdjPoint2D::lastBin(self, var1) }
}

impl KoClosable for AdjPoint2D {
    fn close(&mut self) { AdjPoint2D::close(self); }
}
