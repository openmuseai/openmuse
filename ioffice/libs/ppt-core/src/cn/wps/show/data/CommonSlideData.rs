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
// Java: import cn.wps.show.data.background.Background
// M-PPT-KMOSLIDE W6c-3（形态 C 消除）：此处原先是**内联简化桩** `Background`（`equals` 恒
//   `false`、`saveBin` 不写 map），顶替真实 `data/background/Background.rs`（833 行真身）。
//   其历史阻塞是 `drawing::data::effect` 被 `#[cfg(feature = "full")]` 关在默认构建之外
//   （假编译面形态 D）。本波已解除该 gate（见 `drawing/data/mod.rs`），并在
//   `show/mod.rs` 的内联桩 data 模块中登记真实 `background`，故此处改用**真身**。
//   owner: show.data    wave: M-PPT-KMOSLIDE-W6c-3
pub use crate::cn::wps::show::data::background::Background::Background;
// rust: ★M-PPT-KMOSLIDE W6c-3 —— `background_handle()` 的 `Rc<RefCell<_>>` 承载类型
//   （Java 无对应 import；Java 用裸引用）。owner: show.data.CommonSlideData · wave: M-PPT-KMOSLIDE-W6c-3
use std::cell::RefCell;
use std::rc::Rc;
// Java: import cn.wps.show.data.control.Control
// M-PPT-KMOSLIDE SP1.0（1.0.3）：原先此处以内联简化桩顶替 `control::Control`（同属假编译面形态 C）。
// `Control.java`（391 行）现已真实移植，改用真实端口。
// owner: show.data    wave: M-PPT-KMOSLIDE-SP1
use crate::cn::wps::show::data::control::Control::Control;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
use crate::cn::wps::show::data::custdatalst::CustDataLst::CustDataLst;
// Java: import java.util.ArrayList
// LLM Rust No Need: using Vec instead
// Java: import java.util.Collection
// LLM Rust No Need: using Vec/slice instead

// Java: public class CommonSlideData implements KoAr, KoClosable
pub struct CommonSlideData {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Background background
    pub(crate) background: Option<Box<Background>>,
    // rust: ★M-PPT-KMOSLIDE W6c-3 —— `background()` 的**共享句柄**槽（无 Java 对应字段）。
    //   Java 的 `background()` 返回**同一个对象引用**，`BackgroundHandler` 长期持有它并在
    //   `enterSubElement` 中写回同一对象；Rust 的 `&mut T` 无法跨处理器持有 ⇒ 按
    //   `ShapePr::{fill,ln,..}_handle`（PLAN §27.4 2B-②）的既有约定给出 `Rc<RefCell<_>>` 句柄。
    //   owner: show.data.CommonSlideData    wave: M-PPT-KMOSLIDE-W6c-3
    pub(crate) background_handle: Option<Rc<RefCell<Background>>>,
    // Java field: ControlList controlList
    pub(crate) controlList: Option<Box<ControlList>>, // Java field:controlList // LLM: ControlList is a KpList wrapper, not java.util.List
    // Java field: CustDataLst custDataLst
    pub(crate) custDataLst: Option<Box<CustDataLst>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: CommonSlideData parent
    pub(crate) parent: Option<Box<CommonSlideData>>,
}

// Java: public class ControlList
pub struct ControlList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList, // Java field:list // LLM: KpList is binary protocol list, not java.util.List
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class CommonSlideData implements KoAr, KoClosable
impl CommonSlideData {
    // Java: static protected static <Value1 extends CommonSlideData> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = CommonSlideData::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static CommonSlideData create()
    pub fn create() -> Self {
      // Java: return initialize(CommonSlideData.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }

    // Java: static CommonSlideData open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CommonSlideData.class, bin);
      Self::initialize(bin)
    }

    // Java: CommonSlideData()
    pub fn new(zone: &'static KoZone) -> Self {
      Self {
        zone,
        background: None,
        background_handle: None,
        controlList: None,
        custDataLst: None,
        // Java: this.box = new KpBinBox();
        // LLM: box renamed to bin_box (Rust keyword)
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM: map init in struct literal (no separate assignment needed)
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: CommonSlideData(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        zone,
        background: None,
        background_handle: None,
        controlList: None,
        custDataLst: None,
        // Java: this.box = new KpBinBox(bin);
        // LLM: box renamed to bin_box (Rust keyword)
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM: map init in struct literal (no separate assignment needed)
        map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }
    pub fn CommonSlideData_from(bin: &dyn KoBinTrait) -> Self { Self::new_from_bin(bin, KoZone::defaultZone()) }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get(); self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }

    // Java: void load(CommonSlideData value)
    pub fn load_value(&mut self, value: Option<&CommonSlideData>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.background != null && this.background.dirty())
      if self.background.is_some() && self.background.as_ref().unwrap().dirty() {
         // Java: this.map.put(1, this.background.saveBin(zone));
         let bin = self.background.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.controlList != null && this.controlList.dirty())
      if self.controlList.is_some() && self.controlList.as_ref().unwrap().dirty() {
         // Java: this.map.put(3, this.controlList.saveBin(zone));
         let bin = self.controlList.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }
      // Java: if (this.custDataLst != null && this.custDataLst.dirty())
      if self.custDataLst.is_some() && self.custDataLst.as_ref().unwrap().dirty() {
         // Java: this.map.put(4, this.custDataLst.saveBin(zone));
         let bin = CustDataLst::saveBin(self.custDataLst.as_mut().unwrap(), zone);
         self.map.put(4, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(CommonSlideData o)
    pub fn equalsLax(&mut self, o: &mut CommonSlideData) -> bool {
      // Java: return this.equals(o, false);
      // LLM: equals renamed to equals_inner (Rust overload)
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut CommonSlideData) -> bool {
      // Java: return this.equals(o, true);
      // LLM: equals renamed to equals_inner (Rust overload)
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut CommonSlideData, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof CommonSlideData)
      if true && true {
         // Java: CommonSlideData dst = (CommonSlideData)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBackground() || !dst.hasBackground()) && (!this.hasBackground() || dst.hasBackground()))
         else if (self.hasBackground() || !dst.hasBackground()) && (!self.hasBackground() || dst.hasBackground()) {
            // Java: if (this.hasBackground() && dst.hasBackground())
            if self.hasBackground() && dst.hasBackground() {
               // Java: if (isStrict && !this.background().equals(dst.background()))
               if isStrict && !self.get_background_mut().equals(dst.get_background_mut()) {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.background().equalsLax(dst.background()))
               if !isStrict && !self.get_background_mut().equalsLax(dst.get_background_mut()) {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
               // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
               if self.hasName() && dst.hasName() && self.name() != dst.name() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasControlList() || !dst.hasControlList()) && (!this.hasControlList() || dst.hasControlList()))
               else if (self.hasControlList() || !dst.hasControlList()) && (!self.hasControlList() || dst.hasControlList()) {
                  // Java: if (this.hasControlList() && dst.hasControlList())
                  if self.hasControlList() && dst.hasControlList() {
                     // Java: if (isStrict && !this.controlList().equals(dst.controlList()))
                     if isStrict && !self.get_controlList_mut().equals(dst.get_controlList_mut()) {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.controlList().equalsLax(dst.controlList()))
                     if !isStrict && !self.get_controlList_mut().equalsLax(dst.get_controlList_mut()) {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasCustDataLst() || !dst.hasCustDataLst()) && (!this.hasCustDataLst() || dst.hasCustDataLst()))
                  if (self.hasCustDataLst() || !dst.hasCustDataLst()) && (!self.hasCustDataLst() || dst.hasCustDataLst()) {
                     // Java: if (this.hasCustDataLst() && dst.hasCustDataLst())
                     if self.hasCustDataLst() && dst.hasCustDataLst() {
                        // Java: if (isStrict && !this.custDataLst().equals(dst.custDataLst()))
                        if isStrict && !self.get_custDataLst_mut().equals(dst.get_custDataLst_mut()) {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.custDataLst().equalsLax(dst.custDataLst()))
                        if !isStrict && !self.get_custDataLst_mut().equalsLax(dst.get_custDataLst_mut()) {
                           // Java: return false;
                           return false;
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

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBackground())
      if self.hasBackground() {
         // Java: ret += this.background().hashCode();
         ret = ret.wrapping_add(self.get_background_mut().hashCode());
      }
      // Java: if (this.hasName())
      if self.hasName() {
         // Java: ret += String.valueOf(this.name()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.name()));
      }
      // Java: if (this.hasControlList())
      if self.hasControlList() {
         // Java: ret += this.controlList().hashCode();
         ret = ret.wrapping_add(self.get_controlList_mut().hashCode());
      }
      // Java: if (this.hasCustDataLst())
      if self.hasCustDataLst() {
         // Java: ret += this.custDataLst().hashCode();
         ret = ret.wrapping_add(self.get_custDataLst_mut().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.background != null && this.background.dirty())
      if self.background.is_some() && self.background.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.controlList != null && this.controlList.dirty())
      else if self.controlList.is_some() && self.controlList.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: return this.custDataLst != null && this.custDataLst.dirty() ? true : this.map.dirty();
         if self.custDataLst.is_some() && self.custDataLst.as_ref().unwrap().dirty() { true } else { self.map.dirty() }
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
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
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
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
      let null_bin = Null::value1(); self.map.load(null_bin.as_ref());
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
      // Java: this.background = null;
      self.background = None;
      // Java: this.controlList = null;
      self.controlList = None;
      // Java: this.custDataLst = null;
      self.custDataLst = None;
    }

    // Java: void follow(CommonSlideData parent)
    pub fn follow(&mut self, parent: Option<Box<CommonSlideData>>) {
      // Java: if (parent != null)
      if let Some(mut parent) = parent {
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
      if self.parent.is_some() {
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

    // Java: CommonSlideData parent()
    pub fn parent(&self) -> Option<&CommonSlideData> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM: discard renamed to discard_with_zone (Rust overload)
      let zone = self.zone; self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: Background background()
    pub fn get_background(&mut self) -> &Background {
      // Java: if (this.background == null)
      if self.background.is_none() {
         // Java: if (this.map.has(1))
         if self.map.has(1) {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.background = (Background)KoBin.Decode.toAny(Background.class, bin, this.zone);
            self.background = Some(Box::new(Background::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasBackground())
         else if self.parent.as_ref().map_or(false, |p| p.hasBackground()) {
            // Java: this.background = this.parent.background();
            let bin = self.parent.as_mut().unwrap().get_background().lastBin(self.zone);
            self.background = Some(Box::new(Background::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.background = (Background)KoBin.Decode.toAny(Background.class, bin, this.zone);
            self.background = Some(Box::new(Background::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.background;
      self.background.as_deref().unwrap()
    }

    pub fn get_background_mut(&mut self) -> &mut Background {
      if self.background.is_none() { self.get_background(); }
      self.background.as_deref_mut().unwrap()
    }

    // rust: ★M-PPT-KMOSLIDE W6c-3 —— `background()` 的**共享句柄**访问器（Java 无此法名）。
    //   Java 的 `background()` 返回**同一个对象引用**，`BackgroundHandler` 长期持有它并在
    //   `enterSubElement` 中写回同一对象。Rust 的 `&mut T` 无法跨处理器持有（独占借用），
    //   故按 `ShapePr::{fill,ln,..}_handle`（PLAN §27.4 2B-②）的既有约定给出
    //   `Rc<RefCell<_>>` 句柄：首次取句柄时把自有槽位内容**移入**句柄，之后所有读写都经句柄，
    //   即「真身槽位」与「处理器看到的对象」恒为同一个。
    //   owner: show.data.CommonSlideData    wave: M-PPT-KMOSLIDE-W6c-3
    pub fn background_handle(&mut self) -> Rc<RefCell<Background>> {
      if self.background_handle.is_none() {
        let value = match self.background.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(1);
            Background::new_from_bin(bin.as_ref(), self.zone)
          }
        };
        self.background_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.background_handle.as_ref().unwrap().clone()
    }

    // Java: boolean hasBackground()
    pub fn hasBackground(&self) -> bool {
      // Java: return this.map.has(1) || this.background != null && this.background.dirty() || this.parent != null && this.parent.hasBackground();
      // rust: ★共享句柄分支（PLAN §37.2(2)）。owner: show.data.CommonSlideData · wave: M-PPT-KMOSLIDE-W6c-3
      self.map.has(1) || self.background.as_ref().map_or(false, |v| v.dirty())
        || self.background_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasBackground())
    }

    // Java: void deleteBackground()
    pub fn deleteBackground(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.background != null)
      if self.background.is_some() {
         // Java: this.background = null;
         self.background = None;
      }
    }

    // Java: void writeBackground(Background value)
    pub fn writeBackground(&mut self, value: Option<&Background>) {
      // Java: if (value != null)
      if let Some(value) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.background != null)
            if self.background.is_some() {
               // Java: this.background = null;
               self.background = None;
            }
         }
      }
    }

    // Java: void forceWriteBackground(Background value)
    pub fn forceWriteBackground(&mut self, value: Option<&Background>) {
      // Java: if (value != null)
      if let Some(value) = value {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, value.lastBin(self.zone));
         // Java: if (this.background != null)
         if self.background.is_some() {
            // Java: this.background = null;
            self.background = None;
         }
      }
    }

    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? "" : this.parent.name();
         return if self.parent.is_none() { String::new() } else { self.parent.as_ref().unwrap().name() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasName();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_string(value, self.zone));
    }

    // Java: ControlList controlList()
    pub fn get_controlList(&mut self) -> &ControlList {
      // Java: if (this.controlList == null)
      if self.controlList.is_none() {
         // Java: if (this.map.has(3))
         if self.map.has(3) {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.controlList = (ControlList)KoBin.Decode.toAny(ControlList.class, bin, this.zone);
            self.controlList = Some(Box::new(ControlList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasControlList())
         else if self.parent.as_ref().map_or(false, |p| p.hasControlList()) {
            // Java: this.controlList = this.parent.controlList();
            let bin = self.parent.as_mut().unwrap().get_controlList().lastBin(self.zone);
            self.controlList = Some(Box::new(ControlList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.controlList = (ControlList)KoBin.Decode.toAny(ControlList.class, bin, this.zone);
            self.controlList = Some(Box::new(ControlList::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.controlList;
      self.controlList.as_deref().unwrap()
    }

    pub fn get_controlList_mut(&mut self) -> &mut ControlList {
      if self.controlList.is_none() { self.get_controlList(); }
      self.controlList.as_deref_mut().unwrap()
    }

    // Java: boolean hasControlList()
    pub fn hasControlList(&self) -> bool {
      // Java: return this.map.has(3) || this.controlList != null && this.controlList.dirty() || this.parent != null && this.parent.hasControlList();
      self.map.has(3) || self.controlList.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasControlList())
    }

    // Java: void deleteControlList()
    pub fn deleteControlList(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.controlList != null)
      if self.controlList.is_some() {
         // Java: this.controlList = null;
         self.controlList = None;
      }
    }

    // Java: void writeControlList(ControlList value)
    pub fn writeControlList(&mut self, value: Option<&ControlList>) {
      // Java: if (value != null)
      if let Some(value) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.controlList != null)
            if self.controlList.is_some() {
               // Java: this.controlList = null;
               self.controlList = None;
            }
         }
      }
    }

    // Java: void forceWriteControlList(ControlList value)
    pub fn forceWriteControlList(&mut self, value: Option<&ControlList>) {
      // Java: if (value != null)
      if let Some(value) = value {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, value.lastBin(self.zone));
         // Java: if (this.controlList != null)
         if self.controlList.is_some() {
            // Java: this.controlList = null;
            self.controlList = None;
         }
      }
    }

    // Java: CustDataLst custDataLst()
    pub fn get_custDataLst(&mut self) -> &CustDataLst {
      // Java: if (this.custDataLst == null)
      if self.custDataLst.is_none() {
         // Java: if (this.map.has(4))
         if self.map.has(4) {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.custDataLst = (CustDataLst)KoBin.Decode.toAny(CustDataLst.class, bin, this.zone);
            self.custDataLst = Some(Box::new(CustDataLst::new_from_bin(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasCustDataLst())
         else if self.parent.as_ref().map_or(false, |p| p.hasCustDataLst()) {
            // Java: this.custDataLst = this.parent.custDataLst();
            let bin = self.parent.as_mut().unwrap().get_custDataLst().lastBin(self.zone);
            self.custDataLst = Some(Box::new(CustDataLst::new_from_bin(bin.as_ref())));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.custDataLst = (CustDataLst)KoBin.Decode.toAny(CustDataLst.class, bin, this.zone);
            self.custDataLst = Some(Box::new(CustDataLst::new_from_bin(bin.as_ref())));
         }
      }
      // Java: return this.custDataLst;
      self.custDataLst.as_deref().unwrap()
    }

    pub fn get_custDataLst_mut(&mut self) -> &mut CustDataLst {
      if self.custDataLst.is_none() { self.get_custDataLst(); }
      self.custDataLst.as_deref_mut().unwrap()
    }

    // Java: boolean hasCustDataLst()
    pub fn hasCustDataLst(&self) -> bool {
      // Java: return this.map.has(4) || this.custDataLst != null && this.custDataLst.dirty() || this.parent != null && this.parent.hasCustDataLst();
      self.map.has(4) || self.custDataLst.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCustDataLst())
    }

    // Java: void deleteCustDataLst()
    pub fn deleteCustDataLst(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.custDataLst != null)
      if self.custDataLst.is_some() {
         // Java: this.custDataLst = null;
         self.custDataLst = None;
      }
    }

    // Java: void writeCustDataLst(CustDataLst value)
    pub fn writeCustDataLst(&mut self, value: Option<&CustDataLst>) {
      // Java: if (value != null)
      if let Some(value) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.custDataLst != null)
            if self.custDataLst.is_some() {
               // Java: this.custDataLst = null;
               self.custDataLst = None;
            }
         }
      }
    }

    // Java: void forceWriteCustDataLst(CustDataLst value)
    pub fn forceWriteCustDataLst(&mut self, value: Option<&CustDataLst>) {
      // Java: if (value != null)
      if let Some(value) = value {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, value.lastBin(self.zone));
         // Java: if (this.custDataLst != null)
         if self.custDataLst.is_some() {
            // Java: this.custDataLst = null;
            self.custDataLst = None;
         }
      }
    }

}

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

// Java: public class ControlList
impl ControlList {
    // Java: ControlList()
    pub fn new(zone: &'static KoZone) -> Self {
         Self {
           // Java: this.box = new KpBinBox();
           // LLM: box renamed to bin_box (Rust keyword)
           bin_box: KpBinBox::new(),
           // Java: this.list = new KpList(this.zone);
           // LLM: list init in struct literal (no separate assignment needed)
           list: KpList::new(zone),
           zone,
         }
    }

    // Java: ControlList(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         Self {
           // Java: this.box = new KpBinBox(bin);
           // LLM: box renamed to bin_box (Rust keyword)
           bin_box: KpBinBox::KpBinBox_from(Some(bin)),
           // Java: this.list = new KpList(bin, this.zone);
           // LLM: list init in struct literal (no separate assignment needed)
           list: KpList::KpList_from(bin, zone),
           zone,
         }
    }
    pub fn ControlList_from(bin: &dyn KoBinTrait) -> Self { Self::new_from_bin(bin, KoZone::defaultZone()) }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get(); self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone; self.loadBin(bin, zone);
    }

    // Java: void load(ControlList value)
    pub fn load_value(&mut self, value: Option<&ControlList>) {
         // Java: if (value != null)
         if let Some(v) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(ControlList o)
    pub fn equalsLax(&mut self, o: &mut ControlList) -> bool {
         // Java: return this.equals(o, false);
         // LLM: equals renamed to equals_inner (Rust overload)
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut ControlList) -> bool {
         // Java: return this.equals(o, true);
         // LLM: equals renamed to equals_inner (Rust overload)
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut ControlList, isStrict: bool) -> bool {
         // Java: if (null == o)
         if false {
            // Java: return false;
            return false;
         }
         // Java: else
         else {
            // Java: ControlList dst = (ControlList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self as *const _, dst as *const _) {
               // Java: return true;
               return true;
            }
            // Java: else
            else {
               // Java: ArrayList<Control> dstList = new ArrayList();
               let mut dstList: Vec<Control> = Vec::new();
               // Java: dst.read(dstList);
               dst.read(&mut dstList);
               // Java: ArrayList<Control> srcList = new ArrayList();
               let mut srcList: Vec<Control> = Vec::new();
               // Java: this.read(srcList);
               self.read(&mut srcList);
               // Java: if (dstList.size() != srcList.size())
               if dstList.len() != srcList.len() {
                  // Java: return false;
                  return false;
               }
               // Java: else
               else {
                  // Java: int size = dstList.size();
                  let size = dstList.len();
                  // Java: for(int i = 0; i < size; ++i)
                  for i in 0..size {
                     // Java: Control dstClrTrans = (Control)dstList.get(i);
                     let dstClrTrans = &mut dstList[i];
                     // Java: Control srcClrTrans = (Control)srcList.get(i);
                     let srcClrTrans = &mut srcList[i];
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict && !dstClrTrans.equals(srcClrTrans) {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict && !dstClrTrans.equalsLax(srcClrTrans) {
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
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1(); self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends ControlList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = ControlList::new(zone);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static ControlList create()
    pub fn create() -> Self {
         // Java: return initialize(ControlList.class, KoBin.Null.empty());
         let empty = Null::empty(); Self::initialize(empty.as_ref())
    }

    // Java: static ControlList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(ControlList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Control> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Control>) {
         // Java: valueLst.clear();
         valueLst.clear();
         // Java: int size = this.list.size();
         let size = self.list.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size {
            // Java: valueLst.add((Control)KoBin.Decode.toAny(Control.class, this.list.get(i), this.zone));
            let bin = self.list.get(i);
            let mut inst = Control::new();
            // LLM Rust Fix: `KpList.get()` 返回 `Box<dyn KoBin>`，端口方法按 Java 取 `KoBin` 引用，
            // 与同族真实端口 `slides/Slide.rs::CommentList::read` 一致补 `.as_ref()`。
            inst.loadBin(bin.as_ref(), self.zone);
            valueLst.push(inst);
         }
    }

    // Java: void write(Iterable<Control> valueLst)
    pub fn write(&mut self, valueLst: &[Control]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Control i : valueLst)
         for i in valueLst {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = i.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
               // Java: this.list.add(bin);
               self.list.add(bin);
            }
         }
    }

    // Java: void forceWrite(Iterable<Control> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Control]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Control i : valueLst)
         for i in valueLst {
            // Java: this.list.add(i.lastBin(this.zone));
            self.list.add(i.lastBin(self.zone));
         }
    }

}

impl KoAr for CommonSlideData {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) {
        CommonSlideData::loadBin(self, var1, var2);
    }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { CommonSlideData::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CommonSlideData::lastBin(self, var1) }
}

impl KoClosable for CommonSlideData {
    fn close(&mut self) { CommonSlideData::close(self); }
}
