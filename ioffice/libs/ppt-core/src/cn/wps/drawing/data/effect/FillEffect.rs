#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.fill.Fill
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

// Java: public class FillEffect implements KoAr, KoClosable
pub struct FillEffect {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<dyn KoBinTrait>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: FillEffect parent
    pub(crate) parent: Option<Box<FillEffect>>,
}

// Java: public class FillEffect implements KoAr, KoClosable
impl FillEffect {
    // Java: static protected static <Value1 extends FillEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = FillEffect::FillEffect();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static FillEffect create()
    pub fn create() -> Self {
        // Java: return initialize(FillEffect.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static FillEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(FillEffect.class, bin);
        Self::initialize(bin)
    }

    // Java: FillEffect()
    pub fn FillEffect() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        FillEffect { zone: KoZone, fill: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: FillEffect(KoBin bin)
    pub fn FillEffect_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        FillEffect { zone: KoZone, fill: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(FillEffect value)
    pub fn load_from(&mut self, value: Option<&FillEffect>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.fill != null && this.fill.dirty())
        // Java: this.map.put(1, this.fill.saveBin(zone));
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(FillEffect o)
    pub fn equalsLax(&self, o: &FillEffect) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &FillEffect) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &FillEffect, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof FillEffect)
        if true {
        // Java: FillEffect dst = (FillEffect)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
        else if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill()) {
            // Java: if (this.hasFill() && dst.hasFill())
            if self.hasFill() && dst.hasFill() {
                // Java: if (isStrict && !this.fill().equals(dst.fill()))
                if isStrict {
                    // Java: return false;
                }
                // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
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
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasFill())
        if self.hasFill() {
            // Java: ret += this.fill().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.fill != null && this.fill.dirty() ? true : this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
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
        // Java: this.fill = null;
        self.fill = None;
    }

    // Java: void follow(FillEffect parent)
    pub fn follow(&mut self, parent: Option<Box<FillEffect>>) {
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

    // Java: FillEffect parent()
    pub fn parent_ref(&self) -> Option<&FillEffect> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: Fill fill()
    pub fn fill(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.fill == null)
        if self.fill.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(1) {
                // Java: KoBin bin = this.map.byKey(1);
                let bin = self.map.byKey(1);
                // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
                self.fill = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasFill())
            else if self.parent.as_ref().map_or(false, |p| p.hasFill()) {
                // Java: this.fill = this.parent.fill();
                let bin = Null::value1();
                self.fill = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
                self.fill = Some(bin);
            }
        }
        // Java: return this.fill;
        self.fill.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
        // Java: return this.map.has(1) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
        self.map.has(1) || self.fill.is_some() || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
        // Java: if (this.fill != null)
        if self.fill.is_some() {
            // Java: this.fill = null;
            self.fill = None;
        }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(1, bin);
                // Java: if (this.fill != null)
                if self.fill.is_some() {
                    // Java: this.fill = null;
                    self.fill = None;
                }
            }
        }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(1, bin);
            // Java: if (this.fill != null)
            if self.fill.is_some() {
                // Java: this.fill = null;
                self.fill = None;
            }
        }
    }
}

impl KoAr for FillEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { FillEffect::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { FillEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { FillEffect::lastBin(self, var1) }
}
impl KoClosable for FillEffect { fn close(&mut self) { FillEffect::close(self); } }
