#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
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

const CNVPR_ID: i32 = 1;
const CNVPR_NAME: i32 = 2;
const CNVPR_DESCR: i32 = 3;
const CNVPR_HIDDEN: i32 = 4;
const CNVPR_TITLE: i32 = 5;
const CNVPR_HYPERLINK_ON_CLICK: i32 = 6;
const CNVPR_HYPERLINK_ON_OVER: i32 = 7;

// Java: public class CNvPr implements KoAr, KoClosable
pub struct CNvPr {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Hyperlink hyperlinkOnClick
    pub(crate) hyperlinkOnClick: Option<Box<Hyperlink>>,
    // Java field: Hyperlink hyperlinkOnOver
    pub(crate) hyperlinkOnOver: Option<Box<Hyperlink>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: CNvPr parent
    pub(crate) parent: Option<Box<CNvPr>>,
}

// Java: public class CNvPr implements KoAr, KoClosable
impl CNvPr {
    // Java: static protected static <Value1 extends CNvPr> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = CNvPr::CNvPr();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static CNvPr create()
    pub fn create() -> Self {
        // Java: return initialize(CNvPr.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static CNvPr open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(CNvPr.class, bin);
        Self::initialize(bin)
    }

    // Java: CNvPr()
    pub fn CNvPr() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        CNvPr {
            zone: KoZone::defaultZone(),
            hyperlinkOnClick: None,
            hyperlinkOnOver: None,
            bin_box: KpBinBox::new(),
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: CNvPr(KoBin bin)
    pub fn CNvPr_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        CNvPr {
            zone: KoZone::defaultZone(),
            hyperlinkOnClick: None,
            hyperlinkOnOver: None,
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

    // Java: void load(CNvPr value)
    pub fn load_from(&mut self, value: Option<&CNvPr>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.hyperlinkOnClick != null && this.hyperlinkOnClick.dirty())
        if let Some(ref mut child) = self.hyperlinkOnClick {
            if child.dirty() {
                // Java: this.map.put(6, this.hyperlinkOnClick.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(CNVPR_HYPERLINK_ON_CLICK, bin);
            }
        }
        // Java: if (this.hyperlinkOnOver != null && this.hyperlinkOnOver.dirty())
        if let Some(ref mut child) = self.hyperlinkOnOver {
            if child.dirty() {
                // Java: this.map.put(7, this.hyperlinkOnOver.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(CNVPR_HYPERLINK_ON_OVER, bin);
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

    // Java: boolean equalsLax(CNvPr o)
    pub fn equalsLax(&self, o: &CNvPr) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CNvPr) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &CNvPr, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof CNvPr)
        if true {
            // Java: CNvPr dst = (CNvPr)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasId() || !dst.hasId()) && (!this.hasId() || dst.hasId()))
            else if (self.hasId() || !dst.hasId()) && (!self.hasId() || dst.hasId()) {
                // Java: if (isStrict && this.hasId() && dst.hasId() && this.id() != dst.id())
                if isStrict && self.hasId() && dst.hasId() && self.id() != dst.id() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
                else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
                    // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
                    if self.hasName() && dst.hasName() && self.name() != dst.name() {
                        // Java: return false;
                        return false;
                    }
                    // Java: else if ((this.hasDescr() || !dst.hasDescr()) && (!this.hasDescr() || dst.hasDescr()))
                    else if (self.hasDescr() || !dst.hasDescr()) && (!self.hasDescr() || dst.hasDescr()) {
                        // Java: if (this.hasDescr() && dst.hasDescr() && !this.descr().equals(dst.descr()))
                        if self.hasDescr() && dst.hasDescr() && self.descr() != dst.descr() {
                            // Java: return false;
                            return false;
                        }
                        // Java: else if ((this.hasHidden() || !dst.hasHidden()) && (!this.hasHidden() || dst.hasHidden()))
                        else if (self.hasHidden() || !dst.hasHidden()) && (!self.hasHidden() || dst.hasHidden()) {
                            // Java: if (this.hasHidden() && dst.hasHidden() && this.isHidden() != dst.isHidden())
                            if self.hasHidden() && dst.hasHidden() && self.isHidden() != dst.isHidden() {
                                // Java: return false;
                                return false;
                            }
                            // Java: else if ((this.hasTitle() || !dst.hasTitle()) && (!this.hasTitle() || dst.hasTitle()))
                            else if (self.hasTitle() || !dst.hasTitle()) && (!self.hasTitle() || dst.hasTitle()) {
                                // Java: if (this.hasTitle() && dst.hasTitle() && !this.title().equals(dst.title()))
                                if self.hasTitle() && dst.hasTitle() && self.title() != dst.title() {
                                    // Java: return false;
                                    return false;
                                }
                                // Java: else if ((this.hasHyperlinkOnClick() || !dst.hasHyperlinkOnClick()) && (!this.hasHyperlinkOnClick() || dst.hasHyperlinkOnClick()))
                                else if (self.hasHyperlinkOnClick() || !dst.hasHyperlinkOnClick()) && (!self.hasHyperlinkOnClick() || dst.hasHyperlinkOnClick()) {
                                    // Java: if (this.hasHyperlinkOnClick() && dst.hasHyperlinkOnClick())
                                    if self.hasHyperlinkOnClick() && dst.hasHyperlinkOnClick() {
                                        // Java: if (isStrict && !this.hyperlinkOnClick().equals(dst.hyperlinkOnClick()))
                                        if isStrict {
                                            // Java: return false;
                                        }
                                        // Java: if (!isStrict && !this.hyperlinkOnClick().equalsLax(dst.hyperlinkOnClick()))
                                        if !isStrict {
                                            // Java: return false;
                                        }
                                    }
                                    // Java: if ((this.hasHyperlinkOnOver() || !dst.hasHyperlinkOnOver()) && (!this.hasHyperlinkOnOver() || dst.hasHyperlinkOnOver()))
                                    if (self.hasHyperlinkOnOver() || !dst.hasHyperlinkOnOver()) && (!self.hasHyperlinkOnOver() || dst.hasHyperlinkOnOver()) {
                                        // Java: if (this.hasHyperlinkOnOver() && dst.hasHyperlinkOnOver())
                                        if self.hasHyperlinkOnOver() && dst.hasHyperlinkOnOver() {
                                            // Java: if (isStrict && !this.hyperlinkOnOver().equals(dst.hyperlinkOnOver()))
                                            if isStrict {
                                                // Java: return false;
                                            }
                                            // Java: if (!isStrict && !this.hyperlinkOnOver().equalsLax(dst.hyperlinkOnOver()))
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
        // Java: if (this.hasId())
        if self.hasId() {
            // Java: ret += Integer.valueOf(this.id()).hashCode();
        }
        // Java: if (this.hasName())
        if self.hasName() {
            // Java: ret += String.valueOf(this.name()).hashCode();
        }
        // Java: if (this.hasDescr())
        if self.hasDescr() {
            // Java: ret += String.valueOf(this.descr()).hashCode();
        }
        // Java: if (this.hasHidden())
        if self.hasHidden() {
            // Java: ret += Boolean.valueOf(this.isHidden()).hashCode();
        }
        // Java: if (this.hasTitle())
        if self.hasTitle() {
            // Java: ret += String.valueOf(this.title()).hashCode();
        }
        // Java: if (this.hasHyperlinkOnClick())
        if self.hasHyperlinkOnClick() {
            // Java: ret += this.hyperlinkOnClick().hashCode();
        }
        // Java: if (this.hasHyperlinkOnOver())
        if self.hasHyperlinkOnOver() {
            // Java: ret += this.hyperlinkOnOver().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.hyperlinkOnClick != null && this.hyperlinkOnClick.dirty())
        if self.hyperlinkOnClick.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.hyperlinkOnOver != null && this.hyperlinkOnOver.dirty() ? true : this.map.dirty();
            if self.hyperlinkOnOver.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.hyperlinkOnClick = null;
        self.hyperlinkOnClick = None;
        // Java: this.hyperlinkOnOver = null;
        self.hyperlinkOnOver = None;
    }

    // Java: void follow(CNvPr parent)
    pub fn follow(&mut self, parent: &mut CNvPr) {
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

    // Java: CNvPr parent()
    pub fn parent_ref(&self) -> Option<&CNvPr> {
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

    // Java: int id()
    pub fn id(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(CNVPR_ID) {
            // Java: return this.parent == null ? -1 : this.parent.id();
            self.parent.as_ref().map_or(-1, |p| p.id())
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(CNVPR_ID);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasId()
    pub fn hasId(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasId();
        self.map.has(CNVPR_ID) || self.parent.as_ref().map_or(false, |p| p.hasId())
    }

    // Java: void deleteId()
    pub fn deleteId(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(CNVPR_ID);
    }

    // Java: void putId(int value)
    pub fn putId(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(CNVPR_ID, bin);
    }

    // Java: String name()
    pub fn name(&self) -> String {
        // Java: if (!this.map.has(2))
        if !self.map.has(CNVPR_NAME) {
            // Java: return this.parent == null ? "" : this.parent.name();
            self.parent.as_ref().map_or(String::new(), |p| p.name())
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(CNVPR_NAME);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasName();
        self.map.has(CNVPR_NAME) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(CNVPR_NAME);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(CNVPR_NAME, bin);
    }

    // Java: String descr()
    pub fn descr(&self) -> String {
        // Java: if (!this.map.has(3))
        if !self.map.has(CNVPR_DESCR) {
            // Java: return this.parent == null ? "" : this.parent.descr();
            self.parent.as_ref().map_or(String::new(), |p| p.descr())
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(CNVPR_DESCR);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasDescr()
    pub fn hasDescr(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasDescr();
        self.map.has(CNVPR_DESCR) || self.parent.as_ref().map_or(false, |p| p.hasDescr())
    }

    // Java: void deleteDescr()
    pub fn deleteDescr(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(CNVPR_DESCR);
    }

    // Java: void putDescr(String value)
    pub fn putDescr(&mut self, value: &str) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(CNVPR_DESCR, bin);
    }

    // Java: boolean isHidden()
    pub fn isHidden(&self) -> bool {
        // Java: if (!this.map.has(4))
        if !self.map.has(CNVPR_HIDDEN) {
            // Java: return this.parent == null ? false : this.parent.isHidden();
            self.parent.as_ref().map_or(false, |p| p.isHidden())
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(CNVPR_HIDDEN);
            // Java: return KoBin.Decode.toBool(bin);
            Decode::toBool(bin.as_ref())
        }
    }

    // Java: boolean hasHidden()
    pub fn hasHidden(&self) -> bool {
        // Java: return this.map.has(4) || this.parent != null && this.parent.hasHidden();
        self.map.has(CNVPR_HIDDEN) || self.parent.as_ref().map_or(false, |p| p.hasHidden())
    }

    // Java: void deleteHidden()
    pub fn deleteHidden(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(CNVPR_HIDDEN);
    }

    // Java: void putHidden(boolean value)
    pub fn putHidden(&mut self, value: bool) {
        // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_bool(value, self.zone);
        self.map.put(CNVPR_HIDDEN, bin);
    }

    // Java: String title()
    pub fn title(&self) -> String {
        // Java: if (!this.map.has(5))
        if !self.map.has(CNVPR_TITLE) {
            // Java: return this.parent == null ? "" : this.parent.title();
            self.parent.as_ref().map_or(String::new(), |p| p.title())
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(CNVPR_TITLE);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasTitle()
    pub fn hasTitle(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasTitle();
        self.map.has(CNVPR_TITLE) || self.parent.as_ref().map_or(false, |p| p.hasTitle())
    }

    // Java: void deleteTitle()
    pub fn deleteTitle(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(CNVPR_TITLE);
    }

    // Java: void putTitle(String value)
    pub fn putTitle(&mut self, value: &str) {
        // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(CNVPR_TITLE, bin);
    }

    // Java: Hyperlink hyperlinkOnClick()
    pub fn hyperlinkOnClick(&self) -> &Hyperlink {
        // Java: if (this.hyperlinkOnClick == null)
        if self.hyperlinkOnClick.is_none() {
            // Java: if (this.map.has(6))
            if self.map.has(CNVPR_HYPERLINK_ON_CLICK) {
                // Java: KoBin bin = this.map.byKey(6);
                let _bin = self.map.byKey(CNVPR_HYPERLINK_ON_CLICK);
                // Java: this.hyperlinkOnClick = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasHyperlinkOnClick())
            else if self.parent.as_ref().map_or(false, |p| p.hasHyperlinkOnClick()) {
                // Java: this.hyperlinkOnClick = this.parent.hyperlinkOnClick();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.hyperlinkOnClick = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
            }
        }
        // Java: return this.hyperlinkOnClick;
        self.hyperlinkOnClick.as_ref().unwrap()
    }

    // Java: boolean hasHyperlinkOnClick()
    pub fn hasHyperlinkOnClick(&self) -> bool {
        // Java: return this.map.has(6) || this.hyperlinkOnClick != null && this.hyperlinkOnClick.dirty() || this.parent != null && this.parent.hasHyperlinkOnClick();
        self.map.has(CNVPR_HYPERLINK_ON_CLICK) || self.hyperlinkOnClick.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHyperlinkOnClick())
    }

    // Java: void deleteHyperlinkOnClick()
    pub fn deleteHyperlinkOnClick(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(CNVPR_HYPERLINK_ON_CLICK);
        // Java: if (this.hyperlinkOnClick != null)
        if self.hyperlinkOnClick.is_some() {
            // Java: this.hyperlinkOnClick = null;
            self.hyperlinkOnClick = None;
        }
    }

    // Java: void writeHyperlinkOnClick(Hyperlink value)
    pub fn writeHyperlinkOnClick(&mut self, value: &Hyperlink) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(CNVPR_HYPERLINK_ON_CLICK, bin);
                // Java: if (this.hyperlinkOnClick != null)
                if self.hyperlinkOnClick.is_some() {
                    // Java: this.hyperlinkOnClick = null;
                    self.hyperlinkOnClick = None;
                }
            }
        }
    }

    // Java: void forceWriteHyperlinkOnClick(Hyperlink value)
    pub fn forceWriteHyperlinkOnClick(&mut self, value: &Hyperlink) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(6, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(CNVPR_HYPERLINK_ON_CLICK, bin);
            // Java: if (this.hyperlinkOnClick != null)
            if self.hyperlinkOnClick.is_some() {
                // Java: this.hyperlinkOnClick = null;
                self.hyperlinkOnClick = None;
            }
        }
    }

    // Java: Hyperlink hyperlinkOnOver()
    pub fn hyperlinkOnOver(&self) -> &Hyperlink {
        // Java: if (this.hyperlinkOnOver == null)
        if self.hyperlinkOnOver.is_none() {
            // Java: if (this.map.has(7))
            if self.map.has(CNVPR_HYPERLINK_ON_OVER) {
                // Java: KoBin bin = this.map.byKey(7);
                let _bin = self.map.byKey(CNVPR_HYPERLINK_ON_OVER);
                // Java: this.hyperlinkOnOver = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasHyperlinkOnOver())
            else if self.parent.as_ref().map_or(false, |p| p.hasHyperlinkOnOver()) {
                // Java: this.hyperlinkOnOver = this.parent.hyperlinkOnOver();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.hyperlinkOnOver = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
            }
        }
        // Java: return this.hyperlinkOnOver;
        self.hyperlinkOnOver.as_ref().unwrap()
    }

    // Java: boolean hasHyperlinkOnOver()
    pub fn hasHyperlinkOnOver(&self) -> bool {
        // Java: return this.map.has(7) || this.hyperlinkOnOver != null && this.hyperlinkOnOver.dirty() || this.parent != null && this.parent.hasHyperlinkOnOver();
        self.map.has(CNVPR_HYPERLINK_ON_OVER) || self.hyperlinkOnOver.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHyperlinkOnOver())
    }

    // Java: void deleteHyperlinkOnOver()
    pub fn deleteHyperlinkOnOver(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(CNVPR_HYPERLINK_ON_OVER);
        // Java: if (this.hyperlinkOnOver != null)
        if self.hyperlinkOnOver.is_some() {
            // Java: this.hyperlinkOnOver = null;
            self.hyperlinkOnOver = None;
        }
    }

    // Java: void writeHyperlinkOnOver(Hyperlink value)
    pub fn writeHyperlinkOnOver(&mut self, value: &Hyperlink) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(7, bin);
                self.map.put(CNVPR_HYPERLINK_ON_OVER, bin);
                // Java: if (this.hyperlinkOnOver != null)
                if self.hyperlinkOnOver.is_some() {
                    // Java: this.hyperlinkOnOver = null;
                    self.hyperlinkOnOver = None;
                }
            }
        }
    }

    // Java: void forceWriteHyperlinkOnOver(Hyperlink value)
    pub fn forceWriteHyperlinkOnOver(&mut self, value: &Hyperlink) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(7, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(CNVPR_HYPERLINK_ON_OVER, bin);
            // Java: if (this.hyperlinkOnOver != null)
            if self.hyperlinkOnOver.is_some() {
                // Java: this.hyperlinkOnOver = null;
                self.hyperlinkOnOver = None;
            }
        }
    }
}

impl KoAr for CNvPr {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { CNvPr::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { CNvPr::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { CNvPr::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CNvPr::lastBin(self, var1) }
}

impl KoClosable for CNvPr {
    fn close(&mut self) { CNvPr::close(self); }
}
