#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::TextListStyle::TextListStyle;
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

// Java: public class SlideMaster implements KoAr, KoClosable
pub struct SlideMaster {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ColorMapping clrMap
    pub(crate) clrMap: Option<Box<ColorMapping>>, // Java field:clrMap // LLM: ColorMapping is a proto object, not java.util.Map
    // Java field: HeaderFooter hf
    pub(crate) hf: Option<Box<HeaderFooter>>,
    // Java field: TextListStyle bodyStyle
    pub(crate) bodyStyle: Option<Box<TextListStyle>>, // Java field:bodyStyle // LLM: TextListStyle is a proto object, not java.util.List
    // Java field: TextListStyle otherStyle
    pub(crate) otherStyle: Option<Box<TextListStyle>>, // Java field:otherStyle // LLM: TextListStyle is a proto object, not java.util.List
    // Java field: TextListStyle titleStyle
    pub(crate) titleStyle: Option<Box<TextListStyle>>, // Java field:titleStyle // LLM: TextListStyle is a proto object, not java.util.List
    // Java field: Timing timing
    pub(crate) timing: Option<Box<Timing>>,
    // Java field: Transition transition
    pub(crate) transition: Option<Box<Transition>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: SlideMaster parent
    pub(crate) parent: Option<Box<SlideMaster>>,
}

// Java: public class SlideMaster implements KoAr, KoClosable
impl SlideMaster {
    // Java: static protected static <Value1 extends SlideMaster> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::defaultZone();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = SlideMaster { zone, clrMap: None, hf: None, bodyStyle: None, otherStyle: None, titleStyle: None, timing: None, transition: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None };
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      return inst;
    }

    // Java: static SlideMaster create()
    pub fn create() -> SlideMaster {
      // Java: return initialize(SlideMaster.class, KoBin.Null.empty());
      return SlideMaster::new_from(Null::empty().as_ref());
    }

    // Java: static SlideMaster open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> SlideMaster {
      // Java: return initialize(SlideMaster.class, bin);
      return SlideMaster::new_from(bin);
    }

    // Java: SlideMaster()
    pub fn SlideMaster_new(&mut self) {
      // Java: this.box = new KpBinBox();
      // LLM: box renamed to bin_box (Rust keyword)
      self.bin_box = KpBinBox::new();
      // Java: this.map = new KpMap();
      self.map = KpMap::new();
    }

    // Java: SlideMaster(KoBin bin)
    pub fn SlideMaster_from(&mut self, bin: &dyn KoBinTrait) {
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
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      self.map.load(self.bin_box.get().as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin(bin, self.zone);
    }

    // Java: void load(SlideMaster value)
    pub fn load_from(&mut self, value: &mut SlideMaster) {
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
      // Java: if (this.bodyStyle != null && this.bodyStyle.dirty())
        if self.bodyStyle.is_some() && self.bodyStyle.as_ref().unwrap().dirty() {
         // Java: this.map.put(3, this.bodyStyle.saveBin(zone));
         self.map.put(3, self.bodyStyle.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.otherStyle != null && this.otherStyle.dirty())
        if self.otherStyle.is_some() && self.otherStyle.as_ref().unwrap().dirty() {
         // Java: this.map.put(4, this.otherStyle.saveBin(zone));
         self.map.put(4, self.otherStyle.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.titleStyle != null && this.titleStyle.dirty())
        if self.titleStyle.is_some() && self.titleStyle.as_ref().unwrap().dirty() {
         // Java: this.map.put(5, this.titleStyle.saveBin(zone));
         self.map.put(5, self.titleStyle.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.timing != null && this.timing.dirty())
        if self.timing.is_some() && self.timing.as_ref().unwrap().dirty() {
         // Java: this.map.put(7, this.timing.saveBin(zone));
         self.map.put(7, self.timing.as_mut().unwrap().saveBin(zone));
      }
      // Java: if (this.transition != null && this.transition.dirty())
        if self.transition.is_some() && self.transition.as_ref().unwrap().dirty() {
         // Java: this.map.put(8, this.transition.saveBin(zone));
         self.map.put(8, self.transition.as_mut().unwrap().saveBin(zone));
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      return if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() };
    }

    // Java: boolean equalsLax(SlideMaster o)
    pub fn equalsLax(&mut self, o: &mut SlideMaster) -> bool {
      // Java: return this.equals(o, false);
      // LLM: equals renamed to equals_inner (Rust overload)
      return self.equals_inner(o, false);
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut dyn std::any::Any) -> bool {
      // Java: return this.equals(o, true);
      // LLM: equals renamed to equals_inner (Rust overload)
      return self.equals_inner(o, true);
    }

    // Java: boolean equals(Object o, boolean isStrict)
    pub fn equals_inner(&mut self, o: &mut dyn std::any::Any, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof SlideMaster)
        if true && true {
         // Java: SlideMaster dst = (SlideMaster)o;
         let dst: &mut SlideMaster = unsafe { &mut *(o as *mut dyn std::any::Any as *mut SlideMaster) };
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
               // Java: if ((this.hasBodyStyle() || !dst.hasBodyStyle()) && (!this.hasBodyStyle() || dst.hasBodyStyle()))
                    if (self.hasBodyStyle() || !dst.hasBodyStyle()) && (!self.hasBodyStyle() || dst.hasBodyStyle()) {
                  // Java: if (this.hasBodyStyle() && dst.hasBodyStyle())
                        if self.hasBodyStyle() && dst.hasBodyStyle() {
                     // Java: if (isStrict && !this.bodyStyle().equals(dst.bodyStyle()))
                            if isStrict && !self.bodyStyle().equals(&dst.bodyStyle()) {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.bodyStyle().equalsLax(dst.bodyStyle()))
                            if !isStrict && !self.bodyStyle().equalsLax(&dst.bodyStyle()) {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasOtherStyle() || !dst.hasOtherStyle()) && (!this.hasOtherStyle() || dst.hasOtherStyle()))
                        if (self.hasOtherStyle() || !dst.hasOtherStyle()) && (!self.hasOtherStyle() || dst.hasOtherStyle()) {
                     // Java: if (this.hasOtherStyle() && dst.hasOtherStyle())
                            if self.hasOtherStyle() && dst.hasOtherStyle() {
                        // Java: if (isStrict && !this.otherStyle().equals(dst.otherStyle()))
                                if isStrict && !self.otherStyle().equals(&dst.otherStyle()) {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.otherStyle().equalsLax(dst.otherStyle()))
                                if !isStrict && !self.otherStyle().equalsLax(&dst.otherStyle()) {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasTitleStyle() || !dst.hasTitleStyle()) && (!this.hasTitleStyle() || dst.hasTitleStyle()))
                            if (self.hasTitleStyle() || !dst.hasTitleStyle()) && (!self.hasTitleStyle() || dst.hasTitleStyle()) {
                        // Java: if (this.hasTitleStyle() && dst.hasTitleStyle())
                                if self.hasTitleStyle() && dst.hasTitleStyle() {
                           // Java: if (isStrict && !this.titleStyle().equals(dst.titleStyle()))
                                    if isStrict && !self.titleStyle().equals(&dst.titleStyle()) {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.titleStyle().equalsLax(dst.titleStyle()))
                                    if !isStrict && !self.titleStyle().equalsLax(&dst.titleStyle()) {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasPreserve() || !dst.hasPreserve()) && (!this.hasPreserve() || dst.hasPreserve()))
                                if (self.hasPreserve() || !dst.hasPreserve()) && (!self.hasPreserve() || dst.hasPreserve()) {
                           // Java: if (this.hasPreserve() && dst.hasPreserve() && this.isPreserve() != dst.isPreserve())
                                    if self.hasPreserve() && dst.hasPreserve() && self.isPreserve() != dst.isPreserve() {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasTiming() || !dst.hasTiming()) && (!this.hasTiming() || dst.hasTiming()))
                                    else if (self.hasTiming() || !dst.hasTiming()) && (!self.hasTiming() || dst.hasTiming()) {
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret = 0;
      // Java: if (this.hasClrMap())
        if self.hasClrMap() {
         // Java: ret += this.clrMap().hashCode();
         ret += self.clrMap().hashCode();
      }
      // Java: if (this.hasHf())
        if self.hasHf() {
         // Java: ret += this.hf().hashCode();
         ret += self.hf().hashCode();
      }
      // Java: if (this.hasBodyStyle())
        if self.hasBodyStyle() {
         // Java: ret += this.bodyStyle().hashCode();
         ret += self.bodyStyle().hashCode();
      }
      // Java: if (this.hasOtherStyle())
        if self.hasOtherStyle() {
         // Java: ret += this.otherStyle().hashCode();
         ret += self.otherStyle().hashCode();
      }
      // Java: if (this.hasTitleStyle())
        if self.hasTitleStyle() {
         // Java: ret += this.titleStyle().hashCode();
         ret += self.titleStyle().hashCode();
      }
      // Java: if (this.hasPreserve())
        if self.hasPreserve() {
         // Java: ret += Boolean.valueOf(this.isPreserve()).hashCode();
         ret += if self.isPreserve() { 1231 } else { 1237 };
      }
      // Java: if (this.hasTiming())
        if self.hasTiming() {
         // Java: ret += this.timing().hashCode();
         ret += self.timing().hashCode();
      }
      // Java: if (this.hasTransition())
        if self.hasTransition() {
         // Java: ret += this.transition().hashCode();
         ret += self.transition().hashCode();
      }
      // Java: return ret;
      return ret;
    }

    // Java: boolean dirty()
    pub fn dirty(&mut self) -> bool {
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
      // Java: else if (this.bodyStyle != null && this.bodyStyle.dirty())
        else if self.bodyStyle.is_some() && self.bodyStyle.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.otherStyle != null && this.otherStyle.dirty())
        else if self.otherStyle.is_some() && self.otherStyle.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.titleStyle != null && this.titleStyle.dirty())
        else if self.titleStyle.is_some() && self.titleStyle.as_ref().unwrap().dirty() {
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
      self.saveBin(self.zone);
      // Java: return this.box.get();
      return self.bin_box.get();
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      return Factory::dup(self.bin_box.get().as_ref(), zone);
    }

    // Java: void discard(KoZone zone)
    pub fn discard_zone(&mut self, zone: &KoZone) {
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
      self.map.load(Null::value1().as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      // LLM Rust No Need: KoZone lifetime is managed by Rust's borrow checker, no explicit leave needed
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
      // Java: this.bodyStyle = null;
      self.bodyStyle = None;
      // Java: this.otherStyle = null;
      self.otherStyle = None;
      // Java: this.titleStyle = null;
      self.titleStyle = None;
      // Java: this.timing = null;
      self.timing = None;
      // Java: this.transition = null;
      self.transition = None;
    }

    // Java: void follow(SlideMaster parent)
    pub fn follow(&mut self, parent: &mut SlideMaster) {
      // Java: if (parent != null)
        if true {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         // LLM Platform-Specific: Java parent reference sharing cannot be directly translated to Rust Box ownership
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
    pub fn hasParent(&mut self) -> bool {
      // Java: return this.parent != null;
      return self.parent.is_some();
    }

    // Java: SlideMaster parent()
    pub fn parent(&mut self) -> &SlideMaster {
      // Java: return this.parent;
      return self.parent.as_ref().unwrap();
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM: discard renamed to discard_zone (Rust overload)
      self.discard_zone(self.zone);
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

    // Java: boolean hasClrMap()
    pub fn hasClrMap(&mut self) -> bool {
      // Java: return this.map.has(1) || this.clrMap != null && this.clrMap.dirty() || this.parent != null && this.parent.hasClrMap();
      return self.map.has(1) || self.clrMap.is_some() && self.clrMap.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasClrMap();
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
         self.map.put(1, value.lastBin(self.zone));
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

    // Java: boolean hasHf()
    pub fn hasHf(&mut self) -> bool {
      // Java: return this.map.has(2) || this.hf != null && this.hf.dirty() || this.parent != null && this.parent.hasHf();
      return self.map.has(2) || self.hf.is_some() && self.hf.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasHf();
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
         self.map.put(2, value.lastBin(self.zone));
         // Java: if (this.hf != null)
            if self.hf.is_some() {
            // Java: this.hf = null;
            self.hf = None;
         }
      }
    }

    // Java: TextListStyle bodyStyle()
    pub fn bodyStyle(&mut self) -> &mut TextListStyle {
      // Java: if (this.bodyStyle == null)
        if self.bodyStyle.is_none() {
         // Java: if (this.map.has(3))
            if self.map.has(3) {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.bodyStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.bodyStyle = Some(Box::new(TextListStyle::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBodyStyle())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasBodyStyle() {
            // Java: this.bodyStyle = this.parent.bodyStyle();
            let parent_val = self.parent.as_mut().unwrap().bodyStyle();
            self.bodyStyle = Some(Box::new(TextListStyle::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.bodyStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.bodyStyle = Some(Box::new(TextListStyle::open(bin.as_ref())));
         }
      }
      // Java: return this.bodyStyle;
      return self.bodyStyle.as_mut().unwrap();
    }

    // Java: boolean hasBodyStyle()
    pub fn hasBodyStyle(&mut self) -> bool {
      // Java: return this.map.has(3) || this.bodyStyle != null && this.bodyStyle.dirty() || this.parent != null && this.parent.hasBodyStyle();
      return self.map.has(3) || self.bodyStyle.is_some() && self.bodyStyle.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasBodyStyle();
    }

    // Java: void deleteBodyStyle()
    pub fn deleteBodyStyle(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.bodyStyle != null)
        if self.bodyStyle.is_some() {
         // Java: this.bodyStyle = null;
         self.bodyStyle = None;
      }
    }

    // Java: void writeBodyStyle(TextListStyle value)
    pub fn writeBodyStyle(&mut self, value: &mut TextListStyle) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.bodyStyle != null)
                if self.bodyStyle.is_some() {
               // Java: this.bodyStyle = null;
               self.bodyStyle = None;
            }
         }
      }
    }

    // Java: void forceWriteBodyStyle(TextListStyle value)
    pub fn forceWriteBodyStyle(&mut self, value: &mut TextListStyle) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, value.lastBin(self.zone));
         // Java: if (this.bodyStyle != null)
            if self.bodyStyle.is_some() {
            // Java: this.bodyStyle = null;
            self.bodyStyle = None;
         }
      }
    }

    // Java: TextListStyle otherStyle()
    pub fn otherStyle(&mut self) -> &mut TextListStyle {
      // Java: if (this.otherStyle == null)
        if self.otherStyle.is_none() {
         // Java: if (this.map.has(4))
            if self.map.has(4) {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.otherStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.otherStyle = Some(Box::new(TextListStyle::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasOtherStyle())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasOtherStyle() {
            // Java: this.otherStyle = this.parent.otherStyle();
            let parent_val = self.parent.as_mut().unwrap().otherStyle();
            self.otherStyle = Some(Box::new(TextListStyle::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.otherStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.otherStyle = Some(Box::new(TextListStyle::open(bin.as_ref())));
         }
      }
      // Java: return this.otherStyle;
      return self.otherStyle.as_mut().unwrap();
    }

    // Java: boolean hasOtherStyle()
    pub fn hasOtherStyle(&mut self) -> bool {
      // Java: return this.map.has(4) || this.otherStyle != null && this.otherStyle.dirty() || this.parent != null && this.parent.hasOtherStyle();
      return self.map.has(4) || self.otherStyle.is_some() && self.otherStyle.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasOtherStyle();
    }

    // Java: void deleteOtherStyle()
    pub fn deleteOtherStyle(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.otherStyle != null)
        if self.otherStyle.is_some() {
         // Java: this.otherStyle = null;
         self.otherStyle = None;
      }
    }

    // Java: void writeOtherStyle(TextListStyle value)
    pub fn writeOtherStyle(&mut self, value: &mut TextListStyle) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.otherStyle != null)
                if self.otherStyle.is_some() {
               // Java: this.otherStyle = null;
               self.otherStyle = None;
            }
         }
      }
    }

    // Java: void forceWriteOtherStyle(TextListStyle value)
    pub fn forceWriteOtherStyle(&mut self, value: &mut TextListStyle) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, value.lastBin(self.zone));
         // Java: if (this.otherStyle != null)
            if self.otherStyle.is_some() {
            // Java: this.otherStyle = null;
            self.otherStyle = None;
         }
      }
    }

    // Java: TextListStyle titleStyle()
    pub fn titleStyle(&mut self) -> &mut TextListStyle {
      // Java: if (this.titleStyle == null)
        if self.titleStyle.is_none() {
         // Java: if (this.map.has(5))
            if self.map.has(5) {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: this.titleStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.titleStyle = Some(Box::new(TextListStyle::open(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTitleStyle())
            else if self.parent.is_some() && self.parent.as_mut().unwrap().hasTitleStyle() {
            // Java: this.titleStyle = this.parent.titleStyle();
            let parent_val = self.parent.as_mut().unwrap().titleStyle();
            self.titleStyle = Some(Box::new(TextListStyle::open(parent_val.lastBin(self.zone).as_ref())));
         }
         // Java: else
            else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.titleStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.titleStyle = Some(Box::new(TextListStyle::open(bin.as_ref())));
         }
      }
      // Java: return this.titleStyle;
      return self.titleStyle.as_mut().unwrap();
    }

    // Java: boolean hasTitleStyle()
    pub fn hasTitleStyle(&mut self) -> bool {
      // Java: return this.map.has(5) || this.titleStyle != null && this.titleStyle.dirty() || this.parent != null && this.parent.hasTitleStyle();
      return self.map.has(5) || self.titleStyle.is_some() && self.titleStyle.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasTitleStyle();
    }

    // Java: void deleteTitleStyle()
    pub fn deleteTitleStyle(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.titleStyle != null)
        if self.titleStyle.is_some() {
         // Java: this.titleStyle = null;
         self.titleStyle = None;
      }
    }

    // Java: void writeTitleStyle(TextListStyle value)
    pub fn writeTitleStyle(&mut self, value: &mut TextListStyle) {
      // Java: if (value != null)
        if true {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(5, bin);
            self.map.put(5, bin);
            // Java: if (this.titleStyle != null)
                if self.titleStyle.is_some() {
               // Java: this.titleStyle = null;
               self.titleStyle = None;
            }
         }
      }
    }

    // Java: void forceWriteTitleStyle(TextListStyle value)
    pub fn forceWriteTitleStyle(&mut self, value: &mut TextListStyle) {
      // Java: if (value != null)
        if true {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, value.lastBin(self.zone));
         // Java: if (this.titleStyle != null)
            if self.titleStyle.is_some() {
            // Java: this.titleStyle = null;
            self.titleStyle = None;
         }
      }
    }

    // Java: boolean isPreserve()
    pub fn isPreserve(&mut self) -> bool {
      // Java: if (!this.map.has(6))
        if !self.map.has(6) {
         // Java: return this.parent == null ? false : this.parent.isPreserve();
         return if self.parent.is_none() { false } else { self.parent.as_mut().unwrap().isPreserve() };
      }
      // Java: else
        else {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasPreserve()
    pub fn hasPreserve(&mut self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasPreserve();
      return self.map.has(6) || self.parent.is_some() && self.parent.as_mut().unwrap().hasPreserve();
    }

    // Java: void deletePreserve()
    pub fn deletePreserve(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putPreserve(boolean value)
    pub fn putPreserve(&mut self, value: bool) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_bool(value, self.zone));
    }

    // Java: Timing timing()
    pub fn timing(&mut self) -> &mut Timing {
      // Java: if (this.timing == null)
        if self.timing.is_none() {
         // Java: if (this.map.has(7))
            if self.map.has(7) {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
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

    // Java: boolean hasTiming()
    pub fn hasTiming(&mut self) -> bool {
      // Java: return this.map.has(7) || this.timing != null && this.timing.dirty() || this.parent != null && this.parent.hasTiming();
      return self.map.has(7) || self.timing.is_some() && self.timing.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasTiming();
    }

    // Java: void deleteTiming()
    pub fn deleteTiming(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
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
            // Java: this.map.put(7, bin);
            self.map.put(7, bin);
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
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, value.lastBin(self.zone));
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
         // Java: if (this.map.has(8))
            if self.map.has(8) {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
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

    // Java: boolean hasTransition()
    pub fn hasTransition(&mut self) -> bool {
      // Java: return this.map.has(8) || this.transition != null && this.transition.dirty() || this.parent != null && this.parent.hasTransition();
      return self.map.has(8) || self.transition.is_some() && self.transition.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_mut().unwrap().hasTransition();
    }

    // Java: void deleteTransition()
    pub fn deleteTransition(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
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
            // Java: this.map.put(8, bin);
            self.map.put(8, bin);
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
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, value.lastBin(self.zone));
         // Java: if (this.transition != null)
            if self.transition.is_some() {
            // Java: this.transition = null;
            self.transition = None;
         }
      }
    }

}

impl KoAr for SlideMaster {
    fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) { SlideMaster::loadBin(self, bin, zone); }
    fn saveBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    fn dirty(&self) -> bool { self.map.dirty() }
    fn discard(&mut self, zone: &KoZone) { self.discard_zone(zone); }
    fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
}

impl KoClosable for SlideMaster {
    fn close(&mut self) { SlideMaster::close(self); }
}
