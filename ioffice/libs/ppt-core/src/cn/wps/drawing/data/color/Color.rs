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
// Java: import java.util.ArrayList
// Java: import java.util.Collection
use super::ColorTransform::ColorTransform;

// Java: public class Color implements KoAr, KoClosable
pub struct Color {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: CRGB scrgb
    pub(crate) scrgb: Option<Box<CRGB>>,
    // Java field: RGB srgb
    pub(crate) srgb: Option<Box<RGB>>,
    // Java field: HSL hsl
    pub(crate) hsl: Option<Box<HSL>>,
    // Java field: RGB sysLastClr
    pub(crate) sysLastClr: Option<Box<RGB>>,
    // Java field: ColorTransformList transformLst
    pub(crate) transformLst: Option<Box<ColorTransformList>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Color parent
    pub(crate) parent: Option<Box<Color>>,
}

// Java: public class RGB
pub struct RGB {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RGB parent
    pub(crate) parent: Option<Box<RGB>>,
}

// Java: public class CRGB
pub struct CRGB {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: CRGB parent
    pub(crate) parent: Option<Box<CRGB>>,
}

// Java: public class HSL
pub struct HSL {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: HSL parent
    pub(crate) parent: Option<Box<HSL>>,
}

// Java: public class ColorTransformList
pub struct ColorTransformList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: (),
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Color implements KoAr, KoClosable
impl Color {
    // Java: static protected static <Value1 extends Color> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Color::Color();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Color create()
    pub fn create() -> Self {
        // Java: return initialize(Color.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Color open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Color.class, bin);
        Self::initialize(bin)
    }

    // Java: Color()
    pub fn Color() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Color {
            zone: KoZone::defaultZone(),
            scrgb: None,
            srgb: None,
            hsl: None,
            sysLastClr: None,
            transformLst: None,
            bin_box: KpBinBox::new(),
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: Color(KoBin bin)
    pub fn Color_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Color {
            zone: KoZone::defaultZone(),
            scrgb: None,
            srgb: None,
            hsl: None,
            sysLastClr: None,
            transformLst: None,
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            map: KpMap::KpMap_from(bin),
            parent: None,
        }
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

    // Java: void load(Color value)
    pub fn load_from(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.scrgb != null && this.scrgb.dirty())
        if self.scrgb.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: this.map.put(2, this.scrgb.saveBin(zone));
            let bin = self.scrgb.as_mut().unwrap().saveBin(zone);
            self.map.put(2, bin);
        }
        // Java: if (this.srgb != null && this.srgb.dirty())
        if self.srgb.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: this.map.put(3, this.srgb.saveBin(zone));
            let bin = self.srgb.as_mut().unwrap().saveBin(zone);
            self.map.put(3, bin);
        }
        // Java: if (this.hsl != null && this.hsl.dirty())
        if self.hsl.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: this.map.put(4, this.hsl.saveBin(zone));
            let bin = self.hsl.as_mut().unwrap().saveBin(zone);
            self.map.put(4, bin);
        }
        // Java: if (this.sysLastClr != null && this.sysLastClr.dirty())
        if self.sysLastClr.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: this.map.put(6, this.sysLastClr.saveBin(zone));
            let bin = self.sysLastClr.as_mut().unwrap().saveBin(zone);
            self.map.put(6, bin);
        }
        // Java: if (this.transformLst != null && this.transformLst.dirty())
        if self.transformLst.as_ref().map_or(false, |t| t.dirty()) {
            // Java: this.map.put(9, this.transformLst.saveBin(zone));
            let bin = self.transformLst.as_mut().unwrap().saveBin(zone);
            self.map.put(9, bin);
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

    // Java: boolean equalsLax(Color o)
    pub fn equalsLax(&self, o: &Color) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Color) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Color, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Color)
        if true {
        // Java: Color dst = (Color)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasMode() || !dst.hasMode()) && (!this.hasMode() || dst.hasMode()))
        else if (self.hasMode() || !dst.hasMode()) && (!self.hasMode() || dst.hasMode()) {
            // Java: if (this.hasMode() && dst.hasMode() && this.mode() != dst.mode())
            if self.hasMode() && dst.hasMode() && self.mode() != dst.mode() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasScrgb() || !dst.hasScrgb()) && (!this.hasScrgb() || dst.hasScrgb()))
            else if (self.hasScrgb() || !dst.hasScrgb()) && (!self.hasScrgb() || dst.hasScrgb()) {
                // Java: if (this.hasScrgb() && dst.hasScrgb())
                if self.hasScrgb() && dst.hasScrgb() {
                    // Java: if (isStrict && !this.scrgb().equals(dst.scrgb()))
                    // Java: return false;
                    // Java: if (!isStrict && !this.scrgb().equalsLax(dst.scrgb()))
                    // Java: return false;
                }
                // Java: if ((this.hasSrgb() || !dst.hasSrgb()) && (!this.hasSrgb() || dst.hasSrgb()))
                if (self.hasSrgb() || !dst.hasSrgb()) && (!self.hasSrgb() || dst.hasSrgb()) {
                    // Java: if (this.hasSrgb() && dst.hasSrgb())
                    if self.hasSrgb() && dst.hasSrgb() {
                        // Java: if (isStrict && !this.srgb().equals(dst.srgb()))
                        // Java: return false;
                        // Java: if (!isStrict && !this.srgb().equalsLax(dst.srgb()))
                        // Java: return false;
                    }
                    // Java: if ((this.hasHsl() || !dst.hasHsl()) && (!this.hasHsl() || dst.hasHsl()))
                    if (self.hasHsl() || !dst.hasHsl()) && (!self.hasHsl() || dst.hasHsl()) {
                        // Java: if (this.hasHsl() && dst.hasHsl())
                        if self.hasHsl() && dst.hasHsl() {
                            // Java: if (isStrict && !this.hsl().equals(dst.hsl()))
                            // Java: return false;
                            // Java: if (!isStrict && !this.hsl().equalsLax(dst.hsl()))
                            // Java: return false;
                        }
                        // Java: if ((this.hasSys() || !dst.hasSys()) && (!this.hasSys() || dst.hasSys()))
                        if (self.hasSys() || !dst.hasSys()) && (!self.hasSys() || dst.hasSys()) {
                            // Java: if (this.hasSys() && dst.hasSys() && this.sys() != dst.sys())
                            if self.hasSys() && dst.hasSys() && self.sys() != dst.sys() {
                                // Java: return false;
                                return false;
                            }
                            // Java: else if ((this.hasSysLastClr() || !dst.hasSysLastClr()) && (!this.hasSysLastClr() || dst.hasSysLastClr()))
                            else if (self.hasSysLastClr() || !dst.hasSysLastClr()) && (!self.hasSysLastClr() || dst.hasSysLastClr()) {
                                // Java: if (this.hasSysLastClr() && dst.hasSysLastClr())
                                if self.hasSysLastClr() && dst.hasSysLastClr() {
                                    // Java: if (isStrict && !this.sysLastClr().equals(dst.sysLastClr()))
                                    // Java: return false;
                                    // Java: if (!isStrict && !this.sysLastClr().equalsLax(dst.sysLastClr()))
                                    // Java: return false;
                                }
                                // Java: if ((this.hasScheme() || !dst.hasScheme()) && (!this.hasScheme() || dst.hasScheme()))
                                if (self.hasScheme() || !dst.hasScheme()) && (!self.hasScheme() || dst.hasScheme()) {
                                    // Java: if (this.hasScheme() && dst.hasScheme() && this.scheme() != dst.scheme())
                                    if self.hasScheme() && dst.hasScheme() && self.scheme() != dst.scheme() {
                                        // Java: return false;
                                        return false;
                                    }
                                    // Java: else if ((this.hasPrst() || !dst.hasPrst()) && (!this.hasPrst() || dst.hasPrst()))
                                    else if (self.hasPrst() || !dst.hasPrst()) && (!self.hasPrst() || dst.hasPrst()) {
                                        // Java: if (this.hasPrst() && dst.hasPrst() && this.prst() != dst.prst())
                                        if self.hasPrst() && dst.hasPrst() && self.prst() != dst.prst() {
                                            // Java: return false;
                                            return false;
                                        }
                                        // Java: else if ((this.hasTransformLst() || !dst.hasTransformLst()) && (!this.hasTransformLst() || dst.hasTransformLst()))
                                        else if (self.hasTransformLst() || !dst.hasTransformLst()) && (!self.hasTransformLst() || dst.hasTransformLst()) {
                                            // Java: if (this.hasTransformLst() && dst.hasTransformLst())
                                            if self.hasTransformLst() && dst.hasTransformLst() {
                                                // Java: if (isStrict && !this.transformLst().equals(dst.transformLst()))
                                                // Java: return false;
                                                // Java: if (!isStrict && !this.transformLst().equalsLax(dst.transformLst()))
                                                // Java: return false;
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
        // Java: if (this.hasMode())
        if self.hasMode() {
            // Java: ret += Integer.valueOf(this.mode()).hashCode();
            ret += self.mode();
        }
        // Java: if (this.hasScrgb())
        if self.hasScrgb() {
            // Java: ret += this.scrgb().hashCode();
        }
        // Java: if (this.hasSrgb())
        if self.hasSrgb() {
            // Java: ret += this.srgb().hashCode();
        }
        // Java: if (this.hasHsl())
        if self.hasHsl() {
            // Java: ret += this.hsl().hashCode();
        }
        // Java: if (this.hasSys())
        if self.hasSys() {
            // Java: ret += Integer.valueOf(this.sys()).hashCode();
            ret += self.sys();
        }
        // Java: if (this.hasSysLastClr())
        if self.hasSysLastClr() {
            // Java: ret += this.sysLastClr().hashCode();
        }
        // Java: if (this.hasScheme())
        if self.hasScheme() {
            // Java: ret += Integer.valueOf(this.scheme()).hashCode();
            ret += self.scheme();
        }
        // Java: if (this.hasPrst())
        if self.hasPrst() {
            // Java: ret += Integer.valueOf(this.prst()).hashCode();
            ret += self.prst();
        }
        // Java: if (this.hasTransformLst())
        if self.hasTransformLst() {
            // Java: ret += this.transformLst().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.scrgb != null && this.scrgb.dirty())
        if self.scrgb.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.srgb != null && this.srgb.dirty())
        else if self.srgb.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.hsl != null && this.hsl.dirty())
        else if self.hsl.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.sysLastClr != null && this.sysLastClr.dirty())
        else if self.sysLastClr.as_ref().map_or(false, |s| s.map.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.transformLst != null && this.transformLst.dirty() ? true : this.map.dirty();
            if self.transformLst.as_ref().map_or(false, |t| t.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.scrgb = null;
        self.scrgb = None;
        // Java: this.srgb = null;
        self.srgb = None;
        // Java: this.hsl = null;
        self.hsl = None;
        // Java: this.sysLastClr = null;
        self.sysLastClr = None;
        // Java: this.transformLst = null;
        self.transformLst = None;
    }

    // Java: void follow(Color parent)
    pub fn follow(&mut self, parent: Option<Box<Color>>) {
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

    // Java: Color parent()
    pub fn parent_ref(&self) -> Option<&Color> {
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

    // Java: int mode()
    pub fn mode(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 1 : this.parent.mode();
            match &self.parent { None => 1, Some(p) => p.mode() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasMode()
    pub fn hasMode(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasMode();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasMode())
    }

    // Java: void deleteMode()
    pub fn deleteMode(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putMode(int value)
    pub fn putMode(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(1, bin);
    }

    // Java: CRGB scrgb()
    pub fn scrgb(&mut self) -> &CRGB {
        // Java: if (this.scrgb == null)
        if self.scrgb.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(2) {
                // Java: KoBin bin = this.map.byKey(2);
                // Java: this.scrgb = (CRGB)KoBin.Decode.toAny(CRGB.class, bin, this.zone);
                let bin = self.map.byKey(2);
                self.scrgb = Some(Box::new(CRGB::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasScrgb())
            else if self.parent.as_ref().map_or(false, |p| p.hasScrgb()) {
                // Java: this.scrgb = this.parent.scrgb();
                self.scrgb = Some(Box::new(CRGB::create()));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.scrgb = (CRGB)KoBin.Decode.toAny(CRGB.class, bin, this.zone);
                self.scrgb = Some(Box::new(CRGB::open(bin.as_ref())));
            }
        }
        // Java: return this.scrgb;
        self.scrgb.as_deref().unwrap()
    }

    // Java: boolean hasScrgb()
    pub fn hasScrgb(&self) -> bool {
        // Java: return this.map.has(2) || this.scrgb != null && this.scrgb.dirty() || this.parent != null && this.parent.hasScrgb();
        self.map.has(2) || self.scrgb.as_ref().map_or(false, |s| s.map.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasScrgb())
    }

    // Java: void deleteScrgb()
    pub fn deleteScrgb(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.scrgb != null)
        if self.scrgb.is_some() {
            // Java: this.scrgb = null;
            self.scrgb = None;
        }
    }

    // Java: void writeScrgb(CRGB value)
    pub fn writeScrgb(&mut self, value: Option<&CRGB>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.scrgb != null)
                if self.scrgb.is_some() {
                    // Java: this.scrgb = null;
                    self.scrgb = None;
                }
            }
        }
    }

    // Java: void forceWriteScrgb(CRGB value)
    pub fn forceWriteScrgb(&mut self, value: Option<&CRGB>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = val.lastBin(self.zone);
            self.map.put(2, bin);
            // Java: if (this.scrgb != null)
            if self.scrgb.is_some() {
                // Java: this.scrgb = null;
                self.scrgb = None;
            }
        }
    }

    // Java: RGB srgb()
    // rust: ★W6c-6 本方法无对应 Java 声明 —— **只读取色**（`&self`）。
    //   Java 侧 `Color.srgb()` 是实例方法、调用处持可变引用；本 port 的部分只读路径
    //   （`Fill::plain_solid_argb`）拿不到 `&mut`，而 `srgb()` 会惰性物化。
    //   此处按「已有 `srgb` 槽位 → 已设 scheme/prst 则无 srgb」直接读，返回 `None`
    //   表示该颜色不是 srgb（交由调用方回退）。语义与 `srgb()` 的物化结果一致。
    //   owner: ppt.drawing.data.color.Color    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn srgb_readonly(&self) -> Option<(i32, i32, i32)> {
        if let Some(rgb) = self.srgb.as_ref() {
            return Some((rgb.r(), rgb.g(), rgb.b()));
        }
        if self.map.has(3) {
            let bin = self.map.byKey(3);
            let rgb = RGB::open(bin.as_ref());
            return Some((rgb.r(), rgb.g(), rgb.b()));
        }
        None
    }

    pub fn srgb(&mut self) -> &mut RGB {
        // Java: if (this.srgb == null)
        if self.srgb.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(3) {
                // Java: KoBin bin = this.map.byKey(3);
                // Java: this.srgb = (RGB)KoBin.Decode.toAny(RGB.class, bin, this.zone);
                let bin = self.map.byKey(3);
                self.srgb = Some(Box::new(RGB::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasSrgb())
            else if self.parent.as_ref().map_or(false, |p| p.hasSrgb()) {
                // Java: this.srgb = this.parent.srgb();
                self.srgb = Some(Box::new(RGB::create()));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.srgb = (RGB)KoBin.Decode.toAny(RGB.class, bin, this.zone);
                self.srgb = Some(Box::new(RGB::open(bin.as_ref())));
            }
        }
        // Java: return this.srgb;
        self.srgb.as_deref_mut().unwrap()
    }

    // Java: boolean hasSrgb()
    pub fn hasSrgb(&self) -> bool {
        // Java: return this.map.has(3) || this.srgb != null && this.srgb.dirty() || this.parent != null && this.parent.hasSrgb();
        self.map.has(3) || self.srgb.as_ref().map_or(false, |s| s.map.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSrgb())
    }

    // Java: void deleteSrgb()
    pub fn deleteSrgb(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.srgb != null)
        if self.srgb.is_some() {
            // Java: this.srgb = null;
            self.srgb = None;
        }
    }

    // Java: void writeSrgb(RGB value)
    pub fn writeSrgb(&mut self, value: Option<&RGB>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.srgb != null)
                if self.srgb.is_some() {
                    // Java: this.srgb = null;
                    self.srgb = None;
                }
            }
        }
    }

    // Java: void forceWriteSrgb(RGB value)
    pub fn forceWriteSrgb(&mut self, value: Option<&RGB>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = val.lastBin(self.zone);
            self.map.put(3, bin);
            // Java: if (this.srgb != null)
            if self.srgb.is_some() {
                // Java: this.srgb = null;
                self.srgb = None;
            }
        }
    }

    // Java: HSL hsl()
    pub fn hsl(&mut self) -> &HSL {
        // Java: if (this.hsl == null)
        if self.hsl.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(4) {
                // Java: KoBin bin = this.map.byKey(4);
                let bin = self.map.byKey(4);
                // Java: this.hsl = (HSL)KoBin.Decode.toAny(HSL.class, bin, this.zone);
                self.hsl = Some(Box::new(HSL::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasHsl())
            else if self.parent.as_ref().map_or(false, |p| p.hasHsl()) {
                // Java: this.hsl = this.parent.hsl();
                self.hsl = Some(Box::new(HSL::create()));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.hsl = (HSL)KoBin.Decode.toAny(HSL.class, bin, this.zone);
                self.hsl = Some(Box::new(HSL::open(bin.as_ref())));
            }
        }
        // Java: return this.hsl;
        self.hsl.as_deref().unwrap()
    }

    // Java: boolean hasHsl()
    pub fn hasHsl(&self) -> bool {
        // Java: return this.map.has(4) || this.hsl != null && this.hsl.dirty() || this.parent != null && this.parent.hasHsl();
        self.map.has(4) || self.hsl.as_ref().map_or(false, |s| s.map.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHsl())
    }

    // Java: void deleteHsl()
    pub fn deleteHsl(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.hsl != null)
        if self.hsl.is_some() {
            // Java: this.hsl = null;
            self.hsl = None;
        }
    }

    // Java: void writeHsl(HSL value)
    pub fn writeHsl(&mut self, value: Option<&HSL>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.hsl != null)
                if self.hsl.is_some() {
                    // Java: this.hsl = null;
                    self.hsl = None;
                }
            }
        }
    }

    // Java: void forceWriteHsl(HSL value)
    pub fn forceWriteHsl(&mut self, value: Option<&HSL>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = val.lastBin(self.zone);
            self.map.put(4, bin);
            // Java: if (this.hsl != null)
            if self.hsl.is_some() {
                // Java: this.hsl = null;
                self.hsl = None;
            }
        }
    }

    // Java: int sys()
    pub fn sys(&self) -> i32 {
        // Java: if (!this.map.has(5))
        if !self.map.has(5) {
            // Java: return this.parent == null ? 0 : this.parent.sys();
            match &self.parent { None => 0, Some(p) => p.sys() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasSys()
    pub fn hasSys(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasSys();
        self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasSys())
    }

    // Java: void deleteSys()
    pub fn deleteSys(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
    }

    // Java: void putSys(int value)
    pub fn putSys(&mut self, value: i32) {
        // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(5, bin);
    }

    // Java: RGB sysLastClr()
    pub fn sysLastClr(&mut self) -> &RGB {
        // Java: if (this.sysLastClr == null)
        if self.sysLastClr.is_none() {
            // Java: if (this.map.has(6))
            if self.map.has(6) {
                // Java: KoBin bin = this.map.byKey(6);
                let bin = self.map.byKey(6);
                // Java: this.sysLastClr = (RGB)KoBin.Decode.toAny(RGB.class, bin, this.zone);
                self.sysLastClr = Some(Box::new(RGB::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasSysLastClr())
            else if self.parent.as_ref().map_or(false, |p| p.hasSysLastClr()) {
                // Java: this.sysLastClr = this.parent.sysLastClr();
                self.sysLastClr = Some(Box::new(RGB::create()));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.sysLastClr = (RGB)KoBin.Decode.toAny(RGB.class, bin, this.zone);
                self.sysLastClr = Some(Box::new(RGB::open(bin.as_ref())));
            }
        }
        // Java: return this.sysLastClr;
        self.sysLastClr.as_deref().unwrap()
    }

    // Java: boolean hasSysLastClr()
    pub fn hasSysLastClr(&self) -> bool {
        // Java: return this.map.has(6) || this.sysLastClr != null && this.sysLastClr.dirty() || this.parent != null && this.parent.hasSysLastClr();
        self.map.has(6) || self.sysLastClr.as_ref().map_or(false, |s| s.map.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSysLastClr())
    }

    // Java: void deleteSysLastClr()
    pub fn deleteSysLastClr(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(6);
        // Java: if (this.sysLastClr != null)
        if self.sysLastClr.is_some() {
            // Java: this.sysLastClr = null;
            self.sysLastClr = None;
        }
    }

    // Java: void writeSysLastClr(RGB value)
    pub fn writeSysLastClr(&mut self, value: Option<&RGB>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(6, bin);
                // Java: if (this.sysLastClr != null)
                if self.sysLastClr.is_some() {
                    // Java: this.sysLastClr = null;
                    self.sysLastClr = None;
                }
            }
        }
    }

    // Java: void forceWriteSysLastClr(RGB value)
    pub fn forceWriteSysLastClr(&mut self, value: Option<&RGB>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(6, value.lastBin(this.zone));
            let bin = val.lastBin(self.zone);
            self.map.put(6, bin);
            // Java: if (this.sysLastClr != null)
            if self.sysLastClr.is_some() {
                // Java: this.sysLastClr = null;
                self.sysLastClr = None;
            }
        }
    }

    // Java: int scheme()
    pub fn scheme(&self) -> i32 {
        // Java: if (!this.map.has(7))
        if !self.map.has(7) {
            // Java: return this.parent == null ? 0 : this.parent.scheme();
            match &self.parent { None => 0, Some(p) => p.scheme() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasScheme()
    pub fn hasScheme(&self) -> bool {
        // Java: return this.map.has(7) || this.parent != null && this.parent.hasScheme();
        self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasScheme())
    }

    // Java: void deleteScheme()
    pub fn deleteScheme(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(7);
    }

    // Java: void putScheme(int value)
    pub fn putScheme(&mut self, value: i32) {
        // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(7, bin);
    }

    // Java: int prst()
    pub fn prst(&self) -> i32 {
        // Java: if (!this.map.has(8))
        if !self.map.has(8) {
            // Java: return this.parent == null ? 0 : this.parent.prst();
            match &self.parent { None => 0, Some(p) => p.prst() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasPrst()
    pub fn hasPrst(&self) -> bool {
        // Java: return this.map.has(8) || this.parent != null && this.parent.hasPrst();
        self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasPrst())
    }

    // Java: void deletePrst()
    pub fn deletePrst(&mut self) {
        // Java: this.map.put(8, (KoBin)null);
        self.map.clear_key(8);
    }

    // Java: void putPrst(int value)
    pub fn putPrst(&mut self, value: i32) {
        // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(8, bin);
    }

    // Java: ColorTransformList transformLst()
    pub fn transformLst(&mut self) -> &mut ColorTransformList {
        // Java: if (this.transformLst == null)
        if self.transformLst.is_none() {
            // Java: if (this.map.has(9))
            if self.map.has(9) {
                // Java: KoBin bin = this.map.byKey(9);
                let bin = self.map.byKey(9);
                // Java: this.transformLst = (ColorTransformList)KoBin.Decode.toAny(ColorTransformList.class, bin, this.zone);
                self.transformLst = Some(Box::new(ColorTransformList::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasTransformLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasTransformLst()) {
                // Java: this.transformLst = this.parent.transformLst();
                self.transformLst = Some(Box::new(ColorTransformList::create()));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.transformLst = (ColorTransformList)KoBin.Decode.toAny(ColorTransformList.class, bin, this.zone);
                self.transformLst = Some(Box::new(ColorTransformList::open(bin.as_ref())));
            }
        }
        // Java: return this.transformLst;
        self.transformLst.as_deref_mut().unwrap()
    }

    // Java: boolean hasTransformLst()
    pub fn hasTransformLst(&self) -> bool {
        // Java: return this.map.has(9) || this.transformLst != null && this.transformLst.dirty() || this.parent != null && this.parent.hasTransformLst();
        self.map.has(9) || self.transformLst.as_ref().map_or(false, |t| t.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTransformLst())
    }

    // Java: void deleteTransformLst()
    pub fn deleteTransformLst(&mut self) {
        // Java: this.map.put(9, (KoBin)null);
        self.map.clear_key(9);
        // Java: if (this.transformLst != null)
        if self.transformLst.is_some() {
            // Java: this.transformLst = null;
            self.transformLst = None;
        }
    }

    // Java: void writeTransformLst(ColorTransformList value)
    pub fn writeTransformLst(&mut self, value: Option<&ColorTransformList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(9, bin);
                self.map.put(9, bin);
                // Java: if (this.transformLst != null)
                if self.transformLst.is_some() {
                    // Java: this.transformLst = null;
                    self.transformLst = None;
                }
            }
        }
    }

    // Java: void forceWriteTransformLst(ColorTransformList value)
    pub fn forceWriteTransformLst(&mut self, value: Option<&ColorTransformList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(9, value.lastBin(this.zone));
            let bin = val.lastBin(self.zone);
            self.map.put(9, bin);
            // Java: if (this.transformLst != null)
            if self.transformLst.is_some() {
                // Java: this.transformLst = null;
                self.transformLst = None;
            }
        }
    }
}

// Java: public class RGB
impl RGB {
    // Java: static protected static <Value1 extends RGB> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = RGB::RGB();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static RGB create()
    pub fn create() -> Self {
        // Java: return initialize(RGB.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static RGB open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(RGB.class, bin);
        Self::initialize(bin)
    }

    // Java: RGB()
    pub fn RGB() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        RGB { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: RGB(KoBin bin)
    pub fn RGB_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        RGB { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(RGB value)
    pub fn load_from(&mut self, value: Option<&RGB>) {
        // Java: if (value != null)
        if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let lb = v.lastBin(self.zone);
            self.loadBin(lb.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let s = self.map.save(zone);
            self.bin_box.set(Some(s.as_ref()));
            s
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(RGB o)
    pub fn equalsLax(&self, o: &RGB) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RGB) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &RGB, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof RGB)
        if true {
        // Java: RGB dst = (RGB)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasR() || !dst.hasR()) && (!this.hasR() || dst.hasR()))
        else if (self.hasR() || !dst.hasR()) && (!self.hasR() || dst.hasR()) {
            // Java: if (this.hasR() && dst.hasR() && this.r() != dst.r())
            if self.hasR() && dst.hasR() && self.r() != dst.r() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasG() || !dst.hasG()) && (!this.hasG() || dst.hasG()))
            else if (self.hasG() || !dst.hasG()) && (!self.hasG() || dst.hasG()) {
                // Java: if (this.hasG() && dst.hasG() && this.g() != dst.g())
                if self.hasG() && dst.hasG() && self.g() != dst.g() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasB() || !dst.hasB()) && (!this.hasB() || dst.hasB()))
                else if (self.hasB() || !dst.hasB()) && (!self.hasB() || dst.hasB()) {
                    // Java: return !this.hasB() || !dst.hasB() || this.b() == dst.b();
                    return !self.hasB() || !dst.hasB() || self.b() == dst.b();
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
        // Java: if (this.hasR())
        if self.hasR() {
            // Java: ret += Integer.valueOf(this.r()).hashCode();
            ret += self.r();
        }
        // Java: if (this.hasG())
        if self.hasG() {
            // Java: ret += Integer.valueOf(this.g()).hashCode();
            ret += self.g();
        }
        // Java: if (this.hasB())
        if self.hasB() {
            // Java: ret += Integer.valueOf(this.b()).hashCode();
            ret += self.b();
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
    }

    // Java: void follow(RGB parent)
    pub fn follow(&mut self, parent: Option<Box<RGB>>) {
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

    // Java: RGB parent()
    pub fn parent_ref(&self) -> Option<&RGB> {
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

    // Java: int r()
    pub fn r(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.r();
            match &self.parent { None => 0, Some(p) => p.r() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasR()
    pub fn hasR(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasR();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasR())
    }

    // Java: void deleteR()
    pub fn deleteR(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putR(int value)
    pub fn putR(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(1, bin);
    }

    // Java: int g()
    pub fn g(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? 0 : this.parent.g();
            match &self.parent { None => 0, Some(p) => p.g() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasG()
    pub fn hasG(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasG();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasG())
    }

    // Java: void deleteG()
    pub fn deleteG(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putG(int value)
    pub fn putG(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(2, bin);
    }

    // Java: int b()
    pub fn b(&self) -> i32 {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? 0 : this.parent.b();
            match &self.parent { None => 0, Some(p) => p.b() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasB()
    pub fn hasB(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasB();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasB())
    }

    // Java: void deleteB()
    pub fn deleteB(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
    }

    // Java: void putB(int value)
    pub fn putB(&mut self, value: i32) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(3, bin);
    }
}

// Java: public class CRGB
impl CRGB {
    // Java: static protected static <Value1 extends CRGB> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = CRGB::CRGB();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static CRGB create()
    pub fn create() -> Self {
        // Java: return initialize(CRGB.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static CRGB open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(CRGB.class, bin);
        Self::initialize(bin)
    }

    // Java: CRGB()
    pub fn CRGB() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        CRGB { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: CRGB(KoBin bin)
    pub fn CRGB_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        CRGB { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(CRGB value)
    pub fn load_from(&mut self, value: Option<&CRGB>) {
        // Java: if (value != null)
        if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let lb = v.lastBin(self.zone);
            self.loadBin(lb.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let s = self.map.save(zone);
            self.bin_box.set(Some(s.as_ref()));
            s
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(CRGB o)
    pub fn equalsLax(&self, o: &CRGB) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CRGB) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &CRGB, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof CRGB)
        if true {
        // Java: CRGB dst = (CRGB)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasR() || !dst.hasR()) && (!this.hasR() || dst.hasR()))
        else if (self.hasR() || !dst.hasR()) && (!self.hasR() || dst.hasR()) {
            // Java: if (this.hasR() && dst.hasR() && this.r() != dst.r())
            if self.hasR() && dst.hasR() && (self.r() - dst.r()).abs() > f64::EPSILON {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasG() || !dst.hasG()) && (!this.hasG() || dst.hasG()))
            else if (self.hasG() || !dst.hasG()) && (!self.hasG() || dst.hasG()) {
                // Java: if (this.hasG() && dst.hasG() && this.g() != dst.g())
                if self.hasG() && dst.hasG() && (self.g() - dst.g()).abs() > f64::EPSILON {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasB() || !dst.hasB()) && (!this.hasB() || dst.hasB()))
                else if (self.hasB() || !dst.hasB()) && (!self.hasB() || dst.hasB()) {
                    // Java: return !this.hasB() || !dst.hasB() || this.b() == dst.b();
                    return !self.hasB() || !dst.hasB() || (self.b() - dst.b()).abs() <= f64::EPSILON;
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
        // Java: if (this.hasR())
        if self.hasR() {
            // Java: ret += Double.valueOf(this.r()).hashCode();
            ret += self.r().to_bits() as i32;
        }
        // Java: if (this.hasG())
        if self.hasG() {
            // Java: ret += Double.valueOf(this.g()).hashCode();
            ret += self.g().to_bits() as i32;
        }
        // Java: if (this.hasB())
        if self.hasB() {
            // Java: ret += Double.valueOf(this.b()).hashCode();
            ret += self.b().to_bits() as i32;
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
    }

    // Java: void follow(CRGB parent)
    pub fn follow(&mut self, parent: Option<Box<CRGB>>) {
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

    // Java: CRGB parent()
    pub fn parent_ref(&self) -> Option<&CRGB> {
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

    // Java: double r()
    pub fn r(&self) -> f64 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.r();
            match &self.parent { None => 0.0, Some(p) => p.r() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasR()
    pub fn hasR(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasR();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasR())
    }

    // Java: void deleteR()
    pub fn deleteR(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putR(double value)
    pub fn putR(&mut self, value: f64) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, self.zone);
        self.map.put(1, bin);
    }

    // Java: double g()
    pub fn g(&self) -> f64 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.g();
            match &self.parent { None => 0.0, Some(p) => p.g() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasG()
    pub fn hasG(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasG();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasG())
    }

    // Java: void deleteG()
    pub fn deleteG(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putG(double value)
    pub fn putG(&mut self, value: f64) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, self.zone);
        self.map.put(2, bin);
    }

    // Java: double b()
    pub fn b(&self) -> f64 {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.b();
            match &self.parent { None => 0.0, Some(p) => p.b() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasB()
    pub fn hasB(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasB();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasB())
    }

    // Java: void deleteB()
    pub fn deleteB(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
    }

    // Java: void putB(double value)
    pub fn putB(&mut self, value: f64) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, self.zone);
        self.map.put(3, bin);
    }
}

// Java: public class HSL
impl HSL {
    // Java: static protected static <Value1 extends HSL> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = HSL::HSL();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static HSL create()
    pub fn create() -> Self {
        // Java: return initialize(HSL.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static HSL open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(HSL.class, bin);
        Self::initialize(bin)
    }

    // Java: HSL()
    pub fn HSL() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        HSL { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: HSL(KoBin bin)
    pub fn HSL_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        HSL { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(HSL value)
    pub fn load_from(&mut self, value: Option<&HSL>) {
        // Java: if (value != null)
        if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let lb = v.lastBin(self.zone);
            self.loadBin(lb.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let s = self.map.save(zone);
            self.bin_box.set(Some(s.as_ref()));
            s
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(HSL o)
    pub fn equalsLax(&self, o: &HSL) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &HSL) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &HSL, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof HSL)
        if true {
        // Java: HSL dst = (HSL)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasH() || !dst.hasH()) && (!this.hasH() || dst.hasH()))
        else if (self.hasH() || !dst.hasH()) && (!self.hasH() || dst.hasH()) {
            // Java: if (this.hasH() && dst.hasH() && this.h() != dst.h())
            if self.hasH() && dst.hasH() && self.h() != dst.h() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasS() || !dst.hasS()) && (!this.hasS() || dst.hasS()))
            else if (self.hasS() || !dst.hasS()) && (!self.hasS() || dst.hasS()) {
                // Java: if (this.hasS() && dst.hasS() && this.s() != dst.s())
                if self.hasS() && dst.hasS() && (self.s() - dst.s()).abs() > f64::EPSILON {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasL() || !dst.hasL()) && (!this.hasL() || dst.hasL()))
                else if (self.hasL() || !dst.hasL()) && (!self.hasL() || dst.hasL()) {
                    // Java: return !this.hasL() || !dst.hasL() || this.l() == dst.l();
                    return !self.hasL() || !dst.hasL() || (self.l() - dst.l()).abs() <= f64::EPSILON;
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
        // Java: if (this.hasH())
        if self.hasH() {
            // Java: ret += Integer.valueOf(this.h()).hashCode();
            ret += self.h();
        }
        // Java: if (this.hasS())
        if self.hasS() {
            // Java: ret += Double.valueOf(this.s()).hashCode();
            ret += self.s().to_bits() as i32;
        }
        // Java: if (this.hasL())
        if self.hasL() {
            // Java: ret += Double.valueOf(this.l()).hashCode();
            ret += self.l().to_bits() as i32;
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
    }

    // Java: void follow(HSL parent)
    pub fn follow(&mut self, parent: Option<Box<HSL>>) {
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

    // Java: HSL parent()
    pub fn parent_ref(&self) -> Option<&HSL> {
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

    // Java: int h()
    pub fn h(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.h();
            match &self.parent { None => 0, Some(p) => p.h() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasH()
    pub fn hasH(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasH();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasH())
    }

    // Java: void deleteH()
    pub fn deleteH(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putH(int value)
    pub fn putH(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(1, bin);
    }

    // Java: double s()
    pub fn s(&self) -> f64 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.s();
            match &self.parent { None => 0.0, Some(p) => p.s() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasS()
    pub fn hasS(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasS();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasS())
    }

    // Java: void deleteS()
    pub fn deleteS(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putS(double value)
    pub fn putS(&mut self, value: f64) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, self.zone);
        self.map.put(2, bin);
    }

    // Java: double l()
    pub fn l(&self) -> f64 {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.l();
            match &self.parent { None => 0.0, Some(p) => p.l() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasL()
    pub fn hasL(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasL();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasL())
    }

    // Java: void deleteL()
    pub fn deleteL(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
    }

    // Java: void putL(double value)
    pub fn putL(&mut self, value: f64) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, self.zone);
        self.map.put(3, bin);
    }
}

// Java: public class ColorTransformList
impl ColorTransformList {
    // Java: ColorTransformList()
    pub fn ColorTransformList() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        ColorTransformList { bin_box: KpBinBox::new(), list: (), zone: KoZone::defaultZone() }
    }

    // Java: ColorTransformList(KoBin bin)
    pub fn ColorTransformList_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        ColorTransformList { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: (), zone: KoZone::defaultZone() }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.list.load(this.box.get(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }

    // Java: void load(ColorTransformList value)
    pub fn load_from(&mut self, value: Option<&ColorTransformList>) {
        // Java: if (value != null)
        if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let lb = v.lastBin(self.zone);
            self.loadBin(lb.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
        self.bin_box.get()
    }

    // Java: boolean equalsLax(ColorTransformList o)
    pub fn equalsLax(&self, o: &ColorTransformList) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ColorTransformList) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &ColorTransformList, _isStrict: bool) -> bool {
        // Java: if (null == o)
        if false {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: ColorTransformList dst = (ColorTransformList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: ArrayList<ColorTransform> dstList = new ArrayList();
                // Java: dst.read(dstList);
                // Java: ArrayList<ColorTransform> srcList = new ArrayList();
                // Java: this.read(srcList);
                // Java: if (dstList.size() != srcList.size())
                // Java: return false;
                // Java: else
                // Java: int size = dstList.size();
                // Java: for(int i = 0; i < size; ++i)
                // Java: ColorTransform dstClrTrans = (ColorTransform)dstList.get(i);
                // Java: ColorTransform srcClrTrans = (ColorTransform)srcList.get(i);
                // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                // Java: return false;
                // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                // Java: return false;
                // Java: return true;
                return true;
            }
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.list.hashCode();
        0
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.list.dirty();
        false
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
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends ColorTransformList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Self::ColorTransformList();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static ColorTransformList create()
    pub fn create() -> Self {
        // Java: return initialize(ColorTransformList.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static ColorTransformList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(ColorTransformList.class, bin);
        Self::initialize(bin)
    }

    // Java: void read(Collection<ColorTransform> valueLst)
    pub fn read(&self, _valueLst: &mut Vec<ColorTransform>) {
        // Java: valueLst.clear();
        // Java: int size = this.list.size();
        // Java: for(int i = 0; i < size; ++i)
        // Java: valueLst.add((ColorTransform)KoBin.Decode.toAny(ColorTransform.class, this.list.get(i), this.zone));
    }

    // Java: void write(Iterable<ColorTransform> valueLst)
    pub fn write(&mut self, _valueLst: &[ColorTransform]) {
        // Java: this.list.clear();
        // Java: for(ColorTransform i : valueLst)
        // Java: KoBin bin = i.lastBin(this.zone);
        // Java: if (KoBin.Null.notEmpty(bin))
        // Java: this.list.add(bin);
    }

    // Java: void forceWrite(Iterable<ColorTransform> valueLst)
    pub fn forceWrite(&mut self, _valueLst: &[()]) {
        // Java: this.list.clear();
        // Java: for(ColorTransform i : valueLst)
        // Java: this.list.add(i.lastBin(this.zone));
    }
}

impl KoAr for Color {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Color::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Color::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Color::lastBin(self, var1) }
}

impl KoClosable for Color {
    fn close(&mut self) { Color::close(self); }
}
