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
// Java: import cn.wps.kso.kproto.util.KoTextList
use crate::cn::wps::kso::kproto::util::KoTextList::KoTextList;

// Java: public class Headers implements KoAr, KoClosable
pub struct Headers {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KoTextList headerList
    pub(crate) headerList: Option<Box<KoTextList>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Headers parent
    pub(crate) parent: Option<Box<Headers>>,
}

// Java: public class Headers implements KoAr, KoClosable
impl Headers {
    // Java: static protected static <Value1 extends Headers> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      // Java: inst.zone = zone;
      // Java: inst.load(bin);
      let mut inst = Self::new_from(bin);
      inst.zone = zone;
      // Java: return inst;
      inst
    }

    // Java: static Headers create()
    pub fn create() -> Self {
      // Java: return initialize(Headers.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Headers open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Headers.class, bin);
      Self::initialize(bin)
    }

    // Java: Headers()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        headerList: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: Headers(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        headerList: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(Headers value)
    pub fn load_from(&mut self, value: Option<&Headers>) {
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
      // Java: if (this.headerList != null && this.headerList.dirty())
        if self.headerList.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.headerList.saveBin(zone));
         let bin = self.headerList.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(Headers o)
    pub fn equalsLax(&self, o: &Headers) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Headers) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Headers, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Headers)
      {
         // Java: Headers dst = (Headers)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasHeaderList() || !dst.hasHeaderList()) && (!this.hasHeaderList() || dst.hasHeaderList()))
            else if (self.hasHeaderList() || !dst.hasHeaderList()) && (!self.hasHeaderList() || dst.hasHeaderList())
         {
            // Java: if (this.hasHeaderList() && dst.hasHeaderList())
                if self.hasHeaderList() && dst.hasHeaderList()
            {
               // Java: if (isStrict && !this.headerList().equals(dst.headerList()))
                    if isStrict
               {
                  // Java: return false;
                  todo!("headerList equals");
               }
               // Java: if (!isStrict && !this.headerList().equalsLax(dst.headerList()))
                    if !isStrict
               {
                  // Java: return false;
                  todo!("headerList equalsLax");
               }
            }
            // Java: return true;
            return true;
         }
         // Java: else
         else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasHeaderList())
        if self.hasHeaderList()
      {
         // Java: ret += this.headerList().hashCode();
         self.headerList();
         ret = ret.wrapping_add(self.headerList.as_ref().unwrap().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.headerList != null && this.headerList.dirty() ? true : this.map.dirty();
      self.headerList.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.headerList = null;
      self.headerList = None;
    }

    // Java: void follow(Headers parent)
    pub fn follow(&mut self, parent: Option<Box<Headers>>) {
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

    // Java: Headers parent()
    pub fn parent(&self) -> &Headers {
      // Java: return this.parent;
      self.parent.as_deref().unwrap()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: KoTextList headerList()
    pub fn headerList(&mut self) -> &KoTextList {
      // Java: if (this.headerList == null)
        if self.headerList.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.headerList = (KoTextList)KoBin.Decode.toAny(KoTextList.class, bin, this.zone);
            self.headerList = Some(Box::new(KoTextList::new_from(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasHeaderList())
            else if self.parent.as_ref().map_or(false, |p| p.hasHeaderList())
         {
            // Java: this.headerList = this.parent.headerList();
            todo!("headerList follow parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.headerList = (KoTextList)KoBin.Decode.toAny(KoTextList.class, bin, this.zone);
            self.headerList = Some(Box::new(KoTextList::new_from(null_bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.headerList;
      self.headerList.as_deref().unwrap()
    }

    // Java: boolean hasHeaderList()
    pub fn hasHeaderList(&self) -> bool {
      // Java: return this.map.has(1) || this.headerList != null && this.headerList.dirty() || this.parent != null && this.parent.hasHeaderList();
      self.map.has(1)
        || self.headerList.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasHeaderList())
    }

    // Java: void deleteHeaderList()
    pub fn deleteHeaderList(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.headerList != null)
        if self.headerList.is_some()
      {
         // Java: this.headerList = null;
         self.headerList = None;
      }
    }

    // Java: void writeHeaderList(KoTextList value)
    pub fn writeHeaderList(&mut self, value: Option<&KoTextList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.headerList != null)
                if self.headerList.is_some()
            {
               // Java: this.headerList = null;
               self.headerList = None;
            }
         }
      }
    }

    // Java: void forceWriteHeaderList(KoTextList value)
    pub fn forceWriteHeaderList(&mut self, value: Option<&KoTextList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.headerList != null)
            if self.headerList.is_some()
         {
            // Java: this.headerList = null;
            self.headerList = None;
         }
      }
    }

}

impl KoAr for Headers {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for Headers { fn close(&mut self) { self.close(); } }
