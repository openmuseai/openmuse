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

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for ch in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

fn java_bool_hash(b: bool) -> i32 {
    if b { 1231 } else { 1237 }
}

// Java: public class ShapeExt implements KoAr, KoClosable
pub struct ShapeExt {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ShapeExt parent
    pub(crate) parent: Option<Box<ShapeExt>>,
}

// Java: public class ShapeExt implements KoAr, KoClosable
impl ShapeExt {
    // Java: static protected static <Value1 extends ShapeExt> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ShapeExt_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ShapeExt create()
    pub fn create() -> Self {
      // Java: return initialize(ShapeExt.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ShapeExt open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ShapeExt.class, bin);
      Self::initialize(bin)
    }

    // Java: ShapeExt()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), parent: None,
      }
    }

    // Java: ShapeExt(KoBin bin)
    pub fn ShapeExt_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(ShapeExt value)
    pub fn load_value(&mut self, value: Option<&ShapeExt>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin_with_zone(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(ShapeExt o)
    pub fn equalsLax(&self, o: &ShapeExt) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ShapeExt) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ShapeExt, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ShapeExt)
        if true
      {
         // Java: ShapeExt dst = (ShapeExt)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasUri() || !dst.hasUri()) && (!this.hasUri() || dst.hasUri()))
            else if (self.hasUri() || !dst.hasUri()) && (!self.hasUri() || dst.hasUri())
         {
            // Java: if (this.hasUri() && dst.hasUri() && !this.uri().equals(dst.uri()))
                if self.hasUri() && dst.hasUri() && self.uri() != dst.uri()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
            {
               // Java: if (this.hasType() && dst.hasType() && !this.type().equals(dst.type()))
                    if self.hasType() && dst.hasType() && self.get_type() != dst.get_type()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasIndex() || !dst.hasIndex()) && (!this.hasIndex() || dst.hasIndex()))
                    else if (self.hasIndex() || !dst.hasIndex()) && (!self.hasIndex() || dst.hasIndex())
               {
                  // Java: if (this.hasIndex() && dst.hasIndex() && !this.index().equals(dst.index()))
                        if self.hasIndex() && dst.hasIndex() && self.index() != dst.index()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasIsDefault() || !dst.hasIsDefault()) && (!this.hasIsDefault() || dst.hasIsDefault()))
                        else if (self.hasIsDefault() || !dst.hasIsDefault()) && (!self.hasIsDefault() || dst.hasIsDefault())
                  {
                     // Java: return !this.hasIsDefault() || !dst.hasIsDefault() || this.isIsDefault() == dst.isIsDefault();
                     return !self.hasIsDefault() || !dst.hasIsDefault() || self.isIsDefault() == dst.isIsDefault();
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
      // Java: if (this.hasUri())
        if self.hasUri()
      {
         // Java: ret += String.valueOf(this.uri()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.uri()));
      }
      // Java: if (this.hasType())
        if self.hasType()
      {
         // Java: ret += String.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.get_type()));
      }
      // Java: if (this.hasIndex())
        if self.hasIndex()
      {
         // Java: ret += String.valueOf(this.index()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.index()));
      }
      // Java: if (this.hasIsDefault())
        if self.hasIsDefault()
      {
         // Java: ret += Boolean.valueOf(this.isIsDefault()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isIsDefault()));
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
      self.loadBin_with_zone(bin.as_ref(), zone);
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
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
    }

    // Java: void follow(ShapeExt parent)
    pub fn follow(&mut self, parent: Option<Box<ShapeExt>>) {
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

    // Java: ShapeExt parent()
    pub fn get_parent(&self) -> Option<&ShapeExt> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: String uri()
    pub fn uri(&self) -> String {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? "" : this.parent.uri();
         return self.parent.as_ref().map_or(String::new(), |p| p.uri());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasUri()
    pub fn hasUri(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasUri();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasUri())
    }

    // Java: void deleteUri()
    pub fn deleteUri(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putUri(String value)
    pub fn putUri(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_string(value, self.zone));
    }

    // Java: String type()
    pub fn get_type(&self) -> String {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? "" : this.parent.type();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_type());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasType();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putType(String value)
    pub fn putType(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_string(value, self.zone));
    }

    // Java: String index()
    pub fn index(&self) -> String {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? "" : this.parent.index();
         return self.parent.as_ref().map_or(String::new(), |p| p.index());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasIndex()
    pub fn hasIndex(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasIndex();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasIndex())
    }

    // Java: void deleteIndex()
    pub fn deleteIndex(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putIndex(String value)
    pub fn putIndex(&mut self, value: &str) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_string(value, self.zone));
    }

    // Java: boolean isIsDefault()
    pub fn isIsDefault(&self) -> bool {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? false : this.parent.isIsDefault();
         return self.parent.as_ref().map_or(false, |p| p.isIsDefault());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasIsDefault()
    pub fn hasIsDefault(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasIsDefault();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasIsDefault())
    }

    // Java: void deleteIsDefault()
    pub fn deleteIsDefault(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putIsDefault(boolean value)
    pub fn putIsDefault(&mut self, value: bool) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_bool(value, self.zone));
    }

}

impl KoAr for ShapeExt {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ShapeExt::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ShapeExt::lastBin(self, var1) }
}

impl KoClosable for ShapeExt {
    fn close(&mut self) { ShapeExt::close(self); }
}
