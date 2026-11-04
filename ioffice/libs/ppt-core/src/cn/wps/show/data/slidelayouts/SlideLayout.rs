#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
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
// Java: import cn.wps.show.data.HeaderFooter
use crate::cn::wps::show::data::HeaderFooter::HeaderFooter;
// Java: import cn.wps.show.data.timing.Timing
// NOTE: Timing stub for compilation isolation
pub struct Timing { pub(crate) zone: &'static KoZone, pub(crate) bin_box: KpBinBox, pub(crate) map: KpMap, pub(crate) parent: Option<Box<Timing>> }
impl Timing {
    pub fn open(bin: &dyn KoBinTrait) -> Self { Self { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None } }
    pub fn dirty(&self) -> bool { self.map.dirty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); self.bin_box.get() } else { self.bin_box.get() } }
    pub fn hashCode(&self) -> i32 { 0 }
    pub fn equals(&mut self, _o: &mut Timing) -> bool { true }
    pub fn equalsLax(&mut self, _o: &mut Timing) -> bool { true }
}
// Java: import cn.wps.show.data.transition.Transition
// NOTE: Transition stub for compilation isolation
pub struct Transition { pub(crate) zone: &'static KoZone, pub(crate) bin_box: KpBinBox, pub(crate) map: KpMap, pub(crate) parent: Option<Box<Transition>> }
impl Transition {
    pub fn open(bin: &dyn KoBinTrait) -> Self { Self { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None } }
    pub fn dirty(&self) -> bool { self.map.dirty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); self.bin_box.get() } else { self.bin_box.get() } }
    pub fn hashCode(&self) -> i32 { 0 }
    pub fn equals(&mut self, _o: &mut Transition) -> bool { true }
    pub fn equalsLax(&mut self, _o: &mut Transition) -> bool { true }
}

// Java: public class SlideLayout implements KoAr, KoClosable
pub struct SlideLayout {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ColorMapping clrMap
    pub(crate) clrMap: Option<Box<ColorMapping>>, // Java field:clrMap // LLM: ColorMapping is a proto object, not java.util.Map
    // Java field: HeaderFooter hf
    pub(crate) hf: Option<Box<HeaderFooter>>,
    // Java field: Timing timing
    pub(crate) timing: Option<Box<Timing>>,
    // Java field: Transition transition
    pub(crate) transition: Option<Box<Transition>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: SlideLayout parent
    pub(crate) parent: Option<Box<SlideLayout>>,
}

// Java: public class SlideLayout implements KoAr, KoClosable
impl SlideLayout {
    // Java: static protected static <Value1 extends SlideLayout> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = SlideLayout { zone, clrMap: None, hf: None, timing: None, transition: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None };
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      inst
    }

    // Java: static SlideLayout create()
    pub fn create() -> SlideLayout {
      // Java: return initialize(SlideLayout.class, KoBin.Null.empty());
      Self::initialize(Null::empty().as_ref())
    }

    // Java: static SlideLayout open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> SlideLayout {
      // Java: return initialize(SlideLayout.class, bin);
      Self::initialize(bin)
    }

    // Java: SlideLayout()
    pub fn SlideLayout_new(&mut self) {
      // Java: this.box = new KpBinBox();
      // LLM: box renamed to bin_box (Rust keyword)
      self.bin_box = KpBinBox::new();
      // Java: this.map = new KpMap();
      self.map = KpMap::new();
    }

    // Java: SlideLayout(KoBin bin)
    pub fn SlideLayout_from(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.box = new KpBinBox(bin);
      // LLM: box renamed to bin_box (Rust keyword)
      self.bin_box = KpBinBox::KpBinBox_from(Some(bin));
      // Java: this.map = new KpMap(bin);
      self.map = KpMap::KpMap_from(bin);
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      // LLM: box renamed to bin_box (Rust keyword)
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin(bin, self.zone);
    }

    // Java: void load(SlideLayout value)
    pub fn load_from(&mut self, value: &mut SlideLayout) {
      // Java: if (value != null)
        if true {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = value.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.clrMap != null && this.clrMap.dirty())
        if self.clrMap.is_some() && self.clrMap.as_ref().unwrap().dirty() {
         // Java: this.map.put(1, this.clrMap.saveBin(zone));
         self.map.put(1, self.clrMap.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.hf != null && this.hf.dirty())
        if self.hf.is_some() && self.hf.as_ref().unwrap().dirty() {
         // Java: this.map.put(2, this.hf.saveBin(zone));
         self.map.put(2, self.hf.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.timing != null && this.timing.dirty())
        if self.timing.is_some() && self.timing.as_ref().unwrap().dirty() {
         // Java: this.map.put(3, this.timing.saveBin(zone));
         self.map.put(3, self.timing.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.transition != null && this.transition.dirty())
        if self.transition.is_some() && self.transition.as_ref().unwrap().dirty() {
         // Java: this.map.put(4, this.transition.saveBin(zone));
         self.map.put(4, self.transition.as_mut().unwrap().saveBin(zone));
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      return if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() };
    }

    // Java: boolean equalsLax(SlideLayout o)
    pub fn equalsLax(&mut self, o: &mut SlideLayout) -> bool {
      // Java: return this.equals(o, false);
      // LLM: equals renamed to equals_inner (Rust overload)
      return self.equals_inner(o, false);
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut SlideLayout) -> bool {
      // Java: return this.equals(o, true);
      // LLM: equals renamed to equals_inner (Rust overload)
      return self.equals_inner(o, true);
    }

    // Java: boolean equals(Object o, boolean isStrict)
    pub fn equals_inner(&mut self, o: &mut SlideLayout, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof SlideLayout)
        if true && true {
         // Java: SlideLayout dst = (SlideLayout)o;
         let dst = o;
         // Java: if (this == dst)
            if false {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasClrMap() || !dst.hasClrMap()) && (!this.hasClrMap() || dst.hasClrMap()))
            else if (self.hasClrMap() || !dst.hasClrMap()) && (!self.hasClrMap() || dst.hasClrMap()) {
            // Java: if (this.hasClrMap() && dst.hasClrMap())
                if self.hasClrMap() && dst.hasClrMap() {
               // Java: if (isStrict && !this.clrMap().equals(dst.clrMap()))
                    if isStrict && !self.clrMap().equals(&dst.clrMap()) {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.clrMap().equalsLax(dst.clrMap()))
                    if !isStrict && !self.clrMap().equalsLax(&dst.clrMap()) {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasHf() || !dst.hasHf()) && (!this.hasHf() || dst.hasHf()))
                if (self.hasHf() || !dst.hasHf()) && (!self.hasHf() || dst.hasHf()) {
               // Java: if (this.hasHf() && dst.hasHf())
                    if self.hasHf() && dst.hasHf() {
                  // Java: if (isStrict && !this.hf().equals(dst.hf()))
                        if isStrict && !self.hf().equals(&dst.hf()) {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.hf().equalsLax(dst.hf()))
                        if !isStrict && !self.hf().equalsLax(&dst.hf()) {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasTiming() || !dst.hasTiming()) && (!this.hasTiming() || dst.hasTiming()))
                    if (self.hasTiming() || !dst.hasTiming()) && (!self.hasTiming() || dst.hasTiming()) {
                  // Java: if (this.hasTiming() && dst.hasTiming())
                        if self.hasTiming() && dst.hasTiming() {
                     // Java: if (isStrict && !this.timing().equals(dst.timing()))
                            if isStrict && !self.timing().equals(&mut dst.timing()) {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.timing().equalsLax(dst.timing()))
                            if !isStrict && !self.timing().equalsLax(&mut dst.timing()) {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasTransition() || !dst.hasTransition()) && (!this.hasTransition() || dst.hasTransition()))
                        if (self.hasTransition() || !dst.hasTransition()) && (!self.hasTransition() || dst.hasTransition()) {
                     // Java: if (this.hasTransition() && dst.hasTransition())
                            if self.hasTransition() && dst.hasTransition() {
                        // Java: if (isStrict && !this.transition().equals(dst.transition()))
                                if isStrict && !self.transition().equals(&mut dst.transition()) {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.transition().equalsLax(dst.transition()))
                                if !isStrict && !self.transition().equalsLax(&mut dst.transition()) {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasShowMasterPhAnim() || !dst.hasShowMasterPhAnim()) && (!this.hasShowMasterPhAnim() || dst.hasShowMasterPhAnim()))
                            if (self.hasShowMasterPhAnim() || !dst.hasShowMasterPhAnim()) && (!self.hasShowMasterPhAnim() || dst.hasShowMasterPhAnim()) {
                        // Java: if (this.hasShowMasterPhAnim() && dst.hasShowMasterPhAnim() && this.isShowMasterPhAnim() != dst.isShowMasterPhAnim())
                                if self.hasShowMasterPhAnim() && dst.hasShowMasterPhAnim() && self.isShowMasterPhAnim() != dst.isShowMasterPhAnim() {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasShowMasterSp() || !dst.hasShowMasterSp()) && (!this.hasShowMasterSp() || dst.hasShowMasterSp()))
                                else if (self.hasShowMasterSp() || !dst.hasShowMasterSp()) && (!self.hasShowMasterSp() || dst.hasShowMasterSp()) {
                           // Java: if (this.hasShowMasterSp() && dst.hasShowMasterSp() && this.isShowMasterSp() != dst.isShowMasterSp())
                                    if self.hasShowMasterSp() && dst.hasShowMasterSp() && self.isShowMasterSp() != dst.isShowMasterSp() {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasMatchingName() || !dst.hasMatchingName()) && (!this.hasMatchingName() || dst.hasMatchingName()))
                                    else if (self.hasMatchingName() || !dst.hasMatchingName()) && (!self.hasMatchingName() || dst.hasMatchingName()) {
                              // Java: if (this.hasMatchingName() && dst.hasMatchingName() && !this.matchingName().equals(dst.matchingName()))
                                        if self.hasMatchingName() && dst.hasMatchingName() && self.matchingName() != dst.matchingName() {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasPreserve() || !dst.hasPreserve()) && (!this.hasPreserve() || dst.hasPreserve()))
                                        else if (self.hasPreserve() || !dst.hasPreserve()) && (!self.hasPreserve() || dst.hasPreserve()) {
                                 // Java: if (this.hasPreserve() && dst.hasPreserve() && this.isPreserve() != dst.isPreserve())
                                            if self.hasPreserve() && dst.hasPreserve() && self.isPreserve() != dst.isPreserve() {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasUserDraw() || !dst.hasUserDraw()) && (!this.hasUserDraw() || dst.hasUserDraw()))
                                            else if (self.hasUserDraw() || !dst.hasUserDraw()) && (!self.hasUserDraw() || dst.hasUserDraw()) {
                                    // Java: if (this.hasUserDraw() && dst.hasUserDraw() && this.isUserDraw() != dst.isUserDraw())
                                                if self.hasUserDraw() && dst.hasUserDraw() && self.isUserDraw() != dst.isUserDraw() {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                                                else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType()) {
                                       // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
                                       return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
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
        else {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasClrMap())
        if self.hasClrMap() {
         // Java: ret += this.clrMap().hashCode();
         ret = ret.wrapping_add(self.clrMap_ro().hashCode());
      }
      // Java: if (this.hasHf())
        if self.hasHf() {
         // Java: ret += this.hf().hashCode();
         ret = ret.wrapping_add(self.hf_ro().hashCode());
      }
      // Java: if (this.hasTiming())
        if self.hasTiming() {
         // Java: ret += this.timing().hashCode();
         ret = ret.wrapping_add(self.timing_ro().hashCode());
      }
      // Java: if (this.hasTransition())
        if self.hasTransition() {
         // Java: ret += this.transition().hashCode();
         ret = ret.wrapping_add(self.transition_ro().hashCode());
      }
      // Java: if (this.hasShowMasterPhAnim())
        if self.hasShowMasterPhAnim() {
         // Java: ret += Boolean.valueOf(this.isShowMasterPhAnim()).hashCode();
         let val = self.isShowMasterPhAnim();
         ret = ret.wrapping_add(if val { 1231 } else { 1237 });
      }
      // Java: if (this.hasShowMasterSp())
        if self.hasShowMasterSp() {
         // Java: ret += Boolean.valueOf(this.isShowMasterSp()).hashCode();
         let val = self.isShowMasterSp();
         ret = ret.wrapping_add(if val { 1231 } else { 1237 });
      }
      // Java: if (this.hasMatchingName())
        if self.hasMatchingName() {
         // Java: ret += String.valueOf(this.matchingName()).hashCode();
         ret = ret.wrapping_add(java_string_hashCode(&self.matchingName()));
      }
      // Java: if (this.hasPreserve())
        if self.hasPreserve() {
         // Java: ret += Boolean.valueOf(this.isPreserve()).hashCode();
         let val = self.isPreserve();
         ret = ret.wrapping_add(if val { 1231 } else { 1237 });
      }
      // Java: if (this.hasUserDraw())
        if self.hasUserDraw() {
         // Java: ret += Boolean.valueOf(this.isUserDraw()).hashCode();
         let val = self.isUserDraw();
         ret = ret.wrapping_add(if val { 1231 } else { 1237 });
      }
      // Java: if (this.hasType())
        if self.hasType() {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.r#type());
      }
      // Java: return ret;
      return ret;
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.clrMap != null && this.clrMap.dirty())
        if self.clrMap.is_some() && self.clrMap.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.hf != null && this.hf.dirty())
        else if self.hf.is_some() && self.hf.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.timing != null && this.timing.dirty())
        else if self.timing.is_some() && self.timing.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else
        else {
         // Java: return this.transition != null && this.transition.dirty() ? true : this.map.dirty();
         return if self.transition.is_some() && self.transition.as_ref().unwrap().dirty() { true } else { self.map.dirty() };
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
      // Java: return this.box.get();
      // LLM: box renamed to bin_box (Rust keyword)
      return self.bin_box.get();
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      // LLM: box renamed to bin_box (Rust keyword)
      return Factory::dup(self.bin_box.get().as_ref(), zone);
    }

    // Java: void discard(KoZone zone)
    // LLM: discard renamed to discard_with_zone (Rust overload)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      // LLM: box renamed to bin_box (Rust keyword)
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
      self.map.load(Null::value1().as_ref());
      // Java: this.box.clear();
      // LLM: box renamed to bin_box (Rust keyword)
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
      // Java: this.clrMap = null;
      self.clrMap = None;
      // Java: this.hf = null;
      self.hf = None;
      // Java: this.timing = null;
      self.timing = None;
      // Java: this.transition = null;
      self.transition = None;
    }

    // Java: void follow(SlideLayout parent)
    pub fn follow(&mut self, parent: Option<Box<SlideLayout>>) {
      // Java: if (parent != null)
        if let Some(mut parent) = parent {
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
        if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         self.parent.as_mut().unwrap().discard();
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      return self.parent.is_some();
    }

    // Java: SlideLayout parent()
    pub fn parent(&self) -> Option<&SlideLayout> {
      // Java: return this.parent;
      return self.parent.as_deref();
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM: discard renamed to discard_with_zone (Rust overload)
      self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: ColorMapping clrMap()
    pub fn clrMap(&mut self) -> &mut ColorMapping {
      // Java: if (this.clrMap == null)
        if self.clrMap.is_none() {
         // Java: if (this.map.has(1))
            if self.map.has(1) {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.clrMap = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            self.clrMap = Some(Box::new(ColorMapping::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasClrMap())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasClrMap() {
            // Java: this.clrMap = this.parent.clrMap();
            let parent_val = self.parent.as_mut().unwrap().clrMap();
            self.clrMap = Some(Box::new(ColorMapping::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.clrMap = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            self.clrMap = Some(Box::new(ColorMapping::open(bin.as_ref())));
         }
      }
      // Java: return this.clrMap;
      return self.clrMap.as_mut().unwrap();
    }

    fn clrMap_ro(&self) -> &ColorMapping {
      self.clrMap.as_ref().unwrap()
    }

    // Java: boolean hasClrMap()
    pub fn hasClrMap(&self) -> bool {
      // Java: return this.map.has(1) || this.clrMap != null && this.clrMap.dirty() || this.parent != null && this.parent.hasClrMap();
      return self.map.has(1) || self.clrMap.is_some() && self.clrMap.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_ref().map_or(false, |p| p.hasClrMap());
    }

    // Java: void deleteClrMap()
    pub fn deleteClrMap(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.clrMap != null)
        if self.clrMap.is_some() {
         // Java: this.clrMap = null;
         self.clrMap = None;
      }
    }

    // Java: void writeClrMap(ColorMapping value)
    pub fn writeClrMap(&mut self, value: &mut ColorMapping) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.clrMap != null)
                if self.clrMap.is_some() {
               // Java: this.clrMap = null;
               self.clrMap = None;
            }
         }
      }
    }

    // Java: void forceWriteClrMap(ColorMapping value)
    pub fn forceWriteClrMap(&mut self, value: &mut ColorMapping) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(1, value.lastBin(this.zone));
         let bin = value.lastBin(self.zone);
         self.map.put(1, bin);
         // Java: if (this.clrMap != null)
            if self.clrMap.is_some() {
            // Java: this.clrMap = null;
            self.clrMap = None;
         }
      }
    }

    // Java: HeaderFooter hf()
    pub fn hf(&mut self) -> &mut HeaderFooter {
      // Java: if (this.hf == null)
        if self.hf.is_none() {
         // Java: if (this.map.has(2))
            if self.map.has(2) {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.hf = (HeaderFooter)KoBin.Decode.toAny(HeaderFooter.class, bin, this.zone);
            self.hf = Some(Box::new(HeaderFooter::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasHf())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasHf() {
            // Java: this.hf = this.parent.hf();
            let parent_val = self.parent.as_mut().unwrap().hf();
            self.hf = Some(Box::new(HeaderFooter::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.hf = (HeaderFooter)KoBin.Decode.toAny(HeaderFooter.class, bin, this.zone);
            self.hf = Some(Box::new(HeaderFooter::open(bin.as_ref())));
         }
      }
      // Java: return this.hf;
      return self.hf.as_mut().unwrap();
    }

    fn hf_ro(&self) -> &HeaderFooter {
      self.hf.as_ref().unwrap()
    }

    // Java: boolean hasHf()
    pub fn hasHf(&self) -> bool {
      // Java: return this.map.has(2) || this.hf != null && this.hf.dirty() || this.parent != null && this.parent.hasHf();
      return self.map.has(2) || self.hf.is_some() && self.hf.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_ref().map_or(false, |p| p.hasHf());
    }

    // Java: void deleteHf()
    pub fn deleteHf(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.hf != null)
        if self.hf.is_some() {
         // Java: this.hf = null;
         self.hf = None;
      }
    }

    // Java: void writeHf(HeaderFooter value)
    pub fn writeHf(&mut self, value: &mut HeaderFooter) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(2, bin);
            self.map.put(2, bin);
            // Java: if (this.hf != null)
                if self.hf.is_some() {
               // Java: this.hf = null;
               self.hf = None;
            }
         }
      }
    }

    // Java: void forceWriteHf(HeaderFooter value)
    pub fn forceWriteHf(&mut self, value: &mut HeaderFooter) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(2, value.lastBin(this.zone));
         let bin = value.lastBin(self.zone);
         self.map.put(2, bin);
         // Java: if (this.hf != null)
            if self.hf.is_some() {
            // Java: this.hf = null;
            self.hf = None;
         }
      }
    }

    // Java: Timing timing()
    pub fn timing(&mut self) -> &mut Timing {
      // Java: if (this.timing == null)
        if self.timing.is_none() {
         // Java: if (this.map.has(3))
            if self.map.has(3) {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.timing = (Timing)KoBin.Decode.toAny(Timing.class, bin, this.zone);
            self.timing = Some(Box::new(Timing::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTiming())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasTiming() {
            // Java: this.timing = this.parent.timing();
            let parent_val = self.parent.as_mut().unwrap().timing();
            self.timing = Some(Box::new(Timing::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.timing = (Timing)KoBin.Decode.toAny(Timing.class, bin, this.zone);
            self.timing = Some(Box::new(Timing::open(bin.as_ref())));
         }
      }
      // Java: return this.timing;
      return self.timing.as_mut().unwrap();
    }

    fn timing_ro(&self) -> &Timing {
      self.timing.as_ref().unwrap()
    }

    // Java: boolean hasTiming()
    pub fn hasTiming(&self) -> bool {
      // Java: return this.map.has(3) || this.timing != null && this.timing.dirty() || this.parent != null && this.parent.hasTiming();
      return self.map.has(3) || self.timing.is_some() && self.timing.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_ref().map_or(false, |p| p.hasTiming());
    }

    // Java: void deleteTiming()
    pub fn deleteTiming(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.timing != null)
        if self.timing.is_some() {
         // Java: this.timing = null;
         self.timing = None;
      }
    }

    // Java: void writeTiming(Timing value)
    pub fn writeTiming(&mut self, value: &mut Timing) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.timing != null)
                if self.timing.is_some() {
               // Java: this.timing = null;
               self.timing = None;
            }
         }
      }
    }

    // Java: void forceWriteTiming(Timing value)
    pub fn forceWriteTiming(&mut self, value: &mut Timing) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(3, value.lastBin(this.zone));
         let bin = value.lastBin(self.zone);
         self.map.put(3, bin);
         // Java: if (this.timing != null)
            if self.timing.is_some() {
            // Java: this.timing = null;
            self.timing = None;
         }
      }
    }

    // Java: Transition transition()
    pub fn transition(&mut self) -> &mut Transition {
      // Java: if (this.transition == null)
        if self.transition.is_none() {
         // Java: if (this.map.has(4))
            if self.map.has(4) {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.transition = (Transition)KoBin.Decode.toAny(Transition.class, bin, this.zone);
            self.transition = Some(Box::new(Transition::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTransition())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasTransition() {
            // Java: this.transition = this.parent.transition();
            let parent_val = self.parent.as_mut().unwrap().transition();
            self.transition = Some(Box::new(Transition::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.transition = (Transition)KoBin.Decode.toAny(Transition.class, bin, this.zone);
            self.transition = Some(Box::new(Transition::open(bin.as_ref())));
         }
      }
      // Java: return this.transition;
      return self.transition.as_mut().unwrap();
    }

    fn transition_ro(&self) -> &Transition {
      self.transition.as_ref().unwrap()
    }

    // Java: boolean hasTransition()
    pub fn hasTransition(&self) -> bool {
      // Java: return this.map.has(4) || this.transition != null && this.transition.dirty() || this.parent != null && this.parent.hasTransition();
      return self.map.has(4) || self.transition.is_some() && self.transition.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_ref().map_or(false, |p| p.hasTransition());
    }

    // Java: void deleteTransition()
    pub fn deleteTransition(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.transition != null)
        if self.transition.is_some() {
         // Java: this.transition = null;
         self.transition = None;
      }
    }

    // Java: void writeTransition(Transition value)
    pub fn writeTransition(&mut self, value: &mut Transition) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.transition != null)
                if self.transition.is_some() {
               // Java: this.transition = null;
               self.transition = None;
            }
         }
      }
    }

    // Java: void forceWriteTransition(Transition value)
    pub fn forceWriteTransition(&mut self, value: &mut Transition) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(4, value.lastBin(this.zone));
         let bin = value.lastBin(self.zone);
         self.map.put(4, bin);
         // Java: if (this.transition != null)
            if self.transition.is_some() {
            // Java: this.transition = null;
            self.transition = None;
         }
      }
    }

    // Java: boolean isShowMasterPhAnim()
    pub fn isShowMasterPhAnim(&self) -> bool {
      // Java: if (!this.map.has(5))
        if !self.map.has(5) {
         // Java: return this.parent == null ? true : this.parent.isShowMasterPhAnim();
         return self.parent.as_ref().map_or(true, |p| p.isShowMasterPhAnim());
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShowMasterPhAnim()
    pub fn hasShowMasterPhAnim(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasShowMasterPhAnim();
      return self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasShowMasterPhAnim());
    }

    // Java: void deleteShowMasterPhAnim()
    pub fn deleteShowMasterPhAnim(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putShowMasterPhAnim(boolean value)
    pub fn putShowMasterPhAnim(&mut self, value: bool) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isShowMasterSp()
    pub fn isShowMasterSp(&self) -> bool {
      // Java: if (!this.map.has(6))
        if !self.map.has(6) {
         // Java: return this.parent == null ? true : this.parent.isShowMasterSp();
         return self.parent.as_ref().map_or(true, |p| p.isShowMasterSp());
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShowMasterSp()
    pub fn hasShowMasterSp(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasShowMasterSp();
      return self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasShowMasterSp());
    }

    // Java: void deleteShowMasterSp()
    pub fn deleteShowMasterSp(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putShowMasterSp(boolean value)
    pub fn putShowMasterSp(&mut self, value: bool) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_bool(value, self.zone));
    }

    // Java: String matchingName()
    pub fn matchingName(&self) -> String {
      // Java: if (!this.map.has(7))
        if !self.map.has(7) {
         // Java: return this.parent == null ? "" : this.parent.matchingName();
         return self.parent.as_ref().map_or(String::new(), |p| p.matchingName());
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasMatchingName()
    pub fn hasMatchingName(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasMatchingName();
      return self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasMatchingName());
    }

    // Java: void deleteMatchingName()
    pub fn deleteMatchingName(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putMatchingName(String value)
    pub fn putMatchingName(&mut self, value: &str) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_string(value, self.zone));
    }

    // Java: boolean isPreserve()
    pub fn isPreserve(&self) -> bool {
      // Java: if (!this.map.has(8))
        if !self.map.has(8) {
         // Java: return this.parent == null ? false : this.parent.isPreserve();
         return self.parent.as_ref().map_or(false, |p| p.isPreserve());
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasPreserve()
    pub fn hasPreserve(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasPreserve();
      return self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasPreserve());
    }

    // Java: void deletePreserve()
    pub fn deletePreserve(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }

    // Java: void putPreserve(boolean value)
    pub fn putPreserve(&mut self, value: bool) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(8, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isUserDraw()
    pub fn isUserDraw(&self) -> bool {
      // Java: if (!this.map.has(9))
        if !self.map.has(9) {
         // Java: return this.parent == null ? false : this.parent.isUserDraw();
         return self.parent.as_ref().map_or(false, |p| p.isUserDraw());
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasUserDraw()
    pub fn hasUserDraw(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasUserDraw();
      return self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasUserDraw());
    }

    // Java: void deleteUserDraw()
    pub fn deleteUserDraw(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putUserDraw(boolean value)
    pub fn putUserDraw(&mut self, value: bool) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(9, Encode::from_bool(value, self.zone));
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
      // Java: if (!this.map.has(10))
        if !self.map.has(10) {
         // Java: return this.parent == null ? 23 : this.parent.type();
         return self.parent.as_ref().map_or(23, |p| p.r#type());
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasType();
      return self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasType());
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_int(value, self.zone));
    }

}

fn java_string_hashCode(s: &str) -> i32 {
    let mut h: i32 = 0;
    for ch in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

impl KoAr for SlideLayout {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { SlideLayout::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { SlideLayout::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { SlideLayout::lastBin(self, var1) }
}

impl KoClosable for SlideLayout {
    fn close(&mut self) { SlideLayout::close(self); }
}
