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

use super::Color::Color;

// Java: public class CustomColor implements KoAr, KoClosable
pub struct CustomColor {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color color
    pub(crate) color: Option<Box<Color>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: CustomColor parent
    pub(crate) parent: Option<Box<CustomColor>>,
}

// Java: public class CustomColor implements KoAr, KoClosable
impl CustomColor {
    // Java: static protected static <Value1 extends CustomColor> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = CustomColor::CustomColor();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static CustomColor create()
    pub fn create() -> Self {
        // Java: return initialize(CustomColor.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static CustomColor open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(CustomColor.class, bin);
        Self::initialize(bin)
    }

    // Java: CustomColor()
    pub fn CustomColor() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        CustomColor { zone: KoZone::defaultZone(), color: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: CustomColor(KoBin bin)
    pub fn CustomColor_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        CustomColor { zone: KoZone::defaultZone(), color: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(CustomColor value)
    pub fn load_from(&mut self, value: Option<&CustomColor>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.color != null && this.color.dirty())
        if self.color.as_ref().map_or(false, |c| c.dirty()) {
            // Java: this.map.put(1, this.color.saveBin(zone));
            let bin = self.color.as_mut().unwrap().saveBin(zone);
            self.map.put(1, bin);
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

    // Java: boolean equalsLax(CustomColor o)
    pub fn equalsLax(&self, o: &CustomColor) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CustomColor) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &CustomColor, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof CustomColor)
        if true {
        // Java: CustomColor dst = (CustomColor)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasColor() || !dst.hasColor()) && (!this.hasColor() || dst.hasColor()))
        else if (self.hasColor() || !dst.hasColor()) && (!self.hasColor() || dst.hasColor()) {
            // Java: if (this.hasColor() && dst.hasColor())
            if self.hasColor() && dst.hasColor() {
                // Java: if (isStrict && !this.color().equals(dst.color()))
                // Java: return false;
                // Java: if (!isStrict && !this.color().equalsLax(dst.color()))
                // Java: return false;
            }
            // Java: if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
                // Java: return !this.hasName() || !dst.hasName() || this.name().equals(dst.name());
                return !self.hasName() || !dst.hasName() || self.name() == dst.name();
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
        let ret: i32 = 0;
        // Java: if (this.hasColor())
        if self.hasColor() {
            // Java: ret += this.color().hashCode();
        }
        // Java: if (this.hasName())
        if self.hasName() {
            // Java: ret += String.valueOf(this.name()).hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.color != null && this.color.dirty() ? true : this.map.dirty();
        if self.color.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.color = null;
        self.color = None;
    }

    // Java: void follow(CustomColor parent)
    pub fn follow(&mut self, parent: Option<Box<CustomColor>>) {
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

    // Java: CustomColor parent()
    pub fn parent_ref(&self) -> Option<&CustomColor> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: Color color()
    pub fn color(&mut self) -> &Color {
        // Java: if (this.color == null)
        if self.color.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(1) {
                // Java: KoBin bin = this.map.byKey(1);
                let bin = self.map.byKey(1);
                // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.color = Some(Box::new(Color::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasColor()) {
                // Java: this.color = this.parent.color();
                self.color = Some(Box::new(Color::create()));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.color = Some(Box::new(Color::open(bin.as_ref())));
            }
        }
        // Java: return this.color;
        self.color.as_deref().unwrap()
    }

    // Java: boolean hasColor()
    pub fn hasColor(&self) -> bool {
        // Java: return this.map.has(1) || this.color != null && this.color.dirty() || this.parent != null && this.parent.hasColor();
        self.map.has(1) || self.color.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColor())
    }

    // Java: void deleteColor()
    pub fn deleteColor(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
        // Java: if (this.color != null)
        if self.color.is_some() {
            // Java: this.color = null;
            self.color = None;
        }
    }

    // Java: void writeColor(Color value)
    pub fn writeColor(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(1, bin);
                // Java: if (this.color != null)
                if self.color.is_some() {
                    // Java: this.color = null;
                    self.color = None;
                }
            }
        }
    }

    // Java: void forceWriteColor(Color value)
    pub fn forceWriteColor(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = val.lastBin(self.zone);
            self.map.put(1, bin);
            // Java: if (this.color != null)
            if self.color.is_some() {
                // Java: this.color = null;
                self.color = None;
            }
        }
    }

    // Java: String name()
    pub fn name(&self) -> String {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? "" : this.parent.name();
            match &self.parent { None => String::new(), Some(p) => p.name() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasName();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(2, bin);
    }
}

impl KoAr for CustomColor {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { CustomColor::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { CustomColor::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CustomColor::lastBin(self, var1) }
}

impl KoClosable for CustomColor {
    fn close(&mut self) { CustomColor::close(self); }
}
