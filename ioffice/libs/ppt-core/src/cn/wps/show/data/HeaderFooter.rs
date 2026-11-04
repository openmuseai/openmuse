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
// Java: import cn.wps.kso.kproto.internal.KpBool
use crate::cn::wps::kso::kproto::internal::KpBool::KpBool;

// Java: public class HeaderFooter implements KoAr, KoClosable
pub struct HeaderFooter {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpBool map
    pub(crate) map: KpBool,
    // Java field: HeaderFooter parent
    pub(crate) parent: Option<Box<HeaderFooter>>,
}

// Java: public class HeaderFooter implements KoAr, KoClosable
impl HeaderFooter {
    // Java: static protected static <Value1 extends HeaderFooter> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = HeaderFooter::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static HeaderFooter create()
    pub fn create() -> Self {
      // Java: return initialize(HeaderFooter.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static HeaderFooter open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(HeaderFooter.class, bin);
      Self::initialize(bin)
    }

    // Java: HeaderFooter()
    pub fn new() -> Self {
      Self {
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpBool();
        map: KpBool::new(),
        parent: None,
      }
    }

    // Java: HeaderFooter(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        zone,
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpBool(bin);
        map: KpBool::KpBool_from(bin),
        parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(HeaderFooter value)
    pub fn load_value(&mut self, value: Option<&HeaderFooter>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(HeaderFooter o)
    pub fn equalsLax(&self, o: &HeaderFooter) -> bool {
      // Java: return this.equals(o);
      self.equals(o)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &HeaderFooter) -> bool {
      // Java: if (null != o && o instanceof HeaderFooter)
      {
         // Java: HeaderFooter dst = (HeaderFooter)o;
         let dst = o;
         // Java: return this == dst ? true : this.map.equals(dst.map);
         if std::ptr::eq(self as *const _, dst as *const _) { return true; }
         return self.map.equals(&dst.map);
      }
      // Java: else
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.map.hashCode();
      self.map.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.map.dirty();
      self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.cleanParent();
      self.cleanParent();
      // Java: this.map.load(KoBin.Null.value1());
      let null_bin = Null::value1(); self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void follow(HeaderFooter parent)
    pub fn follow(&mut self, parent: Option<Box<HeaderFooter>>) {
      // Java: if (parent != null)
      if let Some(mut parent) = parent
      {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(parent);
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

    // Java: HeaderFooter parent()
    pub fn parent(&self) -> Option<&HeaderFooter> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone; self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: boolean isSlideNum()
    pub fn isSlideNum(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(1)
      {
         // Java: return this.parent == null ? true : this.parent.isSlideNum();
         return self.parent.as_ref().map_or(true, |p| p.isSlideNum());
      }
      // Java: else
      else
      {
         // Java: return this.map.get(1);
         return self.map.get(1);
      }
    }

    // Java: boolean hasSlideNum()
    pub fn hasSlideNum(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasSlideNum();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasSlideNum())
    }

    // Java: void deleteSlideNum()
    pub fn deleteSlideNum(&mut self) {
      // Java: this.map.clear(1);
      self.map.clear(1);
    }

    // Java: void putSlideNum(boolean value)
    pub fn putSlideNum(&mut self, value: bool) {
      // Java: this.map.set(1, value);
      self.map.set(1, value);
    }

    // Java: boolean isDateTime()
    pub fn isDateTime(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(2)
      {
         // Java: return this.parent == null ? true : this.parent.isDateTime();
         return self.parent.as_ref().map_or(true, |p| p.isDateTime());
      }
      // Java: else
      else
      {
         // Java: return this.map.get(2);
         return self.map.get(2);
      }
    }

    // Java: boolean hasDateTime()
    pub fn hasDateTime(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasDateTime();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasDateTime())
    }

    // Java: void deleteDateTime()
    pub fn deleteDateTime(&mut self) {
      // Java: this.map.clear(2);
      self.map.clear(2);
    }

    // Java: void putDateTime(boolean value)
    pub fn putDateTime(&mut self, value: bool) {
      // Java: this.map.set(2, value);
      self.map.set(2, value);
    }

    // Java: boolean isFooter()
    pub fn isFooter(&self) -> bool {
      // Java: if (!this.map.has(3))
      if !self.map.has(3)
      {
         // Java: return this.parent == null ? true : this.parent.isFooter();
         return self.parent.as_ref().map_or(true, |p| p.isFooter());
      }
      // Java: else
      else
      {
         // Java: return this.map.get(3);
         return self.map.get(3);
      }
    }

    // Java: boolean hasFooter()
    pub fn hasFooter(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasFooter();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasFooter())
    }

    // Java: void deleteFooter()
    pub fn deleteFooter(&mut self) {
      // Java: this.map.clear(3);
      self.map.clear(3);
    }

    // Java: void putFooter(boolean value)
    pub fn putFooter(&mut self, value: bool) {
      // Java: this.map.set(3, value);
      self.map.set(3, value);
    }

    // Java: boolean isHeader()
    pub fn isHeader(&self) -> bool {
      // Java: if (!this.map.has(4))
      if !self.map.has(4)
      {
         // Java: return this.parent == null ? true : this.parent.isHeader();
         return self.parent.as_ref().map_or(true, |p| p.isHeader());
      }
      // Java: else
      else
      {
         // Java: return this.map.get(4);
         return self.map.get(4);
      }
    }

    // Java: boolean hasHeader()
    pub fn hasHeader(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasHeader();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasHeader())
    }

    // Java: void deleteHeader()
    pub fn deleteHeader(&mut self) {
      // Java: this.map.clear(4);
      self.map.clear(4);
    }

    // Java: void putHeader(boolean value)
    pub fn putHeader(&mut self, value: bool) {
      // Java: this.map.set(4, value);
      self.map.set(4, value);
    }

}

impl KoAr for HeaderFooter {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { HeaderFooter::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { HeaderFooter::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { HeaderFooter::lastBin(self, var1) }
}

impl KoClosable for HeaderFooter {
    fn close(&mut self) { HeaderFooter::close(self); }
}
