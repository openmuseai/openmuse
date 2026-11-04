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

// Java: public class Tag implements KoAr, KoClosable
pub struct Tag {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: Tag parent
    pub(crate) parent: Option<Box<Tag>>,
}

// Java: private static final int $name = 1;
const TAG_NAME: i32 = 1;
// Java: private static final int $val = 2;
const TAG_VAL: i32 = 2;

// Java: public class Tag implements KoAr, KoClosable
impl Tag {
    // Java: static protected static <Value1 extends Tag> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Tag::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Tag create()
    pub fn create() -> Self {
      // Java: return initialize(Tag.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Tag open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Tag.class, bin);
      Self::initialize(bin)
    }

    // Java: Tag()
    pub fn new() -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        // Java field initializer: Tag parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: Tag(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait) -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        // Java field initializer: Tag parent（Java 默认 null）
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(Tag value)
    pub fn load_value(&mut self, value: Option<&Tag>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
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

    // Java: boolean equalsLax(Tag o)
    pub fn equalsLax(&mut self, o: &mut Tag) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut Tag) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级真实端口 `comment/Comment.rs::equals_inner` 的既有约定命名。
    fn equals_inner(&mut self, o: &mut Tag, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Tag)
      if true
      {
         // Java: Tag dst = (Tag)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
         else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName())
         {
            // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
            if self.hasName() && dst.hasName() && self.name() != dst.name()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasVal() || !dst.hasVal()) && (!this.hasVal() || dst.hasVal()))
            else if (self.hasVal() || !dst.hasVal()) && (!self.hasVal() || dst.hasVal())
            {
               // Java: return !this.hasVal() || !dst.hasVal() || this.val().equals(dst.val());
               return !self.hasVal() || !dst.hasVal() || self.val() == dst.val();
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasName())
      if self.hasName()
      {
         // Java: ret += String.valueOf(this.name()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.name()));
      }
      // Java: if (this.hasVal())
      if self.hasVal()
      {
         // Java: ret += String.valueOf(this.val()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.val()));
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
      Tag::saveBin(self, zone);
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
      // Java: （空实现）
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: （空实现）
    }

    // Java: void follow(Tag parent)
    pub fn follow(&mut self, parent: Option<Box<Tag>>) {
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

    // Java: Tag parent()
    pub fn get_parent(&self) -> Option<&Tag> {
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

    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(1))
      if !self.map.has(TAG_NAME)
      {
         // Java: return this.parent == null ? "" : this.parent.name();
         return self.parent.as_ref().map_or(String::new(), |p| p.name());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(TAG_NAME);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
      self.map.has(TAG_NAME) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(TAG_NAME);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(TAG_NAME, Encode::from_string(value, self.zone));
    }

    // Java: String val()
    pub fn val(&self) -> String {
      // Java: if (!this.map.has(2))
      if !self.map.has(TAG_VAL)
      {
         // Java: return this.parent == null ? "" : this.parent.val();
         return self.parent.as_ref().map_or(String::new(), |p| p.val());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(TAG_VAL);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasVal()
    pub fn hasVal(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasVal();
      self.map.has(TAG_VAL) || self.parent.as_ref().map_or(false, |p| p.hasVal())
    }

    // Java: void deleteVal()
    pub fn deleteVal(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(TAG_VAL);
    }

    // Java: void putVal(String value)
    pub fn putVal(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(TAG_VAL, Encode::from_string(value, self.zone));
    }

}

impl KoAr for Tag {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Tag::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`（写 `this.box`/`this.map`），
    // 而 `KoAr` 接口在 Rust 侧声明为 `&self`。沿用同级真实端口
    // `comment/Comment.rs`、`background/Background.rs` 的既有约定，在此处显式声明该限制。
    // owner: show.data.custdatalst    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Tag::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Tag::lastBin(self, var1) }
}

impl KoClosable for Tag {
    fn close(&mut self) { Tag::close(self); }
}

// Java: int hashCode() —— `String.valueOf(x).hashCode()` 的 Java 语义（UTF-16 code unit 31 进制）
fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
