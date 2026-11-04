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

// Java: public class Ole implements KoAr, KoClosable
pub struct Ole {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: Ole parent
    pub(crate) parent: Option<Box<Ole>>,
}

// Java: private static final int $name = 1;
const OLE_NAME: i32 = 1;
// Java: private static final int $showAsIcon = 2;
const OLE_SHOW_AS_ICON: i32 = 2;
// Java: private static final int $imageWidth = 3;
const OLE_IMAGE_WIDTH: i32 = 3;
// Java: private static final int $imageHeight = 4;
const OLE_IMAGE_HEIGHT: i32 = 4;
// Java: private static final int $spid = 5;
const OLE_SPID: i32 = 5;
// Java: private static final int $mediaId = 6;
const OLE_MEDIA_ID: i32 = 6;

// Java: public class Ole implements KoAr, KoClosable
impl Ole {
    // Java: static protected static <Value1 extends Ole> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Ole::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Ole create()
    pub fn create() -> Self {
      // Java: return initialize(Ole.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Ole open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Ole.class, bin);
      Self::initialize(bin)
    }

    // Java: Ole()
    pub fn new() -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        // Java field initializer: Ole parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: Ole(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait) -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        // Java field initializer: Ole parent（Java 默认 null）
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

    // Java: void load(Ole value)
    pub fn load_value(&mut self, value: Option<&Ole>) {
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

    // Java: boolean equalsLax(Ole o)
    pub fn equalsLax(&mut self, o: &mut Ole) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut Ole) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级真实端口 `comment/Comment.rs::equals_inner` 的既有约定命名。
    fn equals_inner(&mut self, o: &mut Ole, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Ole)
      if true
      {
         // Java: Ole dst = (Ole)o;
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
            // Java: else if ((this.hasShowAsIcon() || !dst.hasShowAsIcon()) && (!this.hasShowAsIcon() || dst.hasShowAsIcon()))
            else if (self.hasShowAsIcon() || !dst.hasShowAsIcon()) && (!self.hasShowAsIcon() || dst.hasShowAsIcon())
            {
               // Java: if (this.hasShowAsIcon() && dst.hasShowAsIcon() && this.isShowAsIcon() != dst.isShowAsIcon())
               if self.hasShowAsIcon() && dst.hasShowAsIcon() && self.isShowAsIcon() != dst.isShowAsIcon()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasImageWidth() || !dst.hasImageWidth()) && (!this.hasImageWidth() || dst.hasImageWidth()))
               else if (self.hasImageWidth() || !dst.hasImageWidth()) && (!self.hasImageWidth() || dst.hasImageWidth())
               {
                  // Java: if (this.hasImageWidth() && dst.hasImageWidth() && this.imageWidth() != dst.imageWidth())
                  if self.hasImageWidth() && dst.hasImageWidth() && self.imageWidth() != dst.imageWidth()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasImageHeight() || !dst.hasImageHeight()) && (!this.hasImageHeight() || dst.hasImageHeight()))
                  else if (self.hasImageHeight() || !dst.hasImageHeight()) && (!self.hasImageHeight() || dst.hasImageHeight())
                  {
                     // Java: if (this.hasImageHeight() && dst.hasImageHeight() && this.imageHeight() != dst.imageHeight())
                     if self.hasImageHeight() && dst.hasImageHeight() && self.imageHeight() != dst.imageHeight()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasSpid() || !dst.hasSpid()) && (!this.hasSpid() || dst.hasSpid()))
                     else if (self.hasSpid() || !dst.hasSpid()) && (!self.hasSpid() || dst.hasSpid())
                     {
                        // Java: if (this.hasSpid() && dst.hasSpid() && !this.spid().equals(dst.spid()))
                        if self.hasSpid() && dst.hasSpid() && self.spid() != dst.spid()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasMediaId() || !dst.hasMediaId()) && (!this.hasMediaId() || dst.hasMediaId()))
                        else if (self.hasMediaId() || !dst.hasMediaId()) && (!self.hasMediaId() || dst.hasMediaId())
                        {
                           // Java: return !this.hasMediaId() || !dst.hasMediaId() || this.mediaId() == dst.mediaId();
                           return !self.hasMediaId() || !dst.hasMediaId() || self.mediaId() == dst.mediaId();
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
      // Java: if (this.hasShowAsIcon())
      if self.hasShowAsIcon()
      {
         // Java: ret += Boolean.valueOf(this.isShowAsIcon()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isShowAsIcon()));
      }
      // Java: if (this.hasImageWidth())
      if self.hasImageWidth()
      {
         // Java: ret += Integer.valueOf(this.imageWidth()).hashCode();
         ret = ret.wrapping_add(self.imageWidth());
      }
      // Java: if (this.hasImageHeight())
      if self.hasImageHeight()
      {
         // Java: ret += Integer.valueOf(this.imageHeight()).hashCode();
         ret = ret.wrapping_add(self.imageHeight());
      }
      // Java: if (this.hasSpid())
      if self.hasSpid()
      {
         // Java: ret += String.valueOf(this.spid()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.spid()));
      }
      // Java: if (this.hasMediaId())
      if self.hasMediaId()
      {
         // Java: ret += Integer.valueOf(this.mediaId()).hashCode();
         ret = ret.wrapping_add(self.mediaId());
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
      // Java: （空实现）
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: （空实现）
    }

    // Java: void follow(Ole parent)
    pub fn follow(&mut self, parent: Option<Box<Ole>>) {
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

    // Java: Ole parent()
    pub fn get_parent(&self) -> Option<&Ole> {
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
      if !self.map.has(OLE_NAME)
      {
         // Java: return this.parent == null ? "" : this.parent.name();
         return self.parent.as_ref().map_or(String::new(), |p| p.name());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(OLE_NAME);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
      self.map.has(OLE_NAME) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(OLE_NAME);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(OLE_NAME, Encode::from_string(value, self.zone));
    }

    // Java: boolean isShowAsIcon()
    pub fn isShowAsIcon(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(OLE_SHOW_AS_ICON)
      {
         // Java: return this.parent == null ? false : this.parent.isShowAsIcon();
         return self.parent.as_ref().map_or(false, |p| p.isShowAsIcon());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(OLE_SHOW_AS_ICON);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasShowAsIcon()
    pub fn hasShowAsIcon(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasShowAsIcon();
      self.map.has(OLE_SHOW_AS_ICON) || self.parent.as_ref().map_or(false, |p| p.hasShowAsIcon())
    }

    // Java: void deleteShowAsIcon()
    pub fn deleteShowAsIcon(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(OLE_SHOW_AS_ICON);
    }

    // Java: void putShowAsIcon(boolean value)
    pub fn putShowAsIcon(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(OLE_SHOW_AS_ICON, Encode::from_bool(value, self.zone));
    }

    // Java: int imageWidth()
    pub fn imageWidth(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(OLE_IMAGE_WIDTH)
      {
         // Java: return this.parent == null ? 0 : this.parent.imageWidth();
         return self.parent.as_ref().map_or(0, |p| p.imageWidth());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(OLE_IMAGE_WIDTH);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasImageWidth()
    pub fn hasImageWidth(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasImageWidth();
      self.map.has(OLE_IMAGE_WIDTH) || self.parent.as_ref().map_or(false, |p| p.hasImageWidth())
    }

    // Java: void deleteImageWidth()
    pub fn deleteImageWidth(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(OLE_IMAGE_WIDTH);
    }

    // Java: void putImageWidth(int value)
    pub fn putImageWidth(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(OLE_IMAGE_WIDTH, Encode::from_int(value, self.zone));
    }

    // Java: int imageHeight()
    pub fn imageHeight(&self) -> i32 {
      // Java: if (!this.map.has(4))
      if !self.map.has(OLE_IMAGE_HEIGHT)
      {
         // Java: return this.parent == null ? 0 : this.parent.imageHeight();
         return self.parent.as_ref().map_or(0, |p| p.imageHeight());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(OLE_IMAGE_HEIGHT);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasImageHeight()
    pub fn hasImageHeight(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasImageHeight();
      self.map.has(OLE_IMAGE_HEIGHT) || self.parent.as_ref().map_or(false, |p| p.hasImageHeight())
    }

    // Java: void deleteImageHeight()
    pub fn deleteImageHeight(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(OLE_IMAGE_HEIGHT);
    }

    // Java: void putImageHeight(int value)
    pub fn putImageHeight(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(OLE_IMAGE_HEIGHT, Encode::from_int(value, self.zone));
    }

    // Java: String spid()
    pub fn spid(&self) -> String {
      // Java: if (!this.map.has(5))
      if !self.map.has(OLE_SPID)
      {
         // Java: return this.parent == null ? "" : this.parent.spid();
         return self.parent.as_ref().map_or(String::new(), |p| p.spid());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(OLE_SPID);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasSpid()
    pub fn hasSpid(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasSpid();
      self.map.has(OLE_SPID) || self.parent.as_ref().map_or(false, |p| p.hasSpid())
    }

    // Java: void deleteSpid()
    pub fn deleteSpid(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(OLE_SPID);
    }

    // Java: void putSpid(String value)
    pub fn putSpid(&mut self, value: &str) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(OLE_SPID, Encode::from_string(value, self.zone));
    }

    // Java: int mediaId()
    pub fn mediaId(&self) -> i32 {
      // Java: if (!this.map.has(6))
      if !self.map.has(OLE_MEDIA_ID)
      {
         // Java: return this.parent == null ? 0 : this.parent.mediaId();
         return self.parent.as_ref().map_or(0, |p| p.mediaId());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(OLE_MEDIA_ID);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasMediaId()
    pub fn hasMediaId(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasMediaId();
      self.map.has(OLE_MEDIA_ID) || self.parent.as_ref().map_or(false, |p| p.hasMediaId())
    }

    // Java: void deleteMediaId()
    pub fn deleteMediaId(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(OLE_MEDIA_ID);
    }

    // Java: void putMediaId(int value)
    pub fn putMediaId(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(OLE_MEDIA_ID, Encode::from_int(value, self.zone));
    }

}

impl KoAr for Ole {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Ole::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`（写 `this.box`/`this.map`），
    // 而 `KoAr` 接口在 Rust 侧声明为 `&self`。沿用同级真实端口
    // `comment/Comment.rs`、`background/Background.rs` 的既有约定，在此处显式声明该限制。
    // owner: show.data.ole    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Ole::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Ole::lastBin(self, var1) }
}

impl KoClosable for Ole {
    fn close(&mut self) { Ole::close(self); }
}

// Java: int hashCode() —— `String.valueOf(x).hashCode()` 的 Java 语义（UTF-16 code unit 31 进制）
fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

// Java: int hashCode() —— `Boolean.valueOf(x).hashCode()` 的 Java 语义（true=1231 / false=1237）
fn java_bool_hash(b: bool) -> i32 {
    if b { 1231 } else { 1237 }
}
