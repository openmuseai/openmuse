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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

const FIELD_TYPE: i32 = 1;
const FIELD_NAME: i32 = 2;
const FIELD_EFFECTS: i32 = 3;

// Java: public class EffectContainer implements KoAr, KoClosable
pub struct EffectContainer {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Effects effects
    pub(crate) effects: Option<Effects>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: EffectContainer parent
    pub(crate) parent: Option<Box<EffectContainer>>,
}

// Java: public class Effects
pub struct Effects {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class EffectContainer implements KoAr, KoClosable
impl EffectContainer {
    // Java: static protected static <Value1 extends EffectContainer> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = EffectContainer::EffectContainer_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static EffectContainer create()
    pub fn create() -> Self {
      // Java: return initialize(EffectContainer.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static EffectContainer open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EffectContainer.class, bin);
      Self::initialize(bin)
    }

    // Java: EffectContainer()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      EffectContainer { zone: KoZone::defaultZone(), effects: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: EffectContainer(KoBin bin)
    pub fn EffectContainer_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      EffectContainer { zone: KoZone::defaultZone(), effects: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(EffectContainer value)
    pub fn load_value(&mut self, value: Option<&EffectContainer>) {
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
      // Java: if (this.effects != null && this.effects.dirty())
      if self.effects.as_ref().map_or(false, |e| e.dirty())
      {
         // Java: this.map.put(3, this.effects.saveBin(zone));
         let bin = self.effects.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_EFFECTS, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(EffectContainer o)
    pub fn equalsLax(&self, o: &EffectContainer) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EffectContainer) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &EffectContainer, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof EffectContainer)
      if true
      {
         // Java: EffectContainer dst = (EffectContainer)o;
         let dst = o;
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
            // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName())
            {
               // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
               if self.hasName() && dst.hasName() && self.name() != dst.name()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasEffects() || !dst.hasEffects()) && (!this.hasEffects() || dst.hasEffects()))
               else if (self.hasEffects() || !dst.hasEffects()) && (!self.hasEffects() || dst.hasEffects())
               {
                  // Java: if (this.hasEffects() && dst.hasEffects())
                  if self.hasEffects() && dst.hasEffects()
                  {
                     // Java: if (isStrict && !this.effects().equals(dst.effects()))
                     if isStrict
                     {
                        // Java: return false;
                        // TODO: effects comparison requires lazy-init
                     }
                     // Java: if (!isStrict && !this.effects().equalsLax(dst.effects()))
                     if !isStrict
                     {
                        // Java: return false;
                        // TODO: effects comparison requires lazy-init
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
      // Java: if (this.hasName())
      if self.hasName()
      {
         // Java: ret += String.valueOf(this.name()).hashCode();
         let name = self.name();
         let mut h: i32 = 0;
         for c in name.chars() { h = h.wrapping_mul(31).wrapping_add(c as i32); }
         ret = ret.wrapping_add(h);
      }
      // Java: if (this.hasEffects())
      if self.hasEffects()
      {
         // Java: ret += this.effects().hashCode();
         // TODO: effects lazy-init
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.effects != null && this.effects.dirty() ? true : this.map.dirty();
      if self.effects.as_ref().map_or(false, |e| e.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.effects = null;
      self.effects = None;
    }

    // Java: void follow(EffectContainer parent)
    pub fn follow(&mut self, mut parent: Box<EffectContainer>) {
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

    // Java: EffectContainer parent()
    pub fn get_parent(&self) -> Option<&EffectContainer> {
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

    // Java: int type()
    pub fn get_type(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_TYPE)
      {
         // Java: return this.parent == null ? 0 : this.parent.type();
         return self.parent.as_ref().map_or(0, |p| p.get_type());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_TYPE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
      self.map.has(FIELD_TYPE) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_TYPE);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_TYPE, Encode::from_int(value, self.zone));
    }

    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_NAME)
      {
         // Java: return this.parent == null ? "" : this.parent.name();
         return self.parent.as_ref().map_or(String::new(), |p| p.name());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_NAME);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasName();
      self.map.has(FIELD_NAME) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_NAME);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_NAME, Encode::from_string(value, self.zone));
    }

    // Java: Effects effects()
    pub fn effects(&self) -> &Effects {
      // Java: if (this.effects == null)
      // Java: if (this.map.has(3))
      // Java: KoBin bin = this.map.byKey(3);
      // Java: this.effects = (Effects)KoBin.Decode.toAny(Effects.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasEffects())
      // Java: this.effects = this.parent.effects();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.effects = (Effects)KoBin.Decode.toAny(Effects.class, bin, this.zone);
      // Java: return this.effects;
      todo!("effects lazy-init requires &mut self")
    }

    // Java: boolean hasEffects()
    pub fn hasEffects(&self) -> bool {
      // Java: return this.map.has(3) || this.effects != null && this.effects.dirty() || this.parent != null && this.parent.hasEffects();
      self.map.has(FIELD_EFFECTS) || self.effects.as_ref().map_or(false, |e| e.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffects())
    }

    // Java: void deleteEffects()
    pub fn deleteEffects(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_EFFECTS);
      // Java: if (this.effects != null)
      if self.effects.is_some()
      {
         // Java: this.effects = null;
         self.effects = None;
      }
    }

    // Java: void writeEffects(Effects value)
    pub fn writeEffects(&mut self, value: &Effects) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(FIELD_EFFECTS, bin);
            // Java: if (this.effects != null)
            if self.effects.is_some()
            {
               // Java: this.effects = null;
               self.effects = None;
            }
         }
      }
    }

    // Java: void forceWriteEffects(Effects value)
    pub fn forceWriteEffects(&mut self, value: &Effects) {
      // Java: if (value != null)
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(FIELD_EFFECTS, value.lastBin(self.zone));
         // Java: if (this.effects != null)
         if self.effects.is_some()
         {
            // Java: this.effects = null;
            self.effects = None;
         }
      }
    }

}

// Java: public class Effects
impl Effects {
    // Java: Effects()
    pub fn new() -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         let zone = KoZone::defaultZone();
         Effects { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: Effects(KoBin bin)
    pub fn Effects_from(bin: &dyn KoBinTrait) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         let zone = KoZone::defaultZone();
         Effects { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone), zone }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let boxed = self.bin_box.get();
         self.list.load(boxed.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         self.loadBin_with_zone(bin, self.zone);
    }

    // Java: void load(Effects value)
    pub fn load_value(&mut self, value: Option<&Effects>) {
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
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let s = self.list.save(zone); self.bin_box.set(Some(s.as_ref())); }
         self.bin_box.get()
    }

    // Java: boolean equalsLax(Effects o)
    pub fn equalsLax(&self, o: &Effects) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Effects) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Effects, _isStrict: bool) -> bool {
         // Java: if (null == o)
         if false
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: Effects dst = (Effects)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<Effect> dstList = new ArrayList();
               let dstList: Vec<()> = Vec::new();
               // Java: dst.read(dstList);
               // TODO: Effect type not yet available
               // Java: ArrayList<Effect> srcList = new ArrayList();
               let srcList: Vec<()> = Vec::new();
               // Java: this.read(srcList);
               // TODO: Effect type not yet available
               // Java: if (dstList.size() != srcList.size())
               if dstList.len() != srcList.len()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
               else
               {
                  // Java: int size = dstList.size();
                  let _size = dstList.len();
                  // Java: for(int i = 0; i < size; ++i)
                  // TODO: Effect type not yet available
                  {
                     // Java: Effect dstClrTrans = (Effect)dstList.get(i);
                     // Java: Effect srcClrTrans = (Effect)srcList.get(i);
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
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends Effects> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Effects::Effects_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static Effects create()
    pub fn create() -> Self {
         // Java: return initialize(Effects.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static Effects open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(Effects.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Effect> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Box<dyn KoBinTrait>>) {
         // Java: valueLst.clear();
         valueLst.clear();
         // Java: int size = this.list.size();
         let size = self.list.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: valueLst.add((Effect)KoBin.Decode.toAny(Effect.class, this.list.get(i), this.zone));
            todo!("Effect type not yet available");
         }
    }

    // Java: void write(Iterable<Effect> valueLst)
    pub fn write(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Effect i : valueLst)
         for _item in valueLst
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            {
               // Java: this.list.add(bin);
               todo!("Effect type not yet available");
            }
         }
    }

    // Java: void forceWrite(Iterable<Effect> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Effect i : valueLst)
         for _item in valueLst
         {
            // Java: this.list.add(i.lastBin(this.zone));
            todo!("Effect type not yet available");
         }
    }

}

impl KoAr for EffectContainer {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { EffectContainer::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { EffectContainer::lastBin(self, var1) }
}

impl KoClosable for EffectContainer {
    fn close(&mut self) { EffectContainer::close(self); }
}
