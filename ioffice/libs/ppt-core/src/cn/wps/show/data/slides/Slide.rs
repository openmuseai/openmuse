#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::collections::HashMap;
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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import cn.wps.show.data.comment.Comment
use crate::cn::wps::show::data::comment::Comment::Comment;
// Java: import cn.wps.show.data.timing.Timing
// LLM: Timing module not yet available, using inline stub below
// Java: import cn.wps.show.data.transition.Transition
// LLM: Transition module not yet available, using inline stub below
// Java: import java.util.ArrayList
// LLM Rust No Need: using Vec instead
// Java: import java.util.Collection
// LLM Rust No Need: using Vec/slice instead

type KpHashMap = HashMap<i32, Option<Box<dyn KoBinTrait>>>;

trait KpMapOps {
    fn kp_new() -> Self;
    fn kp_from(bin: &dyn KoBinTrait) -> Self;
    fn has(&self, key: i32) -> bool;
    fn byKey(&self, key: i32) -> Box<dyn KoBinTrait>;
    fn put(&mut self, key: i32, value: Box<dyn KoBinTrait>);
    fn clear_key(&mut self, key: i32);
    fn dirty(&self) -> bool;
    fn save(&mut self, _zone: &KoZone) -> Box<dyn KoBinTrait>;
    fn load(&mut self, _bin: &dyn KoBinTrait);
}

impl KpMapOps for KpHashMap {
    fn kp_new() -> Self { HashMap::new() }
    fn kp_from(_bin: &dyn KoBinTrait) -> Self { HashMap::new() }
    fn has(&self, key: i32) -> bool { matches!(self.get(&key), Some(Some(_))) }
    fn byKey(&self, key: i32) -> Box<dyn KoBinTrait> {
        match self.get(&key) {
            Some(Some(v)) => Factory::dup(v.as_ref(), KoZone::enter()),
            _ => Null::value1(),
        }
    }
    fn put(&mut self, key: i32, value: Box<dyn KoBinTrait>) { self.insert(key, Some(value)); }
    fn clear_key(&mut self, key: i32) { self.insert(key, None); }
    fn dirty(&self) -> bool { !self.is_empty() }
    fn save(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { Encode::fromBH(0, zone) }
    fn load(&mut self, _bin: &dyn KoBinTrait) { self.clear(); }
}

struct KpVecList {
    list: Vec<Box<dyn KoBinTrait>>,
    dirtied: bool,
}
impl KpVecList {
    fn new(_zone: &KoZone) -> Self { KpVecList { list: Vec::new(), dirtied: false } }
    fn from_bin(_bin: &dyn KoBinTrait, _zone: &KoZone) -> Self { KpVecList { list: Vec::new(), dirtied: false } }
    fn size(&self) -> i32 { self.list.len() as i32 }
    fn get(&self, index: i32) -> &dyn KoBinTrait { self.list[index as usize].as_ref() }
    fn add(&mut self, bin: Box<dyn KoBinTrait>) { self.dirtied = true; self.list.push(bin); }
    fn clear(&mut self) { if !self.list.is_empty() { self.dirtied = true; self.list.clear(); } }
    fn dirty(&self) -> bool { self.dirtied }
    fn load(&mut self, _bin: &dyn KoBinTrait, _zone: &KoZone) { self.dirtied = false; self.list.clear(); }
    fn save(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { self.dirtied = false; Encode::fromBH(0, zone) }
    fn hashCode(&self) -> i32 { let mut h: i32 = 1; for item in &self.list { h = h.wrapping_mul(31).wrapping_add(item.asInt()); } h }
}

const SLIDE_SHOW: i32 = 1;
const SLIDE_SHOW_MASTER_PH_ANIM: i32 = 2;
const SLIDE_SHOW_MASTER_SP: i32 = 3;
const SLIDE_CLR_MAP: i32 = 4;
const SLIDE_TIMING: i32 = 5;
const SLIDE_TRANSITION: i32 = 6;
const SLIDE_COMMENT_LST: i32 = 7;

pub struct Timing {
    pub(crate) zone: &'static KoZone,
    pub(crate) bin_box: KpBinBox,
    pub(crate) map: KpMap,
    pub(crate) parent: Option<Box<Timing>>,
}
impl Timing {
    pub fn new() -> Self { Timing { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None } }
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) { self.bin_box.set(Some(bin)); let b = self.bin_box.get(); self.map.load(b.as_ref()); }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())) } else { self.bin_box.get() } }
    pub fn dirty(&self) -> bool { self.map.dirty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    pub fn hashCode(&self) -> i32 { 0 }
    pub fn equals(&mut self, _o: &mut Timing) -> bool { true }
    pub fn equalsLax(&mut self, _o: &mut Timing) -> bool { true }
}

pub struct Transition {
    pub(crate) zone: &'static KoZone,
    pub(crate) bin_box: KpBinBox,
    pub(crate) map: KpMap,
    pub(crate) parent: Option<Box<Transition>>,
}
impl Transition {
    pub fn new() -> Self { Transition { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None } }
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) { self.bin_box.set(Some(bin)); let b = self.bin_box.get(); self.map.load(b.as_ref()); }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())) } else { self.bin_box.get() } }
    pub fn dirty(&self) -> bool { self.map.dirty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    pub fn hashCode(&self) -> i32 { 0 }
    pub fn equals(&mut self, _o: &mut Transition) -> bool { true }
    pub fn equalsLax(&mut self, _o: &mut Transition) -> bool { true }
}

// Java: public class Slide implements KoAr, KoClosable
pub struct Slide {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ColorMapping clrMap
    pub(crate) clrMap: Option<Box<ColorMapping>>, // Java field:clrMap // LLM: ColorMapping is a proto object, not java.util.Map
    // Java field: Timing timing
    pub(crate) timing: Option<Box<Timing>>,
    // Java field: Transition transition
    pub(crate) transition: Option<Box<Transition>>,
    // Java field: CommentList commentLst
    pub(crate) commentLst: Option<Box<CommentList>>, // Java field:commentLst // LLM Not Collection: Java type is CommentList (inner class wrapping KpList), not java.util.List
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: Slide parent
    pub(crate) parent: Option<Box<Slide>>,
}

// Java: public class CommentList
pub struct CommentList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    // LLM Not Collection: Java type is KpList (internal proto), not java.util.List
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Slide implements KoAr, KoClosable
impl Slide {
    fn _default() -> Self {
        Slide { zone: KoZone::defaultZone(), clrMap: None, timing: None, transition: None, commentLst: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: static protected static <Value1 extends Slide> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Slide::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      inst
    }

    // Java: static Slide create()
    pub fn create() -> Self {
      // Java: return initialize(Slide.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Slide open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Slide.class, bin);
      Self::initialize(bin)
    }

    // Java: Slide()
    pub fn new() -> Self {
      let mut inst = Self::_default();
      // Java: this.box = new KpBinBox();
      // LLM Rust Renamed: box is reserved keyword, using self.bin_box
      inst.bin_box = KpBinBox::new();
      // Java: this.map = new KpMap();
      // LLM Rust Pattern: factory constructor uses inst instead of self
      inst.map = KpMap::new();
      inst
    }

    // Java: Slide(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait) -> Self {
      let mut inst = Self::_default();
      // Java: this.box = new KpBinBox(bin);
      // LLM Rust Renamed: box is reserved keyword, using self.bin_box
      inst.bin_box = KpBinBox::KpBinBox_from(Some(bin));
      // Java: this.map = new KpMap(bin);
      // LLM Rust Pattern: factory constructor uses inst instead of self
      inst.map = KpMap::KpMap_from(bin);
      inst
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
      let b = self.bin_box.get();
      self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin(bin, self.zone);
    }

    // Java: void load(Slide value)
    pub fn load_from(&mut self, value: Option<&Slide>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let last = val.lastBin(self.zone);
         self.loadBin(last.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.clrMap != null && this.clrMap.dirty())
      if self.clrMap.is_some() && self.clrMap.as_ref().unwrap().dirty() {
         // Java: this.map.put(4, this.clrMap.saveBin(zone));
         let b = self.clrMap.as_mut().unwrap().saveBin(zone);
         self.map.put(SLIDE_CLR_MAP, b);
      }
      // Java: if (this.timing != null && this.timing.dirty())
      if self.timing.is_some() && self.timing.as_ref().unwrap().dirty() {
         // Java: this.map.put(5, this.timing.saveBin(zone));
         let b = self.timing.as_mut().unwrap().saveBin(zone);
         self.map.put(SLIDE_TIMING, b);
      }
      // Java: if (this.transition != null && this.transition.dirty())
      if self.transition.is_some() && self.transition.as_ref().unwrap().dirty() {
         // Java: this.map.put(6, this.transition.saveBin(zone));
         let b = self.transition.as_mut().unwrap().saveBin(zone);
         self.map.put(SLIDE_TRANSITION, b);
      }
      // Java: if (this.commentLst != null && this.commentLst.dirty())
      if self.commentLst.is_some() && self.commentLst.as_ref().unwrap().dirty() {
         // Java: this.map.put(7, this.commentLst.saveBin(zone));
         let b = self.commentLst.as_mut().unwrap().saveBin(zone);
         self.map.put(SLIDE_COMMENT_LST, b);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
          let saved = self.map.save(zone);
          self.bin_box.set(Some(saved.as_ref()))
      } else {
          self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(Slide o)
    pub fn equalsLax(&mut self, o: &mut Slide) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut Slide) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    pub fn equals(&mut self, o: &mut Slide, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Slide)
      if true && true {
         // Java: Slide dst = (Slide)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         // Java: else if ((this.hasShow() || !dst.hasShow()) && (!this.hasShow() || dst.hasShow()))
         } else if (self.hasShow() || !dst.hasShow()) && (!self.hasShow() || dst.hasShow()) {
            // Java: if (this.hasShow() && dst.hasShow() && this.isShow() != dst.isShow())
            if self.hasShow() && dst.hasShow() && self.isShow() != dst.isShow() {
               // Java: return false;
               return false;
            // Java: else if ((this.hasShowMasterPhAnim() || !dst.hasShowMasterPhAnim()) && (!this.hasShowMasterPhAnim() || dst.hasShowMasterPhAnim()))
            } else if (self.hasShowMasterPhAnim() || !dst.hasShowMasterPhAnim()) && (!self.hasShowMasterPhAnim() || dst.hasShowMasterPhAnim()) {
               // Java: if (this.hasShowMasterPhAnim() && dst.hasShowMasterPhAnim() && this.isShowMasterPhAnim() != dst.isShowMasterPhAnim())
               if self.hasShowMasterPhAnim() && dst.hasShowMasterPhAnim() && self.isShowMasterPhAnim() != dst.isShowMasterPhAnim() {
                  // Java: return false;
                  return false;
               // Java: else if ((this.hasShowMasterSp() || !dst.hasShowMasterSp()) && (!this.hasShowMasterSp() || dst.hasShowMasterSp()))
               } else if (self.hasShowMasterSp() || !dst.hasShowMasterSp()) && (!self.hasShowMasterSp() || dst.hasShowMasterSp()) {
                  // Java: if (this.hasShowMasterSp() && dst.hasShowMasterSp() && this.isShowMasterSp() != dst.isShowMasterSp())
                  if self.hasShowMasterSp() && dst.hasShowMasterSp() && self.isShowMasterSp() != dst.isShowMasterSp() {
                     // Java: return false;
                     return false;
                  // Java: else if ((this.hasClrMap() || !dst.hasClrMap()) && (!this.hasClrMap() || dst.hasClrMap()))
                  } else if (self.hasClrMap() || !dst.hasClrMap()) && (!self.hasClrMap() || dst.hasClrMap()) {
                     // Java: if (this.hasClrMap() && dst.hasClrMap())
                     if self.hasClrMap() && dst.hasClrMap() {
                        // Java: if (isStrict && !this.clrMap().equals(dst.clrMap()))
                        if isStrict && !self.clrMap().equals(dst.clrMap()) {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.clrMap().equalsLax(dst.clrMap()))
                        if !isStrict && !self.clrMap().equalsLax(dst.clrMap()) {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasTiming() || !dst.hasTiming()) && (!this.hasTiming() || dst.hasTiming()))
                     if (self.hasTiming() || !dst.hasTiming()) && (!self.hasTiming() || dst.hasTiming()) {
                        // Java: if (this.hasTiming() && dst.hasTiming())
                        if self.hasTiming() && dst.hasTiming() {
                           // Java: if (isStrict && !this.timing().equals(dst.timing()))
                           if isStrict && !self.timing().equals(dst.timing()) {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.timing().equalsLax(dst.timing()))
                           if !isStrict && !self.timing().equalsLax(dst.timing()) {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasTransition() || !dst.hasTransition()) && (!this.hasTransition() || dst.hasTransition()))
                        if (self.hasTransition() || !dst.hasTransition()) && (!self.hasTransition() || dst.hasTransition()) {
                           // Java: if (this.hasTransition() && dst.hasTransition())
                           if self.hasTransition() && dst.hasTransition() {
                              // Java: if (isStrict && !this.transition().equals(dst.transition()))
                              if isStrict && !self.transition().equals(dst.transition()) {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.transition().equalsLax(dst.transition()))
                              if !isStrict && !self.transition().equalsLax(dst.transition()) {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: if ((this.hasCommentLst() || !dst.hasCommentLst()) && (!this.hasCommentLst() || dst.hasCommentLst()))
                           if (self.hasCommentLst() || !dst.hasCommentLst()) && (!self.hasCommentLst() || dst.hasCommentLst()) {
                              // Java: if (this.hasCommentLst() && dst.hasCommentLst())
                              if self.hasCommentLst() && dst.hasCommentLst() {
                                 // Java: if (isStrict && !this.commentLst().equals(dst.commentLst()))
                                 if isStrict && !self.commentLst().equals(dst.commentLst(), true) {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.commentLst().equalsLax(dst.commentLst()))
                                 if !isStrict && !self.commentLst().equalsLax(dst.commentLst()) {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: return true;
                              return true;
                           // Java: else
                           } else {
                              // Java: return false;
                              return false;
                           }
                        // Java: else
                        } else {
                           // Java: return false;
                           return false;
                        }
                     // Java: else
                     } else {
                        // Java: return false;
                        return false;
                     }
                  // Java: else
                  } else {
                     // Java: return false;
                     return false;
                  }
               // Java: else
               } else {
                  // Java: return false;
                  return false;
               }
            // Java: else
            } else {
               // Java: return false;
               return false;
            }
         // Java: else
         } else {
            // Java: return false;
            return false;
         }
      // Java: else
      } else {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasShow())
      if self.hasShow() {
         // Java: ret += Boolean.valueOf(this.isShow()).hashCode();
         ret += if self.isShow() { 1231 } else { 1237 };
      }
      // Java: if (this.hasShowMasterPhAnim())
      if self.hasShowMasterPhAnim() {
         // Java: ret += Boolean.valueOf(this.isShowMasterPhAnim()).hashCode();
         ret += if self.isShowMasterPhAnim() { 1231 } else { 1237 };
      }
      // Java: if (this.hasShowMasterSp())
      if self.hasShowMasterSp() {
         // Java: ret += Boolean.valueOf(this.isShowMasterSp()).hashCode();
         ret += if self.isShowMasterSp() { 1231 } else { 1237 };
      }
      // Java: if (this.hasClrMap())
      if self.hasClrMap() {
         // Java: ret += this.clrMap().hashCode();
         ret += self.clrMap().hashCode();
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
      // Java: if (this.hasCommentLst())
      if self.hasCommentLst() {
         // Java: ret += this.commentLst().hashCode();
         ret += self.commentLst().hashCode();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.clrMap != null && this.clrMap.dirty())
      if self.clrMap.is_some() && self.clrMap.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      // Java: else if (this.timing != null && this.timing.dirty())
      } else if self.timing.is_some() && self.timing.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      // Java: else if (this.transition != null && this.transition.dirty())
      } else if self.transition.is_some() && self.transition.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      // Java: else
      } else {
         // Java: return this.commentLst != null && this.commentLst.dirty() ? true : this.map.dirty();
         return (self.commentLst.is_some() && self.commentLst.as_ref().unwrap().dirty()) || self.map.dirty();
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
      let b = self.bin_box.get();
      self.loadBin(b.as_ref(), zone);
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
      let v = Null::value1();
      self.map.load(v.as_ref());
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
      // Java: this.clrMap = null;
      self.clrMap = None;
      // Java: this.timing = null;
      self.timing = None;
      // Java: this.transition = null;
      self.transition = None;
      // Java: this.commentLst = null;
      self.commentLst = None;
    }

    // Java: void follow(Slide parent)
    pub fn follow(&mut self, parent: Option<&mut Slide>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         // LLM No Rust equivalent: cannot move borrowed reference into owned Box; parent relationship managed externally
         self.parent = None;
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
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

    // Java: Slide parent()
    pub fn parent(&self) -> Option<&Slide> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: boolean isShow()
    pub fn isShow(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(SLIDE_SHOW) {
         // Java: return this.parent == null ? true : this.parent.isShow();
         return self.parent.as_ref().map_or(true, |p| p.isShow());
      // Java: else
      } else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(SLIDE_SHOW);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShow()
    pub fn hasShow(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasShow();
      self.map.has(SLIDE_SHOW) || self.parent.as_ref().map_or(false, |p| p.hasShow())
    }

    // Java: void deleteShow()
    pub fn deleteShow(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(SLIDE_SHOW);
    }

    // Java: void putShow(boolean value)
    pub fn putShow(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(SLIDE_SHOW, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isShowMasterPhAnim()
    pub fn isShowMasterPhAnim(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(SLIDE_SHOW_MASTER_PH_ANIM) {
         // Java: return this.parent == null ? true : this.parent.isShowMasterPhAnim();
         return self.parent.as_ref().map_or(true, |p| p.isShowMasterPhAnim());
      // Java: else
      } else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(SLIDE_SHOW_MASTER_PH_ANIM);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShowMasterPhAnim()
    pub fn hasShowMasterPhAnim(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasShowMasterPhAnim();
      self.map.has(SLIDE_SHOW_MASTER_PH_ANIM) || self.parent.as_ref().map_or(false, |p| p.hasShowMasterPhAnim())
    }

    // Java: void deleteShowMasterPhAnim()
    pub fn deleteShowMasterPhAnim(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(SLIDE_SHOW_MASTER_PH_ANIM);
    }

    // Java: void putShowMasterPhAnim(boolean value)
    pub fn putShowMasterPhAnim(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(SLIDE_SHOW_MASTER_PH_ANIM, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isShowMasterSp()
    pub fn isShowMasterSp(&self) -> bool {
      // Java: if (!this.map.has(3))
      if !self.map.has(SLIDE_SHOW_MASTER_SP) {
         // Java: return this.parent == null ? true : this.parent.isShowMasterSp();
         return self.parent.as_ref().map_or(true, |p| p.isShowMasterSp());
      // Java: else
      } else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(SLIDE_SHOW_MASTER_SP);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShowMasterSp()
    pub fn hasShowMasterSp(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasShowMasterSp();
      self.map.has(SLIDE_SHOW_MASTER_SP) || self.parent.as_ref().map_or(false, |p| p.hasShowMasterSp())
    }

    // Java: void deleteShowMasterSp()
    pub fn deleteShowMasterSp(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(SLIDE_SHOW_MASTER_SP);
    }

    // Java: void putShowMasterSp(boolean value)
    pub fn putShowMasterSp(&mut self, value: bool) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(SLIDE_SHOW_MASTER_SP, Encode::from_bool(value, self.zone));
    }

    // Java: ColorMapping clrMap()
    pub fn clrMap(&mut self) -> &mut ColorMapping {
      // Java: if (this.clrMap == null)
      if self.clrMap.is_none() {
         // Java: if (this.map.has(4))
         if self.map.has(SLIDE_CLR_MAP) {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(SLIDE_CLR_MAP);
            // Java: this.clrMap = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            let mut inst = ColorMapping::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.clrMap = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasClrMap())
         } else if self.parent.is_some() && self.parent.as_ref().unwrap().hasClrMap() {
            // Java: this.clrMap = this.parent.clrMap();
            let mut inst = ColorMapping::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.clrMap = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.clrMap = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            let mut inst = ColorMapping::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.clrMap = Some(Box::new(inst));
         }
      }
      // Java: return this.clrMap;
      self.clrMap.as_mut().unwrap()
    }

    // Java: boolean hasClrMap()
    pub fn hasClrMap(&self) -> bool {
      // Java: return this.map.has(4) || this.clrMap != null && this.clrMap.dirty() || this.parent != null && this.parent.hasClrMap();
      self.map.has(SLIDE_CLR_MAP) || self.clrMap.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasClrMap())
    }

    // Java: void deleteClrMap()
    pub fn deleteClrMap(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(SLIDE_CLR_MAP);
      // Java: if (this.clrMap != null)
      if self.clrMap.is_some() {
         // Java: this.clrMap = null;
         self.clrMap = None;
      }
    }

    // Java: void writeClrMap(ColorMapping value)
    pub fn writeClrMap(&mut self, value: Option<&ColorMapping>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = val.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(4, bin);
            self.map.put(SLIDE_CLR_MAP, bin);
            // Java: if (this.clrMap != null)
            if self.clrMap.is_some() {
               // Java: this.clrMap = null;
               self.clrMap = None;
            }
         }
      }
    }

    // Java: void forceWriteClrMap(ColorMapping value)
    pub fn forceWriteClrMap(&mut self, value: Option<&ColorMapping>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: this.map.put(4, value.lastBin(this.zone));
         let bin = val.lastBin(self.zone);
         self.map.put(SLIDE_CLR_MAP, bin);
         // Java: if (this.clrMap != null)
         if self.clrMap.is_some() {
            // Java: this.clrMap = null;
            self.clrMap = None;
         }
      }
    }

    // Java: Timing timing()
    pub fn timing(&mut self) -> &mut Timing {
      // Java: if (this.timing == null)
      if self.timing.is_none() {
         // Java: if (this.map.has(5))
         if self.map.has(SLIDE_TIMING) {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(SLIDE_TIMING);
            // Java: this.timing = (Timing)KoBin.Decode.toAny(Timing.class, bin, this.zone);
            let mut inst = Timing::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.timing = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasTiming())
         } else if self.parent.is_some() && self.parent.as_ref().unwrap().hasTiming() {
            // Java: this.timing = this.parent.timing();
            let mut inst = Timing::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.timing = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.timing = (Timing)KoBin.Decode.toAny(Timing.class, bin, this.zone);
            let mut inst = Timing::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.timing = Some(Box::new(inst));
         }
      }
      // Java: return this.timing;
      self.timing.as_mut().unwrap()
    }

    // Java: boolean hasTiming()
    pub fn hasTiming(&self) -> bool {
      // Java: return this.map.has(5) || this.timing != null && this.timing.dirty() || this.parent != null && this.parent.hasTiming();
      self.map.has(SLIDE_TIMING) || self.timing.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTiming())
    }

    // Java: void deleteTiming()
    pub fn deleteTiming(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(SLIDE_TIMING);
      // Java: if (this.timing != null)
      if self.timing.is_some() {
         // Java: this.timing = null;
         self.timing = None;
      }
    }

    // Java: void writeTiming(Timing value)
    pub fn writeTiming(&mut self, value: Option<&Timing>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = val.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(5, bin);
            self.map.put(SLIDE_TIMING, bin);
            // Java: if (this.timing != null)
            if self.timing.is_some() {
               // Java: this.timing = null;
               self.timing = None;
            }
         }
      }
    }

    // Java: void forceWriteTiming(Timing value)
    pub fn forceWriteTiming(&mut self, value: Option<&Timing>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: this.map.put(5, value.lastBin(this.zone));
         let bin = val.lastBin(self.zone);
         self.map.put(SLIDE_TIMING, bin);
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
         // Java: if (this.map.has(6))
         if self.map.has(SLIDE_TRANSITION) {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(SLIDE_TRANSITION);
            // Java: this.transition = (Transition)KoBin.Decode.toAny(Transition.class, bin, this.zone);
            let mut inst = Transition::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.transition = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasTransition())
         } else if self.parent.is_some() && self.parent.as_ref().unwrap().hasTransition() {
            // Java: this.transition = this.parent.transition();
            let mut inst = Transition::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.transition = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.transition = (Transition)KoBin.Decode.toAny(Transition.class, bin, this.zone);
            let mut inst = Transition::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.transition = Some(Box::new(inst));
         }
      }
      // Java: return this.transition;
      self.transition.as_mut().unwrap()
    }

    // Java: boolean hasTransition()
    pub fn hasTransition(&self) -> bool {
      // Java: return this.map.has(6) || this.transition != null && this.transition.dirty() || this.parent != null && this.parent.hasTransition();
      self.map.has(SLIDE_TRANSITION) || self.transition.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTransition())
    }

    // Java: void deleteTransition()
    pub fn deleteTransition(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(SLIDE_TRANSITION);
      // Java: if (this.transition != null)
      if self.transition.is_some() {
         // Java: this.transition = null;
         self.transition = None;
      }
    }

    // Java: void writeTransition(Transition value)
    pub fn writeTransition(&mut self, value: Option<&Transition>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = val.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(6, bin);
            self.map.put(SLIDE_TRANSITION, bin);
            // Java: if (this.transition != null)
            if self.transition.is_some() {
               // Java: this.transition = null;
               self.transition = None;
            }
         }
      }
    }

    // Java: void forceWriteTransition(Transition value)
    pub fn forceWriteTransition(&mut self, value: Option<&Transition>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: this.map.put(6, value.lastBin(this.zone));
         let bin = val.lastBin(self.zone);
         self.map.put(SLIDE_TRANSITION, bin);
         // Java: if (this.transition != null)
         if self.transition.is_some() {
            // Java: this.transition = null;
            self.transition = None;
         }
      }
    }

    // Java: CommentList commentLst()
    pub fn commentLst(&mut self) -> &mut CommentList {
      // Java: if (this.commentLst == null)
      if self.commentLst.is_none() {
         // Java: if (this.map.has(7))
         if self.map.has(SLIDE_COMMENT_LST) {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(SLIDE_COMMENT_LST);
            // Java: this.commentLst = (CommentList)KoBin.Decode.toAny(CommentList.class, bin, this.zone);
            let mut inst = CommentList::new(self.zone);
            inst.loadBin(bin.as_ref(), self.zone);
            self.commentLst = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasCommentLst())
         } else if self.parent.is_some() && self.parent.as_ref().unwrap().hasCommentLst() {
            // Java: this.commentLst = this.parent.commentLst();
            let mut inst = CommentList::new(self.zone);
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.commentLst = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.commentLst = (CommentList)KoBin.Decode.toAny(CommentList.class, bin, this.zone);
            let mut inst = CommentList::new(self.zone);
            inst.loadBin(bin.as_ref(), self.zone);
            self.commentLst = Some(Box::new(inst));
         }
      }
      // Java: return this.commentLst;
      self.commentLst.as_mut().unwrap()
    }

    // Java: boolean hasCommentLst()
    pub fn hasCommentLst(&self) -> bool {
      // Java: return this.map.has(7) || this.commentLst != null && this.commentLst.dirty() || this.parent != null && this.parent.hasCommentLst();
      self.map.has(SLIDE_COMMENT_LST) || self.commentLst.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCommentLst())
    }

    // Java: void deleteCommentLst()
    pub fn deleteCommentLst(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(SLIDE_COMMENT_LST);
      // Java: if (this.commentLst != null)
      if self.commentLst.is_some() {
         // Java: this.commentLst = null;
         self.commentLst = None;
      }
    }

    // Java: void writeCommentLst(CommentList value)
    pub fn writeCommentLst(&mut self, value: Option<&CommentList>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = val.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(7, bin);
            self.map.put(SLIDE_COMMENT_LST, bin);
            // Java: if (this.commentLst != null)
            if self.commentLst.is_some() {
               // Java: this.commentLst = null;
               self.commentLst = None;
            }
         }
      }
    }

    // Java: void forceWriteCommentLst(CommentList value)
    pub fn forceWriteCommentLst(&mut self, value: Option<&CommentList>) {
      // Java: if (value != null)
      if let Some(val) = value {
         // Java: this.map.put(7, value.lastBin(this.zone));
         let bin = val.lastBin(self.zone);
         self.map.put(SLIDE_COMMENT_LST, bin);
         // Java: if (this.commentLst != null)
         if self.commentLst.is_some() {
            // Java: this.commentLst = null;
            self.commentLst = None;
         }
      }
    }

}

// Java: public class CommentList
impl CommentList {
    fn _default(zone: &'static KoZone) -> Self {
        CommentList { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: CommentList()
    pub fn new(zone: &'static KoZone) -> Self {
         let mut inst = Self::_default(zone);
         // Java: this.box = new KpBinBox();
         // LLM Rust Renamed: box is reserved keyword, using self.bin_box
         inst.bin_box = KpBinBox::new();
         // Java: this.list = new KpList(this.zone);
         // LLM Rust Pattern: factory constructor uses inst instead of self
         inst.list = KpList::new(zone);
         inst
    }

    // Java: CommentList(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         let mut inst = Self::_default(zone);
         // Java: this.box = new KpBinBox(bin);
         // LLM Rust Renamed: box is reserved keyword, using self.bin_box
         inst.bin_box = KpBinBox::KpBinBox_from(Some(bin));
         // Java: this.list = new KpList(bin, this.zone);
         // LLM Rust Pattern: factory constructor uses inst instead of self
         inst.list = KpList::KpList_from(bin, zone);
         inst
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let b = self.bin_box.get();
         self.list.load(b.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         self.loadBin(bin, self.zone);
    }

    // Java: void load(CommentList value)
    pub fn load_from(&mut self, value: Option<&CommentList>) {
         // Java: if (value != null)
         if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() {
             let saved = self.list.save(zone);
             self.bin_box.set(Some(saved.as_ref()))
         } else {
             self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(CommentList o)
    pub fn equalsLax(&mut self, o: &mut CommentList) -> bool {
         // Java: return this.equals(o, false);
         self.equals(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut CommentList) -> bool {
         // Java: return this.equals(o, true);
         self.equals(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    pub fn equals(&mut self, o: &mut CommentList, isStrict: bool) -> bool {
         // Java: if (null == o)
         if false {
            // Java: return false;
            return false;
         // Java: else
         } else {
            // Java: CommentList dst = (CommentList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self as *const _, dst as *const _) {
               // Java: return true;
               return true;
            // Java: else
            } else {
               // Java: ArrayList<Comment> dstList = new ArrayList();
               let mut dstList: Vec<Comment> = Vec::new();
               // Java: dst.read(dstList);
               dst.read(&mut dstList);
               // Java: ArrayList<Comment> srcList = new ArrayList();
               let mut srcList: Vec<Comment> = Vec::new();
               // Java: this.read(srcList);
               self.read(&mut srcList);
               // Java: if (dstList.size() != srcList.size())
               if dstList.len() != srcList.len() {
                  // Java: return false;
                  return false;
               // Java: else
               } else {
                  // Java: int size = dstList.size();
                  let size = dstList.len();
                  // Java: for(int i = 0; i < size; ++i)
                  for i in 0..size {
                     // Java: Comment dstClrTrans = (Comment)dstList.get(i);
                     // LLM JavaLine Aggregation: merged into the next Rust statement
                     // Java: Comment srcClrTrans = (Comment)srcList.get(i);
                     // LLM JavaLine Aggregation: merged into the next Rust statement
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict && !dstList[i].equals(&mut srcList[i]) {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict && !dstList[i].equalsLax(&mut srcList[i]) {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
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
         self.saveBin(self.zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let b = self.bin_box.get();
         self.loadBin(b.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let v = Null::value1();
         self.list.load(v.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends CommentList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = CommentList::new(zone);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load(bin);
         // Java: return inst;
         inst
    }

    // Java: static CommentList create()
    pub fn create() -> Self {
         // Java: return initialize(CommentList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static CommentList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(CommentList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Comment> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Comment>) {
         // Java: valueLst.clear();
         valueLst.clear();
         // Java: int size = this.list.size();
         let size = self.list.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size {
            // Java: valueLst.add((Comment)KoBin.Decode.toAny(Comment.class, this.list.get(i), this.zone));
            let bin = self.list.get(i);
            let mut inst = Comment::new();
            // LLM Rust Fix: `KpList.get()` 返回 `Box<dyn KoBin>`，端口方法按 Java 取 `KoBin` 引用，
            // 与同文件其它调用点一致补 `.as_ref()`。
            inst.loadBin(bin.as_ref(), self.zone);
            valueLst.push(inst);
         }
    }

    // Java: void write(Iterable<Comment> valueLst)
    pub fn write(&mut self, valueLst: &[Comment]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Comment i : valueLst)
         for item in valueLst {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = item.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
               // Java: this.list.add(bin);
               self.list.add(bin);
            }
         }
    }

    // Java: void forceWrite(Iterable<Comment> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Comment]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Comment i : valueLst)
         for item in valueLst {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = item.lastBin(self.zone);
            self.list.add(bin);
         }
    }

}

impl KoAr for Slide {
    fn saveBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) { self.loadBin(bin, zone); }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, zone: &KoZone) { self.discard_with_zone(zone); }
    fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(zone) }
}

impl KoClosable for Slide {
    fn close(&mut self) { self.close(); }
}
