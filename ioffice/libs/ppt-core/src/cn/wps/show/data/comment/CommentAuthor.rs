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

// Java: public class CommentAuthor implements KoAr, KoClosable
pub struct CommentAuthor {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: CommentAuthor parent
    pub(crate) parent: Option<Box<CommentAuthor>>,
}

// Java: private static final int $colorIndex = 1;
const COMMENT_AUTHOR_COLOR_INDEX: i32 = 1;
// Java: private static final int $id = 2;
const COMMENT_AUTHOR_ID: i32 = 2;
// Java: private static final int $initials = 3;
const COMMENT_AUTHOR_INITIALS: i32 = 3;
// Java: private static final int $lastCommentIndex = 4;
const COMMENT_AUTHOR_LAST_COMMENT_INDEX: i32 = 4;
// Java: private static final int $name = 5;
const COMMENT_AUTHOR_NAME: i32 = 5;

// Java: public class CommentAuthor implements KoAr, KoClosable
impl CommentAuthor {
    // Java: static protected static <Value1 extends CommentAuthor> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::CommentAuthor_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static CommentAuthor create()
    pub fn create() -> Self {
      // Java: return initialize(CommentAuthor.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static CommentAuthor open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CommentAuthor.class, bin);
      Self::initialize(bin)
    }

    // Java: CommentAuthor()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), parent: None,
      }
    }

    // Java: CommentAuthor(KoBin bin)
    pub fn CommentAuthor_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), parent: None,
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

    // Java: void load(CommentAuthor value)
    pub fn load_value(&mut self, value: Option<&CommentAuthor>) {
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
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(CommentAuthor o)
    pub fn equalsLax(&self, o: &CommentAuthor) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CommentAuthor) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级 `Hyperlink.rs` 的既有约定，私有重载取名 `equals_inner`。
    fn equals_inner(&self, o: &CommentAuthor, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof CommentAuthor)
        if true
      {
         // Java: CommentAuthor dst = (CommentAuthor)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasColorIndex() || !dst.hasColorIndex()) && (!this.hasColorIndex() || dst.hasColorIndex()))
            else if (self.hasColorIndex() || !dst.hasColorIndex()) && (!self.hasColorIndex() || dst.hasColorIndex())
         {
            // Java: if (this.hasColorIndex() && dst.hasColorIndex() && this.colorIndex() != dst.colorIndex())
                if self.hasColorIndex() && dst.hasColorIndex() && self.colorIndex() != dst.colorIndex()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasId() || !dst.hasId()) && (!this.hasId() || dst.hasId()))
                else if (self.hasId() || !dst.hasId()) && (!self.hasId() || dst.hasId())
            {
               // Java: if (this.hasId() && dst.hasId() && this.id() != dst.id())
                   if self.hasId() && dst.hasId() && self.id() != dst.id()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasInitials() || !dst.hasInitials()) && (!this.hasInitials() || dst.hasInitials()))
                   else if (self.hasInitials() || !dst.hasInitials()) && (!self.hasInitials() || dst.hasInitials())
               {
                  // Java: if (this.hasInitials() && dst.hasInitials() && !this.initials().equals(dst.initials()))
                      if self.hasInitials() && dst.hasInitials() && self.initials() != dst.initials()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasLastCommentIndex() || !dst.hasLastCommentIndex()) && (!this.hasLastCommentIndex() || dst.hasLastCommentIndex()))
                      else if (self.hasLastCommentIndex() || !dst.hasLastCommentIndex()) && (!self.hasLastCommentIndex() || dst.hasLastCommentIndex())
                  {
                     // Java: if (this.hasLastCommentIndex() && dst.hasLastCommentIndex() && this.lastCommentIndex() != dst.lastCommentIndex())
                         if self.hasLastCommentIndex() && dst.hasLastCommentIndex() && self.lastCommentIndex() != dst.lastCommentIndex()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
                         else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName())
                     {
                        // Java: return !this.hasName() || !dst.hasName() || this.name().equals(dst.name());
                        return !self.hasName() || !dst.hasName() || self.name() == dst.name();
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
      // Java: if (this.hasColorIndex())
        if self.hasColorIndex()
      {
         // Java: ret += Integer.valueOf(this.colorIndex()).hashCode();
         ret = ret.wrapping_add(self.colorIndex());
      }
      // Java: if (this.hasId())
        if self.hasId()
      {
         // Java: ret += Integer.valueOf(this.id()).hashCode();
         ret = ret.wrapping_add(self.id());
      }
      // Java: if (this.hasInitials())
        if self.hasInitials()
      {
         // Java: ret += String.valueOf(this.initials()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.initials()));
      }
      // Java: if (this.hasLastCommentIndex())
        if self.hasLastCommentIndex()
      {
         // Java: ret += Integer.valueOf(this.lastCommentIndex()).hashCode();
         ret = ret.wrapping_add(self.lastCommentIndex());
      }
      // Java: if (this.hasName())
        if self.hasName()
      {
         // Java: ret += String.valueOf(this.name()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.name()));
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
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
    }

    // Java: void follow(CommentAuthor parent)
    pub fn follow(&mut self, parent: Option<Box<CommentAuthor>>) {
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

    // Java: CommentAuthor parent()
    pub fn get_parent(&self) -> Option<&CommentAuthor> {
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

    // Java: int colorIndex()
    pub fn colorIndex(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(COMMENT_AUTHOR_COLOR_INDEX)
      {
         // Java: return this.parent == null ? 0 : this.parent.colorIndex();
         return self.parent.as_ref().map_or(0, |p| p.colorIndex());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(COMMENT_AUTHOR_COLOR_INDEX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasColorIndex()
    pub fn hasColorIndex(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasColorIndex();
      self.map.has(COMMENT_AUTHOR_COLOR_INDEX) || self.parent.as_ref().map_or(false, |p| p.hasColorIndex())
    }

    // Java: void deleteColorIndex()
    pub fn deleteColorIndex(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(COMMENT_AUTHOR_COLOR_INDEX);
    }

    // Java: void putColorIndex(int value)
    pub fn putColorIndex(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_AUTHOR_COLOR_INDEX, Encode::from_int(value, self.zone));
    }

    // Java: int id()
    pub fn id(&self) -> i32 {
      // Java: if (!this.map.has(2))
        if !self.map.has(COMMENT_AUTHOR_ID)
      {
         // Java: return this.parent == null ? 0 : this.parent.id();
         return self.parent.as_ref().map_or(0, |p| p.id());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(COMMENT_AUTHOR_ID);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasId()
    pub fn hasId(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasId();
      self.map.has(COMMENT_AUTHOR_ID) || self.parent.as_ref().map_or(false, |p| p.hasId())
    }

    // Java: void deleteId()
    pub fn deleteId(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(COMMENT_AUTHOR_ID);
    }

    // Java: void putId(int value)
    pub fn putId(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_AUTHOR_ID, Encode::from_int(value, self.zone));
    }

    // Java: String initials()
    pub fn initials(&self) -> String {
      // Java: if (!this.map.has(3))
        if !self.map.has(COMMENT_AUTHOR_INITIALS)
      {
         // Java: return this.parent == null ? "" : this.parent.initials();
         return self.parent.as_ref().map_or(String::new(), |p| p.initials());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(COMMENT_AUTHOR_INITIALS);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasInitials()
    pub fn hasInitials(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasInitials();
      self.map.has(COMMENT_AUTHOR_INITIALS) || self.parent.as_ref().map_or(false, |p| p.hasInitials())
    }

    // Java: void deleteInitials()
    pub fn deleteInitials(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(COMMENT_AUTHOR_INITIALS);
    }

    // Java: void putInitials(String value)
    pub fn putInitials(&mut self, value: &str) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_AUTHOR_INITIALS, Encode::from_string(value, self.zone));
    }

    // Java: int lastCommentIndex()
    pub fn lastCommentIndex(&self) -> i32 {
      // Java: if (!this.map.has(4))
        if !self.map.has(COMMENT_AUTHOR_LAST_COMMENT_INDEX)
      {
         // Java: return this.parent == null ? 0 : this.parent.lastCommentIndex();
         return self.parent.as_ref().map_or(0, |p| p.lastCommentIndex());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(COMMENT_AUTHOR_LAST_COMMENT_INDEX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasLastCommentIndex()
    pub fn hasLastCommentIndex(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasLastCommentIndex();
      self.map.has(COMMENT_AUTHOR_LAST_COMMENT_INDEX) || self.parent.as_ref().map_or(false, |p| p.hasLastCommentIndex())
    }

    // Java: void deleteLastCommentIndex()
    pub fn deleteLastCommentIndex(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(COMMENT_AUTHOR_LAST_COMMENT_INDEX);
    }

    // Java: void putLastCommentIndex(int value)
    pub fn putLastCommentIndex(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_AUTHOR_LAST_COMMENT_INDEX, Encode::from_int(value, self.zone));
    }

    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(5))
        if !self.map.has(COMMENT_AUTHOR_NAME)
      {
         // Java: return this.parent == null ? "" : this.parent.name();
         return self.parent.as_ref().map_or(String::new(), |p| p.name());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(COMMENT_AUTHOR_NAME);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasName();
      self.map.has(COMMENT_AUTHOR_NAME) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(COMMENT_AUTHOR_NAME);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_AUTHOR_NAME, Encode::from_string(value, self.zone));
    }

}

impl KoAr for CommentAuthor {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { CommentAuthor::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CommentAuthor::lastBin(self, var1) }
}

impl KoClosable for CommentAuthor {
    fn close(&mut self) { CommentAuthor::close(self); }
}

// Java: int hashCode() —— `String.valueOf(x).hashCode()` 的 Java 语义（UTF-16 code unit 31 进制）
fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
