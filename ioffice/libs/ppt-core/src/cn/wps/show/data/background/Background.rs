#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
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
// rust: ★M-PPT-KMOSLIDE W6c-3 —— `fill_handle()` / `effect_handle()` / `bgRef_handle()` 的
//   `Rc<RefCell<_>>` 承载类型（Java 无对应 import；Java 用裸引用）。
//   owner: show.data.background    wave: M-PPT-KMOSLIDE-W6c-3
use std::cell::RefCell;
use std::rc::Rc;

// rust: 别名（Java 无）—— 共享句柄，见上方句柄槽说明。
pub type Shared<T> = Rc<RefCell<T>>;

fn java_bool_hash(b: bool) -> i32 { if b { 1231 } else { 1237 } }

const BW_MODE: i32 = 1;
const SHADE_TO_TITLE: i32 = 2;
const FILL: i32 = 3;
const EFFECT: i32 = 4;
const BG_REF: i32 = 5;

// Java: public class Background implements KoAr, KoClosable
pub struct Background {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<Fill>>,
    // Java field: EffectProperties effect
    pub(crate) effect: Option<Box<EffectProperties>>,
    // Java field: StyleMatrixReference bgRef
    pub(crate) bgRef: Option<Box<StyleMatrixReference>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Background parent
    pub(crate) parent: Option<Box<Background>>,
    // rust: ★M-PPT-KMOSLIDE W6c-3 —— 共享句柄槽（无 Java 对应字段）。
    //   Java 的 `fill()` / `effect()` / `bgRef()` 返回**同一个对象引用**，`BackgroundHandler` 的
    //   子处理器（`FillPropertiesHandler` / `EffectPropertiesHandler` /
    //   `StyleMatrixReferenceHandler`）长期持有它并在 `enterSubElement` 中写回同一对象。
    //   Rust 的 `&mut T` 无法跨处理器持有 ⇒ 按 `ShapePr::{fill,geom,ln,xfrm}_handle`
    //   （PLAN §27.4 2B-②）的既有约定给出 `Rc<RefCell<T>>` 句柄。
    //   owner: show.data.background    wave: M-PPT-KMOSLIDE-W6c-3
    pub(crate) fill_handle: Option<Shared<Fill>>,
    pub(crate) effect_handle: Option<Shared<EffectProperties>>,
    pub(crate) bgRef_handle: Option<Shared<StyleMatrixReference>>,
}

// Java: public class Background implements KoAr, KoClosable
impl Background {
    // Java: static protected static <Value1 extends Background> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Background::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Background create()
    pub fn create() -> Self {
      // Java: return initialize(Background.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Background open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Background.class, bin);
      Self::initialize(bin)
    }

    // Java: Background()
    pub fn new() -> Self {
      Self {
        zone: KoZone::defaultZone(),
        fill: None, effect: None, bgRef: None,
        fill_handle: None, effect_handle: None, bgRef_handle: None,
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: Background(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        zone,
        fill: None, effect: None, bgRef: None,
        fill_handle: None, effect_handle: None, bgRef_handle: None,
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
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

    // Java: void load(Background value)
    pub fn load_value(&mut self, value: Option<&Background>) {
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
      // Java: if (this.fill != null && this.fill.dirty())
      if self.fill.as_ref().map_or(false, |f| f.dirty())
      {
         // Java: this.map.put(3, this.fill.saveBin(zone));
         let bin = self.fill.as_mut().unwrap().saveBin(zone);
         self.map.put(FILL, bin);
      }
      // rust: ★共享句柄分支（PLAN §37.2(3) 同款教训：句柄持有对象后，仅看具体槽位
      //   会让该子树序列化为空）。owner: show.data.background · wave: M-PPT-KMOSLIDE-W6c-3
      else if self.fill_handle.as_ref().map_or(false, |h| h.borrow().dirty())
      {
         let bin = self.fill_handle.as_ref().unwrap().borrow_mut().saveBin(zone);
         self.map.put(FILL, bin);
      }
      // Java: if (this.effect != null && this.effect.dirty())
      if self.effect.as_ref().map_or(false, |e| e.dirty())
      {
         // Java: this.map.put(4, this.effect.saveBin(zone));
         let bin = self.effect.as_mut().unwrap().saveBin(zone);
         self.map.put(EFFECT, bin);
      }
      // rust: ★共享句柄分支，理由同上。owner: show.data.background · wave: M-PPT-KMOSLIDE-W6c-3
      else if self.effect_handle.as_ref().map_or(false, |h| h.borrow().dirty())
      {
         let bin = self.effect_handle.as_ref().unwrap().borrow_mut().saveBin(zone);
         self.map.put(EFFECT, bin);
      }
      // Java: if (this.bgRef != null && this.bgRef.dirty())
      if self.bgRef.as_ref().map_or(false, |b| b.dirty())
      {
         // Java: this.map.put(5, this.bgRef.saveBin(zone));
         let bin = self.bgRef.as_mut().unwrap().saveBin(zone);
         self.map.put(BG_REF, bin);
      }
      // rust: ★共享句柄分支，理由同上。owner: show.data.background · wave: M-PPT-KMOSLIDE-W6c-3
      else if self.bgRef_handle.as_ref().map_or(false, |h| h.borrow().dirty())
      {
         let bin = self.bgRef_handle.as_ref().unwrap().borrow_mut().saveBin(zone);
         self.map.put(BG_REF, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(Background o)
    pub fn equalsLax(&mut self, o: &mut Background) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut Background) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut Background, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Background)
      {
         // Java: Background dst = (Background)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBwMode() || !dst.hasBwMode()) && (!this.hasBwMode() || dst.hasBwMode()))
         else if (self.hasBwMode() || !dst.hasBwMode()) && (!self.hasBwMode() || dst.hasBwMode())
         {
            // Java: if (this.hasBwMode() && dst.hasBwMode() && this.bwMode() != dst.bwMode())
            if self.hasBwMode() && dst.hasBwMode() && self.bwMode() != dst.bwMode()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasShadeToTitle() || !dst.hasShadeToTitle()) && (!this.hasShadeToTitle() || dst.hasShadeToTitle()))
            else if (self.hasShadeToTitle() || !dst.hasShadeToTitle()) && (!self.hasShadeToTitle() || dst.hasShadeToTitle())
            {
               // Java: if (this.hasShadeToTitle() && dst.hasShadeToTitle() && this.isShadeToTitle() != dst.isShadeToTitle())
               if self.hasShadeToTitle() && dst.hasShadeToTitle() && self.isShadeToTitle() != dst.isShadeToTitle()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
               else if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill())
               {
                  // Java: if (this.hasFill() && dst.hasFill())
                  if self.hasFill() && dst.hasFill()
                  {
                     // Java: if (isStrict && !this.fill().equals(dst.fill()))
                     if isStrict && !self.get_fill_mut().equals(dst.get_fill_mut())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                     if !isStrict && !self.get_fill_mut().equalsLax(dst.get_fill_mut())
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasEffect() || !dst.hasEffect()) && (!this.hasEffect() || dst.hasEffect()))
                  if (self.hasEffect() || !dst.hasEffect()) && (!self.hasEffect() || dst.hasEffect())
                  {
                     // Java: if (this.hasEffect() && dst.hasEffect())
                     if self.hasEffect() && dst.hasEffect()
                     {
                        // Java: if (isStrict && !this.effect().equals(dst.effect()))
                        if isStrict && !self.get_effect_mut().equals(dst.get_effect_mut())
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.effect().equalsLax(dst.effect()))
                        if !isStrict && !self.get_effect_mut().equalsLax(dst.get_effect_mut())
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasBgRef() || !dst.hasBgRef()) && (!this.hasBgRef() || dst.hasBgRef()))
                     if (self.hasBgRef() || !dst.hasBgRef()) && (!self.hasBgRef() || dst.hasBgRef())
                     {
                        // Java: if (this.hasBgRef() && dst.hasBgRef())
                        if self.hasBgRef() && dst.hasBgRef()
                        {
                           // Java: if (isStrict && !this.bgRef().equals(dst.bgRef()))
                           if isStrict && !self.get_bgRef_mut().equals(dst.get_bgRef_mut())
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.bgRef().equalsLax(dst.bgRef()))
                           if !isStrict && !self.get_bgRef_mut().equalsLax(dst.get_bgRef_mut())
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
      // Java: else
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBwMode())
      if self.hasBwMode()
      {
         // Java: ret += Integer.valueOf(this.bwMode()).hashCode();
         ret = ret.wrapping_add(self.bwMode());
      }
      // Java: if (this.hasShadeToTitle())
      if self.hasShadeToTitle()
      {
         // Java: ret += Boolean.valueOf(this.isShadeToTitle()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isShadeToTitle()));
      }
      // Java: if (this.hasFill())
      if self.hasFill()
      {
         // Java: ret += this.fill().hashCode();
         ret = ret.wrapping_add(self.get_fill_mut().hashCode());
      }
      // Java: if (this.hasEffect())
      if self.hasEffect()
      {
         // Java: ret += this.effect().hashCode();
         ret = ret.wrapping_add(self.get_effect_mut().hashCode());
      }
      // Java: if (this.hasBgRef())
      if self.hasBgRef()
      {
         // Java: ret += this.bgRef().hashCode();
         ret = ret.wrapping_add(self.get_bgRef_mut().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.fill != null && this.fill.dirty())
      if self.fill.as_ref().map_or(false, |f| f.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.effect != null && this.effect.dirty())
      else if self.effect.as_ref().map_or(false, |e| e.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.bgRef != null && this.bgRef.dirty() ? true : this.map.dirty();
         return if self.bgRef.as_ref().map_or(false, |b| b.dirty()) { true } else { self.map.dirty() };
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
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
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
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
      let null_bin = Null::value1(); self.map.load(null_bin.as_ref());
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
      // Java: this.fill = null;
      self.fill = None;
      // Java: this.effect = null;
      self.effect = None;
      // Java: this.bgRef = null;
      self.bgRef = None;
    }

    // Java: void follow(Background parent)
    pub fn follow(&mut self, parent: Option<Box<Background>>) {
      // Java: if (parent != null)
      if let Some(mut parent) = parent
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

    // Java: Background parent()
    pub fn parent(&self) -> Option<&Background> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone; self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int bwMode()
    pub fn bwMode(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(BW_MODE)
      {
         // Java: return this.parent == null ? 9 : this.parent.bwMode();
         return self.parent.as_ref().map_or(9, |p| p.bwMode());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(BW_MODE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBwMode()
    pub fn hasBwMode(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBwMode();
      self.map.has(BW_MODE) || self.parent.as_ref().map_or(false, |p| p.hasBwMode())
    }

    // Java: void deleteBwMode()
    pub fn deleteBwMode(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(BW_MODE);
    }

    // Java: void putBwMode(int value)
    pub fn putBwMode(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(BW_MODE, Encode::from_int(value, self.zone));
    }

    // Java: boolean isShadeToTitle()
    pub fn isShadeToTitle(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(SHADE_TO_TITLE)
      {
         // Java: return this.parent == null ? false : this.parent.isShadeToTitle();
         return self.parent.as_ref().map_or(false, |p| p.isShadeToTitle());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(SHADE_TO_TITLE);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShadeToTitle()
    pub fn hasShadeToTitle(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasShadeToTitle();
      self.map.has(SHADE_TO_TITLE) || self.parent.as_ref().map_or(false, |p| p.hasShadeToTitle())
    }

    // Java: void deleteShadeToTitle()
    pub fn deleteShadeToTitle(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(SHADE_TO_TITLE);
    }

    // Java: void putShadeToTitle(boolean value)
    pub fn putShadeToTitle(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(SHADE_TO_TITLE, Encode::from_bool(value, self.zone));
    }

    // Java: Fill fill()
    pub fn get_fill(&mut self) -> &Fill {
      // Java: if (this.fill == null)
      if self.fill.is_none()
      {
         // Java: if (this.map.has(3))
         if self.map.has(FILL)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(FILL);
            // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            self.fill = Some(Box::new(Fill::Fill_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFill())
         else if self.parent.as_ref().map_or(false, |p| p.hasFill())
         {
            // Java: this.fill = this.parent.fill();
            let bin = self.parent.as_ref().unwrap().lastBin(self.zone);
            self.fill = Some(Box::new(Fill::Fill_from(bin.as_ref())));
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            self.fill = Some(Box::new(Fill::Fill_from(bin.as_ref())));
         }
      }
      // Java: return this.fill;
      self.fill.as_deref().unwrap()
    }

    pub fn get_fill_mut(&mut self) -> &mut Fill {
      if self.fill.is_none() { self.get_fill(); }
      self.fill.as_deref_mut().unwrap()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: return this.map.has(3) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
      // rust: ★共享句柄分支（PLAN §37.2(2)）—— `fill_handle()` 把对象移入句柄后，
      //   `this.fill != null` 的原判据会恒假。owner: show.data.background · wave: M-PPT-KMOSLIDE-W6c-3
      self.map.has(FILL) || self.fill.as_ref().map_or(false, |f| f.dirty())
        || self.fill_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FILL);
      // Java: if (this.fill != null)
      if self.fill.is_some()
      {
         // Java: this.fill = null;
         self.fill = None;
      }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: Option<&Fill>) {
      // Java: if (value != null)
      if let Some(value) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(FILL, bin);
            // Java: if (this.fill != null)
            if self.fill.is_some()
            {
               // Java: this.fill = null;
               self.fill = None;
            }
         }
      }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: Option<&Fill>) {
      // Java: if (value != null)
      if let Some(value) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(FILL, value.lastBin(self.zone));
         // Java: if (this.fill != null)
         if self.fill.is_some()
         {
            // Java: this.fill = null;
            self.fill = None;
         }
      }
    }

    // Java: EffectProperties effect()
    pub fn get_effect(&mut self) -> &EffectProperties {
      // Java: if (this.effect == null)
      if self.effect.is_none()
      {
         // Java: if (this.map.has(4))
         if self.map.has(EFFECT)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(EFFECT);
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            self.effect = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEffect())
         else if self.parent.as_ref().map_or(false, |p| p.hasEffect())
         {
            // Java: this.effect = this.parent.effect();
            let bin = self.parent.as_ref().unwrap().lastBin(self.zone);
            self.effect = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            self.effect = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
         }
      }
      // Java: return this.effect;
      self.effect.as_deref().unwrap()
    }

    pub fn get_effect_mut(&mut self) -> &mut EffectProperties {
      if self.effect.is_none() { self.get_effect(); }
      self.effect.as_deref_mut().unwrap()
    }

    // Java: boolean hasEffect()
    pub fn hasEffect(&self) -> bool {
      // Java: return this.map.has(4) || this.effect != null && this.effect.dirty() || this.parent != null && this.parent.hasEffect();
      // rust: ★共享句柄分支，理由同 `hasFill`。owner: show.data.background · wave: M-PPT-KMOSLIDE-W6c-3
      self.map.has(EFFECT) || self.effect.as_ref().map_or(false, |e| e.dirty())
        || self.effect_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffect())
    }

    // Java: void deleteEffect()
    pub fn deleteEffect(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(EFFECT);
      // Java: if (this.effect != null)
      if self.effect.is_some()
      {
         // Java: this.effect = null;
         self.effect = None;
      }
    }

    // Java: void writeEffect(EffectProperties value)
    pub fn writeEffect(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
      if let Some(value) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(EFFECT, bin);
            // Java: if (this.effect != null)
            if self.effect.is_some()
            {
               // Java: this.effect = null;
               self.effect = None;
            }
         }
      }
    }

    // Java: void forceWriteEffect(EffectProperties value)
    pub fn forceWriteEffect(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
      if let Some(value) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(EFFECT, value.lastBin(self.zone));
         // Java: if (this.effect != null)
         if self.effect.is_some()
         {
            // Java: this.effect = null;
            self.effect = None;
         }
      }
    }

    // Java: StyleMatrixReference bgRef()
    pub fn get_bgRef(&mut self) -> &StyleMatrixReference {
      // Java: if (this.bgRef == null)
      if self.bgRef.is_none()
      {
         // Java: if (this.map.has(5))
         if self.map.has(BG_REF)
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(BG_REF);
            // Java: this.bgRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            self.bgRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBgRef())
         else if self.parent.as_ref().map_or(false, |p| p.hasBgRef())
         {
            // Java: this.bgRef = this.parent.bgRef();
            let bin = self.parent.as_ref().unwrap().lastBin(self.zone);
            self.bgRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.bgRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            self.bgRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
      }
      // Java: return this.bgRef;
      self.bgRef.as_deref().unwrap()
    }

    pub fn get_bgRef_mut(&mut self) -> &mut StyleMatrixReference {
      if self.bgRef.is_none() { self.get_bgRef(); }
      self.bgRef.as_deref_mut().unwrap()
    }

    // rust: ★M-PPT-KMOSLIDE W6c-3 —— **共享句柄**访问器（Java 无此法名）。
    //   Java 的 `fill()` / `effect()` / `bgRef()` 返回的是**同一个对象引用**，`BackgroundHandler`
    //   的子处理器各自长期持有它并在 `enterSubElement` 中写回同一对象。Rust 的 `&mut T`
    //   无法跨处理器持有（独占借用），故按 `ShapePr::{fill,geom,ln,xfrm}_handle`
    //   （PLAN §27.4 2B-②）的既有约定给出 `Rc<RefCell<T>>` 句柄，语义与 Java 引用一致：
    //   首次取句柄时把自有槽位内容**移入** `Rc<RefCell<_>>`，之后所有读写都经该句柄，
    //   即「真身槽位」与「处理器看到的对象」恒为同一个。
    //   owner: show.data.background    wave: M-PPT-KMOSLIDE-W6c-3

    // Java: Fill fill()
    pub fn fill_handle(&mut self) -> Shared<Fill> {
      if self.fill_handle.is_none() {
        let value = match self.fill.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(FILL);
            Fill::Fill_from(bin.as_ref())
          }
        };
        self.fill_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.fill_handle.as_ref().unwrap().clone()
    }

    // Java: EffectProperties effect()
    pub fn effect_handle(&mut self) -> Shared<EffectProperties> {
      if self.effect_handle.is_none() {
        let value = match self.effect.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(EFFECT);
            EffectProperties::EffectProperties_from(bin.as_ref())
          }
        };
        self.effect_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.effect_handle.as_ref().unwrap().clone()
    }

    // Java: StyleMatrixReference bgRef()
    pub fn bgRef_handle(&mut self) -> Shared<StyleMatrixReference> {
      if self.bgRef_handle.is_none() {
        let value = match self.bgRef.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(BG_REF);
            StyleMatrixReference::new_from(bin.as_ref())
          }
        };
        self.bgRef_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.bgRef_handle.as_ref().unwrap().clone()
    }

    // Java: boolean hasBgRef()
    pub fn hasBgRef(&self) -> bool {
      // Java: return this.map.has(5) || this.bgRef != null && this.bgRef.dirty() || this.parent != null && this.parent.hasBgRef();
      // rust: ★共享句柄分支，理由同 `hasFill`。owner: show.data.background · wave: M-PPT-KMOSLIDE-W6c-3
      self.map.has(BG_REF) || self.bgRef.as_ref().map_or(false, |b| b.dirty())
        || self.bgRef_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasBgRef())
    }

    // Java: void deleteBgRef()
    pub fn deleteBgRef(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(BG_REF);
      // Java: if (this.bgRef != null)
      if self.bgRef.is_some()
      {
         // Java: this.bgRef = null;
         self.bgRef = None;
      }
    }

    // Java: void writeBgRef(StyleMatrixReference value)
    pub fn writeBgRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
      if let Some(value) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(5, bin);
            self.map.put(BG_REF, bin);
            // Java: if (this.bgRef != null)
            if self.bgRef.is_some()
            {
               // Java: this.bgRef = null;
               self.bgRef = None;
            }
         }
      }
    }

    // Java: void forceWriteBgRef(StyleMatrixReference value)
    pub fn forceWriteBgRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
      if let Some(value) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(BG_REF, value.lastBin(self.zone));
         // Java: if (this.bgRef != null)
         if self.bgRef.is_some()
         {
            // Java: this.bgRef = null;
            self.bgRef = None;
         }
      }
    }

}

impl KoAr for Background {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Background::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Background::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Background::lastBin(self, var1) }
}

impl KoClosable for Background {
    fn close(&mut self) { Background::close(self); }
}
