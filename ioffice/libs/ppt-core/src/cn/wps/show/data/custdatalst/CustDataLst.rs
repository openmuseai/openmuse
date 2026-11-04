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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import cn.wps.show.data.custdatalst.Tag
use crate::cn::wps::show::data::custdatalst::Tag::Tag;
// Java: import java.util.ArrayList
// LLM Rust No Need: using Vec instead
// Java: import java.util.Collection
// LLM Rust No Need: using Vec/slice instead

// Java: public class CustDataLst implements KoAr, KoClosable
pub struct CustDataLst {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TagLst tags
    pub(crate) tags: Option<Box<TagLst>>,
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: CustDataLst parent
    pub(crate) parent: Option<Box<CustDataLst>>,
}

// Java: private static final int $tags = 1;
const CUSTDATALST_TAGS: i32 = 1;

// Java: public class CustDataLst implements KoAr, KoClosable
impl CustDataLst {
    // Java: static protected static <Value1 extends CustDataLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = CustDataLst::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static CustDataLst create()
    pub fn create() -> Self {
      // Java: return initialize(CustDataLst.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static CustDataLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CustDataLst.class, bin);
      Self::initialize(bin)
    }

    // Java: CustDataLst()
    pub fn new() -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java field initializer: TagLst tags（Java 默认 null）
        tags: None,
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        // Java field initializer: CustDataLst parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: CustDataLst(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait) -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java field initializer: TagLst tags（Java 默认 null）
        tags: None,
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        // Java field initializer: CustDataLst parent（Java 默认 null）
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

    // Java: void load(CustDataLst value)
    pub fn load_value(&mut self, value: Option<&CustDataLst>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.tags != null && this.tags.dirty())
      if self.tags.as_ref().map_or(false, |t| t.dirty()) {
         // Java: this.map.put(1, this.tags.saveBin(zone));
         let bin = TagLst::saveBin(self.tags.as_mut().unwrap(), zone);
         self.map.put(CUSTDATALST_TAGS, bin);
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

    // Java: boolean equalsLax(CustDataLst o)
    pub fn equalsLax(&mut self, o: &mut CustDataLst) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut CustDataLst) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级真实端口 `comment/Comment.rs::equals_inner` 的既有约定命名。
    fn equals_inner(&mut self, o: &mut CustDataLst, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof CustDataLst)
      if true
      {
         // Java: CustDataLst dst = (CustDataLst)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasTags() || !dst.hasTags()) && (!this.hasTags() || dst.hasTags()))
         else if (self.hasTags() || !dst.hasTags()) && (!self.hasTags() || dst.hasTags())
         {
            // Java: if (this.hasTags() && dst.hasTags())
            if self.hasTags() && dst.hasTags()
            {
               // Java: if (isStrict && !this.tags().equals(dst.tags()))
               if isStrict && !self.tags().equals(dst.tags())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.tags().equalsLax(dst.tags()))
               if !isStrict && !self.tags().equalsLax(dst.tags())
               {
                  // Java: return false;
                  return false;
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
      // Java: if (this.hasTags())
      if self.hasTags()
      {
         // Java: ret += this.tags().hashCode();
         ret = ret.wrapping_add(self.tags().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.tags != null && this.tags.dirty() ? true : this.map.dirty();
      if self.tags.as_ref().map_or(false, |t| t.dirty()) { true } else { self.map.dirty() }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      CustDataLst::saveBin(self, zone);
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
      // Java: this.tags = null;
      self.tags = None;
    }

    // Java: void follow(CustDataLst parent)
    pub fn follow(&mut self, parent: Option<Box<CustDataLst>>) {
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

    // Java: CustDataLst parent()
    pub fn get_parent(&self) -> Option<&CustDataLst> {
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

    // Java: TagLst tags()
    pub fn tags(&mut self) -> &mut TagLst {
      // Java: if (this.tags == null)
      if self.tags.is_none() {
         // Java: if (this.map.has(1))
         if self.map.has(CUSTDATALST_TAGS) {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(CUSTDATALST_TAGS);
            // Java: this.tags = (TagLst)KoBin.Decode.toAny(TagLst.class, bin, this.zone);
            let mut inst = TagLst::new(self.zone);
            inst.loadBin(bin.as_ref(), self.zone);
            self.tags = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasTags())
         } else if self.parent.as_ref().map_or(false, |p| p.hasTags()) {
            // Java: this.tags = this.parent.tags();
            // LLM: Java 此处与父对象共享同一个 `TagLst` 引用；Rust 的 `Option<Box<TagLst>>`
            // 无法持有父引用，故沿用同级真实端口 `comment/Comment.rs::position()` 的既有约定：
            // 由父的 `lastBin(this.zone)` 重新物化一个等价实例。
            let mut inst = TagLst::new(self.zone);
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.tags = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.tags = (TagLst)KoBin.Decode.toAny(TagLst.class, bin, this.zone);
            let mut inst = TagLst::new(self.zone);
            inst.loadBin(bin.as_ref(), self.zone);
            self.tags = Some(Box::new(inst));
         }
      }
      // Java: return this.tags;
      self.tags.as_mut().unwrap()
    }

    // Java: boolean hasTags()
    pub fn hasTags(&self) -> bool {
      // Java: return this.map.has(1) || this.tags != null && this.tags.dirty() || this.parent != null && this.parent.hasTags();
      self.map.has(CUSTDATALST_TAGS) || self.tags.as_ref().map_or(false, |t| t.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTags())
    }

    // Java: void deleteTags()
    pub fn deleteTags(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(CUSTDATALST_TAGS);
      // Java: if (this.tags != null)
      if self.tags.is_some()
      {
         // Java: this.tags = null;
         self.tags = None;
      }
    }

    // Java: void writeTags(TagLst value)
    pub fn writeTags(&mut self, value: Option<&TagLst>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(CUSTDATALST_TAGS, bin);
            // Java: if (this.tags != null)
            if self.tags.is_some()
            {
               // Java: this.tags = null;
               self.tags = None;
            }
         }
      }
    }

    // Java: void forceWriteTags(TagLst value)
    pub fn forceWriteTags(&mut self, value: Option<&TagLst>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(CUSTDATALST_TAGS, v.lastBin(self.zone));
         // Java: if (this.tags != null)
         if self.tags.is_some()
         {
            // Java: this.tags = null;
            self.tags = None;
         }
      }
    }

}

impl KoAr for CustDataLst {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { CustDataLst::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`（写 `this.box`/`this.map`），
    // 而 `KoAr` 接口在 Rust 侧声明为 `&self`。沿用同级真实端口
    // `comment/Comment.rs`、`background/Background.rs` 的既有约定，在此处显式声明该限制。
    // owner: show.data.custdatalst    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { CustDataLst::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CustDataLst::lastBin(self, var1) }
}

impl KoClosable for CustDataLst {
    fn close(&mut self) { CustDataLst::close(self); }
}

// Java: public static class TagLst implements KoAr, KoClosable
pub struct TagLst {
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList, // Java field:list // LLM: KpList is binary protocol list, not java.util.List
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public static class TagLst implements KoAr, KoClosable
impl TagLst {
    // Java: TagLst()
    pub fn new(zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.list = new KpList(this.zone);
        list: KpList::new(zone),
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone,
      }
    }

    // Java: TagLst(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.list = new KpList(bin, this.zone);
        list: KpList::KpList_from(bin, zone),
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.list.load(this.box.get(), zone);
      let bin_ref = self.bin_box.get();
      self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(TagLst value)
    pub fn load_value(&mut self, value: Option<&TagLst>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
      if self.list.dirty() {
         let saved = self.list.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(TagLst o)
    pub fn equalsLax(&mut self, o: &mut TagLst) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut TagLst) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级真实端口 `comment/Comment.rs::equals_inner` 的既有约定命名。
    fn equals_inner(&mut self, o: &mut TagLst, isStrict: bool) -> bool {
      // Java: if (null == o)
      if false {
         // Java: return false;
         return false;
      // Java: else
      } else {
         // Java: TagLst dst = (TagLst)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         // Java: else
         } else {
            // Java: ArrayList<Tag> dstList = new ArrayList();
            let mut dstList: Vec<Tag> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<Tag> srcList = new ArrayList();
            let mut srcList: Vec<Tag> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
            if dstList.len() != srcList.len() {
               // Java: return false;
               return false;
            // Java: else
            } else {
               // Java: int size = dstList.size();
               let size = dstList.len();
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size {
                  // Java: Tag dstClrTrans = (Tag)dstList.get(i);
                  // LLM JavaLine Aggregation: merged into the next Rust statement
                  // Java: Tag srcClrTrans = (Tag)srcList.get(i);
                  // LLM JavaLine Aggregation: merged into the next Rust statement
                  // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                  if isStrict && !dstList[i].equals(&mut srcList[i]) {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                  if !isStrict && !dstList[i].equalsLax(&mut srcList[i]) {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: return true;
               return true;
            }
         }
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.list.hashCode();
      self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.list.dirty();
      self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      TagLst::saveBin(self, zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get();
      self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.list.load(KoBin.Null.value1(), this.zone);
      let null_bin = Null::value1();
      self.list.load(null_bin.as_ref(), self.zone);
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends TagLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = TagLst::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TagLst create()
    pub fn create() -> Self {
      // Java: return initialize(TagLst.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TagLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TagLst.class, bin);
      Self::initialize(bin)
    }

    // Java: void read(Collection<Tag> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Tag>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: valueLst.add((Tag)KoBin.Decode.toAny(Tag.class, this.list.get(i), this.zone));
         let bin = self.list.get(i);
         let mut inst = Tag::new();
         // LLM Rust Fix: `KpList.get()` 返回 `Box<dyn KoBin>`，端口方法按 Java 取 `KoBin` 引用，
         // 与同级真实端口 `slides/Slide.rs::CommentList::read` 一致补 `.as_ref()`。
         inst.loadBin(bin.as_ref(), self.zone);
         valueLst.push(inst);
      }
    }

    // Java: void write(Iterable<Tag> valueLst)
    pub fn write(&mut self, valueLst: &[Tag]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(Tag i : valueLst)
      for item in valueLst {
         // Java: KoBin bin = i.lastBin(this.zone);
         let bin = item.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.list.add(bin);
            self.list.add(bin);
         }
      }
    }

    // Java: void forceWrite(Iterable<Tag> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Tag]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(Tag i : valueLst)
      for item in valueLst {
         // Java: this.list.add(i.lastBin(this.zone));
         let bin = item.lastBin(self.zone);
         self.list.add(bin);
      }
    }

}

impl KoAr for TagLst {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { TagLst::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`（写 `this.box`/`this.list`），
    // 而 `KoAr` 接口在 Rust 侧声明为 `&self`。沿用同级真实端口
    // `slides/Slide.rs::CommentList` 家族的既有约定，在此处显式声明该限制。
    // owner: show.data.custdatalst    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { TagLst::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TagLst::lastBin(self, var1) }
}

impl KoClosable for TagLst {
    fn close(&mut self) { TagLst::close(self); }
}
