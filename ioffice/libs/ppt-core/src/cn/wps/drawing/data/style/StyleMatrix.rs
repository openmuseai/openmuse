#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.fill.Fill
// use crate::cn::wps::drawing::data::fill::Fill; // not yet enabled
pub struct FillStub;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection
use crate::cn::wps::drawing::data::style::EffectStyle::EffectStyle;

// Java: public class StyleMatrix implements KoAr, KoClosable
pub struct StyleMatrix {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: BackgroundFillStyleList bgFillLst
    pub(crate) bgFillLst: Option<Box<BackgroundFillStyleList>>,
    // Java field: FillStyleLst fillLst
    pub(crate) fillLst: Option<Box<FillStyleLst>>,
    // Java field: EffectStyleLst effectLst
    pub(crate) effectLst: Option<Box<EffectStyleLst>>,
    // Java field: LineStyleLst outlineLst
    pub(crate) outlineLst: Option<Box<LineStyleLst>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: StyleMatrix parent
    pub(crate) parent: Option<Box<StyleMatrix>>,
}

// Java: public class BackgroundFillStyleList
pub struct BackgroundFillStyleList {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
}

// Java: public class FillStyleLst
pub struct FillStyleLst {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
}

// Java: public class EffectStyleLst
pub struct EffectStyleLst {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
}

// Java: public class LineStyleLst
pub struct LineStyleLst {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
}

// Java: public class StyleMatrix implements KoAr, KoClosable
impl StyleMatrix {
    // Java: private static final int $bgFillLst = 1
    const FIELD_BGFILLLST: i32 = 1;
    // Java: private static final int $fillLst = 2
    const FIELD_FILLLST: i32 = 2;
    // Java: private static final int $effectLst = 3
    const FIELD_EFFECTLST: i32 = 3;
    // Java: private static final int $outlineLst = 4
    const FIELD_OUTLINELST: i32 = 4;
    // Java: private static final int $name = 5
    const FIELD_NAME: i32 = 5;

    // Java: static protected static <Value1 extends StyleMatrix> Value1 initialize(Class<Value1> clazz, KoBin bin)
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

    // Java: static StyleMatrix create()
    pub fn create() -> Self {
      // Java: return initialize(StyleMatrix.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static StyleMatrix open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(StyleMatrix.class, bin);
      Self::initialize(bin)
    }

    // Java: StyleMatrix()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        bgFillLst: None,
        fillLst: None,
        effectLst: None,
        outlineLst: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: StyleMatrix(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        bgFillLst: None,
        fillLst: None,
        effectLst: None,
        outlineLst: None,
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

    // Java: void load(StyleMatrix value)
    pub fn load_from(&mut self, value: Option<&StyleMatrix>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.bgFillLst != null && this.bgFillLst.dirty())
        if self.bgFillLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.bgFillLst.saveBin(zone));
         let bin = self.bgFillLst.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.fillLst != null && this.fillLst.dirty())
        if self.fillLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.fillLst.saveBin(zone));
         let bin = self.fillLst.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.effectLst != null && this.effectLst.dirty())
        if self.effectLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.effectLst.saveBin(zone));
         let bin = self.effectLst.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: if (this.outlineLst != null && this.outlineLst.dirty())
        if self.outlineLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.outlineLst.saveBin(zone));
         let bin = self.outlineLst.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(StyleMatrix o)
    pub fn equalsLax(&self, o: &StyleMatrix) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &StyleMatrix) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &StyleMatrix, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof StyleMatrix)
      {
         // Java: StyleMatrix dst = (StyleMatrix)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBgFillLst() || !dst.hasBgFillLst()) && (!this.hasBgFillLst() || dst.hasBgFillLst()))
            else if (self.hasBgFillLst() || !dst.hasBgFillLst()) && (!self.hasBgFillLst() || dst.hasBgFillLst())
         {
            // Java: if (this.hasBgFillLst() && dst.hasBgFillLst())
               if self.hasBgFillLst() && dst.hasBgFillLst()
            {
               // Java: if (isStrict && !this.bgFillLst().equals(dst.bgFillLst()))
                  if isStrict
               {
                  // Java: return false;
                  todo!("bgFillLst equals");
               }
               // Java: if (!isStrict && !this.bgFillLst().equalsLax(dst.bgFillLst()))
                  if !isStrict
               {
                  // Java: return false;
                  todo!("bgFillLst equalsLax");
               }
            }
            // Java: if ((this.hasFillLst() || !dst.hasFillLst()) && (!this.hasFillLst() || dst.hasFillLst()))
               if (self.hasFillLst() || !dst.hasFillLst()) && (!self.hasFillLst() || dst.hasFillLst())
            {
               // Java: if (this.hasFillLst() && dst.hasFillLst())
                  if self.hasFillLst() && dst.hasFillLst()
               {
                  // Java: if (isStrict && !this.fillLst().equals(dst.fillLst()))
                     if isStrict
                  {
                     // Java: return false;
                     todo!("fillLst equals");
                  }
                  // Java: if (!isStrict && !this.fillLst().equalsLax(dst.fillLst()))
                     if !isStrict
                  {
                     // Java: return false;
                     todo!("fillLst equalsLax");
                  }
               }
               // Java: if ((this.hasEffectLst() || !dst.hasEffectLst()) && (!this.hasEffectLst() || dst.hasEffectLst()))
                  if (self.hasEffectLst() || !dst.hasEffectLst()) && (!self.hasEffectLst() || dst.hasEffectLst())
               {
                  // Java: if (this.hasEffectLst() && dst.hasEffectLst())
                     if self.hasEffectLst() && dst.hasEffectLst()
                  {
                     // Java: if (isStrict && !this.effectLst().equals(dst.effectLst()))
                        if isStrict
                     {
                        // Java: return false;
                        todo!("effectLst equals");
                     }
                     // Java: if (!isStrict && !this.effectLst().equalsLax(dst.effectLst()))
                        if !isStrict
                     {
                        // Java: return false;
                        todo!("effectLst equalsLax");
                     }
                  }
                  // Java: if ((this.hasOutlineLst() || !dst.hasOutlineLst()) && (!this.hasOutlineLst() || dst.hasOutlineLst()))
                     if (self.hasOutlineLst() || !dst.hasOutlineLst()) && (!self.hasOutlineLst() || dst.hasOutlineLst())
                  {
                     // Java: if (this.hasOutlineLst() && dst.hasOutlineLst())
                        if self.hasOutlineLst() && dst.hasOutlineLst()
                     {
                        // Java: if (isStrict && !this.outlineLst().equals(dst.outlineLst()))
                           if isStrict
                        {
                           // Java: return false;
                           todo!("outlineLst equals");
                        }
                        // Java: if (!isStrict && !this.outlineLst().equalsLax(dst.outlineLst()))
                           if !isStrict
                        {
                           // Java: return false;
                           todo!("outlineLst equalsLax");
                        }
                     }
                     // Java: if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
                        if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName())
                     {
                        // Java: return !this.hasName() || !dst.hasName() || this.name().equals(dst.name());
                        return !self.hasName() || !dst.hasName() || self.name() == dst.name();
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
      // Java: if (this.hasBgFillLst())
      if self.hasBgFillLst() {
         // Java: ret += this.bgFillLst().hashCode();
         self.bgFillLst();
         ret = ret.wrapping_add(self.bgFillLst.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasFillLst())
      if self.hasFillLst() {
         // Java: ret += this.fillLst().hashCode();
         self.fillLst();
         ret = ret.wrapping_add(self.fillLst.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasEffectLst())
      if self.hasEffectLst() {
         // Java: ret += this.effectLst().hashCode();
         self.effectLst();
         ret = ret.wrapping_add(self.effectLst.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasOutlineLst())
      if self.hasOutlineLst() {
         // Java: ret += this.outlineLst().hashCode();
         self.outlineLst();
         ret = ret.wrapping_add(self.outlineLst.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasName())
      if self.hasName() {
         // Java: ret += String.valueOf(this.name()).hashCode();
         todo!("name hashCode");
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.bgFillLst != null && this.bgFillLst.dirty())
        if self.bgFillLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fillLst != null && this.fillLst.dirty())
        if self.fillLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.effectLst != null && this.effectLst.dirty())
        if self.effectLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      // Java: return this.outlineLst != null && this.outlineLst.dirty() ? true : this.map.dirty();
      self.outlineLst.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
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
      // Java: this.bgFillLst = null;
      self.bgFillLst = None;
      // Java: this.fillLst = null;
      self.fillLst = None;
      // Java: this.effectLst = null;
      self.effectLst = None;
      // Java: this.outlineLst = null;
      self.outlineLst = None;
    }

    // Java: void follow(StyleMatrix parent)
    pub fn follow(&mut self, parent: Option<Box<StyleMatrix>>) {
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

    // Java: StyleMatrix parent()
    pub fn parent(&self) -> &StyleMatrix {
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

    // Java: BackgroundFillStyleList bgFillLst()
    pub fn bgFillLst(&mut self) -> &BackgroundFillStyleList {
      // Java: if (this.bgFillLst == null)
        if self.bgFillLst.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.bgFillLst = (BackgroundFillStyleList)KoBin.Decode.toAny(BackgroundFillStyleList.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.bgFillLst = Some(Box::new(BackgroundFillStyleList::new_from(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasBgFillLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasBgFillLst())
         {
            // Java: this.bgFillLst = this.parent.bgFillLst();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.bgFillLst = (BackgroundFillStyleList)KoBin.Decode.toAny(BackgroundFillStyleList.class, bin, this.zone);
            let null_bin = Null::empty();
            self.bgFillLst = Some(Box::new(BackgroundFillStyleList::new_from(null_bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.bgFillLst;
      self.bgFillLst.as_deref().unwrap()
    }

    // Java: boolean hasBgFillLst()
    pub fn hasBgFillLst(&self) -> bool {
      // Java: return this.map.has(1) || this.bgFillLst != null && this.bgFillLst.dirty() || this.parent != null && this.parent.hasBgFillLst();
      self.map.has(1)
        || self.bgFillLst.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasBgFillLst())
    }

    // Java: void deleteBgFillLst()
    pub fn deleteBgFillLst(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.bgFillLst != null)
        if self.bgFillLst.is_some()
      {
         // Java: this.bgFillLst = null;
         self.bgFillLst = None;
      }
    }

    // Java: void writeBgFillLst(BackgroundFillStyleList value)
    pub fn writeBgFillLst(&mut self, value: Option<&BackgroundFillStyleList>) {
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
            // Java: if (this.bgFillLst != null)
               if self.bgFillLst.is_some()
            {
               // Java: this.bgFillLst = null;
               self.bgFillLst = None;
            }
         }
      }
    }

    // Java: void forceWriteBgFillLst(BackgroundFillStyleList value)
    pub fn forceWriteBgFillLst(&mut self, value: Option<&BackgroundFillStyleList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.bgFillLst != null)
            if self.bgFillLst.is_some()
         {
            // Java: this.bgFillLst = null;
            self.bgFillLst = None;
         }
      }
    }

    // Java: FillStyleLst fillLst()
    pub fn fillLst(&mut self) -> &FillStyleLst {
      // Java: if (this.fillLst == null)
        if self.fillLst.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.fillLst = (FillStyleLst)KoBin.Decode.toAny(FillStyleLst.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.fillLst = Some(Box::new(FillStyleLst::new_from(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasFillLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillLst())
         {
            // Java: this.fillLst = this.parent.fillLst();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.fillLst = (FillStyleLst)KoBin.Decode.toAny(FillStyleLst.class, bin, this.zone);
            let null_bin = Null::empty();
            self.fillLst = Some(Box::new(FillStyleLst::new_from(null_bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.fillLst;
      self.fillLst.as_deref().unwrap()
    }

    // Java: boolean hasFillLst()
    pub fn hasFillLst(&self) -> bool {
      // Java: return this.map.has(2) || this.fillLst != null && this.fillLst.dirty() || this.parent != null && this.parent.hasFillLst();
      self.map.has(2)
        || self.fillLst.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasFillLst())
    }

    // Java: void deleteFillLst()
    pub fn deleteFillLst(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.fillLst != null)
        if self.fillLst.is_some()
      {
         // Java: this.fillLst = null;
         self.fillLst = None;
      }
    }

    // Java: void writeFillLst(FillStyleLst value)
    pub fn writeFillLst(&mut self, value: Option<&FillStyleLst>) {
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
            // Java: if (this.fillLst != null)
               if self.fillLst.is_some()
            {
               // Java: this.fillLst = null;
               self.fillLst = None;
            }
         }
      }
    }

    // Java: void forceWriteFillLst(FillStyleLst value)
    pub fn forceWriteFillLst(&mut self, value: Option<&FillStyleLst>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.fillLst != null)
            if self.fillLst.is_some()
         {
            // Java: this.fillLst = null;
            self.fillLst = None;
         }
      }
    }

    // Java: EffectStyleLst effectLst()
    pub fn effectLst(&mut self) -> &EffectStyleLst {
      // Java: if (this.effectLst == null)
        if self.effectLst.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.effectLst = (EffectStyleLst)KoBin.Decode.toAny(EffectStyleLst.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.effectLst = Some(Box::new(EffectStyleLst::new_from(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasEffectLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectLst())
         {
            // Java: this.effectLst = this.parent.effectLst();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.effectLst = (EffectStyleLst)KoBin.Decode.toAny(EffectStyleLst.class, bin, this.zone);
            let null_bin = Null::empty();
            self.effectLst = Some(Box::new(EffectStyleLst::new_from(null_bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.effectLst;
      self.effectLst.as_deref().unwrap()
    }

    // Java: boolean hasEffectLst()
    pub fn hasEffectLst(&self) -> bool {
      // Java: return this.map.has(3) || this.effectLst != null && this.effectLst.dirty() || this.parent != null && this.parent.hasEffectLst();
      self.map.has(3)
        || self.effectLst.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffectLst())
    }

    // Java: void deleteEffectLst()
    pub fn deleteEffectLst(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.effectLst != null)
        if self.effectLst.is_some()
      {
         // Java: this.effectLst = null;
         self.effectLst = None;
      }
    }

    // Java: void writeEffectLst(EffectStyleLst value)
    pub fn writeEffectLst(&mut self, value: Option<&EffectStyleLst>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.effectLst != null)
               if self.effectLst.is_some()
            {
               // Java: this.effectLst = null;
               self.effectLst = None;
            }
         }
      }
    }

    // Java: void forceWriteEffectLst(EffectStyleLst value)
    pub fn forceWriteEffectLst(&mut self, value: Option<&EffectStyleLst>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.effectLst != null)
            if self.effectLst.is_some()
         {
            // Java: this.effectLst = null;
            self.effectLst = None;
         }
      }
    }

    // Java: LineStyleLst outlineLst()
    pub fn outlineLst(&mut self) -> &LineStyleLst {
      // Java: if (this.outlineLst == null)
        if self.outlineLst.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            // Java: this.outlineLst = (LineStyleLst)KoBin.Decode.toAny(LineStyleLst.class, bin, this.zone);
            let bin = self.map.byKey(4);
            self.outlineLst = Some(Box::new(LineStyleLst::new_from(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasOutlineLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasOutlineLst())
         {
            // Java: this.outlineLst = this.parent.outlineLst();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.outlineLst = (LineStyleLst)KoBin.Decode.toAny(LineStyleLst.class, bin, this.zone);
            let null_bin = Null::empty();
            self.outlineLst = Some(Box::new(LineStyleLst::new_from(null_bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.outlineLst;
      self.outlineLst.as_deref().unwrap()
    }

    // Java: boolean hasOutlineLst()
    pub fn hasOutlineLst(&self) -> bool {
      // Java: return this.map.has(4) || this.outlineLst != null && this.outlineLst.dirty() || this.parent != null && this.parent.hasOutlineLst();
      self.map.has(4)
        || self.outlineLst.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasOutlineLst())
    }

    // Java: void deleteOutlineLst()
    pub fn deleteOutlineLst(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.outlineLst != null)
        if self.outlineLst.is_some()
      {
         // Java: this.outlineLst = null;
         self.outlineLst = None;
      }
    }

    // Java: void writeOutlineLst(LineStyleLst value)
    pub fn writeOutlineLst(&mut self, value: Option<&LineStyleLst>) {
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
            // Java: if (this.outlineLst != null)
               if self.outlineLst.is_some()
            {
               // Java: this.outlineLst = null;
               self.outlineLst = None;
            }
         }
      }
    }

    // Java: void forceWriteOutlineLst(LineStyleLst value)
    pub fn forceWriteOutlineLst(&mut self, value: Option<&LineStyleLst>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.outlineLst != null)
            if self.outlineLst.is_some()
         {
            // Java: this.outlineLst = null;
            self.outlineLst = None;
         }
      }
    }

    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? "" : this.parent.name();
         if let Some(ref p) = self.parent { p.name() } else { String::new() }
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasName();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_string(value, self.zone));
    }

}

// Java: public class BackgroundFillStyleList
impl BackgroundFillStyleList {
    // Java: BackgroundFillStyleList()
    pub fn new(zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         Self { zone, bin_box: KpBinBox::new(), list: KpList::new(zone) }
    }

    // Java: BackgroundFillStyleList(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         Self { zone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone) }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin(bin, zone);
    }

    // Java: void load(BackgroundFillStyleList value)
    pub fn load_from(&mut self, value: Option<&BackgroundFillStyleList>) {
         // Java: if (value != null)
         if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
         else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(BackgroundFillStyleList o)
    pub fn equalsLax(&self, o: &BackgroundFillStyleList) -> bool {
         // Java: return this.equals(o, false);
         todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &BackgroundFillStyleList) -> bool {
         // Java: return this.equals(o, true);
         todo!()
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &BackgroundFillStyleList, isStrict: bool) -> bool {
         // Java: if (null == o)
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: BackgroundFillStyleList dst = (BackgroundFillStyleList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            {
               // Java: ArrayList<Fill> dstList = new ArrayList();
               // Java: dst.read(dstList);
               // Java: ArrayList<Fill> srcList = new ArrayList();
               // Java: this.read(srcList);
               // Java: if (dstList.size() != srcList.size())
               {
                  // Java: return false;
               }
               // Java: else
               {
                  // Java: int size = dstList.size();
                  // Java: for(int i = 0; i < size; ++i)
                  {
                     // Java: Fill dstClrTrans = (Fill)dstList.get(i);
                     // Java: Fill srcClrTrans = (Fill)srcList.get(i);
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     {
                        // Java: return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     {
                        // Java: return false;
                     }
                  }
                  // Java: return true;
               }
            }
         }
         todo!("BackgroundFillStyleList equals_inner")
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::empty();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends BackgroundFillStyleList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         // Java: inst.zone = zone;
         // Java: inst.load(bin);
         // Java: return inst;
         Self::new_from(bin, zone)
    }

    // Java: static BackgroundFillStyleList create()
    pub fn create() -> Self {
         // Java: return initialize(BackgroundFillStyleList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static BackgroundFillStyleList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(BackgroundFillStyleList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Fill> valueLst)
    pub fn read(&self, valueLst: &mut Vec<FillStub>) {
         // Java: valueLst.clear();
         valueLst.clear();
         // Java: int size = this.list.size();
         // Java: for(int i = 0; i < size; ++i)
         // Java: valueLst.add((Fill)KoBin.Decode.toAny(Fill.class, this.list.get(i), this.zone));
         todo!("read Fill list")
    }

    // Java: void write(Iterable<Fill> valueLst)
    pub fn write(&mut self, valueLst: &[FillStub]) {
         // Java: this.list.clear();
         // Java: for(Fill i : valueLst)
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            {
               // Java: this.list.add(bin);
            }
         }
         todo!("write Fill list")
    }

    // Java: void forceWrite(Iterable<Fill> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[FillStub]) {
         // Java: this.list.clear();
         // Java: for(Fill i : valueLst)
         {
            // Java: this.list.add(i.lastBin(this.zone));
         }
         todo!("forceWrite Fill list")
    }

}

// Java: public class FillStyleLst
impl FillStyleLst {
    // Java: FillStyleLst()
    pub fn new(zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         Self { zone, bin_box: KpBinBox::new(), list: KpList::new(zone) }
    }

    // Java: FillStyleLst(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         Self { zone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone) }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin(bin, zone);
    }

    // Java: void load(FillStyleLst value)
    pub fn load_from(&mut self, value: Option<&FillStyleLst>) {
         // Java: if (value != null)
         if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
         else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(FillStyleLst o)
    pub fn equalsLax(&self, o: &FillStyleLst) -> bool {
         // Java: return this.equals(o, false);
         todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &FillStyleLst) -> bool {
         // Java: return this.equals(o, true);
         todo!()
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &FillStyleLst, isStrict: bool) -> bool {
         // Java: if (null == o)
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: FillStyleLst dst = (FillStyleLst)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            {
               // Java: ArrayList<Fill> dstList = new ArrayList();
               // Java: dst.read(dstList);
               // Java: ArrayList<Fill> srcList = new ArrayList();
               // Java: this.read(srcList);
               // Java: if (dstList.size() != srcList.size())
               {
                  // Java: return false;
               }
               // Java: else
               {
                  // Java: int size = dstList.size();
                  // Java: for(int i = 0; i < size; ++i)
                  {
                     // Java: Fill dstClrTrans = (Fill)dstList.get(i);
                     // Java: Fill srcClrTrans = (Fill)srcList.get(i);
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     {
                        // Java: return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     {
                        // Java: return false;
                     }
                  }
                  // Java: return true;
               }
            }
         }
         todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::empty();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends FillStyleLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         // Java: inst.zone = zone;
         // Java: inst.load(bin);
         // Java: return inst;
         Self::new_from(bin, zone)
    }

    // Java: static FillStyleLst create()
    pub fn create() -> Self {
         // Java: return initialize(FillStyleLst.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static FillStyleLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(FillStyleLst.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Fill> valueLst)
    pub fn read(&self, valueLst: &mut Vec<FillStub>) {
         // Java: valueLst.clear();
         // Java: int size = this.list.size();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: valueLst.add((Fill)KoBin.Decode.toAny(Fill.class, this.list.get(i), this.zone));
         }
         todo!("read Fill list")
    }

    // Java: void write(Iterable<Fill> valueLst)
    pub fn write(&mut self, valueLst: &[FillStub]) {
         // Java: this.list.clear();
         // Java: for(Fill i : valueLst)
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            {
               // Java: this.list.add(bin);
            }
         }
         todo!("write Fill list")
    }

    // Java: void forceWrite(Iterable<Fill> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[FillStub]) {
         // Java: this.list.clear();
         // Java: for(Fill i : valueLst)
         {
            // Java: this.list.add(i.lastBin(this.zone));
         }
         todo!("forceWrite Fill list")
    }
}

// Java: public class EffectStyleLst
impl EffectStyleLst {
    // Java: EffectStyleLst()
    pub fn new(zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         Self { zone, bin_box: KpBinBox::new(), list: KpList::new(zone) }
    }

    // Java: EffectStyleLst(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         Self { zone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone) }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin(bin, zone);
    }

    // Java: void load(EffectStyleLst value)
    pub fn load_from(&mut self, value: Option<&EffectStyleLst>) {
         // Java: if (value != null)
         if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
         else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(EffectStyleLst o)
    pub fn equalsLax(&self, o: &EffectStyleLst) -> bool {
         // Java: return this.equals(o, false);
         todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EffectStyleLst) -> bool {
         // Java: return this.equals(o, true);
         todo!()
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &EffectStyleLst, isStrict: bool) -> bool {
         // Java: if (null == o)
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: EffectStyleLst dst = (EffectStyleLst)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            {
               // Java: ArrayList<EffectStyle> dstList = new ArrayList();
               // Java: dst.read(dstList);
               // Java: ArrayList<EffectStyle> srcList = new ArrayList();
               // Java: this.read(srcList);
               // Java: if (dstList.size() != srcList.size())
               {
                  // Java: return false;
               }
               // Java: else
               {
                  // Java: int size = dstList.size();
                  // Java: for(int i = 0; i < size; ++i)
                  {
                     // Java: EffectStyle dstClrTrans = (EffectStyle)dstList.get(i);
                     // Java: EffectStyle srcClrTrans = (EffectStyle)srcList.get(i);
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     {
                        // Java: return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     {
                        // Java: return false;
                     }
                  }
                  // Java: return true;
               }
            }
         }
         todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::empty();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends EffectStyleLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         // Java: inst.zone = zone;
         // Java: inst.load(bin);
         // Java: return inst;
         Self::new_from(bin, zone)
    }

    // Java: static EffectStyleLst create()
    pub fn create() -> Self {
         // Java: return initialize(EffectStyleLst.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static EffectStyleLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(EffectStyleLst.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<EffectStyle> valueLst)
    pub fn read(&self, valueLst: &mut Vec<EffectStyle>) {
         // Java: valueLst.clear();
         // Java: int size = this.list.size();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: valueLst.add((EffectStyle)KoBin.Decode.toAny(EffectStyle.class, this.list.get(i), this.zone));
         }
         todo!("read EffectStyle list")
    }

    // Java: void write(Iterable<EffectStyle> valueLst)
    pub fn write(&mut self, valueLst: &[EffectStyle]) {
         // Java: this.list.clear();
         // Java: for(EffectStyle i : valueLst)
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            {
               // Java: this.list.add(bin);
            }
         }
         todo!("write EffectStyle list")
    }

    // Java: void forceWrite(Iterable<EffectStyle> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[EffectStyle]) {
         // Java: this.list.clear();
         // Java: for(EffectStyle i : valueLst)
         {
            // Java: this.list.add(i.lastBin(this.zone));
         }
         todo!("forceWrite EffectStyle list")
    }
}

// Java: public class LineStyleLst
impl LineStyleLst {
    // Java: LineStyleLst()
    pub fn new(zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         Self { zone, bin_box: KpBinBox::new(), list: KpList::new(zone) }
    }

    // Java: LineStyleLst(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         Self { zone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone) }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin(bin, zone);
    }

    // Java: void load(LineStyleLst value)
    pub fn load_from(&mut self, value: Option<&LineStyleLst>) {
         // Java: if (value != null)
         if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
         else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(LineStyleLst o)
    pub fn equalsLax(&self, o: &LineStyleLst) -> bool {
         // Java: return this.equals(o, false);
         todo!()
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &LineStyleLst) -> bool {
         // Java: return this.equals(o, true);
         todo!()
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &LineStyleLst, isStrict: bool) -> bool {
         // Java: if (null == o)
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: LineStyleLst dst = (LineStyleLst)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            {
               // Java: ArrayList<Outline> dstList = new ArrayList();
               // Java: dst.read(dstList);
               // Java: ArrayList<Outline> srcList = new ArrayList();
               // Java: this.read(srcList);
               // Java: if (dstList.size() != srcList.size())
               {
                  // Java: return false;
               }
               // Java: else
               {
                  // Java: int size = dstList.size();
                  // Java: for(int i = 0; i < size; ++i)
                  {
                     // Java: Outline dstClrTrans = (Outline)dstList.get(i);
                     // Java: Outline srcClrTrans = (Outline)srcList.get(i);
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     {
                        // Java: return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     {
                        // Java: return false;
                     }
                  }
                  // Java: return true;
               }
            }
         }
         todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::empty();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends LineStyleLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         // Java: inst.zone = zone;
         // Java: inst.load(bin);
         // Java: return inst;
         Self::new_from(bin, zone)
    }

    // Java: static LineStyleLst create()
    pub fn create() -> Self {
         // Java: return initialize(LineStyleLst.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static LineStyleLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(LineStyleLst.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Outline> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Outline>) {
         // Java: valueLst.clear();
         // Java: int size = this.list.size();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: valueLst.add((Outline)KoBin.Decode.toAny(Outline.class, this.list.get(i), this.zone));
         }
         todo!("read Outline list")
    }

    // Java: void write(Iterable<Outline> valueLst)
    pub fn write(&mut self, valueLst: &[Outline]) {
         // Java: this.list.clear();
         // Java: for(Outline i : valueLst)
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            {
               // Java: this.list.add(bin);
            }
         }
         todo!("write Outline list")
    }

    // Java: void forceWrite(Iterable<Outline> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Outline]) {
         // Java: this.list.clear();
         // Java: for(Outline i : valueLst)
         {
            // Java: this.list.add(i.lastBin(this.zone));
         }
         todo!("forceWrite Outline list")
    }
}

impl KoAr for StyleMatrix {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for StyleMatrix { fn close(&mut self) { self.close(); } }
