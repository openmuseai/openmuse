#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.Point2D
use crate::cn::wps::drawing::data::basic::Point2D::Point2D;
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

// Java: public class Comment implements KoAr, KoClosable
pub struct Comment {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Point2D position
    pub(crate) position: Option<Box<Point2D>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Comment parent
    pub(crate) parent: Option<Box<Comment>>,
}

// Java: private static final int $position = 1;
const COMMENT_POSITION: i32 = 1;
// Java: private static final int $textContent = 2;
const COMMENT_TEXT_CONTENT: i32 = 2;
// Java: private static final int $authorId = 3;
const COMMENT_AUTHOR_ID: i32 = 3;
// Java: private static final int $dateTime = 4;
const COMMENT_DATE_TIME: i32 = 4;
// Java: private static final int $index = 5;
const COMMENT_INDEX: i32 = 5;

// Java: public class Comment implements KoAr, KoClosable
impl Comment {
    // Java: static protected static <Value1 extends Comment> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::Comment_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Comment create()
    pub fn create() -> Self {
      // Java: return initialize(Comment.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Comment open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Comment.class, bin);
      Self::initialize(bin)
    }

    // Java: Comment()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), position: None, parent: None,
      }
    }

    // Java: Comment(KoBin bin)
    pub fn Comment_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), position: None, parent: None,
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

    // Java: void load(Comment value)
    pub fn load_value(&mut self, value: Option<&Comment>) {
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
      // Java: if (this.position != null && this.position.dirty())
        if self.position.as_ref().map_or(false, |p| p.dirty())
      {
         // Java: this.map.put(1, this.position.saveBin(zone));
         let bin = self.position.as_mut().unwrap().saveBin(zone);
         self.map.put(COMMENT_POSITION, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(Comment o)
    pub fn equalsLax(&mut self, o: &mut Comment) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut Comment) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级 `Hyperlink.rs` 的既有约定，私有重载取名 `equals_inner`。
    fn equals_inner(&mut self, o: &mut Comment, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Comment)
        if true
      {
         // Java: Comment dst = (Comment)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasPosition() || !dst.hasPosition()) && (!this.hasPosition() || dst.hasPosition()))
            else if (self.hasPosition() || !dst.hasPosition()) && (!self.hasPosition() || dst.hasPosition())
         {
            // Java: if (this.hasPosition() && dst.hasPosition())
                if self.hasPosition() && dst.hasPosition()
            {
               // Java: if (isStrict && !this.position().equals(dst.position()))
                   if isStrict && !self.position().equals(dst.position())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.position().equalsLax(dst.position()))
                   if !isStrict && !self.position().equalsLax(dst.position())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasTextContent() || !dst.hasTextContent()) && (!this.hasTextContent() || dst.hasTextContent()))
                if (self.hasTextContent() || !dst.hasTextContent()) && (!self.hasTextContent() || dst.hasTextContent())
            {
               // Java: if (this.hasTextContent() && dst.hasTextContent() && !this.textContent().equals(dst.textContent()))
                   if self.hasTextContent() && dst.hasTextContent() && self.textContent() != dst.textContent()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasAuthorId() || !dst.hasAuthorId()) && (!this.hasAuthorId() || dst.hasAuthorId()))
                   else if (self.hasAuthorId() || !dst.hasAuthorId()) && (!self.hasAuthorId() || dst.hasAuthorId())
               {
                  // Java: if (this.hasAuthorId() && dst.hasAuthorId() && this.authorId() != dst.authorId())
                      if self.hasAuthorId() && dst.hasAuthorId() && self.authorId() != dst.authorId()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasDateTime() || !dst.hasDateTime()) && (!this.hasDateTime() || dst.hasDateTime()))
                      else if (self.hasDateTime() || !dst.hasDateTime()) && (!self.hasDateTime() || dst.hasDateTime())
                  {
                     // Java: if (this.hasDateTime() && dst.hasDateTime() && !this.dateTime().equals(dst.dateTime()))
                         if self.hasDateTime() && dst.hasDateTime() && self.dateTime() != dst.dateTime()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasIndex() || !dst.hasIndex()) && (!this.hasIndex() || dst.hasIndex()))
                         else if (self.hasIndex() || !dst.hasIndex()) && (!self.hasIndex() || dst.hasIndex())
                     {
                        // Java: return !this.hasIndex() || !dst.hasIndex() || this.index() == dst.index();
                        return !self.hasIndex() || !dst.hasIndex() || self.index() == dst.index();
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasPosition())
        if self.hasPosition()
      {
         // Java: ret += this.position().hashCode();
         ret = ret.wrapping_add(self.position().hashCode());
      }
      // Java: if (this.hasTextContent())
        if self.hasTextContent()
      {
         // Java: ret += String.valueOf(this.textContent()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.textContent()));
      }
      // Java: if (this.hasAuthorId())
        if self.hasAuthorId()
      {
         // Java: ret += Integer.valueOf(this.authorId()).hashCode();
         ret = ret.wrapping_add(self.authorId());
      }
      // Java: if (this.hasDateTime())
        if self.hasDateTime()
      {
         // Java: ret += String.valueOf(this.dateTime()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.dateTime()));
      }
      // Java: if (this.hasIndex())
        if self.hasIndex()
      {
         // Java: ret += Integer.valueOf(this.index()).hashCode();
         ret = ret.wrapping_add(self.index());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.position != null && this.position.dirty() ? true : this.map.dirty();
      if self.position.as_ref().map_or(false, |p| p.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.position = null;
      self.position = None;
    }

    // Java: void follow(Comment parent)
    pub fn follow(&mut self, parent: Option<Box<Comment>>) {
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

    // Java: Comment parent()
    pub fn get_parent(&self) -> Option<&Comment> {
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

    // Java: Point2D position()
    pub fn position(&mut self) -> &mut Point2D {
      // Java: if (this.position == null)
      if self.position.is_none() {
         // Java: if (this.map.has(1))
         if self.map.has(COMMENT_POSITION) {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(COMMENT_POSITION);
            // Java: this.position = (Point2D)KoBin.Decode.toAny(Point2D.class, bin, this.zone);
            let mut inst = Point2D::new();
            inst.loadBin_with_zone(bin.as_ref(), self.zone);
            self.position = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasPosition())
         } else if self.parent.is_some() && self.parent.as_ref().unwrap().hasPosition() {
            // Java: this.position = this.parent.position();
            // LLM: Java 此处与父对象共享同一个 `Point2D` 引用；Rust 的 `Option<Box<Point2D>>`
            // 无法持有父引用，故沿用同级真实端口 `Slide.rs::clrMap()` 的既有约定：
            // 由父的 `lastBin(this.zone)` 重新物化一个等价实例。
            let mut inst = Point2D::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin_with_zone(bin.as_ref(), self.zone); }
            self.position = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.position = (Point2D)KoBin.Decode.toAny(Point2D.class, bin, this.zone);
            let mut inst = Point2D::new();
            inst.loadBin_with_zone(bin.as_ref(), self.zone);
            self.position = Some(Box::new(inst));
         }
      }
      // Java: return this.position;
      self.position.as_mut().unwrap()
    }

    // Java: boolean hasPosition()
    pub fn hasPosition(&self) -> bool {
      // Java: return this.map.has(1) || this.position != null && this.position.dirty() || this.parent != null && this.parent.hasPosition();
      self.map.has(COMMENT_POSITION) || self.position.as_ref().map_or(false, |p| p.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPosition())
    }

    // Java: void deletePosition()
    pub fn deletePosition(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(COMMENT_POSITION);
      // Java: if (this.position != null)
        if self.position.is_some()
      {
         // Java: this.position = null;
         self.position = None;
      }
    }

    // Java: void writePosition(Point2D value)
    pub fn writePosition(&mut self, value: Option<&Point2D>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(COMMENT_POSITION, bin);
            // Java: if (this.position != null)
                if self.position.is_some()
            {
               // Java: this.position = null;
               self.position = None;
            }
         }
      }
    }

    // Java: void forceWritePosition(Point2D value)
    pub fn forceWritePosition(&mut self, value: Option<&Point2D>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(COMMENT_POSITION, v.lastBin(self.zone));
         // Java: if (this.position != null)
             if self.position.is_some()
         {
            // Java: this.position = null;
            self.position = None;
         }
      }
    }

    // Java: String textContent()
    pub fn textContent(&self) -> String {
      // Java: if (!this.map.has(2))
        if !self.map.has(COMMENT_TEXT_CONTENT)
      {
         // Java: return this.parent == null ? "" : this.parent.textContent();
         return self.parent.as_ref().map_or(String::new(), |p| p.textContent());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(COMMENT_TEXT_CONTENT);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTextContent()
    pub fn hasTextContent(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasTextContent();
      self.map.has(COMMENT_TEXT_CONTENT) || self.parent.as_ref().map_or(false, |p| p.hasTextContent())
    }

    // Java: void deleteTextContent()
    pub fn deleteTextContent(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(COMMENT_TEXT_CONTENT);
    }

    // Java: void putTextContent(String value)
    pub fn putTextContent(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_TEXT_CONTENT, Encode::from_string(value, self.zone));
    }

    // Java: int authorId()
    pub fn authorId(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(COMMENT_AUTHOR_ID)
      {
         // Java: return this.parent == null ? 0 : this.parent.authorId();
         return self.parent.as_ref().map_or(0, |p| p.authorId());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(COMMENT_AUTHOR_ID);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasAuthorId()
    pub fn hasAuthorId(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasAuthorId();
      self.map.has(COMMENT_AUTHOR_ID) || self.parent.as_ref().map_or(false, |p| p.hasAuthorId())
    }

    // Java: void deleteAuthorId()
    pub fn deleteAuthorId(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(COMMENT_AUTHOR_ID);
    }

    // Java: void putAuthorId(int value)
    pub fn putAuthorId(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_AUTHOR_ID, Encode::from_int(value, self.zone));
    }

    // Java: String dateTime()
    pub fn dateTime(&self) -> String {
      // Java: if (!this.map.has(4))
        if !self.map.has(COMMENT_DATE_TIME)
      {
         // Java: return this.parent == null ? "" : this.parent.dateTime();
         return self.parent.as_ref().map_or(String::new(), |p| p.dateTime());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(COMMENT_DATE_TIME);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasDateTime()
    pub fn hasDateTime(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasDateTime();
      self.map.has(COMMENT_DATE_TIME) || self.parent.as_ref().map_or(false, |p| p.hasDateTime())
    }

    // Java: void deleteDateTime()
    pub fn deleteDateTime(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(COMMENT_DATE_TIME);
    }

    // Java: void putDateTime(String value)
    pub fn putDateTime(&mut self, value: &str) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_DATE_TIME, Encode::from_string(value, self.zone));
    }

    // Java: int index()
    pub fn index(&self) -> i32 {
      // Java: if (!this.map.has(5))
        if !self.map.has(COMMENT_INDEX)
      {
         // Java: return this.parent == null ? 0 : this.parent.index();
         return self.parent.as_ref().map_or(0, |p| p.index());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(COMMENT_INDEX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasIndex()
    pub fn hasIndex(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasIndex();
      self.map.has(COMMENT_INDEX) || self.parent.as_ref().map_or(false, |p| p.hasIndex())
    }

    // Java: void deleteIndex()
    pub fn deleteIndex(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(COMMENT_INDEX);
    }

    // Java: void putIndex(int value)
    pub fn putIndex(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(COMMENT_INDEX, Encode::from_int(value, self.zone));
    }

}

impl KoAr for Comment {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Comment::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Comment::lastBin(self, var1) }
}

impl KoClosable for Comment {
    fn close(&mut self) { Comment::close(self); }
}

// Java: int hashCode() —— `String.valueOf(x).hashCode()` 的 Java 语义（UTF-16 code unit 31 进制）
fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
