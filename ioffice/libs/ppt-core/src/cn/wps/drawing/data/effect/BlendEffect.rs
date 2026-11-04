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

use super::EffectContainer::EffectContainer;

// Java: public class BlendEffect implements KoAr, KoClosable
pub struct BlendEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: EffectContainer effectContainer
    pub(crate) effectContainer: Option<Box<EffectContainer>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: BlendEffect parent
    pub(crate) parent: Option<Box<BlendEffect>>,
}

// Java: public class BlendEffect implements KoAr, KoClosable
impl BlendEffect {
    // Java: static protected static <Value1 extends BlendEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = BlendEffect::BlendEffect();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static BlendEffect create()
    pub fn create() -> Self {
        // Java: return initialize(BlendEffect.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static BlendEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(BlendEffect.class, bin);
        Self::initialize(bin)
    }

    // Java: BlendEffect()
    pub fn BlendEffect() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        BlendEffect { zone: KoZone::enter(), effectContainer: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: BlendEffect(KoBin bin)
    pub fn BlendEffect_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        BlendEffect { zone: KoZone::enter(), effectContainer: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
        self.loadBin(bin, KoZone::enter());
    }

    // Java: void load(BlendEffect value)
    pub fn load_from(&mut self, value: Option<&BlendEffect>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(KoZone::enter());
            self.loadBin(last.as_ref(), KoZone::enter());
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.effectContainer != null && this.effectContainer.dirty())
        if let Some(ref mut ec) = self.effectContainer {
            if ec.dirty() {
                // Java: this.map.put(2, this.effectContainer.saveBin(zone));
                let bin = ec.saveBin(zone);
                self.map.put(2, bin);
            }
        }
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(BlendEffect o)
    pub fn equalsLax(&self, o: &BlendEffect) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &BlendEffect) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &BlendEffect, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof BlendEffect)
        if true {
        // Java: BlendEffect dst = (BlendEffect)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasBlendMode() || !dst.hasBlendMode()) && (!this.hasBlendMode() || dst.hasBlendMode()))
        else if (self.hasBlendMode() || !dst.hasBlendMode()) && (!self.hasBlendMode() || dst.hasBlendMode()) {
            // Java: if (this.hasBlendMode() && dst.hasBlendMode() && this.blendMode() != dst.blendMode())
            if self.hasBlendMode() && dst.hasBlendMode() && self.blendMode() != dst.blendMode() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasEffectContainer() || !dst.hasEffectContainer()) && (!this.hasEffectContainer() || dst.hasEffectContainer()))
            else if (self.hasEffectContainer() || !dst.hasEffectContainer()) && (!self.hasEffectContainer() || dst.hasEffectContainer()) {
                // Java: if (this.hasEffectContainer() && dst.hasEffectContainer())
                if self.hasEffectContainer() && dst.hasEffectContainer() {
                    // Java: if (isStrict && !this.effectContainer().equals(dst.effectContainer()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.effectContainer().equalsLax(dst.effectContainer()))
                    if !isStrict {
                        // Java: return false;
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
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasBlendMode())
        if self.hasBlendMode() {
            // Java: ret += Integer.valueOf(this.blendMode()).hashCode();
            ret = ret.wrapping_add(self.blendMode());
        }
        // Java: if (this.hasEffectContainer())
        if self.hasEffectContainer() {
            // Java: ret += this.effectContainer().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.effectContainer != null && this.effectContainer.dirty() ? true : this.map.dirty();
        if self.effectContainer.as_ref().map_or(false, |ec| ec.dirty()) { true } else { self.map.dirty() }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(KoZone::enter());
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
        let n = Null::value1();
        self.map.load(n.as_ref());
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
        // Java: this.effectContainer = null;
        self.effectContainer = None;
    }

    // Java: void follow(BlendEffect parent)
    pub fn follow(&mut self, parent: Option<Box<BlendEffect>>) {
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

    // Java: BlendEffect parent()
    pub fn parent_ref(&self) -> Option<&BlendEffect> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(KoZone::enter());
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int blendMode()
    pub fn blendMode(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 3 : this.parent.blendMode();
            match &self.parent { None => 3, Some(p) => p.blendMode() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasBlendMode()
    pub fn hasBlendMode(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasBlendMode();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBlendMode())
    }

    // Java: void deleteBlendMode()
    pub fn deleteBlendMode(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putBlendMode(int value)
    pub fn putBlendMode(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, KoZone::enter());
        self.map.put(1, bin);
    }

    // Java: EffectContainer effectContainer()
    pub fn effectContainer(&mut self) -> &mut EffectContainer {
        // Java: if (this.effectContainer == null)
        if self.effectContainer.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(2) {
                // Java: KoBin bin = this.map.byKey(2);
                let bin = self.map.byKey(2);
                // Java: this.effectContainer = (EffectContainer)KoBin.Decode.toAny(EffectContainer.class, bin, this.zone);
                self.effectContainer = Some(Box::new(EffectContainer::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasEffectContainer())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectContainer()) {
                // Java: this.effectContainer = this.parent.effectContainer();
                let bin = Null::value1();
                self.effectContainer = Some(Box::new(EffectContainer::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.effectContainer = (EffectContainer)KoBin.Decode.toAny(EffectContainer.class, bin, this.zone);
                self.effectContainer = Some(Box::new(EffectContainer::open(bin.as_ref())));
            }
        }
        // Java: return this.effectContainer;
        self.effectContainer.as_mut().unwrap()
    }

    // Java: boolean hasEffectContainer()
    pub fn hasEffectContainer(&self) -> bool {
        // Java: return this.map.has(2) || this.effectContainer != null && this.effectContainer.dirty() || this.parent != null && this.parent.hasEffectContainer();
        self.map.has(2)
            || self.effectContainer.as_ref().map_or(false, |ec| ec.dirty())
            || self.parent.as_ref().map_or(false, |p| p.hasEffectContainer())
    }

    // Java: void deleteEffectContainer()
    pub fn deleteEffectContainer(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.effectContainer != null)
        if self.effectContainer.is_some() {
            // Java: this.effectContainer = null;
            self.effectContainer = None;
        }
    }

    // Java: void writeEffectContainer(EffectContainer value)
    pub fn writeEffectContainer(&mut self, value: Option<&EffectContainer>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.effectContainer != null)
                if self.effectContainer.is_some() {
                    // Java: this.effectContainer = null;
                    self.effectContainer = None;
                }
            }
        }
    }

    // Java: void forceWriteEffectContainer(EffectContainer value)
    pub fn forceWriteEffectContainer(&mut self, value: Option<&EffectContainer>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = val.lastBin(KoZone::enter());
            self.map.put(2, bin);
            // Java: if (this.effectContainer != null)
            if self.effectContainer.is_some() {
                // Java: this.effectContainer = null;
                self.effectContainer = None;
            }
        }
    }
}

impl KoAr for BlendEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { BlendEffect::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { BlendEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { BlendEffect::lastBin(self, var1) }
}

impl KoClosable for BlendEffect {
    fn close(&mut self) { BlendEffect::close(self); }
}
