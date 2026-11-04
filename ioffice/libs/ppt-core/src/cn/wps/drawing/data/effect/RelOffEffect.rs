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

// Java: public class RelOffEffect implements KoAr, KoClosable
pub struct RelOffEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RelOffEffect parent
    pub(crate) parent: Option<Box<RelOffEffect>>,
}

// Java: public class RelOffEffect implements KoAr, KoClosable
impl RelOffEffect {
    // Java: static protected static <Value1 extends RelOffEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = RelOffEffect::RelOffEffect();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static RelOffEffect create()
    pub fn create() -> Self {
        // Java: return initialize(RelOffEffect.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static RelOffEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(RelOffEffect.class, bin);
        Self::initialize(bin)
    }

    // Java: RelOffEffect()
    pub fn RelOffEffect() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        RelOffEffect { zone: KoZone::enter(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: RelOffEffect(KoBin bin)
    pub fn RelOffEffect_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        RelOffEffect { zone: KoZone::enter(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(RelOffEffect value)
    pub fn load_from(&mut self, value: Option<&RelOffEffect>) {
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

    // Java: boolean equalsLax(RelOffEffect o)
    pub fn equalsLax(&self, o: &RelOffEffect) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RelOffEffect) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &RelOffEffect, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof RelOffEffect)
        if true {
        // Java: RelOffEffect dst = (RelOffEffect)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasTx() || !dst.hasTx()) && (!this.hasTx() || dst.hasTx()))
        else if (self.hasTx() || !dst.hasTx()) && (!self.hasTx() || dst.hasTx()) {
            // Java: if (this.hasTx() && dst.hasTx() && this.tx() != dst.tx())
            if self.hasTx() && dst.hasTx() && self.tx() != dst.tx() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasTy() || !dst.hasTy()) && (!this.hasTy() || dst.hasTy()))
            else if (self.hasTy() || !dst.hasTy()) && (!self.hasTy() || dst.hasTy()) {
                // Java: return !this.hasTy() || !dst.hasTy() || this.ty() == dst.ty();
                return !self.hasTy() || !dst.hasTy() || self.ty() == dst.ty();
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
        // Java: if (this.hasTx())
        if self.hasTx() {
            // Java: ret += Double.valueOf(this.tx()).hashCode();
            ret = ret.wrapping_add((self.tx()).to_bits() as i32);
        }
        // Java: if (this.hasTy())
        if self.hasTy() {
            // Java: ret += Double.valueOf(this.ty()).hashCode();
            ret = ret.wrapping_add((self.ty()).to_bits() as i32);
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

    // Java: void follow(RelOffEffect parent)
    pub fn follow(&mut self, parent: Option<Box<RelOffEffect>>) {
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

    // Java: RelOffEffect parent()
    pub fn parent_ref(&self) -> Option<&RelOffEffect> {
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

    // Java: double tx()
    pub fn tx(&self) -> f64 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.tx();
            match &self.parent { None => 0.0, Some(p) => p.tx() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasTx()
    pub fn hasTx(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasTx();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasTx())
    }

    // Java: void deleteTx()
    pub fn deleteTx(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putTx(double value)
    pub fn putTx(&mut self, value: f64) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, KoZone::enter());
        self.map.put(1, bin);
    }

    // Java: double ty()
    pub fn ty(&self) -> f64 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.ty();
            match &self.parent { None => 0.0, Some(p) => p.ty() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasTy()
    pub fn hasTy(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasTy();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasTy())
    }

    // Java: void deleteTy()
    pub fn deleteTy(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putTy(double value)
    pub fn putTy(&mut self, value: f64) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, KoZone::enter());
        self.map.put(2, bin);
    }
}

impl KoAr for RelOffEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { RelOffEffect::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { RelOffEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RelOffEffect::lastBin(self, var1) }
}

impl KoClosable for RelOffEffect {
    fn close(&mut self) { RelOffEffect::close(self); }
}
