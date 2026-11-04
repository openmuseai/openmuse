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

use crate::cn::wps::drawing::data::text::TextCharacterProperties::TextCharacterProperties;
use crate::cn::wps::drawing::data::text::TextParagraphProperties::TextParagraphProperties;

// Java: public class TextRun implements KoAr, KoClosable
pub struct TextRun {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: RegularTextRun regularRun
    pub(crate) regularRun: Option<Box<RegularTextRun>>,
    // Java field: Linebreak linebreakRun
    pub(crate) linebreakRun: Option<Box<Linebreak>>,
    // Java field: TextField fieldRun
    pub(crate) fieldRun: Option<Box<TextField>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextRun parent
    pub(crate) parent: Option<Box<TextRun>>,
}

// Java: public class RegularTextRun
pub struct RegularTextRun {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextCharacterProperties rPr
    pub(crate) rPr: Option<Box<TextCharacterProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RegularTextRun parent
    pub(crate) parent: Option<Box<RegularTextRun>>,
}

// Java: public class Linebreak
pub struct Linebreak {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextCharacterProperties rPr
    pub(crate) rPr: Option<Box<TextCharacterProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Linebreak parent
    pub(crate) parent: Option<Box<Linebreak>>,
}

// Java: public class TextField
pub struct TextField {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextCharacterProperties rPr
    pub(crate) rPr: Option<Box<TextCharacterProperties>>,
    // Java field: TextParagraphProperties pPr
    pub(crate) pPr: Option<Box<TextParagraphProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextField parent
    pub(crate) parent: Option<Box<TextField>>,
}

// Java: public class TextRun implements KoAr, KoClosable
impl TextRun {
    // Java: static protected static <Value1 extends TextRun> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = TextRun::TextRun();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TextRun create()
    pub fn create() -> Self {
        // Java: return initialize(TextRun.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static TextRun open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TextRun.class, bin);
        Self::initialize(bin)
    }

    // Java: TextRun()
    pub fn TextRun() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        TextRun { zone: KoZone::defaultZone(), regularRun: None, linebreakRun: None, fieldRun: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: TextRun(KoBin bin)
    pub fn TextRun_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        TextRun { zone: KoZone::defaultZone(), regularRun: None, linebreakRun: None, fieldRun: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(TextRun value)
    pub fn load_from(&mut self, value: Option<&TextRun>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.regularRun != null && this.regularRun.dirty())
        if let Some(ref mut child) = self.regularRun {
            if child.dirty() {
                // Java: this.map.put(1, this.regularRun.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(1, bin);
            }
        }
        // Java: if (this.linebreakRun != null && this.linebreakRun.dirty())
        if let Some(ref mut child) = self.linebreakRun {
            if child.dirty() {
                // Java: this.map.put(2, this.linebreakRun.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(2, bin);
            }
        }
        // Java: if (this.fieldRun != null && this.fieldRun.dirty())
        if let Some(ref mut child) = self.fieldRun {
            if child.dirty() {
                // Java: this.map.put(3, this.fieldRun.saveBin(zone));
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

    // Java: boolean equalsLax(TextRun o)
    pub fn equalsLax(&self, o: &TextRun) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextRun) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &TextRun, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof TextRun)
        if true {
            // Java: TextRun dst = (TextRun)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasRegularRun() || !dst.hasRegularRun()) && (!this.hasRegularRun() || dst.hasRegularRun()))
            else if (self.hasRegularRun() || !dst.hasRegularRun()) && (!self.hasRegularRun() || dst.hasRegularRun()) {
                // Java: if (this.hasRegularRun() && dst.hasRegularRun())
                if self.hasRegularRun() && dst.hasRegularRun() {
                    // Java: if (isStrict && !this.regularRun().equals(dst.regularRun()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.regularRun().equalsLax(dst.regularRun()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasLinebreakRun() || !dst.hasLinebreakRun()) && (!this.hasLinebreakRun() || dst.hasLinebreakRun()))
                if (self.hasLinebreakRun() || !dst.hasLinebreakRun()) && (!self.hasLinebreakRun() || dst.hasLinebreakRun()) {
                    // Java: if (this.hasLinebreakRun() && dst.hasLinebreakRun())
                    if self.hasLinebreakRun() && dst.hasLinebreakRun() {
                        // Java: if (isStrict && !this.linebreakRun().equals(dst.linebreakRun()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.linebreakRun().equalsLax(dst.linebreakRun()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasFieldRun() || !dst.hasFieldRun()) && (!this.hasFieldRun() || dst.hasFieldRun()))
                    if (self.hasFieldRun() || !dst.hasFieldRun()) && (!self.hasFieldRun() || dst.hasFieldRun()) {
                        // Java: if (this.hasFieldRun() && dst.hasFieldRun())
                        if self.hasFieldRun() && dst.hasFieldRun() {
                            // Java: if (isStrict && !this.fieldRun().equals(dst.fieldRun()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.fieldRun().equalsLax(dst.fieldRun()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasMode() || !dst.hasMode()) && (!this.hasMode() || dst.hasMode()))
                        if (self.hasMode() || !dst.hasMode()) && (!self.hasMode() || dst.hasMode()) {
                            // Java: if (this.hasMode() && dst.hasMode() && this.mode() != dst.mode())
                            if self.hasMode() && dst.hasMode() && self.mode() != dst.mode() {
                                // Java: return false;
                                return false;
                            }
                            // Java: else if ((this.hasLength() || !dst.hasLength()) && (!this.hasLength() || dst.hasLength()))
                            else if (self.hasLength() || !dst.hasLength()) && (!self.hasLength() || dst.hasLength()) {
                                // Java: return !this.hasLength() || !dst.hasLength() || this.length() == dst.length();
                                return !self.hasLength() || !dst.hasLength() || self.length() == dst.length();
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
        // Java: if (this.hasRegularRun())
        if self.hasRegularRun() {
            // Java: ret += this.regularRun().hashCode();
        }
        // Java: if (this.hasLinebreakRun())
        if self.hasLinebreakRun() {
            // Java: ret += this.linebreakRun().hashCode();
        }
        // Java: if (this.hasFieldRun())
        if self.hasFieldRun() {
            // Java: ret += this.fieldRun().hashCode();
        }
        // Java: if (this.hasMode())
        if self.hasMode() {
            // Java: ret += Integer.valueOf(this.mode()).hashCode();
            ret += self.mode();
        }
        // Java: if (this.hasLength())
        if self.hasLength() {
            // Java: ret += Integer.valueOf(this.length()).hashCode();
            ret += self.length();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.regularRun != null && this.regularRun.dirty())
        if self.regularRun.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.linebreakRun != null && this.linebreakRun.dirty())
        else if self.linebreakRun.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.fieldRun != null && this.fieldRun.dirty() ? true : this.map.dirty();
            if self.fieldRun.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.regularRun = null;
        self.regularRun = None;
        // Java: this.linebreakRun = null;
        self.linebreakRun = None;
        // Java: this.fieldRun = null;
        self.fieldRun = None;
    }

    // Java: void follow(TextRun parent)
    pub fn follow(&mut self, parent: &mut TextRun) {
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

    // Java: TextRun parent()
    pub fn parent_ref(&self) -> Option<&TextRun> {
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

    // Java: RegularTextRun regularRun()
    pub fn regularRun(&self) -> &RegularTextRun {
        // Java: if (this.regularRun == null)
        // Java: if (this.map.has(1))
        // Java: KoBin bin = this.map.byKey(1);
        // Java: this.regularRun = (RegularTextRun)KoBin.Decode.toAny(RegularTextRun.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasRegularRun())
        // Java: this.regularRun = this.parent.regularRun();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.regularRun = (RegularTextRun)KoBin.Decode.toAny(RegularTextRun.class, bin, this.zone);
        // Java: return this.regularRun;
        todo!()
    }

    // Java: boolean hasRegularRun()
    pub fn hasRegularRun(&self) -> bool {
        // Java: return this.map.has(1) || this.regularRun != null && this.regularRun.dirty() || this.parent != null && this.parent.hasRegularRun();
        self.map.has(1) || self.regularRun.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRegularRun())
    }

    // Java: void deleteRegularRun()
    pub fn deleteRegularRun(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
        // Java: if (this.regularRun != null)
        if self.regularRun.is_some() {
            // Java: this.regularRun = null;
            self.regularRun = None;
        }
    }

    // Java: void writeRegularRun(RegularTextRun value)
    pub fn writeRegularRun(&mut self, value: &RegularTextRun) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(1, bin);
                // Java: if (this.regularRun != null)
                if self.regularRun.is_some() {
                    // Java: this.regularRun = null;
                    self.regularRun = None;
                }
            }
        }
    }

    // Java: void forceWriteRegularRun(RegularTextRun value)
    pub fn forceWriteRegularRun(&mut self, value: &RegularTextRun) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(1, bin);
            // Java: if (this.regularRun != null)
            if self.regularRun.is_some() {
                // Java: this.regularRun = null;
                self.regularRun = None;
            }
        }
    }

    // Java: Linebreak linebreakRun()
    pub fn linebreakRun(&self) -> &Linebreak {
        // Java: if (this.linebreakRun == null)
        // Java: if (this.map.has(2))
        // Java: KoBin bin = this.map.byKey(2);
        // Java: this.linebreakRun = (Linebreak)KoBin.Decode.toAny(Linebreak.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasLinebreakRun())
        // Java: this.linebreakRun = this.parent.linebreakRun();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.linebreakRun = (Linebreak)KoBin.Decode.toAny(Linebreak.class, bin, this.zone);
        // Java: return this.linebreakRun;
        todo!()
    }

    // Java: boolean hasLinebreakRun()
    pub fn hasLinebreakRun(&self) -> bool {
        // Java: return this.map.has(2) || this.linebreakRun != null && this.linebreakRun.dirty() || this.parent != null && this.parent.hasLinebreakRun();
        self.map.has(2) || self.linebreakRun.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLinebreakRun())
    }

    // Java: void deleteLinebreakRun()
    pub fn deleteLinebreakRun(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.linebreakRun != null)
        if self.linebreakRun.is_some() {
            // Java: this.linebreakRun = null;
            self.linebreakRun = None;
        }
    }

    // Java: void writeLinebreakRun(Linebreak value)
    pub fn writeLinebreakRun(&mut self, value: &Linebreak) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.linebreakRun != null)
                if self.linebreakRun.is_some() {
                    // Java: this.linebreakRun = null;
                    self.linebreakRun = None;
                }
            }
        }
    }

    // Java: void forceWriteLinebreakRun(Linebreak value)
    pub fn forceWriteLinebreakRun(&mut self, value: &Linebreak) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(2, bin);
            // Java: if (this.linebreakRun != null)
            if self.linebreakRun.is_some() {
                // Java: this.linebreakRun = null;
                self.linebreakRun = None;
            }
        }
    }

    // Java: TextField fieldRun()
    pub fn fieldRun(&self) -> &TextField {
        // Java: if (this.fieldRun == null)
        // Java: if (this.map.has(3))
        // Java: KoBin bin = this.map.byKey(3);
        // Java: this.fieldRun = (TextField)KoBin.Decode.toAny(TextField.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasFieldRun())
        // Java: this.fieldRun = this.parent.fieldRun();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.fieldRun = (TextField)KoBin.Decode.toAny(TextField.class, bin, this.zone);
        // Java: return this.fieldRun;
        todo!()
    }

    // Java: boolean hasFieldRun()
    pub fn hasFieldRun(&self) -> bool {
        // Java: return this.map.has(3) || this.fieldRun != null && this.fieldRun.dirty() || this.parent != null && this.parent.hasFieldRun();
        self.map.has(3) || self.fieldRun.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFieldRun())
    }

    // Java: void deleteFieldRun()
    pub fn deleteFieldRun(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.fieldRun != null)
        if self.fieldRun.is_some() {
            // Java: this.fieldRun = null;
            self.fieldRun = None;
        }
    }

    // Java: void writeFieldRun(TextField value)
    pub fn writeFieldRun(&mut self, value: &TextField) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.fieldRun != null)
                if self.fieldRun.is_some() {
                    // Java: this.fieldRun = null;
                    self.fieldRun = None;
                }
            }
        }
    }

    // Java: void forceWriteFieldRun(TextField value)
    pub fn forceWriteFieldRun(&mut self, value: &TextField) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(3, bin);
            // Java: if (this.fieldRun != null)
            if self.fieldRun.is_some() {
                // Java: this.fieldRun = null;
                self.fieldRun = None;
            }
        }
    }

    // Java: int mode()
    pub fn mode(&self) -> i32 {
        // Java: if (!this.map.has(4))
        if !self.map.has(4) {
            // Java: return this.parent == null ? 0 : this.parent.mode();
            match &self.parent { None => 0, Some(p) => p.mode() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasMode()
    pub fn hasMode(&self) -> bool {
        // Java: return this.map.has(4) || this.parent != null && this.parent.hasMode();
        self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasMode())
    }

    // Java: void deleteMode()
    pub fn deleteMode(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
    }

    // Java: void putMode(int value)
    pub fn putMode(&mut self, value: i32) {
        // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(4, bin);
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: if (!this.map.has(5))
        if !self.map.has(5) {
            // Java: return this.parent == null ? 0 : this.parent.length();
            match &self.parent { None => 0, Some(p) => p.length() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasLength()
    pub fn hasLength(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasLength();
        self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasLength())
    }

    // Java: void deleteLength()
    pub fn deleteLength(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
    }

    // Java: void putLength(int value)
    pub fn putLength(&mut self, value: i32) {
        // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(5, bin);
    }
}

// Java: public class RegularTextRun
impl RegularTextRun {
    // Java: static protected static <Value1 extends RegularTextRun> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = RegularTextRun::RegularTextRun();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static RegularTextRun create()
    pub fn create() -> Self {
        // Java: return initialize(RegularTextRun.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static RegularTextRun open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(RegularTextRun.class, bin);
        Self::initialize(bin)
    }

    // Java: RegularTextRun()
    pub fn RegularTextRun() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        RegularTextRun { zone: KoZone::defaultZone(), rPr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: RegularTextRun(KoBin bin)
    pub fn RegularTextRun_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        RegularTextRun { zone: KoZone::defaultZone(), rPr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(RegularTextRun value)
    pub fn load_from(&mut self, value: Option<&RegularTextRun>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.rPr != null && this.rPr.dirty())
        if let Some(ref mut child) = self.rPr {
            if child.dirty() {
                // Java: this.map.put(2, this.rPr.saveBin(zone));
                let bin = child.saveBin(zone);
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

    // Java: boolean equalsLax(RegularTextRun o)
    pub fn equalsLax(&self, o: &RegularTextRun) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RegularTextRun) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &RegularTextRun, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof RegularTextRun)
        if true {
            // Java: RegularTextRun dst = (RegularTextRun)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if (this.mode() != dst.mode())
            else if self.mode() != dst.mode() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasRPr() || !dst.hasRPr()) && (!this.hasRPr() || dst.hasRPr()))
            else if (self.hasRPr() || !dst.hasRPr()) && (!self.hasRPr() || dst.hasRPr()) {
                // Java: if (this.hasRPr() && dst.hasRPr())
                if self.hasRPr() && dst.hasRPr() {
                    // Java: if (isStrict && !this.rPr().equals(dst.rPr()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.rPr().equalsLax(dst.rPr()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasText() || !dst.hasText()) && (!this.hasText() || dst.hasText()))
                if (self.hasText() || !dst.hasText()) && (!self.hasText() || dst.hasText()) {
                    // Java: return !this.hasText() || !dst.hasText() || this.text().equals(dst.text());
                    return !self.hasText() || !dst.hasText() || self.text() == dst.text();
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
        // Java: ret += Integer.valueOf(this.mode()).hashCode();
        ret += self.mode();
        // Java: if (this.hasRPr())
        if self.hasRPr() {
            // Java: ret += this.rPr().hashCode();
        }
        // Java: if (this.hasText())
        if self.hasText() {
            // Java: ret += String.valueOf(this.text()).hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.rPr != null && this.rPr.dirty() ? true : this.map.dirty();
        if self.rPr.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.rPr = null;
        self.rPr = None;
    }

    // Java: void follow(RegularTextRun parent)
    pub fn follow(&mut self, parent: &mut RegularTextRun) {
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

    // Java: RegularTextRun parent()
    pub fn parent_ref(&self) -> Option<&RegularTextRun> {
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
            // Java: return this.parent == null ? 0 : this.parent.mode();
            match &self.parent { None => 0, Some(p) => p.mode() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: TextCharacterProperties rPr()
    pub fn rPr(&self) -> &TextCharacterProperties {
        // Java: if (this.rPr == null)
        // Java: if (this.map.has(2))
        // Java: KoBin bin = this.map.byKey(2);
        // Java: this.rPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasRPr())
        // Java: this.rPr = this.parent.rPr();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.rPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
        // Java: return this.rPr;
        todo!()
    }

    // Java: boolean hasRPr()
    pub fn hasRPr(&self) -> bool {
        // Java: return this.map.has(2) || this.rPr != null && this.rPr.dirty() || this.parent != null && this.parent.hasRPr();
        self.map.has(2) || self.rPr.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRPr())
    }

    // Java: void deleteRPr()
    pub fn deleteRPr(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.rPr != null)
        if self.rPr.is_some() {
            // Java: this.rPr = null;
            self.rPr = None;
        }
    }

    // Java: void writeRPr(TextCharacterProperties value)
    pub fn writeRPr(&mut self, value: &TextCharacterProperties) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.rPr != null)
                if self.rPr.is_some() {
                    // Java: this.rPr = null;
                    self.rPr = None;
                }
            }
        }
    }

    // Java: void forceWriteRPr(TextCharacterProperties value)
    pub fn forceWriteRPr(&mut self, value: &TextCharacterProperties) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(2, bin);
            // Java: if (this.rPr != null)
            if self.rPr.is_some() {
                // Java: this.rPr = null;
                self.rPr = None;
            }
        }
    }

    // Java: String text()
    pub fn text(&self) -> String {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? "" : this.parent.text();
            match &self.parent { None => String::new(), Some(p) => p.text() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasText();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasText())
    }

    // Java: void deleteText()
    pub fn deleteText(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
    }

    // Java: void putText(String value)
    pub fn putText(&mut self, value: &str) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(3, bin);
    }
}

// Java: public class Linebreak
impl Linebreak {
    // Java: static protected static <Value1 extends Linebreak> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Linebreak::Linebreak();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Linebreak create()
    pub fn create() -> Self {
        // Java: return initialize(Linebreak.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Linebreak open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Linebreak.class, bin);
        Self::initialize(bin)
    }

    // Java: Linebreak()
    pub fn Linebreak() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Linebreak { zone: KoZone::defaultZone(), rPr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Linebreak(KoBin bin)
    pub fn Linebreak_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Linebreak { zone: KoZone::defaultZone(), rPr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(Linebreak value)
    pub fn load_from(&mut self, value: Option<&Linebreak>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.rPr != null && this.rPr.dirty())
        if let Some(ref mut child) = self.rPr {
            if child.dirty() {
                // Java: this.map.put(2, this.rPr.saveBin(zone));
                let bin = child.saveBin(zone);
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

    // Java: boolean equalsLax(Linebreak o)
    pub fn equalsLax(&self, o: &Linebreak) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Linebreak) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Linebreak, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Linebreak)
        if true {
            // Java: Linebreak dst = (Linebreak)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if (this.mode() != dst.mode())
            else if self.mode() != dst.mode() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasRPr() || !dst.hasRPr()) && (!this.hasRPr() || dst.hasRPr()))
            else if (self.hasRPr() || !dst.hasRPr()) && (!self.hasRPr() || dst.hasRPr()) {
                // Java: if (this.hasRPr() && dst.hasRPr())
                if self.hasRPr() && dst.hasRPr() {
                    // Java: if (isStrict && !this.rPr().equals(dst.rPr()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.rPr().equalsLax(dst.rPr()))
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
        // Java: ret += Integer.valueOf(this.mode()).hashCode();
        ret += self.mode();
        // Java: if (this.hasRPr())
        if self.hasRPr() {
            // Java: ret += this.rPr().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.rPr != null && this.rPr.dirty() ? true : this.map.dirty();
        if self.rPr.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.rPr = null;
        self.rPr = None;
    }

    // Java: void follow(Linebreak parent)
    pub fn follow(&mut self, parent: &mut Linebreak) {
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

    // Java: Linebreak parent()
    pub fn parent_ref(&self) -> Option<&Linebreak> {
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

    // Java: TextCharacterProperties rPr()
    pub fn rPr(&self) -> &TextCharacterProperties {
        // Java: if (this.rPr == null)
        // Java: if (this.map.has(2))
        // Java: KoBin bin = this.map.byKey(2);
        // Java: this.rPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasRPr())
        // Java: this.rPr = this.parent.rPr();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.rPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
        // Java: return this.rPr;
        todo!()
    }

    // Java: boolean hasRPr()
    pub fn hasRPr(&self) -> bool {
        // Java: return this.map.has(2) || this.rPr != null && this.rPr.dirty() || this.parent != null && this.parent.hasRPr();
        self.map.has(2) || self.rPr.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRPr())
    }

    // Java: void deleteRPr()
    pub fn deleteRPr(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.rPr != null)
        if self.rPr.is_some() {
            // Java: this.rPr = null;
            self.rPr = None;
        }
    }

    // Java: void writeRPr(TextCharacterProperties value)
    pub fn writeRPr(&mut self, value: &TextCharacterProperties) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.rPr != null)
                if self.rPr.is_some() {
                    // Java: this.rPr = null;
                    self.rPr = None;
                }
            }
        }
    }

    // Java: void forceWriteRPr(TextCharacterProperties value)
    pub fn forceWriteRPr(&mut self, value: &TextCharacterProperties) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(2, bin);
            // Java: if (this.rPr != null)
            if self.rPr.is_some() {
                // Java: this.rPr = null;
                self.rPr = None;
            }
        }
    }
}

// Java: public class TextField
impl TextField {
    // Java: static protected static <Value1 extends TextField> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = TextField::TextField();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TextField create()
    pub fn create() -> Self {
        // Java: return initialize(TextField.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static TextField open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TextField.class, bin);
        Self::initialize(bin)
    }

    // Java: TextField()
    pub fn TextField() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        TextField { zone: KoZone::defaultZone(), rPr: None, pPr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: TextField(KoBin bin)
    pub fn TextField_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        TextField { zone: KoZone::defaultZone(), rPr: None, pPr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(TextField value)
    pub fn load_from(&mut self, value: Option<&TextField>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.rPr != null && this.rPr.dirty())
        if let Some(ref mut child) = self.rPr {
            if child.dirty() {
                // Java: this.map.put(3, this.rPr.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(3, bin);
            }
        }
        // Java: if (this.pPr != null && this.pPr.dirty())
        if let Some(ref mut child) = self.pPr {
            if child.dirty() {
                // Java: this.map.put(4, this.pPr.saveBin(zone));
                let bin = child.saveBin(zone);
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

    // Java: boolean equalsLax(TextField o)
    pub fn equalsLax(&self, o: &TextField) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextField) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &TextField, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof TextField)
        if true {
            // Java: TextField dst = (TextField)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if (this.mode() != dst.mode())
            else if self.mode() != dst.mode() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasText() || !dst.hasText()) && (!this.hasText() || dst.hasText()))
            else if (self.hasText() || !dst.hasText()) && (!self.hasText() || dst.hasText()) {
                // Java: if (this.hasText() && dst.hasText() && !this.text().equals(dst.text()))
                if self.hasText() && dst.hasText() && self.text() != dst.text() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasRPr() || !dst.hasRPr()) && (!this.hasRPr() || dst.hasRPr()))
                else if (self.hasRPr() || !dst.hasRPr()) && (!self.hasRPr() || dst.hasRPr()) {
                    // Java: if (this.hasRPr() && dst.hasRPr())
                    if self.hasRPr() && dst.hasRPr() {
                        // Java: if (isStrict && !this.rPr().equals(dst.rPr()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.rPr().equalsLax(dst.rPr()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasPPr() || !dst.hasPPr()) && (!this.hasPPr() || dst.hasPPr()))
                    if (self.hasPPr() || !dst.hasPPr()) && (!self.hasPPr() || dst.hasPPr()) {
                        // Java: if (this.hasPPr() && dst.hasPPr())
                        if self.hasPPr() && dst.hasPPr() {
                            // Java: if (isStrict && !this.pPr().equals(dst.pPr()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.pPr().equalsLax(dst.pPr()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasId() || !dst.hasId()) && (!this.hasId() || dst.hasId()))
                        if (self.hasId() || !dst.hasId()) && (!self.hasId() || dst.hasId()) {
                            // Java: if (this.hasId() && dst.hasId() && !this.id().equals(dst.id()))
                            if self.hasId() && dst.hasId() && self.id() != dst.id() {
                                // Java: return false;
                                return false;
                            }
                            // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                            else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType()) {
                                // Java: return !this.hasType() || !dst.hasType() || this.type().equals(dst.type());
                                return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
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
        // Java: ret += Integer.valueOf(this.mode()).hashCode();
        ret += self.mode();
        // Java: if (this.hasText())
        if self.hasText() {
            // Java: ret += String.valueOf(this.text()).hashCode();
        }
        // Java: if (this.hasRPr())
        if self.hasRPr() {
            // Java: ret += this.rPr().hashCode();
        }
        // Java: if (this.hasPPr())
        if self.hasPPr() {
            // Java: ret += this.pPr().hashCode();
        }
        // Java: if (this.hasId())
        if self.hasId() {
            // Java: ret += String.valueOf(this.id()).hashCode();
        }
        // Java: if (this.hasType())
        if self.hasType() {
            // Java: ret += String.valueOf(this.type()).hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.rPr != null && this.rPr.dirty())
        if self.rPr.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.pPr != null && this.pPr.dirty() ? true : this.map.dirty();
            if self.pPr.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.rPr = null;
        self.rPr = None;
        // Java: this.pPr = null;
        self.pPr = None;
    }

    // Java: void follow(TextField parent)
    pub fn follow(&mut self, parent: &mut TextField) {
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

    // Java: TextField parent()
    pub fn parent_ref(&self) -> Option<&TextField> {
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
            // Java: return this.parent == null ? 2 : this.parent.mode();
            match &self.parent { None => 2, Some(p) => p.mode() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: String text()
    pub fn text(&self) -> String {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? "" : this.parent.text();
            match &self.parent { None => String::new(), Some(p) => p.text() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasText();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasText())
    }

    // Java: void deleteText()
    pub fn deleteText(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putText(String value)
    pub fn putText(&mut self, value: &str) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(2, bin);
    }

    // Java: TextCharacterProperties rPr()
    pub fn rPr(&self) -> &TextCharacterProperties {
        // Java: if (this.rPr == null)
        // Java: if (this.map.has(3))
        // Java: KoBin bin = this.map.byKey(3);
        // Java: this.rPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasRPr())
        // Java: this.rPr = this.parent.rPr();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.rPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
        // Java: return this.rPr;
        todo!()
    }

    // Java: boolean hasRPr()
    pub fn hasRPr(&self) -> bool {
        // Java: return this.map.has(3) || this.rPr != null && this.rPr.dirty() || this.parent != null && this.parent.hasRPr();
        self.map.has(3) || self.rPr.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRPr())
    }

    // Java: void deleteRPr()
    pub fn deleteRPr(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.rPr != null)
        if self.rPr.is_some() {
            // Java: this.rPr = null;
            self.rPr = None;
        }
    }

    // Java: void writeRPr(TextCharacterProperties value)
    pub fn writeRPr(&mut self, value: &TextCharacterProperties) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.rPr != null)
                if self.rPr.is_some() {
                    // Java: this.rPr = null;
                    self.rPr = None;
                }
            }
        }
    }

    // Java: void forceWriteRPr(TextCharacterProperties value)
    pub fn forceWriteRPr(&mut self, value: &TextCharacterProperties) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(3, bin);
            // Java: if (this.rPr != null)
            if self.rPr.is_some() {
                // Java: this.rPr = null;
                self.rPr = None;
            }
        }
    }

    // Java: TextParagraphProperties pPr()
    pub fn pPr(&self) -> &TextParagraphProperties {
        // Java: if (this.pPr == null)
        // Java: if (this.map.has(4))
        // Java: KoBin bin = this.map.byKey(4);
        // Java: this.pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasPPr())
        // Java: this.pPr = this.parent.pPr();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
        // Java: return this.pPr;
        todo!()
    }

    // Java: boolean hasPPr()
    pub fn hasPPr(&self) -> bool {
        // Java: return this.map.has(4) || this.pPr != null && this.pPr.dirty() || this.parent != null && this.parent.hasPPr();
        self.map.has(4) || self.pPr.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPPr())
    }

    // Java: void deletePPr()
    pub fn deletePPr(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.pPr != null)
        if self.pPr.is_some() {
            // Java: this.pPr = null;
            self.pPr = None;
        }
    }

    // Java: void writePPr(TextParagraphProperties value)
    pub fn writePPr(&mut self, value: &TextParagraphProperties) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.pPr != null)
                if self.pPr.is_some() {
                    // Java: this.pPr = null;
                    self.pPr = None;
                }
            }
        }
    }

    // Java: void forceWritePPr(TextParagraphProperties value)
    pub fn forceWritePPr(&mut self, value: &TextParagraphProperties) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(4, bin);
            // Java: if (this.pPr != null)
            if self.pPr.is_some() {
                // Java: this.pPr = null;
                self.pPr = None;
            }
        }
    }

    // Java: String id()
    pub fn id(&self) -> String {
        // Java: if (!this.map.has(5))
        if !self.map.has(5) {
            // Java: return this.parent == null ? "" : this.parent.id();
            match &self.parent { None => String::new(), Some(p) => p.id() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasId()
    pub fn hasId(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasId();
        self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasId())
    }

    // Java: void deleteId()
    pub fn deleteId(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
    }

    // Java: void putId(String value)
    pub fn putId(&mut self, value: &str) {
        // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(5, bin);
    }

    // Java: String type()
    pub fn r#type(&self) -> String {
        // Java: if (!this.map.has(6))
        if !self.map.has(6) {
            // Java: return this.parent == null ? "" : this.parent.type();
            match &self.parent { None => String::new(), Some(p) => p.r#type() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(6);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
        // Java: return this.map.has(6) || this.parent != null && this.parent.hasType();
        self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(6);
    }

    // Java: void putType(String value)
    pub fn putType(&mut self, value: &str) {
        // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(6, bin);
    }
}

impl KoAr for TextRun {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { TextRun::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { TextRun::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { TextRun::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TextRun::lastBin(self, var1) }
}

impl KoClosable for TextRun {
    fn close(&mut self) { TextRun::close(self); }
}
