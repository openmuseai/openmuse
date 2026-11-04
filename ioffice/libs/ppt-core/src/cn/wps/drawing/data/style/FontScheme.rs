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

use crate::cn::wps::drawing::data::style::FontCollection::FontCollection;

// Java: public class FontScheme implements KoAr, KoClosable
pub struct FontScheme {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: FontCollection major
    pub(crate) major: Option<Box<FontCollection>>,
    // Java field: FontCollection minor
    pub(crate) minor: Option<Box<FontCollection>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: FontScheme parent
    pub(crate) parent: Option<Box<FontScheme>>,
}

// Java: public class FontScheme implements KoAr, KoClosable
impl FontScheme {
    // Java: static protected static <Value1 extends FontScheme> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = FontScheme::FontScheme();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static FontScheme create()
    pub fn create() -> Self {
        // Java: return initialize(FontScheme.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static FontScheme open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(FontScheme.class, bin);
        Self::initialize(bin)
    }

    // Java: FontScheme()
    pub fn FontScheme() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        FontScheme { zone: KoZone::defaultZone(), major: None, minor: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: FontScheme(KoBin bin)
    pub fn FontScheme_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        FontScheme { zone: KoZone::defaultZone(), major: None, minor: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(FontScheme value)
    pub fn load_from(&mut self, value: Option<&FontScheme>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.major != null && this.major.dirty())
        if let Some(ref mut child) = self.major {
            if child.dirty() {
                // Java: this.map.put(2, this.major.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(2, bin);
            }
        }
        // Java: if (this.minor != null && this.minor.dirty())
        if let Some(ref mut child) = self.minor {
            if child.dirty() {
                // Java: this.map.put(3, this.minor.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(3, bin);
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

    // Java: boolean equalsLax(FontScheme o)
    pub fn equalsLax(&self, o: &FontScheme) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &FontScheme) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &FontScheme, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof FontScheme)
        if true {
            // Java: FontScheme dst = (FontScheme)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
                // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
                if self.hasName() && dst.hasName() && self.name() != dst.name() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasMajor() || !dst.hasMajor()) && (!this.hasMajor() || dst.hasMajor()))
                else if (self.hasMajor() || !dst.hasMajor()) && (!self.hasMajor() || dst.hasMajor()) {
                    // Java: if (this.hasMajor() && dst.hasMajor())
                    if self.hasMajor() && dst.hasMajor() {
                        // Java: if (isStrict && !this.major().equals(dst.major()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.major().equalsLax(dst.major()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasMinor() || !dst.hasMinor()) && (!this.hasMinor() || dst.hasMinor()))
                    if (self.hasMinor() || !dst.hasMinor()) && (!self.hasMinor() || dst.hasMinor()) {
                        // Java: if (this.hasMinor() && dst.hasMinor())
                        if self.hasMinor() && dst.hasMinor() {
                            // Java: if (isStrict && !this.minor().equals(dst.minor()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.minor().equalsLax(dst.minor()))
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
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasName())
        if self.hasName() {
            // Java: ret += String.valueOf(this.name()).hashCode();
        }
        // Java: if (this.hasMajor())
        if self.hasMajor() {
            // Java: ret += this.major().hashCode();
        }
        // Java: if (this.hasMinor())
        if self.hasMinor() {
            // Java: ret += this.minor().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.major != null && this.major.dirty())
        if self.major.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.minor != null && this.minor.dirty() ? true : this.map.dirty();
            if self.minor.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.major = null;
        self.major = None;
        // Java: this.minor = null;
        self.minor = None;
    }

    // Java: void follow(FontScheme parent)
    pub fn follow(&mut self, parent: &mut FontScheme) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: FontScheme parent()
    pub fn parent_ref(&self) -> Option<&FontScheme> {
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

    // Java: String name()
    pub fn name(&self) -> String {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? "" : this.parent.name();
            match &self.parent { None => String::new(), Some(p) => p.name() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(1, bin);
    }

    // Java: FontCollection major()
    pub fn major(&self) -> &FontCollection {
        // Java: if (this.major == null)
        if self.major.is_none()
        {
            // Java: if (this.map.has(2))
            if self.map.has(2)
            {
                // Java: KoBin bin = this.map.byKey(2);
                let _bin = self.map.byKey(2);
                // Java: this.major = (FontCollection)KoBin.Decode.toAny(FontCollection.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasMajor())
            else if self.parent.as_ref().map_or(false, |p| p.hasMajor())
            {
                // Java: this.major = this.parent.major();
            }
            // Java: else
            else
            {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.major = (FontCollection)KoBin.Decode.toAny(FontCollection.class, bin, this.zone);
            }
        }
        // Java: return this.major;
        self.major.as_ref().unwrap()
    }

    // Java: boolean hasMajor()
    pub fn hasMajor(&self) -> bool {
        // Java: return this.map.has(2) || this.major != null && this.major.dirty() || this.parent != null && this.parent.hasMajor();
        self.map.has(2) || self.major.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMajor())
    }

    // Java: void deleteMajor()
    pub fn deleteMajor(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.major != null)
        if self.major.is_some() {
            // Java: this.major = null;
            self.major = None;
        }
    }

    // Java: void writeMajor(FontCollection value)
    pub fn writeMajor(&mut self, value: &FontCollection) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.major != null)
                if self.major.is_some() {
                    // Java: this.major = null;
                    self.major = None;
                }
            }
        }
    }

    // Java: void forceWriteMajor(FontCollection value)
    pub fn forceWriteMajor(&mut self, value: &FontCollection) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(2, bin);
            // Java: if (this.major != null)
            if self.major.is_some() {
                // Java: this.major = null;
                self.major = None;
            }
        }
    }

    // Java: FontCollection minor()
    pub fn minor(&self) -> &FontCollection {
        // Java: if (this.minor == null)
        if self.minor.is_none()
        {
            // Java: if (this.map.has(3))
            if self.map.has(3)
            {
                // Java: KoBin bin = this.map.byKey(3);
                let _bin = self.map.byKey(3);
                // Java: this.minor = (FontCollection)KoBin.Decode.toAny(FontCollection.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasMinor())
            else if self.parent.as_ref().map_or(false, |p| p.hasMinor())
            {
                // Java: this.minor = this.parent.minor();
            }
            // Java: else
            else
            {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.minor = (FontCollection)KoBin.Decode.toAny(FontCollection.class, bin, this.zone);
            }
        }
        // Java: return this.minor;
        self.minor.as_ref().unwrap()
    }

    // Java: boolean hasMinor()
    pub fn hasMinor(&self) -> bool {
        // Java: return this.map.has(3) || this.minor != null && this.minor.dirty() || this.parent != null && this.parent.hasMinor();
        self.map.has(3) || self.minor.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMinor())
    }

    // Java: void deleteMinor()
    pub fn deleteMinor(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.minor != null)
        if self.minor.is_some() {
            // Java: this.minor = null;
            self.minor = None;
        }
    }

    // Java: void writeMinor(FontCollection value)
    pub fn writeMinor(&mut self, value: &FontCollection) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.minor != null)
                if self.minor.is_some() {
                    // Java: this.minor = null;
                    self.minor = None;
                }
            }
        }
    }

    // Java: void forceWriteMinor(FontCollection value)
    pub fn forceWriteMinor(&mut self, value: &FontCollection) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(3, bin);
            // Java: if (this.minor != null)
            if self.minor.is_some() {
                // Java: this.minor = null;
                self.minor = None;
            }
        }
    }
}

impl KoAr for FontScheme {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { FontScheme::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { FontScheme::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { FontScheme::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { FontScheme::lastBin(self, var1) }
}

impl KoClosable for FontScheme {
    fn close(&mut self) { FontScheme::close(self); }
}
