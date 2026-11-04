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

// Java: public class IndexRange implements KoAr, KoClosable
pub struct IndexRange {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: IndexRange parent
    pub(crate) parent: Option<Box<IndexRange>>,
}

const START: i32 = 1;
const END: i32 = 2;

// Java: public class IndexRange implements KoAr, KoClosable
impl IndexRange {
    // Java: static protected static <Value1 extends IndexRange> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = IndexRange::IndexRange();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static IndexRange create()
    pub fn create() -> Self {
        // Java: return initialize(IndexRange.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static IndexRange open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(IndexRange.class, bin);
        Self::initialize(bin)
    }

    // Java: IndexRange()
    pub fn IndexRange() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        IndexRange {
            zone: KoZone,
            bin_box: KpBinBox::new(),
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: IndexRange(KoBin bin)
    pub fn IndexRange_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        IndexRange {
            zone: KoZone,
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
        let bin_ref = self.bin_box.get();
        self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(IndexRange value)
    pub fn load_from(&mut self, value: Option<&IndexRange>) {
        // Java: if (value != null)
        if let Some(val) = value
        {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last_bin = val.lastBin(&KoZone);
            self.loadBin(last_bin.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, _zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(&KoZone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(IndexRange o)
    pub fn equalsLax(&self, o: &IndexRange) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &IndexRange) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &IndexRange, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof IndexRange)
        if true
        {
            // Java: IndexRange dst = (IndexRange)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasStart() || !dst.hasStart()) && (!this.hasStart() || dst.hasStart()))
            else if (self.hasStart() || !dst.hasStart()) && (!self.hasStart() || dst.hasStart())
            {
                // Java: if (this.hasStart() && dst.hasStart() && this.start() != dst.start())
                if self.hasStart() && dst.hasStart() && self.start() != dst.start()
                {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasEnd() || !dst.hasEnd()) && (!this.hasEnd() || dst.hasEnd()))
                else if (self.hasEnd() || !dst.hasEnd()) && (!self.hasEnd() || dst.hasEnd())
                {
                    // Java: return !this.hasEnd() || !dst.hasEnd() || this.end() == dst.end();
                    return !self.hasEnd() || !dst.hasEnd() || self.end() == dst.end();
                }
                // Java: else
                else
                {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else
            {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else
        {
            // Java: return false;
            return false;
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasStart())
        if self.hasStart()
        {
            // Java: ret += Integer.valueOf(this.start()).hashCode();
            ret += self.start();
        }
        // Java: if (this.hasEnd())
        if self.hasEnd()
        {
            // Java: ret += Integer.valueOf(this.end()).hashCode();
            ret += self.end();
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
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        let bin = self.bin_box.get();
        Factory::dup(bin.as_ref(), zone)
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
        let null_bin = Null::value1();
        self.map.load(null_bin.as_ref());
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

    // Java: void follow(IndexRange parent)
    pub fn follow(&mut self, parent: Option<Box<IndexRange>>) {
        // Java: if (parent != null)
        if let Some(mut p) = parent
        {
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
        if self.parent.is_some()
        {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut p) = self.parent {
                p.discard();
            }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: IndexRange parent()
    pub fn parent_ref(&self) -> Option<&IndexRange> {
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

    // Java: int start()
    pub fn start(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(START)
        {
            // Java: return this.parent == null ? 0 : this.parent.start();
            match &self.parent {
                None => 0,
                Some(p) => p.start(),
            }
        }
        // Java: else
        else
        {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(START);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasStart()
    pub fn hasStart(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasStart();
        self.map.has(START) || self.parent.as_ref().map_or(false, |p| p.hasStart())
    }

    // Java: void deleteStart()
    pub fn deleteStart(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(START);
    }

    // Java: void putStart(int value)
    pub fn putStart(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &KoZone);
        self.map.put(START, bin);
    }

    // Java: int end()
    pub fn end(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(END)
        {
            // Java: return this.parent == null ? 0 : this.parent.end();
            match &self.parent {
                None => 0,
                Some(p) => p.end(),
            }
        }
        // Java: else
        else
        {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(END);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasEnd()
    pub fn hasEnd(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasEnd();
        self.map.has(END) || self.parent.as_ref().map_or(false, |p| p.hasEnd())
    }

    // Java: void deleteEnd()
    pub fn deleteEnd(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(END);
    }

    // Java: void putEnd(int value)
    pub fn putEnd(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &KoZone);
        self.map.put(END, bin);
    }
}

impl KoAr for IndexRange {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) {
        IndexRange::loadBin(self, var1, var2);
    }

    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> {
        self.bin_box.get()
    }

    fn dirty(&self) -> bool {
        self.map.dirty()
    }

    fn discard(&mut self, var1: &KoZone) {
        self.discard_with_zone(var1);
    }

    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> {
        IndexRange::lastBin(self, var1)
    }
}

impl KoClosable for IndexRange {
    fn close(&mut self) {
        IndexRange::close(self);
    }
}
