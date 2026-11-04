#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.effect.Effect
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

// Java: public class Blip implements KoAr, KoClosable
pub struct Blip {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: Effects effects
    pub(crate) effects: Option<Box<Effects>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Blip parent
    pub(crate) parent: Option<Box<Blip>>,
}

// Java: public class Effects
pub struct Effects {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
}

// Java: public class Blip implements KoAr, KoClosable
impl Blip {
    // Java: static protected static <Value1 extends Blip> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Blip::Blip();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Blip create()
    pub fn create() -> Self {
        // Java: return initialize(Blip.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Blip open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Blip.class, bin);
        Self::initialize(bin)
    }

    // Java: Blip()
    pub fn Blip() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Blip { zone: KoZone, effects: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Blip(KoBin bin)
    pub fn Blip_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Blip { zone: KoZone, effects: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(Blip value)
    pub fn load_from(&mut self, value: Option<&Blip>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.effects != null && this.effects.dirty())
        if let Some(ref mut eff) = self.effects {
            if eff.dirty() {
                // Java: this.map.put(4, this.effects.saveBin(zone));
                let bin = eff.saveBin(zone);
                self.map.put(4, bin);
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

    // Java: boolean equalsLax(Blip o)
    pub fn equalsLax(&self, o: &Blip) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Blip) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Blip, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Blip)
        if true {
        // Java: Blip dst = (Blip)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasCstate() || !dst.hasCstate()) && (!this.hasCstate() || dst.hasCstate()))
        else if (self.hasCstate() || !dst.hasCstate()) && (!self.hasCstate() || dst.hasCstate()) {
            // Java: if (this.hasCstate() && dst.hasCstate() && this.cstate() != dst.cstate())
            if self.hasCstate() && dst.hasCstate() && self.cstate() != dst.cstate() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasEmbedMediaID() || !dst.hasEmbedMediaID()) && (!this.hasEmbedMediaID() || dst.hasEmbedMediaID()))
            else if (self.hasEmbedMediaID() || !dst.hasEmbedMediaID()) && (!self.hasEmbedMediaID() || dst.hasEmbedMediaID()) {
                // Java: if (isStrict && this.hasEmbedMediaID() && dst.hasEmbedMediaID() && this.embedMediaID() != dst.embedMediaID())
                if isStrict && self.hasEmbedMediaID() && dst.hasEmbedMediaID() && self.embedMediaID() != dst.embedMediaID() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasLinkMediaID() || !dst.hasLinkMediaID()) && (!this.hasLinkMediaID() || dst.hasLinkMediaID()))
                else if (self.hasLinkMediaID() || !dst.hasLinkMediaID()) && (!self.hasLinkMediaID() || dst.hasLinkMediaID()) {
                    // Java: if (isStrict && this.hasLinkMediaID() && dst.hasLinkMediaID() && this.linkMediaID() != dst.linkMediaID())
                    if isStrict && self.hasLinkMediaID() && dst.hasLinkMediaID() && self.linkMediaID() != dst.linkMediaID() {
                        // Java: return false;
                        return false;
                    }
                    // Java: else if ((this.hasEffects() || !dst.hasEffects()) && (!this.hasEffects() || dst.hasEffects()))
                    else if (self.hasEffects() || !dst.hasEffects()) && (!self.hasEffects() || dst.hasEffects()) {
                        // Java: if (this.hasEffects() && dst.hasEffects())
                        if self.hasEffects() && dst.hasEffects() {
                            // Java: if (isStrict && !this.effects().equals(dst.effects()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.effects().equalsLax(dst.effects()))
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
        // Java: if (this.hasCstate())
        if self.hasCstate() {
            // Java: ret += Integer.valueOf(this.cstate()).hashCode();
            ret = ret.wrapping_add(self.cstate());
        }
        // Java: if (this.hasEmbedMediaID())
        if self.hasEmbedMediaID() {
            // Java: ret += Integer.valueOf(this.embedMediaID()).hashCode();
            ret = ret.wrapping_add(self.embedMediaID());
        }
        // Java: if (this.hasLinkMediaID())
        if self.hasLinkMediaID() {
            // Java: ret += Integer.valueOf(this.linkMediaID()).hashCode();
            ret = ret.wrapping_add(self.linkMediaID());
        }
        // Java: if (this.hasEffects())
        if self.hasEffects() {
            // Java: ret += this.effects().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.effects != null && this.effects.dirty() ? true : this.map.dirty();
        if let Some(ref eff) = self.effects {
            if eff.dirty() { return true; }
        }
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
        // Java: this.effects = null;
        self.effects = None;
    }

    // Java: void follow(Blip parent)
    pub fn follow(&mut self, parent: Option<Box<Blip>>) {
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

    // Java: Blip parent()
    pub fn parent_ref(&self) -> Option<&Blip> {
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

    // Java: int cstate()
    pub fn cstate(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.cstate();
            match &self.parent { None => 0, Some(p) => p.cstate() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasCstate()
    pub fn hasCstate(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasCstate();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasCstate())
    }

    // Java: void deleteCstate()
    pub fn deleteCstate(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putCstate(int value)
    pub fn putCstate(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &KoZone);
        self.map.put(1, bin);
    }

    // Java: int embedMediaID()
    pub fn embedMediaID(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? 0 : this.parent.embedMediaID();
            match &self.parent { None => 0, Some(p) => p.embedMediaID() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasEmbedMediaID()
    pub fn hasEmbedMediaID(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasEmbedMediaID();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasEmbedMediaID())
    }

    // Java: void deleteEmbedMediaID()
    pub fn deleteEmbedMediaID(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putEmbedMediaID(int value)
    pub fn putEmbedMediaID(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &KoZone);
        self.map.put(2, bin);
    }

    // Java: int linkMediaID()
    pub fn linkMediaID(&self) -> i32 {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? 0 : this.parent.linkMediaID();
            match &self.parent { None => 0, Some(p) => p.linkMediaID() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasLinkMediaID()
    pub fn hasLinkMediaID(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasLinkMediaID();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasLinkMediaID())
    }

    // Java: void deleteLinkMediaID()
    pub fn deleteLinkMediaID(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
    }

    // Java: void putLinkMediaID(int value)
    pub fn putLinkMediaID(&mut self, value: i32) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &KoZone);
        self.map.put(3, bin);
    }

    // Java: Effects effects()
    pub fn effects(&mut self) -> &mut Effects {
        // Java: if (this.effects == null)
        if self.effects.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(4) {
                // Java: KoBin bin = this.map.byKey(4);
                let bin = self.map.byKey(4);
                // Java: this.effects = (Effects)KoBin.Decode.toAny(Effects.class, bin, this.zone);
                self.effects = Some(Box::new(Effects::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasEffects())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffects()) {
                // Java: this.effects = this.parent.effects();
                let bin = Null::value1();
                self.effects = Some(Box::new(Effects::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.effects = (Effects)KoBin.Decode.toAny(Effects.class, bin, this.zone);
                self.effects = Some(Box::new(Effects::open(bin.as_ref())));
            }
        }
        // Java: return this.effects;
        self.effects.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasEffects()
    pub fn hasEffects(&self) -> bool {
        // Java: return this.map.has(4) || this.effects != null && this.effects.dirty() || this.parent != null && this.parent.hasEffects();
        self.map.has(4) || self.effects.as_ref().map_or(false, |e| e.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffects())
    }

    // Java: void deleteEffects()
    pub fn deleteEffects(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.effects != null)
        if self.effects.is_some() {
            // Java: this.effects = null;
            self.effects = None;
        }
    }

    // Java: void writeEffects(Effects value)
    pub fn writeEffects(&mut self, value: Option<&Effects>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(&KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.effects != null)
                if self.effects.is_some() {
                    // Java: this.effects = null;
                    self.effects = None;
                }
            }
        }
    }

    // Java: void forceWriteEffects(Effects value)
    pub fn forceWriteEffects(&mut self, value: Option<&Effects>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = val.lastBin(&KoZone);
            self.map.put(4, bin);
            // Java: if (this.effects != null)
            if self.effects.is_some() {
                // Java: this.effects = null;
                self.effects = None;
            }
        }
    }
}

// Java: public class Effects
impl Effects {
    // Java: Effects()
    pub fn Effects() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        Effects { bin_box: KpBinBox::new(), list: KpList::new(&KoZone), zone: KoZone::defaultZone() }
    }

    // Java: Effects(KoBin bin)
    pub fn Effects_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        Effects { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, &KoZone), zone: KoZone::defaultZone() }
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
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(Effects value)
    pub fn load_from(&mut self, value: Option<&Effects>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
        if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(Effects o)
    pub fn equalsLax(&self, o: &Effects) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Effects) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Effects, isStrict: bool) -> bool {
        // Java: if (null == o)
        if false {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: Effects dst = (Effects)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: ArrayList<Effect> dstList = new ArrayList();
                let mut dstList: Vec<Box<dyn KoBinTrait>> = Vec::new();
                // Java: dst.read(dstList);
                dst.read(&mut dstList);
                // Java: ArrayList<Effect> srcList = new ArrayList();
                let mut srcList: Vec<Box<dyn KoBinTrait>> = Vec::new();
                // Java: this.read(srcList);
                self.read(&mut srcList);
                // Java: if (dstList.size() != srcList.size())
                if dstList.len() != srcList.len() {
                    // Java: return false;
                    return false;
                }
                // Java: else
                else {
                    // Java: int size = dstList.size();
                    let size = dstList.len();
                    // Java: for(int i = 0; i < size; ++i)
                    for i in 0..size {
                        // Java: Effect dstClrTrans = (Effect)dstList.get(i);
                        let _dstClrTrans = &dstList[i];
                        // Java: Effect srcClrTrans = (Effect)srcList.get(i);
                        let _srcClrTrans = &srcList[i];
                        // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict {
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
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.list.load(KoBin.Null.value1(), this.zone);
        let n = Null::value1();
        self.list.load(n.as_ref(), &KoZone);
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends Effects> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Effects::Effects();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
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
        for i in 0..size {
            // Java: valueLst.add((Effect)KoBin.Decode.toAny(Effect.class, this.list.get(i), this.zone));
            let item = self.list.get(i);
            valueLst.push(Factory::dup(&*item, &KoZone));
        }
    }

    // Java: void write(Iterable<Effect> valueLst)
    pub fn write(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(Effect i : valueLst)
        for i in valueLst {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = Factory::dup(i.as_ref(), &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.list.add(bin);
                self.list.add(bin);
            }
        }
    }

    // Java: void forceWrite(Iterable<Effect> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(Effect i : valueLst)
        for i in valueLst {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = Factory::dup(i.as_ref(), &KoZone);
            self.list.add(bin);
        }
    }
}

impl KoAr for Blip {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Blip::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Blip::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Blip::lastBin(self, var1) }
}
impl KoClosable for Blip { fn close(&mut self) { Blip::close(self); } }
