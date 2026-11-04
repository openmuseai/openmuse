#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

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

// Java: public class TintEffect implements KoAr, KoClosable
pub struct TintEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TintEffect parent
    pub(crate) parent: Option<Box<TintEffect>>,
}

// Java: public class TintEffect implements KoAr, KoClosable
impl TintEffect {
    // Java: static protected static <Value1 extends TintEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = TintEffect::TintEffect();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TintEffect create()
    pub fn create() -> Self {
        // Java: return initialize(TintEffect.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static TintEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TintEffect.class, bin);
        Self::initialize(bin)
    }

    // Java: TintEffect()
    pub fn TintEffect() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        TintEffect { zone: KoZone::enter(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: TintEffect(KoBin bin)
    pub fn TintEffect_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        TintEffect { zone: KoZone::enter(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(TintEffect value)
    pub fn load_from(&mut self, value: Option<&TintEffect>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(KoZone::enter());
            self.loadBin(last.as_ref(), KoZone::enter());
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(TintEffect o)
    pub fn equalsLax(&self, o: &TintEffect) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TintEffect) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &TintEffect, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof TintEffect)
        if true {
        // Java: TintEffect dst = (TintEffect)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasAmount() || !dst.hasAmount()) && (!this.hasAmount() || dst.hasAmount()))
        else if (self.hasAmount() || !dst.hasAmount()) && (!self.hasAmount() || dst.hasAmount()) {
            // Java: if (this.hasAmount() && dst.hasAmount() && this.amount() != dst.amount())
            if self.hasAmount() && dst.hasAmount() && self.amount() != dst.amount() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasHue() || !dst.hasHue()) && (!this.hasHue() || dst.hasHue()))
            else if (self.hasHue() || !dst.hasHue()) && (!self.hasHue() || dst.hasHue()) {
                // Java: return !this.hasHue() || !dst.hasHue() || this.hue() == dst.hue();
                return !self.hasHue() || !dst.hasHue() || self.hue() == dst.hue();
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
        // Java: if (this.hasAmount())
        if self.hasAmount() {
            // Java: ret += Double.valueOf(this.amount()).hashCode();
            ret = ret.wrapping_add((self.amount()).to_bits() as i32);
        }
        // Java: if (this.hasHue())
        if self.hasHue() {
            // Java: ret += Integer.valueOf(this.hue()).hashCode();
            ret = ret.wrapping_add(self.hue());
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
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
    }

    // Java: void follow(TintEffect parent)
    pub fn follow(&mut self, parent: Option<Box<TintEffect>>) {
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

    // Java: TintEffect parent()
    pub fn parent_ref(&self) -> Option<&TintEffect> {
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

    // Java: double amount()
    pub fn amount(&self) -> f64 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.amount();
            match &self.parent { None => 0.0, Some(p) => p.amount() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasAmount()
    pub fn hasAmount(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasAmount();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasAmount())
    }

    // Java: void deleteAmount()
    pub fn deleteAmount(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putAmount(double value)
    pub fn putAmount(&mut self, value: f64) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, KoZone::enter());
        self.map.put(1, bin);
    }

    // Java: int hue()
    pub fn hue(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? 0 : this.parent.hue();
            match &self.parent { None => 0, Some(p) => p.hue() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasHue()
    pub fn hasHue(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasHue();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasHue())
    }

    // Java: void deleteHue()
    pub fn deleteHue(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putHue(int value)
    pub fn putHue(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, KoZone::enter());
        self.map.put(2, bin);
    }
}

impl KoAr for TintEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { TintEffect::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { TintEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TintEffect::lastBin(self, var1) }
}

impl KoClosable for TintEffect {
    fn close(&mut self) { TintEffect::close(self); }
}
