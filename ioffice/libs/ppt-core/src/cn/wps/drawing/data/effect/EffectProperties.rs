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
// rust: ★M-PPT-KMOSLIDE W6c-3 —— Java `effectLst()` / `effectDag()` 是**就地惰性装配**
//   访问器，返回 `EffectList` / `EffectContainer`；本 port 的真 `ShapePropertiesHandler`
//   系 handler（`EffectPropertiesHandler`）需要**可长期持有**的共享句柄，故按
//   `ShapePr::{fill_handle,geom_handle,…}`（§27.4 2B-②）的既有约定补 `Shared<..>` 句柄槽。
//   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-3
use crate::cn::wps::drawing::data::effect::EffectContainer::EffectContainer;
use crate::cn::wps::drawing::data::effect::EffectList::EffectList;
use std::cell::RefCell;
use std::rc::Rc;

// rust: 别名（Java 无）—— `Rc<RefCell<..>>` 承载 Java 的共享可变引用语义（repo 既有约定）。
pub type Shared<T> = Rc<RefCell<T>>;

// Java: public class EffectProperties implements KoAr, KoClosable
pub struct EffectProperties {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: EffectList effectLst
    pub(crate) effectLst: Option<Box<dyn KoAr>>,
    // Java field: EffectContainer effectDag
    pub(crate) effectDag: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: EffectProperties parent
    pub(crate) parent: Option<Box<EffectProperties>>,
    // rust: ★共享句柄槽（无 Java 对应字段）—— `effectLst()` / `effectDag()` 的长期落点。
    pub(crate) effectLst_handle: Option<Shared<EffectList>>,
    pub(crate) effectDag_handle: Option<Shared<EffectContainer>>,
}

// Java: public class EffectProperties implements KoAr, KoClosable
impl EffectProperties {
    // Java: static protected static <Value1 extends EffectProperties> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::EffectProperties_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static EffectProperties create()
    pub fn create() -> Self {
      // Java: return initialize(EffectProperties.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static EffectProperties open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EffectProperties.class, bin);
      Self::initialize(bin)
    }

    // Java: EffectProperties()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), effectLst: None, effectDag: None, parent: None,
      effectLst_handle: None, effectDag_handle: None,
      }
    }

    // Java: EffectProperties(KoBin bin)
    pub fn EffectProperties_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), effectLst: None, effectDag: None, parent: None,
      effectLst_handle: None, effectDag_handle: None,
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

    // Java: void load(EffectProperties value)
    pub fn load_value(&mut self, value: Option<&EffectProperties>) {
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
      // Java: if (this.effectLst != null && this.effectLst.dirty())
        if self.effectLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.effectLst.saveBin(zone));
         let saved = self.effectLst.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // rust: ★共享句柄分支（PLAN §37.2(3) 同款教训）—— `effectLst()` 把对象移入句柄后，
      //   仅看具体槽位会让该子树序列化为空。owner: ppt.drawing.data.effect · wave: M-PPT-KMOSLIDE-W6c-3
      else if self
        .effectLst_handle
        .as_ref()
        .map_or(false, |h| h.borrow().dirty())
      {
        let saved = self.effectLst_handle.as_ref().unwrap().borrow_mut().saveBin(zone);
        self.map.put(2, saved);
      }
      // Java: if (this.effectDag != null && this.effectDag.dirty())
        if self.effectDag.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(3, this.effectDag.saveBin(zone));
         let saved = self.effectDag.as_mut().unwrap().saveBin(zone);
         self.map.put(3, saved);
      }
      // rust: ★共享句柄分支，理由同上。owner: ppt.drawing.data.effect · wave: M-PPT-KMOSLIDE-W6c-3
      else if self
        .effectDag_handle
        .as_ref()
        .map_or(false, |h| h.borrow().dirty())
      {
        let saved = self.effectDag_handle.as_ref().unwrap().borrow_mut().saveBin(zone);
        self.map.put(3, saved);
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

    // Java: boolean equalsLax(EffectProperties o)
    pub fn equalsLax(&self, o: &EffectProperties) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EffectProperties) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &EffectProperties, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof EffectProperties)
        if true
      {
          let dst = o;
         // Java: EffectProperties dst = (EffectProperties)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
            else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
         {
            // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
                if self.hasType() && dst.hasType() && self.get_type() != dst.get_type()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasEffectLst() || !dst.hasEffectLst()) && (!this.hasEffectLst() || dst.hasEffectLst()))
                else if (self.hasEffectLst() || !dst.hasEffectLst()) && (!self.hasEffectLst() || dst.hasEffectLst())
            {
               // Java: if (this.hasEffectLst() && dst.hasEffectLst())
                    if self.hasEffectLst() && dst.hasEffectLst()
               {
                  // Java: if (isStrict && !this.effectLst().equals(dst.effectLst()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.effectLst().equalsLax(dst.effectLst()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasEffectDag() || !dst.hasEffectDag()) && (!this.hasEffectDag() || dst.hasEffectDag()))
                    if (self.hasEffectDag() || !dst.hasEffectDag()) && (!self.hasEffectDag() || dst.hasEffectDag())
               {
                  // Java: if (this.hasEffectDag() && dst.hasEffectDag())
                        if self.hasEffectDag() && dst.hasEffectDag()
                  {
                     // Java: if (isStrict && !this.effectDag().equals(dst.effectDag()))
                            if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.effectDag().equalsLax(dst.effectDag()))
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
      // Java: if (this.hasType())
        if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.get_type());
      }
      // Java: if (this.hasEffectLst())
        if self.hasEffectLst()
      {
         // Java: ret += this.effectLst().hashCode();
         ret = ret.wrapping_add(todo!("effectLst hashCode"));
      }
      // Java: if (this.hasEffectDag())
        if self.hasEffectDag()
      {
         // Java: ret += this.effectDag().hashCode();
         ret = ret.wrapping_add(todo!("effectDag hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.effectLst != null && this.effectLst.dirty())
        if self.effectLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.effectDag != null && this.effectDag.dirty() ? true : this.map.dirty();
         return self.effectDag.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
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
      // Java: this.effectLst = null;
      self.effectLst = None;
      // Java: this.effectDag = null;
      self.effectDag = None;
    }

    // Java: void follow(EffectProperties parent)
    pub fn follow(&mut self, parent: Option<Box<EffectProperties>>) {
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

    // Java: EffectProperties parent()
    pub fn get_parent(&self) -> Option<&EffectProperties> {
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

    // Java: int type()
    pub fn get_type(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 1 : this.parent.type();
         return self.parent.as_ref().map_or(1, |p| p.get_type());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: EffectList effectLst()
    //
    // rust: ★门面（M-PPT-KMOSLIDE W6c-3）—— Java 的 `effectLst()` 在 `&self` 语义下**就地**
    //   惰性装配字段并返回该对象；Rust 的 `&self` 无法写字段，故按本仓既有约定
    //   （`ShapePr::fill_handle` 等，§27.4 2B-②）改由**共享句柄**承载同一对象：
    //   首次访问装配、后续返回**同一 `Rc`**（与 Java「同一个 effectLst 实例」等价）。
    //   三条分支与 Java `EffectProperties.java:241-253` 逐条对应。
    pub fn effectLst(&mut self) -> Shared<EffectList> {
      if self.effectLst_handle.is_none() {
        if self.map.has(2) {
          // Java: KoBin bin = this.map.byKey(2);
          let bin = self.map.byKey(2);
          // Java: this.effectLst = (EffectList)KoBin.Decode.toAny(EffectList.class, bin, this.zone);
          let decoded = EffectList::open(bin.as_ref());
          self.effectLst_handle = Some(Rc::new(RefCell::new(decoded)));
        } else if self.parent.as_ref().map_or(false, |p| p.hasEffectLst()) {
          // Java: this.effectLst = this.parent.effectLst();
          let parent_handle = self
            .parent
            .as_mut()
            .expect("parent != null")
            .effectLst();
          self.effectLst_handle = Some(parent_handle);
        } else {
          // Java: KoBin bin = KoBin.Null.value1();
          let bin = Null::value1();
          // Java: this.effectLst = (EffectList)KoBin.Decode.toAny(EffectList.class, bin, this.zone);
          let decoded = EffectList::open(bin.as_ref());
          self.effectLst_handle = Some(Rc::new(RefCell::new(decoded)));
        }
      }
      // Java: return this.effectLst;
      self.effectLst_handle.as_ref().unwrap().clone()
    }

    // Java: boolean hasEffectLst()
    pub fn hasEffectLst(&self) -> bool {
      // Java: return this.map.has(2) || this.effectLst != null && this.effectLst.dirty() || this.parent != null && this.parent.hasEffectLst();
      //
      // rust: ★共享句柄分支（M-PPT-KMOSLIDE W6c-3）—— 与 `ShapePr::has{Fill,Ln,..}`
      //   （PLAN §37.2(2)）同一教训：`effectLst()` 把对象**移入**句柄槽后，
      //   `this.effectLst != null` 的原判据会恒假，必须把句柄并入谓词。
      //   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-3
      self.map.has(2)
        || self.effectLst.as_ref().map_or(false, |x| x.dirty())
        || self.effectLst_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffectLst())
    }

    // Java: void deleteEffectLst()
    pub fn deleteEffectLst(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.effectLst != null)
        if self.effectLst.is_some()
      {
         // Java: this.effectLst = null;
         self.effectLst = None;
      }
    }

    // Java: void writeEffectLst(EffectList value)
    pub fn writeEffectLst(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.effectLst != null)
                if self.effectLst.is_some()
            {
               // Java: this.effectLst = null;
               self.effectLst = None;
            }
         }
      }
    }

    // Java: void forceWriteEffectLst(EffectList value)
    pub fn forceWriteEffectLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.effectLst != null)
            if self.effectLst.is_some()
         {
            // Java: this.effectLst = null;
            self.effectLst = None;
         }
      }
    }

    // Java: EffectContainer effectDag()
    //
    // rust: ★门面（M-PPT-KMOSLIDE W6c-3）—— 与上方 `effectLst()` 同形：Java 的就地惰性装配
    //   在 Rust 侧以**共享句柄**承载（`&self` 无法写字段，见 §27.4 2B-②）。
    //   三条分支与 Java `EffectProperties.java:292-304` 逐条对应。
    pub fn effectDag(&mut self) -> Shared<EffectContainer> {
      if self.effectDag_handle.is_none() {
        if self.map.has(3) {
          // Java: KoBin bin = this.map.byKey(3);
          let bin = self.map.byKey(3);
          // Java: this.effectDag = (EffectContainer)KoBin.Decode.toAny(EffectContainer.class, bin, this.zone);
          let decoded = EffectContainer::open(bin.as_ref());
          self.effectDag_handle = Some(Rc::new(RefCell::new(decoded)));
        } else if self.parent.as_ref().map_or(false, |p| p.hasEffectDag()) {
          // Java: this.effectDag = this.parent.effectDag();
          let parent_handle = self
            .parent
            .as_mut()
            .expect("parent != null")
            .effectDag();
          self.effectDag_handle = Some(parent_handle);
        } else {
          // Java: KoBin bin = KoBin.Null.value1();
          let bin = Null::value1();
          // Java: this.effectDag = (EffectContainer)KoBin.Decode.toAny(EffectContainer.class, bin, this.zone);
          let decoded = EffectContainer::open(bin.as_ref());
          self.effectDag_handle = Some(Rc::new(RefCell::new(decoded)));
        }
      }
      // Java: return this.effectDag;
      self.effectDag_handle.as_ref().unwrap().clone()
    }

    // Java: boolean hasEffectDag()
    pub fn hasEffectDag(&self) -> bool {
      // Java: return this.map.has(3) || this.effectDag != null && this.effectDag.dirty() || this.parent != null && this.parent.hasEffectDag();
      // rust: ★共享句柄分支，理由同上方 `hasEffectLst`。owner: ppt.drawing.data.effect · wave: M-PPT-KMOSLIDE-W6c-3
      self.map.has(3)
        || self.effectDag.as_ref().map_or(false, |x| x.dirty())
        || self.effectDag_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffectDag())
    }

    // Java: void deleteEffectDag()
    pub fn deleteEffectDag(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.effectDag != null)
        if self.effectDag.is_some()
      {
         // Java: this.effectDag = null;
         self.effectDag = None;
      }
    }

    // Java: void writeEffectDag(EffectContainer value)
    pub fn writeEffectDag(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.effectDag != null)
                if self.effectDag.is_some()
            {
               // Java: this.effectDag = null;
               self.effectDag = None;
            }
         }
      }
    }

    // Java: void forceWriteEffectDag(EffectContainer value)
    pub fn forceWriteEffectDag(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.effectDag != null)
            if self.effectDag.is_some()
         {
            // Java: this.effectDag = null;
            self.effectDag = None;
         }
      }
    }

}

impl KoAr for EffectProperties {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { EffectProperties::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { EffectProperties::lastBin(self, var1) }
}

impl KoClosable for EffectProperties {
    fn close(&mut self) { EffectProperties::close(self); }
}
